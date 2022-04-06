use digest::Digest;
use itertools::Itertools;

pub fn total_bits(data: &[u8]) -> u32 {
    data.iter().map(|a| a.count_ones()).sum()
}

pub fn decompress<D: Digest>(data_len: usize, bits_set: u32, hash: &str) -> Option<Vec<u8>> {
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

            let mut hasher = D::new();
            hasher.update(&a);
            let sus_hash = hasher.finalize();
            if hash == base16ct::lower::encode_string(&sus_hash) {
                return Some(a);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Sha256;

    fn test_256(bytes: &[u8], hash: &str) {
        let total_bits = total_bits(bytes);
        assert_eq!(
            Some(bytes.to_vec()),
            decompress::<Sha256>(bytes.len(), total_bits, hash)
        );
    }

    #[test]
    fn test_01() {
        let bytes = &[0xa; 1];
        let hash = "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b";
        test_256(bytes, hash);
    }

    #[test]
    fn test_02() {
        let bytes = &[0xa; 2];
        let hash = "75a11da44c802486bc6f65640aa48a730f0f684c5c07a42ba3cd1735eb3fb070";
        test_256(bytes, hash);
    }

    #[test]
    fn test_03() {
        let bytes = &[0xa; 3];
        let hash = "6a3cf5192354f71615ac51034b3e97c20eda99643fcaf5bbe6d41ad59bd12167";
        test_256(bytes, hash);
    }

    #[test]
    fn test_04() {
        let bytes = &[0xa; 4];
        let hash = "545c38b0922de19734fbffde62792c37c2aef6a3216cfa472449173165220f7d";
        test_256(bytes, hash);
    }
}
