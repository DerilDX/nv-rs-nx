//! `NiXenonRenderer/nid3dpass.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00e7ec90` (no stack words): the step `fn_00e80620` ends with, by a jump.
const NEXT_STEP: u32 = 0x00e7_ec90;

// Translated from 00e80150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`; no arguments): allocates 0x18 bytes from
/// the memory manager, zeroes the first four words, sets the words at `+0x10` and `+0x14`
/// to 8 and keeps the block in `0126f72c` (null when the allocation fails).
pub fn fn_00e80150(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(0x0126_f72c, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(0x0126_f72c, block);
}

// Translated from 00e80620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; no arguments): runs `fn_00e80150`, then jumps to
/// `00e7ec90`.
pub fn fn_00e80620(e: &mut Engine) {
    fn_00e80150(e);
    e.call(NEXT_STEP, &args![]);
}

layout! {
    /// `NiD3DPass` (Xbox PDB), 0x68 bytes; the offsets match the PC code's.
    pub struct NiD3DPass: 0x68 {
        /// `m_uiCurrentStage` (Xbox PDB).
        0x14 m_uiCurrentStage: u32,
        /// `m_uiStageCount` (Xbox PDB).
        0x18 m_uiStageCount: u32,
        /// `m_uiTexturesPerPass` (Xbox PDB).
        0x1c m_uiTexturesPerPass: u32,
        /// `m_kStages.m_pBase` (Xbox PDB): the stage array of the `NiTObjectArray` at +0x20.
        0x24 m_kStages_m_pBase: u32,
        /// `m_kStages.m_usMaxSize` (Xbox PDB).
        0x28 m_kStages_m_usMaxSize: u16,
        /// `m_kStages.m_usGrowBy` (Xbox PDB).
        0x2e m_kStages_m_usGrowBy: u16,
        /// `m_pkRenderStateGroup` (Xbox PDB).
        0x30 m_pkRenderStateGroup: u32,
        /// `m_bSoftwareVP` (Xbox PDB).
        0x60 m_bSoftwareVP: u8,
        /// `m_bRendererOwned` (Xbox PDB).
        0x61 m_bRendererOwned: u8,
    }
}

/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The render-state block release `00e7f6b0` (unnamed, `cdecl`, the block), the string free
/// `00aa10f0` (unnamed, `cdecl`, one word), the stage array's clear `00e7fc30` (unit
/// `nid3dpass.cpp`, unnamed; `thiscall` on the array, no stack words) and
/// `NiTPrimitiveSet<..>::AddUnique` (`00e7ec00`, `thiscall` on the set, the address of the
/// element; the engine map names it `NiControllerSequence::SetState`), which gives the pass
/// back to the pass pool (`0126f72c`).
const RELEASE_RENDER_STATES_00E7F6B0: u32 = 0x00e7_f6b0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const STAGES_CLEAR_00E7FC30: u32 = 0x00e7_fc30;
const POOL_ADD_UNIQUE_00E7EC00: u32 = 0x00e7_ec00;
const PASS_POOL_0126F72C: u32 = 0x0126_f72c;

/// `NiPointer<T>` cleared as the code expands it: when it holds an object, the object's count
/// at +4 goes down (`InterlockedDecrement`; deleted through vtable slot +4 at 0) and the
/// pointer is cleared.
fn clear_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(slot, 0);
    }
}

// Translated from 00e80190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): the pass delete that runs when a pass's
/// count at +0x64 reaches 0, `thiscall`, no stack words. A pass the renderer does not own
/// (`m_bRendererOwned`, +0x61, clear) is deleted through its vtable slot 0 with 1. Otherwise
/// the pass is reset for the pass pool: the `NiPointer`s `m_spPixelConstantMap` (+0x34),
/// `m_spPixelShader` (+0x44), `m_spVertexConstantMap` (+0x48) and `m_spVertexShader` (+0x5c)
/// are cleared, the render-state group (+0x30) released (`00e7f6b0`) and cleared, the six
/// program strings (+0x38, +0x3c, +0x40, +0x50, +0x54, +0x58) freed (`00aa10f0`) and cleared
/// (each word cleared just after the next one is read), the stage array (+0x20) cleared
/// (`00e7fc30`), the current stage, stage count and textures per pass (+0x14 to +0x1c) and
/// `m_bSoftwareVP` (+0x60) cleared, and the pass given back to the pass pool (`00e7ec00` on
/// the pool at `0126f72c`, the pass in the stack word the function saved `ECX` in).
pub fn fn_00e80190(e: &mut Engine, this: Ptr<NiD3DPass>) {
    let base = this.addr();
    if e.get(this, NiD3DPass::m_bRendererOwned) == 0 {
        e.vcall(base, 0, &args![1u32]);
        return;
    }
    for offset in [0x34u32, 0x44, 0x48, 0x5c] {
        clear_pointer(e, base + offset);
    }
    let group = e.get(this, NiD3DPass::m_pkRenderStateGroup);
    e.call(RELEASE_RENDER_STATES_00E7F6B0, &args![group]);
    let mut cleared = 0x30u32;
    for offset in [0x38u32, 0x3c, 0x40, 0x50, 0x54, 0x58] {
        let text = e.mem.u32(base + offset);
        e.mem.set_u32(base + cleared, 0);
        e.call(FREE_00AA10F0, &args![text]);
        cleared = offset;
    }
    e.mem.set_u32(base + cleared, 0);
    e.call(STAGES_CLEAR_00E7FC30, &args![base + 0x20]);
    e.set(this, NiD3DPass::m_uiCurrentStage, 0);
    e.set(this, NiD3DPass::m_uiStageCount, 0);
    e.set(this, NiD3DPass::m_uiTexturesPerPass, 0);
    e.set(this, NiD3DPass::m_bSoftwareVP, 0);
    let pool = e.global::<u32>(PASS_POOL_0126F72C);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

/// The two stage limits `SetStage` compares the index with (`011bf560`: at or above it the
/// call fails; `011bf558`: at or above it the stage array may grow first).
const STAGE_LIMIT_011BF560: u32 = 0x011b_f560;
const GROWING_STAGES_011BF558: u32 = 0x011b_f558;
/// The stage array's set-size `00e7fd20` (`thiscall` on the array, one word), its set-at
/// `00e7fb80` (`thiscall` on the array: index, address of the stage pointer) and the stage
/// delete `00e7ecd0` (unnamed; `thiscall`, no stack words).
const STAGES_SET_SIZE_00E7FD20: u32 = 0x00e7_fd20;
const STAGES_SET_AT_00E7FB80: u32 = 0x00e7_fb80;
const STAGE_DELETE_00E7ECD0: u32 = 0x00e7_ecd0;

// Translated from 00e802b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DPass::SetStage` (engine map), `thiscall` with two stack words: the index and the
/// stage. Answers 1 when the index is at or above the limit at `011bf560`. Otherwise the stage
/// is held (count at +0x60 up, when not null), the stage array (+0x20) grows first to the
/// index plus its grow-by (`00e7fd20`) when the index is at or above the word at `011bf558`
/// and at or above the array's max size, and `00e7fb80` stores the stage (through the
/// argument's own stack word) at the index. With a stage: the hold is dropped (`00e7ecd0` at
/// 0), the stage's +4 takes the index, the current stage (+0x14) moves past it when equal, the
/// stage count (+0x18) goes up, and the textures per pass (+0x1c) too when the stage's +8 is
/// set. Without one, when the current stage is the index plus one (and not 0) and the array's
/// element before it is null, the current stage goes down by one. Answers 0.
pub fn ni_d3d_pass_set_stage(e: &mut Engine, this: Ptr<NiD3DPass>, index: u32, stage: Ptr) -> u32 {
    let base = this.addr();
    if index >= e.global::<u32>(STAGE_LIMIT_011BF560) {
        return 1;
    }
    let stage = stage.addr();
    let growing = index >= e.global::<u32>(GROWING_STAGES_011BF558);
    if stage != 0 {
        let held = e.mem.u32(stage + 0x60).wrapping_add(1);
        e.mem.set_u32(stage + 0x60, held);
    }
    if growing && index >= u32::from(e.get(this, NiD3DPass::m_kStages_m_usMaxSize)) {
        let grow_by = u32::from(e.get(this, NiD3DPass::m_kStages_m_usGrowBy));
        e.call(
            STAGES_SET_SIZE_00E7FD20,
            &args![base + 0x20, grow_by.wrapping_add(index)],
        );
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stage);
        e.call(STAGES_SET_AT_00E7FB80, &args![base + 0x20, index, slot]);
    });
    if stage != 0 {
        let left = e.mem.u32(stage + 0x60).wrapping_sub(1);
        e.mem.set_u32(stage + 0x60, left);
        if left == 0 {
            e.call(STAGE_DELETE_00E7ECD0, &args![stage]);
        }
        e.mem.set_u32(stage + 4, index);
        let current = e.get(this, NiD3DPass::m_uiCurrentStage);
        if current == index {
            e.set(this, NiD3DPass::m_uiCurrentStage, current.wrapping_add(1));
        }
        let count = e.get(this, NiD3DPass::m_uiStageCount).wrapping_add(1);
        e.set(this, NiD3DPass::m_uiStageCount, count);
        if e.mem.u32(stage + 8) != 0 {
            let textures = e.get(this, NiD3DPass::m_uiTexturesPerPass).wrapping_add(1);
            e.set(this, NiD3DPass::m_uiTexturesPerPass, textures);
        }
        return 0;
    }
    let current = e.get(this, NiD3DPass::m_uiCurrentStage);
    if current == index.wrapping_add(1) && current != 0 {
        let array = e.get(this, NiD3DPass::m_kStages_m_pBase);
        let last = e
            .mem
            .u32(array.wrapping_add(current.wrapping_mul(4)).wrapping_sub(4));
        // `TEST ECX,ECX / JZ / TEST EAX,EAX / JNZ` back to the first test: with the element
        // set (and the current stage not 0) the exe spins here for good. `00e7fb80` has just
        // stored the null stage at that index, so the element is null whenever the store
        // succeeded.
        loop {
            if last == 0 {
                e.set(this, NiD3DPass::m_uiCurrentStage, current.wrapping_sub(1));
                return 0;
            }
            if current == 0 {
                return 0;
            }
        }
    }
    0
}

/// A stage's reference count (+0x60) down by one; the stage deleted (`00e7ecd0`) at 0.
fn stage_release(e: &mut Engine, stage: u32) {
    let left = e.mem.u32(stage.wrapping_add(0x60)).wrapping_sub(1);
    e.mem.set_u32(stage.wrapping_add(0x60), left);
    if left == 0 {
        e.call(STAGE_DELETE_00E7ECD0, &args![stage]);
    }
}

/// The stage array's function-local `static` null pointer (`0126f730`) and its guard word
/// (`0126f734`, bit 0), `_atexit` (`00ec658f`) and the destructor it registers (`00fde350`).
const NULL_STAGE_0126F730: u32 = 0x0126_f730;
const NULL_STAGE_GUARD_0126F734: u32 = 0x0126_f734;
const ATEXIT: u32 = 0x00ec_658f;
const NULL_STAGE_DESTRUCT_00FDE350: u32 = 0x00fd_e350;

// Translated from 00e7fb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): the set-at of the pass's stage array
/// (`NiTObjectArray<NiPointer<NiD3DTextureStage>>`: base +4, size +0xa, element count +0xc),
/// `thiscall` with two stack words: the index and the address of a stage pointer. The first
/// call sets up the function's `static` null pointer (`0126f730`, guard bit in `0126f734`,
/// its destructor `00fde350` given to `_atexit`). At or past the size the size becomes the
/// index plus one and the element count goes up for a stage that is not that null pointer;
/// below it the count goes down when a stage is replaced by the null pointer and up when the
/// null pointer is replaced by a stage. Then the slot is assigned the stage when they differ
/// (the old one's count at +0x60 dropped, `00e7ecd0` at 0; the new one's raised).
pub fn fn_00e7fb80(e: &mut Engine, this: Ptr, index: u32, stage: Ptr) {
    let base = this.addr();
    let stage = stage.addr();
    let guard = e.global::<u32>(NULL_STAGE_GUARD_0126F734);
    if guard & 1 == 0 {
        e.set_global(NULL_STAGE_GUARD_0126F734, guard | 1);
        e.set_global(NULL_STAGE_0126F730, 0u32);
        e.call(ATEXIT, &args![NULL_STAGE_DESTRUCT_00FDE350]);
    }
    let size = u32::from(e.mem.u16(base + 0xa));
    if index >= size {
        e.mem.set_u16(base + 0xa, index.wrapping_add(1) as u16);
        if e.mem.u32(stage) != e.global::<u32>(NULL_STAGE_0126F730) {
            let count = e.mem.u16(base + 0xc).wrapping_add(1);
            e.mem.set_u16(base + 0xc, count);
        }
    } else {
        let null = e.global::<u32>(NULL_STAGE_0126F730);
        let elements = e.mem.u32(base + 4);
        let held = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if e.mem.u32(stage) != null {
            if held == null {
                let count = e.mem.u16(base + 0xc).wrapping_add(1);
                e.mem.set_u16(base + 0xc, count);
            }
        } else if held != null {
            let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
            e.mem.set_u16(base + 0xc, count);
        }
    }
    let slot = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(4));
    let old = e.mem.u32(slot);
    if old != e.mem.u32(stage) {
        if old != 0 {
            stage_release(e, old);
        }
        let new = e.mem.u32(stage);
        e.mem.set_u32(slot, new);
        if new != 0 {
            let held = e.mem.u32(new + 0x60).wrapping_add(1);
            e.mem.set_u32(new + 0x60, held);
        }
    }
}

// Translated from 00e7fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): the clear of the pass's stage array,
/// `thiscall` on the array, no stack words: each stage up to the size (+0xa, read again each
/// time) is let go (count at +0x60 down, `00e7ecd0` at 0) and its slot cleared; the size and
/// the element count (+0xc) become 0.
pub fn fn_00e7fc30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let slot = e.mem.u32(base + 4).wrapping_add(u32::from(index) * 4);
        let stage = e.mem.u32(slot);
        if stage != 0 {
            stage_release(e, stage);
            e.mem.set_u32(slot, 0);
        }
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
}

/// The stage pointer array allocation `00c1d790` (unit `nipsysmeshemitter.cpp` in the engine
/// map, unnamed; `cdecl`, the count) and its free `00e7fcd0` (unit `nid3dpass.cpp`, unnamed;
/// `cdecl`, the array).
const NEW_POINTERS_00C1D790: u32 = 0x00c1_d790;
const FREE_POINTERS_00E7FCD0: u32 = 0x00e7_fcd0;

// Translated from 00e7fd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): the set-size of the pass's stage array
/// (base +4, max size +8, size +0xa, element count +0xc), `thiscall` with one stack word: the
/// new size (compared whole with the 16-bit fields). Nothing when it equals the max size. Below
/// the current size, the set slots from the new size up are let go (stage count at +0x60 down,
/// `00e7ecd0` at 0), cleared and taken off the element count, and the size becomes the new
/// size. The max size takes the new size; with 0 the base is cleared, else a new array
/// (`00c1d790`) gets the stages up to the size (each slot assigned as an `NiPointer`: the slot's
/// old stage let go, the new one held) and the slots up to the max size are let go and cleared.
/// The old array is freed (`00e7fcd0`).
pub fn fn_00e7fd20(e: &mut Engine, this: Ptr, size: u32) {
    let base = this.addr();
    if size == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    let narrow = size as u16;
    if size < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = narrow;
        if index < e.mem.u16(base + 0xa) {
            loop {
                let slot = e.mem.u32(base + 4).wrapping_add(u32::from(index) * 4);
                if e.mem.u32(slot) != 0 {
                    let stage = e.mem.u32(slot);
                    if stage != 0 {
                        stage_release(e, stage);
                        e.mem.set_u32(slot, 0);
                    }
                    let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(base + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(base + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(base + 0xa, narrow);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, narrow);
    if size == 0 {
        e.mem.set_u32(base + 4, 0);
        e.call(FREE_POINTERS_00E7FCD0, &args![old]);
        return;
    }
    let array = e
        .call(NEW_POINTERS_00C1D790, &args![u32::from(narrow)])
        .u32();
    e.mem.set_u32(base + 4, array);
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let offset = u32::from(index) * 4;
        let slot = e.mem.u32(base + 4).wrapping_add(offset);
        let held = e.mem.u32(slot);
        if held != e.mem.u32(old.wrapping_add(offset)) {
            if held != 0 {
                stage_release(e, held);
            }
            let stage = e.mem.u32(old.wrapping_add(offset));
            e.mem.set_u32(slot, stage);
            if stage != 0 {
                let count = e.mem.u32(stage + 0x60).wrapping_add(1);
                e.mem.set_u32(stage + 0x60, count);
            }
        }
        index = index.wrapping_add(1);
    }
    let mut index = e.mem.u16(base + 0xa);
    while index < e.mem.u16(base + 8) {
        let slot = e.mem.u32(base + 4).wrapping_add(u32::from(index) * 4);
        let stage = e.mem.u32(slot);
        if stage != 0 {
            stage_release(e, stage);
            e.mem.set_u32(slot, 0);
        }
        index = index.wrapping_add(1);
    }
    e.call(FREE_POINTERS_00E7FCD0, &args![old]);
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00e7fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): `delete[]` of an array of stage pointers
/// (the count word before it), `cdecl` with one word (the array). Nothing for null. Otherwise
/// each stage, from the last one down, is let go (count at +0x60 down, `00e7ecd0` at 0; the
/// signed count stops at 0) and the block (from the count word) goes back to the memory
/// manager.
pub fn fn_00e7fcd0(e: &mut Engine, array: Ptr) {
    let array = array.addr();
    if array == 0 {
        return;
    }
    let count = e.mem.u32(array.wrapping_sub(4));
    let mut index = count.wrapping_sub(1) as i32;
    while index >= 0 {
        let stage = e
            .mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)));
        if stage != 0 {
            stage_release(e, stage);
        }
        index = index.wrapping_sub(1);
    }
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER_OBJECT, array.wrapping_sub(4)],
    );
}

/// `NiD3DPass`'s vtable (`010ef364`, RTTI `.?AVNiD3DPass@@`) and the stage array's
/// constructor `00e80020` (unit `nid3dpass.cpp`, unnamed; `thiscall` on the array: the max size
/// and the grow-by).
const PASS_VTABLE_010EF364: u32 = 0x010e_f364;
const STAGES_CONSTRUCT_00E80020: u32 = 0x00e8_0020;

// Translated from 00e800f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): `NiD3DPass`'s constructor (the vtable it
/// stores has that class's RTTI), `thiscall`, no stack words. Stores the vtable `010ef364`,
/// clears the current stage, stage count and textures per pass (+0x14 to +0x1c), builds the
/// stage array at +0x20 (`00e80020` with the word at `011bf558` and 4), clears +0x30 to +0x48,
/// the byte +0x4c, +0x50 to +0x5c, the bytes `m_bSoftwareVP` and `m_bRendererOwned`, the count
/// +0x64 and the first byte of the name (+4). Answers `this`.
pub fn fn_00e800f0(e: &mut Engine, this: Ptr<NiD3DPass>) -> Ptr<NiD3DPass> {
    let base = this.addr();
    e.mem.set_u32(base, PASS_VTABLE_010EF364);
    e.set(this, NiD3DPass::m_uiCurrentStage, 0);
    e.set(this, NiD3DPass::m_uiStageCount, 0);
    e.set(this, NiD3DPass::m_uiTexturesPerPass, 0);
    let max = e.global::<u32>(GROWING_STAGES_011BF558);
    e.call(STAGES_CONSTRUCT_00E80020, &args![base + 0x20, max, 4u32]);
    for offset in (0x30u32..=0x48).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x4c, 0);
    for offset in [0x50u32, 0x54, 0x58, 0x5c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.set(this, NiD3DPass::m_bSoftwareVP, 0);
    e.set(this, NiD3DPass::m_bRendererOwned, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u8(base + 4, 0);
    this
}

/// The stage arrays' vtables: `NiTArray<NiPointer<NiD3DTextureStage>,..>` (`010ef35c`) and
/// `NiTObjectArray<NiPointer<NiD3DTextureStage>>` (`010ef378`) (RTTI names read in Ghidra).
const STAGE_ARRAY_BASE_VTABLE_010EF35C: u32 = 0x010e_f35c;
const STAGE_ARRAY_VTABLE_010EF378: u32 = 0x010e_f378;

// Translated from 00e80020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dpass.cpp`): the stage array's constructor
/// (`NiTObjectArray<NiPointer<NiD3DTextureStage>>`), `thiscall` with two stack words: the max
/// size and the grow-by (both used as 16 bits). Grow-by +0xe, the base class's vtable
/// `010ef35c`, max size +8, size and element count 0; an array of that many pointers
/// (`00c1d790`) for a non-zero max size, else none; then the vtable `010ef378`. Answers `this`.
pub fn fn_00e80020(e: &mut Engine, this: Ptr, max_size: u32, grow_by: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 0xe, grow_by as u16);
    e.mem.set_u32(base, STAGE_ARRAY_BASE_VTABLE_010EF35C);
    e.mem.set_u16(base + 8, max_size as u16);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    let elements = if max_size as u16 != 0 {
        e.call(NEW_POINTERS_00C1D790, &args![u32::from(max_size as u16)])
            .u32()
    } else {
        0
    };
    e.mem.set_u32(base + 4, elements);
    e.mem.set_u32(base, STAGE_ARRAY_VTABLE_010EF378);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_0150, fn_00e80150()),
        entry!(0x00e8_0620, fn_00e80620()),
        entry!(0x00e8_0190, fn_00e80190(Ptr<NiD3DPass>)),
        entry!(
            0x00e8_02b0,
            ni_d3d_pass_set_stage(Ptr<NiD3DPass>, u32, Ptr) -> u32
        ),
        entry!(0x00e7_fb80, fn_00e7fb80(Ptr, u32, Ptr)),
        entry!(0x00e7_fc30, fn_00e7fc30(Ptr)),
        entry!(0x00e7_fd20, fn_00e7fd20(Ptr, u32)),
        entry!(0x00e7_fcd0, fn_00e7fcd0(Ptr)),
        entry!(0x00e8_00f0, fn_00e800f0(Ptr<NiD3DPass>) -> Ptr<NiD3DPass>),
        entry!(0x00e8_0020, fn_00e80020(Ptr, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_and_runs_the_next_step() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.register(NEXT_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00e8_0620, &args![]);
        assert_eq!(e.mem.u32(0x0126_f72c), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
        let order: Vec<u32> = e
            .call_log
            .unwrap()
            .into_iter()
            .map(|(a, _)| a)
            .filter(|a| *a != MEMORY_MANAGER_ALLOCATE)
            .collect();
        assert_eq!(order, vec![0x00e8_0620, NEXT_STEP]);
    }

    #[test]
    fn failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(0x0126_f72c, 5);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e8_0150, &args![]);
        assert_eq!(e.mem.u32(0x0126_f72c), 0);
    }
}

#[cfg(test)]
mod pass_tests {
    use super::*;

    const IMPORT_INTERLOCKED: u32 = IMPORT_INTERLOCKED_DECREMENT;
    const OBJECT_DELETE: u32 = 0x0e70_0004;
    const OBJECT_VTABLE: u32 = 0x0e71_0000;
    const PASS_DELETE: u32 = 0x0e72_0000;
    const PASS_VTABLE: u32 = 0x0e73_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011b_f000, 0x1000);
        e.register(IMPORT_INTERLOCKED, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(OBJECT_VTABLE, &[0, OBJECT_DELETE]);
        e.put_vtable(PASS_VTABLE, &[PASS_DELETE]);
        for target in [
            OBJECT_DELETE,
            PASS_DELETE,
            RELEASE_RENDER_STATES_00E7F6B0,
            FREE_00AA10F0,
            STAGES_CLEAR_00E7FC30,
            STAGES_SET_SIZE_00E7FD20,
            STAGE_DELETE_00E7ECD0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        // The add-unique double notes the element its slot holds in the pool's first word.
        e.register(POOL_ADD_UNIQUE_00E7EC00, |e, a| {
            let element = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], element);
            Ret::default()
        });
        // The set-at double stores the stage its slot holds into the array.
        e.register(STAGES_SET_AT_00E7FB80, |e, a| {
            let stage = e.mem.u32(a[2]);
            let array = e.mem.u32(a[0] + 4);
            e.mem.set_u32(array + 4 * a[1], stage);
            Ret::default()
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

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00e80190_resets_an_owned_pass_for_the_pool() {
        let mut e = engine();
        let pool = e.mem.alloc(4);
        e.set_global(PASS_POOL_0126F72C, pool);
        let pass = e.mem.alloc(0x68);
        e.mem.write(pass, &[0x11; 0x68]);
        e.mem.set_u32(pass, PASS_VTABLE);
        let kept = object(&mut e, 2);
        let dropped = object(&mut e, 1);
        e.mem.set_u32(pass + 0x34, kept);
        e.mem.set_u32(pass + 0x44, dropped);
        e.mem.set_u32(pass + 0x48, 0);
        e.mem.set_u32(pass + 0x5c, 0);
        e.mem.set_u32(pass + 0x30, 0x0e74_0000);
        let strings = [0x38u32, 0x3c, 0x40, 0x50, 0x54, 0x58];
        for (index, offset) in strings.iter().enumerate() {
            e.mem.set_u32(pass + offset, 0x0e75_0000 + index as u32);
        }
        e.mem.set_u8(pass + 0x61, 1);
        e.call(0x00e8_0190, &args![pass]);
        assert_eq!(e.mem.u32(kept + 4), 1);
        assert_eq!(calls(&e, OBJECT_DELETE), vec![vec![dropped]]);
        for offset in [0x30u32, 0x34, 0x44, 0x38, 0x3c, 0x40, 0x50, 0x54, 0x58] {
            assert_eq!(e.mem.u32(pass + offset), 0, "{offset:#x}");
        }
        assert_eq!(
            calls(&e, RELEASE_RENDER_STATES_00E7F6B0),
            vec![vec![0x0e74_0000]]
        );
        let freed: Vec<Vec<u32>> = (0..6).map(|i| vec![0x0e75_0000 + i]).collect();
        assert_eq!(calls(&e, FREE_00AA10F0), freed);
        assert_eq!(calls(&e, STAGES_CLEAR_00E7FC30), vec![vec![pass + 0x20]]);
        assert_eq!(e.mem.bytes(pass + 0x14, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u8(pass + 0x60), 0);
        assert_eq!(e.mem.u8(pass + 0x61), 1);
        assert_eq!(e.mem.u32(pool), pass);
        assert!(calls(&e, PASS_DELETE).is_empty());
    }

    #[test]
    fn fn_00e80190_deletes_a_pass_the_renderer_does_not_own() {
        let mut e = engine();
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass, PASS_VTABLE);
        e.call(0x00e8_0190, &args![pass]);
        assert_eq!(calls(&e, PASS_DELETE), vec![vec![pass, 1]]);
        assert!(calls(&e, FREE_00AA10F0).is_empty());
    }

    /// A pass whose stage array has room for 4 stages (grow-by 2), the limits 8 and 2.
    fn pass(e: &mut Engine) -> u32 {
        e.set_global::<u32>(STAGE_LIMIT_011BF560, 8);
        e.set_global::<u32>(GROWING_STAGES_011BF558, 2);
        let pass = e.mem.alloc(0x68);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(pass + 0x24, array);
        e.mem.set_u16(pass + 0x28, 4);
        e.mem.set_u16(pass + 0x2e, 2);
        pass
    }

    fn stage(e: &mut Engine, textures: u32) -> u32 {
        let stage = e.mem.alloc(0x70);
        e.mem.set_u32(stage + 8, textures);
        e.mem.set_u32(stage + 0x60, 1);
        stage
    }

    #[test]
    fn set_stage_stores_and_counts_a_stage() {
        let mut e = engine();
        let pass = pass(&mut e);
        let first = stage(&mut e, 1);
        assert_eq!(e.call(0x00e8_02b0, &args![pass, 0u32, first]).u32(), 0);
        let array = e.mem.u32(pass + 0x24);
        assert_eq!(e.mem.u32(array), first);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(first + 0x60), 1);
        assert_eq!(e.mem.u32(pass + 0x14), 1);
        assert_eq!(e.mem.u32(pass + 0x18), 1);
        assert_eq!(e.mem.u32(pass + 0x1c), 1);
        // A stage without textures at index 3: no textures counted, the current stage stays.
        let second = stage(&mut e, 0);
        e.call(0x00e8_02b0, &args![pass, 3u32, second]);
        assert_eq!(e.mem.u32(second + 4), 3);
        assert_eq!(e.mem.u32(pass + 0x14), 1);
        assert_eq!(e.mem.u32(pass + 0x18), 2);
        assert_eq!(e.mem.u32(pass + 0x1c), 1);
        assert!(calls(&e, STAGES_SET_SIZE_00E7FD20).is_empty());
        assert!(calls(&e, STAGE_DELETE_00E7ECD0).is_empty());
    }

    #[test]
    fn set_stage_grows_the_array_and_refuses_indices_past_the_limit() {
        let mut e = engine();
        let pass = pass(&mut e);
        // Index 4 is past the max size (4) and the growing limit (2): set-size to 4 + 2. The
        // set-at double then writes past the array's 0x10 bytes, so it gets a larger one.
        let array = e.mem.alloc(0x20);
        e.mem.set_u32(pass + 0x24, array);
        let new = stage(&mut e, 0);
        e.mem.set_u32(new + 0x60, 0);
        e.call(0x00e8_02b0, &args![pass, 4u32, new]);
        assert_eq!(
            calls(&e, STAGES_SET_SIZE_00E7FD20),
            vec![vec![pass + 0x20, 6]]
        );
        // Held then dropped: back to 0 and deleted.
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![new]]);
        assert_eq!(e.call(0x00e8_02b0, &args![pass, 8u32, new]).u32(), 1);
        assert_eq!(calls(&e, STAGES_SET_AT_00E7FB80).len(), 1);
    }

    #[test]
    fn set_stage_with_no_stage_steps_the_current_stage_back() {
        let mut e = engine();
        let pass = pass(&mut e);
        e.mem.set_u32(pass + 0x14, 2);
        assert_eq!(e.call(0x00e8_02b0, &args![pass, 1u32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(pass + 0x14), 1);
        // Not the last stage: nothing changes.
        e.call(0x00e8_02b0, &args![pass, 3u32, 0u32]);
        assert_eq!(e.mem.u32(pass + 0x14), 1);
        assert_eq!(e.mem.u32(pass + 0x18), 0);
    }
}

#[cfg(test)]
mod stage_array_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for target in [STAGE_DELETE_00E7ECD0, ATEXIT, FREE_POINTERS_00E7FCD0] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(NEW_POINTERS_00C1D790, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn stage(e: &mut Engine, count: u32) -> u32 {
        let stage = e.mem.alloc(0x68);
        e.mem.set_u32(stage + 0x60, count);
        stage
    }

    /// An array of max size 4 holding `stages` from slot 0 (size and count their number).
    fn array(e: &mut Engine, stages: &[u32]) -> u32 {
        let array = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x10);
        for (index, stage) in stages.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *stage);
        }
        e.mem.set_u32(array + 4, elements);
        e.mem.set_u16(array + 8, 4);
        e.mem.set_u16(array + 0xa, stages.len() as u16);
        e.mem.set_u16(array + 0xc, stages.len() as u16);
        array
    }

    #[test]
    fn fn_00e7fb80_sets_stages_and_counts_them() {
        let mut e = engine();
        let old = stage(&mut e, 1);
        let array = array(&mut e, &[old]);
        let new = stage(&mut e, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, new);
        // Past the size: the size grows, the count too.
        e.call(0x00e7_fb80, &args![array, 2u32, slot]);
        assert_eq!(e.mem.u16(array + 0xa), 3);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        let elements = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(elements + 8), new);
        assert_eq!(e.mem.u32(new + 0x60), 2);
        assert_eq!(calls(&e, ATEXIT), vec![vec![NULL_STAGE_DESTRUCT_00FDE350]]);
        // A null stage over the old one: count down, the old one let go and deleted.
        e.mem.set_u32(slot, 0);
        e.call(0x00e7_fb80, &args![array, 0u32, slot]);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u32(elements), 0);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![old]]);
        // A stage into the empty slot 1: count up; the guard is set up only once.
        e.mem.set_u32(slot, new);
        e.call(0x00e7_fb80, &args![array, 1u32, slot]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        assert_eq!(e.mem.u32(new + 0x60), 3);
        assert_eq!(calls(&e, ATEXIT).len(), 1);
    }

    #[test]
    fn fn_00e7fc30_lets_every_stage_go() {
        let mut e = engine();
        let kept = stage(&mut e, 2);
        let dropped = stage(&mut e, 1);
        let array = array(&mut e, &[kept, 0, dropped]);
        e.call(0x00e7_fc30, &args![array]);
        let elements = e.mem.u32(array + 4);
        assert_eq!(e.mem.bytes(elements, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(kept + 0x60), 1);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![dropped]]);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
    }

    #[test]
    fn fn_00e7fd20_shrinks_and_grows_the_stage_array() {
        let mut e = engine();
        let stages = [stage(&mut e, 1), stage(&mut e, 2), stage(&mut e, 1)];
        let array = array(&mut e, &stages);
        let old = e.mem.u32(array + 4);
        e.call(0x00e7_fd20, &args![array, 1u32]);
        // Slots 1 and 2 let go (2 deleted), the first copied into the new array and held.
        assert_eq!(e.mem.u16(array + 0xa), 1);
        assert_eq!(e.mem.u16(array + 8), 1);
        assert_eq!(e.mem.u16(array + 0xc), 1);
        assert_eq!(e.mem.u32(stages[1] + 0x60), 1);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![stages[2]]]);
        let new = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(new), stages[0]);
        assert_eq!(e.mem.u32(stages[0] + 0x60), 2);
        assert_eq!(calls(&e, NEW_POINTERS_00C1D790), vec![vec![1]]);
        assert_eq!(calls(&e, FREE_POINTERS_00E7FCD0), vec![vec![old]]);
        // Growing to 3: the stage carried over, the rest empty.
        e.call(0x00e7_fd20, &args![array, 3u32]);
        let grown = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(grown), stages[0]);
        assert_eq!(e.mem.bytes(grown + 4, 8), vec![0; 8]);
        assert_eq!(e.mem.u16(array + 8), 3);
        // The same max size: nothing; 0: the base cleared and the array freed.
        e.call(0x00e7_fd20, &args![array, 3u32]);
        assert_eq!(calls(&e, NEW_POINTERS_00C1D790).len(), 2);
        e.call(0x00e7_fd20, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(
            calls(&e, FREE_POINTERS_00E7FCD0).last().unwrap(),
            &vec![grown]
        );
    }
}

#[cfg(test)]
mod b000p_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e7fcd0_lets_the_stages_go_and_frees_the_block() {
        let mut e = Engine::new();
        e.register(STAGE_DELETE_00E7ECD0, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let kept = e.mem.alloc(0x68);
        e.mem.set_u32(kept + 0x60, 2);
        let dropped = e.mem.alloc(0x68);
        e.mem.set_u32(dropped + 0x60, 1);
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, 3);
        e.mem.set_u32(block + 4, dropped);
        e.mem.set_u32(block + 0xc, kept);
        e.call(0x00e7_fcd0, &args![block + 4]);
        assert_eq!(e.mem.u32(kept + 0x60), 1);
        assert_eq!(calls(&e, STAGE_DELETE_00E7ECD0), vec![vec![dropped]]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, block]]
        );
        e.call(0x00e7_fcd0, &args![0u32]);
        assert_eq!(calls(&e, MEMORY_MANAGER_DEALLOCATE).len(), 1);
    }

    #[test]
    fn fn_00e800f0_builds_an_empty_pass() {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.set_global(GROWING_STAGES_011BF558, 8u32);
        e.register(STAGES_CONSTRUCT_00E80020, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let pass = e.mem.alloc(0x68);
        e.mem.write(pass, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00e8_00f0, &args![pass]).u32(), pass);
        assert_eq!(e.mem.u32(pass), PASS_VTABLE_010EF364);
        assert_eq!(e.mem.u8(pass + 4), 0);
        assert_eq!(e.mem.u8(pass + 5), 0xcc);
        assert_eq!(e.mem.bytes(pass + 0x14, 0xc), vec![0; 0xc]);
        assert_eq!(e.mem.u32(pass + 0x20), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(pass + 0x30, 0x1d), vec![0; 0x1d]);
        assert_eq!(e.mem.bytes(pass + 0x4d, 3), vec![0xcc; 3]);
        assert_eq!(e.mem.bytes(pass + 0x50, 0x12), vec![0; 0x12]);
        assert_eq!(e.mem.u32(pass + 0x64), 0);
        assert_eq!(
            calls(&e, STAGES_CONSTRUCT_00E80020),
            vec![vec![pass + 0x20, 8, 4]]
        );
    }
}

#[cfg(test)]
mod b000r_tests {
    use super::*;

    #[test]
    fn fn_00e80020_builds_the_stage_array() {
        let mut e = Engine::new();
        e.register(NEW_POINTERS_00C1D790, |e, a| {
            e.mem.alloc(4 * a[0]).into_ret()
        });
        e.call_log = Some(vec![]);
        let array = e.mem.alloc(0x10);
        e.mem.write(array, &[0xcc; 0x10]);
        let answer = e
            .call(0x00e8_0020, &args![array, 0x1_0008u32, 0x2_0004u32])
            .u32();
        assert_eq!(answer, array);
        assert_eq!(e.mem.u32(array), STAGE_ARRAY_VTABLE_010EF378);
        assert_ne!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u16(array + 8), 8);
        assert_eq!(e.mem.u16(array + 0xa), 0);
        assert_eq!(e.mem.u16(array + 0xc), 0);
        assert_eq!(e.mem.u16(array + 0xe), 4);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NEW_POINTERS_00C1D790, vec![8]));
        e.call_log = Some(vec![]);
        e.call(0x00e8_0020, &args![array, 0x1_0000u32, 4u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
