//! `BSMain/bsparticlesystemmanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5ea70` (named `NiNode::GetViewerStrings` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const VIEWER_STRINGS_TARGET_00A5EA70: u32 = 0x00a5_ea70;

// Translated from 00c51570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::GetViewerStrings` (engine map vtable name, vtable word 010c245c):
/// forwards `this` and the string array to `00a5ea70` (`NiNode::GetViewerStrings`). Returns
/// nothing: Xbox PDB `?GetViewerStrings@NiObject@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z` is void;
/// `strings` is the `NiTPrimitiveArray<char *>` pointer. Parameter count from the `RET 4` of
/// both bodies.
pub fn bs_particle_system_manager_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VIEWER_STRINGS_TARGET_00A5EA70, &args![this, strings]);
}

// Translated from 00c516f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01203110 without reading the receiver. The map names this body `BSParticleSystemManager::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c516f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_3110
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_16f0, fn_00c516f0(Ptr) -> u32),
        entry!(
            0x00c5_1570,
            bs_particle_system_manager_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_particle_system_manager_get_viewer_strings_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET_00A5EA70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_1570, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VIEWER_STRINGS_TARGET_00A5EA70)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c516f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_16f0, &args![0x1000u32]).u32(), 0x0120_3110);
        assert_eq!(e.call(0x00c5_16f0, &args![0u32]).u32(), 0x0120_3110);
    }
}
