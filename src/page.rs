// page.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::chart::Chart;
use crate::point::IntoPoint;
use hatmil::Html;
use std::fmt;

/// Page to render charts as HTML
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

impl<P> Default for Page<'_, P>
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

    /// Add a `Chart`
    pub fn chart(mut self, chart: Chart<'a, P>) -> Self {
        self.charts.push(chart.stand_alone(false));
        self
    }
}

impl<P> fmt::Display for Page<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut html = Html::new();
        html.html();
        html.head();
        html.meta().attr("charset", "UTF-8");
        html.link().href("./css/splot.css").rel("stylesheet");
        html.end(); // head
        html.body();
        html.div().class("page");
        for chart in &self.charts {
            html.div().class("chart");
            chart.display(&mut html);
            chart.legend(&mut html);
            html.end(); // div
        }
        writeln!(f, "{html}")
    }
}
