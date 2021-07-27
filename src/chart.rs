// chart.rs
//
// Copyright (c) 2021  Douglas P Lau
//
use crate::axis::Axis;
use crate::page::{AspectRatio, Edge, Rect};
use crate::plot::Plot;
use crate::text::{Anchor, Text};
use std::fmt;

/// Marker shapes
const MARKERS: &[&str] = &[
    "<circle r='1' />",
    "<rect x='-1' y='-1' width='2' height='2' />",
    "<path d='M0 -1 1 1 -1 1z' />",
    "<path d='M1 0 -1 1 -1 -1z' />",
    "<path d='M0 1 -1 -1 1 -1z' />",
    "<path d='M-1 0 1 -1 1 1z' />",
    "<path d='M0 -1 1 0 0 1 -1 0z' />",
    "<path d='M-1 -1 0 -0.5 1 -1 0.5 0 1 1 0 0.5 -1 1 -0.5 0z' />",
];

/// Chart title
pub struct Title {
    text: String,
    anchor: Anchor,
    edge: Edge,
}

/// Builder for charts
pub struct ChartBuilder<'a> {
    stand_alone: bool,
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<&'a (dyn Plot + 'a)>,
}

/// Chart for plotting data
pub struct Chart<'a> {
    stand_alone: bool,
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<&'a (dyn Plot + 'a)>,
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
    pub(crate) fn new_with_edge<T>(text: T, edge: Edge) -> Self
    where
        T: Into<String>,
    {
        Title {
            text: text.into(),
            anchor: Anchor::Middle,
            edge,
        }
    }

    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new_with_edge(text, Edge::Top)
    }

    pub fn at_start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    pub fn at_end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        let text = Text::new(self.edge, self.anchor)
            .rect(rect)
            .class_name("title");
        text.display(f)?;
        writeln!(f, "{}", self.text)?;
        text.display_done(f)
    }
}

impl<'a> Default for ChartBuilder<'a> {
    fn default() -> Self {
        Self {
            stand_alone: false,
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            axes: vec![],
            plots: vec![],
        }
    }
}

impl<'a> ChartBuilder<'a> {
    pub fn stand_alone(mut self) -> Self {
        self.stand_alone = true;
        self
    }

    pub fn aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    pub fn with_title<T>(mut self, title: T) -> Self
    where
        T: Into<Title>,
    {
        self.titles.push(title.into());
        self
    }

    pub fn with_axis<A: Axis + 'a>(mut self, axis: A) -> Self {
        self.axes.push(Box::new(axis));
        self
    }

    pub fn with_plot(mut self, plot: &'a dyn Plot) -> Self {
        self.plots.push(plot);
        self
    }

    pub fn build(self) -> Chart<'a> {
        Chart {
            stand_alone: self.stand_alone,
            aspect_ratio: self.aspect_ratio,
            titles: self.titles,
            axes: self.axes,
            plots: self.plots,
        }
    }
}

impl<'a> Chart<'a> {
    pub fn builder() -> ChartBuilder<'a> {
        ChartBuilder::default()
    }

    fn svg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        write!(f, "<svg")?;
        if self.stand_alone {
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
            write!(f, "<marker id='marker-{}'", i)?;
            write!(f, " class='plot-{}'", i)?;
            write!(f, " viewBox='-1 -1 2 2'")?;
            writeln!(f, " markerWidth='5' markerHeight='5'>")?;
            writeln!(f, "  {}", MARKERS[i % MARKERS.len()])?;
            writeln!(f, "</marker>")?;
        }
        let area = self.area();
        writeln!(f, "<clipPath id='clip-chart'>")?;
        write!(f, "  <rect x='{}' y='{}'", area.x, area.y)?;
        writeln!(f, " width='{}' height='{}' />", area.width, area.height)?;
        writeln!(f, "</clipPath>")?;
        writeln!(f, "</defs>")
    }

    fn footer(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    pub(crate) fn legend(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<div class='legend'>")?;
        for (i, plot) in self.plots.iter().enumerate() {
            writeln!(f, "<div>")?;
            writeln!(f, "<svg width='20' height='10' viewBox='0 0 60 30'>")?;
            write!(f, "<g class='plot-{}'>", i)?;
            write!(f, "<path class='legend-line' d='M0 15h30h30'/>")?;
            writeln!(f, "</g>")?;
            writeln!(f, "</svg>")?;
            writeln!(f, "{}", plot.name())?;
            writeln!(f, "</div>")?;
        }
        writeln!(f, "</div>")
    }
}

impl<'a> fmt::Display for Chart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.svg(f)?;
        if self.stand_alone {
            self.link(f)?;
        }
        self.defs(f)?;
        let mut area = self.aspect_ratio.rect().inset(40);
        for title in &self.titles {
            let rect = area.split(title.edge, 100);
            title.display(f, rect)?;
        }
        let mut rects = vec![];
        for axis in &self.axes {
            rects.push(axis.split(&mut area));
        }
        for (axis, rect) in self.axes.iter().zip(rects) {
            axis.display(f, rect, area)?;
        }
        writeln!(f, "<g clip-path='url(#clip-chart)'>")?;
        for (plot, num) in self.plots.iter().zip((0..10).cycle()) {
            writeln!(f, "<g class='plot-{}'>", num)?;
            plot.display(f, area)?;
            writeln!(f, "</g>")?;
        }
        writeln!(f, "</g>")?;
        self.footer(f)?;
        Ok(())
    }
}
