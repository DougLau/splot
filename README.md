# splot

Plot data to HTML / SVG

Project goals:

- Simple but powerful API
- Styling using CSS
- Usable in WebAssembly

## Example Line Plot

```rust
use splot::{plot, Chart, Domain, Page};

let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
let domain = Domain::from_data(&data).set_x(&[0.0, 200.0]);
let plot = plot::Line::new("Series", &domain, &data);
let page = Page::default().chart(
    Chart::default()
        .title("Line Plot")
        .axis(domain.bottom("X Axis"))
        .axis(domain.right("Y Axis"))
        .plot(&plot),
);
println!("{page}");
```
