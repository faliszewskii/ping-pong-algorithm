use crate::matrix::matrix::Matrix;

pub fn naive_mul(m1: &Matrix<i32>, m2: &Matrix<i32>) -> Matrix<i32> {
    assert_eq!(m1.cols(), m2.rows(), "Matrix dimensions mismatch");
    let dim = m1.cols();
    let mut result = Matrix::new(m2.cols(), m1.rows());

    for col in 0..result.cols() {
        for row in 0..result.rows() {
            for i in 0..dim {
                result[col][row] += m1[i][row] * m2[col][i];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::naive_mul::naive_mul;

    #[test]
    pub fn correct_mul() {
        let rows = 3;
        let cols = 4;
        let m1: Matrix<i32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());
        let m2: Matrix<i32> = Matrix::with_flat_data(rows as usize, (1..=rows*cols).rev().collect());

        let expected = Matrix::with_data(vec![vec![216, 258, 300], vec![128, 154, 180], vec![40, 50, 60]]);

        assert_eq!(expected, naive_mul(&m1, &m2));
    }
}