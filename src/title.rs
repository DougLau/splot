// title.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use crate::rect::{Edge, Rect};
use crate::text::Anchor;

/// Chart title
///
/// ```rust
/// use splot::{Edge, Title};
///
/// let title = Title::from("Grand Title");
/// let left_title = Title::from("A Title").on_edge(Edge::Left);
/// ````
pub struct Title<'a> {
    /// Text of title
    text: &'a str,
    /// Edge of chart
    edge: Edge,
    /// Anchor point
    anchor: Anchor,
    /// Rectangle
    rect: Option<Rect>,
}

impl<'a> From<&'a str> for Title<'a> {
    fn from(text: &'a str) -> Self {
        Title::new(text)
    }
}

impl<'a> Title<'a> {
    /// Create a new title
    fn new(text: &'a str) -> Self {
        Title {
            text,
            edge: Edge::Top,
            anchor: Anchor::Middle,
            rect: None,
        }
    }

    /// Place title on an edge
    pub fn on_edge(mut self, edge: Edge) -> Self {
        self.edge = edge;
        self
    }

    /// Anchor title text
    pub fn anchor_to(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Split title area from rectangle
    pub(crate) fn split(&mut self, area: &mut Rect, height: u16) {
        self.rect = Some(area.split(self.edge, height));
    }

    /// Get title text
    pub fn text(&self) -> &str {
        self.text
    }

    /// Get title edge
    pub fn edge(&self) -> Edge {
        self.edge
    }

    /// Get title anchor
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    /// Get rectangle
    pub fn rect(&self) -> Option<Rect> {
        self.rect
    }

    /// Get title transform
    pub fn transform(&self) -> Option<String> {
        let Some(rect) = &self.rect else {
            return None;
        };
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
