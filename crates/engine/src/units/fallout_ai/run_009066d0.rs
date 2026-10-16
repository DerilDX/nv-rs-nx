//! `fallout/ai/run_009066d0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00906dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906dc0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructor of the object `0092c950` extends; `ECX` is the object and the answer is the
/// object. It runs the base constructor (`008d0270`), stores the vtable pointer `0x010886e4`,
/// runs the component constructors (`00906f60` on `+0x38`, `0096a2d0` on `+0x5c`, `+0x6c`,
/// `+0x74` and `+0x7c`, `00937000` on `+0x94`), sets the float at `+0x20` to the float at
/// `0x01012054`, runs `008d03a0`, clears the other fields, runs `00906f80` with 0, 0xff and 0
/// and finishes with `0061cc40` with 3. The engine map gives it no name. The
/// exception-unwinding frame is not translated.
pub fn fn_00906dc0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x008d_0270, &args![this]);
    e.mem.set_u32(base, 0x0108_86e4);
    e.call(0x0090_6f60, &args![base + 0x38]);
    e.mem.set_u32(base + 0x40, 0);
    e.call(0x0096_a2d0, &args![base + 0x5c]);
    e.call(0x0096_a2d0, &args![base + 0x6c]);
    e.call(0x0096_a2d0, &args![base + 0x74]);
    e.call(0x0096_a2d0, &args![base + 0x7c]);
    e.call(0x0093_7000, &args![base + 0x94]);
    let value = x87_quiet(e.global::<f32>(0x0101_2054));
    e.mem.set_f32(base + 0x20, value);
    e.call(0x008d_03a0, &args![this]);
    e.mem.set_f32(base + 0x1c, 0.0);
    for offset in [0x24, 0x64, 0x68, 0x44, 0x84, 0x88, 0x8c, 0x34, 0x58] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_f32(base + 0xa8, 0.0);
    e.mem.set_u32(base + 0x54, 0);
    e.call(0x0090_6f80, &args![this, 0u32, 0xffu32, 0u32]);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_f32(base + 0xac, 0.0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u8(base + 0xb0, 0);
    e.call(0x0061_cc40, &args![this, 3u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0090_6dc0, fn_00906dc0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00906dc0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0090_6dc0,
            size: 0xb4,
            globals: &[(0x0101_2054, 0x3f80_0000)],
            returns: &[],
            calls: &[
                (0x008d_0270, &[A::T(0)]),
                (0x0090_6f60, &[A::T(0x38)]),
                (0x0096_a2d0, &[A::T(0x5c)]),
                (0x0096_a2d0, &[A::T(0x6c)]),
                (0x0096_a2d0, &[A::T(0x74)]),
                (0x0096_a2d0, &[A::T(0x7c)]),
                (0x0093_7000, &[A::T(0x94)]),
                (0x008d_03a0, &[A::T(0)]),
                (0x0090_6f80, &[A::T(0), A::V(0), A::V(0xff), A::V(0)]),
                (0x0061_cc40, &[A::T(0), A::V(3)]),
            ],
            stores: &[
                (0x00, 4, 0x0108_86e4),
                (0x1c, 4, 0),
                (0x20, 4, 0x3f80_0000),
                (0x24, 4, 0),
                (0x34, 4, 0),
                (0x40, 4, 0),
                (0x44, 4, 0),
                (0x48, 4, 0),
                (0x4c, 4, 0),
                (0x54, 4, 0),
                (0x58, 4, 0),
                (0x64, 4, 0),
                (0x68, 4, 0),
                (0x84, 4, 0),
                (0x88, 4, 0),
                (0x8c, 4, 0),
                (0x90, 4, 0),
                (0xa8, 4, 0),
                (0xac, 4, 0),
                (0xb0, 1, 0),
            ],
        });
    }
}
