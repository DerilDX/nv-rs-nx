//! `fallout shared/region system/tesregiondatamap.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f3d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap` scalar deleting destructor (engine map name): runs the destructor body `004f3de0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_3de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The `TESRegionData` copy constructor (`004f1460`, `thiscall`, the source).
const TES_REGION_DATA_COPY_CTOR: u32 = 0x004f_1460;
/// The `TESRegionData` destructor body (`004f14a0`).
const TES_REGION_DATA_DTOR: u32 = 0x004f_14a0;
/// The vtable of `TESRegionDataMap` (RTTI `.?AVTESRegionDataMap@@`, read at the word before it).
const TES_REGION_DATA_MAP_VTABLE: u32 = 0x0102_3d28;
/// `"Default Region Name"`.
const DEFAULT_REGION_NAME: u32 = 0x0102_3d10;
/// `004037b0` (unnamed in the engine map): the `BSStringT<char>` constructor.
const STRING_CTOR: u32 = 0x0040_37b0;
/// `004037d0` (unnamed in the engine map): the `BSStringT<char>` destructor.
const STRING_DTOR: u32 = 0x0040_37d0;
/// `004037f0` (unnamed in the engine map; `thiscall`, `RET 8`): sets the
/// string from a C string and a length (0 here).
const STRING_SET: u32 = 0x0040_37f0;
/// `00559450` (unnamed in the engine map; no stack words): the string's
/// characters.
const STRING_CHARS: u32 = 0x0055_9450;
/// `BSStringT<char>::StrCmp` (`00408a80`, engine map; `thiscall`, `RET 8`).
const STRING_COMPARE: u32 = 0x0040_8a80;
/// The virtual slot (byte offset) `GetMapName` (Xbox PDB slot +0x2C) the
/// copy constructor reads the source's name through.
const SLOT_GET_MAP_NAME: u32 = 0x2c;
/// `TESFile::GetTESChunk` (`004726b0`, Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (unnamed in the engine map): the file's word at +0x25C (the
/// current chunk's size).
const CHUNK_SIZE_00401660: u32 = 0x0040_1660;
/// `TESFile::GetChunkData_ov3` (`00472890`, Xbox PDB; `thiscall`, buffer and size).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// `RDMP` (as a little-endian word), the chunk type of the map name.
const CHUNK_RDMP: u32 = 0x504d_4452;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;

// Translated from 004f3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3ca0 - its FS:[0] exception frame (the unwind states that destroy the base and the name) is not translated
/// `TESRegionDataMap::TESRegionDataMap` (Xbox PDB): runs the `TESRegionData`
/// constructor, stores the class's vtable, constructs the name string at +8
/// (`MapName`, Xbox PDB) and sets it to `"Default Region Name"`. Returns
/// `this`.
pub fn tes_region_data_map_tes_region_data_map(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_CTOR, &args![this.addr() + 8]);
    e.call(
        STRING_SET,
        &args![this.addr() + 8, DEFAULT_REGION_NAME, 0u32],
    );
    this
}

// Translated from 004f3d50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3d50 - its FS:[0] exception frame (the unwind states that destroy the base and the name) is not translated
/// `TESRegionDataMap` copy constructor (unnamed in the engine map;
/// `thiscall`, `RET 4`): runs the `TESRegionData` copy constructor with
/// `source`, stores the class's vtable, constructs the name string at +8,
/// then sets it (`SetMapName`, `004f3f40`) to the name the source's virtual
/// slot +0x2C returns. Returns `this`.
pub fn fn_004f3d50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_COPY_CTOR, &args![this, source]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_CTOR, &args![this.addr() + 8]);
    let name = e.vcall(source.addr(), SLOT_GET_MAP_NAME, &args![]).u32();
    tes_region_data_map_set_map_name(e, this, name);
    this
}

// Translated from 004f3de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3de0 - its FS:[0] exception frame (the unwind state that runs the base destructor when the name's destructor throws) is not translated
/// `TESRegionDataMap` destructor body (unnamed in the engine map): stores
/// the class's vtable, destroys the name string at +8, then runs the
/// `TESRegionData` destructor body.
pub fn fn_004f3de0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_DATA_MAP_VTABLE);
    e.call(STRING_DTOR, &args![this.addr() + 8]);
    e.call(TES_REGION_DATA_DTOR, &args![this]);
}

// Translated from 004f3e90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3e90 - the /GS cookie check (00ec408c) of its stack frame is not translated
/// `TESRegionDataMap::LoadRegionDataMap` (Xbox PDB; `thiscall`, `RET 4`):
/// false for a null `file` or when the current chunk is not `RDMP`.
/// Otherwise the chunk is read (`GetChunkData_ov3` with size 0) into a
/// buffer of the chunk's size (`00401660`) that the exe makes on its stack
/// (`_alloca`, the probe `00ec5ec0`; here the engine's stack block), the
/// name is set from it (length 0) and true is returned.
pub fn tes_region_data_map_load_region_data_map(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if file.is_null() || e.call(GET_TES_CHUNK, &args![file]).u32() != CHUNK_RDMP {
        return false;
    }
    let size = e.call(CHUNK_SIZE_00401660, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        e.call(STRING_SET, &args![this.addr() + 8, buffer, 0u32]);
    });
    true
}

// Translated from 004f3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::GetMapName` (Xbox PDB; `thiscall`, `RET 4`): sets the
/// string `out` to the characters of the name at +8 (`00559450`), length 0.
/// The pushed 0 belongs to the setter: `00559450` takes no stack words.
pub fn tes_region_data_map_get_map_name(e: &mut Engine, this: Ptr, out: Ptr) {
    let chars = e.call(STRING_CHARS, &args![this.addr() + 8]).u32();
    e.call(STRING_SET, &args![out, chars, 0u32]);
}

// Translated from 004f3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataMap::SetMapName` (Xbox PDB; `thiscall`, `RET 4`): when
/// `name` differs from the current name (`StrCmp` ignoring case, non-zero),
/// sets the name to it (length 0) and returns true; otherwise false.
pub fn tes_region_data_map_set_map_name(e: &mut Engine, this: Ptr, name: u32) -> bool {
    if e.call(STRING_COMPARE, &args![this.addr() + 8, name, 1u32])
        .u32()
        == 0
    {
        return false;
    }
    e.call(STRING_SET, &args![this.addr() + 8, name, 0u32]);
    true
}

// Translated from 004f3f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f3f80 - its FS:[0] exception frame (the unwind state that frees the block when the copy constructor throws) is not translated
/// `TESRegionDataMap::Copy` (Xbox PDB): allocates 0x10 bytes and, when that
/// succeeds, copy-constructs a `TESRegionDataMap` of `this` in them
/// (`004f3d50`). Returns the copy, or null.
pub fn tes_region_data_map_copy(e: &mut Engine, this: Ptr) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_004f3d50(e, Ptr::new(block), this)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_3d20, tes_region_data_map_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_3ca0, tes_region_data_map_tes_region_data_map(Ptr) -> Ptr),
        entry!(0x004f_3d50, fn_004f3d50(Ptr, Ptr) -> Ptr),
        entry!(0x004f_3de0, fn_004f3de0(Ptr)),
        entry!(0x004f_3e90, tes_region_data_map_load_region_data_map(Ptr, Ptr) -> bool),
        entry!(0x004f_3f10, tes_region_data_map_get_map_name(Ptr, Ptr)),
        entry!(0x004f_3f40, tes_region_data_map_set_map_name(Ptr, u32) -> bool),
        entry!(0x004f_3f80, tes_region_data_map_copy(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod map_tests {
    use super::*;

    fn others(log: Vec<(u32, Vec<u32>)>, own: u32) -> Vec<(u32, Vec<u32>)> {
        log.into_iter().filter(|c| c.0 != own).collect()
    }

    #[test]
    fn at_004f3ca0_sets_the_default_name() {
        let mut e = Engine::new();
        for a in [TES_REGION_DATA_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_3ca0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3ca0),
            vec![
                (TES_REGION_DATA_CTOR, vec![this]),
                (STRING_CTOR, vec![this + 8]),
                (STRING_SET, vec![this + 8, DEFAULT_REGION_NAME, 0]),
            ]
        );
    }

    fn map_source(e: &mut Engine) -> u32 {
        e.put_vtable(0x0f01_0000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x0f01_1000]);
        e.register(0x0f01_1000, |_, _| 0x00ab_cdefu32.into_ret());
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, 0x0f01_0000);
        source
    }

    #[test]
    fn at_004f3d50_copies_the_base_and_the_name() {
        let mut e = Engine::new();
        for a in [TES_REGION_DATA_COPY_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        let source = map_source(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_3d50, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3d50),
            vec![
                (TES_REGION_DATA_COPY_CTOR, vec![this, source]),
                (STRING_CTOR, vec![this + 8]),
                (0x0f01_1000, vec![source]),
                (STRING_COMPARE, vec![this + 8, 0x00ab_cdef, 1]),
                (STRING_SET, vec![this + 8, 0x00ab_cdef, 0]),
            ]
        );
    }

    #[test]
    fn at_004f3de0_destroys_the_name_then_the_base() {
        let mut e = Engine::new();
        for a in [STRING_DTOR, TES_REGION_DATA_DTOR] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f_3de0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_MAP_VTABLE);
        assert_eq!(
            others(log, 0x004f_3de0),
            vec![
                (STRING_DTOR, vec![this + 8]),
                (TES_REGION_DATA_DTOR, vec![this])
            ]
        );
    }

    #[test]
    fn at_004f3e90_reads_the_name_chunk() {
        let mut e = Engine::new();
        e.register(GET_TES_CHUNK, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CHUNK_SIZE_00401660, |_, _| 0x20u32.into_ret());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_cstr(a[1], b"Goodsprings");
            Ret::default()
        });
        e.register_double(STRING_SET, |e, a| {
            let text = e.mem.cstr(a[1]);
            assert_eq!(text, b"Goodsprings".to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        assert!(!e.call(0x004f_3e90, &args![this, 0u32]).bool());
        let file = e.mem.alloc(0x300);
        e.mem.set_u32(file, 0x4449_4452);
        assert!(!e.call(0x004f_3e90, &args![this, file]).bool());
        e.mem.set_u32(file, CHUNK_RDMP);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_3e90, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        let read = log.iter().find(|c| c.0 == GET_CHUNK_DATA).unwrap();
        assert_eq!(read.1[0], file);
        assert_eq!(read.1[2], 0);
        let set = log.iter().find(|c| c.0 == STRING_SET).unwrap();
        assert_eq!(set.1, vec![this + 8, read.1[1], 0]);
    }

    #[test]
    fn at_004f3f10_sets_out_from_the_name_characters() {
        let mut e = Engine::new();
        e.register(STRING_CHARS, |_, _| 0x0055_0000u32.into_ret());
        e.register(STRING_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x004f_3f10, &args![this, out]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            others(log, 0x004f_3f10),
            vec![
                (STRING_CHARS, vec![this + 8]),
                (STRING_SET, vec![out, 0x0055_0000, 0]),
            ]
        );
    }

    #[test]
    fn at_004f3f40_sets_only_a_different_name() {
        let mut e = Engine::new();
        e.register(STRING_COMPARE, |_, a| u32::from(a[1] != 0x10).into_ret());
        e.register(STRING_SET, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_3f40, &args![this, 0x10u32]).bool());
        assert!(e.call(0x004f_3f40, &args![this, 0x20u32]).bool());
        let log = e.call_log.take().unwrap();
        let sets: Vec<_> = log.into_iter().filter(|c| c.0 == STRING_SET).collect();
        assert_eq!(sets, vec![(STRING_SET, vec![this + 8, 0x20, 0])]);
    }

    #[test]
    fn at_004f3f80_copies_into_a_new_block_or_returns_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| Ret::default());
        let this = map_source(&mut e);
        assert_eq!(e.call(0x004f_3f80, &args![this]).u32(), 0);

        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for a in [TES_REGION_DATA_COPY_CTOR, STRING_CTOR, STRING_SET] {
            e.register(a, |_, _| Ret::default());
        }
        e.register(STRING_COMPARE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let copy = e.call(0x004f_3f80, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(copy, 0);
        assert_eq!(e.mem.u32(copy), TES_REGION_DATA_MAP_VTABLE);
        let alloc = log.iter().find(|c| c.0 == ALLOCATE).unwrap();
        assert_eq!(alloc.1, vec![0x10]);
        let base = log
            .iter()
            .find(|c| c.0 == TES_REGION_DATA_COPY_CTOR)
            .unwrap();
        assert_eq!(base.1, vec![copy, this]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f3d20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_3de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_3d20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_3d20)
                .collect();
            let mut expected = vec![(0x004f_3de0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
