//! `NiMain/nidefaultavobjectpalette.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a84c70`: the jump target of `00a6e540`.
const JUMP_TARGET_00A6E540: u32 = 0x00a8_4c70;

// Translated from 00a6e540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDefaultAVObjectPalette::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a84c70` (engine map: `BSAnimNote::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiDefaultAVObjectPalette@@UAA_NAAVNiStream@@@Z`.
pub fn ni_default_av_object_palette_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A6E540, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_e540, ni_default_av_object_palette_register_streamables(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a6e540_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a6_e540;
        const TARGET: u32 = 0x00a8_4c70;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
