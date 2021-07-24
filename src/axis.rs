// axis.rs
//
// Copyright (c) 2021  Douglas P Lau
//
//! Axis rendering for charts
use crate::page::{Edge, Rect};
use crate::private::SealedAxis;
use crate::text::{Anchor, Label, Text, Tspan};
use std::fmt;

/// Tick marks for axis labels
#[derive(Debug, PartialEq)]
pub struct Tick {
    value: f32,
    text: String,
}

/// Axis renderer
pub trait Axis: SealedAxis {}

/// Horizontal axis
#[derive(Debug, PartialEq)]
pub struct Horizontal {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

/// Vertical axis
#[derive(Debug, PartialEq)]
pub struct Vertical {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

impl Tick {
    const LEN: i32 = 20;
    const HLEN: i32 = Tick::LEN + 8;
    const VLEN: i32 = Tick::LEN * 2;
    pub fn new<T>(value: f32, text: T) -> Self
    where
        T: Into<String>,
    {
        let text = text.into();
        Tick { value, text }
    }
    fn x(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Left => (rect.right() - len),
            Edge::Right => (rect.x + len),
            _ => rect.x + (self.value * rect.width as f32).round() as i32,
        }
    }
    fn y(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Top => (rect.bottom() - len),
            Edge::Bottom => (rect.y + len),
            _ => rect.y + (self.value * rect.height as f32).round() as i32,
        }
    }
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
        let (y, span) = match self.edge {
            Edge::Top => (rect.bottom(), Tick::LEN),
            Edge::Bottom => (rect.y, -Tick::LEN),
            _ => unreachable!(),
        };
        write!(f, "<path class='axis-line' d='M{} {} h{}", x, y, rect.width)?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {} v{}", x, y, span)?;
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
            let x = tick.x(self.edge, rect, Tick::HLEN);
            let y = tick.y(self.edge, rect, Tick::VLEN);
            let tspan = Tspan::new(&tick.text).x(x).y(y).dy(0.33);
            tspan.display(f)?;
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
        let (x, span) = match self.edge {
            Edge::Left => (rect.right(), Tick::LEN),
            Edge::Right => (rect.x, -Tick::LEN),
            _ => unreachable!(),
        };
        let y = rect.y;
        write!(
            f,
            "<path class='axis-line' d='M{} {} v{}",
            x, y, rect.height
        )?;
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {} h{}", x, y, span)?;
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
            let x = tick.x(self.edge, rect, Tick::HLEN);
            let y = tick.y(self.edge, rect, Tick::VLEN);
            let tspan = Tspan::new(&tick.text).x(x).y(y).dy(0.33);
            tspan.display(f)?;
        }
        text.display_done(f)
    }
}
