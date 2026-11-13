/// SplitMix64: PRNG local, con algoritmo fijado para no depender de `rand`.
pub(crate) struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, upper: usize) -> usize {
        debug_assert!(upper > 0);
        let bound = upper as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let value = self.next_u64();
            if value >= threshold {
                return (value % bound) as usize;
            }
        }
    }

    pub fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

/// FNV-1a con longitudes: ["ab", "c"] difiere de ["a", "bc"].
pub(crate) fn hash_seeds(seeds: &[&str]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for seed in seeds {
        for byte in (seed.len() as u64)
            .to_le_bytes()
            .iter()
            .chain(seed.as_bytes())
        {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

/// Dominio separado de las semillas de texto; conserva los 64 bits de cada número.
pub(crate) fn hash_numeros(seeds: &[u64]) -> u64 {
    let mut hash = 0x8b6d_4f4e_4f4d_4149_u64;
    for byte in (seeds.len() as u64)
        .to_le_bytes()
        .into_iter()
        .chain(seeds.iter().flat_map(|n| n.to_le_bytes()))
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
