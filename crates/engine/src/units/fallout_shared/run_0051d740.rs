//! `fallout shared/run_0051d740` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051d770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051d770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x28 (0x388 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_c51c`) is `TESObjectWEAP`, so this is the constructor of `TESObjectWEAP`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051d770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_30a0, &args![base + 0x74]);
    e.call(0x0048_e7a0, &args![base + 0x84]);
    e.call(0x0048_eac0, &args![base + 0x8c]);
    e.call(0x0048_71d0, &args![base + 0x94]);
    e.call(0x0047_fdc0, &args![base + 0x9c]);
    e.call(0x0047_48f0, &args![base + 0xa4]);
    e.call(0x0047_50b0, &args![base + 0xac]);
    e.call(0x0047_5350, &args![base + 0xb4]);
    e.call(0x0047_b850, &args![base + 0xbc]);
    e.call(0x0047_9320, &args![base + 0xc4]);
    e.call(0x004f_9d90, &args![base + 0xcc]);
    e.call(0x0047_a130, &args![base + 0xd0]);
    e.call(0x0047_4d10, &args![base + 0xe0]);
    e.call(0x0047_b4f0, &args![base + 0xe8]);
    e.mem.set_u32(base, 0x0102_c51c);
    e.mem.set_u32(base + 0x30, 0x0102_c504);
    e.mem.set_u32(base + 0x3c, 0x0102_c4e0);
    e.mem.set_u32(base + 0x5c, 0x0102_c4c0);
    e.mem.set_u32(base + 0x68, 0x0102_c4ac);
    e.mem.set_u32(base + 0x74, 0x0102_c494);
    e.mem.set_u32(base + 0x84, 0x0102_c474);
    e.mem.set_u32(base + 0x8c, 0x0102_c460);
    e.mem.set_u32(base + 0x94, 0x0102_c448);
    e.mem.set_u32(base + 0x9c, 0x0102_c430);
    e.mem.set_u32(base + 0xa4, 0x0102_c41c);
    e.mem.set_u32(base + 0xac, 0x0102_c408);
    e.mem.set_u32(base + 0xb4, 0x0102_c3f4);
    e.mem.set_u32(base + 0xbc, 0x0102_c3e0);
    e.mem.set_u32(base + 0xc4, 0x0102_c3cc);
    e.mem.set_u32(base + 0xcc, 0x0102_c3b4);
    e.mem.set_u32(base + 0xd0, 0x0102_c3a0);
    e.mem.set_u32(base + 0xe0, 0x0102_c38c);
    e.mem.set_u32(base + 0xe8, 0x0102_c378);
    e.call(0x0048_8f50, &args![base + 0x1d0]);
    e.call(0x0048_8f50, &args![base + 0x1e8]);
    e.call(0x0048_8f50, &args![base + 0x200]);
    e.call(0x0048_a3d0, &args![base + 0x270]);
    e.call(0x0048_a3d0, &args![base + 0x290]);
    e.call(0x0048_a3d0, &args![base + 0x2b0]);
    e.call(0x0048_a3d0, &args![base + 0x2d0]);
    e.call(0x0048_a3d0, &args![base + 0x2f0]);
    e.call(0x0048_a3d0, &args![base + 0x310]);
    e.call(0x0048_a3d0, &args![base + 0x330]);
    e.call(0x0040_37b0, &args![base + 0x35c]);
    e.call(0x0040_37b0, &args![base + 0x368]);
    e.call(0x004f_15a0, &args![this, 0x28u32]);
    e.call(0x0051_ddf0, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_d770, fn_0051d770(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051d770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_d770,
            size: 0x388,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_30a0, &[A::T(0x74)]),
                (0x0048_e7a0, &[A::T(0x84)]),
                (0x0048_eac0, &[A::T(0x8c)]),
                (0x0048_71d0, &[A::T(0x94)]),
                (0x0047_fdc0, &[A::T(0x9c)]),
                (0x0047_48f0, &[A::T(0xa4)]),
                (0x0047_50b0, &[A::T(0xac)]),
                (0x0047_5350, &[A::T(0xb4)]),
                (0x0047_b850, &[A::T(0xbc)]),
                (0x0047_9320, &[A::T(0xc4)]),
                (0x004f_9d90, &[A::T(0xcc)]),
                (0x0047_a130, &[A::T(0xd0)]),
                (0x0047_4d10, &[A::T(0xe0)]),
                (0x0047_b4f0, &[A::T(0xe8)]),
                (0x0048_8f50, &[A::T(0x1d0)]),
                (0x0048_8f50, &[A::T(0x1e8)]),
                (0x0048_8f50, &[A::T(0x200)]),
                (0x0048_a3d0, &[A::T(0x270)]),
                (0x0048_a3d0, &[A::T(0x290)]),
                (0x0048_a3d0, &[A::T(0x2b0)]),
                (0x0048_a3d0, &[A::T(0x2d0)]),
                (0x0048_a3d0, &[A::T(0x2f0)]),
                (0x0048_a3d0, &[A::T(0x310)]),
                (0x0048_a3d0, &[A::T(0x330)]),
                (0x0040_37b0, &[A::T(0x35c)]),
                (0x0040_37b0, &[A::T(0x368)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x28)]),
                (0x0051_ddf0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_c51c),
                (0x30, 4, 0x0102_c504),
                (0x3c, 4, 0x0102_c4e0),
                (0x5c, 4, 0x0102_c4c0),
                (0x68, 4, 0x0102_c4ac),
                (0x74, 4, 0x0102_c494),
                (0x84, 4, 0x0102_c474),
                (0x8c, 4, 0x0102_c460),
                (0x94, 4, 0x0102_c448),
                (0x9c, 4, 0x0102_c430),
                (0xa4, 4, 0x0102_c41c),
                (0xac, 4, 0x0102_c408),
                (0xb4, 4, 0x0102_c3f4),
                (0xbc, 4, 0x0102_c3e0),
                (0xc4, 4, 0x0102_c3cc),
                (0xcc, 4, 0x0102_c3b4),
                (0xd0, 4, 0x0102_c3a0),
                (0xe0, 4, 0x0102_c38c),
                (0xe8, 4, 0x0102_c378),
            ],
        });
    }
}
