[![crates.io](https://img.shields.io/crates/v/dirs-cli.svg)](https://crates.io/crates/dirs-cli)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![License: MPL-2.0](https://shields.io/badge/license-MPL--2.0-orange.svg)

# `dirs-cli`

## Introduction

- a tiny low-level command line application with a minimal API
- that provides the platform-specific, user-accessible locations
- for retrieving and storing configuration, cache and other data
- on Linux, Redox, Windows (≥ Vista), macOS and other platforms.

The application provides the location of these directories by leveraging the mechanisms defined by
- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and
  the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux and Redox
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows
- the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
  guidelines on macOS

## Platforms

This application is written in Rust, and supports Linux, Redox, macOS and Windows.
Other platforms are also supported; they use the Linux conventions.

## Usage

#### Dependency

Add the application as a dependency to your project by inserting

```toml
dirs-cli = "0.1.0"
```

into the `[dependencies]` section of your Cargo.toml file.

## Build

It's possible to cross-compile this application if the necessary toolchains are installed with rustup.
This is helpful to ensure a change hasn't broken code on a different platform.

The following commands will build this application on Linux, macOS and Windows:

```
cargo build --target=x86_64-unknown-linux-gnu
cargo build --target=x86_64-pc-windows-gnu
cargo build --target=x86_64-apple-darwin
cargo build --target=x86_64-unknown-redox
```

The [Cargo.toml](Cargo.toml) file contains additional information on how to minimize the binary size.

## Changelog

### 0.1.0
- Initial release, based on dirs-rs version 5

## License

Licensed under the MPL 2.0 ([LICENSE-MPL-2.0](LICENSE-MPL-2.0) or https://opensource.org/licenses/MPL-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MPL-2.0 license, shall be
licensed as above, without any additional terms or conditions.
