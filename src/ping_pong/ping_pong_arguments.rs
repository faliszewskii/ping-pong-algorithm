
pub struct PingPongArguments {
    pub input_file: String,
    pub output_file: Option<String>,
}

impl PingPongArguments {
    pub fn new(input_file: &str, output_file: Option<String>) -> Self {
        PingPongArguments {
            input_file: input_file.to_string(),
            output_file: output_file.clone()
        }
    }
}