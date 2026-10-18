//! `BSMain/bsfurnituremarker.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! x87 note: `FurnitureMark::SetHeading` computes in extended precision and
//! stores `float` results. The translation computes in `f64` and rounds to
//! `f32` at each store; the product it truncates (a `float` times 1000) is
//! exact in `f64`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c54130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSFurnitureMarker` vtable word 010c2ae4, right after
/// `BSFurnitureMarker::LoadBinary` (010c2ae0). Forwards `this` and the stream to `00a7b3b0`
/// (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c54130(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

/// `00a7b290` (named `NiExtraData::RegisterStreamables` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00A7B290: u32 = 0x00a7_b290;

// Translated from 00c54150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFurnitureMarker::RegisterStreamables` (engine map vtable name, vtable word 010c2ae8):
/// forwards `this` and the stream to `00a7b290` (`NiExtraData::RegisterStreamables`). Returns
/// the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns
/// `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn bs_furniture_marker_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00A7B290, &args![this, stream])
        .bool()
}

/// The full circle in radians (`float`, `012031cc`): the range the heading
/// is wrapped into.
const FULL_CIRCLE: u32 = 0x0120_31cc;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1000.0` (`double`): the scale of the stored heading.
const HEADING_SCALE: u32 = 0x0101_7b70;

layout! {
    /// `FurnitureMark` (Xbox PDB), 0x10 bytes.
    pub struct FurnitureMark: 0x10 {
        /// `sHeading` (Xbox PDB): the heading in thousandths of a radian.
        0x0c sHeading: u16,
        /// `cNumber` (Xbox PDB).
        0x0e cNumber: u8,
    }
}

/// `FISTP dword` with the control word set to truncate: the integer, or
/// `0x80000000` ("integer indefinite") for a NaN or a value out of range.
fn truncate_to_i32(value: f64) -> i32 {
    let whole = value.trunc();
    if whole.is_nan() || !(-2_147_483_648.0..2_147_483_648.0).contains(&whole) {
        i32::MIN
    } else {
        whole as i32
    }
}

// Translated from 00c54550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FurnitureMark::SetHeading` (Xbox PDB): wraps `heading` into the range
/// from 0 up to the full circle (adds the circle while the heading is below
/// `0.0`, subtracts it while the circle is not above the heading; a NaN stays
/// as it is), and stores the heading times 1000, truncated, as the low 16
/// bits at `+0xc`.
pub fn furniture_mark_set_heading(e: &mut Engine, this: Ptr<FurnitureMark>, heading: f32) {
    let circle: f32 = e.global(FULL_CIRCLE);
    let zero: f64 = e.global(ZERO);
    let mut heading = heading;
    while (heading as f64) < zero {
        heading = (heading as f64 + circle as f64) as f32;
    }
    while (circle as f64) <= heading as f64 {
        heading = (heading as f64 - circle as f64) as f32;
    }
    let scale: f64 = e.global(HEADING_SCALE);
    let scaled = truncate_to_i32(heading as f64 * scale);
    e.set(this, FurnitureMark::sHeading, scaled as u16);
}

// Translated from 00c54d60 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031c4 without reading the receiver. The map names this body `BSFurnitureMarker::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c54d60(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31c4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_4d60, fn_00c54d60(Ptr) -> u32),
        entry!(0x00c5_4130, fn_00c54130(Ptr, Ptr)),
        entry!(0x00c5_4150, bs_furniture_marker_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00c54550,
            furniture_mark_set_heading(Ptr<FurnitureMark>, f32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c54130_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_4130, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_furniture_marker_register_streamables_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00A7B290, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c5_4150, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00A7B290)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod tests_b {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(FULL_CIRCLE, 4);
        e.set_global(FULL_CIRCLE, 6.0f32);
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.map(HEADING_SCALE, 8);
        e.set_global(HEADING_SCALE, 1000.0f64);
        e
    }

    fn heading_of(e: &mut Engine, heading: f32) -> u16 {
        let mark = e.mem.alloc(0x10);
        e.mem.write(mark, &[0xee; 0x10]);
        e.call(0x00c54550, &args![mark, heading]);
        assert_eq!(e.mem.u16(mark + 0xe), 0xeeee, "only the heading is written");
        e.mem.u16(mark + 0xc)
    }

    #[test]
    fn a_heading_inside_the_circle_is_scaled_and_truncated() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, 1.25), 1250);
        assert_eq!(heading_of(&mut e, 0.0), 0);
        assert_eq!(heading_of(&mut e, 0.0009), 0);
    }

    #[test]
    fn a_negative_heading_is_wrapped_up() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, -1.0), 5000);
        assert_eq!(heading_of(&mut e, -13.0), 5000);
    }

    #[test]
    fn a_heading_at_or_past_the_circle_is_wrapped_down() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, 6.0), 0);
        assert_eq!(heading_of(&mut e, 7.5), 1500);
        assert_eq!(heading_of(&mut e, 20.0), 2000);
    }

    #[test]
    fn a_nan_heading_stores_the_low_half_of_the_integer_indefinite() {
        let mut e = engine();
        assert_eq!(heading_of(&mut e, f32::NAN), 0);
    }

    #[test]
    fn the_stored_value_is_the_low_16_bits() {
        let mut e = engine();
        e.set_global(FULL_CIRCLE, 100.0f32);
        // 70.0 * 1000 = 70000 = 0x11170.
        assert_eq!(heading_of(&mut e, 70.0), 0x1170);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c54d60_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_4d60, &args![0x1000u32]).u32(), 0x0120_31c4);
        assert_eq!(e.call(0x00c5_4d60, &args![0u32]).u32(), 0x0120_31c4);
    }
}
