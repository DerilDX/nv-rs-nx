//! `fallout shared/run_004b0cf0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004b0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004b0de0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004b0f40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004b_0de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The singleton's pointer (`004b0e20` makes it on demand); its class is not named in the
/// engine map.
const SINGLETON_011C5F40: u32 = 0x011c_5f40;
/// The table of the twelve months' day counts (`u16` each).
const DAYS_PER_MONTH_01188ECC: u32 = 0x0118_8ecc;

// Translated from 004b0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b0cf0 - the C++ exception-unwinding frame (FS:[0]) that destroys the loop's temporary on a throw is not translated
/// Destructor body of an object (the engine map gives it no name) with a word at `+0` and a list at
/// `+4`: clears `+0`, then, while the list is not empty (`004a4460` on `this + 4` returns false),
/// builds a four-byte temporary on the stack (`00633c90(temp, 0)`), hands it to `004b0f70(this +
/// 4, temp)`, destroys it (`0045cec0`) and removes the list's head (`004ee8a0`); finally runs
/// `004ee840` on the list.
pub fn fn_004b0cf0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    let list = this.addr().wrapping_add(4);
    loop {
        if e.call(0x004a_4460, &args![list]).bool() {
            break;
        }
        e.with_stack(4, |e, temp| {
            e.call(0x0063_3c90, &args![temp, 0u32]);
            e.call(0x004b_0f70, &args![list, temp]);
            e.call(0x0045_cec0, &args![temp]);
        });
        e.call(0x004e_e8a0, &args![list]);
    }
    e.call(0x004e_e840, &args![list]);
}

// Translated from 004b0da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the engine map gives it no name) of a three-word handle: stores `first` at `+0`,
/// `second` at `+4` and 0 at `+8` (the count `004b0ef0` decrements), runs `004b0eb0` of this unit
/// on it and returns `this`. `RET 8`.
pub fn fn_004b0da0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), first);
    e.mem.set_u32(this.addr().wrapping_add(4), second);
    e.mem.set_u32(this.addr().wrapping_add(8), 0);
    fn_004b0eb0(e, this);
    this
}

// Translated from 004b0de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the handle `004b0da0` builds: when both words (`+0`, `+4`) are non-zero,
/// gets the singleton (`004b0e20(1)` of this unit, which makes it if needed) and calls
/// `004b0aa0(singleton, word0, word1)` on it.
pub fn fn_004b0de0(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr());
    if first == 0 {
        return;
    }
    let second = e.mem.u32(this.addr().wrapping_add(4));
    if second == 0 {
        return;
    }
    let singleton = fn_004b0e20(e, 1);
    e.call(0x004b_0aa0, &args![singleton, first, second]);
}

// Translated from 004b0e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004b0e20 - the C++ exception-unwinding frame (FS:[0]) that frees the block when the constructor throws is not translated
/// Singleton getter (`cdecl`, one word; the engine map gives it no name): when the pointer at
/// `011c5f40` is null and the low byte of `create` is non-zero, takes 8 bytes from `00401000`
/// and, when that succeeded, constructs them with `004b0880` (its result is stored; 0 when the
/// allocation failed). Returns the pointer at `011c5f40`.
pub fn fn_004b0e20(e: &mut Engine, create: u32) -> u32 {
    if e.mem.u32(SINGLETON_011C5F40) == 0 && create as u8 != 0 {
        let block = e.call(0x0040_1000, &args![8u32]).u32();
        let object = if block != 0 {
            e.call(0x004b_0880, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(SINGLETON_011C5F40, object);
    }
    e.mem.u32(SINGLETON_011C5F40)
}

// Translated from 004b0eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The handle's acquire step (the engine map gives it no name): when both words (`+0`, `+4`) are
/// non-zero, gets the singleton (`004b0e20(1)` of this unit) and calls `004b0910(singleton,
/// word0, word1)` on it (the engine map names `004b0910` `BSClonePools::MakeClones`).
pub fn fn_004b0eb0(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr());
    if first == 0 {
        return;
    }
    let second = e.mem.u32(this.addr().wrapping_add(4));
    if second == 0 {
        return;
    }
    let singleton = fn_004b0e20(e, 1);
    e.call(0x004b_0910, &args![singleton, first, second]);
}

// Translated from 004b0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Release of the handle (the engine map gives it no name): decrements the count at `+8`
/// through `004019a0` (`InterlockedDecrement`, import `00fdf1c4`) and, when it reaches 0 and
/// `this` is not null, deletes the handle through `004b0f40(this, 1)` of this unit.
pub fn fn_004b0ef0(e: &mut Engine, this: Ptr) {
    let count = this.addr().wrapping_add(8);
    if e.call(0x0040_19a0, &args![count]).u32() == 0 && !this.is_null() {
        fn_004b0f40(e, this, 1);
    }
}

// Translated from 004b0fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the engine map gives it no name) of a two-word date: stores at `+0` the day of
/// the year `004b10f0(day, month)` of this unit gives for the low words of `day` and `month`,
/// and the low word of `third` at `+2`. Returns `this`. `RET 0xc`.
pub fn fn_004b0fc0(e: &mut Engine, this: Ptr, day: u32, month: u32, third: u32) -> Ptr {
    let day_of_year = fn_004b10f0(e, day as u16, month as u16);
    e.mem.set_u16(this.addr(), day_of_year);
    e.mem.set_u16(this.addr().wrapping_add(2), third as u16);
    this
}

// Translated from 004b10d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of days of month `month` (0 = January) from the `u16` table at `01188ecc`, or 0
/// when `month >= 12` (a signed compare: a negative month reads before the table). The result is
/// the table's word in `AX`; the upper half of `EAX` keeps the upper half of `month`, as the exe
/// leaves it (0 is `XOR EAX,EAX`).
pub fn fn_004b10d0(e: &mut Engine, month: i32) -> u32 {
    if month >= 12 {
        return 0;
    }
    let days = e
        .mem
        .u16(DAYS_PER_MONTH_01188ECC.wrapping_add((month as u32).wrapping_mul(2)));
    (month as u32 & 0xffff_0000) | u32::from(days)
}

// Translated from 004b10f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The day of the year (`cdecl`, two words, the low words read; the engine map gives it no name):
/// a month above 12 becomes 11 (12 itself stays and reads the word after the table), a day at or
/// past the month's count from the table at `01188ecc` becomes the count minus 1, and the counts
/// of the months before it are added to the day (in `u16` arithmetic).
pub fn fn_004b10f0(e: &mut Engine, day: u16, month: u16) -> u16 {
    let mut month = month;
    let mut day = day;
    if month > 12 {
        month = 11;
    }
    let count = |e: &Engine, index: u32| e.mem.u16(DAYS_PER_MONTH_01188ECC + index * 2);
    let month_days = count(e, u32::from(month));
    if day >= month_days {
        day = month_days.wrapping_sub(1);
    }
    let mut result = day;
    for index in 0..u32::from(month) {
        result = result.wrapping_add(count(e, index));
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_0f40, fn_004b0f40(Ptr, u32) -> Ptr),
        entry!(0x004b_0cf0, fn_004b0cf0(Ptr)),
        entry!(0x004b_0da0, fn_004b0da0(Ptr, u32, u32) -> Ptr),
        entry!(0x004b_0de0, fn_004b0de0(Ptr)),
        entry!(0x004b_0e20, fn_004b0e20(u32) -> u32),
        entry!(0x004b_0eb0, fn_004b0eb0(Ptr)),
        entry!(0x004b_0ef0, fn_004b0ef0(Ptr)),
        entry!(0x004b_0fc0, fn_004b0fc0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x004b_10d0, fn_004b10d0(i32) -> u32),
        entry!(0x004b_10f0, fn_004b10f0(u16, u16) -> u16),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004b0f40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004b_0de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004b_0f40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004b_0f40)
                .collect();
            let mut expected = vec![(0x004b_0de0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    const DAYS: [u16; 13] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 0x7777];

    fn month_table(e: &mut Engine) {
        e.map(DAYS_PER_MONTH_01188ECC & !0xfff, 0x2000);
        for (index, days) in DAYS.iter().enumerate() {
            e.mem
                .set_u16(DAYS_PER_MONTH_01188ECC + index as u32 * 2, *days);
        }
    }

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_004b0cf0_drains_the_list_then_finishes_it() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x55);
        // The test list keeps its remaining count in the word after the list (`this + 8`).
        e.mem.set_u32(this + 8, 2);
        e.register(0x004a_4460, |e, a| (e.mem.u32(a[0] + 4) == 0).into_ret());
        e.register(0x004e_e8a0, |e, a| {
            let left = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, left - 1);
            Ret::default()
        });
        for address in [0x0063_3c90, 0x004b_0f70, 0x0045_cec0, 0x004e_e840] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x004b_0cf0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), 0);
        let made = calls(&log, 0x0063_3c90);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1], 0);
        let temp = made[0][0];
        assert_eq!(calls(&log, 0x004b_0f70), vec![vec![this + 4, temp]; 2]);
        assert_eq!(calls(&log, 0x0045_cec0), vec![vec![temp]; 2]);
        assert_eq!(calls(&log, 0x004e_e8a0).len(), 2);
        assert_eq!(calls(&log, 0x004e_e840), vec![vec![this + 4]]);
    }

    #[test]
    fn fn_004b0da0_stores_the_words_and_acquires() {
        let mut e = Engine::new();
        e.map(SINGLETON_011C5F40 & !0xfff, 0x1000);
        e.mem.set_u32(SINGLETON_011C5F40, 0x4000);
        e.register(0x004b_0910, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 9);
        e.call_log = Some(vec![]);
        let result = e.call(0x004b_0da0, &args![this, 0x11u32, 0x22u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(
            (e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)),
            (0x11, 0x22, 0)
        );
        assert_eq!(calls(&log, 0x004b_0910), vec![vec![0x4000, 0x11, 0x22]]);
    }

    #[test]
    fn fn_004b0de0_and_fn_004b0eb0_need_both_words() {
        let mut e = Engine::new();
        e.map(SINGLETON_011C5F40 & !0xfff, 0x1000);
        e.mem.set_u32(SINGLETON_011C5F40, 0x4000);
        e.register(0x004b_0910, |_, _| Ret::default());
        e.register(0x004b_0aa0, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        for (first, second, runs) in [(0u32, 2u32, false), (1, 0, false), (1, 2, true)] {
            e.mem.set_u32(this, first);
            e.mem.set_u32(this + 4, second);
            e.call_log = Some(vec![]);
            e.call(0x004b_0de0, &args![this]);
            e.call(0x004b_0eb0, &args![this]);
            let log = e.call_log.take().unwrap();
            let expected = if runs {
                vec![vec![0x4000, first, second]]
            } else {
                vec![]
            };
            assert_eq!(calls(&log, 0x004b_0aa0), expected);
            assert_eq!(calls(&log, 0x004b_0910), expected);
        }
    }

    #[test]
    fn fn_004b0e20_makes_the_singleton_once_when_asked() {
        let mut e = Engine::new();
        e.map(SINGLETON_011C5F40 & !0xfff, 0x1000);
        e.register(0x0040_1000, |_, _| 0x5000u32.into_ret());
        e.register(0x004b_0880, |_, a| (a[0] + 4).into_ret());
        // Without the create byte (the upper bytes are not read) nothing is made.
        assert_eq!(e.call(0x004b_0e20, &args![0x100u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_0e20, &args![1u32]).u32(), 0x5004);
        assert_eq!(e.call(0x004b_0e20, &args![1u32]).u32(), 0x5004);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0040_1000), vec![vec![8]]);
        assert_eq!(calls(&log, 0x004b_0880), vec![vec![0x5000]]);
        // A failed allocation stores 0.
        e.mem.set_u32(SINGLETON_011C5F40, 0);
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x004b_0e20, &args![1u32]).u32(), 0);
    }

    #[test]
    fn fn_004b0ef0_deletes_when_the_count_reaches_zero() {
        let mut e = Engine::new();
        e.register(0x004b_0de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.register(0x0040_19a0, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x004b_0ef0, &args![this]);
        assert!(calls(e.call_log.as_ref().unwrap(), 0x0040_1030).is_empty());
        e.call(0x004b_0ef0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0040_19a0), vec![vec![this + 8]; 2]);
        assert_eq!(calls(&log, 0x004b_0de0), vec![vec![this]]);
        assert_eq!(calls(&log, 0x0040_1030), vec![vec![this]]);
    }

    #[test]
    fn fn_004b0fc0_stores_the_day_of_year_and_the_third_word() {
        let mut e = Engine::new();
        month_table(&mut e);
        let this = e.mem.alloc(4);
        let result = e
            .call(
                0x004b_0fc0,
                &args![this, 0xffff_0004u32, 2u32, 0x1234_5678u32],
            )
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this), 31 + 28 + 4);
        assert_eq!(e.mem.u16(this + 2), 0x5678);
    }

    #[test]
    fn fn_004b10d0_reads_the_table_below_twelve() {
        let mut e = Engine::new();
        month_table(&mut e);
        assert_eq!(e.call(0x004b_10d0, &args![1i32]).u32(), 28);
        assert_eq!(e.call(0x004b_10d0, &args![11i32]).u32(), 31);
        assert_eq!(e.call(0x004b_10d0, &args![12i32]).u32(), 0);
        // A negative month reads the word before the table; EAX keeps the upper half.
        e.mem.set_u16(DAYS_PER_MONTH_01188ECC - 2, 0x0102);
        assert_eq!(e.call(0x004b_10d0, &args![-1i32]).u32(), 0xffff_0102);
    }

    #[test]
    fn fn_004b10f0_clamps_month_and_day() {
        let mut e = Engine::new();
        month_table(&mut e);
        assert_eq!(e.call(0x004b_10f0, &args![5u32, 0u32]).u16(), 5);
        assert_eq!(e.call(0x004b_10f0, &args![40u32, 1u32]).u16(), 31 + 27);
        // A month above 12 becomes 11.
        assert_eq!(e.call(0x004b_10f0, &args![0u32, 13u32]).u16(), 334);
        // Month 12 reads the word after the table.
        assert_eq!(e.call(0x004b_10f0, &args![3u32, 12u32]).u16(), 365 + 3);
        assert_eq!(
            e.call(0x004b_10f0, &args![0x8000u32, 12u32]).u16(),
            365 + 0x7776
        );
    }
}
