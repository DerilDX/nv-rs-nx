//! `fallout shared/run_00519cb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00519cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00519cc0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x67 (0xb0 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_b5ac`) is `TESObjectIMOD`, so this is the constructor of `TESObjectIMOD`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00519cc0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_2c50, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x7c]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0047_5350, &args![base + 0x8c]);
    e.call(0x0047_a130, &args![base + 0x94]);
    e.call(0x0047_b4f0, &args![base + 0xa4]);
    e.mem.set_u32(base, 0x0102_b5ac);
    e.mem.set_u32(base + 0x30, 0x0102_b594);
    e.mem.set_u32(base + 0x3c, 0x0102_b570);
    e.mem.set_u32(base + 0x5c, 0x0102_b550);
    e.mem.set_u32(base + 0x68, 0x0102_b53c);
    e.mem.set_u32(base + 0x74, 0x0102_b524);
    e.mem.set_u32(base + 0x7c, 0x0102_b504);
    e.mem.set_u32(base + 0x84, 0x0102_b4f0);
    e.mem.set_u32(base + 0x8c, 0x0102_b4dc);
    e.mem.set_u32(base + 0x94, 0x0102_b4c8);
    e.mem.set_u32(base + 0xa4, 0x0102_b4b4);
    e.call(0x004f_15a0, &args![this, 0x67u32]);
    e.call(0x0048_4ab0, &args![this, 0x1u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_9cc0, fn_00519cc0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00519cc0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_9cc0,
            size: 0xb0,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_2c50, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x7c)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0047_5350, &[A::T(0x8c)]),
                (0x0047_a130, &[A::T(0x94)]),
                (0x0047_b4f0, &[A::T(0xa4)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x67)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b5ac),
                (0x30, 4, 0x0102_b594),
                (0x3c, 4, 0x0102_b570),
                (0x5c, 4, 0x0102_b550),
                (0x68, 4, 0x0102_b53c),
                (0x74, 4, 0x0102_b524),
                (0x7c, 4, 0x0102_b504),
                (0x84, 4, 0x0102_b4f0),
                (0x8c, 4, 0x0102_b4dc),
                (0x94, 4, 0x0102_b4c8),
                (0xa4, 4, 0x0102_b4b4),
            ],
        });
    }
}
