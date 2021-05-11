use std::fmt;

#[derive(Clone, Copy)]
pub enum AspectRatio {
    Landscape,
    Square,
    Portrait,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Edge {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u16,
    pub height: u16,
}

impl AspectRatio {
    pub(crate) fn rect(self) -> Rect {
        match self {
            AspectRatio::Landscape => Rect::new(0, 0, 1000, 750),
            AspectRatio::Square => Rect::new(0, 0, 1000, 1000),
            AspectRatio::Portrait => Rect::new(0, 0, 750, 1000),
        }
    }
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn inset(mut self, value: u16) -> Self {
        let vi = i32::from(value);
        self.x += vi;
        self.y += vi;
        let v2 = 2 * value;
        self.width = self.width.saturating_sub(v2);
        self.height = self.height.saturating_sub(v2);
        self
    }
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
}
