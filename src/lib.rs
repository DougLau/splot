#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod charts;
mod domain;
mod plot;
mod point;
mod rect;
mod scale;
mod text;
mod title;

pub use chart::{AspectRatio, Chart};
pub use charts::Charts;
pub use domain::Domain;
pub use plot::Plot;
pub use point::{IntoPoint, Point};
pub use rect::Edge;
pub use title::Title;
