use splot::{plot, Chart, Domain, Page};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
    let domain = Domain::from_data(&data_a);
    let plot_a = plot::Area::new("Series A", &domain, &data_a);
    let plot_b = plot::Area::new("Series B", &domain, &data_b);
    let mut page = Page::default();
    page.add_chart(
        Chart::builder()
            .with_title("Area Plot")
            .with_axis(domain.x_axis().with_name("X Axis Name"))
            .with_axis(domain.y_axis().with_name("Y Axis Name"))
            .with_axis(domain.y_axis().on_right())
            .with_plot(&plot_a)
            .with_plot(&plot_b)
            .build(),
    );
    print!("{}", page);
}
