# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Option to adjust the width of each printed line.

### Changed

- *Considerably* optimize code.
- Make each printed line wider (by default), from 47 characters to 80.
- Switch from lowercase letters to uppercase letters for hexadecimal bytes.

## [0.1.0] - 2025-05-15

### Added

- Core functionality: printing file contents as hexadecimal bytes.
- "Help" option (`-h`/`--help`): prints a useful "help" message, explaining what
  the binary does and what command-line arguments it takes.
- "Version" option (`-V`/`--version`): prints the version number of the
  downloaded binary.
- Essential documentation (read-me, license, changelog, & cargo file).
- Availability on [GitHub](https://github.com/mattaroni/hexcat).

[unreleased]: https://github.com/mattaroni/hexcat/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mattaroni/hexcat/releases/tag/v0.1.0
