use crate::axis::{Horizontal, Vertical};
use crate::point::Point;
use crate::private::SealedScale;
use crate::scale::NumScale;

#[derive(Default)]
pub struct Domain {
    x_domain: Option<NumScale>,
    y_domain: Option<NumScale>,
}

impl Domain {
    pub fn with_data<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        if self.x_domain.is_none() {
            self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        }
        if self.y_domain.is_none() {
            self.y_domain = Some(NumScale::of_data(data, |pt| pt.y()));
        }
        self
    }

    pub fn with_x<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        self
    }

    pub fn with_y<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
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

    pub fn x_axis(&self) -> Horizontal {
        Horizontal::new(self.x_scale().ticks())
    }

    pub fn y_axis(&self) -> Vertical {
        Vertical::new(self.y_scale().inverted().ticks())
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
        assert_eq!(domain.x_axis(), Horizontal::new(ticks));
    }
}
