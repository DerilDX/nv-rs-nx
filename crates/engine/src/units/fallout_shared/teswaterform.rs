//! `fallout shared/teswaterform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0057eba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0057eba0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x4e (0x194 bytes); `ECX` is the object.
/// The engine map names it `TESWaterForm::TESWaterForm` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_140c`) is `TESWaterForm`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It copies the floats at `01016248` (three times) and `010162c0` into `+0x168..+0x174`.
/// The exception-unwinding frame is not translated.
pub fn fn_0057eba0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0047_fdc0, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_140c);
    e.mem.set_u32(base + 0x18, 0x0103_13f4);
    e.mem.set_u32(base + 0x24, 0x0103_13dc);
    e.call(0x0063_3c90, &args![base + 0x30, 0x0u32]);
    e.call(
        0x0041_4430,
        &args![base + 0x34, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(
        0x0041_4430,
        &args![base + 0x44, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(
        0x0041_4430,
        &args![base + 0x54, 0x0u32, 0x0u32, 0x0u32, 0x0u32],
    );
    e.call(0x0048_e270, &args![base + 0x64]);
    e.call(0x0040_37b0, &args![base + 0x74]);
    e.call(0x0057_e9a0, &args![base + 0x84]);
    e.call(0x0063_3c90, &args![base + 0x17c, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x4eu32]);
    e.call(0x0057_ef10, &args![this]);
    e.mem.set_u32(base + 0x154, 0x0);
    e.mem.set_u32(base + 0x158, 0x0);
    e.mem.set_u32(base + 0x15c, 0x9);
    e.mem.set_u32(base + 0x160, 0x9);
    e.mem.set_u32(base + 0x164, 0x4);
    let f1 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x168, f1);
    let f2 = e.global::<f32>(0x0101_62c0);
    e.mem.set_f32(base + 0x16c, f2);
    let f3 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x170, f3);
    let f4 = e.global::<f32>(0x0101_6248);
    e.mem.set_f32(base + 0x174, f4);
    e.mem.set_u32(base + 0x178, 0x0);
    e.mem.set_u8(base + 0x2c, 0x1);
    e.mem.set_u8(base + 0x190, 0x0);
    e.mem.set_u32(base + 0x180, 0x0);
    e.mem.set_u32(base + 0x184, 0x0);
    e.mem.set_u32(base + 0x188, 0x0);
    e.mem.set_u32(base + 0x18c, 0x0);
    e.mem.set_u32(base + 0x80, 0x0);
    this
}

// Translated from 0057ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0057ee10` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0057ede0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0057_ee10, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0057_eba0, fn_0057eba0(Ptr) -> Ptr),
        entry!(0x0057_ede0, fn_0057ede0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0057eba0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0057_eba0,
            size: 0x194,
            globals: &[(0x0101_6248, 0x0100_0001), (0x0101_62c0, 0x0100_0002)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0047_fdc0, &[A::T(0x24)]),
                (0x0063_3c90, &[A::T(0x30), A::V(0x0)]),
                (
                    0x0041_4430,
                    &[A::T(0x34), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (
                    0x0041_4430,
                    &[A::T(0x44), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (
                    0x0041_4430,
                    &[A::T(0x54), A::V(0x0), A::V(0x0), A::V(0x0), A::V(0x0)],
                ),
                (0x0048_e270, &[A::T(0x64)]),
                (0x0040_37b0, &[A::T(0x74)]),
                (0x0057_e9a0, &[A::T(0x84)]),
                (0x0063_3c90, &[A::T(0x17c), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4e)]),
                (0x0057_ef10, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_140c),
                (0x18, 4, 0x0103_13f4),
                (0x24, 4, 0x0103_13dc),
                (0x154, 4, 0x0),
                (0x158, 4, 0x0),
                (0x15c, 4, 0x9),
                (0x160, 4, 0x9),
                (0x164, 4, 0x4),
                (0x168, 4, 0x0100_0001),
                (0x16c, 4, 0x0100_0002),
                (0x170, 4, 0x0100_0001),
                (0x174, 4, 0x0100_0001),
                (0x178, 4, 0x0),
                (0x2c, 1, 0x1),
                (0x190, 1, 0x0),
                (0x180, 4, 0x0),
                (0x184, 4, 0x0),
                (0x188, 4, 0x0),
                (0x18c, 4, 0x0),
                (0x80, 4, 0x0),
            ],
        });
    }
    #[test]
    fn at_0057ede0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0057_ee10, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0057_ede0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0057_ede0)
                .collect();
            let mut expected = vec![(0x0057_ee10, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
