//! `Havok SDK/run_00da6e60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkAgent1nMachine_UpdateShapeCollectionFilter` (`00da48e0`, cdecl `(track, input)`), the
/// agent track's first-sector function `00da4430` (cdecl `(track)`) and the dispatcher's agent
/// registration `00cfbd60` (thiscall `(table, -1, 9)`).
const UPDATE_SHAPE_COLLECTION_FILTER: u32 = 0x00da_48e0;
const TRACK_FIRST_SECTOR: u32 = 0x00da_4430;
const REGISTER_AGENT: u32 = 0x00cf_bd60;

// Translated from 00da6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, seven stack words `(a0, track, a2, a3, a4, a5, a6)`: asks the object `[a3]` for the
/// value of its virtual slot `+0x10`, packs `a2`, `a3`, that value (at `+0xc`), `a4`, `a5` and
/// `a6` in a 0x1c-byte block on the stack (its word at `+8` is never written) and runs
/// `hkAgent1nMachine_UpdateShapeCollectionFilter(track, block)`. `a0` is not used. The exe's
/// name for it is unknown (a function of the list agent's table, see `00da6fd0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00da6e60(
    e: &mut Engine,
    _a0: u32,
    track: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) {
    let object = e.mem.u32(a3);
    let value = e.vcall(object, 0x10, &args![]).u32();
    e.with_stack(0x20, |e, block| {
        let block = block.addr();
        e.mem.set_u32(block, a2);
        e.mem.set_u32(block + 0xc, value);
        e.mem.set_u32(block + 0x10, a4);
        e.mem.set_u32(block + 0x14, a5);
        e.mem.set_u32(block + 4, a3);
        e.mem.set_u32(block + 0x18, a6);
        e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![track, block]);
    });
}

// Translated from 00da6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, three stack words `(a0, a1, track)` of which the first two are not used: a non-null
/// `track` (an `hkAgent1nMachine` track: sector array of data, size 0 and capacity word
/// `0x80000000`) is cleared, then it is given its first sector (`00da4430`; also called with a
/// null `track`, as the exe does). Answers `track + 0x10`. The exe's name for it is unknown (the
/// list agent's first table entry, see `00da6fd0`).
pub fn fn_00da6fa0(e: &mut Engine, _a0: u32, _a1: u32, track: u32) -> u32 {
    if track != 0 {
        e.mem.set_u32(track, 0);
        e.mem.set_u32(track + 4, 0);
        e.mem.set_u32(track + 8, 0x8000_0000);
    }
    e.call(TRACK_FIRST_SECTOR, &args![track]);
    track.wrapping_add(0x10)
}

// Translated from 00da6fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the 0x34-byte agent function table `table` (cdecl `(table)`): the function `00da6fa0`
/// at `+0`, the code at `00da6dc0` at `+4`, zeros at `+8` to `+0x14`, `00da6e60` at `+0x18`,
/// `00da6de0` at `+0x1c` (the engine map names it `hkListAgent3::calcStatistics`), `00da6e10`
/// at `+0x20`, `00da6e30` at `+0x24`, 0 at `+0x28`, `00da6eb0` at `+0x2c` and the byte 1 at
/// `+0x30`. The exe's name for it is unknown (the list agent's table).
pub fn fn_00da6fd0(e: &mut Engine, table: u32) {
    e.mem.set_u32(table, 0x00da_6fa0);
    e.mem.set_u32(table + 0x2c, 0x00da_6eb0);
    e.mem.set_u32(table + 0x28, 0);
    for offset in [0x8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(table + offset, 0);
    }
    e.mem.set_u32(table + 0x18, 0x00da_6e60);
    e.mem.set_u32(table + 0x1c, 0x00da_6de0);
    e.mem.set_u32(table + 0x20, 0x00da_6e10);
    e.mem.set_u32(table + 0x24, 0x00da_6e30);
    e.mem.set_u32(table + 4, 0x00da_6dc0);
    e.mem.set_u8(table + 0x30, 1);
}

// Translated from 00da7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the list agent with the dispatcher (cdecl, one word: the dispatcher): builds the
/// function table on the stack (`00da6fd0` after clearing the bytes `+0x30` to `+0x32` and the
/// words `+0x18` to `+0x28`) and calls `00cfbd60` on the dispatcher with the table, -1 and 9.
/// The exe's name for it is unknown.
pub fn fn_00da7020(e: &mut Engine, dispatcher: u32) {
    e.with_stack(0x40, |e, table| {
        let table = table.addr();
        for offset in [0x30, 0x31, 0x32] {
            e.mem.set_u8(table + offset, 0);
        }
        for offset in [0x18, 0x1c, 0x20, 0x24, 0x28] {
            e.mem.set_u32(table + offset, 0);
        }
        fn_00da6fd0(e, table);
        e.call(REGISTER_AGENT, &args![dispatcher, table, u32::MAX, 9u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_6e60, fn_00da6e60(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00da_6fa0, fn_00da6fa0(u32, u32, u32) -> u32),
        entry!(0x00da_6fd0, fn_00da6fd0(u32)),
        entry!(0x00da_7020, fn_00da7020(u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const VALUE_SLOT: u32 = 0x0900_0010;

    #[test]
    fn the_block_is_packed_and_handed_to_the_filter_update() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, VALUE_SLOT);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, object);
        e.register(VALUE_SLOT, |_, a| a[0].into_ret());
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(UPDATE_SHAPE_COLLECTION_FILTER, move |e, a| {
            let block: Vec<u32> = [0u32, 4, 0xc, 0x10, 0x14, 0x18]
                .iter()
                .map(|offset| e.mem.u32(a[1] + offset))
                .collect();
            log.borrow_mut().push((a[0], block));
            Ret::default()
        });
        e.call(
            0x00da_6e60,
            &args![1u32, 0x7000u32, 0x22u32, cell, 0x44u32, 0x55u32, 0x66u32],
        );
        assert_eq!(
            *seen.borrow(),
            vec![(0x7000, vec![0x22, cell, object, 0x44, 0x55, 0x66])]
        );
    }

    #[test]
    fn the_track_is_cleared_and_given_its_first_sector() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TRACK_FIRST_SECTOR, |_, _| Ret::default());
        let track = e.mem.alloc(0x20);
        for offset in [0, 4, 8, 0xc] {
            e.mem.set_u32(track + offset, 0x1111);
        }
        let result = e.call(0x00da_6fa0, &args![5u32, 6u32, track]).u32();
        assert_eq!(result, track + 0x10);
        assert_eq!(e.mem.u32(track), 0);
        assert_eq!(e.mem.u32(track + 4), 0);
        assert_eq!(e.mem.u32(track + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(track + 0xc), 0x1111);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(TRACK_FIRST_SECTOR, vec![track])));
    }

    #[test]
    fn a_null_track_still_runs_the_first_sector_function() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(TRACK_FIRST_SECTOR, |_, _| Ret::default());
        let result = e.call(0x00da_6fa0, &args![0u32, 0u32, 0u32]).u32();
        assert_eq!(result, 0x10);
        let log = e.call_log.as_ref().unwrap();
        assert!(log.contains(&(TRACK_FIRST_SECTOR, vec![0])));
    }

    #[test]
    fn the_function_table_is_filled() {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        for word in 0..0x10u32 {
            e.mem.set_u32(table + 4 * word, 0xdead_beef);
        }
        e.call(0x00da_6fd0, &args![table]);
        let words: Vec<u32> = (0..0xc).map(|i| e.mem.u32(table + 4 * i)).collect();
        assert_eq!(
            words,
            [
                0x00da_6fa0,
                0x00da_6dc0,
                0,
                0,
                0,
                0,
                0x00da_6e60,
                0x00da_6de0,
                0x00da_6e10,
                0x00da_6e30,
                0,
                0x00da_6eb0,
            ]
        );
        assert_eq!(e.mem.u8(table + 0x30), 1);
        // Only the byte at +0x30 is written there; the rest of that word is untouched.
        assert_eq!(e.mem.u32(table + 0x30), 0xdead_be01);
        assert_eq!(e.mem.u32(table + 0x34), 0xdead_beef);
    }

    #[test]
    fn the_agent_is_registered_with_the_table_minus_one_and_nine() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(REGISTER_AGENT, move |e, a| {
            let words: Vec<u32> = (0..0xc).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut()
                .push((a[0], a[2], a[3], words, e.mem.u8(a[1] + 0x30)));
            Ret::default()
        });
        e.call(0x00da_7020, &args![0x9000u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0].0, seen[0].1, seen[0].2), (0x9000, u32::MAX, 9));
        assert_eq!(seen[0].3[0], 0x00da_6fa0);
        assert_eq!(seen[0].3[6], 0x00da_6e60);
        assert_eq!(seen[0].4, 1);
    }
}
