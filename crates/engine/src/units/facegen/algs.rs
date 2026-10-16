//! `FaceGen/algs.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00acbb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `algs.obj`). Thiscall that ignores
/// `this` and returns 0.
pub fn fn_00acbb70(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00acbb70, fn_00acbb70(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ac_bb70, &args![Ptr::<()>::new(0x10)]).u32(), 0);
    }
}
