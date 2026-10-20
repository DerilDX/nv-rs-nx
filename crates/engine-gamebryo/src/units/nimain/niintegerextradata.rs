//! `NiMain/niintegerextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00a7b2e0`: the `NiExtraData` constructor (`thiscall`, no stack word).
const EXTRA_DATA_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// `00a7b330`: `NiExtraData`'s member copy (`thiscall` on the source: clone, cloning
/// process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// The `NiIntegerExtraData` virtual table (`010a0d24`).
const INTEGER_EXTRA_DATA_VTABLE: u32 = 0x010a_0d24;

// Translated from 00a99c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process):
/// allocates 0x10 bytes and, when that succeeds, constructs an `NiIntegerExtraData` in them
/// (`NiExtraData` constructor `00a7b2e0`, virtual table `010a0d24`, value 0). The source's
/// `NiExtraData` member copy (`00a7b330`) fills the clone (null when the allocation failed),
/// then the value at `+0xC` is copied. Answers the clone.
pub fn ni_integer_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if clone != 0 {
        e.call(EXTRA_DATA_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone, INTEGER_EXTRA_DATA_VTABLE);
        e.mem.set_u32(clone + 0xc, 0);
    }
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    // NiIntegerExtraData::m_iValue (Xbox PDB) +0x0C.
    let value = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(clone.wrapping_add(0xc), value);
    Ptr::new(clone)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9c80, ni_integer_extra_data_create_clone(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_clone_constructs_copies_and_takes_the_value() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + 0xc, 0x55);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x10);
            block.into_ret()
        });
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |e, a| {
            // The value is still the constructor's 0 when the base copy runs.
            assert_eq!(e.mem.u32(a[1] + 0xc), 0);
            Ret::default()
        });
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, 0xabcd);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9c80, &args![source, 0x7000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), INTEGER_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + 0xc), 0xabcd);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_COPY_MEMBERS)
            .collect();
        assert_eq!(copies[0].1, vec![source, block, 0x7000]);
        let built: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_CONSTRUCTOR)
            .collect();
        assert_eq!(built[0].1, vec![block]);
    }

    #[test]
    fn create_clone_copies_into_a_null_clone_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(EXTRA_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.map(0, 0x10);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9c80, &args![source, 0x7000u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == EXTRA_DATA_CONSTRUCTOR));
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_COPY_MEMBERS)
            .collect();
        assert_eq!(copies[0].1, vec![source, 0, 0x7000]);
        // The exe then writes the value through the null clone (address 0x0C).
        assert_eq!(e.mem.u32(0xc), 9);
    }
}
