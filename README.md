# splot

Plot data with SVG

A `Chart` can be turned into an SVG document using the `Display` trait.  That
is, using `println!`, or even `to_string()` is all that's needed.

## Example Line Plot

```rust
fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let domain = Domain::from_data(&data).with_x(&[0.0, 200.0]);
    let plot = LinePlot::new(&domain, &data).with_label(Label::new().above());
    let chart = Chart::builder()
        .with_title("Line Plot")
        .with_axis(domain.x_axis().with_name("X Axis Name"))
        .with_axis(domain.y_axis().with_name("Y Axis Name").on_right())
        .with_plot(&plot)
        .build();
    println!("{}", chart);
}
```
