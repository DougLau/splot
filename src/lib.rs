//! # splot
//!
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod line;
mod page;
mod plot;
mod point;
mod scale;
mod text;

pub use axis::Axis;
pub use chart::{Chart, ChartBuilder, Title};
pub use line::LinePlot;
pub use page::AspectRatio;
pub use plot::Plot;
pub use point::Point;
