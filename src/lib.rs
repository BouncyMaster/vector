use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize>([f32; N]);

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

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[buil]
    fn diff_size() {
        let v1 = Vector([1f32, 2f32]);
        let v2 = Vector([3f32, 4f32, 5f32]);

        let _ = v1 + v2;
    }
}
*/
