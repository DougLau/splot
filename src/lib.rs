//! # splot
//!
#![forbid(unsafe_code)]

pub mod axis;
mod chart;
mod domain;
mod page;
mod plot;
mod point;
mod private;
mod scale;
mod text;

pub use chart::{Chart, ChartBuilder, Title};
pub use domain::Domain;
pub use page::AspectRatio;
pub use plot::{AreaPlot, LinePlot, Plot, ScatterPlot};
pub use point::Point;
