// axis.rs
//
// Copyright (c) 2021  Douglas P Lau
//
//! Axis rendering
use crate::page::{Edge, Rect};
use crate::private::SealedAxis;
use crate::text::{Anchor, Label, Text, Tick};
use std::fmt;

/// Axis renderer
pub trait Axis: SealedAxis {}

/// Horizontal (X) axis
#[derive(Debug, PartialEq)]
pub struct Horizontal {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

/// Vertical (Y) axis
#[derive(Debug, PartialEq)]
pub struct Vertical {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

impl SealedAxis for Horizontal {
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
            let text = Text::new(self.edge, Anchor::Middle)
                .rect(r)
                .class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }
}

impl Axis for Horizontal {}

impl Horizontal {
    pub(crate) fn new(ticks: Vec<Tick>) -> Self {
        Self {
            edge: Edge::Bottom,
            ticks,
            name: None,
            label: Label::new(),
        }
    }

    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

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
        write!(f, "<path class='axis-line' d='M{} {} h{}", x, y, rect.width)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {} v{}", x, y, height)?;
        }
        writeln!(f, "' />")
    }

    fn display_tick_labels(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let text = Text::new(Edge::Top, Anchor::Middle).class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}

impl SealedAxis for Vertical {
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
            let text = Text::new(self.edge, Anchor::Middle)
                .rect(r)
                .class_name("axis");
            text.display(f)?;
            writeln!(f, "{}", name)?;
            text.display_done(f)?;
        }
        self.display_tick_lines(f, rect)?;
        self.display_tick_labels(f, rect)
    }
}

impl Axis for Vertical {}

impl Vertical {
    pub(crate) fn new(ticks: Vec<Tick>) -> Self {
        Self {
            edge: Edge::Left,
            ticks,
            name: None,
            label: Label::new(),
        }
    }

    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

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
        write!(f, " d='M{} {} v{}", x, rect.y, rect.height)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {} h{}", x, y, width)?;
        }
        writeln!(f, "' />")
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
        let text = Text::new(Edge::Top, anchor).class_name("tick");
        text.display(f)?;
        for tick in &self.ticks {
            tick.tspan(self.edge, rect).display(f)?;
        }
        text.display_done(f)
    }
}
