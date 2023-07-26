/*
import("../pkg/promql_parser.js")
  .then((module) => {
	const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
	const s = promql_parse(query);
	console.log(s);
  })
  .catch(console.error);
*/

const args = process.argv.slice(2);
const { promql_parse } = require("../pkg/promql_parser_js.js");
const query = args[0] || 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
try {
  const parsed = promql_parse(query);
  console.log(parsed);
} catch(e) { console.log(e) }
