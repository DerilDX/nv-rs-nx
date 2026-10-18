//! `NiMain/nisortadjustnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSortAdjustNode::ms_RTTI` (the class's `NiRTTI` record).
const NI_SORT_ADJUST_NODE_RTTI: u32 = 0x011f_5ed8;

// Translated from 00a95310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_sort_adjust_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SORT_ADJUST_NODE_RTTI)
}

/// `00a5dce0`: the jump target of `00a95290`.
const JUMP_TARGET_00A95290: u32 = 0x00a5_dce0;

// Translated from 00a95290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::ProcessClone` (Xbox PDB): a 5-byte jump to `00a5dce0` (engine map: `NiNode::ProcessClone`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?ProcessClone@NiSortAdjustNode@@UAAXAAVNiCloningProcess@@@Z`.
pub fn ni_sort_adjust_node_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(JUMP_TARGET_00A95290, &args![this, process]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a95310, ni_sort_adjust_node_get_rtti() -> Ptr),
        entry!(0x00a9_5290, ni_sort_adjust_node_process_clone(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_sort_adjust_node_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a95310, &args![]).u32(), NI_SORT_ADJUST_NODE_RTTI);
    }

    #[test]
    fn thunk_00a95290_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_5290;
        const TARGET: u32 = 0x00a5_dce0;
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
