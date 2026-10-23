//! `BSMain/bsmultiboundnode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c47610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c46af0` and frees 0xb4 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_multi_bound_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c4_6af0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xb4u32]);
    }
    this
}

// Translated from 00c47650 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e74 without reading the receiver. The map names this body `BSMultiBoundNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c47650(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e74
}

/// `BSNiNode::BSNiNode` (engine map, `00c5f570`, `thiscall`, no stack words).
const BS_NI_NODE_CONSTRUCT: u32 = 0x00c5_f570;
/// `BSMultiBoundNode`'s vtable (`010c1d14`, RTTI `.?AVBSMultiBoundNode@@`).
const BS_MULTI_BOUND_NODE_VTABLE: u32 = 0x010c_1d14;
/// The `NiPointer` assignment `0066b0d0` (`thiscall`, the new pointer).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00c46970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::BSMultiBoundNode` (Xbox PDB), `thiscall`, no stack words; answers
/// `this`. Runs `BSNiNode`'s constructor, sets the vtable, constructs the `NiPointer` at
/// `+0xac` as null (the inlined constructor's increment of a non-null pointer reads the word
/// just written, so it never runs), assigns null to it (`0066b0d0`) and sets flag 0x800 of
/// `NiAVObject::m_uFlags` (+0x30). The flag is set through a test of the constant 1; the
/// other arm (clearing the flag) never runs and is translated with it.
// Translation incomplete: 00c46970 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) is not translated.
pub fn bs_multi_bound_node_bs_multi_bound_node(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_NI_NODE_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BS_MULTI_BOUND_NODE_VTABLE);
    let pointer = base.wrapping_add(0xac);
    e.mem.set_u32(pointer, 0);
    let held = e.mem.u32(pointer);
    if held != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    }
    e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
    // `MOV ECX,1; TEST ECX,ECX`: the set arm always runs.
    let set_flag = std::hint::black_box(true);
    let flags = e.mem.u32(base.wrapping_add(0x30));
    if set_flag {
        e.mem.set_u32(base.wrapping_add(0x30), flags | 0x800);
    } else {
        e.mem.set_u32(base.wrapping_add(0x30), flags & !0x800);
    }
    this
}

/// `NiFrustumPlanes::NiFrustumPlanes` (engine map, `0045c620`; `thiscall`, no stack words)
/// and the size of the planes the code keeps on its stack (`EBP-0x70`).
const FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
const FRUSTUM_PLANES_SIZE: u32 = 0x6c;
/// `BSOcclusionPlane::GetFrustumPlanes` (engine map, `00c33870`; `thiscall` on the occluder,
/// four words: a point by value and the planes; `RET 0x10`).
const OCCLUSION_PLANE_GET_FRUSTUM_PLANES: u32 = 0x00c3_3870;

// Translated from 00c47150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::VisibleAgainstActiveOccluders` (engine map; `thiscall`, five words: a
/// point by value (x, y, z), the culler and an occluder; `RET 0x14`; the answer in `AL`).
/// Visible (true) when the culler's mode (`+0x90`) is 3 or 1, or without an occluder.
/// Otherwise the occluder's frustum planes for the point (`BSOcclusionPlane::
/// GetFrustumPlanes` into `NiFrustumPlanes` on the stack) go to the multi-bound's shape
/// (`spMultiBound` +0xC) vtable +0xA0: when that answers true the shape's state (+8)
/// becomes 2 and the answer is false (occluded), else true.
#[allow(clippy::too_many_arguments)]
pub fn bs_multi_bound_node_visible_against_active_occluders(
    e: &mut Engine,
    this: Ptr,
    x: u32,
    y: u32,
    z: u32,
    culler: Ptr,
    occluder: Ptr,
) -> bool {
    let mode = e.mem.u32(culler.addr() + 0x90);
    if mode == 3 || e.mem.u32(culler.addr() + 0x90) == 1 {
        return true;
    }
    if occluder.is_null() {
        return true;
    }
    e.with_stack(FRUSTUM_PLANES_SIZE, |e, planes| {
        e.call(FRUSTUM_PLANES_CONSTRUCT, &args![planes]);
        e.call(
            OCCLUSION_PLANE_GET_FRUSTUM_PLANES,
            &args![occluder, x, y, z, planes],
        );
        let shape = e.mem.u32(e.mem.u32(this.addr() + MULTI_BOUND) + 0xc);
        if e.vcall(shape, 0xa0, &args![planes]).bool() {
            let shape = e.mem.u32(e.mem.u32(this.addr() + MULTI_BOUND) + 0xc);
            e.mem.set_u32(shape + 8, 2);
            false
        } else {
            true
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_7150,
            bs_multi_bound_node_visible_against_active_occluders(Ptr, u32, u32, u32, Ptr, Ptr) -> bool
        ),
        entry!(0x00c4_6970, bs_multi_bound_node_bs_multi_bound_node(Ptr) -> Ptr),
        entry!(0x00c4_7650, fn_00c47650(Ptr) -> u32),
        entry!(0x00c4_7610, bs_multi_bound_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c4_6af0, fn_00c46af0(Ptr)),
        entry!(0x00c4_6df0, fn_00c46df0() -> Ptr),
        entry!(0x00c4_6ca0, bs_multi_bound_node_load_binary(Ptr, Ptr)),
        entry!(0x00c4_6cd0, bs_multi_bound_node_link_object(Ptr, Ptr)),
        entry!(0x00c4_6f60, bs_multi_bound_node_on_visible(Ptr, Ptr)),
        entry!(0x00c4_7250, bs_multi_bound_node_update_world_bound(Ptr)),
    ]
}

// ---- Batch b012 --------------------------------------------------------------------------

/// `BSMultiBoundNode` (Xbox PDB) on PC: `NiNode::m_kChildren` (an `NiTArray`) at `+0x9c`
/// (elements `+0xa0`, size `+0xa6`, non-null count `+0xa8`), `spMultiBound` at `+0xac`,
/// `CullingMode` at `+0xb0`; `NiAVObject::m_pWorldBound` at `+0x20` and `m_uFlags` at
/// `+0x30`.
const CHILDREN: u32 = 0xa0;
const CHILD_COUNT: u32 = 0xa6;
const CHILDREN_USED: u32 = 0xa8;
const MULTI_BOUND: u32 = 0xac;
const CULLING_MODE: u32 = 0xb0;
const WORLD_BOUND: u32 = 0x20;
/// `00c5faa0` (`thiscall`, unit `bsninode.obj`): the `BSNiNode` destructor body.
const BS_NI_NODE_DESTRUCTOR: u32 = 0x00c5_faa0;
/// `00aa13e0` (`cdecl`, one word): allocates.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiNode::LoadBinary` (`00a5dd40`) and `NiNode::LinkObject` (`00a5e9d0`), `RET 4`.
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NI_NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
/// `00a645b0` (`thiscall` on the stream, unit `nistream.cpp`) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
const STREAM_00A645B0: u32 = 0x00a6_45b0;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiAVObject::Cull` (`00a59e00`, `thiscall`, `RET 4`).
const NI_AV_OBJECT_CULL: u32 = 0x00a5_9e00;
/// `005467e0` (`thiscall`, no stack word; engine-map unit `tesobjectcell.cpp`): run on the
/// node before its world bound is set.
const NODE_005467E0: u32 = 0x0054_67e0;
/// `NiBound::Merge` (`00a7f3f0`, `thiscall`, `RET 4`).
const NI_BOUND_MERGE: u32 = 0x00a7_f3f0;
/// The bound copied for a child without a world bound (`011f4288`) and the `double` the
/// child radii are compared with (`01012060`).
const EMPTY_BOUND: u32 = 0x011f_4288;
const RADIUS_ZERO: u32 = 0x0101_2060;
/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c46af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the `BSMultiBoundNode` destructor body (`thiscall`): sets the
/// vtable, assigns null to `spMultiBound` (`0066b0d0`), then (the member's destructor)
/// releases what it still holds, and ends in the `BSNiNode` destructor body (`00c5faa0`).
// Translation incomplete: 00c46af0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) and the /GS cookie word are not translated.
pub fn fn_00c46af0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BS_MULTI_BOUND_NODE_VTABLE);
    let pointer = base.wrapping_add(MULTI_BOUND);
    e.call(NI_POINTER_ASSIGN, &args![pointer, 0u32]);
    let held = e.mem.u32(pointer);
    if held != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(held, 4, &[]);
    }
    e.call(BS_NI_NODE_DESTRUCTOR, &args![this]);
}

// Translated from 00c46df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra matched a library `CreateObject` signature): the
/// `BSMultiBoundNode` factory (`cdecl`, no argument): 0xb4 bytes from `00aa13e0`,
/// constructed by `00c46970`, or null when the allocation fails.
// Translation incomplete: 00c46df0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn fn_00c46df0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0xb4u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    bs_multi_bound_node_bs_multi_bound_node(e, Ptr::new(block))
}

// Translated from 00c46ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`): `NiNode::LoadBinary`,
/// then `00a645b0` on the stream (the multi-bound's link id).
pub fn bs_multi_bound_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_00A645B0, &args![stream]);
}

// Translated from 00c46cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::LinkObject` (Xbox PDB; `thiscall`, `RET 4`): `NiNode::LinkObject`,
/// then the stream's next linked object assigned to `spMultiBound` (`0066b0d0`).
pub fn bs_multi_bound_node_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_NODE_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.call(
        NI_POINTER_ASSIGN,
        &args![this.addr().wrapping_add(MULTI_BOUND), object],
    );
}

// Translated from 00c46f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::OnVisible` (Xbox PDB; `thiscall`, `RET 4`, the culler): nothing
/// without non-null children (`+0xa8`). With the culler's mode (`+0x90`) not 3 or 1 and a
/// multi-bound, the bound's shape (`+0xc`) state (`+8`) decides: in mode 4 it is first
/// replaced by `2 - (shape slot 0x9c (culler + 0x2c) != 0)`; state 0 asks the shape again
/// and stores `2 - (answer != 0)` as its state, the answer deciding; state 1 is visible,
/// any other state not. When visible, the culler's mode becomes the node's `CullingMode`
/// (unless it was 3), each non-null child is culled (`NiAVObject::Cull`), and the mode is
/// put back.
pub fn bs_multi_bound_node_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let (node, process) = (this.addr(), culler.addr());
    if e.mem.u16(node + CHILDREN_USED) == 0 {
        return;
    }
    let mode = e.mem.u32(process + 0x90);
    let mut visible = true;
    if mode != 3 && mode != 1 && e.mem.u32(node + MULTI_BOUND) != 0 {
        let shape = |e: &Engine| e.mem.u32(e.mem.u32(node + MULTI_BOUND) + 0xc);
        let mut state = e.mem.u32(shape(e) + 8);
        if e.mem.u32(process + 0x90) == 4 {
            let answer = e.vcall(shape(e), 0x9c, &args![process + 0x2c]).u8();
            state = if answer != 0 { 1 } else { 2 };
        }
        match state {
            0 => {
                let answer = e.vcall(shape(e), 0x9c, &args![process + 0x2c]).u8();
                visible = answer != 0;
                let at = shape(e) + 8;
                e.mem.set_u32(at, if answer != 0 { 1 } else { 2 });
            }
            1 => visible = true,
            _ => visible = false,
        }
    }
    if !visible {
        return;
    }
    if mode != 3 {
        let culling = e.mem.u32(node + CULLING_MODE);
        e.mem.set_u32(process + 0x90, culling);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + CHILD_COUNT)) {
        let child = e.mem.u32(e.mem.u32(node + CHILDREN) + 4 * index);
        if child != 0 {
            e.call(NI_AV_OBJECT_CULL, &args![child, culler]);
        }
        index += 1;
    }
    e.mem.set_u32(process + 0x90, mode);
}

/// Whether a child has a world bound whose radius (`FLD`/`FSTP` copy) is not the `double`
/// at `01012060` (`FCOMP; TEST AH,0x44; JNP`: unordered counts as different).
fn has_radius(e: &Engine, child: u32) -> bool {
    let bound = e.mem.u32(child + WORLD_BOUND);
    if bound == 0 {
        return false;
    }
    let radius = x87_load32(x87_quiet_bits(e.mem.u32(bound + 0xc)));
    radius != f64::from_bits(e.mem.u64(RADIUS_ZERO))
}

// Translated from 00c47250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundNode::UpdateWorldBound` (Xbox PDB; `thiscall`): with a multi-bound that has
/// a shape, `005467e0` runs on the node and the shape sets the world bound (its slot
/// `0xa4` with `m_pWorldBound`). Otherwise: the world bound's radius is cleared when the
/// node has one and flag `0x80000` is set; the first child with a non-zero radius gives
/// the world bound its centre and radius (after `005467e0` on the node), and every later
/// such child is merged into it (`NiBound::Merge`). A child without a world bound would use
/// the bound at `011f4288` (it never qualifies, as the exe tests it first).
pub fn bs_multi_bound_node_update_world_bound(e: &mut Engine, this: Ptr) {
    let node = this.addr();
    let multi_bound = e.mem.u32(node + MULTI_BOUND);
    if multi_bound != 0 {
        let shape = e.mem.u32(multi_bound + 0xc);
        if shape != 0 {
            e.call(NODE_005467E0, &args![this]);
            let world = e.mem.u32(node + WORLD_BOUND);
            e.vcall(shape, 0xa4, &args![world]);
            return;
        }
    }
    let world = e.mem.u32(node + WORLD_BOUND);
    if world != 0 && e.mem.u32(node + 0x30) & 0x8_0000 != 0 {
        e.mem.set_u32(world + 0xc, x87_store32(0.0));
    }
    let count = u32::from(e.mem.u16(node + CHILD_COUNT));
    let bound_of = |e: &Engine, child: u32| {
        let bound = e.mem.u32(child + WORLD_BOUND);
        if bound == 0 {
            EMPTY_BOUND
        } else {
            bound
        }
    };
    let mut index = 0u32;
    while index < count {
        let child = e.mem.u32(e.mem.u32(node + CHILDREN) + 4 * index);
        index += 1;
        if child != 0 && has_radius(e, child) {
            e.call(NODE_005467E0, &args![this]);
            let source = bound_of(e, child);
            let world = e.mem.u32(node + WORLD_BOUND);
            let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
            for word in 0..3 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(world + 4 * word, value);
            }
            e.mem.set_u32(world + 0xc, x87_quiet_bits(radius));
            break;
        }
    }
    while index < count {
        let child = e.mem.u32(e.mem.u32(node + CHILDREN) + 4 * index);
        if child != 0 && has_radius(e, child) {
            let source = bound_of(e, child);
            let world = e.mem.u32(node + WORLD_BOUND);
            e.call(NI_BOUND_MERGE, &args![world, source]);
        }
        index += 1;
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
    fn bs_multi_bound_node_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c4_7610, 0x00c4_6af0, 0xb4);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c47650_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_7650, &args![0x1000u32]).u32(), 0x0120_2e74);
        assert_eq!(e.call(0x00c4_7650, &args![0u32]).u32(), 0x0120_2e74);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtable_the_null_pointer_and_flag_0x800() {
        let mut e = Engine::new();
        e.register(BS_NI_NODE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x1);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this + 0xac, 0xdddd_dddd);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_6970, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00c4_6970, BS_NI_NODE_CONSTRUCT, NI_POINTER_ASSIGN]
        );
        let assign = log.iter().find(|c| c.0 == NI_POINTER_ASSIGN).unwrap();
        assert_eq!(assign.1, vec![this + 0xac, 0]);
        assert_eq!(e.mem.u32(this), BS_MULTI_BOUND_NODE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0x801);
    }
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    const SHAPE_VTABLE: u32 = 0x0300_0000;
    const SHAPE_VISIBLE: u32 = 0x0300_1000;
    const SHAPE_SET_BOUND: u32 = 0x0300_2000;
    const DELETE_THIS: u32 = 0x0300_3000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for address in [
            NI_POINTER_ASSIGN,
            BS_NI_NODE_DESTRUCTOR,
            BS_NI_NODE_CONSTRUCT,
            NI_NODE_LOAD_BINARY,
            NI_NODE_LINK_OBJECT,
            STREAM_00A645B0,
            NI_AV_OBJECT_CULL,
            NODE_005467E0,
            NI_BOUND_MERGE,
            SHAPE_SET_BOUND,
            DELETE_THIS,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let mut slots = vec![0u32; 0x2a];
        slots[1] = DELETE_THIS;
        slots[0x9c / 4] = SHAPE_VISIBLE;
        slots[0xa4 / 4] = SHAPE_SET_BOUND;
        e.put_vtable(SHAPE_VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    /// A node with `children` (null allowed) and, with `shape_state`, a multi-bound whose
    /// shape has that state.
    fn node(e: &mut Engine, children: &[u32], shape_state: Option<u32>) -> (u32, u32) {
        let this = e.mem.alloc(0xb4);
        let elements = e.mem.alloc(4 * (children.len() as u32).max(1));
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *child);
        }
        e.mem.set_u32(this + CHILDREN, elements);
        e.mem.set_u16(this + CHILD_COUNT, children.len() as u16);
        let used = children.iter().filter(|c| **c != 0).count() as u16;
        e.mem.set_u16(this + CHILDREN_USED, used);
        let mut shape = 0;
        if let Some(state) = shape_state {
            let multi_bound = e.mem.alloc(0x10);
            shape = e.mem.alloc(0x10);
            e.mem.set_u32(shape, SHAPE_VTABLE);
            e.mem.set_u32(shape + 8, state);
            e.mem.set_u32(multi_bound + 0xc, shape);
            e.mem.set_u32(this + MULTI_BOUND, multi_bound);
        }
        (this, shape)
    }

    fn child(e: &mut Engine, radius: Option<f32>) -> u32 {
        let at = e.mem.alloc(0x40);
        if let Some(radius) = radius {
            let bound = e.mem.alloc(0x10);
            e.mem.set_f32(bound, 1.0);
            e.mem.set_f32(bound + 4, 2.0);
            e.mem.set_f32(bound + 8, 3.0);
            e.mem.set_f32(bound + 0xc, radius);
            e.mem.set_u32(at + WORLD_BOUND, bound);
        }
        at
    }

    #[test]
    fn the_destructor_drops_the_multi_bound() {
        let mut e = engine();
        let this = e.mem.alloc(0xb4);
        let held = e.mem.alloc(0x10);
        e.mem.set_u32(held, SHAPE_VTABLE);
        e.mem.set_u32(held + 4, 1);
        e.mem.set_u32(this + MULTI_BOUND, held);
        e.call(0x00c4_6af0, &args![this]);
        assert_eq!(e.mem.u32(this), BS_MULTI_BOUND_NODE_VTABLE);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![this + MULTI_BOUND, 0]]
        );
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![held]]);
        assert_eq!(calls(&e, BS_NI_NODE_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn the_factory_constructs_or_answers_null() {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00c4_6df0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), BS_MULTI_BOUND_NODE_VTABLE);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c4_6df0, &args![]).u32(), 0);
    }

    #[test]
    fn loading_and_linking() {
        let mut e = engine();
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| 0x7777u32.into_ret());
        let this = e.mem.alloc(0xb4);
        e.call(0x00c4_6ca0, &args![this, 0x5000u32]);
        assert_eq!(calls(&e, NI_NODE_LOAD_BINARY), vec![vec![this, 0x5000]]);
        assert_eq!(calls(&e, STREAM_00A645B0), vec![vec![0x5000]]);
        e.call(0x00c4_6cd0, &args![this, 0x5000u32]);
        assert_eq!(calls(&e, NI_NODE_LINK_OBJECT), vec![vec![this, 0x5000]]);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN),
            vec![vec![this + MULTI_BOUND, 0x7777]]
        );
    }

    fn culler(e: &mut Engine, mode: u32) -> u32 {
        let at = e.mem.alloc(0xa0);
        e.mem.set_u32(at + 0x90, mode);
        at
    }

    #[test]
    fn visible_nodes_cull_their_children_in_their_own_mode() {
        let mut e = engine();
        let a = child(&mut e, None);
        let b = child(&mut e, None);
        let (this, _) = node(&mut e, &[a, 0, b], None);
        e.mem.set_u32(this + CULLING_MODE, 6);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(NI_AV_OBJECT_CULL, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1] + 0x90)));
            Ret::default()
        });
        let process = culler(&mut e, 0);
        e.call(0x00c4_6f60, &args![this, process]);
        assert_eq!(*seen.borrow(), vec![(a, 6), (b, 6)]);
        assert_eq!(e.mem.u32(process + 0x90), 0);
        // Mode 3 keeps the culler's mode.
        seen.borrow_mut().clear();
        let process = culler(&mut e, 3);
        e.call(0x00c4_6f60, &args![this, process]);
        assert_eq!(*seen.borrow(), vec![(a, 3), (b, 3)]);
        // No non-null children: nothing.
        let (empty, _) = node(&mut e, &[0], None);
        let process = culler(&mut e, 0);
        e.call(0x00c4_6f60, &args![empty, process]);
        assert_eq!(seen.borrow().len(), 2);
    }

    #[test]
    fn the_shape_state_decides_visibility() {
        let mut e = engine();
        let a = child(&mut e, None);
        // (state, mode, shape answer, culled, new state)
        for (state, mode, answer, culled, after) in [
            (1u32, 0u32, 0u8, true, 1u32),
            (2, 0, 1, false, 2),
            (0, 0, 1, true, 1),
            (0, 0, 0, false, 2),
            (2, 4, 1, true, 2),
            (1, 4, 0, false, 1),
            (2, 1, 0, true, 2),
        ] {
            let (this, shape) = node(&mut e, &[a], Some(state));
            e.register_double(SHAPE_VISIBLE, move |_, _| answer.into_ret());
            e.call_log = Some(vec![]);
            let process = culler(&mut e, mode);
            e.call(0x00c4_6f60, &args![this, process]);
            assert_eq!(
                !calls(&e, NI_AV_OBJECT_CULL).is_empty(),
                culled,
                "{state} {mode}"
            );
            assert_eq!(e.mem.u32(shape + 8), after, "{state} {mode}");
            for call in calls(&e, SHAPE_VISIBLE) {
                assert_eq!(call, vec![shape, process + 0x2c]);
            }
        }
    }

    #[test]
    fn a_shape_sets_the_world_bound() {
        let mut e = engine();
        let (this, shape) = node(&mut e, &[], Some(0));
        e.mem.set_u32(this + WORLD_BOUND, 0x4440);
        e.call(0x00c4_7250, &args![this]);
        assert_eq!(calls(&e, NODE_005467E0), vec![vec![this]]);
        assert_eq!(calls(&e, SHAPE_SET_BOUND), vec![vec![shape, 0x4440]]);
    }

    #[test]
    fn the_children_make_the_world_bound() {
        let mut e = engine();
        let flat = child(&mut e, Some(0.0));
        let none = child(&mut e, None);
        let first = child(&mut e, Some(5.0));
        let second = child(&mut e, Some(f32::NAN));
        let (this, _) = node(&mut e, &[0, flat, none, first, second], None);
        let world = e.mem.alloc(0x10);
        e.mem.set_f32(world + 0xc, 9.0);
        e.mem.set_u32(this + WORLD_BOUND, world);
        e.mem.set_u32(this + 0x30, 0x8_0000);
        e.call(0x00c4_7250, &args![this]);
        assert_eq!(calls(&e, NODE_005467E0), vec![vec![this]]);
        let values: Vec<f32> = (0..4).map(|i| e.mem.f32(world + 4 * i)).collect();
        assert_eq!(values, vec![1.0, 2.0, 3.0, 5.0]);
        let second_bound = e.mem.u32(second + WORLD_BOUND);
        assert_eq!(calls(&e, NI_BOUND_MERGE), vec![vec![world, second_bound]]);
        // No qualifying child: only the radius is cleared.
        let (lonely, _) = node(&mut e, &[flat], None);
        let world = e.mem.alloc(0x10);
        e.mem.set_f32(world + 0xc, 9.0);
        e.mem.set_u32(lonely + WORLD_BOUND, world);
        e.mem.set_u32(lonely + 0x30, 0x8_0000);
        e.call(0x00c4_7250, &args![lonely]);
        assert_eq!(e.mem.f32(world + 0xc), 0.0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    /// The shape's test slot (vtable +0xA0) as the test places it.
    const SHAPE_TEST: u32 = 0x0090_00a0;

    fn rig(occluded: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(FRUSTUM_PLANES_CONSTRUCT, |_, _| Ret::default());
        e.register(OCCLUSION_PLANE_GET_FRUSTUM_PLANES, |_, _| Ret::default());
        e.register_double(SHAPE_TEST, move |_, _| occluded.into_ret());
        let mut slots = [0u32; 0x29];
        slots[0x28] = SHAPE_TEST;
        e.put_vtable(0x0090_1000, &slots);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, 0x0090_1000);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, shape);
        let node = e.mem.alloc(0xb4);
        e.mem.set_u32(node + MULTI_BOUND, bound);
        let culler = e.mem.alloc(0x94);
        (e, node, culler, shape)
    }

    #[test]
    fn modes_3_and_1_and_no_occluder_are_visible() {
        let (mut e, node, culler, _) = rig(1);
        for mode in [3u32, 1] {
            e.mem.set_u32(culler + 0x90, mode);
            let visible = e
                .call(
                    0x00c4_7150,
                    &args![node, 1u32, 2u32, 3u32, culler, 0x7000u32],
                )
                .bool();
            assert!(visible);
        }
        e.mem.set_u32(culler + 0x90, 0);
        e.call_log = Some(vec![]);
        let visible = e
            .call(0x00c4_7150, &args![node, 1u32, 2u32, 3u32, culler, 0u32])
            .bool();
        assert!(visible);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn an_occluded_shape_is_marked_and_not_visible() {
        let (mut e, node, culler, shape) = rig(0x1234_5601);
        e.call_log = Some(vec![]);
        let visible = e
            .call(
                0x00c4_7150,
                &args![node, 1u32, 2u32, 3u32, culler, 0x7000u32],
            )
            .bool();
        assert!(!visible);
        assert_eq!(e.mem.u32(shape + 8), 2);
        let log = e.call_log.take().unwrap();
        let planes = log
            .iter()
            .find(|(a, _)| *a == FRUSTUM_PLANES_CONSTRUCT)
            .unwrap()
            .1[0];
        let get = log
            .iter()
            .find(|(a, _)| *a == OCCLUSION_PLANE_GET_FRUSTUM_PLANES)
            .unwrap();
        assert_eq!(get.1, vec![0x7000, 1, 2, 3, planes]);
        let test = log.iter().find(|(a, _)| *a == SHAPE_TEST).unwrap();
        assert_eq!(test.1, vec![shape, planes]);
    }

    #[test]
    fn an_unoccluded_shape_is_visible() {
        let (mut e, node, culler, shape) = rig(0x100);
        let visible = e
            .call(
                0x00c4_7150,
                &args![node, 1u32, 2u32, 3u32, culler, 0x7000u32],
            )
            .bool();
        assert!(visible);
        assert_eq!(e.mem.u32(shape + 8), 0);
    }
}
