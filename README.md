# Rusty TypeSh - Type Pattern Matching ![Crates.io](https://img.shields.io/crates/v/rusty-typesh) ![docs.rs](https://img.shields.io/docsrs/rusty-typesh) ![License](https://img.shields.io/crates/l/rusty-typesh)

Welcome to Rusty TypeSh, a flexible type pattern matching system for Rust. This library provides a convenient way to perform runtime type checking and pattern matching with custom handlers.

## Table of Contents
- [Rusty TypeSh - Type Pattern Matching](#rusty-typesh---type-pattern-matching)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Getting Started](#getting-started)
    - [Basic Type Matching](#basic-type-matching)
    - [Custom Type Matching](#custom-type-matching)
  - [Documentation](#documentation)
  - [License](#license)

## Installation

To use this library, add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
rusty-typesh = "0.1.0"
```

## Getting Started

To get started with Rusty TypeSh, follow these steps:

### Basic Type Matching

Use the `type_match!` macro for simple type matching:

```rust
use rusty_typesh::type_match;

let value = 42i32;
let result = type_match!(
    value,
    i32 => |x: &i32| format!("Got integer: {}", x),
    String => |x: &String| format!("Got string: {}", x)
);
assert_eq!(result, Some("Got integer: 42".to_string()));
```

### Custom Type Matching

For more control, use the manual type matching approach:

```rust
use rusty_typesh::{TypeMatcher, TypePattern};

let value = 42i32;
let patterns: Vec<(Box<dyn TypePattern<i32>>, Box<dyn Fn(&i32) -> String>)> = vec![
    (
        Box::new(TypeMatcher::<i32>::new()),
        Box::new(|x: &i32| format!("Integer: {}", x)),
    ),
];

let result = rusty_typesh::match_type(&value, &patterns);
assert_eq!(result, Some("Integer: 42".to_string()));
```

## Documentation

For detailed information on all available features and their usage, please refer to the full [SDK Documentation](https://rusty-libraries.github.io/rusty-typesh/).

## License

This library is licensed under the MIT License. For more details, see the [LICENSE](LICENSE.md) file.