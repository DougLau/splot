use splot::{plot, Chart, Domain, Page};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
    let domain = Domain::from_data(&data_a);
    let plot_a = plot::Scatter::new("Series A", &domain, &data_a).label();
    let plot_b = plot::Scatter::new("Series B", &domain, &data_b);
    let page = Page::default().chart(
        Chart::default()
            .title("Scatter Plot")
            .axis(domain.x_axis("X Axis Name"))
            .axis(domain.y_axis("Y Axis Name"))
            .axis(domain.y_axis("").on_right())
            .plot(&plot_a)
            .plot(&plot_b),
    );
    print!("{page}");
}
