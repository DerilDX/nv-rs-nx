//! `fallout/ai/modifierlist.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::run_00936ef0::ModifierList;

// Translated from 009373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this`. The engine map has no name for it; the quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_009373f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr())
}

// Translated from 00937b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `009370b0` (Ghidra name
/// `ModifierList::DeleteAllModifiers`), which deletes every item of the list and the
/// fast-modifier collection at `+0xc`. The engine map names this address
/// `ModifierList::Revert` from the call graph only (row `cg`), which this translation does
/// not confirm, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `009370b0`'s disassembly ends on
/// the store `MOV dword ptr [ECX+0xc],0x0` with no result (Ghidra types it void).
pub fn fn_00937b30(e: &mut Engine, this: Ptr) {
    e.call(0x0093_70b0, &args![this]);
}

/// `ModifierList::DeleteAllModifiers` (`009370b0`; `thiscall`, no stack words).
const DELETE_ALL_MODIFIERS: u32 = 0x0093_70b0;
/// `00401000` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x0040_1000;
/// `00937e10` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): constructs, in the
/// block it is given, a copy of the collection at its argument (the `pFastModifiers` word of
/// a `ModifierList`): a pointer array sized by the word at `0x011e0750`, each used slot a
/// copy made with `00936fd0`. Returns `this`.
const FAST_MODIFIERS_COPY: u32 = 0x0093_7e10;
/// `006815c0` (`thiscall`, plain `RET`): returns its `this` (the address of the list node's
/// item word).
const NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
/// `00936fd0` (`thiscall`, `RET 4`, unit `fallout/ai` run, translated in `run_00936ef0.rs`):
/// constructs, in the block it is given, a copy of the modifier at its argument.
const MODIFIER_COPY: u32 = 0x0093_6fd0;
/// `00937380` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): takes a modifier
/// into the list.
const ADD_MODIFIER: u32 = 0x0093_7380;
/// `00726070` (`thiscall`, plain `RET`): returns the word at `+4` of its `this` (the next
/// node of a `BSSimpleList`).
const NEXT_NODE: u32 = 0x0072_6070;

// Translated from 00937160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00937160 - the C++ exception unwinding states (FS:[0] frame, handler 00f177b6) are not translated
/// Unnamed in the engine map (unit `modifierlist.cpp`; `thiscall`, `RET 4`): makes the
/// `ModifierList` `this` a copy of `source` (a `ModifierList`, Xbox PDB layout: a
/// `BSSimpleList<Modifier *>` head, `pFastModifiers` at `+0xc`).
///
/// - Deletes everything `this` holds first (`009370b0`).
/// - A null `source` ends there.
/// - When `source`'s `pFastModifiers` is set, allocates 4 bytes (`00401000`) and, when the
///   block came back, builds the copy in it with `00937e10(block, source->pFastModifiers)`;
///   the result (0 for a failed allocation) becomes `this`'s `pFastModifiers`.
/// - Then walks the list nodes from `source` itself: the node's item word (the address
///   `006815c0` returns, so the node's first word) must be non-zero, otherwise the walk ends.
///   For each node, allocates 8 bytes, copies the item into it with `00936fd0` (0 for a failed
///   allocation) and hands the copy to `00937380` on `this`; the next node is `00726070`.
pub fn fn_00937160(e: &mut Engine, this: Ptr<ModifierList>, source: Ptr<ModifierList>) {
    e.call(DELETE_ALL_MODIFIERS, &args![this]);
    if source.addr() == 0 {
        return;
    }
    if e.get(source, ModifierList::pFastModifiers) != 0 {
        let block = e.call(ALLOCATE, &args![4u32]).u32();
        let collection = if block != 0 {
            let fast_modifiers = e.get(source, ModifierList::pFastModifiers);
            e.call(FAST_MODIFIERS_COPY, &args![block, fast_modifiers])
                .u32()
        } else {
            0
        };
        e.set(this, ModifierList::pFastModifiers, collection);
    }
    let mut node = source.addr();
    while node != 0 {
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(NODE_ITEM_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(item_address);
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let modifier = if block != 0 {
            e.call(MODIFIER_COPY, &args![block, item]).u32()
        } else {
            0
        };
        e.call(ADD_MODIFIER, &args![this, modifier]);
        node = e.call(NEXT_NODE, &args![node]).u32();
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0093_7160,
            fn_00937160(Ptr<ModifierList>, Ptr<ModifierList>)
        ),
        entry!(0x0093_7b30, fn_00937b30(Ptr)),
        entry!(0x009373f0, fn_009373f0(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_first_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x1122_33a5);
        assert_eq!(e.call(0x0093_73f0, &[object]).u32() & 0xff, 0xa5);
    }
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
    fn fwd_a03_00937b30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0093_70b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0093_7b30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0093_70b0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod copy_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const FIXED_BLOCK: u32 = 0x0e00_0000;

    /// Doubles for every callee outside this file. `allocation` is what `00401000` answers
    /// (0 for a failure); `00937e10` answers its block, `00936fd0` its block plus 1 and
    /// the next node of `00726070` comes from the word at `+4`. `00401000` answers
    /// `FIXED_BLOCK` and then `FIXED_BLOCK + 0x10` per call when `allocation` is non-zero.
    fn fixture(allocation: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let calls = std::rc::Rc::new(std::cell::Cell::new(0u32));
        for address in [
            DELETE_ALL_MODIFIERS,
            ALLOCATE,
            FAST_MODIFIERS_COPY,
            NODE_ITEM_ADDRESS,
            MODIFIER_COPY,
            ADD_MODIFIER,
            NEXT_NODE,
        ] {
            let log = log.clone();
            let calls = calls.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let eax = match address {
                    ALLOCATE if allocation == 0 => 0,
                    ALLOCATE => {
                        calls.set(calls.get() + 1);
                        FIXED_BLOCK + 0x10 * calls.get()
                    }
                    FAST_MODIFIERS_COPY => a[0],
                    MODIFIER_COPY => a[0] + 1,
                    NODE_ITEM_ADDRESS => a[0],
                    NEXT_NODE => e.mem.u32(a[0] + 4),
                    _ => 0,
                };
                Ret {
                    eax,
                    ..Ret::default()
                }
            });
        }
        (e, log)
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn a_null_source_only_deletes_what_this_holds() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        e.call(0x0093_7160, &args![this, 0u32]);
        assert_eq!(*log.borrow(), [(DELETE_ALL_MODIFIERS, vec![this])]);
    }

    #[test]
    fn copies_the_fast_modifiers_and_every_item() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0xbbb0);
        e.mem.set_u32(second + 4, 0);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xaaa0);
        e.mem.set_u32(source + 4, second);
        e.mem.set_u32(source + 0xc, 0x5000);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, DELETE_ALL_MODIFIERS), [vec![this]]);
        assert_eq!(calls_to(&log, ALLOCATE), [vec![4], vec![8], vec![8]]);
        assert_eq!(
            calls_to(&log, FAST_MODIFIERS_COPY),
            [vec![FIXED_BLOCK + 0x10, 0x5000]]
        );
        assert_eq!(e.mem.u32(this + 0xc), FIXED_BLOCK + 0x10);
        assert_eq!(
            calls_to(&log, MODIFIER_COPY),
            [
                vec![FIXED_BLOCK + 0x20, 0xaaa0],
                vec![FIXED_BLOCK + 0x30, 0xbbb0]
            ]
        );
        assert_eq!(
            calls_to(&log, ADD_MODIFIER),
            [
                vec![this, FIXED_BLOCK + 0x21],
                vec![this, FIXED_BLOCK + 0x31]
            ]
        );
        assert_eq!(calls_to(&log, NEXT_NODE), [vec![source], vec![second]]);
    }

    #[test]
    fn without_fast_modifiers_only_the_items_are_copied() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xccc0);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, ALLOCATE), [vec![8]]);
        assert!(calls_to(&log, FAST_MODIFIERS_COPY).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0x7777);
        assert_eq!(calls_to(&log, ADD_MODIFIER).len(), 1);
    }

    #[test]
    fn an_empty_head_ends_the_walk() {
        let (mut e, log) = fixture(1);
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        e.call(0x0093_7160, &args![this, source]);
        assert_eq!(calls_to(&log, NODE_ITEM_ADDRESS), [vec![source]]);
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert!(calls_to(&log, ADD_MODIFIER).is_empty());
    }

    #[test]
    fn a_failed_allocation_hands_over_a_null_copy() {
        let (mut e, log) = fixture(0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x7777);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0xccc0);
        e.mem.set_u32(source + 0xc, 0x5000);
        e.call(0x0093_7160, &args![this, source]);
        assert!(calls_to(&log, FAST_MODIFIERS_COPY).is_empty());
        assert!(calls_to(&log, MODIFIER_COPY).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&log, ADD_MODIFIER), [vec![this, 0]]);
    }
}
