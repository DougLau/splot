#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod domain;
mod page;
pub mod plot;
mod point;
mod scale;
mod text;

pub use axis::Axis;
pub use chart::{AspectRatio, Chart, Title};
pub use domain::Domain;
pub use page::{Edge, Page};
pub use point::{IntoPoint, Point};
