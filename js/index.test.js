const { promql_parse } = require("../pkg/promql_parser_js.js");

describe('parse_promql', () => {
  test('convert promql to json ast', () => {

    const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
    const result = promql_parse(query);
    const compare = JSON.parse('{"name":"sum","args":[{"name":"rate","args":[{"name":"foo","label_matchers":[{"op":"Equal","name":"bar","value":"baz"}],"offset":false,"range":"Some(300s)"}],"aggregation":false}],"aggregation":{"action":"By","labels":["x","y"]}}');
    expect(result).toEqual(compare);
  });
});
