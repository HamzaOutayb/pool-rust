#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if first == self.r {
            self.r = second
        } else if second == self.r {
            self.r = first
        }
        if first == self.g {
            self.g = second
        } else if second == self.g {
            self.g = first
        }
        if first == self.b {
            self.b = second
        } else if second == self.b {
            self.b = first
        }
        if first == self.a {
            self.a = second
        } else if second == self.a {
            self.a = first
        }
        self
    }
}