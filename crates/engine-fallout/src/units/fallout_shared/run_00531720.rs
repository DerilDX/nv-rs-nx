//! `fallout shared/run_00531720` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00531720` and `00531d20` push the values of an image-space modifier's interpolators
//! (objects inside `this`: 21 pairs at `+0x114`, stride 0x30 and 0x18, then single ones from
//! `+0x504`) into the image-space manager (`004e3270`); the class names are not confirmed.
//! The others are small helpers of four-float values and clamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// `004e3270` (`cdecl`, no argument; engine-map unit `teswater.cpp`): the manager the values go to.
const MANAGER: u32 = 0x004e_3270;
/// The float the modifier's time is measured from (`011c96fc`).
const CURRENT_TIME: u32 = 0x011c_96fc;
/// The upper clamp of the first single value in `00531720` (`01018f98`), the float both
/// halves of the point `00452dc0` builds (`01016248`).
const FIRST_VALUE_LIMIT: u32 = 0x0101_8f98;
const POINT_HALF: u32 = 0x0101_6248;
/// `00414430` (`thiscall`, `RET 0x10`): sets the four floats of a colour.
const COLOR_SET: u32 = 0x0041_4430;
/// `00452dc0` (`thiscall`, `RET 8`): sets a two-float point, answers it.
const POINT_SET: u32 = 0x0045_2dc0;
/// `NiAVObject::GetWorldBound` (engine map, `0043d450`, `thiscall`).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `004a4f40` (`thiscall`, `RET 8`: the answer object and a float; engine-map unit
/// `bgsprimitive.cpp`): scales the object `this` holds into the answer.
const SCALE_INTO: u32 = 0x004a_4f40;
/// `004f5d90` (`thiscall`, `RET 4`, a float) and the float `005330a0` passes (`01021d84`).
const OBJECT_004F5D90: u32 = 0x004f_5d90;
const ARGUMENT_01021D84: u32 = 0x0102_1d84;
/// The address `00532ff0` answers (`011c9750`).
const OBJECT_011C9750: u32 = 0x011c_9750;
/// The manager calls of `00531720` (all `thiscall` on the manager, unit
/// `imagespacemanager.cpp`; the floats as their bits).
const SET_TABLE_VALUE: u32 = 0x00b8_cb30;
const SET_COLOR_A: u32 = 0x00b8_b540;
const SET_COLOR_B: u32 = 0x00b8_b610;
const SET_VALUE_A: u32 = 0x00b8_af10;
const SET_VALUE_B: u32 = 0x00b8_afb0;
const SET_POINT_GROUP: u32 = 0x00b8_b000;
const SET_BOUND_GROUP: u32 = 0x00b8_b760;
const SET_PAIR_GROUP: u32 = 0x00b8_b0d0;
const SET_PAIR_BOUND_GROUP: u32 = 0x00b8_b8d0;
const SET_VALUE_C: u32 = 0x00b8_af60;
/// The manager calls of `00531d20`.
const ADD_TABLE_VALUE: u32 = 0x00b8_cc20;
const ADD_COLOR_A: u32 = 0x00b8_b5c0;
const ADD_COLOR_B: u32 = 0x00b8_b690;
const ADD_VALUE_A: u32 = 0x00b8_af40;
const ADD_VALUE_B: u32 = 0x00b8_afe0;
const ADD_POINT_GROUP: u32 = 0x00b8_b6e0;
const ADD_PAIR_GROUP: u32 = 0x00b8_b130;
const ADD_VALUE_C: u32 = 0x00b8_af90;

/// The single interpolators after the pairs, and the frame slot (offset below `EBP`) each
/// one's value goes to, in the order the exe asks them.
const SINGLES: [(u32, u32); 11] = [
    (0x504, 0x04),
    (0x51c, 0x30),
    (0x57c, 0x2c),
    (0x594, 0x38),
    (0x5ac, 0x40),
    (0x5c4, 0x44),
    (0x5dc, 0x24),
    (0x5f4, 0x18),
    (0x60c, 0x28),
    (0x624, 0x34),
    (0x63c, 0x48),
];

/// The game's frame of the two pushers: `slot(n)` is `EBP - n`.
struct Frame(u32);

impl Frame {
    const SIZE: u32 = 0x90;
    fn slot(&self, below: u32) -> u32 {
        self.0 + Self::SIZE - below
    }
}

/// `value * scale` (`FLD; FMUL dword`), stored as a float and passed on (`FSTP; FLD; FSTP`).
fn scaled(e: &Engine, at: u32, scale: u32) -> u32 {
    x87_quiet_bits(x87_store32(x87_mul(
        x87_load32(e.mem.u32(at)),
        x87_m32(scale),
    )))
}

/// The modifier's position in its time: `(time(011c96fc) - start) / length(+0x24)` when the
/// byte at `+0x20` is set, else 0, stored as a float.
fn position(e: &Engine, this: u32, start: u32) -> u32 {
    if e.mem.u8(this + 0x20) == 0 {
        return x87_store32(0.0);
    }
    let elapsed = x87_sub(x87_load32(e.mem.u32(CURRENT_TIME)), x87_m32(start));
    x87_store32(x87_div(elapsed, x87_m32(e.mem.u32(this + 0x24))))
}

/// The 21 pairs: both of a pair are asked (slot `0x9c` with `(t, 0, &buffer[j])`), then the
/// buffer goes to the manager's `table` call by index range: 0-13 as kind 1, 14-16 as kind
/// 0 (index - 14), 17-20 as kind 3 (index - 17), with `scale`.
fn push_pairs(
    e: &mut Engine,
    this: u32,
    frame: &Frame,
    t: u32,
    manager: u32,
    scale: u32,
    table: u32,
) {
    let buffer = frame.slot(0x20);
    for index in 0..0x15u32 {
        for half in 0..2u32 {
            let object = this + 0x114 + index * 0x30 + half * 0x18;
            let t = x87_quiet_bits(t);
            e.vcall(object, 0x9c, &args![t, 0u32, buffer + 4 * half]);
        }
        let scale = x87_quiet_bits(scale);
        if index <= 0xd {
            e.call(table, &args![manager, 1u32, index, buffer, scale]);
        }
        if (0xe..=0x10).contains(&index) {
            e.call(table, &args![manager, 0u32, index - 0xe, buffer, scale]);
        }
        if (0x11..=0x14).contains(&index) {
            e.call(table, &args![manager, 3u32, index - 0x11, buffer, scale]);
        }
    }
}

// Translated from 005330e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the address of a float, then `low` and `high` as
/// their bits): clamps the float: set to `high` when `high` is below it, else to `low` when
/// `low` is above it (ordered compares; copies through the x87 stack).
pub fn fn_005330e0(e: &mut Engine, value: Ptr, low: u32, high: u32) {
    let at = value.addr();
    // FLD *v; FLD high; FCOMPP; TEST AH,5; JP: set when high < *v.
    if x87_load32(high) < x87_load32(e.mem.u32(at)) {
        e.mem.set_u32(at, x87_quiet_bits(high));
    } else if x87_load32(low) > x87_load32(e.mem.u32(at)) {
        // FLD *v; FLD low; FCOMPP; TEST AH,0x41; JNZ: set when low > *v.
        e.mem.set_u32(at, x87_quiet_bits(low));
    }
}

// Translated from 005321d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, a float as its bits): multiplies the four
/// floats of `this` by `scale` in place (`FMUL dword`). Answers `this`.
pub fn fn_005321d0(e: &mut Engine, this: Ptr, scale: u32) -> Ptr {
    for word in 0..4 {
        let at = this.addr() + 4 * word;
        let product = x87_mul(x87_load32(e.mem.u32(at)), x87_m32(scale));
        e.mem.set_u32(at, x87_store32(product));
    }
    this
}

// Translated from 00532e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `answer = this + other` for four
/// floats (`this` copied first, each sum through the x87 stack). Answers `answer`.
pub fn fn_00532e40(e: &mut Engine, this: Ptr, answer: Ptr, other: Ptr) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    let sums: Vec<u32> = (0..4u32)
        .map(|word| {
            let other_word = e.mem.u32(other.addr() + 4 * word);
            x87_store32(x87_add(
                x87_load32(copy[word as usize]),
                x87_m32(other_word),
            ))
        })
        .collect();
    for (word, value) in sums.iter().enumerate() {
        e.mem.set_u32(answer.addr() + 4 * word as u32, *value);
    }
    answer
}

// Translated from 00532ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `answer = this - other` for four
/// floats, as `00532e40` does the sum. Answers `answer`.
pub fn fn_00532ec0(e: &mut Engine, this: Ptr, answer: Ptr, other: Ptr) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    let differences: Vec<u32> = (0..4u32)
        .map(|word| {
            let other_word = e.mem.u32(other.addr() + 4 * word);
            x87_store32(x87_sub(
                x87_load32(copy[word as usize]),
                x87_m32(other_word),
            ))
        })
        .collect();
    for (word, value) in differences.iter().enumerate() {
        e.mem.set_u32(answer.addr() + 4 * word as u32, *value);
    }
    answer
}

// Translated from 00532f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the answer and a float as its bits):
/// `answer = this * scale` for four floats (`this` copied first). Answers `answer`.
pub fn fn_00532f60(e: &mut Engine, this: Ptr, answer: Ptr, scale: u32) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    for (word, value) in copy.iter().enumerate() {
        let product = x87_store32(x87_mul(x87_load32(*value), x87_m32(scale)));
        e.mem.set_u32(answer.addr() + 4 * word as u32, product);
    }
    answer
}

// Translated from 00532f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the answer, a float as its bits, the value): the
/// `scale * value` operator, `00532f60` on `value` (the float copied through the x87
/// stack). Answers `answer`.
pub fn fn_00532f40(e: &mut Engine, answer: Ptr, scale: u32, value: Ptr) -> Ptr {
    fn_00532f60(e, value, answer, x87_quiet_bits(scale));
    answer
}

// Translated from 00532fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the answer, a float as its bits, the value): the
/// same operator for another class, `004a4f40` on `value`. Answers `answer`.
pub fn fn_00532fd0(e: &mut Engine, answer: Ptr, scale: u32, value: Ptr) -> Ptr {
    e.call(SCALE_INTO, &args![value, answer, x87_quiet_bits(scale)]);
    answer
}

// Translated from 00532ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument): the address `011c9750`.
pub fn fn_00532ff0(_e: &mut Engine) -> u32 {
    OBJECT_011C9750
}

// Translated from 005330a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the object's own slot `0x14`, the byte at
/// `+0x20` set, then `004f5d90` on the object at `+0x700` with the float at `01021d84`.
pub fn fn_005330a0(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x14, &[]);
    e.mem.set_u8(this.addr() + 0x20, 1);
    let value = x87_quiet_bits(e.mem.u32(ARGUMENT_01021D84));
    let target = e.mem.u32(this.addr() + 0x700);
    e.call(OBJECT_004F5D90, &args![target, value]);
}

// Translated from 00531720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `start` and `scale` as float bits, an
/// object): sets the manager's values from the interpolators at position `t` (see
/// `position`, clamped to 0..1 with `005330e0`): the 21 pairs (`00b8cb30`), the colours at
/// `+0x534` and `+0x558` (slot `0x90`; alpha times `scale`, clamped to 0..1;
/// `00b8b540`/`00b8b610`), then the eleven single values. The first, times `scale` and
/// clamped to 0..`[01018f98]`, goes to `00b8af10`, the second times `scale` to `00b8afb0`.
/// The next group (times `scale` for the first only) goes with the object's world bound
/// (`00b8b760`, after the object's slot `0xbc`) when the byte at `+0xe8` is set and there
/// is an object, else with a point (`00452dc0` of the float at `01016248` twice; byte set)
/// or the point at `+0xec` (`00b8b000`). The pair group: with the byte at `+0x100` and an
/// object, `00b8b8d0` with the world bound; with the byte only, `00b8b0d0` with 0 and 1;
/// else `00b8b0d0` with two of the values; each with the byte at `+0x101`. The last value
/// goes to `00b8af60`.
pub fn fn_00531720(e: &mut Engine, this: Ptr, start: u32, scale: u32, object: Ptr) {
    let node = this.addr();
    e.with_stack(Frame::SIZE, |e, base| {
        let frame = Frame(base.addr());
        let t_at = frame.slot(0x3c);
        let t = position(e, node, start);
        e.mem.set_u32(t_at, t);
        fn_005330e0(e, Ptr::new(t_at), x87_store32(0.0), x87_store32(1.0));
        let manager = e.call(MANAGER, &args![]).u32();
        let t = e.mem.u32(t_at);
        push_pairs(e, node, &frame, t, manager, scale, SET_TABLE_VALUE);
        let color = frame.slot(0x14);
        let zero = x87_store32(0.0);
        e.call(COLOR_SET, &args![color, zero, zero, zero, zero]);
        for (offset, setter) in [(0x534, SET_COLOR_A), (0x558, SET_COLOR_B)] {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x90, &args![t, 0u32, color]);
            let alpha = frame.slot(8);
            let product = x87_mul(x87_load32(e.mem.u32(alpha)), x87_m32(scale));
            e.mem.set_u32(alpha, x87_store32(product));
            fn_005330e0(e, Ptr::new(alpha), x87_store32(0.0), x87_store32(1.0));
            e.call(setter, &args![manager, color]);
        }
        for (offset, slot) in SINGLES {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x9c, &args![t, 0u32, frame.slot(slot)]);
        }
        let value = |e: &Engine, slot: u32| x87_quiet_bits(e.mem.u32(frame.slot(slot)));
        let first = frame.slot(4);
        let product = x87_mul(x87_load32(e.mem.u32(first)), x87_m32(scale));
        e.mem.set_u32(first, x87_store32(product));
        let limit = x87_quiet_bits(e.mem.u32(FIRST_VALUE_LIMIT));
        fn_005330e0(e, Ptr::new(first), x87_store32(0.0), limit);
        let first = value(e, 4);
        e.call(SET_VALUE_A, &args![manager, first]);
        let second = scaled(e, frame.slot(0x30), scale);
        e.call(SET_VALUE_B, &args![manager, second]);
        let use_bound = e.mem.u8(node + 0xe8) != 0;
        if use_bound && !object.is_null() {
            e.vcall(object.addr(), 0xbc, &[]);
            let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
            let group = [
                scaled(e, frame.slot(0x2c), scale),
                value(e, 0x38),
                value(e, 0x40),
                value(e, 0x44),
                value(e, 0x24),
            ];
            e.call(
                SET_BOUND_GROUP,
                &args![manager, group[0], group[1], group[2], group[3], group[4], bound],
            );
        } else {
            let point = if use_bound {
                let half = x87_quiet_bits(e.mem.u32(POINT_HALF));
                e.call(POINT_SET, &args![frame.slot(0x5c), half, half])
                    .u32()
            } else {
                node + 0xec
            };
            let group = [
                scaled(e, frame.slot(0x2c), scale),
                value(e, 0x38),
                value(e, 0x40),
                value(e, 0x44),
                value(e, 0x24),
            ];
            e.call(
                SET_POINT_GROUP,
                &args![manager, group[0], group[1], group[2], group[3], group[4], point],
            );
        }
        let use_pair_bound = e.mem.u8(node + 0x100) != 0;
        let flag = u32::from(e.mem.u8(node + 0x101));
        if use_pair_bound && !object.is_null() {
            e.vcall(object.addr(), 0xbc, &[]);
            let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(
                SET_PAIR_BOUND_GROUP,
                &args![manager, pair, flag, zero, bound],
            );
        } else if use_pair_bound {
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(
                SET_PAIR_GROUP,
                &args![manager, pair, flag, zero, zero, x87_store32(1.0)],
            );
        } else {
            let (low, high) = (value(e, 0x28), value(e, 0x34));
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(SET_PAIR_GROUP, &args![manager, pair, flag, zero, low, high]);
        }
        let last = value(e, 0x48);
        e.call(SET_VALUE_C, &args![manager, last]);
    });
}

// Translated from 00531d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `start` and `scale` as float bits): adds
/// the interpolators' values at position `t` (not clamped) to the manager: the 21 pairs
/// (`00b8cc20`), the colours scaled by `scale` (`005321d0`; `00b8b5c0`/`00b8b690`), and the
/// eleven single values each times `scale`: `00b8af40`, `00b8afe0`, a group of five with
/// the point at `+0xec` scaled into a local (`004a4f40`; `00b8b6e0`), the pair group with
/// the byte at `+0x101` and 0 (`00b8b130`) and the last value (`00b8af90`).
pub fn fn_00531d20(e: &mut Engine, this: Ptr, start: u32, scale: u32) {
    let node = this.addr();
    e.with_stack(Frame::SIZE, |e, base| {
        let frame = Frame(base.addr());
        let t_at = frame.slot(0x3c);
        let t = position(e, node, start);
        e.mem.set_u32(t_at, t);
        let manager = e.call(MANAGER, &args![]).u32();
        push_pairs(e, node, &frame, t, manager, scale, ADD_TABLE_VALUE);
        let color = frame.slot(0x14);
        let zero = x87_store32(0.0);
        e.call(COLOR_SET, &args![color, zero, zero, zero, zero]);
        for (offset, adder) in [(0x534, ADD_COLOR_A), (0x558, ADD_COLOR_B)] {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x90, &args![t, 0u32, color]);
            fn_005321d0(e, Ptr::new(color), x87_quiet_bits(scale));
            e.call(adder, &args![manager, color]);
        }
        for (offset, slot) in SINGLES {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x9c, &args![t, 0u32, frame.slot(slot)]);
        }
        let first = scaled(e, frame.slot(4), scale);
        e.call(ADD_VALUE_A, &args![manager, first]);
        let second = scaled(e, frame.slot(0x30), scale);
        e.call(ADD_VALUE_B, &args![manager, second]);
        let point = e
            .call(
                SCALE_INTO,
                &args![node + 0xec, frame.slot(0x5c), x87_quiet_bits(scale)],
            )
            .u32();
        let group: Vec<u32> = [0x24u32, 0x44, 0x40, 0x38, 0x2c]
            .iter()
            .map(|slot| scaled(e, frame.slot(*slot), scale))
            .collect();
        e.call(
            ADD_POINT_GROUP,
            &args![manager, group[4], group[3], group[2], group[1], group[0], point],
        );
        let high = scaled(e, frame.slot(0x34), scale);
        let low = scaled(e, frame.slot(0x28), scale);
        let flag = u32::from(e.mem.u8(node + 0x101));
        let pair = scaled(e, frame.slot(0x18), scale);
        e.call(ADD_PAIR_GROUP, &args![manager, pair, flag, zero, low, high]);
        let last = scaled(e, frame.slot(0x48), scale);
        e.call(ADD_VALUE_C, &args![manager, last]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0053_30e0, fn_005330e0(Ptr, u32, u32)),
        entry!(0x0053_21d0, fn_005321d0(Ptr, u32) -> Ptr),
        entry!(0x0053_2e40, fn_00532e40(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0053_2ec0, fn_00532ec0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0053_2f60, fn_00532f60(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0053_2f40, fn_00532f40(Ptr, u32, Ptr) -> Ptr),
        entry!(0x0053_2fd0, fn_00532fd0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x0053_2ff0, fn_00532ff0() -> u32),
        entry!(0x0053_30a0, fn_005330a0(Ptr)),
        entry!(0x0053_1720, fn_00531720(Ptr, u32, u32, Ptr)),
        entry!(0x0053_1d20, fn_00531d20(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn clamping() {
        let mut e = Engine::new();
        let at = floats(&mut e, &[5.0]);
        let (low, high) = (1.0f32.to_bits(), 2.0f32.to_bits());
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 2.0);
        e.mem.set_f32(at, -1.0);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 1.0);
        e.mem.set_f32(at, 1.5);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 1.5);
        e.mem.set_f32(at, f32::NAN);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert!(e.mem.f32(at).is_nan());
    }

    #[test]
    fn four_float_arithmetic() {
        let mut e = Engine::new();
        let a = floats(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        let b = floats(&mut e, &[0.5, 0.5, 1.0, 2.0]);
        let out = floats(&mut e, &[0.0; 4]);
        assert_eq!(e.call(0x0053_2e40, &args![a, out, b]).u32(), out);
        assert_eq!(read(&e, out, 4), vec![1.5, 2.5, 4.0, 6.0]);
        e.call(0x0053_2ec0, &args![a, out, b]);
        assert_eq!(read(&e, out, 4), vec![0.5, 1.5, 2.0, 2.0]);
        e.call(0x0053_2f60, &args![a, out, 2.0f32.to_bits()]);
        assert_eq!(read(&e, out, 4), vec![2.0, 4.0, 6.0, 8.0]);
        assert_eq!(
            e.call(0x0053_2f40, &args![out, 0.5f32.to_bits(), a]).u32(),
            out
        );
        assert_eq!(read(&e, out, 4), vec![0.5, 1.0, 1.5, 2.0]);
        assert_eq!(e.call(0x0053_21d0, &args![a, 3.0f32.to_bits()]).u32(), a);
        assert_eq!(read(&e, a, 4), vec![3.0, 6.0, 9.0, 12.0]);
        // The sum into the second operand itself.
        e.call(0x0053_2e40, &args![a, b, b]);
        assert_eq!(read(&e, b, 4), vec![3.5, 6.5, 10.0, 14.0]);
    }

    #[test]
    fn the_other_operator_and_the_small_ones() {
        let mut e = Engine::new();
        e.register(SCALE_INTO, |_, _| Ret::default());
        e.register(OBJECT_004F5D90, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        let mut slots = vec![0u32; 6];
        slots[5] = 0x0300_1000;
        e.put_vtable(0x0300_0000, &slots);
        e.map(0x0102_1000, 0x1000);
        e.mem.set_f32(ARGUMENT_01021D84, 0.75);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0053_2fd0, &args![0x6000u32, 2.0f32.to_bits(), 0x7000u32])
                .u32(),
            0x6000
        );
        assert_eq!(
            calls(&e, SCALE_INTO),
            vec![vec![0x7000, 0x6000, 2.0f32.to_bits()]]
        );
        assert_eq!(e.call(0x0053_2ff0, &args![]).u32(), OBJECT_011C9750);
        let this = e.mem.alloc(0x704);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 0x700, 0x8800);
        e.call(0x0053_30a0, &args![this]);
        assert_eq!(calls(&e, 0x0300_1000), vec![vec![this]]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(
            calls(&e, OBJECT_004F5D90),
            vec![vec![0x8800, 0.75f32.to_bits()]]
        );
    }

    const INTERPOLATOR_VTABLE: u32 = 0x0300_2000;
    const VALUE_SLOT: u32 = 0x0300_3000;
    const COLOR_SLOT: u32 = 0x0300_4000;
    const OBJECT_VTABLE: u32 = 0x0300_5000;
    const OBJECT_SLOT_BC: u32 = 0x0300_6000;

    /// A modifier whose interpolators all answer 2.0 (colours: alpha 2.0) and the engine
    /// with every manager call doubled.
    fn modifier(e: &mut Engine) -> u32 {
        e.map(0x011c_9000, 0x1000);
        e.map(0x0101_6000, 0x3000);
        e.map(0x0102_1000, 0x1000);
        e.mem.set_f32(CURRENT_TIME, 10.0);
        e.mem.set_f32(FIRST_VALUE_LIMIT, 7.0);
        e.mem.set_f32(POINT_HALF, 0.5);
        let mut slots = vec![0u32; 0x28];
        slots[0x90 / 4] = COLOR_SLOT;
        slots[0x9c / 4] = VALUE_SLOT;
        e.put_vtable(INTERPOLATOR_VTABLE, &slots);
        let mut object_slots = vec![0u32; 0x30];
        object_slots[0xbc / 4] = OBJECT_SLOT_BC;
        e.put_vtable(OBJECT_VTABLE, &object_slots);
        e.register(VALUE_SLOT, |e, a| {
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        e.register(COLOR_SLOT, |e, a| {
            e.mem.set_f32(a[3] + 0xc, 2.0);
            Ret::default()
        });
        e.register(MANAGER, |_, _| 0x9900u32.into_ret());
        e.register(GET_WORLD_BOUND, |_, _| 0xb0b0u32.into_ret());
        e.register(POINT_SET, |_, a| a[0].into_ret());
        e.register(SCALE_INTO, |_, a| a[1].into_ret());
        for address in [
            COLOR_SET,
            OBJECT_SLOT_BC,
            SET_TABLE_VALUE,
            SET_COLOR_A,
            SET_COLOR_B,
            SET_VALUE_A,
            SET_VALUE_B,
            SET_POINT_GROUP,
            SET_BOUND_GROUP,
            SET_PAIR_GROUP,
            SET_PAIR_BOUND_GROUP,
            SET_VALUE_C,
            ADD_TABLE_VALUE,
            ADD_COLOR_A,
            ADD_COLOR_B,
            ADD_VALUE_A,
            ADD_VALUE_B,
            ADD_POINT_GROUP,
            ADD_PAIR_GROUP,
            ADD_VALUE_C,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x660);
        for index in 0..0x2a {
            e.mem
                .set_u32(this + 0x114 + 0x18 * index, INTERPOLATOR_VTABLE);
        }
        for offset in [0x534, 0x558] {
            e.mem.set_u32(this + offset, INTERPOLATOR_VTABLE);
        }
        for (offset, _) in SINGLES {
            e.mem.set_u32(this + offset, INTERPOLATOR_VTABLE);
        }
        e.mem.set_u8(this + 0x20, 1);
        e.mem.set_f32(this + 0x24, 4.0);
        e.mem.set_u8(this + 0x101, 1);
        e.call_log = Some(vec![]);
        this
    }

    /// Records the colour alpha each call of `address` sees.
    fn alpha_of(e: &mut Engine, address: u32) -> std::rc::Rc<std::cell::Cell<f32>> {
        let seen = std::rc::Rc::new(std::cell::Cell::new(0.0f32));
        let sink = seen.clone();
        e.register_double(address, move |e, a| {
            sink.set(e.mem.f32(a[1] + 0xc));
            Ret::default()
        });
        seen
    }

    #[test]
    fn setting_the_values() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let alpha = alpha_of(&mut e, SET_COLOR_A);
        let (two, three, six) = (2.0f32.to_bits(), 3.0f32.to_bits(), 6.0f32.to_bits());
        // Position (10 - 8) / 4 = 0.5; scale 3.
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), three, 0u32]);
        let t = 0.5f32.to_bits();
        let asked = calls(&e, VALUE_SLOT);
        assert_eq!(asked.len(), 42 + 11);
        assert!(asked.iter().all(|c| c[1] == t && c[2] == 0));
        let table = calls(&e, SET_TABLE_VALUE);
        assert_eq!(table.len(), 21);
        assert_eq!((table[0][1], table[0][2]), (1, 0));
        assert_eq!((table[14][1], table[14][2]), (0, 0));
        assert_eq!((table[20][1], table[20][2], table[20][4]), (3, 3, three));
        assert_eq!(calls(&e, SET_COLOR_A).len(), 1);
        // The colour alpha 2 * 3 clamped to 1.
        assert_eq!(alpha.get(), 1.0);
        // 2 * 3 = 6, below the limit 7.
        assert_eq!(calls(&e, SET_VALUE_A), vec![vec![0x9900, six]]);
        assert_eq!(calls(&e, SET_VALUE_B), vec![vec![0x9900, six]]);
        assert_eq!(
            calls(&e, SET_POINT_GROUP),
            vec![vec![0x9900, six, two, two, two, two, this + 0xec]]
        );
        assert_eq!(
            calls(&e, SET_PAIR_GROUP),
            vec![vec![0x9900, six, 1, 0, two, two]]
        );
        assert_eq!(calls(&e, SET_VALUE_C), vec![vec![0x9900, two]]);
    }

    #[test]
    fn setting_the_values_with_an_object_and_the_flags() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        e.mem.set_u8(this + 0xe8, 1);
        e.mem.set_u8(this + 0x100, 1);
        e.mem.set_u8(this + 0x20, 0);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        let (one, two, zero) = (1.0f32.to_bits(), 2.0f32.to_bits(), 0u32);
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), one, object]);
        assert!(calls(&e, VALUE_SLOT).iter().all(|c| c[1] == zero));
        assert_eq!(calls(&e, OBJECT_SLOT_BC).len(), 2);
        assert_eq!(
            calls(&e, SET_BOUND_GROUP),
            vec![vec![0x9900, two, two, two, two, two, 0xb0b0]]
        );
        assert_eq!(
            calls(&e, SET_PAIR_BOUND_GROUP),
            vec![vec![0x9900, two, 1, zero, 0xb0b0]]
        );
        // Without the object: the point of two halves and the pair 0..1.
        e.call_log = Some(vec![]);
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), one, 0u32]);
        let half = 0.5f32.to_bits();
        let point = calls(&e, POINT_SET);
        assert_eq!((point[0][1], point[0][2]), (half, half));
        assert_eq!(calls(&e, SET_POINT_GROUP)[0][6], point[0][0]);
        assert_eq!(
            calls(&e, SET_PAIR_GROUP),
            vec![vec![0x9900, two, 1, zero, zero, one]]
        );
    }

    #[test]
    fn adding_the_values() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let alpha = alpha_of(&mut e, ADD_COLOR_A);
        let (four, half) = (4.0f32.to_bits(), 0.5f32.to_bits());
        e.call(
            0x0053_1d20,
            &args![this, 8.0f32.to_bits(), 2.0f32.to_bits()],
        );
        let asked = calls(&e, VALUE_SLOT);
        assert!(asked.iter().all(|c| c[1] == half));
        assert_eq!(calls(&e, ADD_TABLE_VALUE).len(), 21);
        assert_eq!(alpha.get(), 4.0);
        assert_eq!(calls(&e, ADD_VALUE_A), vec![vec![0x9900, four]]);
        assert_eq!(calls(&e, ADD_VALUE_B), vec![vec![0x9900, four]]);
        let scaled_point = calls(&e, SCALE_INTO);
        assert_eq!(scaled_point[0][0], this + 0xec);
        assert_eq!(
            calls(&e, ADD_POINT_GROUP),
            vec![vec![
                0x9900,
                four,
                four,
                four,
                four,
                four,
                scaled_point[0][1]
            ]]
        );
        assert_eq!(
            calls(&e, ADD_PAIR_GROUP),
            vec![vec![0x9900, four, 1, 0, four, four]]
        );
        assert_eq!(calls(&e, ADD_VALUE_C), vec![vec![0x9900, four]]);
    }
}
