//! `fallout shared/run_00595310` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00595310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00595310 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x70 (0xb8 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_40c4`) is `MediaLocationController`, so this is the constructor of `MediaLocationController`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the floats at `01026994` and `010340a4` into `+0x70` and `+0x74`.
/// The exception-unwinding frame is not translated.
pub fn fn_00595310(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_40c4);
    e.mem.set_u32(base + 0x18, 0x0103_40ac);
    e.call(0x0040_2d00, &args![base + 0x54]);
    e.call(0x0096_a2d0, &args![base + 0x88]);
    e.call(0x0096_a2d0, &args![base + 0x90]);
    e.call(0x0096_a2d0, &args![base + 0x98]);
    e.call(0x0096_a2d0, &args![base + 0xa0]);
    e.call(0x0096_a2d0, &args![base + 0xa8]);
    e.call(0x0096_a2d0, &args![base + 0xb0]);
    e.call(0x004f_15a0, &args![this, 0x70u32]);
    e.mem.set_u32(base + 0x6c, 0x4);
    let f1 = e.global::<f32>(0x0102_6994);
    e.mem.set_f32(base + 0x70, f1);
    let f2 = e.global::<f32>(0x0103_40a4);
    e.mem.set_f32(base + 0x74, f2);
    e.mem.set_f32(base + 0x7c, 0.0);
    e.mem.set_u32(base + 0x80, 0x40);
    e.mem.set_u32(base + 0x84, 0x150);
    e.call(0x0047_0470, &args![base + 0x88]);
    e.call(0x0047_0470, &args![base + 0x90]);
    e.call(0x0047_0470, &args![base + 0x98]);
    e.call(0x0047_0470, &args![base + 0xa0]);
    e.call(0x0047_0470, &args![base + 0xa8]);
    e.call(0x0047_0470, &args![base + 0xb0]);
    e.mem.set_u32(base + 0x60, 0x0);
    e.mem.set_u32(base + 0x64, 0x1);
    e.mem.set_u32(base + 0x68, 0x0);
    e.mem.set_u32(base + 0x24, 0x0);
    e.mem.set_u32(base + 0x28, 0x0);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.mem.set_u32(base + 0x30, 0x0);
    e.mem.set_u8(base + 0x35, 0x0);
    e.mem.set_u8(base + 0x36, 0x0);
    e.mem.set_u8(base + 0x37, 0x0);
    e.mem.set_u8(base + 0x38, 0x0);
    e.mem.set_u8(base + 0x4c, 0x0);
    e.mem.set_u32(base + 0x50, 0x0);
    e.mem.set_u32(base + 0x40, 0x0);
    e.mem.set_u32(base + 0x44, 0x0);
    e.mem.set_u32(base + 0x48, 0x1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0059_5310, fn_00595310(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00595310_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_5310,
            size: 0xb8,
            globals: &[(0x0102_6994, 0x0100_0001), (0x0103_40a4, 0x0100_0002)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2d00, &[A::T(0x54)]),
                (0x0096_a2d0, &[A::T(0x88)]),
                (0x0096_a2d0, &[A::T(0x90)]),
                (0x0096_a2d0, &[A::T(0x98)]),
                (0x0096_a2d0, &[A::T(0xa0)]),
                (0x0096_a2d0, &[A::T(0xa8)]),
                (0x0096_a2d0, &[A::T(0xb0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x70)]),
                (0x0047_0470, &[A::T(0x88)]),
                (0x0047_0470, &[A::T(0x90)]),
                (0x0047_0470, &[A::T(0x98)]),
                (0x0047_0470, &[A::T(0xa0)]),
                (0x0047_0470, &[A::T(0xa8)]),
                (0x0047_0470, &[A::T(0xb0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_40c4),
                (0x18, 4, 0x0103_40ac),
                (0x6c, 4, 0x4),
                (0x70, 4, 0x0100_0001),
                (0x74, 4, 0x0100_0002),
                (0x7c, 4, 0x0),
                (0x80, 4, 0x40),
                (0x84, 4, 0x150),
                (0x60, 4, 0x0),
                (0x64, 4, 0x1),
                (0x68, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0),
                (0x2c, 4, 0x0),
                (0x30, 4, 0x0),
                (0x35, 1, 0x0),
                (0x36, 1, 0x0),
                (0x37, 1, 0x0),
                (0x38, 1, 0x0),
                (0x4c, 1, 0x0),
                (0x50, 4, 0x0),
                (0x40, 4, 0x0),
                (0x44, 4, 0x0),
                (0x48, 4, 0x1),
            ],
        });
    }
}
