#![doc = include_str!("../README.md")]
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
pub use chart::{AspectRatio, Chart, Title};
pub use domain::Domain;
pub use page::{Edge, Page};
pub use plot::{Area, Line, Plot, Scatter};
pub use point::{IntoPoint, Point};
