//! `BSMain/bsportal.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c4f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPortalSharedNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c4f550` and frees 0xc4 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_portal_shared_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c4_f550, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xc4u32]);
    }
    this
}

// Translated from 00c4f8d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012030b8 without reading the receiver. The map names this body `BSPortalSharedNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c4f8d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_30b8
}

/// `00a5ecb0` (`thiscall`, one word; unit `ninode.cpp`, the `NiNode` constructor shape).
const NODE_CONSTRUCT_00A5ECB0: u32 = 0x00a5_ecb0;
/// `BSPortalSharedNode`'s vtable (RTTI `.?AVBSPortalSharedNode@@`).
const PORTAL_SHARED_NODE_VTABLE: u32 = 0x010c_21fc;
/// `BSSimpleArray<ShadowSceneLight*,1024>::_InitialSetup` (engine map, `00c3a220`;
/// `thiscall`, no stack words).
const LIGHT_ARRAY_CONSTRUCT: u32 = 0x00c3_a220;
/// The memory manager (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `BSCompoundFrustum::BSCompoundFrustum` (engine map, `00c47660`; `thiscall`, no stack
/// words) and the frustum's size.
const COMPOUND_FRUSTUM_CONSTRUCT: u32 = 0x00c4_7660;
const COMPOUND_FRUSTUM_SIZE: u32 = 0xa4;
/// `BSPortalSharedNode` (Xbox PDB) on PC, 0x24 lower than the PDB (as `m_kChildren`, PDB
/// +0xC0, is at +0x9C): `pCompoundFrustum` +0xAC, `kLights` +0xB0, `pPortal` +0xC0;
/// `NiAVObject::m_uFlags` at +0x30.
const COMPOUND_FRUSTUM: u32 = 0xac;
const LIGHTS: u32 = 0xb0;
const PORTAL: u32 = 0xc0;
const AV_OBJECT_FLAGS: u32 = 0x30;

// Translated from 00c4f460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4f460 - the compiler's exception frame (FS:[0] chain, handler 00f2eb9e, states 0 to 2 that free the frustum block when its constructor throws) is not translated
/// `BSPortalSharedNode::BSPortalSharedNode` (engine map; `thiscall`, one word: the portal;
/// `RET 4`): the node base (`00a5ecb0` with 0), the vtable, the light array (`00c3a220`), a
/// new 0xA4-byte `BSCompoundFrustum` from the memory manager (null when the allocation
/// fails), flag `0x800` of `m_uFlags` set (the compiler kept a test of the constant 1; its
/// clearing branch never runs) and `pPortal`. Returns `this`.
pub fn bs_portal_shared_node_bs_portal_shared_node(e: &mut Engine, this: Ptr, portal: u32) -> Ptr {
    let base = this.addr();
    e.call(NODE_CONSTRUCT_00A5ECB0, &args![this, 0u32]);
    e.mem.set_u32(base, PORTAL_SHARED_NODE_VTABLE);
    e.call(LIGHT_ARRAY_CONSTRUCT, &args![base + LIGHTS]);
    let block = e
        .call(
            MEMORY_ALLOCATE,
            &args![MEMORY_MANAGER, COMPOUND_FRUSTUM_SIZE],
        )
        .u32();
    let frustum = if block != 0 {
        e.call(COMPOUND_FRUSTUM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + COMPOUND_FRUSTUM, frustum);
    // `MOV ECX,1; TEST ECX,ECX; JZ`: the set branch always runs; the clear branch is the
    // exe's unreachable one (`black_box` keeps it as written).
    let flags = e.mem.u32(base + AV_OBJECT_FLAGS);
    if std::hint::black_box(1u32) != 0 {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, flags | 0x800);
    } else {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, !0x800 & flags);
    }
    e.mem.set_u32(base + PORTAL, portal);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_f460, bs_portal_shared_node_bs_portal_shared_node(Ptr, u32) -> Ptr),
        entry!(0x00c4_f8d0, fn_00c4f8d0(Ptr) -> u32),
        entry!(0x00c4_f820, bs_portal_shared_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_portal_shared_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c4_f820, 0x00c4_f550, 0xc4);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c4f8d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_f8d0, &args![0x1000u32]).u32(), 0x0120_30b8);
        assert_eq!(e.call(0x00c4_f8d0, &args![0u32]).u32(), 0x0120_30b8);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn rig(block: u32) -> Engine {
        let mut e = Engine::new();
        e.register(NODE_CONSTRUCT_00A5ECB0, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x10);
            Ret::default()
        });
        e.register(LIGHT_ARRAY_CONSTRUCT, |_, _| Ret::default());
        e.register_double(MEMORY_ALLOCATE, move |_, a| {
            assert_eq!(a, [MEMORY_MANAGER, COMPOUND_FRUSTUM_SIZE]);
            block.into_ret()
        });
        e.register(COMPOUND_FRUSTUM_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn the_shared_node_constructor_builds_its_frustum() {
        let mut e = rig(0x0d00_0000);
        let this = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        let back = e.call(0x00c4_f460, &args![this, 0x7000u32]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this), PORTAL_SHARED_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0x0d00_0000);
        assert_eq!(e.mem.u32(this + 0x30), 0x810);
        assert_eq!(e.mem.u32(this + PORTAL), 0x7000);
        let log = e.call_log.take().unwrap();
        let node = log
            .iter()
            .find(|(a, _)| *a == NODE_CONSTRUCT_00A5ECB0)
            .unwrap();
        assert_eq!(node.1, vec![this, 0]);
        let lights = log
            .iter()
            .find(|(a, _)| *a == LIGHT_ARRAY_CONSTRUCT)
            .unwrap();
        assert_eq!(lights.1, vec![this + LIGHTS]);
    }

    #[test]
    fn a_failed_allocation_leaves_no_frustum() {
        let mut e = rig(0);
        let this = e.mem.alloc(0xc4);
        e.mem.set_u32(this + COMPOUND_FRUSTUM, 0xdddd_dddd);
        e.call_log = Some(vec![]);
        e.call(0x00c4_f460, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != COMPOUND_FRUSTUM_CONSTRUCT));
    }
}
