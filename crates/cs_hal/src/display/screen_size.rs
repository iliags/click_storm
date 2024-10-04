#[derive(Debug, Clone, PartialEq)]
pub struct ScreenSize {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for ScreenSize {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl ScreenSize {
    pub fn center(&self) -> ScreenSize {
        (self.x / 2, self.y / 2).into()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_size() {
        let size = ScreenSize { x: 10, y: 20 };
        assert_eq!(size.x(), 10);
        assert_eq!(size.y(), 20);
        assert_eq!(size.center(), ScreenSize { x: 5, y: 10 });
    }
}
