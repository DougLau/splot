// axis.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
//! Axis for charts
//!
use crate::rect::{Edge, Rect};
use crate::text::{Anchor, Label, Text, Tick};
use hatmil::{Html, PathDef, Svg};

/// Axis for drawing labels on a `Chart`
#[derive(Debug, PartialEq)]
pub struct Axis<'a> {
    edge: Edge,
    ticks: Vec<Tick>,
    name: &'a str,
    label: Label,
    rect: Rect,
}

impl<'a> Axis<'a> {
    /// Create a new axis
    pub fn new(name: &'a str, edge: Edge, ticks: Vec<Tick>) -> Self {
        Self {
            edge,
            ticks,
            name,
            label: Label::new(),
            rect: Rect::default(),
        }
    }

    /// Split axis area from rectangle
    pub fn split(&mut self, mut area: Rect) -> Rect {
        (area, self.rect) = area.split(self.edge, self.space());
        area
    }

    /// Get the space required
    fn space(&self) -> u16 {
        if self.name.is_empty() { 80 } else { 160 }
    }

    /// Display the axis
    pub fn display(&self, area: Rect, html: &mut Html) {
        match self.edge {
            Edge::Bottom | Edge::Top => {
                self.display_grid_horizontal(area, html);
                self.display_horizontal(area, html);
            }
            Edge::Left | Edge::Right => {
                self.display_grid_vertical(area, html);
                self.display_vertical(area, html)
            }
        }
    }

    /// Display horizontal grid lines
    fn display_grid_horizontal(&self, area: Rect, html: &mut Html) {
        let mut d = PathDef::new();
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, area, 0);
            d.move_to((x, area.y)).line((x, area.bottom()));
        }
        let path = Svg::new(html).path();
        path.class("grid-x").d::<String>(d.into()).end();
    }

    /// Display horizontal axis
    fn display_horizontal(&self, area: Rect, html: &mut Html) {
        let mut rect = self.rect;
        rect.intersect_horiz(&area);
        if !self.name.is_empty() {
            let r;
            (rect, r) = rect.split(self.edge, self.space() / 2);
            let text = Text::new(self.edge).rect(r).class_name("axis");
            text.display(html);
            html.text(self.name).end();
        }
        self.display_tick_lines(rect, html);
        self.display_tick_labels(rect, html);
    }

    /// Display vertical grid lines
    fn display_grid_vertical(&self, area: Rect, html: &mut Html) {
        let mut d = PathDef::new();
        for tick in self.ticks.iter() {
            let y = tick.y(self.edge, area, 0);
            d.move_to((area.x, y)).line((area.right(), y));
        }
        let path = Svg::new(html).path();
        path.class("grid-y").d::<String>(d.into()).end();
    }

    /// Display vertical axis
    fn display_vertical(&self, area: Rect, html: &mut Html) {
        let mut rect = self.rect;
        rect.intersect_vert(&area);
        if !&self.name.is_empty() {
            let r;
            (rect, r) = rect.split(self.edge, self.space() / 2);
            let text = Text::new(self.edge).rect(r).class_name("axis");
            text.display(html);
            html.text(self.name).end();
        }
        self.display_tick_lines(rect, html);
        self.display_tick_labels(rect, html);
    }

    /// Display tick lines
    fn display_tick_lines(&self, rect: Rect, html: &mut Html) {
        match self.edge {
            Edge::Bottom | Edge::Top => {
                self.display_tick_lines_horizontal(rect, html)
            }
            Edge::Left | Edge::Right => {
                self.display_tick_lines_vertical(rect, html)
            }
        }
    }

    /// Display horizontal tick lines
    fn display_tick_lines_horizontal(&self, rect: Rect, html: &mut Html) {
        let x = rect.x;
        let (y, height) = match self.edge {
            Edge::Top => (rect.bottom(), Tick::LEN),
            Edge::Bottom => (rect.y, -Tick::LEN),
            _ => unreachable!(),
        };
        let mut d = PathDef::new();
        d.move_to((x, y)).line((rect.right(), y));
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let y0 = y.min(y + height);
            let y1 = y.max(y + height);
            d.move_to((x, y0)).line((x, y1));
        }
        let path = Svg::new(html).path();
        path.class("axis-line").d::<String>(d.into()).end();
    }

    /// Display vertical tick lines
    fn display_tick_lines_vertical(&self, rect: Rect, html: &mut Html) {
        let (x, width) = match self.edge {
            Edge::Left => (rect.right(), Tick::LEN),
            Edge::Right => (rect.x, -Tick::LEN),
            _ => unreachable!(),
        };
        let mut d = PathDef::new();
        d.move_to((x, rect.y)).line((x, rect.bottom()));
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let x0 = x.min(x + width);
            let x1 = x.max(x + width);
            d.move_to((x0, y)).line((x1, y));
        }
        let path = Svg::new(html).path();
        path.class("axis-line").d::<String>(d.into()).end();
    }

    /// Display tick labels
    fn display_tick_labels(&self, rect: Rect, html: &mut Html) {
        match self.edge {
            Edge::Bottom | Edge::Top => {
                self.display_tick_labels_horizontal(rect, html);
            }
            Edge::Left | Edge::Right => {
                self.display_tick_labels_vertical(rect, html);
            }
        }
    }

    /// Display horizontal tick labels
    fn display_tick_labels_horizontal(&self, rect: Rect, html: &mut Html) {
        let text = Text::new(Edge::Top).class_name("tick");
        text.display(html);
        for tick in &self.ticks {
            let tspan = tick.tspan(self.edge, rect);
            tspan.display(html);
        }
        html.end();
    }

    /// Display vertical tick labels
    fn display_tick_labels_vertical(&self, rect: Rect, html: &mut Html) {
        let anchor = match self.edge {
            Edge::Left => Anchor::End,
            Edge::Right => Anchor::Start,
            _ => unreachable!(),
        };
        let text = Text::new(Edge::Top).anchor(anchor).class_name("tick");
        text.display(html);
        for tick in &self.ticks {
            let tspan = tick.tspan(self.edge, rect);
            tspan.display(html);
        }
        html.end();
    }
}
