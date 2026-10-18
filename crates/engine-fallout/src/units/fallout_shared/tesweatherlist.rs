//! `fallout shared/tesweatherlist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004f10b0` calls on the member (`005827d0`).
const CALLEE_005827D0: u32 = 0x0058_27d0;

// Translated from 004f10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESWeatherList::GetRandomWeather` (Xbox PDB name in the engine map). Thin wrapper: calls `005827d0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_004f10b0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_005827D0, &args![this.byte_add(0x8)]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_10b0, fn_004f10b0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004f10b0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_005827D0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_10b0, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_005827D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
}
