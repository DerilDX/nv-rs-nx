//! `NiMain/nigeometry.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometry::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOMETRY_RTTI: u32 = 0x011f_4acc;

// Translated from 00a80530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geometry_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOMETRY_RTTI)
}

/// `00a59dc0`: the jump target of `00a7fdb0`.
const JUMP_TARGET_00A7FDB0: u32 = 0x00a5_9dc0;

/// `00a5c6a0`: the jump target of `00a7fdc0`.
const JUMP_TARGET_00A7FDC0: u32 = 0x00a5_c6a0;

// Translated from 00a7fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetBound` (Xbox PDB): a 5-byte jump to `00a59dc0` (engine map: `NiAVObject::SetBound`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetBound@NiGeometry@@UAAXPAVBSNiBound@@@Z`.
pub fn ni_geometry_set_bound(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00A7FDB0, &args![this, arg]);
}

// Translated from 00a7fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::PostLinkObject` (Xbox PDB): a 5-byte jump to `00a5c6a0` (engine map: `NiObjectNET::PostLinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?PostLinkObject@NiGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_geometry_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A7FDC0, &args![this, stream]);
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// `NiPropertyState::NiPropertyState` (engine map, `00a89ab0`, `thiscall`, no stack words).
const NI_PROPERTY_STATE_CONSTRUCT: u32 = 0x00a8_9ab0;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiGeometry`'s vtable (`0109e04c`; its RTTI type descriptor reads `.?AVNiGeometry@@`).
const NI_GEOMETRY_VTABLE: u32 = 0x0109_e04c;

// Translated from 00a804e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::NiGeometry` (Xbox PDB), `thiscall` with one stack word (the geometry data).
/// Runs the `NiAVObject` constructor (`00a5aa20`), stores the vtable `0109e04c`, constructs
/// `m_PropertyState` at +0x9c (`00a89ab0`), keeps the data in `m_spModelData` (+0xb8; its
/// reference count at +4 incremented through `InterlockedIncrement` when it is not null) and
/// clears `m_spSkinInstance` (+0xbc) and `m_spShader` (+0xc0). PC offsets are the Xbox PDB's
/// less 0x24 (the PC `NiAVObject` is 0x9c bytes). Answers `this`.
pub fn ni_geometry_ni_geometry(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    e.call(NI_PROPERTY_STATE_CONSTRUCT, &args![base + 0x9c]);
    e.mem.set_u32(base + 0xb8, data.addr());
    if data.addr() != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![data.addr() + 4]);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_04e0, ni_geometry_ni_geometry(Ptr, Ptr) -> Ptr),
        entry!(0x00a80530, ni_geometry_get_rtti() -> Ptr),
        entry!(0x00a7_fdb0, ni_geometry_set_bound(Ptr, Ptr)),
        entry!(0x00a7_fdc0, ni_geometry_post_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geometry_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a80530, &args![]).u32(), NI_GEOMETRY_RTTI);
    }

    #[test]
    fn thunk_00a7fdb0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdb0;
        const TARGET: u32 = 0x00a5_9dc0;
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
    fn thunk_00a7fdc0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdc0;
        const TARGET: u32 = 0x00a5_c6a0;
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
mod constructor_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_PROPERTY_STATE_CONSTRUCT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn ni_geometry_ni_geometry_keeps_the_data_and_counts_a_reference() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0xbc, 9);
        e.mem.set_u32(this + 0xc0, 9);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 4, 2);
        assert_eq!(e.call(0x00a8_04e0, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(data + 4), 3);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(calls(&e, NI_AV_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&e, NI_PROPERTY_STATE_CONSTRUCT),
            vec![vec![this + 0x9c]]
        );
    }

    #[test]
    fn ni_geometry_ni_geometry_without_data_counts_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.call(0x00a8_04e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
    }
}
