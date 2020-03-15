# Impl for sha-256.
Example:
```rust
let mut sha = SHA256::new("abc".to_ascii_lowercase().as_bytes());
assert_eq!(
	sha.cal_sha_256(),
	"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
);
```
