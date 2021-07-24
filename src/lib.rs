//! # splot
//!
//! Plot data with SVG
//!
//! A `Chart` can be turned into an SVG document using the `Display` trait.
//! That is, using `println!`, or even `to_string()` is all that's needed.
//!
//! ## Example Line Plot
//! 
//! ```rust
//! use splot::{Chart, Domain, plot};
//!
//! let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
//! let domain = Domain::from_data(&data).with_x(&[0.0, 200.0]);
//! let plot = plot::Line::new(&domain, &data);
//! let chart = Chart::builder()
//!     .with_title("Line Plot")
//!     .with_axis(domain.x_axis().with_name("X Axis Name"))
//!     .with_axis(domain.y_axis().with_name("Y Axis Name").on_right())
//!     .with_plot(&plot)
//!     .build();
//! println!("{}", chart);
//! ```
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
