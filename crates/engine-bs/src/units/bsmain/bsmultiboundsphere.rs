//! `BSMain/bsmultiboundsphere.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c4fa10` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C4FA10: u32 = 0x00c4_fa10;

// Translated from 00c38000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map calls this `BSMultiBoundSphere::LinkObject`, but its vtable word 0101e934
/// sits in the table whose neighbours are `BSMultiBoundAABB::CreateClone` and `LoadBinary`, so
/// it is named by address. Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map), the `LinkObject` body shared by the
/// multibound shapes. Returns nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is
/// void; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn fn_00c38000(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

// Translated from 00c4fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSMultiBoundSphere` vtable word 0101eadc, right after
/// `BSMultiBoundSphere::LoadBinary` (0101ead8). Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4fc50(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

/// `BSMultiBoundShape::BSMultiBoundShape` (`00c4f980`, `thiscall`) and
/// `BSMultiBoundSphere`'s vtable (`0101ea8c`).
const SHAPE_CONSTRUCTOR: u32 = 0x00c4_f980;
const SPHERE_VTABLE: u32 = 0x0101_ea8c;

// Translated from 00c4fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::BSMultiBoundSphere` (Xbox PDB, the overload the
/// engine map calls `_ov2`): the `BSMultiBoundShape` constructor, then the
/// class's vtable. Answers `this`.
pub fn bs_multi_bound_sphere_ov2(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SHAPE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), SPHERE_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_8000, fn_00c38000(Ptr, Ptr)),
        entry!(0x00c4_fc50, fn_00c4fc50(Ptr, Ptr)),
        entry!(0x00c4_fb10, bs_multi_bound_sphere_ov2(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c38000_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_8000, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4fc50_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_fc50, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn sphere_constructor_runs_the_shape_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(SHAPE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c4_fb10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SPHERE_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == SHAPE_CONSTRUCTOR && call.1 == vec![this]));
    }
}
