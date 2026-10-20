//! `BSMain/bsrangenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c457d0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202e0c without reading the receiver. The map names this body `BSRangeNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c457d0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2e0c
}

/// `BSRangeNode`'s vtable (its class confirmed by the RTTI before it).
const BS_RANGE_NODE_VTABLE: u32 = 0x010c_1b2c;
/// `BSNiNode::BSNiNode` (`thiscall`, no arguments), the base `CopyMembers` (`00c5fb60`,
/// `thiscall`, `(destination, cloning)`), `NiNode::LoadBinary` and `NiNode::LinkObject`
/// (`thiscall`, `(stream)`) and `NiStreamLoadBinary<unsigned char>` (`cdecl`, `(stream,
/// &byte)`).
const BS_NI_NODE_CONSTRUCTOR: u32 = 0x00c5_f570;
const BASE_COPY_MEMBERS: u32 = 0x00c5_fb60;
const NI_NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NI_NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
const STREAM_LOAD_BYTE: u32 = 0x00a6_43b0;
/// The callback `SetCurrent` calls when the node's visibility changes (`cdecl`, `(node,
/// visible)`), null when none is set.
const RANGE_CHANGED_CALLBACK: u32 = 0x0120_2e08;

layout! {
    /// `BSRangeNode` (Xbox PDB), 0xB0 bytes on PC (its own fields 0x24 lower than the Xbox PDB says).
    pub struct BSRangeNode: 0xB0 {
        /// `NiAVObject::m_uFlags` (Xbox PDB): `SetCurrent` sets bit 0 when the index is out of range.
        0x30 m_uFlags: u32,
        /// `bEnabled` (Xbox PDB +0xD0).
        0xAC bEnabled: u8,
        /// `cCurrent` (Xbox PDB +0xD1).
        0xAD cCurrent: u8,
        /// `cMin` (Xbox PDB +0xD2): the lowest index shown.
        0xAE cMin: u8,
        /// `cMax` (Xbox PDB +0xD3): the highest index shown.
        0xAF cMax: u8,
    }
}

// Translated from 00c450d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::BSRangeNode` (Xbox PDB): `BSNiNode`'s constructor, the class's vtable, the
/// range 0..=255, current 0 and `bEnabled` true. Answers `this`.
pub fn bs_range_node_constructor(e: &mut Engine, this: Ptr<BSRangeNode>) -> Ptr<BSRangeNode> {
    e.call(BS_NI_NODE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BS_RANGE_NODE_VTABLE);
    e.set(this, BSRangeNode::cMin, 0);
    e.set(this, BSRangeNode::cMax, 0xff);
    e.set(this, BSRangeNode::cCurrent, 0);
    e.set(this, BSRangeNode::bEnabled, 1);
    this
}

// Translated from 00c455a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::SetCurrent` (Xbox PDB): the node is to be shown when `index` lies in
/// `[min, max]`; when that differs from its shown state (bit 0 of the flags
/// clear), the bit is set (hidden) or cleared (shown) and the callback at `01202e08`, when
/// set, is called with the node and the new state. The index itself is not stored. Answers
/// the new state.
pub fn bs_range_node_set_current(e: &mut Engine, this: Ptr<BSRangeNode>, index: u8) -> bool {
    let shown = e.get(this, BSRangeNode::m_uFlags) & 1 == 0;
    let visible =
        !(index < e.get(this, BSRangeNode::cMin) || index > e.get(this, BSRangeNode::cMax));
    if shown != visible {
        let flags = e.get(this, BSRangeNode::m_uFlags);
        let flags = if visible { flags & !1 } else { flags | 1 };
        e.set(this, BSRangeNode::m_uFlags, flags);
        let callback = e.global::<u32>(RANGE_CHANGED_CALLBACK);
        if callback != 0 {
            e.call(callback, &args![this, u32::from(visible)]);
        }
    }
    visible
}

// Translated from 00c451b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::CopyMembers` (Xbox PDB), `(destination, cloning)`: the base's
/// `CopyMembers` (`00c5fb60`), the range copied, and `SetCurrent` on the destination with
/// this node's current index.
pub fn bs_range_node_copy_members(
    e: &mut Engine,
    this: Ptr<BSRangeNode>,
    destination: Ptr<BSRangeNode>,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, destination, cloning]);
    let low = e.get(this, BSRangeNode::cMin);
    e.set(destination, BSRangeNode::cMin, low);
    let high = e.get(this, BSRangeNode::cMax);
    e.set(destination, BSRangeNode::cMax, high);
    let current = e.get(this, BSRangeNode::cCurrent);
    bs_range_node_set_current(e, destination, current);
}

// Translated from 00c45220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary`, then three bytes from the
/// stream (through one local): the lowest and highest index, and the current index given to
/// `SetCurrent`.
pub fn bs_range_node_load_binary(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) {
    e.call(NI_NODE_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, local| {
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let low = e.mem.u8(local.addr());
        e.set(this, BSRangeNode::cMin, low);
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let high = e.mem.u8(local.addr());
        e.set(this, BSRangeNode::cMax, high);
        e.call(STREAM_LOAD_BYTE, &args![stream, local]);
        let current = e.mem.u8(local.addr());
        bs_range_node_set_current(e, this, current);
    });
}

// Translated from 00c452a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRangeNode`'s `LinkObject` (unnamed in the map): `NiNode::LinkObject` with the stream.
pub fn fn_00c452a0(e: &mut Engine, this: Ptr<BSRangeNode>, stream: Ptr) {
    e.call(NI_NODE_LINK_OBJECT, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_57d0, fn_00c457d0(Ptr) -> u32),
        entry!(
            0x00c450d0,
            bs_range_node_constructor(Ptr<BSRangeNode>) -> Ptr<BSRangeNode>
        ),
        entry!(
            0x00c455a0,
            bs_range_node_set_current(Ptr<BSRangeNode>, u8) -> bool
        ),
        entry!(
            0x00c451b0,
            bs_range_node_copy_members(Ptr<BSRangeNode>, Ptr<BSRangeNode>, Ptr)
        ),
        entry!(0x00c45220, bs_range_node_load_binary(Ptr<BSRangeNode>, Ptr)),
        entry!(0x00c452a0, fn_00c452a0(Ptr<BSRangeNode>, Ptr)),
    ]
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c457d0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_57d0, &args![0x1000u32]).u32(), 0x0120_2e0c);
        assert_eq!(e.call(0x00c4_57d0, &args![0u32]).u32(), 0x0120_2e0c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn range_node(e: &mut Engine, low: u8, high: u8, flags: u32) -> Ptr<BSRangeNode> {
        let this = e.new_object::<BSRangeNode>();
        e.set(this, BSRangeNode::cMin, low);
        e.set(this, BSRangeNode::cMax, high);
        e.set(this, BSRangeNode::m_uFlags, flags);
        this
    }

    #[test]
    fn constructor_sets_the_full_range() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BS_NI_NODE_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<BSRangeNode>();
        e.set(this, BSRangeNode::cCurrent, 9);
        assert_eq!(e.call(0x00c450d0, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), BS_RANGE_NODE_VTABLE);
        assert_eq!(e.get(this, BSRangeNode::cMin), 0);
        assert_eq!(e.get(this, BSRangeNode::cMax), 0xff);
        assert_eq!(e.get(this, BSRangeNode::cCurrent), 0);
        assert_eq!(e.get(this, BSRangeNode::bEnabled), 1);
        assert_eq!(
            calls_at(&e, BS_NI_NODE_CONSTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn set_current_toggles_bit_0_and_calls_back_on_change() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let this = range_node(&mut e, 2, 4, 0x10);
        // Shown and in range: no change, no callback (none set anyway).
        assert!(e.call(0x00c455a0, &args![this, 3u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x10);
        // Out of range below: hidden, no callback set.
        assert!(!e.call(0x00c455a0, &args![this, 1u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x11);
        e.set_global::<u32>(RANGE_CHANGED_CALLBACK, 0x0090_0500);
        e.register(0x0090_0500, |_, _| Ret::default());
        // Back in range: shown, callback with 1.
        assert!(e.call(0x00c455a0, &args![this, 4u32]).bool());
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 0x10);
        // Above the range: hidden, callback with 0.
        assert!(!e.call(0x00c455a0, &args![this, 5u32]).bool());
        assert_eq!(
            calls_at(&e, 0x0090_0500),
            vec![vec![this.addr(), 1], vec![this.addr(), 0]]
        );
        assert_eq!(e.get(this, BSRangeNode::cCurrent), 0);
    }

    #[test]
    fn copy_members_copies_the_range_and_applies_the_current_index() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let this = range_node(&mut e, 1, 3, 0);
        e.set(this, BSRangeNode::cCurrent, 7);
        let destination = range_node(&mut e, 0, 0xff, 0);
        e.call(0x00c451b0, &args![this, destination, 0x5555u32]);
        assert_eq!(
            calls_at(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), 0x5555]]
        );
        assert_eq!(e.get(destination, BSRangeNode::cMin), 1);
        assert_eq!(e.get(destination, BSRangeNode::cMax), 3);
        assert_eq!(e.get(destination, BSRangeNode::m_uFlags), 1);
    }

    #[test]
    fn load_binary_reads_the_range_and_the_current_index() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_NODE_LOAD_BINARY, |_, _| Ret::default());
        e.map(RANGE_CHANGED_CALLBACK, 4);
        let mut bytes = vec![9u8, 6, 5].into_iter();
        e.register_double(STREAM_LOAD_BYTE, move |e, a| {
            e.mem.set_u8(a[1], bytes.next_back().unwrap());
            Ret::default()
        });
        let this = range_node(&mut e, 0, 0xff, 0);
        e.call(0x00c45220, &args![this, 0x6000u32]);
        assert_eq!(e.get(this, BSRangeNode::cMin), 5);
        assert_eq!(e.get(this, BSRangeNode::cMax), 6);
        assert_eq!(e.get(this, BSRangeNode::m_uFlags), 1);
        let reads = calls_at(&e, STREAM_LOAD_BYTE);
        assert_eq!(reads.len(), 3);
        assert!(reads.iter().all(|r| r[0] == 0x6000 && r[1] == reads[0][1]));
        assert_eq!(
            calls_at(&e, NI_NODE_LOAD_BINARY),
            vec![vec![this.addr(), 0x6000]]
        );
    }

    #[test]
    fn link_object_forwards_to_ni_node() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_NODE_LINK_OBJECT, |_, _| Ret::default());
        e.call(0x00c452a0, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_at(&e, NI_NODE_LINK_OBJECT),
            vec![vec![0x5000, 0x6000]]
        );
    }
}
