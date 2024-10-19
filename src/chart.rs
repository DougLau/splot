// chart.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::axis::Axis;
use crate::page::{AspectRatio, Edge, Rect};
use crate::plot::Plot;
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

/// Chart for plotting data
///
/// Multiple `Plot`s can be rendered in a single Chart, even with unrelated
/// domains and axes.
pub struct Chart<'a> {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    plots: Vec<&'a (dyn Plot + 'a)>,
    axes: Vec<Axis>,
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

impl<'a> Default for Chart<'a> {
    fn default() -> Self {
        Self {
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            axes: vec![],
            plots: vec![],
        }
    }
}

impl<'a> Chart<'a> {
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

    /// Add an `Axis`
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axes.push(axis);
        self
    }

    /// Add a `Plot`
    pub fn plot(mut self, plot: &'a dyn Plot) -> Self {
        self.plots.push(plot);
        self
    }

    /// Display the chart embedded in HTML
    pub(crate) fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.svg(f, false)?;
        self.defs(f)?;
        self.body(f)
    }

    fn svg(&self, f: &mut fmt::Formatter, stand_alone: bool) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        write!(f, "<svg")?;
        if stand_alone {
            write!(f, " xmlns='http://www.w3.org/2000/svg'")?;
        }
        write!(f, " viewBox='")?;
        writeln!(f, "{} {} {} {}'>", rect.x, rect.y, rect.width, rect.height)
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
        let area = self.area();
        writeln!(f, "<clipPath id='clip-chart'>")?;
        write!(f, "<rect x='{}' y='{}'", area.x, area.y)?;
        writeln!(f, " width='{}' height='{}'/>", area.width, area.height)?;
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
        for (plot, num) in self.plots.iter().zip((0..10).cycle()) {
            plot.display(f, num, area)?;
            plot.display_labels(f, area)?;
        }
        writeln!(f, "</g>")?;
        writeln!(f, "</svg>")
    }

    fn area(&self) -> Rect {
        let mut area = self.aspect_ratio.rect().inset(40);
        for title in &self.titles {
            area.split(title.edge, 100);
        }
        for axis in &self.axes {
            axis.split(&mut area);
        }
        area
    }

    /// Render the legend as an HTML fragment
    pub(crate) fn legend(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl<'a> fmt::Display for Chart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.svg(f, true)?;
        self.link(f)?;
        self.defs(f)?;
        self.body(f)
    }
}
