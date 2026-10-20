//! `(unplaced)/run_00649530` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006499c0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d57a4 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_006499c0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_57a4
}

/// `ActorValueOwner::GetClampedActorFloatValue` (`0066ef50`, engine map; `thiscall`:
/// actor value), answering in ST0.
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef50;
/// The getter of a game setting's float (`00403e20`; `thiscall` on the setting): the
/// address of the value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The two settings `00649530` reads (`011d17d0`, `011d1674`) and the divisor (the double
/// at `01017a40`).
const SETTING_BASE: u32 = 0x011d_17d0;
const SETTING_FACTOR: u32 = 0x011d_1674;
const PERCENT_DIVISOR: u32 = 0x0101_7a40;
/// The global `006499d0` answers (`011d5744`).
const CLASS_GLOBAL: u32 = 0x011d_5744;
/// The array lookup the three getters use (`005dd4e0`; `thiscall`: index), answering in
/// ST0.
const ARRAY_VALUE: u32 = 0x005d_d4e0;
/// Callees of the constructor `00649680`, by address.
const BASE_CONSTRUCTOR: u32 = 0x00a7_b440;
const VALUE_ARRAY_CONSTRUCTOR: u32 = 0x0064_eb60;
const LIST_CONSTRUCTOR: u32 = 0x0048_f200;
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
const BIT_SET_CONSTRUCTOR: u32 = 0x0096_a2d0;
/// The constructor's virtual table (`01064e1c`) and the point it copies twice into
/// `+0x124` (`011f426c`, three words).
const CONSTRUCTED_VTABLE: u32 = 0x0106_4e1c;
const DEFAULT_POINT: u32 = 0x011f_426c;

// Translated from 00649530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the owner's clamped actor value 0x25
/// (`0066ef50`) divided by the double at `01017a40` and stored as a float, times the float
/// of the setting `011d1674` plus the float of the setting `011d17d0` (`00403e20`), stored
/// as a float and answered (x87, in ST0).
pub fn fn_00649530(e: &mut Engine, owner: Ptr) -> f32 {
    let value = e
        .call(GET_CLAMPED_ACTOR_VALUE, &args![owner, 0x25u32])
        .f64();
    let divisor = f64::from_bits(e.mem.u64(PERCENT_DIVISOR));
    let fraction = x87_store32(x87_div(value, divisor));
    let base = e.call(SETTING_VALUE, &args![SETTING_BASE]).u32();
    let factor = e.call(SETTING_VALUE, &args![SETTING_FACTOR]).u32();
    let product = x87_mul(x87_load32(fraction), x87_m32(e.mem.u32(factor)));
    let sum = x87_add(product, x87_m32(e.mem.u32(base)));
    f32::from_bits(x87_store32(sum))
}

// Translated from 006499d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (plain `RET`, no arguments): the word at `011d5744`.
pub fn fn_006499d0(e: &mut Engine) -> u32 {
    e.global::<u32>(CLASS_GLOBAL)
}

/// The three getters' shared body: below `limit` (signed) the array value of `index`
/// (`005dd4e0` on `this + offset`), else 0.0; stored as a float and answered.
fn array_value_below(e: &mut Engine, this: Ptr, index: i32, limit: i32, offset: u32) -> f32 {
    let stored = if index < limit {
        let value = e
            .call(ARRAY_VALUE, &args![this.addr() + offset, index as u32])
            .f64();
        x87_store32(value)
    } else {
        x87_store32(0.0)
    };
    f32::from_bits(stored)
}

// Translated from 006499e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): for an index below 15 the value of the
/// array at `this + 0x74` (`005dd4e0`), else 0.0, as a float in ST0.
pub fn fn_006499e0(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    array_value_below(e, this, index, 0xf, 0x74)
}

// Translated from 00649aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): for an index below 16 the value of the
/// array at `this + 0xDC` (`005dd4e0`), else 0.0, as a float in ST0.
pub fn fn_00649aa0(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    array_value_below(e, this, index, 0x10, 0xdc)
}

// Translated from 00649b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): for an index below 1 the value of the
/// array at `this + 0x110` (`005dd4e0`), else 0.0, as a float in ST0.
pub fn fn_00649b10(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    array_value_below(e, this, index, 1, 0x110)
}

// Translated from 00649680 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00649680 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// A constructor (unnamed in the engine map; `thiscall`, plain `RET`): the base
/// constructor (`00a7b440`) with [`fn_006499d0`]'s word, the virtual table `01064e1c`,
/// twelve value arrays (`0064eb60`, each with its two words) and three lists (`0048f200`)
/// from `+0x10` to `+0x110`, the point at `+0x124` (`006815c0`) and the bit set at `+0x1A0`
/// (`0096a2d0`); then the plain fields in the exe's order (the point twice set to the
/// three words at `011f426c`, floats 0.0, two flags 1, the rest 0). Answers `this`.
pub fn fn_00649680(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    let base_value = fn_006499d0(e);
    e.call(BASE_CONSTRUCTOR, &args![this, base_value]);
    e.mem.set_u32(object, CONSTRUCTED_VTABLE);
    let arrays: [(u32, Option<(u32, u32)>); 15] = [
        (0x10, Some((1, 0xf))),
        (0x24, Some((1, 0xf))),
        (0x38, Some((2, 0x11))),
        (0x4c, Some((0, 0x10))),
        (0x60, Some((3, 1))),
        (0x74, Some((1, 0xf))),
        (0x88, None),
        (0x94, Some((2, 0x11))),
        (0xa8, Some((2, 0x11))),
        (0xbc, None),
        (0xc8, Some((0, 0x10))),
        (0xdc, Some((0, 0x10))),
        (0xf0, None),
        (0xfc, Some((3, 1))),
        (0x110, Some((3, 1))),
    ];
    for (offset, words) in arrays {
        match words {
            Some((first, second)) => {
                e.call(
                    VALUE_ARRAY_CONSTRUCTOR,
                    &args![object + offset, first, second],
                );
            }
            None => {
                e.call(LIST_CONSTRUCTOR, &args![object + offset]);
            }
        }
    }
    e.call(POINT_CONSTRUCTOR, &args![object + 0x124]);
    e.call(BIT_SET_CONSTRUCTOR, &args![object + 0x1a0]);
    e.mem.set_u32(object + 0xc, 0);
    copy_default_point(e, object + 0x124);
    for offset in [0x18b, 0x188, 0x18c, 0x18e, 0x191] {
        e.mem.set_u8(object + offset, 0);
    }
    e.mem.set_u32(object + 0x194, x87_store32(0.0));
    e.mem.set_u32(object + 0x174, 0);
    for offset in [0x178, 0x17c, 0x140, 0x144] {
        e.mem.set_u32(object + offset, x87_store32(0.0));
    }
    e.mem.set_u32(object + 0x148, 0);
    e.mem.set_u8(object + 0x14c, 1);
    for offset in [0x138, 0x13c, 0x130, 0x134] {
        e.mem.set_u32(object + offset, x87_store32(0.0));
    }
    for offset in (0x150..=0x170).step_by(4) {
        e.mem.set_u32(object + offset, x87_store32(0.0));
    }
    e.mem.set_u8(object + 0x189, 1);
    e.mem.set_u8(object + 0x18d, 0);
    copy_default_point(e, object + 0x124);
    e.mem.set_u8(object + 0x18f, 0);
    e.mem.set_u8(object + 0x190, 0);
    this
}

/// The three words at `011f426c` copied to `target`.
fn copy_default_point(e: &mut Engine, target: u32) {
    for index in 0..3 {
        let word = e.global::<u32>(DEFAULT_POINT + 4 * index);
        e.mem.set_u32(target + 4 * index, word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_99c0, fn_006499c0(Ptr) -> u32),
        entry!(0x0064_9530, fn_00649530(Ptr) -> f32),
        entry!(0x0064_99d0, fn_006499d0() -> u32),
        entry!(0x0064_99e0, fn_006499e0(Ptr, i32) -> f32),
        entry!(0x0064_9aa0, fn_00649aa0(Ptr, i32) -> f32),
        entry!(0x0064_9b10, fn_00649b10(Ptr, i32) -> f32),
        entry!(0x0064_9680, fn_00649680(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_006499c0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0064_99c0, &args![0x1000u32]).u32(), 0x011d_57a4);
        assert_eq!(e.call(0x0064_99c0, &args![0u32]).u32(), 0x011d_57a4);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn scaled_value_combines_the_two_settings() {
        let mut e = Engine::new();
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        let base = e.mem.alloc(4);
        e.mem.set_u32(base, 2.0f32.to_bits());
        let factor = e.mem.alloc(4);
        e.mem.set_u32(factor, 8.0f32.to_bits());
        e.register_double(SETTING_VALUE, move |_, a| {
            if a[0] == SETTING_BASE {
                base.into_ret()
            } else {
                factor.into_ret()
            }
        });
        e.map(PERCENT_DIVISOR, 8);
        e.mem.set_u64(PERCENT_DIVISOR, 100.0f64.to_bits());
        e.call_log = Some(vec![]);
        let value = e.call(0x0064_9530, &args![0x4000u32]).f64();
        assert_eq!(value, 6.0);
        assert_eq!(calls(&e, GET_CLAMPED_ACTOR_VALUE), vec![vec![0x4000, 0x25]]);
        assert_eq!(
            calls(&e, SETTING_VALUE),
            vec![vec![SETTING_BASE], vec![SETTING_FACTOR]]
        );
    }

    #[test]
    fn global_getter_reads_its_word() {
        let mut e = Engine::new();
        e.map(CLASS_GLOBAL, 4);
        e.set_global(CLASS_GLOBAL, 0x1234u32);
        assert_eq!(e.call(0x0064_99d0, &args![]).u32(), 0x1234);
    }

    #[test]
    fn array_getters_check_their_limits() {
        let mut e = Engine::new();
        e.register(ARRAY_VALUE, |_, a| Ret {
            st0: f64::from(a[1]) + 0.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        for (address, limit, offset) in [
            (0x0064_99e0u32, 15i32, 0x74u32),
            (0x0064_9aa0, 16, 0xdc),
            (0x0064_9b10, 1, 0x110),
        ] {
            let below = (limit - 1) as u32;
            assert_eq!(
                e.call(address, &args![0x1000u32, below]).f64(),
                f64::from(below) + 0.5
            );
            assert_eq!(e.call(address, &args![0x1000u32, limit as u32]).f64(), 0.0);
            assert_eq!(
                calls(&e, ARRAY_VALUE).last().unwrap(),
                &vec![0x1000 + offset, below]
            );
        }
        // A negative index is below every limit.
        assert_eq!(
            e.call(0x0064_9b10, &args![0x1000u32, 0xffff_ffffu32]).f64(),
            4294967295.5f64 as f32 as f64
        );
        assert_eq!(calls(&e, ARRAY_VALUE).len(), 4);
    }

    #[test]
    fn constructor_builds_the_members_in_order() {
        let mut e = Engine::new();
        e.map(CLASS_GLOBAL, 4);
        e.set_global(CLASS_GLOBAL, 0x99u32);
        e.map(DEFAULT_POINT, 12);
        for index in 0..3 {
            e.set_global(DEFAULT_POINT + 4 * index, 0x70 + index);
        }
        for address in [
            BASE_CONSTRUCTOR,
            VALUE_ARRAY_CONSTRUCTOR,
            LIST_CONSTRUCTOR,
            POINT_CONSTRUCTOR,
            BIT_SET_CONSTRUCTOR,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x1b0);
        for offset in (0x124..0x1a0).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_9680, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![this, 0x99]]);
        assert_eq!(e.mem.u32(this), CONSTRUCTED_VTABLE);
        let arrays = calls(&e, VALUE_ARRAY_CONSTRUCTOR);
        assert_eq!(arrays.len(), 12);
        assert_eq!(arrays[0], vec![this + 0x10, 1, 0xf]);
        assert_eq!(arrays[4], vec![this + 0x60, 3, 1]);
        assert_eq!(arrays[11], vec![this + 0x110, 3, 1]);
        assert_eq!(
            calls(&e, LIST_CONSTRUCTOR),
            vec![vec![this + 0x88], vec![this + 0xbc], vec![this + 0xf0]]
        );
        assert_eq!(e.mem.u32(this + 0x124), 0x70);
        assert_eq!(e.mem.u32(this + 0x12c), 0x72);
        assert_eq!(e.mem.u8(this + 0x14c), 1);
        assert_eq!(e.mem.u8(this + 0x189), 1);
        for offset in [0x130, 0x148, 0x170, 0x174, 0x17c, 0x194] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        for offset in [0x188, 0x18b, 0x18f, 0x191] {
            assert_eq!(e.mem.u8(this + offset), 0, "{offset:#x}");
        }
        // Words the constructor does not write keep their value.
        assert_eq!(e.mem.u32(this + 0x180), 0xffff_ffff);
    }
}
