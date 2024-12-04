use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

impl P {
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
        Self::down() + Self::left()
    }

    pub fn down_right() -> Self {
        Self::down() + Self::right()
    }

    pub fn up_left() -> Self {
        Self::up() + Self::left()
    }

    pub fn up_right() -> Self {
        Self::up() + Self::right()
    }
}

impl ops::Add for P {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i32> for P {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<P> for i32 {
    type Output = P;

    fn mul(self, rhs: P) -> Self::Output {
        rhs * self
    }
}
