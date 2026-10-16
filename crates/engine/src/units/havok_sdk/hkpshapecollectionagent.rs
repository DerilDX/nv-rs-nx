//! `Havok SDK/hkpshapecollectionagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpworld::{profile_record, PROFILE_END};

/// The profiling label `"TtShapeCollection"`.
const LABEL_SHAPE_COLLECTION: u32 = 0x010c_f300;

/// Size of the buffer a shape collection builds a child shape in (0x200 bytes on the stack).
const CHILD_SHAPE_BUFFER: u32 = 0x200;

/// Offset of the table of linear-cast functions inside the dispatcher object, and the size of
/// one table entry (five words).
const DISPATCH_TABLE: u32 = 0x9ac;
const DISPATCH_ENTRY: u32 = 0x14;

// Translated from 00d43aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeCollectionAgent::linearCast` (Xbox PDB), between profiling records
/// `"TtShapeCollection"` and the end record. `entry` (a collision entry: object at +0, a word
/// at +8) provides the shape collection by its virtual method at slot 0x10. For every
/// `(key, agent)` pair of the array at `this + 0xc` (count at `this + 0x10`), first to last,
/// the collection builds the child shape for `key` in a 0x200-byte local buffer (slot 0x14:
/// `(key, buffer)`), and the pair's agent gets its virtual method at slot 0x14 called with a
/// 16-byte record (child shape, `key`, the word at `entry + 8`, `entry`) and the four remaining
/// words.
pub fn hkp_shape_collection_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    entry: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(entry.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(entry.addr() + 8);
    let mut remaining = e.mem.i32(this.addr() + 0x10).wrapping_sub(1);
    let mut pair = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0x20 + CHILD_SHAPE_BUFFER, |e, frame| {
        let record = frame.addr();
        let buffer = frame.addr() + 0x20;
        while remaining >= 0 {
            let key = e.mem.u32(pair);
            let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
            e.mem.set_u32(record, child);
            e.mem.set_u32(record + 4, key);
            e.mem.set_u32(record + 8, entry_word);
            e.mem.set_u32(record + 12, entry.addr());
            let agent = e.mem.u32(pair + 4);
            e.vcall(agent, 0x14, &args![record, a, b, c, d]);
            pair = pair.wrapping_add(8);
            remaining -= 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d43b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, five stack words, no result; the engine map files it under
/// `hkpshapecollectionagent.cpp`). Between profiling records `"TtShapeCollection"` and the
/// end record, walks the keys of the shape collection that `first` provides (virtual method at
/// slot 0x10 of the object at `first + 0`) with its slots 8 (first key) and 0xC (next key) until
/// -1. For each key the filter object embedded at `[third + 0xc] + 0xc` is asked (its slot 4,
/// `(&answer, third, second, first, collection, key)`, answering through the returned pointer)
/// whether the pair is enabled; if it is, the collection builds the child shape for the key in
/// a local buffer (slot 0x14), and the function found in the dispatcher table
/// `[third] + 0x9ac + 0x14 * index` is called (cdecl) with a 16-byte record (child shape, key,
/// the word at `first + 8`, `first`) and `second`, `third`, `fourth`, `fifth`. The table index
/// is the byte at `[third] + 0x20 * (child shape's byte-word at +0xc + 0xd) + [[second] + 0xc]`.
pub fn fn_00d43b90(e: &mut Engine, first: Ptr, second: Ptr, third: Ptr, fourth: u32, fifth: u32) {
    profile_record(e, LABEL_SHAPE_COLLECTION);
    let owner = e.mem.u32(first.addr());
    let collection = e.vcall(owner, 0x10, &args![]).u32();
    let entry_word = e.mem.u32(first.addr() + 8);
    let second_object = e.mem.u32(second.addr());
    let selector = e.mem.u32(second_object + 0xc);
    e.with_stack(0x20 + 0x10 + CHILD_SHAPE_BUFFER, |e, frame| {
        let answer = frame.addr();
        let record = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(collection, 8, &args![]).u32();
        while key != u32::MAX {
            let filter_owner = e.mem.u32(third.addr() + 0xc);
            let filter = filter_owner + 0xc;
            let returned = e
                .vcall(
                    filter,
                    4,
                    &args![answer, third, second, first, collection, key],
                )
                .u32();
            if e.mem.u8(returned) != 0 {
                let child = e.vcall(collection, 0x14, &args![key, buffer]).u32();
                e.mem.set_u32(record, child);
                e.mem.set_u32(record + 4, key);
                e.mem.set_u32(record + 8, entry_word);
                e.mem.set_u32(record + 12, first.addr());
                let table = e.mem.u32(third.addr());
                let type_word = e.mem.u32(child.wrapping_add(0xc));
                let index_address = type_word
                    .wrapping_add(0xd)
                    .wrapping_shl(5)
                    .wrapping_add(table)
                    .wrapping_add(selector);
                let index = u32::from(e.mem.u8(index_address));
                let function = e
                    .mem
                    .u32(table + DISPATCH_TABLE + index.wrapping_mul(DISPATCH_ENTRY));
                e.call(function, &args![record, second, third, fourth, fifth]);
            }
            key = e.vcall(collection, 0xc, &args![key]).u32();
        }
    });
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d43aa0,
            hkp_shape_collection_agent_linear_cast(Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d43b90, fn_00d43b90(Ptr, Ptr, Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::hkpworld::profiling_support::{labels, profiling_engine};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OWNER_SLOT_10: u32 = 0x0900_0110;
    const COLLECTION_FIRST_KEY: u32 = 0x0900_0108;
    const COLLECTION_NEXT_KEY: u32 = 0x0900_010c;
    const COLLECTION_CHILD: u32 = 0x0900_0114;
    const AGENT_LINEAR_CAST: u32 = 0x0900_0214;
    const FILTER_SLOT_4: u32 = 0x0900_0304;
    const DISPATCHED: u32 = 0x0900_0400;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// An object with the given vtable slots.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    /// A collection of the keys `keys` (the next-key double walks them, ending with -1).
    fn collection(e: &mut Engine, keys: &'static [u32], log: &Log) -> u32 {
        e.register_double(COLLECTION_FIRST_KEY, move |_, _| Ret {
            eax: keys.first().copied().unwrap_or(u32::MAX),
            ..Ret::default()
        });
        e.register_double(COLLECTION_NEXT_KEY, move |_, a| {
            let at = keys.iter().position(|k| *k == a[1]).unwrap();
            Ret {
                eax: keys.get(at + 1).copied().unwrap_or(u32::MAX),
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(COLLECTION_CHILD, move |_, a| {
            sink.borrow_mut().push(("child", vec![a[1], a[2]]));
            // The child shape is a block whose +0xc word is the key (the type).
            Ret {
                eax: a[2] + 0x100,
                ..Ret::default()
            }
        });
        object(
            e,
            8,
            &[
                (8, COLLECTION_FIRST_KEY),
                (0xc, COLLECTION_NEXT_KEY),
                (0x14, COLLECTION_CHILD),
            ],
        )
    }

    fn entry_with(e: &mut Engine, collection: u32) -> Ptr {
        let owner = object(e, 8, &[(0x10, OWNER_SLOT_10)]);
        e.register_double(OWNER_SLOT_10, move |_, _| Ret {
            eax: collection,
            ..Ret::default()
        });
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, owner);
        e.mem.set_u32(entry + 8, 0xe8e8);
        Ptr::new(entry)
    }

    #[test]
    fn linear_cast_visits_every_pair_inside_profiling_records() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let sink = log.clone();
        e.register_double(AGENT_LINEAR_CAST, move |e, a| {
            let record = (0..4).map(|i| e.mem.u32(a[1] + 4 * i)).collect::<Vec<_>>();
            let mut words = vec![a[0]];
            words.extend(record);
            words.extend(&a[2..]);
            sink.borrow_mut().push(("agent", words));
            Ret::default()
        });
        let agent_a = object(&mut e, 8, &[(0x14, AGENT_LINEAR_CAST)]);
        let agent_b = object(&mut e, 8, &[(0x14, AGENT_LINEAR_CAST)]);
        let this = e.mem.alloc(0x20);
        let pairs = e.mem.alloc(16);
        for (i, (key, agent)) in [(5u32, agent_a), (9, agent_b)].into_iter().enumerate() {
            e.mem.set_u32(pairs + 8 * i as u32, key);
            e.mem.set_u32(pairs + 8 * i as u32 + 4, agent);
        }
        e.mem.set_u32(this + 0xc, pairs);
        e.mem.set_i32(this + 0x10, 2);
        e.call(0x00d43aa0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        let log = log.borrow();
        let agents: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(kind, _)| *kind == "agent")
            .map(|(_, words)| words)
            .collect();
        assert_eq!(agents.len(), 2);
        // Agent, record (child, key, entry word, entry), then the four words.
        assert_eq!(agents[0][0], agent_a);
        assert_eq!(agents[0][2], 5);
        assert_eq!(agents[0][3..], [0xe8e8, entry.addr(), 1, 2, 3, 4]);
        assert_eq!(agents[1][0], agent_b);
        assert_eq!(agents[1][2], 9);
        // The child shapes were built in a buffer, in pair order.
        let children: Vec<u32> = log
            .iter()
            .filter(|(kind, _)| *kind == "child")
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(children, vec![5, 9]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    #[test]
    fn linear_cast_with_no_pairs_only_writes_the_records() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[], &log);
        let entry = entry_with(&mut e, collection);
        let this = e.mem.alloc(0x20);
        e.mem.set_i32(this + 0x10, 0);
        e.call(0x00d43aa0, &args![this, entry, 1u32, 2u32, 3u32, 4u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }

    /// Scene for `fn_00d43b90`: keys 4, 6 and 8; the filter enables 4 and 8 only. `second`'s
    /// first word points to an object whose +0xc word is the selector 2; the dispatcher object
    /// (`third`'s first word) holds a table of index bytes and one function per index.
    #[test]
    fn the_filtered_keys_are_dispatched_through_the_table() {
        let (mut e, stream, start) = profiling_engine(4);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let collection = collection(&mut e, &[4, 6, 8], &log);
        let first = entry_with(&mut e, collection);

        // `second`: [second] -> object with selector 2 at +0xc.
        let second_object = e.mem.alloc(0x20);
        e.mem.set_u32(second_object + 0xc, 2);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, second_object);

        // The dispatcher (`[third]`): index byte for type t at table + 0x20 * (t + 0xd) + 2.
        let dispatcher = e.mem.alloc(0x1000);
        // The child shape of key k is at buffer + 0x100; its +0xc word is set below in the
        // child double through the buffer, so give the type word its value by key.
        e.register_double(COLLECTION_CHILD, move |e, a| {
            let child = a[2] + 0x100;
            e.mem.set_u32(child + 0xc, a[1] - 4);
            Ret {
                eax: child,
                ..Ret::default()
            }
        });
        // Key 4 has type 0 and key 8 has type 4: index bytes 1 and 2.
        for (type_word, index) in [(0u32, 1u8), (4, 2)] {
            e.mem
                .set_u8(dispatcher + 0x20 * (type_word + 0xd) + 2, index);
        }
        // The functions of index 1 and 2.
        for (index, target) in [(1u32, DISPATCHED), (2, DISPATCHED + 4)] {
            e.mem
                .set_u32(dispatcher + DISPATCH_TABLE + 0x14 * index, target);
        }
        let seen = Rc::new(RefCell::new(vec![]));
        for target in [DISPATCHED, DISPATCHED + 4] {
            let sink = seen.clone();
            e.register_double(target, move |e, a| {
                let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
                sink.borrow_mut().push((target, record, a[1..].to_vec()));
                Ret::default()
            });
        }
        let filter_owner = e.mem.alloc(0x20);
        let filter_table = e.mem.alloc(0x20);
        e.mem.set_u32(filter_table + 4, FILTER_SLOT_4);
        e.mem.set_u32(filter_owner + 0xc, filter_table);
        let third = e.mem.alloc(0x20);
        e.mem.set_u32(third, dispatcher);
        e.mem.set_u32(third + 0xc, filter_owner);
        let sink = log.clone();
        e.register_double(FILTER_SLOT_4, move |e, a| {
            // The answer goes through the hidden pointer, which is returned.
            let enabled = a[6] != 6;
            e.mem.set_u8(a[1], u8::from(enabled));
            sink.borrow_mut().push(("filter", a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(0x00d43b90, &args![first, second, third, 0xaau32, 0xbbu32]);

        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        // Key 4: the record is (child, key, entry word, first); the stack words follow.
        let child_of = |record: &Vec<u32>| record[0];
        assert_eq!(seen[0].0, DISPATCHED);
        assert_eq!(seen[0].1[1..], [4, 0xe8e8, first.addr()]);
        assert_eq!(seen[0].2, vec![second, third, 0xaa, 0xbb]);
        assert_eq!(seen[1].0, DISPATCHED + 4);
        assert_eq!(seen[1].1[1..], [8, 0xe8e8, first.addr()]);
        assert_ne!(child_of(&seen[0].1), 0);
        // The filter saw every key with (collection, key) last.
        let log = log.borrow();
        let asked: Vec<u32> = log
            .iter()
            .filter(|(kind, _)| *kind == "filter")
            .map(|(_, words)| words[6])
            .collect();
        assert_eq!(asked, vec![4, 6, 8]);
        assert_eq!(
            labels(&e, stream, start),
            vec![LABEL_SHAPE_COLLECTION, PROFILE_END]
        );
    }
}
