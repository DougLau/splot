use crate::page::{Edge, Rect};
use crate::text::{Anchor, Label, Text};
use std::fmt;

#[derive(Debug)]
pub struct Tick {
    value: f32,
    text: String,
}

impl Tick {
    pub fn new<T>(value: f32, text: T) -> Self
    where
        T: Into<String>,
    {
        let text = text.into();
        Tick { value, text }
    }
}

pub struct Axis {
    edge: Edge,
    ticks: Vec<Tick>,
    name: Option<String>,
    inverted: bool,
    label: Label,
}

impl Axis {
    pub(crate) fn new(edge: Edge, ticks: Vec<Tick>) -> Self {
        Self {
            edge,
            ticks,
            name: None,
            inverted: false,
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

    pub fn inverted(mut self) -> Self {
        self.inverted = !self.inverted;
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
            Some(_) => 80,
            None => 40,
        }
    }

    pub(crate) fn display(
        &self,
        f: &mut fmt::Formatter,
        mut rect: Rect,
        area: &Rect,
    ) -> fmt::Result {
        match self.edge {
            Edge::Top | Edge::Bottom => rect.intersect_horiz(&area),
            Edge::Left | Edge::Right => rect.intersect_vert(&area),
        }
        self.display_line(f, &rect)?;
        if let Some(name) = &self.name {
            let r = rect.split(self.edge, self.space() / 2);
            let text =
                Text::new(name, self.edge, Anchor::Middle).class_name("axis");
            text.display(f, r)?;
        }
        Ok(())
    }

    fn display_line(&self, f: &mut fmt::Formatter, rect: &Rect) -> fmt::Result {
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
        writeln!(f, "<path class='line' d='M{} {} {}{}' />", x, y, hv, span)
    }
}
