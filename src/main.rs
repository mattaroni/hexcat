use std::{fs::read_to_string, io::{Error, ErrorKind}};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to the file
    filepath: String,
}

fn main() {
    let args = Args::parse();

    let content = match read_to_string(&args.filepath) {
        Ok(x) => x,
        Err(e) => {
            print_error_msg(e);
            return;
        },
    };

    print_hex(content);
}

fn print_hex(text: String) {
    let bytes: Vec<u8> = text.bytes().collect();

    /*
    [NOTE] These are defaults. They will become user-adjustable in a later
    version (maybe).
    */
    let width: usize = 16;

    // chunks the vector into slices of 16
    let byte_chunks: Vec<&[u8]> = bytes.chunks(width).collect();

    // initializes a string vector containing the hex values of each byte in
    // byte_chunks
    // let mut hex_chunks: Vec<String> = Vec::new();

    for byte_chunk in byte_chunks {
        let mut hex_line = String::with_capacity(width * 3 + 2);

        for byte in byte_chunk {
            let hex = format!("{:02x} ", byte);
            hex_line.push_str(&hex);
        }

        // trims trailing whitespace
        hex_line.remove(hex_line.len() - 1);
        
        if 23 < hex_line.len() {
            hex_line.insert(23, ' ');
        }

        println!("{}", hex_line);
    }
}

fn print_error_msg(error: Error) {
    let binding = error.to_string().to_ascii_lowercase();
    let message = String::from(match error.kind() {
        ErrorKind::NotFound => "file not found",
        ErrorKind::PermissionDenied => "file access denied",
        ErrorKind::InvalidData => "file contents are not valid UTF-8",
        ErrorKind::IsADirectory => "is a directory",
        _ => &binding,
    });

    eprintln!("\x1b[91;1merror:\x1b[0m {message}.");
}
