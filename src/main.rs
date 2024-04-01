use std::env;
use std::process::exit;

use ping_pong::ping_pong_arguments::PingPongArguments;

use crate::console_interface::ConsoleInterface;
use crate::data_parser::DataParser;

mod console_interface;
mod data_parser;
mod graph;
mod matrix;
mod ping_pong;

fn main() {

    let arguments = read_arguments();

    let graphs = match DataParser::parse_graph_input(&arguments.input_file) {
        Ok(graphs) => graphs,
        Err(e) => {
            eprintln!("IO error: {}", e);
            exit(1);
        }
    };

    for graph in graphs {
        print!("{:?}", graph);
    }

    print!("{}, {:?}", arguments.input_file, arguments.output_file);
}

fn read_arguments() -> PingPongArguments {
    // Read console arguments
    // ping_pong -i example.txt -o result.txt
    let console_args: Vec<String> = env::args().collect();
    let mut console_interface = ConsoleInterface::new();
    match console_interface.parse_arguments(console_args) {
        Some(arguments) => arguments,
        None => exit(0)
    }
}
