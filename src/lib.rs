#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod domain;
mod page;
pub mod plot;
mod point;
pub mod scale;
mod text;

pub use axis::Axis;
pub use chart::{AspectRatio, Chart, Title};
pub use domain::Domain;
pub use page::Page;
pub use point::Point;
