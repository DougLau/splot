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
        let x_domain = NumScale::of_data(data, |pt| pt.x());
        self.x_domain = match self.x_domain {
            Some(xd) => Some(x_domain.union(xd)),
            None => Some(x_domain),
        };
        let y_domain = NumScale::of_data(data, |pt| pt.y());
        self.y_domain = match self.y_domain {
            Some(yd) => Some(y_domain.union(yd)),
            None => Some(y_domain),
        };
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

    fn x_scale(&self) -> NumScale {
        match &self.x_domain {
            Some(domain) => domain.clone(),
            None => NumScale::default(),
        }
    }

    fn y_scale(&self) -> NumScale {
        match &self.y_domain {
            Some(domain) => domain.clone(),
            None => NumScale::default(),
        }.inverted()
    }

    pub fn x_axis(&self) -> Horizontal {
        Horizontal::new(self.x_scale().ticks())
    }

    pub fn y_axis(&self) -> Vertical {
        Vertical::new(self.y_scale().ticks())
    }

    pub(crate) fn x_norm(&self, x: f32) -> f32 {
        self.x_scale().normalize(x)
    }

    pub(crate) fn y_norm(&self, y: f32) -> f32 {
        self.y_scale().normalize(y)
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
