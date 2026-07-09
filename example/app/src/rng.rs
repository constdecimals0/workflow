//! Hand-rolled xorshift64 PRNG (Marsaglia's 13/7/17 variant) — the game's
//! only randomness source, deliberately preferred over the `rand` crate for
//! one uniform pick of 4 (ADR 0002).

pub(crate) struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub(crate) fn new(seed: u64) -> Self {
        // Xorshift is a fixed point at zero; remap so a zero seed still
        // produces a live stream.
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
