//! `fallout/ai/run_009c6490` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009c6a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3e0 (`SUB ECX,0x3e0`) and jumps to `008d1130`. `008d1130` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_009c6a20(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x008d_1130, &args![this.addr().wrapping_sub(0x3e0), flags])
        .u32()
}

/// The base constructor `0096a2d0` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
/// `00633c90` and `0066b0d0` (each `this + 0x34` plus one stack word, `RET 4`).
const MEMBER_CONSTRUCTOR_00633C90: u32 = 0x0063_3c90;
const MEMBER_CONSTRUCTOR_0066B0D0: u32 = 0x0066_b0d0;
/// A global word the constructor clears (`.data`).
const CLEARED_GLOBAL_011F21B8: u32 = 0x011f_21b8;

// Translated from 009c6a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009c6a30 - the compiler's exception frame (FS:[0] chain and the
// unwinding that would destroy the member at +0x34) is not translated.
/// Unnamed in the engine map: runs `0096a2d0` on `this`, calls `00633c90(this + 0x34, 0)`,
/// clears the words at `+0x08`, `+0x0c`, `+0x10`, the floats `0.0` at `+0x14` and `+0x18`, the
/// words at `+0x1c` to `+0x2c` and the global word `011f21b8`, calls
/// `0066b0d0(this + 0x34, 0)`, stores the byte 1 at `+0x38` and clears the words at `+0x3c`,
/// `+0x40` and `+0x44`. Returns `this`.
pub fn fn_009c6a30(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR_0096A2D0, &args![this]);
    e.call(MEMBER_CONSTRUCTOR_00633C90, &args![base + 0x34, 0u32]);
    e.mem.set_u32(base + 0x8, 0);
    e.set_global(CLEARED_GLOBAL_011F21B8, 0u32);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x2c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMBER_CONSTRUCTOR_0066B0D0, &args![base + 0x34, 0u32]);
    e.mem.set_u8(base + 0x38, 1);
    e.mem.set_u32(base + 0x3c, 0);
    e.mem.set_u32(base + 0x40, 0);
    e.mem.set_u32(base + 0x44, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_6a20, fn_009c6a20(Ptr, u32) -> u32),
        entry!(0x009c_6a30, fn_009c6a30(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_009c6a20() {
        assert_eq!(
            check(0x009c_6a20, 0x008d_1130, 0x3e0, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn calls_in_order_and_clears_the_fields() {
        let mut e = Engine::new();
        e.map(CLEARED_GLOBAL_011F21B8 & !0xfff, 0x1000);
        e.set_global(CLEARED_GLOBAL_011F21B8, 0x55u32);
        e.register(BASE_CONSTRUCTOR_0096A2D0, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCTOR_00633C90, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCTOR_0066B0D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        for i in 0..0x13u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_6a30, &args![this]).u32(), this);
        let order: Vec<u32> = e
            .call_log
            .clone()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x009c_6a30,
                BASE_CONSTRUCTOR_0096A2D0,
                MEMBER_CONSTRUCTOR_00633C90,
                MEMBER_CONSTRUCTOR_0066B0D0
            ]
        );
        assert_eq!(
            calls_to(&mut e, MEMBER_CONSTRUCTOR_00633C90),
            vec![vec![this + 0x34, 0]]
        );
        assert_eq!(
            calls_to(&mut e, MEMBER_CONSTRUCTOR_0066B0D0),
            vec![vec![this + 0x34, 0]]
        );
        assert_eq!(e.global::<u32>(CLEARED_GLOBAL_011F21B8), 0);
        for offset in [
            0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x3c, 0x40, 0x44,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x38), 1);
        // The words between the cleared ones (+0 to +7, +0x30 to +0x37) are not written here.
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x30), 0xffff_ffff);
    }
}
