use crate::axis::{Horizontal, Vertical};
use crate::point::Point;
use crate::private::SealedScale;
use crate::scale::Linear;

#[derive(Default)]
pub struct Domain {
    x_domain: Option<Linear>,
    y_domain: Option<Linear>,
}

impl Domain {
    pub fn with_data<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        let x_domain = Linear::of_data(data, |pt| pt.x());
        self.x_domain = match self.x_domain {
            Some(xd) => Some(x_domain.union(xd)),
            None => Some(x_domain),
        };
        let y_domain = Linear::of_data(data, |pt| pt.y());
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
        self.x_domain = Some(Linear::of_data(data, |pt| pt.x()));
        self
    }

    pub fn with_y<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.y_domain = Some(Linear::of_data(data, |pt| pt.y()));
        self
    }

    fn x_scale(&self) -> Linear {
        match &self.x_domain {
            Some(domain) => domain.clone(),
            None => Linear::default(),
        }
    }

    fn y_scale(&self) -> Linear {
        match &self.y_domain {
            Some(domain) => domain.clone(),
            None => Linear::default(),
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
        let ticks = Linear::new(45.0, 90.0).ticks();
        assert_eq!(domain.x_axis(), Horizontal::new(ticks));
    }
}
