//! `fallout shared/pathfinding/run_006b4850` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0069bda0` (unnamed in the engine map): the destructor body the
/// scalar deleting destructor `006b4850` runs.
const DTOR_BODY_0069BDA0: u32 = 0x0069_bda0;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `00401460` (unnamed in the engine map; cdecl: destination, source,
/// byte count): copies bytes out of the record.
const COPY_BYTES_00401460: u32 = 0x0040_1460;
/// `00401680` (unnamed in the engine map): the file's byte at +0x299.
const FILE_FLAG_00401680: u32 = 0x0040_1680;
/// `00401080` (unnamed in the engine map; cdecl, the address of a word and
/// a byte): run on each word read while the file's byte at +0x299 is set.
const FIX_UP_00401080: u32 = 0x0040_1080;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl, the address of
/// a form id and a file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `0069bcf0` (unnamed in the engine map): the constructor of the 0x38-byte
/// object kept at +0x58.
const EXTRA_CTOR_0069BCF0: u32 = 0x0069_bcf0;
/// `0069c8f0` (unnamed in the engine map; `thiscall`, `RET 0xC`: data,
/// file, version): reads the object at +0x58 from the record.
const EXTRA_LOAD_0069C8F0: u32 = 0x0069_c8f0;
/// `0069c460` (unnamed in the engine map): the byte count that object read.
const EXTRA_SIZE_0069C460: u32 = 0x0069_c460;

// Translated from 006b4850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (unnamed in the engine map; `thiscall`,
/// `RET 4`): runs the destructor body `0069bda0` on `this`, then frees
/// `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006b4850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DTOR_BODY_0069BDA0, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// Copies `size` bytes from `data` to `this + offset`, then, when the
/// file's byte is set, runs the word fix-up on each of `words` offsets.
fn read_field(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    data: u32,
    size: u32,
    file: Ptr,
    words: &[u32],
) {
    e.call(
        COPY_BYTES_00401460,
        &args![this.addr() + offset, data, size],
    );
    if e.call(FILE_FLAG_00401680, &args![file]).u8() != 0 {
        for word in words {
            e.call(FIX_UP_00401080, &args![this.addr() + word, 0u32]);
        }
    }
}

// Translated from 006b4b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b4b50 - its FS:[0] exception frame (the unwind state that frees the 0x38-byte block when its constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, `RET 0xC`): reads the navmesh
/// info's fixed part from the record bytes at `data`, each field copied
/// with `00401460` and, when the file's byte (`00401680`) is set, fixed up
/// word by word with `00401080`: the word at +8 (from `data`), the word at
/// +0 (from `data + 4`, then given its compile index from `file`), the word
/// at +4 (from `data + 8`, compile index too), the word at +0xC (from
/// `data + 0xC`), and 0xC bytes at +0x10 (from `data + 0x10`, three
/// words). The data then continues at `data + 0x1C`. When bit 0x20 of +8 is
/// set, the 0x38-byte object at +0x58 is made first if missing (allocated,
/// then constructed by `0069bcf0`, null when the allocation fails), reads
/// itself (`0069c8f0(data, file, version)`) and the data moves on by the
/// size `0069c460` reports. With `version` 10 or more (signed), one more
/// word goes to +0x20.
pub fn fn_006b4b50(e: &mut Engine, this: Ptr, data: u32, file: Ptr, version: i32) {
    let mut data = data;
    read_field(e, this, 8, data, 4, file, &[8]);
    data = data.wrapping_add(4);
    read_field(e, this, 0, data, 4, file, &[0]);
    data = data.wrapping_add(4);
    e.call(TES_FORM_ADD_COMPILE_INDEX, &args![this, file]);
    read_field(e, this, 4, data, 4, file, &[4]);
    data = data.wrapping_add(4);
    e.call(TES_FORM_ADD_COMPILE_INDEX, &args![this.addr() + 4, file]);
    read_field(e, this, 0xc, data, 4, file, &[0xc]);
    data = data.wrapping_add(4);
    read_field(e, this, 0x10, data, 0xc, file, &[0x10, 0x14, 0x18]);
    data = data.wrapping_add(0xc);
    if e.mem.u32(this.addr() + 8) & 0x20 != 0 {
        if e.mem.u32(this.addr() + 0x58) == 0 {
            let block = e.call(ALLOCATE, &args![0x38u32]).u32();
            let extra = if block != 0 {
                e.call(EXTRA_CTOR_0069BCF0, &args![block]).u32()
            } else {
                0
            };
            e.mem.set_u32(this.addr() + 0x58, extra);
        }
        let extra = e.mem.u32(this.addr() + 0x58);
        e.call(EXTRA_LOAD_0069C8F0, &args![extra, data, file, version]);
        let extra = e.mem.u32(this.addr() + 0x58);
        let read = e.call(EXTRA_SIZE_0069C460, &args![extra]).u32();
        data = read.wrapping_add(data);
    }
    if version >= 10 {
        read_field(e, this, 0x20, data, 4, file, &[0x20]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_4850, fn_006b4850(Ptr, u32) -> Ptr),
        entry!(0x006b_4b50, fn_006b4b50(Ptr, u32, Ptr, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006b4850_runs_destructor_body_then_deletes_on_flag_one() {
        let mut e = Engine::new();
        e.register(DTOR_BODY_0069BDA0, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006b_4850, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x006b_4850).collect();
            let mut expected = vec![(DTOR_BODY_0069BDA0, vec![this])];
            if deletes {
                expected.push((DEALLOCATE, vec![this]));
            }
            assert_eq!(calls, expected);
        }
    }

    fn engine(flag: u32) -> Engine {
        let mut e = Engine::new();
        e.register(COPY_BYTES_00401460, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        if flag != 0 {
            e.register(FILE_FLAG_00401680, |_, _| 1u32.into_ret());
        } else {
            e.register(FILE_FLAG_00401680, |_, _| Ret::default());
        }
        e.register(FIX_UP_00401080, |_, _| Ret::default());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(EXTRA_CTOR_0069BCF0, |_, a| a[0].into_ret());
        e.register(EXTRA_LOAD_0069C8F0, |_, _| Ret::default());
        e.register(EXTRA_SIZE_0069C460, |_, _| 0x10u32.into_ret());
        e
    }

    fn record(e: &mut Engine, flags: u32) -> u32 {
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, flags);
        for i in 1..16u32 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        data
    }

    #[test]
    fn at_006b4b50_reads_the_fixed_fields() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x1);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), 0x1);
        assert_eq!(e.mem.u32(this), 0x101);
        assert_eq!(e.mem.u32(this + 4), 0x102);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x10), 0x104);
        assert_eq!(e.mem.u32(this + 0x18), 0x106);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let compile: Vec<_> = log
            .iter()
            .filter(|c| c.0 == TES_FORM_ADD_COMPILE_INDEX)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(compile, vec![vec![this, file], vec![this + 4, file]]);
        assert!(log
            .iter()
            .all(|c| c.0 != FIX_UP_00401080 && c.0 != ALLOCATE));
    }

    #[test]
    fn at_006b4b50_fixes_words_reads_the_extra_part_and_the_version_word() {
        let mut e = engine(1);
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x20);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 10u32]);
        let log = e.call_log.take().unwrap();
        let fixes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == FIX_UP_00401080)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(fixes, vec![8, 0, 4, 0xc, 0x10, 0x14, 0x18, 0x20]);
        let extra = e.mem.u32(this + 0x58);
        assert_ne!(extra, 0);
        let load = log.iter().find(|c| c.0 == EXTRA_LOAD_0069C8F0).unwrap();
        assert_eq!(load.1, vec![extra, data + 0x1c, file, 10]);
        // The version word comes after the extra part's 0x10 bytes.
        assert_eq!(e.mem.u32(this + 0x20), 0x10b);

        // An existing extra object is reused.
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 10u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != ALLOCATE));
    }

    #[test]
    fn at_006b4b50_stores_null_when_the_extra_allocation_fails() {
        let mut e = engine(0);
        e.register(ALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        let data = record(&mut e, 0x20);
        let file = e.mem.alloc(0x300);
        e.call_log = Some(vec![]);
        e.call(0x006b_4b50, &args![this, data, file, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert!(log.iter().all(|c| c.0 != EXTRA_CTOR_0069BCF0));
        let load = log.iter().find(|c| c.0 == EXTRA_LOAD_0069C8F0).unwrap();
        assert_eq!(load.1[0], 0);
    }
}
