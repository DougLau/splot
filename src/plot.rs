pub(crate) mod private {
    use crate::page::Rect;
    use std::fmt;

    pub trait SealedPlot {
        fn display(
            &self,
            f: &mut fmt::Formatter,
            num: usize,
            rect: Rect,
        ) -> fmt::Result;
    }
}

pub trait Plot: private::SealedPlot {}
