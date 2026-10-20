//! `NiMain/nigeometry.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    assign_ref_counted, interlocked_decrement, interlocked_increment, EMPTY_FIXED_STRING,
};

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

/// The spin lock at `011f42a0` (owner word, then the entry count at `011f42a4`) the property
/// walks hold: entered through `0040fbf0` (`thiscall`, `RET 4`, here with a spin
/// limit of 0), left inline by a plain decrement of the count that clears the owner at zero.
const PROPERTY_LOCK: u32 = 0x011f_42a0;
const PROPERTY_LOCK_COUNT: u32 = 0x011f_42a4;
const SPIN_LOCK_ENTER_0040FBF0: u32 = 0x0040_fbf0;
/// The memory manager (`011f6238`) and its allocate `00aa3e40` (`thiscall`, one stack word).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The default bound `UpdateWorldBound` reads when +0x20 is null (`011f4288`, 16 bytes) and
/// the centre it stores into a new bound (`011f426c`, three words).
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const BOUND_CENTER_011F426C: u32 = 0x011f_426c;
/// Engine-map names: `NiAVObject::UpdatePropertiesDownward` (`00a5a140`, `RET 4`),
/// `NiAVObject::SetSelectiveUpdateFlags` (`00a5a450`, `RET 0xc`), `NiAVObject::LoadBinary`
/// (`00a5a510`, `RET 4`), `NiAVObject::LinkObject` (`00a5a7f0`, `RET 4`),
/// `NiAVObject::ProcessClone` (`00a59ec0`, `RET 4`), `NiPropertyState::~NiPropertyState`
/// (`00a89890`), `NiBound::Update` (`00a7fdd0`, `RET 8`), `NiStream::LoadFixedString`
/// (`00a64df0`, `RET 4`), `NiStream::LoadCStringAsFixedString` (`00a64ea0`, `RET 4`),
/// `NiStream::GetObjectFromLinkID` (`00a63430`), `NiAVObject::SetCollisionObject`
/// (`0062bc90`, `RET 4`) and `NiTPointerListBase<..NiPointer<NiProperty>>::AddHead`
/// (`00439fd0`, `RET 4`).
const AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD: u32 = 0x00a5_a140;
const AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS: u32 = 0x00a5_a450;
const AV_OBJECT_LOAD_BINARY: u32 = 0x00a5_a510;
const AV_OBJECT_LINK_OBJECT: u32 = 0x00a5_a7f0;
const AV_OBJECT_PROCESS_CLONE: u32 = 0x00a5_9ec0;
const PROPERTY_STATE_DESTRUCT: u32 = 0x00a8_9890;
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const AV_OBJECT_SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
const PROPERTY_LIST_ADD_HEAD: u32 = 0x0043_9fd0;
/// Unnamed callees: the `NiAVObject` destructor body `00a5a390`, the property-state copy
/// `00a89900` (`RET 4`), the `NiObjectNET` member copy `00a5c290` (`RET 8`), the object
/// clone `00a5d3f0` (`RET 4`), the skin instance's bound step `00a86530` (`RET 4`), the
/// bound steps `00a7fea0` (`RET 4`) and `00a92660` (`RET 4`, a byte answer), the stream's
/// link-id read `00a645b0`, the `cdecl` stream read `00a26df0` (stream, out), the clone
/// map's lookup `00a53c30` (`RET 8`), the `NiPointer` assignment `0066b0d0` (`RET 4`), the
/// allocator `00aa13e0` (`cdecl`), the constructor `00a8f440`, the getter `00a62660` (its
/// engine-map name is a folded one), the name setter `00a5b950` (`RET 4`),
/// `NiD3DShaderProgram::GetName` (engine map, `00cc0760`) and `00ba3760` (`RET 4`).
const AV_OBJECT_DESTRUCT_00A5A390: u32 = 0x00a5_a390;
const PROPERTY_STATE_COPY_00A89900: u32 = 0x00a8_9900;
const OBJECT_NET_COPY_MEMBERS_00A5C290: u32 = 0x00a5_c290;
const OBJECT_CLONE_00A5D3F0: u32 = 0x00a5_d3f0;
const SKIN_INSTANCE_BOUND_00A86530: u32 = 0x00a8_6530;
const BOUND_STEP_00A7FEA0: u32 = 0x00a7_fea0;
const BOUND_COMPARE_00A92660: u32 = 0x00a9_2660;
const STREAM_READ_LINK_ID_00A645B0: u32 = 0x00a6_45b0;
const STREAM_READ_00A26DF0: u32 = 0x00a2_6df0;
const CLONE_MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;
const POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const CONSTRUCT_00A8F440: u32 = 0x00a8_f440;
const GETTER_00A62660: u32 = 0x00a6_2660;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const SHADER_PROGRAM_GET_NAME: u32 = 0x00cc_0760;
const SETTER_00BA3760: u32 = 0x00ba_3760;
/// The `NiRTTI` record `ProcessClone` compares the shader's `GetRTTI` with.
const SHADER_RTTI_011F5D08: u32 = 0x011f_5d08;
/// `NiStream`: `m_uiNifFileVersion` (+0xd8) and `m_pkIstr` (+0x24c); `NiBinaryStream`: the
/// position (+4) and `m_pfnRead` (+8) (Xbox PDB names).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;

/// Enters the property lock (`0040fbf0` with 0).
fn enter_property_lock(e: &mut Engine) {
    e.call(SPIN_LOCK_ENTER_0040FBF0, &args![PROPERTY_LOCK, 0u32]);
}

/// Leaves the property lock: the count drops by one and the owner is cleared at zero.
fn leave_property_lock(e: &mut Engine) {
    let count = e.mem.u32(PROPERTY_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(PROPERTY_LOCK_COUNT, count);
    if count == 0 {
        e.mem.set_u32(PROPERTY_LOCK, 0);
    }
}

/// Releases a counted object (`InterlockedDecrement` on +4, its slot +4 at zero).
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One read through the stream's `m_pfnRead` into `buffer`, the element size held in
/// `size_slot`, the position advanced by the result.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size_slot: u32, size: u32) {
    let binary = e.mem.u32(stream + STREAM_INPUT);
    let function = e.mem.u32(binary.wrapping_add(8));
    e.mem.set_u32(size_slot, size);
    let moved = e
        .call(function, &args![binary, buffer, bytes, size_slot, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(binary.wrapping_add(4), position);
}

// Translated from 00a7fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::OnVisible` (Xbox PDB), `thiscall`, `RET 4`: hands `this` to the culler's
/// slot +0x4c (a tail jump with `this` in the argument slot).
pub fn ni_geometry_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    e.vcall(culler.addr(), 0x4c, &args![this]);
}

// Translated from 00a80540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometry.cpp`): the `NiGeometry` constructor without
/// data (`thiscall`, no stack words). Runs the `NiAVObject` constructor (`00a5aa20`), stores
/// the vtable `0109e04c`, constructs `m_PropertyState` at +0x9c (`00a89ab0`) and clears
/// `m_spModelData` (+0xb8), `m_spSkinInstance` (+0xbc) and `m_spShader` (+0xc0). Answers
/// `this`.
pub fn fn_00a80540(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    e.call(NI_PROPERTY_STATE_CONSTRUCT, &args![base + 0x9c]);
    e.mem.set_u32(base + 0xb8, 0);
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    this
}

// Translated from 00a80610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::RenderImmediate` (Xbox PDB), `thiscall`, `RET 4`: stores the address of
/// `m_PropertyState` (+0x9c) at +0xc of `renderer`, then calls slot +0xa8 of each object in
/// the chain that starts at `m_spControllers` (+0xc) and goes on through +0x30.
pub fn ni_geometry_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    e.mem.set_u32(renderer.addr() + 0xc, base + 0x9c);
    let mut controller = e.mem.u32(base + 0xc);
    while controller != 0 {
        e.vcall(controller, 0xa8, &args![]);
        controller = e.mem.u32(controller + 0x30);
    }
}

// Translated from 00a808f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetSelectiveUpdateFlags` (Xbox PDB), `thiscall`, `RET 0xc`: runs
/// `NiAVObject::SetSelectiveUpdateFlags` (`00a5a450`) with the same words; with a skin
/// instance (+0xbc) it then sets bit 4 of the flags (+0x30), the byte at `selective_update`
/// to 1, bit 2 of the flags and the byte at `rigid` to 0.
pub fn ni_geometry_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr,
    selective_update: Ptr,
    selective_update_transforms: u32,
    rigid: Ptr,
) {
    let base = this.addr();
    e.call(
        AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS,
        &args![this, selective_update, selective_update_transforms, rigid],
    );
    if e.mem.u32(base + 0xbc) != 0 {
        let flags = e.mem.u32(base + 0x30) | 4;
        e.mem.set_u32(base + 0x30, flags);
        e.mem.set_u8(selective_update.addr(), 1);
        let flags = e.mem.u32(base + 0x30) | 2;
        e.mem.set_u32(base + 0x30, flags);
        e.mem.set_u8(rigid.addr(), 0);
    }
}

// Translated from 00a5abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometry.cpp`; `thiscall`, `RET 0xc`: the copy, the
/// property list, the cloning process). Under the property lock, walks `list` from its tail
/// (+4) back through each node's +4; for each node's property (+8) whose type (slot +0x8c)
/// is below 7, clones it (`00a5d3f0`) and, when the clone's type is below 7 too, holds it in
/// the `list` argument's own slot (counted) while `AddHead` (`00439fd0`) puts it on the
/// copy's list (+0x24), then releases that hold.
pub fn fn_00a5abe0(e: &mut Engine, _unused_this: Ptr, copy: Ptr, list: Ptr, cloning: Ptr) {
    enter_property_lock(e);
    let mut node = e.mem.u32(list.addr() + 4);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node + 4);
        if property == 0 || e.vcall(property, 0x8c, &args![]).i32() >= 7 {
            continue;
        }
        let clone = e
            .call(OBJECT_CLONE_00A5D3F0, &args![property, cloning])
            .u32();
        if e.vcall(clone, 0x8c, &args![]).i32() >= 7 {
            continue;
        }
        interlocked_increment(e, clone.wrapping_add(4));
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), clone);
            e.call(
                PROPERTY_LIST_ADD_HEAD,
                &args![copy.addr() + 0x24, held.addr()],
            );
        });
        release(e, clone);
    }
    leave_property_lock(e);
}

// Translated from 00a5ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `NiGeometry::CopyMembers`, but the body touches only
/// `NiAVObject` fields (`thiscall`, `RET 8`: the copy, the cloning process): the
/// `NiObjectNET` copy `00a5c290`, the flags (+0x30) with bit 0x80000 set, the 13 words of
/// the local transform (+0x34), the properties (`00a5abe0` with the list at +0x24) when the
/// list's count (+0x2c) is not zero, and a clone (slot +0x48) of the collision object (+0x1c)
/// given to `SetCollisionObject` (`0062bc90`).
pub fn fn_00a5ac90(e: &mut Engine, this: Ptr, copy: Ptr, cloning: Ptr) {
    let base = this.addr();
    let dest = copy.addr();
    e.call(
        OBJECT_NET_COPY_MEMBERS_00A5C290,
        &args![this, copy, cloning],
    );
    let flags = e.mem.u32(base + 0x30) | 0x80000;
    e.mem.set_u32(dest + 0x30, flags);
    for index in 0..0xdu32 {
        let word = e.mem.u32(base + 0x34 + 4 * index);
        e.mem.set_u32(dest + 0x34 + 4 * index, word);
    }
    if e.mem.u32(base + 0x2c) != 0 {
        fn_00a5abe0(e, this, copy, Ptr::new(base + 0x24), cloning);
    }
    let collision = e.mem.u32(base + 0x1c);
    if collision != 0 {
        let clone = e.vcall(collision, 0x48, &args![cloning]).u32();
        e.call(AV_OBJECT_SET_COLLISION_OBJECT, &args![copy, clone]);
    }
}

// Translated from 00a80580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::~NiGeometry` (Xbox PDB; `thiscall`, no stack words): stores the vtable
/// `0109e04c`, releases `m_spModelData` (+0xb8) and clears it, then the member destructors:
/// `m_spSkinInstance` (+0xbc) and `m_spModelData` again (each released when set),
/// `m_PropertyState` (`00a89890` on +0x9c), and ends in the `NiAVObject` destructor body
/// `00a5a390`.
pub fn ni_geometry_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    let data = e.mem.u32(base + 0xb8);
    if data != 0 {
        release(e, data);
        e.mem.set_u32(base + 0xb8, 0);
    }
    for offset in [0xbcu32, 0xb8] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release(e, object);
        }
    }
    e.call(PROPERTY_STATE_DESTRUCT, &args![base + 0x9c]);
    e.call(AV_OBJECT_DESTRUCT_00A5A390, &args![this]);
}

// Translated from 00a80680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetModelData` (Xbox PDB), `thiscall`, `RET 4`: nothing for null data.
/// Otherwise, with a collision object (+0x1c): its slot +0x94, then its slot +0x90 with a
/// local of a `float` 0.0 and five zero bytes; then `data` becomes `m_spModelData` (+0xb8,
/// the old one released, the new one counted, nothing when it is already there).
pub fn ni_geometry_set_model_data(e: &mut Engine, this: Ptr, data: Ptr) {
    let base = this.addr();
    if data.addr() == 0 {
        return;
    }
    if e.mem.u32(base + 0x1c) != 0 {
        let collision = e.mem.u32(base + 0x1c);
        e.vcall(collision, 0x94, &args![]);
        let collision = e.mem.u32(base + 0x1c);
        e.with_stack(0xc, |e, local| {
            e.mem.set_u32(local.addr(), 0.0f32.to_bits());
            for offset in 4..9u32 {
                e.mem.set_u8(local.addr() + offset, 0);
            }
            e.vcall(collision, 0x90, &args![local.addr()]);
        });
    }
    assign_ref_counted(e, base + 0xb8, data.addr());
}

// Translated from 00a80710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::UpdateWorldBound` (Xbox PDB; `thiscall`, no stack words). Copies the world
/// bound (+0x20, or the default at `011f4288` when null) into a local; unless bit 19 of the
/// flags (+0x30) is set with bit 13 clear, goes on only when that radius (+0xc) is exactly
/// 0.0. Without a bound one is allocated (16 bytes) and set through slot +0x78, with the
/// centre at `011f426c` and radius 0.0. Then with a skin instance (+0xbc) `00a86530` and
/// `00a7fea0` with the world transform (+0x68), else `NiBound::Update` from the model
/// data's bound (+0x10) and the world transform. When bit 22 of the flags is set, it stays
/// set only if `00a92660` answers true for the old bound.
pub fn ni_geometry_update_world_bound(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let bound = e.mem.u32(base + 0x20);
    let source = if bound == 0 {
        DEFAULT_BOUND_011F4288
    } else {
        bound
    };
    e.with_stack(0x10, |e, old| {
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(source + offset);
            e.mem.set_u32(old.addr() + offset, word);
        }
        let flags = e.mem.u32(base + 0x30);
        let forced = (flags >> 19) & 1 != 0 && (flags >> 13) & 1 == 0;
        if !forced {
            let radius = e.mem.f32(old.addr() + 0xc);
            // FLDZ; FCOMP radius; TEST AH,0x44; JP: on only for an ordered equal (a NaN
            // radius returns).
            if radius != 0.0 {
                return;
            }
        }
        if bound == 0 {
            let block = e
                .call(
                    MEMORY_MANAGER_ALLOCATE,
                    &args![MEMORY_MANAGER_OBJECT, 0x10u32],
                )
                .u32();
            e.vcall(base, 0x78, &args![block]);
            let bound = e.mem.u32(base + 0x20);
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(BOUND_CENTER_011F426C + offset);
                e.mem.set_u32(bound + offset, word);
            }
            e.mem.set_u32(bound + 0xc, 0.0f32.to_bits());
        }
        let skin = e.mem.u32(base + 0xbc);
        if skin != 0 {
            let bound = e.mem.u32(base + 0x20);
            e.call(SKIN_INSTANCE_BOUND_00A86530, &args![skin, bound]);
            let bound = e.mem.u32(base + 0x20);
            e.call(BOUND_STEP_00A7FEA0, &args![bound, base + 0x68]);
        } else {
            let data = e.mem.u32(base + 0xb8);
            let bound = e.mem.u32(base + 0x20);
            e.call(
                BOUND_UPDATE,
                &args![bound, data.wrapping_add(0x10), base + 0x68],
            );
        }
        let flags = e.mem.u32(base + 0x30);
        if (flags >> 22) & 1 != 0 {
            let bound = e.mem.u32(base + 0x20);
            if e.call(BOUND_COMPARE_00A92660, &args![bound, old.addr()])
                .bool()
            {
                e.mem.set_u32(base + 0x30, flags | 0x40_0000);
            } else {
                e.mem.set_u32(base + 0x30, flags & 0xffbf_ffff);
            }
        }
    });
}

// Translated from 00a80820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::UpdatePropertiesDownward` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiAVObject::UpdatePropertiesDownward` (`00a5a140`), then under the property lock the
/// state is copied into `m_PropertyState` (`00a89900` on +0x9c) and each property of the
/// list (head at +0x24, next at +0 of each node, the property at +8) whose type (slot +0x8c)
/// is at most 7 is put in the state's slot for that type (+0x9c + 4 * type, counted); then
/// slot +0x94 (`SetMaterialNeedsUpdate`) with 1.
pub fn ni_geometry_update_properties_downward(e: &mut Engine, this: Ptr, state: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD, &args![this, state]);
    enter_property_lock(e);
    e.call(PROPERTY_STATE_COPY_00A89900, &args![base + 0x9c, state]);
    let mut node = e.mem.u32(base + 0x24);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property == 0 || e.vcall(property, 0x8c, &args![]).i32() > 7 {
            continue;
        }
        let kind = e.vcall(property, 0x8c, &args![]).u32();
        let slot = base.wrapping_add(kind.wrapping_mul(4)).wrapping_add(0x9c);
        assign_ref_counted(e, slot, property);
    }
    leave_property_lock(e);
    e.vcall(base, 0x94, &args![1u32]);
}

// Translated from 00a80a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::ProcessClone` (Xbox PDB), `thiscall`, `RET 4`. After
/// `NiAVObject::ProcessClone` the clone of `this` is looked up in the cloning process's map
/// (`00a53c30`, the result in the argument's own slot). A skin instance (+0xbc) already
/// cloned is given to the clone's +0xbc (`0066b0d0`); otherwise its clone (slot +0x48) is,
/// and its slot +0x68 (`ProcessClone`) runs. A shader (+0xc0) whose `GetRTTI` (slot +8) is
/// `011f5d08` gets a new 0x14-byte object (`00aa13e0`, `00a8f440`) named like it
/// (`00a62660`, `00a5b950`) with its program name (`00cc0760`, `00ba3760`) as the clone's
/// shader; any other shader is shared: stored in the clone, given the clone through its slot
/// +0x50, and under the property lock the clone's slot +0xb0 gets `m_PropertyState` (+0x9c).
pub fn ni_geometry_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP_00A53C30, &args![map, base, slot.addr()]);
        let clone = e.mem.u32(slot.addr());
        let skin = e.mem.u32(base + 0xbc);
        if skin != 0 {
            let map = e.mem.u32(cloning.addr());
            if e.call(CLONE_MAP_LOOKUP_00A53C30, &args![map, skin, slot.addr()])
                .bool()
            {
                let found = e.mem.u32(slot.addr());
                e.call(POINTER_ASSIGN_0066B0D0, &args![clone + 0xbc, found]);
            } else {
                let skin = e.mem.u32(base + 0xbc);
                let copy = e.vcall(skin, 0x48, &args![cloning]).u32();
                e.call(POINTER_ASSIGN_0066B0D0, &args![clone + 0xbc, copy]);
                let skin = e.mem.u32(base + 0xbc);
                e.vcall(skin, 0x68, &args![cloning]);
            }
        }
        let shader = e.mem.u32(base + 0xc0);
        if shader == 0 {
            return;
        }
        if e.vcall(shader, 8, &args![]).u32() == SHADER_RTTI_011F5D08 {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x14u32]).u32();
            let made = if block != 0 {
                e.call(CONSTRUCT_00A8F440, &args![block]).u32()
            } else {
                0
            };
            let shader = e.mem.u32(base + 0xc0);
            let name = e.call(GETTER_00A62660, &args![shader]).u32();
            e.call(SET_NAME_00A5B950, &args![made, name]);
            let shader = e.mem.u32(base + 0xc0);
            let program = e.call(SHADER_PROGRAM_GET_NAME, &args![shader]).u32();
            e.call(SETTER_00BA3760, &args![made, program]);
            e.mem.set_u32(clone + 0xc0, made);
        } else {
            let shader = e.mem.u32(base + 0xc0);
            e.mem.set_u32(clone + 0xc0, shader);
            let shader = e.mem.u32(base + 0xc0);
            e.vcall(shader, 0x50, &args![clone]);
            enter_property_lock(e);
            e.vcall(clone, 0xb0, &args![base + 0x9c]);
            leave_property_lock(e);
        }
    });
}

// Translated from 00a80b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::LinkObject` (Xbox PDB), `thiscall`, `RET 4`: `NiAVObject::LinkObject`, then
/// the objects of the next two link ids (`GetObjectFromLinkID`) become `m_spModelData`
/// (+0xb8) and `m_spSkinInstance` (+0xbc), each counted.
pub fn ni_geometry_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_LINK_OBJECT, &args![this, stream]);
    for offset in [0xb8u32, 0xbc] {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        assign_ref_counted(e, base + offset, object);
    }
}

/// Releases a fixed-string handle held in a local unless it is the empty string's.
fn release_local_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        interlocked_decrement(e, handle.wrapping_sub(8));
    }
}

// Translated from 00a80bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`: `NiAVObject::LoadBinary`, two
/// link ids (`00a645b0`), then what the file has and the game drops: from 20.2.0.5 a count
/// (into the stream argument's slot, zeroed first) of fixed strings (`LoadFixedString` into
/// a local starting as the empty handle) each followed by a word, and one more word; from
/// 5.0.0.21 to 20.2.0.4 a byte that, when set, is followed by a string (`LoadFixedString`
/// from 20.1.0.1, `LoadCStringAsFixedString` before) and a value read by `00a26df0`. The
/// strings read are released. From 20.2.0.7 one more byte.
pub fn ni_geometry_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let stream = stream.addr();
    e.call(AV_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_LINK_ID_00A645B0, &args![stream]);
    e.call(STREAM_READ_LINK_ID_00A645B0, &args![stream]);
    // Locals: +0 the string handle, +4 the element size, +8 and +0xc words read and
    // dropped, +0x10 the stream argument's own slot.
    e.with_stack(0x14, |e, locals| {
        let l = locals.addr();
        let argument = l + 0x10;
        e.mem.set_u32(argument, stream);
        let version = e.mem.u32(stream + STREAM_VERSION);
        if version >= 0x1402_0005 {
            e.mem.set_u32(argument, 0);
            stream_read(e, stream, argument, 4, l, 4);
            let mut index = 0u32;
            while index < e.mem.u32(argument) {
                let empty = e.mem.u32(EMPTY_FIXED_STRING);
                e.mem.set_u32(l, empty);
                e.call(STREAM_LOAD_FIXED_STRING, &args![stream, l]);
                stream_read(e, stream, l + 8, 4, l + 4, 4);
                let handle = e.mem.u32(l);
                release_local_string(e, handle);
                index += 1;
            }
            stream_read(e, stream, l + 0xc, 4, l + 4, 4);
        } else if version >= 0x0500_0015 {
            e.mem.set_u8(argument, 0);
            stream_read(e, stream, argument, 1, l + 4, 1);
            if e.mem.u8(argument) != 0 {
                let empty = e.mem.u32(EMPTY_FIXED_STRING);
                e.mem.set_u32(l, empty);
                if e.mem.u32(stream + STREAM_VERSION) < 0x1401_0001 {
                    e.call(STREAM_LOAD_C_STRING_AS_FIXED_STRING, &args![stream, l]);
                } else {
                    e.call(STREAM_LOAD_FIXED_STRING, &args![stream, l]);
                }
                e.call(STREAM_READ_00A26DF0, &args![stream, l + 0xc]);
                let handle = e.mem.u32(l);
                release_local_string(e, handle);
            }
        }
        if e.mem.u32(stream + STREAM_VERSION) >= 0x1402_0007 {
            e.mem.set_u8(argument, 0);
            stream_read(e, stream, argument, 1, l + 4, 1);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_fd90, ni_geometry_on_visible(Ptr, Ptr)),
        entry!(0x00a8_0540, fn_00a80540(Ptr) -> Ptr),
        entry!(0x00a8_0610, ni_geometry_render_immediate(Ptr, Ptr)),
        entry!(
            0x00a8_08f0,
            ni_geometry_set_selective_update_flags(Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00a5_abe0, fn_00a5abe0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a5_ac90, fn_00a5ac90(Ptr, Ptr, Ptr)),
        entry!(0x00a8_0580, ni_geometry_destructor(Ptr)),
        entry!(0x00a8_0680, ni_geometry_set_model_data(Ptr, Ptr)),
        entry!(0x00a8_0710, ni_geometry_update_world_bound(Ptr)),
        entry!(
            0x00a8_0820,
            ni_geometry_update_properties_downward(Ptr, Ptr)
        ),
        entry!(0x00a8_0a10, ni_geometry_process_clone(Ptr, Ptr)),
        entry!(0x00a8_0b50, ni_geometry_link_object(Ptr, Ptr)),
        entry!(0x00a8_0bf0, ni_geometry_load_binary(Ptr, Ptr)),
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

#[cfg(test)]
mod batch_b011_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One vtable for every object of these tests: each slot is a double at
    /// `SLOT_DOUBLES + slot` answering the word stored at `ANSWERS + slot` (0 by default).
    const VTABLE: u32 = 0x0320_0000;
    const SLOT_DOUBLES: u32 = 0x0321_0000;
    const ANSWERS: u32 = 0x0322_0000;
    const READ_DOUBLE: u32 = 0x0323_0000;

    type Log = Rc<RefCell<Vec<(u32, u32)>>>;

    fn slot(offset: u32) -> u32 {
        SLOT_DOUBLES + offset
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(ANSWERS, 0x1000);
        let slots: Vec<u32> = (0..0x40u32).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x40u32 {
            e.register(slot(4 * i), |e, _| {
                // The double's own address is not passed; every slot answers the same word,
                // so tests that need different answers register their own double.
                e.mem.u32(ANSWERS).into_ret()
            });
        }
        for double in [
            NI_AV_OBJECT_CONSTRUCT,
            NI_PROPERTY_STATE_CONSTRUCT,
            SPIN_LOCK_ENTER_0040FBF0,
            AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD,
            AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS,
            AV_OBJECT_LOAD_BINARY,
            AV_OBJECT_LINK_OBJECT,
            AV_OBJECT_PROCESS_CLONE,
            PROPERTY_STATE_DESTRUCT,
            BOUND_UPDATE,
            STREAM_LOAD_FIXED_STRING,
            STREAM_LOAD_C_STRING_AS_FIXED_STRING,
            AV_OBJECT_SET_COLLISION_OBJECT,
            PROPERTY_LIST_ADD_HEAD,
            AV_OBJECT_DESTRUCT_00A5A390,
            PROPERTY_STATE_COPY_00A89900,
            OBJECT_NET_COPY_MEMBERS_00A5C290,
            SKIN_INSTANCE_BOUND_00A86530,
            BOUND_STEP_00A7FEA0,
            STREAM_READ_LINK_ID_00A645B0,
            STREAM_READ_00A26DF0,
            POINTER_ASSIGN_0066B0D0,
            SET_NAME_00A5B950,
            SETTER_00BA3760,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_00A8F440, |_, a| a[0].into_ret());
        e.register(GETTER_00A62660, |_, _| 0x6666u32.into_ret());
        e.register(SHADER_PROGRAM_GET_NAME, |_, _| 0x7777u32.into_ret());
        e.register(BOUND_COMPARE_00A92660, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32, count: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn on_visible_hands_this_to_the_culler() {
        let mut e = engine();
        let culler = object(&mut e, 8, 0);
        e.call(0x00a7_fd90, &args![0x1234u32, culler]);
        assert_eq!(calls_to(&e, slot(0x4c)), vec![vec![culler, 0x1234]]);
    }

    #[test]
    fn default_constructor_clears_the_pointers() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.mem.write(this + 0xb8, &[0xcc; 0xc]);
        assert_eq!(e.call(0x00a8_0540, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(
            calls_to(&e, NI_PROPERTY_STATE_CONSTRUCT),
            vec![vec![this + 0x9c]]
        );
        for offset in [0xb8u32, 0xbc, 0xc0] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn render_immediate_sets_the_state_and_walks_the_controllers() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let renderer = e.mem.alloc(0x10);
        let first = object(&mut e, 0x40, 0);
        let second = object(&mut e, 0x40, 0);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(first + 0x30, second);
        e.call(0x00a8_0610, &args![this, renderer]);
        assert_eq!(e.mem.u32(renderer + 0xc), this + 0x9c);
        assert_eq!(calls_to(&e, slot(0xa8)), vec![vec![first], vec![second]]);
    }

    #[test]
    fn selective_update_flags_mark_skinned_geometry() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let selective = e.mem.alloc(4);
        let rigid = e.mem.alloc(4);
        e.mem.set_u8(rigid, 1);
        e.call(0x00a8_08f0, &args![this, selective, 1u32, rigid]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS),
            vec![vec![this, selective, 1, rigid]]
        );
        assert_eq!((e.mem.u8(selective), e.mem.u8(rigid)), (0, 1));
        e.mem.set_u32(this + 0xbc, 9);
        e.mem.set_u32(this + 0x30, 0x100);
        e.call(0x00a8_08f0, &args![this, selective, 1u32, rigid]);
        assert_eq!((e.mem.u8(selective), e.mem.u8(rigid)), (1, 0));
        assert_eq!(e.mem.u32(this + 0x30), 0x106);
    }

    #[test]
    fn copy_properties_clones_from_the_tail() {
        let mut e = engine();
        let property = object(&mut e, 0x20, 1);
        let ignored = object(&mut e, 0x20, 1);
        let clone = object(&mut e, 0x20, 0);
        // Node layout: +0 next, +4 previous, +8 property; the list's tail at +4.
        let tail = e.mem.alloc(0xc);
        let before = e.mem.alloc(0xc);
        e.mem.set_u32(tail + 4, before);
        e.mem.set_u32(tail + 8, property);
        e.mem.set_u32(before + 8, ignored);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list + 4, tail);
        // Types: the property 3, the ignored one 7, the clone 3.
        e.register_double(slot(0x8c), move |_, a| {
            (if a[0] == ignored { 7u32 } else { 3 }).into_ret()
        });
        e.register_double(OBJECT_CLONE_00A5D3F0, move |_, _| clone.into_ret());
        let held: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = held.clone();
        e.register_double(PROPERTY_LIST_ADD_HEAD, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            seen.borrow_mut().push(e.mem.u32(clone + 4));
            Ret::default()
        });
        let copy = e.mem.alloc(0xd0);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.mem.set_u32(PROPERTY_LOCK, 0x55);
        e.call(0x00a5_abe0, &args![0x9999u32, copy, list, 0x4444u32]);
        assert_eq!(
            calls_to(&e, OBJECT_CLONE_00A5D3F0),
            vec![vec![property, 0x4444]]
        );
        // Held (counted once) while added, released after: count back to 0, deleted.
        assert_eq!(*held.borrow(), vec![clone, 1]);
        assert_eq!(e.mem.u32(clone + 4), 0);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![clone]]);
        assert_eq!(calls_to(&e, PROPERTY_LIST_ADD_HEAD)[0][0], copy + 0x24);
        assert_eq!(
            calls_to(&e, SPIN_LOCK_ENTER_0040FBF0),
            vec![vec![PROPERTY_LOCK, 0]]
        );
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(PROPERTY_LOCK), 0);
    }

    #[test]
    fn copy_members_copies_flags_transform_and_collision() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let copy = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0x30, 0x11);
        for index in 0..0xdu32 {
            e.mem.set_u32(this + 0x34 + 4 * index, index + 1);
        }
        let collision = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0x1c, collision);
        e.mem.set_u32(ANSWERS, 0xc0ffee);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a5_ac90, &args![this, copy, 0x4444u32]);
        assert_eq!(e.mem.u32(copy + 0x30), 0x80011);
        for index in 0..0xdu32 {
            assert_eq!(e.mem.u32(copy + 0x34 + 4 * index), index + 1);
        }
        assert_eq!(e.mem.u32(copy + 0x68), 0);
        // An empty property list (count +0x2c zero): no property copy.
        assert!(calls_to(&e, SPIN_LOCK_ENTER_0040FBF0).is_empty());
        assert_eq!(calls_to(&e, slot(0x48)), vec![vec![collision, 0x4444]]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![copy, 0xc0ffee]]
        );
        // With a property count the list walk runs (an empty list here).
        e.mem.set_u32(this + 0x2c, 1);
        e.call(0x00a5_ac90, &args![this, copy, 0x4444u32]);
        assert_eq!(calls_to(&e, SPIN_LOCK_ENTER_0040FBF0).len(), 1);
    }

    #[test]
    fn destructor_releases_data_and_skin() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let data = object(&mut e, 0x40, 1);
        let skin = object(&mut e, 0x40, 2);
        e.mem.set_u32(this + 0xb8, data);
        e.mem.set_u32(this + 0xbc, skin);
        e.call(0x00a8_0580, &args![this]);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.u32(data + 4), 0);
        assert_eq!(e.mem.u32(skin + 4), 1);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![data]]);
        assert_eq!(
            calls_to(&e, PROPERTY_STATE_DESTRUCT),
            vec![vec![this + 0x9c]]
        );
        assert_eq!(calls_to(&e, AV_OBJECT_DESTRUCT_00A5A390), vec![vec![this]]);
    }

    #[test]
    fn set_model_data_tells_the_collision_object() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let data = object(&mut e, 0x40, 0);
        // Null data: nothing.
        e.call(0x00a8_0680, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let collision = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0x1c, collision);
        let seen: Rc<RefCell<Vec<u8>>> = Rc::default();
        let local = seen.clone();
        e.register_double(slot(0x90), move |e, a| {
            local.borrow_mut().extend(e.mem.bytes(a[1], 9));
            Ret::default()
        });
        e.call(0x00a8_0680, &args![this, data]);
        assert_eq!(calls_to(&e, slot(0x94)), vec![vec![collision]]);
        assert_eq!(*seen.borrow(), vec![0; 9]);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_world_bound_runs_only_for_a_zero_radius_or_forced() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 2.0);
        e.mem.set_u32(this + 0x20, bound);
        e.call(0x00a8_0710, &args![this]);
        assert!(calls_to(&e, BOUND_UPDATE).is_empty());
        // A NaN radius stops too.
        e.mem.set_u32(bound + 0xc, 0x7fc0_0000);
        e.call(0x00a8_0710, &args![this]);
        assert!(calls_to(&e, BOUND_UPDATE).is_empty());
        // Bit 19 set, bit 13 clear: forced.
        e.mem.set_u32(this + 0x30, 1 << 19);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xb8, data);
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(
            calls_to(&e, BOUND_UPDATE),
            vec![vec![bound, data + 0x10, this + 0x68]]
        );
        // Bit 13 too: back to the radius test.
        e.mem.set_u32(this + 0x30, (1 << 19) | (1 << 13));
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(calls_to(&e, BOUND_UPDATE).len(), 1);
    }

    #[test]
    fn update_world_bound_makes_a_bound_and_tracks_changes() {
        let mut e = engine();
        e.mem.set_u32(BOUND_CENTER_011F426C, 1);
        e.mem.set_u32(BOUND_CENTER_011F426C + 4, 2);
        e.mem.set_u32(BOUND_CENTER_011F426C + 8, 3);
        let this = object(&mut e, 0xd0, 0);
        e.register(slot(0x78), |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.mem.set_u32(this + 0xbc, 0x5150);
        e.mem.set_u32(this + 0x30, 1 << 22);
        e.register(BOUND_COMPARE_00A92660, |_, _| 0x100u32.into_ret());
        e.call(0x00a8_0710, &args![this]);
        let bound = e.mem.u32(this + 0x20);
        assert_ne!(bound, 0);
        assert_eq!(
            e.mem.bytes(bound, 12),
            vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
        );
        assert_eq!(e.mem.u32(bound + 0xc), 0);
        assert_eq!(
            calls_to(&e, SKIN_INSTANCE_BOUND_00A86530),
            vec![vec![0x5150, bound]]
        );
        assert_eq!(
            calls_to(&e, BOUND_STEP_00A7FEA0),
            vec![vec![bound, this + 0x68]]
        );
        // The compare answered false: bit 22 cleared.
        assert_eq!(e.mem.u32(this + 0x30), 0);
        e.mem.set_u32(this + 0x30, 1 << 22);
        e.register(BOUND_COMPARE_00A92660, |_, _| 1u32.into_ret());
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(e.mem.u32(this + 0x30), 1 << 22);
    }

    #[test]
    fn update_properties_downward_fills_the_state() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let alpha = object(&mut e, 0x20, 0);
        let other = object(&mut e, 0x20, 0);
        let old = object(&mut e, 0x20, 1);
        e.register_double(slot(0x8c), move |_, a| {
            (if a[0] == other { 8u32 } else { 2 }).into_ret()
        });
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, alpha);
        e.mem.set_u32(second + 8, other);
        e.mem.set_u32(this + 0x24, first);
        e.mem.set_u32(this + 0x9c + 8, old);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a8_0820, &args![this, 0x3000u32]);
        assert_eq!(e.mem.u32(this + 0x9c + 8), alpha);
        assert_eq!(e.mem.u32(alpha + 4), 1);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![old]]);
        assert_eq!(
            calls_to(&e, PROPERTY_STATE_COPY_00A89900),
            vec![vec![this + 0x9c, 0x3000]]
        );
        assert_eq!(calls_to(&e, slot(0x94)), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);
    }

    #[test]
    fn process_clone_shares_or_rebuilds_the_shader() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let clone = object(&mut e, 0xd0, 0);
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x8888);
        e.register_double(CLONE_MAP_LOOKUP_00A53C30, move |e, a| {
            if a[1] == this {
                e.mem.set_u32(a[2], clone);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let skin = object(&mut e, 0x40, 0);
        e.mem.set_u32(this + 0xbc, skin);
        let shader = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0xc0, shader);
        e.mem.set_u32(ANSWERS, 0x1357);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a8_0a10, &args![this, cloning]);
        // The skin was not in the map: its clone (slot +0x48) is given and processed.
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_0066B0D0),
            vec![vec![clone + 0xbc, 0x1357]]
        );
        assert_eq!(calls_to(&e, slot(0x68)), vec![vec![skin, cloning]]);
        // A shared shader.
        assert_eq!(e.mem.u32(clone + 0xc0), shader);
        assert_eq!(calls_to(&e, slot(0x50)), vec![vec![shader, clone]]);
        assert_eq!(calls_to(&e, slot(0xb0)), vec![vec![clone, this + 0x9c]]);
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);

        // The shader of RTTI 011f5d08 is rebuilt.
        e.mem.set_u32(ANSWERS, SHADER_RTTI_011F5D08);
        e.mem.set_u32(this + 0xbc, 0);
        e.call(0x00a8_0a10, &args![this, cloning]);
        let made = e.mem.u32(clone + 0xc0);
        assert_ne!(made, shader);
        assert_eq!(calls_to(&e, SET_NAME_00A5B950), vec![vec![made, 0x6666]]);
        assert_eq!(calls_to(&e, SETTER_00BA3760), vec![vec![made, 0x7777]]);
    }

    #[test]
    fn link_object_takes_data_and_skin() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let data = object(&mut e, 0x40, 0);
        let mut next = vec![data, 0];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.remove(0).into_ret()
        });
        e.call(0x00a8_0b50, &args![this, 0x500u32]);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    /// A stream whose reads answer the given chunks in order and log (bytes, size).
    fn make_stream(e: &mut Engine, version: u32, chunks: Vec<Vec<u8>>) -> (u32, Log) {
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        let log: Log = Rc::default();
        let reads = log.clone();
        let mut chunks = chunks;
        e.register_double(READ_DOUBLE, move |e, a| {
            reads.borrow_mut().push((a[2], e.mem.u32(a[3])));
            let chunk = chunks.remove(0);
            e.mem.write(a[1], &chunk);
            a[2].into_ret()
        });
        (stream, log)
    }

    #[test]
    fn load_binary_drops_the_shader_strings_of_new_files() {
        let mut e = engine();
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 5);
        e.register_double(STREAM_LOAD_FIXED_STRING, move |e, a| {
            e.mem.set_u32(a[1], handle);
            Ret::default()
        });
        let chunks = vec![
            vec![2, 0, 0, 0],
            vec![0; 4],
            vec![0; 4],
            vec![0; 4],
            vec![1],
        ];
        let (stream, log) = make_stream(&mut e, 0x1402_0007, chunks);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(*log.borrow(), vec![(4, 4), (4, 4), (4, 4), (4, 4), (1, 1)]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING).len(), 2);
        assert_eq!(e.mem.u32(handle - 8), 3);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID_00A645B0).len(), 2);
        assert_eq!(
            calls_to(&e, AV_OBJECT_LOAD_BINARY),
            vec![vec![0x7000, stream]]
        );
    }

    #[test]
    fn load_binary_of_older_files_reads_an_optional_string() {
        let mut e = engine();
        // 5.0.0.21 to 20.1.0.0: a set byte, then a C string and a value.
        let (stream, log) = make_stream(&mut e, 0x1401_0000, vec![vec![1]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(*log.borrow(), vec![(1, 1)]);
        assert_eq!(calls_to(&e, STREAM_LOAD_C_STRING_AS_FIXED_STRING).len(), 1);
        assert_eq!(calls_to(&e, STREAM_READ_00A26DF0)[0][0], stream);
        // 20.1.0.1: the fixed-string form.
        let (stream, _) = make_stream(&mut e, 0x1401_0001, vec![vec![1]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING).len(), 1);
        // A clear byte: nothing more; before 5.0.0.21 nothing at all.
        let (stream, _) = make_stream(&mut e, 0x1401_0001, vec![vec![0]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(calls_to(&e, STREAM_READ_00A26DF0).len(), 2);
        let (stream, log) = make_stream(&mut e, 0x0500_0014, vec![]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert!(log.borrow().is_empty());
    }
}
