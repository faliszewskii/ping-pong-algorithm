use crate::graph::graph::Graph;
use crate::matrix::matrix::Matrix;

pub struct PingPongSolver {
    multiplication_algorithm: fn(&Matrix<i32>, &Matrix<i32>) -> Matrix<i32>
}

impl PingPongSolver {

    pub fn new(mul_alg: fn(&Matrix<i32>, &Matrix<i32>) -> Matrix<i32>) -> Self {
        PingPongSolver{ multiplication_algorithm: mul_alg}
    }

    pub fn solve(&self, input: &Graph) -> Vec<i32> {
        let a = &input.adj_matrix;
        let a_sq = (self.multiplication_algorithm)(&a, &a);
        let n = a.rows();
        let mut result = Vec::new();

        // We are looking for rows which have the X property.
        for row in 0..n {
            let mut has_x = true;
            for col in 0..n {
                if col != row && a[col][row] == 0 && a_sq[col][row] == 0 {
                    has_x = false;
                    break;
                }
            }
            if has_x {
                result.push(row as i32);
            }
        }
        result
    }
}
