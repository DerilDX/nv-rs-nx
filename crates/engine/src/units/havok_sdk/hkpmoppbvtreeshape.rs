//! `Havok SDK/hkpmoppbvtreeshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

layout! {
    /// `hkpMoppBvTreeShape` (Xbox PDB), 0x40 bytes; offsets checked against the code below.
    pub struct HkpMoppBvTreeShape: 0x40 {
        /// `m_code` (Xbox PDB): the `hkpMoppCode` the queries run on.
        0x14 m_code: Ptr,
        /// `m_moppData` (Xbox PDB).
        0x18 m_moppData: u32,
        /// `m_moppDataSize` (Xbox PDB).
        0x1c m_moppDataSize: u32,
        /// The shape of the `hkpSingleShapeContainer` (its vtable is at +0x30).
        0x34 m_childShape: Ptr,
    }
}

/// `hkpMoppObbVirtualMachine::queryObb` and `::queryAabb` (Xbox PDB): `this` is the virtual
/// machine on the caller's stack.
const MOPP_QUERY_OBB: u32 = 0x00dd_1bf0;
const MOPP_QUERY_AABB: u32 = 0x00dd_1da0;
/// `hkpMoppLongRayVirtualMachine::queryLongRay` and its collector overload (Xbox PDB), and
/// `hkpMoppRayBundleVirtualMachine::queryRayBundle`.
const MOPP_QUERY_LONG_RAY: u32 = 0x00dd_2df0;
const MOPP_QUERY_LONG_RAY_COLLECTOR: u32 = 0x00dd_2ed0;
const MOPP_QUERY_RAY_BUNDLE: u32 = 0x00dd_4870;
/// The double `16777216.0` the query scale is divided into.
const QUERY_RANGE: u32 = 0x0109_1c00;
/// Four ray-lane masks, one per bundle ray.
const RAY_LANE_MASKS: u32 = 0x010c_92e0;
/// Profiling labels: `"TtrcMopp"`, `"TtrcBundleMopp"` and the shared end label.
const LABEL_MOPP: u32 = 0x010d_0de4;
const LABEL_BUNDLE_MOPP: u32 = 0x010d_0df0;
const LABEL_END: u32 = 0x010c_41f8;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// The 0x30-byte code description the ray queries run on, built at `blob`: the shape's
/// `+0x20` vector at `+0x10`, then the MOPP data pointer at `+0x20`, the size twice (`+0x24`,
/// `+0x28`) and the type byte `2` at `+0x2C`.
fn build_code_blob(e: &mut Engine, this: Ptr<HkpMoppBvTreeShape>, blob: u32) {
    let info = e.mem.bytes(this.addr() + 0x20, 16);
    e.mem.write(blob + 0x10, &info);
    let data = e.get(this, HkpMoppBvTreeShape::m_moppData);
    let size = e.get(this, HkpMoppBvTreeShape::m_moppDataSize);
    e.mem.set_u32(blob + 0x20, data);
    e.mem.set_u32(blob + 0x24, size);
    e.mem.set_u32(blob + 0x28, size);
    e.mem.set_u8(blob + 0x2c, 2);
}

/// The container embedded in a child shape (`+0x10`), or 0 for no child.
fn child_container(child: Ptr) -> u32 {
    if child.addr() == 0 {
        0
    } else {
        child.addr().wrapping_add(0x10)
    }
}

/// The query box clamped to the shape: `max(info, box.min)` and
/// `min(info + 16777216.0 / info.w, box.max)`, per lane (`MAXPS`/`MINPS` return the second
/// operand unless the first is strictly greater/smaller). The quotient goes through the x87
/// stack as a double and is stored as a float.
fn clamped_box(e: &Engine, this: Ptr<HkpMoppBvTreeShape>, aabb: Ptr) -> (Vec4, Vec4) {
    let info = load_vec(e, this.addr() + 0x20);
    let scale = e.mem.f32(this.addr() + 0x2c);
    let step = (e.mem.f64(QUERY_RANGE) / f64::from(scale)) as f32;
    let min = load_vec(e, aabb.addr());
    let max = load_vec(e, aabb.addr() + 0x10);
    let lo = std::array::from_fn(|i| if info[i] > min[i] { info[i] } else { min[i] });
    let hi = std::array::from_fn(|i| {
        let high = step + info[i];
        if high < max[i] {
            high
        } else {
            max[i]
        }
    });
    (lo, hi)
}

// Translated from 00d54350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMoppBvTreeShapeBase::queryObb` (Xbox PDB): runs `hkpMoppObbVirtualMachine::queryObb`
/// with a virtual machine on the stack and `(m_code, a1, a2, a3, a4)`. The float `a3` goes
/// through the x87 stack.
pub fn query_obb(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    a1: u32,
    a2: u32,
    a3: f32,
    a4: u32,
) {
    let code = e.get(this, HkpMoppBvTreeShape::m_code);
    e.with_stack(0x40, |e, machine| {
        e.call(
            MOPP_QUERY_OBB,
            &args![machine, code, a1, a2, x87_quiet(a3).to_bits(), a4],
        );
    });
}

// Translated from 00d54380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMoppBvTreeShapeBase::queryAabb` (Xbox PDB): clamps the box `aabb` (min at `+0`, max at
/// `+0x10`) to the shape's range and runs `hkpMoppObbVirtualMachine::queryAabb` with
/// `(m_code, clamped box, output)`.
pub fn query_aabb(e: &mut Engine, this: Ptr<HkpMoppBvTreeShape>, aabb: Ptr, output: u32) {
    let (lo, hi) = clamped_box(e, this, aabb);
    let code = e.get(this, HkpMoppBvTreeShape::m_code);
    e.with_stack(0x60, |e, block| {
        let block = block.addr();
        store_vec(e, block, lo);
        store_vec(e, block + 0x10, hi);
        let machine = block + 0x20;
        e.call(MOPP_QUERY_AABB, &args![machine, code, block, output]);
    });
}

// Translated from 00d54420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppBvTreeShape::getAabbImpl` (Xbox PDB): asks the child shape (slot `+0x1C`) for the
/// AABB, passing the tolerance through the x87 stack.
pub fn get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    transform: u32,
    tolerance: f32,
    out: u32,
) {
    let child = e.get(this, HkpMoppBvTreeShape::m_childShape);
    e.vcall(
        child.addr(),
        0x1c,
        &args![transform, x87_quiet(tolerance).to_bits(), out],
    );
}

// Translated from 00d54520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppBvTreeShape::castRayImpl` (Xbox PDB; hidden result pointer `result`, which is
/// returned): runs `hkpMoppLongRayVirtualMachine::queryLongRay` for the ray `input` against
/// the child with the output's child depth (`output + 0x40`) raised, and on a hit sets the
/// child key at `output + 0x20 + depth * 4` to 0. Brackets the work with the `"TtrcMopp"`
/// and end profiling records.
pub fn cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    result: Ptr,
    input: u32,
    output: Ptr,
) -> Ptr {
    profile_record(e, LABEL_MOPP);
    let depth = e.mem.u32(output.addr() + 0x40);
    e.mem.set_u32(output.addr() + 0x40, depth.wrapping_add(1));
    let child = e.get(this, HkpMoppBvTreeShape::m_childShape);
    e.with_stack(0xc0, |e, frame| {
        let frame = frame.addr();
        let blob = frame + 0x10;
        build_code_blob(e, this, blob);
        let machine = frame + 0x50;
        e.mem.set_u32(frame + 0x90, 0);
        e.mem.set_u32(frame + 0x94, 0);
        e.call(
            MOPP_QUERY_LONG_RAY,
            &args![machine, result, child_container(child), blob, input, output],
        );
    });
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    if e.mem.u8(result.addr()) != 0 {
        e.mem
            .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), 0);
    }
    profile_record(e, LABEL_END);
    result
}

// Translated from 00d54610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppBvTreeShape::castRayBundleImpl` (Xbox PDB; hidden result pointer `result`, a
/// 16-byte hit mask that is returned): raises the child depths of the four rays' outputs
/// (`output + 0x40 + 0x50 * lane`), runs `hkpMoppRayBundleVirtualMachine::queryRayBundle` and
/// lowers the depths again; for each ray whose mask bit (`RAY_LANE_MASKS[lane]`) is set in the
/// sign bits of the result, the child key at `depth * 4 + 0x20 - 0x40` from its depth field
/// is set to 0. Brackets the work with the `"TtrcBundleMopp"` and end profiling records.
pub fn cast_ray_bundle_impl(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    result: Ptr,
    input: u32,
    output: Ptr,
    extra: u32,
) -> Ptr {
    profile_record(e, LABEL_BUNDLE_MOPP);
    for lane in 0..4 {
        let depth_field = output.addr() + 0x40 + 0x50 * lane;
        let depth = e.mem.u32(depth_field);
        e.mem.set_u32(depth_field, depth.wrapping_add(1));
    }
    let child = e.get(this, HkpMoppBvTreeShape::m_childShape);
    e.with_stack(0x130, |e, frame| {
        let frame = frame.addr();
        let blob = frame + 0x10;
        build_code_blob(e, this, blob);
        let machine = frame + 0x50;
        e.call(
            MOPP_QUERY_RAY_BUNDLE,
            &args![
                machine,
                result,
                child_container(child),
                blob,
                input,
                output,
                extra
            ],
        );
    });
    for lane in 0..4 {
        let depth_field = output.addr() + 0x40 + 0x50 * lane;
        let depth = e.mem.u32(depth_field).wrapping_sub(1);
        e.mem.set_u32(depth_field, depth);
        let signs = (0..4).fold(0u32, |mask, i| {
            mask | ((e.mem.u32(result.addr() + 4 * i) >> 31) << i)
        });
        if e.mem.u32(RAY_LANE_MASKS + 4 * lane) & signs != 0 {
            let slot = depth_field
                .wrapping_add(depth.wrapping_mul(4))
                .wrapping_sub(0x20);
            e.mem.set_u32(slot, 0);
        }
    }
    profile_record(e, LABEL_END);
    result
}

// Translated from 00d54720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppBvTreeShape::castRayWithCollectorImpl` (Xbox PDB): runs the collector overload of
/// `hkpMoppLongRayVirtualMachine::queryLongRay` for the ray `input` against the child, with a
/// stack record `(child, 0, body[2], body)` standing for the body. Brackets the work with
/// the `"TtrcMopp"` and end profiling records.
pub fn cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    input: u32,
    body: Ptr,
    collector: u32,
) {
    profile_record(e, LABEL_MOPP);
    let child = e.get(this, HkpMoppBvTreeShape::m_childShape);
    let body_field = e.mem.u32(body.addr() + 8);
    e.with_stack(0xd0, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x10, child.addr());
        e.mem.set_u32(frame + 0x14, 0);
        e.mem.set_u32(frame + 0x18, body_field);
        e.mem.set_u32(frame + 0x1c, body.addr());
        let blob = frame + 0x30;
        build_code_blob(e, this, blob);
        let machine = frame + 0x60;
        e.mem.set_u32(frame + 0xa0, 0);
        e.mem.set_u32(frame + 0xa4, 0);
        e.call(
            MOPP_QUERY_LONG_RAY_COLLECTOR,
            &args![
                machine,
                child_container(child),
                blob,
                input,
                frame + 0x10,
                collector
            ],
        );
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d54810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMoppBvTreeShapeBase::queryAabbImpl` (Xbox PDB): clamps the box `aabb` as `queryAabb`
/// does and runs `hkpMoppObbVirtualMachine::queryAabb` on a code description built on the
/// stack, collecting hits in a stack `hkArray` that starts as `(buffer, 0, capacity |
/// don't-free flag)`. Returns the hit count; afterwards the array's block is freed through
/// the memory router's allocator (slot `+0x10`) unless the flag is still set.
pub fn query_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpMoppBvTreeShape>,
    aabb: Ptr,
    buffer: u32,
    capacity: u32,
) -> u32 {
    let (lo, hi) = clamped_box(e, this, aabb);
    e.with_stack(0xc0, |e, frame| {
        let frame = frame.addr();
        let array = frame + 0x10;
        e.mem.set_u32(array, buffer);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        store_vec(e, frame + 0x20, lo);
        store_vec(e, frame + 0x30, hi);
        let machine = frame + 0x40;
        let blob = frame + 0x80;
        build_code_blob(e, this, blob);
        e.call(MOPP_QUERY_AABB, &args![machine, blob, frame + 0x20, array]);
        let list = e.mem.u32(machine);
        let count = e.mem.u32(list + 4);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(array + 4, 0);
        let flags = e.mem.u32(array + 8);
        if flags as i32 >= 0 {
            let allocator = e.mem.u32(router + 0x10);
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (flags & 0x3fff_ffff) * 4]);
        }
        count
    })
}

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d54930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d5_43e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d54930(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d5_43e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_4350,
            query_obb(Ptr<HkpMoppBvTreeShape>, u32, u32, f32, u32)
        ),
        entry!(0x00d5_4380, query_aabb(Ptr<HkpMoppBvTreeShape>, Ptr, u32)),
        entry!(
            0x00d5_4420,
            get_aabb_impl(Ptr<HkpMoppBvTreeShape>, u32, f32, u32)
        ),
        entry!(
            0x00d5_4520,
            cast_ray_impl(Ptr<HkpMoppBvTreeShape>, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00d5_4610,
            cast_ray_bundle_impl(Ptr<HkpMoppBvTreeShape>, Ptr, u32, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d5_4720,
            cast_ray_with_collector_impl(Ptr<HkpMoppBvTreeShape>, u32, Ptr, u32)
        ),
        entry!(
            0x00d5_4810,
            query_aabb_impl(Ptr<HkpMoppBvTreeShape>, Ptr, u32, u32) -> u32
        ),
        entry!(0x00d5_4930, deleting_destructor_00d54930(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d54930_frees_only_when_bit_zero_is_set() {
        check(0x00d5_4930, 0x00d5_43e0);
    }

    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};

    const CHILD_DOUBLE: u32 = 0x0900_0300;
    const ALLOCATOR_FREE: u32 = 0x0900_0310;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An engine with the profiling stream installed and the data the code reads mapped.
    fn mopp_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e.mem.set_f64(QUERY_RANGE, 16777216.0);
        e
    }

    /// A shape with code `0x1111`, MOPP data `0x2222` of size `0x33`, the vector
    /// (1, 2, 3, 2) and a child whose vtable slot `slot` is `CHILD_DOUBLE`.
    fn shape(e: &mut Engine, slot: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, CHILD_DOUBLE);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child, vtable);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0x14, 0x1111);
        e.mem.set_u32(shape + 0x18, 0x2222);
        e.mem.set_u32(shape + 0x1c, 0x33);
        store_vec(e, shape + 0x20, [1.0, 2.0, 3.0, 2.0]);
        e.mem.set_u32(shape + 0x34, child);
        (shape, child)
    }

    /// Registers a double at `address` that logs its arguments and the 16 words at the pointer
    /// in argument `deref` (when there is one).
    fn logger(e: &mut Engine, address: u32, deref: Option<usize>) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            if let Some(index) = deref {
                words.extend((0..16).map(|i| e.mem.u32(a[index] + 4 * i)));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        calls
    }

    #[test]
    fn query_obb_passes_the_code_and_the_quieted_float() {
        let mut e = mopp_engine();
        let (shape, _) = shape(&mut e, 0x1c);
        let calls = logger(&mut e, MOPP_QUERY_OBB, None);
        e.call(
            0x00d5_4350,
            &args![shape, 0x11u32, 0x22u32, 0x7fa0_0000u32, 0x44u32],
        );
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1..], [0x1111, 0x11, 0x22, 0x7fe0_0000, 0x44]);
    }

    #[test]
    fn query_aabb_clamps_the_box_to_the_shape_range() {
        let mut e = mopp_engine();
        let (shape, _) = shape(&mut e, 0x1c);
        let calls = logger(&mut e, MOPP_QUERY_AABB, Some(2));
        let aabb = e.mem.alloc(0x20);
        store_vec(&mut e, aabb, [0.0, 5.0, -1.0, 10.0]);
        store_vec(&mut e, aabb + 0x10, [1.0e9, 1.0, 100.0, 3.0]);
        e.call(0x00d5_4380, &args![shape, aabb, 0x55u32]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        let call = &calls[0];
        assert_eq!((call[1], call[3]), (0x1111, 0x55));
        let floats: Vec<f32> = call[4..12].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(floats[0..4], [1.0, 5.0, 3.0, 10.0]);
        // 16777216 / 2 = 8388608 added to the vector, then clamped to the maximum.
        assert_eq!(floats[4..8], [8388609.0, 1.0, 100.0, 3.0]);
    }

    #[test]
    fn get_aabb_impl_forwards_to_the_child_with_the_quieted_float() {
        let mut e = mopp_engine();
        let (shape, child) = shape(&mut e, 0x1c);
        let calls = logger(&mut e, CHILD_DOUBLE, None);
        e.call(0x00d5_4420, &args![shape, 0x10u32, 0x7fa0_0000u32, 0x30u32]);
        assert_eq!(*calls.borrow(), vec![vec![child, 0x10, 0x7fe0_0000, 0x30]]);
    }

    #[test]
    fn cast_ray_impl_clears_the_key_on_a_hit_only() {
        let mut e = mopp_engine();
        let (shape, child) = shape(&mut e, 0x1c);
        let seen: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let hit = Rc::new(RefCell::new(1u8));
        let hit_value = Rc::clone(&hit);
        e.register_double(MOPP_QUERY_LONG_RAY, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..12).map(|i| e.mem.u32(a[3] + 4 * i)));
            words.push(e.mem.u32(a[5] + 0x40));
            words.extend([e.mem.u32(a[0] + 0x40), e.mem.u32(a[0] + 0x44)]);
            log.borrow_mut().push(words);
            e.mem.set_u8(a[1], *hit_value.borrow());
            Ret::default()
        });
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + 0x40, 1);
        e.mem.set_u32(output + 0x24, 9);
        let result = e.mem.alloc(4);
        let got = e
            .call(0x00d5_4520, &args![shape, result, 0x77u32, output])
            .u32();
        assert_eq!(got, result);
        assert_eq!(e.mem.u32(output + 0x40), 1);
        assert_eq!(e.mem.u32(output + 0x24), 0);
        {
            let seen = seen.borrow();
            let call = &seen[0];
            assert_eq!(call[1..3], [result, child + 0x10]);
            assert_eq!(call[4..6], [0x77, output]);
            // The code description: vector, data pointer, size twice, type byte.
            assert_eq!(f32::from_bits(call[6 + 4]), 1.0);
            assert_eq!(f32::from_bits(call[6 + 7]), 2.0);
            assert_eq!(call[6 + 8..6 + 11], [0x2222, 0x33, 0x33]);
            assert_eq!(e.mem.u8(call[3] + 0x2c), 2);
            assert_eq!(call[6 + 12], 2);
            assert_eq!(call[6 + 13..6 + 15], [0, 0]);
        }
        // A miss leaves the key; no child passes a null container.
        *hit.borrow_mut() = 0;
        e.mem.set_u32(output + 0x24, 9);
        e.mem.set_u32(shape + 0x34, 0);
        e.call(0x00d5_4520, &args![shape, result, 0x77u32, output]);
        assert_eq!(e.mem.u32(output + 0x24), 9);
        assert_eq!(seen.borrow()[1][2], 0);
    }

    #[test]
    fn cast_ray_bundle_impl_clears_the_keys_of_the_rays_that_hit() {
        let mut e = mopp_engine();
        let (shape, child) = shape(&mut e, 0x1c);
        for (lane, mask) in [1u32, 2, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(RAY_LANE_MASKS + 4 * lane as u32, mask);
        }
        let output = e.mem.alloc(0x180);
        for lane in 0..4u32 {
            e.mem.set_u32(output + 0x40 + 0x50 * lane, lane + 1);
        }
        for slot in [0x24, 0x78, 0xcc, 0x120] {
            e.mem.set_u32(output + slot, 9);
        }
        let result = e.mem.alloc(0x10);
        let depths = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&depths);
        e.register_double(MOPP_QUERY_RAY_BUNDLE, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|lane| e.mem.u32(a[5] + 0x40 + 0x50 * lane)));
            log.borrow_mut().push(words);
            // Lanes 0 and 2 have their sign bits set.
            e.mem.set_u32(a[1], 0x8000_0000);
            e.mem.set_u32(a[1] + 8, 0xbf80_0000);
            Ret::default()
        });
        let got = e
            .call(0x00d5_4610, &args![shape, result, 0x77u32, output, 0x88u32])
            .u32();
        assert_eq!(got, result);
        let depths = depths.borrow();
        assert_eq!(depths[0][1], result);
        assert_eq!(depths[0][2], child + 0x10);
        assert_eq!(depths[0][4..7], [0x77, output, 0x88]);
        assert_eq!(depths[0][7..11], [2, 3, 4, 5]);
        for lane in 0..4u32 {
            assert_eq!(e.mem.u32(output + 0x40 + 0x50 * lane), lane + 1);
        }
        let keys: Vec<u32> = [0x24, 0x78, 0xcc, 0x120]
            .iter()
            .map(|slot| e.mem.u32(output + slot))
            .collect();
        assert_eq!(keys, vec![0, 9, 0, 9]);
    }

    #[test]
    fn cast_ray_with_collector_impl_describes_the_body_on_the_stack() {
        let mut e = mopp_engine();
        let (shape, child) = shape(&mut e, 0x1c);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(MOPP_QUERY_LONG_RAY_COLLECTOR, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|i| e.mem.u32(a[4] + 4 * i)));
            words.extend((0..4).map(|i| e.mem.u32(a[2] + 0x20 + 4 * i)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 8, 0xb0b0);
        e.call(0x00d5_4720, &args![shape, 0x77u32, body, 0x99u32]);
        let seen = seen.borrow();
        let call = &seen[0];
        assert_eq!(call[1], child + 0x10);
        assert_eq!((call[3], call[5]), (0x77, 0x99));
        assert_eq!(call[6..10], [child, 0, 0xb0b0, body]);
        assert_eq!(call[10..13], [0x2222, 0x33, 0x33]);
    }

    #[test]
    fn query_aabb_impl_returns_the_count_and_frees_an_owned_array() {
        let mut e = mopp_engine();
        let (shape, _) = shape(&mut e, 0x1c);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let freed = logger(&mut e, ALLOCATOR_FREE, None);
        let owned = Rc::new(RefCell::new(false));
        let owned_flag = Rc::clone(&owned);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(MOPP_QUERY_AABB, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|i| e.mem.u32(a[2] + 4 * i)));
            words.extend((0..3).map(|i| e.mem.u32(a[3] + 4 * i)));
            words.extend([e.mem.u32(a[1] + 0x20), e.mem.u32(a[1] + 0x2c)]);
            log.borrow_mut().push(words);
            e.mem.set_u32(a[0], a[3]);
            e.mem.set_u32(a[3] + 4, 5);
            if *owned_flag.borrow() {
                e.mem.set_u32(a[3], 0x7777);
                e.mem.set_u32(a[3] + 8, 6);
            }
            Ret::default()
        });
        let aabb = e.mem.alloc(0x20);
        store_vec(&mut e, aabb, [0.0, 5.0, -1.0, 10.0]);
        store_vec(&mut e, aabb + 0x10, [1.0e9, 1.0, 100.0, 3.0]);
        // Borrowed buffer: the flag stays set and nothing is freed.
        let count = e
            .call(0x00d5_4810, &args![shape, aabb, 0x6000u32, 8u32])
            .u32();
        assert_eq!(count, 5);
        assert!(freed.borrow().is_empty());
        {
            let seen = seen.borrow();
            let call = &seen[0];
            assert_ne!(call[0], 0);
            assert_eq!(f32::from_bits(call[4]), 1.0);
            assert_eq!(f32::from_bits(call[8]), 8388609.0);
            assert_eq!(call[12..15], [0x6000, 0, 0x8000_0008]);
            assert_eq!(call[15], 0x2222);
            assert_eq!(call[16] & 0xff, 2);
        }
        // Owned array: its block (6 elements of 4 bytes) is freed through the allocator.
        *owned.borrow_mut() = true;
        let count = e
            .call(0x00d5_4810, &args![shape, aabb, 0x6000u32, 8u32])
            .u32();
        assert_eq!(count, 5);
        assert_eq!(*freed.borrow(), vec![vec![allocator, 0x7777, 24]]);
    }
}
