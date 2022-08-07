extern crate promql;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn promql_parse(query: String) -> String {
    let (_, ast) = promql::parse_expr(&query).unwrap();
    let mut x: String = format!("{:?}", ast);
    // there has to be a better way to remove the Expression types?
    x = x.replace("FunCallExpr(FunCall ","");
    x = x.replace("SubQueryExpr(SubqueryExpr","");
    x = x.replace("VectorExprVector", "");
    x = x.replace("LabelMatcher","").replace("Some(AggregationModifier","").replace("None","false");
    x = x.replace("})","}").replace("VectorExpr(Vector","");
    x = x.replace("Some(","\"").replace("s)", "s\"");
    return x;
}


#[test]
fn check_parser() {
    let (_, ast) = promql::parse_expr("min_over_time(rate(http_requests_total[5m])[30m:1m])").unwrap();
    print_type_of(&ast);
    let mut x: String = format!("{:?}", ast);
    x = x.replace("FunCallExpr(FunCall ","");
    x = x.replace("SubQueryExpr(SubqueryExpr","");
    x = x.replace("VectorExprVector", "");
    x = x.replace("LabelMatcher","").replace("Some(AggregationModifier","").replace("None","false");
    x = x.replace("})","}").replace("VectorExpr(Vector","");
    x = x.replace("Some(","\"").replace("s)", "s\"");
    // print_type_of(&x);
    println!("{}", x);
    assert_eq!(x, x); // dummy test
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
