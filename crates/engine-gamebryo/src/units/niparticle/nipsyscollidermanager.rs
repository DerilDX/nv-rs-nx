//! `NiParticle/nipsyscollidermanager.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2780.
pub fn ni_p_sys_collider_manager_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2780
}

/// `00aa13e0` (`cdecl`, one word: the size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiPSysModifier::LoadBinary` (one word: the stream).
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
/// `00a645b0` (unit `nistream.cpp`, `thiscall` on the stream, no words): reads
/// one link ID.
const READ_LINK_ID: u32 = 0x00a6_45b0;
/// `00c31780` (unnamed; two words: the clone, the cloning process): the
/// modifier part of the member copy.
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
/// `NiPSysModifier::ProcessClone` (one word).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `NiPSysModifier::LinkObject` (one word).
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
/// `NiStream::GetObjectFromLinkID` (no words).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00c31720` (unnamed, no words): the `NiPSysModifier` part of the constructor.
const MODIFIER_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysModifier::~NiPSysModifier`.
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
/// The `NiPSysColliderManager` vtable.
const COLLIDER_MANAGER_VTABLE: u32 = 0x010c_008c;
/// `m_spColliders` (Xbox PDB), the `NiPointer<NiPSysCollider>` at `+0x18`.
const COLLIDERS: u32 = 0x18;

/// Stores `value` in the `NiPointer` at `slot` when it differs: the old
/// object released (`DeleteThis`, vtable `+4`, at a zero count), the new
/// one's count incremented.
fn assign_pointer(e: &mut Engine, slot: u32, value: u32) {
    let old = e.mem.u32(slot);
    if old != value {
        if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
            e.vcall(old, 4, &[]);
        }
        e.mem.set_u32(slot, value);
        if value != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![value + 4]);
        }
    }
}

// Translated from 00c2cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::LoadBinary` (Xbox PDB): the modifier part, then the
/// colliders' link ID (`00a645b0` on the stream).
pub fn ni_p_sys_collider_manager_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
}

// Translated from 00c2d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the clone, the cloning
/// process; the manager's `CopyMembers` shape): the modifier part
/// (`00c31780`), then, when this manager has colliders, their `CreateClone`
/// (vtable `+0x48`) goes into the clone's `m_spColliders`.
pub fn fn_00c2d3b0(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders == 0 {
        return;
    }
    let copy = e.vcall(colliders, 0x48, &args![process]).u32();
    assign_pointer(e, clone.addr() + COLLIDERS, copy);
}

// Translated from 00c2d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::ProcessClone` (Xbox PDB): the modifier part, then
/// the colliders' `ProcessClone` (vtable `+0x68`) when there are colliders.
pub fn ni_p_sys_collider_manager_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 {
        e.vcall(colliders, 0x68, &args![process]);
    }
}

// Translated from 00c2d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::LinkObject` (Xbox PDB): the modifier part, then the
/// linked object becomes `m_spColliders`.
pub fn ni_p_sys_collider_manager_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + COLLIDERS, object);
}

/// The inlined constructor: `00c31720`, the vtable, no colliders.
fn construct_collider_manager(e: &mut Engine, block: u32) -> Ptr {
    e.call(MODIFIER_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, COLLIDER_MANAGER_VTABLE);
    e.mem.set_u32(block + COLLIDERS, 0);
    Ptr::new(block)
}

// Translated from 00c2d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::CreateClone` (Xbox PDB): 0x1C bytes constructed as
/// a manager (null when the allocation fails, still passed on as the exe
/// does), the members copied (`fn_00c2d3b0`). Answers the clone.
pub fn ni_p_sys_collider_manager_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        construct_collider_manager(e, block)
    };
    fn_00c2d3b0(e, this, clone, process);
    clone
}

// Translated from 00c2d5b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's creation function (Ghidra made no function of it; listed for
/// `NiPSysColliderManager` in `00c1aa00`'s loader table; `cdecl`, no words):
/// 0x1C bytes constructed as a manager, or null.
pub fn fn_00c2d5b0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    construct_collider_manager(e, block)
}

// Translated from 00c2d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysColliderManager::_scalar_deleting_destructor_` (Xbox PDB): releases
/// the colliders (the field itself is left), the modifier destructor, then
/// the sized delete of 0x1C bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_p_sys_collider_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let colliders = e.mem.u32(this.addr() + COLLIDERS);
    if colliders != 0 && e.call(INTERLOCKED_DECREMENT, &args![colliders + 4]).u32() == 0 {
        e.vcall(colliders, 4, &[]);
    }
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c2d540, ni_p_sys_collider_manager_get_r_t_t_i() -> u32),
        entry!(0x00c2_cf90, ni_p_sys_collider_manager_load_binary(Ptr, Ptr)),
        entry!(0x00c2_d3b0, fn_00c2d3b0(Ptr, Ptr, Ptr)),
        entry!(
            0x00c2_d410,
            ni_p_sys_collider_manager_process_clone(Ptr, Ptr)
        ),
        entry!(0x00c2_d440, ni_p_sys_collider_manager_link_object(Ptr, Ptr)),
        entry!(0x00c2_d550, ni_p_sys_collider_manager_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c2_d5b0, fn_00c2d5b0() -> Ptr),
        entry!(
            0x00c2_d5e0,
            ni_p_sys_collider_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_collider_manager_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_d540, &args![]).u32(), 0x0120_2780);
    }
}

#[cfg(test)]
mod manager_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn with_refcounts(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            word(v)
        });
    }

    const DELETE_THIS: u32 = 0x0100_0004;
    const CREATE_CLONE: u32 = 0x0100_0048;
    const PROCESS_CLONE: u32 = 0x0100_0068;

    fn collider(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.mem.set_u32(vtable + 0x48, CREATE_CLONE);
        e.mem.set_u32(vtable + 0x68, PROCESS_CLONE);
        let object = e.mem.alloc(0x2c);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn load_binary_reads_the_modifier_and_one_link_id() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c2_cf90, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_LOAD_BINARY),
            vec![vec![0x5000, 0x6000]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![0x6000]]);
    }

    #[test]
    fn copy_members_clones_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let copy = collider(&mut e, 0);
        e.register_double(CREATE_CLONE, move |_, _| word(copy));
        let this = e.mem.alloc(0x1c);
        let clone = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        // Without colliders only the modifier part runs.
        e.call(0x00c2_d3b0, &args![this, clone, 0x7000u32]);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert!(calls_to(&e, CREATE_CLONE).is_empty());
        let own = collider(&mut e, 1);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call(0x00c2_d3b0, &args![this, clone, 0x7000u32]);
        assert_eq!(calls_to(&e, CREATE_CLONE), vec![vec![own, 0x7000]]);
        assert_eq!(e.mem.u32(clone + COLLIDERS), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
    }

    #[test]
    fn process_clone_forwards_to_the_colliders() {
        let mut e = Engine::new();
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d410, &args![this, 0x7000u32]);
        assert!(calls_to(&e, PROCESS_CLONE).is_empty());
        let own = collider(&mut e, 1);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call(0x00c2_d410, &args![this, 0x7000u32]);
        assert_eq!(calls_to(&e, PROCESS_CLONE), vec![vec![own, 0x7000]]);
        assert_eq!(calls_to(&e, MODIFIER_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn link_object_replaces_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(DELETE_THIS, |_, _| Ret::default());
        let linked = collider(&mut e, 0);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| word(linked));
        let old = collider(&mut e, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + COLLIDERS, old);
        e.call_log = Some(vec![]);
        e.call(0x00c2_d440, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + COLLIDERS), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
    }

    #[test]
    fn create_clone_and_create_function() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MODIFIER_CONSTRUCT, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_d550, &args![this, 0x7000u32]).u32();
        assert_eq!(e.mem.u32(clone), COLLIDER_MANAGER_VTABLE);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        let made = e.call(0x00c2_d5b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), COLLIDER_MANAGER_VTABLE);
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x1c], vec![0x1c]]);
        // A failed allocation: the copy still runs, on a null clone.
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_d550, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS)[1],
            vec![this, 0, 0x7000]
        );
        assert_eq!(e.call(0x00c2_d5b0, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_colliders() {
        let mut e = Engine::new();
        with_refcounts(&mut e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
        e.register(NI_SIZED_DELETE, |_, _| Ret::default());
        let own = collider(&mut e, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + COLLIDERS, own);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_d5e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![own]]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE), vec![vec![this, 0x1c]]);
        e.mem.set_u32(this + COLLIDERS, 0);
        e.call(0x00c2_d5e0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, NI_SIZED_DELETE).len(), 1);
    }
}
