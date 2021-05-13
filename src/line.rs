use crate::axis::Axis;
use crate::page::{Edge, Rect};
use crate::plot::Plot;
use crate::point::Point;
use crate::scale::{NumScale, Scale, Value};
use std::fmt;

pub struct LinePlot<'a, P, X, Y>
where
    P: Point<X, Y> + 'a,
    X: Value,
    Y: Value,
{
    data: &'a [P],
    x_domain: Option<NumScale<X>>,
    y_domain: Option<NumScale<Y>>,
}

impl<'a, P, X, Y> Plot for LinePlot<'a, P, X, Y>
where
    P: Point<X, Y> + 'a,
    X: Value,
    Y: Value,
{
    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        write!(f, "<path class='series-a' d='")?;
        let x_scale = self.x_scale();
        let y_scale = self.y_scale();
        for (i, pt) in self.data.iter().enumerate() {
            let x = rect.x as f32
                + f32::from(rect.width) * x_scale.proportion(pt.x().as_f32());
            let y = rect.y as f32
                + f32::from(rect.height) * y_scale.proportion(pt.y().as_f32());
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "' />")
    }
}

impl<'a, P, X, Y> From<LinePlot<'a, P, X, Y>> for Box<dyn Plot + 'a>
where
    P: Point<X, Y> + 'a,
    X: Value + 'a,
    Y: Value + 'a,
{
    fn from(plot: LinePlot<'a, P, X, Y>) -> Self {
        Box::new(plot)
    }
}

impl<'a, P, X, Y> LinePlot<'a, P, X, Y>
where
    P: Point<X, Y> + 'a,
    X: Value,
    Y: Value,
{
    pub fn new(data: &'a [P]) -> Self {
        LinePlot {
            data,
            x_domain: None,
            y_domain: None,
        }
    }

    pub fn x_domain<T>(mut self, data: &[T]) -> Self
    where
        T: Point<X, Y>,
    {
        self.x_domain = Some(NumScale::of_data(data, |pt| pt.x()));
        self
    }

    pub fn y_domain<T>(mut self, data: &[T]) -> Self
    where
        T: Point<X, Y>,
    {
        self.y_domain = Some(NumScale::of_data(data, |pt| pt.y()));
        self
    }

    fn x_scale(&self) -> NumScale<X> {
        match &self.x_domain {
            Some(domain) => domain.clone(),
            None => NumScale::of_data(&self.data[..], |pt| pt.x()),
        }
    }

    pub fn x_axis(&self) -> Axis {
        let ticks = self.x_scale().ticks();
        Axis::new(Edge::Bottom, ticks)
    }

    fn y_scale(&self) -> NumScale<Y> {
        match &self.y_domain {
            Some(domain) => domain.clone(),
            None => NumScale::of_data(&self.data[..], |pt| pt.y()),
        }
    }

    pub fn y_axis(&self) -> Axis {
        let ticks = self.y_scale().ticks();
        Axis::new(Edge::Left, ticks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [(45.0, 150.0), (90.0, 200.0)];
        let plot = LinePlot::new(&data).x_domain(&[0, 100]);
    }
}
