extern crate promql;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn promql_parse(query: String) -> String {
    let (_, ast) = promql::parse_expr(&query).unwrap();
    let mut x: String = format!("{:?}", ast);
    // there has to be a better way to remove the Expression types?
    x = x.replace("FunCallExpr(FunCall ","").replace("SubQueryExpr(SubQueryExpr","").replace("SubQueryExpr(SubQueryExpr","").replace("VectorExprVector", "").replace("})","}").replace("VectorExpr(Vector","").replace("LabelMatcher","").replace("Some(AggregationModifier","").replace("None","false");
    return x;
}


#[test]
fn check_parser() {
    let (_, ast) = promql::parse_expr("min_over_time(rate(http_requests_total[5m])[30m:1m])").unwrap();
    let mut x: String = format!("{:?}", ast);
    x = x.replace("FunCallExpr(FunCall ","").replace("SubQueryExpr(SubQueryExpr","").replace("SubQueryExpr(SubQueryExpr","").replace("VectorExprVector", "").replace("})","}").replace("VectorExpr(Vector","").replace("LabelMatcher","").replace("Some(AggregationModifier","").replace("None","false");
    println!("{}", x);
    assert_eq!(x, x); // dummy test
}
