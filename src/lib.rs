//! # splot
//!
#![forbid(unsafe_code)]

pub mod axis;
mod chart;
mod domain;
mod page;
pub mod plot;
mod point;
mod private;
pub mod scale;
mod text;

pub use chart::{Chart, ChartBuilder, Title};
pub use domain::Domain;
pub use page::AspectRatio;
pub use point::Point;
