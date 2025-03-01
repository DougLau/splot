#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod axis;
mod chart;
mod domain;
mod page;
mod plot;
mod point;
mod rect;
mod scale;
mod text;
mod title;

pub use chart::{AspectRatio, Chart, Legend};
pub use domain::Domain;
pub use page::Page;
pub use plot::Plot;
pub use point::{IntoPoint, Point};
pub use rect::Edge;
pub use title::Title;
