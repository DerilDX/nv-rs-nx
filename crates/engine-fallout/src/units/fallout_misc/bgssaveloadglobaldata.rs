//! `fallout/misc/saveload/bgssaveloadglobaldata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0084d030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A folded float getter: `FLD dword [this+0xc]`, the value in `ST0`.
/// `Script::Run` (`005ac1e0`) calls it on the timer object at `011f6394`
/// for the frame's elapsed seconds. The `FLD` quiets a signaling NaN.
pub fn fn_0084d030(e: &mut Engine, this: Ptr) -> f64 {
    e.mem.f32(this.addr() + 0xc) as f64
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0084d030, fn_0084d030(Ptr) -> f64)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_0xc_into_st0() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_f32(object + 8, 7.0);
        for value in [0.016_666_668f32, -1.0, 0.0] {
            e.mem.set_f32(object + 0xc, value);
            assert_eq!(e.call(0x0084d030, &[object]).f64(), value as f64);
        }
    }
}
