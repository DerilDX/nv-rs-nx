//! `(unplaced)/run_00a257c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions of the PC input object (built by `00a22660`): two that work on
//! the 0x40-byte object it keeps at +0x1B80 (whose constructor and set-up
//! method are here too), the 0x14280-byte singleton at `011f3634` and its
//! constructor. None has a name in the engine map; they are described by
//! what the code does.

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte global that must be non-zero for `00a257c0` to act.
const ENABLED_FLAG: u32 = 0x011f_35c8;
/// The double (1000.0 in the exe) `00a257c0` scales its argument by.
const SCALE: u32 = 0x0101_7b70;
/// `_ftol2` (CRT; the value in x87 `ST0`, a leading `f64` argument).
const FTOL2: u32 = 0x00ec_62c0;
/// Method `00a269f0` of the object at +0x1B80 (thiscall, one word).
const OBJECT_1B80_START: u32 = 0x00a2_69f0;
/// The `XInputSetState` thunk (`JMP [00fdf398]`; stdcall: user index,
/// state).
const XINPUT_SET_STATE: u32 = 0x00ee_820a;
/// `GetTickCount` (import slot).
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
/// The singleton `00a25920` builds on first use.
const SINGLETON: u32 = 0x011f_3634;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00a257c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the byte at `011f35c8` and the input object's byte +0x1B88 are both
/// set, passes `value * 1000.0` (the double at `01017b70`, `FMUL`, then
/// `_ftol2`) to `00a269f0` of the object at +0x1B80.
pub fn fn_00a257c0(e: &mut Engine, this: u32, value: f32) {
    if e.mem.u8(ENABLED_FLAG) == 0 {
        return;
    }
    if e.mem.u8(this + 0x1b88) == 0 {
        return;
    }
    let scaled = x87_mul(x87_load32(value.to_bits()), e.mem.f64(SCALE));
    let whole = e.call(FTOL2, &args![scaled]).u32();
    let object = e.mem.u32(this + 0x1b80);
    e.call(OBJECT_1B80_START, &[object, whole]);
}

// Translated from 00a25810 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the input object's byte +0x1B88 is set: clears the words +0x0C and
/// +0x20 of the object at +0x1B80 and sends user 0 a state of two zero
/// 16-bit values (`XInputSetState(0, &state)`, the state on the stack).
pub fn fn_00a25810(e: &mut Engine, this: u32) {
    if e.mem.u8(this + 0x1b88) == 0 {
        return;
    }
    e.with_stack(4, |e, state| {
        e.mem.set_u16(state.addr(), 0);
        e.mem.set_u16(state.addr() + 2, 0);
        let object = e.mem.u32(this + 0x1b80);
        e.mem.set_u32(object + 0xc, 0);
        e.mem.set_u32(object + 0x20, 0);
        e.call(XINPUT_SET_STATE, &[0, state.addr()]);
    });
}

// Translated from 00a25920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The singleton at `011f3634`: while it is null, allocates 0x14280 bytes
/// and constructs them with [`fn_00a259b0`] (null when the allocation
/// fails). Returns the singleton.
// Translation incomplete: 00a25920 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00a25920(e: &mut Engine) -> u32 {
    if e.mem.u32(SINGLETON) == 0 {
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x14280]).u32();
        let object = if block != 0 { fn_00a259b0(e, block) } else { 0 };
        e.mem.set_u32(SINGLETON, object);
    }
    e.mem.u32(SINGLETON)
}

// Translated from 00a259b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x14280-byte singleton: clears the two words at
/// +0x3AC, the bytes +0x00..+0x03, the words +0x68, +0x50, +0x6C, +0x70,
/// +0x64, the bytes +0x04 and +0x24, the 0x1C bytes at +0x08, the 0x13EC0
/// bytes at +0x3B8 and the 0x320 bytes at +0x8C, and the words +0x14278
/// and +0x3B4; sets +0x74 and +0x78 to 1, +0x7C and +0x80 to -1 and the
/// bytes +0x84..+0x88 to 1, 1, 0, 1, 0. Returns `this`.
pub fn fn_00a259b0(e: &mut Engine, this: u32) -> u32 {
    let pair = this + 0x3ac;
    e.mem.set_u32(pair, 0);
    e.mem.set_u32(pair + 4, 0);
    for index in 0..4 {
        e.mem.set_u8(this + index, 0);
    }
    e.mem.set_u32(this + 0x68, 0);
    e.mem.set_u8(this + 4, 0);
    e.call(MEMSET, &[this + 8, 0, 0x1c]);
    e.mem.set_u8(this + 0x24, 0);
    e.mem.set_u32(this + 0x50, 0);
    e.mem.set_u32(this + 0x6c, 0);
    e.mem.set_u32(this + 0x70, 0);
    e.mem.set_u32(this + 0x64, 0);
    e.mem.set_u32(this + 0x74, 1);
    e.mem.set_u32(this + 0x78, 1);
    e.mem.set_u32(this + 0x80, u32::MAX);
    e.mem.set_u32(this + 0x7c, u32::MAX);
    e.mem.set_u8(this + 0x84, 1);
    e.mem.set_u8(this + 0x85, 1);
    e.mem.set_u8(this + 0x86, 0);
    e.mem.set_u8(this + 0x87, 1);
    e.mem.set_u8(this + 0x88, 0);
    e.call(MEMSET, &[this + 0x3b8, 0, 0x13ec0]);
    e.call(MEMSET, &[this + 0x8c, 0, 0x320]);
    e.mem.set_u32(this + 0x14278, 0);
    e.mem.set_u32(this + 0x3b4, 0);
    this
}

// Translated from 00a262d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x40-byte object the input object keeps at +0x1B80:
/// stores `owner` in its first word and `GetTickCount()` at +0x38. Returns
/// `this`.
pub fn fn_00a262d0(e: &mut Engine, this: u32, owner: u32) -> u32 {
    e.mem.set_u32(this, owner);
    let ticks = e.call(IMPORT_GET_TICK_COUNT, &[]).u32();
    e.mem.set_u32(this + 0x38, ticks);
    this
}

// Translated from 00a26310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches `block` to one of the object's two channels (`which` 0: words
/// +0x04 and +0x0C..+0x1C; otherwise +0x08 and +0x20..+0x30): clears the
/// channel's last two words, sets the block's +0x0C to its +0x08 when it is
/// 0, keeps the block and copies its +0x08, +0x14 and +0x1C, and copies the
/// block's first 16-bit value to its +0x1A and +0x20.
pub fn fn_00a26310(e: &mut Engine, this: u32, which: u32, block: u32) {
    let (cleared, kept, copies) = if which == 0 {
        ([0x18, 0x1c], 4, [0xc, 0x10, 0x14])
    } else {
        ([0x2c, 0x30], 8, [0x20, 0x24, 0x28])
    };
    for off in cleared {
        e.mem.set_u32(this + off, 0);
    }
    if e.mem.u32(block + 0xc) == 0 {
        let start = e.mem.u32(block + 8);
        e.mem.set_u32(block + 0xc, start);
    }
    e.mem.set_u32(this + kept, block);
    for (to, from) in copies.into_iter().zip([8, 0x14, 0x1c]) {
        let word = e.mem.u32(block + from);
        e.mem.set_u32(this + to, word);
    }
    let first = e.mem.u16(block);
    e.mem.set_u16(block + 0x1a, first);
    let first = e.mem.u16(block);
    e.mem.set_u16(block + 0x20, first);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_57c0, fn_00a257c0(u32, f32)),
        entry!(0x00a2_5810, fn_00a25810(u32)),
        entry!(0x00a2_5920, fn_00a25920() -> u32),
        entry!(0x00a2_59b0, fn_00a259b0(u32) -> u32),
        entry!(0x00a2_62d0, fn_00a262d0(u32, u32) -> u32),
        entry!(0x00a2_6310, fn_00a26310(u32, u32, u32)),
        entry!(0x00a2_69f0, fn_00a269f0(u32, i32)),
    ]
}

/// Method `00a26520` of the object at +0x1B80 (thiscall: channel, state,
/// value).
const OBJECT_1B80_CHANNEL: u32 = 0x00a2_6520;

// Translated from 00a269f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the 0x40-byte object (the one `00a257c0` calls): for a
/// `value` of at least 1, fills a stack state of two zero 16-bit values
/// through `00a26520(0, &state, value)` and `00a26520(1, &state, value)`
/// and sends it to the user index in the object's first word
/// (`XInputSetState`).
pub fn fn_00a269f0(e: &mut Engine, this: u32, value: i32) {
    if value < 1 {
        return;
    }
    e.with_stack(4, |e, state| {
        let state = state.addr();
        e.mem.set_u16(state, 0);
        e.mem.set_u16(state + 2, 0);
        e.call(OBJECT_1B80_CHANNEL, &[this, 0, state, value as u32]);
        e.call(OBJECT_1B80_CHANNEL, &[this, 1, state, value as u32]);
        let user = e.mem.u32(this);
        e.call(XINPUT_SET_STATE, &[user, state]);
    });
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn positive_values_fill_both_channels_and_send_the_state() {
        let mut e = Engine::new();
        // The channel double writes its value into its half of the state.
        e.register(OBJECT_1B80_CHANNEL, |e, a| {
            e.mem.set_u16(a[2] + a[1] * 2, (a[3] + a[1]) as u16);
            0u32.into_ret()
        });
        let sent = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let record = sent.clone();
        e.register_double(XINPUT_SET_STATE, move |e, a| {
            record.set(e.mem.u32(a[1]));
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a2_69f0, &[this, 500]);
        let log = e.call_log.take().unwrap();
        let channels: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == OBJECT_1B80_CHANNEL)
            .map(|(_, w)| (w[0], w[1], w[3]))
            .collect();
        assert_eq!(channels, vec![(this, 0, 500), (this, 1, 500)]);
        let send = log.iter().find(|(a, _)| *a == XINPUT_SET_STATE).unwrap();
        assert_eq!(send.1[0], 2);
        assert_eq!(sent.get(), (501 << 16) | 500);
        // Zero and negative values do nothing.
        for value in [0i32, -3] {
            e.call_log = Some(vec![]);
            e.call(0x00a2_69f0, &[this, value as u32]);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn scaled_call_needs_both_flags() {
        for (flag, enabled, called) in [(1u8, 1u8, true), (0, 1, false), (1, 0, false)] {
            let mut e = Engine::new();
            e.map(0x011f_3000, 0x1000);
            e.map(0x0101_7000, 0x1000);
            e.mem.set_u8(ENABLED_FLAG, flag);
            e.mem.set_f64(SCALE, 1000.0);
            e.register(FTOL2, |_, a| {
                let value = f64::take(a, &mut 0);
                (value as i32 as u32).into_ret()
            });
            e.register(OBJECT_1B80_START, |_, _| 0u32.into_ret());
            let this = e.mem.alloc(0x1c04);
            e.mem.set_u8(this + 0x1b88, enabled);
            e.mem.set_u32(this + 0x1b80, 0x0055_0000);
            e.call_log = Some(vec![]);
            e.call(0x00a2_57c0, &[this, 0.25f32.to_bits()]);
            let starts = calls_to(&e, OBJECT_1B80_START);
            if called {
                assert_eq!(starts, vec![vec![0x0055_0000, 250]]);
                assert_eq!(calls_to(&e, FTOL2)[0], args![250.0f64]);
            } else {
                assert!(starts.is_empty());
                assert!(calls_to(&e, FTOL2).is_empty());
            }
        }
    }

    #[test]
    fn stop_clears_the_object_and_sends_a_zero_state() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::Cell::new(u32::MAX));
        let record = seen.clone();
        e.register_double(XINPUT_SET_STATE, move |e, a| {
            record.set(e.mem.u32(a[1]));
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x1c04);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xc, 5);
        e.mem.set_u32(object + 0x20, 6);
        e.mem.set_u32(this + 0x1b80, object);
        e.mem.set_u8(this + 0x1b88, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5810, &[this]);
        assert_eq!(calls_to(&e, XINPUT_SET_STATE).len(), 1);
        assert_eq!(calls_to(&e, XINPUT_SET_STATE)[0][0], 0);
        assert_eq!(seen.get(), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        // Flag clear: nothing happens.
        e.mem.set_u8(this + 0x1b88, 0);
        e.mem.set_u32(object + 0xc, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a2_5810, &[this]);
        assert!(calls_to(&e, XINPUT_SET_STATE).is_empty());
        assert_eq!(e.mem.u32(object + 0xc), 5);
    }

    fn memset_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn singleton_is_built_once() {
        let mut e = memset_engine();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        let first = e.call(0x00a2_5920, &[]).u32();
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(first + 0x74), 1);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), first);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14280]]
        );
        // A failed allocation leaves it null, and the next call tries again.
        let mut e = memset_engine();
        e.map(0x011f_3000, 0x1000);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), 0);
        assert_eq!(e.call(0x00a2_5920, &[]).u32(), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE).len(), 2);
    }

    #[test]
    fn singleton_constructor_sets_its_defaults() {
        let mut e = memset_engine();
        let this = e.mem.alloc(0x14280);
        e.mem.write(this, &vec![0xaa; 0x14280]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_59b0, &[this]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![
                vec![this + 8, 0, 0x1c],
                vec![this + 0x3b8, 0, 0x13ec0],
                vec![this + 0x8c, 0, 0x320],
            ]
        );
        for off in [
            0x3ac, 0x3b0, 0x0, 0x68, 0x50, 0x6c, 0x70, 0x64, 0x14278, 0x3b4,
        ] {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x74), 1);
        assert_eq!(e.mem.u32(this + 0x78), 1);
        assert_eq!(e.mem.u32(this + 0x7c), u32::MAX);
        assert_eq!(e.mem.u32(this + 0x80), u32::MAX);
        assert_eq!(e.mem.bytes(this + 0x84, 5), vec![1, 1, 0, 1, 0]);
        // Bytes the constructor does not touch keep their value.
        assert_eq!(e.mem.u8(this + 0x25), 0xaa);
        assert_eq!(e.mem.u8(this + 0x89), 0xaa);
    }

    #[test]
    fn object_constructor_keeps_the_owner_and_the_tick_count() {
        let mut e = Engine::new();
        e.register(IMPORT_GET_TICK_COUNT, |_, _| 12345u32.into_ret());
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00a2_62d0, &[this, 0x77]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x77);
        assert_eq!(e.mem.u32(this + 0x38), 12345);
    }

    #[test]
    fn attach_fills_the_chosen_channel_and_fixes_the_block() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for off in (0..0x40).step_by(4) {
            e.mem.set_u32(this + off, 0x1111_1111);
        }
        let block = e.mem.alloc(0x24);
        e.mem.set_u16(block, 0x0203);
        e.mem.set_u32(block + 8, 0x80);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x14, 0x140);
        e.mem.set_u32(block + 0x1c, 0x1c0);
        e.call(0x00a2_6310, &[this, 0, block]);
        assert_eq!(e.mem.u32(block + 0xc), 0x80);
        assert_eq!(e.mem.u16(block + 0x1a), 0x0203);
        assert_eq!(e.mem.u16(block + 0x20), 0x0203);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(e.mem.u32(this + 0xc), 0x80);
        assert_eq!(e.mem.u32(this + 0x10), 0x140);
        assert_eq!(e.mem.u32(this + 0x14), 0x1c0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // The other channel is untouched.
        assert_eq!(e.mem.u32(this + 8), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x20), 0x1111_1111);

        // Channel 1, with the block's +0x0C already set.
        e.mem.set_u32(block + 0xc, 0x99);
        e.mem.set_u16(block, 0x0405);
        e.call(0x00a2_6310, &[this, 1, block]);
        assert_eq!(e.mem.u32(block + 0xc), 0x99);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u32(this + 0x20), 0x80);
        assert_eq!(e.mem.u32(this + 0x24), 0x140);
        assert_eq!(e.mem.u32(this + 0x28), 0x1c0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u16(block + 0x1a), 0x0405);
        assert_eq!(e.mem.u16(block + 0x20), 0x0405);
    }
}
