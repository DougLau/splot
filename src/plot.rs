// plot.rs
//
// Copyright (c) 2021  Douglas P Lau
//
//! Plot types
use crate::domain::Domain;
use crate::page::Rect;
use crate::point::Point;
use crate::private::SealedPlot;
use crate::scale::Numeric;
use std::fmt;

/// Plot for visualizing data
pub trait Plot: SealedPlot {}

/// Area plot
pub struct Area<'a, P>
where
    P: Point + 'a,
{
    domain: &'a Domain<Numeric, Numeric>,
    data: &'a [P],
}

/// Line plot
pub struct Line<'a, P>
where
    P: Point + 'a,
{
    domain: &'a Domain<Numeric, Numeric>,
    data: &'a [P],
}

/// Scatter plot
pub struct Scatter<'a, P>
where
    P: Point + 'a,
{
    domain: &'a Domain<Numeric, Numeric>,
    data: &'a [P],
}

impl<'a, P> Plot for Area<'a, P> where P: Point {}

impl<'a, P> SealedPlot for Area<'a, P>
where
    P: Point,
{
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path id='plot-{}' class='plot-area' d='", num)?;
        let rx = rect.x as f32;
        let ry = rect.y as f32;
        let rw = f32::from(rect.width);
        let rh = f32::from(rect.height);
        if let Some(pt) = self.data.first() {
            let x = rx + rw * self.domain.x_norm(pt.x());
            let y = ry + rh * self.domain.y_norm(0.0);
            write!(f, "M{} {}", x, y)?;
        }
        for pt in self.data.iter() {
            let x = rx + rw * self.domain.x_norm(pt.x());
            let y = ry + rh * self.domain.y_norm(pt.y());
            write!(f, " {} {}", x, y)?;
        }
        if let Some(pt) = self.data.last() {
            let x = rx + rw * self.domain.x_norm(pt.x());
            let y = ry + rh * self.domain.y_norm(0.0);
            write!(f, " {} {}", x, y)?;
        }
        writeln!(f, "' />")
    }
}

impl<'a, P> Area<'a, P>
where
    P: Point,
{
    pub fn new(domain: &'a Domain<Numeric, Numeric>, data: &'a [P]) -> Self {
        Area { domain, data }
    }
}

impl<'a, P> Plot for Line<'a, P> where P: Point {}

impl<'a, P> SealedPlot for Line<'a, P>
where
    P: Point,
{
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path id='plot-{}' class='plot-line' d='", num)?;
        let rx = rect.x as f32;
        let ry = rect.y as f32;
        let rw = f32::from(rect.width);
        let rh = f32::from(rect.height);
        for (i, pt) in self.data.iter().enumerate() {
            let x = rx + rw * self.domain.x_norm(pt.x());
            let y = ry + rh * self.domain.y_norm(pt.y());
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "' />")
    }
}

impl<'a, P> Line<'a, P>
where
    P: Point,
{
    pub fn new(domain: &'a Domain<Numeric, Numeric>, data: &'a [P]) -> Self {
        Line { domain, data }
    }
}

impl<'a, P> Plot for Scatter<'a, P> where P: Point {}

impl<'a, P> SealedPlot for Scatter<'a, P>
where
    P: Point,
{
    fn display(
        &self,
        f: &mut fmt::Formatter,
        num: usize,
        rect: Rect,
    ) -> fmt::Result {
        write!(f, "<path id='plot-{}' class='plot-scatter' d='", num)?;
        let rx = rect.x as f32;
        let ry = rect.y as f32;
        let rw = f32::from(rect.width);
        let rh = f32::from(rect.height);
        for (i, pt) in self.data.iter().enumerate() {
            let x = rx + rw * self.domain.x_norm(pt.x());
            let y = ry + rh * self.domain.y_norm(pt.y());
            if i == 0 {
                write!(f, "M{} {}", x, y)?;
            } else {
                write!(f, " {} {}", x, y)?;
            }
        }
        writeln!(f, "' />")
    }
}

impl<'a, P> Scatter<'a, P>
where
    P: Point,
{
    pub fn new(domain: &'a Domain<Numeric, Numeric>, data: &'a [P]) -> Self {
        Scatter { domain, data }
    }
}
