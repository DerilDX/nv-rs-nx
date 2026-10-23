//! `FaceGen/run_00ab0d50` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT, VECTOR_FIRST, VECTOR_LAST,
};
use crate::units::facegen::run_00aba3c0::{
    advance_by_address, erase_by_address, grown_capacity, max_or_one, resize_with_value,
    vector_capacity, vector_size, with_value_slot, ElementSize, ResizeShape, WordsArg,
    VECTOR_ALLOCATOR, VECTOR_END,
};

/// The vectors' "too long" throw (`0065f640`, no arguments; it does not
/// return in the game).
const THROW_LENGTH: u32 = 0x0065_f640;
/// The 6-byte element vector's helpers: allocation (`0065f9e0`, `cdecl`:
/// count, hint), the 4-word copies (`0065fde0` into new storage and into
/// the gap, `00ab2260` past the end; `cdecl`: first, last, destination,
/// allocator) and the backward move (`00ab2210`, `cdecl`: first, last,
/// end).
const ALLOCATE_6: u32 = 0x0065_f9e0;
const COPY_6: u32 = 0x0065_fde0;
const COPY_TAIL_6: u32 = 0x00ab_2260;
const MOVE_BACKWARD_6: u32 = 0x00ab_2210;
/// The 0x1C-byte element vector's helpers: allocation (`0065e670`), the
/// copies (`00660340` before the gap and to move the tail up, `0065ff90`
/// after it and past the end), the element destructor (`0065eb40`,
/// `cdecl`), the backward move (`00ab2380`, `cdecl`, six words), the
/// advance of an iterator (`00ad35e0`, `thiscall`: the count) and the
/// erase (`0065d740`, `thiscall`: result, first and last iterators).
const ALLOCATE_1C: u32 = 0x0065_e670;
const COPY_1C: u32 = 0x0066_0340;
const COPY_AFTER_1C: u32 = 0x0065_ff90;
const DESTRUCT_1C: u32 = 0x0065_eb40;
const MOVE_BACKWARD_1C: u32 = 0x00ab_2380;
const ADVANCE_1C: u32 = 0x00ad_35e0;
const ERASE_1C: u32 = 0x0065_d740;
/// The float vector inside a 0x1C-byte element (at +4): its copy
/// constructor (`0065fb10`, `thiscall`: the source), assignment
/// (`0065eed0`, `thiscall`: the source), element tidy (`0065f430`,
/// `thiscall`) and proxy free (`0065d6e0`, `thiscall`).
const FLOATS_COPY: u32 = 0x0065_fb10;
const FLOATS_ASSIGN: u32 = 0x0065_eed0;
const FLOATS_TIDY: u32 = 0x0065_f430;
const FLOATS_FREE_PROXY: u32 = 0x0065_d6e0;

/// A 6-byte element (a word and a `u16`) as the code copies it.
#[derive(Clone, Copy)]
struct Six(u32, u16);

impl Six {
    fn load(e: &Engine, address: u32) -> Self {
        Six(e.mem.u32(address), e.mem.u16(address + 4))
    }
    fn store(self, e: &mut Engine, address: u32) {
        e.mem.set_u32(address, self.0);
        e.mem.set_u16(address + 4, self.1);
    }
}

/// The insert-n reallocation: the new storage, the elements before `at`,
/// `count` copies, the rest, the old storage given back (after `retire`
/// walks or destroys its elements; `None` when the code has no such loop),
/// and the three pointers.
#[allow(clippy::too_many_arguments)]
pub(crate) fn reallocate(
    e: &mut Engine,
    vector: u32,
    at: u32,
    count: u32,
    value: u32,
    size: ElementSize,
    maximum: u32,
    allocate: u32,
    copy_before: u32,
    fill: fn(&mut Engine, u32, u32, u32, u32, u32, u32),
    copy_after: u32,
    retire: Option<fn(&mut Engine, u32)>,
) {
    let bytes = size.bytes();
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    let needed = vector_size(e, vector, size).wrapping_add(count);
    let new_capacity = grown_capacity(capacity, maximum, needed);
    let block = e.call(allocate, &args![new_capacity, 0u32]).u32();
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let mut cursor = e
        .call(copy_before, &args![first, at, block, allocator])
        .u32();
    fill(e, cursor, count, value, allocator, 0, 0);
    cursor = count.wrapping_mul(bytes).wrapping_add(cursor);
    let last = e.mem.u32(vector + VECTOR_LAST);
    e.call(copy_after, &args![at, last, cursor, allocator]);
    let new_size = vector_size(e, vector, size).wrapping_add(count);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    if first != 0 {
        if let Some(retire) = retire {
            let last = e.mem.u32(vector + VECTOR_LAST);
            let mut element = first;
            while element != last {
                retire(e, element);
                element = element.wrapping_add(bytes);
            }
        }
        let first = e.mem.u32(vector + VECTOR_FIRST);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
    }
    e.mem.set_u32(
        vector + VECTOR_END,
        new_capacity.wrapping_mul(bytes).wrapping_add(block),
    );
    e.mem.set_u32(
        vector + VECTOR_LAST,
        new_size.wrapping_mul(bytes).wrapping_add(block),
    );
    e.mem.set_u32(vector + VECTOR_FIRST, block);
}

/// The 6-byte reallocation's walk over the old elements (an empty loop body).
fn walk_only(_e: &mut Engine, _element: u32) {}

/// The 0x1C-byte reallocation's destruction of an old element (`0065eb40`, `cdecl`).
fn destruct_1c(e: &mut Engine, element: u32) {
    e.call(DESTRUCT_1C, &args![element]);
}

// Translated from 00ab0d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a 0x1C-byte-element vector's `resize(count, value)` (`thiscall`,
/// eight words: the count and the value by value: a float and a float
/// vector). Grows by inserting copies at the end (`00ab1800`) or shrinks by
/// erasing from `begin + count` (`00ad35e0`) to the end (`0065d740`); then
/// the value's vector is destroyed (`0065f430`, its proxy freed).
// Translation incomplete: 00ab0d50 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f77e, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab0d50(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<7>) {
    with_value_slot(e, &value.0, |e, slot| {
        resize_from_slot(e, this, count, slot)
    });
}

/// `00ab0d50`'s body over the value's argument slot at `slot` (where a caller that builds
/// the value in its outgoing argument area has it).
pub(crate) fn resize_from_slot(e: &mut Engine, this: Ptr, count: u32, slot: u32) {
    let shape = ResizeShape {
        size: ElementSize::Divide(0x1c),
        insert: fn_00ab1800,
        advance: |e, iterator, count| advance_by_address(ADVANCE_1C, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(ERASE_1C, e, this, result, a, b, c, d);
        },
    };
    resize_with_value(e, this, count, slot, &shape);
    e.call(FLOATS_TIDY, &args![slot + 4]);
    let proxy = e.mem.u32(slot + 4);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

// Translated from 00ab0fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a 6-byte-element vector's `_Insert_n(where, count, value)`
/// (`thiscall`, four words: the iterator's proxy and pointer, the count, the
/// value's address). Nothing for 0; more than the maximum (0x2AAAAAAA)
/// calls the length throw (`0065f640`). Past the capacity, a block of half
/// as much again (at least enough; `0065f9e0`) gets the elements before
/// `where` (`0065fde0`), `count` copies (`00ab23c0`) and the rest, the old
/// elements are walked and the old block freed. Otherwise, when fewer than
/// `count` elements follow `where`, they move up (`0065fde0`), the gap past
/// the old end is filled (`00ab23c0`) and the rest set; else the last
/// `count` elements are copied past the end (`00ab2260`), the others move up
/// (`00ab2210`) and `count` elements from `where` are set. The value is
/// copied before anything moves.
// Translation incomplete: 00ab0fd0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f7a0, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab0fd0(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Divide(6);
    let maximum = 0x2aaa_aaaa;
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    if max_or_one(maximum).wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        reallocate(
            e,
            vector,
            at,
            count,
            value,
            size,
            maximum,
            ALLOCATE_6,
            COPY_6,
            fn_00ab23c0,
            COPY_6,
            Some(walk_only),
        );
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    let copy = Six::load(e, value);
    let span = count.wrapping_mul(6);
    if after < count {
        e.with_stack(8, |e, local| {
            copy.store(e, local.addr());
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(COPY_6, &args![at, last, at.wrapping_add(span), allocator]);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            fn_00ab23c0(e, last, missing, local.addr(), allocator, 0, 0);
        });
        let last = span.wrapping_add(e.mem.u32(vector + VECTOR_LAST));
        e.mem.set_u32(vector + VECTOR_LAST, last);
        let mut element = at;
        while element != last.wrapping_sub(span) {
            copy.store(e, element);
            element = element.wrapping_add(6);
        }
    } else {
        let old_last = e.mem.u32(vector + VECTOR_LAST);
        let tail = old_last.wrapping_sub(span);
        let last = e.mem.u32(vector + VECTOR_LAST);
        let new_last = e
            .call(COPY_TAIL_6, &args![tail, old_last, last, allocator])
            .u32();
        e.mem.set_u32(vector + VECTOR_LAST, new_last);
        e.call(MOVE_BACKWARD_6, &args![at, tail, old_last]);
        let mut element = at;
        while element != at.wrapping_add(span) {
            copy.store(e, element);
            element = element.wrapping_add(6);
        }
    }
}

// Translated from 00ab1800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a 0x1C-byte-element (a float and a float vector) vector's
/// `_Insert_n(where, count, value)`, as `00ab0fd0` with the maximum
/// 0x9249249, the allocation `0065e670`, the copies `00660340` and
/// `0065ff90`, the fill `00ab2490`, the element destructor `0065eb40` (run
/// on every old element of a reallocation) and the backward move
/// `00ab2380`. Without a reallocation the value is first copied into a
/// temporary (the float through the x87 stack, the vector by its copy
/// constructor), the elements are set by the float and the vector's
/// assignment (`0065eed0`), and the temporary's vector is destroyed after
/// (`0065f430`, then `0065d6e0` or its inlined proxy free).
// Translation incomplete: 00ab1800 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f1f7f1, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab1800(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Divide(0x1c);
    let maximum = 0x0924_9249;
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    if max_or_one(maximum).wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        reallocate(
            e,
            vector,
            at,
            count,
            value,
            size,
            maximum,
            ALLOCATE_1C,
            COPY_1C,
            fn_00ab2490,
            COPY_AFTER_1C,
            Some(destruct_1c),
        );
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    let span = count.wrapping_mul(0x1c);
    e.with_stack(0x1c, |e, temporary| {
        let temporary = temporary.addr();
        let first_word = x87_quiet_bits(e.mem.u32(value));
        e.mem.set_u32(temporary, first_word);
        e.call(FLOATS_COPY, &args![temporary + 4, value + 4]);
        let set = |e: &mut Engine, from: u32, to: u32| {
            let mut element = from;
            while element != to {
                let word = x87_quiet_bits(e.mem.u32(temporary));
                e.mem.set_u32(element, word);
                e.call(FLOATS_ASSIGN, &args![element + 4, temporary + 4]);
                element = element.wrapping_add(0x1c);
            }
        };
        if after < count {
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(COPY_1C, &args![at, last, at.wrapping_add(span), allocator]);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            fn_00ab2490(e, last, missing, temporary, allocator, 0, 0);
            let last = span.wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            set(e, at, last.wrapping_sub(span));
            e.call(FLOATS_TIDY, &args![temporary + 4]);
            e.call(FLOATS_FREE_PROXY, &args![temporary + 4]);
        } else {
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(span);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let new_last = e
                .call(COPY_AFTER_1C, &args![tail, old_last, last, allocator])
                .u32();
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            e.call(
                MOVE_BACKWARD_1C,
                &args![at, tail, old_last, 0u32, 0u32, 0u32],
            );
            set(e, at, at.wrapping_add(span));
            e.call(FLOATS_TIDY, &args![temporary + 4]);
            let proxy = e.mem.u32(temporary + 4);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
        }
    });
}

// Translated from 00ab23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, six words, the last three not read: the allocator and
/// two tags): `count` copies of the 6-byte value at `value` from
/// `destination` on (a null destination skips the copy but still steps).
// Translation incomplete: 00ab23c0 - the C++ exception frame (FS:[0] chain, handler 00f1f810) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab23c0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count != 0 {
        if destination != 0 {
            Six::load(e, value).store(e, destination);
        }
        count -= 1;
        destination = destination.wrapping_add(6);
    }
}

// Translated from 00ab2490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, six words, the last three not read): `count` copies of
/// the 0x1C-byte value at `value` (its float through the x87 stack, its
/// float vector by the copy constructor `0065fb10`) from `destination` on
/// (a null destination skips the copy but still steps).
// Translation incomplete: 00ab2490 - the C++ exception frame (FS:[0] chain, handler 00f1f841) and the security-cookie bookkeeping are not translated.
pub fn fn_00ab2490(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count != 0 {
        if destination != 0 {
            let word = x87_quiet_bits(e.mem.u32(value));
            e.mem.set_u32(destination, word);
            e.call(FLOATS_COPY, &args![destination + 4, value + 4]);
        }
        count -= 1;
        destination = destination.wrapping_add(0x1c);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_0d50, fn_00ab0d50(Ptr, u32, WordsArg<7>)),
        entry!(0x00ab_0fd0, fn_00ab0fd0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_1800, fn_00ab1800(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_23c0, fn_00ab23c0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ab_2490, fn_00ab2490(u32, u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::facegen::binaryfile::{INVALID_PARAMETER, VECTOR_PROXY};

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A forward copy of `[first, last)` to `destination` returning its end.
    fn copy(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
        let bytes = e.mem.bytes(first, last - first);
        e.mem.write(destination, &bytes);
        destination + (last - first)
    }

    /// The 6-byte helpers as real copies; allocation from the test heap.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_6, |e, a| e.mem.alloc(6 * a[0]).into_ret());
        e.register(ALLOCATE_1C, |e, a| e.mem.alloc(0x1c * a[0]).into_ret());
        for addr in [COPY_6, COPY_TAIL_6, COPY_1C, COPY_AFTER_1C] {
            e.register(addr, |e, a| copy(e, a[0], a[1], a[2]).into_ret());
        }
        e.register(MOVE_BACKWARD_6, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2] - (a[1] - a[0]), &bytes);
            Ret::default()
        });
        for addr in [
            THROW_LENGTH,
            MEMORY_MANAGER_FREE,
            DESTRUCT_1C,
            MOVE_BACKWARD_1C,
            ADVANCE_1C,
            ERASE_1C,
            FLOATS_COPY,
            FLOATS_ASSIGN,
            FLOATS_TIDY,
            FLOATS_FREE_PROXY,
            INVALID_PARAMETER,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A vector object with `count` elements of `bytes` and room for `capacity`, each element
    /// filled with its index byte.
    fn vector(e: &mut Engine, bytes: u32, count: u32, capacity: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, vector);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let block = e.mem.alloc(bytes * capacity);
        for i in 0..count {
            for b in 0..bytes {
                e.mem.set_u8(block + i * bytes + b, i as u8);
            }
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(vector + VECTOR_LAST, block + bytes * count);
        e.mem.set_u32(vector + VECTOR_END, block + bytes * capacity);
        Ptr::new(vector)
    }

    fn sixes(e: &Engine, vector: Ptr) -> Vec<(u32, u16)> {
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        (first..last)
            .step_by(6)
            .map(|a| (e.mem.u32(a), e.mem.u16(a + 4)))
            .collect()
    }

    #[test]
    fn resize_1c_grows_through_insert_then_destroys_the_value() {
        let mut e = engine();
        let this = vector(&mut e, 0x1c, 1, 1);
        let old = e.mem.u32(this.addr() + VECTOR_FIRST);
        let value = WordsArg([0x3f80_0000, 0x0bad_0001, 2, 3, 4, 5, 6]);
        fn_00ab0d50(&mut e, this, 3, value);
        let log = e.call_log.take().unwrap();
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 3 * 0x1c);
        assert_eq!(e.mem.u32(this.addr() + VECTOR_END), first + 3 * 0x1c);
        assert_eq!(e.mem.u32(first + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(first + 0x38), 0x3f80_0000);
        assert_eq!(calls_to(&log, FLOATS_COPY).len(), 2);
        assert_eq!(calls_to(&log, DESTRUCT_1C), vec![vec![old]]);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, old],
                vec![MEMORY_MANAGER_OBJECT, 0x0bad_0001]
            ]
        );
        assert_eq!(calls_to(&log, FLOATS_TIDY).len(), 1);
        // Shrinking advances begin by the count and erases to the end.
        let this = vector(&mut e, 0x1c, 3, 3);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        fn_00ab0d50(&mut e, this, 1, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADVANCE_1C).len(), 1);
        let erase = calls_to(&log, ERASE_1C);
        assert_eq!(erase.len(), 1);
        assert_eq!(erase[0][0], this.addr());
        assert_eq!(erase[0][5], first + 3 * 0x1c);
    }

    #[test]
    fn insert_6_reallocates_or_moves_in_place() {
        let mut e = engine();
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0xaaaa_aaaa);
        e.mem.set_u16(value + 4, 0xbbbb);
        let v = (0xaaaa_aaaa, 0xbbbb);
        // Past the capacity: a new block of exactly enough.
        let this = vector(&mut e, 6, 2, 2);
        let at = e.mem.u32(this.addr() + VECTOR_FIRST) + 6;
        fn_00ab0fd0(&mut e, this, 0, at, 2, value);
        let one = (0x0101_0101, 0x0101);
        assert_eq!(sixes(&e, this), vec![(0, 0), v, v, one]);
        // Room, fewer elements after `where` than the count.
        let this = vector(&mut e, 6, 2, 8);
        let at = e.mem.u32(this.addr() + VECTOR_FIRST) + 6;
        fn_00ab0fd0(&mut e, this, 0, at, 3, value);
        assert_eq!(sixes(&e, this), vec![(0, 0), v, v, v, one]);
        // Room, more elements after `where` than the count.
        let this = vector(&mut e, 6, 4, 8);
        let at = e.mem.u32(this.addr() + VECTOR_FIRST) + 6;
        fn_00ab0fd0(&mut e, this, 0, at, 1, value);
        let two = (0x0202_0202, 0x0202);
        let three = (0x0303_0303, 0x0303);
        assert_eq!(sixes(&e, this), vec![(0, 0), v, one, two, three]);
        // Nothing for 0; past the maximum, the throw.
        e.call_log = Some(vec![]);
        fn_00ab0fd0(&mut e, this, 0, at, 0, value);
        fn_00ab0fd0(&mut e, this, 0, at, 0x2aaa_aaaa, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, THROW_LENGTH).len(), 1);
        assert_eq!(sixes(&e, this).len(), 5);
    }

    #[test]
    fn insert_1c_sets_floats_and_vectors_from_a_temporary() {
        let mut e = engine();
        let value = e.mem.alloc(0x1c);
        e.mem.set_u32(value, 0x7f80_0001);
        // More elements after `where` than the count.
        let this = vector(&mut e, 0x1c, 3, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        fn_00ab1800(&mut e, this, 0, first, 1, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 4 * 0x1c);
        assert_eq!(e.mem.u32(first), 0x7fc0_0001);
        assert_eq!(calls_to(&log, MOVE_BACKWARD_1C).len(), 1);
        let assign = calls_to(&log, FLOATS_ASSIGN);
        assert_eq!(assign.len(), 1);
        assert_eq!(assign[0][0], first + 4);
        let temporary = assign[0][1];
        assert_eq!(
            calls_to(&log, FLOATS_COPY),
            vec![vec![temporary, value + 4]]
        );
        assert_eq!(calls_to(&log, FLOATS_TIDY), vec![vec![temporary]]);
        assert_eq!(calls_to(&log, MEMORY_MANAGER_FREE).len(), 1);
        // Fewer after `where`: the gap is filled and the proxy freed by `0065d6e0`.
        let this = vector(&mut e, 0x1c, 1, 8);
        let first = e.mem.u32(this.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        fn_00ab1800(&mut e, this, 0, first, 2, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + VECTOR_LAST), first + 3 * 0x1c);
        assert_eq!(e.mem.u32(first + 0x1c), 0x7fc0_0001);
        assert_eq!(calls_to(&log, FLOATS_ASSIGN).len(), 1);
        assert_eq!(calls_to(&log, FLOATS_COPY).len(), 2);
        assert_eq!(calls_to(&log, FLOATS_FREE_PROXY).len(), 1);
        assert!(calls_to(&log, MEMORY_MANAGER_FREE).is_empty());
    }

    #[test]
    fn fill_6_skips_null_destinations() {
        let mut e = engine();
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x1234_5678);
        e.mem.set_u16(value + 4, 0x9abc);
        let block = e.mem.alloc(18);
        fn_00ab23c0(&mut e, block, 3, value, 0, 0, 0);
        for i in 0..3 {
            assert_eq!(e.mem.u32(block + 6 * i), 0x1234_5678);
            assert_eq!(e.mem.u16(block + 6 * i + 4), 0x9abc);
        }
        fn_00ab23c0(&mut e, 0, 1, value, 0, 0, 0);
    }

    #[test]
    fn fill_1c_copies_the_float_and_the_vector() {
        let mut e = engine();
        let value = e.mem.alloc(0x1c);
        e.mem.set_u32(value, 0xff80_0002);
        let block = e.mem.alloc(0x38);
        fn_00ab2490(&mut e, block, 2, value, 0, 0, 0);
        fn_00ab2490(&mut e, 0, 1, value, 0, 0, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(block), 0xffc0_0002);
        assert_eq!(e.mem.u32(block + 0x1c), 0xffc0_0002);
        assert_eq!(
            calls_to(&log, FLOATS_COPY),
            vec![vec![block + 4, value + 4], vec![block + 0x20, value + 4]]
        );
    }
}
