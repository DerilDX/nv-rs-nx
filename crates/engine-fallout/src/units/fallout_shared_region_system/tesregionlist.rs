//! `fallout shared/region system/tesregionlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0096a2d0` (`ECX` = list): clears the list's two words (the constructor
/// of the `BSSimpleList<TESRegion *>` base at `+4`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TESRegionList`'s vtable (RTTI `.?AVTESRegionList@@`).
const VTABLE: u32 = 0x0102_40a0;
/// How many lists of region data the constructor discards, once, when the
/// first region list that owns its memory is made (9 pairs of pointers).
const DISCARDED_PAIRS: u32 = 9;
/// The table of those pointers: pair `i` is at `011c7f80 + i * 8`, its two
/// words at `+0` and `+4`.
const POINTER_PAIRS: u32 = 0x011c_7f80;
/// The count of region lists made so far that own their memory.
const OWNING_LISTS: u32 = 0x011c_7fc8;

layout! {
    /// `TESRegionList` (Xbox PDB), 0x10 bytes.
    pub struct TESRegionList: 0x10 {
        /// `bOwnsRegionMemory` (Xbox PDB).
        0x0c bOwnsRegionMemory: u8,
    }
}

/// Deletes the object whose address is in the table slot at `slot`, if any:
/// the slot is cleared, then the object's scalar deleting destructor (vtable
/// slot 0, argument 1) is called.
fn discard(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        e.mem.set_u32(slot, 0);
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 004f6320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f6320 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESRegionList::TESRegionList` (Xbox PDB; `thiscall`, `RET 4`): runs the
/// list constructor on the base at `+4`, stores the vtable and
/// `bOwnsRegionMemory` (`owns_region_memory`, a byte). A list that owns its
/// region memory, when none has been made before (the count at `011c7fc8` is
/// 0), first discards the nine pairs of objects in the table at `011c7f80`
/// (each non-null one is cleared in the table and deleted through its
/// virtual slot 0 with argument 1); it then counts itself in. Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn tes_region_list_tes_region_list(
    e: &mut Engine,
    this: Ptr<TESRegionList>,
    owns_region_memory: u8,
) -> Ptr<TESRegionList> {
    e.call(LIST_CONSTRUCT, &args![this.byte_add(4)]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, TESRegionList::bOwnsRegionMemory, owns_region_memory);
    if e.get(this, TESRegionList::bOwnsRegionMemory) != 0 {
        if e.global::<u32>(OWNING_LISTS) == 0 {
            for pair in 0..DISCARDED_PAIRS {
                discard(e, POINTER_PAIRS + pair * 8);
                discard(e, POINTER_PAIRS + pair * 8 + 4);
            }
        }
        let count: u32 = e.global(OWNING_LISTS);
        e.set_global(OWNING_LISTS, count.wrapping_add(1));
    }
    this
}

// Translated from 004f6480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionList` scalar deleting destructor (engine map name): runs the destructor body `004f64b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_list_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_64b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f6320,
            tes_region_list_tes_region_list(Ptr<TESRegionList>, u8) -> Ptr<TESRegionList>
        ),
        entry!(0x004f_6480, tes_region_list_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(POINTER_PAIRS, 0x80);
        e.map(OWNING_LISTS, 4);
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn a_list_that_does_not_own_its_memory_only_stores_its_fields() {
        let mut e = engine();
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f6320, &args![this, 0u32]).u32(), this);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u8(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 8), 0xcdcd_cdcd);
        assert_eq!(e.mem.bytes(this + 0xd, 3), [0xcd; 3]);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 0);
        assert_eq!(e.mem.u32(POINTER_PAIRS), 0x0800_0000);
    }

    #[test]
    fn the_first_owning_list_deletes_the_tabled_objects() {
        let mut e = engine();
        // Two objects share one vtable whose slot 0 is a deleting destructor.
        e.put_vtable(0x0700_0000, &[0x0050_0000]);
        for object in [0x0800_0000, 0x0800_0100, 0x0800_0200] {
            e.map(object, 8);
            e.mem.set_u32(object, 0x0700_0000);
        }
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        e.mem.set_u32(POINTER_PAIRS + 4 + 8 * 3, 0x0800_0100);
        e.mem.set_u32(POINTER_PAIRS + 8 * 8 + 4, 0x0800_0200);
        e.register(0x0050_0000, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f6320, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(
            calls_to(&e, 0x0050_0000),
            vec![
                vec![0x0800_0000, 1],
                vec![0x0800_0100, 1],
                vec![0x0800_0200, 1],
            ]
        );
        for pair in 0..18 {
            assert_eq!(e.mem.u32(POINTER_PAIRS + pair * 4), 0, "word {pair}");
        }
        assert_eq!(e.global::<u32>(OWNING_LISTS), 1);
    }

    #[test]
    fn a_later_owning_list_only_counts_itself() {
        let mut e = engine();
        e.set_global(OWNING_LISTS, 2u32);
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f6320, &args![this, 0x101u32]);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 3);
        assert_eq!(e.mem.u32(POINTER_PAIRS), 0x0800_0000);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
    #[test]
    fn at_004f6480_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_64b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_6480, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_6480)
                .collect();
            let mut expected = vec![(0x004f_64b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
