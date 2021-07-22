//! # splot
//!
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod domain;
mod page;
mod plot;
mod point;
mod scale;
mod text;

pub use axis::Axis;
pub use chart::{Chart, ChartBuilder, Title};
pub use domain::Domain;
pub use page::AspectRatio;
pub use plot::{Plot, PlotType, Plotter};
pub use point::Point;
