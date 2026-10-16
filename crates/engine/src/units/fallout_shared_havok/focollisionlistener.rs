//! `fallout shared/havok/focollisionlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00624050` calls on the member (`00577bf0`).
const CALLEE_00577BF0: u32 = 0x0057_7bf0;

// Translated from 00624050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00577bf0` with `ECX` = `this + 0x4` and passes on its `float` result (`ST0`).
pub fn fn_00624050(e: &mut Engine, this: Ptr) -> f32 {
    e.call(CALLEE_00577BF0, &args![this.byte_add(0x4)]).f32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_4050, fn_00624050(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00624050_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00577BF0, |_, a| Ret {
            st0: a[0] as f64 / 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_4050, &args![0x4100u32]).f32(), 4161.0);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00577BF0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}
