use clap::Parser;

mod printer;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path of the file to read
    file: String,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = printer::print_as_hexadecimal(args.file) {
        eprintln!("\x1b[91;1merror:\x1b[0m {e}");
    }
}
