//! `Havok SDK/hkpcachingshapephantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d4d310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_ce60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d4d310(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_ce60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`, cdecl `(allocator, array, elementSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// A halving factor stored as a double in the exe image (`01011588`).
const HALF: u32 = 0x0101_1588;
/// A small float constant stored in the exe image (`0104da4c`); the code stores it in a local
/// that is overwritten before anything reads it.
const SMALL_FLOAT: u32 = 0x0104_da4c;

/// The phantom's list of overlapping collidables: an array at `this + 0x160` of 8-byte
/// entries `{agent, collidable}` with its count at `this + 0x164`.
const LIST: u32 = 0x160;
/// Set to 1 whenever the list changes.
const LIST_CHANGED: u32 = 0x16c;

// Translated from 00d4cab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::setPositionAndLinearCast` (Xbox PDB). `this` is ECX; `from` and
/// `to` point to 16-byte vectors; `arg3` and `arg4` are handed unchanged to every agent.
/// Stores `from` at `this + 0xe0`; asks the object at `this + 0x10` (vtable offset `0x1c`,
/// arguments `this + 0xb0`, the float `0.5 * owner->0x78 record float +8 + to[5]`, a local
/// 32-byte bounding box) for the bounding box, grows it by the displacement `to - from` (the
/// lane-wise minimum and maximum against zero added to its corners) and passes it to
/// `00d07ec0(this, box)`. It then builds a local cast-input record from the owner's `0x78`
/// record (words 0 to 5, 16-byte blocks at `+0x20`, `+0x30` and `+0x40`, the word at `+0x70`),
/// the displacement and its length, and the floats `to[5]` and `to[4]`, and calls, for each
/// entry of the overlapping list from the last to the first, the agent's vtable offset `0x14`
/// with `(this + 0x10, collidable, record, arg3, arg4)`. The record's gaps keep whatever the
/// stack held in the exe; they are zero here.
pub fn hkp_caching_shape_phantom_set_position_and_linear_cast(
    e: &mut Engine,
    this: Ptr,
    from: Ptr,
    to: Ptr,
    arg3: u32,
    arg4: u32,
) {
    let this = this.addr();
    let (from, to) = (from.addr(), to.addr());
    e.with_stack(0xb0, |e, local| {
        let b = local.addr();
        let lane = |e: &Engine, address: u32| -> [f32; 4] {
            [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
        };
        let put = |e: &mut Engine, address: u32, value: [f32; 4]| {
            for (i, v) in value.into_iter().enumerate() {
                e.mem.set_u32(address + 4 * i as u32, v.to_bits());
            }
        };
        let start = lane(e, from);
        put(e, this + 0xe0, start);
        let shape_owner = e.mem.u32(this + 8);
        let record = e.mem.u32(shape_owner + 0x78);
        let owner = e.mem.u32(this + 0x10);
        let half = (f64::from(e.mem.f32(record + 8)) * e.mem.f64(HALF)) as f32;
        let radius = x87_quiet(half + e.mem.f32(to + 0x14));
        e.vcall(owner, 0x1c, &args![this + 0xb0, radius.to_bits(), b + 0x10]);
        let end = lane(e, to);
        let mut delta = [0.0f32; 4];
        let mut low = [0.0f32; 4];
        let mut high = [0.0f32; 4];
        let bounds_min = lane(e, b + 0x10);
        let bounds_max = lane(e, b + 0x20);
        for i in 0..4 {
            delta[i] = end[i] - start[i];
            let zero = 0.0f32;
            let lo = if zero < delta[i] { zero } else { delta[i] };
            let hi = if zero > delta[i] { zero } else { delta[i] };
            low[i] = lo + bounds_min[i];
            high[i] = hi + bounds_max[i];
        }
        put(e, b + 0xa0, delta);
        put(e, b + 0x10, low);
        put(e, b + 0x20, high);
        e.call(0x00d0_7ec0, &args![this, b + 0x10]);
        let small = x87_quiet_bits(e.mem.u32(SMALL_FLOAT));
        e.mem.set_u32(b + 0x90, small);
        e.mem.set_u32(b + 0x34, 0);
        let record = e.mem.u32(e.mem.u32(this + 8) + 0x78);
        for k in 0..6u32 {
            let word = e.mem.u32(record + 4 * k);
            e.mem.set_u32(b + 0x30 + 4 * k, word);
        }
        for (to_offset, from_offset) in [(0x50, 0x20), (0x60, 0x30), (0x70, 0x40)] {
            let block = e.mem.bytes(record + from_offset, 16);
            e.mem.write(b + to_offset, &block);
        }
        let tail = e.mem.u32(record + 0x70);
        put(e, b + 0x80, delta);
        let squares = delta.map(|v| v * v);
        let sum = squares[2] + (squares[1] + squares[0]);
        e.mem.set_u32(b + 0x98, tail);
        let length = sum.sqrt();
        e.mem.set_u32(b + 0x94, x87_quiet_bits(length.to_bits()));
        let z = e.mem.u32(to + 0x14);
        e.mem.set_u32(b + 0x38, x87_quiet_bits(z));
        let w = e.mem.u32(to + 0x10);
        let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1) as i32;
        e.mem.set_u32(b + 0x90, x87_quiet_bits(w));
        while index >= 0 {
            let entry = e.mem.u32(this + LIST) + (index as u32) * 8;
            let agent = e.mem.u32(entry);
            let collidable = e.mem.u32(entry + 4);
            e.vcall(
                agent,
                0x14,
                &args![this + 0x10, collidable, b + 0x30, arg3, arg4],
            );
            index -= 1;
        }
    });
}

// Translated from 00d4cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::removeOverlappingCollidable` (Xbox PDB). `this` is ECX. Does
/// nothing when the collidable's first word is zero. Otherwise it searches the overlapping
/// list from the last entry down for the collidable; either way it runs
/// `00ca9520(this, collidable, flag)`, where `flag` is the collidable pointer with its low
/// byte replaced by 1 (found) or 0 (not found). When found, it also calls the entry's agent
/// (vtable offset `0x1c`) with a 24-byte local object whose vtable is `010ccebc` (RTTI name
/// `hkpConstraintOwner`) and whose 16-bit word at +6 is 1, removes the entry by moving the
/// last entry into its place and sets the list-changed byte.
pub fn hkp_caching_shape_phantom_remove_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: Ptr,
) {
    let this = this.addr();
    let c = collidable.addr();
    if e.mem.u32(c) == 0 {
        return;
    }
    let mut index = e.mem.u32(this + LIST + 4).wrapping_sub(1) as i32;
    let mut found = false;
    while index >= 0 {
        let entry = e.mem.u32(this + LIST) + (index as u32) * 8;
        if e.mem.u32(entry + 4) == c {
            found = true;
            break;
        }
        index -= 1;
    }
    if !found {
        e.call(0x00ca_9520, &args![this, c, c & 0xffff_ff00]);
        return;
    }
    e.call(0x00ca_9520, &args![this, c, (c & 0xffff_ff00) | 1]);
    let agent = e.mem.u32(e.mem.u32(this + LIST) + (index as u32) * 8);
    if agent != 0 {
        e.with_stack(0x18, |e, local| {
            e.mem.set_u32(local.addr(), 0x010c_cebc);
            e.mem.set_u16(local.addr() + 6, 1);
            e.vcall(agent, 0x1c, &args![local]);
        });
    }
    let count = e.mem.u32(this + LIST + 4).wrapping_sub(1);
    e.mem.set_u32(this + LIST + 4, count);
    if count != index as u32 {
        let data = e.mem.u32(this + LIST);
        let last = e.mem.u32(data + count * 8);
        let last_collidable = e.mem.u32(data + count * 8 + 4);
        e.mem.set_u32(data + (index as u32) * 8, last);
        e.mem
            .set_u32(data + (index as u32) * 8 + 4, last_collidable);
    }
    e.mem.set_u8(this + LIST_CHANGED, 1);
}

// Translated from 00d4cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCachingShapePhantom::addOverlappingCollidable` (Xbox PDB). `this` is ECX. Does nothing
/// when the collidable's first word is zero or when `00ca94c0(this, collidable)` returns
/// non-zero. Otherwise it appends a slot to the overlapping list (`_reserveMore` with 8-byte
/// elements when the count equals the capacity), copies the owner's `0x78` record (80 bytes)
/// into a local, lets `00cfa7d0(local, record)` set it up, picks the agent-creation function
/// from the table the local's first word points to (index byte at
/// `table + 0x1a0 + (this->0x10 word +0xc << 5) + collidable->first word +0xc`, five words
/// per function at `table + 0x9a0`) and calls it as `(this + 0x10, collidable, local, 0)`,
/// storing its result and the collidable in the new slot. Sets the list-changed byte and frees
/// a 0x200-byte block (through the allocator's vtable offset 8) if the local's word at
/// `+0x84` ends up non-zero. The local frame is 0x94 bytes; the copied record is at its `+0x20`.
pub fn hkp_caching_shape_phantom_add_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: Ptr,
) {
    let this = this.addr();
    let c = collidable.addr();
    if e.mem.u32(c) == 0 {
        return;
    }
    if e.call(0x00ca_94c0, &args![this, c]).u32() != 0 {
        return;
    }
    let list = this + LIST;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(list + 8) & 0x3fff_ffff;
    if e.mem.u32(list + 4) == capacity {
        let heap = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![heap, list, 8u32]);
    }
    let count = e.mem.u32(list + 4);
    let slot = e.mem.u32(list) + count * 8;
    e.mem.set_u32(list + 4, count + 1);
    let record = e.mem.u32(e.mem.u32(this + 8) + 0x78);
    e.with_stack(0xa0, |e, local| {
        let b = local.addr();
        let words = e.mem.bytes(record, 0x50);
        e.mem.write(b + 0x20, &words);
        e.call(0x00cf_a7d0, &args![b + 0x20, record]);
        let table = e.mem.u32(b + 0x20);
        e.mem.set_u32(b + 0x34, 0);
        let first = e.mem.u32(c);
        let other = e.mem.u32(this + 0x10);
        e.mem.set_u32(b + 0x80, table.wrapping_add(0x1c60));
        let row = e.mem.u32(other + 0xc) << 5;
        let column = e.mem.u32(first + 0xc);
        let index = u32::from(
            e.mem.u8(table
                .wrapping_add(0x1a0)
                .wrapping_add(row)
                .wrapping_add(column)),
        );
        let function = e
            .mem
            .u32(table.wrapping_add(index * 20).wrapping_add(0x9a0));
        let agent = e
            .call(function, &args![this + 0x10, c, b + 0x20, 0u32])
            .u32();
        e.mem.set_u32(slot, agent);
        e.mem.set_u32(slot + 4, c);
        e.mem.set_u8(this + LIST_CHANGED, 1);
        let buffer = e.mem.u32(b + 0x84);
        if buffer != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(allocator, 8, &args![buffer, 0x200u32]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_d310, deleting_destructor_00d4d310(Ptr, u8) -> Ptr),
        entry!(
            0x00d4_cab0,
            hkp_caching_shape_phantom_set_position_and_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d4_cca0,
            hkp_caching_shape_phantom_remove_overlapping_collidable(Ptr, Ptr)
        ),
        entry!(
            0x00d4_cf10,
            hkp_caching_shape_phantom_add_overlapping_collidable(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d4d310_frees_only_when_bit_zero_is_set() {
        check(0x00d4_d310, 0x00d4_ce60);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn log_double(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn vtable_with(e: &mut Engine, offset: u32, target: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + offset, target);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn remove_moves_the_last_entry_into_the_gap() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_9520, 0);
        log_double(&mut e, &log, 0x0900_001c, 0);
        let first = vtable_with(&mut e, 0x1c, 0x0900_001c);
        let last = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x180);
        let data = e.mem.alloc(0x20);
        let c1 = e.mem.alloc(0x10);
        let c2 = e.mem.alloc(0x10);
        e.mem.set_u32(c1, 1);
        e.mem.set_u32(c2, 1);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, c1);
        e.mem.set_u32(data + 8, last);
        e.mem.set_u32(data + 12, c2);
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x164, 2);
        e.call(0x00d4_cca0, &args![this, c1]);
        assert_eq!(e.mem.u32(this + 0x164), 1);
        assert_eq!(e.mem.u32(data), last);
        assert_eq!(e.mem.u32(data + 4), c2);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        let calls = log.borrow();
        assert_eq!(calls[0].0, 0x00ca_9520);
        assert_eq!(calls[0].1, vec![this, c1, (c1 & 0xffff_ff00) | 1]);
        assert_eq!(calls[1].0, 0x0900_001c);
        assert_eq!(calls[1].1[0], first);
    }

    #[test]
    fn remove_of_an_unknown_collidable_only_notifies() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_9520, 0);
        let this = e.mem.alloc(0x180);
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, 1);
        e.call(0x00d4_cca0, &args![this, c]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0].1[2], c & 0xffff_ff00);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        // A collidable whose first word is zero is ignored entirely.
        let zero = e.mem.alloc(0x10);
        e.call(0x00d4_cca0, &args![this, zero]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn add_appends_an_entry_made_by_the_table_function() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_94c0, 0);
        log_double(&mut e, &log, 0x00c9_0940, 0);
        log_double(&mut e, &log, 0x0900_00a0, 0x5555);
        let table = e.mem.alloc(0x2000);
        let row = 3u32;
        let column = 5u32;
        e.mem.set_u8(table + 0x1a0 + (row << 5) + column, 2);
        e.mem.set_u32(table + 2 * 20 + 0x9a0, 0x0900_00a0);
        let sink = Rc::clone(&log);
        e.register_double(0x00cf_a7d0, move |e, a| {
            sink.borrow_mut().push((0x00cf_a7d0, a.to_vec()));
            e.mem.set_u32(a[0], table);
            Ret::default()
        });
        let heap = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x180);
        let owner = e.mem.alloc(0x100);
        let record = e.mem.alloc(0x100);
        e.mem.set_u32(record + 4, 0x77);
        e.mem.set_u32(owner + 0x78, record);
        e.mem.set_u32(this + 8, owner);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, row);
        e.mem.set_u32(this + 0x10, other);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x164, 1);
        e.mem.set_u32(this + 0x168, 1);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, column);
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, shape);
        e.call(0x00d4_cf10, &args![this, c]);
        assert_eq!(e.mem.u32(this + 0x164), 2);
        assert_eq!(e.mem.u32(data + 8), 0x5555);
        assert_eq!(e.mem.u32(data + 12), c);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        let calls = log.borrow();
        let reserve = calls.iter().find(|c| c.0 == 0x00c9_0940).unwrap();
        assert_eq!(reserve.1, vec![heap, this + 0x160, 8]);
        let made = calls.iter().find(|c| c.0 == 0x0900_00a0).unwrap();
        assert_eq!(made.1[0], this + 0x10);
        assert_eq!(made.1[1], c);
        assert_eq!(made.1[3], 0);
    }

    #[test]
    fn add_skips_a_zero_or_rejected_collidable() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        log_double(&mut e, &log, 0x00ca_94c0, 1);
        let this = e.mem.alloc(0x180);
        let zero = e.mem.alloc(0x10);
        e.call(0x00d4_cf10, &args![this, zero]);
        assert!(log.borrow().is_empty());
        let c = e.mem.alloc(0x10);
        e.mem.set_u32(c, 1);
        e.call(0x00d4_cf10, &args![this, c]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(this + 0x164), 0);
    }

    #[test]
    fn linear_cast_grows_the_box_and_calls_each_agent() {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        e.mem.set_f64(HALF, 0.5);
        let sink = Rc::clone(&log);
        e.register_double(0x00d0_7ec0, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|i| e.mem.u32(a[1] + 4 * i)));
            sink.borrow_mut().push((0x00d0_7ec0, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_001c, move |e, a| {
            sink.borrow_mut().push((0x0900_001c, a.to_vec()));
            let corners = [-1.0f32, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0];
            for (i, v) in corners.into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, v);
            }
            Ret::default()
        });
        log_double(&mut e, &log, 0x0900_0014, 0);
        let this = e.mem.alloc(0x180);
        let owner_object = vtable_with(&mut e, 0x1c, 0x0900_001c);
        e.mem.set_u32(this + 0x10, owner_object);
        let shape_owner = e.mem.alloc(0x100);
        let record = e.mem.alloc(0x100);
        e.mem.set_f32(record + 8, 4.0);
        e.mem.set_u32(record + 0x70, 0x99);
        e.mem.set_u32(shape_owner + 0x78, record);
        e.mem.set_u32(this + 8, shape_owner);
        let agent = vtable_with(&mut e, 0x14, 0x0900_0014);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, agent);
        e.mem.set_u32(list + 4, 0xc011);
        e.mem.set_u32(this + 0x160, list);
        e.mem.set_u32(this + 0x164, 1);
        let from = e.mem.alloc(0x10);
        let to = e.mem.alloc(0x20);
        for (i, v) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(from + 4 * i as u32, v);
        }
        for (i, v) in [4.0f32, 2.0, -1.0, 0.0, 0.25, 1.5].into_iter().enumerate() {
            e.mem.set_f32(to + 4 * i as u32, v);
        }
        e.call(0x00d4_cab0, &args![this, from, to, 7u32, 8u32]);
        assert_eq!(e.mem.f32(this + 0xe0), 1.0);
        assert_eq!(e.mem.f32(this + 0xe8), 3.0);
        let calls = log.borrow();
        let owner_call = calls.iter().find(|c| c.0 == 0x0900_001c).unwrap();
        assert_eq!(owner_call.1[1], this + 0xb0);
        // 0.5 * 4.0 + to[5]
        assert_eq!(f32::from_bits(owner_call.1[2]), 3.5);
        let boxed = calls.iter().find(|c| c.0 == 0x00d0_7ec0).unwrap();
        // displacement is (3, 0, -4, 0): the minimum adds -4 on z, the maximum 3 on x.
        let words: Vec<f32> = boxed.1[2..].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(words[0], -1.0);
        assert_eq!(words[2], -5.0);
        assert_eq!(words[4], 4.0);
        assert_eq!(words[6], 1.0);
        let agent_call = calls.iter().find(|c| c.0 == 0x0900_0014).unwrap();
        assert_eq!(agent_call.1[1..3], [this + 0x10, 0xc011]);
        assert_eq!(agent_call.1[4..], [7, 8]);
    }
}
