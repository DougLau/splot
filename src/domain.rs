// domain.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
use crate::axis::Axis;
use crate::point::IntoPoint;
use crate::rect::{Edge, Rect};
use crate::scale::Scale;

/// Data domain in two dimensions
///
/// The scales are:
///
/// - `X`, abscissa (horizontal)
/// - `Y`, ordinate (vertical)
#[derive(Clone, Default)]
pub struct Domain {
    x_scale: Scale,
    y_scale: Scale,
}

/// Domain bound to a rectangle
#[derive(Clone, Default)]
pub struct BoundDomain {
    domain: Domain,
    rect: Rect,
}

impl Domain {
    /// Create a domain from a set of points
    pub fn from_data<P>(data: &[P]) -> Self
    where
        P: IntoPoint,
    {
        let x_scale = Scale::from_data(data, |pt| pt.x);
        let y_scale = Scale::from_data(data, |pt| pt.y);
        Domain { x_scale, y_scale }
    }

    /// Adjust domain to include a set of points
    pub fn including<P>(mut self, data: &[P]) -> Self
    where
        P: IntoPoint,
    {
        self.x_scale = self.x_scale.union(Scale::from_data(data, |pt| pt.x));
        self.y_scale = self.y_scale.union(Scale::from_data(data, |pt| pt.y));
        self
    }

    /// Set `X` domain to a set of points
    pub fn set_x<P>(mut self, data: &[P]) -> Self
    where
        P: IntoPoint,
    {
        self.x_scale = Scale::from_data(data, |pt| pt.x);
        self
    }

    /// Set `Y` domain to a set of points
    pub fn set_y<P>(mut self, data: &[P]) -> Self
    where
        P: IntoPoint,
    {
        self.y_scale = Scale::from_data(data, |pt| pt.y);
        self
    }

    /// Get axis on one edge
    pub(crate) fn axis<N>(&self, name: N, edge: Edge) -> Axis
    where
        N: Into<String>,
    {
        match edge {
            Edge::Bottom | Edge::Top => {
                Axis::new(name, edge, self.x_scale.ticks())
            }
            Edge::Left | Edge::Right => {
                Axis::new(name, edge, self.y_scale.inverted().ticks())
            }
        }
    }

    /// Normalize an `X` value
    fn x_norm(&self, x: f32) -> f32 {
        self.x_scale.normalize(x)
    }

    /// Normalize a `Y` value
    fn y_norm(&self, y: f32) -> f32 {
        self.y_scale.inverted().normalize(y)
    }

    /// Bind domain to a rectangle
    pub(crate) fn bind(&self, rect: Rect) -> BoundDomain {
        BoundDomain {
            domain: self.clone(),
            rect,
        }
    }
}

impl BoundDomain {
    /// Map an `X` value
    pub fn x_map(&self, x: f32) -> i32 {
        let rx = self.rect.x as f32;
        let rw = f32::from(self.rect.width);
        let mx = rx + rw * self.domain.x_norm(x);
        mx.round() as i32
    }

    /// Map a `Y` value
    pub fn y_map(&self, y: f32) -> i32 {
        let ry = self.rect.y as f32;
        let rh = f32::from(self.rect.height);
        let my = ry + rh * self.domain.y_norm(y);
        my.round() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::Numeric;

    #[test]
    fn test() {
        let data = [(45.0, 150.0), (90.0, 200.0)];
        let domain = Domain::from_data(&data);
        let ticks = Numeric::new(45.0, 90.0).ticks();
        assert_eq!(
            domain.axis("", Edge::Bottom),
            Axis::new("", Edge::Bottom, ticks)
        );
    }
}
