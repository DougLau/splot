use splot::{Chart, Domain, Plotter, PlotType};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
    let domain = Domain::default().with_data(&data_a);
    let chart = Chart::builder()
        .with_title("Scatter Plot")
        .with_axis(domain.x_axis().name("X Axis Name"))
        .with_axis(domain.y_axis().name("Y Axis Name"))
        .with_axis(domain.y_axis().on_right())
        .with_plot(
            Plotter::new(&domain, &data_a).with_plot_type(PlotType::Scatter),
        )
        .with_plot(
            Plotter::new(&domain, &data_b).with_plot_type(PlotType::Scatter),
        )
        .build();
    print!("{}", chart);
}
