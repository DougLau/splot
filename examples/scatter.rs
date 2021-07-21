use splot::{Chart, PointPlot};

fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
    let line = PointPlot::new_line(&data);
    let line_b = line.to_line(&data_b);
    let chart = Chart::builder()
        .title("Chart Title")
        .axis(line.x_axis().name("X Axis Name"))
        .axis(line.y_axis().name("Y Axis Name"))
        .axis(line.y_axis().on_right())
        .plot(line)
        .plot(line_b)
        .build();
    print!("{}", chart);
}
