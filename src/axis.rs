use crate::page::{Edge, Rect};
use crate::text::{Anchor, Label, Text, Tspan};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Tick {
    value: f32,
    text: String,
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

#[derive(Debug, PartialEq)]
pub struct Axis {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    label: Label,
}

impl Axis {
    pub(crate) fn new(edge: Edge, ticks: Vec<Tick>) -> Self {
        Self {
            edge,
            ticks,
            name: None,
            label: Label::new(),
        }
    }

    pub(crate) fn edge(&self) -> Edge {
        self.edge
    }

    pub fn name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn on_top(mut self) -> Self {
        if self.edge == Edge::Bottom {
            self.edge = Edge::Top;
        }
        self
    }

    pub fn on_right(mut self) -> Self {
        if self.edge == Edge::Left {
            self.edge = Edge::Right;
        }
        self
    }

    pub(crate) fn space(&self) -> u16 {
        match self.name {
            Some(_) => 160,
            None => 80,
        }
    }

    pub(crate) fn display(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Top | Edge::Bottom => rect.intersect_horiz(&area),
            Edge::Left | Edge::Right => rect.intersect_vert(&area),
        }
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

    fn display_tick_lines(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let x = match self.edge {
            Edge::Left => rect.right(),
            _ => rect.x,
        };
        let y = match self.edge {
            Edge::Top => rect.bottom(),
            _ => rect.y,
        };
        let (hv, span) = match self.edge {
            Edge::Top | Edge::Bottom => ("h", rect.width),
            Edge::Left | Edge::Right => ("v", rect.height),
        };
        write!(f, "<path class='axis-line' d='M{} {} {}{}", x, y, hv, span)?;
        let (hv, span) = match self.edge {
            Edge::Top => ("v", Tick::LEN),
            Edge::Bottom => ("v", -Tick::LEN),
            Edge::Left => ("h", Tick::LEN),
            Edge::Right => ("h", -Tick::LEN),
        };
        for tick in self.ticks.iter() {
            let x = tick.x(self.edge, rect, Tick::LEN);
            let y = tick.y(self.edge, rect, Tick::LEN);
            write!(f, " M{} {} {}{}", x, y, hv, span)?;
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
            _ => Anchor::Middle,
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
