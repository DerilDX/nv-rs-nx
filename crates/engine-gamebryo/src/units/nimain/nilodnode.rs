//! `NiMain/nilodnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLODNode::ms_RTTI` (the class's `NiRTTI` record).
const NI_LOD_NODE_RTTI: u32 = 0x011f_5f1c;

// Translated from 00a994c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lod_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LOD_NODE_RTTI)
}

// Translated from 00a99670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLODNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a99470`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xd4 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_lod_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_9470;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xd4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a994c0, ni_lod_node_get_rtti() -> Ptr),
        entry!(0x00a99670, ni_lod_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lod_node_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a994c0, &args![]).u32(), NI_LOD_NODE_RTTI);
    }

    #[test]
    fn ni_lod_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_9470, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a99670, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a99670, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xd4]]);
        assert_eq!(calls_to(&e, 0x00a9_9470), vec![vec![this]; 2]);
    }
}
