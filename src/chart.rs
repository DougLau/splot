// chart.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::axis::Axis;
use crate::domain::Domain;
use crate::plot::Plot;
use crate::point::IntoPoint;
use crate::rect::{Edge, Rect, ViewBox};
use crate::text::{Anchor, Text};
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

/// Chart title
pub struct Title {
    text: String,
    anchor: Anchor,
    edge: Edge,
}

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

/// Chart for plotting data
///
/// Multiple `Plot`s can be rendered in a single Chart, even with unrelated
/// domains and axes.
pub struct Chart<'a, P>
where
    P: IntoPoint,
{
    stand_alone: bool,
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    domain: Domain,
    axes: Vec<Axis>,
    plots: Vec<Plot<'a, P>>,
    num: u32,
}

impl<T> From<T> for Title
where
    T: Into<String>,
{
    fn from(text: T) -> Self {
        Title::new(text.into())
    }
}

impl Title {
    /// Create a new title
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Title {
            text: text.into(),
            anchor: Anchor::Middle,
            edge: Edge::Top,
        }
    }

    /// Anchor title text at start
    pub fn at_start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    /// Anchor title text at end
    pub fn at_end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    /// Put title on bottom of chart
    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    /// Put title on left side of chart
    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    /// Put title on right side of chart
    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        let text = Text::new(self.edge)
            .rect(rect)
            .anchor(self.anchor)
            .class_name("title");
        text.display(f)?;
        writeln!(f, "{}", self.text)?;
        text.display_done(f)
    }
}

impl AspectRatio {
    pub(crate) fn rect(self) -> Rect {
        match self {
            AspectRatio::Landscape => Rect::new(0, 0, 2000, 1500),
            AspectRatio::Square => Rect::new(0, 0, 2000, 2000),
            AspectRatio::Portrait => Rect::new(0, 0, 1500, 2000),
        }
    }
}

impl<'a, P> Default for Chart<'a, P>
where
    P: IntoPoint,
{
    fn default() -> Self {
        Self {
            stand_alone: false,
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            domain: Domain::default(),
            axes: vec![],
            plots: vec![],
            num: 0,
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

    /// Make the chart stand-alone
    pub fn stand_alone(mut self, stand_alone: bool) -> Self {
        self.stand_alone = stand_alone;
        self
    }

    /// Adjust the aspect ratio
    pub fn aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    /// Add a chart title
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title>,
    {
        self.titles.push(title.into());
        self
    }

    /// Set the domain
    pub fn domain(mut self, domain: Domain) -> Self {
        assert!(self.axes.is_empty());
        assert!(self.plots.is_empty());
        self.domain = domain;
        // FIXME: bind domain to all plots
        self
    }

    /// Add an `Axis`
    pub fn axis<N>(mut self, name: N, edge: Edge) -> Self
    where
        N: Into<String>,
    {
        assert!(self.plots.is_empty());
        let axis = self.domain.axis(name, edge);
        self.axes.push(axis);
        self
    }

    /// Add a `Plot`
    pub fn plot<T>(mut self, plot: T) -> Self
    where
        T: Into<Plot<'a, P>>,
    {
        let mut plot = plot.into();
        plot.num(self.num);
        self.num = if self.num < 10 { self.num + 1 } else { 0 };
        let area = self.plot_area();
        plot.bind_domain(self.domain.bind(area));
        self.plots.push(plot);
        self
    }

    /// Get area of plots
    fn plot_area(&self) -> Rect {
        let mut area = self.aspect_ratio.rect().inset(40);
        for title in &self.titles {
            area.split(title.edge, 100);
        }
        for axis in &self.axes {
            axis.split(&mut area);
        }
        area
    }

    fn svg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let view_box = ViewBox(self.aspect_ratio.rect());
        write!(f, "<svg")?;
        if self.stand_alone {
            write!(f, " xmlns='http://www.w3.org/2000/svg'")?;
        }
        writeln!(f, " {view_box}>")
    }

    fn link(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<link")?;
        write!(f, " xmlns='http://www.w3.org/1999/xhtml'")?;
        write!(f, " type='text/css'")?;
        write!(f, " rel='stylesheet'")?;
        writeln!(f, " href='./css/splot.css' />")
    }

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
        writeln!(f, "{}", self.plot_area())?;
        writeln!(f, "</clipPath>")?;
        writeln!(f, "</defs>")
    }

    fn body(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut area = self.aspect_ratio.rect().inset(40);
        for title in &self.titles {
            let rect = area.split(title.edge, 100);
            title.display(f, rect)?;
        }
        let mut axis_rects = vec![];
        for axis in &self.axes {
            axis_rects.push(axis.split(&mut area));
        }
        for axis in self.axes.iter() {
            axis.display_grid(f, area)?;
        }
        for (axis, rect) in self.axes.iter().zip(axis_rects) {
            axis.display(f, rect, area)?;
        }
        writeln!(f, "<g clip-path='url(#clip-chart)'>")?;
        for plot in self.plots.iter() {
            writeln!(f, "{plot}")?;
        }
        writeln!(f, "</g>")
    }

    /// Render the legend as an HTML fragment
    fn legend(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<div class='legend'>")?;
        for (i, plot) in self.plots.iter().enumerate() {
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

impl<'a, P> fmt::Display for Chart<'a, P>
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
        writeln!(f, "</svg>")?;
        self.legend(f)
    }
}
