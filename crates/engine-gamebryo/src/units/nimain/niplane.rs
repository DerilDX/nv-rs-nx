//! `NiMain/niplane.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPlane;

// Translated from 00a69970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane_ov2` (engine map): the normal copied as three words from `normal`, the
/// constant through `FLD`/`FSTP` (a signalling NaN is stored quiet). Returns `this`.
pub fn ni_plane_ni_plane_ov2(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    normal: Ptr,
    constant: f32,
) -> Ptr<NiPlane> {
    let base = this.addr();
    for i in 0..2u32 {
        let word = e.mem.u32(normal.addr() + 4 * i);
        e.mem.set_u32(base + 4 * i, word);
    }
    let z = e.mem.u32(normal.addr() + 8);
    e.mem
        .set_u32(base + 0xc, x87_quiet_bits(constant.to_bits()));
    e.mem.set_u32(base + 8, z);
    this
}

// Translated from 00a69990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPlane::NiPlane_ov3` (engine map): the normal copied as three words from `normal`, the
/// constant the dot product of the normal and `point`, as the x87 code sums it:
/// `(ny·py + nx·px) + nz·pz`, rounded to a float. Returns `this`.
pub fn ni_plane_ni_plane_ov3(
    e: &mut Engine,
    this: Ptr<NiPlane>,
    normal: Ptr,
    point: Ptr,
) -> Ptr<NiPlane> {
    let base = this.addr();
    let n = normal.addr();
    let p = point.addr();
    for i in 0..3u32 {
        let word = e.mem.u32(n + 4 * i);
        e.mem.set_u32(base + 4 * i, word);
    }
    let y = x87_mul(x87_load32(e.mem.u32(n + 4)), x87_m32(e.mem.u32(p + 4)));
    let x = x87_mul(x87_load32(e.mem.u32(n)), x87_m32(e.mem.u32(p)));
    let sum = x87_add(y, x);
    let z = x87_mul(x87_load32(e.mem.u32(n + 8)), x87_m32(e.mem.u32(p + 8)));
    let constant = x87_add(sum, z);
    e.mem.set_u32(base + 0xc, x87_store32(constant));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6_9970,
            ni_plane_ni_plane_ov2(Ptr<NiPlane>, Ptr, f32) -> Ptr<NiPlane>
        ),
        entry!(
            0x00a6_9990,
            ni_plane_ni_plane_ov3(Ptr<NiPlane>, Ptr, Ptr) -> Ptr<NiPlane>
        ),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn words(e: &Engine, p: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(p + 4 * i)).collect()
    }

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let p = e.mem.alloc(0xc);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(p + 4 * i as u32, v.to_bits());
        }
        p
    }

    #[test]
    fn plane_from_normal_and_constant() {
        let mut e = Engine::new();
        let n = e.mem.alloc(0xc);
        e.mem.set_u32(n, 0x7f80_0001);
        e.mem.set_u32(n + 4, 5);
        e.mem.set_u32(n + 8, 6);
        let this = e.mem.alloc(0x10);
        let r = e.call(0x00a6_9970, &[this, n, 0x7f80_0002]);
        assert_eq!(r.u32(), this);
        assert_eq!(words(&e, this), vec![0x7f80_0001, 5, 6, 0x7fc0_0002]);
        e.call(0x00a6_9970, &[this, n, 2.5f32.to_bits()]);
        assert_eq!(e.mem.u32(this + 0xc), 2.5f32.to_bits());
    }

    #[test]
    fn plane_from_normal_and_point() {
        let mut e = Engine::new();
        let n = point(&mut e, [0.0, 0.6, 0.8]);
        let p = point(&mut e, [10.0, 2.0, -3.0]);
        let this = e.mem.alloc(0x10);
        let r = e.call(0x00a6_9990, &args![this, n, p]);
        assert_eq!(r.u32(), this);
        // The x term is an exact 0.
        let want = (f64::from(0.6f32) * 2.0 + f64::from(0.8f32) * -3.0) as f32;
        assert_eq!(
            words(&e, this),
            vec![0, 0.6f32.to_bits(), 0.8f32.to_bits(), want.to_bits()]
        );
    }
}
