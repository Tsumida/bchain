# BloomFilter
## Usage
```rust
use bloom_filter::BloomFilterBuilder;

let mut bf = BloomFilterBuilder::new()
.set_bit_size(50)
.set_hash_steps(&[2, 3, 5, 7])
.build().unwrap();

bf.print_mem_view();

bf.insert("string".as_bytes());
bf.print_mem_view();

bf.insert("hello, world!".as_bytes());
bf.print_mem_view();

assert!(bf.contains("string".as_bytes()));
assert!(bf.contains("hello, world!".as_bytes()));

assert!(!bf.contains("helloworld".as_bytes()));
```

## Benchmark
```
bench_contains: 1000000 checking takes 144 ms
test bf_test::bf_bench_contains ... ok

bench_insertion: 1000000 insertions takes 148 ms
test bf_test::bf_bench_insertion ... ok
```