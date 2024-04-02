use clap::{Arg, ArgAction, ArgMatches, Command};
use crate::console_arguments::{ConsoleArguments, GenerateArguments, SolveArguments};
use crate::console_arguments::MultiplicationMethod::{Naive, Strassen};

pub fn parse_console_arguments() -> ConsoleArguments {
    let matches = construct_command().get_matches();

    construct_arguments(matches)
}

fn construct_arguments(matches: ArgMatches) -> ConsoleArguments {
    match matches.subcommand() {
        Some(("solve", opts)) => {

            let input_files: Vec<String> = if opts.contains_id("input") {
                opts
                    .get_many::<String>("input")
                    .expect("contains_id")
                    .map(|s| s.into())
                    .collect()
            } else { unreachable!("Argument is required") };

            let output_file: Option<String> = if opts.contains_id("output") {
                Some(opts
                    .get_one::<String>("output")
                    .expect("contains_id")
                    .into()
                )
            } else {
                None
            };

            let mul_method = if opts.get_flag("naive") { Naive } else { Strassen };

            let verbose = opts.get_flag("verbose");

            ConsoleArguments::Solve(SolveArguments{ input_files, output_file, mul_method, verbose })
        }
        Some(("generate", opts)) => {
            let sizes: Vec<i32> = if opts.contains_id("sizes") {
                opts
                    .get_many::<i32>("sizes")
                    .expect("contains_id")
                    .map(|i| *i)
                    .collect()
            } else { unreachable!("Argument is required") };

            let output_file: String = if opts.contains_id("output") {
                opts
                    .get_one::<String>("output")
                    .expect("contains_id")
                    .into()
            } else { unreachable!("Argument is required") };

            ConsoleArguments::Generate(GenerateArguments{ sizes, output_file })
        }
        _ => unreachable!()
    }
}

fn construct_command() -> Command {
    Command::new("ping-pong")
        .about("Algorithm to solve the ping-pong problem")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Command to generate graphs
        .subcommand(
            Command::new("generate")
                .short_flag('g')
                .long_flag("generate")
                .about("Generate graphs.")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("sizes")
                        .short('s')
                        .long("sizes")
                        .help("sizes of graphs to generate")
                        .value_parser(clap::value_parser!(i32))
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("output file to store the generated graphs to")
                        .action(ArgAction::Set)
                        .num_args(1),
                ),
        )
        // Command to run solver
        .subcommand(
            Command::new("solve")
                .short_flag('s')
                .long_flag("solve")
                .about("Solve the ping-pong problem.")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("input files with one or more graphs to solve"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .action(ArgAction::Set)
                        .help("optional output file to store the solutions in. Solutions are passed to standard output if option not present")
                        .num_args(0..=1),
                )
                .arg(
                    Arg::new("strassen")
                        .long("strassen")
                        .help("Use strassen multiplication method (default)")
                        .conflicts_with("naive")
                        .action(ArgAction::SetTrue)
                        .num_args(0)
                )
                .arg(
                    Arg::new("naive")
                        .long("naive")
                        .help("Use naive multiplication method")
                        .conflicts_with("strassen")
                        .action(ArgAction::SetTrue)
                        .num_args(0)
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Print input graphs and their solutions side by side")
                        .action(ArgAction::SetTrue)
                        .num_args(0),
                ),
        )
}