//! `BSMain/bspsysarrayemitter.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31b40` (named `NiPSysVolumeEmitter::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31B40: u32 = 0x00c3_1b40;

// Translated from 00c61f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::LinkObject` (engine map vtable name, vtable word 010c3714): forwards
/// `this` and the stream to `00c31b40` (`NiPSysVolumeEmitter::LinkObject`). Returns nothing:
/// Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31B40, &args![this, stream]);
}

/// `00c2f4f0` (named `NiPSysAirFieldModifier::RegisterStreamables` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C2F4F0: u32 = 0x00c2_f4f0;

// Translated from 00c61f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::RegisterStreamables` (engine map vtable name, vtable word 010c3718):
/// forwards `this` and the stream to `00c2f4f0`, a one-instruction jump to `00c28b60`, itself
/// a jump to `NiPSysModifier::RegisterStreamables` (`00c31670`). Returns the `bool` in `AL`:
/// Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z` returns `_N`; `stream` is
/// the `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C2F4F0, &args![this, stream])
        .bool()
}

/// `00c31b60` (named `NiPSysVolumeEmitter::SaveBinary` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00C31B60: u32 = 0x00c3_1b60;

// Translated from 00c61f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::SaveBinary` (engine map vtable name, vtable word 010c371c): forwards
/// `this` and the stream to `00c31b60` (`NiPSysVolumeEmitter::SaveBinary`). Returns nothing:
/// Xbox PDB `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_array_emitter_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00C31B60, &args![this, stream]);
}

/// `00c31b80` (named `NiPSysVolumeEmitter::IsEqual` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const IS_EQUAL_TARGET_00C31B80: u32 = 0x00c3_1b80;

// Translated from 00c61fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysArrayEmitter::IsEqual` (engine map vtable name, vtable word 010c3720): forwards
/// `this` and the other object to `00c31b80` (`NiPSysVolumeEmitter::IsEqual`). Returns the
/// `bool` in `AL`: Xbox PDB `?IsEqual@NiObject@@UAA_NPAV1@@Z` returns `_N` and the callee's
/// equal path ends `MOV AL,1`; `other` is the other `NiObject`. Parameter count from the `RET
/// 4` of both bodies.
pub fn bs_psys_array_emitter_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(IS_EQUAL_TARGET_00C31B80, &args![this, other]).bool()
}

// Translated from 00c632e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043b8 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00c632e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43b8
}

// Translated from 00c63320 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043b0 without reading the receiver. The map names this body `BSPSysArrayEmitter::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c63320(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43b0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_32e0, fn_00c632e0(Ptr) -> u32),
        entry!(0x00c6_3320, fn_00c63320(Ptr) -> u32),
        entry!(0x00c6_1f10, bs_psys_array_emitter_link_object(Ptr, Ptr)),
        entry!(0x00c6_1f60, bs_psys_array_emitter_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c6_1f80, bs_psys_array_emitter_save_binary(Ptr, Ptr)),
        entry!(0x00c6_1fa0, bs_psys_array_emitter_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_psys_array_emitter_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31B40, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1f10, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31B40)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_register_streamables_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C2F4F0, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_1f60, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == REGISTER_STREAMABLES_TARGET_00C2F4F0)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_save_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00C31B60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_1f80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00C31B60)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bs_psys_array_emitter_is_equal_forwards_this_and_the_argument_and_returns_the_byte(
    ) {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(IS_EQUAL_TARGET_00C31B80, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_1fa0, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == IS_EQUAL_TARGET_00C31B80)
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
    fn fn_00c632e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_32e0, &args![0x1000u32]).u32(), 0x0120_43b8);
        assert_eq!(e.call(0x00c6_32e0, &args![0u32]).u32(), 0x0120_43b8);
    }

    #[test]
    fn fn_00c63320_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_3320, &args![0x1000u32]).u32(), 0x0120_43b0);
        assert_eq!(e.call(0x00c6_3320, &args![0u32]).u32(), 0x0120_43b0);
    }
}
