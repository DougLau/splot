// text.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::rect::{Edge, Rect};
use hatmil::Value;

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

/// Tick marks for axis labels
#[derive(Debug, PartialEq)]
pub struct Tick {
    value: f32,
    text: String,
}

impl From<Anchor> for Value<'_> {
    fn from(anchor: Anchor) -> Self {
        Value::from(match anchor {
            Anchor::Start => "start",
            Anchor::Middle => "middle",
            Anchor::End => "end",
        })
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
            Some(digits) => format!("{value:.0$}", digits),
            None => value.to_string(),
        }
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
}
