// chart.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::axis::Axis;
use crate::domain::Domain;
use crate::plot::Plot;
use crate::point::IntoPoint;
use crate::rect::{Edge, Rect};
use crate::title::Title;
use hatmil::{
    Page,
    html::Div,
    svg::{Defs, G, Svg},
};
use std::fmt;

/// Marker shapes
const MARKERS: &[&str] = &[
    "M0 1A1 1 0 0 1 0 -1 A1 1 0 0 1 0 1",
    "M-1 -1h2v2h-2z",
    "M0 -1 1 1 -1 1z",
    "M1 0 -1 1 -1 -1z",
    "M0 1 -1 -1 1 -1z",
    "M-1 0 1 -1 1 1z",
    "M0 -1 1 0 0 1 -1 0z",
    "M-1 -1 0 -0.5 1 -1 0.5 0 1 1 0 0.5 -1 1 -0.5 0z",
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
    /// Stand-alone SVG
    stand_alone: bool,
    /// Aspect ratio
    aspect_ratio: AspectRatio,
    /// Chart titles
    titles: Vec<Title<'a>>,
    /// Plotting domain
    domain: Domain,
    /// Axes (vertical/horizontal)
    axes: Vec<Axis<'a>>,
    /// Chart plots
    plots: Vec<Plot<'a, P>>,
    /// Chart number
    num: u32,
    /// Total chart area rectangle
    area: Rect,
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
        title.split(&mut self.area, 100);
        self.titles.push(title);
        self
    }

    /// Add an `Axis`
    ///
    /// Panics if called after `plot`.
    pub fn axis(mut self, name: &'a str, edge: Edge) -> Self {
        assert!(self.plots.is_empty());
        let mut axis = self.domain.axis(name, edge);
        axis.split(&mut self.area);
        self.axes.push(axis);
        self
    }

    /// Add a `Plot`
    pub fn plot(mut self, mut plot: Plot<'a, P>) -> Self {
        *plot.num_mut() = self.num;
        self.num = if self.num < 10 { self.num + 1 } else { 0 };
        *plot.domain_mut() = self.domain.bind(self.area);
        self.plots.push(plot);
        self
    }

    /// Render defs element
    fn defs<'p>(&self, defs: &'p mut Defs<'p>) {
        for i in 0..self.plots.len() {
            let mut marker = defs.marker();
            marker
                .id(format!("marker-{i}"))
                .class(format!("plot-{i}"))
                .view_box("-1 -1 2 2")
                .marker_width("5")
                .marker_height("5");
            let mut path = marker.path();
            path.d(MARKERS[i % MARKERS.len()]).close(); // path
            marker.close(); // marker
        }
        let mut cp = defs.clip_path();
        cp.id("clip-plot")
            .rect()
            .x(self.area.x)
            .y(self.area.y)
            .width(self.area.width)
            .height(self.area.height)
            .close() // rect
            .close(); // clipPath
        defs.close(); // defs
    }

    /// Render the chart "body"
    fn body<'p>(&self, g: &'p mut G<'p>) {
        for title in &self.titles {
            let mut text = g.text();
            text.class("title");
            if let Some(transform) = title.transform() {
                text.transform(transform);
            }
            text.text_anchor(title.anchor());
            text.cdata(title.text());
            text.close();
        }
        for axis in &self.axes {
            axis.display(self.area, &mut g.g());
        }
        for plot in self.plots.iter() {
            plot.display(&mut g.g());
        }
        g.close(); // body
    }

    /// Display chart as SVG
    pub fn display<'p>(&self, svg: &'p mut Svg<'p>) {
        svg.view_box(self.aspect_ratio.rect().view_box());
        if self.stand_alone {
            svg.link()
                .xmlns("http://www.w3.org/1999/xhtml")
                .r#type("text/css")
                .rel("stylesheet")
                .href("./css/splot.css")
                .close();
        }
        self.defs(&mut svg.defs());
        self.body(&mut svg.g());
        svg.close(); // svg
    }

    /// Display legend as HTML `<div>`
    pub fn legend<'p>(&self, div: &'p mut Div<'p>) {
        div.class("legend");
        for (i, plot) in self.plots.iter().enumerate() {
            let mut div2 = div.div();
            div2.svg()
                .width("20")
                .height("10")
                .view_box("0 0 60 30")
                .path()
                .class(format!("plot-{i} legend-line"))
                .d("M0 15h30h30")
                .close() // path
                .close(); // svg
            div2.cdata(plot.name()).close(); // div2
        }
        div.close(); // div
    }
}

impl<P> fmt::Display for Chart<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut page = Page::default();
        let mut svg = page.frag::<Svg>();
        self.display(&mut svg);
        writeln!(f, "{}", String::from(page))
    }
}
