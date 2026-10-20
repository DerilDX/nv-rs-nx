//! `(unplaced)/run_008001b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The object here is the VATS effect manager by the message `00800370`
//! prints (`"====== VATSEffectOnIdle ======"`); its fields are unnamed and
//! read at their offsets: two arrays of 16 effect pointers at +0x12C and
//! +0x16C, 16 bytes at +0x119 and +0x1AC, two arrays of 16 words at +0x1BC
//! and +0x1FC, and a slot list (count +0xC, 8-byte slots from +0x14).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0066b0d0` (`thiscall`, one word): sets the reference-counted pointer at
/// `this` (here always to null).
const SET_POINTER_0066B0D0: u32 = 0x0066_b0d0;
/// `004a4410` (`thiscall`, one word): the effects' scalar deleting
/// destructor, called with 1.
const EFFECT_DELETE: u32 = 0x004a_4410;
/// `0045cec0` (`thiscall`): the destructor of the members at +0x114,
/// +0x2C4 and +0x2CC.
const MEMBER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, `stdcall`:
/// array, element size, count, destructor) and the element destructor
/// (`00800190`, `thiscall`), also run on +4.
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
const ELEMENT_DESTRUCT_00800190: u32 = 0x0080_0190;
/// `00559450` (`thiscall` on +8): the pointer the idle update needs.
const OWNER_00559450: u32 = 0x0055_9450;
/// `00825c00` (`thiscall` on `011f6394`): the current time word.
const TIMER_OBJECT: u32 = 0x011f_6394;
const TIMER_READ: u32 = 0x0082_5c00;
/// The update steps (`thiscall`, one float each, except `008006b0`).
const STEP_00800900: u32 = 0x0080_0900;
const STEP_00800870: u32 = 0x0080_0870;
const STEP_008006B0: u32 = 0x0080_06b0;
const STEP_008007B0: u32 = 0x0080_07b0;
const STEP_00800820: u32 = 0x0080_0820;
const STEP_00800740: u32 = 0x0080_0740;
/// `00800ac0` (`thiscall`, three words).
const STEP_00800AC0: u32 = 0x0080_0ac0;
/// `Error` (`0040fbe0`, engine map; `cdecl`) and
/// `"====== VATSEffectOnIdle ======"` (`01078dd4`).
const ERROR: u32 = 0x0040_fbe0;
const ON_IDLE_MESSAGE: u32 = 0x0107_8dd4;
/// `0043c4b0` (no arguments): the lock object `004a0370` locks and
/// `004a03c0` unlocks (`thiscall` both).
const LOCK_OBJECT: u32 = 0x0043_c4b0;
const LOCK: u32 = 0x004a_0370;
const UNLOCK: u32 = 0x004a_03c0;
/// `00c4edf0` (`thiscall`: an out word, a flag): whether the effect has
/// finished, with a value written to the out word.
const EFFECT_QUERY_00C4EDF0: u32 = 0x00c4_edf0;

// Translation incomplete: 008001b0 - the FS:[0] exception frame (unwinding the member destructors when one throws) is not translated
// Translated from 008001b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor
/// body. Clears +4, nulls the pointer at +8 (`0066b0d0`); for each of the
/// 16 slots clears the words at +0x1BC and +0x1FC and deletes (`004a4410`
/// with 1) and clears the effects at +0x16C and +0x12C; nulls the pointer
/// at +0x2C4, empties the slot list (`00800ed0`), destroys the members at
/// +0x2CC, +0x2C4 and +0x114 (`0045cec0`), the 0x20 elements of 8 bytes at
/// +0x14 (`00800190` through the vector destructor iterator) and the member
/// at +4 (`00800190`).
pub fn fn_008001b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at + 4, 0);
    e.call(SET_POINTER_0066B0D0, &args![at + 8, 0u32]);
    for i in 0..0x10u32 {
        e.mem.set_u32(at + 0x1bc + i * 4, 0);
        e.mem.set_u32(at + 0x1fc + i * 4, 0);
        for array in [0x16cu32, 0x12c] {
            let effect = e.mem.u32(at + array + i * 4);
            if effect != 0 {
                e.call(EFFECT_DELETE, &args![effect, 1u32]);
            }
            e.mem.set_u32(at + array + i * 4, 0);
        }
    }
    e.call(SET_POINTER_0066B0D0, &args![at + 0x2c4, 0u32]);
    fn_00800ed0(e, this);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x2cc]);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x2c4]);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x114]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![at + 0x14, 8u32, 0x20u32, ELEMENT_DESTRUCT_00800190],
    );
    e.call(ELEMENT_DESTRUCT_00800190, &args![at + 4]);
}

// Translated from 00800370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the VATS effect
/// idle update. Without `00559450`'s pointer it clears the bytes at +0x2B4
/// and +0x2B5. Otherwise, with the time (`00825c00`, unsigned, as a float):
/// byte +0x2B4 set runs `00800900`; else byte +0x284 set runs `00800870`,
/// or the time past the float at +0x294 (ordered) runs `008006b0`; then
/// `008007b0`, and with +0 == 1 and +0x10 != -1 `00800820`. Byte +0x298 set
/// runs `00800740`. When the object at +4 answers true from its vtable slot
/// +0x22C (with 0), `00800ac0(0, -1, 0)`. Then it prints the idle message
/// and, for each of the 16 slots with both effects (+0x16C, +0x12C) while
/// `00559450` still answers, under the lock (`0043c4b0`): when both effects
/// report finished (`00c4edf0` with 1, each writing a word) and the slot's
/// byte at +0x1AC is clear, deletes both effects, sets byte +0x1AC, clears
/// byte +0x119, and stores the first word at +0x1BC and the second times
/// the word at +0x2C8 at +0x1FC.
pub fn fn_00800370(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.call(OWNER_00559450, &args![at + 8]).u32() == 0 {
        e.mem.set_u8(at + 0x2b4, 0);
        e.mem.set_u8(at + 0x2b5, 0);
        return;
    }
    let ticks = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    // FILD qword (the word zero-extended), FSTP float.
    let now = x87_store32(f64::from(ticks));
    if e.mem.u8(at + 0x2b4) != 0 {
        e.call(STEP_00800900, &args![at, now]);
    } else {
        if e.mem.u8(at + 0x284) != 0 {
            e.call(STEP_00800870, &args![at, now]);
        } else {
            let start = x87_load32(e.mem.u32(at + 0x294));
            if start < x87_load32(now) {
                e.call(STEP_008006B0, &args![at]);
            }
        }
        e.call(STEP_008007B0, &args![at, now]);
        if e.mem.u32(at) == 1 && e.mem.u32(at + 0x10) != 0xffff_ffff {
            e.call(STEP_00800820, &args![at, now]);
        }
    }
    if e.mem.u8(at + 0x298) != 0 {
        e.call(STEP_00800740, &args![at, now]);
    }
    let object = e.mem.u32(at + 4);
    if object != 0 && e.vcall(object, 0x22c, &args![0u32]).bool() {
        e.call(STEP_00800AC0, &args![at, 0u32, 0xffff_ffffu32, 0u32]);
    }
    e.call(ERROR, &args![ON_IDLE_MESSAGE]);
    for i in 0..0x10u32 {
        if e.call(OWNER_00559450, &args![at + 8]).u32() == 0
            || e.mem.u32(at + 0x16c + i * 4) == 0
            || e.mem.u32(at + 0x12c + i * 4) == 0
        {
            continue;
        }
        let lock = e.call(LOCK_OBJECT, &[]).u32();
        e.call(LOCK, &args![lock]);
        e.with_stack(8, |e, words| {
            let first = words.addr();
            let second = words.addr() + 4;
            let effect = e.mem.u32(at + 0x16c + i * 4);
            if !e
                .call(EFFECT_QUERY_00C4EDF0, &args![effect, first, 1u32])
                .bool()
            {
                return;
            }
            let effect = e.mem.u32(at + 0x12c + i * 4);
            if !e
                .call(EFFECT_QUERY_00C4EDF0, &args![effect, second, 1u32])
                .bool()
            {
                return;
            }
            if e.mem.u8(at + 0x1ac + i) != 0 {
                return;
            }
            for array in [0x16cu32, 0x12c] {
                let effect = e.mem.u32(at + array + i * 4);
                if effect != 0 {
                    e.call(EFFECT_DELETE, &args![effect, 1u32]);
                }
                e.mem.set_u32(at + array + i * 4, 0);
            }
            e.mem.set_u8(at + 0x1ac + i, 1);
            e.mem.set_u8(at + 0x119 + i, 0);
            let value = e.mem.u32(first);
            e.mem.set_u32(at + 0x1bc + i * 4, value);
            let scaled = e.mem.u32(second).wrapping_mul(e.mem.u32(at + 0x2c8));
            e.mem.set_u32(at + 0x1fc + i * 4, scaled);
        });
        let lock = e.call(LOCK_OBJECT, &[]).u32();
        e.call(UNLOCK, &args![lock]);
    }
}

// Translated from 00800ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): empties the slot
/// list: for each of the first `count` (+0xC, signed) 8-byte slots from
/// +0x14, clears its first word and nulls its pointer (`0066b0d0` on the
/// second word); then the count becomes 0 and the word at +0x10 -1.
pub fn fn_00800ed0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut i = 0i32;
    while i < e.mem.u32(at + 0xc) as i32 {
        let slot = at.wrapping_add((i as u32).wrapping_mul(8));
        e.mem.set_u32(slot + 0x14, 0);
        e.call(SET_POINTER_0066B0D0, &args![slot + 0x18, 0u32]);
        i += 1;
    }
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 0x10, 0xffff_ffff);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008001b0, fn_008001b0(Ptr)),
        entry!(0x00800370, fn_00800370(Ptr)),
        entry!(0x00800ed0, fn_00800ed0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            SET_POINTER_0066B0D0,
            EFFECT_DELETE,
            MEMBER_DESTRUCT_0045CEC0,
            VECTOR_DESTRUCTOR_ITERATOR,
            ELEMENT_DESTRUCT_00800190,
            STEP_00800900,
            STEP_00800870,
            STEP_008006B0,
            STEP_008007B0,
            STEP_00800820,
            STEP_00800740,
            STEP_00800AC0,
            ERROR,
            LOCK,
            UNLOCK,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(LOCK_OBJECT, |_, _| 0x7000u32.into_ret());
        e.register(TIMER_READ, |_, _| 100u32.into_ret());
        e
    }

    #[test]
    fn slot_list_is_emptied() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x14, 5);
        e.mem.set_u32(this + 0x1c, 6);
        e.call_log = Some(vec![]);
        e.call(0x0080_0ed0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(
            calls_to(&e, SET_POINTER_0066B0D0),
            vec![vec![this + 0x18, 0], vec![this + 0x20, 0]]
        );
    }

    #[test]
    fn destructor_deletes_the_effects() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this + 4, 9);
        e.mem.set_u32(this + 0x16c + 3 * 4, 0x5000);
        e.mem.set_u32(this + 0x12c + 15 * 4, 0x6000);
        e.mem.set_u32(this + 0x1bc, 7);
        e.call_log = Some(vec![]);
        e.call(0x0080_01b0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x1bc), 0);
        assert_eq!(e.mem.u32(this + 0x16c + 3 * 4), 0);
        assert_eq!(
            calls_to(&e, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(
            calls_to(&e, MEMBER_DESTRUCT_0045CEC0),
            vec![vec![this + 0x2cc], vec![this + 0x2c4], vec![this + 0x114]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![this + 0x14, 8, 0x20, ELEMENT_DESTRUCT_00800190]]
        );
        assert_eq!(
            calls_to(&e, ELEMENT_DESTRUCT_00800190),
            vec![vec![this + 4]]
        );
    }

    #[test]
    fn idle_without_owner_clears_the_flags() {
        let mut e = engine();
        e.register(OWNER_00559450, |_, _| Ret::default());
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x2b4, 1);
        e.mem.set_u8(this + 0x2b5, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        assert_eq!(e.mem.u8(this + 0x2b4), 0);
        assert_eq!(e.mem.u8(this + 0x2b5), 0);
        assert!(calls_to(&e, ERROR).is_empty());
    }

    #[test]
    fn idle_runs_the_steps_and_retires_finished_effects() {
        let mut e = engine();
        e.register(OWNER_00559450, |_, _| 1u32.into_ret());
        // Effects finish; the out word is the effect's address / 0x1000.
        e.register(EFFECT_QUERY_00C4EDF0, |e, a| {
            e.mem.set_u32(a[1], a[0] / 0x1000);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_f32(this + 0x294, 50.0);
        e.mem.set_u8(this + 0x298, 1);
        e.mem.set_u32(this + 0x2c8, 3);
        e.mem.set_u32(this + 0x16c + 2 * 4, 0x5000);
        e.mem.set_u32(this + 0x12c + 2 * 4, 0x6000);
        e.mem.set_u8(this + 0x119 + 2, 1);
        // A slot already retired keeps its effects.
        e.mem.set_u32(this + 0x16c + 4 * 4, 0x7000);
        e.mem.set_u32(this + 0x12c + 4 * 4, 0x8000);
        e.mem.set_u8(this + 0x1ac + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        let now = 100.0f32.to_bits();
        assert_eq!(calls_to(&e, STEP_008006B0), vec![vec![this]]);
        assert_eq!(calls_to(&e, STEP_008007B0), vec![vec![this, now]]);
        assert_eq!(calls_to(&e, STEP_00800820), vec![vec![this, now]]);
        assert_eq!(calls_to(&e, STEP_00800740), vec![vec![this, now]]);
        assert!(calls_to(&e, STEP_00800900).is_empty());
        assert_eq!(calls_to(&e, ERROR), vec![vec![ON_IDLE_MESSAGE]]);
        assert_eq!(
            calls_to(&e, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x1ac + 2), 1);
        assert_eq!(e.mem.u8(this + 0x119 + 2), 0);
        assert_eq!(e.mem.u32(this + 0x1bc + 2 * 4), 5);
        assert_eq!(e.mem.u32(this + 0x1fc + 2 * 4), 18);
        assert_eq!(e.mem.u32(this + 0x16c + 4 * 4), 0x7000);
        assert_eq!(calls_to(&e, LOCK).len(), 2);
        assert_eq!(calls_to(&e, UNLOCK).len(), 2);
        // With +0x2B4 set only the other step runs.
        e.mem.set_u8(this + 0x2b4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        assert_eq!(calls_to(&e, STEP_00800900), vec![vec![this, now]]);
        assert!(calls_to(&e, STEP_008007B0).is_empty());
    }
}
