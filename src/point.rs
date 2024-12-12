use std::ops;

#[allow(dead_code)]
pub const DIRECTIONS_4: [P; 4] = [P::up(), P::right(), P::down(), P::left()];
#[allow(dead_code)]
pub const DIRECTIONS_8: [P; 8] = [P::up(), P::up_right(), P::right(), P::down_right(), P::down(), P::down_left(), P::left(), P::up_left()];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

impl P {
    pub const fn pair(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self::pair(0, 0)
    }

    pub const fn one() -> Self {
        Self::pair(1, 1)
    }

    pub const fn up() -> Self {
        Self::pair(0, 1)
    }

    pub const fn down() -> Self {
        Self::pair(0, -1)
    }

    pub const fn left() -> Self {
        Self::pair(-1, 0)
    }

    pub const fn right() -> Self {
        Self::pair(1, 0)
    }

    pub const fn down_left() -> Self {
        Self::pair(-1, -1)
    }

    pub const fn down_right() -> Self {
        Self::pair(1, -1)
    }

    pub const fn up_left() -> Self {
        Self::pair(-1, 1)
    }

    pub const fn up_right() -> Self {
        Self::pair(1, 1)
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

impl ops::Sub for P {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

impl ops::Div<i32> for P {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
