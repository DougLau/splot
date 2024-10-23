// title.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::rect::{Edge, Rect};
use crate::text::{Anchor, Text};
use std::fmt;

/// Chart title
pub struct Title<'a> {
    text: &'a str,
    anchor: Anchor,
    pub(crate) edge: Edge,
}

impl<'a> From<&'a str> for Title<'a> {
    fn from(text: &'a str) -> Self {
        Title::new(text)
    }
}

impl<'a> Title<'a> {
    /// Create a new title
    pub fn new(text: &'a str) -> Self {
        Title {
            text,
            anchor: Anchor::Middle,
            edge: Edge::Top,
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

    /// Put title on bottom of chart
    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    /// Put title on left side of chart
    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    /// Put title on right side of chart
    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    /// Display title
    pub(crate) fn display(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
    ) -> fmt::Result {
        let text = Text::new(self.edge)
            .rect(rect)
            .anchor(self.anchor)
            .class_name("title");
        text.display(f)?;
        writeln!(f, "{}", self.text)?;
        text.display_done(f)
    }
}
