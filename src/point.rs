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

impl Point for isize {
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

impl Point for (isize, isize) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (isize, isize, &str) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (f32, f32, &str) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point for (isize, isize, String) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (f32, f32, String) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point for (String, f32) {
    fn x(&self) -> f32 {
        self.1
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point for (&str, i32) {
    fn x(&self) -> f32 {
        self.1 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point for (f32, &str) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.0
    }
}
