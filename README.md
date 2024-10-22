# splot

Plot data to HTML / SVG

- Simple but powerful API
- Styling using CSS
- Usable in WebAssembly

## Example Line Plot

```rust
use splot::{Chart, Domain, Edge, Line, Page};

let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
let domain = Domain::from_data(&data).set_x(&[0.0, 200.0]);
let page = Page::new().chart(
    Chart::default()
        .title("Line Plot")
        .domain(domain)
        .axis("X Axis", Edge::Bottom)
        .axis("Y Axis", Edge::Right)
        .plot(Line::new("Series", &data)),
);
println!("{page}");
```
