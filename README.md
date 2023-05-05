# Rust Weekly Projects Report

- Build and Push weekly Rust Projects into this repository

![image](https://user-images.githubusercontent.com/104106034/236558308-2836e06d-e88b-4d1f-ba1b-c406c8e7daea.png)

## Introduction
- _Cargo.toml_ file and _./src/_ are created with command `cargo init --name "[project name]"`
- In the _./src/_ directory, there are two files (_main.rs_ and _lib.rs_)
    -  The `main` function is in the _main.rs_ file.
    -  Most logics and self-defined functions are in the _lib.rs_ file.

## Usage and reproduce
- Open Codespaces or Local IDE
    - set up your own CPU/GPU choice
    - Run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` to install [Rust](https://rustup.rs/) if you haven't installed one
- `cd` into each project
    * Run `cargo run -- --help`
    * Edit and run `make format` to format code

## References

* [rust-cli-example](https://github.com/nogibjj/hello-rust)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-tutorial-spring2023](https://nogibjj.github.io/rust-tutorial/)
