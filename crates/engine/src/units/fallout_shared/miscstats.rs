//! `fallout shared/miscstats.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004d5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004d5c20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004d5dd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004d_5c20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The table of 0x2b `MiscStat*` pointers (`.data`); its first slot is the "built" test.
const MISC_STAT_TABLE: u32 = 0x011c_6d50;
/// The table of the 0x2b name pointers handed to the constructor (`.data`).
const MISC_STAT_NAMES: u32 = 0x0118_9280;
/// Number of entries in both tables.
const MISC_STAT_COUNT: u32 = 0x2b;
/// `operator new` (`00401000`, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `MiscStat::MiscStat` (Xbox PDB, `004d5b60`): `this` plus name, index and a third word
/// (`RET 0xc`).
const MISC_STAT_CONSTRUCTOR: u32 = 0x004d_5b60;

// Translated from 004d5c80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d5c80 - the compiler's exception frame (FS:[0] chain, state
// bookkeeping around the allocation) is not translated.
/// Unnamed in the engine map (unit `miscstats.cpp`): builds the table of `MiscStat` objects
/// once. If the first table slot is empty, for each of the 0x2b indices it allocates 8 bytes
/// and, when the allocation succeeded, constructs a `MiscStat` in it with the name pointer
/// at `MISC_STAT_NAMES[index]`, the index and 0 (a failed allocation stores null), and stores
/// the result in the table. Returns `this`, which it does not otherwise read.
pub fn fn_004d5c80(e: &mut Engine, this: Ptr) -> Ptr {
    if e.global::<u32>(MISC_STAT_TABLE) != 0 {
        return this;
    }
    for index in 0..MISC_STAT_COUNT {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let stat = if block != 0 {
            let name = e.global::<u32>(MISC_STAT_NAMES + index * 4);
            e.call(MISC_STAT_CONSTRUCTOR, &args![block, name, index, 0u32])
                .u32()
        } else {
            0
        };
        e.set_global(MISC_STAT_TABLE + index * 4, stat);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004d_5dd0, fn_004d5dd0(Ptr, u32) -> Ptr),
        entry!(0x004d_5c80, fn_004d5c80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004d5dd0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004d_5c20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004d_5dd0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004d_5dd0)
                .collect();
            let mut expected = vec![(0x004d_5c20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn tables(e: &mut Engine) {
        e.map(MISC_STAT_TABLE & !0xfff, 0x1000);
        e.map(MISC_STAT_NAMES & !0xfff, 0x1000);
        for i in 0..MISC_STAT_COUNT {
            e.set_global(MISC_STAT_NAMES + i * 4, 0x5000 + i);
        }
    }

    #[test]
    fn builds_every_entry_when_the_first_slot_is_empty() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OPERATOR_NEW, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        e.register(MISC_STAT_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5c80, &args![0x1234u32]).u32(), 0x1234);
        let built = calls_to(&mut e, MISC_STAT_CONSTRUCTOR);
        assert_eq!(built.len(), 0x2b);
        for (index, words) in built.iter().enumerate() {
            assert_eq!(words[1], 0x5000 + index as u32);
            assert_eq!(words[2], index as u32);
            assert_eq!(words[3], 0);
            assert_eq!(
                e.global::<u32>(MISC_STAT_TABLE + index as u32 * 4),
                words[0]
            );
        }
        assert_eq!(calls_to(&mut e, OPERATOR_NEW).len(), 0x2b);
        assert_eq!(calls_to(&mut e, OPERATOR_NEW)[0], vec![8]);
    }

    #[test]
    fn stores_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(MISC_STAT_CONSTRUCTOR, |_, _| panic!("not constructed"));
        e.set_global(MISC_STAT_TABLE + 4, 0x99u32);
        e.call(0x004d_5c80, &args![0u32]);
        for i in 0..MISC_STAT_COUNT {
            assert_eq!(e.global::<u32>(MISC_STAT_TABLE + i * 4), 0);
        }
    }

    #[test]
    fn does_nothing_when_the_first_slot_is_filled() {
        let mut e = Engine::new();
        tables(&mut e);
        e.set_global(MISC_STAT_TABLE, 0x77u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004d_5c80, &args![0x42u32]).u32(), 0x42);
        assert_eq!(
            e.call_log.take().unwrap().len(),
            1,
            "only the top-level call"
        );
        assert_eq!(e.global::<u32>(MISC_STAT_TABLE), 0x77);
        assert_eq!(e.global::<u32>(MISC_STAT_TABLE + 4), 0);
    }
}
