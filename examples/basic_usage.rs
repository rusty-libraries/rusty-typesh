use rusty_typesh::type_match;

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    // Example 1: Basic type matching
    let number = 42;
    let result = type_match!(
        number,
        i32 => |n: &i32| format!("Got an integer: {}", n),
        f64 => |n: &f64| format!("Got a float: {}", n)
    );
    println!("Example 1: {:?}", result);

    // Example 2: String matching
    let text = String::from("Hello, World!");
    let result = type_match!(
        text,
        String => |s: &String| format!("Got a string of length {}: '{}'", s.len(), s),
        i32 => |_: &i32| "This won't match".to_string()
    );
    println!("Example 2: {:?}", result);

    // Example 3: Custom type matching
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };
    let result = type_match!(
        user,
        User => |u: &User| format!("Got a user: {} (age {})", u.name, u.age),
        String => |_: &String| "This won't match".to_string()
    );
    println!("Example 3: {:?}", result);

    // Example 4: No match case
    let value = 3.14f32;
    let result = type_match!(
        value,
        i32 => |_: &i32| "integer".to_string(),
        String => |_: &String| "string".to_string()
    );
    println!("Example 4: {:?}", result);
} 