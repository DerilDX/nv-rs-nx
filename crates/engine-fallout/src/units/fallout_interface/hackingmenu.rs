//! `fallout/interface/menus/hackingmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00700270` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const TEST_TARGET_00700270: u32 = 0x0070_0270;

// Translated from 0076a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (hacking menu, only caller `00769dfe`, which pushes the address
/// of a byte local and tests `AL`): forwards `this` and that pointer to `00700270`, which
/// returns 1 when `007002a0` returns 0 in `AL` and 0 otherwise (`MOVZX; NEG; SBB; ADD 1`). The
/// wrapper returns it as a byte.
pub fn fn_0076a3d0(e: &mut Engine, this: Ptr, byte_address: Ptr) -> bool {
    e.call(TEST_TARGET_00700270, &args![this, byte_address])
        .bool()
}

/// `1280.0` (`double`, `0106e960`).
const SCALE_0106E960: u32 = 0x0106_e960;
/// `004dc1f0` (`renderer.cpp`, no words): the integer at `0118947c` (the screen width
/// the scale is divided by, by its use here; not confirmed).
const SCREEN_SIZE_004DC1F0: u32 = 0x004d_c1f0;

// Translated from 007697f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (hacking menu; `thiscall`, plain `RET`, the result in ST0):
/// the float +0x4ac times 1280.0 (`0106e960`), divided by the integer `004dc1f0`
/// returns, times the float +0x4d0, rounded to a `float`.
pub fn fn_007697f0(e: &mut Engine, this: Ptr) -> f64 {
    let base = this.addr();
    let scale = f64::from_bits(e.mem.u64(SCALE_0106E960));
    let scaled = x87_mul(x87_load32(e.mem.u32(base + 0x4ac)), scale);
    let size = e.call(SCREEN_SIZE_004DC1F0, &args![]).i32();
    // FDIVR double: the stored product over the integer.
    let ratio = x87_div(scaled, f64::from(size));
    let result = x87_store32(x87_mul(ratio, x87_m32(e.mem.u32(base + 0x4d0))));
    x87_load32(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_a3d0, fn_0076a3d0(Ptr, Ptr) -> bool),
        entry!(0x0076_97f0, fn_007697f0(Ptr) -> f64),
    ]
}

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[test]
    fn fn_007697f0_scales_by_1280_over_the_size() {
        let mut e = Engine::new();
        e.map(0x0106_e000, 0x1000);
        e.mem.set_f64(SCALE_0106E960, 1280.0);
        e.register(SCREEN_SIZE_004DC1F0, |_, _| 640u32.into_ret());
        let this = e.mem.alloc(0x4d4);
        e.mem.set_f32(this + 0x4ac, 0.5);
        e.mem.set_f32(this + 0x4d0, 3.0);
        assert_eq!(e.call(0x0076_97f0, &[this]).f64(), 3.0);
        // A negative size divides as a signed integer.
        e.register(SCREEN_SIZE_004DC1F0, |_, _| (-640i32).into_ret());
        assert_eq!(e.call(0x0076_97f0, &[this]).f64(), -3.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0076a3d0_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(TEST_TARGET_00700270, |_, a| (0x100 | (a[1] & 1)).into_ret());
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0076_a3d0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == TEST_TARGET_00700270)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}
