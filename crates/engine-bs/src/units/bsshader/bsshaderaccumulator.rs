//! `BSShader/bsshaderaccumulator.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTPointerList, NiTPointerMap};

layout! {
    /// `BSShaderAccumulator` (Xbox PDB; 0x280 bytes on PC, the size
    /// `Main::Main` allocates; the Xbox build has a second map and critical
    /// section after +0x280). The offsets used below match the PC code.
    pub struct BSShaderAccumulator: 0x280 {
        /// `bDoDepthPasses` (Xbox PDB).
        0x03a bDoDepthPasses: u8,
        /// `uiSunPixelCount` (Xbox PDB).
        0x03c uiSunPixelCount: u32,
        /// `bWaitingForSunQueury` (Xbox PDB).
        0x040 bWaitingForSunQueury: u8,
        /// `fPercentSunOccludedStored` (Xbox PDB), as its bit pattern.
        0x044 fPercentSunOccludedStored: u32,
        /// `bAccumulate` (Xbox PDB).
        0x084 bAccumulate: u8,
        /// `b1stPerson` (Xbox PDB).
        0x085 b1stPerson: u8,
        /// `bRenderNoShader` (Xbox PDB).
        0x086 bRenderNoShader: u8,
        /// `usQueryCount` (Xbox PDB).
        0x088 usQueryCount: u16,
        /// `usQuerySize` (Xbox PDB).
        0x08a usQuerySize: u16,
        /// `ppOcclusionQuery` (Xbox PDB).
        0x08c ppOcclusionQuery: u32,
        /// `pVolumetricFogGeometry` (Xbox PDB), an `NiTPointerList`.
        0x090 pVolumetricFogGeometry: Inline<NiTPointerList>,
        /// `spBoundTestGeometry` (Xbox PDB).
        0x13c spBoundTestGeometry: u32,
        /// `ReferenceVolumeList` (Xbox PDB), an `NiTPointerList`.
        0x140 ReferenceVolumeList: Inline<NiTPointerList>,
        /// `uiLastOcclusionRef` (Xbox PDB).
        0x14c uiLastOcclusionRef: u32,
        /// `bLastOcclusionResult` (Xbox PDB).
        0x150 bLastOcclusionResult: u8,
        /// `kSilhouetteColor.r` (Xbox PDB, `NiColorA`), as its bit pattern.
        0x154 kSilhouetteColor_r: u32,
        /// `kSilhouetteColor.g` (Xbox PDB).
        0x158 kSilhouetteColor_g: u32,
        /// `kSilhouetteColor.b` (Xbox PDB).
        0x15c kSilhouetteColor_b: u32,
        /// `kSilhouetteColor.a` (Xbox PDB).
        0x160 kSilhouetteColor_a: u32,
        /// `bRenderDecals` (Xbox PDB).
        0x16c bRenderDecals: u8,
        /// `pInstanceRenderer` (Xbox PDB).
        0x170 pInstanceRenderer: u32,
        /// `pBatchRenderer[0]` (Xbox PDB).
        0x174 pBatchRenderer0: u32,
        /// `pBatchRenderer[1]` (Xbox PDB).
        0x178 pBatchRenderer1: u32,
        /// `puicurrentPassA` (Xbox PDB).
        0x17c puicurrentPassA: u32,
        /// `picurrentBucketA` (Xbox PDB).
        0x180 picurrentBucketA: u32,
        /// `pbcurrentFirstPassA` (Xbox PDB).
        0x184 pbcurrentFirstPassA: u32,
        /// `pbcurrentActiveA` (Xbox PDB).
        0x188 pbcurrentActiveA: u32,
        /// `uiBatchRendererCount` (Xbox PDB).
        0x18c uiBatchRendererCount: u32,
        /// `uiBatchRendererPassCount` (Xbox PDB).
        0x190 uiBatchRendererPassCount: u32,
        /// `spWaterFogRenderTarget` (Xbox PDB), an `NiPointer`.
        0x198 spWaterFogRenderTarget: u32,
        /// `eRenderMode` (Xbox PDB).
        0x19c eRenderMode: u32,
        /// `spShadowLight` (Xbox PDB), an `NiPointer`.
        0x1a0 spShadowLight: u32,
        /// `kRoomSharedMap` (Xbox PDB), an `NiTPointerMap`.
        0x1b4 kRoomSharedMap: Inline<NiTPointerMap>,
        /// `BSCPSharedCrit` (Xbox PDB): its `CRITICAL_SECTION` comes first.
        0x200 BSCPSharedCrit: u32,
    }
}

/// The base-class constructor `00b660d0` runs first (thiscall, no
/// arguments; unnamed in the engine map).
const BASE_CONSTRUCTOR: u32 = 0x00a9_b480;
/// The class's vtable.
const ACCUMULATOR_VTABLE: u32 = 0x010a_dff8;
/// The compiler's `eh vector constructor iterator` (stdcall: array, element
/// size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// Constructor of one `BSTPersistentList` element (0x14 bytes).
const PERSISTENT_LIST_CONSTRUCTOR: u32 = 0x00b9_88c0;
/// Destructor of one `BSTPersistentList` element.
const PERSISTENT_LIST_DESTRUCTOR: u32 = 0x00b6_56b0;
/// `BSShaderProperty::RenderPass::RenderPass` (engine map; cdecl, six words).
const RENDER_PASS_CONSTRUCTOR: u32 = 0x00ba_8ec0;
/// The vtable the inlined `NiTPointerMap` constructor sets first.
const MAP_BASE_VTABLE: u32 = 0x010a_dfbc;
/// The map vtable it sets last.
const MAP_VTABLE: u32 = 0x010a_dfd8;
/// The allocator the inlined map constructor calls for its buckets (cdecl,
/// one word: the size).
const ALLOCATE_BUCKETS: u32 = 0x00aa_1070;
/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// `InitializeCriticalSection` (import slot).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The allocator used for the batch renderers (cdecl, one word: the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// Constructor of the 4-byte object kept in `pInstanceRenderer` (thiscall,
/// no arguments).
const INSTANCE_RENDERER_CONSTRUCTOR: u32 = 0x00bd_9120;
/// Constructor of a 0x98-byte batch renderer (engine map unit
/// `bsbatchrenderer.cpp`; thiscall, two words).
const BATCH_RENDERER_CONSTRUCTOR: u32 = 0x00b9_a5b0;
/// `NiTPointerListBase<..>::RemoveAll` (engine map; thiscall).
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// The float the three colour channels of `kSilhouetteColor` take.
const SILHOUETTE_CHANNEL: u32 = 0x0101_7870;
/// Word of the thread's TLS block the constructor clears.
const TLS_WORD_2BC: u32 = 0x2bc;
/// `1.0f` as the `FLD1; FSTP dword` stores it.
const ONE_BITS: u32 = 0x3f80_0000;

/// Drops the `NiPointer` held at `slot` (the inlined smart-pointer
/// release): when the count at +4 reaches zero calls the object's virtual
/// method at +4, then clears the slot.
fn release_pointer(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        if e.call(IMPORT_INTERLOCKED_DECREMENT, &[object + 4]).u32() == 0 {
            e.vcall(object, 4, &[]);
        }
        e.mem.set_u32(slot, 0);
    }
}

/// `count * 4` as the code computes the array size (`MUL`, `SETO`, `NEG`,
/// `OR`: on overflow the size is `0xFFFFFFFF`).
fn word_array_size(count: u32) -> u32 {
    count.checked_mul(4).unwrap_or(u32::MAX)
}

// Translated from 00b660d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderAccumulator::BSShaderAccumulator` (Xbox PDB): runs the base
/// constructor, sets the vtable, clears the sun tests, constructs the two
/// rendered-geometry caches (4 `BSTPersistentList`s each), the alpha render
/// pass, the room map (37 buckets) and its critical section, allocates the
/// 50 occlusion-query slots, the instance renderer, one batch renderer (two
/// when `batch_renderer_count` is 2) built with `pass_count` and
/// `batch_argument`, and the four per-renderer arrays; drops the shadow
/// light and water fog target (both already cleared earlier in the
/// constructor, so the releases only run if a callee in between set them),
/// clears word +0x2BC of the thread block and
/// sets the silhouette colour to the constant at `01017870` (alpha 1).
/// Returns `this`.
// Translation incomplete: 00b660d0 - C++ exception unwinding (SEH frame state) is not translated.
pub fn bs_shader_accumulator_bs_shader_accumulator(
    e: &mut Engine,
    this: Ptr<BSShaderAccumulator>,
    batch_argument: u32,
    batch_renderer_count: u32,
    pass_count: u32,
) -> Ptr<BSShaderAccumulator> {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &[base]);
    e.mem.set_u32(base, ACCUMULATOR_VTABLE);
    // FLDZ; FST to the three sun tests' floats.
    e.mem.set_f32(base + 0x50, 0.0);
    e.mem.set_f32(base + 0x64, 0.0);
    e.mem.set_f32(base + 0x78, 0.0);
    // pSunTests[0..3] (Xbox PDB `SunOcclusionTest`, 0x14 bytes each).
    for test in [0x48, 0x5c, 0x70] {
        e.mem.set_u32(base + test, 0);
        e.mem.set_u8(base + test + 4, 0);
        e.mem.set_u32(base + test + 0xc, 0);
        e.mem.set_u32(base + test + 0x10, 0);
    }
    e.mem.set_u32(base + 0x80, 0);
    let fog = this.at(BSShaderAccumulator::pVolumetricFogGeometry);
    e.set(fog, NiTPointerList::m_uiCount, 0);
    e.set(fog, NiTPointerList::m_pkHead, 0);
    e.set(fog, NiTPointerList::m_pkTail, 0);
    for cache in [0x9c, 0xec] {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &[
                base + cache,
                0x14,
                4,
                PERSISTENT_LIST_CONSTRUCTOR,
                PERSISTENT_LIST_DESTRUCTOR,
            ],
        );
    }
    e.set(this, BSShaderAccumulator::spBoundTestGeometry, 0);
    let volumes = this.at(BSShaderAccumulator::ReferenceVolumeList);
    e.set(volumes, NiTPointerList::m_uiCount, 0);
    e.set(volumes, NiTPointerList::m_pkHead, 0);
    e.set(volumes, NiTPointerList::m_pkTail, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_r, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_g, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_b, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_a, 0);
    e.set(this, BSShaderAccumulator::spWaterFogRenderTarget, 0);
    e.set(this, BSShaderAccumulator::spShadowLight, 0);
    e.call(RENDER_PASS_CONSTRUCTOR, &[base + 0x1a4, 0, 0x2f5, 1, 0, 0]);
    // The inlined NiTPointerMap constructor (37 buckets).
    let map = this.at(BSShaderAccumulator::kRoomSharedMap);
    e.mem.set_u32(map.addr(), MAP_BASE_VTABLE);
    e.set(map, NiTPointerMap::m_uiHashSize, 0x25);
    e.set(map, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(ALLOCATE_BUCKETS, &[0x94]).u32();
    let hash_size = e.get(map, NiTPointerMap::m_uiHashSize);
    e.set(map, NiTPointerMap::m_ppkHashTable, buckets);
    e.call(MEMSET, &[buckets, 0, hash_size.wrapping_mul(4)]);
    e.mem.set_u32(map.addr(), MAP_VTABLE);
    e.call(IMPORT_INITIALIZE_CRITICAL_SECTION, &[base + 0x200]);
    e.set(this, BSShaderAccumulator::bAccumulate, 1);
    e.set(this, BSShaderAccumulator::b1stPerson, 0);
    e.set(this, BSShaderAccumulator::bRenderNoShader, 1);
    let queries = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 200]).u32();
    e.set(this, BSShaderAccumulator::ppOcclusionQuery, queries);
    e.set(this, BSShaderAccumulator::usQueryCount, 0);
    e.set(this, BSShaderAccumulator::usQuerySize, 0x32);
    let mut offset = 0u32;
    while offset < 200 {
        let slots = e.get(this, BSShaderAccumulator::ppOcclusionQuery);
        e.mem.set_u32(offset.wrapping_add(slots), 0);
        offset += 4;
    }
    e.call(POINTER_LIST_REMOVE_ALL, &[fog.addr()]);
    e.set(
        this,
        BSShaderAccumulator::fPercentSunOccludedStored,
        ONE_BITS,
    );
    e.set(this, BSShaderAccumulator::bWaitingForSunQueury, 0);
    e.set(this, BSShaderAccumulator::uiSunPixelCount, 0);
    for test in [0x4c, 0x60, 0x74] {
        e.mem.set_f32(base + test + 4, 0.0);
        e.mem.set_u32(base + test - 4, 0);
        e.mem.set_u8(base + test, 0);
        e.mem.set_u32(base + test + 8, 0);
        e.mem.set_u32(base + test + 0xc, 0);
    }
    e.set(this, BSShaderAccumulator::uiLastOcclusionRef, 0);
    e.set(this, BSShaderAccumulator::bLastOcclusionResult, 0);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 4]).u32();
    let instance = if block != 0 {
        e.call(INSTANCE_RENDERER_CONSTRUCTOR, &[block]).u32()
    } else {
        0
    };
    e.set(this, BSShaderAccumulator::pInstanceRenderer, instance);
    e.set(
        this,
        BSShaderAccumulator::uiBatchRendererCount,
        batch_renderer_count,
    );
    e.set(
        this,
        BSShaderAccumulator::uiBatchRendererPassCount,
        pass_count,
    );
    let block = e.call(OPERATOR_NEW, &[0x98]).u32();
    let first = if block != 0 {
        let passes = e.get(this, BSShaderAccumulator::uiBatchRendererPassCount);
        e.call(BATCH_RENDERER_CONSTRUCTOR, &[block, passes, batch_argument])
            .u32()
    } else {
        0
    };
    e.set(this, BSShaderAccumulator::pBatchRenderer0, first);
    if e.get(this, BSShaderAccumulator::uiBatchRendererCount) == 2 {
        let block = e.call(OPERATOR_NEW, &[0x98]).u32();
        let second = if block != 0 {
            let passes = e.get(this, BSShaderAccumulator::uiBatchRendererPassCount);
            e.call(BATCH_RENDERER_CONSTRUCTOR, &[block, passes, batch_argument])
                .u32()
        } else {
            0
        };
        e.set(this, BSShaderAccumulator::pBatchRenderer1, second);
    } else {
        e.set(this, BSShaderAccumulator::pBatchRenderer1, 0);
    }
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let passes = e
        .call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, word_array_size(count)])
        .u32();
    e.set(this, BSShaderAccumulator::puicurrentPassA, passes);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let buckets = e
        .call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, word_array_size(count)])
        .u32();
    e.set(this, BSShaderAccumulator::picurrentBucketA, buckets);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let first_pass = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, count]).u32();
    e.set(this, BSShaderAccumulator::pbcurrentFirstPassA, first_pass);
    let count = e.get(this, BSShaderAccumulator::uiBatchRendererCount);
    let active = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, count]).u32();
    e.set(this, BSShaderAccumulator::pbcurrentActiveA, active);
    e.set(this, BSShaderAccumulator::eRenderMode, 0);
    release_pointer(e, base + 0x1a0);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_WORD_2BC, 0);
    e.set(this, BSShaderAccumulator::bRenderDecals, 1);
    release_pointer(e, base + 0x198);
    // FLD dword [01017870]; FST to three temporaries (an x87 load quiets a
    // signalling NaN), then FLD1 for the alpha.
    let channel = x87_store32(x87_load32(e.mem.u32(SILHOUETTE_CHANNEL)));
    e.set(this, BSShaderAccumulator::bDoDepthPasses, 0);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_r, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_g, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_b, channel);
    e.set(this, BSShaderAccumulator::kSilhouetteColor_a, ONE_BITS);
    this
}

/// The vtable of a rendered-geometry cache element (`BSTPersistentList`,
/// 0x14 bytes), set by its constructor `00b988c0` and its destructor.
const PERSISTENT_LIST_VTABLE: u32 = 0x010b_7dc0;
/// The element's clean-up `00b63e90` (this unit, not yet translated;
/// thiscall), run before and after the words are reset.
const PERSISTENT_LIST_CLEAR: u32 = 0x00b6_3e90;

// Translated from 00b656b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of a rendered-geometry cache element (unnamed in the engine
/// map; the `eh vector constructor iterator` of
/// `BSShaderAccumulator::BSShaderAccumulator` names it): sets the vtable,
/// runs `00b63e90`, copies the word at +0x04 to +0x0C, clears +0x04, +0x08
/// and +0x10, and jumps to `00b63e90` again, passing its result on.
pub fn fn_00b656b0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, PERSISTENT_LIST_VTABLE);
    e.call(PERSISTENT_LIST_CLEAR, &[base]);
    let first = e.mem.u32(base + 4);
    e.mem.set_u32(base + 0xc, first);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.call(PERSISTENT_LIST_CLEAR, &[base]).u32()
}

/// The node pool's lock object (`this` of the lock enter `0040fbf0`); its
/// owner word.
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's entry count, which the inlined leave decrements.
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The lock enter (`0040fbf0`; thiscall, one word).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// Head of the node pool's free list.
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00b63e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clean-up of a rendered-geometry cache element (`BSTPersistentList`;
/// unnamed in the engine map): when its node list (+0x0C) is not empty,
/// enters the node pool's lock and, for each node, enters it again, clears
/// the node's +0x04, pushes the node on the free list at `011c5f58` and
/// leaves (the count at `011c5f84` drops by one; at zero the owner word
/// `011c5f80` is cleared); then leaves the outer entry the same way. Clears
/// +0x0C and the word the pointer at +0x08 points to (when set). Returns
/// that pointer.
pub fn fn_00b63e90(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let mut next = e.mem.u32(base + 0xc);
    if next != 0 {
        e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
        loop {
            let node = next;
            next = e.mem.u32(node);
            e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
            e.mem.set_u32(node + 4, 0);
            let free = e.mem.u32(NODE_POOL_FREE_LIST);
            e.mem.set_u32(node, free);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
            e.mem.set_u32(NODE_POOL_FREE_LIST, node);
            if count == 0 {
                e.mem.set_u32(NODE_POOL_LOCK, 0);
            }
            if next == 0 {
                break;
            }
        }
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
    }
    let tail = e.mem.u32(base + 8);
    e.mem.set_u32(base + 0xc, 0);
    if tail != 0 {
        e.mem.set_u32(tail, 0);
    }
    tail
}

#[cfg(test)]
mod element_clear_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        // The lock enter double takes the lock: owner 1, count + 1.
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        e
    }

    #[test]
    fn element_clear_returns_the_nodes_to_the_pool() {
        let mut e = engine();
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0099_0000);
        let this = e.mem.alloc(0x14);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        let tail_slot = e.mem.alloc(4);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x11);
        e.mem.set_u32(first + 8, 0x12);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, 0x21);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(this + 8, tail_slot);
        e.mem.set_u32(tail_slot, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3e90, &[this]).u32(), tail_slot);
        let locks = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == LOCK_ENTER)
            .count();
        assert_eq!(locks, 3);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first), 0x0099_0000);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(second + 4), 0);
        // The node's element (+0x08) is left as it is.
        assert_eq!(e.mem.u32(first + 8), 0x12);
        // Balanced: the lock is released.
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(tail_slot), 0);
    }

    #[test]
    fn element_clear_of_an_empty_list_takes_no_lock() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_3e90, &[this]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 5);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b660d0,
            bs_shader_accumulator_bs_shader_accumulator(
                Ptr<BSShaderAccumulator>,
                u32,
                u32,
                u32,
            ) -> Ptr<BSShaderAccumulator>
        ),
        entry!(0x00b6_56b0, fn_00b656b0(Ptr) -> u32),
        entry!(0x00b6_3e90, fn_00b63e90(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod element_destructor_tests {
    use super::*;

    #[test]
    fn element_destructor_resets_the_words_between_two_clean_ups() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for (off, value) in [(0, 1u32), (4, 0x40), (8, 0x80), (0xc, 0xc0), (0x10, 0x100)] {
            e.mem.set_u32(this + off, value);
        }
        // The clean-up sees the vtable set; its second run sees the words
        // reset and its result is passed on.
        e.register_double(PERSISTENT_LIST_CLEAR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), PERSISTENT_LIST_VTABLE);
            (e.mem.u32(a[0] + 0xc) + e.mem.u32(a[0] + 4)).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_56b0, &[this]).u32(), 0x40);
        let clears = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == PERSISTENT_LIST_CLEAR)
            .count();
        assert_eq!(clears, 2);
        assert_eq!(e.mem.u32(this + 0xc), 0x40);
        for off in [4, 8, 0x10] {
            assert_eq!(e.mem.u32(this + off), 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine whose callees outside this file are doubles: allocators
    /// hand out fresh blocks (0 for the size `fail` names), the
    /// constructors return their `this`.
    fn accumulator_engine(fail: u32) -> Engine {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| ret(0));
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register(RENDER_PASS_CONSTRUCTOR, |_, _| ret(0));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| ret(0));
        e.register(ALLOCATE_BUCKETS, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            if a[1] == fail {
                ret(0)
            } else {
                ret(e.mem.alloc(a[1]))
            }
        });
        e.register_double(OPERATOR_NEW, move |e, a| {
            if a[0] == fail {
                ret(0)
            } else {
                ret(e.mem.alloc(a[0]))
            }
        });
        e.register(INSTANCE_RENDERER_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(BATCH_RENDERER_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(POINTER_LIST_REMOVE_ALL, |_, _| ret(0));
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u32(SILHOUETTE_CHANNEL, 0x3e99_999a);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn constructor_builds_one_batch_renderer_and_sets_the_defaults() {
        let mut e = accumulator_engine(u32::MAX);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        // Garbage the constructor must overwrite (no smart pointers held).
        for off in (0x04..0x280).step_by(4) {
            e.mem.set_u32(base + off, 0xdead_beef);
        }
        e.mem.set_u32(base + 0x198, 0);
        e.mem.set_u32(base + 0x1a0, 0);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_WORD_2BC, 7);
        e.call_log = Some(vec![]);
        let result = e.call(0x00b6_60d0, &[base, 0x55, 1, 9]).u32();
        assert_eq!(result, base);
        assert_eq!(e.mem.u32(base), ACCUMULATOR_VTABLE);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![base]]);
        let element = [PERSISTENT_LIST_CONSTRUCTOR, PERSISTENT_LIST_DESTRUCTOR];
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![
                vec![base + 0x9c, 0x14, 4, element[0], element[1]],
                vec![base + 0xec, 0x14, 4, element[0], element[1]],
            ]
        );
        assert_eq!(
            calls_to(&e, RENDER_PASS_CONSTRUCTOR),
            vec![vec![base + 0x1a4, 0, 0x2f5, 1, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_INITIALIZE_CRITICAL_SECTION),
            vec![vec![base + 0x200]]
        );
        assert_eq!(
            calls_to(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![base + 0x90]]
        );
        // The room map.
        assert_eq!(e.mem.u32(base + 0x1b4), MAP_VTABLE);
        assert_eq!(e.mem.u32(base + 0x1b8), 0x25);
        assert_eq!(e.mem.u32(base + 0x1c0), 0);
        let buckets = e.mem.u32(base + 0x1bc);
        assert_eq!(calls_to(&e, ALLOCATE_BUCKETS), vec![vec![0x94]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![buckets, 0, 0x94]]);
        // Sun tests and lists cleared.
        for off in [
            0x3c, 0x48, 0x50, 0x54, 0x58, 0x5c, 0x64, 0x68, 0x6c, 0x70, 0x78, 0x7c, 0x80,
        ] {
            assert_eq!(e.mem.u32(base + off), 0, "+{off:#x}");
        }
        for off in [0x4c, 0x60, 0x74, 0x40, 0x85, 0x150, 0x3a] {
            assert_eq!(e.mem.u8(base + off), 0, "+{off:#x}");
        }
        for off in [0x90, 0x94, 0x98, 0x13c, 0x140, 0x144, 0x148, 0x14c, 0x19c] {
            assert_eq!(e.mem.u32(base + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(base + 0x44), ONE_BITS);
        assert_eq!(e.mem.u8(base + 0x84), 1);
        assert_eq!(e.mem.u8(base + 0x86), 1);
        assert_eq!(e.mem.u8(base + 0x16c), 1);
        // Occlusion queries: 50 cleared slots.
        assert_eq!(e.mem.u16(base + 0x88), 0);
        assert_eq!(e.mem.u16(base + 0x8a), 0x32);
        let queries = e.mem.u32(base + 0x8c);
        assert_ne!(queries, 0);
        assert!((0..200).step_by(4).all(|i| e.mem.u32(queries + i) == 0));
        // One batch renderer built from the pass count and the argument.
        assert_ne!(e.mem.u32(base + 0x170), 0);
        assert_eq!(e.mem.u32(base + 0x18c), 1);
        assert_eq!(e.mem.u32(base + 0x190), 9);
        let first = e.mem.u32(base + 0x174);
        assert_eq!(
            calls_to(&e, BATCH_RENDERER_CONSTRUCTOR),
            vec![vec![first, 9, 0x55]]
        );
        assert_eq!(e.mem.u32(base + 0x178), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, vec![200, 4, 4, 4, 1, 1]);
        for off in [0x17c, 0x180, 0x184, 0x188] {
            assert_ne!(e.mem.u32(base + off), 0);
        }
        assert_eq!(e.mem.u32(tls + TLS_WORD_2BC), 0);
        // Silhouette colour: the constant three times, alpha 1.
        assert_eq!(e.mem.u32(base + 0x154), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x158), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x15c), 0x3e99_999a);
        assert_eq!(e.mem.u32(base + 0x160), ONE_BITS);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn constructor_releases_pointers_and_keeps_failed_renderers_null() {
        // The 0x98-byte allocations fail: both batch renderers stay null.
        let mut e = accumulator_engine(0x98);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        // A held shadow light (last reference) and water fog target (shared).
        let light = e.mem.alloc(8);
        let target = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x0077_0001);
        e.register(0x0077_0001, |_, _| ret(0));
        e.mem.set_u32(light, vtable);
        e.mem.set_u32(light + 4, 1);
        e.mem.set_u32(target, vtable);
        e.mem.set_u32(target + 4, 3);
        // The constructor clears both pointers before it releases them; the
        // instance-renderer double stores them in between so the release
        // runs.
        e.register_double(INSTANCE_RENDERER_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(base + 0x1a0, light);
            e.mem.set_u32(base + 0x198, target);
            ret(a[0])
        });
        // A signalling NaN constant is quieted by the x87 load.
        e.mem.set_u32(SILHOUETTE_CHANNEL, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[base, 0x55, 2, 3]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x98], vec![0x98]]);
        assert!(calls_to(&e, BATCH_RENDERER_CONSTRUCTOR).is_empty());
        assert_eq!(e.mem.u32(base + 0x174), 0);
        assert_eq!(e.mem.u32(base + 0x178), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, vec![200, 4, 8, 8, 2, 2]);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![light + 4], vec![target + 4]]
        );
        assert_eq!(calls_to(&e, 0x0077_0001), vec![vec![light]]);
        assert_eq!(e.mem.u32(base + 0x1a0), 0);
        assert_eq!(e.mem.u32(base + 0x198), 0);
        assert_eq!(e.mem.u32(base + 0x154), 0x7fc0_0001);
        assert_eq!(e.mem.u32(base + 0x160), ONE_BITS);
    }

    #[test]
    fn constructor_builds_two_renderers_and_a_null_instance_renderer() {
        let mut e = accumulator_engine(u32::MAX);
        let this = e.new_object::<BSShaderAccumulator>();
        let base = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[base, 0x66, 2, 5]);
        let first = e.mem.u32(base + 0x174);
        let second = e.mem.u32(base + 0x178);
        assert_ne!(first, 0);
        assert_ne!(second, 0);
        assert_eq!(
            calls_to(&e, BATCH_RENDERER_CONSTRUCTOR),
            vec![vec![first, 5, 0x66], vec![second, 5, 0x66]]
        );
        // The instance renderer's allocation failing leaves it null.
        let mut e = accumulator_engine(4);
        let this = e.new_object::<BSShaderAccumulator>();
        e.call_log = Some(vec![]);
        e.call(0x00b6_60d0, &[this.addr(), 0, 3, 1]);
        assert_eq!(e.mem.u32(this.addr() + 0x170), 0);
        assert!(calls_to(&e, INSTANCE_RENDERER_CONSTRUCTOR).is_empty());
        // The overflow rule of the word arrays.
        assert_eq!(word_array_size(0x4000_0000), u32::MAX);
        assert_eq!(word_array_size(3), 12);
    }
}
