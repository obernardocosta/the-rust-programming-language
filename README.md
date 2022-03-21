# the-rust-programming-language
Welcome to The Rust Programming Language, an introductory book about Rust. The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.


## Compile and run

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed

1. Compile
```bash
rustc main.rs
```
2. Run
```bash
./main
```

## Auto Formater

```bash
rustfmt main.rs
```

# Cargo

Cargo is Rust’s build system and pack

## Creating a Project with Cargo

```bash
cargo new hello_cargo
cd hello_cargo
```

1. Compile

- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

```bash
cargo build
```
2. Run
```bash
./target/debug/hello_cargo
```

3. Compile and Run
```bash
cargo run
```

4. Check your code it compiles, but doesn’t produce an executable.

- much faster than `cargo build`

```bash
cargo check
```

5. build release

- Compiles with optimization and the executable goes to `target/release` instead of `target/debug`
- Run faster but takes longer to compile.

```bash
cargo build --release
```

6. build a project is simples with cargo

```bash
git clone https://github.com/obernardocosta/the-rust-programming-language.git
cd 101-getting-started/hello_cargo
cargo build
```

# best practices

1. Use Cargo
2. use a lot of `cargo check` to make sure it compiles than run `cargo build` when it ready to use the executable.
