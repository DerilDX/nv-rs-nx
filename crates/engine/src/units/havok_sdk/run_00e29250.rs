//! `Havok SDK/run_00e29250` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__time64` (`00ecadda`): cdecl, a `__time64_t*` (null here); returns the
/// 64-bit time in EDX:EAX.
const TIME64: u32 = 0x00ec_adda;

// Translated from 00e29910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Returns `__time64(0)`.
pub fn fn_00e29910(e: &mut Engine) -> u64 {
    e.call(TIME64, &args![0u32]).u64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e29910, fn_00e29910() -> u64)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e29910_returns_the_time_of_a_null_pointer_call() {
        let mut e = Engine::new();
        e.register(TIME64, |_, a| {
            assert_eq!(a, [0]);
            Ret {
                eax: 0x1234_5678,
                edx: 0x9,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00e2_9910, &args![]).u64(), 0x9_1234_5678);
    }
}
