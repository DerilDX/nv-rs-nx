//! `(unplaced)/run_00afe7b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable `00afe9f0` installs: its RTTI type descriptor reads
/// `.?AV?$BSSimpleArray@PBD$0EAA@@@` (`BSSimpleArray<char const *, 1024>`).
const SIMPLE_ARRAY_OF_STRINGS_VTABLE: u32 = 0x0104_72d0;
/// `006b3eb0` (thiscall, two words: size, reserved size): clears the array's
/// three words and allocates the larger of the two through the array's own
/// vtable +4.
const SIMPLE_ARRAY_INITIALISE: u32 = 0x006b_3eb0;
/// `0042fb60` (thiscall, three words: destination, source, count): moves
/// `count` words, forwards or backwards as the ranges need.
const MOVE_WORDS: u32 = 0x0042_fb60;
/// Import slot of KERNEL32 `CreateFileA` (stdcall, seven words).
const CREATE_FILE_A: u32 = 0x00fd_f088;
/// Import slot of KERNEL32 `GetLastError` (stdcall, no words).
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `00afeef0` (thiscall on the same file object, four words: two distance
/// words, a move method, an out address): the seek of this unit, not
/// translated here.
const FILE_SEEK: u32 = 0x00af_eef0;

// Translated from 00afe9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<char const *, 1024>`
/// constructor with a size (the class from the RTTI of the vtable it
/// installs). Sets the vtable, then initialises the array with
/// `006b3eb0(size, size)`. Returns `this`.
pub fn fn_00afe9f0(e: &mut Engine, this: Ptr<BSSimpleArray>, size: u32) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_OF_STRINGS_VTABLE);
    e.call(SIMPLE_ARRAY_INITIALISE, &args![this, size, size]);
    this
}

// Translated from 00afea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes `count` 4-byte elements at `index`
/// from a `BSSimpleArray`. The elements after the gap are not shifted one by
/// one: the last `min(iSize - index - count, count)` elements (unsigned
/// arithmetic) are moved into the gap with `0042fb60`, then `iSize` is
/// lowered by `count`.
///
/// The exe also counts an index from 0 up to `count` with an empty body
/// (an element destructor that compiled to nothing); it has no effect.
pub fn fn_00afea20(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32, count: u32) {
    let size = e.get(this, BSSimpleArray::iSize);
    let tail = size.wrapping_sub(index).wrapping_sub(count);
    let moved = if tail < count { tail } else { count };
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let source = buffer
        .wrapping_add(size.wrapping_mul(4))
        .wrapping_sub(moved.wrapping_mul(4));
    let destination = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(MOVE_WORDS, &args![this, destination, source, moved]);
    let size = e.get(this, BSSimpleArray::iSize);
    e.set(this, BSSimpleArray::iSize, size.wrapping_sub(count));
}

/// The file object `00afeac0` opens (class not named in the exe's RTTI or
/// the engine map): the access kind at +4, the creation kind at +8, a flag
/// byte at +0xc and the Windows handle at +0x10.
const FILE_ACCESS_KIND: u32 = 0x4;
const FILE_CREATION_KIND: u32 = 0x8;
const FILE_OVERLAPPED_FLAG: u32 = 0xc;
const FILE_HANDLE: u32 = 0x10;

// Translated from 00afeac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: opens `path` with `CreateFileA` and keeps the
/// handle at +0x10. The access kind (+4) 0, 1, 2 asks for `GENERIC_READ`,
/// read and write, `GENERIC_WRITE` (any other value for no access); the
/// creation kind (+8) 0 and 1 open an existing file (3), 2 creates always
/// (2), 3 truncates (5), 4 creates a new file (1), anything else passes 0.
/// The flags are `FILE_FLAG_RANDOM_ACCESS` (0x08000000), with
/// `FILE_FLAG_NO_BUFFERING | FILE_FLAG_OVERLAPPED` (0x60000000) added when
/// the byte at +0xc is set; the share mode is `FILE_SHARE_READ`.
///
/// When creation kind 1 fails, the open is retried with "create always".
/// A failed open answers `GetLastError` mapped to 0 (no error), 1
/// (`ERROR_FILE_NOT_FOUND`), 2 (`ERROR_PATH_NOT_FOUND`), 3
/// (`ERROR_WRITE_PROTECT`, 0x13) or 4 (anything else). A successful open
/// with creation kind 1 and the flag byte clear answers the seek
/// `00afeef0(0, 0, 2, &end)` (to the end of the file); otherwise 0.
pub fn fn_00afeac0(e: &mut Engine, this: Ptr, path: Ptr) -> u32 {
    let base = this.addr();
    let access = match e.mem.u32(base + FILE_ACCESS_KIND) {
        0 => 0x8000_0000u32,
        1 => 0xc000_0000,
        2 => 0x4000_0000,
        _ => 0,
    };
    let creation = match e.mem.u32(base + FILE_CREATION_KIND) {
        0 | 1 => 3u32,
        2 => 2,
        3 => 5,
        4 => 1,
        _ => 0,
    };
    let mut flags = 0x0800_0000u32;
    if e.mem.u8(base + FILE_OVERLAPPED_FLAG) != 0 {
        flags |= 0x6000_0000;
    }
    let share = 1u32;
    let handle = e
        .call(
            CREATE_FILE_A,
            &args![path, access, share, 0u32, creation, flags, 0u32],
        )
        .u32();
    e.mem.set_u32(base + FILE_HANDLE, handle);
    if e.mem.u32(base + FILE_HANDLE) == u32::MAX && e.mem.u32(base + FILE_CREATION_KIND) == 1 {
        let handle = e
            .call(
                CREATE_FILE_A,
                &args![path, access, share, 0u32, 2u32, flags, 0u32],
            )
            .u32();
        e.mem.set_u32(base + FILE_HANDLE, handle);
    }
    let mut result = 0u32;
    if e.mem.u32(base + FILE_HANDLE) == u32::MAX {
        // `switch (GetLastError())` through a byte map over 0..=0x13.
        result = match e.call(GET_LAST_ERROR, &args![]).u32() {
            0 => 0,
            2 => 1,
            3 => 2,
            0x13 => 3,
            _ => 4,
        };
    }
    if result == 0
        && e.mem.u32(base + FILE_CREATION_KIND) == 1
        && e.mem.u8(base + FILE_OVERLAPPED_FLAG) == 0
    {
        result = e.with_stack(8, |e, end| {
            e.call(FILE_SEEK, &args![this, 0u32, 0u32, 2u32, end]).u32()
        });
    }
    result
}

/// Import slot of KERNEL32 `SetFilePointerEx` (stdcall, five words: the handle, the
/// distance's low and high words, the new position's address and the move method).
const SET_FILE_POINTER_EX: u32 = 0x00fd_f140;

// Translated from 00afeef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the seek of the file object (thiscall, four words: the
/// distance's low and high words, the move method and the address of an 8-byte position;
/// `RET 0x10`). Answers 4 while the flag byte (+0xc) is set. Otherwise moves the handle
/// (+0x10) with `SetFilePointerEx` (the new position on its stack, cleared first); on
/// success copies the new position to `position` and answers 0; on failure answers
/// `GetLastError` mapped as [`fn_00afeac0`] does: 0 to 0, 2 to 1, 3 to 2, 0x13 to 3 and
/// anything else to 4 (the exe's byte map at `00afefe8`, as `match` arms).
pub fn fn_00afeef0(
    e: &mut Engine,
    this: Ptr,
    distance_low: u32,
    distance_high: u32,
    method: u32,
    position: Ptr,
) -> u32 {
    let base = this.addr();
    e.with_stack(8, |e, moved| {
        if e.mem.u8(base + FILE_OVERLAPPED_FLAG) != 0 {
            return 4;
        }
        let handle = e.mem.u32(base + FILE_HANDLE);
        let done = e
            .call(
                SET_FILE_POINTER_EX,
                &args![handle, distance_low, distance_high, moved, method],
            )
            .u32();
        if done == 0 {
            return match e.call(GET_LAST_ERROR, &args![]).u32() {
                0 => 0,
                2 => 1,
                3 => 2,
                0x13 => 3,
                _ => 4,
            };
        }
        let low = e.mem.u32(moved.addr());
        e.mem.set_u32(position.addr(), low);
        let high = e.mem.u32(moved.addr() + 4);
        e.mem.set_u32(position.addr() + 4, high);
        0
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_eef0, fn_00afeef0(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(
            0x00af_e9f0,
            fn_00afe9f0(Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>
        ),
        entry!(0x00af_ea20, fn_00afea20(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00af_eac0, fn_00afeac0(Ptr, Ptr) -> u32),
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

    #[test]
    fn constructor_sets_the_vtable_and_initialises_with_the_size_twice() {
        let mut e = Engine::new();
        e.register(SIMPLE_ARRAY_INITIALISE, |_, _| Ret::default());
        let array: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        let result = e.call(0x00af_e9f0, &args![array, 7u32]).u32();
        assert_eq!(result, array.addr());
        assert_eq!(e.mem.u32(array.addr()), SIMPLE_ARRAY_OF_STRINGS_VTABLE);
        assert_eq!(
            calls_to(&e, SIMPLE_ARRAY_INITIALISE),
            vec![vec![array.addr(), 7, 7]]
        );
    }

    fn array(e: &mut Engine, size: u32) -> (Ptr<BSSimpleArray>, u32) {
        e.register(MOVE_WORDS, |_, _| Ret::default());
        let array: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x40);
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iSize, size);
        (array, buffer)
    }

    #[test]
    fn remove_moves_the_last_elements_into_the_gap() {
        let mut e = Engine::new();
        let (array, buffer) = array(&mut e, 10);
        e.call_log = Some(vec![]);
        // Index 2, count 3: the tail after the gap is 5 long, so 3 move.
        e.call(0x00af_ea20, &args![array, 2u32, 3u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array.addr(), buffer + 8, buffer + 28, 3]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 7);
    }

    #[test]
    fn remove_near_the_end_moves_only_the_short_tail() {
        let mut e = Engine::new();
        let (array, buffer) = array(&mut e, 6);
        e.call_log = Some(vec![]);
        // Index 1, count 4: one element follows the gap.
        e.call(0x00af_ea20, &args![array, 1u32, 4u32]);
        assert_eq!(
            calls_to(&e, MOVE_WORDS),
            vec![vec![array.addr(), buffer + 4, buffer + 20, 1]]
        );
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
    }

    /// A file object with the given kinds and flag byte; `CreateFileA`
    /// answers the handles in turn, `GetLastError` answers `error` and the
    /// seek 0x55.
    fn open(access: u32, creation: u32, flag: u8, handles: &[u32], error: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + FILE_ACCESS_KIND, access);
        e.mem.set_u32(object + FILE_CREATION_KIND, creation);
        e.mem.set_u8(object + FILE_OVERLAPPED_FLAG, flag);
        let queue = std::rc::Rc::new(std::cell::RefCell::new(handles.to_vec()));
        e.register_double(CREATE_FILE_A, move |_, _| {
            queue.borrow_mut().remove(0).into_ret()
        });
        e.register_double(GET_LAST_ERROR, move |_, _| error.into_ret());
        e.register(FILE_SEEK, |_, _| 0x55u32.into_ret());
        e.call_log = Some(vec![]);
        let result = e.call(0x00af_eac0, &args![object, 0x1234u32]).u32();
        assert_eq!(e.mem.u32(object + FILE_HANDLE), *handles.last().unwrap());
        (e, result)
    }

    #[test]
    fn open_for_reading_passes_the_access_creation_and_flags() {
        let (e, result) = open(0, 0, 0, &[0x40], 0);
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&e, CREATE_FILE_A),
            vec![vec![0x1234, 0x8000_0000, 1, 0, 3, 0x0800_0000, 0]]
        );
        assert!(calls_to(&e, FILE_SEEK).is_empty());
    }

    #[test]
    fn the_kinds_map_to_access_and_creation_values() {
        for (access, creation, want_access, want_creation) in [
            (1, 2, 0xc000_0000u32, 2u32),
            (2, 3, 0x4000_0000, 5),
            (3, 4, 0, 1),
            (0, 7, 0x8000_0000, 0),
        ] {
            let (e, _) = open(access, creation, 1, &[0x40], 0);
            assert_eq!(
                calls_to(&e, CREATE_FILE_A),
                vec![vec![
                    0x1234,
                    want_access,
                    1,
                    0,
                    want_creation,
                    0x6800_0000,
                    0
                ]]
            );
        }
    }

    #[test]
    fn creation_kind_one_retries_and_then_seeks_to_the_end() {
        let (e, result) = open(1, 1, 0, &[u32::MAX, 0x40], 0);
        assert_eq!(result, 0x55);
        let opens = calls_to(&e, CREATE_FILE_A);
        assert_eq!(opens.len(), 2);
        assert_eq!(opens[1], vec![0x1234, 0xc000_0000, 1, 0, 2, 0x0800_0000, 0]);
        let seeks = calls_to(&e, FILE_SEEK);
        assert_eq!(seeks.len(), 1);
        assert_eq!(&seeks[0][1..4], &[0, 0, 2]);
    }

    #[test]
    fn creation_kind_one_with_the_flag_does_not_seek() {
        let (e, result) = open(1, 1, 1, &[0x40], 0);
        assert_eq!(result, 0);
        assert!(calls_to(&e, FILE_SEEK).is_empty());
    }

    #[test]
    fn a_failed_open_maps_the_last_error() {
        for (error, want) in [(0u32, 0u32), (2, 1), (3, 2), (0x13, 3), (5, 4), (0x14, 4)] {
            let (_, result) = open(0, 0, 0, &[u32::MAX], error);
            assert_eq!(result, want, "error {error:#x}");
        }
        // A failure with no error code still counts as success for the seek.
        let (e, result) = open(0, 1, 0, &[u32::MAX, u32::MAX], 0);
        assert_eq!(result, 0x55);
        assert_eq!(calls_to(&e, FILE_SEEK).len(), 1);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn seek_copies_the_new_position() {
        let mut e = Engine::new();
        e.register(SET_FILE_POINTER_EX, |e, a| {
            // The new position is the distance itself.
            e.mem.set_u32(a[3], a[1]);
            e.mem.set_u32(a[3] + 4, a[2]);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + FILE_HANDLE, 0x44);
        let position = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00af_eef0, &args![this, 0x10u32, 0x2u32, 1u32, position])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(position), 0x10);
        assert_eq!(e.mem.u32(position + 4), 2);
        let log = e.call_log.take().unwrap();
        let call = log.iter().find(|(a, _)| *a == SET_FILE_POINTER_EX).unwrap();
        assert_eq!(
            [call.1[0], call.1[1], call.1[2], call.1[4]],
            [0x44, 0x10, 2, 1]
        );
    }

    #[test]
    fn seek_maps_the_error_and_refuses_with_the_flag() {
        let mut e = Engine::new();
        e.register(SET_FILE_POINTER_EX, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x14);
        let position = e.mem.alloc(8);
        for (error, want) in [
            (0u32, 0u32),
            (1, 4),
            (2, 1),
            (3, 2),
            (5, 4),
            (0x13, 3),
            (0x14, 4),
        ] {
            e.register_double(GET_LAST_ERROR, move |_, _| error.into_ret());
            let result = e
                .call(0x00af_eef0, &args![this, 0u32, 0u32, 0u32, position])
                .u32();
            assert_eq!(result, want, "error {error:#x}");
        }
        assert_eq!(e.mem.u32(position), 0);
        e.mem.set_u8(this + FILE_OVERLAPPED_FLAG, 1);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00af_eef0, &args![this, 0u32, 0u32, 0u32, position])
            .u32();
        assert_eq!(result, 4);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
