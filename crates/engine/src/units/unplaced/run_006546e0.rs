//! `(unplaced)/run_006546e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006546e0` calls on the member (`00559450`).
const CALLEE_00559450: u32 = 0x0055_9450;

// Translated from 006546e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00559450` with `ECX` = `this + 0xd0` and passes on its result.
pub fn fn_006546e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(0xd0)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_46e0, fn_006546e0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006546e0_passes_this_plus_d0_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_46e0, &args![0x4100u32]).u32(), 0x1485);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00559450).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xd0]);
    }
}
