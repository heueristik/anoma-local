global N: u32 = 8;

fn main(x: [u64; 25]) -> pub u64 {
    let mut s = x;

    for _ in 0..N {
        s = std::hash::keccakf1600(s);
    }

    s[0]
}
