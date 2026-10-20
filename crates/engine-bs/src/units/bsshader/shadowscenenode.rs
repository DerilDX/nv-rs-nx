//! `BSShader/shadowscenenode.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cb9ce0` (unnamed in the engine map): `thiscall`, `RET 8` (new capacity, element count);
/// reallocates the array's buffer.
const ARRAY_REALLOCATE: u32 = 0x00cb_9ce0;
/// The virtual slot (byte offset) that allocates a block of the requested number of elements.
const SLOT_ALLOCATE: u32 = 0x04;

// Translated from 00b5c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`shadowscenenode.cpp`). `__fastcall`: reserves the next slot of a
/// `BSSimpleArray`-shaped array (vtable, buffer at +4, count at +8, capacity at +0xc) and answers
/// its index. When the count equals the capacity: with no capacity yet, a four-element buffer
/// comes from virtual slot +4; otherwise the capacity doubles (grows by `0x400` once it is above
/// `0x400`) through `00cb9ce0`. This is the same body as the generic slot reservation
/// `00761540`, with the capacity helper inlined.
pub fn fn_00b5c360(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let size = e.mem.u32(base.wrapping_add(8));
    let capacity = e.mem.u32(base.wrapping_add(0xc));
    if size == capacity {
        if capacity == 0 {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![4u32]).u32();
            let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
            e.mem.set_u32(base.wrapping_add(8), count);
            e.mem.set_u32(base.wrapping_add(4), buffer);
            e.mem.set_u32(base.wrapping_add(0xc), 4);
            return count.wrapping_sub(1);
        }
        let new_capacity = if capacity > 0x400 {
            capacity.wrapping_add(0x400)
        } else {
            capacity.wrapping_mul(2)
        };
        e.call(ARRAY_REALLOCATE, &args![this, new_capacity, size]);
        e.mem.set_u32(base.wrapping_add(0xc), new_capacity);
    }
    let count = e.mem.u32(base.wrapping_add(8)).wrapping_add(1);
    e.mem.set_u32(base.wrapping_add(8), count);
    count.wrapping_sub(1)
}

layout! {
    /// `ShadowSceneNode` (Xbox PDB), an `NiNode`; 0x200 bytes on PC (`InitSceneGraph`
    /// allocates 0x200 for it at `0086d633`). Its own fields sit 0x24 lower on PC than in the
    /// Xbox PDB (the PC `NiNode` is 0xac bytes); the fields the constructor sets.
    pub struct ShadowSceneNode: 0x200 {
        /// `m_kWorldBound` (Xbox PDB, `NiAVObject`): `BSNiBound*`.
        0x20 m_kWorldBound: u32,
        /// `m_uFlags` (Xbox PDB, `NiAVObject`).
        0x30 m_uFlags: u32,
        /// `kShadowLightActiveFence` (Xbox PDB).
        0xD0 kShadowLightActiveFence: u32,
        /// `kShadowLightFadingInFence` (Xbox PDB).
        0xD4 kShadowLightFadingInFence: u32,
        /// `kShadowLightActiveFenceObject` (Xbox PDB): `NiPointer<ShadowSceneLight>`.
        0xD8 kShadowLightActiveFenceObject: u32,
        /// `kShadowLightFadingInFenceObject` (Xbox PDB): `NiPointer<ShadowSceneLight>`.
        0xDC kShadowLightFadingInFenceObject: u32,
        /// `pSunLight` (Xbox PDB): `ShadowSceneLight*`.
        0xE0 pSunLight: u32,
        /// `cSceneGraphIndex` (Xbox PDB).
        0x120 cSceneGraphIndex: u8,
        /// `pWindowReflectionEnvMap` (Xbox PDB): `NiPointer<BSRenderedTexture>`.
        0x124 pWindowReflectionEnvMap: u32,
        /// `spCubeMapCamera` (Xbox PDB): `NiPointer<BSCubeMapCamera>`.
        0x128 spCubeMapCamera: u32,
        /// `iCurrentCubeMapFace` (Xbox PDB).
        0x12C iCurrentCubeMapFace: u32,
        /// `bDisableLightUpdate` (Xbox PDB).
        0x130 bDisableLightUpdate: u8,
        /// `bWireframe` (Xbox PDB).
        0x131 bWireframe: u8,
        /// `spFog` (Xbox PDB): `NiPointer<BSFogProperty>`.
        0x134 spFog: u32,
        /// `bVisibleUnboundSpace` (Xbox PDB).
        0x1DC bVisibleUnboundSpace: u8,
        /// `pBSPortalGraph` (Xbox PDB).
        0x1E0 pBSPortalGraph: u32,
        /// `bAllowLightRemoveQueues` (Xbox PDB).
        0x1FC bAllowLightRemoveQueues: u8,
    }
}

/// The node's `NiTPointerList` members (Xbox PDB names), three words each, which the
/// constructor clears in the order `m_uiCount`, `m_pkHead`, `m_pkTail`: `lLightList`
/// (+0xb4), `lShadowLightList` (+0xc0), and the queues `lLightQueueAdd` (+0xe4),
/// `lLightQueueRemove` (+0xf0), `lLightQueueUpdate` (+0xfc), `lObjectQueueUpdate` (+0x108),
/// `lObjectQueueUpdateActiveOnly` (+0x114).
const LIGHT_LIST: u32 = 0xb4;
const SHADOW_LIGHT_LIST: u32 = 0xc0;
/// The `ShadowSceneNode` vtable (`010adcf8`).
const SHADOW_SCENE_NODE_VTABLE: u32 = 0x010a_dcf8;
/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map, `00c47660`), on `+0x138`.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
/// The node's name: `00a5b690` makes the `NiFixedString` of the text at `010adce0`, `00a5b950`
/// sets it (`thiscall`, a pointer to the string), and the string's release compares it with
/// the word at `0109b220` before decrementing the count 8 bytes before its text.
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const NAME_TEXT: u32 = 0x010a_dce0;
const FIXED_STRING_NONE: u32 = 0x0109_b220;
/// `NiTPointerListBase<..>::RemoveAll` (engine map, `00e74d40`), on the two light lists.
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// The allocation `00aa13e0` (`cdecl`, size), the memory manager singleton and its allocate.
const NEW_OBJECT: u32 = 0x00aa_13e0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `ShadowSceneLight::ShadowSceneLight` (`00b9fda0`, 0x250 bytes) and `SetLight`
/// (`00b9dda0`, `thiscall`, the light); `NiDirectionalLight::NiDirectionalLight` (`00a75c20`,
/// 0xfc bytes); `BSFogProperty::BSFogProperty` (`00bb8180`, 0x64 bytes).
const SHADOW_SCENE_LIGHT_CONSTRUCT: u32 = 0x00b9_fda0;
const SHADOW_SCENE_LIGHT_SET_LIGHT: u32 = 0x00b9_dda0;
const DIRECTIONAL_LIGHT_CONSTRUCT: u32 = 0x00a7_5c20;
const FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Three words copied into the node's points (`011f426c`) and into the light's two colours
/// (`011f4998`).
const POINT_011F426C: u32 = 0x011f_426c;
const COLOR_011F4998: u32 = 0x011f_4998;
/// The `float`s the fog's distances (`010ad6e0`) and colours (`01016248`) are set to.
const FOG_DISTANCE_010AD6E0: u32 = 0x010a_d6e0;
const FOG_COLOR_01016248: u32 = 0x0101_6248;

/// Copies the three words at `from` to `to`.
fn copy_three_words(e: &mut Engine, to: u32, from: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The inlined `NiPointer` release of the pointer `object` (not null): the reference count
/// at `+4` is decremented and the object deleted through its vtable `+4` when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The inlined `NiPointer` assignment of `object` to the word at `slot`.
fn assign(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old != object {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// A new `ShadowSceneLight` (0x250 bytes), or 0 when the allocation fails.
fn new_shadow_scene_light(e: &mut Engine) -> u32 {
    let memory = e.call(NEW_OBJECT, &args![0x250u32]).u32();
    if memory != 0 {
        e.call(SHADOW_SCENE_LIGHT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    }
}

// Translated from 00b5e0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::ShadowSceneNode` (Xbox PDB), `thiscall`, no stack words; answers
/// `this`. Constructs the `NiNode` (child array size 0), sets the vtable, clears the light
/// lists, fences, queues and pointers, constructs the compound frustum (`+0x138`), names the
/// node (the string at `010adce0`), empties the two light lists (`00e74d40`), releases the
/// window reflection map, sets `bVisibleUnboundSpace`, copies the point at `011f426c` to
/// `+0x1f0`; makes the two fence-object lights and the sun light (a `ShadowSceneLight` set to
/// a new `NiDirectionalLight` whose two colours at `+0xc8` and `+0xd4` are the words at
/// `011f4998`, its word at `+0xa8` incremented twice; the light is written even when its
/// allocation failed); sets flag 0x800, gives the node a world bound when it has none
/// (`SetBound`, vtable `+0x78`, with 0x10 bytes from the memory manager: the point at
/// `011f426c` and radius 0) and sets its radius to 1.0; sets flag 0x2000, copies the point
/// to `+0x1e4` and sets `bAllowLightRemoveQueues`; then makes the fog property (`spFog`)
/// with its four distances at the `float` at `010ad6e0` and its two colours (`+0x20`,
/// `+0x54`) at the `float` at `01016248`.
// Translation incomplete: 00b5e0f0 - the C++ exception frame (FS:[0] chain, unwind states 12 to 19) is not translated.
pub fn shadow_scene_node_shadow_scene_node(
    e: &mut Engine,
    this: Ptr<ShadowSceneNode>,
) -> Ptr<ShadowSceneNode> {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    e.mem.set_u32(base, SHADOW_SCENE_NODE_VTABLE);
    // `lLightList` and `lShadowLightList`: count, head, tail.
    for list in [LIGHT_LIST, SHADOW_LIGHT_LIST] {
        e.mem.set_u32(base + list + 8, 0);
        e.mem.set_u32(base + list, 0);
        e.mem.set_u32(base + list + 4, 0);
    }
    e.set(this, ShadowSceneNode::kShadowLightActiveFenceObject, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFenceObject, 0);
    // The five queues (`+0xe4` to `+0x11c`): count, head, tail.
    for list in [0xe4u32, 0xf0, 0xfc, 0x108, 0x114] {
        e.mem.set_u32(base + list + 8, 0);
        e.mem.set_u32(base + list, 0);
        e.mem.set_u32(base + list + 4, 0);
    }
    e.set(this, ShadowSceneNode::pWindowReflectionEnvMap, 0);
    e.set(this, ShadowSceneNode::spCubeMapCamera, 0);
    e.set(this, ShadowSceneNode::spFog, 0);
    e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![base + 0x138]);

    let name = e.call(FIXED_STRING_MAKE, &args![NAME_TEXT]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), name);
        e.call(SET_NAME, &args![base, slot]);
        let name = e.mem.u32(slot.addr());
        if name != e.global::<u32>(FIXED_STRING_NONE) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
        }
    });
    e.call(LIST_REMOVE_ALL, &args![base + LIGHT_LIST]);
    e.call(LIST_REMOVE_ALL, &args![base + SHADOW_LIGHT_LIST]);

    let map = e.get(this, ShadowSceneNode::pWindowReflectionEnvMap);
    if map != 0 {
        release(e, map);
        e.set(this, ShadowSceneNode::pWindowReflectionEnvMap, 0);
    }
    e.set(this, ShadowSceneNode::iCurrentCubeMapFace, 0);
    e.set(this, ShadowSceneNode::bDisableLightUpdate, 0);
    e.set(this, ShadowSceneNode::bVisibleUnboundSpace, 1);
    copy_three_words(e, base + 0x1f0, POINT_011F426C);
    e.set(this, ShadowSceneNode::pBSPortalGraph, 0);

    let light = new_shadow_scene_light(e);
    assign(e, base + 0xd8, light);
    let light = new_shadow_scene_light(e);
    assign(e, base + 0xdc, light);
    e.set(this, ShadowSceneNode::kShadowLightActiveFence, 0);
    e.set(this, ShadowSceneNode::kShadowLightFadingInFence, 0);

    let memory = e.call(NEW_OBJECT, &args![0xfcu32]).u32();
    let directional = if memory != 0 {
        e.call(DIRECTIONAL_LIGHT_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    copy_three_words(e, directional.wrapping_add(0xc8), COLOR_011F4998);
    let count = e.mem.u32(directional.wrapping_add(0xa8)).wrapping_add(1);
    e.mem.set_u32(directional.wrapping_add(0xa8), count);
    copy_three_words(e, directional.wrapping_add(0xd4), COLOR_011F4998);
    let count = e.mem.u32(directional.wrapping_add(0xa8)).wrapping_add(1);
    e.mem.set_u32(directional.wrapping_add(0xa8), count);
    let sun = new_shadow_scene_light(e);
    e.set(this, ShadowSceneNode::pSunLight, sun);
    e.call(SHADOW_SCENE_LIGHT_SET_LIGHT, &args![sun, directional]);

    let flags = e.get(this, ShadowSceneNode::m_uFlags);
    e.set(this, ShadowSceneNode::m_uFlags, flags | 0x800);
    if e.get(this, ShadowSceneNode::m_kWorldBound) == 0 {
        let bound = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
            .u32();
        e.vcall(base, 0x78, &args![bound]);
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        let x = e.global::<u32>(POINT_011F426C);
        e.mem.set_u32(bound, x);
        let y = e.global::<u32>(POINT_011F426C + 4);
        e.mem.set_u32(bound + 4, y);
        let z = e.global::<u32>(POINT_011F426C + 8);
        e.mem.set_u32(bound + 0xc, 0);
        e.mem.set_u32(bound + 8, z);
    }
    let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
    e.mem.set_u32(bound + 0xc, 1.0f32.to_bits());

    let flags = e.get(this, ShadowSceneNode::m_uFlags);
    e.set(this, ShadowSceneNode::m_uFlags, flags | 0x2000);
    e.set(this, ShadowSceneNode::bWireframe, 0);
    copy_three_words(e, base + 0x1e4, POINT_011F426C);
    e.set(this, ShadowSceneNode::bAllowLightRemoveQueues, 1);

    let memory = e.call(NEW_OBJECT, &args![0x64u32]).u32();
    let fog = if memory != 0 {
        e.call(FOG_PROPERTY_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    assign(e, base + 0x134, fog);

    // `BSFogProperty` (Xbox PDB): `fStartDistance` to `fEndWaterDistance` (+0x2c to +0x38),
    // then `m_kColor` (+0x20) and `kWaterColor` (+0x54); `FLD`/`FST` copies.
    let distance = x87_quiet_bits(e.global::<u32>(FOG_DISTANCE_010AD6E0));
    let fog = e.get(this, ShadowSceneNode::spFog);
    e.mem.set_u32(fog + 0x2c, distance);
    e.mem.set_u32(fog + 0x30, distance);
    let fog = e.get(this, ShadowSceneNode::spFog);
    e.mem.set_u32(fog + 0x34, distance);
    e.mem.set_u32(fog + 0x38, distance);
    let color = x87_quiet_bits(e.global::<u32>(FOG_COLOR_01016248));
    let fog = e.get(this, ShadowSceneNode::spFog);
    for offset in [0x20u32, 0x24, 0x28] {
        e.mem.set_u32(fog + offset, color);
    }
    let fog = e.get(this, ShadowSceneNode::spFog);
    for offset in [0x54u32, 0x58, 0x5c] {
        e.mem.set_u32(fog + offset, color);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_c360, fn_00b5c360(Ptr) -> u32),
        entry!(
            0x00b5_e0f0,
            shadow_scene_node_shadow_scene_node(Ptr<ShadowSceneNode>) -> Ptr<ShadowSceneNode>
        ),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.put_vtable(0x0f00_d100, &[0, 0x0f00_d000]);
        e.register(0x0f00_d000, |_, a| (0x4000_0000 + a[1]).into_ret());
        e.register(ARRAY_REALLOCATE, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x0f00_d100);
        (e, array)
    }

    fn set_counts(e: &mut Engine, array: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array + 4, 0x5000);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, capacity);
    }

    #[test]
    fn at_00b5c360_allocates_four_elements_for_an_empty_array() {
        let (mut e, array) = rig();
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 0);
        assert_eq!(calls(&e, 0x0f00_d000), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(array + 4), 0x4000_0004);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0xc), 4);
    }

    #[test]
    fn at_00b5c360_doubles_a_full_array_up_to_0x400_then_grows_by_0x400() {
        for (capacity, grown) in [
            (4u32, 8u32),
            (0x200, 0x400),
            (0x400, 0x800),
            (0x401, 0x801),
            (0x800, 0xc00),
        ] {
            let (mut e, array) = rig();
            set_counts(&mut e, array, capacity, capacity);
            assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), capacity);
            assert_eq!(
                calls(&e, ARRAY_REALLOCATE),
                vec![vec![array, grown, capacity]]
            );
            assert_eq!(e.mem.u32(array + 0xc), grown);
            assert_eq!(e.mem.u32(array + 8), capacity + 1);
            assert_eq!(e.mem.u32(array + 4), 0x5000);
        }
    }

    #[test]
    fn at_00b5c360_just_counts_when_there_is_room() {
        let (mut e, array) = rig();
        set_counts(&mut e, array, 3, 8);
        assert_eq!(e.call(0x00b5_c360, &args![array]).u32(), 3);
        assert_eq!(e.mem.u32(array + 8), 4);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert!(calls(&e, ARRAY_REALLOCATE).is_empty());
        assert!(calls(&e, 0x0f00_d000).is_empty());
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SET_BOUND: u32 = 0x0f00_2000;
    const OBJECT_VTABLE: u32 = 0x0f00_3000;
    const DELETE: u32 = 0x0f00_4000;

    /// Doubles for the constructor's callees: constructors answer `this` (objects get a
    /// vtable whose `+4` is `DELETE`), the node's vtable has `SetBound` at `+0x78`, which
    /// stores the bound at `+0x20`.
    fn rig(with_bound: bool) -> (Engine, Ptr<ShadowSceneNode>) {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0109_b000, 0x010a_d000, 0x011f_4000] {
            e.map(page, 0x1000);
        }
        e.set_global(FOG_DISTANCE_010AD6E0, 0x7f80_0001u32);
        e.set_global(FOG_COLOR_01016248, 0.5f32);
        for i in 0..3u32 {
            e.set_global(POINT_011F426C + 4 * i, 0x100 + i);
            e.set_global(COLOR_011F4998 + 4 * i, 0x200 + i);
        }
        e.set_global(FIXED_STRING_NONE, 0x0bad_0000u32);
        let mut slots = vec![0u32; 0x20];
        slots[0x78 / 4] = SET_BOUND;
        e.put_vtable(SHADOW_SCENE_NODE_VTABLE, &slots);
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE]);
        e.register(DELETE, |_, _| Ret::default());
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            if with_bound {
                let bound = e.mem.alloc(0x10);
                e.mem.set_u32(a[0] + 0x20, bound);
            }
            a[0].into_ret()
        });
        e.register(SET_BOUND, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        for constructor in [
            SHADOW_SCENE_LIGHT_CONSTRUCT,
            DIRECTIONAL_LIGHT_CONSTRUCT,
            FOG_PROPERTY_CONSTRUCT,
        ] {
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0], OBJECT_VTABLE);
                a[0].into_ret()
            });
        }
        e.register(FIXED_STRING_MAKE, |e, _| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 2);
            (block + 8).into_ret()
        });
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(SHADOW_SCENE_LIGHT_SET_LIGHT, |_, _| Ret::default());
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
        let this: Ptr<ShadowSceneNode> = e.new_object();
        // Garbage in every word the constructor must clear.
        for offset in (0xb4..0x200).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdddd_dddd);
        }
        (e, this)
    }

    #[test]
    fn constructor_builds_lights_bound_and_fog() {
        let (mut e, this) = rig(false);
        // The name setter leaves a reflection map behind: the constructor releases it.
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, OBJECT_VTABLE);
        e.mem.set_u32(map + 4, 1);
        e.register_double(SET_NAME, move |e, a| {
            e.mem.set_u32(a[0] + 0x124, map);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let base = this.addr();
        assert_eq!(e.call(0x00b5_e0f0, &args![this]).u32(), base);

        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![base, 0]]);
        assert_eq!(e.mem.u32(base), SHADOW_SCENE_NODE_VTABLE);
        for list in [0xb4u32, 0xc0, 0xe4, 0xf0, 0xfc, 0x108, 0x114] {
            for word in 0..3 {
                assert_eq!(e.mem.u32(base + list + 4 * word), 0, "{list:x}");
            }
        }
        assert_eq!(
            calls(&e, COMPOUND_FRUSTUM_CONSTRUCT),
            vec![vec![base + 0x138]]
        );
        assert_eq!(calls(&e, FIXED_STRING_MAKE), vec![vec![NAME_TEXT]]);
        // The name's temporary was released once (count 2 -> 1).
        let name_block = calls(&e, IMPORT_INTERLOCKED_DECREMENT)[0][0];
        assert_eq!(e.mem.u32(name_block), 1);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL),
            vec![vec![base + 0xb4], vec![base + 0xc0]]
        );
        assert_eq!(calls(&e, DELETE), vec![vec![map]]);
        assert_eq!(e.get(this, ShadowSceneNode::pWindowReflectionEnvMap), 0);
        assert_eq!(e.get(this, ShadowSceneNode::bVisibleUnboundSpace), 1);
        assert_eq!(e.get(this, ShadowSceneNode::bAllowLightRemoveQueues), 1);
        assert_eq!(e.get(this, ShadowSceneNode::bDisableLightUpdate), 0);
        assert_eq!(e.get(this, ShadowSceneNode::bWireframe), 0);
        assert_eq!(e.get(this, ShadowSceneNode::iCurrentCubeMapFace), 0);
        for (i, offset) in [0x1e4u32, 0x1e8, 0x1ec].iter().enumerate() {
            assert_eq!(e.mem.u32(base + offset), 0x100 + i as u32);
            assert_eq!(e.mem.u32(base + offset + 0xc), 0x100 + i as u32);
        }

        // Fence lights held once; the sun light is set to the directional light.
        for slot in [0xd8u32, 0xdc] {
            let light = e.mem.u32(base + slot);
            assert_ne!(light, 0);
            assert_eq!(e.mem.u32(light + 4), 1);
        }
        let directional = calls(&e, DIRECTIONAL_LIGHT_CONSTRUCT)[0][0];
        let sun = e.get(this, ShadowSceneNode::pSunLight);
        assert_eq!(
            calls(&e, SHADOW_SCENE_LIGHT_SET_LIGHT),
            vec![vec![sun, directional]]
        );
        assert_eq!(e.mem.u32(directional + 0xa8), 2);
        for i in 0..3u32 {
            assert_eq!(e.mem.u32(directional + 0xc8 + 4 * i), 0x200 + i);
            assert_eq!(e.mem.u32(directional + 0xd4 + 4 * i), 0x200 + i);
        }
        assert_eq!(calls(&e, NEW_OBJECT).len(), 5);

        // A new bound: the point, then radius 1.0.
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x10]]);
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        assert_eq!(calls(&e, SET_BOUND), vec![vec![base, bound]]);
        assert_eq!(e.mem.u32(bound), 0x100);
        assert_eq!(e.mem.u32(bound + 8), 0x102);
        assert_eq!(e.mem.f32(bound + 0xc), 1.0);
        assert_eq!(e.get(this, ShadowSceneNode::m_uFlags) & 0x2800, 0x2800);

        // The fog: held once, distances a quiet NaN (FLD of a signalling one), colours 0.5.
        let fog = e.get(this, ShadowSceneNode::spFog);
        assert_eq!(e.mem.u32(fog + 4), 1);
        for offset in [0x2cu32, 0x30, 0x34, 0x38] {
            assert_eq!(e.mem.u32(fog + offset), 0x7fc0_0001);
        }
        for offset in [0x20u32, 0x24, 0x28, 0x54, 0x58, 0x5c] {
            assert_eq!(e.mem.f32(fog + offset), 0.5);
        }
    }

    #[test]
    fn constructor_keeps_an_existing_bound_and_sets_its_radius() {
        let (mut e, this) = rig(true);
        e.call_log = Some(vec![]);
        e.call(0x00b5_e0f0, &args![this]);
        assert!(calls(&e, SET_BOUND).is_empty());
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, DELETE).is_empty());
        let bound = e.get(this, ShadowSceneNode::m_kWorldBound);
        assert_eq!(e.mem.f32(bound + 0xc), 1.0);
        assert_eq!(e.mem.u32(bound), 0);
    }
}
