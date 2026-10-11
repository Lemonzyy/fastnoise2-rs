//! SIMD feature sets FastNoise2 nodes are compiled for.
use std::{cell::Cell, fmt, sync::LazyLock};

use bitflags::bitflags;
use fastnoise2_sys::*;

use crate::FastNoiseError;

thread_local! {
    /// `maxFeatureSet` passed to FastNoise2 when creating nodes, `u32::MAX` detects the best one.
    static MAX_FEATURE_SET: Cell<u32> = const { Cell::new(u32::MAX) };
}

/// The feature set FastNoise2 picks when detecting it.
static DETECTED: LazyLock<FeatureSet> = LazyLock::new(|| {
    let node = unsafe { fnNewFromMetadata(0, u32::MAX) };
    assert!(!node.is_null(), "FastNoise2 has a node with metadata id 0");

    let bits = unsafe { fnGetActiveFeatureSet(node) };
    unsafe { fnDeleteNodeRef(node) };

    FeatureSet::from_bits(bits).expect("FastNoise2 nodes have a known feature set")
});

/// Runs `f` creating nodes with at most the `max` feature set, on this thread. It applies to every
/// node created in `f`, by [`Generator::build`](crate::Generator::build), [`NodeBuilder`](crate::NodeBuilder)
/// or [`Node::from_encoded_node_tree`](crate::Node::from_encoded_node_tree), and nodes use the best
/// feature set FastNoise2 is compiled for up to `max`.
///
/// A higher feature set than [`FeatureSet::detected`] is lowered to it, as the CPU doesn't support it.
///
/// ```rust
/// use fastnoise2::{FeatureSet, prelude::*, with_max_feature_set};
///
/// # #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
/// let node = with_max_feature_set(FeatureSet::Sse41, || perlin().fractal_f_bm().build())?;
/// assert_eq!(node.get_active_feature_set(), FeatureSet::Sse41);
/// # }
/// # Ok::<(), fastnoise2::FastNoiseError>(())
/// ```
///
/// # Errors
/// Returns an error if FastNoise2 isn't compiled for `max` or a lower feature set on this target
/// (e.g. [`FeatureSet::Scalar`], or [`FeatureSet::Neon`] on x86).
pub fn with_max_feature_set<R>(
    max: FeatureSet,
    f: impl FnOnce() -> R,
) -> Result<R, FastNoiseError> {
    let detected = FeatureSet::detected();
    if !max.is_compiled_up_to(detected) {
        return Err(FastNoiseError::FeatureSetNotAvailable {
            requested: max,
            detected,
        });
    }

    // Restores the previous maximum, also when `f` panics
    struct Restore(u32);

    impl Drop for Restore {
        fn drop(&mut self) {
            MAX_FEATURE_SET.set(self.0);
        }
    }

    let bits = if max.bits() >= detected.bits() {
        u32::MAX
    } else {
        max.bits()
    };
    let _restore = Restore(MAX_FEATURE_SET.replace(bits));

    Ok(f())
}

/// `maxFeatureSet` to create nodes with on this thread, see [`with_max_feature_set`].
#[inline]
pub(crate) fn max_feature_set_bits() -> u32 {
    MAX_FEATURE_SET.get()
}

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

bitflags! {
    /// `FastSIMD::FeatureFlag`, the flags a feature set is made of.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct FeatureFlags: u32 {
        const SCALAR = 1 << 0;
        const X86 = 1 << 1;
        const SSE = 1 << 2;
        const SSE2 = 1 << 3;
        const SSE3 = 1 << 4;
        const SSSE3 = 1 << 5;
        const SSE41 = 1 << 6;
        const SSE42 = 1 << 7;
        const AVX = 1 << 8;
        const AVX2 = 1 << 9;
        const AVX512_F = 1 << 10;
        const AVX512_VL = 1 << 11;
        const AVX512_DQ = 1 << 12;
        const AVX512_BW = 1 << 13;
        const ARM = 1 << 14;
        const NEON = 1 << 15;
        const AARCH64 = 1 << 16;
        const WASM = 1 << 17;
    }
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

    /// Every flag of the feature set and of the ones it extends, like `FastSIMD::FeatureSet`.
    fn flags(self) -> FeatureFlags {
        type F = FeatureFlags;

        match self {
            Self::Scalar => F::SCALAR,
            Self::Sse => F::X86 | F::SSE,
            Self::Sse2 => Self::Sse.flags() | F::SSE2,
            Self::Sse3 => Self::Sse2.flags() | F::SSE3,
            Self::Ssse3 => Self::Sse3.flags() | F::SSSE3,
            Self::Sse41 => Self::Ssse3.flags() | F::SSE41,
            Self::Sse42 => Self::Sse41.flags() | F::SSE42,
            Self::Avx => Self::Sse42.flags() | F::AVX,
            Self::Avx2 => Self::Avx.flags() | F::AVX2,
            Self::Avx512 => {
                Self::Avx2.flags() | F::AVX512_F | F::AVX512_VL | F::AVX512_DQ | F::AVX512_BW
            }
            Self::Neon => F::ARM | F::NEON,
            Self::Aarch64 => Self::Neon.flags() | F::AARCH64,
            Self::Wasm => F::WASM,
        }
    }

    /// The `FastSIMD::FeatureSet` value.
    pub(crate) fn bits(self) -> u32 {
        self.flags().bits()
    }

    /// The best feature set FastNoise2 is compiled for and the CPU supports, used by default.
    pub fn detected() -> Self {
        *DETECTED
    }

    /// Whether FastNoise2 is compiled for this feature set or a lower one of the same
    /// architecture as `detected`, so creating a node with it as maximum succeeds.
    fn is_compiled_up_to(self, detected: Self) -> bool {
        // The lowest feature set FastSIMD compiles by default for each architecture
        let detected = detected.flags();
        let minimum = if detected.contains(FeatureFlags::X86) {
            Self::Sse2
        } else if detected.contains(FeatureFlags::ARM) {
            Self::Neon
        } else {
            Self::Wasm
        };
        let architecture = FeatureFlags::X86 | FeatureFlags::ARM | FeatureFlags::WASM;

        self.flags() & architecture == detected & architecture && self.bits() >= minimum.bits()
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

    fn perlin_feature_set() -> FeatureSet {
        crate::NodeBuilder::new("Perlin")
            .unwrap()
            .build()
            .unwrap()
            .get_active_feature_set()
    }

    #[test]
    fn test_detected_by_default() {
        assert_eq!(perlin_feature_set(), FeatureSet::detected());
    }

    #[test]
    fn test_with_max_feature_set() {
        let minimum = FeatureSet::ALL
            .into_iter()
            .find(|feature_set| feature_set.is_compiled_up_to(FeatureSet::detected()))
            .unwrap();

        let (inner, nested) = with_max_feature_set(minimum, || {
            let nested = with_max_feature_set(FeatureSet::detected(), perlin_feature_set).unwrap();
            (perlin_feature_set(), nested)
        })
        .unwrap();

        assert_eq!(inner, minimum);
        assert_eq!(nested, FeatureSet::detected());
        assert_eq!(perlin_feature_set(), FeatureSet::detected());
    }

    /// A typed tree built in the scope has every node at the minimum feature set.
    #[test]
    fn test_with_max_feature_set_generates() {
        use crate::{Generator, nodes::*};

        let minimum = FeatureSet::ALL
            .into_iter()
            .find(|feature_set| feature_set.is_compiled_up_to(FeatureSet::detected()))
            .unwrap();
        let node = with_max_feature_set(minimum, || {
            (perlin().fractal_f_bm().domain_warp_gradient().build() + 1.0).build()
        })
        .unwrap();

        let mut noise = vec![0.0; 64 * 64];
        node.gen_uniform_grid_2d(&mut noise, 0.0, 0.0, 64, 64, 0.1, 0.1, 1337);
        assert_eq!(node.get_active_feature_set(), minimum);
        assert!(noise.iter().all(|value| value.is_finite()));
    }

    /// Generating noise with an input of another feature set crashes FastNoise2.
    #[test]
    fn test_input_feature_set_mismatch() {
        use crate::{Hybrid, NodeBuilder};

        let minimum = FeatureSet::ALL
            .into_iter()
            .find(|feature_set| feature_set.is_compiled_up_to(FeatureSet::detected()))
            .unwrap();
        if minimum == FeatureSet::detected() {
            return;
        }

        let source = NodeBuilder::new("Perlin").unwrap().build().unwrap();
        let (fbm, abs) = with_max_feature_set(minimum, || {
            (
                NodeBuilder::new("FractalFBm")
                    .unwrap()
                    .set("Source", &source),
                NodeBuilder::new("Add")
                    .unwrap()
                    .set("RHS", Hybrid::from(&source)),
            )
        })
        .unwrap();

        for result in [fbm, abs] {
            let Err(FastNoiseError::FeatureSetMismatch {
                expected, found, ..
            }) = result
            else {
                panic!("expected FeatureSetMismatch");
            };
            assert_eq!(expected, minimum);
            assert_eq!(found, FeatureSet::detected());
        }
    }

    /// Typed nodes can't return the error, they panic with it.
    #[test]
    fn test_typed_input_feature_set_mismatch_panics() {
        use std::panic;

        use crate::{Generator, nodes::*};

        let minimum = FeatureSet::ALL
            .into_iter()
            .find(|feature_set| feature_set.is_compiled_up_to(FeatureSet::detected()))
            .unwrap();
        if minimum == FeatureSet::detected() {
            return;
        }

        let source = perlin().build();
        let panic = panic::catch_unwind(|| {
            with_max_feature_set(minimum, || source.fractal_f_bm().build()).unwrap()
        })
        .unwrap_err();

        let message = panic.downcast_ref::<String>().unwrap();
        assert_eq!(
            *message,
            format!(
                "input 'Source' of node 'FractalFBm' has the {} feature set, expected {minimum} \
                 like the node",
                FeatureSet::detected()
            )
        );
    }

    #[test]
    fn test_with_max_feature_set_lowers_to_detected() {
        let highest = FeatureSet::ALL
            .into_iter()
            .rfind(|feature_set| feature_set.is_compiled_up_to(FeatureSet::detected()))
            .unwrap();

        let feature_set = with_max_feature_set(highest, perlin_feature_set).unwrap();
        assert_eq!(feature_set, FeatureSet::detected());
    }

    #[test]
    fn test_with_max_feature_set_not_available() {
        let result = with_max_feature_set(FeatureSet::Scalar, perlin_feature_set);
        assert!(matches!(
            result,
            Err(FastNoiseError::FeatureSetNotAvailable {
                requested: FeatureSet::Scalar,
                ..
            })
        ));
    }

    #[test]
    fn test_from_bits() {
        for feature_set in FeatureSet::ALL {
            assert_eq!(FeatureSet::from_bits(feature_set.bits()), Some(feature_set));
        }
        assert_eq!(FeatureSet::from_bits(0), None);
    }
}
