//! `fallout shared/tesidlemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005ffbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600b90`, which stores the vtable `0104a060` at
/// `+0` and frees the block held at `+4` through the cdecl `004ede70`, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `00600b90`'s disassembly ends with the stack cleanup of that call
/// (`ADD ESP,4`) and computes no result (Ghidra types it void).
pub fn fn_005ffbd0(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0b90, &args![this]);
}

// Translated from 00600d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00600d90`, a destructor with an exception frame
/// that stores the vtable `0104a070` and, when the byte at `+0x10` is set, frees the items
/// of the chains in the table at `+8`, and returns nothing. Parameters: one word, `this`;
/// the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `00600d90`'s disassembly ends with its exception-frame unlink and returns no value
/// (Ghidra types it void).
pub fn fn_00600d70(e: &mut Engine, this: Ptr) {
    e.call(0x0060_0d90, &args![this]);
}

// Translated from 00600b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0104a040` (RTTI `.?AV?$BSTCaseInsensitiveStringMap@PAVIDLE_ANIM_ROOT@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00600d70`. One word, plain `RET`, returns nothing.
pub fn fn_00600b00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_a040);
    fn_00600d70(e, this);
}

/// `00406d30` (unnamed in the engine map): `cdecl`, three stack words (destination, size,
/// source): copies a string into a sized buffer.
const STRING_COPY: u32 = 0x0040_6d30;
/// `0040ab30` (`strrchr`, `magicitem.rs`): `cdecl`, the last occurrence of a character.
const STRRCHR: u32 = 0x0040_ab30;
/// `00853130` (unnamed in the engine map, `bgssaveloadreferencesmap.cpp`): `thiscall`, `RET 8`;
/// looks the key string up in a map and stores the entry's value through the second word,
/// answering whether it was found.
const MAP_LOOKUP: u32 = 0x0085_3130;
/// `"IdleAnims"`.
const IDLE_ANIMS_TEXT: u32 = 0x0101_70ec;
/// `00600560` (unnamed in the engine map): `thiscall`, `RET 4`; the root idle array registered
/// for a path, or null.
const GET_ROOT_IDLE_ARRAY: u32 = 0x0060_0560;
/// `00401000` (unnamed in the engine map): `cdecl`, one stack word; allocates that many bytes.
const ALLOCATE: u32 = 0x0040_1000;
/// `005ff890` (unnamed in the engine map): `thiscall`, `RET 4`; constructs a root idle array for
/// a path.
const ROOT_IDLE_ARRAY_CONSTRUCTOR: u32 = 0x005f_f890;
/// `005ff950` (unnamed in the engine map): `thiscall`, `RET 4`; hands the array its file name
/// list.
const ADD_ROOT_FILENAME_LIST: u32 = 0x005f_f950;
/// `00447850` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 8`; the
/// decompiler calls it `ModelLoader::CopyFilenameList(list, 0)`.
const MODEL_LOADER_COPY_FILENAME_LIST: u32 = 0x0044_7850;
/// `00447330` (unnamed in the engine map): `thiscall` on the `ModelLoader`; the decompiler calls
/// it `ModelLoader::BuildKFFileList(path, 1, 0, 0)`.
const MODEL_LOADER_BUILD_KF_FILE_LIST: u32 = 0x0044_7330;
/// `00470200` (unnamed in the engine map): `thiscall`, `RET 8`; registers the root idle array
/// under a path.
const REGISTER_ROOT_IDLE_ARRAY: u32 = 0x0047_0200;
/// The `ModelLoader` singleton (pointer global).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The byte global that is set while the file name list of a root is being built.
const BUILDING_ROOT_FLAG: u32 = 0x011c_b71c;

// Translated from 00600700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600700 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// `TESIdleManager::GetRootFilenameList` (Xbox PDB): `thiscall`, `RET 8` (a path and an index).
/// For a non-empty path, copies it into a 0x80-byte buffer, replaces its file name (everything
/// after the last backslash) with `"IdleAnims"`, looks that key up in the manager's map
/// (`00853130`) and answers the pointer stored at `index * 4 + 0x38` of the entry found; null
/// otherwise. The decompiler shows the lookup receiving the wrong arguments; the disassembly
/// passes the manager as `this`, the key and the address of the result.
pub fn tes_idle_manager_get_root_filename_list(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    index: u32,
) -> u32 {
    let mut result = 0;
    if !path.is_null() && e.mem.i8(path.addr()) != 0 {
        let buffer = e.mem.alloc(0x90);
        let entry_cell = e.mem.alloc(4);
        e.mem.set_u32(entry_cell, 0);
        e.call(STRING_COPY, &args![buffer, 0x80u32, path]);
        let separator = e.call(STRRCHR, &args![buffer, 0x5cu32]).u32();
        let room = 0x80u32
            .wrapping_sub(separator.wrapping_sub(buffer))
            .wrapping_sub(1);
        e.call(
            STRING_COPY,
            &args![separator.wrapping_add(1), room, IDLE_ANIMS_TEXT],
        );
        let found = e.call(MAP_LOOKUP, &args![this, buffer, entry_cell]).bool();
        let entry = e.mem.u32(entry_cell);
        if found && entry != 0 {
            result = e
                .mem
                .u32(entry.wrapping_add(index.wrapping_mul(4)).wrapping_add(0x38));
        }
        e.mem.free(entry_cell);
        e.mem.free(buffer);
    }
    result
}

// Translated from 00600170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00600170 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00efab6c, state variable) and the /GS stack-cookie check (00ec408c) are not translated
/// `TESIdleManager::AddRootIdleArray` (Xbox PDB): `thiscall`, `RET 0xc` (a path, an optional
/// file name list, a force byte). Does nothing (answers null) when the global flag at
/// `011cb71c` is set and `force` is zero, when the path is null or empty. Otherwise answers the
/// root idle array already registered for the path (`00600560`); when there is none, builds one
/// (`0x68` bytes, constructed with the path): from the given list copied by the `ModelLoader`
/// after copying the path into a 0x80-byte buffer whose file name becomes `"IdleAnims"` and
/// registering it under that buffer; or, without a list, from the `ModelLoader`'s animation file
/// list for the path (the global flag is set around that) and registering it under the path.
/// Answers the new array. The allocation result is checked for null as the exe does.
pub fn tes_idle_manager_add_root_idle_array(
    e: &mut Engine,
    this: Ptr,
    path: Ptr,
    filenames: Ptr,
    force: u8,
) -> u32 {
    let mut result = 0;
    let building = e.global::<u8>(BUILDING_ROOT_FLAG);
    if (building == 0 || force != 0) && !path.is_null() && e.mem.i8(path.addr()) != 0 {
        result = e.call(GET_ROOT_IDLE_ARRAY, &args![this, path]).u32();
        if result == 0 {
            if !filenames.is_null() {
                let buffer = e.mem.alloc(0x90);
                e.call(STRING_COPY, &args![buffer, 0x80u32, path]);
                let separator = e.call(STRRCHR, &args![buffer, 0x5cu32]).u32();
                let room = 0x80u32
                    .wrapping_sub(separator.wrapping_sub(buffer))
                    .wrapping_sub(1);
                e.call(
                    STRING_COPY,
                    &args![separator.wrapping_add(1), room, IDLE_ANIMS_TEXT],
                );
                let array = new_root_idle_array(e, path);
                let loader = e.global::<u32>(MODEL_LOADER);
                let list = e
                    .call(
                        MODEL_LOADER_COPY_FILENAME_LIST,
                        &args![loader, filenames, 0u32],
                    )
                    .u32();
                e.call(ADD_ROOT_FILENAME_LIST, &args![array, list]);
                e.call(REGISTER_ROOT_IDLE_ARRAY, &args![this, buffer, array]);
                e.mem.free(buffer);
                result = array;
            } else {
                let array = new_root_idle_array(e, path);
                e.set_global::<u8>(BUILDING_ROOT_FLAG, 1);
                let loader = e.global::<u32>(MODEL_LOADER);
                let list = e
                    .call(
                        MODEL_LOADER_BUILD_KF_FILE_LIST,
                        &args![loader, path, 1u32, 0u32, 0u32],
                    )
                    .u32();
                e.call(ADD_ROOT_FILENAME_LIST, &args![array, list]);
                e.set_global::<u8>(BUILDING_ROOT_FLAG, 0);
                e.call(REGISTER_ROOT_IDLE_ARRAY, &args![this, path, array]);
                result = array;
            }
        }
    }
    result
}

/// `new RootIdleArray(path)`: allocates `0x68` bytes (`00401000`) and, if that worked,
/// constructs the array in them (`005ff890`); null otherwise.
fn new_root_idle_array(e: &mut Engine, path: Ptr) -> u32 {
    let block = e.call(ALLOCATE, &args![0x68u32]).u32();
    if block != 0 {
        e.call(ROOT_IDLE_ARRAY_CONSTRUCTOR, &args![block, path])
            .u32()
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_0700, tes_idle_manager_get_root_filename_list(Ptr, Ptr, u32) -> u32),
        entry!(0x0060_0170, tes_idle_manager_add_root_idle_array(Ptr, Ptr, Ptr, u8) -> u32),
        entry!(0x0060_0b00, fn_00600b00(Ptr)),
        entry!(0x005f_fbd0, fn_005ffbd0(Ptr)),
        entry!(0x0060_0d70, fn_00600d70(Ptr)),
    ]
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
    fn fwd_a03_005ffbd0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0b90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_fbd0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0b90), vec![vec![0x5000]]);
    }

    #[test]
    fn fwd_a03_00600d70_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0060_0d90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_0d70, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0060_0d90), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00600b00_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0060_0d90, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0060_0b00, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0060_0d90)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0104_a040);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0104_a040,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text.as_bytes());
        address
    }

    /// Doubles for the string helpers, working on game memory.
    fn register_strings(e: &mut Engine) {
        e.map(IDLE_ANIMS_TEXT, 16);
        e.mem.set_cstr(IDLE_ANIMS_TEXT, b"IdleAnims");
        e.register(STRING_COPY, |e, a| {
            let mut source = e.mem.cstr(a[2]);
            source.truncate(a[1] as usize - 1);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| u32::from(*b) == a[1]) {
                Some(index) => (a[0] + index as u32).into_ret(),
                None => Ret::default(),
            }
        });
    }

    #[test]
    fn at_00600700_looks_the_idle_anims_key_up_and_indexes_the_entry() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        let keys: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let entry = e.mem.alloc(0x60);
        e.mem.set_u32(entry + 0x38 + 8, 0xfeed_0002);
        let sink = keys.clone();
        e.register_double(MAP_LOOKUP, move |e, a| {
            sink.borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
            e.mem.set_u32(a[2], entry);
            1u32.into_ret()
        });
        let manager = e.mem.alloc(0x20);
        let path = put_text(&mut e, "meshes\\characters\\_1stperson\\idle.kf");
        let result = e.call(0x0060_0700, &args![manager, path, 2u32]).u32();
        assert_eq!(result, 0xfeed_0002);
        assert_eq!(
            *keys.borrow(),
            vec!["meshes\\characters\\_1stperson\\IdleAnims".to_string()]
        );
        let lookup = &calls(&e, MAP_LOOKUP)[0];
        assert_eq!(lookup[0], manager);
    }

    #[test]
    fn at_00600700_answers_null_for_missing_paths_and_lookups() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        e.register(MAP_LOOKUP, |_, _| Ret::default());
        let manager = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0060_0700, &args![manager, 0u32, 0u32]).u32(), 0);
        let empty = put_text(&mut e, "");
        assert_eq!(e.call(0x0060_0700, &args![manager, empty, 0u32]).u32(), 0);
        assert!(calls(&e, MAP_LOOKUP).is_empty());
        let path = put_text(&mut e, "a\\b.kf");
        // Not found.
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
        // Found, but the entry is null.
        e.register(MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
        // Not found even though the entry cell was filled in.
        let entry = e.mem.alloc(0x60);
        e.register_double(MAP_LOOKUP, move |e, a| {
            e.mem.set_u32(a[2], entry);
            Ret::default()
        });
        assert_eq!(e.call(0x0060_0700, &args![manager, path, 0u32]).u32(), 0);
    }

    fn rig_add() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        register_strings(&mut e);
        e.map(MODEL_LOADER, 4);
        e.map(BUILDING_ROOT_FLAG, 4);
        e.set_global(MODEL_LOADER, 0x00c0_ffee);
        let manager = e.mem.alloc(0x20);
        let array_block = e.mem.alloc(0x68);
        e.register(GET_ROOT_IDLE_ARRAY, |_, _| Ret::default());
        e.register_double(ALLOCATE, move |_, a| {
            assert_eq!(a, [0x68]);
            array_block.into_ret()
        });
        e.register(ROOT_IDLE_ARRAY_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(ADD_ROOT_FILENAME_LIST, |_, _| Ret::default());
        e.register(MODEL_LOADER_COPY_FILENAME_LIST, |_, _| 0x1111u32.into_ret());
        e.register(MODEL_LOADER_BUILD_KF_FILE_LIST, |_, _| 0x2222u32.into_ret());
        e.register(REGISTER_ROOT_IDLE_ARRAY, |_, _| Ret::default());
        (e, manager, array_block)
    }

    #[test]
    fn at_00600170_answers_the_registered_array_without_building() {
        let (mut e, manager, _) = rig_add();
        e.register(GET_ROOT_IDLE_ARRAY, |_, _| 0x7777u32.into_ret());
        let path = put_text(&mut e, "a\\b.kf");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u32]).u32(),
            0x7777
        );
        assert_eq!(calls(&e, GET_ROOT_IDLE_ARRAY), vec![vec![manager, path]]);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn at_00600170_does_nothing_for_a_missing_path_or_while_building() {
        let (mut e, manager, _) = rig_add();
        assert_eq!(
            e.call(0x0060_0170, &args![manager, 0u32, 0u32, 0u32]).u32(),
            0
        );
        let empty = put_text(&mut e, "");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, empty, 0u32, 1u8]).u32(),
            0
        );
        let path = put_text(&mut e, "a\\b.kf");
        e.set_global::<u8>(BUILDING_ROOT_FLAG, 1);
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32(),
            0
        );
        assert!(calls(&e, GET_ROOT_IDLE_ARRAY).is_empty());
        // Forced, the flag is ignored.
        let array = e.call(0x0060_0170, &args![manager, path, 0u32, 1u8]).u32();
        assert_ne!(array, 0);
        assert_eq!(calls(&e, GET_ROOT_IDLE_ARRAY).len(), 1);
    }

    #[test]
    fn at_00600170_builds_the_array_from_a_given_file_list() {
        let (mut e, manager, block) = rig_add();
        let path = put_text(&mut e, "meshes\\actors\\idle.kf");
        let list = e.mem.alloc(8);
        let array = e.call(0x0060_0170, &args![manager, path, list, 0u8]).u32();
        assert_eq!(array, block);
        assert_eq!(
            calls(&e, ROOT_IDLE_ARRAY_CONSTRUCTOR),
            vec![vec![block, path]]
        );
        assert_eq!(
            calls(&e, MODEL_LOADER_COPY_FILENAME_LIST),
            vec![vec![0x00c0_ffee, list, 0]]
        );
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![block, 0x1111]]);
        let register = &calls(&e, REGISTER_ROOT_IDLE_ARRAY)[0];
        assert_eq!(register[0], manager);
        assert_eq!(register[2], block);
        assert_eq!(
            String::from_utf8(e.mem.cstr(register[1])).unwrap(),
            "meshes\\actors\\IdleAnims"
        );
        assert!(calls(&e, MODEL_LOADER_BUILD_KF_FILE_LIST).is_empty());
        assert_eq!(e.global::<u8>(BUILDING_ROOT_FLAG), 0);
    }

    #[test]
    fn at_00600170_builds_the_array_from_the_animation_files_without_a_list() {
        let (mut e, manager, block) = rig_add();
        let path = put_text(&mut e, "meshes\\actors\\idle.kf");
        let flag_seen = Rc::new(RefCell::new(Vec::new()));
        let sink = flag_seen.clone();
        e.register_double(MODEL_LOADER_BUILD_KF_FILE_LIST, move |e, _| {
            sink.borrow_mut().push(e.global::<u8>(BUILDING_ROOT_FLAG));
            0x2222u32.into_ret()
        });
        let array = e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32();
        assert_eq!(array, block);
        assert_eq!(*flag_seen.borrow(), vec![1]);
        assert_eq!(e.global::<u8>(BUILDING_ROOT_FLAG), 0);
        assert_eq!(
            calls(&e, MODEL_LOADER_BUILD_KF_FILE_LIST),
            vec![vec![0x00c0_ffee, path, 1, 0, 0]]
        );
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![block, 0x2222]]);
        assert_eq!(
            calls(&e, REGISTER_ROOT_IDLE_ARRAY),
            vec![vec![manager, path, block]]
        );
        assert!(calls(&e, MODEL_LOADER_COPY_FILENAME_LIST).is_empty());
    }

    #[test]
    fn at_00600170_a_failed_allocation_registers_a_null_array() {
        let (mut e, manager, _) = rig_add();
        e.register(ALLOCATE, |_, _| Ret::default());
        let path = put_text(&mut e, "a\\b.kf");
        assert_eq!(
            e.call(0x0060_0170, &args![manager, path, 0u32, 0u8]).u32(),
            0
        );
        assert!(calls(&e, ROOT_IDLE_ARRAY_CONSTRUCTOR).is_empty());
        assert_eq!(calls(&e, ADD_ROOT_FILENAME_LIST), vec![vec![0, 0x2222]]);
        assert_eq!(
            calls(&e, REGISTER_ROOT_IDLE_ARRAY),
            vec![vec![manager, path, 0]]
        );
    }
}
