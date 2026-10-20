//! `NiMain/nioldscreenspace.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiScreenGeometry::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_GEOMETRY_RTTI: u32 = 0x011f_5e5c;

/// `NiScreenPolygon::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_POLYGON_RTTI: u32 = 0x011f_5e64;

/// `NiScreenGeometryData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_GEOMETRY_DATA_RTTI: u32 = 0x011f_5e54;

/// `NiScreenSpaceCamera::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_SPACE_CAMERA_RTTI: u32 = 0x011f_5e6c;

// Translated from 00a8ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_geometry_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_GEOMETRY_RTTI)
}

// Translated from 00a8ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_polygon_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_POLYGON_RTTI)
}

// Translated from 00a90ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_GEOMETRY_DATA_RTTI)
}

// Translated from 00a91690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_space_camera_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_SPACE_CAMERA_RTTI)
}

/// `00a98ea0`: the jump target of `00a877d0`.
const JUMP_TARGET_00A877D0: u32 = 0x00a9_8ea0;

/// `00a71d10`: the jump target of `00a877e0`.
const JUMP_TARGET_00A877E0: u32 = 0x00a7_1d10;

/// `00a71450`: the jump target of `00a90000`.
const JUMP_TARGET_00A90000: u32 = 0x00a7_1450;

/// `00a716c0`: the jump target of `00a90020`.
const JUMP_TARGET_00A90020: u32 = 0x00a7_16c0;

/// `00a6ff20`: the jump target of `00a90030`.
const JUMP_TARGET_00A90030: u32 = 0x00a6_ff20;

// Translated from 00a877d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::LoadBinary` (Xbox PDB): a 5-byte jump to `00a98ea0` (engine map: `NiParticleMeshes::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiScreenGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_geometry_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A877D0, &args![this, stream]);
}

// Translated from 00a877e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::SaveBinary` (Xbox PDB): a 5-byte jump to `00a71d10` (engine map: `NiParticleMeshes::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiScreenGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_geometry_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A877E0, &args![this, stream]);
}

// Translated from 00a90000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::LoadBinary` (Xbox PDB): a 5-byte jump to `00a71450` (engine map: `NiCamera::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiScreenSpaceCamera@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_space_camera_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A90000, &args![this, stream]);
}

// Translated from 00a90020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::SaveBinary` (Xbox PDB): a 5-byte jump to `00a716c0` (engine map: `NiCamera::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiScreenSpaceCamera@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_space_camera_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A90020, &args![this, stream]);
}

// Translated from 00a90030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::IsEqual` (Xbox PDB): a 5-byte jump to `00a6ff20` (engine map: `NiCamera::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiScreenSpaceCamera@@UAA_NPAVNiObject@@@Z`.
pub fn ni_screen_space_camera_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A90030, &args![this, other]).bool()
}

/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;

// Translated from 00a90180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetSize` of an `NiTArray`-shaped array of pointers
/// (`thiscall`, one stack word; the layout is [`NiTArray`]: 16-bit capacity,
/// used size and non-null count; the stream's group array is one). Does nothing when
/// `new_max` is the current capacity. When it is below the used size, the
/// elements from `new_max` on are cleared (the non-null count goes down for
/// each non-null one) and the used size becomes `new_max`. Then new storage of
/// `new_max & 0xffff` pointers (`00aa1070`) takes the used elements and its
/// other slots are zeroed, the old storage is freed (`00aa10f0`), and the
/// capacity is the low 16 bits of `new_max`; for `new_max` 0 the pointer is
/// null. All counts are 16-bit and wrap as the exe's do.
pub fn fn_00a90180(e: &mut Engine, this: Ptr<NiTArray>, new_max: u32) {
    if new_max == e.get(this, NiTArray::m_usMaxSize) as u32 {
        return;
    }
    let low = new_max as u16;
    if new_max < e.get(this, NiTArray::m_usSize) as u32 {
        let mut index = low;
        if index < e.get(this, NiTArray::m_usSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let slot = base.wrapping_add(index as u32 * 4);
                if e.mem.u32(slot) != 0 {
                    e.mem.set_u32(slot, 0);
                    let present = e.get(this, NiTArray::m_usESize);
                    e.set(this, NiTArray::m_usESize, present.wrapping_sub(1));
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        e.set(this, NiTArray::m_usSize, low);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, low);
    if new_max != 0 {
        let storage = e.call(ALLOCATE, &args![(low as u32) * 4]).u32();
        e.set(this, NiTArray::m_pBase, storage);
        let size = e.get(this, NiTArray::m_usSize);
        if size != 0 {
            let mut index = 0u16;
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let value = e.mem.u32(old.wrapping_add(index as u32 * 4));
                e.mem.set_u32(base.wrapping_add(index as u32 * 4), value);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        if index < e.get(this, NiTArray::m_usMaxSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                e.mem.set_u32(base.wrapping_add(index as u32 * 4), 0);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usMaxSize) {
                    break;
                }
            }
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    e.call(FREE, &args![old]);
}

/// `011f5e50`: the pointer `00a91290` stores the object it builds in.
const SCREEN_SPACE_OBJECT_SLOT: u32 = 0x011f_5e50;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00a91290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nioldscreenspace.cpp`; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x18 bytes from the memory manager, zeroes the
/// first four words, sets the words at `+0x10` and `+0x14` to 8 and stores the block in
/// `011f5e50` (null when the allocation fails).
pub fn fn_00a91290(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, block);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8ff70, ni_screen_geometry_get_rtti() -> Ptr),
        entry!(0x00a8ffb0, ni_screen_polygon_get_rtti() -> Ptr),
        entry!(0x00a90ab0, ni_screen_geometry_data_get_rtti() -> Ptr),
        entry!(0x00a91690, ni_screen_space_camera_get_rtti() -> Ptr),
        entry!(0x00a8_77d0, ni_screen_geometry_load_binary(Ptr, Ptr)),
        entry!(0x00a8_77e0, ni_screen_geometry_save_binary(Ptr, Ptr)),
        entry!(0x00a9_0000, ni_screen_space_camera_load_binary(Ptr, Ptr)),
        entry!(0x00a9_0020, ni_screen_space_camera_save_binary(Ptr, Ptr)),
        entry!(0x00a9_0030, ni_screen_space_camera_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_0180, fn_00a90180(Ptr<NiTArray>, u32)),
        entry!(0x00a9_1290, fn_00a91290()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_geometry_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8ff70, &args![]).u32(), NI_SCREEN_GEOMETRY_RTTI);
    }

    #[test]
    fn ni_screen_polygon_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8ffb0, &args![]).u32(), NI_SCREEN_POLYGON_RTTI);
    }

    #[test]
    fn ni_screen_geometry_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a90ab0, &args![]).u32(),
            NI_SCREEN_GEOMETRY_DATA_RTTI
        );
    }

    #[test]
    fn ni_screen_space_camera_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a91690, &args![]).u32(),
            NI_SCREEN_SPACE_CAMERA_RTTI
        );
    }

    #[test]
    fn thunk_00a877d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_77d0;
        const TARGET: u32 = 0x00a9_8ea0;
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
    fn thunk_00a877e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_77e0;
        const TARGET: u32 = 0x00a7_1d10;
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
    fn thunk_00a90000_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_0000;
        const TARGET: u32 = 0x00a7_1450;
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
    fn thunk_00a90020_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_0020;
        const TARGET: u32 = 0x00a7_16c0;
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
    fn thunk_00a90030_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_0030;
        const TARGET: u32 = 0x00a6_ff20;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    /// An array with room for `max` pointers holding `items`.
    fn array_fixture(max: u16, items: &[u32]) -> (Engine, Ptr<NiTArray>, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(max as u32 * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTArray::m_usESize, present as u16);
        (e, array, base)
    }

    #[test]
    fn fn_00a90180_does_nothing_for_the_current_capacity() {
        let (mut e, array, base) = array_fixture(4, &[0x10]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.get(array, NiTArray::m_pBase), base);
    }

    #[test]
    fn fn_00a90180_grows_copies_and_zeroes_the_rest() {
        let (mut e, array, base) = array_fixture(3, &[0x10, 0, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 6u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![24u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 6);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        let new = e.get(array, NiTArray::m_pBase);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(new + i * 4)).collect();
        assert_eq!(words, vec![0x10, 0, 0x30, 0, 0, 0]);
    }

    #[test]
    fn fn_00a90180_below_the_used_size_clears_the_cut_elements() {
        let (mut e, array, base) = array_fixture(4, &[0x10, 0x20, 0, 0x40]);
        e.call(0x00a9_0180, &args![array, 2u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        // The cut slots of the old block were cleared before the copy.
        assert_eq!(e.mem.u32(base + 12), 0);
        let new = e.get(array, NiTArray::m_pBase);
        assert_eq!([e.mem.u32(new), e.mem.u32(new + 4)], [0x10, 0x20]);
    }

    #[test]
    fn fn_00a90180_to_zero_drops_the_storage() {
        let (mut e, array, base) = array_fixture(2, &[0x10]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_with_two_sizes_of_eight() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00a9_1290, &args![]);
        assert_eq!(e.mem.u32(SCREEN_SPACE_OBJECT_SLOT), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, 0x1234);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a9_1290, &args![]);
        assert_eq!(e.mem.u32(SCREEN_SPACE_OBJECT_SLOT), 0);
    }
}
