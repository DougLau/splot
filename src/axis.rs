// axis.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
//! Axis for charts
//!
use crate::rect::{Edge, Rect};
use crate::text::{Anchor, Label, Text, Tick};
use std::fmt;

/// Axis for drawing labels on a `Chart`
#[derive(Debug, PartialEq)]
pub struct Axis {
    edge: Edge,
    ticks: Vec<Tick>,
    name: String,
    label: Label,
}

impl Axis {
    /// Create a new axis
    pub fn new<N>(name: N, edge: Edge, ticks: Vec<Tick>) -> Self
    where
        N: Into<String>,
    {
        Self {
            edge,
            ticks,
            name: name.into(),
            label: Label::new(),
        }
    }

    /// Split axis area from rectangle
    pub fn split(&self, area: &mut Rect) -> Rect {
        area.split(self.edge, self.space())
    }

    /// Get the space required
    fn space(&self) -> u16 {
        if self.name.is_empty() {
            80
        } else {
            160
        }
    }

    /// Render the axis
    pub fn display(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Bottom | Edge::Top => self.display_horizontal(f, rect, area),
            Edge::Left | Edge::Right => self.display_vertical(f, rect, area),
        }
    }

    /// Render horizontal axis
    fn display_horizontal(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        rect.intersect_horiz(&area);
        if !self.name.is_empty() {
            let r = rect.split(self.edge, self.space() / 2);
            let text = Text::new(self.edge).rect(r).class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", &self.name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    /// Render vertical axis
    fn display_vertical(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        rect.intersect_vert(&area);
        if !&self.name.is_empty() {
            let r = rect.split(self.edge, self.space() / 2);
            let text = Text::new(self.edge).rect(r).class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", &self.name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    /// Render grid lines
    pub fn display_grid(
        &self,
        f: &mut fmt::Formatter,
        area: Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Bottom | Edge::Top => self.display_grid_horizontal(f, area),
            Edge::Left | Edge::Right => self.display_grid_vertical(f, area),
        }
    }

    /// Render horizontal grid lines
    fn display_grid_horizontal(
        &self,
        f: &mut fmt::Formatter,
        area: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='grid-x' d='")?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, area, 0);
            write!(f, "M{x} {}v{}", area.y, area.height)?;
        }
        writeln!(f, "'/>")
    }

    /// Render vertical grid lines
    fn display_grid_vertical(
        &self,
        f: &mut fmt::Formatter,
        area: Rect,
    ) -> fmt::Result {
        write!(f, "<path class='grid-y' d='")?;
        for tick in self.ticks.iter() {
            let y = tick.y(self.edge, area, 0);
            write!(f, "M{} {y}h{}", area.x, area.width)?;
        }
        writeln!(f, "'/>")
    }

    /// Render tick lines
    fn display_tick_lines(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Bottom | Edge::Top => {
                self.display_tick_lines_horizontal(f, rect)
            }
            Edge::Left | Edge::Right => {
                self.display_tick_lines_vertical(f, rect)
            }
        }
    }

    /// Render horizontal tick lines
    fn display_tick_lines_horizontal(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let x = rect.x;
        let (y, height) = match self.edge {
            Edge::Top => (rect.bottom(), Tick::LEN),
            Edge::Bottom => (rect.y, -Tick::LEN),
            _ => unreachable!(),
        };
        write!(f, "<path class='axis-line' d='M{x} {y}h{}", rect.width)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let y0 = y.min(y + height);
            let h = y.max(y + height) - y0;
            write!(f, "M{x} {y0}v{h}")?;
        }
        writeln!(f, "'/>")
    }

    /// Render vertical tick lines
    fn display_tick_lines_vertical(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let (x, width) = match self.edge {
            Edge::Left => (rect.right(), Tick::LEN),
            Edge::Right => (rect.x, -Tick::LEN),
            _ => unreachable!(),
        };
        write!(f, "<path class='axis-line'")?;
        write!(f, " d='M{x} {}v{}", rect.y, rect.height)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            let x0 = x.min(x + width);
            let w = x.max(x + width) - x0;
            write!(f, " M{x0} {y}h{w}")?;
        }
        writeln!(f, "'/>")
    }

    /// Render tick labels
    fn display_tick_labels(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Bottom | Edge::Top => {
                self.display_tick_labels_horizontal(f, rect)
            }
            Edge::Left | Edge::Right => {
                self.display_tick_labels_vertical(f, rect)
            }
        }
    }

    /// Render horizontal tick labels
    fn display_tick_labels_horizontal(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let text = Text::new(Edge::Top).class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            let tspan = tick.tspan(self.edge, rect);
            write!(f, "{tspan}")?;
        }
        text.display_done(f)
    }

    /// Render vertical tick labels
    fn display_tick_labels_vertical(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let anchor = match self.edge {
            Edge::Left => Anchor::End,
            Edge::Right => Anchor::Start,
            _ => unreachable!(),
        };
        let text = Text::new(Edge::Top).anchor(anchor).class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            let tspan = tick.tspan(self.edge, rect);
            write!(f, "{tspan}")?;
        }
        text.display_done(f)
    }
}
