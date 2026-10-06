//! SIMD feature sets FastNoise2 nodes are compiled for.
use std::fmt;

/// A SIMD feature set (`FastSIMD::FeatureSet`), the instructions a node generates noise with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeatureSet {
    Scalar,
    Sse,
    Sse2,
    Sse3,
    Ssse3,
    Sse41,
    Sse42,
    Avx,
    Avx2,
    Avx512,
    Neon,
    Aarch64,
    Wasm,
}

/// `FastSIMD::FeatureFlag`, a bit of a feature set.
mod flag {
    pub const SCALAR: u32 = 1 << 0;
    pub const X86: u32 = 1 << 1;
    pub const SSE: u32 = 1 << 2;
    pub const SSE2: u32 = 1 << 3;
    pub const SSE3: u32 = 1 << 4;
    pub const SSSE3: u32 = 1 << 5;
    pub const SSE41: u32 = 1 << 6;
    pub const SSE42: u32 = 1 << 7;
    pub const AVX: u32 = 1 << 8;
    pub const AVX2: u32 = 1 << 9;
    pub const AVX512: u32 = 0b1111 << 10;
    pub const ARM: u32 = 1 << 14;
    pub const NEON: u32 = 1 << 15;
    pub const AARCH64: u32 = 1 << 16;
    pub const WASM: u32 = 1 << 17;
}

impl FeatureSet {
    const ALL: [Self; 13] = [
        Self::Scalar,
        Self::Sse,
        Self::Sse2,
        Self::Sse3,
        Self::Ssse3,
        Self::Sse41,
        Self::Sse42,
        Self::Avx,
        Self::Avx2,
        Self::Avx512,
        Self::Neon,
        Self::Aarch64,
        Self::Wasm,
    ];

    /// The `FastSIMD::FeatureSet` value: every flag of the feature set and of the ones it extends.
    pub(crate) const fn bits(self) -> u32 {
        use flag::*;

        const SSE_BITS: u32 = X86 | SSE;
        const SSE2_BITS: u32 = SSE_BITS | SSE2;
        const SSE3_BITS: u32 = SSE2_BITS | SSE3;
        const SSSE3_BITS: u32 = SSE3_BITS | SSSE3;
        const SSE41_BITS: u32 = SSSE3_BITS | SSE41;
        const SSE42_BITS: u32 = SSE41_BITS | SSE42;
        const AVX_BITS: u32 = SSE42_BITS | AVX;
        const AVX2_BITS: u32 = AVX_BITS | AVX2;
        const NEON_BITS: u32 = ARM | NEON;

        match self {
            Self::Scalar => SCALAR,
            Self::Sse => SSE_BITS,
            Self::Sse2 => SSE2_BITS,
            Self::Sse3 => SSE3_BITS,
            Self::Ssse3 => SSSE3_BITS,
            Self::Sse41 => SSE41_BITS,
            Self::Sse42 => SSE42_BITS,
            Self::Avx => AVX_BITS,
            Self::Avx2 => AVX2_BITS,
            Self::Avx512 => AVX2_BITS | AVX512,
            Self::Neon => NEON_BITS,
            Self::Aarch64 => NEON_BITS | AARCH64,
            Self::Wasm => WASM,
        }
    }

    pub(crate) fn from_bits(bits: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|feature_set| feature_set.bits() == bits)
    }
}

impl fmt::Display for FeatureSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Scalar => "SCALAR",
            Self::Sse => "SSE",
            Self::Sse2 => "SSE2",
            Self::Sse3 => "SSE3",
            Self::Ssse3 => "SSSE3",
            Self::Sse41 => "SSE4.1",
            Self::Sse42 => "SSE4.2",
            Self::Avx => "AVX",
            Self::Avx2 => "AVX2",
            Self::Avx512 => "AVX512",
            Self::Neon => "NEON",
            Self::Aarch64 => "AARCH64",
            Self::Wasm => "WASM",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values of `FastSIMD::FeatureSet`.
    #[test]
    fn test_bits() {
        assert_eq!(FeatureSet::Scalar.bits(), 0x1);
        assert_eq!(FeatureSet::Sse2.bits(), 0xe);
        assert_eq!(FeatureSet::Avx2.bits(), 0x3fe);
        assert_eq!(FeatureSet::Avx512.bits(), 0x3ffe);
        assert_eq!(FeatureSet::Neon.bits(), 0xc000);
        assert_eq!(FeatureSet::Aarch64.bits(), 0x1c000);
        assert_eq!(FeatureSet::Wasm.bits(), 0x20000);
    }

    #[test]
    fn test_from_bits() {
        for feature_set in FeatureSet::ALL {
            assert_eq!(FeatureSet::from_bits(feature_set.bits()), Some(feature_set));
        }
        assert_eq!(FeatureSet::from_bits(0), None);
    }
}
