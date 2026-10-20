//! `(unplaced)/run_004e0050` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A class tag made from `"SPD"` and freed again, and a per-frame update
//! that ages a list of nodes' timers (their classes are not confirmed).

#[allow(unused_imports)]
use crate::prelude::*;

/// The tag `004e0050` makes and `004e00d0` frees, and its string (`"SPD"`).
const SPD_TAG: u32 = 0x011c_7830;
const SPD_TAG_TEXT: u32 = 0x0102_2a18;
/// The byte that enables `004e0110`'s update.
const UPDATE_ENABLED: u32 = 0x011c_7834;
/// The pointer (read through `00559450`) to the object whose nodes
/// `004e0110` walks.
const NODE_OWNER: u32 = 0x011c_786c;
/// A running total the update lowers by an expired entry's word at `+0xC`.
const RUNNING_TOTAL: u32 = 0x011c_7828;
/// The frame timer object (`0084d030` on it gives the frame's seconds).
const FRAME_TIMER: u32 = 0x011f_6394;
/// Settings whose floats the update reads (`00403e20` gives the address).
const FADE_SETTING: u32 = 0x011c_fc2c;
const SETTING_A: u32 = 0x011c_eeb4;
const SETTING_B: u32 = 0x011c_f6a8;
/// `1.0` and `0.4` (`double`).
const ONE: u32 = 0x0101_2070;
const RATE_DIVISOR: u32 = 0x0102_28a0;

const FLOAT_ZERO: u32 = 0;
const FLOAT_ONE: u32 = 0x3f80_0000;

// Translated from 004e0050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e0050 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Makes the class tag at `011c7830`: allocates 4 bytes and, when that
/// succeeds, constructs them with `00438170` from `"SPD"`; stores the
/// result (or null).
pub fn fn_004e0050(e: &mut Engine) {
    let block = e.call(0x0040_1000, &args![4u32]).u32();
    let tag = if block == 0 {
        0
    } else {
        e.call(0x0043_8170, &args![block, SPD_TAG_TEXT]).u32()
    };
    e.set_global(SPD_TAG, tag);
}

// Translated from 004e00d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the class tag at `011c7830` when set (`004b9900` with flag 1) and
/// clears it.
pub fn fn_004e00d0(e: &mut Engine) {
    let tag: u32 = e.global(SPD_TAG);
    if tag != 0 {
        e.call(0x004b_9900, &args![tag, 1u32]);
    }
    e.set_global(SPD_TAG, 0u32);
}

/// The value of the `Setting` at `setting`, as `FLD dword` from the pointer
/// `00403e20` returns sees its bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(0x0040_3e20, &args![setting]).u32();
    e.mem.u32(value)
}

// Translated from 004e0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update enabled by the byte at `011c7834`: for every node
/// of the object at `011c786c` (`0043b480` count, `0043b4a0` node `i`), its
/// extra data (`NiObjectNET::GetExtraData` with the key `004e0320` gives)
/// holds a timer record (`0048d150`: elapsed, duration, a fade value, a
/// word). The frame's seconds (`0084d030`) are added to the elapsed time;
/// once it reaches the duration the record's word is taken off the running
/// total at `011c7828` and the node is removed from the owner (its vtable
/// +0xF0 with the index); otherwise, past the fraction of the duration in
/// the setting at `011cfc2c`, the fade value becomes `1 - 004b3ab0(0, 1,
/// setting * duration, duration, elapsed)`, and the elapsed time goes to
/// the node (`0043d410` builds a 12-byte value from it, `00a59c60` takes
/// it). Afterwards two settings are passed on (`004de8a0`, `004de8b0`), and
/// the level from `004de8d0`, when below 1, grows by `seconds / 0.4`
/// (limited by `0040ebd0` with 1.0) and is stored back (`004de8c0`).
pub fn fn_004e0110(e: &mut Engine) {
    if e.global::<u8>(UPDATE_ENABLED) == 0 {
        return;
    }
    let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
    let count = e.call(0x0043_b480, &args![owner]).u32();
    let seconds = x87_store32(e.call(0x0084_d030, &args![FRAME_TIMER]).st0);
    let mut index = 0u32;
    while index < count {
        let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
        let node = e.call(0x0043_b4a0, &args![owner, index]).u32();
        if node != 0 {
            let key = e.call(0x004e_0320, &[]).u32();
            let extra = e.call(0x00a5_bdd0, &args![node, key]).u32();
            let timer = e.call(0x0048_d150, &args![extra]).u32();
            let elapsed = x87_store32(x87_add(x87_load32(e.mem.u32(timer)), x87_m32(seconds)));
            e.mem.set_u32(timer, elapsed);
            let elapsed = x87_load32(e.mem.u32(timer));
            let duration = x87_load32(e.mem.u32(timer + 4));
            if duration <= elapsed {
                let word = e.mem.u32(timer + 0xc);
                let total: u32 = e.global(RUNNING_TOTAL);
                e.set_global(RUNNING_TOTAL, total.wrapping_sub(word));
                let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
                e.vcall(owner, 0xf0, &args![index]);
            } else {
                let ratio = x87_div(x87_load32(e.mem.u32(timer)), x87_m32(e.mem.u32(timer + 4)));
                let threshold = x87_load32(setting_bits(e, FADE_SETTING));
                if threshold <= ratio {
                    let elapsed = x87_quiet_bits(e.mem.u32(timer));
                    let duration = x87_quiet_bits(e.mem.u32(timer + 4));
                    let fraction = setting_bits(e, FADE_SETTING);
                    let start =
                        x87_store32(x87_mul(x87_load32(fraction), x87_m32(e.mem.u32(timer + 4))));
                    let curve = e
                        .call(
                            0x004b_3ab0,
                            &args![
                                FLOAT_ZERO,
                                FLOAT_ONE,
                                x87_quiet_bits(start),
                                duration,
                                elapsed
                            ],
                        )
                        .st0;
                    e.mem.set_u32(timer + 8, x87_store32(x87_sub(1.0, curve)));
                }
                let elapsed = x87_quiet_bits(e.mem.u32(timer));
                e.with_stack(0xc, |e, value| {
                    e.call(0x0043_d410, &args![value, elapsed, 1u32, 0u32]);
                    e.call(0x00a5_9c60, &args![node, value]);
                });
            }
        }
        index += 1;
    }
    let value = x87_quiet_bits(setting_bits(e, SETTING_A));
    e.call(0x004d_e8a0, &args![value]);
    let value = x87_quiet_bits(setting_bits(e, SETTING_B));
    e.call(0x004d_e8b0, &args![value]);
    let level = x87_store32(e.call(0x004d_e8d0, &[]).st0);
    if x87_load32(level) < e.global::<f64>(ONE) {
        let level = x87_store32(x87_add(
            x87_div(x87_load32(seconds), e.global::<f64>(RATE_DIVISOR)),
            x87_m32(level),
        ));
        let limited = x87_store32(
            e.call(0x0040_ebd0, &args![FLOAT_ONE, x87_quiet_bits(level)])
                .st0,
        );
        e.call(0x004d_e8c0, &args![x87_quiet_bits(limited)]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_0050, fn_004e0050()),
        entry!(0x004e_00d0, fn_004e00d0()),
        entry!(0x004e_0110, fn_004e0110()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn spd_tag_is_made_and_freed() {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.register(0x0040_1000, |_, _| 0x4000u32.into_ret());
        e.register(0x0043_8170, |_, a| {
            assert_eq!(a[1], SPD_TAG_TEXT);
            (a[0] + 8).into_ret()
        });
        e.register(0x004b_9900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004e_0050, &[]);
        assert_eq!(e.global::<u32>(SPD_TAG), 0x4008);
        e.call(0x004e_00d0, &[]);
        e.call(0x004e_00d0, &[]);
        assert_eq!(calls_to(&e, 0x004b_9900), vec![vec![0x4008, 1]]);
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        e.call(0x004e_0050, &[]);
        assert_eq!(e.global::<u32>(SPD_TAG), 0);
    }

    /// The engine with doubles for the update: two nodes (the second
    /// null), timers in memory, settings at `+4`, and recording doubles.
    fn update_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0102_2000,
            0x011c_7000,
            0x011c_e000,
            0x011c_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(RATE_DIVISOR, f64::from_bits(0x3fd9_9999_9999_999a));
        e.set_global(UPDATE_ENABLED, 1u8);
        e.set_global(NODE_OWNER, 0x0011_0000u32);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0043_b480, |_, _| 2u32.into_ret());
        e.register(0x0043_b4a0, |_, a| {
            if a[1] == 0 { 0x0022_0000u32 } else { 0 }.into_ret()
        });
        e.register(0x0084_d030, |_, _| 0.25f64.into_ret());
        e.register(0x004e_0320, |_, _| 0x77u32.into_ret());
        e.register(0x00a5_bdd0, |_, _| 0x0033_0000u32.into_ret());
        e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
        e.register(0x004b_3ab0, |_, _| 0.25f64.into_ret());
        for target in [
            0x0043_d410,
            0x00a5_9c60,
            0x004d_e8a0,
            0x004d_e8b0,
            0x004d_e8c0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x004d_e8d0, |_, _| 0.5f64.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))).into_ret()
        });
        e
    }

    #[test]
    fn update_fades_a_running_timer() {
        let mut e = update_engine();
        let timer = e.mem.alloc(0x10);
        e.register_double(0x0048_d150, move |_, _| timer.into_ret());
        e.mem.set_f32(timer, 0.5);
        e.mem.set_f32(timer + 4, 1.0);
        e.mem.set_f32(FADE_SETTING + 4, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.mem.f32(timer), 0.75);
        assert_eq!(e.mem.f32(timer + 8), 0.75);
        assert_eq!(
            calls_to(&e, 0x004b_3ab0),
            vec![vec![
                0,
                FLOAT_ONE,
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                0.75f32.to_bits()
            ]]
        );
        let built = calls_to(&e, 0x0043_d410);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0.75f32.to_bits(), 1, 0]);
        assert_eq!(calls_to(&e, 0x00a5_9c60)[0][0], 0x0022_0000);
        // The level grows by 0.25 / 0.4 and is limited to 1.
        assert_eq!(calls_to(&e, 0x004d_e8c0), vec![vec![FLOAT_ONE]]);
        assert_eq!(calls_to(&e, 0x0043_b4a0).len(), 2);
    }

    #[test]
    fn update_removes_an_expired_timer() {
        let mut e = update_engine();
        let timer = e.mem.alloc(0x10);
        e.register_double(0x0048_d150, move |_, _| timer.into_ret());
        e.mem.set_f32(timer, 0.9);
        e.mem.set_f32(timer + 4, 1.0);
        e.mem.set_u32(timer + 0xc, 5);
        e.set_global(RUNNING_TOTAL, 12u32);
        e.put_vtable(0x0044_0000, &[0; 0x3d]);
        e.mem.set_u32(0x0044_0000 + 0xf0, 0x0045_0000);
        e.register(0x0045_0000, |_, _| Ret::default());
        e.mem.map(0x0011_0000, 4);
        e.mem.set_u32(0x0011_0000, 0x0044_0000);
        e.register(0x004d_e8d0, |_, _| 1.0f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.global::<u32>(RUNNING_TOTAL), 7);
        assert_eq!(calls_to(&e, 0x0045_0000), vec![vec![0x0011_0000, 0]]);
        assert!(calls_to(&e, 0x0043_d410).is_empty());
        // A level of 1 is left alone.
        assert!(calls_to(&e, 0x004d_e8c0).is_empty());
        // Disabled: nothing.
        e.set_global(UPDATE_ENABLED, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
