use crate::axis::{Horizontal, Vertical};
use crate::point::Point;
use crate::scale::{Linear, Scale};

#[derive(Default)]
pub struct Domain {
    x_scale: Scale,
    y_scale: Scale,
}

impl Domain {
    pub fn with_data<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        let x_scale = Linear::of_data(data, |pt| pt.x());
        self.x_scale = Scale::Linear(match self.x_scale {
            Scale::Linear(xd) => x_scale.union(xd),
            _ => x_scale,
        });
        let y_scale = Linear::of_data(data, |pt| pt.y());
        self.y_scale = Scale::Linear(match self.y_scale {
            Scale::Linear(yd) => y_scale.union(yd),
            _ => y_scale,
        });
        self
    }

    pub fn with_x<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.x_scale = Scale::Linear(Linear::of_data(data, |pt| pt.x()));
        self
    }

    pub fn with_y<P>(mut self, data: &[P]) -> Self
    where
        P: Point,
    {
        self.y_scale = Scale::Linear(Linear::of_data(data, |pt| pt.y()));
        self
    }

    fn x_scale(&self) -> Scale {
        match &self.x_scale {
            Scale::Unset => Scale::Linear(Linear::default()),
            _ => self.x_scale.clone(),
        }
    }

    fn y_scale(&self) -> Scale {
        match &self.y_scale {
            Scale::Unset => Scale::Linear(Linear::default().inverted()),
            Scale::Linear(dom) => {
                Scale::Linear(dom.clone().inverted())
            }
        }
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
