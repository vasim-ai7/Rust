# Learning Rust from Scratch

This is my personal repository for learning the Rust programming language from the ground up.

## Folder Structure

rust-learning/
  01_hello_world/       - First program and toolchain setup
  02_variables/         - Variables, data types, mutability
  03_control_flow/      - if expressions, loops, match
  04_ownership/         - Ownership, borrowing, slices
  05_structs_enums/     - Structs, enums, methods
  06_error_handling/    - panic!, Result, Option, ? operator
  07_collections/       - Vec, HashMap, String
  08_modules/           - Modules, paths, privacy rules
  09_testing/           - Unit tests, integration tests
  projects/             - Small practice projects
    guessing_game/      - From The Rust Book
    cli_calculator/     - Simple command-line calculator

## How to Run

1. Install Rust:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Check installation:
   rustc --version
   cargo --version

3. Run any example:
   cd 04_ownership
   cargo run

4. Run tests:
   cargo test

## Resources I'm Using

- The Rust Programming Language Book (free online)
  https://doc.rust-lang.org/book/

- Rustlings: small exercises to learn Rust syntax
  https://github.com/rust-lang/rustlings

- Rust by Example
  https://doc.rust-lang.org/rust-by-example/

## My Progress

[ ] Chapter 1: Getting Started
[ ] Chapter 2: Writing a Guessing Game
[ ] Chapter 3: Common Programming Concepts
[ ] Chapter 4: Understanding Ownership
[ ] Chapter 5: Using Structs
[ ] Chapter 6: Enums and Pattern Matching
[ ] Chapter 7: Managing Growing Projects
[ ] Chapter 8: Common Collections
[ ] Chapter 9: Error Handling
[ ] Chapter 10: Generic Types and Traits

## License

MIT
