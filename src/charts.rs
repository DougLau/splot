// charts.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::chart::Chart;
use crate::point::IntoPoint;
use hatmil::Page;
use std::fmt;

/// Container for multiple Charts as HTML
///
/// A `Charts` containing one or more `Chart`s can be rendered as HTML using
/// the `Display` trait.  That is, using `println!`, or even `to_string()` is
/// all that's needed.
pub struct Charts<'a, P>
where
    P: IntoPoint,
{
    charts: Vec<Chart<'a, P>>,
}

impl<P> Default for Charts<'_, P>
where
    P: IntoPoint,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, P> Charts<'a, P>
where
    P: IntoPoint,
{
    /// Create a new charts container
    pub fn new() -> Self {
        Charts { charts: Vec::new() }
    }

    /// Add a `Chart`
    pub fn chart(mut self, chart: Chart<'a, P>) -> Self {
        self.charts.push(chart.stand_alone(false));
        self
    }
}

impl<P> fmt::Display for Charts<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut page = Page::default();
        let mut html = page.html();
        let mut head = html.head();
        head.link()
            .href("./css/splot.css")
            .rel("stylesheet")
            .close();
        head.close(); // head
        let mut body = html.body();
        body.div().class("charts");
        for chart in &self.charts {
            let mut div = body.div();
            div.class("chart");
            chart.display(&mut div.svg());
            chart.legend(&mut div.div());
            div.close(); // div
        }
        writeln!(f, "{page}")
    }
}
