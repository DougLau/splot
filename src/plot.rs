use crate::page::Rect;
use std::fmt;

pub trait Plot {
    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result;
}
