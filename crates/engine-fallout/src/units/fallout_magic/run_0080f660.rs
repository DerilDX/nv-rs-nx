//! `fallout/magic/run_0080f660` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Returns a pointer to a record inside the object (`this + 0x24`; the engine
/// map files it under `fallout shared/extradataobjects.cpp`).
const OBJECT_RECORD_POINTER: u32 = 0x0043_0830;

// Translated from 0080f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: `__fastcall` with one stack word it never reads. Asks
/// `00430830` for the record pointer of `this` and returns the `float` at
/// `+8` of that record (loaded into `ST0`, so a signalling NaN comes back
/// quiet).
pub fn fn_0080f790(e: &mut Engine, this: Ptr, _unused_1: u32) -> f32 {
    let record = e.call(OBJECT_RECORD_POINTER, &args![this]).u32();
    x87_quiet(e.mem.f32(record + 8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080f790, fn_0080f790(Ptr, u32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_float_at_offset_8_of_the_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x10);
        e.mem.set_f32(record + 8, 2.5);
        e.register_double(OBJECT_RECORD_POINTER, move |_, a| {
            assert_eq!(a[0], 0x1234);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let value = e.call(0x0080f790, &args![0x1234u32, 0u32]).f32();
        assert_eq!(value, 2.5);
        // A signalling NaN is quieted by the FLD.
        e.mem.set_u32(record + 8, 0x7f80_0001);
        let nan = e.call(0x0080f790, &args![0x1234u32, 0u32]).f32();
        assert!(nan.is_nan());
    }
}
