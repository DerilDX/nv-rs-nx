//! `BSMain/bsparentvelocitymodifier.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31650` (named `NiPSysModifier::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C31650: u32 = 0x00c3_1650;

// Translated from 00c63930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c384c, right after
/// `BSParentVelocityModifier::LoadBinary` (010c3848). Forwards `this` and the stream to
/// `00c31650` (`NiPSysModifier::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c63930(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C31650, &args![this, stream]);
}

/// `00c31670` (named `NiPSysModifier::RegisterStreamables` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const REGISTER_STREAMABLES_TARGET_00C31670: u32 = 0x00c3_1670;

// Translated from 00c63950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSParentVelocityModifier` vtable word 010c3850, after the
/// `LinkObject` word 010c384c and before `BSParentVelocityModifier::SaveBinary` (010c3854).
/// Forwards `this` and the stream to `00c31670` (`NiPSysModifier::RegisterStreamables`).
/// Returns the `bool` in `AL`: Xbox PDB `?RegisterStreamables@NiObject@@UAA_NAAVNiStream@@@Z`
/// returns `_N`; `stream` is the `NiStream` reference as an address. Parameter count from the
/// `RET 4` of both bodies.
pub fn fn_00c63950(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_TARGET_00C31670, &args![this, stream])
        .bool()
}

// Translated from 00c63aa0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043c8 without reading the receiver. The map names this body `BSParentVelocityModifier::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c63aa0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43c8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_3aa0, fn_00c63aa0(Ptr) -> u32),
        entry!(0x00c6_3930, fn_00c63930(Ptr, Ptr)),
        entry!(0x00c6_3950, fn_00c63950(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c63930_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C31650, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_3930, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C31650)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c63950_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(REGISTER_STREAMABLES_TARGET_00C31670, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x00c6_3950, &args![0x1000u32, argument]);
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
    fn fn_00c63aa0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_3aa0, &args![0x1000u32]).u32(), 0x0120_43c8);
        assert_eq!(e.call(0x00c6_3aa0, &args![0u32]).u32(), 0x0120_43c8);
    }
}
