use splot::{Chart, LinePlot};

fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let plot = LinePlot::new(&data);
    let chart = Chart::builder()
        .title("Chart Title")
        .axis(plot.x_axis().name("X Axis Name"))
        .axis(plot.y_axis().name("Y Axis Name"))
        .plot(plot)
        .build();
    print!("{}", chart);
}
