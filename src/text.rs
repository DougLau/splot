// text.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::point::{IntoPoint, Point};
use crate::rect::{Edge, Rect};
use std::fmt;

/// Text label point
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum LabelPoint {
    /// Minimum point (start of bar/column)
    Minimum,
    /// Center point
    Center,
    /// Maximum point (end of bar/column)
    Maximum,
}

/// Vertical offset relative to point
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum VerticalOffset {
    /// Label below point
    Below,
    /// Label at point
    At,
    /// Label above point
    Above,
}

/// Text anchor
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Anchor {
    /// Anchor at start of text
    Start,
    /// Anchor at middle of text
    Middle,
    /// Anchor at end of text
    End,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    point: LabelPoint,
    offset: VerticalOffset,
    anchor: Anchor,
    rounding_precision: Option<usize>,
}

pub struct Text<'a> {
    edge: Edge,
    anchor: Anchor,
    rect: Option<Rect>,
    dy: Option<f32>,
    class_name: Option<&'a str>,
}

pub struct Tspan<'a> {
    text: &'a str,
    x: Option<i32>,
    y: Option<i32>,
    dy: Option<f32>,
}

/// Tick marks for axis labels
#[derive(Debug, PartialEq)]
pub struct Tick {
    value: f32,
    text: String,
}

impl fmt::Display for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Anchor::Start => write!(f, " text-anchor='start'"),
            Anchor::Middle => write!(f, " text-anchor='middle'"),
            Anchor::End => write!(f, " text-anchor='end'"),
        }
    }
}

impl Default for Label {
    fn default() -> Self {
        Label {
            point: LabelPoint::Center,
            offset: VerticalOffset::At,
            anchor: Anchor::Middle,
            rounding_precision: None,
        }
    }
}

#[allow(dead_code)]
impl Label {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label_point(&self) -> LabelPoint {
        self.point
    }

    pub fn vertical_offset(&self) -> f32 {
        match self.offset {
            VerticalOffset::Above => -1.0,
            VerticalOffset::At => 0.0,
            VerticalOffset::Below => 1.0,
        }
    }

    pub fn minimum(mut self) -> Self {
        self.point = LabelPoint::Minimum;
        self
    }

    pub fn maximum(mut self) -> Self {
        self.point = LabelPoint::Maximum;
        self
    }

    pub fn above(mut self) -> Self {
        self.offset = VerticalOffset::Above;
        self
    }

    pub fn below(mut self) -> Self {
        self.offset = VerticalOffset::Below;
        self
    }

    pub fn start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    pub fn end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    pub fn rounded(&self, value: f32) -> String {
        match self.rounding_precision {
            None => value.to_string(),
            Some(digits) => format!("{:.1$}", value, digits),
        }
    }

    pub fn display<P>(
        &self,
        f: &mut fmt::Formatter,
        x: i32,
        y: i32,
        pt: P,
    ) -> fmt::Result
    where
        P: IntoPoint,
    {
        let pt: Point = pt.into();
        let lbl = format!("({} {})", pt.x, pt.y);
        let tspan = Tspan::new(&lbl).x(x).y(y).dy(-0.66);
        tspan.display(f)
    }
}

impl<'a> Text<'a> {
    pub fn new(edge: Edge) -> Self {
        Text {
            edge,
            anchor: Anchor::Middle,
            rect: None,
            dy: None,
            class_name: None,
        }
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    #[allow(dead_code)]
    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    pub fn class_name(mut self, class_name: &'a str) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<text")?;
        if let Some(class_name) = self.class_name {
            write!(f, " class='{}'", class_name)?;
        }
        if let Some(rect) = self.rect {
            self.transform(f, rect)?;
        }
        if let Some(dy) = self.dy {
            write!(f, " dy='{dy}em'")?;
        }
        writeln!(f, "{}>", self.anchor)
    }

    pub fn display_done(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "</text>")
    }

    fn transform(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        let x = match (self.edge, self.anchor) {
            (Edge::Top, Anchor::Start) | (Edge::Bottom, Anchor::Start) => {
                rect.x
            }
            (Edge::Top, Anchor::End) | (Edge::Bottom, Anchor::End) => {
                rect.right()
            }
            _ => rect.x + i32::from(rect.width) / 2,
        };
        let y = match (self.edge, self.anchor) {
            (Edge::Left, Anchor::End) | (Edge::Right, Anchor::Start) => rect.y,
            (Edge::Left, Anchor::Start) | (Edge::Right, Anchor::End) => {
                rect.bottom()
            }
            _ => rect.y + i32::from(rect.height) / 2,
        };
        write!(f, " transform='translate({x} {y})")?;
        match self.edge {
            Edge::Left => write!(f, " rotate(-90)")?,
            Edge::Right => write!(f, " rotate(90)")?,
            _ => (),
        }
        write!(f, "'")
    }
}

impl<'a> Tspan<'a> {
    pub fn new(text: &'a str) -> Self {
        Tspan {
            text,
            x: None,
            y: None,
            dy: None,
        }
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<tspan")?;
        if let Some(x) = self.x {
            write!(f, " x='{x}'")?;
        }
        if let Some(y) = self.y {
            write!(f, " y='{y}'")?;
        }
        if let Some(dy) = self.dy {
            write!(f, " dy='{dy}em'")?;
        }
        write!(f, ">{}", &self.text)?;
        writeln!(f, "</tspan>")
    }
}

impl Tick {
    pub const LEN: i32 = 20;
    pub const HLEN: i32 = Tick::LEN + 8;
    pub const VLEN: i32 = Tick::LEN * 2;

    pub fn new<T>(value: f32, text: T) -> Self
    where
        T: Into<String>,
    {
        let text = text.into();
        Tick { value, text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn x(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Left => rect.right() - len,
            Edge::Right => rect.x + len,
            _ => rect.x + (self.value * rect.width as f32).round() as i32,
        }
    }

    pub fn y(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Top => rect.bottom() - len,
            Edge::Bottom => rect.y + len,
            _ => rect.y + (self.value * rect.height as f32).round() as i32,
        }
    }

    pub fn tspan(&self, edge: Edge, rect: Rect) -> Tspan {
        let x = self.x(edge, rect, Tick::HLEN);
        let y = self.y(edge, rect, Tick::VLEN);
        Tspan::new(self.text()).x(x).y(y).dy(0.33)
    }
}
