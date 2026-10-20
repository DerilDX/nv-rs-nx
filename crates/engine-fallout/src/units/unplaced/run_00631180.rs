//! `(unplaced)/run_00631180` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const LIST_FIRST_NODE: u32 = 0x0055_9450;
/// `007b52d0` (`run_007b52a0.rs`): `thiscall`, `RET 4`; the next node (`*node`, null for null).
const LIST_NEXT_NODE: u32 = 0x007b_52d0;
/// `006317a0` (`run_006317a0.rs`): `thiscall`, `RET 4`; the address of the node's item
/// (`node + 8`).
const LIST_ITEM_SLOT: u32 = 0x0063_17a0;
/// `0062fc70` (unnamed in the engine map, `teswaterlistener.cpp` range): `thiscall`, `RET 4`;
/// takes a pair back (what it does is unconfirmed).
const PAIR_TAKE_BACK: u32 = 0x0062_fc70;
/// `0049c5d0` (unnamed in the engine map): `thiscall`, `RET 4`; unlinks the node the iterator
/// cell points to from the list and moves the cell to the next node.
const LIST_REMOVE_NODE: u32 = 0x0049_c5d0;
/// `00401030` (unnamed in the engine map): `cdecl`, one stack word; frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `00437c10` (unnamed in the engine map): `thiscall`; constructs two vectors (at +0 and +0x10)
/// with `006815c0` and returns `this`.
const VECTOR_PAIR_CONSTRUCTOR: u32 = 0x0043_7c10;
/// `00560d30` (unnamed in the engine map): `thiscall`, `RET 4`; the address of element `index`
/// of a dword array (`this + index * 4`).
const DWORD_ARRAY_ELEMENT: u32 = 0x0056_0d30;
/// `004a3e90` (unnamed in the engine map): `cdecl`, one `float` stack word; multiplies it by the
/// `float` at `011c582c` and answers it in `ST0`.
const FLOAT_SCALE: u32 = 0x004a_3e90;
/// The virtual slot (byte offset) of the pair's first object that says the pair is still
/// valid (zero: it is not).
const SLOT_IS_VALID: u32 = 0x1d0;
/// The virtual slot of the second object that fills the vector pair.
const SLOT_FILL_VECTORS: u32 = 0xf8;
/// The virtual slot of the third object that answers a pointer to a record with a float at +8.
const SLOT_RECORD: u32 = 0x1f4;

// Translated from 00631370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631370 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// Unnamed in the engine map. `thiscall`, `RET 0xc` (three stack words: a form pointer and two
/// flag bytes). Walks the list at `this + 0x34`; each node's item is a heap triple (`first`,
/// `second`, `third`). A triple whose first object is no longer valid (virtual slot +0x1d0 is
/// zero) is taken back (`0062fc70`), unlinked and freed. Otherwise it is selected when `matching`
/// is null or equals the triple's first or third pointer; when the first flag byte is set the
/// selection is replaced by "the scaled `float` at +8 of the record the third object answers is
/// below the `float` at index 2 of the vector pair the second object fills". A selected triple
/// is taken back first unless the second flag byte is set, then unlinked and freed. The next
/// node is fetched before a node is touched, and the loop test on the current node inside the
/// body can never fail (kept for fidelity).
pub fn fn_00631370(e: &mut Engine, this: Ptr, matching: u32, check_distance: u8, keep: u8) {
    let list = this.addr().wrapping_add(0x34);
    let mut node = e.call(LIST_FIRST_NODE, &args![list]).u32();
    while node != 0 {
        let next = e.call(LIST_NEXT_NODE, &args![list, node]).u32();
        if node != 0 {
            let item_slot = e.call(LIST_ITEM_SLOT, &args![list, node]).u32();
            let triple = e.mem.u32(item_slot);
            let first = e.mem.u32(triple);
            if e.vcall(first, SLOT_IS_VALID, &args![]).u32() == 0 {
                e.call(PAIR_TAKE_BACK, &args![this, triple]);
                unlink_node(e, list, node);
                e.call(FREE_BLOCK, &args![triple]);
            } else {
                let mut selected = matching == 0
                    || e.mem.u32(triple) == matching
                    || e.mem.u32(triple.wrapping_add(8)) == matching;
                if check_distance != 0 {
                    selected = below_limit(e, triple);
                }
                if selected {
                    if keep == 0 {
                        e.call(PAIR_TAKE_BACK, &args![this, triple]);
                    }
                    unlink_node(e, list, node);
                    e.call(FREE_BLOCK, &args![triple]);
                }
            }
        }
        node = next;
    }
}

/// `0049c5d0` on a node held in a one-word cell (the exe passes the address of its local).
fn unlink_node(e: &mut Engine, list: u32, node: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), node);
        e.call(LIST_REMOVE_NODE, &args![list, cell]);
    });
}

/// The distance test of [`fn_00631370`]: fills the vector pair through the second object, takes
/// element 2 of it and compares it with `004a3e90` of the `float` at +8 of the third object's
/// record: true when the scaled value is strictly below it (a NaN on either side gives false,
/// as the exe's `FCOMPP` flag test does).
fn below_limit(e: &mut Engine, triple: u32) -> bool {
    let vectors = e.mem.alloc(0x20);
    e.call(VECTOR_PAIR_CONSTRUCTOR, &args![vectors]);
    let second = e.mem.u32(triple.wrapping_add(4));
    e.vcall(second, SLOT_FILL_VECTORS, &args![vectors]);
    let limit_slot = e.call(DWORD_ARRAY_ELEMENT, &args![vectors, 2u32]).u32();
    let third = e.mem.u32(triple.wrapping_add(8));
    let record = e.vcall(third, SLOT_RECORD, &args![]).u32();
    let value = x87_quiet_bits(e.mem.u32(record.wrapping_add(8)));
    let scaled = e.call(FLOAT_SCALE, &args![value]).f32();
    let limit = x87_quiet(e.mem.f32(limit_slot));
    e.mem.free(vectors);
    scaled < limit
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063_1370, fn_00631370(Ptr, u32, u8, u8))]
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

    /// Three objects per triple: the first is valid when its +8 is non-zero, the second fills
    /// the vector pair's element 2 from its +8, the third answers the record at its +8.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut valid = [0u32; 0x80];
        valid[SLOT_IS_VALID as usize / 4] = 0x0f00_b000;
        e.put_vtable(0x0f00_b100, &valid);
        let mut fill = [0u32; 0x80];
        fill[SLOT_FILL_VECTORS as usize / 4] = 0x0f00_b010;
        e.put_vtable(0x0f00_b600, &fill);
        let mut record = [0u32; 0x80];
        record[SLOT_RECORD as usize / 4] = 0x0f00_b020;
        e.put_vtable(0x0f00_bb00, &record);
        e.register(0x0f00_b000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0f00_b010, |e, a| {
            let limit = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[1] + 8, limit);
            Ret::default()
        });
        e.register(0x0f00_b020, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LIST_FIRST_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[1]).into_ret());
        e.register(LIST_ITEM_SLOT, |_, a| (a[1] + 8).into_ret());
        e.register(PAIR_TAKE_BACK, |_, _| Ret::default());
        e.register(LIST_REMOVE_NODE, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(VECTOR_PAIR_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(DWORD_ARRAY_ELEMENT, |_, a| (a[0] + a[1] * 4).into_ret());
        // The scale doubles its argument.
        e.register(FLOAT_SCALE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]) * 2.0),
            ..Ret::default()
        });
        e
    }

    struct Fixture {
        this: u32,
        triples: Vec<u32>,
        nodes: Vec<u32>,
    }

    /// A target with one node per entry of `specs` = (valid, limit bits, record float bits).
    fn target(e: &mut Engine, specs: &[(u32, u32, u32)]) -> Fixture {
        let this = e.mem.alloc(0x40);
        let mut triples = Vec::new();
        let mut nodes = Vec::new();
        for (valid, limit, record_value) in specs {
            let first = e.mem.alloc(0x10);
            e.mem.set_u32(first, 0x0f00_b100);
            e.mem.set_u32(first + 8, *valid);
            let second = e.mem.alloc(0x10);
            e.mem.set_u32(second, 0x0f00_b600);
            e.mem.set_u32(second + 8, *limit);
            let third = e.mem.alloc(0x10);
            e.mem.set_u32(third, 0x0f00_bb00);
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record + 8, *record_value);
            e.mem.set_u32(third + 8, record);
            let triple = e.mem.alloc(0xc);
            e.mem.set_u32(triple, first);
            e.mem.set_u32(triple + 4, second);
            e.mem.set_u32(triple + 8, third);
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, triple);
            triples.push(triple);
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        if let Some(first) = nodes.first() {
            e.mem.set_u32(this + 0x34, *first);
        }
        Fixture {
            this,
            triples,
            nodes,
        }
    }

    #[test]
    fn at_00631370_takes_back_and_frees_triples_that_are_no_longer_valid() {
        let mut e = rig();
        let t = target(&mut e, &[(0, 0, 0), (1, 0, 0), (0, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0x1234u32, 0u8, 0u8]);
        assert_eq!(
            calls(&e, PAIR_TAKE_BACK),
            vec![vec![t.this, t.triples[0]], vec![t.this, t.triples[2]]]
        );
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![vec![t.triples[0]], vec![t.triples[2]]]
        );
        let removed: Vec<u32> = calls(&e, LIST_REMOVE_NODE).iter().map(|c| c[0]).collect();
        assert_eq!(removed, vec![t.this + 0x34, t.this + 0x34]);
        // The next node is fetched before each node is handled.
        let nexts = calls(&e, LIST_NEXT_NODE);
        assert_eq!(nexts.len(), 3);
        assert_eq!(nexts[0], vec![t.this + 0x34, t.nodes[0]]);
    }

    #[test]
    fn at_00631370_selects_by_the_first_or_third_pointer() {
        // Offset 0 is the triple's first pointer, offset 8 its third.
        for field in [0u32, 8] {
            let mut e = rig();
            let t = target(&mut e, &[(1, 0, 0), (1, 0, 0), (1, 0, 0)]);
            let wanted = e.mem.u32(t.triples[1] + field);
            e.call(0x0063_1370, &args![t.this, wanted, 0u8, 0u8]);
            assert_eq!(calls(&e, FREE_BLOCK), vec![vec![t.triples[1]]]);
            assert_eq!(calls(&e, PAIR_TAKE_BACK), vec![vec![t.this, t.triples[1]]]);
        }
    }

    #[test]
    fn at_00631370_a_null_match_selects_everything_and_keep_skips_the_take_back() {
        let mut e = rig();
        let t = target(&mut e, &[(1, 0, 0), (1, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0u32, 0u8, 1u8]);
        assert!(calls(&e, PAIR_TAKE_BACK).is_empty());
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![vec![t.triples[0]], vec![t.triples[1]]]
        );
        let mut e = rig();
        let t = target(&mut e, &[(1, 0, 0), (1, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0u32, 0u8, 0u8]);
        assert_eq!(calls(&e, PAIR_TAKE_BACK).len(), 2);
    }

    #[test]
    fn at_00631370_the_distance_test_compares_the_scaled_value_with_the_limit() {
        let mut e = rig();
        // The scale doubles: record 1.0 -> 2.0. Limit 3.0: below, selected. Limit 2.0: equal,
        // not selected. Limit 1.0: above, not selected. NaN limit: not selected.
        let specs = [
            (1, 3.0f32.to_bits(), 1.0f32.to_bits()),
            (1, 2.0f32.to_bits(), 1.0f32.to_bits()),
            (1, 1.0f32.to_bits(), 1.0f32.to_bits()),
            (1, f32::NAN.to_bits(), 1.0f32.to_bits()),
        ];
        let t = target(&mut e, &specs);
        // With the distance flag the match pointer is ignored.
        e.call(0x0063_1370, &args![t.this, 0x7777u32, 1u8, 0u8]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![t.triples[0]]]);
        assert_eq!(calls(&e, VECTOR_PAIR_CONSTRUCTOR).len(), 4);
        assert_eq!(calls(&e, FLOAT_SCALE).len(), 4);
    }

    #[test]
    fn at_00631370_a_nan_record_value_is_not_below_any_limit() {
        let mut e = rig();
        let t = target(&mut e, &[(1, 3.0f32.to_bits(), f32::NAN.to_bits())]);
        e.call(0x0063_1370, &args![t.this, 0u32, 1u8, 0u8]);
        assert!(calls(&e, FREE_BLOCK).is_empty());
    }

    #[test]
    fn at_00631370_an_empty_list_does_nothing() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x0063_1370, &args![this, 0u32, 0u8, 0u8]);
        assert_eq!(calls(&e, LIST_FIRST_NODE), vec![vec![this + 0x34]]);
        assert!(calls(&e, LIST_NEXT_NODE).is_empty());
    }
}
