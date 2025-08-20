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
use hatmil::{Html, Svg};
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
    stand_alone: bool,
    aspect_ratio: AspectRatio,
    titles: Vec<Title<'a>>,
    domain: Domain,
    axes: Vec<Axis<'a>>,
    plots: Vec<Plot<'a, P>>,
    num: u32,
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
        *plot.num_mut() = self.num;
        self.num = if self.num < 10 { self.num + 1 } else { 0 };
        *plot.domain_mut() = self.domain.bind(self.area);
        self.plots.push(plot);
        self
    }

    /// Render SVG element start
    fn svg(&self, html: &mut Html) {
        let mut svg = html.svg();
        if self.stand_alone {
            svg = svg.attr("xmlns", "http://www.w3.org/2000/svg");
        }
        svg.view_box(self.aspect_ratio.rect().view_box());
    }

    /// Render link to CSS
    fn link(&self, html: &mut Html) {
        let svg = Svg::new(html);
        svg.link()
            .attr("xmlns", "http://www.w3.org/1999/xhtml")
            .r#type("text/css")
            .rel("stylesheet")
            .href("./css/splot.css")
            .end();
    }

    /// Render defs element
    fn defs(&self, html: &mut Html) {
        Svg::new(html).defs();
        for i in 0..self.plots.len() {
            let marker = Svg::new(html)
                .marker()
                .id(format!("marker-{i}"))
                .class(format!("plot-{i}"))
                .view_box("-1 -1 2 2")
                .marker_width("5")
                .marker_height("5");
            let path = marker.path();
            path.d(MARKERS[i % MARKERS.len()]);
            html.end().end(); // path, marker
        }
        let cp = Svg::new(html).clip_path();
        cp.id("clip-chart");
        self.area.display(html);
        html.end().end(); // clipPath, defs
    }

    /// Render the chart "body"
    fn body(&self, html: &mut Html) {
        for title in &self.titles {
            title.display(html);
        }
        for axis in &self.axes {
            axis.display(self.area, html);
        }
        let g = Svg::new(html).g();
        g.attr("clip-path", "url(#clip-chart)");
        for plot in self.plots.iter() {
            plot.display(html);
        }
        html.end(); // g
    }

    /// Display char as HTML
    pub fn display(&self, html: &mut Html) {
        self.svg(html);
        if self.stand_alone {
            self.link(html);
        }
        self.defs(html);
        self.body(html);
        html.end(); // svg
    }

    /// Display legend as HTML `<div>`
    pub fn legend(&self, html: &mut Html) {
        html.div().class("legend");
        for (i, plot) in self.plots.iter().enumerate() {
            html.div();
            let svg = html.svg().width("20").height("10").view_box("0 0 60 30");
            svg.path()
                .class(format!("plot-{i} legend-line"))
                .d("M0 15h30h30")
                .end();
            html.end(); // svg
            html.text(plot.name());
            html.end(); // div
        }
        html.end(); // div
    }
}

impl<P> fmt::Display for Chart<'_, P>
where
    P: IntoPoint,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut html = Html::new_xml_compatible();
        self.display(&mut html);
        writeln!(f, "{}", String::from(html))
    }
}
