// axis.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
//! Axis for charts
//!
use crate::rect::{Edge, Rect};
use crate::text::{Anchor, Tick};
use crate::title::Title;
use hatmil::svg::{G, Path};

/// Axis for drawing labels on a `Chart`
#[derive(Debug, PartialEq)]
pub struct Axis<'a> {
    /// Chart edge
    edge: Edge,
    /// Ticks to display
    ticks: Vec<Tick>,
    /// Axis name
    name: &'a str,
    /// Rectangle containing axis name and ticks
    rect: Rect,
}

impl<'a> Axis<'a> {
    /// Create a new axis
    pub fn new(name: &'a str, edge: Edge, ticks: Vec<Tick>) -> Self {
        Self {
            edge,
            ticks,
            name,
            rect: Rect::default(),
        }
    }

    /// Split axis area from rectangle
    pub fn split(&mut self, area: &mut Rect) {
        self.rect = area.split(self.edge, self.space());
    }

    /// Get the space required
    fn space(&self) -> u16 {
        if self.name.is_empty() { 80 } else { 160 }
    }

    /// Display the axis
    pub fn display<'p>(&self, area: Rect, g: &'p mut G<'p>) {
        match self.edge {
            Edge::Top | Edge::Bottom => self.display_x(area, g),
            Edge::Left | Edge::Right => self.display_y(area, g),
        }
    }

    /// Display X-axis lines and labels
    fn display_x<'p>(&self, area: Rect, g: &'p mut G<'p>) {
        let mut rect = self.rect;
        // Shrink axis width to match plot area
        rect.intersect_horiz(&area);
        if !self.name.is_empty() {
            let mut title = Title::from(self.name).on_edge(self.edge);
            title.split(&mut rect, self.space() / 2);
            let mut text = g.text();
            text.class("axis");
            if let Some(transform) = title.transform() {
                text.transform(transform);
            }
            text.text_anchor(Anchor::Middle);
            text.cdata(self.name);
            text.close();
        }
        g.path().class("grid-x").d(self.build_x_ticks(area)).close();
        g.path()
            .class("axis-line")
            .d(self.build_x_tick_lines(rect))
            .close();
        self.display_tick_labels(rect, g);
    }

    /// Build X ticks path
    fn build_x_ticks(&self, area: Rect) -> String {
        let mut d = Path::def_builder();
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, area, 0);
            d.move_to((x, area.y)).line((x, area.bottom()));
        }
        String::from(d)
    }

    /// Build X tick lines path
    fn build_x_tick_lines(&self, rect: Rect) -> String {
        let x = rect.x;
        let (y, height) = match self.edge {
            Edge::Top => (rect.bottom(), Tick::LEN),
            Edge::Bottom => (rect.y, -Tick::LEN),
            _ => unreachable!(),
        };
        let mut d = Path::def_builder();
        d.move_to((x, y)).line((rect.right(), y));
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let y0 = y.min(y + height);
            let y1 = y.max(y + height);
            d.move_to((x, y0)).line((x, y1));
        }
        String::from(d)
    }

    /// Display Y-axis lines and labels
    fn display_y<'p>(&self, area: Rect, g: &'p mut G<'p>) {
        let mut rect = self.rect;
        // Shrink axis height to match plot area
        rect.intersect_vert(&area);
        if !&self.name.is_empty() {
            let mut title = Title::from(self.name).on_edge(self.edge);
            title.split(&mut rect, self.space() / 2);
            let mut text = g.text();
            text.class("axis");
            if let Some(transform) = title.transform() {
                text.transform(transform);
            }
            text.text_anchor(Anchor::Middle);
            text.cdata(self.name);
            text.close();
        }
        g.path().class("grid-y").d(self.build_y_ticks(area)).close();
        g.path()
            .class("axis-line")
            .d(self.build_y_tick_lines(rect))
            .close();
        self.display_tick_labels(rect, g);
    }

    /// Build Y ticks path
    fn build_y_ticks(&self, area: Rect) -> String {
        let mut d = Path::def_builder();
        for tick in self.ticks.iter() {
            let y = tick.y(self.edge, area, 0);
            d.move_to((area.x, y)).line((area.right(), y));
        }
        String::from(d)
    }

    /// Build Y tick lines
    fn build_y_tick_lines(&self, rect: Rect) -> String {
        let (x, width) = match self.edge {
            Edge::Left => (rect.right(), Tick::LEN),
            Edge::Right => (rect.x, -Tick::LEN),
            _ => unreachable!(),
        };
        let mut d = Path::def_builder();
        d.move_to((x, rect.y)).line((x, rect.bottom()));
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let x0 = x.min(x + width);
            let x1 = x.max(x + width);
            d.move_to((x0, y)).line((x1, y));
        }
        String::from(d)
    }

    /// Display tick labels
    fn display_tick_labels<'p>(&self, rect: Rect, g: &'p mut G<'p>) {
        let anchor = match self.edge {
            Edge::Top | Edge::Bottom => Anchor::Middle,
            Edge::Left => Anchor::End,
            Edge::Right => Anchor::Start,
        };
        let mut text = g.text();
        text.class("tick");
        text.text_anchor(anchor);
        for tick in &self.ticks {
            let x = tick.x(self.edge, rect, Tick::HLEN);
            let y = tick.y(self.edge, rect, Tick::VLEN);
            text.tspan()
                .x(x)
                .y(y)
                .dy("0.33em")
                .cdata(tick.text())
                .close();
        }
        text.close(); // text
        g.close(); // g
    }
}
