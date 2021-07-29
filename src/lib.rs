//! # splot
//!
//! Plot data to HTML
//!
//! Project goals:
//!
//! - Simple but powerful API
//! - Styling using CSS
//! - Usable in WebAssembly
//!
//! ## Example Line Plot
//!
//! ```rust
//! use splot::{plot, Chart, Domain, Page};
//!
//! let data = vec![(13, 74), (111, 37), (125, 52), (190, 66)];
//! let domain = Domain::from_data(&data).with_x(&[0.0, 200.0]);
//! let plot = plot::Line::new("Series", &domain, &data);
//! let page = Page::default().with_chart(
//!     Chart::default()
//!         .with_title("Line Plot")
//!         .with_axis(domain.x_axis().with_name("X Axis Name"))
//!         .with_axis(domain.y_axis().with_name("Y Axis Name").on_right())
//!         .with_plot(&plot),
//! );
//! println!("{}", page);
//! ```
#![forbid(unsafe_code)]

pub mod axis;
mod chart;
mod domain;
mod page;
pub mod plot;
mod point;
pub mod scale;
mod text;

pub use chart::{Chart, Title};
pub use domain::Domain;
pub use page::{AspectRatio, Page};
pub use point::Point;
