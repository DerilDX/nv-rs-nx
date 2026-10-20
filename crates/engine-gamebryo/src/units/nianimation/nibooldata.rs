//! `NiAnimation/nibooldata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiBoolData` (PC offsets as the code uses them): `NiObject`, then the key
//! count +0x08, the key array +0x0c, the key type +0x10 and the key size
//! byte +0x14 (the layout of `NiFloatData`). The key arrays are handled
//! through the bool-key tables in `.data`, indexed by the key type and
//! filled by the static initializers: `011f3918` (a `cdecl` function
//! deleting a key array), `011f3776` (the key size, one byte per type),
//! `011f3e28` (a `cdecl` function `(stream, count)` that reads a key array
//! and returns it) and `011f37f8` (a `cdecl` function `(keys, count, key
//! size)` run on the array read). Their Xbox PDB names are not confirmed.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// Key count, key array, key type and key size byte (see the module comment).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;

/// The bool-key tables, indexed by key type (see the module comment).
const KEY_DELETE_TABLE: u32 = 0x011f_3918;
const KEY_SIZE_TABLE: u32 = 0x011f_3776;
const KEY_LOAD_TABLE: u32 = 0x011f_3e28;
const KEY_AFTER_LOAD_TABLE: u32 = 0x011f_37f8;

/// `NiObject::LoadBinary` (`00a5d610`, `NiMain/niobject.cpp`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; offsets as the PC code reads them), and the input stream's
/// position (+4) and read function (+8, `cdecl` `(stream, buffer, size,
/// &component size, component count)`, returns the bytes read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// The first file version whose bool data stores its key type
/// (`CMP [stream+0xd8], 0x0a010068`; older files use type 5).
const VERSION_WITH_KEY_TYPE: u32 = 0x0a01_0068;
/// The key type older files imply.
const OLD_FILE_KEY_TYPE: u32 = 5;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a538b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::ReplaceAnim` (Xbox PDB): deletes the current key array (when
/// there is one) through its type's delete function, then takes `keys`,
/// `num_keys` and `key_type` with the type's key size when `keys` and
/// `num_keys` are non-zero, and clears all four fields otherwise.
pub fn ni_bool_data_replace_anim(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    num_keys: u32,
    key_type: u32,
) {
    let this = this.addr();
    let old = e.mem.u32(this.wrapping_add(KEYS));
    if old != 0 {
        let old_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let delete = e
            .mem
            .u32(KEY_DELETE_TABLE.wrapping_add(old_type.wrapping_mul(4)));
        e.call(delete, &args![old]);
    }
    if keys != 0 && num_keys != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
    }
}

// Translated from 00a53a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the key
/// count (4 bytes, read into the argument's own stack word). With keys, the
/// key type follows (4 bytes) from file version `0x0a010068` on (type 5
/// before); the type's key size is stored, the type's load function reads
/// the array (`(stream, count)`), the type's after-load function runs on it
/// (`(keys, count, key size)`) and `ReplaceAnim` takes it. A count of 0
/// reads nothing more.
pub fn ni_bool_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    // The frame: the component size word +0, the key type +4, and the
    // argument's own word (the key count) +8.
    e.with_stack(12, |e, frame| {
        let component_size = frame.addr();
        let key_type_word = component_size + 4;
        let num_keys_word = component_size + 8;
        e.mem.set_u32(num_keys_word, stream);
        stream_read(e, stream, num_keys_word, 4, 4, component_size);
        let num_keys = e.mem.u32(num_keys_word);
        if num_keys == 0 {
            return;
        }
        let mut key_type = OLD_FILE_KEY_TYPE;
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= VERSION_WITH_KEY_TYPE {
            stream_read(e, stream, key_type_word, 4, 4, component_size);
            key_type = e.mem.u32(key_type_word);
        }
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u8(this.addr().wrapping_add(KEY_SIZE), key_size);
        let load = e
            .mem
            .u32(KEY_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(load, &args![stream, num_keys]).u32();
        let key_size = e.mem.u8(this.addr().wrapping_add(KEY_SIZE));
        let after_load = e
            .mem
            .u32(KEY_AFTER_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(after_load, &args![keys, num_keys, u32::from(key_size)]);
        ni_bool_data_replace_anim(e, this, keys, num_keys, key_type);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5_38b0, ni_bool_data_replace_anim(Ptr, u32, u32, u32)),
        entry!(0x00a5_3a50, ni_bool_data_load_binary(Ptr, Ptr)),
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

    /// Key types 3 and 5: delete `DELETE`, sizes 0x08 and 0x0c, load
    /// `LOAD`, after-load `AFTER_LOAD`.
    fn tables(e: &mut Engine, log: &Log) {
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.map(KEY_LOAD_TABLE, 0x40);
        e.map(KEY_AFTER_LOAD_TABLE, 0x40);
        for (key_type, size) in [(3u32, 0x08u8), (5, 0x0c)] {
            e.mem.set_u8(KEY_SIZE_TABLE + key_type, size);
            e.mem.set_u32(KEY_DELETE_TABLE + 4 * key_type, DELETE);
            e.mem.set_u32(KEY_LOAD_TABLE + 4 * key_type, LOAD);
            e.mem
                .set_u32(KEY_AFTER_LOAD_TABLE + 4 * key_type, AFTER_LOAD);
        }
        logged(e, log, DELETE, 0);
        logged(e, log, LOAD, 0x7000);
        logged(e, log, AFTER_LOAD, 0);
    }

    #[test]
    fn replace_anim_takes_keys_and_count_or_clears_everything() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        let this = e.mem.alloc(0x18);
        // No old keys: nothing deleted.
        e.call(0x00a5_38b0, &args![this, 0x6000u32, 5u32, 3u32]);
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x08);
        assert!(log.borrow().is_empty());
        for (keys, count) in [(0u32, 5u32), (0x6100, 0)] {
            e.mem.set_u32(this + KEYS, 0x6000);
            e.mem.set_u32(this + KEY_TYPE, 3);
            log.borrow_mut().clear();
            e.call(0x00a5_38b0, &args![this, keys, count, 3u32]);
            assert_eq!(*log.borrow(), [(DELETE, vec![0x6000])]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
    }

    /// `LoadBinary` on a stream of file `version` whose reads hand out
    /// `words` in turn.
    fn load(version: u32, words: &'static [u32]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
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
        e.call(0x00a5_3a50, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_count_and_type_in_new_files() {
        let (e, log, this, stream) = load(0x0a01_0068, &[4, 3]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 8);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
                (READ, vec![input, 4, 4, 1]),
                (LOAD, vec![stream, 4]),
                (AFTER_LOAD, vec![0x7000, 4, 0x08]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x7000);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
    }

    #[test]
    fn load_binary_uses_type_5_in_old_files_and_stops_without_keys() {
        let (e, log, this, stream) = load(0x0a01_0067, &[2]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(log.borrow()[2], (LOAD, vec![stream, 2]));
        assert_eq!(log.borrow()[3], (AFTER_LOAD, vec![0x7000, 2, 0x0c]));
        assert_eq!(e.mem.u32(this + KEY_TYPE), 5);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);

        let (e, log, this, _) = load(0x0a01_0068, &[0]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(this + KEYS), 0);
    }
}
