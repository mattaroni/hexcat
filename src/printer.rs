use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
};

pub fn print_as_hexadecimal(filename: String, width: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut bytes = file.bytes();
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
