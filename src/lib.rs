//! Type pattern matching system for Rust.
//! 
//! This crate provides a flexible type pattern matching system that allows:
//! - Runtime type checking and matching
//! - Pattern matching with custom handlers
//! - Both macro-based and manual matching approaches
//! 
//! # Examples
//! ```
//! use rusty_typesh::type_match;
//! 
//! let value = 42i32;
//! let result = type_match!(
//!     value,
//!     i32 => |x: &i32| format!("Got integer: {}", x),
//!     String => |x: &String| format!("Got string: {}", x)
//! );
//! assert_eq!(result, Some("Got integer: 42".to_string()));
//! ```

mod macros;

use std::any::TypeId;

/// A trait for implementing type pattern matching behavior.
/// 
/// This trait is used to define how types should be matched against patterns.
/// Implementors must provide a `matches()` method that determines if a type
/// matches the pattern.
pub trait TypePattern<T: 'static> {
    /// Returns true if the type matches this pattern
    fn matches(&self) -> bool;
}

/// A concrete type matcher implementation.
/// 
/// TypeMatcher provides runtime type checking capabilities by storing
/// and comparing TypeIds.
/// 
/// # Type Parameters
/// * `U` - The type to match against
pub struct TypeMatcher<U: 'static> {
    /// The TypeId of the type to match
    type_id: TypeId,
    /// PhantomData to handle the generic type parameter
    _phantom: std::marker::PhantomData<U>,
}

impl<U: 'static> TypeMatcher<U> {
    /// Creates a new TypeMatcher for the given type.
    /// 
    /// # Examples
    /// ```
    /// use rusty_typesh::TypeMatcher;
    /// let matcher = TypeMatcher::<i32>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            type_id: TypeId::of::<U>(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: 'static, U: 'static> TypePattern<T> for TypeMatcher<U> {
    fn matches(&self) -> bool {
        TypeId::of::<T>() == self.type_id
    }
}

/// Matches a value against a series of type patterns and handlers.
/// 
/// # Type Parameters
/// * `T` - The type of the value to match
/// * `R` - The return type of the pattern handlers
/// 
/// # Arguments
/// * `value` - The value to match against patterns
/// * `patterns` - A slice of tuples containing patterns and their handlers
/// 
/// # Returns
/// * `Some(R)` if a pattern matches and its handler succeeds
/// * `None` if no pattern matches
pub fn match_type<T: 'static, R>(
    value: &T,
    patterns: &[(Box<dyn TypePattern<T>>, Box<dyn Fn(&T) -> R>)],
) -> Option<R> {
    for (pattern, handler) in patterns {
        if pattern.matches() {
            return Some(handler(value));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_matching() {
        let value = 42i32;
        let patterns = vec![
            (
                Box::new(TypeMatcher::<i32>::new()) as Box<dyn TypePattern<i32>>,
                Box::new(|x: &i32| format!("Integer: {}", x)) as Box<dyn Fn(&i32) -> String>,
            ),
            (
                Box::new(TypeMatcher::<i32>::new()) as Box<dyn TypePattern<i32>>,
                Box::new(|x: &i32| format!("String length: {}", x.to_string().len())) as Box<dyn Fn(&i32) -> String>,
            ),
        ];

        let result = match_type(&value, &patterns);
        assert_eq!(result, Some("Integer: 42".to_string()));
    }
} 