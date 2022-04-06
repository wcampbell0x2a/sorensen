use sha2::{Digest, Sha256};

fn main() {
    // sample input
    let data = &[0xa; 4];

    // example usage of creating a sha256 hash
    let mut hasher = sha2::Sha256::new();
    hasher.update(&data);
    let sus_hash = hasher.finalize();
    let orig_hash = base16ct::lower::encode_string(&sus_hash);
    println!("{data:x?}");
    println!("{orig_hash:x?}");

    // saving the amount of total bits
    let total_bits = sorensen::total_bits(data);

    // decompress data with sha256
    let uncompressed_data = sorensen::decompress::<Sha256>(data.len(), total_bits, &orig_hash);

    println!("recovered: {uncompressed_data:x?}");
}
