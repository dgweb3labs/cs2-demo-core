use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cs2_demo_core::CS2DemoCore;

fn bench_demo_parsing(c: &mut Criterion) {
    let demo_core = CS2DemoCore::new();
    
    c.bench_function("demo_parser_creation", |b| {
        b.iter(|| {
            black_box(CS2DemoCore::new());
        });
    });
    
    // Note: This benchmark requires an actual demo file
    // Uncomment when you have a test demo file
    /*
    c.bench_function("demo_file_parsing", |b| {
        b.iter(|| {
            // This would require an actual demo file
            // demo_core.parse_file("test.dem").await.unwrap();
        });
    });
    */
}

criterion_group!(benches, bench_demo_parsing);
criterion_main!(benches);
