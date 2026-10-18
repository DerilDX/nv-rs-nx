//! `NiMain/nibillboardnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5e5b0`: the jump target of `00a93c40`.
const JUMP_TARGET_00A93C40: u32 = 0x00a5_e5b0;

/// `00a5e9d0`: the jump target of `00a9a730`.
const JUMP_TARGET_00A9A730: u32 = 0x00a5_e9d0;

// Translated from 00a93c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a5e5b0` (engine map: `NiNode::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBillboardNode@@UAA_NAAVNiStream@@@Z`.
pub fn ni_billboard_node_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A93C40, &args![this, stream]).bool()
}

// Translated from 00a9a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBillboardNode::LinkObject` (Xbox PDB): a 5-byte jump to `00a5e9d0` (engine map: `NiNode::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiBillboardNode@@UAAXAAVNiStream@@@Z`.
pub fn ni_billboard_node_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9A730, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_3c40, ni_billboard_node_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9_a730, ni_billboard_node_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a93c40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_3c40;
        const TARGET: u32 = 0x00a5_e5b0;
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

    #[test]
    fn thunk_00a9a730_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_a730;
        const TARGET: u32 = 0x00a5_e9d0;
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
