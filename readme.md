# Rust Code Examples & Experiments 
![Screenshot 2025-01-21 at 08-39-30 Learning Rust - YouTube](https://github.com/user-attachments/assets/3b5a268b-0e6b-4965-bcc0-e1ef3b682231)


This repository contains various Rust examples to help learn and understand key concepts in the Rust programming language. Each example is designed to demonstrate a specific feature or idea, making it easier to grasp Rust's "unique and powerful capabilities".

## Getting Started

Ensure you have Rust installed on your system. If not, install it using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify it with:

```bash
rustc --version
cargo --version
```

## Project Structure

All examples are located in the `src/bin` directory. Each example is a standalone binary and can be run independently using Cargo.

### Example Directory Layout

```plaintext
src/
├── main.rs         # Primary entry point (if any)
└── bin/
    ├── cow_example.rs    # Example of Copy on Write (Cow)
    ├── another_example.rs
    └── more_examples.rs
```

## How to Compile and Run

To compile and run a specific example, use the following command:

```bash
cargo run --bin <example_name>
```

### Example: Running the `cow_example`

Navigate to the project directory and run:

```bash
~/Documents/rust/xp_learn/src/bin  main ?3 
❯ cargo run --bin cow_example
```

This will compile and execute the `cow_example.rs` file located in the `src/bin` directory.

## Contributing

If you'd like to contribute new examples or improve the existing ones, feel free to submit a pull request. Make sure your code is well-documented and includes comments explaining the concepts.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
<<<<<<< HEAD
=======

>>>>>>> 5ac8023 (chore: add comments to cow example)
