//! `Havok SDK/hkpsampledheightfieldshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d262e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_4660`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d262e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_4660, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Mask clearing the sign bit of every lane (`.rdata`), used as `ANDPS`.
const ABS_MASK_01017D10: u32 = 0x0101_7d10;
/// Mask holding the sign bit of every lane (`.rdata`), used as `XORPS`.
const SIGN_MASK_010C44D0: u32 = 0x010c_44d0;

// Translated from 00d26230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbUtil::calcAabb_ov3` (map name). cdecl, five words, SSE (`f32` per
/// lane): the box of an oriented box with half extents `half` and centre
/// `centre` under the transform whose columns are the four vectors at
/// `transform` (`+0`, `+0x10`, `+0x20` and the translation at `+0x30`).
/// With `a0`, `a1`, `a2` the first three columns, per lane:
/// `extent = (|hx*a0| + |hy*a1|) + (|hz*a2| + tolerance)` and
/// `middle = (cx*a0 + cy*a1) + (cz*a2 + translation)`; `out[0..4]` gets
/// `-extent + middle` (the sign flipped by the mask at `010c44d0`) and
/// `out[4..8]` gets `middle + extent`.
pub fn hk_aabb_util_calc_aabb_ov3(
    e: &mut Engine,
    transform: Ptr,
    half: Ptr,
    centre: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let lane = |e: &Engine, base: u32, i: u32| e.mem.f32(base + 4 * i);
    let abs = |e: &Engine, value: f32, i: u32| {
        f32::from_bits(value.to_bits() & e.mem.u32(ABS_MASK_01017D10 + 4 * i))
    };
    let (half_x, half_y, half_z) = (
        lane(e, half.addr(), 0),
        lane(e, half.addr(), 1),
        lane(e, half.addr(), 2),
    );
    let (centre_x, centre_y, centre_z) = (
        lane(e, centre.addr(), 0),
        lane(e, centre.addr(), 1),
        lane(e, centre.addr(), 2),
    );
    let mut minimum = [0.0f32; 4];
    let mut maximum = [0.0f32; 4];
    for i in 0..4u32 {
        let (a0, a1, a2) = (
            lane(e, transform.addr(), i),
            lane(e, transform.addr() + 0x10, i),
            lane(e, transform.addr() + 0x20, i),
        );
        let translation = lane(e, transform.addr() + 0x30, i);
        let extent = (abs(e, half_x * a0, i) + abs(e, half_y * a1, i))
            + (abs(e, half_z * a2, i) + tolerance);
        let middle = (centre_x * a0 + centre_y * a1) + (centre_z * a2 + translation);
        let negated = f32::from_bits(extent.to_bits() ^ e.mem.u32(SIGN_MASK_010C44D0 + 4 * i));
        minimum[i as usize] = negated + middle;
        maximum[i as usize] = middle + extent;
    }
    for (i, value) in maximum.iter().enumerate() {
        e.mem.set_f32(out.addr() + 0x10 + 4 * i as u32, *value);
    }
    for (i, value) in minimum.iter().enumerate() {
        e.mem.set_f32(out.addr() + 4 * i as u32, *value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d26230,
            hk_aabb_util_calc_aabb_ov3(Ptr, Ptr, Ptr, f32, Ptr)
        ),
        entry!(0x00d2_62e0, deleting_destructor_00d262e0(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00d262e0_frees_only_when_bit_zero_is_set() {
        check(0x00d2_62e0, 0x00d2_4660);
    }

    #[test]
    fn hk_aabb_util_calc_aabb_ov3_boxes_a_rotated_box() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x010c_4000, 0x1000);
        for i in 0..4 {
            e.mem.set_u32(ABS_MASK_01017D10 + 4 * i, 0x7fff_ffff);
            e.mem.set_u32(SIGN_MASK_010C44D0 + 4 * i, 0x8000_0000);
        }
        // A quarter turn about z (x axis -> y, y axis -> -x) and a translation.
        let transform = e.mem.alloc(0x40);
        let columns: [[f32; 4]; 4] = [
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 20.0, 30.0, 0.0],
        ];
        for (c, column) in columns.iter().enumerate() {
            for (i, value) in column.iter().enumerate() {
                e.mem
                    .set_f32(transform + 0x10 * c as u32 + 4 * i as u32, *value);
            }
        }
        let half = e.mem.alloc(16);
        let centre = e.mem.alloc(16);
        for (i, (h, c)) in [(1.0f32, 0.5f32), (2.0, 0.0), (3.0, 0.0)]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(half + 4 * i as u32, *h);
            e.mem.set_f32(centre + 4 * i as u32, *c);
        }
        let out = e.mem.alloc(0x20);
        e.call(
            0x00d2_6230,
            &args![
                Ptr::<()>::new(transform),
                Ptr::<()>::new(half),
                Ptr::<()>::new(centre),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        let read = |offset: u32| [0, 4, 8].map(|o| e.mem.f32(out + offset + o));
        // The x half extent 1 maps onto y, the y half extent 2 onto x.
        // extent = (2.25, 1.25, 3.25); middle = (10, 20.5, 30).
        assert_eq!(read(0), [7.75, 19.25, 26.75]);
        assert_eq!(read(0x10), [12.25, 21.75, 33.25]);
    }
}
