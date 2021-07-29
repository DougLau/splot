// axis.rs
//
// Copyright (c) 2021  Douglas P Lau
//
//! Axis for charts
//!
use crate::page::{Edge, Rect};
use crate::text::{Anchor, Label, Text, Tick};
use std::fmt;

/// Private module for sealed Axis trait
mod sealed {
    use crate::page::Rect;
    use std::fmt;

    pub trait Axis {
        fn split(&self, area: &mut Rect) -> Rect;
        fn display(
            &self,
            f: &mut fmt::Formatter,
            rect: Rect,
            area: Rect,
        ) -> fmt::Result;
        fn display_grid(
            &self,
            f: &mut fmt::Formatter,
            area: Rect,
        ) -> fmt::Result;
    }
}

/// Axis for drawing labels on a `Chart`
///
/// This trait is *sealed* to hide details.  There are two implementors:
/// - `axis::Horizontal`
/// - `axis::Vertical`
pub trait Axis: sealed::Axis {}

/// Horizontal `X` axis
#[derive(Debug, PartialEq)]
pub struct Horizontal {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

/// Vertical `Y` axis
#[derive(Debug, PartialEq)]
pub struct Vertical {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

impl sealed::Axis for Horizontal {
    fn split(&self, area: &mut Rect) -> Rect {
        area.split(self.edge, self.space())
    }

    fn display(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        rect.intersect_horiz(&area);
        if let Some(name) = &self.name {
            let r = rect.split(self.edge, self.space() / 2);
            let text =
                Text::new(self.edge).with_rect(r).with_class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    fn display_grid(&self, f: &mut fmt::Formatter, area: Rect) -> fmt::Result {
        write!(f, "<path class='grid-x' d='")?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, area, 0);
            write!(f, "M{} {}v{}", x, area.y, area.height)?;
        }
        writeln!(f, "'/>")
    }
}

impl Axis for Horizontal {}

impl Horizontal {
    /// Create a new horizontal axis
    pub(crate) fn new(ticks: Vec<Tick>) -> Self {
        Self {
            edge: Edge::Bottom,
            ticks,
            name: None,
            label: Label::new(),
        }
    }

    /// Set the name of the axis
    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Attach to the top of a `Chart`
    ///
    /// By default, a `Horizontal` axis is attached to the bottom of a `Chart`.
    pub fn on_top(mut self) -> Self {
        self.edge = Edge::Top;
        self
    }

    fn space(&self) -> u16 {
        match self.name {
            Some(_) => 160,
            None => 80,
        }
    }

    fn display_tick_lines(
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
        write!(f, "<path class='axis-line' d='M{} {}h{}", x, y, rect.width)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {}v{}", x, y, height)?;
        }
        writeln!(f, "'/>")
    }

    fn display_tick_labels(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let text = Text::new(Edge::Top).with_class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}

impl sealed::Axis for Vertical {
    fn split(&self, area: &mut Rect) -> Rect {
        area.split(self.edge, self.space())
    }

    fn display(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        rect.intersect_vert(&area);
        if let Some(name) = &self.name {
            let r = rect.split(self.edge, self.space() / 2);
            let text =
                Text::new(self.edge).with_rect(r).with_class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }

    fn display_grid(&self, f: &mut fmt::Formatter, area: Rect) -> fmt::Result {
        write!(f, "<path class='grid-y' d='")?;
        for tick in self.ticks.iter() {
            let y = tick.y(self.edge, area, 0);
            write!(f, "M{} {}h{}", area.x, y, area.width)?;
        }
        writeln!(f, "'/>")
    }
}

impl Axis for Vertical {}

impl Vertical {
    /// Create a new vertical axis
    pub(crate) fn new(ticks: Vec<Tick>) -> Self {
        Self {
            edge: Edge::Left,
            ticks,
            name: None,
            label: Label::new(),
        }
    }

    /// Set the name of the axis
    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Attach to the right side of a `Chart`
    ///
    /// By default, a `Vertical` axis is attached to the left side of a `Chart`.
    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn space(&self) -> u16 {
        match self.name {
            Some(_) => 160,
            None => 80,
        }
    }

    fn display_tick_lines(
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
        write!(f, " d='M{} {}v{}", x, rect.y, rect.height)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {}h{}", x, y, width)?;
        }
        writeln!(f, "'/>")
    }

    fn display_tick_labels(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let anchor = match self.edge {
            Edge::Left => Anchor::End,
            Edge::Right => Anchor::Start,
            _ => unreachable!(),
        };
        let text = Text::new(Edge::Top)
            .with_anchor(anchor)
            .with_class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}
