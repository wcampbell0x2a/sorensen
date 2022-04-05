use itertools::Itertools;

pub fn total_bits(data: &[u8]) -> u32 {
    data.iter().map(|a| a.count_ones()).sum()
}

pub fn decompress(data_len: usize, bits_set: u32, hash: &str) -> Option<Vec<u8>> {
    for combinations in (0..(data_len as usize * 8))
        .collect::<Vec<usize>>()
        .iter()
        .combinations(bits_set as usize)
    {
        {
            let mut a = vec![0; data_len as usize];

            for bit in combinations {
                a[*bit / 8] |= 1 << (bit % 8);
            }

            let sus_hash = sha256::digest_bytes(&a);
            if sus_hash == hash {
                return Some(a);
            }
        }
    }

    None
}
