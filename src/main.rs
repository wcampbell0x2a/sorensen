fn main() {
    let data = &[0xa; 6];
    let orig_hash = sha256::digest_bytes(data);
    println!("{data:x?}");
    println!("{orig_hash:x?}");
    let total_bits = sorensen::total_bits(data);
    let uncompressed_data = sorensen::decompress(data.len(), total_bits, &orig_hash);

    println!("recovered: {uncompressed_data:x?}");
}
