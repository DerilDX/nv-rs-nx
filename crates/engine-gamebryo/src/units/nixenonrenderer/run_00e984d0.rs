//! `NiXenonRenderer/run_00e984d0` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The chain of 0x20-byte records `00e98610` frees (`0126fa58`, next at +0x18) and the Ni free
/// `00aa1460` (`cdecl`: block, size).
const CHAIN_0126FA58: u32 = 0x0126_fa58;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e98610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`), no arguments: frees every record of the chain
/// at `0126fa58` (next at +0x18 read first, +8 cleared, 0x20 bytes freed through `00aa1460`),
/// then clears the chain's head.
pub fn fn_00e98610(e: &mut Engine) {
    let mut record = e.mem.u32(CHAIN_0126FA58);
    while record != 0 {
        let next = e.mem.u32(record.wrapping_add(0x18));
        e.mem.set_u32(record.wrapping_add(8), 0);
        e.call(NI_FREE_00AA1460, &args![record, 0x20u32]);
        record = next;
    }
    e.mem.set_u32(CHAIN_0126FA58, 0);
}

/// The base table (`010f1644`, `_purecall` in its slots) and the `InterlockedExchange` import.
const BASE_VTABLE_010F1644: u32 = 0x010f_1644;
const IMPORT_INTERLOCKED_EXCHANGE: u32 = 0x00fd_f0e4;

// Translated from 00e986f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): a base constructor (the geometry groups'),
/// `thiscall`, no stack words: the table `010f1644`, the word +4 set to 0 through
/// `InterlockedExchange`, +8 cleared. Answers `this`.
pub fn fn_00e986f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE_010F1644);
    e.call(
        IMPORT_INTERLOCKED_EXCHANGE,
        &args![base.wrapping_add(4), 0u32],
    );
    e.mem.set_u32(base.wrapping_add(8), 0);
    this
}

// Translated from 00e98710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e984d0`): the base destructor matching `00e986f0`,
/// `thiscall`, no stack words: the table `010f1644`, then the object at +8 released (its
/// slot 8, `stdcall` with the object) when set.
pub fn fn_00e98710(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BASE_VTABLE_010F1644);
    let held = e.mem.u32(base.wrapping_add(8));
    if held != 0 {
        e.vcall(held, 8, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_8610, fn_00e98610()),
        entry!(0x00e9_86f0, fn_00e986f0(Ptr) -> Ptr),
        entry!(0x00e9_8710, fn_00e98710(Ptr)),
    ]
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e98610_frees_the_chain() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(NI_FREE_00AA1460, |_, _| Ret::default());
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(first + 0x18, second);
        e.mem.set_u32(first + 8, 1);
        e.mem.set_u32(second + 8, 2);
        e.mem.set_u32(CHAIN_0126FA58, first);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8610, &args![]);
        assert_eq!(
            calls(&e, NI_FREE_00AA1460),
            vec![vec![first, 0x20], vec![second, 0x20]]
        );
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(second + 8), 0);
        assert_eq!(e.mem.u32(CHAIN_0126FA58), 0);
    }

    #[test]
    fn fn_00e986f0_and_00e98710_build_and_release() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_EXCHANGE, |_, _| Ret::default());
        e.register(0x0e50_0008, |_, _| Ret::default());
        e.put_vtable(0x0e51_0000, &[0, 0, 0x0e50_0008]);
        let this = e.mem.alloc(0xc);
        e.mem.write(this, &[0xaa; 0xc]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_86f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_010F1644);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_EXCHANGE),
            vec![vec![this + 4, 0]]
        );
        // Nothing held: no release.
        e.call(0x00e9_8710, &args![this]);
        assert!(calls(&e, 0x0e50_0008).is_empty());
        let held = e.mem.alloc(4);
        e.mem.set_u32(held, 0x0e51_0000);
        e.mem.set_u32(this + 8, held);
        e.mem.set_u32(this, 0);
        e.call(0x00e9_8710, &args![this]);
        assert_eq!(calls(&e, 0x0e50_0008), vec![vec![held]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE_010F1644);
    }
}
