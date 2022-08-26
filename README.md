# Helio ☀
[![](https://img.shields.io/badge/rust-1.15.0-brightgreen.svg)](https://crates.io/crates/helio)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A library for creating oop-oriented code in rust. Intended originally for my project: [Lightfetch](https://github.com/bwte/lightfetch/).

## Features and Benefits :sunglasses:

- Create safe, fast, and (hopefully) secure Object-Oriented code in Rust.
- Static instances of types.

## Use cases 💡

Annoyed by the fact that Rust is missing a lot of object-oriented features, I decided to create this library to fill a couple of gaps in a lightweight, safe, and fast manner.

## Usage 🤔

#### Create a Static Instance of a Type

```rust
#[macro_use]
use helio::{Instance, New};

/// Make sure to use the Clone trait!
#[derive(Clone)]
struct Example {
    counter: u32,
}

/// Regular implementation of a struct with any methods you want.
impl Example {
    fn new() -> Self {
        Example { counter: 0 }
    }
    fn count(&mut self) {
        self.counter += 1;
    }
}

/// Implement the Instance trait for Example.
impl Instance for Example {
    fn counter(&self) -> u32 {
        self.counter
    }
}

fn main() {
    let example = Example::new(); /// Create a new instance of Example.
    example.count(); /// Use that instance to call the count() method and count up.
    println!("{}", example.counter()); /// Access the counter and print it.
}
```
## Installing 📦

```toml
[dependencies]
helio = "0.1.0"
```

## Show your support 💛

[![](https://img.shields.io/badge/github-helio-blue.svg)](https://github.com/bwte/helio) [![](https://img.shields.io/badge/support-me-pink)](https://github.com/sponsors/bwte)

Leave a ⭐ if you like this project.

## Contribution 🚩

Found a problem or have a suggestion? Feel free to open an issue or a pull request!

## License

This template itself is licensed under the [MIT License](LICENSE) and includes this as the default project license.
