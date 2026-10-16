//! `BSHavok/bhkcharcontrollershape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cd7d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCharControllerShape::getAabb` (map name, taken from a vtable).
/// Thiscall, three words (`RET 0xc`), SSE (`f32` per lane). With the
/// transform column `+0x30` of `other` and of `this` summed into the centre
/// and `radius` added to the half extents at `this + 0x20`, writes
/// `centre - extents` to `out[0..4]` and `extents + centre` to `out[4..8]`.
pub fn hk_char_controller_shape_get_aabb(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    radius: f32,
    out: Ptr,
) {
    for lane in 0..4u32 {
        let centre =
            e.mem.f32(other.addr() + 0x30 + 4 * lane) + e.mem.f32(this.addr() + 0x30 + 4 * lane);
        let extent = radius + e.mem.f32(this.addr() + 0x20 + 4 * lane);
        e.mem.set_f32(out.addr() + 4 * lane, centre - extent);
        e.mem.set_f32(out.addr() + 0x10 + 4 * lane, extent + centre);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cd7d20,
        hk_char_controller_shape_get_aabb(Ptr, Ptr, f32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hk_char_controller_shape_get_aabb_writes_min_then_max() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        for lane in 0..4u32 {
            e.mem
                .set_f32(this + 0x30 + 4 * lane, 10.0 * (lane + 1) as f32);
            e.mem.set_f32(other + 0x30 + 4 * lane, 1.0);
            e.mem.set_f32(this + 0x20 + 4 * lane, lane as f32);
        }
        e.call(
            0x00cd_7d20,
            &args![
                Ptr::<()>::new(this),
                Ptr::<()>::new(other),
                0.5f32,
                Ptr::<()>::new(out)
            ],
        );
        // Centres 11, 21, 31, 41; extents 0.5, 1.5, 2.5, 3.5.
        let read = |e: &Engine, offset: u32| e.mem.f32(out + offset);
        assert_eq!([0, 4, 8, 12].map(|o| read(&e, o)), [10.5, 19.5, 28.5, 37.5]);
        assert_eq!(
            [16, 20, 24, 28].map(|o| read(&e, o)),
            [11.5, 22.5, 33.5, 44.5]
        );
    }
}
