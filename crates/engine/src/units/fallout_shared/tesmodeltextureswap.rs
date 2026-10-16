//! `fallout shared/tesmodeltextureswap.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048a470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00489040` (Ghidra name
/// `TESModel::InitializeDataComponent`), which tests `00559450` and clears the byte at
/// `+0x14`. The Xbox PDB name `TESModelTextureSwap::InitializeDataComponent` is the class's
/// vslot +0x000 (pdbdump `type ... TESModelTextureSwap`; the engine map row is `vt
/// 822daa78`), and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `00489040`'s disassembly ends on
/// the byte store `MOV byte ptr [EAX+0x14],0x0` with no result computed (Ghidra types it
/// void).
pub fn tes_model_texture_swap_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.call(0x0048_9040, &args![this]);
}

// Translated from 0048aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap::InitItem` (Xbox PDB): resolves the texture-set form ids of the swap list.
/// `owner` is the form that owns the model (it names the form in the error message).
///
/// It walks the list that `00500940` returns for `this` node by node (`00726070` is the next
/// node, `008256d0` tells whether a node ends the walk, `006815c0` gives the address of the
/// node's data pointer). Each entry holds an id; the entry gets the owner file's compile index
/// (`00484e60` with -1, `00485d50`), is looked up (`004839c0`) and cast with `__RTDynamicCast`
/// from the type at `0x01183028` to the type at `0x01187788`. An entry that is not found prints
/// `MASTERFILE: Failed to find texture set ...` (`005b5e40`), is taken out of the list (after a
/// previous good node: `00905330` on it, then the walk goes on with that node's next; at the
/// head: `0063f7b0` on the node, which stays the current node) and is freed (`00401030`).
pub fn tesmodeltextureswap_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let mut node = e.call(0x0050_0940, &args![this]).u32();
    let mut previous = 0u32;
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let data = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(data);
        let id_slot = e.mem.alloc(4);
        let entry_id = e.mem.u32(entry);
        e.mem.set_u32(id_slot, entry_id);
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![id_slot, file]);
        let id = e.mem.u32(id_slot);
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let found = e
            .call(
                0x00ec_43fb,
                &args![form, 0u32, 0x0118_3028u32, 0x0118_7788u32, 0u32],
            )
            .u32();
        e.mem.set_u32(entry, found);
        if found == 0 {
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            let owner_name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(
                0x005b_5e40,
                &args![0x0101_d148u32, id, owner_name, owner_id],
            );
            if previous != 0 {
                let list_entry = e.mem.alloc(4);
                e.mem.set_u32(list_entry, entry);
                e.call(0x0090_5330, &args![previous, list_entry]);
                e.mem.free(list_entry);
                node = e.call(0x0072_6070, &args![previous]).u32();
            } else {
                e.call(0x0063_f7b0, &args![node]);
            }
            e.call(0x0040_1030, &args![entry]);
        } else {
            previous = node;
            node = e.call(0x0072_6070, &args![node]).u32();
        }
        e.mem.free(id_slot);
    }
}

// Translated from 0048a440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESModelTextureSwap` scalar deleting destructor (engine map name): runs the destructor body `0048a490` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_model_texture_swap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_a490, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0048b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<int_TEX_SWAP_P>` scalar deleting destructor (engine map name): runs the destructor body `0048b460` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_int_tex_swap_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_b460, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0048b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<int_TEX_SWAP_P>` scalar deleting destructor (engine map name): runs the destructor body `0048b4c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_int_tex_swap_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0048_b4c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0048_a470,
            tes_model_texture_swap_initialize_data_component(Ptr)
        ),
        entry!(0x0048_aa80, tesmodeltextureswap_init_item(Ptr, Ptr)),
        entry!(0x0048_a440, tes_model_texture_swap_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0048_b3b0, bs_map_int_tex_swap_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0048_b530, bs_map_base_int_tex_swap_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
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

    #[test]
    fn fwd_a03_0048a470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0048_9040, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_a470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0048_9040), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{calls_to, run, stub};
    use std::cell::Cell;
    use std::rc::Rc;

    const OWNER: u32 = 0x0800_0000;
    const FIRST_NODE: u32 = 0x0800_3000;
    const SECOND_NODE: u32 = 0x0800_3100;
    const OWNER_NAME: u32 = 0xaaaa_0001;
    /// An id the lookup (`004839c0`) does not find.
    const MISSING_ID: u32 = 0xbad;

    struct Setup {
        e: Engine,
        entries: [u32; 2],
    }

    /// A list of two nodes whose entries hold `ids`. `ends_after_first_visit` makes
    /// `008256d0` answer true from its second call on (the walk at a head that stays).
    fn setup(ids: [u32; 2], ends_after_first_visit: bool) -> Setup {
        let mut e = Engine::new();
        let entries = ids.map(|id| {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, id);
            entry
        });
        let slots = entries.map(|entry| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, entry);
            slot
        });
        e.map(OWNER, 0x40);
        e.map(0x0900_0000, 0x200);
        e.mem.set_u32(OWNER, 0x0900_0000);
        e.mem.set_u32(0x0900_0130, 0x0090_0300);
        e.register_double(0x0090_0300, |_, _| Ret {
            eax: OWNER_NAME,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[0x005b_5e40, 0x0063_f7b0, 0x0040_1030, 0x0090_5330],
        );
        e.register_double(0x0050_0940, move |_, _| Ret {
            eax: FIRST_NODE,
            ..Ret::default()
        });
        let visits = Rc::new(Cell::new(0));
        e.register_double(0x0082_56d0, move |_, _| {
            visits.set(visits.get() + 1);
            Ret {
                eax: u32::from(ends_after_first_visit && visits.get() > 1),
                ..Ret::default()
            }
        });
        e.register_double(0x0068_15c0, move |_, a| Ret {
            eax: if a[0] == FIRST_NODE {
                slots[0]
            } else {
                slots[1]
            },
            ..Ret::default()
        });
        // `00905330` takes the second node out of the list: the first node then has no next.
        let removed = Rc::new(Cell::new(false));
        let removal = removed.clone();
        e.register_double(0x0090_5330, move |_, _| {
            removal.set(true);
            Ret::default()
        });
        e.register_double(0x0072_6070, move |_, a| Ret {
            eax: match (a[0], removed.get()) {
                (FIRST_NODE, false) => SECOND_NODE,
                _ => 0,
            },
            ..Ret::default()
        });
        e.register(0x0048_4e60, |_, _| Ret {
            eax: 0x0a11_e000,
            ..Ret::default()
        });
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: if a[0] & 0xffff == MISSING_ID {
                0
            } else {
                a[0] ^ 0x1000_0000
            },
            ..Ret::default()
        });
        // The cast keeps what the lookup found.
        e.register(0x00ec_43fb, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 0x77,
            ..Ret::default()
        });
        Setup { e, entries }
    }

    #[test]
    fn found_entries_are_replaced_by_their_forms() {
        let mut s = setup([0x11, 0x22], false);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(s.e.mem.u32(s.entries[0]), 0x1100_0011);
        assert_eq!(s.e.mem.u32(s.entries[1]), 0x1100_0022);
        assert_eq!(
            calls_to(&calls, 0x0072_6070),
            [vec![FIRST_NODE], vec![SECOND_NODE]]
        );
        assert!(calls_to(&calls, 0x005b_5e40).is_empty());
        assert_eq!(calls_to(&calls, 0x0050_0940), [vec![0x1000]]);
    }

    #[test]
    fn a_missing_entry_after_a_good_one_is_taken_out_of_the_list() {
        let mut s = setup([0x11, MISSING_ID], false);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_d148, 0x0100_0bad, OWNER_NAME, OWNER + 0x77]]
        );
        // The list entry holds the entry's address; the walk continues after the good node.
        let removal = calls_to(&calls, 0x0090_5330);
        assert_eq!(removal.len(), 1);
        assert_eq!(removal[0][0], FIRST_NODE);
        assert_eq!(
            calls_to(&calls, 0x0072_6070),
            [vec![FIRST_NODE], vec![FIRST_NODE]]
        );
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![s.entries[1]]]);
        assert!(calls_to(&calls, 0x0063_f7b0).is_empty());
        // The entry was cleared before it was freed.
        assert_eq!(s.e.mem.u32(s.entries[1]), 0);
    }

    #[test]
    fn a_missing_head_entry_is_removed_in_place() {
        let mut s = setup([MISSING_ID, 0x22], true);
        let (_, calls) = run(&mut s.e, 0x0048_aa80, &[0x1000, OWNER]);
        assert_eq!(calls_to(&calls, 0x0063_f7b0), [vec![FIRST_NODE]]);
        assert!(calls_to(&calls, 0x0090_5330).is_empty());
        assert!(calls_to(&calls, 0x0072_6070).is_empty());
        assert_eq!(calls_to(&calls, 0x0040_1030), [vec![s.entries[0]]]);
        // The head node is asked again and ends the walk.
        assert_eq!(
            calls_to(&calls, 0x0082_56d0),
            [vec![FIRST_NODE], vec![FIRST_NODE]]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0048a440_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_a490, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_a440, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_a440)
                .collect();
            let mut expected = vec![(0x0048_a490, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0048b3b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_b460, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_b3b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_b3b0)
                .collect();
            let mut expected = vec![(0x0048_b460, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0048b530_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_b4c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_b530, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_b530)
                .collect();
            let mut expected = vec![(0x0048_b4c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
