use std::fs::File;
use std::io::Write;
use std::process::exit;

use crate::console::parse_console_arguments;
use crate::console_arguments::{ConsoleArguments, MultiplicationMethod};
use crate::data_parser::DataParser;
use crate::graph::generator::generate_ping_pong;
use crate::graph::graph::Graph;
use crate::matrix::naive_mul::naive_mul;
use crate::matrix::strassen_mul::strassen_mul;
use crate::ping_pong::ping_pong_solver::PingPongSolver;

mod data_parser;
mod graph;
mod matrix;
mod ping_pong;
mod console;
mod console_arguments;

fn main() {
    let arguments = parse_console_arguments();

    match arguments {
        ConsoleArguments::Solve(solve_args) => {

            let graphs: Vec<_> = solve_args.input_files.iter().map(|input| {
                match DataParser::parse_graph_input(&input) {
                    Ok(graphs) => graphs,
                    Err(e) => {
                        eprintln!("IO error: {}", e);
                        exit(1);
                    }
                }}).flatten().collect();

            let solver = PingPongSolver::new(match solve_args.mul_method {
                MultiplicationMethod::Naive => naive_mul,
                MultiplicationMethod::Strassen => strassen_mul
            });

            let results: Vec<_> = graphs.iter()
                .map(|g| {
                    if solve_args.verbose { print!("{:}", g); }
                    let result = solver.solve(&g);
                    if solve_args.verbose || solve_args.output_file.is_none() {
                        println!("{:?}", result.iter().map(|i| i+1).collect::<Vec<_>>());
                        if solve_args.verbose { println!() }
                    }
                    result
                })
                .collect();

            match solve_args.output_file {
                None => {}
                Some(output) => {
                    save_results_to_file(results, output);
                }
            }
        }
        ConsoleArguments::Generate(generate_args) => {
            let graphs = generate_args.sizes.iter().map(|size| generate_ping_pong(*size, 0.01)).collect();
            save_graphs_to_file(graphs, generate_args.output_file);
        }
    }
}

fn save_graphs_to_file(graphs: Vec<Graph>, output: String) {
    let file = match File::create(output) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error creating file");
            return;
        }
    };

    for (i, graph) in graphs.iter().enumerate() {
        for row in 0..graph.adj_matrix.rows() {
            for col in 0..graph.adj_matrix.cols() {
                write!(&file, "{:>2} ", graph.adj_matrix[col][row]).expect("Failed to write");
            }
            writeln!(&file).expect("Failed to write");
        }
        if i != graphs.len()-1 { writeln!(&file).expect("Failed to write") };
    }
}

fn save_results_to_file(results: Vec<Vec<i32>>, output: String) {
    let formatted_string: String = results
        .iter()
        .map(|inner_vec| inner_vec.iter().map(|i|i+1).map(|arg0: i32| ToString::to_string(&arg0)).collect::<Vec<String>>().join(" "))
        .chain(vec![String::new()])
        .collect::<Vec<String>>()
        .join("\n");

    // Write the string to a file
    if let Ok(mut file) = File::create(output) {
        if let Err(err) = file.write_all(formatted_string.as_bytes()) {
            eprintln!("Error writing to file: {}", err);
        }
    } else {
        eprintln!("Error creating file");
    }
}
