use splot::{Chart, Domain, Edge, Plot};

fn main() {
    let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
    let chart = Chart::new()
        .title("Line Plot")
        .domain(Domain::from(&data[..]).set_x(&[0.0, 200.0]))
        .axis("X Axis", Edge::Bottom)
        .axis("Y Axis", Edge::Left)
        .plot(Plot::line("Series", &data));
    print!("{chart}");
}
