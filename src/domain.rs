use crate::axis::Axis;
use crate::page::Edge;
use crate::point::Point;
use crate::scale::{NumScale, Scale};

#[derive(Default)]
pub struct Domain {
    x_domain: Option<NumScale>,
    y_domain: Option<NumScale>,
}

impl Domain {
    pub fn with_data<T>(mut self, data: &[T]) -> Self
    where
        T: Point,
    {
        if self.x_domain.is_none() {
            self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        }
        if self.y_domain.is_none() {
            self.y_domain = Some(NumScale::of_data(data, |pt| pt.y()));
        }
        self
    }

    pub fn with_x<T>(mut self, data: &[T]) -> Self
    where
        T: Point,
    {
        self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        self
    }

    pub fn with_y<T>(mut self, data: &[T]) -> Self
    where
        T: Point,
    {
        self.y_domain = Some(NumScale::of_data(data, |pt| pt.y()));
        self
    }

    pub(crate) fn x_scale(&self) -> NumScale {
        match &self.x_domain {
            Some(domain) => domain.clone(),
            None => NumScale::new(0.0, 1.0),
        }
    }

    pub(crate) fn y_scale(&self) -> NumScale {
        match &self.y_domain {
            Some(domain) => domain.clone(),
            None => NumScale::new(0.0, 1.0),
        }
    }

    pub fn x_axis(&self) -> Axis {
        let ticks = self.x_scale().ticks();
        Axis::new(Edge::Bottom, ticks)
    }

    pub fn y_axis(&self) -> Axis {
        let ticks = self.y_scale().inverted().ticks();
        Axis::new(Edge::Left, ticks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [(45.0, 150.0), (90.0, 200.0)];
        let domain = Domain::default().with_data(&data);
        let ticks = NumScale::new(45.0, 90.0).ticks();
        assert_eq!(domain.x_axis(), Axis::new(Edge::Bottom, ticks));
    }
}
