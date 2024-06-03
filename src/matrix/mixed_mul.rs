use crate::matrix::matrix::Matrix;
use crate::matrix::naive_mul::naive_mul;
use crate::matrix::generic_strassen::generic_strassen;


pub fn mixed_mul(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    assert_eq!(a.cols(), b.rows(), "Matrix dimensions mismatch");
    assert_eq!(a.cols(), a.rows(), "Only square matrices are supported");
    assert_eq!(b.cols(), b.rows(), "Only square matrices are supported");

    return mixed_mul_impl(a, b);
}


fn mixed_mul_impl(a: &Matrix<i32>, b: &Matrix<i32>) -> Matrix<i32> {
    if a.cols() <= 32 {
        return naive_mul(&a, &b);
    }
    
    return generic_strassen(&a, &b, &mixed_mul_impl);
}
