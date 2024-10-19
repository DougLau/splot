use splot::{plot, Chart, Domain, Page};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39), (210, 43)];
    let domain = Domain::from_data(&data_a).including(&data_b);
    let plot_a = plot::Line::new("Series A", &domain, &data_a).label();
    let plot_b = plot::Line::new("Series B", &domain, &data_b);
    let page = Page::default().chart(
        Chart::default()
            .title("Line Plot")
            .axis(domain.bottom("X Axis Name"))
            .axis(domain.left("Y Axis Name"))
            .axis(domain.right(""))
            .plot(&plot_a)
            .plot(&plot_b),
    );
    print!("{page}");
}
