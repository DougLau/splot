use splot::{Chart, LinePlot};

fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let line = LinePlot::new(&data);
    let chart = Chart::builder()
        .title("Chart Title")
        .axis(line.x_axis().name("X Axis Name"))
        .axis(line.y_axis().name("Y Axis Name"))
        .axis(line.y_axis().on_right())
        .plot(line)
        .build();
    print!("{}", chart);
}