extern crate promql_parser;
use wasm_bindgen::prelude::*;
use promql_parser::parser;
use promql_parser::parser::*;
use promql_parser::label::*;
use std::time::{Duration, SystemTime};
use serde_json::{json, Value};
use iso8601_timestamp::Timestamp;
use serde::ser::Serialize;

trait ToSerde {
    fn to_serde(&self) -> Value;
}

impl<T: ToSerde> ToSerde for Box<T> {
    fn to_serde(&self) -> Value {
        self.as_ref().to_serde()
    }
}

impl<T: ToSerde> ToSerde for Option<T> {
    fn to_serde(&self) -> Value {
        match self {
            Some(something) => something.to_serde(),
            None => json!(null),
        }
    }
}

impl<T: ToSerde> ToSerde for Vec<T> {
    fn to_serde(&self) -> Value {
        json!(self.iter().map(|item| item.to_serde()).collect::<Vec<Value>>())
    }
}

impl ToSerde for str {
    fn to_serde(&self) -> Value {
        json!(self)
    }
}

impl ToSerde for String {
    fn to_serde(&self) -> Value {
        json!(self)
    }
}

impl ToSerde for bool {
    fn to_serde(&self) -> Value {
        json!(self)
    }
}

impl ToSerde for TokenType {
    fn to_serde(&self) -> Value {
        json!(self.to_string())
    }
}

impl ToSerde for Offset {
    fn to_serde(&self) -> Value {
        match self {
            Offset::Pos(dur) => dur.to_serde(),
            Offset::Neg(dur) => json!(dur.as_secs() as i32 * -1),
        }
    }
}

impl ToSerde for Duration {
    fn to_serde(&self) -> Value {
        json!(self.as_secs())
    }
}

impl ToSerde for SystemTime {
    fn to_serde(&self) -> Value {
        json!(Timestamp::from(*self))
    }
}

impl ToSerde for AtModifier {
    fn to_serde(&self) -> Value {
        match self {
            AtModifier::Start => json!("start"),
            AtModifier::End => json!("at": "end"),
            AtModifier::At(offset) => json!(offset.to_serde()),
        }
    }
}

impl ToSerde for VectorMatchCardinality {
    fn to_serde(&self) -> Value {
        match self {
            VectorMatchCardinality::OneToOne => json!({ "@type": "one-to-one" }),
            VectorMatchCardinality::ManyToOne(labels) =>
                json!({ "@type": "many-to-one", "labels": labels.to_serde() }),
            VectorMatchCardinality::OneToMany(labels) =>
                json!({ "@type": "one-to-many", "labels": labels.to_serde() }),
            VectorMatchCardinality::ManyToMany => json!({ "@type": "many-to-many" }),
        }
    }
}

impl ToSerde for Labels {
    fn to_serde(&self) -> Value {
        self.labels.to_serde()
    }
}

impl ToSerde for MatchOp {
    fn to_serde(&self) -> Value {
        match self {
            MatchOp::Equal => json!("="),
            MatchOp::NotEqual => json!("!="),
            MatchOp::Re(_) => json!("=~"),
            MatchOp::NotRe(_) => json!("!~"),
        }
    }
}

impl ToSerde for Matcher {
    fn to_serde(&self) -> Value {
        json!({
            "name": self.name.to_serde(),
            "op": self.op.to_serde(),
            "value": self.value.to_serde(),
        })
    }
}

impl ToSerde for Matchers {
    fn to_serde(&self) -> Value {
        self.matchers.to_serde()
    }
}

impl ToSerde for LabelModifier {
    fn to_serde(&self) -> Value {
        match self {
            LabelModifier::Include(labels) =>
                json!({ "include": labels.to_serde() }),
            LabelModifier::Exclude(labels) =>
                json!({ "exclude": labels.to_serde() }),
        }
    }
}

impl ToSerde for BinModifier {
    fn to_serde(&self) -> Value {
        json!({
            "card": self.card.to_serde(),
            "matching": self.matching.to_serde(),
            "return_bool": self.return_bool.to_serde(),
        })
    }
}

impl ToSerde for VectorSelector {
    fn to_serde(&self) -> Value {
        json!({
            "@type": "vector_selector",
            "name": self.name.to_serde(),
            "matchers": self.matchers.to_serde(),
            "offset": self.offset.to_serde(),
            "at": self.at.to_serde(),
        })
    }
}

impl ToSerde for ValueType {
    fn to_serde(&self) -> Value {
        match self {
            ValueType::Vector => json!("vector"),
            ValueType::Scalar => json!("scalar"),
            ValueType::Matrix => json!("matrix"),
            ValueType::String => json!("string"),
        }
    }
}

impl ToSerde for Function {
    fn to_serde(&self) -> Value {
        json!({
            "name": self.name.to_serde(),
            "arg_types": self.arg_types.to_serde(),
            "variadic": self.variadic.to_serde(),
            "return_type": self.return_type.to_serde(),
        })
    }
}

impl ToSerde for FunctionArgs {
    fn to_serde(&self) -> Value {
        self.args.to_serde()
    }
}

impl ToSerde for Expr {
    fn to_serde(&self) -> Value {
        match self {
            Expr::Aggregate(AggregateExpr { op, expr, param, modifier }) =>
                json!({
                    "@type": "aggregate",
                    "op": op.to_serde(),
                    "expr": expr.to_serde(),
                    "param": param.to_serde(),
                    "modifier": modifier.to_serde(),
                }),
            Expr::Unary(UnaryExpr { expr }) =>
                json!({
                    "@type": "unary",
                    "expr": expr.to_serde(),
                }),
            Expr::Binary(BinaryExpr { lhs, op, rhs, modifier }) =>
                json!({
                    "@type": "binary",
                    "lhs": lhs.to_serde(),
                    "op": op.to_serde(),
                    "rhs": rhs.to_serde(),
                    "modifier": modifier.to_serde(),
                }),
            Expr::Paren(ParenExpr { expr }) =>
                json!({
                    "@type": "paren",
                    "expr": expr.to_serde(),
                }),
            Expr::Subquery(SubqueryExpr { expr, offset, at, range, step }) =>
                json!({
                    "@type": "subquery",
                    "expr": expr.to_serde(),
                    "offset": offset.to_serde(),
                    "at": at.to_serde(),
                    "range": range.to_serde(),
                    "step": step.to_serde(),
                }),
            Expr::NumberLiteral(NumberLiteral { val }) =>
                json!({
                    "@type": "number",
                    "value": val,
                }),
            Expr::StringLiteral(StringLiteral { val }) =>
                json!({
                    "@type": "string",
                    "value": val,
                }),
            Expr::VectorSelector(vs) =>
                vs.to_serde(),
            Expr::MatrixSelector(MatrixSelector { vs, range }) =>
                json!({
                    "@type": "matrix_selector",
                    "vector": vs.to_serde(),
                    "range": range.to_serde(),
                }),
            Expr::Call(Call { func, args }) =>
                json!({
                    "@type": "call",
                    "function": func.to_serde(),
                    "args": args.to_serde(),
                }),
            Expr::Extension(_) => json!({ "expr": {} }),
        }
    }
}

#[wasm_bindgen]
pub fn promql_parse(query: String) -> Result<JsValue, JsError> {
    match parser::parse(&query) {
        Err(err) => Err(JsError::new(&err)),
        Ok(expr) =>
            Ok(
                expr
                    .to_serde()
                    .serialize(
                        &serde_wasm_bindgen::Serializer::new()
                            .serialize_missing_as_null(true)
                            .serialize_maps_as_objects(true)
                    )
                    .unwrap()
            ),
    }
}


#[test]
fn check_parser() {
    let payloads: Vec<String> = vec![
        "a or b".to_string(),
        "max
  by(node) (max by(instance) (kubelet_running_pod_count{job=\"kubelet\"}) *
  on(instance) group_left(node) group(kubelet_node_name{job=\"kubelet\"}) by (instance)) / max by(node)
  (kube_node_status_capacity_pods{job=\"kube-state-metrics\"}) > 0.95".to_string(),
        "http_requests_total{code=\"200\"}[30m:1m]".to_string(),
        "min_over_time(rate(http_requests_total{code=\"200\"}[5m])[30m:1m]) > 1".to_string(),
        "sum(rate(foo{bar=\"baz\"}[5m])) by (x,y)".to_string(),
        "foo{bar=~\"abc\"}".to_string(),
        "foo == bar".to_string(),
    ];
    for payload in payloads.iter() {
        println!("Payload: {}", payload);
        assert!(
            parser::parse(&payload)
                .and_then(|v| Ok(v.to_serde())).is_ok(),
            "failed to parse or serialize"
        );
    }
}
