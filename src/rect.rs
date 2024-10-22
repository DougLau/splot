// rect.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use std::fmt;

/// Edge of rendered item
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edge {
    Top,
    Left,
    Bottom,
    Right,
}

/// Rendering rectangle
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

/// SVG view box
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewBox(pub Rect);

impl Rect {
    /// Create a new rectangle
    pub fn new(x: i32, y: i32, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(&self) -> i32 {
        self.x + i32::from(self.width)
    }

    pub fn bottom(&self) -> i32 {
        self.y + i32::from(self.height)
    }

    /// Make a new rectangle inset on all edges
    pub fn inset(&self, value: u16) -> Self {
        let vi = i32::from(value);
        let x = self.x + vi;
        let y = self.y + vi;
        let v2 = 2 * value;
        let width = self.width.saturating_sub(v2);
        let height = self.height.saturating_sub(v2);
        Rect::new(x, y, width, height)
    }

    /// Split off rectangle from an edge
    pub fn split(&mut self, edge: Edge, value: u16) -> Self {
        match edge {
            Edge::Top => {
                let y = self.y;
                let height = self.height.saturating_sub(value);
                let h = self.height - height;
                self.y += h as i32;
                self.height = height;
                Rect::new(self.x, y, self.width, h)
            }
            Edge::Left => {
                let x = self.x;
                let width = self.width.saturating_sub(value);
                let w = self.width - width;
                self.x += w as i32;
                self.width = width;
                Rect::new(x, self.y, w, self.height)
            }
            Edge::Bottom => {
                let height = self.height.saturating_sub(value);
                let h = self.height - height;
                let y = self.y + i32::from(height);
                self.height = height;
                Rect::new(self.x, y, self.width, h)
            }
            Edge::Right => {
                let width = self.width.saturating_sub(value);
                let w = self.width - width;
                let x = self.x + i32::from(width);
                self.width = width;
                Rect::new(x, self.y, w, self.height)
            }
        }
    }

    pub fn intersect_horiz(&mut self, rhs: &Rect) {
        let x = self.x.max(rhs.x);
        let x2 = self.right().min(rhs.right());
        self.x = x;
        self.width = (x2 - x) as u16;
    }

    pub fn intersect_vert(&mut self, rhs: &Rect) {
        let y = self.y.max(rhs.y);
        let y2 = self.bottom().min(rhs.bottom());
        self.y = y;
        self.height = (y2 - y) as u16;
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<rect x='{}' y='{}' width='{}' height='{}'/>",
            self.x, self.y, self.width, self.height
        )
    }
}

impl fmt::Display for ViewBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "viewBox='{} {} {} {}'",
            self.0.x, self.0.y, self.0.width, self.0.height
        )
    }
}
