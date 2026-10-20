//! `fallout/misc/seendata.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SeenData` keeps one bit per piece of a cell (the local map's fog of war);
//! `IntSeenData` is the interior form, one section of bits per 4096-unit
//! square, chained through `pNextSection`.
//!
//! Notes on the exe's code:
//! - The Xbox PDB gives `SeenData` two statics, `fPosSize` and
//!   `iTotalBitCount`. The code reads a float at `011a31e8` as the size of
//!   one bit's square and a word at `011a31ec` as the number of bits; the
//!   names fit, but the addresses are not tied to the symbols, so the
//!   constants here are named by use.
//! - Floats are x87: each `FLD`/`FSTP` pair quiets a signalling NaN, each
//!   arithmetic instruction goes through `crate::x87`, and floats pushed as
//!   arguments are passed as bit patterns.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `SeenData` (Xbox PDB), 0x24 bytes.
    pub struct SeenData: 0x24 {
        /// `pSeenBits` (Xbox PDB): 0x20 bytes of bits.
        0x04 pSeenBits: u32,
    }

    /// `IntSeenData` (Xbox PDB), 0x2c bytes: a `SeenData` and its section.
    pub struct IntSeenData: 0x2c {
        /// `cSectionX` (Xbox PDB).
        0x24 cSectionX: u8,
        /// `cSectionY` (Xbox PDB).
        0x25 cSectionY: u8,
        /// `pNextSection` (Xbox PDB): `IntSeenData*`.
        0x28 pNextSection: u32,
    }
}

/// The `SeenData` vtable.
const SEEN_DATA_VTABLE: u32 = 0x0108_3fc4;
/// The `IntSeenData` vtable.
const INT_SEEN_DATA_VTABLE: u32 = 0x0108_3fe4;
/// The size of one seen bit's square (a `float`; see the module notes).
const BIT_SQUARE_SIZE: u32 = 0x011a_31e8;
/// The number of seen bits (a word; see the module notes).
const BIT_COUNT: u32 = 0x011a_31ec;
/// `FLT_MAX` (`float`).
const FLOAT_MAX: u32 = 0x0101_6970;
/// The setting whose value (`00403e20`) is the distance under which a piece
/// counts as seen.
const SEEN_DISTANCE_SETTING: u32 = 0x011c_a094;
/// The setting whose value (`0043d4d0`) is the number of seen pieces that
/// makes a section count as fully seen.
const SEEN_COUNT_SETTING: u32 = 0x011d_f198;

/// `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiPoint3` constructor (`00416870`, `thiscall`: x, y, z), answers `this`.
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// `00406d90` (`cdecl`, one `float`): the float as an integer (`EAX`).
const FLOAT_TO_INT: u32 = 0x0040_6d90;
/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0` (an `f64` here).
const FTOL: u32 = 0x00ec_62c0;
/// A float setting's value address (`00403e20`, `thiscall`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// An integer setting's value address (`0043d4d0`, `thiscall`).
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// `006815c0` (`thiscall`): called on the point local before it is filled.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// `0063c8a0` (`thiscall`, `RET 4`): `point += other`.
const POINT_ADD: u32 = 0x0063_c8a0;
/// `00439ef0` (`thiscall`, `RET 8`): `(point, result, other)`, the
/// difference of two points into `result`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00457990` (`thiscall`): a point's length, in `ST0`.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::StartVariableSizedValue` (engine map), `thiscall`.
const START_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f20;
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (engine map),
/// `thiscall`: count, handle.
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5ff0;
/// The `SeenData` constructor (`00879940`, unit `run_008798e0`).
const SEEN_DATA_CONSTRUCT: u32 = 0x0087_9940;

// Translated from 00879970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData` scalar deleting destructor (Xbox PDB name from the engine
/// map): runs the destructor body `008799a0`, frees `this` (`00401030`) when
/// `flags & 1`, returns `this`.
pub fn seen_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_008799a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 008799a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SeenData` destructor body: stores the `SeenData` vtable.
pub fn fn_008799a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SEEN_DATA_VTABLE);
}

/// The word of the seen bits that holds `bit` and the bit's mask in it.
fn bit_word_and_mask(this: Ptr, bit: u32) -> (u32, u32) {
    let word = bit >> 5;
    let shift = bit.wrapping_sub(word << 5);
    (
        this.addr()
            .wrapping_add(word.wrapping_mul(4))
            .wrapping_add(4),
        1u32 << (shift & 0x1f),
    )
}

// Translated from 008799f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::IsSeenBitTrue_ov2` (Xbox PDB): whether bit `bit` of the seen
/// bits is set.
pub fn seen_data_is_seen_bit_true_ov2(e: &mut Engine, this: Ptr, bit: u32) -> bool {
    let (address, mask) = bit_word_and_mask(this, bit);
    mask & e.mem.u32(address) != 0
}

// Translated from 00879a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` non-zero) or clears bit `bit` of the seen bits and returns
/// whether it was set before.
pub fn fn_00879a40(e: &mut Engine, this: Ptr, bit: u32, value: u8) -> bool {
    let (address, mask) = bit_word_and_mask(this, bit);
    let was_set = mask & e.mem.u32(address) != 0;
    let word = e.mem.u32(address);
    if value != 0 {
        e.mem.set_u32(address, mask | word);
    } else {
        e.mem.set_u32(address, !mask & word);
    }
    was_set
}

// Translated from 00879ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks every unseen bit whose piece is near enough as seen and answers
/// whether enough bits are seen (`thiscall`, `RET 8`). For each bit below
/// the bit count (re-read each pass, unsigned): a set bit counts; for an
/// unset one, the piece's point (`00879bd0`) is moved by `origin`
/// (`0063c8a0`), its difference from `position` (`00439ef0`) measured
/// (`00457990`), and when the seen distance (`00879bb0`) is greater than
/// that length the bit is set (`00879a40`) and counts. Answers whether the
/// count (signed) reaches the value of the setting at `011df198`.
pub fn fn_00879ad0(e: &mut Engine, this: Ptr, position: u32, origin: u32) -> bool {
    e.with_stack(0x18, |e, locals| {
        let point = locals.addr();
        let difference = locals.addr() + 0xc;
        let mut count = 0i32;
        let mut bit = 0u32;
        while bit < e.global::<u32>(BIT_COUNT) {
            if seen_data_is_seen_bit_true_ov2(e, this, bit) {
                count = count.wrapping_add(1);
            } else {
                e.call(POINT_PREPARE, &args![point]);
                fn_00879bd0(e, bit, point);
                e.call(POINT_ADD, &args![point, origin]);
                e.call(POINT_SUBTRACT, &args![point, difference, position]);
                let length = e.call(POINT_LENGTH, &args![difference]).f64();
                let seen_distance = fn_00879bb0(e) as f64;
                if seen_distance > length {
                    fn_00879a40(e, this, bit, 1);
                    count = count.wrapping_add(1);
                }
            }
            bit = bit.wrapping_add(1);
        }
        let needed = e.call(INT_SETTING_VALUE, &args![SEEN_COUNT_SETTING]).u32();
        count >= e.mem.i32(needed)
    })
}

// Translated from 00879bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The value of the float setting at `011ca094` (the seen distance), loaded
/// with `FLD`.
pub fn fn_00879bb0(e: &mut Engine) -> f32 {
    let value = e
        .call(FLOAT_SETTING_VALUE, &args![SEEN_DISTANCE_SETTING])
        .u32();
    x87_quiet(e.mem.f32(value))
}

// Translated from 00879bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The point of bit `bit` (`cdecl`, two words), written to `out`: for a bit
/// below the bit count, `(size * (bit % 16), size * (bit / 16), 0)` built
/// with the `NiPoint3` constructor (`00416870`) on a local, with `size` the
/// float at `011a31e8`; otherwise `(FLT_MAX, FLT_MAX, FLT_MAX)`.
pub fn fn_00879bd0(e: &mut Engine, bit: u32, out: u32) {
    e.with_stack(0xc, |e, local| {
        let point = if bit < e.global::<u32>(BIT_COUNT) {
            let column = x87_store32(f64::from(bit % 0x10));
            let row = x87_store32(f64::from(bit >> 4));
            let size = e.global::<u32>(BIT_SQUARE_SIZE);
            let y = x87_store32(x87_mul(x87_load32(size), x87_m32(row)));
            let x = x87_store32(x87_mul(x87_load32(size), x87_m32(column)));
            e.call(
                POINT_CONSTRUCT,
                &args![local, x87_quiet_bits(x), x87_quiet_bits(y), 0u32],
            )
            .u32()
        } else {
            let max = x87_quiet_bits(e.global::<u32>(FLOAT_MAX));
            e.call(POINT_CONSTRUCT, &args![local, max, max, max]).u32()
        };
        for i in 0..3 {
            let word = e.mem.u32(point + 4 * i);
            e.mem.set_u32(out + 4 * i, word);
        }
    });
}

// Translated from 00879ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Snaps `point` (`cdecl`, one word) to the corner of its seen bit's square:
/// for x and y, the 4096-unit cell origin (`00406d90(value) >> 12 << 12`)
/// plus the whole number of squares (`_ftol2` of
/// `00406d90(value - origin) / size`) times the square size; z becomes 0.
pub fn fn_00879ca0(e: &mut Engine, point: u32) {
    let x_bits = e.mem.u32(point);
    let cell_x = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(x_bits)]).i32() >> 12;
    let y_bits = e.mem.u32(point + 4);
    let cell_y = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(y_bits)]).i32() >> 12;

    let size = e.global::<u32>(BIT_SQUARE_SIZE);

    let x_bits = e.mem.u32(point);
    let offset_x = x87_store32(x87_sub(x87_m32(x_bits), f64::from(cell_x.wrapping_shl(12))));
    let whole_x = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(offset_x)]).i32();
    let squares_x = e
        .call(FTOL, &args![x87_div(f64::from(whole_x), x87_m32(size))])
        .i32();

    let y_bits = e.mem.u32(point + 4);
    let offset_y = x87_store32(x87_sub(x87_m32(y_bits), f64::from(cell_y.wrapping_shl(12))));
    let whole_y = e.call(FLOAT_TO_INT, &args![x87_quiet_bits(offset_y)]).i32();
    let squares_y = e
        .call(FTOL, &args![x87_div(f64::from(whole_y), x87_m32(size))])
        .i32();

    let x = x87_add(
        f64::from(cell_x.wrapping_shl(12)),
        x87_mul(f64::from(squares_x), x87_m32(size)),
    );
    e.mem.set_u32(point, x87_store32(x));
    let y = x87_add(
        f64::from(cell_y.wrapping_shl(12)),
        x87_mul(f64::from(squares_y), x87_m32(size)),
    );
    e.mem.set_u32(point + 4, x87_store32(y));
    e.mem.set_u32(point + 8, 0);
}

// Translated from 0087a0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SeenData::SaveGame_ov2` (Xbox PDB): saves the 0x20 bytes of seen bits
/// (`00865e50(buffer, this + 4, 0x20, 0)`).
pub fn seen_data_save_game_ov2(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(
        SAVE_DATA,
        &args![buffer, this.addr().wrapping_add(4), 0x20u32, 0u32],
    );
}

// Translated from 0087a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `IntSeenData` constructor (no name in the engine map): the
/// `SeenData` constructor (`00879940`), the `IntSeenData` vtable, the
/// section's x and y bytes and no next section. Returns `this`.
pub fn fn_0087a100(e: &mut Engine, this: Ptr<IntSeenData>, x: u8, y: u8) -> Ptr<IntSeenData> {
    e.call(SEEN_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), INT_SEEN_DATA_VTABLE);
    e.set(this, IntSeenData::cSectionX, x);
    e.set(this, IntSeenData::cSectionY, y);
    e.set(this, IntSeenData::pNextSection, 0);
    this
}

// Translated from 0087a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData` scalar deleting destructor (Xbox PDB name from the engine
/// map): runs the destructor body `0087a170`, frees `this` (`00401030`) when
/// `flags & 1`, returns `this`.
pub fn int_seen_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<IntSeenData>,
    flags: u32,
) -> Ptr<IntSeenData> {
    fn_0087a170(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0087a170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0087a170 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// The `IntSeenData` destructor body: stores the `IntSeenData` vtable,
/// deletes the next section through its virtual slot 0 (the scalar deleting
/// destructor, flags 1) when there is one, then runs the `SeenData`
/// destructor body (`008799a0`).
pub fn fn_0087a170(e: &mut Engine, this: Ptr<IntSeenData>) {
    e.mem.set_u32(this.addr(), INT_SEEN_DATA_VTABLE);
    let next = e.get(this, IntSeenData::pNextSection);
    if next != 0 {
        e.vcall(next, 0, &args![1u32]);
    }
    fn_008799a0(e, this.cast());
}

// Translated from 0087a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IntSeenData::SaveGame_ov2` (Xbox PDB): starts a variable-sized value
/// (`00865f20`), saves each section of the chain from `this` on (its x and
/// y bytes with `00865e50`, then its bits, `0087a0c0`) and ends the value
/// with the number of sections (`00865ff0`).
pub fn int_seen_data_save_game_ov2(e: &mut Engine, this: Ptr<IntSeenData>, buffer: u32) {
    let mut count = 0u32;
    let handle = e.call(START_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let mut section = this;
    while !section.is_null() {
        e.call(
            SAVE_DATA,
            &args![buffer, section.addr().wrapping_add(0x24), 1u32, 0u32],
        );
        e.call(
            SAVE_DATA,
            &args![buffer, section.addr().wrapping_add(0x25), 1u32, 0u32],
        );
        seen_data_save_game_ov2(e, section.cast(), buffer);
        section = Ptr::new(e.get(section, IntSeenData::pNextSection));
        count = count.wrapping_add(1);
    }
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count, handle]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0087_9970, seen_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0087_99a0, fn_008799a0(Ptr)),
        entry!(0x0087_99f0, seen_data_is_seen_bit_true_ov2(Ptr, u32) -> bool),
        entry!(0x0087_9a40, fn_00879a40(Ptr, u32, u8) -> bool),
        entry!(0x0087_9ad0, fn_00879ad0(Ptr, u32, u32) -> bool),
        entry!(0x0087_9bb0, fn_00879bb0() -> f32),
        entry!(0x0087_9bd0, fn_00879bd0(u32, u32)),
        entry!(0x0087_9ca0, fn_00879ca0(u32)),
        entry!(0x0087_a0c0, seen_data_save_game_ov2(Ptr, u32)),
        entry!(
            0x0087_a100,
            fn_0087a100(Ptr<IntSeenData>, u8, u8) -> Ptr<IntSeenData>
        ),
        entry!(
            0x0087_a140,
            int_seen_data_scalar_deleting_destructor(Ptr<IntSeenData>, u32) -> Ptr<IntSeenData>
        ),
        entry!(0x0087_a170, fn_0087a170(Ptr<IntSeenData>)),
        entry!(
            0x0087_a4e0,
            int_seen_data_save_game_ov2(Ptr<IntSeenData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_9970, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0087_9970, &args![this, 3u32]).u32(), this);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn fn_008799a0_restores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.call(0x0087_99a0, &args![this]);
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
    }

    #[test]
    fn is_seen_bit_true_reads_the_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 4, 0x8000_0001);
        e.mem.set_u32(this + 8, 0x0000_0004);
        assert!(e.call(0x0087_99f0, &args![this, 0u32]).bool());
        assert!(e.call(0x0087_99f0, &args![this, 31u32]).bool());
        assert!(!e.call(0x0087_99f0, &args![this, 1u32]).bool());
        assert!(e.call(0x0087_99f0, &args![this, 34u32]).bool());
        assert!(!e.call(0x0087_99f0, &args![this, 35u32]).bool());
    }

    #[test]
    fn fn_00879a40_sets_and_clears_and_answers_the_old_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        assert!(!e.call(0x0087_9a40, &args![this, 33u32, 1u8]).bool());
        assert_eq!(e.mem.u32(this + 8), 2);
        assert!(e.call(0x0087_9a40, &args![this, 33u32, 1u8]).bool());
        assert!(e.call(0x0087_9a40, &args![this, 33u32, 0u8]).bool());
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(!e.call(0x0087_9a40, &args![this, 33u32, 0u8]).bool());
    }

    #[test]
    fn fn_00879bb0_reads_the_seen_distance_setting() {
        let mut e = Engine::new();
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 512.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, a| {
            assert_eq!(a[0], SEEN_DISTANCE_SETTING);
            word(value)
        });
        assert_eq!(e.call(0x0087_9bb0, &[]).f64(), 512.0);
    }

    fn map_seen_globals(e: &mut Engine, size: f32, count: u32) {
        e.map(0x011a_3000, 0x1000);
        e.set_global(BIT_SQUARE_SIZE, size);
        e.set_global(BIT_COUNT, count);
        e.map(0x0101_6000, 0x1000);
        e.set_global(FLOAT_MAX, f32::MAX);
    }

    fn point_constructor(e: &mut Engine) {
        e.register(POINT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            word(a[0])
        });
    }

    #[test]
    fn fn_00879bd0_builds_the_point_of_a_bit() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 256.0, 256);
        point_constructor(&mut e);
        let out = e.mem.alloc(0xc);
        e.call(0x0087_9bd0, &args![0x23u32, out]);
        assert_eq!(e.mem.f32(out), 768.0);
        assert_eq!(e.mem.f32(out + 4), 512.0);
        assert_eq!(e.mem.f32(out + 8), 0.0);
        e.call(0x0087_9bd0, &args![256u32, out]);
        for i in 0..3 {
            assert_eq!(e.mem.f32(out + 4 * i), f32::MAX);
        }
    }

    #[test]
    fn fn_00879ca0_snaps_the_point_to_its_square() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 256.0, 256);
        e.register(
            FLOAT_TO_INT,
            |_, a| word(f32::from_bits(a[0]) as i32 as u32),
        );
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(value as i32 as u32)
        });
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(point, 5000.0);
        e.mem.set_f32(point + 4, -100.0);
        e.mem.set_f32(point + 8, 77.0);
        e.call(0x0087_9ca0, &args![point]);
        // x: cell 4096 + 3 squares of 256 (904 / 256 = 3).
        assert_eq!(e.mem.f32(point), 4096.0 + 768.0);
        // y: -100 >> 12 is cell -1 (-4096); -100 + 4096 = 3996, 15 squares.
        assert_eq!(e.mem.f32(point + 4), -4096.0 + 15.0 * 256.0);
        assert_eq!(e.mem.u32(point + 8), 0);
    }

    #[test]
    fn fn_00879ad0_marks_near_pieces_and_compares_the_count() {
        let mut e = Engine::new();
        map_seen_globals(&mut e, 1.0, 4);
        point_constructor(&mut e);
        let this = e.mem.alloc(0x24);
        // Bit 0 already seen.
        e.mem.set_u32(this + 4, 1);
        let distance = e.mem.alloc(4);
        e.mem.set_f32(distance, 2.5);
        let needed = e.mem.alloc(4);
        e.mem.set_u32(needed, 3);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| word(distance));
        e.register_double(INT_SETTING_VALUE, move |_, a| {
            assert_eq!(a[0], SEEN_COUNT_SETTING);
            word(needed)
        });
        e.register(POINT_PREPARE, |_, a| word(a[0]));
        e.register(POINT_ADD, |_, _| Ret::default());
        // The difference keeps the point (bit index as x).
        e.register(POINT_SUBTRACT, |e, a| {
            let x = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], x);
            word(a[1])
        });
        e.register(POINT_LENGTH, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        // Bits 1 and 2 are nearer than 2.5, bit 3 is not: 3 seen, 3 needed.
        assert!(e.call(0x0087_9ad0, &args![this, 0x10u32, 0x20u32]).bool());
        assert_eq!(e.mem.u32(this + 4), 0b0111);
        assert_eq!(calls_to(&e, POINT_ADD).len(), 3);
        assert_eq!(calls_to(&e, POINT_ADD)[0][1], 0x20);
        assert_eq!(calls_to(&e, POINT_SUBTRACT)[0][2], 0x10);
        // With 4 needed the answer is false.
        e.mem.set_u32(needed, 4);
        assert!(!e.call(0x0087_9ad0, &args![this, 0x10u32, 0x20u32]).bool());
    }

    #[test]
    fn save_game_ov2_saves_the_bits() {
        let mut e = Engine::new();
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_a0c0, &args![0x4000u32, 0x9000u32]);
        assert_eq!(calls_to(&e, SAVE_DATA), vec![vec![0x9000, 0x4004, 0x20, 0]]);
    }

    #[test]
    fn int_seen_data_constructor_sets_its_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, 0x1234);
        e.register(SEEN_DATA_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a100, &args![this, 3u8, 0xfeu8]).u32(), this);
        assert_eq!(calls_to(&e, SEEN_DATA_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), INT_SEEN_DATA_VTABLE);
        assert_eq!(e.mem.u8(this + 0x24), 3);
        assert_eq!(e.mem.u8(this + 0x25), 0xfe);
        assert_eq!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn int_seen_data_destructor_deletes_the_next_section() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        let next = e.mem.alloc(0x2c);
        e.put_vtable(0x0300_0000, &[0x0300_1000]);
        e.mem.set_u32(next, 0x0300_0000);
        e.mem.set_u32(this + 0x28, next);
        e.register(0x0300_1000, |_, a| word(a[0]));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_a140, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![next, 1]]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        // The body ends with the base vtable.
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
        // Without a next section nothing is deleted through the vtable.
        e.mem.set_u32(this + 0x28, 0);
        e.call_log = Some(vec![]);
        e.call(0x0087_a170, &args![this]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(e.mem.u32(this), SEEN_DATA_VTABLE);
    }

    #[test]
    fn int_seen_data_save_game_saves_every_section() {
        let mut e = Engine::new();
        let first = e.mem.alloc(0x2c);
        let second = e.mem.alloc(0x2c);
        e.mem.set_u32(first + 0x28, second);
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(START_VARIABLE_SIZED_VALUE, |_, _| word(0x77));
        e.register(SAVE_VARIABLE_SIZED_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0087_a4e0, &args![first, 0x9000u32]);
        assert_eq!(calls_to(&e, START_VARIABLE_SIZED_VALUE), vec![vec![0x9000]]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x9000, first + 0x24, 1, 0],
                vec![0x9000, first + 0x25, 1, 0],
                vec![0x9000, first + 4, 0x20, 0],
                vec![0x9000, second + 0x24, 1, 0],
                vec![0x9000, second + 0x25, 1, 0],
                vec![0x9000, second + 4, 0x20, 0],
            ]
        );
        assert_eq!(
            calls_to(&e, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x9000, 2, 0x77]]
        );
    }
}
