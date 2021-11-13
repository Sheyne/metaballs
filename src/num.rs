use core::ops::{Add, Div, Mul};

#[derive(Clone, Copy, Debug)]
pub struct F32(pub f32);

impl From<usize> for F32 {
    fn from(x: usize) -> Self {
        F32(x as f32)
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
