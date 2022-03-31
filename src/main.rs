use itertools::Itertools;

fn total_bits(data: &[u8]) -> u32 {
    data.iter().map(|a| a.count_ones()).sum()
}

fn main() {
    let data = &[b'a'; 4];
    let length = data.len();
    println!("data {data:x?}");

    let og_digest = sha256::digest_bytes(data);
    println!("sha256: {}", og_digest);

    let total_bits = total_bits(data);
    for combinations in (0..(length * 8))
        .collect::<Vec<usize>>()
        .iter()
        .combinations(total_bits as usize)
    {
        {
            let mut a = vec![0; length];

            for bit in combinations {
                a[*bit / 8] |= 1 << (bit % 8);
            }

            let digest = sha256::digest_bytes(&a);
            if digest == og_digest {
                println!("recovered: {a:x?}");
                return;
            }
        }
    }
    println!("sadpanda.png.jpeg.exe");
}
