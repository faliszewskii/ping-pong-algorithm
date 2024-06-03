use crate::matrix::matrix::Matrix;
use crate::matrix::generic_strassen::generic_strassen;


pub fn strassen_mul(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    assert_eq!(a.cols(), b.rows(), "Matrix dimensions mismatch");
    assert_eq!(a.cols(), a.rows(), "Only square matrices are supported");
    assert_eq!(b.cols(), b.rows(), "Only square matrices are supported");

    return strassen_mul_impl(a, b);
}


fn strassen_mul_impl(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    if a.cols() == 1 {
        let mut result =  Matrix::new(1, 1);
        result[0][0] = a[0][0] * b[0][0];

        return result;
    }

    return generic_strassen(a, b, &strassen_mul_impl);
}


#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::strassen_mul::strassen_mul;


    #[test]
    pub fn correct_mul_mat_size_3() {
        let rows = 3;
        let cols = 3;
        let m1: Matrix<i32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());
        let m2: Matrix<i32> = Matrix::with_flat_data(rows as usize, (1..=rows*cols).rev().collect());

        let expected = Matrix::with_data(vec![
            vec![90, 114, 138],
            vec![54, 69, 84],
            vec![18, 24, 30],
        ]);

        assert_eq!(expected, strassen_mul(&m1, &m2));
    }


    #[test]
    pub fn correct_mul_mat_size_2() {
        let rows = 2;
        let cols = 2;
        let m1: Matrix<i32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());
        let m2: Matrix<i32> = Matrix::with_flat_data(rows as usize, (1..=rows*cols).rev().collect());

        let expected = Matrix::with_data(vec![vec![13, 20], vec![5, 8]]);

        assert_eq!(expected, strassen_mul(&m1, &m2));
    }


    #[test]
    pub fn mut_returns_matrix_with_same_size_as_arguments() {
        for size in 1..=100 {
            let m1 = Matrix::new(size, size);
            let m2 = Matrix::new(size, size);

            let mul = strassen_mul(&m1, &m2);

            assert_eq!(mul.cols(), size);
            assert_eq!(mul.rows(), size);
        }
    }
}
