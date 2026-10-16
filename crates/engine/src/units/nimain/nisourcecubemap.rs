//! `NiMain/nisourcecubemap.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSourceCubeMap::ms_RTTI` (the class's `NiRTTI` record).
const NI_SOURCE_CUBE_MAP_RTTI: u32 = 0x011f_4454;

// Translated from 00a60cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_source_cube_map_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SOURCE_CUBE_MAP_RTTI)
}

/// `00a60600`: the jump target of `00a60870`.
const JUMP_TARGET_00A60870: u32 = 0x00a6_0600;

// Translated from 00a60870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSourceCubeMap::SaveBinary` (Xbox PDB): a 5-byte jump to `00a60600` (engine map: `NiSourceTexture::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiSourceCubeMap@@UAAXAAVNiStream@@@Z`.
pub fn ni_source_cube_map_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A60870, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a60cb0, ni_source_cube_map_get_rtti() -> Ptr),
        entry!(0x00a6_0870, ni_source_cube_map_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_source_cube_map_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a60cb0, &args![]).u32(), NI_SOURCE_CUBE_MAP_RTTI);
    }

    #[test]
    fn thunk_00a60870_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a6_0870;
        const TARGET: u32 = 0x00a6_0600;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}
