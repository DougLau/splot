// plot.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
//! Plot types
//!
use crate::domain::BoundDomain;
use crate::point::{IntoPoint, Point};
use crate::rect::Edge;
use crate::text::{Label, Text};
use std::fmt;

/// Plot settings
pub struct PlotSettings<'a, P>
where
    P: IntoPoint,
{
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

/// Plot for rendering data
pub enum Plot<'a, P>
where
    P: IntoPoint,
{
    /// Stacked area plot
    Area(PlotSettings<'a, P>),
    /// Line plot
    Line(PlotSettings<'a, P>),
    /// Scatter plot
    Scatter(PlotSettings<'a, P>),
}

impl<'a, P> PlotSettings<'a, P>
where
    P: IntoPoint,
{
    fn new(name: &'a str, data: &'a [P]) -> Self {
        PlotSettings {
            name,
            num: 0,
            domain: BoundDomain::default(),
            data,
            label: None,
        }
    }

    fn area_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-area' d='", self.num)?;
        if let Some(pt) = self.data.first() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(0.0);
            write!(f, "M{x} {y}")?;
        }
        for pt in self.data.iter() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            write!(f, " {x} {y}")?;
        }
        if let Some(pt) = self.data.last() {
            let pt: Point = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(0.0);
            write!(f, " {x} {y}")?;
        }
        writeln!(f, "' />")
    }

    fn line_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-line' d='", self.num)?;
        for (i, pt) in self.data.iter().enumerate() {
            let pt = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            if i == 0 {
                write!(f, "M{x} {y}")?;
            } else {
                write!(f, " {x} {y}")?;
            }
        }
        writeln!(f, "'/>")?;
        self.labels_fmt(f)
    }

    fn scatter_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<path class='plot-{} plot-scatter' d='", self.num)?;
        for (i, pt) in self.data.iter().enumerate() {
            let pt = (*pt).into();
            let x = self.domain.x_map(pt.x);
            let y = self.domain.y_map(pt.y);
            if i == 0 {
                write!(f, "M{x} {y}")?;
            } else {
                write!(f, " {x} {y}")?;
            }
        }
        writeln!(f, "' />")?;
        self.labels_fmt(f)
    }

    fn labels_fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(label) = &self.label {
            let text = Text::new(Edge::Top).class_name("plot-label");
            text.display(f)?;
            for pt in self.data.iter() {
                let pt = (*pt).into();
                let x = self.domain.x_map(pt.x);
                let y = self.domain.y_map(pt.y);
                label.display(f, x, y, pt)?;
            }
            text.display_done(f)?;
        }
        Ok(())
    }
}

impl<'a, P> fmt::Display for Plot<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Plot::Area(p) => p.area_fmt(f),
            Plot::Line(p) => p.line_fmt(f),
            Plot::Scatter(p) => p.scatter_fmt(f),
        }
    }
}

impl<'a, P> Plot<'a, P>
where
    P: IntoPoint,
{
    /// Create a new area plot
    pub fn area(name: &'a str, data: &'a [P]) -> Self {
        Plot::Area(PlotSettings::new(name, data))
    }

    /// Create a new line plot
    pub fn line(name: &'a str, data: &'a [P]) -> Self {
        Plot::Line(PlotSettings::new(name, data))
    }

    /// Create a new scatter plot
    pub fn scatter(name: &'a str, data: &'a [P]) -> Self {
        Plot::Scatter(PlotSettings::new(name, data))
    }

    fn settings(&self) -> &PlotSettings<'a, P> {
        match self {
            Plot::Area(p) => p,
            Plot::Line(p) => p,
            Plot::Scatter(p) => p,
        }
    }

    fn settings_mut(&mut self) -> &mut PlotSettings<'a, P> {
        match self {
            Plot::Area(p) => p,
            Plot::Line(p) => p,
            Plot::Scatter(p) => p,
        }
    }

    /// Get plot name
    pub fn name(&self) -> &str {
        self.settings().name
    }

    pub fn num(&mut self, num: u32) {
        self.settings_mut().num = num;
    }

    pub fn bind_domain(&mut self, domain: BoundDomain) {
        self.settings_mut().domain = domain;
    }

    /// Add labels to plot
    pub fn label(mut self) -> Self {
        self.settings_mut().label = Some(Label::new());
        self
    }
}
