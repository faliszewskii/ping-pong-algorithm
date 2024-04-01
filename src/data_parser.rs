use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::graph::graph::Graph;
use crate::matrix::matrix::Matrix;

pub struct DataParser {
}

impl DataParser {

    fn add_graph(graphs: &mut Vec<Graph>, matrix_rows: &Vec<i32>, rows: usize) {
        let cols = matrix_rows.len() / rows;

        let data: Vec<_> = (0..cols)
            .map(|col| matrix_rows
                .iter()
                .skip(col)
                .step_by(rows)
                .cloned()
                .collect::<Vec<_>>())
            .flatten()
            .collect();

        graphs.push(Graph::new(Matrix::with_flat_data(cols, data)));
    }

    pub fn parse_graph_input(input_file: &str) -> Result<Vec<Graph>, io::Error> {
        let file = match File::open(input_file) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let reader = BufReader::new(file);

        let mut graphs = Vec::new();

        let mut matrix_rows = Vec::new();
        let mut rows = 0;

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.eq("") {
                    Self::add_graph(&mut graphs, &matrix_rows, rows);
                    matrix_rows = Vec::new();
                    rows = 0;
                    continue;
                }
                let row: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                matrix_rows.extend_from_slice(&row);
                rows += 1;
            }
        }
        if rows != 0 {
            Self::add_graph(&mut graphs, &matrix_rows, rows);
        }
        Ok(graphs)
    }
}