// point.rs
//
// Copyright (c) 2021-2024  Douglas P Lau
//

/// Data point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// `X` value
    pub x: f32,
    /// `Y` value
    pub y: f32,
}

/// Data which can represent a point
pub trait IntoPoint: Clone + Copy + Into<Point> {}

impl IntoPoint for Point {}

macro_rules! impl_point_from {
    ($val:ty) => {
        impl From<$val> for Point {
            fn from(item: $val) -> Self {
                Point {
                    x: item as f32,
                    y: 0.0,
                }
            }
        }
        impl IntoPoint for $val {}

        impl From<&$val> for Point {
            fn from(item: &$val) -> Self {
                Point {
                    x: *item as f32,
                    y: 0.0,
                }
            }
        }
        impl IntoPoint for &$val {}
    };
}

impl_point_from!(f32);
impl_point_from!(f64);
impl_point_from!(isize);
impl_point_from!(i8);
impl_point_from!(i16);
impl_point_from!(i32);
impl_point_from!(i64);
impl_point_from!(i128);

macro_rules! impl_point_from_tuple {
    ($val:ty) => {
        impl From<($val, $val)> for Point {
            fn from(item: ($val, $val)) -> Self {
                Point {
                    x: item.0 as f32,
                    y: item.1 as f32,
                }
            }
        }
        impl IntoPoint for ($val, $val) {}

        impl From<(&$val, &$val)> for Point {
            fn from(item: (&$val, &$val)) -> Self {
                Point {
                    x: *item.0 as f32,
                    y: *item.1 as f32,
                }
            }
        }
        impl IntoPoint for (&$val, &$val) {}
    };
}

impl_point_from_tuple!(f32);
impl_point_from_tuple!(f64);
impl_point_from_tuple!(isize);
impl_point_from_tuple!(i8);
impl_point_from_tuple!(i16);
impl_point_from_tuple!(i32);
impl_point_from_tuple!(i64);
impl_point_from_tuple!(i128);

macro_rules! impl_point_from_arr {
    ($val:ty) => {
        impl From<[$val; 2]> for Point {
            fn from(item: [$val; 2]) -> Self {
                Point {
                    x: item[0] as f32,
                    y: item[1] as f32,
                }
            }
        }
        impl IntoPoint for [$val; 2] {}
    };
}

impl_point_from_arr!(f32);
impl_point_from_arr!(f64);
impl_point_from_arr!(isize);
impl_point_from_arr!(i8);
impl_point_from_arr!(i16);
impl_point_from_arr!(i32);
impl_point_from_arr!(i64);
impl_point_from_arr!(i128);
