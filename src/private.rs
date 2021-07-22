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

pub(crate) trait SealedScale<V> {
    fn normalize(&self, value: V) -> f32;
    fn ticks(&self) -> Vec<Tick>;
}

pub trait SealedPlot {
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result;
}
