//! `NiAnimation/niboolinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_404c;

// Translated from 00a532c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiBoolInterpolator` (Xbox PDB, 0x18 bytes; the same offsets on PC).
    pub struct NiBoolInterpolator: 0x18 {
        /// `m_fLastTime` (Xbox PDB, `NiInterpolator`).
        0x08 m_fLastTime: u32,
        /// `m_bBoolValue` (Xbox PDB).
        0x0C m_bBoolValue: u8,
        /// `m_spBoolData` (Xbox PDB): `NiPointer<NiBoolData>`.
        0x10 m_spBoolData: u32,
        /// `m_uiLastIdx` (Xbox PDB).
        0x14 m_uiLastIdx: u32,
    }
}

/// The vtable of `NiBoolInterpolator` (RTTI `.?AVNiBoolInterpolator@@`).
const VTABLE: u32 = 0x0109_a0b4;
/// The byte that marks an unset value (`NiBoolInterpolator`'s invalid
/// bool, `0109a914`).
const INVALID_BOOL: u32 = 0x0109_a914;
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (`00a3ca10`, engine
/// map; `thiscall`).
const KEY_BASED_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_ca10;
/// The base destructor `00a3ca30` (`thiscall`; stores the
/// `NiKeyBasedInterpolator` vtable and continues in `00a57010`).
const KEY_BASED_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_ca30;
/// `NiNew` allocation (`00aa13e0`, `cdecl`, the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiDelete` of a sized block (`00aa1460`, `cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiBoolKey::GenInterp` (`00a29250`, engine map; `cdecl`, six words:
/// time, keys, key type, key count, `&last_index`, key size).
const BOOL_KEY_GEN_INTERP: u32 = 0x00a2_9250;
/// The base `CopyMembers` (`00aa0ab0`, a jump to `00aa1000`; `thiscall`,
/// two words).
const BASE_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiKeyBasedInterpolator::LoadBinary` (`00aa0ac0`, engine map; a jump;
/// `thiscall`, one word).
const BASE_LOAD_BINARY: u32 = 0x00aa_0ac0;
/// `NiStream::ResolveLinkID` (`00a64620`, engine map; `thiscall`).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;

/// Releases a reference-counted object (`InterlockedDecrement` of +4, its
/// vtable +4 deleting it at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `m_spBoolData = data` as `NiPointer` assignment does: when it changes,
/// the old object is released and the new one gains a reference.
fn set_bool_data(e: &mut Engine, this: Ptr<NiBoolInterpolator>, data: u32) {
    let old = e.get(this, NiBoolInterpolator::m_spBoolData);
    if old == data {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.set(this, NiBoolInterpolator::m_spBoolData, data);
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
}

// Translated from 00a53500 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; Ghidra has no function
/// here and the engine map no name): a 0x18-byte block (`00aa13e0`) set up
/// as a `NiBoolInterpolator` (base constructor, vtable, the invalid value,
/// no data, index 0), or null when the allocation fails.
pub fn fn_00a53500(e: &mut Engine) -> Ptr<NiBoolInterpolator> {
    let block = e.call(NI_NEW, &args![0x18u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let this = Ptr::<NiBoolInterpolator>::new(block);
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(block, VTABLE);
    let invalid: u8 = e.global(INVALID_BOOL);
    e.set(this, NiBoolInterpolator::m_bBoolValue, invalid);
    e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a530a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::GetChannelPosed` (Xbox PDB; `thiscall`, one word: the
/// channel, not read): true when the interpolator has no data and its value
/// is set (not the invalid byte).
pub fn ni_bool_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    _unused_channel: u32,
) -> bool {
    if e.get(this, NiBoolInterpolator::m_spBoolData) != 0 {
        return false;
    }
    let invalid: u8 = e.global(INVALID_BOOL);
    e.get(this, NiBoolInterpolator::m_bBoolValue) != invalid
}

// Translated from 00a53280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBoolInterpolator(NiBoolData*)` constructor (`thiscall`, one word;
/// no Xbox PDB name): base constructor, vtable, the invalid value, the data
/// (a reference added) and index 0. Returns `this`.
pub fn fn_00a53280(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    data: u32,
) -> Ptr<NiBoolInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let invalid: u8 = e.global(INVALID_BOOL);
    e.set(this, NiBoolInterpolator::m_bBoolValue, invalid);
    e.set(this, NiBoolInterpolator::m_spBoolData, data);
    if data != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a532d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::NiBoolInterpolator(bool)` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word): base constructor, the value, vtable,
/// no data, index 0. Returns `this`.
pub fn ni_bool_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    value: u8,
) -> Ptr<NiBoolInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.set(this, NiBoolInterpolator::m_bBoolValue, value);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, NiBoolInterpolator::m_spBoolData, 0);
    e.set(this, NiBoolInterpolator::m_uiLastIdx, 0);
    this
}

// Translated from 00a533c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::Update` (Xbox PDB, matched by the engine map;
/// `thiscall (this; time, interp_target, value)`, `value` a `bool*`): at an
/// unchanged time (`FUCOMP` equal) the stored value is reported; otherwise,
/// with data and keys, the value becomes `NiBoolKey::GenInterp(time, keys,
/// type, count, &m_uiLastIdx, key size)` as a `bool` (the key-size word
/// carries the upper bytes of the stack slot the exe reuses, which held
/// `this`). An invalid value reports false (and leaves `m_fLastTime`); a
/// valid one is written to `*value`, answers true and, after a new
/// evaluation, records the time.
pub fn ni_bool_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    time: f32,
    _unused_target: u32,
    value: Ptr,
) -> bool {
    let invalid: u8 = e.global(INVALID_BOOL);
    let last = x87_load32(e.get(this, NiBoolInterpolator::m_fLastTime));
    let now = x87_load32(time.to_bits());
    // FUCOMP, TEST AH,0x44 / JP: only an equal time reuses the value.
    if now == last {
        let stored = e.get(this, NiBoolInterpolator::m_bBoolValue);
        if stored == invalid {
            e.mem.set_u8(value.addr(), 0);
            return false;
        }
        e.mem.set_u8(value.addr(), u8::from(stored != 0));
        return true;
    }
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        let count = e.mem.u32(data + 8);
        let key_size = e.mem.u8(data + 0x14);
        let key_type = e.mem.u32(data + 0x10);
        let keys = e.mem.u32(data + 0xc);
        if count != 0 {
            let size_word = (this.addr() & 0xffff_ff00) | u32::from(key_size);
            let result = e
                .call(
                    BOOL_KEY_GEN_INTERP,
                    &args![
                        x87_quiet_bits(time.to_bits()),
                        keys,
                        key_type,
                        count,
                        this.addr() + 0x14,
                        size_word
                    ],
                )
                .bool();
            e.set(this, NiBoolInterpolator::m_bBoolValue, u8::from(result));
        }
    }
    let stored = e.get(this, NiBoolInterpolator::m_bBoolValue);
    if stored == invalid {
        e.mem.set_u8(value.addr(), 0);
        return false;
    }
    e.mem.set_u8(value.addr(), u8::from(stored != 0));
    e.set(
        this,
        NiBoolInterpolator::m_fLastTime,
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00a53480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::CopyMembers` (`thiscall (this; dest, cloning)`; the
/// engine map gives this body no name): the base `CopyMembers`, then the
/// value, the data (as a `NiPointer` assignment) and the last index into
/// `dest`.
pub fn fn_00a53480(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    dest: Ptr<NiBoolInterpolator>,
    cloning: u32,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let value = e.get(this, NiBoolInterpolator::m_bBoolValue);
    e.set(dest, NiBoolInterpolator::m_bBoolValue, value);
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    set_bool_data(e, dest, data);
    let index = e.get(this, NiBoolInterpolator::m_uiLastIdx);
    e.set(dest, NiBoolInterpolator::m_uiLastIdx, index);
}

// Translated from 00a53540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::_scalar_deleting_destructor_` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: flags): releases the data, runs
/// the base destructor (`00a3ca30`) and with flag bit 0 frees the 0x18-byte
/// block. Returns `this`.
pub fn ni_bool_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    flags: u32,
) -> Ptr<NiBoolInterpolator> {
    let data = e.get(this, NiBoolInterpolator::m_spBoolData);
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_INTERPOLATOR_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a53590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::CreateClone` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the cloning process): a new interpolator set up as
/// [`fn_00a53500`] does, with this one's members copied in
/// ([`fn_00a53480`]; also called, on null, when the allocation fails).
pub fn ni_bool_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    cloning: u32,
) -> Ptr<NiBoolInterpolator> {
    let block = e.call(NI_NEW, &args![0x18u32]).u32();
    let clone = Ptr::<NiBoolInterpolator>::new(block);
    if block != 0 {
        e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(block, VTABLE);
        let invalid: u8 = e.global(INVALID_BOOL);
        e.set(clone, NiBoolInterpolator::m_bBoolValue, invalid);
        e.set(clone, NiBoolInterpolator::m_spBoolData, 0);
        e.set(clone, NiBoolInterpolator::m_uiLastIdx, 0);
    }
    fn_00a53480(e, this, clone, cloning);
    clone
}

// Translated from 00a53600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpolator::LoadBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `LoadBinary`, one byte into
/// the value through the stream's file (+0x24C: its read function at +8,
/// `cdecl (file, buffer, 1, &count, 1)` with the count word set to 1; the
/// result added to the file's +4), then the data from the stream's next
/// link id (`NiStream::ResolveLinkID`, a `NiPointer` assignment).
pub fn ni_bool_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiBoolInterpolator>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let file = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(file + 8);
    let got = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        e.call(read, &args![file, this.addr() + 0xc, 1u32, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(got));
    let data = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    set_bool_data(e, this, data);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a532c0, ni_bool_interpolator_get_rtti() -> Ptr),
        entry!(0x00a5_3500, fn_00a53500() -> Ptr<NiBoolInterpolator>),
        entry!(
            0x00a5_30a0,
            ni_bool_interpolator_get_channel_posed(Ptr<NiBoolInterpolator>, u32) -> bool
        ),
        entry!(
            0x00a5_3280,
            fn_00a53280(Ptr<NiBoolInterpolator>, u32) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_32d0,
            ni_bool_interpolator_constructor(
                Ptr<NiBoolInterpolator>,
                u8,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_33c0,
            ni_bool_interpolator_update(Ptr<NiBoolInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_3480,
            fn_00a53480(Ptr<NiBoolInterpolator>, Ptr<NiBoolInterpolator>, u32)
        ),
        entry!(
            0x00a5_3540,
            ni_bool_interpolator_scalar_deleting_destructor(
                Ptr<NiBoolInterpolator>,
                u32,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_3590,
            ni_bool_interpolator_create_clone(
                Ptr<NiBoolInterpolator>,
                u32,
            ) -> Ptr<NiBoolInterpolator>
        ),
        entry!(
            0x00a5_3600,
            ni_bool_interpolator_load_binary(Ptr<NiBoolInterpolator>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a532c0, &args![]).u32(),
            NI_BOOL_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod interpolator_tests {
    use super::*;

    type B = NiBoolInterpolator;

    const DELETE_SLOT: u32 = 0x0001_0010;
    const READ: u32 = 0x0001_0020;
    const INVALID: u8 = 2;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_a000, 0x1000);
        e.set_global(INVALID_BOOL, INVALID);
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(KEY_BASED_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e
    }

    /// A reference-counted object (count at +4) whose vtable +4 is
    /// `DELETE_SLOT`.
    fn counted(e: &mut Engine, references: u32) -> u32 {
        let object = e.mem.alloc(0x18);
        e.put_vtable(0x0002_0000, &[0, DELETE_SLOT]);
        e.mem.set_u32(object, 0x0002_0000);
        e.mem.set_u32(object + 4, references);
        object
    }

    fn deleted(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .filter(|c| c.0 == DELETE_SLOT)
            .map(|c| c.1[0])
            .collect()
    }

    #[test]
    fn create_object_sets_up_an_empty_interpolator() {
        let mut e = engine();
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xcd; 0x18]);
        e.register_double(NI_NEW, move |_, a| {
            assert_eq!(a[0], 0x18);
            word(block)
        });
        assert_eq!(e.call(0x00a5_3500, &args![]).u32(), block);
        let this = Ptr::<B>::new(block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.get(this, B::m_bBoolValue), INVALID);
        assert_eq!(e.get(this, B::m_spBoolData), 0);
        assert_eq!(e.get(this, B::m_uiLastIdx), 0);
        e.register(NI_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_3500, &args![]).u32(), 0);
    }

    #[test]
    fn channel_is_posed_with_a_value_and_no_data() {
        let mut e = engine();
        let this = e.new_object::<B>();
        e.set(this, B::m_bBoolValue, 1);
        assert!(e.call(0x00a5_30a0, &args![this, 0u32]).bool());
        e.set(this, B::m_bBoolValue, INVALID);
        assert!(!e.call(0x00a5_30a0, &args![this, 0u32]).bool());
        e.set(this, B::m_bBoolValue, 0);
        e.set(this, B::m_spBoolData, 0x500);
        assert!(!e.call(0x00a5_30a0, &args![this, 0u32]).bool());
    }

    #[test]
    fn constructors_set_the_members() {
        let mut e = engine();
        let data = counted(&mut e, 1);
        let this = e.new_object::<B>();
        assert_eq!(e.call(0x00a5_3280, &args![this, data]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, B::m_bBoolValue), INVALID);
        assert_eq!(e.get(this, B::m_spBoolData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.get(this, B::m_uiLastIdx), 0);

        let other = e.new_object::<B>();
        e.mem.write(other.addr(), &[0xcd; 0x18]);
        assert_eq!(
            e.call(0x00a5_32d0, &args![other, 0x101u32]).u32(),
            other.addr()
        );
        assert_eq!(e.get(other, B::m_bBoolValue), 1);
        assert_eq!(e.mem.u32(other.addr()), VTABLE);
        assert_eq!(e.get(other, B::m_spBoolData), 0);
        assert_eq!(e.get(other, B::m_uiLastIdx), 0);
    }

    fn bool_data(e: &mut Engine, count: u32) -> u32 {
        let data = counted(e, 1);
        e.mem.set_u32(data + 8, count);
        e.mem.set_u32(data + 0xc, 0x7000);
        e.mem.set_u32(data + 0x10, 3);
        e.mem.set_u8(data + 0x14, 2);
        data
    }

    #[test]
    fn update_evaluates_the_keys_at_a_new_time() {
        let mut e = engine();
        let this = e.new_object::<B>();
        let data = bool_data(&mut e, 4);
        e.set(this, B::m_spBoolData, data);
        e.set(this, B::m_fLastTime, 1.0f32.to_bits());
        e.register(BOOL_KEY_GEN_INTERP, |_, _| word(0x0000_0100));
        let value = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_33c0, &args![this, 2.0f32, 0u32, value])
            .bool());
        let log = e.call_log.take().unwrap();
        let gen: Vec<_> = log.iter().filter(|c| c.0 == BOOL_KEY_GEN_INTERP).collect();
        assert_eq!(
            gen[0].1,
            vec![
                2.0f32.to_bits(),
                0x7000,
                3,
                4,
                this.addr() + 0x14,
                (this.addr() & 0xffff_ff00) | 2
            ]
        );
        // The key answered false (AL = 0): the value is false.
        assert_eq!(e.get(this, B::m_bBoolValue), 0);
        assert_eq!(e.mem.u8(value), 0);
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());

        e.register(BOOL_KEY_GEN_INTERP, |_, _| word(1));
        assert!(e
            .call(0x00a5_33c0, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 1);
        assert_eq!(e.get(this, B::m_fLastTime), 3.0f32.to_bits());
    }

    #[test]
    fn update_reuses_the_value_at_the_same_time() {
        let mut e = engine();
        let this = e.new_object::<B>();
        e.set(this, B::m_fLastTime, 1.0f32.to_bits());
        e.set(this, B::m_bBoolValue, 5);
        let value = e.mem.alloc(4);
        assert!(e
            .call(0x00a5_33c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 1);
        e.set(this, B::m_bBoolValue, INVALID);
        e.mem.set_u8(value, 9);
        assert!(!e
            .call(0x00a5_33c0, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u8(value), 0);
        // A new time without data keeps the value but records the time;
        // an invalid value records nothing.
        e.set(this, B::m_bBoolValue, 0);
        assert!(e
            .call(0x00a5_33c0, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());
        e.set(this, B::m_bBoolValue, INVALID);
        assert!(!e
            .call(0x00a5_33c0, &args![this, 4.0f32, 0u32, value])
            .bool());
        assert_eq!(e.get(this, B::m_fLastTime), 2.0f32.to_bits());
        // Data without keys is not evaluated.
        let data = bool_data(&mut e, 0);
        e.set(this, B::m_spBoolData, data);
        e.call_log = Some(vec![]);
        e.call(0x00a5_33c0, &args![this, 5.0f32, 0u32, value]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != BOOL_KEY_GEN_INTERP));
    }

    #[test]
    fn copy_members_shares_the_data() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let source = e.new_object::<B>();
        let dest = e.new_object::<B>();
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.set(source, B::m_bBoolValue, 1);
        e.set(source, B::m_spBoolData, data);
        e.set(source, B::m_uiLastIdx, 7);
        e.set(dest, B::m_spBoolData, old);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3480, &args![source, dest, 0x900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (BASE_COPY_MEMBERS, vec![source.addr(), dest.addr(), 0x900])
        );
        assert_eq!(e.get(dest, B::m_bBoolValue), 1);
        assert_eq!(e.get(dest, B::m_spBoolData), data);
        assert_eq!(e.get(dest, B::m_uiLastIdx), 7);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(deleted(&log), vec![old]);
        // The same data: no reference change.
        e.call(0x00a5_3480, &args![source, dest, 0x900u32]);
        assert_eq!(e.mem.u32(data + 4), 2);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data() {
        let mut e = engine();
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.new_object::<B>();
        let data = counted(&mut e, 1);
        e.set(this, B::m_spBoolData, data);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_3540, &args![this, 1u32]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![data]);
        assert_eq!(log.last().unwrap(), &(NI_DELETE, vec![this.addr(), 0x18]));
        e.set(this, B::m_spBoolData, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3540, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(KEY_BASED_INTERPOLATOR_DESTRUCTOR, vec![this.addr()])]
        );
    }

    #[test]
    fn create_clone_copies_into_a_new_interpolator() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x18);
        e.register_double(NI_NEW, move |_, _| word(block));
        let this = e.new_object::<B>();
        e.set(this, B::m_bBoolValue, 1);
        e.set(this, B::m_uiLastIdx, 3);
        assert_eq!(e.call(0x00a5_3590, &args![this, 0x900u32]).u32(), block);
        let clone = Ptr::<B>::new(block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.get(clone, B::m_bBoolValue), 1);
        assert_eq!(e.get(clone, B::m_uiLastIdx), 3);
    }

    #[test]
    fn load_binary_reads_the_value_and_links_the_data() {
        let mut e = engine();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        let this = e.new_object::<B>();
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 4, 0x40);
        e.mem.set_u32(file + 8, READ);
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], 1);
            word(1)
        });
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, file);
        let data = counted(&mut e, 1);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| word(data));
        e.call(0x00a5_3600, &args![this, stream]);
        assert_eq!(e.get(this, B::m_bBoolValue), 1);
        assert_eq!(e.mem.u32(file + 4), 0x41);
        assert_eq!(e.get(this, B::m_spBoolData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
    }
}
