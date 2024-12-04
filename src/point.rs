#[derive(Debug, Clone)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

impl P {
    pub fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn mul(self, mul: i32) -> Self {
        Self {
            x: self.x * mul,
            y: self.y * mul,
        }
    }

    pub fn pair(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::pair(0, 0)
    }

    pub fn one() -> Self {
        Self::pair(1, 1)
    }

    pub fn up() -> Self {
        Self::pair(0, 1)
    }

    pub fn down() -> Self {
        Self::pair(0, -1)
    }

    pub fn left() -> Self {
        Self::pair(-1, 0)
    }

    pub fn right() -> Self {
        Self::pair(1, 0)
    }

    pub fn down_left() -> Self {
        Self::down().add(&Self::left())
    }

    pub fn down_right() -> Self {
        Self::down().add(&Self::right())
    }

    pub fn up_left() -> Self {
        Self::up().add(&Self::left())
    }

    pub fn up_right() -> Self {
        Self::up().add(&Self::right())
    }
}
