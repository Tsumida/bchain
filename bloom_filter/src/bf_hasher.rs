
/// Simple hash function.
/// reference: https://www.cnblogs.com/zhoug2020/p/6984177.html
pub fn simple_hasher(key:&[u8], step: usize) -> usize{
    let p = 16777619;
    let mut hash = 2166136261usize;

    for &b in key{
        hash = (hash ^ b as usize).wrapping_mul(p);
    }

    hash = hash.wrapping_add(hash.rotate_left(13));
    hash ^= hash.rotate_right(7);
    hash = hash.wrapping_add(hash.rotate_left(3));
    hash ^= hash.rotate_right(17);
    hash = hash.wrapping_add(hash.rotate_left(step as u32));

    hash
}


#[test]
fn overflow() {
    eprintln!("{}", simple_hasher("string".as_bytes(), 2));
    eprintln!("{}", simple_hasher("string".as_bytes(), 3));
    /*
    eprintln!("{}", simple_hasher("string".as_bytes(), 5));
    eprintln!("{}", simple_hasher("string".as_bytes(), 7));
    eprintln!("{}", simple_hasher("string".as_bytes(), 11)); */
}