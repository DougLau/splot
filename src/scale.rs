use crate::axis::Tick;

pub trait Scale<V> {
    fn proportion(&self, value: V) -> f32;
    fn ticks(&self) -> Vec<Tick>;
}

#[derive(Clone)]
pub struct NumScale {
    start: f32,
    stop: f32,
    tick_spacing: f32,
}

impl Default for NumScale {
    fn default() -> Self {
        Self::new(f32::default(), f32::default())
    }
}

impl NumScale {
    fn new(min: f32, max: f32) -> Self {
        let tick_spacing = Self::spacing(min, max);
        let start = (min / tick_spacing).floor() * tick_spacing;
        let stop = (max / tick_spacing).ceil() * tick_spacing;
        Self {
            start,
            stop,
            tick_spacing,
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

    pub fn of_data<'a, I, P>(data: I, get: fn(&P) -> f32) -> Self
    where
        I: IntoIterator<Item = &'a P>,
        P: 'a,
    {
        let mut it = data.into_iter();
        if let Some(pt) = it.next() {
            let mut min = get(pt);
            let mut max = min;
            while let Some(pt) = it.next() {
                let x = get(pt);
                if x < min {
                    min = x;
                }
                if x > max {
                    max = x;
                }
            }
            Self::new(min, max)
        } else {
            Self::default()
        }
    }

    pub fn inverted(mut self) -> Self {
        self.tick_spacing = -self.tick_spacing;
        self
    }

    pub fn tick_spacing(&self) -> f32 {
        self.tick_spacing
    }

    fn add_tick(&self, val: f32, ticks: &mut Vec<Tick>) {
        let value = self.proportion(val);
        let text = format!("{}", val);
        let tick = Tick::new(value, text);
        ticks.push(tick);
    }
}

impl Scale<f32> for NumScale {
    fn proportion(&self, value: f32) -> f32 {
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
    fn ticks(&self) -> Vec<Tick> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(NumScale::new(0.0, 10.0).tick_spacing(), 1.0);
        assert_eq!(NumScale::new(9.5, 10.0).tick_spacing(), 0.1);
        assert_eq!(NumScale::new(0.0, 25.0).tick_spacing(), 5.0);
        assert_eq!(NumScale::new(0.0, 30.0).tick_spacing(), 5.0);
        assert_eq!(NumScale::new(0.0, 40.0).tick_spacing(), 5.0);
        assert_eq!(NumScale::new(0.0, 50.0).tick_spacing(), 10.0);
        assert_eq!(NumScale::new(0.0, 75.0).tick_spacing(), 10.0);
        assert_eq!(NumScale::new(0.0, 100.0).tick_spacing(), 10.0);
        //assert_eq!(NumScale::new(-50.0, 50.0).tick_spacing(), 10.0);
        assert_eq!(NumScale::new(0.0, 1.0).tick_spacing(), 0.1);
        assert_eq!(NumScale::new(0.0, 1.5).tick_spacing(), 0.25);
        assert_eq!(NumScale::new(0.0, 2.0).tick_spacing(), 0.25);
        assert_eq!(NumScale::new(0.0, 0.1).tick_spacing(), 0.01);
        assert_eq!(NumScale::new(0.0, 0.1).tick_spacing(), 0.01);
    }
}
