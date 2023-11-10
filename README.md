# breacher-rs

Breacher is a tool written in Rust for scanning web applications to discover potential admin panels and paths. It can be used to identify common paths used by content management systems (CMS) and other web applications.

## Features

- **Robots.txt Checker:** Quickly check for the existence of the robots.txt file on the target website.

- **Admin Panel Scanner:** Scan for common admin panel paths to identify potential access points.

- **Multithreading Support:** Utilize multithreading for faster scanning (optional).

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - Make sure you have Rust installed on your system.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/dannyhvc/breacher-rs.git
   cd breacher-rust
   ```
   
2. Build the Project:
   
   ```bash
   cargo build --release
   ```
   
3. Usage:

   ```bash
   # Display help and options
   ./target/release/breacher --help

   # Example: Scan a target website without multithreading
   ./target/release/breacher -u http://example.com

   # Example: Scan a target website with multithreading
   ./target/release/breacher -u http://example.com --fast --type php

   ```

## License
This project is licensed under the MIT License.

## Acknowledgments

- [s0md3v/Breacher](https://github.com/s0md3v/Breacher): The repository and work by s0md3v served as inspiration for this project.
- [Tokio](https://tokio.rs): Asynchronous runtime for Rust.
- [Reqwest](https://docs.rs/reqwest): An ergonomic HTTP client for Rust.


+ Email: danherrerav@gamil.com

## Disclaimer
This tool is provided for educational and informational purposes only. The author is not responsible for any misuse or damage caused by this tool.
