use crate::axis::Axis;
use crate::domain::Domain;
use crate::page::{AspectRatio, Edge, Rect};
use crate::plot::{Plot, Plotter};
use crate::point::Point;
use crate::text::{Anchor, Text};
use std::fmt;

pub struct Title {
    text: String,
    anchor: Anchor,
    edge: Edge,
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

pub struct ChartBuilder<'a> {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<Box<dyn Plot + 'a>>,
}

pub struct Chart<'a> {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
    axes: Vec<Box<dyn Axis + 'a>>,
    plots: Vec<Box<dyn Plot + 'a>>,
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

    pub fn with_plot<P: Point>(mut self, plot: Plotter<'a, P>) -> Self {
        self.plots.push(Box::new(plot));
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
        write!(f, "{}", include_str!("defs.svg"))?;
        Ok(())
    }

    fn footer(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "</svg>")
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
        for (plot, num) in self.plots.iter().zip((0..10).cycle()) {
            plot.display(f, num, area)?;
        }
        self.footer(f)?;
        Ok(())
    }
}
