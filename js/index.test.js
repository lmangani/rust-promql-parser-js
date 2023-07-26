const { promql_parse } = require("../pkg/promql_parser_js.js");

describe('parse_promql', () => {
  test('convert promql to json ast', () => {

    const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
    const result = promql_parse(query);
    expect(result).toEqual({
      '@type': 'aggregate',
      op: 'sum',
      modifier: {
        include: ['x', 'y'],
      },
      param: null,
      expr: {
        '@type': 'call',
        function: {
          name: 'rate',
          arg_types: ['matrix'],
          return_type: 'vector',
          variadic: false,
        },
        args: [
          {
            '@type': 'matrix_selector',
            range: 300,
            offset: null,
            vector: {
              '@type': 'vector_selector',
              name: 'foo',
              matchers: [
                { name: 'bar', op: '=', value: 'baz' },
              ],
              at: null,
            },
          },
        ],
      },
    });
  });
});
