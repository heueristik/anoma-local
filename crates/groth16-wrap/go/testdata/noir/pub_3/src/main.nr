// Three public inputs (y, z and the return value), so their order and the public input delta are exercised.
fn main(x: Field, y: pub Field, z: pub Field) -> pub Field {
    assert(x * y != z);

    let mut acc = x;

    for i in 0..64 {
        acc = acc * acc + y + (i as Field);
    }

    acc + z
}
