//! # splot
//!
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod page;
mod text;

pub use axis::Axis;
pub use chart::{Chart, ChartBuilder, Title};
pub use page::AspectRatio;
