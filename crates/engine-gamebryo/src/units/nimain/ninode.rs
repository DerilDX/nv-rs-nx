//! `NiMain/ninode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a5ece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a5e930`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xac bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_node_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_e930;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xacu32]);
    }
    this
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// `NiNode`'s vtable (`0109b5ac`, RTTI `.?AVNiNode@@`).
const NI_NODE_VTABLE: u32 = 0x0109_b5ac;
/// The child array's constructor `004aff00` (`thiscall`, `RET 8`: initial size, grow-by).
const CHILD_ARRAY_CONSTRUCT: u32 = 0x004a_ff00;

// Translated from 00a5ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiNode::NiNode` (unnamed in the engine map; the vtable it sets is `NiNode`'s by its RTTI).
/// `thiscall`, one word (`RET 4`): runs `NiAVObject`'s constructor, sets the vtable and
/// constructs the child array `m_kChildren` (+0x9c) with that initial size and a grow-by of 1
/// (`004aff00`). Answers `this`.
pub fn ni_node_ni_node(e: &mut Engine, this: Ptr, child_count: u32) -> Ptr {
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_NODE_VTABLE);
    e.call(
        CHILD_ARRAY_CONSTRUCT,
        &args![this.addr().wrapping_add(0x9c), child_count, 1u32],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a5ece0, ni_node_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_ecb0, ni_node_ni_node(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_node_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_e930, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a5ece0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a5ece0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xac]]);
        assert_eq!(calls_to(&e, 0x00a5_e930), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    #[test]
    fn ni_node_ni_node_builds_the_object_and_its_child_array() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CHILD_ARRAY_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xac);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_ecb0, &args![this, 3u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00a5_ecb0, NI_AV_OBJECT_CONSTRUCT, CHILD_ARRAY_CONSTRUCT]
        );
        let args_of = |addr: u32| log.iter().find(|c| c.0 == addr).unwrap().1.clone();
        assert_eq!(args_of(NI_AV_OBJECT_CONSTRUCT), vec![this]);
        assert_eq!(args_of(CHILD_ARRAY_CONSTRUCT), vec![this + 0x9c, 3, 1]);
        assert_eq!(e.mem.u32(this), NI_NODE_VTABLE);
    }
}
