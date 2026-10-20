//! `NiMain/nidynamiceffect.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDynamicEffect::ms_RTTI` (the class's `NiRTTI` record).
const NI_DYNAMIC_EFFECT_RTTI: u32 = 0x011f_4988;

// Translated from 00a69060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dynamic_effect_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DYNAMIC_EFFECT_RTTI)
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiDynamicEffect`'s vtable (`0109c174`; its RTTI type descriptor reads
/// `.?AVNiDynamicEffect@@`).
const NI_DYNAMIC_EFFECT_VTABLE: u32 = 0x0109_c174;
/// `NiDynamicEffect::ms_iNextIndex` (Xbox PDB static; the word at `011a9b58` the constructor
/// increments).
const NEXT_INDEX_011A9B58: u32 = 0x011a_9b58;

// Translated from 00a69320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::NiDynamicEffect` (Xbox PDB), `thiscall`, no stack words. Runs the
/// `NiAVObject` constructor (`00a5aa20`), stores the vtable `0109c174`, then (PC offsets, the
/// Xbox PDB's less 0x24): `m_ucEffectType` (+0x9d) 0xff, `m_uiPushCount` (+0xa4) 0, `m_bOn`
/// (+0x9c) 1, `m_uiRevID` (+0xa8) 1, the two node lists (+0xac..+0xc3) cleared, and
/// `m_iIndex` (+0xa0) the value `InterlockedIncrement` answers for `ms_iNextIndex`
/// (`011a9b58`). Answers `this`.
pub fn ni_dynamic_effect_ni_dynamic_effect(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_DYNAMIC_EFFECT_VTABLE);
    e.mem.set_u8(base + 0x9d, 0xff);
    e.mem.set_u32(base + 0xa4, 0);
    e.mem.set_u8(base + 0x9c, 1);
    e.mem.set_u32(base + 0xa8, 1);
    for offset in [0xb4u32, 0xac, 0xb0, 0xc0, 0xb8, 0xbc] {
        e.mem.set_u32(base + offset, 0);
    }
    let index = e
        .call(IMPORT_INTERLOCKED_INCREMENT, &args![NEXT_INDEX_011A9B58])
        .u32();
    e.mem.set_u32(base + 0xa0, index);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_9320, ni_dynamic_effect_ni_dynamic_effect(Ptr) -> Ptr),
        entry!(0x00a69060, ni_dynamic_effect_get_rtti() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dynamic_effect_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a69060, &args![]).u32(), NI_DYNAMIC_EFFECT_RTTI);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    #[test]
    fn ni_dynamic_effect_ni_dynamic_effect_sets_the_fields_and_takes_the_next_index() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.map(0x011a_9000, 0x1000);
        e.set_global::<u32>(NEXT_INDEX_011A9B58, 41);
        let this = e.mem.alloc(0xd0);
        for offset in (0x9cu32..0xc4).step_by(4) {
            e.mem.set_u32(this + offset, 0x5555_5555);
        }
        assert_eq!(e.call(0x00a6_9320, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DYNAMIC_EFFECT_VTABLE);
        assert_eq!(e.mem.u8(this + 0x9c), 1);
        assert_eq!(e.mem.u8(this + 0x9d), 0xff);
        assert_eq!(e.mem.u8(this + 0x9e), 0x55);
        assert_eq!(e.mem.u32(this + 0xa0), 42);
        assert_eq!(e.global::<u32>(NEXT_INDEX_011A9B58), 42);
        assert_eq!(e.mem.u32(this + 0xa4), 0);
        assert_eq!(e.mem.u32(this + 0xa8), 1);
        for offset in (0xacu32..0xc4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
    }
}
