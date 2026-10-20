//! `BSShader/imagespaceeffecthdr.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The HDR effect's vtable (`010b8c8c`; no RTTI locator before it) and the address it keeps at
/// +0x6c (`011add0c`).
const VTABLE_010B8C8C: u32 = 0x010b_8c8c;
const DATA_011ADD0C: u32 = 0x011a_dd0c;

// Translated from 00bad500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`): the HDR image-space effect's
/// constructor, `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the
/// vtable `010b8c8c` and the address `011add0c` at +0x6c, and sizes the array at +0x28 to 3
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00bad500 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bad500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8C8C);
    e.mem.set_u32(base + 0x6c, DATA_011ADD0C);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_d500, fn_00bad500(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bad500_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00ba_d500, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8C8C);
        assert_eq!(e.mem.u32(this + 0x6c), DATA_011ADD0C);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 3]),
            ]
        );
    }
}
