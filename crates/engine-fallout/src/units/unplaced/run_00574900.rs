//! `(unplaced)/run_00574900` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESObjectREFR::HasContainer` (Xbox PDB, `0055d310`): `thiscall`; the reference's container
/// data, or null.
const HAS_CONTAINER: u32 = 0x0055_d310;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB, `004bf220`): `cdecl`, one stack word (the
/// reference).
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges::WearingObject` (Xbox PDB, `004bfda0`): `thiscall`, `RET 8` (an item, a
/// second word).
const WEARING_OBJECT: u32 = 0x004b_fda0;

// Translated from 00575400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4` (an item): false when the reference has no
/// container data (`HasContainer`); otherwise whether its inventory changes
/// (`GetInventoryChanges(this)`) report `WearingObject(item, 0)` non-zero. The decompiler shows
/// the inventory changes call with the wrong `this`; the disassembly passes the reference to
/// the `cdecl` getter and its result as `this` of the second call.
pub fn fn_00575400(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let mut result = false;
    if e.call(HAS_CONTAINER, &args![this]).u32() != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![this]).u32();
        if e.call(WEARING_OBJECT, &args![changes, item, 0u32]).u32() != 0 {
            result = true;
        }
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0057_5400, fn_00575400(Ptr, Ptr) -> bool)]
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

    #[test]
    fn at_00575400_asks_the_inventory_changes_only_for_a_reference_with_a_container() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(HAS_CONTAINER, |_, _| Ret::default());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9000u32.into_ret());
        e.register(WEARING_OBJECT, |_, _| 1u32.into_ret());
        assert!(!e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        assert!(calls(&e, GET_INVENTORY_CHANGES).is_empty());
        e.register(HAS_CONTAINER, |_, _| 0x7000u32.into_ret());
        assert!(e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        assert_eq!(calls(&e, GET_INVENTORY_CHANGES), vec![vec![0x4000]]);
        assert_eq!(calls(&e, WEARING_OBJECT), vec![vec![0x9000, 0x5000, 0]]);
    }

    #[test]
    fn at_00575400_is_false_when_the_item_is_not_worn() {
        let mut e = Engine::new();
        e.register(HAS_CONTAINER, |_, _| 0x7000u32.into_ret());
        e.register(GET_INVENTORY_CHANGES, |_, _| 0x9000u32.into_ret());
        e.register(WEARING_OBJECT, |_, _| Ret::default());
        assert!(!e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
        // Only the full register counts, not just its low byte.
        e.register(WEARING_OBJECT, |_, _| 0x100u32.into_ret());
        assert!(e.call(0x0057_5400, &args![0x4000u32, 0x5000u32]).bool());
    }
}
