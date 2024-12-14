use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusty_typesh::{type_match, TypeMatcher, TypePattern};

fn bench_type_matching(c: &mut Criterion) {
    let value = 42i32;
    
    c.bench_function("direct type match", |bench| {
        bench.iter(|| {
            type_match!(
                black_box(value),
                i32 => |x: &i32| format!("Integer: {}", x),
                String => |x: &String| format!("String length: {}", x.len())
            )
        })
    });

    let patterns: Vec<(Box<dyn TypePattern<i32>>, Box<dyn Fn(&i32) -> String>)> = vec![
        (
            Box::new(TypeMatcher::<i32>::new()),
            Box::new(|x: &i32| format!("Integer: {}", x)),
        ),
        (
            Box::new(TypeMatcher::<i32>::new()),
            Box::new(|x: &i32| format!("String length: {}", x.to_string().len())),
        ),
    ];

    c.bench_function("manual type match", |b| {
        b.iter(|| {
            rusty_typesh::match_type(black_box(&value), black_box(&patterns))
        })
    });

    // Compare with traditional match
    c.bench_function("traditional match", |b| {
        b.iter(|| {
            let x = black_box(value);
            let result = match std::any::TypeId::of::<i32>() {
                t if t == std::any::TypeId::of::<i32>() => {
                    format!("Integer: {}", x)
                }
                _ => format!("String length: {}", x.to_string().len()),
            };
            Some(result)
        })
    });
}

criterion_group!(benches, bench_type_matching);
criterion_main!(benches); 