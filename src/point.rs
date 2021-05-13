pub trait Point<X, Y> {
    fn x(&self) -> X;
    fn y(&self) -> Y;
}

impl Point<f32, f32> for f32 {
    fn x(&self) -> f32 {
        *self
    }

    fn y(&self) -> f32 {
        *self
    }
}

impl Point<f32, f32> for isize {
    fn x(&self) -> f32 {
        *self as f32
    }

    fn y(&self) -> f32 {
        *self as f32
    }
}

impl Point<f32, f32> for (f32, f32) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point<f32, f32> for (isize, isize) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point<f32, f32> for (isize, isize, &str) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point<f32, f32> for (f32, f32, &str) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point<f32, f32> for (isize, isize, String) {
    fn x(&self) -> f32 {
        self.0 as f32
    }

    fn y(&self) -> f32 {
        self.1 as f32
    }
}

impl Point<f32, f32> for (f32, f32, String) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point<String, f32> for (String, f32) {
    fn x(&self) -> String {
        self.0.to_owned()
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Point<f32, String> for (f32, &str) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> String {
        self.1.to_owned()
    }
}

impl Point<f32, String> for (&str, i32) {
    fn x(&self) -> f32 {
        self.1 as f32
    }

    fn y(&self) -> String {
        self.0.to_owned()
    }
}

impl Point<f32, String> for (&str, i32, &str) {
    fn x(&self) -> f32 {
        self.1 as f32
    }

    fn y(&self) -> String {
        self.0.to_owned()
    }
}
