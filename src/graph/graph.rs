use std::fmt;
use crate::matrix::matrix::Matrix;

pub struct Graph {
    adj_matrix : Matrix<i32>
}

impl Graph {
    pub fn new(adj_matrix : Matrix<i32>) -> Self {
        Graph{ adj_matrix }
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph V:{}", self.adj_matrix.cols())?;
        print!("{:?}", &self.adj_matrix);
        Ok(())
    }
}