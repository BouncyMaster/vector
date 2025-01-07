use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>(pub [f32; N]);

impl<const N: usize> Vector<N> {
    pub fn new() -> Vector<N> {
        Vector([0.0; N])
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] + rhs.0[i]);
        Vector(result)
    }
}

impl<const N: usize> Add<f32> for Vector<N> {
    type Output = Self;

    /// Implement scalar addition
    fn add(self, rhs: f32) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] + rhs);
        Vector(result)
    }
}

impl<const N: usize> Add<Vector<N>> for f32 {
    type Output = Vector<N>;

    /// Implement scalar addition
    fn add(self, rhs: Vector<N>) -> Self::Output {
        let result = std::array::from_fn(|i| self + rhs.0[i]);
        Vector(result)
    }
}

impl<const N: usize> AddAssign<Vector<N>> for Vector<N> {
    fn add_assign(&mut self, rhs: Vector<N>) {
        for i in 0..N {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<const N: usize> AddAssign<f32> for Vector<N> {
    /// Implement scalar addition
    fn add_assign(&mut self, rhs: f32) {
        for i in 0..N {
            self.0[i] += rhs;
        }
    }
}

impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] - rhs.0[i]);
        Vector(result)
    }
}

impl<const N: usize> Sub<f32> for Vector<N> {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] - rhs);
        Vector(result)
    }
}

impl<const N: usize> SubAssign<Vector<N>> for Vector<N> {
    fn sub_assign(&mut self, rhs: Vector<N>) {
        for i in 0..N {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<const N: usize> SubAssign<f32> for Vector<N> {
    /// Implement scalar addition
    fn sub_assign(&mut self, rhs: f32) {
        for i in 0..N {
            self.0[i] -= rhs;
        }
    }
}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Self;

    /// Implement scalar multiplication
    fn mul(self, rhs: f32) -> Self::Output {
        let result = std::array::from_fn(|i| self.0[i] * rhs);
        Vector(result)
    }
}

impl<const N: usize> MulAssign<f32> for Vector<N> {
    /// Implement scalar multiplication
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..N {
            self.0[i] *= rhs;
        }
    }
}

impl<const N: usize> Neg for Vector<N> {
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
    fn new() {
        let _ = Vector::<4>::new();
    }

    #[test]
    fn add_vec() {
        let a = Vector([1.0, 0.0]);
        let b = Vector([4.0, 2.0]);

        let _ = a + b;
    }

    #[test]
    fn sub_vec() {
        let a = Vector([1.0, 0.0]);
        let b = Vector([4.0, 2.0]);

        let _ = a - b;
    }

    #[test]
    fn neg() {
        let a = Vector([1.0, 0.0]);

        let _ = -a;
    }
}
