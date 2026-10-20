//! `NiMain/bsresizabletrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSResizableTriShape::ms_RTTI` (the class's `NiRTTI` record).
const BS_RESIZABLE_TRI_SHAPE_RTTI: u32 = 0x011f_5b34;

// Translated from 00a86d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_resizable_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_RESIZABLE_TRI_SHAPE_RTTI)
}

/// `00a98eb0`: the jump target of `00a86d40`.
const JUMP_TARGET_00A86D40: u32 = 0x00a9_8eb0;

/// `00a74500`: the jump target of `00a87190`.
const JUMP_TARGET_00A87190: u32 = 0x00a7_4500;

/// `00a74510`: the jump target of `00a877f0`.
const JUMP_TARGET_00A877F0: u32 = 0x00a7_4510;

// Translated from 00a86d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::LinkObject` (Xbox PDB): a 5-byte jump to `00a98eb0` (engine map: `NiParticleMeshes::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSResizableTriShape@@UAAXAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A86D40, &args![this, stream]);
}

// Translated from 00a87190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a74500` (engine map: `NiParticleMeshes::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSResizableTriShape@@UAA_NAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A87190, &args![this, stream]).bool()
}

// Translated from 00a877f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::IsEqual` (Xbox PDB): a 5-byte jump to `00a74510` (engine map: `NiParticleMeshes::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSResizableTriShape@@UAA_NPAVNiObject@@@Z`.
pub fn bs_resizable_tri_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A877F0, &args![this, other]).bool()
}

/// The `MemoryManager` singleton and its `Allocate` (`thiscall` on it, `(size)`),
/// `NiBound::Update` (`thiscall` on the bound, `(&local bound, &transform)`) and the
/// default bound's centre (`011f426c`, three floats).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
const DEFAULT_BOUND_CENTER: u32 = 0x011f_426c;

// Translated from 00c1a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::UpdateWorldBound` (Xbox PDB): without a world bound (+0x20), a
/// 0x10-byte block (`MemoryManager::Allocate`) is given to the object's vtable slot +0x78
/// and the bound it then holds gets the centre at `011f426c` and radius 0. Then the world
/// bound is updated (`NiBound::Update`) from the local bound (`LocalBound`, +0xC8) and the
/// world transform (+0x68).
pub fn bs_resizable_tri_shape_update_world_bound(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr() + 0x20) == 0 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        e.vcall(this.addr(), 0x78, &args![block]);
        let bound = e.mem.u32(this.addr() + 0x20);
        for word in 0..3u32 {
            let value = e.global::<u32>(DEFAULT_BOUND_CENTER + 4 * word);
            e.mem.set_u32(bound + 4 * word, value);
        }
        e.mem.set_u32(bound + 0xc, x87_store32(0.0));
    }
    let bound = e.mem.u32(this.addr() + 0x20);
    e.call(
        BOUND_UPDATE,
        &args![bound, this.addr() + 0xc8, this.addr() + 0x68],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a86d20, bs_resizable_tri_shape_get_rtti() -> Ptr),
        entry!(0x00a8_6d40, bs_resizable_tri_shape_link_object(Ptr, Ptr)),
        entry!(0x00a8_7190, bs_resizable_tri_shape_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a8_77f0, bs_resizable_tri_shape_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c1a4a0, bs_resizable_tri_shape_update_world_bound(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_resizable_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a86d20, &args![]).u32(),
            BS_RESIZABLE_TRI_SHAPE_RTTI
        );
    }

    #[test]
    fn thunk_00a86d40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_6d40;
        const TARGET: u32 = 0x00a9_8eb0;
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
    fn thunk_00a87190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_7190;
        const TARGET: u32 = 0x00a7_4500;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a877f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_77f0;
        const TARGET: u32 = 0x00a7_4510;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn update_world_bound_makes_a_bound_when_missing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(BOUND_UPDATE, |_, _| Ret::default());
        e.register(0x0090_0078, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x1f];
        slots[0x1e] = 0x0090_0078;
        e.put_vtable(0x0090_1000, &slots);
        e.map(DEFAULT_BOUND_CENTER, 12);
        e.set_global::<f32>(DEFAULT_BOUND_CENTER + 4, 2.5);
        let this = e.mem.alloc(0xd8);
        e.mem.set_u32(this, 0x0090_1000);
        e.mem.set_f32(this + 0x20, 0.0);
        e.call(0x00c1a4a0, &args![this]);
        let bound = e.mem.u32(this + 0x20);
        assert_ne!(bound, 0);
        assert_eq!(e.mem.f32(bound + 4), 2.5);
        assert_eq!(e.mem.f32(bound + 0xc), 0.0);
        let updates: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == BOUND_UPDATE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(updates, vec![vec![bound, this + 0xc8, this + 0x68]]);
        // With a bound: only the update.
        e.mem.set_f32(bound + 4, 7.0);
        e.call(0x00c1a4a0, &args![this]);
        assert_eq!(e.mem.f32(bound + 4), 7.0);
        let allocations = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMORY_ALLOCATE)
            .count();
        assert_eq!(allocations, 1);
    }
}
