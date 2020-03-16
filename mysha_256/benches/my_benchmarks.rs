use criterion::{black_box, criterion_main, criterion_group, Criterion};

#[path="../src/sha_256.rs"]
mod sha_256;
use sha_256::SHA256;

fn case_to_string(path:&str) -> String{
    use std::fs::File;
    use std::io::Read;
    let mut s = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut s).unwrap();
    s
}

fn bench_small(c: &mut Criterion) {
    let s = case_to_string("benches/small.txt");
    c.bench_function(
        "mysha-256: small",    
        |b| b.iter(|| {
            let _ = SHA256::new(s.as_bytes()).cal_sha_256();
        })
    );
}

fn bench_large(c: &mut Criterion) {
    let s = case_to_string("benches/large.txt");
    c.bench_function(
        "mysha-256: large",    
        |b| b.iter(|| {
            let _ = SHA256::new(s.as_bytes()).cal_sha_256();
        })
    );
}
criterion_group!(benches, bench_small, bench_large);
criterion_main!(benches);