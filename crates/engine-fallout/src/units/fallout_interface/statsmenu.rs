//! `fallout/interface/menus/statsmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007df1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range assigned to `statsmenu.cpp`; `fastcall`/`thiscall`
/// with `this` in `ECX`, plain `RET`): returns the float at `this + 0x20` in `ST0`
/// (`FLD`, which quiets a signalling NaN). The process classes call it as
/// `fHourLastProcessed` (Xbox PDB) of a `LowProcess`; the body is a plain field getter, so
/// it is named by address and takes any object.
pub fn fn_007df1f0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x20)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007d_f1f0, fn_007df1f0(Ptr) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_float_at_0x20() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x20, 13.25f32.to_bits());
        assert_eq!(e.call(0x007d_f1f0, &args![object]).f32(), 13.25);
    }

    #[test]
    fn quiets_a_signalling_nan() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x20, 0x7fa0_0001);
        let bits = e.call(0x007d_f1f0, &args![object]).f32().to_bits();
        assert_eq!(bits, 0x7fe0_0001);
    }
}
