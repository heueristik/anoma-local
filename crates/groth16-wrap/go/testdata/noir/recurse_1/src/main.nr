global HONK: u32 = 0;

fn main(vk: [Field; 115], proof: [Field; 410], pi: pub [Field; 1], key_hash: Field) {
    std::verify_proof_with_type(vk, proof, pi, key_hash, HONK);
}
