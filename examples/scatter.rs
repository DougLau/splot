use splot::{Chart, Domain, Edge, Page, Scatter};

fn main() {
    let data_a = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let data_b = vec![(22, 50), (105, 44), (120, 67), (180, 39)];
    let domain = Domain::from_data(&data_a);
    let page = Page::new().chart(
        Chart::new()
            .title("Scatter Plot")
            .domain(domain)
            .axis("X Axis", Edge::Bottom)
            .axis("Y Axis", Edge::Left)
            .axis("", Edge::Right)
            .plot(Scatter::new("Series A", &data_a).label())
            .plot(Scatter::new("Series B", &data_b)),
    );
    print!("{page}");
}
