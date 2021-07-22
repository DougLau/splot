pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Point for f32 {
    fn x(&self) -> f32 {
        *self
    }

    fn y(&self) -> f32 {
        *self
    }
}

impl Point for f64 {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for isize {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for i32 {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point for (f32, f32) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point for (f64, f64) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (isize, isize) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i8, i8) {
    fn x(&self) -> f32 {
        self.0.into()
    }

    fn y(&self) -> f32 {
        self.1.into()
    }
}

impl Point for (i16, i16) {
    fn x(&self) -> f32 {
        self.0.into()
    }

    fn y(&self) -> f32 {
        self.1.into()
    }
}

impl Point for (i32, i32) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i64, i64) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (i128, i128) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}
