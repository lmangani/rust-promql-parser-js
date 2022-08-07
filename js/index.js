/*
import("../pkg/promql_parser.js")
  .then((module) => {
	const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
	const s = promql_parse(query);
	console.log(s);
  })
  .catch(console.error);
*/

const { promql_parse } = require("../pkg/promql_parser_js.js");
const jsonic = require('jsonic');
const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
try {
  const s = promql_parse(query);
  const parsed = JSON.stringify(jsonic(s));
  console.log(parsed);
} catch(e) { console.log(e) }
