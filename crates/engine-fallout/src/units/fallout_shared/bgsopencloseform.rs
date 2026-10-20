//! `fallout shared/bgsopencloseform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual slot (byte offset) of the target that takes a `float` and a zero.
const SLOT_FLOAT_AND_ZERO: u32 = 0x8c;

// Translated from 0047b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bgsopencloseform.cpp`). `thiscall`, `RET 8` (an object and a
/// `float`); the `this` register is saved and never used. Calls virtual slot +0x8c of the
/// object with the `float` (copied with `FLD`/`FSTP`, so a signalling NaN is passed quiet) and
/// a zero, in that order. The decompiler's third argument (`this`) is not in the disassembly.
pub fn fn_0047b220(e: &mut Engine, _unused_this: Ptr, object: Ptr, value: f32) {
    e.vcall(
        object.addr(),
        SLOT_FLOAT_AND_ZERO,
        &args![x87_quiet_bits(value.to_bits()), 0u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047_b220, fn_0047b220(Ptr, Ptr, f32))]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 0x30];
        slots[SLOT_FLOAT_AND_ZERO as usize / 4] = 0x0f00_2000;
        e.put_vtable(0x0f00_2100, &slots);
        e.register(0x0f00_2000, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_2100);
        (e, object)
    }

    fn slot_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x0f00_2000)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0047b220_calls_slot_0x8c_with_the_float_and_zero() {
        let (mut e, object) = rig();
        e.call(0x0047_b220, &args![0x1111u32, object, 0.75f32]);
        assert_eq!(slot_calls(&e), vec![vec![object, 0.75f32.to_bits(), 0]]);
    }

    #[test]
    fn at_0047b220_passes_a_signalling_nan_quiet() {
        let (mut e, object) = rig();
        e.call(
            0x0047_b220,
            &args![0u32, object, f32::from_bits(0x7f80_0001)],
        );
        assert_eq!(slot_calls(&e), vec![vec![object, 0x7fc0_0001, 0]]);
    }
}
