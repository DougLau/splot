# splot

Plot data with SVG

A `Chart` can be turned into an SVG document using the `Display` trait.  That
is, using `println!`, or even `to_string()` is all that's needed.

## Example Line Chart

```rust
fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let line = LinePlot::new(data)
        .x_domain(&[0.0, 200.0])
        .marker(Marker::Circle(5))
        .labels(Label::new().above());
    let chart = Chart::builder()
        .size(800, 600)
    	.title("Line Chart")
        .axis(plot.x_axis().name("X Axis Name"))
        .axis(plot.y_axis().name("Y Axis Name").inverted().on_right())
        .plot(line)
        .build();
    println!("{}", chart);
}
```
