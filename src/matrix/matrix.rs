use std::fmt;
use std::ops::{Index, IndexMut, Sub, Add};


#[derive(Clone)]
pub struct Matrix<T: Clone> {
    cols: usize,
    rows: usize,
    // Data is laid out with contiguous columns
    data: Vec<T>
}

impl<T: Clone + PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.cols != other.cols || self.rows != other.rows { return false };
        for row in 0..self.cols {
            for col in 0..self.rows {
                if self[col][row] != other[col][row] { return false };
            }
        }
        true
    }
}

impl<T: Clone> Index<usize> for Matrix<T> {
    type Output = [T];

    // I want the user to be able to reference the memory of the data. So it should be mutable
    fn index(&self, index: usize) -> &Self::Output {

        let cols_i = index*self.rows;
        &self.data[cols_i..cols_i + self.rows]
    }
}

impl<T: Clone> IndexMut<usize> for Matrix<T> {

    // I want the user to be able to reference the memory of the data. So it should be mutable
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let cols_i = index*self.rows;
        &mut self.data[cols_i..cols_i + self.rows]
    }
}

impl<T> Matrix<T> where T: Default, T: Clone {
    pub fn new(cols : usize, rows : usize) -> Self {
        let data = vec![T::default(); cols *rows];
        Matrix{cols, rows, data}
    }

    /// Data is provided as each vector containing a column.
    pub fn with_data(data : Vec<Vec<T>>) -> Self {
        if data.is_empty() || data[0].is_empty() { return Matrix{ cols: 0, rows: 0, data: Vec::new() } };

        let cols = data.len();
        let rows = data[0].len();

        let mut flat_data = Vec::with_capacity(rows * cols);
        for row in &data {
            flat_data.extend_from_slice(row);
        };

        Matrix{cols, rows, data: flat_data}
    }

    pub fn with_flat_data(cols: usize, data : Vec<T>) -> Self {
        assert_eq!(data.len() % cols, 0, "Cols do not divide data equally");
        if data.is_empty() { return Matrix{ cols: 0, rows: 0, data: Vec::new() } };

        let rows = data.len() / cols;

        Matrix{cols, rows, data }
    }


    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}

impl<T: fmt::Debug + Clone > fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix ({}x{}):", self.cols, self.rows)?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:>2?} ", self[col][row])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display + Clone > fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:>2} ", self[col][row])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


impl<T: Sub<Output = T> + Copy + Default> Sub for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.cols, "Different column number");
        assert_eq!(self.rows, rhs.rows, "Different rows number");

        let mut result = Matrix::new(self.cols, self.rows);

        for col in 0..self.cols {
            for row in 0..self.rows {
                result[col][row] = self[col][row] - rhs[col][row];
            }
        }

        result
    }
}


impl<T: Add<Output = T> + Copy + Default> Add for &Matrix<T>
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Matrix<T> {
        assert_eq!(self.cols, rhs.cols, "Different column number");
        assert_eq!(self.rows, rhs.rows, "Different rows number");

        let mut result = Matrix::new(self.cols, self.rows);

        for col in 0..self.cols {
            for row in 0..self.rows {
                result[col][row] = self[col][row] + rhs[col][row];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn zeros_on_new() {
        let rows = 3;
        let cols = 4;
        let m: Matrix<i32> = Matrix::new(cols as usize, rows as usize);

        for row in 0..rows {
            for col in 0..cols {
                assert_eq!(m[col][row], 0)
            }
        }
    }

    #[test]
    fn referencing_row() {
        let rows = 3;
        let cols = 4;
        let m: Matrix<u32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());

        let middle_row: Vec<_> = (5..9).collect();

        assert_eq!(middle_row, &m[1]);
    }

    #[test]
    fn referencing_element() {
        let rows = 3;
        let cols = 4;
        let m: Matrix<i32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());

        let middle_element = 7;

        assert_eq!(middle_element, m[1][2]);
    }

    #[test]
    fn mutate_element() {
        let rows = 3;
        let cols = 4;
        let mut m: Matrix<i32> = Matrix::with_flat_data(cols as usize, (1..=rows*cols).collect());

        let middle_element = 7;
        assert_eq!(middle_element, m[1][2]);

        let new_element = 99;
        m[1][2] = new_element;
        assert_eq!(new_element, m[1][2]);
    }
}
