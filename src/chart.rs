// chart.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::axis::Axis;
use crate::domain::Domain;
use crate::plot::Plot;
use crate::point::IntoPoint;
use crate::rect::{Edge, Rect, ViewBox};
use crate::title::Title;
use std::fmt;

/// Marker shapes
const MARKERS: &[&str] = &[
    "<circle r='1'/>",
    "<rect x='-1' y='-1' width='2' height='2'/>",
    "<path d='M0 -1 1 1 -1 1z'/>",
    "<path d='M1 0 -1 1 -1 -1z'/>",
    "<path d='M0 1 -1 -1 1 -1z'/>",
    "<path d='M-1 0 1 -1 1 1z'/>",
    "<path d='M0 -1 1 0 0 1 -1 0z'/>",
    "<path d='M-1 -1 0 -0.5 1 -1 0.5 0 1 1 0 0.5 -1 1 -0.5 0z'/>",
];

/// Chart aspect ratio
#[derive(Clone, Copy)]
pub enum AspectRatio {
    /// Wide rectangular aspect
    Landscape,
    /// Square aspect
    Square,
    /// Tall rectangular aspect
    Portrait,
}

/// Chart for plotting data to SVG
///
/// Multiple `Plot`s can be rendered in a single Chart, even with unrelated
/// domains and axes.
pub struct Chart<'a, P>
where
    P: IntoPoint,
{
    stand_alone: bool,
    aspect_ratio: AspectRatio,
    titles: Vec<Title<'a>>,
    domain: Domain,
    axes: Vec<Axis<'a>>,
    plots: Vec<Plot<'a, P>>,
    num: u32,
    area: Rect,
}

/// Legend for Chart as an HTML `<div>`
pub struct Legend<'a, P>
where
    P: IntoPoint,
{
    chart: &'a Chart<'a, P>,
}

impl AspectRatio {
    /// Get rectangle
    pub(crate) fn rect(self) -> Rect {
        match self {
            AspectRatio::Landscape => Rect::new(0, 0, 2000, 1500),
            AspectRatio::Square => Rect::new(0, 0, 2000, 2000),
            AspectRatio::Portrait => Rect::new(0, 0, 1500, 2000),
        }
    }
}

impl<P> Default for Chart<'_, P>
where
    P: IntoPoint,
{
    fn default() -> Self {
        Self {
            stand_alone: true,
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            domain: Domain::default(),
            axes: vec![],
            plots: vec![],
            num: 0,
            area: AspectRatio::Landscape.rect().inset(40),
        }
    }
}

impl<'a, P> Chart<'a, P>
where
    P: IntoPoint,
{
    /// Create a new chart
    pub fn new() -> Self {
        Self::default()
    }

    /// Set stand-alone flag
    pub(crate) fn stand_alone(mut self, stand_alone: bool) -> Self {
        self.stand_alone = stand_alone;
        self
    }

    /// Adjust the aspect ratio
    pub fn aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        assert!(self.axes.is_empty());
        assert!(self.plots.is_empty());
        self.aspect_ratio = aspect;
        self.area = self.aspect_ratio.rect().inset(40);
        self
    }

    /// Set the domain
    ///
    /// Panics if called after `axis` or `plot`.
    pub fn domain<D>(mut self, domain: D) -> Self
    where
        D: Into<Domain>,
    {
        assert!(self.axes.is_empty());
        assert!(self.plots.is_empty());
        self.domain = domain.into();
        self
    }

    /// Add a chart title
    ///
    /// Panics if called after `axis` or `plot`.
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title<'a>>,
    {
        assert!(self.axes.is_empty());
        assert!(self.plots.is_empty());
        let mut title = title.into();
        self.area = title.split(self.area);
        self.titles.push(title);
        self
    }

    /// Add an `Axis`
    ///
    /// Panics if called after `plot`.
    pub fn axis(mut self, name: &'a str, edge: Edge) -> Self {
        assert!(self.plots.is_empty());
        let mut axis = self.domain.axis(name, edge);
        self.area = axis.split(self.area);
        self.axes.push(axis);
        self
    }

    /// Add a `Plot`
    pub fn plot(mut self, mut plot: Plot<'a, P>) -> Self {
        plot.num(self.num);
        self.num = if self.num < 10 { self.num + 1 } else { 0 };
        plot.bind_domain(self.domain.bind(self.area));
        self.plots.push(plot);
        self
    }

    /// Render SVG element start
    fn svg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let view_box = ViewBox(self.aspect_ratio.rect());
        write!(f, "<svg")?;
        if self.stand_alone {
            write!(f, " xmlns='http://www.w3.org/2000/svg'")?;
        }
        writeln!(f, " {view_box}>")
    }

    /// Render link to CSS
    fn link(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<link")?;
        write!(f, " xmlns='http://www.w3.org/1999/xhtml'")?;
        write!(f, " type='text/css'")?;
        write!(f, " rel='stylesheet'")?;
        writeln!(f, " href='./css/splot.css' />")
    }

    /// Render defs element
    fn defs(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<defs>")?;
        for i in 0..self.plots.len() {
            write!(f, "<marker id='marker-{i}'")?;
            write!(f, " class='plot-{i}'")?;
            write!(f, " viewBox='-1 -1 2 2'")?;
            writeln!(f, " markerWidth='5' markerHeight='5'>")?;
            writeln!(f, "{}", MARKERS[i % MARKERS.len()])?;
            writeln!(f, "</marker>")?;
        }
        writeln!(f, "<clipPath id='clip-chart'>")?;
        writeln!(f, "{}", self.area)?;
        writeln!(f, "</clipPath>")?;
        writeln!(f, "</defs>")
    }

    /// Render the chart "body"
    fn body(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for title in &self.titles {
            writeln!(f, "{title}")?;
        }
        for axis in &self.axes {
            axis.render(f, self.area)?;
        }
        writeln!(f, "<g clip-path='url(#clip-chart)'>")?;
        for plot in self.plots.iter() {
            writeln!(f, "{plot}")?;
        }
        writeln!(f, "</g>")
    }

    pub fn legend(&self) -> Legend<P> {
        Legend { chart: self }
    }
}

impl<P> fmt::Display for Chart<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.svg(f)?;
        if self.stand_alone {
            self.link(f)?;
        }
        self.defs(f)?;
        self.body(f)?;
        writeln!(f, "</svg>")
    }
}

impl<P> fmt::Display for Legend<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<div class='legend'>")?;
        for (i, plot) in self.chart.plots.iter().enumerate() {
            writeln!(f, "<div>")?;
            writeln!(f, "<svg width='20' height='10' viewBox='0 0 60 30'>")?;
            write!(f, "<path class='plot-{i} legend-line'")?;
            writeln!(f, " d='M0 15h30h30'/>")?;
            writeln!(f, "</svg>")?;
            writeln!(f, "{}", plot.name())?;
            writeln!(f, "</div>")?;
        }
        writeln!(f, "</div>")
    }
}
