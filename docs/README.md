# Rusty TypeSh - Type Pattern Matching Documentation

## Overview
Rusty TypeSh provides a flexible type pattern matching system for Rust that enables:
- Runtime type checking and matching
- Pattern matching with custom handlers
- Both macro-based and manual matching approaches

## Core Features

### Type Matching Macro
The `type_match!` macro provides a convenient syntax for type pattern matching:

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

### Manual Type Matching
For more control, you can use the manual type matching approach:

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

## API Reference

### TypePattern Trait

**Functionality:**  
A trait for implementing type pattern matching behavior. Types implementing this trait can be used in pattern matching operations.

**Parameters:**

- **T: 'static:**  
  The type parameter representing the type to match against.

**Methods:**

- **matches(&self) -> bool:**  
  Returns true if the type matches this pattern.

**Usage Example:**
```rust
use rusty_typesh::{TypePattern, TypeMatcher};

let matcher = TypeMatcher::<i32>::new();
assert!(matcher.matches());
```

### TypeMatcher Struct

**Functionality:**  
A concrete implementation of `TypePattern` that matches types using their `TypeId`.

**Type Parameters:**

- **U: 'static:**  
  The type to match against.

**Fields:**

- **type_id (TypeId):**  
  The TypeId of the type to match.

- **_phantom (PhantomData<U>):**  
  PhantomData to handle the generic type parameter.

**Methods:**

- **new() -> Self:**  
  Creates a new TypeMatcher for the given type.

**Usage Example:**
```rust
use rusty_typesh::TypeMatcher;

let matcher = TypeMatcher::<i32>::new();
```

### type_match! Macro

**Functionality:**  
A macro for convenient type pattern matching.

**Parameters:**

- **value:**  
  The value to match against patterns.

- **patterns:**  
  One or more type patterns with associated handlers.

**Returns:**
- `Some(R)` if a pattern matches and its handler succeeds
- `None` if no pattern matches

**Usage Example:**
```rust
use rusty_typesh::type_match;

let value = 42i32;
let result = type_match!(
    value,
    i32 => |x: &i32| format!("Got integer: {}", x),
    String => |x: &String| format!("Got string: {}", x)
);
```

### match_type Function

**Functionality:**  
Matches a value against a series of type patterns and handlers.

**Parameters:**

- **value: &T:**  
  The value to match against patterns.

- **patterns: &[(Box<dyn TypePattern<T>>, Box<dyn Fn(&T) -> R>)]:**  
  A slice of tuples containing patterns and their handlers.

**Returns:**
- `Some(R)` if a pattern matches and its handler succeeds
- `None` if no pattern matches

**Usage Example:**
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
```

## Usage Examples

### Basic Type Matching
```rust
use rusty_typesh::type_match;

let number = 42i32;
let result = type_match!(
    number,
    i32 => |n: &i32| format!("Got an integer: {}", n),
    f64 => |n: &f64| format!("Got a float: {}", n)
);
println!("Result: {:?}", result); // Some("Got an integer: 42")
```

### Custom Type Matching
```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

let user = User {
    name: String::from("Alice"),
    age: 30,
};

let result = type_match!(
    user,
    User => |u: &User| format!("Got a user: {} (age {})", u.name, u.age),
    String => |_: &String| "This won't match".to_string()
);
println!("Result: {:?}", result); // Some("Got a user: Alice (age 30)")
```

### No Match Case
```rust
let value = 3.14f32;
let result = type_match!(
    value,
    i32 => |_: &i32| "integer".to_string(),
    String => |_: &String| "string".to_string()
);
println!("Result: {:?}", result); // None
```

## Performance Considerations

The crate provides benchmarks comparing different type matching approaches:
- Direct type matching using the `type_match!` macro
- Manual type matching using `TypeMatcher`
- Traditional match expression approach