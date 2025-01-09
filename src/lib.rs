use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{Signed, NumCast};

pub trait SignedUnified: Signed + Copy + NumCast {}
impl<T> SignedUnified for T where T: Signed + Copy + NumCast {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<const N: usize, T: SignedUnified>(pub [T; N]);

impl<const N: usize, T: SignedUnified> Vector<N, T> {
    pub fn new() -> Vector<N, T> {
        Vector([num::zero(); N])
    }
}

impl<const N: usize, T: SignedUnified> Default for Vector<N, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> Add<Vector<N, U>> for Vector<N, T> {
    type Output = Self;

    fn add(self, rhs: Vector<N, U>) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] + NumCast::from(rhs.0[i]).unwrap());
        Vector(result)
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> Add<U> for Vector<N, T> {
    type Output = Self;

    /// Implement scalar addition
    fn add(self, rhs: U) -> Self::Output {
        let cast = NumCast::from(rhs).unwrap();
        let result = std::array::from_fn(|i| self.0[i] + cast);
        Vector(result)
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> AddAssign<Vector<N, U>> for Vector<N, T> {
    fn add_assign(&mut self, rhs: Vector<N, U>) {
        for i in 0..N {
            self.0[i] = self.0[i] + NumCast::from(rhs.0[i]).unwrap();
        }
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> AddAssign<U> for Vector<N, T> {
    /// Implement scalar addition
    fn add_assign(&mut self, rhs: U) {
        let cast = NumCast::from(rhs).unwrap();
        for i in 0..N {
            self.0[i] = self.0[i] + cast;
        }
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> Sub<Vector<N, U>> for Vector<N, T> {
    type Output = Self;

    fn sub(self, rhs: Vector<N, U>) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] - NumCast::from(rhs.0[i]).unwrap());
        Vector(result)
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> Sub<U> for Vector<N, T> {
    type Output = Self;

    fn sub(self, rhs: U) -> Self::Output {
        let cast = NumCast::from(rhs).unwrap();
        let result = std::array::from_fn(|i| self.0[i] - cast);
        Vector(result)
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> SubAssign<Vector<N, U>> for Vector<N, T> {
    fn sub_assign(&mut self, rhs: Vector<N, U>) {
        for i in 0..N {
            self.0[i] = self.0[i] - NumCast::from(rhs.0[i]).unwrap();
        }
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> SubAssign<U> for Vector<N, T> {
    /// Implement scalar addition
    fn sub_assign(&mut self, rhs: U) {
        let cast = NumCast::from(rhs).unwrap();
        for i in 0..N {
            self.0[i] = self.0[i] - cast;
        }
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> Mul<U> for Vector<N, T> {
    type Output = Self;

    /// Implement scalar multiplication
    fn mul(self, rhs: U) -> Self::Output {
        let cast = NumCast::from(rhs).unwrap();
        let result = std::array::from_fn(|i| self.0[i] * cast);
        Vector(result)
    }
}

impl<const N: usize, T: SignedUnified, U: SignedUnified> MulAssign<U> for Vector<N, T> {
    /// Implement scalar multiplication
    fn mul_assign(&mut self, rhs: U) {
        let cast = NumCast::from(rhs).unwrap();
        for i in 0..N {
            self.0[i] = self.0[i] * cast;
        }
    }
}

impl<const N: usize, T: SignedUnified> Neg for Vector<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let result = std::array::from_fn(|i| -self.0[i]);
        Vector(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec() {
        let a = Vector([1, 0]);
        let b = Vector([4, 2]);

        assert_eq!(a + b, Vector([5, 2]))
    }

    #[test]
    fn sub_vec() {
        let a = Vector([1, 0]);
        let b = Vector([4, 2]);

        assert_eq!(a - b, Vector([-3, -2]))
    }

    #[test]
    fn neg() {
        let a = Vector([1, 0]);

        assert_eq!(-a, Vector([-1, 0]))
    }

    #[test]
    fn mul_scalar() {
        let a = Vector([1, 0]);

        assert_eq!(a * 4, Vector([4, 0]))
    }

    #[test]
    fn add_assign_vec() {
        let mut a = Vector([1, 0]);
        let b = Vector([4, 2]);

        a += b;

        assert_eq!(a, Vector([5, 2]))
    }

    #[test]
    fn add_assign_scalar() {
        let mut a = Vector([1, 0]);
        let b = 5;

        a += b;

        assert_eq!(a, Vector([6, 5]))
    }

    #[test]
    fn sub_assign_vec() {
        let mut a = Vector([1, 0]);
        let b = Vector([4, 0]);

        a -= b;

        assert_eq!(a, Vector([-3, 0]))
    }

    #[test]
    fn sub_assign_scalar() {
        let mut a = Vector([1, 0]);
        let b = -5;

        a -= b;

        assert_eq!(a, Vector([6, 5]))
    }

    #[test]
    fn vec_cast() {
        let a = Vector([1, 0]);
        let b = Vector([4.0, 2.3]);

        assert_eq!(a + b, Vector([5, 2]))
    }

    #[test]
    fn scalar_cast() {
        let a = Vector([1, 0]);
        let b = 1.6;

        assert_eq!(a + b, Vector([2, 1]))
    }
}
