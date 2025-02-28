use std::fs;
use std::io::{BufWriter, Write};

/// Creates a buffered writer for either stdout or a file
///
/// # Arguments
///
/// * `output` - Output target, "stdout" for standard output or a file path
///
/// # Returns
///
/// A boxed writer implementing the Write trait
pub fn writer(output: &str) -> Box<dyn Write> {
    let writer: Box<dyn Write> = if output == "stdout" {
        Box::new(BufWriter::new(std::io::stdout()))
    } else {
        Box::new(BufWriter::new(std::fs::File::create(output).unwrap()))
    };

    writer
}
