//! `fallout shared/sky/precipitation.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00639b00` calls on the member (`009938b0`).
const CALLEE_009938B0: u32 = 0x0099_38b0;

// Translated from 00639b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `009938b0` with `ECX` = `this + 0x54` and passes on its 16-bit result.
pub fn fn_00639b00(e: &mut Engine, this: Ptr) -> u16 {
    e.call(CALLEE_009938B0, &args![this.byte_add(0x54)]).u16()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063_9b00, fn_00639b00(Ptr) -> u16)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00639b00_passes_this_plus_54_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_009938B0, |_, a| Ret {
            eax: a[0] | 0x5_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_9b00, &args![0x4100u32]).u16(), 0x4154);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_009938B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x54]);
    }
}
