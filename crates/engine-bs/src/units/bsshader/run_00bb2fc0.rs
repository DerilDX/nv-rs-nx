//! `BSShader/run_00bb2fc0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, one word): the object allocation; `00aa1460`
/// (`cdecl`, `(block, size)`): the sized free.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// The memory manager (`011f6238`) and its `Allocate` (`00aa3e40`,
/// `thiscall`, one word) and `Deallocate` (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `memcpy` (`00ec44d0`) and `memset` (`00ec61c0`), `cdecl`.
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;
/// `NiRefObject`'s vtable (RTTI `.?AVNiRefObject@@`) and its live-object
/// count `NiRefObject::ms_uiObjects` (`011f4400`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// The thunks of the d3dx9 imports `D3DXMatrixTranslation` (`00ee6e04`,
/// `stdcall`, `(out, x, y, z)`) and `D3DXMatrixTranspose` (`00ee6dd4`,
/// `stdcall`, `(out, in)`).
const D3DX_MATRIX_TRANSLATION: u32 = 0x00ee_6e04;
const D3DX_MATRIX_TRANSPOSE: u32 = 0x00ee_6dd4;
/// The table of four 4x4 `float` matrices `00bb2fc0` fills (`01200688`).
const MATRIX_TABLE: u32 = 0x0120_0688;
/// The constant `00bb3130` returns (`01200788`).
const RECORD_01200788: u32 = 0x0120_0788;

/// The vtables of this unit's classes (none has RTTI): `010b9490` (the
/// base, built on `BSShaderPPLightingProperty`), `010b9338` (derived from
/// it), `010b95c0` (built on the `BSShaderLightingProperty` base `00b71720`),
/// `010b9700` (derived from `010b95c0`'s class), and the two small
/// reference-counted helpers `010b9488` (0x10 bytes) and `010b96f8` (0x20
/// bytes).
const VTABLE_010B9490: u32 = 0x010b_9490;
const VTABLE_010B9338: u32 = 0x010b_9338;
const VTABLE_010B95C0: u32 = 0x010b_95c0;
const VTABLE_010B9700: u32 = 0x010b_9700;
const VTABLE_010B9488: u32 = 0x010b_9488;
const VTABLE_010B96F8: u32 = 0x010b_96f8;

/// `BSShaderPPLightingProperty`'s constructor (`00b66f50`) and destructor
/// (`00b67380`), `thiscall`, no stack words.
const PP_LIGHTING_CONSTRUCTOR: u32 = 0x00b6_6f50;
const PP_LIGHTING_DESTRUCTOR: u32 = 0x00b6_7380;
/// `00b665d0` (unnamed): the member copy of the `010b9490` class's base,
/// `thiscall`, `(clone, process)`.
const PP_LIGHTING_COPY_MEMBERS: u32 = 0x00b6_65d0;
/// The `BSShaderLightingProperty` base constructor (`00b71720`) and
/// destructor (`00b71410`), `thiscall`, no stack words, and its member copy
/// (`00b702b0`, `thiscall`, `(clone, process)`).
const LIGHTING_CONSTRUCTOR: u32 = 0x00b7_1720;
const LIGHTING_DESTRUCTOR: u32 = 0x00b7_1410;
const LIGHTING_COPY_MEMBERS: u32 = 0x00b7_02b0;
/// `NiAVObject::UpdateProperties` (`00a5a040`, `thiscall`, no stack words).
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiAdditionalGeometryData`'s constructor (`00ba63e0`, `thiscall`,
/// `(count, flag)`), `SetDataBlockCount` (`00a72ca0`, one word),
/// `SetDataBlock` (`00a72bb0`, four words) and `SetDataStream` (`00a726a0`,
/// seven words).
const ADDITIONAL_DATA_CONSTRUCTOR: u32 = 0x00ba_63e0;
const ADDITIONAL_DATA_SET_BLOCK_COUNT: u32 = 0x00a7_2ca0;
const ADDITIONAL_DATA_SET_BLOCK: u32 = 0x00a7_2bb0;
const ADDITIONAL_DATA_SET_STREAM: u32 = 0x00a7_26a0;
/// `BSShaderProperty::CheckCreateRenderPassArray` (`00ba95e0`, one word),
/// `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, eight words),
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, no words), and
/// `BSShaderLightingProperty`'s `GetNumberofActiveNonShadowLights`
/// (`00b707d0`, no words, `u16`) and `GetFirstActiveNonShadowLight`
/// (`00b70600`, one word: the iterator).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
const ACTIVE_NON_SHADOW_LIGHT_COUNT: u32 = 0x00b7_07d0;
const FIRST_ACTIVE_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;

/// Fields of the `010b9490` class (and its `010b9338` subclass): the
/// `NiPointer` at +0x104 (an object with a pointer at +8 and a `u16` at
/// +0xC). `BSShaderProperty::iShaderPropertyType` (+0x1C) and `iShader`
/// (+0x58) as the constructors set them.
const POINTER_104: u32 = 0x104;
const SHADER_PROPERTY_TYPE: u32 = 0x1c;
const SHADER: u32 = 0x58;
/// Fields of the `010b95c0` class: the `NiPointer`s at +0x7C and +0x84,
/// the word at +0x80; and of its `010b9700` subclass: the `NiPointer` at
/// +0x88 and the `u16` at +0x8C.
const POINTER_7C: u32 = 0x7c;
const WORD_80: u32 = 0x80;
const POINTER_84: u32 = 0x84;
const POINTER_88: u32 = 0x88;
const WORD_8C: u32 = 0x8c;
/// `BSShaderProperty::iLastRenderPassState` (+0x38), `pRenderPassList`
/// (+0x3C), `pLocalMapRenderPassList` (+0x48) (Xbox PDB) and the byte at
/// +0x74 that forces the passes to be rebuilt.
const LAST_RENDER_PASS_STATE: u32 = 0x38;
const RENDER_PASS_LIST: u32 = 0x3c;
const LOCAL_MAP_RENDER_PASS_LIST: u32 = 0x48;
const REBUILD_PASSES: u32 = 0x74;

/// An `NiPointer`'s release: the count at +4 decremented, the object
/// deleted through its vtable slot +4 when it reaches 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` member's assignment: nothing when the same, else the old
/// one released, the new one stored and its count incremented when not null.
fn assign(e: &mut Engine, member: u32, object: u32) {
    let old = e.mem.u32(member);
    if old != object {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(member, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// An `NiPointer` member's release and clear (`if (p) { release; p = 0 }`).
fn release_and_clear(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
        e.mem.set_u32(member, 0);
    }
}

/// An `NiPointer` member's release without clearing (the destructor's own
/// member destructor).
fn release_member(e: &mut Engine, member: u32) {
    let object = e.mem.u32(member);
    if object != 0 {
        release(e, object);
    }
}

// Translated from 00bb2fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`): for `index` below 4, the translation matrix by `(x,
/// y, 0)` (`D3DXMatrixTranslation` on a stack matrix first set to the
/// identity), transposed in place (`D3DXMatrixTranspose`), copied into
/// matrix `index` of the table at `01200688`; answers true. Answers false
/// for any other index. The two coordinates pass through the x87 stack
/// (`FLD`/`FSTP`).
pub fn fn_00bb2fc0(e: &mut Engine, index: u32, x: f32, y: f32) -> bool {
    if index >= 4 {
        return false;
    }
    e.with_stack(0x40, |e, matrix| {
        for row in 0..4u32 {
            for column in 0..4u32 {
                let value = if row == column { 1.0f32 } else { 0.0 };
                e.mem
                    .set_f32(matrix.addr() + 0x10 * row + 4 * column, value);
            }
        }
        let x = x87_quiet_bits(x.to_bits());
        let y = x87_quiet_bits(y.to_bits());
        e.call(D3DX_MATRIX_TRANSLATION, &args![matrix, x, y, 0u32]);
        e.call(D3DX_MATRIX_TRANSPOSE, &args![matrix, matrix]);
        let bytes = e.mem.bytes(matrix.addr(), 0x40);
        e.mem.write(MATRIX_TABLE + (index << 6), &bytes);
    });
    true
}

// Translated from 00bb3070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010b9338` class: the `010b9490` base
/// constructor with `object`, then the vtable, `iShader` 4 and
/// `iShaderPropertyType` 10. Returns `this`.
pub fn fn_00bb3070(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    fn_00bb3500(e, this, object);
    e.mem.set_u32(this.addr(), VTABLE_010B9338);
    e.mem.set_u32(this.addr() + SHADER, 4);
    e.mem.set_u32(this.addr() + SHADER_PROPERTY_TYPE, 0xa);
    this
}

// Translated from 00bb3130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010b9338` class's vtable slot 2, where `GetRTTI` sits):
/// takes no arguments and returns the constant `01200788`.
pub fn fn_00bb3130(_e: &mut Engine) -> u32 {
    RECORD_01200788
}

// Translated from 00bb3140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9338` class's scalar deleting destructor: its vtable,
/// the `010b9490` base destructor, then the 0x108-byte free when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_00bb3140(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010B9338);
    fn_00bb35b0(e, this);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x108u32]);
    }
    this
}

// Translated from 00bb3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010b9338` class's `CreateClone`): a 0x108-byte object from
/// `00aa13e0`, built as `00bb3070` does with the object `this`'s vtable
/// slot +0x11C returns (0 when the allocation fails), then the base member
/// copy `00b665d0` into it (which still runs for a null clone). Returns the
/// clone.
// Translation incomplete: 00bb3170 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b6e2, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3170(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x108u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        let object = e.vcall(this.addr(), 0x11c, &[]).u32();
        let clone = Ptr::new(block);
        fn_00bb3500(e, clone, object);
        e.mem.set_u32(block, VTABLE_010B9338);
        e.mem.set_u32(block + SHADER, 4);
        e.mem.set_u32(block + SHADER_PROPERTY_TYPE, 0xa);
        clone
    };
    e.call(PP_LIGHTING_COPY_MEMBERS, &args![this, clone, process]);
    clone
}

// Translated from 00bb3210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `NiPointer` at +0x104.
pub fn fn_00bb3210(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + POINTER_104)
}

// Translated from 00bb3220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the pointer at +8 of the object at +0x104, 0 without one.
pub fn fn_00bb3220(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + POINTER_104);
    if object == 0 {
        0
    } else {
        e.mem.u32(object + 8)
    }
}

// Translated from 00bb3240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `u16` at +0xC of the object at +0x104 (zero-extended), 0
/// without one.
pub fn fn_00bb3240(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + POINTER_104);
    if object == 0 {
        0
    } else {
        u32::from(e.mem.u16(object + 0xc))
    }
}

// Translated from 00bb3280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: for a geometry whose data (+0xB8) has a vertex count (`u16` at
/// +8) of at most what `this`'s vtable slot +0x124 answers, a new 0x2C-byte
/// `NiAdditionalGeometryData` for that many vertices (null when the
/// allocation fails) with one data block holding the data `this`'s slot
/// +0x120 answers (16 bytes per vertex, not copied) and one stream (type 4,
/// stride and size 0x10). 0 for a null geometry or too many vertices.
// Translation incomplete: 00bb3280 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b70f, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3280(e: &mut Engine, this: Ptr, geometry: u32) -> u32 {
    if geometry == 0 {
        return 0;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = e.mem.u16(data + 8);
    let limit = e.vcall(this.addr(), 0x124, &[]).u16();
    if vertices > limit {
        return 0;
    }
    let data = e.mem.u32(geometry + 0xb8);
    let vertices = u32::from(e.mem.u16(data + 8));
    build_additional_data(e, this, vertices, 0x120)
}

/// The tail `00bb3280` and `00bb3fc0` share: the `NiAdditionalGeometryData`
/// for `vertices` (null when the allocation fails), one data block (the
/// pointer `this`'s vtable slot `data_slot` answers, `vertices * 16` bytes,
/// not copied) and one stream. The block size and copy flag are pushed
/// before the slot's call and belong to `SetDataBlock`.
fn build_additional_data(e: &mut Engine, this: Ptr, vertices: u32, data_slot: u32) -> u32 {
    let block = e.call(NI_NEW, &args![0x2cu32]).u32();
    let extra = if block == 0 {
        0
    } else {
        e.call(ADDITIONAL_DATA_CONSTRUCTOR, &args![block, vertices, 1u32])
            .u32()
    };
    e.call(ADDITIONAL_DATA_SET_BLOCK_COUNT, &args![extra, 1u32]);
    let size = vertices << 4;
    let source = e.vcall(this.addr(), data_slot, &[]).u32();
    e.call(
        ADDITIONAL_DATA_SET_BLOCK,
        &args![extra, 0u32, source, size, 0u32],
    );
    e.call(
        ADDITIONAL_DATA_SET_STREAM,
        &args![extra, 0u32, 0u32, 0u32, 4u32, vertices, 0x10u32, 0x10u32],
    );
    extra
}

// Translated from 00bb33c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the 0x10-byte `010b9488` helper: an
/// `NiRefObject` (vtable, count 0, `ms_uiObjects` incremented), then the
/// `u16` at +0xC cleared, its vtable, and the pointer at +8 cleared.
/// Returns `this`.
pub fn fn_00bb33c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u16(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr(), VTABLE_010B9488);
    e.mem.set_u32(this.addr() + 8, 0);
    this
}

// Translated from 00bb3400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xC`): the inlined property-state access of
/// `object` (`NiAVObject::UpdateProperties` on it only when `object + 0x9C`
/// is null, as the inlined null check of the member address works out),
/// then `iShader` cleared when either the word the pointer at +0xAC
/// addresses or the one the pointer at +0xB0 addresses is 0. Answers 0.
pub fn fn_00bb3400(e: &mut Engine, this: Ptr, object: u32, _unused_2: u32, _unused_3: u32) -> u32 {
    if object.wrapping_add(0x9c) == 0 {
        e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![object]);
    }
    let first = e.mem.u32(this.addr() + 0xac);
    if e.mem.u32(first) == 0 {
        e.mem.set_u32(this.addr() + SHADER, 0);
        return 0;
    }
    let second = e.mem.u32(this.addr() + 0xb0);
    if e.mem.u32(second) == 0 {
        e.mem.set_u32(this.addr() + SHADER, 0);
    }
    0
}

// Translated from 00bb3470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9488` helper's scalar deleting destructor: its vtable,
/// the buffer at +8 freed through the memory manager when it is set and the
/// count at +0xC is not 0, `NiRefObject`'s vtable and `ms_uiObjects`
/// decremented, then the 0x10-byte free when bit 0 of `flags` is set.
/// Returns `this`.
// Translation incomplete: 00bb3470 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b7b8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3470(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010B9488);
    let buffer = e.mem.u32(this.addr() + 8);
    if buffer != 0 && e.mem.u16(this.addr() + 0xc) != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    }
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00bb3500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010b9490` class: the
/// `BSShaderPPLightingProperty` constructor, the vtable, the pointer at
/// +0x104 cleared and then set to `object` (an `NiPointer` assignment).
/// Returns `this`.
// Translation incomplete: 00bb3500 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b746, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3500(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.call(PP_LIGHTING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B9490);
    e.mem.set_u32(this.addr() + POINTER_104, 0);
    assign(e, this.addr() + POINTER_104, object);
    this
}

// Translated from 00bb35b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9490` class's destructor: its vtable, the pointer at
/// +0x104 released and cleared, then released again by its member
/// destructor (null by then unless the release set it), then the
/// `BSShaderPPLightingProperty` destructor.
// Translation incomplete: 00bb35b0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b746, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb35b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_010B9490);
    release_and_clear(e, this.addr() + POINTER_104);
    release_member(e, this.addr() + POINTER_104);
    e.call(PP_LIGHTING_DESTRUCTOR, &args![this]);
}

// Translated from 00bb3660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010b95c0` class: the
/// `BSShaderLightingProperty` base constructor (`00b71720`), the vtable, the
/// pointer at +0x7C cleared, the word at +0x80 set to 3 and `object` stored
/// at +0x84 with its count incremented when not null. Returns `this`.
pub fn fn_00bb3660(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.call(LIGHTING_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010B95C0);
    e.mem.set_u32(this.addr() + POINTER_7C, 0);
    e.mem.set_u32(this.addr() + WORD_80, 3);
    e.mem.set_u32(this.addr() + POINTER_84, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
    this
}

// Translated from 00bb36c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `NiPointer` at +0x84.
pub fn fn_00bb36c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + POINTER_84)
}

// Translated from 00bb36d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the pointer at +8 of the object at +0x84, 0 without one.
pub fn fn_00bb36d0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + POINTER_84);
    if object == 0 {
        0
    } else {
        e.mem.u32(object + 8)
    }
}

// Translated from 00bb36f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `u16` at +0xC of the object at +0x84 (zero-extended), 0
/// without one.
pub fn fn_00bb36f0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + POINTER_84);
    if object == 0 {
        0
    } else {
        u32::from(e.mem.u16(object + 0xc))
    }
}

// Translated from 00bb3730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `NiPointer` at +0x7C.
pub fn fn_00bb3730(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + POINTER_7C)
}

// Translated from 00bb3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: sets the `NiPointer` at +0x7C to `object` (the old one released,
/// the new one's count incremented; nothing when they are the same).
pub fn fn_00bb3740(e: &mut Engine, this: Ptr, object: u32) {
    assign(e, this.addr() + POINTER_7C, object);
}

// Translated from 00bb3790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b95c0` class's destructor: its vtable, the pointers at
/// +0x84 and +0x7C released and cleared, then released again by their
/// member destructors (+0x84, then +0x7C), then the
/// `BSShaderLightingProperty` base destructor (`00b71410`).
// Translation incomplete: 00bb3790 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b791, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3790(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_010B95C0);
    release_and_clear(e, this.addr() + POINTER_84);
    release_and_clear(e, this.addr() + POINTER_7C);
    release_member(e, this.addr() + POINTER_84);
    release_member(e, this.addr() + POINTER_7C);
    e.call(LIGHTING_DESTRUCTOR, &args![this]);
}

// Translated from 00bb3950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the 0x20-byte `010b96f8` helper: an
/// `NiRefObject` (vtable, count 0, `ms_uiObjects` incremented), its vtable,
/// a 0x300-byte buffer from the memory manager at +8 (zeroed with
/// `memset`), and the three floats +0xC, +0x10, +0x14 set to 0. Returns
/// `this`.
// Translation incomplete: 00bb3950 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b7b8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3950(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u32(this.addr(), VTABLE_010B96F8);
    let buffer = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x300u32])
        .u32();
    e.mem.set_u32(this.addr() + 8, buffer);
    e.call(MEMSET, &args![buffer, 0u32, 0x300u32]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u32(this.addr() + 0x14, 0);
    this
}

// Translated from 00bb39e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: copies `count` words (at most 0xC0) from `source` into the
/// 0xC0-word buffer at +8 and zeroes the rest of it; nothing when `source`
/// or the buffer is null or `count` is 0.
pub fn fn_00bb39e0(e: &mut Engine, this: Ptr, source: u32, count: u32) {
    if source == 0 {
        return;
    }
    let buffer = e.mem.u32(this.addr() + 8);
    if buffer == 0 || count == 0 {
        return;
    }
    let count = if count >= 0xc0 { 0xc0 } else { count };
    let bytes = count * 4;
    e.call(MEMCPY, &args![buffer, source, bytes]);
    if count < 0xc0 {
        let buffer = e.mem.u32(this.addr() + 8);
        e.call(MEMSET, &args![buffer + bytes, 0u32, (0xc0 - count) * 4]);
    }
}

// Translated from 00bb3a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the `010b9700` class's vtable slot 40; `thiscall`, `RET 0x18`):
/// the render passes for `geometry` in `render_mode`. A `kind` of 9 answers
/// the local-map pass list. Otherwise, when the mode differs from the last
/// render-pass state or the rebuild byte (+0x74) is set, the pass list is
/// created or emptied and gets one pass: 0x3E (one light word, the
/// accumulator's) without active non-shadow lights, else 0x3F with the
/// accumulator's word and the first such light; both with the word at +0xE0
/// of the accumulator's +0x194 object, and the state becomes `mode | kind <<
/// 8`. Then `*count = 1` (`u16`) and the pass list is answered. The light
/// iterator lives in the `kind` argument's stack slot (the state was
/// computed before).
pub fn fn_00bb3a50(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    render_mode: u32,
    count: u32,
    kind: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let state = (kind << 8) | render_mode;
    if kind == 9 {
        return e.mem.u32(this.addr() + LOCAL_MAP_RENDER_PASS_LIST);
    }
    if e.mem.u32(this.addr() + LAST_RENDER_PASS_STATE) != render_mode
        || e.mem.u8(this.addr() + REBUILD_PASSES) != 0
    {
        e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, -1i32]);
        let list = e.mem.u32(this.addr() + RENDER_PASS_LIST);
        e.mem.set_u32(list + 0x10, 0);
        let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
        let object = e.mem.u32(accumulator + 0x194);
        let word = e.mem.u32(object + 0xe0);
        let lights = e.call(ACTIVE_NON_SHADOW_LIGHT_COUNT, &args![this]).u16();
        if lights == 0 {
            let list = e.mem.u32(this.addr() + RENDER_PASS_LIST);
            e.call(
                RENDER_PASS_ARRAY_ADD,
                &args![list, geometry, 0x3eu32, 1u32, 1u32, word, 0u32, 0u32, 0u32],
            );
        } else {
            let light = e.with_stack(4, |e, iterator| {
                e.call(FIRST_ACTIVE_NON_SHADOW_LIGHT, &args![this, iterator])
                    .u32()
            });
            let list = e.mem.u32(this.addr() + RENDER_PASS_LIST);
            e.call(
                RENDER_PASS_ARRAY_ADD,
                &args![list, geometry, 0x3fu32, 1u32, 2u32, word, light, 0u32, 0u32],
            );
        }
        e.mem.set_u32(this.addr() + LAST_RENDER_PASS_STATE, state);
    }
    e.mem.set_u16(count, 1);
    e.mem.u32(this.addr() + RENDER_PASS_LIST)
}

// Translated from 00bb3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b96f8` helper's scalar deleting destructor: its vtable,
/// the buffer at +8 freed through the memory manager when set,
/// `NiRefObject`'s vtable and `ms_uiObjects` decremented, then the 0x20-byte
/// free when bit 0 of `flags` is set. Returns `this`.
// Translation incomplete: 00bb3b10 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b7b8, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010B96F8);
    let buffer = e.mem.u32(this.addr() + 8);
    if buffer != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
    }
    e.mem.set_u32(this.addr(), NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x20u32]);
    }
    this
}

// Translated from 00bb3ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the constructor of the `010b9700` class: the `010b95c0` class's
/// constructor with `first`, the vtable, the pointer at +0x88 cleared and
/// then set to `second` (an `NiPointer` assignment), `iShader` and
/// `iShaderPropertyType` 6 and the `u16` at +0x8C set to `value`. Returns
/// `this`.
// Translation incomplete: 00bb3ba0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b7f6, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3ba0(e: &mut Engine, this: Ptr, value: u16, first: u32, second: u32) -> Ptr {
    fn_00bb3660(e, this, first);
    e.mem.set_u32(this.addr(), VTABLE_010B9700);
    e.mem.set_u32(this.addr() + POINTER_88, 0);
    assign(e, this.addr() + POINTER_88, second);
    e.mem.set_u32(this.addr() + SHADER, 6);
    e.mem.set_u32(this.addr() + SHADER_PROPERTY_TYPE, 6);
    e.mem.set_u16(this.addr() + WORD_8C, value);
    this
}

// Translated from 00bb3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `u16` at +0x8C.
pub fn fn_00bb3c70(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + WORD_8C)
}

// Translated from 00bb3c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `NiPointer` at +0x88.
pub fn fn_00bb3c80(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + POINTER_88)
}

// Translated from 00bb3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the pointer at +8 of the object at +0x88, 0 without one.
pub fn fn_00bb3c90(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + POINTER_88);
    if object == 0 {
        0
    } else {
        e.mem.u32(object + 8)
    }
}

// Translated from 00bb3cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9700` class's destructor: its vtable, the pointer at
/// +0x88 released and cleared, then released again by its member
/// destructor, then the `010b95c0` class's destructor.
// Translation incomplete: 00bb3cd0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b7f6, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3cd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_010B9700);
    release_and_clear(e, this.addr() + POINTER_88);
    release_member(e, this.addr() + POINTER_88);
    fn_00bb3790(e, this);
}

// Translated from 00bb3d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9700` class's member copy into `clone` (`RET 8`): the
/// `BSShaderLightingProperty` member copy (`00b702b0`), the clone's vtable
/// slot +0xDC with this object's pointer at +0x7C, the pointer at +0x88 (an
/// `NiPointer` assignment) and the `u16` at +0x8C.
pub fn fn_00bb3d80(e: &mut Engine, this: Ptr, clone: Ptr, process: Ptr) {
    e.call(LIGHTING_COPY_MEMBERS, &args![this, clone, process]);
    let object = e.mem.u32(this.addr() + POINTER_7C);
    e.vcall(clone.addr(), 0xdc, &args![object]);
    let source = e.mem.u32(this.addr() + POINTER_88);
    assign(e, clone.addr() + POINTER_88, source);
    let value = e.mem.u16(this.addr() + WORD_8C);
    e.mem.set_u16(clone.addr() + WORD_8C, value);
}

// Translated from 00bb3ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9700` class's scalar deleting destructor: its
/// destructor, then the 0x90-byte free when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00bb3ee0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb3cd0(e, this);
    if flags as u8 & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x90u32]);
    }
    this
}

// Translated from 00bb3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the `010b9700` class's `CreateClone`: a 0x90-byte object from
/// `00aa13e0` built by its constructor with the low 16 bits of what vtable
/// slot +0xF8 answers, the slot +0xCC answer and the slot +0xFC answer (the
/// slots called +0xFC first, then +0xCC, then +0xF8; 0 when the allocation
/// fails), then the member copy into it (which still runs for a null
/// clone). Returns the clone.
// Translation incomplete: 00bb3f10 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f2b272, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3f10(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x90u32]).u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        let second = e.vcall(this.addr(), 0xfc, &[]).u32();
        let first = e.vcall(this.addr(), 0xcc, &[]).u32();
        let value = e.vcall(this.addr(), 0xf8, &[]).u16();
        fn_00bb3ba0(e, Ptr::new(block), value, first, second)
    };
    fn_00bb3d80(e, this, clone, process);
    clone
}

// Translated from 00bb3fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`, the argument is not read): a new
/// `NiAdditionalGeometryData` for the vertex count `this`'s vtable slot
/// +0xD4 answers (null when the allocation fails), with one data block
/// holding what slot +0xD0 answers (16 bytes per vertex, not copied) and one
/// stream (type 4, stride and size 0x10). Returns it.
// Translation incomplete: 00bb3fc0 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f31c3f, state variable) and the security-cookie bookkeeping are not translated.
pub fn fn_00bb3fc0(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let vertices = e.vcall(this.addr(), 0xd4, &[]).u32();
    build_additional_data(e, this, vertices, 0xd0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_2fc0, fn_00bb2fc0(u32, f32, f32) -> bool),
        entry!(0x00bb_3070, fn_00bb3070(Ptr, u32) -> Ptr),
        entry!(0x00bb_3130, fn_00bb3130() -> u32),
        entry!(0x00bb_3140, fn_00bb3140(Ptr, u32) -> Ptr),
        entry!(0x00bb_3170, fn_00bb3170(Ptr, Ptr) -> Ptr),
        entry!(0x00bb_3210, fn_00bb3210(Ptr) -> u32),
        entry!(0x00bb_3220, fn_00bb3220(Ptr) -> u32),
        entry!(0x00bb_3240, fn_00bb3240(Ptr) -> u32),
        entry!(0x00bb_3280, fn_00bb3280(Ptr, u32) -> u32),
        entry!(0x00bb_33c0, fn_00bb33c0(Ptr) -> Ptr),
        entry!(0x00bb_3400, fn_00bb3400(Ptr, u32, u32, u32) -> u32),
        entry!(0x00bb_3470, fn_00bb3470(Ptr, u32) -> Ptr),
        entry!(0x00bb_3500, fn_00bb3500(Ptr, u32) -> Ptr),
        entry!(0x00bb_35b0, fn_00bb35b0(Ptr)),
        entry!(0x00bb_3660, fn_00bb3660(Ptr, u32) -> Ptr),
        entry!(0x00bb_36c0, fn_00bb36c0(Ptr) -> u32),
        entry!(0x00bb_36d0, fn_00bb36d0(Ptr) -> u32),
        entry!(0x00bb_36f0, fn_00bb36f0(Ptr) -> u32),
        entry!(0x00bb_3730, fn_00bb3730(Ptr) -> u32),
        entry!(0x00bb_3740, fn_00bb3740(Ptr, u32)),
        entry!(0x00bb_3790, fn_00bb3790(Ptr)),
        entry!(0x00bb_3950, fn_00bb3950(Ptr) -> Ptr),
        entry!(0x00bb_39e0, fn_00bb39e0(Ptr, u32, u32)),
        entry!(
            0x00bb_3a50,
            fn_00bb3a50(Ptr, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00bb_3b10, fn_00bb3b10(Ptr, u32) -> Ptr),
        entry!(0x00bb_3ba0, fn_00bb3ba0(Ptr, u16, u32, u32) -> Ptr),
        entry!(0x00bb_3c70, fn_00bb3c70(Ptr) -> u16),
        entry!(0x00bb_3c80, fn_00bb3c80(Ptr) -> u32),
        entry!(0x00bb_3c90, fn_00bb3c90(Ptr) -> u32),
        entry!(0x00bb_3cd0, fn_00bb3cd0(Ptr)),
        entry!(0x00bb_3d80, fn_00bb3d80(Ptr, Ptr, Ptr)),
        entry!(0x00bb_3ee0, fn_00bb3ee0(Ptr, u32) -> Ptr),
        entry!(0x00bb_3f10, fn_00bb3f10(Ptr, Ptr) -> Ptr),
        entry!(0x00bb_3fc0, fn_00bb3fc0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the interlocked imports (real counts), allocation and
    /// free doubles, and a log.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_DELETE, |_, _| Ret::default());
        e.map(0x011f_4000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    /// A reference-counted object with count `count` and a delete slot
    /// (vtable slot +4) double at `0x0040_9004`.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        if !e.mem.is_mapped(0x0090_9000) {
            e.put_vtable(0x0090_9000, &[0, 0x0040_9004]);
            e.register(0x0040_9004, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0090_9000);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A vtable at `table` whose slot `slot` is the double `target`
    /// returning `value`.
    fn slot(e: &mut Engine, table: u32, slot: u32, target: u32, value: u32) {
        if !e.mem.is_mapped(table) {
            e.put_vtable(table, &[0; 0x140 / 4]);
        }
        e.mem.set_u32(table + slot, target);
        e.register_double(target, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    #[test]
    fn matrix_table_gets_the_transposed_translation() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.register(D3DX_MATRIX_TRANSLATION, |e, a| {
            // Check the identity start, then write the translation.
            assert_eq!(e.mem.f32(a[0]), 1.0);
            assert_eq!(e.mem.f32(a[0] + 0x14), 1.0);
            assert_eq!(e.mem.f32(a[0] + 4), 0.0);
            e.mem.set_u32(a[0] + 0x30, a[1]);
            e.mem.set_u32(a[0] + 0x34, a[2]);
            e.mem.set_u32(a[0] + 0x38, a[3]);
            Ret::default()
        });
        e.register(D3DX_MATRIX_TRANSPOSE, |e, a| {
            assert_eq!(a[0], a[1]);
            let x = e.mem.u32(a[0] + 0x30);
            e.mem.set_u32(a[0] + 0x30, 0);
            e.mem.set_u32(a[0] + 0xc, x);
            Ret::default()
        });
        assert!(e.call(0x00bb_2fc0, &args![2u32, 3.0f32, -4.0f32]).bool());
        let matrix = MATRIX_TABLE + 2 * 0x40;
        assert_eq!(e.mem.f32(matrix + 0xc), 3.0);
        assert_eq!(e.mem.f32(matrix + 0x34), -4.0);
        assert_eq!(e.mem.f32(matrix + 0x28), 1.0);
        let translate = calls_to(&e, D3DX_MATRIX_TRANSLATION);
        assert_eq!(
            translate[0][1..],
            [3.0f32.to_bits(), (-4.0f32).to_bits(), 0]
        );
        // Index 4 and above: false, no call.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00bb_2fc0, &args![4u32, 3.0f32, -4.0f32]).bool());
        assert!(calls_to(&e, D3DX_MATRIX_TRANSLATION).is_empty());
    }

    #[test]
    fn constructor_010b9338_builds_on_the_base() {
        let mut e = rig();
        e.register(PP_LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        let object = counted(&mut e, 1);
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        assert_eq!(e.call(0x00bb_3070, &args![this, object]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9338);
        assert_eq!(e.mem.u32(this.addr() + SHADER), 4);
        assert_eq!(e.mem.u32(this.addr() + SHADER_PROPERTY_TYPE), 10);
        assert_eq!(e.mem.u32(this.addr() + POINTER_104), object);
        assert_eq!(e.mem.u32(object + 4), 2);
    }

    #[test]
    fn record_getter_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00bb_3130, &args![]).u32(), 0x0120_0788);
    }

    #[test]
    fn scalar_deleting_destructor_010b9338_frees_with_bit_0() {
        let mut e = rig();
        e.register(PP_LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.call(0x00bb_3140, &args![this, 0u32]);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9490);
        e.call(0x00bb_3140, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x108]]);
        assert_eq!(calls_to(&e, PP_LIGHTING_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn create_clone_010b9338_builds_from_slot_0x11c_and_copies() {
        let mut e = rig();
        e.register(PP_LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PP_LIGHTING_COPY_MEMBERS, |_, _| Ret::default());
        let object = counted(&mut e, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.mem.set_u32(this.addr(), 0x0090_a000);
        slot(&mut e, 0x0090_a000, 0x11c, 0x0040_a11c, object);
        let clone = e.call(0x00bb_3170, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_010B9338);
        assert_eq!(e.mem.u32(clone + POINTER_104), object);
        assert_eq!(e.mem.u32(clone + SHADER), 4);
        assert_eq!(
            calls_to(&e, PP_LIGHTING_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x6000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_3170, &args![this, 0x6000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, PP_LIGHTING_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 0x6000]]
        );
    }

    #[test]
    fn pointer_104_getters() {
        let mut e = rig();
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        assert_eq!(e.call(0x00bb_3210, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00bb_3220, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00bb_3240, &args![this]).u32(), 0);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0x1234_5678);
        e.mem.set_u16(object + 0xc, 0xfffe);
        e.mem.set_u32(this.addr() + POINTER_104, object);
        assert_eq!(e.call(0x00bb_3210, &args![this]).u32(), object);
        assert_eq!(e.call(0x00bb_3220, &args![this]).u32(), 0x1234_5678);
        assert_eq!(e.call(0x00bb_3240, &args![this]).u32(), 0xfffe);
    }

    /// Doubles for the `NiAdditionalGeometryData` calls.
    fn additional_data_doubles(e: &mut Engine) {
        e.register(ADDITIONAL_DATA_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(ADDITIONAL_DATA_SET_BLOCK_COUNT, |_, _| Ret::default());
        e.register(ADDITIONAL_DATA_SET_BLOCK, |_, _| Ret::default());
        e.register(ADDITIONAL_DATA_SET_STREAM, |_, _| Ret::default());
    }

    #[test]
    fn additional_data_for_a_geometry_within_the_vertex_limit() {
        let mut e = rig();
        additional_data_doubles(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.mem.set_u32(this.addr(), 0x0090_b000);
        slot(&mut e, 0x0090_b000, 0x124, 0x0040_b124, 100);
        slot(&mut e, 0x0090_b000, 0x120, 0x0040_b120, 0x7700_0000);
        let data = e.mem.alloc(0x40);
        e.mem.set_u16(data + 8, 100);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, data);
        let extra = e.call(0x00bb_3280, &args![this, geometry]).u32();
        assert_ne!(extra, 0);
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_CONSTRUCTOR),
            vec![vec![extra, 100, 1]]
        );
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_SET_BLOCK_COUNT),
            vec![vec![extra, 1]]
        );
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_SET_BLOCK),
            vec![vec![extra, 0, 0x7700_0000, 1600, 0]]
        );
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_SET_STREAM),
            vec![vec![extra, 0, 0, 0, 4, 100, 0x10, 0x10]]
        );
        // One vertex too many, and a null geometry: 0, nothing built.
        e.mem.set_u16(data + 8, 101);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_3280, &args![this, geometry]).u32(), 0);
        assert_eq!(e.call(0x00bb_3280, &args![this, 0u32]).u32(), 0);
        assert!(calls_to(&e, NI_NEW).is_empty());
    }

    #[test]
    fn helper_010b9488_constructor_and_destructor() {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(this.addr() + 4, 9);
        e.call(0x00bb_33c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9488);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT), 1);
        // A buffer with a count: freed; the object kept (bit 0 clear).
        e.mem.set_u32(this.addr() + 8, 0x5550);
        e.mem.set_u16(this.addr() + 0xc, 1);
        e.call(0x00bb_3470, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x5550]]
        );
        assert_eq!(e.mem.u32(this.addr()), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT), 0);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        // A zero count: the buffer stays; bit 0 frees 0x10 bytes.
        e.mem.set_u16(this.addr() + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00bb_3470, &args![this, 1u32]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x10]]);
    }

    #[test]
    fn property_state_check_clears_the_shader_without_both_words() {
        let mut e = rig();
        e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + 0xac, first);
        e.mem.set_u32(this.addr() + 0xb0, second);
        e.mem.set_u32(first, 1);
        e.mem.set_u32(second, 1);
        e.mem.set_u32(this.addr() + SHADER, 5);
        assert_eq!(
            e.call(0x00bb_3400, &args![this, 0x1000u32, 0u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(this.addr() + SHADER), 5);
        assert!(calls_to(&e, NI_AV_OBJECT_UPDATE_PROPERTIES).is_empty());
        e.mem.set_u32(second, 0);
        e.call(0x00bb_3400, &args![this, 0x1000u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + SHADER), 0);
        e.mem.set_u32(this.addr() + SHADER, 5);
        e.mem.set_u32(second, 1);
        e.mem.set_u32(first, 0);
        e.call(0x00bb_3400, &args![this, 0x1000u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + SHADER), 0);
        // The member address is null only for an object at -0x9C.
        e.call(0x00bb_3400, &args![this, 0xffff_ff64u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_UPDATE_PROPERTIES),
            vec![vec![0xffff_ff64]]
        );
    }

    #[test]
    fn base_010b9490_constructor_assigns_and_destructor_releases() {
        let mut e = rig();
        e.register(PP_LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PP_LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        let object = counted(&mut e, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0x108));
        e.mem.set_u32(this.addr() + POINTER_104, 0xdead);
        e.call(0x00bb_3500, &args![this, object]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9490);
        assert_eq!(e.mem.u32(this.addr() + POINTER_104), object);
        assert_eq!(e.mem.u32(object + 4), 1);
        // The destructor's release drops the count to 0: deleted once.
        e.call(0x00bb_35b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + POINTER_104), 0);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![object]]);
        assert_eq!(
            calls_to(&e, PP_LIGHTING_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
        // A null pointer: nothing released.
        e.call(0x00bb_3500, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + POINTER_104), 0);
    }

    #[test]
    fn constructor_010b95c0_and_its_getters() {
        let mut e = rig();
        e.register(LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        let object = counted(&mut e, 0);
        e.mem.set_u32(object + 8, 0xabcd);
        e.mem.set_u16(object + 0xc, 7);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.call(0x00bb_3660, &args![this, object]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B95C0);
        assert_eq!(e.mem.u32(this.addr() + WORD_80), 3);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(e.call(0x00bb_36c0, &args![this]).u32(), object);
        assert_eq!(e.call(0x00bb_36d0, &args![this]).u32(), 0xabcd);
        assert_eq!(e.call(0x00bb_36f0, &args![this]).u32(), 7);
        assert_eq!(e.call(0x00bb_3730, &args![this]).u32(), 0);
        // Without the object.
        let bare: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.call(0x00bb_3660, &args![bare, 0u32]);
        assert_eq!(e.call(0x00bb_36d0, &args![bare]).u32(), 0);
        assert_eq!(e.call(0x00bb_36f0, &args![bare]).u32(), 0);
    }

    #[test]
    fn set_pointer_7c_releases_the_old_one() {
        let mut e = rig();
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 0);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(this.addr() + POINTER_7C, old);
        e.call(0x00bb_3740, &args![this, new]);
        assert_eq!(e.mem.u32(this.addr() + POINTER_7C), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![old]]);
        // The same object again: nothing changes.
        e.call(0x00bb_3740, &args![this, new]);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    #[test]
    fn destructor_010b95c0_releases_both_pointers() {
        let mut e = rig();
        e.register(LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        let first = counted(&mut e, 1);
        let second = counted(&mut e, 2);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(this.addr() + POINTER_84, first);
        e.mem.set_u32(this.addr() + POINTER_7C, second);
        e.call(0x00bb_3790, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + POINTER_84), 0);
        assert_eq!(e.mem.u32(this.addr() + POINTER_7C), 0);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![first]]);
        assert_eq!(calls_to(&e, LIGHTING_DESTRUCTOR), vec![vec![this.addr()]]);
    }

    #[test]
    fn helper_010b96f8_constructor_and_destructor() {
        let mut e = rig();
        e.register(MEMORY_ALLOCATE, |_, _| Ret {
            eax: 0x0055_0000,
            ..Ret::default()
        });
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 0x10, 0x3f80_0000);
        e.call(0x00bb_3950, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B96F8);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x0055_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x10), 0);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![0x0055_0000, 0, 0x300]]);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x300]]
        );
        e.call(0x00bb_3b10, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0055_0000]]
        );
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x20]]);
        assert_eq!(e.mem.u32(NI_REF_OBJECT_COUNT), 0);
        // Without a buffer: no deallocation; bit 0 clear: no free.
        e.mem.set_u32(this.addr() + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00bb_3b10, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert!(calls_to(&e, NI_DELETE).is_empty());
    }

    #[test]
    fn word_copy_clamps_to_0xc0_and_zeroes_the_rest() {
        let mut e = rig();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(this.addr() + 8, 0x0066_0000);
        e.call(0x00bb_39e0, &args![this, 0x7000u32, 0x10u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x0066_0000, 0x7000, 0x40]]);
        assert_eq!(
            calls_to(&e, MEMSET),
            vec![vec![0x0066_0040, 0, (0xc0 - 0x10) * 4]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00bb_39e0, &args![this, 0x7000u32, 0x200u32]);
        assert_eq!(calls_to(&e, MEMCPY), vec![vec![0x0066_0000, 0x7000, 0x300]]);
        assert!(calls_to(&e, MEMSET).is_empty());
        // Null source, zero count or no buffer: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00bb_39e0, &args![this, 0u32, 4u32]);
        e.call(0x00bb_39e0, &args![this, 0x7000u32, 0u32]);
        e.mem.set_u32(this.addr() + 8, 0);
        e.call(0x00bb_39e0, &args![this, 0x7000u32, 4u32]);
        assert!(calls_to(&e, MEMCPY).is_empty());
    }

    /// The render-pass rig: accumulator object chain, light count `lights`.
    fn pass_rig(lights: u32) -> (Engine, Ptr, u32) {
        let mut e = rig();
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let accumulator = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(accumulator + 0x194, object);
        e.mem.set_u32(object + 0xe0, 0x00ac_c0e0);
        e.register_double(GET_ACCUMULATOR, move |_, _| Ret {
            eax: accumulator,
            ..Ret::default()
        });
        e.register_double(ACTIVE_NON_SHADOW_LIGHT_COUNT, move |_, _| Ret {
            eax: lights,
            ..Ret::default()
        });
        e.register(FIRST_ACTIVE_NON_SHADOW_LIGHT, |_, _| Ret {
            eax: 0x00f1_0000,
            ..Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        let list = e.mem.alloc(0x14);
        e.mem.set_u32(list + 0x10, 5);
        e.mem.set_u32(this.addr() + RENDER_PASS_LIST, list);
        e.mem
            .set_u32(this.addr() + LOCAL_MAP_RENDER_PASS_LIST, 0x0048_0000);
        (e, this, list)
    }

    #[test]
    fn render_passes_rebuild_with_and_without_lights() {
        let (mut e, this, list) = pass_rig(0);
        let count = e.mem.alloc(4);
        let args = args![this, 0x7000u32, 3u32, count, 2u32, 0u32, 0u32];
        assert_eq!(e.call(0x00bb_3a50, &args).u32(), list);
        assert_eq!(e.mem.u16(count), 1);
        assert_eq!(e.mem.u32(list + 0x10), 0);
        assert_eq!(e.mem.u32(this.addr() + LAST_RENDER_PASS_STATE), 0x203);
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![list, 0x7000, 0x3e, 1, 1, 0x00ac_c0e0, 0, 0, 0]]
        );
        let (mut e, this, list) = pass_rig(2);
        let count = e.mem.alloc(4);
        e.call(
            0x00bb_3a50,
            &args![this, 0x7000u32, 3u32, count, 2u32, 0u32, 0u32],
        );
        assert_eq!(
            calls_to(&e, RENDER_PASS_ARRAY_ADD),
            vec![vec![
                list,
                0x7000,
                0x3f,
                1,
                2,
                0x00ac_c0e0,
                0x00f1_0000,
                0,
                0
            ]]
        );
    }

    #[test]
    fn render_passes_keep_the_list_for_the_same_mode_and_answer_local_map_for_9() {
        let (mut e, this, list) = pass_rig(0);
        let count = e.mem.alloc(4);
        e.mem.set_u32(this.addr() + LAST_RENDER_PASS_STATE, 3);
        assert_eq!(
            e.call(
                0x00bb_3a50,
                &args![this, 0x7000u32, 3u32, count, 2u32, 0u32, 0u32]
            )
            .u32(),
            list
        );
        assert!(calls_to(&e, RENDER_PASS_ARRAY_ADD).is_empty());
        assert_eq!(e.mem.u16(count), 1);
        // The rebuild byte forces it.
        e.mem.set_u8(this.addr() + REBUILD_PASSES, 1);
        e.call(
            0x00bb_3a50,
            &args![this, 0x7000u32, 3u32, count, 2u32, 0u32, 0u32],
        );
        assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD).len(), 1);
        // Kind 9: the local-map list, the count untouched.
        e.mem.set_u16(count, 0);
        assert_eq!(
            e.call(
                0x00bb_3a50,
                &args![this, 0x7000u32, 4u32, count, 9u32, 0u32, 0u32]
            )
            .u32(),
            0x0048_0000
        );
        assert_eq!(e.mem.u16(count), 0);
    }

    #[test]
    fn constructor_010b9700_and_its_getters() {
        let mut e = rig();
        e.register(LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        e.mem.set_u32(second + 8, 0x8888);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.call(0x00bb_3ba0, &args![this, 0x1_0042u32, first, second]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B9700);
        assert_eq!(e.mem.u32(this.addr() + POINTER_84), first);
        assert_eq!(e.mem.u32(this.addr() + SHADER), 6);
        assert_eq!(e.mem.u32(this.addr() + SHADER_PROPERTY_TYPE), 6);
        assert_eq!(e.call(0x00bb_3c70, &args![this]).u16(), 0x42);
        assert_eq!(e.call(0x00bb_3c80, &args![this]).u32(), second);
        assert_eq!(e.call(0x00bb_3c90, &args![this]).u32(), 0x8888);
        assert_eq!(e.mem.u32(second + 4), 1);
        e.mem.set_u32(this.addr() + POINTER_88, 0);
        assert_eq!(e.call(0x00bb_3c90, &args![this]).u32(), 0);
    }

    #[test]
    fn destructor_010b9700_releases_and_chains() {
        let mut e = rig();
        e.register(LIGHTING_DESTRUCTOR, |_, _| Ret::default());
        let object = counted(&mut e, 1);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(this.addr() + POINTER_88, object);
        e.call(0x00bb_3cd0, &args![this]);
        assert_eq!(e.mem.u32(this.addr() + POINTER_88), 0);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![object]]);
        // The chained `010b95c0` destructor leaves its vtable.
        assert_eq!(e.mem.u32(this.addr()), VTABLE_010B95C0);
        assert_eq!(calls_to(&e, LIGHTING_DESTRUCTOR).len(), 1);
        e.call(0x00bb_3ee0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x90]]);
    }

    #[test]
    fn member_copy_010b9700_copies_the_pointers_and_word() {
        let mut e = rig();
        e.register(LIGHTING_COPY_MEMBERS, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        let clone: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(clone.addr(), 0x0090_c000);
        slot(&mut e, 0x0090_c000, 0xdc, 0x0040_c0dc, 0);
        let mine = counted(&mut e, 1);
        let theirs = counted(&mut e, 1);
        e.mem.set_u32(this.addr() + POINTER_7C, 0x7c7c);
        e.mem.set_u32(this.addr() + POINTER_88, mine);
        e.mem.set_u16(this.addr() + WORD_8C, 0x33);
        e.mem.set_u32(clone.addr() + POINTER_88, theirs);
        e.call(0x00bb_3d80, &args![this, clone, 0x6000u32]);
        assert_eq!(
            calls_to(&e, LIGHTING_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x6000]]
        );
        assert_eq!(calls_to(&e, 0x0040_c0dc), vec![vec![clone.addr(), 0x7c7c]]);
        assert_eq!(e.mem.u32(clone.addr() + POINTER_88), mine);
        assert_eq!(e.mem.u32(mine + 4), 2);
        assert_eq!(calls_to(&e, 0x0040_9004), vec![vec![theirs]]);
        assert_eq!(e.mem.u16(clone.addr() + WORD_8C), 0x33);
    }

    #[test]
    fn create_clone_010b9700_builds_from_three_slots() {
        let mut e = rig();
        e.register(LIGHTING_CONSTRUCTOR, |_, _| Ret::default());
        e.register(LIGHTING_COPY_MEMBERS, |_, _| Ret::default());
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(this.addr(), 0x0090_d000);
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        slot(&mut e, 0x0090_d000, 0xfc, 0x0040_d0fc, second);
        slot(&mut e, 0x0090_d000, 0xcc, 0x0040_d0cc, first);
        slot(&mut e, 0x0090_d000, 0xf8, 0x0040_d0f8, 0x0009_0021);
        // The clone's own vtable slot +0xDC (the copy's call).
        e.register(0x0040_e0dc, |_, _| Ret::default());
        e.put_vtable(0x010b_9700, &[0; 0x100 / 4]);
        e.mem.set_u32(0x010b_9700 + 0xdc, 0x0040_e0dc);
        let clone = e.call(0x00bb_3f10, &args![this, 0x6000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone + POINTER_84), first);
        assert_eq!(e.mem.u32(clone + POINTER_88), 0);
        assert_eq!(e.mem.u16(clone + WORD_8C), 0);
        let log: Log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| [0x0040_d0fc, 0x0040_d0cc, 0x0040_d0f8].contains(a))
            .collect();
        assert_eq!(order, [0x0040_d0fc, 0x0040_d0cc, 0x0040_d0f8]);
    }

    #[test]
    fn additional_data_from_the_slot_vertex_count() {
        let mut e = rig();
        additional_data_doubles(&mut e);
        let this: Ptr = Ptr::new(e.mem.alloc(0x90));
        e.mem.set_u32(this.addr(), 0x0090_f000);
        slot(&mut e, 0x0090_f000, 0xd4, 0x0040_f0d4, 0x0001_0003);
        slot(&mut e, 0x0090_f000, 0xd0, 0x0040_f0d0, 0x9900);
        let extra = e.call(0x00bb_3fc0, &args![this, 0xdead_beefu32]).u32();
        // The full word from slot +0xD4 is used.
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_CONSTRUCTOR),
            vec![vec![extra, 0x0001_0003, 1]]
        );
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_SET_BLOCK),
            vec![vec![extra, 0, 0x9900, 0x0010_0030, 0]]
        );
        // A failed allocation: the calls get a null object.
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_3fc0, &args![this, 0u32]).u32(), 0);
        assert!(calls_to(&e, ADDITIONAL_DATA_CONSTRUCTOR).is_empty());
        assert_eq!(
            calls_to(&e, ADDITIONAL_DATA_SET_BLOCK_COUNT),
            vec![vec![0, 1]]
        );
    }
}
