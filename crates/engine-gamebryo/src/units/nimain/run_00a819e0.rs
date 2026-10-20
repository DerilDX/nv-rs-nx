//! `NiMain/run_00a819e0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiSkinPartition::Partition` vtable (its RTTI locator names
/// `.?AVPartition@NiSkinPartition@@`).
const PARTITION_VTABLE: u32 = 0x0109_e470;
/// `NiFree` (`00aa10f0`, cdecl, the block).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a81ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition::Partition` constructor (Ghidra made no function of
/// it; `NiSkinPartition::LoadBinary` passes it to the vector constructor):
/// the vtable, the six array pointers (+4 to +0x18), the strip count
/// (+0x22) and the renderer data (+0x28) cleared; the other counts are left
/// as they are. Returns `this`.
pub fn fn_00a81ba0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, PARTITION_VTABLE);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u16(base + 0x22, 0);
    e.mem.set_u32(base + 0x28, 0);
    this
}

// Translated from 00a81bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiSkinPartition::Partition` destructor body: the vtable, then the
/// six arrays (+4 to +0x18) freed with `NiFree`.
pub fn fn_00a81bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let first = e.mem.u32(base + 4);
    e.mem.set_u32(base, PARTITION_VTABLE);
    e.call(NI_FREE, &args![first]);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18] {
        let block = e.mem.u32(base + offset);
        e.call(NI_FREE, &args![block]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a81ba0, fn_00a81ba0(Ptr) -> Ptr),
        entry!(0x00a81bd0, fn_00a81bd0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_constructor_and_destructor() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xaa; 0x2c]);
        assert_eq!(e.call(0x00a8_1ba0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PARTITION_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
        assert_eq!(e.mem.u16(this + 0x1c), 0xaaaa);
        assert_eq!(e.mem.u16(this + 0x22), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        for (i, offset) in [4u32, 8, 0xc, 0x10, 0x14, 0x18].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 + i as u32);
        }
        e.mem.set_u32(this, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_1bd0, &args![this]);
        assert_eq!(e.mem.u32(this), PARTITION_VTABLE);
        let freed: Vec<_> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == NI_FREE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(freed, vec![0x100, 0x101, 0x102, 0x103, 0x104, 0x105]);
    }
}
