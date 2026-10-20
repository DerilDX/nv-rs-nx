//! `BSShader/run_00bc0360` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// This effect's vtable (`010ba350`; no RTTI locator before it) and the four `float`s it sets
/// to 1.0 (`01201f30` to `01201f3c`).
const VTABLE_010BA350: u32 = 0x010b_a350;
const FLOATS_01201F30: u32 = 0x0120_1f30;

// Translated from 00bc0600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010ba350`, sizes the array at +0x28 to 5 (`00a90180`), clears the byte +4 and stores 1.0
/// (`FLD1`, through four stack words) in the `float`s `01201f30`, `01201f3c`, `01201f34` and
/// `01201f38`, in that order. Answers `this`.
// Translation incomplete: 00bc0600 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bc0600(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BA350);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.mem.set_u8(base + 4, 0);
    for index in [0u32, 3, 1, 2] {
        e.set_global(FLOATS_01201F30 + 4 * index, 1.0f32);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_0600, fn_00bc0600(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc0600_builds_the_effect_and_sets_the_floats() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.map(0x0120_1000, 0x1000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 8]);
        assert_eq!(e.call(0x00bc_0600, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BA350);
        assert_eq!(e.mem.u8(this + 4), 0);
        for index in 0..4 {
            assert_eq!(e.global::<f32>(FLOATS_01201F30 + 4 * index), 1.0);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 5]),
            ]
        );
    }
}
