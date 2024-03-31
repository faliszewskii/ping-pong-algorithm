mod args_parser;
mod ping_pong_arguments;

use std::env;
use std::process::exit;
use crate::args_parser::{ConsoleInterface};

fn main() {

    /// Read console arguments
    // ping-pong -i example.txt -o result.txt
    let console_args: Vec<String> = env::args().collect();
    let mut console_interface = ConsoleInterface::new();
    let arguments = match console_interface.parse_arguments(console_args) {
        Some(arguments) => arguments,
        None => exit(0)
    };

    print!("{}, {}", arguments.input_file, arguments.output_file);
}
