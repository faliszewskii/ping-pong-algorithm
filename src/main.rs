use std::env;
use std::process::exit;


use crate::data_parser::DataParser;
use crate::matrix::naive_mul::naive_mul;
use crate::ping_pong::ping_pong_solver::PingPongSolver;
use crate::console::parse_console_arguments;
use crate::console_arguments::ConsoleArguments;

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

            let solver = PingPongSolver::new(naive_mul);

            let results: Vec<_> = graphs.iter()
                .map(|g| {
                    if solve_args.verbose { print!("{:}", g); }
                    let result = solver.solve(&g);
                    if solve_args.verbose || solve_args.output_file.is_none() {
                        println!("{:?}", result);
                        if solve_args.verbose { println!() }
                    }
                    result
                })
                .collect();

            match solve_args.output_file {
                None => {}
                Some(output) => {
                    // TODO Save results to file
                }
            }
        }
        ConsoleArguments::Generate(_) => {
            todo!()
        }
    }
}
