// plot.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
//! Plot types
//!
use crate::domain::BoundDomain;
use crate::page::Edge;
use crate::point::{IntoPoint, Point};
use crate::text::{Label, Text};
use std::fmt;

/// Stacked area plot
///
/// Data is drawn as filled-in areas, stacked vertically.
pub struct Area<'a, P>
where
    P: IntoPoint,
{
    name: &'a str,
    num: u32,
    domain: BoundDomain,
    data: &'a [P],
}

/// Line plot
///
/// Data is drawn as a series of points connected by line segments.
pub struct Line<'a, P>
where
    P: IntoPoint,
{
    name: &'a str,
    num: u32,
    domain: BoundDomain,
    data: &'a [P],
    label: Option<Label>,
}

/// Scatter plot
///
/// Data is drawn as unconnected points.
pub struct Scatter<'a, P>
where
    P: IntoPoint,
{
    name: &'a str,
    num: u32,
    domain: BoundDomain,
    data: &'a [P],
    label: Option<Label>,
}

/// Plot for rendering data
pub enum Plot<'a, P>
where
    P: IntoPoint,
{
    Area(Area<'a, P>),
    Line(Line<'a, P>),
    Scatter(Scatter<'a, P>),
}

impl<'a, P> fmt::Display for Area<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
}

impl<'a, P> Area<'a, P>
where
    P: IntoPoint,
{
    /// Create a new stacked area plot
    pub fn new(name: &'a str, data: &'a [P]) -> Self {
        Area {
            name,
            num: 0,
            domain: BoundDomain::default(),
            data,
        }
    }
}

impl<'a, P> fmt::Display for Line<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        self.display_labels(f)
    }
}

impl<'a, P> Line<'a, P>
where
    P: IntoPoint,
{
    /// Create a new line plot
    pub fn new(name: &'a str, data: &'a [P]) -> Self {
        Line {
            name,
            num: 0,
            domain: BoundDomain::default(),
            data,
            label: None,
        }
    }

    /// Add labels to plot
    pub fn label(mut self) -> Self {
        self.label = Some(Label::new());
        self
    }

    fn display_labels(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl<'a, P> fmt::Display for Scatter<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        self.display_labels(f)
    }
}

impl<'a, P> Scatter<'a, P>
where
    P: IntoPoint,
{
    /// Create a new scatter plot
    pub fn new(name: &'a str, data: &'a [P]) -> Self {
        Scatter {
            name,
            num: 0,
            domain: BoundDomain::default(),
            data,
            label: None,
        }
    }

    /// Add labels to plot
    pub fn label(mut self) -> Self {
        self.label = Some(Label::new());
        self
    }

    fn display_labels(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(label) = &self.label {
            let text = Text::new(Edge::Top).class_name("plot-label");
            text.display(f)?;
            for pt in self.data.iter() {
                let pt: Point = (*pt).into();
                let x = self.domain.x_map(pt.x);
                let y = self.domain.y_map(pt.y);
                label.display(f, x, y, pt)?;
            }
            text.display_done(f)?;
        }
        Ok(())
    }
}

impl<'a, P> From<Area<'a, P>> for Plot<'a, P>
where
    P: IntoPoint,
{
    fn from(area: Area<'a, P>) -> Self {
        Plot::Area(area)
    }
}

impl<'a, P> From<Line<'a, P>> for Plot<'a, P>
where
    P: IntoPoint,
{
    fn from(line: Line<'a, P>) -> Self {
        Plot::Line(line)
    }
}

impl<'a, P> From<Scatter<'a, P>> for Plot<'a, P>
where
    P: IntoPoint,
{
    fn from(scatter: Scatter<'a, P>) -> Self {
        Plot::Scatter(scatter)
    }
}

impl<'a, P> fmt::Display for Plot<'a, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Plot::Area(p) => write!(f, "{p}"),
            Plot::Line(p) => write!(f, "{p}"),
            Plot::Scatter(p) => write!(f, "{p}"),
        }
    }
}

impl<'a, P> Plot<'a, P>
where
    P: IntoPoint,
{
    /// Get plot name
    pub fn name(&self) -> &str {
        match self {
            Plot::Area(p) => p.name,
            Plot::Line(p) => p.name,
            Plot::Scatter(p) => p.name,
        }
    }

    pub fn num(&mut self, num: u32) {
        match self {
            Plot::Area(p) => p.num = num,
            Plot::Line(p) => p.num = num,
            Plot::Scatter(p) => p.num = num,
        }
    }

    pub fn bind_domain(&mut self, domain: BoundDomain) {
        match self {
            Plot::Area(p) => p.domain = domain,
            Plot::Line(p) => p.domain = domain,
            Plot::Scatter(p) => p.domain = domain,
        }
    }
}
