use crate::matrix::matrix::Matrix;
use crate::matrix::naive_mul::naive_mul;


pub fn strassen_mul(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    assert_eq!(a.cols(), b.rows(), "Matrix dimensions mismatch");
    assert_eq!(a.cols(), a.rows(), "Only square matrices are supported");
    assert_eq!(b.cols(), b.rows(), "Only square matrices are supported");

    if a.cols() <= 512 {
        return naive_mul(&a, &b);
    }
    
    return strassen_mul_impl(&a, &b);
}


fn strassen_mul_impl(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    let (a11, a12, a21, a22);
    let (b11, b12, b21, b22);

    (a11, a12, a21, a22) = explode_matrix_to_4(&a);
    (b11, b12, b21, b22) = explode_matrix_to_4(&b);

    let mut tmp1 = Matrix::new(a11.cols(), a11.rows());
    let mut tmp2 = Matrix::new(a11.cols(), a11.rows());

    Matrix::sub(&a21, &a22, &mut tmp1);
    Matrix::add(&b12, &b22, &mut tmp2);
    let s1 = strassen_mul(&tmp1, &tmp2);

    Matrix::add(&a11, &a22, &mut tmp1);
    Matrix::add(&b11, &b22, &mut tmp2);
    let s2 = strassen_mul(&tmp1, &tmp2);

    Matrix::sub(&a11, &a12, &mut tmp1);
    Matrix::add(&b11, &b21, &mut tmp2);
    let s3 = strassen_mul(&tmp1, &tmp2);

    Matrix::add(&a11, &a21, &mut tmp1);
    let s4 = strassen_mul(&tmp1, &b22);

    Matrix::sub(&b21, &b22, &mut tmp2);
    let s5 = strassen_mul(&a11, &tmp2);

    Matrix::sub(&b12, &b11, &mut tmp2);
    let s6 = strassen_mul(&a22, &tmp2);

    Matrix::add(&a12, &a22, &mut tmp1);
    let s7 = strassen_mul(&tmp1, &b11);

    let result = connect_4_matrices(
        &(&(&s1 + &s2) - &(&s4 - &s6)),
        &(&s6 + &s7),
        &(&s4 + &s5),
        &(&(&s2 - &s3) + &(&s5 - &s7))
    );

    if a.cols() % 2 != 0 {
        return compress_matrix(&result);
    }

    return result;
}


fn compress_matrix(m: &Matrix<i32>) -> Matrix<i32> {
    let mut result = Matrix::new(m.cols() - 1, m.rows() - 1);

    // Copying data from input matrix
    for col in 0..result.cols() {
        for row in 0..result.rows() {
            result[col][row] = m[col][row];
        }
    }

    result
}


fn explode_matrix_to_4(m: &Matrix<i32>) -> (Matrix<i32>, Matrix<i32>, Matrix<i32>, Matrix<i32>) {
    let (mut m11, mut m12, mut m21, mut m22);

    if m.cols() % 2 != 0 {
        let size = (m.cols() + 1)/2;

        m11 = Matrix::new(size, size);
        m12 = Matrix::new(size, size);
        m21 = Matrix::new(size, size);
        m22 = Matrix::new(size, size);
    }
    else {
        let size = m.cols()/2;

        m11 = Matrix::new(size, size);
        m12 = Matrix::new(size, size);
        m21 = Matrix::new(size, size);
        m22 = Matrix::new(size, size);
    }

    let ceil = (m.cols() as f32 / 2.0_f32).ceil() as usize;
    let floor = m.cols() / 2;

    for col in 0..m11.cols() {
        for row in 0..m11.rows() {
            m11[col][row] = m[col][row];
        }
    }

    for col in 0..m12.cols() {
        for row in 0..floor {
            m12[col][row] = m[col][row + ceil];
        }
    }

    for col in 0..floor {
        for row in 0..m21.rows() {
            m21[col][row] = m[col + ceil][row];
        }
    }

    for col in 0..floor {
        for row in 0..floor {
            m22[col][row] = m[col + ceil][row + ceil];
        }
    }

    (m11, m12, m21, m22)
}


fn connect_4_matrices(m11: &Matrix<i32>, m12: &Matrix<i32>, m21: &Matrix<i32>, m22: &Matrix<i32>) -> Matrix<i32> {
    assert_eq!(m11.rows(), m21.rows(), "Invalid matrices sizes");
    assert_eq!(m11.cols(), m12.cols(), "Invalid matrices sizes");
    assert_eq!(m22.rows(), m12.rows(), "Invalid matrices sizes");
    assert_eq!(m22.cols(), m21.cols(), "Invalid matrices sizes");

    let mut result = Matrix::new(m11.cols() + m21.cols(), m11.rows() + m12.rows());

    for col in 0..m11.cols() {
        for row in 0..m11.rows() {
            result[col][row] = m11[col][row];
        }
    }

    for col in 0..m12.cols() {
        for row in 0..m12.rows() {
            result[col][row + m11.rows()] = m12[col][row];
        }
    }

    for col in 0..m21.cols() {
        for row in 0..m21.rows() {
            result[col + m11.cols()][row] = m21[col][row];
        }
    }

    for col in 0..m22.cols() {
        for row in 0..m22.rows() {
            result[col + m11.cols()][row + m11.rows()] = m22[col][row];
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::strassen_mul::{
        strassen_mul,
        connect_4_matrices,
        explode_matrix_to_4
    };
    use crate::matrix::naive_mul::naive_mul;


    #[test]
    pub fn correct_mul_mat_size_4() {
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


    #[test]
    pub fn correct_connecting_4_matrices() {
        let m11 = Matrix::with_data(vec![vec![1, 2],  vec![3, 4]]);
        let m12 = Matrix::with_data(vec![vec![5, 6],  vec![7, 8]]);
        let m21 = Matrix::with_data(vec![vec![9, 10], vec![11, 12]]);
        let m22 = Matrix::with_data(vec![vec![13, 14],vec![15, 16]]);

        let expected = Matrix::with_data(vec![
            vec![ 1,  2,  5,  6],
            vec![ 3,  4,  7,  8],
            vec![ 9, 10, 13, 14],
            vec![11, 12, 15, 16]
        ]);

        let m = connect_4_matrices(&m11, &m12, &m21, &m22);

        assert_eq!(expected, m);
    }


    #[test]
    pub fn correct_exploding_matrices_to_4() {
        let m = Matrix::with_data(vec![
            vec![ 1,  2,  5,  6],
            vec![ 3,  4,  7,  8],
            vec![ 9, 10, 13, 14],
            vec![11, 12, 15, 16]
        ]);

        let expected11 = Matrix::with_data(vec![vec![1, 2],  vec![3, 4]]);
        let expected12 = Matrix::with_data(vec![vec![5, 6],  vec![7, 8]]);
        let expected21 = Matrix::with_data(vec![vec![9, 10], vec![11, 12]]);
        let expected22 = Matrix::with_data(vec![vec![13, 14],vec![15, 16]]);

        let (m11, m12, m21, m22) = explode_matrix_to_4(&m);

        assert_eq!(expected11, m11);
        assert_eq!(expected12, m12);
        assert_eq!(expected21, m21);
        assert_eq!(expected22, m22);
    }


    #[test]
    pub fn correct_exploding_matrices_to_4_odd_columns_cnt() {
        let m = Matrix::with_data(vec![
            vec![ 1,  2,  5],
            vec![ 3,  4,  7],
            vec![ 9, 10, 13],
        ]);

        let expected11 = Matrix::with_data(vec![vec![1, 2],  vec![3, 4]]);
        let expected12 = Matrix::with_data(vec![vec![5, 0],  vec![7, 0]]);
        let expected21 = Matrix::with_data(vec![vec![9, 10], vec![0, 0]]);
        let expected22 = Matrix::with_data(vec![vec![13, 0],vec![0, 0]]);

        let (m11, m12, m21, m22) = explode_matrix_to_4(&m);

        assert_eq!(expected11, m11);
        assert_eq!(expected12, m12);
        assert_eq!(expected21, m21);
        assert_eq!(expected22, m22);
    }

    #[test]
    pub fn exploding_and_connecting_not_changes_matrix() {
        let m1 = Matrix::with_data(vec![
            vec![ 1,  2,  5,  6],
            vec![ 3,  4,  7,  8],
            vec![ 9, 10, 13, 14],
            vec![11, 12, 15, 16]
        ]);

        let (m11, m12, m21, m22) = explode_matrix_to_4(&m1);
        let m2 = connect_4_matrices(&m11, &m12, &m21, &m22);

        assert_eq!(m1, m2)
    }
}
