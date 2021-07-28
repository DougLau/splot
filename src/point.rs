// point.rs
//
// Copyright (c) 2021  Douglas P Lau
//

/// Data point
pub trait Point {
    /// Get the `X` value
    fn x(&self) -> f32;

    /// Get the `Y` value
    fn y(&self) -> f32;
}

macro_rules! impl_point_as {
    ($pt:ty) => {
        impl Point for $pt {
            fn x(&self) -> f32 {
                *self as f32
            }

            fn y(&self) -> f32 {
                *self as f32
            }
        }
    };
}

impl_point_as!(f32);
impl_point_as!(f64);
impl_point_as!(isize);
impl_point_as!(i8);
impl_point_as!(i16);
impl_point_as!(i32);
impl_point_as!(i64);
impl_point_as!(i128);

macro_rules! impl_point2_as {
    ($pt:ty) => {
        impl Point for ($pt, $pt) {
            fn x(&self) -> f32 {
                self.0 as f32
            }

            fn y(&self) -> f32 {
                self.1 as f32
            }
        }
    };
}

impl_point2_as!(f32);
impl_point2_as!(f64);
impl_point2_as!(isize);
impl_point2_as!(i8);
impl_point2_as!(i16);
impl_point2_as!(i32);
impl_point2_as!(i64);
impl_point2_as!(i128);

macro_rules! impl_point_arr_as {
    ($pt:ty) => {
        impl Point for [$pt; 2] {
            fn x(&self) -> f32 {
                self[0] as f32
            }

            fn y(&self) -> f32 {
                self[1] as f32
            }
        }
    };
}

impl_point_arr_as!(f32);
impl_point_arr_as!(f64);
impl_point_arr_as!(isize);
impl_point_arr_as!(i8);
impl_point_arr_as!(i16);
impl_point_arr_as!(i32);
impl_point_arr_as!(i64);
impl_point_arr_as!(i128);
