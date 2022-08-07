# promql-parser-js
PromQL parsing wasm module based on Rust

### 🥇 Install
```bash
npm install @qxip/promql-parser-js
```

### Functions
- `promql_parse`

### usage
```javascript
const { promql_parse } = require("../pkg/promql_parser_js.js");
const jsonic = require('jsonic');
const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
try {
  const s = promql_parse(query);
  const parsed = JSON.stringify(jsonic(s));
  console.log(parsed);
} catch(e) { console.log(e) }
```

### 🏗️ Build
Rebuild wasm package release. Not needed for regular module usage.
```bash
npm install
npm run build
npm test
```
