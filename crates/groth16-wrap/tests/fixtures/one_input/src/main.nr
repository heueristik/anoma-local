// One public input: the square of a private value.
fn main(root: Field, square: pub Field) {
    assert(root * root == square);
}
