//! `NiAnimation/nikeybasedinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyBasedInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_KEY_BASED_INTERPOLATOR_RTTI: u32 = 0x011f_3740;

// Translated from 00a3ca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_key_based_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_KEY_BASED_INTERPOLATOR_RTTI)
}

/// `00a88a40`: the jump target of `00aa0ac0`.
const JUMP_TARGET_00AA0AC0: u32 = 0x00a8_8a40;

/// `00a5d450`: the jump target of `00aa1000`.
const JUMP_TARGET_00AA1000: u32 = 0x00a5_d450;

// Translated from 00aa0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a88a40` (engine map: `NiInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiKeyBasedInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_key_based_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00AA0AC0, &args![this, stream]);
}

// Translated from 00aa1000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::CopyMembers` (Xbox PDB): a 5-byte jump to `00a5d450`, passing on `this` and its two arguments. Return type (void): Xbox PDB public name `?CopyMembers@NiKeyBasedInterpolator@@IAAXPAV1@AAVNiCloningProcess@@@Z`.
pub fn ni_key_based_interpolator_copy_members(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00AA1000, &args![this, first, second]);
}

/// The vtable of `NiKeyBasedInterpolator` (RTTI `.?AVNiKeyBasedInterpolator@@`).
const NI_KEY_BASED_INTERPOLATOR_VTABLE: u32 = 0x0109_7054;
/// `NiInterpolator::NiInterpolator` / `~NiInterpolator` (`00a56ff0` /
/// `00a57010`, `NiAnimation/niinterpolator.cpp`).
const NI_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_6ff0;
const NI_INTERPOLATOR_DESTRUCT: u32 = 0x00a5_7010;

// Translated from 00a3ca10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::NiKeyBasedInterpolator` (Xbox PDB): the
/// `NiInterpolator` constructor, then this class's vtable. Returns `this`.
pub fn ni_key_based_interpolator_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_INTERPOLATOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_KEY_BASED_INTERPOLATOR_VTABLE);
    this
}

// Translated from 00a3ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyBasedInterpolator::~NiKeyBasedInterpolator` (unnamed in the engine
/// map): stores this class's vtable, then the `NiInterpolator` destructor
/// (`00a57010`, a tail jump).
pub fn ni_key_based_interpolator_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_KEY_BASED_INTERPOLATOR_VTABLE);
    e.call(NI_INTERPOLATOR_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_ca10, ni_key_based_interpolator_construct(Ptr) -> Ptr),
        entry!(0x00a3_ca30, ni_key_based_interpolator_destruct(Ptr)),
        entry!(0x00a3ca40, ni_key_based_interpolator_get_rtti() -> Ptr),
        entry!(0x00aa_0ac0, ni_key_based_interpolator_load_binary(Ptr, Ptr)),
        entry!(
            0x00aa_1000,
            ni_key_based_interpolator_copy_members(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_key_based_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3ca40, &args![]).u32(),
            NI_KEY_BASED_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00aa0ac0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00aa_0ac0;
        const TARGET: u32 = 0x00a8_8a40;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00aa1000_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00aa_1000;
        const TARGET: u32 = 0x00a5_d450;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn constructor_and_destructor_chain_to_the_interpolator() {
        let mut e = Engine::new();
        e.register(NI_INTERPOLATOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(NI_INTERPOLATOR_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_ca10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_KEY_BASED_INTERPOLATOR_VTABLE);
        e.mem.set_u32(this, 0);
        e.call(0x00a3_ca30, &args![this]);
        assert_eq!(e.mem.u32(this), NI_KEY_BASED_INTERPOLATOR_VTABLE);
        let log = e.call_log.take().unwrap();
        let callees: Vec<_> = log
            .iter()
            .filter(|call| {
                call.0 == NI_INTERPOLATOR_CONSTRUCT || call.0 == NI_INTERPOLATOR_DESTRUCT
            })
            .cloned()
            .collect();
        assert_eq!(
            callees,
            [
                (NI_INTERPOLATOR_CONSTRUCT, vec![this]),
                (NI_INTERPOLATOR_DESTRUCT, vec![this]),
            ]
        );
    }
}
