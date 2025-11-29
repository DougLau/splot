// text.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::point::{IntoPoint, Point};
use crate::rect::{Edge, Rect};
use hatmil::{Html, Svg};

/// Vertical offset relative to point
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// Chart label
#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    offset: VerticalOffset,
    anchor: Anchor,
    rounding_precision: Option<usize>,
}

/// Text element
pub struct Text<'a> {
    edge: Edge,
    anchor: Anchor,
    rect: Option<Rect>,
    dy: Option<f32>,
    class_name: Option<&'a str>,
}

/// Text span
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

impl Anchor {
    fn value(self) -> &'static str {
        match self {
            Anchor::Start => "start",
            Anchor::Middle => "middle",
            Anchor::End => "end",
        }
    }
}

impl Default for Label {
    fn default() -> Self {
        Label {
            offset: VerticalOffset::At,
            anchor: Anchor::Middle,
            rounding_precision: None,
        }
    }
}

#[allow(dead_code)]
impl Label {
    /// Create a new label
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the vertical offset
    pub fn vertical_offset(&self) -> f32 {
        match self.offset {
            VerticalOffset::Above => -1.0,
            VerticalOffset::At => 0.0,
            VerticalOffset::Below => 1.0,
        }
    }

    /// Make label above
    pub fn above(mut self) -> Self {
        self.offset = VerticalOffset::Above;
        self
    }

    /// Make label below
    pub fn below(mut self) -> Self {
        self.offset = VerticalOffset::Below;
        self
    }

    /// Set anchor to start
    pub fn start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    /// Set anchor to end
    pub fn end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    /// Get rounded value
    pub fn rounded(&self, value: f32) -> String {
        match self.rounding_precision {
            None => value.to_string(),
            Some(digits) => format!("{:.1$}", value, digits),
        }
    }

    /// Display at a location
    pub fn display_at<P>(&self, x: i32, y: i32, pt: P, html: &mut Html)
    where
        P: IntoPoint,
    {
        let pt: Point = pt.into();
        let lbl = format!("({} {})", pt.x, pt.y);
        let tspan = Tspan::new(&lbl).x(x).y(y).dy(-0.66);
        tspan.display(html);
    }
}

impl<'a> Text<'a> {
    /// Create a new text element
    pub fn new(edge: Edge) -> Self {
        Text {
            edge,
            anchor: Anchor::Middle,
            rect: None,
            dy: None,
            class_name: None,
        }
    }

    /// Set anchor position
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Set `dy`
    #[allow(dead_code)]
    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    /// Set rectangle
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// Set class name
    pub fn class_name(mut self, class_name: &'a str) -> Self {
        self.class_name = Some(class_name);
        self
    }

    /// Display as HTML
    pub fn display(&self, html: &mut Html) {
        let svg = Svg::new(html);
        let mut t = svg.text();
        if let Some(class_name) = self.class_name {
            t = t.class(class_name);
        }
        if let Some(transform) = self.transform() {
            t = t.transform(transform);
        }
        if let Some(dy) = self.dy {
            t = t.dy(format!("{dy}em'"));
        }
        t.text_anchor(self.anchor.value());
    }

    /// Get transform
    fn transform(&self) -> Option<String> {
        let rect = self.rect?;
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
        let mut t = format!("translate({x} {y})");
        match self.edge {
            Edge::Left => t.push_str(" rotate(-90)"),
            Edge::Right => t.push_str(" rotate(90)"),
            _ => (),
        }
        Some(t)
    }
}

impl<'a> Tspan<'a> {
    /// Create a new text span
    pub fn new(text: &'a str) -> Self {
        Tspan {
            text,
            x: None,
            y: None,
            dy: None,
        }
    }

    /// Set X
    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    /// Set Y
    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    /// Set DY
    pub fn dy(mut self, dy: f32) -> Self {
        self.dy = Some(dy);
        self
    }

    /// Display as HTML
    pub fn display(&self, html: &mut Html) {
        let svg = Svg::new(html);
        let mut t = svg.tspan();
        if let Some(x) = self.x {
            t = t.x(x);
        }
        if let Some(y) = self.y {
            t = t.y(y);
        }
        if let Some(dy) = self.dy {
            t.dy(format!("{dy}em"));
        }
        html.text(self.text);
        html.end(); // tspan
    }
}

impl Tick {
    pub const LEN: i32 = 20;
    pub const HLEN: i32 = Tick::LEN + 8;
    pub const VLEN: i32 = Tick::LEN * 2;

    /// Create a new tick
    pub fn new<T>(value: f32, text: T) -> Self
    where
        T: Into<String>,
    {
        let text = text.into();
        Tick { value, text }
    }

    /// Get text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get X
    pub fn x(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Left => rect.right() - len,
            Edge::Right => rect.x + len,
            _ => rect.x + (self.value * rect.width as f32).round() as i32,
        }
    }

    /// Get Y
    pub fn y(&self, edge: Edge, rect: Rect, len: i32) -> i32 {
        match edge {
            Edge::Top => rect.bottom() - len,
            Edge::Bottom => rect.y + len,
            _ => rect.y + (self.value * rect.height as f32).round() as i32,
        }
    }

    /// Make text span
    pub fn tspan(&self, edge: Edge, rect: Rect) -> Tspan<'_> {
        let x = self.x(edge, rect, Tick::HLEN);
        let y = self.y(edge, rect, Tick::VLEN);
        Tspan::new(self.text()).x(x).y(y).dy(0.33)
    }
}
