use splot::{Chart, Domain, plot};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39), (210, 43)];
    let domain = Domain::from_data(&data_a).with_data(&data_b);
    let plot_a = plot::Line::new(&domain, &data_a);
    let plot_b = plot::Line::new(&domain, &data_b);
    let chart = Chart::builder()
        .with_title("Line Plot")
        .with_axis(domain.x_axis().with_name("X Axis Name"))
        .with_axis(domain.y_axis().with_name("Y Axis Name"))
        .with_axis(domain.y_axis().on_right())
        .with_plot(&plot_a)
        .with_plot(&plot_b)
        .build();
    print!("{}", chart);
}
