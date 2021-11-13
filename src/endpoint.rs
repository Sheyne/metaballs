use core::ops::{Add, Div, Mul, Rem, Sub};
use num::Num;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct F32(pub f32);

impl Num for F32 {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(f32::from_str_radix(str, radix)?))
    }
}

impl num::Zero for F32 {
    fn zero() -> Self {
        F32(0f32)
    }

    fn is_zero(&self) -> bool {
        self.0 < f32::EPSILON && self.0 > -f32::EPSILON
    }
}

impl num::One for F32 {
    fn one() -> Self {
        F32(1f32)
    }
}

impl From<usize> for F32 {
    fn from(x: usize) -> Self {
        F32(x as f32)
    }
}
impl From<F32> for f32 {
    fn from(x: F32) -> Self {
        x.0
    }
}
impl Add<F32> for F32 {
    type Output = F32;

    fn add(self, rhs: F32) -> Self::Output {
        F32(self.0 + rhs.0)
    }
}
impl Mul<F32> for F32 {
    type Output = F32;

    fn mul(self, rhs: F32) -> Self::Output {
        F32(self.0 * rhs.0)
    }
}
impl Div<F32> for F32 {
    type Output = F32;

    fn div(self, rhs: F32) -> Self::Output {
        F32(self.0 / rhs.0)
    }
}
impl Sub<F32> for F32 {
    type Output = F32;

    fn sub(self, rhs: F32) -> Self::Output {
        F32(self.0 - rhs.0)
    }
}
impl Rem<F32> for F32 {
    type Output = F32;

    fn rem(self, rhs: F32) -> Self::Output {
        F32(self.0 % rhs.0)
    }
}
