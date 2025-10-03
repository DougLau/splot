// rect.rs
//
// Copyright (c) 2021-2025  Douglas P Lau
//
use hatmil::{Html, Svg};

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

    /// Get value at right edge
    pub fn right(&self) -> i32 {
        self.x + i32::from(self.width)
    }

    /// Get value at bottom edge
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
    pub fn split(&self, edge: Edge, value: u16) -> (Self, Self) {
        match edge {
            Edge::Top => {
                let height = self.height.saturating_sub(value);
                let h = self.height - height;
                (
                    Rect::new(self.x, self.y + h as i32, self.width, height),
                    Rect::new(self.x, self.y, self.width, h),
                )
            }
            Edge::Left => {
                let width = self.width.saturating_sub(value);
                let w = self.width - width;
                (
                    Rect::new(self.x + w as i32, self.y, width, self.height),
                    Rect::new(self.x, self.y, w, self.height),
                )
            }
            Edge::Bottom => {
                let height = self.height.saturating_sub(value);
                let h = self.height - height;
                (
                    Rect::new(self.x, self.y, self.width, height),
                    Rect::new(self.x, self.y + height as i32, self.width, h),
                )
            }
            Edge::Right => {
                let width = self.width.saturating_sub(value);
                let w = self.width - width;
                (
                    Rect::new(self.x, self.y, width, self.height),
                    Rect::new(self.x + width as i32, self.y, w, self.height),
                )
            }
        }
    }

    /// Intersect horizontally
    pub fn intersect_horiz(&mut self, rhs: &Rect) {
        let x = self.x.max(rhs.x);
        let x2 = self.right().min(rhs.right());
        self.x = x;
        self.width = (x2 - x) as u16;
    }

    /// Intersect vertically
    pub fn intersect_vert(&mut self, rhs: &Rect) {
        let y = self.y.max(rhs.y);
        let y2 = self.bottom().min(rhs.bottom());
        self.y = y;
        self.height = (y2 - y) as u16;
    }

    /// Display the rectangle
    pub fn display(&self, html: &mut Html) {
        Svg::new(html)
            .rect()
            .x(self.x)
            .y(self.y)
            .width(self.width)
            .height(self.height)
            .end();
    }

    pub fn view_box(&self) -> String {
        format!("{} {} {} {}", self.x, self.y, self.width, self.height)
    }
}
