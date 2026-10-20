//! `fallout shared/tesbipedmodelform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (unnamed in the engine map): `thiscall` getter, the form type byte at `this + 4`.
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4` (a list node's
/// next).
const LIST_NEXT: u32 = 0x0072_6070;
/// `00500940` (unnamed in the engine map): `thiscall`; `this + 0x18` (the first node of a
/// container's list).
const LIST_FIRST_NODE_AT_18: u32 = 0x0050_0940;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, whether the node is empty
/// (no item and no next).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: returns `this` (the address of a node's item pointer).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;

// Translated from 00480db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetFormAsBipedModel` (Xbox PDB): `cdecl`, one stack word. For a non-null
/// form, answers its biped model part by the form's type byte (`00401170`): type 0x18 at
/// `form + 0x70`, type 0x1a at `form + 0x68`, type 0x60 at `form + 0x70`; null for any other type
/// or a null form. (The exe tests the form for null again inside each case; those tests cannot
/// fail and are kept.)
pub fn tes_biped_model_form_get_form_as_biped_model(e: &mut Engine, form: Ptr) -> Ptr {
    let mut result = 0;
    if !form.is_null() {
        let part_at = |offset: u32| {
            if form.is_null() {
                0
            } else {
                form.addr().wrapping_add(offset)
            }
        };
        match e.call(FORM_TYPE_BYTE, &args![form]).u32() {
            0x18 => result = part_at(0x70),
            0x1a => result = part_at(0x68),
            0x60 => result = part_at(0x70),
            _ => {}
        }
    }
    Ptr::new(result)
}

// Translated from 00480af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::FillsBipedSlot` (Xbox PDB): `thiscall`, `RET 0xc` (a slot number, a flag
/// byte, a container). False when the flag is set, the slot is 1 and bit 0 of the flags dword
/// at `this + 4` is set. Otherwise true when bit `slot & 31` of that dword is set; if it is
/// not and a container was given, walks the container's list (its first node comes from the
/// container's own list head `00726070`, then `00500940` on that) and asks every item that is a
/// biped model form (`GetFormAsBipedModel`) the same question, without a container, until one
/// answers true or the list ends.
pub fn tes_biped_model_form_fills_biped_slot(
    e: &mut Engine,
    this: Ptr,
    slot: u32,
    flag: u8,
    container: Ptr,
) -> bool {
    let flags = this.addr().wrapping_add(4);
    if flag != 0 && slot == 1 && e.mem.u32(flags) & 1 != 0 {
        return false;
    }
    let mut filled = (1u32.wrapping_shl(slot & 0x1f) & e.mem.u32(flags)) != 0;
    if !filled && !container.is_null() {
        let head = e.call(LIST_NEXT, &args![container]).u32();
        if head != 0 {
            let mut node = e.call(LIST_FIRST_NODE_AT_18, &args![head]).u32();
            while !filled && node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let item_slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let form = e.mem.u32(item_slot);
                let part = tes_biped_model_form_get_form_as_biped_model(e, Ptr::new(form));
                if !part.is_null() {
                    filled = tes_biped_model_form_fills_biped_slot(e, part, slot, flag, Ptr::NULL);
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
        }
    }
    filled
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_0db0, tes_biped_model_form_get_form_as_biped_model(Ptr) -> Ptr),
        entry!(0x0048_0af0, tes_biped_model_form_fills_biped_slot(Ptr, u32, u8, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(FORM_TYPE_BYTE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_FIRST_NODE_AT_18, |_, a| (a[0] + 0x18).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e
    }

    fn form_of_type(e: &mut Engine, kind: u32) -> u32 {
        let form = e.mem.alloc(0x80);
        e.mem.set_u32(form + 4, kind);
        form
    }

    #[test]
    fn at_00480db0_answers_the_part_each_type_selects() {
        let mut e = rig();
        for (kind, offset) in [(0x18u32, 0x70u32), (0x1a, 0x68), (0x60, 0x70)] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(
                e.call(0x0048_0db0, &args![form]).u32(),
                form + offset,
                "{kind:#x}"
            );
        }
        for kind in [0u32, 0x17, 0x19, 0x61, 0xff] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(e.call(0x0048_0db0, &args![form]).u32(), 0, "{kind:#x}");
        }
        assert_eq!(e.call(0x0048_0db0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, FORM_TYPE_BYTE).len(), 8);
    }

    #[test]
    fn at_00480af0_tests_the_slot_bit() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b1010);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, 0u32]).bool());
        assert!(e.call(0x0048_0af0, &args![this, 3u32, 0u8, 0u32]).bool());
        // The shift count is taken modulo 32.
        assert!(e.call(0x0048_0af0, &args![this, 33u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 34u32, 0u8, 0u32]).bool());
    }

    #[test]
    fn at_00480af0_the_flag_with_slot_1_and_bit_0_means_false() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b11);
        assert!(!e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
        // Without the flag the slot-1 bit is true as usual.
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        // Another slot is unaffected by the flag.
        assert!(e.call(0x0048_0af0, &args![this, 0u32, 1u8, 0u32]).bool());
        // Slot 1 with bit 0 clear: the slot bit decides.
        e.mem.set_u32(this + 4, 0b10);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
    }

    /// A container whose list head sits at `container + 4` (`00726070`), its nodes from
    /// `head + 0x18`: (item, next).
    fn container_of(e: &mut Engine, forms: &[u32]) -> u32 {
        let container = e.mem.alloc(0x10);
        let head = e.mem.alloc(0x40);
        e.mem.set_u32(container + 4, head);
        let mut next = 0;
        let mut nodes = Vec::new();
        for form in forms.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *form);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.push(node);
        }
        // The first node is the head's list head: `00500940` answers `head + 0x18`, so the
        // first node is stored there.
        let first = nodes.last().copied().unwrap_or(0);
        if first != 0 {
            e.mem.set_u32(head + 0x18, e.mem.u32(first));
            e.mem.set_u32(head + 0x1c, e.mem.u32(first + 4));
        }
        container
    }

    #[test]
    fn at_00480af0_asks_the_container_s_forms_when_the_slot_is_not_set_here() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0);
        let unrelated = form_of_type(&mut e, 0x19);
        let biped_form = form_of_type(&mut e, 0x18);
        // The biped part of the form (at +0x70) has the slot bit set.
        e.mem.set_u32(biped_form + 0x70 + 4, 0b100);
        let container = container_of(&mut e, &[unrelated, biped_form]);
        assert!(e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        let types = calls(&e, FORM_TYPE_BYTE);
        assert_eq!(types, vec![vec![unrelated], vec![biped_form]]);
    }

    #[test]
    fn at_00480af0_answers_false_when_no_form_of_the_container_fills_the_slot() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        let biped_form = form_of_type(&mut e, 0x18);
        let container = container_of(&mut e, &[biped_form]);
        assert!(!e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        // An empty container (no list head) ends the search at once.
        let bare = e.mem.alloc(0x10);
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, bare]).bool());
    }
}
