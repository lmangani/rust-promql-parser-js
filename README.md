# ⚙️ promql-parser-js
PromQL parsing wasm module based on Rust crate [promql-rs](https://github.com/detailyang/promql-rs)

#### Status
* Experiemental, don't use it!

### Install
```bash
npm install @qxip/promql-parser-js
```

### Functions
- `promql_parse`

#### Usage
```javascript
const { promql_parse } = require("@qxip/promql-parser-js"); // parse PromQL to JSON
const query = 'sum(rate(foo{bar="baz"}[5m])) by (x,y)';
try {
  const parsed = promql_parse(query);
  console.log(parsed);
} catch(e) { console.log(e) }
```

```bash
node js/index.js 'rate(foo{bar="baz"}[5m])'
```
```json
{
  "name": "rate",
  "args": [
    {
      "name": "foo",
      "label_matchers": [
        {
          "op": "Equal",
          "name": "bar",
          "value": "baz"
        }
      ],
      "offset": false,
      "range": "300s"
    }
  ],
  "aggregation": false
}
```

### Build
Rebuild wasm package release. Not needed for regular module usage.
```bash
npm install
npm run build
npm test
```

-------

## TODO
* [x] WASM binding + NODE module
* [ ] Convert `Expr` to json w/o Debug print
* [ ] Implement test cases
* [ ] Document
