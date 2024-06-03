
#[derive(Debug)]
pub enum MultiplicationMethod  {
    Naive,
    Strassen,
    Mixed
}

#[derive(Debug)]
pub struct SolveArguments {
    pub input_files: Vec<String>,
    pub output_file: Option<String>,
    pub mul_method: MultiplicationMethod,
    pub verbose: bool
}

#[derive(Debug)]
pub struct GenerateArguments {
    pub sizes: Vec<i32>,
    pub output_file: String
}

#[derive(Debug)]
pub enum ConsoleArguments {
    Solve(SolveArguments),
    Generate(GenerateArguments)
}