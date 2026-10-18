//! `fallout shared/tesloadscreen.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a66e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a66e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESLoadScreen::TESLoadScreen` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x4b (0x40 bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesloadscreen_tesloadscreen(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_e270, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0103_66cc);
    e.mem.set_u32(base + 0x18, 0x0103_66ac);
    e.mem.set_u32(base + 0x24, 0x0103_6694);
    e.call(0x0096_a2d0, &args![base + 0x2c]);
    e.call(0x0040_37b0, &args![base + 0x38]);
    e.call(0x004f_15a0, &args![this, 0x4b]);
    e.call(0x0048_4ab0, &args![this, 0x1]);
    e.mem.set_u32(base + 0x34, 0x0);
    this
}

/// Byte-swaps the `u32` at the address in place (`00401080`, `cdecl`: the
/// address and a byte the callers push as 0).
const SWAP_WORD_IN_PLACE: u32 = 0x0040_1080;

// Translated from 005a6b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the three words at `+0`, `+4` and `+8` in place, each through
/// `00401080` with 0 as its second word (unnamed in the map; `ECX`, no
/// stack words). The file header reader (`00471400`) calls it on the
/// 12-byte `HEDR` data it read into its `TESFile` (`+0x3dc`) when the file
/// needs its byte order converted.
pub fn fn_005a6b10(e: &mut Engine, this: Ptr) {
    for offset in [0u32, 4, 8] {
        e.call(SWAP_WORD_IN_PLACE, &args![this.byte_add(offset), 0u32]);
    }
}

// Translated from 005a67b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a67e0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a67b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_67e0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_66e0, tesloadscreen_tesloadscreen(Ptr) -> Ptr),
        entry!(0x005a6b10, fn_005a6b10(Ptr)),
        entry!(0x005a_67b0, fn_005a67b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesloadscreen_tesloadscreen_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_66e0,
            size: 0x40,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_e270, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x2c)]),
                (0x0040_37b0, &[A::T(0x38)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x4b)]),
                (0x0048_4ab0, &[A::T(0x0), A::V(0x1)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_66cc),
                (0x18, 4, 0x0103_66ac),
                (0x24, 4, 0x0103_6694),
                (0x34, 4, 0x0),
            ],
        });
    }

    #[test]
    fn swaps_the_three_words_in_order() {
        use super::*;
        let mut e = Engine::new();
        e.register(SWAP_WORD_IN_PLACE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_6b10, &args![Ptr::<()>::new(0x0a00_0000)]);
        let swaps: Vec<Vec<u32>> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == SWAP_WORD_IN_PLACE)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            swaps,
            vec![
                vec![0x0a00_0000, 0],
                vec![0x0a00_0004, 0],
                vec![0x0a00_0008, 0],
            ]
        );
    }
    #[test]
    fn at_005a67b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_67e0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_67b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_67b0)
                .collect();
            let mut expected = vec![(0x005a_67e0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
