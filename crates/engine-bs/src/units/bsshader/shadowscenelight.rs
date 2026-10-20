//! `BSShader/shadowscenelight.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ShadowSceneLight` (Xbox PDB), an `NiRefObject`. The PC constructor (`00b9fda0`)
    /// writes the Xbox PDB's offsets; the fields its functions here use.
    pub struct ShadowSceneLight: 0x244 {
        /// `fLuminance` (Xbox PDB).
        0x0C fLuminance: f32,
        /// `fLODDimmer` (Xbox PDB).
        0xD0 fLODDimmer: f32,
        /// `fShadowLODDimmer` (Xbox PDB).
        0xD4 fShadowLODDimmer: f32,
        /// `fShadowLODDimmerTarget` (Xbox PDB).
        0xD8 fShadowLODDimmerTarget: f32,
        /// `fShadowLODDimmerElapsed` (Xbox PDB).
        0xDC fShadowLODDimmerElapsed: f32,
        /// `bCastShadow` (Xbox PDB).
        0xEC bCastShadow: u8,
        /// `bCubeShadowMap` (Xbox PDB).
        0xED bCubeShadowMap: u8,
        /// `spSpecularAttenuationTexture` (Xbox PDB): `NiPointer<NiTexture>`.
        0xF0 spSpecularAttenuationTexture: u32,
        /// `bPointLight` (Xbox PDB).
        0xF4 bPointLight: u8,
        /// `bAmbientLight` (Xbox PDB).
        0xF5 bAmbientLight: u8,
        /// `spLight` (Xbox PDB): `NiPointer<NiLight>`.
        0xF8 spLight: u32,
        /// `bDynamicLight` (Xbox PDB).
        0xFC bDynamicLight: u8,
        /// `bPointPosition` (Xbox PDB): an `NiPoint3`, three words.
        0x100 bPointPosition: u32,
        /// `spDynamicShadowMap` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x10C spDynamicShadowMap: u32,
        /// `usFrustumCull` (Xbox PDB).
        0x110 usFrustumCull: u16,
        /// `spFrustumVis` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x114 spFrustumVis: u32,
        /// `bShadowSpotlight` (Xbox PDB).
        0x118 bShadowSpotlight: u8,
        /// `fSpotlightFOV` (Xbox PDB).
        0x11C fSpotlightFOV: f32,
        /// `fSpotlightFalloff` (Xbox PDB).
        0x120 fSpotlightFalloff: f32,
        /// `bShowDebugTexture` (Xbox PDB).
        0x124 bShowDebugTexture: u8,
        /// `spShadowScene` (Xbox PDB): `NiPointer<NiAVObject>`.
        0x128 spShadowScene: u32,
        /// `kGeomListFence` (Xbox PDB).
        0x138 kGeomListFence: u32,
        /// `kGeomListFenceObject` (Xbox PDB): `NiPointer<NiTriBasedGeom>`.
        0x13C kGeomListFenceObject: u32,
        /// `spShadowCamera` (Xbox PDB): `NiPointer<NiCamera>`.
        0x140 spShadowCamera: u32,
        /// `kShadowCameraPlanes` (Xbox PDB): `NiFrustumPlanes`, six `NiPlane`s then the
        /// active-planes word at +0x1a4.
        0x144 kShadowCameraPlanes: u32,
        /// `pfClipSpacePlanes` (Xbox PDB): 0x60 bytes of floats.
        0x1A8 pfClipSpacePlanes: u32,
        /// `bClipSpacePlanesReady` (Xbox PDB).
        0x208 bClipSpacePlanesReady: u8,
        /// `spShadowAccumulator` (Xbox PDB): `NiPointer<BSShaderAccumulator>`.
        0x20C spShadowAccumulator: u32,
    }
}

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// An `NiPointer`'s release: the count at +4 decremented, the object deleted through its
/// vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointLight::ms_RTTI` (`011f4a98`) and `NiAmbientLight::ms_RTTI` (`011f5f6c`), the
/// addresses `nipointlight.rs` and `niambientlight.rs` answer from their `GetRTTI`.
const NI_POINT_LIGHT_RTTI: u32 = 0x011f_4a98;
const NI_AMBIENT_LIGHT_RTTI: u32 = 0x011f_5f6c;

/// The inlined kind-of test: the object's `GetRTTI` (vtable slot +8), then the chain of base
/// records (each `NiRTTI`'s word at +4) until `rtti` or null.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(4));
    }
    false
}

// Translated from 00b9dda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::SetLight` (Xbox PDB). `thiscall`, `RET 4`, one word (the light).
/// Assigns the light to `spLight` (the `NiPointer` assignment: the old one released, the new
/// one's count incremented, nothing when they are the same), sets `bPointLight` to whether
/// the light is an `NiPointLight` (kind-of through its RTTI chain; false for none); when it
/// is not, `fLODDimmer` becomes 1.0, and when it is and `bDynamicLight` is set, the light's
/// three words at +0x8c are copied into `bPointPosition`. Then `bAmbientLight` is set to
/// whether the light is an `NiAmbientLight`.
pub fn shadow_scene_light_set_light(e: &mut Engine, this: Ptr<ShadowSceneLight>, light: Ptr) {
    let light = light.addr();
    let old = e.get(this, ShadowSceneLight::spLight);
    let point = if old != light {
        if old != 0 {
            release(e, old);
        }
        e.set(this, ShadowSceneLight::spLight, light);
        if light == 0 {
            false
        } else {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![light.wrapping_add(4)]);
            is_kind_of(e, light, NI_POINT_LIGHT_RTTI)
        }
    } else {
        light != 0 && is_kind_of(e, light, NI_POINT_LIGHT_RTTI)
    };
    e.set(this, ShadowSceneLight::bPointLight, u8::from(point));
    if !point {
        e.set(
            this,
            ShadowSceneLight::fLODDimmer,
            f32::from_bits(x87_store32(1.0)),
        );
    } else if e.get(this, ShadowSceneLight::bDynamicLight) != 0 {
        let position = this.addr().wrapping_add(0x100);
        for word in 0..3u32 {
            let value = e.mem.u32(light.wrapping_add(0x8c + 4 * word));
            e.mem.set_u32(position.wrapping_add(4 * word), value);
        }
    }
    let ambient = light != 0 && is_kind_of(e, light, NI_AMBIENT_LIGHT_RTTI);
    e.set(this, ShadowSceneLight::bAmbientLight, u8::from(ambient));
}

/// `NiRefObject`'s vtable (`0101dce4`, RTTI `.?AVNiRefObject@@`) and its object count
/// (`NiRefObject::ms_uiObjects`, `011f4400`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// `ShadowSceneLight`'s vtable (`010b7eb8`, no RTTI locator before it).
const SHADOW_SCENE_LIGHT_VTABLE: u32 = 0x010b_7eb8;
/// `NiPlane::NiPlane` (engine map, `00a69940`): `thiscall`, no stack words.
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// The vtables of the three `BSSimpleArray` members: `kMultiboundRooms` and `kPortals`
/// (`010b7ea8`, no RTTI locator) and `kProcessedNodes` (`0101a8e8`, RTTI
/// `.?AV?$BSSimpleArray@PAVNiNode@@$0EAA@@`).
const SIMPLE_ARRAY_VTABLE_010B7EA8: u32 = 0x010b_7ea8;
const SIMPLE_ARRAY_VTABLE_0101A8E8: u32 = 0x0101_a8e8;
/// The three words `bPointPosition` is copied from (`011f426c`).
const POINT_POSITION_011F426C: u32 = 0x011f_426c;
/// The `float` `fSpotlightFOV` is loaded from (`010249d8`).
const SPOTLIGHT_FOV_010249D8: u32 = 0x0102_49d8;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;

/// The words the constructor clears first, in the exe's order: `lGeomList` (+0xe0: count,
/// head, tail), the `NiPointer`s, `lRootList` (+0x12c) and the fence objects.
const CLEARED_WORDS_00B9FDA0: [u32; 13] = [
    0xe8, 0xe0, 0xe4, 0xf0, 0xf8, 0x10c, 0x114, 0x128, 0x134, 0x12c, 0x130, 0x13c, 0x140,
];
/// The 4x4 float matrix at +0x10 (`kViewProjection`, Xbox PDB) set to the identity: the
/// words set to 0.0 and those set to 1.0, in the exe's order.
const MATRIX_ZEROS: [u32; 12] = [
    0x48, 0x44, 0x40, 0x3c, 0x34, 0x30, 0x2c, 0x28, 0x20, 0x1c, 0x18, 0x14,
];
const MATRIX_ONES: [u32; 4] = [0x4c, 0x38, 0x24, 0x10];

/// Releases and clears an `NiPointer` member when it is not null.
fn clear_pointer(e: &mut Engine, this: Ptr<ShadowSceneLight>, field: Field<ShadowSceneLight, u32>) {
    let object = e.get(this, field);
    if object != 0 {
        release(e, object);
        e.set(this, field, 0);
    }
}

// Translated from 00b9fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneLight::ShadowSceneLight` (Xbox PDB). `thiscall`, no stack words. The
/// `NiRefObject` part (its vtable, count 0, `ms_uiObjects` incremented through
/// `InterlockedIncrement`), this class's vtable (`010b7eb8`), the lists and pointers cleared,
/// the six `NiPlane`s of `kShadowCameraPlanes` constructed (`00a69940`) and its active-planes
/// word set to 0x3f, the three `BSSimpleArray`s set up (vtable, buffer, size and reserved
/// size 0); `fLuminance`, `fLODDimmer`, `fShadowLODDimmer` 0.0, `bCubeShadowMap` set then
/// cleared, the other flags cleared, `bPointPosition` copied from `011f426c`, `usFrustumCull`
/// 0; the pointers released again (already null, as the exe does it); `kViewProjection` the
/// identity; `fSpotlightFOV` from the `float` at `010249d8`, `fSpotlightFalloff` 1.0; the
/// rest of the pointers released and `kGeomListFence` cleared; `fShadowLODDimmerElapsed` 0.0,
/// `fShadowLODDimmerTarget` 1.0, `bClipSpacePlanesReady` cleared and the 0x60 bytes of
/// `pfClipSpacePlanes` cleared (`_memset`). All in the exe's order. Answers `this`.
// Translation incomplete: 00b9fda0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0xd) is not translated.
pub fn shadow_scene_light_shadow_scene_light(
    e: &mut Engine,
    this: Ptr<ShadowSceneLight>,
) -> Ptr<ShadowSceneLight> {
    let base = this.addr();
    let at = |offset: u32| base.wrapping_add(offset);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(at(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u32(base, SHADOW_SCENE_LIGHT_VTABLE);
    for offset in CLEARED_WORDS_00B9FDA0 {
        e.mem.set_u32(at(offset), 0);
    }
    for plane in 0..6u32 {
        e.call(NI_PLANE_CONSTRUCT, &args![at(0x144 + 0x10 * plane)]);
    }
    e.mem.set_u32(at(0x1a4), 0x3f);
    e.set(this, ShadowSceneLight::spShadowAccumulator, 0);
    for (array, vtable) in [
        (0x210u32, SIMPLE_ARRAY_VTABLE_010B7EA8),
        (0x220, SIMPLE_ARRAY_VTABLE_010B7EA8),
        (0x230, SIMPLE_ARRAY_VTABLE_0101A8E8),
    ] {
        e.mem.set_u32(at(array), vtable);
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(at(array + word), 0);
        }
    }

    let zero = f32::from_bits(x87_store32(0.0));
    let one = f32::from_bits(x87_store32(1.0));
    e.set(this, ShadowSceneLight::fLuminance, zero);
    e.set(this, ShadowSceneLight::fLODDimmer, zero);
    e.set(this, ShadowSceneLight::bCubeShadowMap, 1);
    e.set(this, ShadowSceneLight::fShadowLODDimmer, zero);
    e.set(this, ShadowSceneLight::bCastShadow, 0);
    e.set(this, ShadowSceneLight::bPointLight, 0);
    e.set(this, ShadowSceneLight::bAmbientLight, 0);
    e.set(this, ShadowSceneLight::bDynamicLight, 0);
    for word in 0..3u32 {
        let value = e.global::<u32>(POINT_POSITION_011F426C + 4 * word);
        e.mem.set_u32(at(0x100 + 4 * word), value);
    }
    e.set(this, ShadowSceneLight::usFrustumCull, 0);

    clear_pointer(e, this, ShadowSceneLight::spSpecularAttenuationTexture);
    clear_pointer(e, this, ShadowSceneLight::spLight);
    clear_pointer(e, this, ShadowSceneLight::spFrustumVis);
    clear_pointer(e, this, ShadowSceneLight::spDynamicShadowMap);

    for offset in MATRIX_ZEROS {
        e.mem.set_f32(at(offset), zero);
    }
    for offset in MATRIX_ONES {
        e.mem.set_f32(at(offset), one);
    }
    let fov = x87_quiet_bits(e.global::<u32>(SPOTLIGHT_FOV_010249D8));
    e.set(this, ShadowSceneLight::bCubeShadowMap, 0);
    e.set(this, ShadowSceneLight::fSpotlightFOV, f32::from_bits(fov));
    e.set(this, ShadowSceneLight::bShadowSpotlight, 0);
    e.set(this, ShadowSceneLight::bShowDebugTexture, 0);
    e.set(this, ShadowSceneLight::fSpotlightFalloff, one);

    clear_pointer(e, this, ShadowSceneLight::spShadowScene);
    e.set(this, ShadowSceneLight::kGeomListFence, 0);
    clear_pointer(e, this, ShadowSceneLight::kGeomListFenceObject);
    clear_pointer(e, this, ShadowSceneLight::spShadowCamera);

    e.set(this, ShadowSceneLight::fShadowLODDimmerElapsed, zero);
    e.set(this, ShadowSceneLight::fShadowLODDimmerTarget, one);
    e.set(this, ShadowSceneLight::bClipSpacePlanesReady, 0);
    e.call(MEMSET, &args![at(0x1a8), 0u32, 0x60u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b9_dda0,
            shadow_scene_light_set_light(Ptr<ShadowSceneLight>, Ptr)
        ),
        entry!(
            0x00b9_fda0,
            shadow_scene_light_shadow_scene_light(Ptr<ShadowSceneLight>) -> Ptr<ShadowSceneLight>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    const DELETE: u32 = 0x0f00_2000;
    const GET_RTTI: u32 = 0x0f00_2008;

    /// A light whose RTTI chain is `own -> base -> 0`, held `count` times, with the words
    /// 1, 2, 3 at +0x8c.
    fn light(e: &mut Engine, own: u32, base: u32, count: u32) -> u32 {
        e.mem.set_u32(own + 4, base);
        e.mem.set_u32(base + 4, 0);
        let light = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(16);
        e.mem.set_u32(vtable + 4, DELETE);
        e.mem.set_u32(vtable + 8, GET_RTTI);
        e.mem.set_u32(light, vtable);
        e.mem.set_u32(light + 4, count);
        e.mem.set_u32(light + 0xf8, own);
        for i in 0..3 {
            e.mem.set_u32(light + 0x8c + 4 * i, i + 1);
        }
        light
    }

    fn set_light_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        counting(&mut e);
        e.register(DELETE, |_, _| Ret::default());
        // GetRTTI answers the record kept at the light's +0xf8 (test only).
        e.register(GET_RTTI, |e, a| e.mem.u32(a[0] + 0xf8).into_ret());
        e
    }

    #[test]
    fn set_light_with_a_point_light_copies_its_position_when_dynamic() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x200);
        // A point light that derives from the ambient record (both flags).
        let lamp = light(&mut e, NI_POINT_LIGHT_RTTI, NI_AMBIENT_LIGHT_RTTI, 0);
        let old = light(&mut e, other, other + 0x40, 1);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, old);
        e.mem.set_u8(this + 0xfc, 1);
        e.mem.set_f32(this + 0xd0, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0xf8), lamp);
        assert_eq!(e.mem.u32(lamp + 4), 1);
        assert_eq!(e.mem.u8(this + 0xf4), 1);
        assert_eq!(e.mem.u8(this + 0xf5), 1);
        assert_eq!(e.mem.f32(this + 0xd0), 0.25);
        assert_eq!(
            [
                e.mem.u32(this + 0x100),
                e.mem.u32(this + 0x104),
                e.mem.u32(this + 0x108)
            ],
            [1, 2, 3]
        );
    }

    #[test]
    fn set_light_with_the_same_ambient_light_keeps_the_count_and_resets_the_dimmer() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let lamp = light(&mut e, other, NI_AMBIENT_LIGHT_RTTI, 1);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, lamp);
        e.mem.set_u8(this + 0xfc, 1);
        e.mem.set_u8(this + 0xf4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.mem.u32(lamp + 4), 1);
        assert_eq!(e.mem.u8(this + 0xf4), 0);
        assert_eq!(e.mem.u8(this + 0xf5), 1);
        assert_eq!(e.mem.f32(this + 0xd0), 1.0);
        assert_eq!(e.mem.u32(this + 0x100), 0);
    }

    #[test]
    fn set_light_with_a_static_point_light_keeps_the_position() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let lamp = light(&mut e, NI_POINT_LIGHT_RTTI, other, 0);
        let this = e.mem.alloc(0x244);
        e.call(0x00b9_dda0, &args![this, lamp]);
        assert_eq!(e.mem.u8(this + 0xf4), 1);
        assert_eq!(e.mem.u8(this + 0xf5), 0);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        assert_eq!(e.mem.f32(this + 0xd0), 0.0);
    }

    #[test]
    fn set_light_to_none_releases_the_old_light() {
        let mut e = set_light_rig();
        let other = e.mem.alloc(0x100);
        let old = light(&mut e, NI_POINT_LIGHT_RTTI, other, 2);
        let this = e.mem.alloc(0x244);
        e.mem.set_u32(this + 0xf8, old);
        e.mem.set_u8(this + 0xf4, 1);
        e.mem.set_u8(this + 0xf5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b9_dda0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(old + 4), 1);
        assert!(calls(&e, DELETE).is_empty());
        assert!(calls(&e, GET_RTTI).is_empty());
        assert_eq!(e.mem.u32(this + 0xf8), 0);
        assert_eq!(e.mem.u8(this + 0xf4), 0);
        assert_eq!(e.mem.u8(this + 0xf5), 0);
        assert_eq!(e.mem.f32(this + 0xd0), 1.0);
    }

    #[test]
    fn constructor_sets_every_field_it_owns() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        counting(&mut e);
        e.set_global(NI_REF_OBJECT_COUNT, 5u32);
        e.set_global(POINT_POSITION_011F426C, 0x11u32);
        e.set_global(POINT_POSITION_011F426C + 4, 0x22u32);
        e.set_global(POINT_POSITION_011F426C + 8, 0x33u32);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global(SPOTLIGHT_FOV_010249D8, 0x7f80_0001u32);
        e.register(NI_PLANE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x244);
        for offset in (0..0x244).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_fda0, &args![this]).u32(), this);

        assert_eq!(e.global::<u32>(NI_REF_OBJECT_COUNT), 6);
        assert_eq!(e.mem.u32(this), SHADOW_SCENE_LIGHT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        let planes: Vec<u32> = calls(&e, NI_PLANE_CONSTRUCT).iter().map(|c| c[0]).collect();
        assert_eq!(
            planes,
            (0..6).map(|i| this + 0x144 + 0x10 * i).collect::<Vec<_>>()
        );
        assert_eq!(e.mem.u32(this + 0x1a4), 0x3f);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x1a8, 0, 0x60]]);
        assert_eq!(e.mem.bytes(this + 0x1a8, 0x60), vec![0; 0x60]);
        // No pointer was held, so nothing was released.
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        for offset in CLEARED_WORDS_00B9FDA0 {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        for (array, vtable) in [
            (0x210u32, SIMPLE_ARRAY_VTABLE_010B7EA8),
            (0x220, SIMPLE_ARRAY_VTABLE_010B7EA8),
            (0x230, SIMPLE_ARRAY_VTABLE_0101A8E8),
        ] {
            assert_eq!(e.mem.u32(this + array), vtable);
            assert_eq!(e.mem.bytes(this + array + 4, 12), vec![0; 12]);
        }
        assert_eq!(e.mem.u32(this + 0x20c), 0);
        for (row, column) in (0..4u32).flat_map(|r| (0..4u32).map(move |c| (r, c))) {
            let want = if row == column { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 16 * row + 4 * column), want);
        }
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0xd0), 0);
        assert_eq!(e.mem.u32(this + 0xd4), 0);
        assert_eq!(e.mem.f32(this + 0xd8), 1.0);
        assert_eq!(e.mem.u32(this + 0xdc), 0);
        assert_eq!(e.mem.u32(this + 0xec), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0xf4), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0xfc), 0xdead_be00);
        assert_eq!(
            e.mem.bytes(this + 0x100, 12),
            [0x11u32, 0x22, 0x33]
                .iter()
                .flat_map(|w| w.to_le_bytes())
                .collect::<Vec<u8>>()
        );
        assert_eq!(e.mem.u32(this + 0x110), 0xdead_0000);
        assert_eq!(e.mem.u32(this + 0x118), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x11c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x120), 1.0);
        assert_eq!(e.mem.u32(this + 0x124), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x138), 0);
        assert_eq!(e.mem.u32(this + 0x208), 0xdead_be00);
        // +0x8 (iEnabledPasses) is not the constructor's.
        assert_eq!(e.mem.u32(this + 8), 0xdead_beef);
    }
}
