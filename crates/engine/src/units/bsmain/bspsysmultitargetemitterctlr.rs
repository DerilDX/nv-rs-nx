//! `BSMain/bspsysmultitargetemitterctlr.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1c0e0` (named `NiPSysEmitterCtlr::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C1C0E0: u32 = 0x00c1_c0e0;

// Translated from 00c5cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPSysMultiTargetEmitterCtlr::LinkObject` (engine map vtable name, vtable word 010c2da4):
/// forwards `this` and the stream to `00c1c0e0` (`NiPSysEmitterCtlr::LinkObject`). Returns
/// nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the
/// `NiStream` reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_psys_multi_target_emitter_ctlr_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C1C0E0, &args![this, stream]);
}

// Translated from 00c5cea0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01203340 without reading the receiver. The map names this body `BSPSysMultiTargetEmitterCtlr::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5cea0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_3340
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_cea0, fn_00c5cea0(Ptr) -> u32),
        entry!(
            0x00c5_cc90,
            bs_psys_multi_target_emitter_ctlr_link_object(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_psys_multi_target_emitter_ctlr_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C1C0E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_cc90, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C1C0E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5cea0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_cea0, &args![0x1000u32]).u32(), 0x0120_3340);
        assert_eq!(e.call(0x00c5_cea0, &args![0u32]).u32(), 0x0120_3340);
    }
}
