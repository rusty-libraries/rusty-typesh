use rusty_typesh::{type_match, TypeMatcher, TypePattern};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_multiple_types() {
    let test_cases: Vec<Box<dyn std::any::Any>> = vec![
        Box::new(42i32),
        Box::new(String::from("hello")),
        Box::new(Point { x: 1, y: 2 }),
    ];

    for value in test_cases {
        let result = if let Some(x) = value.as_ref().downcast_ref::<i32>() {
            type_match!(*x, i32 => |n: &i32| format!("Integer: {}", n))
        } else if let Some(s) = value.as_ref().downcast_ref::<String>() {
            type_match!(s.clone(), String => |s: &String| format!("String: {}", s))
        } else if let Some(p) = value.as_ref().downcast_ref::<Point>() {
            type_match!(p.clone(), Point => |p: &Point| format!("Point: ({}, {})", p.x, p.y))
        } else {
            None
        };

        assert!(result.is_some());
    }
}

#[test]
fn test_nested_types() {
    let value = Box::new(42i32);
    let value_ref = &*value;
    let patterns = vec![
        (
            Box::new(TypeMatcher::<i32>::new()) as Box<dyn TypePattern<i32>>,
            Box::new(|x: &i32| format!("Boxed Integer: {}", x)) as Box<dyn Fn(&i32) -> String>,
        ),
    ];
    let result = rusty_typesh::match_type(value_ref, &patterns);
    assert_eq!(result, Some("Boxed Integer: 42".to_string()));
}

#[test]
fn test_no_match() {
    let value = 42f64;
    let result = type_match!(
        value,
        i32 => |_| "integer",
        String => |_| "string"
    );
    assert_eq!(result, None);
} 