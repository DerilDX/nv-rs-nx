//! `NiMain/run_00a8ff30` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `011f5e44`: the global array of material libraries (data pointer; count at `011f5e4c`).
const LIBRARY_LIST: u32 = 0x011f_5e44;
/// `00a311e0` (`thiscall`, `RET 4`, engine map `NiTSet<...>::Realloc`): gives the array the
/// capacity given.
const LIST_REALLOC: u32 = 0x00a3_11e0;

// Translated from 00a8ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): empties the global array of material libraries at `011f5e44` (count at
/// `011f5e4c` set to 0) and gives it capacity 0 (`00a311e0`).
pub fn fn_00a8ff30(e: &mut Engine) {
    e.mem.set_u32(LIBRARY_LIST + 8, 0);
    e.call(LIST_REALLOC, &args![LIBRARY_LIST, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_ff30, fn_00a8ff30())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_the_list_then_reallocates_it_to_nothing() {
        let mut e = Engine::new();
        e.map(LIBRARY_LIST & !0xfff, 0x1000);
        e.mem.set_u32(LIBRARY_LIST + 8, 3);
        e.register(LIST_REALLOC, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 8), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a8_ff30, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(LIST_REALLOC, vec![LIBRARY_LIST, 0])]);
    }
}
