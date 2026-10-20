//! `NiMain/nicollisionobject.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCollisionObject::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLLISION_OBJECT_RTTI: u32 = 0x011f_4ad4;

// Translated from 00a810a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_collision_object_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLLISION_OBJECT_RTTI)
}

/// `NiObject::LoadBinary` (`00a5d610`, `this` and the stream).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiStream` reader of a link id into the stream's link list (`00a645b0`,
/// the stream only; unnamed in the engine map).
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// The base `LinkObject` (`00a89af0`, `this` and the stream; its body is a
/// bare `RET 4`).
const NI_OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, the stream only).
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiAVObject::SetCollisionObject` (`0062bc90`, the scene object and the
/// collision object).
const NI_AV_OBJECT_SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
/// `NiObject`'s constructor (`00a5d3a0`, `this` only).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The `NiCollisionObject` vtable (its RTTI locator names
/// `.?AVNiCollisionObject@@`).
const NI_COLLISION_OBJECT_VTABLE: u32 = 0x0109_e174;
/// `NiCollisionObject::m_pkSceneObject` (Xbox PDB) +0x08.
const SCENE_OBJECT: u32 = 0x08;
/// `NiAVObject::m_spCollisionObject` (Xbox PDB) +0x1C.
const AV_OBJECT_COLLISION_OBJECT: u32 = 0x1c;

// Translated from 00a80fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::LoadBinary` (Xbox PDB): the `NiObject` part, then
/// the stream reads the scene object's link id (`00a645b0`).
pub fn ni_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a80ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::LinkObject` (Xbox PDB): the base part, then the scene
/// object is the stream's object for the next link id.
pub fn ni_collision_object_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LINK_OBJECT, &args![this, stream]);
    let object = e
        .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
        .u32();
    e.mem
        .set_u32(this.addr().wrapping_add(SCENE_OBJECT), object);
}

/// Stores `object` as the scene object and, when it is not null and does
/// not already hold `this` as its collision object, gives it `this`
/// (`NiAVObject::SetCollisionObject`).
fn set_scene_object(e: &mut Engine, this: Ptr, object: Ptr) {
    e.mem
        .set_u32(this.addr().wrapping_add(SCENE_OBJECT), object.addr());
    if !object.is_null()
        && e.mem
            .u32(object.addr().wrapping_add(AV_OBJECT_COLLISION_OBJECT))
            != this.addr()
    {
        e.call(NI_AV_OBJECT_SET_COLLISION_OBJECT, &args![object, this]);
    }
}

// Translated from 00a810b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::SetSceneGraphObject` (Xbox PDB): stores the scene
/// object and, when it is not null and its collision object is not already
/// `this`, sets `this` as its collision object (a tail jump).
pub fn ni_collision_object_set_scene_graph_object(e: &mut Engine, this: Ptr, object: Ptr) {
    set_scene_object(e, this, object);
}

// Translated from 00a810d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiCollisionObject` constructor taking the scene object: runs the
/// `NiObject` constructor, stores the class's vtable, then the scene object
/// as `SetSceneGraphObject` does. Returns `this`.
pub fn fn_00a810d0(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_COLLISION_OBJECT_VTABLE);
    set_scene_object(e, this, object);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a810a0, ni_collision_object_get_rtti() -> Ptr),
        entry!(0x00a80fd0, ni_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00a80ff0, ni_collision_object_link_object(Ptr, Ptr)),
        entry!(
            0x00a810b0,
            ni_collision_object_set_scene_graph_object(Ptr, Ptr)
        ),
        entry!(0x00a810d0, fn_00a810d0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_collision_object_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a810a0, &args![]).u32(), NI_COLLISION_OBJECT_RTTI);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn load_binary_loads_the_base_then_reads_the_link_id() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing);
        e.register(NI_STREAM_READ_LINK_ID, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0fd0, &args![0x100u32, 0x200u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_LOAD_BINARY),
            vec![vec![0x100, 0x200]]
        );
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID), vec![vec![0x200]]);
    }

    #[test]
    fn link_object_stores_the_linked_scene_object() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LINK_OBJECT, nothing);
        e.register(NI_STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| {
            0x4444u32.into_ret()
        });
        let this = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0ff0, &args![this, 0x200u32]);
        assert_eq!(calls_to(&e, NI_OBJECT_LINK_OBJECT), vec![vec![this, 0x200]]);
        assert_eq!(
            calls_to(&e, NI_STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x200]]
        );
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), 0x4444);
    }

    #[test]
    fn set_scene_graph_object_links_back_only_when_needed() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_SET_COLLISION_OBJECT, nothing);
        let this = e.mem.alloc(0x0c);
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // Null object: stored, nothing else.
        e.mem.set_u32(this + SCENE_OBJECT, 7);
        e.call(0x00a8_10b0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), 0);
        // The object already holds `this`.
        e.mem.set_u32(object + AV_OBJECT_COLLISION_OBJECT, this);
        e.call(0x00a8_10b0, &args![this, object]);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), object);
        assert!(calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT).is_empty());
        // Another collision object: gives it `this`.
        e.mem.set_u32(object + AV_OBJECT_COLLISION_OBJECT, 0);
        e.call(0x00a8_10b0, &args![this, object]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![object, this]]
        );
    }

    #[test]
    fn constructor_with_scene_object_sets_vtable_and_links() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        e.register(NI_AV_OBJECT_SET_COLLISION_OBJECT, nothing);
        let this = e.mem.alloc(0x0c);
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_10d0, &args![this, object]).u32(), this);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_COLLISION_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), object);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![object, this]]
        );
        let other = e.mem.alloc(0x0c);
        e.call(0x00a8_10d0, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other + SCENE_OBJECT), 0);
        assert_eq!(calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT).len(), 1);
    }
}
