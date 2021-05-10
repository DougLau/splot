use std::fmt;

#[derive(Clone, Copy)]
enum Edge {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Clone)]
struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    fn inset(mut self, value: u16) -> Self {
        let vi = i32::from(value);
        self.x += vi;
        self.y += vi;
        let v2 = 2 * u32::from(value);
        self.width = self.width.saturating_sub(v2);
        self.height = self.height.saturating_sub(v2);
        self
    }
    fn split(&mut self, edge: Edge, value: u16) -> Self {
        let v = u32::from(value);
        match edge {
            Edge::Top => {
                let y = self.y;
                let height = self.height.saturating_sub(v);
                let h = self.height - height;
                self.y += h as i32;
                self.height = height;
                Rect::new(self.x, y, self.width, h)
            }
            Edge::Left => {
                let x = self.x;
                let width = self.width.saturating_sub(v);
                let w = self.width - width;
                self.x += w as i32;
                self.width = width;
                Rect::new(x, self.y, w, self.height)
            }
            Edge::Bottom => {
                let height = self.height.saturating_sub(v);
                let h = self.height - height;
                let y = self.y + height as i32;
                self.height = height;
                Rect::new(self.x, y, self.width, h)
            }
            Edge::Right => {
                let width = self.width.saturating_sub(v);
                let w = self.width - width;
                let x = self.x + width as i32;
                self.width = width;
                Rect::new(x, self.y, w, self.height)
            }
        }
    }
    fn transform(&self, f: &mut fmt::Formatter, edge: Edge) -> fmt::Result {
        let x = self.x + self.width as i32 / 2;
        let y = self.y + self.height as i32 / 2;
        write!(f," transform='translate({} {})", x, y)?;
        match edge {
            Edge::Left => write!(f, " rotate(-90)")?,
            Edge::Right => write!(f, " rotate(90)")?,
            _ => (),
        }
        write!(f, "'")
    }
}

#[derive(Clone, Copy)]
pub enum AspectRatio {
    Landscape,
    Square,
    Portrait,
}

impl AspectRatio {
    fn rect(self) -> Rect {
        match self {
            AspectRatio::Landscape => Rect::new(0, 0, 1000, 750),
            AspectRatio::Square => Rect::new(0, 0, 1000, 1000),
            AspectRatio::Portrait => Rect::new(0, 0, 750, 1000),
        }
    }
}

pub struct Title {
    text: String,
    edge: Edge,
}

impl From<&str> for Title {
    fn from(text: &str) -> Self {
        Title::new(text)
    }
}

impl Title {
    pub fn new(text: &str) -> Self {
        Title {
            text: text.to_owned(),
            edge: Edge::Top,
        }
    }

    pub fn on_bottom(mut self) -> Self {
        self.edge = Edge::Bottom;
        self
    }

    pub fn on_left(mut self) -> Self {
        self.edge = Edge::Left;
        self
    }

    pub fn on_right(mut self) -> Self {
        self.edge = Edge::Right;
        self
    }

    fn display(&self, f: &mut fmt::Formatter, rect: Rect) -> fmt::Result {
        write!(f, "<text")?;
        rect.transform(f, self.edge)?;
        write!(f, ">")?;
        write!(f, "{}", self.text)?;
        writeln!(f, "</text>")
    }
}

pub struct ChartBuilder {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
}

pub struct Chart {
    aspect_ratio: AspectRatio,
    titles: Vec<Title>,
}

impl Default for ChartBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: AspectRatio::Landscape,
            titles: vec![],
        }
    }
}

impl ChartBuilder {
    pub fn aspect_ratio(mut self, aspect: AspectRatio) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title>,
    {
        self.titles.push(title.into());
        self
    }

    pub fn build(self) -> Chart {
        Chart {
            aspect_ratio: self.aspect_ratio,
            titles: self.titles,
        }
    }
}

impl Chart {
    pub fn builder() -> ChartBuilder {
        ChartBuilder::default()
    }

    fn header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rect = self.aspect_ratio.rect();
        writeln!(
            f,
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            rect.x,
            rect.y,
            rect.width,
            rect.height,
        )?;
        Ok(())
    }

    fn footer(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "</svg>")
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.header(f)?;
        let mut area = self.aspect_ratio.rect().inset(10);
        for title in &self.titles {
            let rect = area.split(title.edge, 25);
            title.display(f, rect)?;
        }
        self.footer(f)?;
        Ok(())
    }
}
