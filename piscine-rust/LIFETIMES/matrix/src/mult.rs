#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

use std::ops::Mul;

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|x| x[n].clone()).collect()
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::AddAssign
        + std::ops::MulAssign
        + std::ops::AddAssign,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = vec![];
        for i in 0..self.number_of_rows() {
            let mut row = vec![];
            for j in 0..other.number_of_cols() {
                let sum = self
                    .row(i)
                    .iter()
                    .zip(other.col(j).iter())
                    .fold(T::default(), |acc, (a, b)| acc + a.clone() * b.clone());
                row.push(sum);
            }
            result.push(row);
        }

        Some(Matrix(result))
    }
}
