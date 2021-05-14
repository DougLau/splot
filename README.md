# splot

Plot data with SVG

A `Chart` can be turned into an SVG document using the `Display` trait.  That
is, using `println!`, or even `to_string()` is all that's needed.

## Example Line Plot

```rust
fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let line = LinePlot::new(&data)
        .x_domain(&[0.0, 200.0])
        .labels(Label::new().above());
    let chart = Chart::builder()
    	.title("Line Chart")
        .axis(line.x_axis().name("X Axis Name"))
        .axis(line.y_axis().name("Y Axis Name").on_right())
        .plot(line)
        .build();
    println!("{}", chart);
}
```
