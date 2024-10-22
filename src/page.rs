// page.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::chart::Chart;
use crate::point::IntoPoint;
use std::fmt;

/// Page to render charts
///
/// A `Page` containing one or more `Chart`s can be rendered as HTML using the
/// `Display` trait.  That is, using `println!`, or even `to_string()` is all
/// that's needed.
pub struct Page<'a, P>
where
    P: IntoPoint,
{
    charts: Vec<Chart<'a, P>>,
}

impl<'a, P> Default for Page<'a, P>
where
    P: IntoPoint,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, P> Page<'a, P>
where
    P: IntoPoint,
{
    /// Create a new page
    pub fn new() -> Self {
        Page { charts: Vec::new() }
    }

    /// Add a `Chart` to `Page`
    pub fn chart(mut self, chart: Chart<'a, P>) -> Self {
        self.charts.push(chart.stand_alone(false));
        self
    }
}

impl<'a, P> fmt::Display for Page<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<html>")?;
        writeln!(f, "<head>")?;
        writeln!(f, "<meta charset='UTF-8'>")?;
        writeln!(f, "<link href='./css/splot.css' rel='stylesheet'/>")?;
        writeln!(f, "</head>")?;
        writeln!(f, "<body>")?;
        writeln!(f, "<div class='page'>")?;
        for chart in &self.charts {
            writeln!(f, "<div class='chart'>")?;
            writeln!(f, "{chart}")?;
            writeln!(f, "</div>")?;
        }
        writeln!(f, "</div>")?;
        writeln!(f, "</body>")?;
        Ok(())
    }
}
