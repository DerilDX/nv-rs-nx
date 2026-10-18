//! `Havok SDK/run_00d61f30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl: the allocator, the array, the capacity,
/// the element size), `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`, cdecl: destination, source,
/// byte count) and `hkStringBuf::operator=(const char*)` (Xbox PDB name `hkStringBuf::operator_`,
/// `00d61c10`, thiscall, one argument).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const MEM_MOVE: u32 = 0x00d2_cc70;
const STRING_ASSIGN: u32 = 0x00d6_1c10;
/// The two string buffer routines the join goes through: `00d61cc0` (thiscall, no arguments:
/// makes the buffer the empty string, growing it to one byte when it holds none) and
/// `hkStringBuf::appendJoin` (Xbox PDB, `00d61e30`, thiscall, six words).
const STRING_CLEAR: u32 = 0x00d6_1cc0;
const STRING_APPEND_JOIN: u32 = 0x00d6_1e30;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d61f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the string buffer `this` (`00d61cc0`) and appends the join of the six words given
/// (`hkStringBuf::appendJoin`, Xbox PDB name of `00d61e30`; the six words are passed on as they
/// came), so the buffer holds exactly the joined text. The class of this function is not
/// confirmed (it behaves as a `join`).
// The exe takes the six join words as separate stack words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d61f30(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) {
    e.call(STRING_CLEAR, &args![this]);
    e.call(
        STRING_APPEND_JOIN,
        &args![this, first, second, third, fourth, fifth, sixth],
    );
}

// Translated from 00d623a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a string buffer from a pointer-with-a-flag word (`source` points at a word whose
/// bit 0 is a flag): the data pointer is the buffer's own inline storage at `this + 0xc`, the
/// size is 0 and the capacity word is `0x80000080` (128 bytes, storage not owned), then the text
/// the word points at (with bit 0 cleared) is assigned (`hkStringBuf::operator=`, `00d61c10`).
/// Answers `this`.
pub fn fn_00d623a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, at.wrapping_add(0xc));
    e.mem.set_u32(at.wrapping_add(4), 0);
    e.mem.set_u32(at.wrapping_add(8), 0x8000_0080);
    let word = e.mem.u32(source.addr());
    e.call(STRING_ASSIGN, &args![this, word & 0xffff_fffe]);
    this
}

// Translated from 00d625a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `removed` characters at `start` of the string buffer `this` by `inserted_count`
/// characters read from `inserted`: the new size is `size - removed + inserted_count`; when it is
/// above the capacity (low 30 bits, signed compare) the array is reserved (`hkArrayUtil::_reserve`
/// with `allocator`, the larger of twice the capacity and the new size, element size 1); the
/// tail behind the removed part (`size - start - removed` bytes) moves to
/// `start + inserted_count` (`hkMemUtil::memMove`); the new characters are copied forward one
/// at a time to `start`; the size is stored.
pub fn fn_00d625a0(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    start: i32,
    removed: i32,
    inserted: u32,
    inserted_count: i32,
) {
    let at = this.addr();
    let size = e.mem.i32(at.wrapping_add(4));
    let tail = size.wrapping_sub(start).wrapping_sub(removed);
    let new_size = size.wrapping_sub(removed).wrapping_add(inserted_count);
    let capacity = (e.mem.u32(at.wrapping_add(8)) & CAPACITY_MASK) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = doubled.max(new_size);
        e.call(ARRAY_RESERVE, &args![allocator, this, wanted, 1u32]);
    }
    let data = e.mem.u32(at).wrapping_add(start as u32);
    e.call(
        MEM_MOVE,
        &args![
            data.wrapping_add(inserted_count as u32),
            data.wrapping_add(removed as u32),
            tail
        ],
    );
    let target = e.mem.u32(at).wrapping_add(start as u32);
    if inserted_count > 0 {
        for index in 0..inserted_count as u32 {
            let byte = e.mem.u8(inserted.wrapping_add(index));
            e.mem.set_u8(target.wrapping_add(index), byte);
        }
    }
    e.mem.set_u32(at.wrapping_add(4), new_size as u32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_1f30, fn_00d61f30(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00d6_23a0, fn_00d623a0(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_25a0, fn_00d625a0(Ptr, u32, i32, i32, u32, i32)),
    ]
}

#[cfg(test)]
mod string_edit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn fn_00d61f30_clears_then_appends_the_join() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, STRING_CLEAR);
        record(&mut e, &log, STRING_APPEND_JOIN);
        e.call(
            0x00d6_1f30,
            &args![0x100u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_eq!(
            *log.borrow(),
            vec![
                (STRING_CLEAR, vec![0x100]),
                (STRING_APPEND_JOIN, vec![0x100, 1, 2, 3, 4, 5, 6]),
            ]
        );
    }

    #[test]
    fn fn_00d623a0_sets_up_inline_storage_and_assigns_the_text() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, STRING_ASSIGN);
        let this = e.mem.alloc(0x100);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x1235);
        let got = e.call(0x00d6_23a0, &args![this, source]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), this + 0xc);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0080);
        assert_eq!(*log.borrow(), vec![(STRING_ASSIGN, vec![this, 0x1234])]);
    }

    /// A string buffer of `text` (size counts the terminator) in `capacity` bytes, and a
    /// double for `memMove` that copies like `memmove`, one for `_reserve` that grows the
    /// storage.
    fn setup(text: &[u8], capacity: u32) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity.max(text.len() as u32 + 1));
        e.mem.write(data, text);
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, text.len() as u32 + 1);
        e.mem.set_u32(this + 8, capacity);
        let seen = Rc::clone(&log);
        e.register_double(MEM_MOVE, move |e, a| {
            seen.borrow_mut().push((MEM_MOVE, a.to_vec()));
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let array = a[1];
            let bytes = e.mem.bytes(e.mem.u32(array), e.mem.u32(array + 4));
            let data = e.mem.alloc(a[2]);
            e.mem.write(data, &bytes);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, a[2]);
            Ret::default()
        });
        (e, this, log)
    }

    fn contents(e: &Engine, this: u32) -> Vec<u8> {
        e.mem
            .bytes(e.mem.u32(this), e.mem.u32(this + 4).saturating_sub(1))
    }

    fn put_text(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text);
        at
    }

    #[test]
    fn fn_00d625a0_replaces_a_range_within_the_capacity() {
        let (mut e, this, log) = setup(b"hello world", 32);
        let new = put_text(&mut e, b"big");
        // "world" (5 characters at 6) becomes "big": only the terminator is moved.
        e.call(0x00d6_25a0, &args![this, 0x77u32, 6i32, 5i32, new, 3i32]);
        assert_eq!(contents(&e, this), b"hello big");
        assert_eq!(e.mem.u32(this + 4), 10);
        assert!(log.borrow().iter().all(|(a, _)| *a != ARRAY_RESERVE));
        // Replace two characters at 3 of "hello" by four:
        // the terminator moves up.
        let (mut e, this, log) = setup(b"hello", 32);
        let new = put_text(&mut e, b"p!!!");
        e.call(0x00d6_25a0, &args![this, 0x77u32, 3i32, 2i32, new, 4i32]);
        assert_eq!(contents(&e, this), b"help!!!");
        let data = e.mem.u32(this);
        // The tail (size 6 - start 3 - removed 2 = 1 byte, the terminator) moves from
        // start + removed to start + inserted.
        assert_eq!(
            *log.borrow(),
            vec![(MEM_MOVE, vec![data + 3 + 4, data + 3 + 2, 1])]
        );
    }

    #[test]
    fn fn_00d625a0_grows_to_the_larger_of_double_and_the_new_size() {
        // Capacity 4, size 4: doubling gives 8; the new size 4 + 10 = 14 is larger.
        let (mut e, this, log) = setup(b"abc", 4);
        let new = put_text(&mut e, b"0123456789");
        e.call(0x00d6_25a0, &args![this, 0x77u32, 3i32, 0i32, new, 10i32]);
        assert_eq!(log.borrow()[0], (ARRAY_RESERVE, vec![0x77, this, 14, 1]));
        assert_eq!(contents(&e, this), b"abc0123456789");
        // Capacity 5, size 6: doubling gives 10; the new size 8 is smaller, so 10 is asked for.
        let (mut e, this, log) = setup(b"abcde", 5);
        let new = put_text(&mut e, b"xy");
        e.call(0x00d6_25a0, &args![this, 0x78u32, 5i32, 0i32, new, 2i32]);
        assert_eq!(log.borrow()[0], (ARRAY_RESERVE, vec![0x78, this, 10, 1]));
        assert_eq!(contents(&e, this), b"abcdexy");
    }

    #[test]
    fn fn_00d625a0_inserting_nothing_only_closes_the_gap() {
        let (mut e, this, _) = setup(b"abcdef", 16);
        e.call(0x00d6_25a0, &args![this, 0u32, 1i32, 2i32, 0u32, 0i32]);
        assert_eq!(contents(&e, this), b"adef");
        assert_eq!(e.mem.u32(this + 4), 5);
    }
}
