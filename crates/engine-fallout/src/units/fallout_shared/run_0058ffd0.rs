//! `fallout shared/run_0058ffd0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00500940` (`thiscall`, no stack words): the form list member of `this`.
const FORM_LIST: u32 = 0x0050_0940;
/// `005f65d0` (`thiscall`, `RET 4`): whether the list holds the form whose
/// word is at the given address (`AL`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `005ae3d0` (`thiscall`, `RET 4`): adds the form whose word is at the
/// given address to the list.
const LIST_ADD: u32 = 0x005a_e3d0;
/// `00484d60` (`thiscall`, `RET 4`): the base class's save, on the buffer.
const BASE_SAVE: u32 = 0x0048_4d60;
/// `00428110` (`thiscall` on the buffer, `RET 4`): fills the local at the
/// given address and answers an object for `004280f0`.
const SAVE_FLAGS: u32 = 0x0042_8110;
/// `004280f0` (`thiscall`, `RET 4`): whether the flag word is set (`AL`).
const FLAGS_HAVE: u32 = 0x0042_80f0;
/// `BGSSaveGameBuffer` raw save (`00865e50`, `thiscall`: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map), `thiscall`: form, 0.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `006815c0` (`thiscall`): a list node's item address.
const NODE_ITEM: u32 = 0x0068_15c0;
/// `00726070` (`thiscall`): the next list node.
const NODE_NEXT: u32 = 0x0072_6070;
/// The flag word both functions use (the form's changed flag and the save
/// flag tested).
const CHANGED_FLAG: u32 = 0x8000_0000;

// Translated from 00590010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `form` to the form list of `this` when it is not null and not in it
/// yet (`005f65d0`, `005ae3d0` on `00500940`'s list, both given the address
/// of the argument's stack word), counts it in the word at +0x20 and marks
/// `this` changed (virtual slot +0x48 with `0x80000000`). No name in the
/// engine map.
pub fn fn_00590010(e: &mut Engine, this: Ptr, form: u32) {
    if form == 0 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), form);
        let list = e.call(FORM_LIST, &args![this]).u32();
        if !e.call(LIST_CONTAINS, &args![list, slot]).bool() {
            let list = e.call(FORM_LIST, &args![this]).u32();
            e.call(LIST_ADD, &args![list, slot]);
            let count = e.mem.u32(this.addr() + 0x20);
            e.mem.set_u32(this.addr() + 0x20, count.wrapping_add(1));
            e.vcall(this.addr(), 0x48, &args![CHANGED_FLAG]);
        }
    });
}

// Translated from 00590080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves `this` to `buffer`: the base save (`00484d60`), then, when the
/// save flags (`00428110` into a local, tested by `004280f0` for
/// `0x80000000`) ask for it, the count word at +0x20 and the form id of each
/// of that many items of the list at +0x18 (`00865df0`). No name in the
/// engine map.
pub fn fn_00590080(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(BASE_SAVE, &args![this, buffer]);
    let wanted = e.with_stack(4, |e, local| {
        let flags = e.call(SAVE_FLAGS, &args![buffer, local]).u32();
        e.call(FLAGS_HAVE, &args![flags, CHANGED_FLAG]).bool()
    });
    if !wanted {
        return;
    }
    let mut node = this.addr().wrapping_add(0x18);
    e.call(
        SAVE_DATA,
        &args![buffer, this.addr().wrapping_add(0x20), 4u32, 0u32],
    );
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x20) {
        let item_address = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_address);
        node = e.call(NODE_NEXT, &args![node]).u32();
        e.call(SAVE_FORM_ID, &args![buffer, item, 0u32]);
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_0010, fn_00590010(Ptr, u32)),
        entry!(0x0059_0080, fn_00590080(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn object(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x30);
        e.put_vtable(0x0300_0000, &[0; 0x13]);
        e.mem.set_u32(0x0300_0000 + 0x48, 0x0300_1000);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, _| Ret::default());
        e.register(FORM_LIST, |_, a| word(a[0] + 0x40));
        e.register(LIST_ADD, |_, _| Ret::default());
        this
    }

    #[test]
    fn fn_00590010_adds_a_new_form_once() {
        let mut e = Engine::new();
        let this = object(&mut e);
        e.register(LIST_CONTAINS, |e, a| {
            word(u32::from(e.mem.u32(a[1]) == 0x77))
        });
        e.call_log = Some(vec![]);
        e.call(0x0059_0010, &args![this, 0u32]);
        assert!(calls_to(&e, FORM_LIST).is_empty());
        e.call(0x0059_0010, &args![this, 0x77u32]);
        assert!(calls_to(&e, LIST_ADD).is_empty());
        e.call(0x0059_0010, &args![this, 0x55u32]);
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], this + 0x40);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![this, CHANGED_FLAG]]);
    }

    #[test]
    fn fn_00590080_saves_the_count_and_the_form_ids() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x30);
        let nodes = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x18, 0x0aaa);
        e.mem.set_u32(this + 0x1c, nodes);
        e.mem.set_u32(nodes, 0x0bbb);
        e.mem.set_u32(nodes + 4, 0);
        e.mem.set_u32(this + 0x20, 2);
        e.register(BASE_SAVE, |_, _| Ret::default());
        e.register(SAVE_FLAGS, |_, a| word(a[1]));
        e.register_double(FLAGS_HAVE, |_, _| word(1));
        e.register(SAVE_DATA, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(NODE_ITEM, |_, a| word(a[0]));
        e.register(NODE_NEXT, |e, a| word(e.mem.u32(a[0] + 4)));
        e.call_log = Some(vec![]);
        e.call(0x0059_0080, &args![this, 0x9000u32]);
        assert_eq!(calls_to(&e, BASE_SAVE), vec![vec![this, 0x9000]]);
        assert_eq!(calls_to(&e, FLAGS_HAVE)[0][1], CHANGED_FLAG);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![vec![0x9000, this + 0x20, 4, 0]]
        );
        assert_eq!(
            calls_to(&e, SAVE_FORM_ID),
            vec![vec![0x9000, 0x0aaa, 0], vec![0x9000, 0x0bbb, 0]]
        );
        // Without the flag only the base save runs.
        e.register(FLAGS_HAVE, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0059_0080, &args![this, 0x9000u32]);
        assert!(calls_to(&e, SAVE_DATA).is_empty());
        assert_eq!(calls_to(&e, BASE_SAVE).len(), 1);
    }
}
