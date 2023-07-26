const { promql_parse } = require("../pkg/promql_parser_js.js");

describe('parse_promql', () => {
  test('convert promql to json ast', () => {
    const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
    expect(promql_parse(query)).toMatchSnapshot();
  });
});
