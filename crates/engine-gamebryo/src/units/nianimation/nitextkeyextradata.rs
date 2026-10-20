//! `NiAnimation/nitextkeyextradata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTextKeyExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_TEXT_KEY_EXTRA_DATA_RTTI: u32 = 0x011f_3f28;

// Translated from 00a46b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_text_key_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TEXT_KEY_EXTRA_DATA_RTTI)
}

/// `00a7b3b0`: the jump target of `00a9af00`.
const JUMP_TARGET_00A9AF00: u32 = 0x00a7_b3b0;

// Translated from 00a9af00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::LinkObject` (Xbox PDB): a 5-byte jump to `00a7b3b0` (engine map: `NiExtraData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiTextKeyExtraData@@UAAXAAVNiStream@@@Z`.
pub fn ni_text_key_extra_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9AF00, &args![this, stream]);
}

layout! {
    /// `NiTextKeyExtraData` (Xbox PDB), 0x14 bytes on PC: an `NiExtraData` (vtable, count,
    /// name) and its text keys (`NiTextKey`, 8 bytes each: the time, then the text as an
    /// `NiFixedString`), allocated with their count in the word before them.
    pub struct NiTextKeyExtraData: 0x14 {
        /// `m_uiNumKeys` (Xbox PDB).
        0x0C m_uiNumKeys: u32,
        /// `m_pKeys` (Xbox PDB): `NiTextKey *`.
        0x10 m_pKeys: u32,
    }
}

/// `NiTextKeyExtraData`'s vtable.
const NI_TEXT_KEY_EXTRA_DATA_VTABLE: u32 = 0x0109_827c;
/// The word holding the handle of the empty `NiFixedString` (stored uncounted).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The memory manager singleton, its `Allocate` (`thiscall`, the size) and `Deallocate`
/// (`thiscall`, the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `NiExtraData`'s constructor and destructor body (`thiscall`, no stack words), its member
/// copy (`00a7b330`, (clone, cloning process)) and `NiExtraData::LoadBinary` (the stream).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// `00a57870` (`thiscall` on a text key): its destructor; `00a57890` (`thiscall` on a text
/// key, the source fixed string): its text assignment; `NiTextKey::LoadBinary` (the stream).
const TEXT_KEY_DESTRUCT: u32 = 0x00a5_7870;
const TEXT_KEY_SET_TEXT: u32 = 0x00a5_7890;
const TEXT_KEY_LOAD_BINARY: u32 = 0x00a5_78d0;

/// The byte count of an array of `count` text keys with its count word: `count * 8 + 4`,
/// all ones when either step overflows (the compiler's `SETO`/`SETC`, `NEG`, `OR`).
fn key_array_bytes(count: u32) -> u32 {
    count
        .checked_mul(8)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX)
}

/// Allocates `count` text keys from the memory manager with the count in the word before
/// them, each built by `00a46b50`; answers the keys, or null when the allocation fails.
fn allocate_keys(e: &mut Engine, count: u32) -> u32 {
    let block = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, key_array_bytes(count)],
        )
        .u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![block + 4, 8u32, count, 0x00a4_6b50u32],
    );
    block + 4
}

// Translated from 00a46b50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body `NiTextKey`'s
/// default constructor, the one the key arrays are built with): the text the empty fixed
/// string (uncounted); the time is left as it is. Answers `this`.
pub fn fn_00a46b50(e: &mut Engine, this: Ptr) -> Ptr {
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(this.addr() + 4, empty);
    this
}

// Translated from 00a46b70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiTextKeyExtraData`): allocates 0x14 bytes (`operator new`); when that
/// succeeds, the `NiExtraData` constructor, the vtable `0109827c`, no keys. Answers the
/// object, or null.
pub fn fn_00a46b70(e: &mut Engine) -> Ptr<NiTextKeyExtraData> {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    let this = Ptr::<NiTextKeyExtraData>::new(block);
    e.mem.set_u32(block, NI_TEXT_KEY_EXTRA_DATA_VTABLE);
    e.set(this, NiTextKeyExtraData::m_uiNumKeys, 0);
    e.set(this, NiTextKeyExtraData::m_pKeys, 0);
    this
}

// Translated from 00a46c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitextkeyextradata.cpp`; by its body the member copy):
/// `NiExtraData`'s copy (`00a7b330`), the key count, and, when there are keys and this
/// object's `GetRTTI` (slot `+8`) is `NiTextKeyExtraData`'s, a new key array (as
/// `LoadBinary` makes one; null when the allocation fails) with each key's time copied and
/// its text assigned (`00a57890`); otherwise no keys.
pub fn fn_00a46c20(
    e: &mut Engine,
    this: Ptr<NiTextKeyExtraData>,
    clone: Ptr<NiTextKeyExtraData>,
    cloning_process: Ptr,
) {
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let count = e.get(this, NiTextKeyExtraData::m_uiNumKeys);
    e.set(clone, NiTextKeyExtraData::m_uiNumKeys, count);
    if e.get(this, NiTextKeyExtraData::m_uiNumKeys) == 0
        || e.vcall(this.addr(), 8, &args![]).u32() != NI_TEXT_KEY_EXTRA_DATA_RTTI
    {
        e.set(clone, NiTextKeyExtraData::m_pKeys, 0);
        return;
    }
    let count = e.get(this, NiTextKeyExtraData::m_uiNumKeys);
    let keys = allocate_keys(e, count);
    e.set(clone, NiTextKeyExtraData::m_pKeys, keys);
    let mut index = 0u32;
    while index < e.get(this, NiTextKeyExtraData::m_uiNumKeys) {
        let offset = index.wrapping_mul(8);
        let source = e
            .get(this, NiTextKeyExtraData::m_pKeys)
            .wrapping_add(offset);
        let target = e
            .get(clone, NiTextKeyExtraData::m_pKeys)
            .wrapping_add(offset);
        let time = x87_store32(x87_load32(e.mem.u32(source)));
        e.mem.set_u32(target, time);
        let source = e.get(this, NiTextKeyExtraData::m_pKeys);
        let text = source.wrapping_add(offset).wrapping_add(4);
        let target = e
            .get(clone, NiTextKeyExtraData::m_pKeys)
            .wrapping_add(offset);
        e.call(TEXT_KEY_SET_TEXT, &args![target, text]);
        index += 1;
    }
}

// Translated from 00a46d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitextkeyextradata.cpp`; the destructor body): stores
/// the vtable `0109827c`; when there are keys, runs each key's destructor (`00a57870`), last
/// first, for the count in the word before them, and gives the block back to the memory
/// manager; then the `NiExtraData` destructor body (`00a7b300`).
pub fn fn_00a46d50(e: &mut Engine, this: Ptr<NiTextKeyExtraData>) {
    e.mem.set_u32(this.addr(), NI_TEXT_KEY_EXTRA_DATA_VTABLE);
    let keys = e.get(this, NiTextKeyExtraData::m_pKeys);
    if keys != 0 {
        let count = e.mem.u32(keys.wrapping_sub(4));
        let mut key = keys.wrapping_add(count.wrapping_mul(8));
        let mut remaining = count.wrapping_sub(1) as i32;
        while remaining >= 0 {
            key = key.wrapping_sub(8);
            e.call(TEXT_KEY_DESTRUCT, &args![key]);
            remaining = remaining.wrapping_sub(1);
        }
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, keys.wrapping_sub(4)],
        );
    }
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 00a46da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::CreateClone` (Xbox PDB): a new object (0x14 bytes, `operator new`,
/// built as `00a46b70` builds one; null when the allocation fails) given this one's members
/// (`00a46c20`, which runs on null too). Answers the clone.
pub fn ni_text_key_extra_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiTextKeyExtraData>,
    cloning_process: Ptr,
) -> Ptr<NiTextKeyExtraData> {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block != 0 {
        e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_TEXT_KEY_EXTRA_DATA_VTABLE);
        e.mem.set_u32(block + 0xc, 0);
        e.mem.set_u32(block + 0x10, 0);
    }
    fn_00a46c20(e, this, Ptr::new(block), cloning_process);
    Ptr::new(block)
}

// Translated from 00a46e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`, then a 4-byte key
/// count; when it is not 0, a key array from the memory manager (`count * 8 + 4` bytes,
/// saturating; the count in its first word, each key built by `00a46b50`; null when the
/// allocation fails), each key read by `NiTextKey::LoadBinary`, and the count and keys
/// stored. A count of 0 leaves the fields as they are.
pub fn ni_text_key_extra_data_load_binary(
    e: &mut Engine,
    this: Ptr<NiTextKeyExtraData>,
    stream: Ptr<NiStream>,
) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let count = e.with_stack(8, |e, slots| {
        let count_word = slots.addr();
        let byte_count = slots.addr() + 4;
        let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        e.mem.set_u32(byte_count, 4);
        let done = e
            .call(function, &args![input, count_word, 4u32, byte_count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
        e.mem.u32(count_word)
    });
    if count == 0 {
        return;
    }
    let keys = allocate_keys(e, count);
    let mut key = keys;
    for _ in 0..count {
        e.call(TEXT_KEY_LOAD_BINARY, &args![key, stream]);
        key = key.wrapping_add(8);
    }
    e.set(this, NiTextKeyExtraData::m_uiNumKeys, count);
    e.set(this, NiTextKeyExtraData::m_pKeys, keys);
}

// Translated from 00a46ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextKeyExtraData::_scalar_deleting_destructor_` (Xbox PDB): the destructor body
/// (`00a46d50`) and, when bit 0 of `flags` is set, the sized delete of 0x14 bytes. Answers
/// `this`.
pub fn ni_text_key_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTextKeyExtraData>,
    flags: u32,
) -> Ptr<NiTextKeyExtraData> {
    fn_00a46d50(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a46b60, ni_text_key_extra_data_get_rtti() -> Ptr),
        entry!(0x00a9_af00, ni_text_key_extra_data_link_object(Ptr, Ptr)),
        entry!(0x00a4_6b50, fn_00a46b50(Ptr) -> Ptr),
        entry!(0x00a4_6b70, fn_00a46b70() -> Ptr<NiTextKeyExtraData>),
        entry!(
            0x00a4_6c20,
            fn_00a46c20(Ptr<NiTextKeyExtraData>, Ptr<NiTextKeyExtraData>, Ptr)
        ),
        entry!(0x00a4_6d50, fn_00a46d50(Ptr<NiTextKeyExtraData>)),
        entry!(
            0x00a4_6da0,
            ni_text_key_extra_data_create_clone(
                Ptr<NiTextKeyExtraData>,
                Ptr,
            ) -> Ptr<NiTextKeyExtraData>
        ),
        entry!(
            0x00a4_6e00,
            ni_text_key_extra_data_load_binary(Ptr<NiTextKeyExtraData>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_6ed0,
            ni_text_key_extra_data_scalar_deleting_destructor(
                Ptr<NiTextKeyExtraData>,
                u32,
            )
                -> Ptr<NiTextKeyExtraData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_text_key_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a46b60, &args![]).u32(),
            NI_TEXT_KEY_EXTRA_DATA_RTTI
        );
    }

    #[test]
    fn thunk_00a9af00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_af00;
        const TARGET: u32 = 0x00a7_b3b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod text_key_tests {
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

    /// An engine with the empty string's handle (0x9000), a heap allocator, an array
    /// constructor double that runs the key constructor on each element, and silent extra
    /// data and key doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |e, a| {
            for i in 0..a[2] {
                e.call(a[3], &args![a[0] + i * a[1]]);
            }
            Ret::default()
        });
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            TEXT_KEY_DESTRUCT,
            TEXT_KEY_SET_TEXT,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn key_array_bytes_saturates() {
        assert_eq!(key_array_bytes(3), 28);
        assert_eq!(key_array_bytes(0x2000_0000), u32::MAX);
        assert_eq!(key_array_bytes(0x1fff_ffff), 0xffff_fffc);
    }

    #[test]
    fn text_key_constructor_sets_the_empty_text() {
        let mut e = engine();
        let key = e.mem.alloc(8);
        e.mem.set_u32(key, 7);
        assert_eq!(e.call(0x00a4_6b50, &args![key]).u32(), key);
        assert_eq!(e.mem.u32(key + 4), 0x9000);
        assert_eq!(e.mem.u32(key), 7);
    }

    #[test]
    fn create_object_00a46b70_builds_an_empty_object() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x14]);
            block.into_ret()
        });
        let object = e.call(0x00a4_6b70, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_TEXT_KEY_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![object]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_6b70, &args![]).u32(), 0);
    }

    /// An object with `times.len()` keys.
    fn with_keys(e: &mut Engine, times: &[f32]) -> Ptr<NiTextKeyExtraData> {
        let this = Ptr::<NiTextKeyExtraData>::new(e.mem.alloc(0x44));
        let block = e.mem.alloc(4 + 8 * times.len() as u32);
        e.mem.set_u32(block, times.len() as u32);
        for (i, t) in times.iter().enumerate() {
            e.mem.set_f32(block + 4 + 8 * i as u32, *t);
            e.mem.set_u32(block + 8 + 8 * i as u32, 0x100 + i as u32);
        }
        e.set(this, NiTextKeyExtraData::m_uiNumKeys, times.len() as u32);
        e.set(this, NiTextKeyExtraData::m_pKeys, block + 4);
        this
    }

    #[test]
    fn copy_members_copies_the_keys_of_a_text_key_object() {
        let mut e = engine();
        e.put_vtable(0x0300_0000, &[0, 0, 0x0300_0008]);
        e.register(0x0300_0008, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        let this = with_keys(&mut e, &[1.0, 2.5]);
        e.mem.set_u32(this.addr(), 0x0300_0000);
        let clone = e.mem.alloc(0x50);
        let clone = Ptr::<NiTextKeyExtraData>::new(clone);
        // Another class (RTTI 0): the count only.
        e.call(0x00a4_6c20, &args![this, clone, 0x55u32]);
        assert_eq!(e.get(clone, NiTextKeyExtraData::m_uiNumKeys), 2);
        assert_eq!(e.get(clone, NiTextKeyExtraData::m_pKeys), 0);
        // NiTextKeyExtraData: a new array, times copied, texts assigned.
        let object = e.mem.alloc(0x44);
        let header = e.mem.bytes(this.addr(), 0x14);
        e.mem.write(object, &header);
        e.mem.set_u32(object + 0x40, NI_TEXT_KEY_EXTRA_DATA_RTTI);
        let source = Ptr::<NiTextKeyExtraData>::new(object);
        e.call_log = Some(vec![]);
        e.call(0x00a4_6c20, &args![source, clone, 0x55u32]);
        let keys = e.get(clone, NiTextKeyExtraData::m_pKeys);
        assert_ne!(keys, 0);
        assert_eq!(e.mem.u32(keys - 4), 2);
        assert_eq!(e.mem.f32(keys), 1.0);
        assert_eq!(e.mem.f32(keys + 8), 2.5);
        let source_keys = e.get(source, NiTextKeyExtraData::m_pKeys);
        assert_eq!(
            calls_to(&e, TEXT_KEY_SET_TEXT),
            vec![
                vec![keys, source_keys + 4],
                vec![keys + 8, source_keys + 12]
            ]
        );
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![object, clone.addr(), 0x55]]
        );
    }

    #[test]
    fn destructor_body_destroys_the_keys_last_first() {
        let mut e = engine();
        let this = with_keys(&mut e, &[1.0, 2.0, 3.0]);
        let keys = e.get(this, NiTextKeyExtraData::m_pKeys);
        e.call(0x00a4_6d50, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_TEXT_KEY_EXTRA_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, TEXT_KEY_DESTRUCT),
            vec![vec![keys + 16], vec![keys + 8], vec![keys]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, keys - 4]]
        );
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT), vec![vec![this.addr()]]);
        // No keys: only the base destructor.
        let empty = e.new_object::<NiTextKeyExtraData>();
        e.call_log = Some(vec![]);
        e.call(0x00a4_6d50, &args![empty]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.put_vtable(0x0300_0000, &[0, 0, 0x0300_0008]);
        e.register(0x0300_0008, |_, _| 0u32.into_ret());
        let this = with_keys(&mut e, &[1.0]);
        e.mem.set_u32(this.addr(), 0x0300_0000);
        let clone = e.call(0x00a4_6da0, &args![this, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_TEXT_KEY_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(clone + 0xc), 1);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x66]]
        );
    }

    #[test]
    fn load_binary_reads_the_count_and_each_key() {
        let mut e = engine();
        e.register(TEXT_KEY_LOAD_BINARY, |_, _| Ret::default());
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0100);
        e.register(0x0300_0100, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], 2);
            4u32.into_ret()
        });
        let this = e.new_object::<NiTextKeyExtraData>();
        e.call(0x00a4_6e00, &args![this, stream]);
        let keys = e.get(this, NiTextKeyExtraData::m_pKeys);
        assert_eq!(e.get(this, NiTextKeyExtraData::m_uiNumKeys), 2);
        assert_eq!(e.mem.u32(keys - 4), 2);
        assert_eq!(e.mem.u32(keys + 4), 0x9000);
        assert_eq!(e.mem.u32(keys + 12), 0x9000);
        assert_eq!(
            calls_to(&e, TEXT_KEY_LOAD_BINARY),
            vec![vec![keys, stream.addr()], vec![keys + 8, stream.addr()]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 20]]
        );
        // A count of 0 leaves the fields.
        e.register(0x0300_0100, |e, a| {
            e.mem.set_u32(a[1], 0);
            4u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a4_6e00, &args![this, stream]);
        assert_eq!(e.get(this, NiTextKeyExtraData::m_pKeys), keys);
        assert!(calls_to(&e, MEMORY_ALLOCATE).is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = engine();
        let this = e.new_object::<NiTextKeyExtraData>();
        assert_eq!(e.call(0x00a4_6ed0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x14]]);
        e.call(0x00a4_6ed0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 2);
    }
}
