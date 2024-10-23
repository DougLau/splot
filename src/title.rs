// title.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::rect::{Edge, Rect};
use crate::text::{Anchor, Text};
use std::fmt;

/// Chart title
///
/// ```rust
/// use splot::{Edge, Title};
///
/// let title = Title::from("Grand Title");
/// let left_title = Title::from(("A Title", Edge::Left));
/// ````
pub struct Title<'a> {
    text: &'a str,
    anchor: Anchor,
    edge: Edge,
    rect: Rect,
}

impl<'a> From<&'a str> for Title<'a> {
    fn from(text: &'a str) -> Self {
        Title::new(text)
    }
}

impl<'a> From<(&'a str, Edge)> for Title<'a> {
    fn from((text, edge): (&'a str, Edge)) -> Self {
        Title {
            text,
            anchor: Anchor::Middle,
            edge,
            rect: Rect::default(),
        }
    }
}

impl<'a> fmt::Display for Title<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = Text::new(self.edge)
            .rect(self.rect)
            .anchor(self.anchor)
            .class_name("title");
        text.display(f)?;
        writeln!(f, "{}", self.text)?;
        text.display_done(f)
    }
}

impl<'a> Title<'a> {
    /// Create a new title
    pub fn new(text: &'a str) -> Self {
        Title {
            text,
            anchor: Anchor::Middle,
            edge: Edge::Top,
            rect: Rect::default(),
        }
    }

    /// Anchor title text at start
    pub fn at_start(mut self) -> Self {
        self.anchor = Anchor::Start;
        self
    }

    /// Anchor title text at end
    pub fn at_end(mut self) -> Self {
        self.anchor = Anchor::End;
        self
    }

    /// Split title area from rectangle
    pub(crate) fn split(&mut self, mut area: Rect) -> Rect {
        (area, self.rect) = area.split(self.edge, 100);
        area
    }
}
