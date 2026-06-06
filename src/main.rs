use clap::Parser;

mod printer;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Number of bytes to print for each line
    #[arg(short, long, default_value_t = 27)]
    width: usize,

    /// Path of the file to read
    file: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = printer::print_as_hexadecimal(args.file, args.width) {
        eprintln!("\x1b[91;1merror:\x1b[0m {}", e.kind());
    }
}
