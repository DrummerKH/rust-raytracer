use std::ops::{Mul, Add};

trait Vector {
    type Type;
    type X = Self::Type;
    type Y = Self::Type;
    type Z = Self::Type;

    // fn unit() -> Self {
    //
    // }

    fn length(&self) -> Self::Type {
        (Self::X*Self.X + Self::Y*Self::Y + Self::Z*Self::Z).sqrt()
    }

}

impl Mul<Self> for dyn Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for dyn Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Add for dyn Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
