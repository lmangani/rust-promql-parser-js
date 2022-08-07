# 🥇 promql-parser-js
PromQL parsing wasm module based on Rust

### Install
```bash
npm install @qxip/promql-parser-js
```

### Functions
- `promql_parse`

#### Usage
```javascript
const jsonic = require('jsonic'); // parse imperfect JSON we might return
const { promql_parse } = require("promql-parser-js"); // parse PromQL to JSON
const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
try {
  const s = promql_parse(query);
  const parsed = JSON.stringify(jsonic(s));
  console.log(parsed);
} catch(e) { console.log(e) }
```

### Build
Rebuild wasm package release. Not needed for regular module usage.
```bash
npm install
npm run build
npm test
```
