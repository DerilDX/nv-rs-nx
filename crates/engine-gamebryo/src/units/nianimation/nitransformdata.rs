//! `NiAnimation/nitransformdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTransformData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRANSFORM_DATA_RTTI: u32 = 0x011f_3fbc;

// Translated from 00a4e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_transform_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRANSFORM_DATA_RTTI)
}

// Translated from 00a4e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a4e270`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_transform_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_e270;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

layout! {
    /// `NiTransformData` (Xbox PDB), 0x2c bytes on PC as on the Xbox: an `NiObject` and the
    /// rotation, position and scale key lists (counts, key types, key sizes, keys).
    pub struct NiTransformData: 0x2C {
        /// `m_usNumRotKeys` (Xbox PDB).
        0x08 m_usNumRotKeys: u16,
        /// `m_usNumPosKeys` (Xbox PDB).
        0x0A m_usNumPosKeys: u16,
        /// `m_usNumScaleKeys` (Xbox PDB).
        0x0C m_usNumScaleKeys: u16,
        /// `m_eRotType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eRotType: u32,
        /// `m_ePosType` (Xbox PDB).
        0x14 m_ePosType: u32,
        /// `m_eScaleType` (Xbox PDB).
        0x18 m_eScaleType: u32,
        /// `m_ucRotSize` (Xbox PDB).
        0x1C m_ucRotSize: u8,
        /// `m_ucPosSize` (Xbox PDB).
        0x1D m_ucPosSize: u8,
        /// `m_ucScaleSize` (Xbox PDB).
        0x1E m_ucScaleSize: u8,
        /// `m_pkRotKeys` (Xbox PDB): `NiRotKey *`.
        0x20 m_pkRotKeys: u32,
        /// `m_pkPosKeys` (Xbox PDB): `NiPosKey *`.
        0x24 m_pkPosKeys: u32,
        /// `m_pkScaleKeys` (Xbox PDB): `NiFloatKey *`.
        0x28 m_pkScaleKeys: u32,
    }
}

/// `NiTransformData`'s vtable.
const NI_TRANSFORM_DATA_VTABLE: u32 = 0x0109_9004;
/// `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor and destructor body (`thiscall`, no stack words) and its
/// `LoadBinary` (`thiscall`, the stream).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a29ad0` (`thiscall` on the keys): what the rotation keys of type 4 (Euler keys) go
/// through before they are deleted.
const EULER_KEYS_RELEASE: u32 = 0x00a2_9ad0;
/// The animation key function tables, one word per key type: the delete functions
/// (`cdecl`, the keys) of the rotation (`011f38d0`), position (`011f38b8`) and scale
/// (`011f38a0`) keys; the create functions (`cdecl`, (stream, count), answer the keys) at
/// `011f3de0`, `011f3dc8` and `011f3db0`; the load functions (`cdecl`, (keys, count, size))
/// at `011f37b0`, `011f3798` and `011f3780`; and the key size bytes at `011f3764`, `011f375e`
/// and `011f3758`.
const ROT_DELETE: u32 = 0x011f_38d0;
const POS_DELETE: u32 = 0x011f_38b8;
const SCALE_DELETE: u32 = 0x011f_38a0;
const ROT_CREATE: u32 = 0x011f_3de0;
const POS_CREATE: u32 = 0x011f_3dc8;
const SCALE_CREATE: u32 = 0x011f_3db0;
const ROT_LOAD: u32 = 0x011f_37b0;
const POS_LOAD: u32 = 0x011f_3798;
const SCALE_LOAD: u32 = 0x011f_3780;
const ROT_SIZE: u32 = 0x011f_3764;
const POS_SIZE: u32 = 0x011f_375e;
const SCALE_SIZE: u32 = 0x011f_3758;
/// `00a29680` (`cdecl`, the message): the warning the loader gives for more than 65535 keys,
/// with the messages `"Transform data loading has more than 65535 rotation keys!"`
/// (`01099104`), `"... position keys!"` (`010990c8`) and `"... scale keys!"` (`01099090`).
const WARNING: u32 = 0x00a2_9680;
const TOO_MANY_ROTATION_KEYS: u32 = 0x0109_9104;
const TOO_MANY_POSITION_KEYS: u32 = 0x0109_90c8;
const TOO_MANY_SCALE_KEYS: u32 = 0x0109_9090;

/// Deletes the rotation keys through the delete function of their type (keys of type 4
/// first through `00a29ad0`), when there are keys.
fn delete_rotation_keys(e: &mut Engine, this: Ptr<NiTransformData>) {
    let keys = e.get(this, NiTransformData::m_pkRotKeys);
    if keys != 0 {
        if e.get(this, NiTransformData::m_eRotType) == 4 {
            e.call(EULER_KEYS_RELEASE, &args![keys]);
        }
        let kind = e.get(this, NiTransformData::m_eRotType);
        let delete = e.mem.u32(ROT_DELETE.wrapping_add(kind.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

/// Deletes the keys at `keys_field` through the delete function of the type at `type_field`
/// in `table`, when there are keys.
fn delete_keys(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    keys_field: Field<NiTransformData, u32>,
    type_field: Field<NiTransformData, u32>,
    table: u32,
) {
    let keys = e.get(this, keys_field);
    if keys != 0 {
        let kind = e.get(this, type_field);
        let delete = e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

// Translated from 00a4e040 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiTransformData`): allocates 0x2c bytes (`operator new`); when that
/// succeeds, the `NiObject` constructor, every key list empty (counts, types, sizes and key
/// pointers 0) and the vtable `01099004`. Answers the object, or null.
pub fn fn_00a4e040(e: &mut Engine) -> Ptr<NiTransformData> {
    let block = e.call(OPERATOR_NEW, &args![0x2cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(OBJECT_CONSTRUCT, &args![block]);
    let this = Ptr::<NiTransformData>::new(block);
    e.set(this, NiTransformData::m_usNumRotKeys, 0);
    e.set(this, NiTransformData::m_pkRotKeys, 0);
    e.set(this, NiTransformData::m_eRotType, 0);
    e.set(this, NiTransformData::m_ucRotSize, 0);
    e.set(this, NiTransformData::m_pkPosKeys, 0);
    e.set(this, NiTransformData::m_ePosType, 0);
    e.set(this, NiTransformData::m_ucPosSize, 0);
    e.set(this, NiTransformData::m_pkScaleKeys, 0);
    e.set(this, NiTransformData::m_eScaleType, 0);
    e.set(this, NiTransformData::m_ucScaleSize, 0);
    e.mem.set_u32(block, NI_TRANSFORM_DATA_VTABLE);
    e.set(this, NiTransformData::m_usNumPosKeys, 0);
    e.set(this, NiTransformData::m_usNumScaleKeys, 0);
    this
}

// Translated from 00a4e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; the destructor body): stores the
/// vtable `01099004`, deletes the rotation, position and scale keys through their types'
/// delete functions (rotation keys of type 4 first through `00a29ad0`) and runs the
/// `NiObject` destructor body (`00a5d3d0`).
pub fn fn_00a4e270(e: &mut Engine, this: Ptr<NiTransformData>) {
    e.mem.set_u32(this.addr(), NI_TRANSFORM_DATA_VTABLE);
    delete_rotation_keys(e, this);
    delete_keys(
        e,
        this,
        NiTransformData::m_pkPosKeys,
        NiTransformData::m_ePosType,
        POS_DELETE,
    );
    delete_keys(
        e,
        this,
        NiTransformData::m_pkScaleKeys,
        NiTransformData::m_eScaleType,
        SCALE_DELETE,
    );
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a4e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the rotation key
/// replacement): deletes the current rotation keys (as the destructor does) and, when `keys`
/// and `count` are both non-zero, takes them with `kind` and that type's key size
/// (`011f3764`); otherwise the rotation list is left empty (count, keys, type and size 0).
pub fn fn_00a4e2e0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_rotation_keys(e, this);
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumRotKeys, count as u16);
        e.set(this, NiTransformData::m_pkRotKeys, keys);
        e.set(this, NiTransformData::m_eRotType, kind);
        let size = e.mem.u8(kind.wrapping_add(ROT_SIZE));
        e.set(this, NiTransformData::m_ucRotSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumRotKeys, 0);
        e.set(this, NiTransformData::m_pkRotKeys, 0);
        e.set(this, NiTransformData::m_eRotType, 0);
        e.set(this, NiTransformData::m_ucRotSize, 0);
    }
}

// Translated from 00a4e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the position key
/// replacement): deletes the current position keys through their type's delete function
/// and, when `keys` and `count` are both non-zero, takes them with `kind` and that type's key
/// size (`011f375e`); otherwise the position list is left empty.
pub fn fn_00a4e350(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_keys(
        e,
        this,
        NiTransformData::m_pkPosKeys,
        NiTransformData::m_ePosType,
        POS_DELETE,
    );
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumPosKeys, count as u16);
        e.set(this, NiTransformData::m_pkPosKeys, keys);
        e.set(this, NiTransformData::m_ePosType, kind);
        let size = e.mem.u8(kind.wrapping_add(POS_SIZE));
        e.set(this, NiTransformData::m_ucPosSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumPosKeys, 0);
        e.set(this, NiTransformData::m_pkPosKeys, 0);
        e.set(this, NiTransformData::m_ePosType, 0);
        e.set(this, NiTransformData::m_ucPosSize, 0);
    }
}

// Translated from 00a4e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransformdata.cpp`; by its body the scale key
/// replacement): deletes the current scale keys through their type's delete function and,
/// when `keys` and `count` are both non-zero, takes them with `kind` and that type's key size
/// (`011f3758`); otherwise the scale list is left empty.
pub fn fn_00a4e3b0(e: &mut Engine, this: Ptr<NiTransformData>, keys: u32, count: u32, kind: u32) {
    delete_keys(
        e,
        this,
        NiTransformData::m_pkScaleKeys,
        NiTransformData::m_eScaleType,
        SCALE_DELETE,
    );
    if keys != 0 && count != 0 {
        e.set(this, NiTransformData::m_usNumScaleKeys, count as u16);
        e.set(this, NiTransformData::m_pkScaleKeys, keys);
        e.set(this, NiTransformData::m_eScaleType, kind);
        let size = e.mem.u8(kind.wrapping_add(SCALE_SIZE));
        e.set(this, NiTransformData::m_ucScaleSize, size);
    } else {
        e.set(this, NiTransformData::m_usNumScaleKeys, 0);
        e.set(this, NiTransformData::m_pkScaleKeys, 0);
        e.set(this, NiTransformData::m_ucScaleSize, 0);
        e.set(this, NiTransformData::m_eScaleType, 0);
    }
}

/// Reads 4 bytes from the stream's input into `destination`, the byte count word at
/// `count` set to 4 first (`function(input, destination, 4, count, 1)`; the answer is added
/// to the input's position).
fn read_word(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// One key list of `LoadBinary`: the stack words it uses (the key count in the stream
/// argument's slot, the type and the byte-count word), its tables and its message.
struct KeyList {
    kind_slot: u32,
    count_slot: u32,
    /// The local whose low byte the key size byte is stored into (the type's own slot for
    /// rotations, the byte-count word otherwise).
    size_slot: u32,
    size_table: u32,
    create_table: u32,
    load_table: u32,
    message: u32,
    replace: fn(&mut Engine, Ptr<NiTransformData>, u32, u32, u32),
}

/// Reads one key list: its count; when not 0 its type, the keys made by the type's create
/// function (`(stream, count)`) and filled by its load function (`(keys, count, size
/// word)`, the size word being the slot the key size byte was stored into), the count cut to
/// 65535 with a warning, and the list replaced.
fn load_key_list(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    stream: Ptr<NiStream>,
    count_word: u32,
    list: &KeyList,
) {
    read_word(e, stream, count_word, list.count_slot);
    if e.mem.u32(count_word) == 0 {
        return;
    }
    read_word(e, stream, list.kind_slot, list.count_slot);
    let kind = e.mem.u32(list.kind_slot);
    let size = e.mem.u8(kind.wrapping_add(list.size_table));
    let count = e.mem.u32(count_word);
    let size_slot = list.size_slot;
    e.mem.set_u8(size_slot, size);
    let create = e
        .mem
        .u32(list.create_table.wrapping_add(kind.wrapping_mul(4)));
    let keys = e.call(create, &args![stream, count]).u32();
    let size_word = e.mem.u32(size_slot);
    let load = e
        .mem
        .u32(list.load_table.wrapping_add(kind.wrapping_mul(4)));
    e.call(load, &args![keys, count, size_word]);
    if e.mem.u32(count_word) > 0xffff {
        e.mem.set_u32(count_word, 0xffff);
        e.call(WARNING, &args![list.message]);
    }
    let count = e.mem.u32(count_word);
    (list.replace)(e, this, keys, count, kind);
}

// Translated from 00a4e5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the rotation,
/// position and scale key lists in turn, each as a 4-byte count and, when not 0, a 4-byte key
/// type; the keys are made and read through the key type's create and load functions
/// (`011f3de0`/`011f37b0`, `011f3dc8`/`011f3798`, `011f3db0`/`011f3780`), the count is cut to
/// 65535 with a warning (`00a29680`) and the list replaced (`00a4e2e0`, `00a4e350`,
/// `00a4e3b0`). The rotation list's load function is given the key size byte over the type
/// word's low byte; the other two the size byte alone (their byte-count word was 4).
pub fn ni_transform_data_load_binary(
    e: &mut Engine,
    this: Ptr<NiTransformData>,
    stream: Ptr<NiStream>,
) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    // Stack words: the stream argument's slot (the counts) and two locals.
    e.with_stack(0xc, |e, slots| {
        let count_word = slots.addr();
        let first = slots.addr() + 4;
        let second = slots.addr() + 8;
        let rotation = KeyList {
            kind_slot: second,
            count_slot: first,
            size_slot: second,
            size_table: ROT_SIZE,
            create_table: ROT_CREATE,
            load_table: ROT_LOAD,
            message: TOO_MANY_ROTATION_KEYS,
            replace: fn_00a4e2e0,
        };
        let position = KeyList {
            kind_slot: first,
            count_slot: second,
            size_slot: second,
            size_table: POS_SIZE,
            create_table: POS_CREATE,
            load_table: POS_LOAD,
            message: TOO_MANY_POSITION_KEYS,
            replace: fn_00a4e350,
        };
        let scale = KeyList {
            kind_slot: first,
            count_slot: second,
            size_slot: second,
            size_table: SCALE_SIZE,
            create_table: SCALE_CREATE,
            load_table: SCALE_LOAD,
            message: TOO_MANY_SCALE_KEYS,
            replace: fn_00a4e3b0,
        };
        load_key_list(e, this, stream, count_word, &rotation);
        load_key_list(e, this, stream, count_word, &position);
        load_key_list(e, this, stream, count_word, &scale);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4e0a0, ni_transform_data_get_rtti() -> Ptr),
        entry!(0x00a4e5b0, ni_transform_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_e040, fn_00a4e040() -> Ptr<NiTransformData>),
        entry!(0x00a4_e270, fn_00a4e270(Ptr<NiTransformData>)),
        entry!(
            0x00a4_e2e0,
            fn_00a4e2e0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e350,
            fn_00a4e350(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e3b0,
            fn_00a4e3b0(Ptr<NiTransformData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_e5d0,
            ni_transform_data_load_binary(Ptr<NiTransformData>, Ptr<NiStream>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_transform_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4e0a0, &args![]).u32(), NI_TRANSFORM_DATA_RTTI);
    }

    #[test]
    fn ni_transform_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_e270, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a4e5b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4e5b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00a4_e270), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod key_list_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The delete functions of key types 0 to 4 for each list: `0x0300_0r0t` with `r` the
    /// list (1 rotation, 2 position, 3 scale) and `t` the type; the key size bytes are
    /// `0x10 + type` for every list.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x2000);
        for kind in 0..5u32 {
            for (list, table) in [(1u32, ROT_DELETE), (2, POS_DELETE), (3, SCALE_DELETE)] {
                let double = 0x0300_0000 + (list << 8) + kind;
                e.mem.set_u32(table + 4 * kind, double);
                e.register(double, |_, _| Ret::default());
            }
            for table in [ROT_SIZE, POS_SIZE, SCALE_SIZE] {
                e.mem.set_u8(table + kind, 0x10 + kind as u8);
            }
        }
        e.register(EULER_KEYS_RELEASE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_object_00a4e040_builds_empty_data() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x2c]);
            block.into_ret()
        });
        e.register(OBJECT_CONSTRUCT, |_, _| Ret::default());
        let data = e.call(0x00a4_e040, &args![]).u32();
        assert_eq!(e.mem.u32(data), NI_TRANSFORM_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 4), 0xeeee_eeee);
        for offset in (8..0x2c).step_by(4) {
            let expected = match offset {
                0xc => 0xeeee_0000,
                0x1c => 0xee00_0000,
                _ => 0,
            };
            assert_eq!(e.mem.u32(data + offset), expected, "{offset:#x}");
        }
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_e040, &args![]).u32(), 0);
    }

    #[test]
    fn destructor_body_deletes_every_key_list() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkRotKeys, 0x100);
        e.set(this, NiTransformData::m_eRotType, 4);
        e.set(this, NiTransformData::m_pkScaleKeys, 0x300);
        e.set(this, NiTransformData::m_eScaleType, 2);
        e.call(0x00a4_e270, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_TRANSFORM_DATA_VTABLE);
        assert_eq!(calls_to(&e, EULER_KEYS_RELEASE), vec![vec![0x100]]);
        assert_eq!(calls_to(&e, 0x0300_0104), vec![vec![0x100]]);
        assert_eq!(calls_to(&e, 0x0300_0302), vec![vec![0x300]]);
        assert!(calls_to(&e, 0x0300_0200).is_empty());
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn replace_rotation_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkRotKeys, 0x100);
        e.set(this, NiTransformData::m_eRotType, 1);
        e.call(0x00a4_e2e0, &args![this, 0x500u32, 0x1_0003u32, 3u32]);
        assert!(calls_to(&e, EULER_KEYS_RELEASE).is_empty());
        assert_eq!(calls_to(&e, 0x0300_0101), vec![vec![0x100]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumRotKeys), 3);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0x500);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 3);
        assert_eq!(e.get(this, NiTransformData::m_ucRotSize), 0x13);
        e.call(0x00a4_e2e0, &args![this, 0x600u32, 0u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0300_0103), vec![vec![0x500]]);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 0);
        assert_eq!(e.get(this, NiTransformData::m_ucRotSize), 0);
    }

    #[test]
    fn replace_position_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.set(this, NiTransformData::m_pkPosKeys, 0x200);
        e.set(this, NiTransformData::m_ePosType, 2);
        e.call(0x00a4_e350, &args![this, 0x700u32, 2u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0300_0202), vec![vec![0x200]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 2);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0x700);
        assert_eq!(e.get(this, NiTransformData::m_ePosType), 1);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0x11);
        e.call(0x00a4_e350, &args![this, 0u32, 2u32, 1u32]);
        assert_eq!(e.get(this, NiTransformData::m_usNumPosKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_pkPosKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_ucPosSize), 0);
    }

    #[test]
    fn replace_scale_keys_deletes_and_takes_or_empties() {
        let mut e = engine();
        let this = e.new_object::<NiTransformData>();
        e.call(0x00a4_e3b0, &args![this, 0x800u32, 4u32, 2u32]);
        assert!(calls_to(&e, 0x0300_0300).is_empty());
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 4);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x800);
        assert_eq!(e.get(this, NiTransformData::m_eScaleType), 2);
        assert_eq!(e.get(this, NiTransformData::m_ucScaleSize), 0x12);
        e.call(0x00a4_e3b0, &args![this, 0x900u32, 0u32, 2u32]);
        assert_eq!(calls_to(&e, 0x0300_0302), vec![vec![0x800]]);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0);
        assert_eq!(e.get(this, NiTransformData::m_eScaleType), 0);
    }

    #[test]
    fn load_binary_reads_the_three_key_lists() {
        let mut e = engine();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(WARNING, |_, _| Ret::default());
        // Create functions answer 0x7000 + list; load functions record their words.
        for (list, create, load) in [
            (1u32, ROT_CREATE, ROT_LOAD),
            (2, POS_CREATE, POS_LOAD),
            (3, SCALE_CREATE, SCALE_LOAD),
        ] {
            for kind in 0..5u32 {
                e.mem.set_u32(create + 4 * kind, 0x0310_0000 + list);
                e.mem.set_u32(load + 4 * kind, 0x0320_0000 + list);
            }
            e.register_double(0x0310_0000 + list, move |_, _| (0x7000 + list).into_ret());
            e.register(0x0320_0000 + list, |_, _| Ret::default());
        }
        // The stream answers: rotation count 0x10002, type 0x103 (size byte 0x13 over it);
        // position count 0; scale count 5, type 2.
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0330_0000);
        let mut answers = std::collections::VecDeque::from(vec![0x1_0002u32, 0x103, 0, 5, 2]);
        e.register_double(0x0330_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], answers.pop_front().unwrap());
            4u32.into_ret()
        });
        e.mem.set_u8(ROT_SIZE + 0x103, 0x13);
        e.mem.set_u32(ROT_CREATE + 4 * 0x103, 0x0310_0001);
        e.mem.set_u32(ROT_LOAD + 4 * 0x103, 0x0320_0001);
        e.mem.set_u32(ROT_DELETE + 4 * 0x103, 0x0300_0100);
        let this = e.new_object::<NiTransformData>();
        e.call(0x00a4_e5d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, 0x0310_0001),
            vec![vec![stream.addr(), 0x1_0002]]
        );
        assert_eq!(
            calls_to(&e, 0x0320_0001),
            vec![vec![0x7001, 0x1_0002, 0x113]]
        );
        assert_eq!(calls_to(&e, WARNING), vec![vec![TOO_MANY_ROTATION_KEYS]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumRotKeys), 0xffff);
        assert_eq!(e.get(this, NiTransformData::m_pkRotKeys), 0x7001);
        assert_eq!(e.get(this, NiTransformData::m_eRotType), 0x103);
        assert!(calls_to(&e, 0x0310_0002).is_empty());
        assert_eq!(calls_to(&e, 0x0320_0003), vec![vec![0x7003, 5, 0x12]]);
        assert_eq!(e.get(this, NiTransformData::m_usNumScaleKeys), 5);
        assert_eq!(e.get(this, NiTransformData::m_pkScaleKeys), 0x7003);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 20);
    }
}
