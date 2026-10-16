//! `Havok SDK/hkpphantomagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d42770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomAgent::linearCast` (map name, taken from a vtable). A virtual
/// method with five stack words (`RET 0x14`) whose body is empty.
pub fn hkp_phantom_agent_linear_cast(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d42770,
        hkp_phantom_agent_linear_cast(Ptr, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_phantom_agent_linear_cast_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(
            0x00d4_2770,
            &args![Ptr::<()>::new(0x10), 1u32, 2u32, 3u32, 4u32, 5u32],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
