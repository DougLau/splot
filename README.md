# splot

Rust crate for plotting to SVG / HTML

- Styling with CSS
- Usable in WebAssembly

## Line Plot to SVG

```rust
use splot::{Chart, Domain, Edge, Plot};

let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
let chart = Chart::new()
    .title("Line Plot")
    .domain(Domain::from(&data[..]).set_x(&[0.0, 200.0]))
    .axis("X Axis", Edge::Bottom)
    .axis("Y Axis", Edge::Left)
    .plot(Plot::line("Series", &data).label());
print!("{chart}");
```

## Scatter Plot to HTML

```rust
use splot::{Chart, Edge, Page, Plot};

let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
let page = Page::new().chart(
    Chart::new()
        .title("Scatter Plot")
        .domain(&data_a[..])
        .axis("X Axis", Edge::Bottom)
        .axis("Y Axis", Edge::Left)
        .axis("", Edge::Right)
        .plot(Plot::scatter("Series A", &data_a).label())
        .plot(Plot::scatter("Series B", &data_b)),
);
print!("{page}");
```
