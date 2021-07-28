// domain.rs
//
// Copyright (c) 2021  Douglas P Lau
//
use crate::axis::{Horizontal, Vertical};
use crate::point::Point;
use crate::scale::Scale;

/// Data domain in two dimensions
///
/// The scales are:
///
/// - `X`, abscissa (horizontal)
/// - `Y`, ordinate (vertical)
#[derive(Default)]
pub struct Domain<X, Y>
where
    X: Scale + Default,
    Y: Scale + Default,
{
    x_scale: X,
    y_scale: Y,
}

impl<X, Y> Domain<X, Y>
where
    X: Scale + Default,
    Y: Scale + Default,
{
    /// Create a domain from a set of points
    pub fn from_data<P>(data: &[P]) -> Self
    where
        P: Point,
    {
        let x_scale = X::from_data(data, |pt| pt.x());
        let y_scale = Y::from_data(data, |pt| pt.y());
        Domain { x_scale, y_scale }
    }

    /// Adjust domain to include a set of points
    pub fn with_data<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.x_scale = self.x_scale.union(X::from_data(data, |pt| pt.x()));
        self.y_scale = self.y_scale.union(Y::from_data(data, |pt| pt.y()));
        self
    }

    /// Set `X` domain to a set of points
    pub fn with_x<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.x_scale = X::from_data(data, |pt| pt.x());
        self
    }

    /// Set `Y` domain to a set of points
    pub fn with_y<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.y_scale = Y::from_data(data, |pt| pt.y());
        self
    }

    /// Get horizontal `X` axis
    pub fn x_axis(&self) -> Horizontal {
        Horizontal::new(self.x_scale.ticks())
    }

    /// Get vertical `Y` axis
    pub fn y_axis(&self) -> Vertical {
        Vertical::new(self.y_scale.inverted().ticks())
    }

    /// Normalize an `X` value
    pub(crate) fn x_norm(&self, x: f32) -> f32 {
        self.x_scale.normalize(x)
    }

    /// Normalize a `Y` value
    pub(crate) fn y_norm(&self, y: f32) -> f32 {
        self.y_scale.inverted().normalize(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::sealed::Scale;
    use crate::scale::Numeric;

    #[test]
    fn test() {
        let data = [(45.0, 150.0), (90.0, 200.0)];
        let domain = Domain::<Numeric, Numeric>::from_data(&data);
        let ticks = Numeric::new(45.0, 90.0).ticks();
        assert_eq!(domain.x_axis(), Horizontal::new(ticks));
    }
}
