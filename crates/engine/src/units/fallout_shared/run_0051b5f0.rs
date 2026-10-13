//! `fallout shared/run_0051b5f0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b6e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form
/// type 0x25 (0x94 bytes); `ECX` is the object. The engine map gives it no
/// name (a `range-sub` function), so the class is unconfirmed. It runs the
/// base and component constructors by address, stores the vtable pointers,
/// registers the form type (`004f15a0`), copies six floats from globals
/// (`010184f4`, `0101712c`, `0102bc30`, `01016248`, `01018238` into
/// `+0x6c..+0x7c`), sets `+0x84` and `+0x88` to `1.0`, `+0x80` to `0x80`, and
/// copies the words at `011f4980` and `011f4984` into `+0x8c` and `+0x90`.
/// The exception-unwinding frame is not translated.
pub fn fn_0051b6e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f7a0, &args![this]);
    e.call(0x0051_b930, &args![base + 0x30]);
    e.call(0x0051_b970, &args![base + 0x48]);
    e.call(0x0047_5350, &args![base + 0x54]);
    e.mem.set_u32(base, 0x0102_bc94);
    e.mem.set_u32(base + 0x30, 0x0102_bc6c);
    e.mem.set_u32(base + 0x48, 0x0102_bc4c);
    e.mem.set_u32(base + 0x54, 0x0102_bc38);
    e.call(0x0047_4790, &args![base + 0x5c, 0u32, 1u32]);
    e.call(0x0068_15c0, &args![base + 0x8c]);
    e.call(0x004f_15a0, &args![this, 0x25u32]);
    for (offset, global) in [
        (0x6c, 0x0101_84f4),
        (0x70, 0x0101_712c),
        (0x74, 0x0102_bc30),
        (0x78, 0x0101_6248),
        (0x7c, 0x0101_8238),
    ] {
        let value = e.global::<f32>(global);
        e.mem.set_f32(base + offset, value);
    }
    e.mem.set_f32(base + 0x84, 1.0);
    e.mem.set_f32(base + 0x88, 1.0);
    e.mem.set_u32(base + 0x80, 0x80);
    let first = e.global::<u32>(0x011f_4980);
    e.mem.set_u32(base + 0x8c, first);
    let second = e.global::<u32>(0x011f_4984);
    e.mem.set_u32(base + 0x90, second);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0051_b6e0, fn_0051b6e0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051b6e0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b6e0,
            size: 0x94,
            globals: &[
                (0x0101_84f4, 0x1000000),
                (0x0101_712c, 0x1000001),
                (0x0102_bc30, 0x1000002),
                (0x0101_6248, 0x1000003),
                (0x0101_8238, 0x1000004),
                (0x011f_4980, 0x1000005),
                (0x011f_4984, 0x1000006),
            ],
            returns: &[],
            calls: &[
                (0x0050_f7a0, &[A::T(0x0)]),
                (0x0051_b930, &[A::T(0x30)]),
                (0x0051_b970, &[A::T(0x48)]),
                (0x0047_5350, &[A::T(0x54)]),
                (0x0047_4790, &[A::T(0x5c), A::V(0x0), A::V(0x1)]),
                (0x0068_15c0, &[A::T(0x8c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x25)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_bc94),
                (0x30, 4, 0x0102_bc6c),
                (0x48, 4, 0x0102_bc4c),
                (0x54, 4, 0x0102_bc38),
                (0x6c, 4, 0x1000000),
                (0x70, 4, 0x1000001),
                (0x74, 4, 0x1000002),
                (0x78, 4, 0x1000003),
                (0x7c, 4, 0x1000004),
                (0x84, 4, 0x3f80_0000),
                (0x88, 4, 0x3f80_0000),
                (0x80, 4, 0x80),
                (0x8c, 4, 0x1000005),
                (0x90, 4, 0x1000006),
            ],
        });
    }
}
