# Hexcat

![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmattaroni%2Fhexcat%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.version&label=version)
![license](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmattaroni%2Fhexcat%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.package.license&label=license&color=blueviolet)

Prints the contents of a file as hexadecimal bytes.

> [!WARNING]
> This is solely a pet-project of mine; a short, fun warmup for me to practice
> writing code in Rust. It's not designed for any serious, real-world use. Do
> not expect this project to be maintained properly.
> 
> Use this code (and the associated binaries) at your own discretion.

## Build from source

To build the binary from source, you will need the following tools:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git](https://git-scm.com/downloads)

Clone the repository with `git`, then compile the binary with `cargo`.

```sh
git clone --depth 1 https://github.com/mattaroni/hexcat.git
cd hexcat/
cargo build
```

The binary will be located in `target/debug/` under the name `hexcat`.
