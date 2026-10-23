//! `NiParticle/nipsysmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `NiPSysModifier` base: its layout and the stream and string helpers
//! the other particle units of this crate share.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPSysModifier` (Xbox PDB), 0x18 bytes; the PC code uses the same
    /// offsets.
    pub struct NiPSysModifier: 0x18 {
        /// `m_kName` (Xbox PDB): an `NiFixedString` handle.
        0x08 m_kName: u32,
        /// `m_uiOrder` (Xbox PDB).
        0x0C m_uiOrder: u32,
        /// `m_pkTarget` (Xbox PDB): `NiParticleSystem*`.
        0x10 m_pkTarget: u32,
        /// `m_bActive` (Xbox PDB).
        0x14 m_bActive: bool,
    }
}

/// The vtable of `NiPSysModifier` (RTTI `.?AVNiPSysModifier@@`).
const VTABLE: u32 = 0x010c_09cc;

/// The global holding the null `NiFixedString` handle (`0109b220`): the
/// code compares and stores it rather than a literal 0.
pub(crate) const FIXED_STRING_NULL: u32 = 0x0109_b220;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; offsets as the PC code reads them), and the input stream's
/// position (+4) and read function (+8, `cdecl` `(stream, buffer, size,
/// &component size, component count)`, returns the bytes read).
pub(crate) const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// `NiObject::NiObject` (`00a5d3a0`, `thiscall`, no stack words).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `NiObject::~NiObject` body (`00a5d3d0`, `thiscall`).
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `NiObject::CopyMembers` (`00a5d450`, `thiscall`, `(dest, process)`).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::ProcessClone` (`00a5d430`, `thiscall`, one word).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiObject::LoadBinary` (`00a5d610`, `thiscall`, one word).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a89af0` (unnamed in the engine map): `thiscall`, one word, the base
/// link step `NiPSysModifier::LinkObject` runs first.
const BASE_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
pub(crate) const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a53c30` (unnamed): the cloning process's map lookup, `thiscall` on
/// the process's first word, `(key, &value)`.
pub(crate) const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiStream::LoadCStringAsFixedString` (`00a64ea0`) and
/// `NiStream::LoadFixedString` (`00a64df0`): `thiscall` on the stream, one
/// word (the string's address).
pub(crate) const LOAD_CSTRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
pub(crate) const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// `00a645b0` (unnamed, `NiMain/nistream.cpp`): `thiscall` on the stream,
/// no stack words; it reads a link id into the stream's list.
pub(crate) const READ_LINK_ID: u32 = 0x00a6_45b0;
/// The first file version whose names are fixed strings
/// (`CMP [stream+0xd8], 0x14010001`).
pub(crate) const VERSION_FIXED_STRINGS: u32 = 0x1401_0001;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
pub(crate) fn stream_read(
    e: &mut Engine,
    stream: u32,
    buffer: u32,
    size: u32,
    component: u32,
    slot: u32,
) {
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

/// The `NiFixedString` assignment the code inlines: when `dest`'s handle
/// differs from `source`, `source` is held (`InterlockedIncrement` on its
/// word at -8) unless it is the null handle, the old handle released
/// (`InterlockedDecrement`) unless null, and `source` stored.
pub(crate) fn fixed_string_assign(e: &mut Engine, dest: u32, source: u32) {
    if e.mem.u32(dest) == source {
        return;
    }
    if source != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_INCREMENT, &args![source.wrapping_sub(8)]);
    }
    let old = e.mem.u32(dest);
    if old != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(dest, source);
}

/// The `NiFixedString` destructor the code inlines: the handle at `string`
/// is released unless it is the null handle.
pub(crate) fn fixed_string_release(e: &mut Engine, string: u32) {
    let handle = e.mem.u32(string);
    if handle != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The name read the modifier and controller `LoadBinary`s share: files
/// before 20.1.0.1 store a C string (`LoadCStringAsFixedString`), later ones
/// a fixed string (`LoadFixedString`), into the string at `string`.
pub(crate) fn load_name(e: &mut Engine, stream: u32, string: u32) {
    let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
    if version < VERSION_FIXED_STRINGS {
        e.call(LOAD_CSTRING_AS_FIXED_STRING, &args![stream, string]);
    } else {
        e.call(LOAD_FIXED_STRING, &args![stream, string]);
    }
}

// Translated from 00c31650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::LinkObject` (Xbox PDB): the base link step `00a89af0`
/// on `this` with the stream, then `NiStream::GetObjectFromLinkID` on the
/// stream; its result is not stored (the exe drops `EAX`).
pub fn ni_p_sys_modifier_link_object(e: &mut Engine, this: Ptr<NiPSysModifier>, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]);
}

// Translated from 00c31720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifier` constructor (it stores
/// the vtable `010c09cc`, RTTI `.?AVNiPSysModifier@@`). `NiObject`'s
/// constructor, then the null name handle (the word at `0109b220`), order
/// and target 0, active. Returns `this`.
pub fn fn_00c31720(e: &mut Engine, this: Ptr<NiPSysModifier>) -> Ptr<NiPSysModifier> {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let null = e.global::<u32>(FIXED_STRING_NULL);
    e.set(this, NiPSysModifier::m_kName, null);
    e.set(this, NiPSysModifier::m_uiOrder, 0);
    e.set(this, NiPSysModifier::m_pkTarget, 0);
    e.set(this, NiPSysModifier::m_bActive, true);
    this
}

// Translated from 00c31750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::~NiPSysModifier` (Xbox PDB): the vtable back to
/// `NiPSysModifier`'s, the name released unless it is the null handle, then
/// `NiObject`'s destructor body (`00a5d3d0`, a tail jump).
pub fn ni_p_sys_modifier_destructor(e: &mut Engine, this: Ptr<NiPSysModifier>) {
    e.mem.set_u32(this.addr(), VTABLE);
    fixed_string_release(e, this.addr() + NiPSysModifier::m_kName.off);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00c31780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifier` member copy into `dest`
/// (`thiscall`, `RET 8`: `dest`, cloning process). `NiObject::CopyMembers`,
/// then the name (fixed-string assignment), the order and the active flag;
/// the target is not copied.
pub fn fn_00c31780(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    dest: Ptr<NiPSysModifier>,
    process: Ptr,
) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, dest, process]);
    let name = e.get(this, NiPSysModifier::m_kName);
    fixed_string_assign(e, dest.addr() + NiPSysModifier::m_kName.off, name);
    let order = e.get(this, NiPSysModifier::m_uiOrder);
    e.set(dest, NiPSysModifier::m_uiOrder, order);
    let active = e.mem.u8(this.addr() + NiPSysModifier::m_bActive.off);
    e.mem
        .set_u8(dest.addr() + NiPSysModifier::m_bActive.off, active);
}

// Translated from 00c317e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`, then
/// the clone of `this` and the clone of its target from the cloning
/// process's map (`00a53c30` on the process's first word, both into one
/// stack word cleared once before the first lookup); the target's clone
/// becomes the clone's target.
pub fn ni_p_sys_modifier_process_clone(e: &mut Engine, this: Ptr<NiPSysModifier>, process: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = e.mem.u32(slot.addr());
        let target = e.get(this, NiPSysModifier::m_pkTarget);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, target, slot]);
        let target_clone = e.mem.u32(slot.addr());
        e.mem.set_u32(
            clone.wrapping_add(NiPSysModifier::m_pkTarget.off),
            target_clone,
        );
    });
}

// Translated from 00c318b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the
/// name (a C string before file version 20.1.0.1, a fixed string from it),
/// the order (4 bytes), a link id (`00a645b0`, the target), then one byte
/// read into the low byte of the stack word that holds the stream argument;
/// the active flag is whether that byte is non-zero.
pub fn ni_p_sys_modifier_load_binary(e: &mut Engine, this: Ptr<NiPSysModifier>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    load_name(e, stream.addr(), this.addr() + NiPSysModifier::m_kName.off);
    e.with_stack(8, |e, frame| {
        let size_slot = frame.addr();
        let argument_slot = frame.addr() + 4;
        stream_read(
            e,
            stream.addr(),
            this.addr() + NiPSysModifier::m_uiOrder.off,
            4,
            4,
            size_slot,
        );
        e.call(READ_LINK_ID, &args![stream]);
        e.mem.set_u32(argument_slot, stream.addr());
        stream_read(e, stream.addr(), argument_slot, 1, 1, size_slot);
        let active = e.mem.u8(argument_slot) != 0;
        e.set(this, NiPSysModifier::m_bActive, active);
    });
}

// Translated from 00c31710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27d0.
pub fn ni_p_sys_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27d0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c31710, ni_p_sys_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c3_1650,
            ni_p_sys_modifier_link_object(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_1720,
            fn_00c31720(Ptr<NiPSysModifier>) -> Ptr<NiPSysModifier>
        ),
        entry!(
            0x00c3_1750,
            ni_p_sys_modifier_destructor(Ptr<NiPSysModifier>)
        ),
        entry!(
            0x00c3_1780,
            fn_00c31780(Ptr<NiPSysModifier>, Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_17e0,
            ni_p_sys_modifier_process_clone(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_18b0,
            ni_p_sys_modifier_load_binary(Ptr<NiPSysModifier>, Ptr)
        ),
    ]
}

/// Test support shared by this crate's particle units: a stream whose read
/// function hands out the bytes queued for it, in order.
#[cfg(test)]
pub(crate) mod test_stream {
    use super::*;

    /// The read function's address in tests.
    pub const READ_FN: u32 = 0x0f7f_0000;

    /// Builds an `NiStream` with file version `version` and an input stream
    /// reading from the bytes in `data`; returns the stream's address. The
    /// read double copies `size` bytes from a cursor kept before the data.
    pub fn new(e: &mut Engine, version: u32, data: &[u8]) -> u32 {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(data.len() as u32 + 8);
        e.mem.write(buffer + 4, data);
        e.mem.set_u32(buffer, buffer + 4);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ_FN);
        e.mem.set_u32(input + 0xc, buffer);
        e.register(READ_FN, |e, a| {
            let (input, dest, size) = (a[0], a[1], a[2]);
            let buffer = e.mem.u32(input + 0xc);
            let cursor = e.mem.u32(buffer);
            let bytes = e.mem.bytes(cursor, size);
            e.mem.write(dest, &bytes);
            e.mem.set_u32(buffer, cursor + size);
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        stream
    }

    /// Doubles for the imported `InterlockedIncrement` and
    /// `InterlockedDecrement` (the platform registers them only with the
    /// executable): they change the word and answer its new value.
    pub fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
    }

    /// The input stream's position word.
    pub fn position(e: &Engine, stream: u32) -> u32 {
        let input = e.mem.u32(stream + STREAM_INPUT);
        e.mem.u32(input + BINARY_STREAM_POSITION)
    }
}

#[cfg(test)]
mod tests {
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

    fn null_handle(e: &mut Engine, value: u32) {
        e.map(FIXED_STRING_NULL & !0xfff, 0x1000);
        e.set_global(FIXED_STRING_NULL, value);
    }

    #[test]
    fn ni_p_sys_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1710, &args![]).u32(), 0x0120_27d0);
    }

    #[test]
    fn link_object_runs_the_base_step_then_reads_a_link() {
        let mut e = Engine::new();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c3_1650, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, BASE_LINK_OBJECT),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID), vec![vec![0x7000]]);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0);
    }

    #[test]
    fn constructor_sets_the_vtable_null_name_and_active() {
        let mut e = Engine::new();
        null_handle(&mut e, 0x55);
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_uiOrder, 9);
        e.set(this, NiPSysModifier::m_pkTarget, 9);
        assert_eq!(e.call(0x00c3_1720, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, NiPSysModifier::m_kName), 0x55);
        assert_eq!(e.get(this, NiPSysModifier::m_uiOrder), 0);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0);
        assert!(e.get(this, NiPSysModifier::m_bActive));
    }

    #[test]
    fn destructor_releases_a_name_but_not_the_null_handle() {
        let mut e = Engine::new();
        null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        let string = e.mem.alloc(16);
        e.mem.set_u32(string, 2);
        e.set(this, NiPSysModifier::m_kName, string + 8);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1750, &args![this]);
        assert_eq!(e.mem.u32(string), 1);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
        e.set(this, NiPSysModifier::m_kName, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1750, &args![this]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn copy_members_assigns_the_name_order_and_active_flag() {
        let mut e = Engine::new();
        null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(NI_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        let dest = e.new_object::<NiPSysModifier>();
        let strings = e.mem.alloc(32);
        e.mem.set_u32(strings, 1);
        e.mem.set_u32(strings + 16, 1);
        e.set(this, NiPSysModifier::m_kName, strings + 8);
        e.set(dest, NiPSysModifier::m_kName, strings + 24);
        e.set(this, NiPSysModifier::m_uiOrder, 7);
        e.set(this, NiPSysModifier::m_pkTarget, 0x99);
        e.set(this, NiPSysModifier::m_bActive, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1780, &args![this, dest, 0x4000u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x4000]]
        );
        assert_eq!(e.get(dest, NiPSysModifier::m_kName), strings + 8);
        assert_eq!(e.mem.u32(strings), 2);
        assert_eq!(e.mem.u32(strings + 16), 0);
        assert_eq!(e.get(dest, NiPSysModifier::m_uiOrder), 7);
        assert_eq!(e.get(dest, NiPSysModifier::m_pkTarget), 0);
        assert!(e.get(dest, NiPSysModifier::m_bActive));
        // The same handle: nothing is held or released.
        e.call_log = Some(vec![]);
        e.call(0x00c3_1780, &args![this, dest, 0x4000u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn process_clone_sets_the_clone_target() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        let clone = e.new_object::<NiPSysModifier>();
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_pkTarget, 0x6000);
        let map = e.mem.alloc(4);
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, map);
        e.mem.set_u32(map, clone.addr());
        e.register(CLONE_MAP_LOOKUP, |e, a| {
            let value = if a[1] == 0x6000 {
                0x6100
            } else {
                e.mem.u32(a[0])
            };
            e.mem.set_u32(a[2], value);
            Ret::default()
        });
        e.call(0x00c3_17e0, &args![this, process]);
        assert_eq!(e.get(clone, NiPSysModifier::m_pkTarget), 0x6100);
    }

    #[test]
    fn load_binary_reads_name_order_link_and_active() {
        for (version, loader) in [
            (0x1401_0000u32, LOAD_CSTRING_AS_FIXED_STRING),
            (0x1401_0001, LOAD_FIXED_STRING),
        ] {
            let mut e = Engine::new();
            e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
            e.register(LOAD_CSTRING_AS_FIXED_STRING, |_, _| Ret::default());
            e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            let stream = test_stream::new(&mut e, version, &[5, 0, 0, 0, 0]);
            let this = e.new_object::<NiPSysModifier>();
            e.set(this, NiPSysModifier::m_bActive, true);
            e.call_log = Some(vec![]);
            e.call(0x00c3_18b0, &args![this, stream]);
            assert_eq!(calls_to(&e, loader), vec![vec![stream, this.addr() + 8]]);
            assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
            assert_eq!(e.get(this, NiPSysModifier::m_uiOrder), 5);
            assert!(!e.get(this, NiPSysModifier::m_bActive));
            assert_eq!(test_stream::position(&e, stream), 5);
        }
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1402_0000, &[1, 0, 0, 0, 2]);
        let this = e.new_object::<NiPSysModifier>();
        e.call(0x00c3_18b0, &args![this, stream]);
        assert!(e.get(this, NiPSysModifier::m_bActive));
    }
}
