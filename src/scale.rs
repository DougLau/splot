// scale.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//
//! Scale items
//!
use crate::point::{IntoPoint, Point};
use crate::text::Tick;

/// Numeric scale
#[derive(Clone, Debug)]
pub struct Numeric {
    start: f32,
    stop: f32,
    tick_spacing: f32,
}

/// Scale
#[derive(Clone, Debug)]
pub enum Scale {
    Numeric(Numeric),
}

impl Numeric {
    /// Create a new numeric scale
    pub(crate) fn new(min: f32, max: f32) -> Self {
        let tick_spacing = Self::spacing(min, max);
        let start = (min / tick_spacing).floor() * tick_spacing;
        let stop = (max / tick_spacing).ceil() * tick_spacing;
        Self {
            start,
            stop,
            tick_spacing,
        }
    }

    fn union(&self, rhs: Self) -> Self {
        let min = self.start.min(rhs.start);
        let max = self.stop.max(rhs.stop);
        Numeric::new(min, max)
    }

    fn inverted(&self) -> Self {
        Numeric {
            start: self.start,
            stop: self.stop,
            tick_spacing: -self.tick_spacing,
        }
    }

    fn spacing(min: f32, max: f32) -> f32 {
        let span = max - min;
        let power = span.log10().floor() as i32;
        let spc = 10_f32.powi(power);
        let start = (min / spc).floor() * spc;
        let stop = (max / spc).ceil() * spc;
        let steps = (stop - start) / spc;
        if steps <= 1.0 {
            spc / 10.0
        } else if steps <= 2.0 {
            spc / 4.0
        } else if steps < 5.0 {
            spc / 2.0
        } else {
            spc
        }
    }

    fn normalize(&self, value: f32) -> f32 {
        let a = self.start;
        let b = self.stop;
        if b - a > f32::EPSILON {
            if self.tick_spacing() > 0.0 {
                (value - a) / (b - a)
            } else {
                (b - value) / (b - a)
            }
        } else {
            0.5
        }
    }

    fn tick_spacing(&self) -> f32 {
        self.tick_spacing
    }

    fn add_tick(&self, val: f32, ticks: &mut Vec<Tick>) {
        let value = self.normalize(val);
        let text = format!("{val}");
        let tick = Tick::new(value, text);
        ticks.push(tick);
    }

    pub(crate) fn ticks(&self) -> Vec<Tick> {
        let mut ticks = vec![];
        let spacing = self.tick_spacing();
        if spacing > 0.0 {
            let mut val = self.start;
            while val <= self.stop {
                self.add_tick(val, &mut ticks);
                val += spacing;
            }
        } else {
            let mut val = self.stop;
            while val >= self.start {
                self.add_tick(val, &mut ticks);
                val += spacing;
            }
        };
        ticks
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Numeric(Numeric::new(0.0, 1.0))
    }
}

impl Scale {
    /// Create a scale from data points
    pub fn from_data<'a, I, P>(data: I, get: fn(Point) -> f32) -> Self
    where
        I: IntoIterator<Item = &'a P>,
        P: IntoPoint + 'a,
    {
        let mut it = data.into_iter();
        if let Some(pt) = it.next() {
            let mut min = get((*pt).into());
            let mut max = min;
            for pt in it {
                let x = get((*pt).into());
                if x < min {
                    min = x;
                }
                if x > max {
                    max = x;
                }
            }
            Scale::Numeric(Numeric::new(min, max))
        } else {
            Scale::Numeric(Numeric::new(0.0, 1.0))
        }
    }

    pub fn union(&self, rhs: Self) -> Self {
        match (self, rhs) {
            (Scale::Numeric(num), Scale::Numeric(rhs)) => {
                Scale::Numeric(num.union(rhs))
            }
        }
    }

    pub fn inverted(&self) -> Self {
        match self {
            Scale::Numeric(num) => Scale::Numeric(num.inverted()),
        }
    }

    pub fn normalize(&self, value: f32) -> f32 {
        match self {
            Scale::Numeric(num) => num.normalize(value),
        }
    }

    pub fn ticks(&self) -> Vec<Tick> {
        match self {
            Scale::Numeric(num) => num.ticks(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Numeric::new(0.0, 10.0).tick_spacing(), 1.0);
        assert_eq!(Numeric::new(9.5, 10.0).tick_spacing(), 0.1);
        assert_eq!(Numeric::new(0.0, 25.0).tick_spacing(), 5.0);
        assert_eq!(Numeric::new(0.0, 30.0).tick_spacing(), 5.0);
        assert_eq!(Numeric::new(0.0, 40.0).tick_spacing(), 5.0);
        assert_eq!(Numeric::new(0.0, 50.0).tick_spacing(), 10.0);
        assert_eq!(Numeric::new(0.0, 75.0).tick_spacing(), 10.0);
        assert_eq!(Numeric::new(0.0, 100.0).tick_spacing(), 10.0);
        //assert_eq!(Numeric::new(-50.0, 50.0).tick_spacing(), 10.0);
        assert_eq!(Numeric::new(0.0, 1.0).tick_spacing(), 0.1);
        assert_eq!(Numeric::new(0.0, 1.5).tick_spacing(), 0.25);
        assert_eq!(Numeric::new(0.0, 2.0).tick_spacing(), 0.25);
        assert_eq!(Numeric::new(0.0, 0.1).tick_spacing(), 0.01);
        assert_eq!(Numeric::new(0.0, 0.1).tick_spacing(), 0.01);
    }
}
