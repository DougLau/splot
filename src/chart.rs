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
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<&'a (dyn Plot + 'a)>,
}

/// Chart for plotting data
pub struct Chart<'a> {
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
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
            axes: vec![],
            plots: vec![],
        }
    }
}

impl<'a> ChartBuilder<'a> {
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

    fn header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        writeln!(
            f,
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            rect.x, rect.y, rect.width, rect.height,
        )?;
        writeln!(f, "<style>")?;
        write!(f, "{}", include_str!("splot.css"))?;
        writeln!(f, "</style>")?;
        self.defs(f)
    }

    fn defs(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<defs>")?;
        for i in 0..self.plots.len() {
            writeln!(f, "<marker id='marker-{}' viewBox='-1 -1 2 2' markerWidth='5' markerHeight='5'>", i)?;
            writeln!(f, "  {}", MARKERS[i % MARKERS.len()])?;
            writeln!(f, "</marker>")?;
        }
        let area = self.area();
        writeln!(f, "<clipPath id='clip-chart'>")?;
        writeln!(
            f,
            "<rect x='{}' y='{}' width='{}' height='{}' />",
            area.x, area.y, area.width, area.height
        )?;
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
}

impl<'a> fmt::Display for Chart<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.header(f)?;
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
            plot.display(f, num, area)?;
        }
        writeln!(f, "</g>")?;
        self.footer(f)?;
        Ok(())
    }
}
