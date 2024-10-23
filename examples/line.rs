use splot::{Chart, Domain, Edge, Page, Plot};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39), (210, 43)];
    let page = Page::new().chart(
        Chart::new()
            .title("Line Plot")
            .domain(Domain::from(&data_a[..]).including(&data_b))
            .axis("X Axis", Edge::Bottom)
            .axis("Y Axis", Edge::Left)
            .axis("", Edge::Right)
            .plot(Plot::line("Series A", &data_a).label())
            .plot(Plot::line("Series B", &data_b)),
    );
    print!("{page}");
}
