//! `NiXenonRenderer/run_00e88fc0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00e895f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem; no arguments):
/// allocates 0x18 bytes from the memory manager, zeroes the first four words, sets the
/// words at `+0x10` and `+0x14` to 8 and keeps the block in `0126f9a0` (null when the
/// allocation fails).
pub fn fn_00e895f0(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f9a0, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f9a0, block);
}

/// The pool this item goes back to (`0126f9a0`) and `NiTPrimitiveSet<..>::AddUnique`
/// (`00e7ec00`, `thiscall` on the set, the address of the element; the engine map names it
/// `NiControllerSequence::SetState`).
const ITEM_POOL_0126F9A0: u32 = 0x0126_f9a0;
const POOL_ADD_UNIQUE_00E7EC00: u32 = 0x00e7_ec00;
/// The words and bytes `00e89670` clears, in its order.
const CLEARED_WORDS_00E89670: [u32; 38] = [
    0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50,
    0x54, 0x58, 0x5c, 0x60, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x90, 0x94, 0x98, 0x9c, 0xa0, 0xa4,
    0xb0, 0x34, 0x4, 0x88, 0x64, 0xac,
];
const CLEARED_BYTES_00E89670: [u32; 3] = [0x84, 0xa8, 0xb4];

// Translated from 00e89670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`): an item's release to its pool, `cdecl` with one
/// word (the item; no null check). When the item's first byte is set (a pool item): clears the
/// words +0xc to +0x30, +0x3c to +0x60, +0x6c to +0x80, +0x90 to +0xa4, +0xb0, +0x34, +4, +0x88,
/// +0x64 and +0xac and the bytes +0x84, +0xa8, +0xb4, then gives the item back to the pool at
/// `0126f9a0` (`00e7ec00`, with the item in the argument's own stack word). Otherwise nothing.
pub fn fn_00e89670(e: &mut Engine, item: Ptr) {
    let item = item.addr();
    if e.mem.u8(item) == 0 {
        return;
    }
    for offset in CLEARED_WORDS_00E89670 {
        e.mem.set_u32(item + offset, 0);
    }
    for offset in CLEARED_BYTES_00E89670 {
        e.mem.set_u8(item + offset, 0);
    }
    let pool = e.global::<u32>(ITEM_POOL_0126F9A0);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

// Translated from 00e891f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 8 steps clears the byte at `+0x2c + i`
/// and the byte at `+0x5c + i`, then the words at `+0x34` and `+4`.
pub fn fn_00e891f0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..8u32 {
        e.mem.set_u8(this + 0x2c + i, 0);
        e.mem.set_u8(this + 0x5c + i, 0);
    }
    e.mem.set_u32(this + 0x34, 0);
    e.mem.set_u32(this + 4, 0);
}

// Translated from 00e89220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem), `thiscall` with no
/// stack words (called from `00e7ecd0`): for each of 5 steps clears the bytes at `+0x80 + i`,
/// `+0xa4 + i` and `+0xb0 + i`, then the words at `+0x88`, `+0x64` and `+0xac`.
pub fn fn_00e89220(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    for i in 0..5u32 {
        e.mem.set_u8(this + 0x80 + i, 0);
        e.mem.set_u8(this + 0xa4 + i, 0);
        e.mem.set_u8(this + 0xb0 + i, 0);
    }
    e.mem.set_u32(this + 0x88, 0);
    e.mem.set_u32(this + 0x64, 0);
    e.mem.set_u32(this + 0xac, 0);
}

/// The render-state slot maps the two setters index (16-bit entries): `0126f958` for
/// `00e88fc0` (slots below 8 are kept), `0126f92c` for `00e890c0` (slots below 5).
const STATE_SLOTS_0126F958: u32 = 0x0126_f958;
const SAMPLER_SLOTS_0126F92C: u32 = 0x0126_f92c;

// Translated from 00e88fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with three words: a state, its
/// value and a byte flag. The state's slot is the 16-bit entry at `0126f958 + 2 * state` (no
/// bounds check); slots 8 and up do nothing. With the flag set: a slot already marked in the
/// bytes at +0x2c only gets the value (words at +0xc); otherwise a slot marked in the bytes at
/// +0x5c moves over (+0x5c byte cleared, +0x2c byte set, value stored, count +0x34 down and
/// count +4 up), and then (in either case) the +0x2c byte is set, the value stored and +4
/// counted up once more. With the flag clear the same with the roles swapped: a slot marked at
/// +0x5c only gets the value (words at +0x3c); otherwise one marked at +0x2c moves over (+0x2c
/// cleared, +0x5c set, value stored, +0x34 up, +4 down), then the +0x5c byte is set, the value
/// stored and +0x34 counted up.
pub fn fn_00e88fc0(e: &mut Engine, this: Ptr, state: u32, value: u32, flag: u8) {
    let base = this.addr();
    let slot = u32::from(
        e.mem
            .u16(STATE_SLOTS_0126F958.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 8 {
        return;
    }
    let add = |e: &mut Engine, offset: u32, delta: u32| {
        let at = base.wrapping_add(offset);
        let counted = e.mem.u32(at).wrapping_add(delta);
        e.mem.set_u32(at, counted);
    };
    let set_byte = |e: &mut Engine, offset: u32, value: u8| {
        e.mem.set_u8(base.wrapping_add(slot + offset), value)
    };
    let byte = |e: &mut Engine, offset: u32| e.mem.u8(base.wrapping_add(slot + offset));
    if flag != 0 {
        let word = base.wrapping_add(0xc + slot * 4);
        if byte(e, 0x2c) != 0 {
            e.mem.set_u32(word, value);
            return;
        }
        if byte(e, 0x5c) != 0 {
            set_byte(e, 0x5c, 0);
            set_byte(e, 0x2c, 1);
            e.mem.set_u32(word, value);
            add(e, 0x34, 0xffff_ffff);
            add(e, 4, 1);
        }
        set_byte(e, 0x2c, 1);
        e.mem.set_u32(word, value);
        add(e, 4, 1);
        return;
    }
    let word = base.wrapping_add(0x3c + slot * 4);
    if byte(e, 0x5c) != 0 {
        e.mem.set_u32(word, value);
        return;
    }
    if byte(e, 0x2c) != 0 {
        set_byte(e, 0x2c, 0);
        set_byte(e, 0x5c, 1);
        e.mem.set_u32(word, value);
        add(e, 0x34, 1);
        add(e, 4, 0xffff_ffff);
    }
    set_byte(e, 0x5c, 1);
    e.mem.set_u32(word, value);
    add(e, 0x34, 1);
}

// Translated from 00e890c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e88fc0`), `thiscall` with four words: a state, its
/// value and two byte flags. The slot is the 16-bit entry at `0126f92c + 2 * state` (no
/// bounds check); slots 5 and up do nothing. With the first flag set: a slot marked in the
/// bytes at +0x80 gets the value (words at +0x6c) and goes on to the second flag's update
/// below; otherwise a slot marked at +0xa4 is unmarked (count +0x88 down, and when its +0xb0
/// byte is set count +0xac down, then that byte cleared), then the +0x80 byte is set, the
/// value stored, +0x64 counted up, +0xac counted up when the second flag is set, and the
/// +0xb0 byte set to the second flag. With the first flag clear the same with +0xa4 (words at
/// +0x90, count +0x88) and +0x80 (count +0x64) swapped. The second flag's update for an
/// already marked slot: a set flag counts +0xac up when the +0xb0 byte was clear, a clear flag
/// counts it down when that byte was set, and the byte becomes the flag.
pub fn fn_00e890c0(e: &mut Engine, this: Ptr, state: u32, value: u32, enable: u8, extra: u8) {
    let base = this.addr();
    let slot = u32::from(
        e.mem
            .u16(SAMPLER_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 5 {
        return;
    }
    let add = |e: &mut Engine, offset: u32, delta: u32| {
        let at = base.wrapping_add(offset);
        let counted = e.mem.u32(at).wrapping_add(delta);
        e.mem.set_u32(at, counted);
    };
    let at = |offset: u32| base.wrapping_add(slot + offset);
    // (marked byte, value word, count) of the side the first flag picks, then the other side.
    let (mine, word, count, other, other_count) = if enable != 0 {
        (0x80u32, 0x6cu32, 0x64u32, 0xa4u32, 0x88u32)
    } else {
        (0xa4, 0x90, 0x88, 0x80, 0x64)
    };
    let word = base.wrapping_add(word + slot * 4);
    if e.mem.u8(at(mine)) != 0 {
        e.mem.set_u32(word, value);
        let extra_byte = at(0xb0);
        if extra != 0 {
            if e.mem.u8(extra_byte) == 0 {
                add(e, 0xac, 1);
            }
        } else if e.mem.u8(extra_byte) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(extra_byte, extra);
        return;
    }
    if e.mem.u8(at(other)) != 0 {
        e.mem.set_u8(at(other), 0);
        add(e, other_count, 0xffff_ffff);
        if e.mem.u8(at(0xb0)) != 0 {
            add(e, 0xac, 0xffff_ffff);
        }
        e.mem.set_u8(at(0xb0), 0);
    }
    e.mem.set_u8(at(mine), 1);
    e.mem.set_u32(word, value);
    add(e, count, 1);
    if extra != 0 {
        add(e, 0xac, 1);
    }
    e.mem.set_u8(at(0xb0), extra);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_95f0, fn_00e895f0()),
        entry!(0x00e8_9670, fn_00e89670(Ptr)),
        entry!(0x00e8_91f0, fn_00e891f0(Ptr)),
        entry!(0x00e8_9220, fn_00e89220(Ptr)),
        entry!(0x00e8_8fc0, fn_00e88fc0(Ptr, u32, u32, u8)),
        entry!(0x00e8_90c0, fn_00e890c0(Ptr, u32, u32, u8, u8)),
    ]
}

#[cfg(test)]
mod clear_tests {
    use super::*;

    /// A 0xc0-byte block of 0xaa; the call clears exactly the bytes in `cleared`.
    fn check(addr: u32, cleared: &[std::ops::Range<u32>]) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xaa; 0xc0]);
        e.call(addr, &args![this]);
        for offset in 0..0xc0u32 {
            let want = if cleared.iter().any(|r| r.contains(&offset)) {
                0
            } else {
                0xaa
            };
            assert_eq!(e.mem.u8(this + offset), want, "offset {offset:#x}");
        }
    }

    #[test]
    fn fn_00e891f0_clears_its_bytes_and_words() {
        check(0x00e8_91f0, &[0x2c..0x38, 0x5c..0x64, 4..8]);
    }

    #[test]
    fn fn_00e89220_clears_its_bytes_and_words() {
        check(
            0x00e8_9220,
            &[
                0x80..0x85,
                0xa4..0xa9,
                0xb0..0xb5,
                0x88..0x8c,
                0x64..0x68,
                0xac..0xb0,
            ],
        );
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_or_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e8_95f0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f9a0), 0);
    }
}

#[cfg(test)]
mod item_release_tests {
    use super::*;

    #[test]
    fn fn_00e89670_clears_a_pool_item_and_gives_it_back() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.set_global(ITEM_POOL_0126F9A0, 0x0e00_0000u32);
        // The double notes the element its slot holds.
        let seen = e.mem.alloc(4);
        e.register_double(POOL_ADD_UNIQUE_00E7EC00, move |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(seen, item);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let item = e.mem.alloc(0xb8);
        e.mem.write(item, &[0xcc; 0xb8]);
        e.call(0x00e8_9670, &args![item]);
        let mut expected = vec![0xccu8; 0xb8];
        for offset in CLEARED_WORDS_00E89670.iter() {
            expected[*offset as usize..*offset as usize + 4].copy_from_slice(&[0; 4]);
        }
        for offset in CLEARED_BYTES_00E89670 {
            expected[offset as usize] = 0;
        }
        assert_eq!(e.mem.bytes(item, 0xb8), expected);
        assert_eq!(e.mem.u32(seen), item);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, POOL_ADD_UNIQUE_00E7EC00);
        assert_eq!(log[1].1[0], 0x0e00_0000);
        // An item whose first byte is clear is left alone.
        let other = e.mem.alloc(0xb8);
        e.mem.write(other + 4, &[0xcc; 4]);
        e.call(0x00e8_9670, &args![other]);
        assert_eq!(e.mem.u32(other + 4), 0xcccc_cccc);
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// The slot maps mapped: state 5 is slot 2 in both, state 6 slot 8 / 5 (ignored).
    fn setup(e: &mut Engine) -> u32 {
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u16(STATE_SLOTS_0126F958 + 10, 2);
        e.mem.set_u16(STATE_SLOTS_0126F958 + 12, 8);
        e.mem.set_u16(SAMPLER_SLOTS_0126F92C + 10, 2);
        e.mem.set_u16(SAMPLER_SLOTS_0126F92C + 12, 5);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 4, 10);
        e.mem.set_u32(this + 0x34, 20);
        e.mem.set_u32(this + 0x64, 30);
        e.mem.set_u32(this + 0x88, 40);
        e.mem.set_u32(this + 0xac, 50);
        this
    }

    fn set_state(e: &mut Engine, this: u32, state: u32, value: u32, flag: u32) {
        e.call(0x00e8_8fc0, &args![this, state, value, flag]);
    }

    /// (+4, +0x34, byte +0x2e, byte +0x5e, word +0x14, word +0x44).
    fn state_view(e: &mut Engine, this: u32) -> (u32, u32, u8, u8, u32, u32) {
        (
            e.mem.u32(this + 4),
            e.mem.u32(this + 0x34),
            e.mem.u8(this + 0x2e),
            e.mem.u8(this + 0x5e),
            e.mem.u32(this + 0x14),
            e.mem.u32(this + 0x44),
        )
    }

    #[test]
    fn fn_00e88fc0_marks_and_moves_slots() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        // Slot 8: nothing.
        set_state(&mut e, this, 6, 7, 1);
        assert_eq!(state_view(&mut e, this), (10, 20, 0, 0, 0, 0));
        // Flag set, unmarked: marked at +0x2c, value at +0xc + 8, +4 up.
        set_state(&mut e, this, 5, 0x77, 1);
        assert_eq!(state_view(&mut e, this), (11, 20, 1, 0, 0x77, 0));
        // Flag set, marked: only the value.
        set_state(&mut e, this, 5, 0x78, 1);
        assert_eq!(state_view(&mut e, this), (11, 20, 1, 0, 0x78, 0));
        // Flag clear, marked at +0x2c: moved (+0x34 up, +4 down), then +0x34 up again.
        set_state(&mut e, this, 5, 0x79, 0);
        assert_eq!(state_view(&mut e, this), (10, 22, 0, 1, 0x78, 0x79));
        // Flag clear, marked at +0x5c: only the value.
        set_state(&mut e, this, 5, 0x7a, 0);
        assert_eq!(state_view(&mut e, this), (10, 22, 0, 1, 0x78, 0x7a));
        // Flag set, marked at +0x5c: moved (+0x34 down, +4 up), then +4 up again.
        set_state(&mut e, this, 5, 0x7b, 1);
        assert_eq!(state_view(&mut e, this), (12, 21, 1, 0, 0x7b, 0x7a));
    }

    #[test]
    fn fn_00e88fc0_flag_clear_unmarked() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        set_state(&mut e, this, 5, 0x55, 0x100);
        assert_eq!(state_view(&mut e, this), (10, 21, 0, 1, 0, 0x55));
    }

    fn set_sampler(e: &mut Engine, this: u32, value: u32, enable: u32, extra: u32) {
        e.call(0x00e8_90c0, &args![this, 5u32, value, enable, extra]);
    }

    /// (+0x64, +0x88, +0xac, bytes +0x82, +0xa6, +0xb2, words +0x74, +0x98).
    type SamplerView = (u32, u32, u32, u8, u8, u8, u32, u32);

    fn sampler_view(e: &mut Engine, this: u32) -> SamplerView {
        (
            e.mem.u32(this + 0x64),
            e.mem.u32(this + 0x88),
            e.mem.u32(this + 0xac),
            e.mem.u8(this + 0x82),
            e.mem.u8(this + 0xa6),
            e.mem.u8(this + 0xb2),
            e.mem.u32(this + 0x74),
            e.mem.u32(this + 0x98),
        )
    }

    #[test]
    fn fn_00e890c0_marks_and_moves_slots() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        // Slot 5: nothing.
        e.call(0x00e8_90c0, &args![this, 6u32, 1u32, 1u32, 1u32]);
        assert_eq!(sampler_view(&mut e, this), (30, 40, 50, 0, 0, 0, 0, 0));
        // Enable, unmarked, extra: +0x80 marked, +0x64 up, +0xac up, +0xb0 byte set.
        set_sampler(&mut e, this, 0x11, 1, 1);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 1, 0x11, 0));
        // Enable, marked, extra again: value only (the +0xb0 byte was set).
        set_sampler(&mut e, this, 0x12, 1, 1);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 1, 0x12, 0));
        // Enable, marked, no extra: +0xac down, byte cleared.
        set_sampler(&mut e, this, 0x13, 1, 0);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 50, 1, 0, 0, 0x13, 0));
        // No extra again: nothing counted.
        set_sampler(&mut e, this, 0x13, 1, 0);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 50, 1, 0, 0, 0x13, 0));
        // Enable, marked, extra with the byte clear: +0xac up.
        set_sampler(&mut e, this, 0x14, 1, 2);
        assert_eq!(sampler_view(&mut e, this), (31, 40, 51, 1, 0, 2, 0x14, 0));
        // Disable, +0x80 marked with the +0xb0 byte set: unmarked (+0x64 down, +0xac down),
        // +0xa4 marked, +0x88 up; no extra.
        set_sampler(&mut e, this, 0x21, 0, 0);
        assert_eq!(
            sampler_view(&mut e, this),
            (30, 41, 50, 0, 1, 0, 0x14, 0x21)
        );
        // Disable, marked: value; extra counts up.
        set_sampler(&mut e, this, 0x22, 0, 1);
        assert_eq!(
            sampler_view(&mut e, this),
            (30, 41, 51, 0, 1, 1, 0x14, 0x22)
        );
        // Enable, +0xa4 marked with the byte set: +0x88 down, +0xac down, then +0x80
        // marked, +0x64 up, and the extra flag set counts +0xac up again.
        set_sampler(&mut e, this, 0x31, 1, 1);
        assert_eq!(
            sampler_view(&mut e, this),
            (31, 40, 51, 1, 0, 1, 0x31, 0x22)
        );
    }

    #[test]
    fn fn_00e890c0_disable_unmarked() {
        let mut e = Engine::new();
        let this = setup(&mut e);
        set_sampler(&mut e, this, 0x41, 0x100, 0);
        assert_eq!(sampler_view(&mut e, this), (30, 41, 50, 0, 1, 0, 0, 0x41));
    }
}
