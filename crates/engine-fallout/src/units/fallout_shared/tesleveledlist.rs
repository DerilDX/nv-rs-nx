//! `fallout shared/tesleveledlist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESLeveledList` (Xbox PDB, a `BaseFormComponent`; 0x1C bytes there):
    /// the fields used so far, at the offsets the PC code uses.
    pub struct TESLeveledList: 0x1C {
        /// `leveledList` (Xbox PDB): `BSSimpleList<LEVELED_OBJECT*>`, its
        /// first node inline at +4.
        0x04 leveledList: u32,
        /// `pChanceGlobal` (Xbox PDB): `TESGlobal*`, a form id until
        /// `InitItem` resolves it.
        0x10 pChanceGlobal: u32,
    }
}

/// `00717e50` (`thiscall`): `this + 4` (the engine map's name,
/// `BaseProcess::GetActorPackageThatIsRunning`, is a folded body); here the
/// list's first node.
const LIST_HEAD: u32 = 0x0071_7e50;
/// `006815c0` (`thiscall`): returns `this`; here the address of a node's
/// item word.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the word at +4; here the next node.
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (`thiscall` on a list node, no stack words; no Xbox PDB
/// name): called on the node whose form is missing.
const NODE_REMOVE_0063F7B0: u32 = 0x0063_f7b0;
/// `00481760` (`thiscall`, one word; no Xbox PDB name): called with 1 on the
/// `LEVELED_OBJECT` whose form is missing.
const ITEM_DELETE_00481760: u32 = 0x0048_1760;
/// `ContainerItemExtra::InitItem` (`0040e7f0`, engine map; `thiscall`, one
/// word: the owner).
const CONTAINER_ITEM_EXTRA_INIT_ITEM: u32 = 0x0040_e7f0;
/// `TESForm::GetFile` (`00484e60`, engine map; `thiscall`, one word).
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, engine map; `cdecl`: `&form_id`,
/// file).
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by form id (`004839c0`, `cdecl`).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor `.?AVTESGlobal@@`.
const TYPE_TES_GLOBAL: u32 = 0x0118_3ad4;
/// `00474cb0` (`thiscall` on the owner): non-zero when the owner has a name.
const OWNER_HAS_NAME: u32 = 0x0047_4cb0;
/// The owner's name: its vtable slot +0x130.
const OWNER_NAME_SLOT: u32 = 0x130;
/// `0084e3a0` (`thiscall`): the owner's word at +0xC (`TESForm` form id).
const OWNER_FORM_ID: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, `cdecl`, format and arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find Leveled Object Form (%08X) for unknown owner."
const MISSING_OBJECT_NO_OWNER: u32 = 0x0101_ce18;
/// "MASTERFILE: Unable to find Leveled Object Form (%08X) for owner object (%08X)."
const MISSING_OBJECT_OWNER_ID: u32 = 0x0101_ce68;
/// "MASTERFILE: Unable to find Leveled Object Form (%08X) for owner object \"%s\"."
const MISSING_OBJECT_OWNER_NAME: u32 = 0x0101_ceb8;
/// "MASTERFILE: Unable to find Leveled Chance Global (%08X) for unknown owner."
const MISSING_GLOBAL_NO_OWNER: u32 = 0x0101_cd20;
/// "MASTERFILE: Unable to find Leveled Chance Global (%08X) for owner object (%08X)."
const MISSING_GLOBAL_OWNER_ID: u32 = 0x0101_cd70;
/// "MASTERFILE: Unable to find Leveled Chance Global (%08X) for owner object \"%s\"."
const MISSING_GLOBAL_OWNER_NAME: u32 = 0x0101_cdc8;

/// Reports a missing form with the owner's name, its form id, or as on an
/// unknown owner.
fn report_missing(e: &mut Engine, owner: Ptr, id: u32, formats: [u32; 3]) {
    let [no_owner, by_id, by_name] = formats;
    if owner.addr() == 0 {
        e.call(PRINT_WARNING, &args![no_owner, id]);
    } else if e.call(OWNER_HAS_NAME, &args![owner]).u32() != 0 {
        let name = e.vcall(owner.addr(), OWNER_NAME_SLOT, &args![]).u32();
        e.call(PRINT_WARNING, &args![by_name, id, name]);
    } else {
        let owner_id = e.call(OWNER_FORM_ID, &args![owner]).u32();
        e.call(PRINT_WARNING, &args![by_id, id, owner_id]);
    }
}

// Translated from 00488570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESLeveledList::InitItem` (Xbox PDB), `thiscall (this; owner)`: resolves
/// the form ids of the list's `LEVELED_OBJECT`s and of `pChanceGlobal`. Each
/// id gets the owner's file compile index (`TESForm::GetFile(owner, -1)`,
/// none without an owner; `TESForm::AddCompileIndex`) and is looked up. A
/// missing object form takes its node out (`0063f7b0` on the node, the
/// object given to `00481760(1)`; the loop then looks at the same node
/// again) and is reported; a found one moves on to the next node. An object
/// left with a `ContainerItemExtra` (+8) has it initialized for the owner.
/// The global is cast to `TESGlobal`; a missing one (null) is reported.
pub fn tes_leveled_list_init_item(e: &mut Engine, this: Ptr<TESLeveledList>, owner: Ptr) {
    let mut node = e.call(LIST_HEAD, &args![this]).u32();
    let file = if owner.addr() != 0 {
        e.call(FORM_GET_FILE, &args![owner, -1i32]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, form_id| {
        while node != 0 {
            let slot = e.call(NODE_ITEM, &args![node]).u32();
            let mut item = e.mem.u32(slot);
            if item == 0 {
                break;
            }
            e.mem.set_u32(form_id.addr(), e.mem.u32(item));
            e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
            let id = e.mem.u32(form_id.addr());
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            e.mem.set_u32(item, form);
            if e.mem.u32(item) == 0 {
                e.call(NODE_REMOVE_0063F7B0, &args![node]);
                if item != 0 {
                    e.call(ITEM_DELETE_00481760, &args![item, 1u32]);
                }
                item = 0;
                report_missing(
                    e,
                    owner,
                    id,
                    [
                        MISSING_OBJECT_NO_OWNER,
                        MISSING_OBJECT_OWNER_ID,
                        MISSING_OBJECT_OWNER_NAME,
                    ],
                );
            } else {
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            if item != 0 && e.mem.u32(item + 8) != 0 {
                let extra = e.mem.u32(item + 8);
                e.call(CONTAINER_ITEM_EXTRA_INIT_ITEM, &args![extra, owner]);
            }
        }
    });
    if e.get(this, TESLeveledList::pChanceGlobal) == 0 {
        return;
    }
    e.with_stack(4, |e, form_id| {
        let id = e.get(this, TESLeveledList::pChanceGlobal);
        e.mem.set_u32(form_id.addr(), id);
        e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
        let id = e.mem.u32(form_id.addr());
        let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
        let global = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0u32],
            )
            .u32();
        e.set(this, TESLeveledList::pChanceGlobal, global);
        if e.get(this, TESLeveledList::pChanceGlobal) == 0 {
            report_missing(
                e,
                owner,
                id,
                [
                    MISSING_GLOBAL_NO_OWNER,
                    MISSING_GLOBAL_OWNER_ID,
                    MISSING_GLOBAL_OWNER_NAME,
                ],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0048_8570,
        tes_leveled_list_init_item(Ptr<TESLeveledList>, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;
    type Run = (Engine, Ptr<TESLeveledList>, CallLog, Vec<u32>);

    const OWNER_VTABLE: u32 = 0x0170_0000;
    const OWNER_NAME: u32 = 0x0170_1000;
    const FILE: u32 = 0x0000_7000;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn list_doubles(e: &mut Engine) {
        e.register(LIST_HEAD, |_, a| word(a[0] + 4));
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.register(CONTAINER_ITEM_EXTRA_INIT_ITEM, |_, _| Ret::default());
    }

    /// A list of `ids` on a fresh leveled list (its first node inline at
    /// +4); ids with the top bit set are not found. The node remover copies
    /// the next node in, as a simple list does.
    fn run(ids: &[u32], global: u32, owner: bool, named: bool) -> Run {
        let mut e = Engine::new();
        let this = e.new_object::<TESLeveledList>();
        let mut items = vec![];
        let mut next = 0;
        for id in ids.iter().rev() {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, *id);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, next);
            next = node;
            items.push(item);
        }
        items.reverse();
        e.mem.set_u32(this.addr() + 4, e.mem.u32(next));
        e.mem.set_u32(this.addr() + 8, e.mem.u32(next + 4));
        e.set(this, TESLeveledList::pChanceGlobal, global);
        let owner_form = if owner {
            let p = e.mem.alloc(0x20);
            let mut slots = vec![0u32; 0x130 / 4 + 1];
            slots[0x130 / 4] = OWNER_NAME;
            e.put_vtable(OWNER_VTABLE, &slots);
            e.mem.set_u32(p, OWNER_VTABLE);
            p
        } else {
            0
        };
        list_doubles(&mut e);
        e.register(NODE_REMOVE_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(ITEM_DELETE_00481760, |_, a| word(a[0]));
        e.register(FORM_GET_FILE, |_, _| word(FILE));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM_BY_ID, |_, a| {
            word(if a[0] & 0x8000_0000 != 0 {
                0
            } else {
                (a[0] ^ 0x0100_0000) | 0x0050_0000
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| word(a[0]));
        e.register_double(OWNER_HAS_NAME, move |_, _| word(u32::from(named)));
        e.register(OWNER_NAME, |_, _| word(0x9000));
        e.register(OWNER_FORM_ID, |_, _| word(0x0012_3456));
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_8570, &args![this, owner_form]);
        let log = e.call_log.take().unwrap();
        (e, this, log, items)
    }

    #[test]
    fn init_item_resolves_objects_and_the_global() {
        let (e, this, log, items) = run(&[0x11, 0x22], 0x33, true, true);
        assert_eq!(e.mem.u32(items[0]), 0x0050_0011);
        assert_eq!(e.mem.u32(items[1]), 0x0050_0022);
        assert_eq!(e.get(this, TESLeveledList::pChanceGlobal), 0x0050_0033);
        assert_eq!(calls_to(&log, FORM_GET_FILE)[0][1], u32::MAX);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x0050_0033, 0, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0]]
        );
        assert!(calls_to(&log, PRINT_WARNING).is_empty());
        assert!(calls_to(&log, CONTAINER_ITEM_EXTRA_INIT_ITEM).is_empty());
    }

    #[test]
    fn init_item_initializes_item_extras() {
        let mut e = Engine::new();
        let this = e.new_object::<TESLeveledList>();
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, 0x11);
        e.mem.set_u32(item + 8, 0x4440);
        e.mem.set_u32(this.addr() + 4, item);
        list_doubles(&mut e);
        e.register(FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(LOOKUP_FORM_BY_ID, |_, a| word(a[0] + 0x100));
        e.call_log = Some(vec![]);
        e.call(0x0048_8570, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(item), 0x111);
        assert_eq!(
            calls_to(&log, CONTAINER_ITEM_EXTRA_INIT_ITEM),
            vec![vec![0x4440, 0]]
        );
        assert_eq!(calls_to(&log, FORM_ADD_COMPILE_INDEX)[0][1], 0);
        assert!(calls_to(&log, FORM_GET_FILE).is_empty());
    }

    #[test]
    fn init_item_removes_and_reports_missing_forms() {
        let (e, this, log, items) = run(&[0x8000_0011, 0x22], 0x8000_0033, true, true);
        assert_eq!(
            calls_to(&log, ITEM_DELETE_00481760),
            vec![vec![items[0], 1]]
        );
        assert_eq!(calls_to(&log, NODE_REMOVE_0063F7B0).len(), 1);
        // The node now holds the second object, which was resolved.
        assert_eq!(e.mem.u32(this.addr() + 4), items[1]);
        assert_eq!(e.mem.u32(items[1]), 0x0050_0022);
        assert_eq!(e.get(this, TESLeveledList::pChanceGlobal), 0);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![MISSING_OBJECT_OWNER_NAME, 0x8100_0011, 0x9000],
                vec![MISSING_GLOBAL_OWNER_NAME, 0x8100_0033, 0x9000]
            ]
        );
        let (_, _, log, _) = run(&[0x8000_0011], 0x8000_0033, true, false);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![MISSING_OBJECT_OWNER_ID, 0x8100_0011, 0x0012_3456],
                vec![MISSING_GLOBAL_OWNER_ID, 0x8100_0033, 0x0012_3456]
            ]
        );
        let (_, _, log, _) = run(&[0x8000_0011], 0x8000_0033, false, false);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![
                vec![MISSING_OBJECT_NO_OWNER, 0x8100_0011],
                vec![MISSING_GLOBAL_NO_OWNER, 0x8100_0033]
            ]
        );
    }
}
