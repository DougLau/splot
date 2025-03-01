// plot.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::domain::BoundDomain;
use crate::point::{IntoPoint, Point};
use crate::rect::Edge;
use crate::text::{Label, Text};
use std::fmt;

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

    /// Format an area plot
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

    /// Format a line plot
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

    /// Format a scatter plot
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

    /// Format plot labels
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

impl<P> fmt::Display for Plot<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.plot_tp {
            PlotType::Area => self.area_fmt(f),
            PlotType::Line => self.line_fmt(f),
            PlotType::Scatter => self.scatter_fmt(f),
        }
    }
}
