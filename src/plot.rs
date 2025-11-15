// plot.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::domain::BoundDomain;
use crate::point::{IntoPoint, Point};
use crate::rect::Edge;
use crate::text::{Label, Text};
use hatmil::{Html, Svg};

/// Plot Type
#[derive(Clone, Copy, Debug)]
pub enum PlotType {
    /// Stacked area plot
    Area,
    /// Line plot
    Line,
    /// Scatter plot
    Scatter,
}

/// Plot for rendering data
pub struct Plot<'a, P>
where
    P: IntoPoint,
{
    /// Plot type
    plot_tp: PlotType,
    /// Values name
    name: &'a str,
    /// Number within chart
    num: u32,
    /// Domain bound to rectangle
    domain: BoundDomain,
    /// Data values
    data: &'a [P],
    /// Label settings
    label: Option<Label>,
}

impl<'a, P> Plot<'a, P>
where
    P: IntoPoint,
{
    /// Create new plot
    fn new(tp: PlotType, name: &'a str, data: &'a [P]) -> Self {
        Plot {
            plot_tp: tp,
            name,
            num: 0,
            domain: BoundDomain::default(),
            data,
            label: None,
        }
    }

    /// Create a new area plot
    pub fn area(name: &'a str, data: &'a [P]) -> Self {
        Plot::new(PlotType::Area, name, data)
    }

    /// Create a new line plot
    pub fn line(name: &'a str, data: &'a [P]) -> Self {
        Plot::new(PlotType::Line, name, data)
    }

    /// Create a new scatter plot
    pub fn scatter(name: &'a str, data: &'a [P]) -> Self {
        Plot::new(PlotType::Scatter, name, data)
    }

    /// Get plot name
    pub(crate) fn name(&self) -> &str {
        self.name
    }

    /// Get mutable plot number
    pub(crate) fn num_mut(&mut self) -> &mut u32 {
        &mut self.num
    }

    /// Get mutable domain
    pub(crate) fn domain_mut(&mut self) -> &mut BoundDomain {
        &mut self.domain
    }

    /// Add labels to plot
    pub fn label(mut self) -> Self {
        self.label = Some(Label::new());
        self
    }

    /// Display a plot
    pub(crate) fn display(&self, html: &mut Html) {
        match self.plot_tp {
            PlotType::Area => self.display_area(html),
            PlotType::Line => self.display_line(html),
            PlotType::Scatter => self.display_scatter(html),
        }
    }

    /// Display an area plot
    fn display_area(&self, html: &mut Html) {
        let cls = format!("plot-{} plot-area", self.num);
        let mut d = String::new();
        if let Some(pt) = self.data.first() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(0.0);
            d.push_str(&format!("M{x} {y}"));
        }
        for pt in self.data.iter() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            d.push_str(&format!(" {x} {y}"));
        }
        if let Some(pt) = self.data.last() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(0.0);
            d.push_str(&format!(" {x} {y}"));
        }
        let path = Svg::new(html).path();
        path.class(cls).d(d).end();
        self.display_labels(html);
    }

    /// Display a line plot
    fn display_line(&self, html: &mut Html) {
        let cls = format!("plot-{} plot-line", self.num);
        let mut d = String::new();
        for (i, pt) in self.data.iter().enumerate() {
            if i == 0 {
                d.push('M');
            } else {
                d.push(' ');
            }
            let pt = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            d.push_str(&format!("{x} {y}"));
        }
        let path = Svg::new(html).path();
        path.class(cls).d(d).end();
        self.display_labels(html);
    }

    /// Display a scatter plot
    fn display_scatter(&self, html: &mut Html) {
        let cls = format!("plot-{} plot-scatter", self.num);
        let mut d = String::new();
        for (i, pt) in self.data.iter().enumerate() {
            if i == 0 {
                d.push('M');
            } else {
                d.push(' ');
            }
            let pt = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            d.push_str(&format!("{x} {y}"));
        }
        let path = Svg::new(html).path();
        path.class(cls).d(d).end();
        self.display_labels(html);
    }

    /// Display plot labels
    fn display_labels(&self, html: &mut Html) {
        if let Some(label) = &self.label {
            let text = Text::new(Edge::Top).class_name("plot-label");
            text.display(html);
            for pt in self.data.iter() {
                let pt = (*pt).into();
                let x = self.domain.x_map(pt.x);
                let y = self.domain.y_map(pt.y);
                label.display_at(x, y, pt, html);
            }
            html.end(); // text
        }
    }
}
