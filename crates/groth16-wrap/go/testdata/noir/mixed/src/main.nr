use std::embedded_curve_ops::{EmbeddedCurveScalar, fixed_base_scalar_mul};

global ROUNDS: u32 = 24;

// Uses every gate type except non-native field arithmetic: Keccak-f (lookup tables), Poseidon2, read-only and
// read-write arrays with witness indices (memory), small range checks and Grumpkin scalar multiplication
// (elliptic). Two public values: pick and the result.
fn main(x: [u64; 25], k: Field, idx: [u32; 16], table: [Field; 64], pick: pub u32) -> pub Field {
    let mut s = x;

    for _ in 0..ROUNDS {
        s = std::hash::keccakf1600(s);
    }

    let h = std::hash::poseidon2_permutation([s[0] as Field, s[1] as Field, k, 0]);

    let mut acc = h[0];

    for i in 0..16 {
        acc += table[idx[i] % 64];
    }

    let mut ram: [Field; 32] = [0; 32];

    for i in 0..16 {
        ram[idx[i] % 32] = acc + i as Field;
    }

    acc += ram[pick % 32];

    let byte = (s[2] as u8) as Field;

    let p = fixed_base_scalar_mul(EmbeddedCurveScalar::from_field(k));

    acc + p.x + byte
}
