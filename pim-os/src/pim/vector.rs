use core::fmt::{Debug, Display};

use half::f16;

pub const ELEMENT_COUNT: usize = 16;

#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct F16x1(pub f16);

impl core::fmt::Debug for F16x1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl core::fmt::Display for F16x1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl num_traits::identities::Zero for F16x1 {
    fn zero() -> Self {
        Self(f16::ZERO)
    }

    fn is_zero(&self) -> bool {
        self.0 == f16::ZERO
    }
}

impl num_traits::identities::One for F16x1 {
    fn one() -> Self {
        Self(f16::ONE)
    }
}

impl core::ops::Add<F16x1> for F16x1 {
    type Output = Self;

    fn add(self, rhs: F16x1) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl core::ops::AddAssign<F16x1> for F16x1 {
    fn add_assign(&mut self, rhs: F16x1) {
        self.0 += rhs.0;
    }
}

impl core::ops::Mul<F16x1> for F16x1 {
    type Output = Self;

    fn mul(self, rhs: F16x1) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl core::ops::MulAssign<F16x1> for F16x1 {
    fn mul_assign(&mut self, rhs: F16x1) {
        self.0 *= rhs.0;
    }
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct F16x16(pub [F16x1; ELEMENT_COUNT]);

impl Display for F16x16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl num_traits::identities::Zero for F16x16 {
    fn zero() -> Self {
        Self([F16x1::zero(); ELEMENT_COUNT])
    }

    fn is_zero(&self) -> bool {
        self.0 == [F16x1::zero(); ELEMENT_COUNT]
    }
}

impl num_traits::identities::One for F16x16 {
    fn one() -> Self {
        Self([F16x1::one(); ELEMENT_COUNT])
    }
}

impl core::ops::Add<F16x16> for F16x16 {
    type Output = Self;

    fn add(self, rhs: F16x16) -> Self::Output {
        Self(core::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl core::ops::AddAssign<F16x16> for F16x16 {
    fn add_assign(&mut self, rhs: F16x16) {
        self.0
            .iter_mut()
            .zip(&rhs.0)
            .for_each(|(left, right)| *left += *right);
    }
}

impl core::ops::Mul<F16x16> for F16x16 {
    type Output = Self;

    fn mul(self, rhs: F16x16) -> Self::Output {
        Self(core::array::from_fn(|i| self.0[i] * rhs.0[i]))
    }
}

impl core::ops::MulAssign<F16x16> for F16x16 {
    fn mul_assign(&mut self, rhs: F16x16) {
        self.0
            .iter_mut()
            .zip(&rhs.0)
            .for_each(|(left, right)| *left *= *right);
    }
}
