//! `fallout/misc/run_00867de0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two small `Calendar` methods that follow `calendar.cpp`'s functions in the
//! exe (`Calendar` is declared in `calendar.rs`; the engine map has no name
//! for either of these two).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::calendar::Calendar;

/// `TESGlobal` value getter (`float TESGlobal::fValue`, returned in ST0);
/// the same code as in `calendar.rs`.
const TES_GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;

// Translated from 00867de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The whole number of days passed: the days-passed global (`pGameDaysPassed`)
/// as a `float`, 1.0 when the calendar has no such global, truncated toward
/// zero (the code switches the x87 rounding mode to truncate around its
/// `FISTP` of a 64-bit integer) and returned as the low 32 bits. A value the
/// 64-bit integer cannot hold gives the processor's "integer indefinite", whose
/// low 32 bits are 0.
pub fn fn_00867de0(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    let days_passed_global = e.get(this, Calendar::pGameDaysPassed);
    let days_passed = if days_passed_global.is_null() {
        1.0f32
    } else {
        e.call(TES_GLOBAL_GET_VALUE, &args![days_passed_global])
            .f32()
    };
    if days_passed.is_finite() && days_passed.abs() < 9.223_372e18 {
        days_passed as i64 as u32
    } else {
        0
    }
}

// Translated from 00867ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The day of the week: the whole days passed (`00867de0`) modulo 7, as an
/// unsigned number.
pub fn fn_00867ef0(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    fn_00867de0(e, this) % 7
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00867de0, fn_00867de0(Ptr<Calendar>) -> u32),
        entry!(0x00867ef0, fn_00867ef0(Ptr<Calendar>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A calendar whose days-passed global (a `TESGlobal`-sized block with
    /// its value at +0x24) holds `value`; the getter is a test double.
    fn calendar_with_days_passed(e: &mut Engine, value: f32) -> Ptr<Calendar> {
        e.register(TES_GLOBAL_GET_VALUE, |e, a| {
            e.mem.f32(a[0] + 0x24).into_ret()
        });
        let calendar: Ptr<Calendar> = e.new_object();
        let global = e.mem.alloc(0x28);
        e.mem.set_f32(global + 0x24, value);
        e.set(calendar, Calendar::pGameDaysPassed, Ptr::new(global));
        calendar
    }

    #[test]
    fn whole_days_truncate_toward_zero() {
        let mut e = Engine::new();
        let calendar = calendar_with_days_passed(&mut e, 12.99);
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 12);
        let calendar = calendar_with_days_passed(&mut e, -2.5);
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), -2i32 as u32);
    }

    #[test]
    fn without_a_days_passed_global_the_count_is_one() {
        let mut e = Engine::new();
        let calendar: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 1);
        assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), 1);
    }

    #[test]
    fn values_the_integer_cannot_hold_give_zero() {
        let mut e = Engine::new();
        for value in [f32::NAN, f32::INFINITY, 1.0e30] {
            let calendar = calendar_with_days_passed(&mut e, value);
            assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 0);
        }
    }

    #[test]
    fn day_of_week_is_the_whole_days_modulo_seven() {
        let mut e = Engine::new();
        for (days, weekday) in [(0.0f32, 0u32), (6.9, 6), (7.0, 0), (100.5, 2)] {
            let calendar = calendar_with_days_passed(&mut e, days);
            assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), weekday);
        }
        // The count is unsigned in the modulo: -1 is 0xFFFFFFFF, which is 3 mod 7.
        let calendar = calendar_with_days_passed(&mut e, -1.0);
        assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), 3);
    }
}
