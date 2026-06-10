use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use clap::Parser;

/// Command-line arguments for the application.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Number of bytes to print for each line
    #[arg(short, long, default_value_t = 27)]
    width: usize,

    /// Path of the file to read
    file: String,
}

/// Runs the application.
fn main() {
    let args = Cli::parse();

    if let Err(e) = print_as_hexadecimal(args.file, args.width) {
        eprintln!("\x1b[91;1merror:\x1b[0m {}", e);
    }
}

/// Prints the contents of a file in hexadecimal.
fn print_as_hexadecimal(filename: String, width: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut bytes = BufReader::new(file).bytes();
    let mut buffer = BufWriter::new(io::stdout());

    loop {
        let mut chunk = bytes.by_ref().take(width);

        let first = match chunk.next() {
            Some(x) => x?,
            None => break,
        };

        write!(buffer, "{first:02X}")?;

        for byte in chunk {
            write!(buffer, " {:02X}", byte?)?;
        }

        writeln!(buffer)?;
    }

    buffer.flush()
}
