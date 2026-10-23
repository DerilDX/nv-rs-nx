//! `BSMain/bsmultiboundroom.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c3a140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundRoom` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c39770` and frees 0xec bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_multi_bound_room_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c3_9770, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xecu32]);
    }
    this
}

// Translated from 00c3a610 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202828 without reading the receiver. The map names this body `BSMultiBoundRoom::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c3a610(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2828
}

/// `BSMultiBoundNode::BSMultiBoundNode` (engine map, `00c46970`; `thiscall`, no stack words).
const MULTI_BOUND_NODE_CONSTRUCT: u32 = 0x00c4_6970;
/// `BSMultiBoundRoom`'s vtable (RTTI `.?AVBSMultiBoundRoom@@`).
const MULTI_BOUND_ROOM_VTABLE: u32 = 0x010c_1064;
/// `BSSimpleArray<ShadowSceneLight*,1024>::_InitialSetup` (engine map, `00c3a220`;
/// `thiscall`, no stack words): the array's constructor.
const LIGHT_ARRAY_CONSTRUCT: u32 = 0x00c3_a220;
/// `BSMultiBoundRoom` (Xbox PDB) on PC, 0x2C lower than the PDB's offsets (as
/// `BSMultiBoundNode::spMultiBound`, PDB +0xD0, is at +0xAC): `kPortalList` +0xB4,
/// `lOccluderList` +0xC0, `pCompoundFrustum` +0xCC, `kJoinedMultiBoundList` +0xD0 (each list
/// an `NiTPointerList` of head, tail and count) and `kLights` +0xDC; `NiAVObject::m_uFlags`
/// at +0x30.
const PORTAL_LIST: u32 = 0xb4;
const OCCLUDER_LIST: u32 = 0xc0;
const COMPOUND_FRUSTUM: u32 = 0xcc;
const JOINED_MULTI_BOUND_LIST: u32 = 0xd0;
const LIGHTS: u32 = 0xdc;
const AV_OBJECT_FLAGS: u32 = 0x30;

/// An inlined `NiTPointerList` constructor: count (+8), head and tail cleared in the exe's
/// order.
fn clear_pointer_list(e: &mut Engine, list: u32) {
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

// Translated from 00c39640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c39640 - the compiler's exception frame (FS:[0] chain, handler 00f2da62, states 0 to 3) is not translated
/// `BSMultiBoundRoom::BSMultiBoundRoom` (engine map; `thiscall`, no stack words): the
/// `BSMultiBoundNode` base (`00c46970`), the vtable, the three pointer lists cleared, the
/// light array constructed (`00c3a220`), flag `0x800` of `m_uFlags` set (the compiler kept a
/// test of the constant 1 here; its clearing branch never runs) and `pCompoundFrustum`
/// cleared. Returns `this`.
pub fn bs_multi_bound_room_bs_multi_bound_room(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MULTI_BOUND_NODE_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, MULTI_BOUND_ROOM_VTABLE);
    clear_pointer_list(e, base + PORTAL_LIST);
    clear_pointer_list(e, base + OCCLUDER_LIST);
    clear_pointer_list(e, base + JOINED_MULTI_BOUND_LIST);
    e.call(LIGHT_ARRAY_CONSTRUCT, &args![base + LIGHTS]);
    // `MOV ECX,1; TEST ECX,ECX; JZ`: the set branch always runs; the clear branch is the
    // exe's unreachable one (`black_box` keeps it as written).
    let flags = e.mem.u32(base + AV_OBJECT_FLAGS);
    if std::hint::black_box(1u32) != 0 {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, flags | 0x800);
    } else {
        e.mem.set_u32(base + AV_OBJECT_FLAGS, !0x800 & flags);
    }
    e.mem.set_u32(base + COMPOUND_FRUSTUM, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_9640, bs_multi_bound_room_bs_multi_bound_room(Ptr) -> Ptr),
        entry!(0x00c3_a610, fn_00c3a610(Ptr) -> u32),
        entry!(0x00c3_a140, bs_multi_bound_room_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c3_a5b0,
            bs_multi_bound_node_update_downward_pass(Ptr, u32, u32)
        ),
        entry!(0x00c3_a5e0, bs_multi_bound_node_update_upward_pass(Ptr)),
        entry!(0x00c3_a640, fn_00c3a640(Ptr, u32) -> u32),
        entry!(0x00c3_a670, fn_00c3a670(Ptr, u32, u32) -> u32),
    ]
}

/// `NiNode::UpdateDownwardPass` (`00a5dd70`, `RET 8`) and `NiNode::UpdateUpwardPass`
/// (`00a5db30`), `thiscall`.
const NI_NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
const NI_NODE_UPDATE_UPWARD_PASS: u32 = 0x00a5_db30;
/// The memory manager object (`011f6238`) and its `thiscall` allocate (`00aa3e40`, one
/// word) and `00aa4150` (block and size; it answers a block, which `0042f5d0` and
/// `0084b630` pass on as the reallocated one).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE_SIZED: u32 = 0x00aa_4150;

// Translated from 00c3a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::UpdateDownwardPass` (Xbox PDB name from the engine map; `thiscall`,
/// `RET 8`): `NiNode::UpdateDownwardPass` with the two words, then the object's own
/// virtual slot `0xbc`.
pub fn bs_multi_bound_node_update_downward_pass(e: &mut Engine, this: Ptr, a1: u32, a2: u32) {
    e.call(NI_NODE_UPDATE_DOWNWARD_PASS, &args![this, a1, a2]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00c3a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::UpdateUpwardPass` (Xbox PDB name from the engine map; `thiscall`):
/// `NiNode::UpdateUpwardPass`, then the object's own virtual slot `0xbc`.
pub fn bs_multi_bound_node_update_upward_pass(e: &mut Engine, this: Ptr) {
    e.call(NI_NODE_UPDATE_UPWARD_PASS, &args![this]);
    e.vcall(this.addr(), 0xbc, &[]);
}

// Translated from 00c3a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsmultiboundroom.obj`; `thiscall`, `RET 4`, `this` not
/// read): an allocator's allocate of `count` pointers, `count * 4` bytes from the memory
/// manager (`00aa3e40`).
pub fn fn_00c3a640(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, count << 2])
        .u32()
}

// Translated from 00c3a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsmultiboundroom.obj`; `thiscall`, `RET 8`, `this` not
/// read): `00aa4150` on the memory manager with `block` and `count * 4`, answering the block
/// it gives (the exe ends on that call and leaves its `EAX`). It sits in the vtable slot
/// `+0xC` of a `BSSimpleArray` (RTTI), `_Reallocate` in the Xbox PDB, which answers the new
/// buffer.
pub fn fn_00c3a670(e: &mut Engine, _this: Ptr, block: u32, count: u32) -> u32 {
    e.call(MEMORY_FREE_SIZED, &args![MEMORY_MANAGER, block, count << 2])
        .u32()
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_update_passes_run_the_node_pass_then_slot_0xbc() {
        let mut e = Engine::new();
        const VTABLE: u32 = 0x0300_0000;
        const SLOT: u32 = 0x0300_1000;
        let mut slots = vec![0u32; 0x30];
        slots[0xbc / 4] = SLOT;
        e.put_vtable(VTABLE, &slots);
        for address in [
            NI_NODE_UPDATE_DOWNWARD_PASS,
            NI_NODE_UPDATE_UPWARD_PASS,
            SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xec);
        e.mem.set_u32(this, VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00c3_a5b0, &args![this, 5u32, 6u32]);
        e.call(0x00c3_a5e0, &args![this]);
        assert_eq!(
            calls(&e, NI_NODE_UPDATE_DOWNWARD_PASS),
            vec![vec![this, 5, 6]]
        );
        assert_eq!(calls(&e, NI_NODE_UPDATE_UPWARD_PASS), vec![vec![this]]);
        assert_eq!(calls(&e, SLOT), vec![vec![this]; 2]);
    }

    #[test]
    fn the_allocator_scales_by_four() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |_, a| (a[1] + 0x1000).into_ret());
        e.register(MEMORY_FREE_SIZED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_a640, &args![0u32, 3u32]).u32(), 0x100c);
        e.call(0x00c3_a670, &args![0u32, 0x5000u32, 3u32]);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 12]]);
        assert_eq!(
            calls(&e, MEMORY_FREE_SIZED),
            vec![vec![MEMORY_MANAGER, 0x5000, 12]]
        );
    }
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
    fn bs_multi_bound_room_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c3_a140, 0x00c3_9770, 0xec);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c3a610_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_a610, &args![0x1000u32]).u32(), 0x0120_2828);
        assert_eq!(e.call(0x00c3_a610, &args![0u32]).u32(), 0x0120_2828);
    }

    #[test]
    fn fn_00c3a670_answers_the_block_the_manager_gives() {
        let mut e = Engine::new();
        e.register(MEMORY_FREE_SIZED, |_, a| (a[1] + a[2]).into_ret());
        assert_eq!(
            e.call(0x00c3_a670, &args![0u32, 0x5000u32, 3u32]).u32(),
            0x500c
        );
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn the_room_constructor_clears_its_lists_and_sets_the_flag() {
        let mut e = Engine::new();
        e.register(MULTI_BOUND_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x0000_0001);
            Ret::default()
        });
        e.register(LIGHT_ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xec);
        for offset in (0xb4u32..0xe8).step_by(4) {
            e.mem.set_u32(this + offset, 0xdddd_dddd);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_9640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), MULTI_BOUND_ROOM_VTABLE);
        for list in [PORTAL_LIST, OCCLUDER_LIST, JOINED_MULTI_BOUND_LIST] {
            for word in 0..3 {
                assert_eq!(e.mem.u32(this + list + 4 * word), 0);
            }
        }
        assert_eq!(e.mem.u32(this + COMPOUND_FRUSTUM), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0x801);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (MULTI_BOUND_NODE_CONSTRUCT, vec![this]),
                (LIGHT_ARRAY_CONSTRUCT, vec![this + LIGHTS]),
            ]
        );
    }
}
