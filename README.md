
# Rust Prime Numbers Generator

Prime Generator is a command-line tool written in Rust that generates prime numbers. It can find the next prime number from a given start or continuously find prime numbers starting from a given number.


## Prime numbers usage in cryptography

Prime numbers are used in many cryptographic algorithms. For example, RSA encryption uses prime numbers to generate public and private keys. The security of RSA encryption relies on the difficulty of factoring large composite numbers into their prime factors. Prime numbers are also used in elliptic curve cryptography, which is another popular encryption method.


## Features

- **Next Prime**: Find the next prime number from a specified start number.
- **Continuous Search**: Continuously find and display prime numbers starting from a specified or default start number.

## Prerequisites

Before you begin, ensure you have met the following requirements:
- Rust (latest stable version)
- Cargo (Rust's package manager)

You can install Rust and Cargo using [rustup](https://rustup.rs/).

## Installation

Build the project using Cargo:

```bash
cargo build --release
```

The executable will be located in `./target/release/`.

## Usage

The Prime Generator application can be run with the following commands:

### Find the Next Prime Number

```bash
cargo run -- next 100
```

This command finds the next prime number starting from 100.

### Continuously Find Primes

```bash
cargo run -- start 1000
```

This command starts finding and displaying prime numbers continuously from 1000. If no start number is specified, it defaults to the maximum value of `usize` divided by 2.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

To contribute to Prime Generator, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively, see the GitHub documentation on [creating a pull request](https://help.github.com/articles/creating-a-pull-request/).

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Vladimir M. - [@vvmspace](https://t.me/vvmspace)