use getopts::Options;

use crate::ping_pong::ping_pong_arguments::PingPongArguments;

pub struct ConsoleInterface {
    options: Options
}

const PROGRAM_NAME: &'static str = "Ping-Pong solver";
const PROGRAM_DESC: &'static str = "Algorithm solving the Ping-Pong problem";
impl ConsoleInterface {
    pub fn new() -> Self {
        let mut opts = Options::new();
        opts.optopt("i", "input", "set input file name", "INPUT");
        opts.optopt("o", "output", "set output file name", "OUTPUT");
        opts.optflag("h", "help", "print this help menu");

        return ConsoleInterface{options: opts}
    }

    fn print_usage(program: &str, opts: &Options) {
        let brief = format!("{}\n{}\nUsage: {} [options]", PROGRAM_NAME, PROGRAM_DESC, program);
        print!("{}", opts.usage(&brief));
    }
    pub fn parse_arguments(&mut self, input: Vec<String>) -> Option<PingPongArguments> {
        let program = input[0].clone();
        let matches = match self.options.parse(&input[1..]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Parsing error: {}", e);
                ConsoleInterface::print_usage(&program, &self.options);
                return None;
            }
        };

        if matches.opt_present("h") {
            ConsoleInterface::print_usage(&program, &self.options);
            return None;
        }

        let input_file = match matches.opt_str("i") {
            Some(file) => file,
            None => {
                eprintln!("Input file not specified");
                ConsoleInterface::print_usage(&program, &self.options);
                return None;
            }
        };

        let output_file = match matches.opt_str("o") {
            Some(file) => file,
            None => {
                return Some(PingPongArguments::new(input_file.as_str(), None))
            }
        };

        Some(PingPongArguments::new(input_file.as_str(), Some(output_file)))
    }
}