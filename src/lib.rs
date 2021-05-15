//! # splot
//!
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod page;
mod plot;
mod point;
mod scale;
mod text;

pub use axis::Axis;
pub use chart::{Chart, ChartBuilder, Title};
pub use page::AspectRatio;
pub use plot::Plot;
pub use point::{Point, PointPlot};
