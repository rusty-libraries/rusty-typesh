//! Macro definitions for the type pattern matching system.
//! 
//! This module provides the `type_match!` macro for convenient
//! pattern matching syntax.

/// Matches a value against multiple type patterns using a convenient macro syntax.
/// 
/// # Arguments
/// * `$value` - The value to match
/// * `$type` - The type to match against
/// * `$handler` - The closure to handle the matched type
/// 
/// # Examples
/// ```
/// use rusty_typesh::type_match;
/// 
/// let value = 42i32;
/// let result = type_match!(
///     value,
///     i32 => |x: &i32| format!("Integer: {}", x),
///     String => |x: &String| format!("String: {}", x)
/// );
/// ```
#[macro_export]
macro_rules! type_match {
    ($value:expr, $($type:ty => $handler:expr),+ $(,)?) => {{
        let value: &dyn std::any::Any = &$value;
        match value {
            $(
                x if x.is::<$type>() => Some(($handler)(x.downcast_ref::<$type>().unwrap())),
            )+
            _ => None,
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_type_match_macro() {
        let value = 42i32;
        let result = type_match!(
            value,
            i32 => |x: &i32| -> Result<String, ()> { Ok(format!("Integer: {}", x)) },
            String => |x: &String| -> Result<String, ()> { Ok(format!("String length: {}", x.to_string().len())) }
        );
        assert_eq!(result, Some(Ok("Integer: 42".to_string())));
    }
} 