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
//! let domain = Domain::from_data(&data).set_x(&[0.0, 200.0]);
//! let plot = plot::Line::new("Series", &domain, &data);
//! let page = Page::default().chart(
//!     Chart::default()
//!         .title("Line Plot")
//!         .axis(domain.x_axis("X Axis Name"))
//!         .axis(domain.y_axis("Y Axis Name").on_right())
//!         .plot(&plot),
//! );
//! println!("{page}");
//! ```
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
pub use chart::{Chart, Title};
pub use domain::Domain;
pub use page::{AspectRatio, Page};
pub use point::Point;
