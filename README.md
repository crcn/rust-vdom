```rust
let ast = parse("<div id='test' />");
let ast2 = parse("<div id='test' />");
let ots = diff(&ast, &ast2);
let ast3 = patch(&ast, &ots);
```