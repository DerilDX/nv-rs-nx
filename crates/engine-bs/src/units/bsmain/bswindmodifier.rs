//! `BSMain/bswindmodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c467f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1cbc of the table whose `CreateClone` is
/// `BSWindModifier::CreateClone` (010c1cb4), the slot after `LoadBinary` (010c1cb8). Forwards
/// `this` and the stream to `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox
/// PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c467f0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c46810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c1cc0 of the `BSWindModifier` table, after the
/// `LinkObject` word 010c1cbc. Forwards `this` and the stream to `00c31670`
/// (`NiPSysModifier::RegisterStreamables`). Returns the `bool` in `AL`: Xbox PDB
/// `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c46810(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c46960 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e60 without reading the receiver. The map names this body `BSWindModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c46960(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e60
}

/// `100.0` (`double`, `01017a40`), the rotation-matrix maker `004a0c90`
/// (`thiscall (matrix, angle)`), the point-times-matrix product `004b3ae0`
/// (`cdecl (&result, &point, &matrix)`, answers the result) and the global
/// `NiPoint3` the result goes to (`01202e68`).
const WIND_SCALE: u32 = 0x0101_7a40;
const MAKE_ROTATION: u32 = 0x004a_0c90;
const POINT_TIMES_MATRIX: u32 = 0x004b_3ae0;
const WIND_VECTOR: u32 = 0x0120_2e68;

// Translated from 00c468c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the global wind vector (the engine map has no name; `cdecl
/// (strength, angle)`): the point `(0, strength * 100.0, 0)` (the product
/// an x87 `FMUL` by the `double`, stored as a `float`) times the rotation
/// matrix `004a0c90` makes from `angle`, its three words stored at
/// `01202e68`.
pub fn fn_00c468c0(e: &mut Engine, strength: f32, angle: f32) {
    let scale = f64::from_bits(e.global::<u64>(WIND_SCALE));
    let scaled = x87_store32(x87_mul(x87_load32(strength.to_bits()), scale));
    e.with_stack(0x3c, |e, frame| {
        let point = frame.addr();
        let matrix = point + 0xc;
        let result = point + 0x30;
        e.mem.set_f32(point, 0.0);
        e.mem.set_u32(point + 4, x87_quiet_bits(scaled));
        e.mem.set_f32(point + 8, 0.0);
        e.call(
            MAKE_ROTATION,
            &args![matrix, x87_quiet_bits(angle.to_bits())],
        );
        let product = e
            .call(POINT_TIMES_MATRIX, &args![result, point, matrix])
            .u32();
        for i in 0..3 {
            let value = e.mem.u32(product + 4 * i);
            e.set_global(WIND_VECTOR + 4 * i, value);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_6960, fn_00c46960(Ptr) -> u32),
        entry!(0x00c4_67f0, fn_00c467f0(Ptr, Ptr)),
        entry!(0x00c4_6810, fn_00c46810(Ptr, Ptr) -> bool),
        entry!(0x00c4_68c0, fn_00c468c0(f32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c467f0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_67f0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c46810_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c4_6810, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C31670)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c46960_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_6960, &args![0x1000u32]).u32(), 0x0120_2e60);
        assert_eq!(e.call(0x00c4_6960, &args![0u32]).u32(), 0x0120_2e60);
    }

    #[test]
    fn wind_vector_is_the_rotated_scaled_point() {
        let mut e = Engine::new();
        e.map(WIND_SCALE, 8);
        e.set_global(WIND_SCALE, 100.0f64);
        e.map(WIND_VECTOR, 0xc);
        e.register(MAKE_ROTATION, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The "product" adds the matrix's first word (the angle) to each
        // coordinate of the point.
        e.register(POINT_TIMES_MATRIX, |e, a| {
            let angle = e.mem.f32(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[1] + 4 * i) + angle;
                e.mem.set_f32(a[0] + 4 * i, value);
            }
            a[0].into_ret()
        });
        e.call(0x00c4_68c0, &args![0.25f32, 1.5f32]);
        assert_eq!(e.global::<f32>(WIND_VECTOR), 1.5);
        assert_eq!(e.global::<f32>(WIND_VECTOR + 4), 26.5);
        assert_eq!(e.global::<f32>(WIND_VECTOR + 8), 1.5);
    }
}
