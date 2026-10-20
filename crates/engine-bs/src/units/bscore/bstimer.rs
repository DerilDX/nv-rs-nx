//! `BSCore/bstimer.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Sleep` (KERNEL32 import slot).
const SLEEP_IMPORT: u32 = 0x00fd_f104;
/// `0.0` (`double`), the value `fClamp` is compared with.
const ZERO: u32 = 0x0101_2060;
/// `0.001` (`double`): milliseconds to seconds.
const MILLISECONDS_TO_SECONDS: u32 = 0x0101_6978;
/// `BSTimer::fGlobalTimeMultiplier` (Xbox PDB static `float`).
const GLOBAL_TIME_MULTIPLIER: u32 = 0x011a_c3a0;

layout! {
    /// `BSTimer` (Xbox PDB), 0x20 bytes.
    pub struct BSTimer: 0x20 {
        /// `cDisableCounter` (Xbox PDB): non-zero while the timer is disabled.
        0x00 cDisableCounter: u8,
        /// `fClamp` (Xbox PDB): fixed step in milliseconds, 0 for real time.
        0x04 fClamp: f32,
        /// `fClampRemainder` (Xbox PDB).
        0x08 fClampRemainder: f32,
        /// `fDelta` (Xbox PDB): seconds of the last step, times the global multiplier.
        0x0C fDelta: f32,
        /// `fDeltaUnmodified` (Xbox PDB): seconds of the last step.
        0x10 fDeltaUnmodified: f32,
        /// `iLastTime` (Xbox PDB).
        0x14 iLastTime: u32,
        /// `iFirstTime` (Xbox PDB).
        0x18 iFirstTime: u32,
        /// `bZeroDelta` (Xbox PDB): set when no time passed since the last step.
        0x1D bZeroDelta: bool,
    }
}

/// `FISTP qword` with the control word set to truncate (`OR 0xC00`): the
/// integer indefinite `i64::MIN` for NaN and values out of range.
fn truncate_to_i64(value: f64) -> i64 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        i64::MIN
    } else {
        value as i64
    }
}

// Translated from 00aa4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTimer` step (the engine map gives no name; the body is the timer's
/// per-frame update, `now` being the current millisecond tick): computes
/// `fDelta` from the time since `iLastTime`.
///
/// Disabled timer: `fDelta` is 0. With `fClamp` 0 (real time) the elapsed
/// milliseconds (1 and `bZeroDelta` set when none passed, otherwise at most
/// 166) are lifted to at least 10 by sleeping the difference. With a
/// non-zero `fClamp` the clamp is added to the remainder, whole milliseconds
/// move `iLastTime` and the fraction stays in `fClampRemainder`. The step in
/// seconds goes to `fDeltaUnmodified`, and `fDelta` is that times
/// `fGlobalTimeMultiplier`.
pub fn fn_00aa4ee0(e: &mut Engine, this: Ptr<BSTimer>, now: u32) {
    if e.get(this, BSTimer::cDisableCounter) != 0 {
        e.set(this, BSTimer::fDelta, 0.0);
        return;
    }
    let milliseconds_to_seconds: f64 = e.global(MILLISECONDS_TO_SECONDS);
    let mut time = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    let clamp = e.get(this, BSTimer::fClamp);
    let zero: f64 = e.global(ZERO);
    if f64::from(clamp) == zero {
        let mut elapsed = time.wrapping_sub(e.get(this, BSTimer::iLastTime));
        if elapsed == 0 {
            e.set(this, BSTimer::bZeroDelta, true);
            elapsed = 1;
        } else {
            e.set(this, BSTimer::bZeroDelta, false);
            if elapsed > 0xa6 {
                elapsed = 0xa6;
            }
        }
        if elapsed < 10 {
            e.call(SLEEP_IMPORT, &args![10 - elapsed]);
            elapsed = 10;
        }
        let seconds = x87_mul(f64::from(elapsed), milliseconds_to_seconds);
        e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(seconds)));
    } else {
        let remainder = e.get(this, BSTimer::fClampRemainder);
        let sum = x87_add(x87_load32(remainder.to_bits()), x87_m32(clamp.to_bits()));
        e.set(
            this,
            BSTimer::fClampRemainder,
            f32::from_bits(x87_store32(sum)),
        );
        let remainder = e.get(this, BSTimer::fClampRemainder);
        // `FISTP qword` truncating; `FILD` of the low word alone (the high
        // word is stored as 0), then `FSUBR`.
        let whole = truncate_to_i64(x87_load32(remainder.to_bits())) as u32;
        let rest = x87_sub(
            x87_m32(e.get(this, BSTimer::fClampRemainder).to_bits()),
            f64::from(whole),
        );
        e.set(
            this,
            BSTimer::fClampRemainder,
            f32::from_bits(x87_store32(rest)),
        );
        time = e.get(this, BSTimer::iLastTime).wrapping_add(whole);
        let seconds = x87_mul(x87_load32(clamp.to_bits()), milliseconds_to_seconds);
        e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(seconds)));
    }
    e.set(this, BSTimer::iLastTime, time);
    let delta = e.get(this, BSTimer::fDelta);
    e.set(
        this,
        BSTimer::fDeltaUnmodified,
        f32::from_bits(x87_quiet_bits(delta.to_bits())),
    );
    let multiplier: u32 = e.global(GLOBAL_TIME_MULTIPLIER);
    let scaled = x87_mul(x87_load32(delta.to_bits()), x87_m32(multiplier));
    e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(scaled)));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa4ee0, fn_00aa4ee0(Ptr<BSTimer>, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timer_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011a_c000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
        e.set_global(GLOBAL_TIME_MULTIPLIER, 2.0f32);
        e
    }

    fn sleeps(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = log.clone();
        e.register_double(SLEEP_IMPORT, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        log
    }

    #[test]
    fn a_disabled_timer_has_zero_delta() {
        let mut e = timer_engine();
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::cDisableCounter, 1);
        e.set(timer, BSTimer::fDelta, 5.0);
        e.set(timer, BSTimer::iLastTime, 77);
        e.call(0x00aa_4ee0, &args![timer, 1000u32]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.0);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 77);
    }

    #[test]
    fn real_time_with_no_elapsed_time_sleeps_ten_milliseconds() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iFirstTime, 100);
        e.set(timer, BSTimer::iLastTime, 50);
        e.call(0x00aa_4ee0, &args![timer, 150u32]);
        assert!(e.get(timer, BSTimer::bZeroDelta));
        // One millisecond elapsed is lifted to ten by sleeping nine.
        assert_eq!(*log.borrow(), vec![9]);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 50);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.01);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.02);
    }

    #[test]
    fn real_time_caps_a_long_step_and_does_not_sleep() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::bZeroDelta, true);
        e.call(0x00aa_4ee0, &args![timer, 5000u32]);
        assert!(!e.get(timer, BSTimer::bZeroDelta));
        assert!(log.borrow().is_empty());
        assert_eq!(e.get(timer, BSTimer::iLastTime), 5000);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.166);
    }

    #[test]
    fn real_time_keeps_a_normal_step() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iLastTime, 1000);
        e.call(0x00aa_4ee0, &args![timer, 1020u32]);
        assert!(log.borrow().is_empty());
        assert!(!e.get(timer, BSTimer::bZeroDelta));
        assert_eq!(e.get(timer, BSTimer::iLastTime), 1020);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.04);
    }

    #[test]
    fn a_fixed_step_carries_the_fraction() {
        let mut e = timer_engine();
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::fClamp, 16.5);
        e.set(timer, BSTimer::fClampRemainder, 0.75);
        e.set(timer, BSTimer::iLastTime, 1000);
        e.call(0x00aa_4ee0, &args![timer, 9999u32]);
        // 0.75 + 16.5 = 17.25: 17 whole milliseconds, 0.25 left.
        assert_eq!(e.get(timer, BSTimer::iLastTime), 1017);
        assert_eq!(e.get(timer, BSTimer::fClampRemainder), 0.25);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.0165);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.033);
    }
}
