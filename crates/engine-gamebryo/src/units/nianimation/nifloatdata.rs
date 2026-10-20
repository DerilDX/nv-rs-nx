//! `NiAnimation/nifloatdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiFloatData` (Xbox PDB, 0x18 bytes; PC offsets as the code uses them):
//! `NiObject`, then `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c, `m_eType` +0x10
//! and `m_ucKeySize` +0x14. The key arrays are handled through the
//! animation key tables in `.data`, indexed by the key type (filled by the
//! static initializers): `011f38a0` (a `cdecl` function deleting a key
//! array), `011f3758` (the key size, one byte per type), `011f3db0` (a
//! `cdecl` function `(stream, count)` that reads a key array and returns
//! it) and `011f3780` (a `cdecl` function `(keys, count, key size)` run on
//! the array read). Their Xbox PDB names are not confirmed.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiFloatData` (RTTI `.?AVNiFloatData@@`).
const NI_FLOAT_DATA_VTABLE: u32 = 0x0102_dabc;
/// `NiFloatData` size (Xbox PDB and the `CreateObject` allocation).
const NI_FLOAT_DATA_SIZE: u32 = 0x18;
/// `m_uiNumKeys`, `m_pkKeys`, `m_eType`, `m_ucKeySize` (Xbox PDB).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;

/// The animation key tables, indexed by key type (see the module comment).
const KEY_DELETE_TABLE: u32 = 0x011f_38a0;
const KEY_SIZE_TABLE: u32 = 0x011f_3758;
const KEY_LOAD_TABLE: u32 = 0x011f_3db0;
const KEY_AFTER_LOAD_TABLE: u32 = 0x011f_3780;

/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject::NiObject` (`00a5d3a0`, `NiMain/niobject.cpp`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject::~NiObject` (`00a5d3d0`, `NiMain/niobject.cpp`).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`, `NiMain/niobject.cpp`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;

/// `NiStream::m_pkIstr` (Xbox PDB) +0x24c, and the input stream's
/// `m_uiAbsoluteCurrentPos` (+4) and `m_pfnRead` (+8, `cdecl`
/// `(stream, buffer, size, &component size, component count)`, returns the
/// bytes read).
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// Deletes the key array (when there is one) through the delete function
/// of its current key type.
fn delete_keys(e: &mut Engine, this: u32) {
    let keys = e.mem.u32(this.wrapping_add(KEYS));
    if keys != 0 {
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let delete = e
            .mem
            .u32(KEY_DELETE_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

// Translated from 00a3cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::~NiFloatData` (unnamed in the engine map; the
/// destructor its scalar deleting destructor `0052cf9a` calls): stores the
/// vtable, deletes the key array through the delete function of its key
/// type, then the `NiObject` destructor (`00a5d3d0`, a tail jump).
pub fn ni_float_data_destruct(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, NI_FLOAT_DATA_VTABLE);
    delete_keys(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a3cd00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::CreateObject` (the loader `00a2d8a0` registers for
/// `"NiFloatData"`; Ghidra has no function here): allocates 0x18 bytes
/// (`00aa13e0`); when that succeeds runs the `NiObject` constructor, clears
/// the key count, keys and type and the key size byte, and stores the
/// vtable. Returns the object, or 0.
pub fn ni_float_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_FLOAT_DATA_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object.wrapping_add(NUM_KEYS), 0);
    e.mem.set_u32(object.wrapping_add(KEYS), 0);
    e.mem.set_u32(object.wrapping_add(KEY_TYPE), 0);
    e.mem.set_u8(object.wrapping_add(KEY_SIZE), 0);
    e.mem.set_u32(object, NI_FLOAT_DATA_VTABLE);
    Ptr::new(object)
}

// Translated from 00a3cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::ReplaceAnim` (Xbox PDB): deletes the current key array
/// (through its type's delete function), then takes `keys`, `num_keys` and
/// `key_type` with the type's key size when all three are non-zero, and
/// clears all four fields otherwise.
pub fn ni_float_data_replace_anim(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    num_keys: u32,
    key_type: u32,
) {
    let this = this.addr();
    delete_keys(e, this);
    if keys != 0 && num_keys != 0 && key_type != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
    }
}

/// One 4-byte read through the stream's `m_pfnRead` into `buffer`, the
/// component size word at `component_size` set to 4 first; advances the
/// position by the bytes read.
fn read_word(e: &mut Engine, stream: u32, buffer: u32, component_size: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(component_size, 4);
    let bytes = e
        .call(read, &args![input, buffer, 4u32, component_size, 1u32])
        .u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a3cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the
/// key count (4 bytes, read into the argument's own stack word). With keys,
/// the key type (4 bytes) follows; the type's key size is stored, the
/// type's load function reads the array (`(stream, count)`), the type's
/// after-load function runs on it (`(keys, count, key size)`), and
/// `ReplaceAnim` takes it. A count of 0 reads nothing more.
pub fn ni_float_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    // The frame: the component size word +0, the key type +4, and the
    // argument's own word (the key count) +8.
    e.with_stack(12, |e, frame| {
        let component_size = frame.addr();
        let key_type_word = component_size + 4;
        let num_keys_word = component_size + 8;
        e.mem.set_u32(num_keys_word, stream);
        read_word(e, stream, num_keys_word, component_size);
        let num_keys = e.mem.u32(num_keys_word);
        if num_keys == 0 {
            return;
        }
        read_word(e, stream, key_type_word, component_size);
        let key_type = e.mem.u32(key_type_word);
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u8(this.addr().wrapping_add(KEY_SIZE), key_size);
        let load = e
            .mem
            .u32(KEY_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(load, &args![stream, num_keys]).u32();
        let after_load = e
            .mem
            .u32(KEY_AFTER_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let key_size = e.mem.u8(this.addr().wrapping_add(KEY_SIZE));
        e.call(after_load, &args![keys, num_keys, u32::from(key_size)]);
        ni_float_data_replace_anim(e, this, keys, num_keys, key_type);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_cc80, ni_float_data_destruct(Ptr)),
        entry!(0x00a3_cd00, ni_float_data_create_object() -> Ptr),
        entry!(0x00a3_cdc0, ni_float_data_replace_anim(Ptr, u32, u32, u32)),
        entry!(0x00a3_cf10, ni_float_data_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE: u32 = 0x00ee_0100;
    const LOAD: u32 = 0x00ee_0200;
    const AFTER_LOAD: u32 = 0x00ee_0300;
    const READ: u32 = 0x00ee_0400;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// Key type 3: delete `DELETE`, size 0x0c, load `LOAD`, after-load
    /// `AFTER_LOAD`.
    fn tables(e: &mut Engine, log: &Log) {
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.map(KEY_LOAD_TABLE, 0x40);
        e.map(KEY_AFTER_LOAD_TABLE, 0x40);
        e.mem.set_u8(KEY_SIZE_TABLE + 3, 0x0c);
        e.mem.set_u32(KEY_DELETE_TABLE + 12, DELETE);
        e.mem.set_u32(KEY_LOAD_TABLE + 12, LOAD);
        e.mem.set_u32(KEY_AFTER_LOAD_TABLE + 12, AFTER_LOAD);
        logged(e, log, DELETE, 0);
        logged(e, log, LOAD, 0x7000);
        logged(e, log, AFTER_LOAD, 0);
    }

    #[test]
    fn destructor_deletes_the_keys_then_runs_the_object_destructor() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + KEYS, 0x5000);
        e.mem.set_u32(this + KEY_TYPE, 3);
        e.call(0x00a3_cc80, &args![this]);
        assert_eq!(e.mem.u32(this), NI_FLOAT_DATA_VTABLE);
        assert_eq!(
            *log.borrow(),
            [(DELETE, vec![0x5000]), (NI_OBJECT_DESTRUCT, vec![this])]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + KEYS, 0);
        e.call(0x00a3_cc80, &args![this]);
        assert_eq!(*log.borrow(), [(NI_OBJECT_DESTRUCT, vec![this])]);
    }

    #[test]
    fn create_object_builds_an_empty_data_object_or_returns_null() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, NI_OBJECT_CONSTRUCT, block);
        assert_eq!(e.call(0x00a3_cd00, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLOAT_DATA_VTABLE);
        assert_eq!(e.mem.bytes(block + 8, 13), vec![0; 13]);
        assert_eq!(e.mem.u8(block + 0x15), 0xaa);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (NI_OBJECT_CONSTRUCT, vec![block])
            ]
        );
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a3_cd00, &args![]).u32(), 0);
        assert_eq!(*log.borrow(), [(NI_ALLOCATE, vec![0x18])]);
    }

    #[test]
    fn replace_anim_takes_the_new_keys_only_when_all_three_are_set() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_cdc0, &args![this, 0x6000u32, 5u32, 3u32]);
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);
        assert!(log.borrow().is_empty());
        for (keys, count, key_type) in [(0u32, 5u32, 3u32), (0x6100, 0, 3), (0x6100, 5, 0)] {
            e.mem.set_u32(this + KEYS, 0x6000);
            e.mem.set_u32(this + KEY_TYPE, 3);
            e.mem.set_u32(this + NUM_KEYS, 5);
            log.borrow_mut().clear();
            e.call(0x00a3_cdc0, &args![this, keys, count, key_type]);
            assert_eq!(*log.borrow(), [(DELETE, vec![0x6000])]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
    }

    /// `LoadBinary` on a stream whose reads hand out `words` in turn.
    fn load(words: &'static [u32]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[0], a[2], e.mem.u32(a[3]), a[4]]));
            e.mem.set_u32(a[1], *next.next().unwrap());
            ret(4)
        });
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_cf10, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_count_type_and_keys() {
        let (e, log, this, stream) = load(&[5, 3]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 8);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
                (READ, vec![input, 4, 4, 1]),
                (LOAD, vec![stream, 5]),
                (AFTER_LOAD, vec![0x7000, 5, 0x0c]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x7000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);
    }

    #[test]
    fn load_binary_without_keys_reads_only_the_count() {
        let (e, log, this, stream) = load(&[0]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0);
    }
}
