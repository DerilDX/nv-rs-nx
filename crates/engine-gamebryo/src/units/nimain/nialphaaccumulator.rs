//! `NiMain/nialphaaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAlphaAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_ALPHA_ACCUMULATOR_RTTI: u32 = 0x011f_5f74;

// Translated from 00a9b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_alpha_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ALPHA_ACCUMULATOR_RTTI)
}

/// `00aa0ac0`: the jump target of `00a9b4f0`.
const JUMP_TARGET_00A9B4F0: u32 = 0x00aa_0ac0;

/// `00aa0ad0`: the jump target of `00a9b500`.
const JUMP_TARGET_00A9B500: u32 = 0x00aa_0ad0;

// Translated from 00a9b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::LoadBinary` (Xbox PDB): a 5-byte jump to `00aa0ac0` (engine map: `NiKeyBasedInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiAlphaAccumulator@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B4F0, &args![this, stream]);
}

// Translated from 00a9b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00aa0ad0`, passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaAccumulator@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B500, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b530, ni_alpha_accumulator_get_rtti() -> Ptr),
        entry!(0x00a9_b4f0, ni_alpha_accumulator_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b500, ni_alpha_accumulator_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9b570, ni_alpha_accumulator_sort(Ptr)),
    ]
}

/// `NiAccumulator::m_pkCamera` (Xbox PDB) +8.
const CAMERA: u32 = 0x8;
/// `NiBackToFrontAccumulator::m_kItems` (Xbox PDB) +0xC: the inline list.
const ITEMS: u32 = 0xc;
/// `NiBackToFrontAccumulator::m_kItemsToSortAndRender` (Xbox PDB) +0x18.
const ITEMS_TO_SORT: u32 = 0x18;
/// `NiBackToFrontAccumulator::m_iNumItems` (Xbox PDB) +0x1C.
const NUM_ITEMS: u32 = 0x1c;
/// `NiBackToFrontAccumulator::m_iMaxItems` (Xbox PDB) +0x20.
const MAX_ITEMS: u32 = 0x20;
/// `NiBackToFrontAccumulator::m_ppkItems` (Xbox PDB) +0x24.
const ITEM_ARRAY: u32 = 0x24;
/// `NiBackToFrontAccumulator::m_pfDepths` (Xbox PDB) +0x28.
const DEPTHS: u32 = 0x28;
/// `NiAlphaAccumulator::m_bSortByClosestPoint` (Xbox PDB) +0x31.
const SORT_BY_CLOSEST_POINT: u32 = 0x31;
/// `NiAlphaAccumulator::m_bInterfaceSort` (Xbox PDB) +0x32.
const INTERFACE_SORT: u32 = 0x32;
/// The camera's world direction (`NiCamera`, `NiAVObject::m_kWorld`'s
/// rotation column the code reads at +0x68, +0x74, +0x80).
const CAMERA_DIRECTION_X: u32 = 0x68;
const CAMERA_DIRECTION_Y: u32 = 0x74;
const CAMERA_DIRECTION_Z: u32 = 0x80;
/// The geometry's world bound pointer at +0x20 (`NiAVObject`), and the
/// float at +0x90 that the interface sort uses as the depth.
const GEOMETRY_BOUND: u32 = 0x20;
const GEOMETRY_INTERFACE_DEPTH: u32 = 0x90;
/// The bound used when a geometry has none (`NiBound`: centre, radius).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// `NiFree` (`00aa10f0`, `cdecl`) and `NiAlloc` (`00aa1070`, `cdecl`).
const NI_FREE: u32 = 0x00aa_10f0;
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiBackToFrontAccumulator::SortObjectsByDepth` (`00aa0b40`, `RET 8`).
const SORT_OBJECTS_BY_DEPTH: u32 = 0x00aa_0b40;

// Translated from 00a9b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::Sort` (Xbox PDB): points `m_kItemsToSortAndRender`
/// at the own list when it is null, takes its count as `m_iNumItems`, and
/// with items: grows the item and depth arrays to the count when it is above
/// `m_iMaxItems` (signed; both freed and allocated again), copies the list's
/// geometries into the item array in order, gives each a depth and sorts
/// `[0, count - 1]` with `SortObjectsByDepth`. The depth is the geometry's
/// float at +0x90 for an interface sort; otherwise the world bound's centre
/// (the default bound when null) dotted with the camera's direction
/// (`x*dx`, `y*dy`, `z*dz`, added as `(y*dy + x*dx) + z*dz`, rounded to a
/// float), less the bound's radius when sorting by closest point. A
/// non-positive count copies nothing but still sorts.
pub fn ni_alpha_accumulator_sort(e: &mut Engine, this: Ptr) {
    let at = |offset: u32| this.addr().wrapping_add(offset);
    if e.mem.u32(at(ITEMS_TO_SORT)) == 0 {
        e.mem.set_u32(at(ITEMS_TO_SORT), at(ITEMS));
    }
    let list = e.mem.u32(at(ITEMS_TO_SORT));
    // NiTPointerList::m_uiCount +8.
    let count = e.mem.i32(list.wrapping_add(8));
    e.mem.set_i32(at(NUM_ITEMS), count);
    if count == 0 {
        return;
    }
    if count > e.mem.i32(at(MAX_ITEMS)) {
        let items = e.mem.u32(at(ITEM_ARRAY));
        e.call(NI_FREE, &args![items]);
        let count = e.mem.u32(at(NUM_ITEMS));
        e.mem.set_u32(at(MAX_ITEMS), count);
        let items = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
        let depths = e.mem.u32(at(DEPTHS));
        e.mem.set_u32(at(ITEM_ARRAY), items);
        e.call(NI_FREE, &args![depths]);
        let max = e.mem.u32(at(MAX_ITEMS));
        let depths = e.call(NI_ALLOC, &args![max.wrapping_mul(4)]).u32();
        e.mem.set_u32(at(DEPTHS), depths);
    }
    let camera = e.mem.u32(at(CAMERA));
    let direction_x = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_X)));
    // NiTPointerList::m_pkHead +0.
    let mut node = e.mem.u32(e.mem.u32(at(ITEMS_TO_SORT)));
    let direction_y = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_Y)));
    let direction_z = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_Z)));
    let interface_sort = e.mem.u8(at(INTERFACE_SORT)) != 0;
    let closest_point = e.mem.u8(at(SORT_BY_CLOSEST_POINT)) != 0;
    let mut index = 0i32;
    while index < e.mem.i32(at(NUM_ITEMS)) {
        let slot = (index as u32).wrapping_mul(4);
        // NiTListItem: next +0, element +8.
        let element = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        let items = e.mem.u32(at(ITEM_ARRAY));
        e.mem.set_u32(items.wrapping_add(slot), element);
        let geometry = e.mem.u32(e.mem.u32(at(ITEM_ARRAY)).wrapping_add(slot));
        let depth = if interface_sort {
            x87_quiet_bits(e.mem.u32(geometry.wrapping_add(GEOMETRY_INTERFACE_DEPTH)))
        } else {
            let mut bound = e.mem.u32(geometry.wrapping_add(GEOMETRY_BOUND));
            if bound == 0 {
                bound = DEFAULT_BOUND;
            }
            let y = x87_mul(x87_load32(e.mem.u32(bound.wrapping_add(4))), direction_y);
            let x = x87_mul(x87_load32(e.mem.u32(bound)), direction_x);
            let sum = x87_add(y, x);
            let z = x87_mul(x87_load32(e.mem.u32(bound.wrapping_add(8))), direction_z);
            let dot = x87_store32(x87_add(sum, z));
            if closest_point {
                let radius = x87_m32(e.mem.u32(bound.wrapping_add(0xc)));
                x87_store32(x87_sub(x87_load32(dot), radius))
            } else {
                dot
            }
        };
        let depths = e.mem.u32(at(DEPTHS));
        e.mem.set_u32(depths.wrapping_add(slot), depth);
        index += 1;
    }
    let last = e.mem.u32(at(NUM_ITEMS)).wrapping_sub(1);
    e.call(SORT_OBJECTS_BY_DEPTH, &args![this, 0u32, last]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_alpha_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9b530, &args![]).u32(),
            NI_ALPHA_ACCUMULATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a9b4f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b4f0;
        const TARGET: u32 = 0x00aa_0ac0;
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
    fn thunk_00a9b500_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b500;
        const TARGET: u32 = 0x00aa_0ad0;
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
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An accumulator with two geometries in its own list, the camera looking
    /// along (1, 2, 3).
    fn setup(e: &mut Engine, interface: bool, closest: bool) -> (u32, [u32; 2]) {
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(NI_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(SORT_OBJECTS_BY_DEPTH, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        let camera = e.mem.alloc(0x90);
        e.mem.set_f32(camera + 0x68, 1.0);
        e.mem.set_f32(camera + 0x74, 2.0);
        e.mem.set_f32(camera + 0x80, 3.0);
        e.mem.set_u32(this + 8, camera);
        e.mem.set_u8(this + 0x31, u8::from(closest));
        e.mem.set_u8(this + 0x32, u8::from(interface));
        let bound = e.mem.alloc(0x10);
        for (i, v) in [4.0f32, 5.0, 6.0, 0.5].iter().enumerate() {
            e.mem.set_f32(bound + 4 * i as u32, *v);
        }
        let with_bound = e.mem.alloc(0x94);
        e.mem.set_u32(with_bound + 0x20, bound);
        e.mem.set_f32(with_bound + 0x90, 7.0);
        let without_bound = e.mem.alloc(0x94);
        e.mem.set_f32(without_bound + 0x90, 8.0);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, without_bound);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, with_bound);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(this + 0x14, 2);
        // The default bound: centre (1, 1, 1), radius 2.
        e.map(DEFAULT_BOUND, 0x10);
        for (i, v) in [1.0f32, 1.0, 1.0, 2.0].iter().enumerate() {
            e.mem.set_f32(DEFAULT_BOUND + 4 * i as u32, *v);
        }
        (this, [with_bound, without_bound])
    }

    #[test]
    fn sort_gives_each_item_its_depth_and_sorts_them() {
        // (interface, closest point, expected depths)
        for (interface, closest, expected) in [
            (false, false, [32.0f32, 6.0]),
            (false, true, [31.5, 4.0]),
            (true, false, [7.0, 8.0]),
        ] {
            let mut e = Engine::new();
            let (this, geometries) = setup(&mut e, interface, closest);
            e.call_log = Some(vec![]);
            e.call(0x00a9b570, &args![this]);
            assert_eq!(e.mem.u32(this + 0x18), this + 0xc);
            assert_eq!(e.mem.u32(this + 0x1c), 2);
            assert_eq!(e.mem.u32(this + 0x20), 2);
            let items = e.mem.u32(this + 0x24);
            assert_eq!([e.mem.u32(items), e.mem.u32(items + 4)], geometries);
            let depths = e.mem.u32(this + 0x28);
            assert_eq!([e.mem.f32(depths), e.mem.f32(depths + 4)], expected);
            assert_eq!(calls(&e, NI_ALLOC), vec![vec![8], vec![8]]);
            assert_eq!(calls(&e, SORT_OBJECTS_BY_DEPTH), vec![vec![this, 0, 1]]);
        }
    }

    #[test]
    fn sort_keeps_big_enough_arrays_and_stops_on_an_empty_list() {
        let mut e = Engine::new();
        let (this, _) = setup(&mut e, true, false);
        let items = e.mem.alloc(0x10);
        let depths = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x20, 4);
        e.mem.set_u32(this + 0x24, items);
        e.mem.set_u32(this + 0x28, depths);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert!(calls(&e, NI_ALLOC).is_empty());
        assert_eq!(e.mem.u32(this + 0x24), items);
        assert_eq!(e.mem.f32(depths + 4), 8.0);

        // An empty list: no sort at all.
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert!(calls(&e, SORT_OBJECTS_BY_DEPTH).is_empty());

        // A negative count copies nothing and still sorts.
        e.mem.set_i32(this + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert_eq!(
            calls(&e, SORT_OBJECTS_BY_DEPTH),
            vec![vec![this, 0, 0xffff_fffe]]
        );
    }
}
