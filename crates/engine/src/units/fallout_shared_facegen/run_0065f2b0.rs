//! `fallout shared/facegen/run_0065f2b0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0065f580` calls on the member (`0065f7c0`).
const CALLEE_0065F7C0: u32 = 0x0065_f7c0;

/// The function `fn_0065f6e0` calls on the member (`0065f7f0`).
const CALLEE_0065F7F0: u32 = 0x0065_f7f0;

// Translated from 0065f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7c0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f580(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7C0, &args![this.byte_add(0x8)]).u32()
}

// Translated from 0065f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7f0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f6e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7F0, &args![this.byte_add(0x8)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_f580, fn_0065f580(Ptr) -> u32),
        entry!(0x0065_f6e0, fn_0065f6e0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0065f580_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f580, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }

    #[test]
    fn l68d_0065f6e0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7F0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f6e0, &args![0x4200u32]).u32(), 0x175d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x8]);
    }
}
