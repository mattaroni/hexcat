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

    let file_contents = match read_to_string(&args.filepath) {
        Ok(x) => x,
        Err(e) => {
            print_error_msg(e);
            return;
        },
    };

    print_to_hex(file_contents);
}

fn print_to_hex(text: String) {
    let bytes: Vec<u8> = text.bytes().collect();

    // the number of bytes represented per line in the terminal.
    // [NOTE] These are defaults. They might become user-adjustable in a later
    // version (maybe)
    let width: usize = 16;

    // chunks the vector into slices of 16
    let byte_chunks: Vec<&[u8]> = bytes.chunks(width).collect();

    for byte_chunk in byte_chunks {
        // every byte will be represented by 2 hexadecimal digits, followed by a
        // space (a total of 3 characters each). The 8th byte will be followed
        // by 2 spaces instead of 1 (+1), and the last byte will be followed by
        // no spaces (-1).
        let mut hex_line = String::with_capacity(width * 3);

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

// [TODO] switch to clap error handling
/// Prints a (somewhat) stylized error message to the user.
fn print_error_msg(error: Error) {
    let binding = error.to_string();

    // rewords error messages from expected and unavoidable errors
    let message = String::from(match error.kind() {
        ErrorKind::NotFound => "file not found",
        ErrorKind::PermissionDenied => "file access denied",
        ErrorKind::InvalidData => "file contents are not valid UTF-8",
        ErrorKind::IsADirectory => "is a directory",
        _ => &binding, // fallback to error message defaults
    });

    eprintln!("\x1b[91;1merror:\x1b[0m {message}.");
}
