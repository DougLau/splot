use crate::axis::Tick;
use crate::page::Rect;
use std::fmt;

pub trait SealedAxis {
    fn split(&self, area: &mut Rect) -> Rect;
    fn display(
        &self,
        f: &mut fmt::Formatter,
        rect: Rect,
        area: Rect,
    ) -> fmt::Result;
}

pub trait SealedPlot {
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result;
}

pub trait SealedScale {
    fn from_data<'a, I, P>(data: I, get: fn(&P) -> f32) -> Self
    where
        I: IntoIterator<Item = &'a P>,
        P: 'a;
    fn union(&self, rhs: Self) -> Self;
    fn inverted(&self) -> Self;
    fn normalize(&self, value: f32) -> f32;
    fn ticks(&self) -> Vec<Tick>;
}
