//! `fallout shared/speedtree/bstreenode.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066d470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0066d410`, which forwards `this` to `00c6c290`
/// (a destructor with an exception frame), and returns nothing. Parameters: one word,
/// `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX` on.
/// `0066d410` ends right after that call and `00c6c290` ends with its frame unlink and
/// register restores, loading no result; Ghidra types both void.
pub fn fn_0066d470(e: &mut Engine, this: Ptr) {
    e.call(0x0066_d410, &args![this]);
}

// Translated from 0066b5b0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d5e70 without reading the receiver. The map names this body `BSTreeNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066b5b0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_5e70
}

// Translated from 0066d400 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01268168 without reading the receiver. The map names this body `bhkPCollisionObject::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d400(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_8168
}

// Translated from 0066d430 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012683d0 without reading the receiver. The map names this body `bhkSPCollisionObject::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d430(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_83d0
}

// Translated from 0066d510 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012683e0 without reading the receiver. The map names this body `bhkMultiSphereShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d510(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_83e0
}

// Translated from 0066d610 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012681b4 without reading the receiver. The map names this body `bhkTransformShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d610(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_81b4
}

// Translated from 0066d720 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0126832c without reading the receiver. The map names this body `bhkCapsuleShape::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_0066d720(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0126_832c
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_b5b0, fn_0066b5b0(Ptr) -> u32),
        entry!(0x0066_d400, fn_0066d400(Ptr) -> u32),
        entry!(0x0066_d430, fn_0066d430(Ptr) -> u32),
        entry!(0x0066_d510, fn_0066d510(Ptr) -> u32),
        entry!(0x0066_d610, fn_0066d610(Ptr) -> u32),
        entry!(0x0066_d720, fn_0066d720(Ptr) -> u32),
        entry!(0x0066_d470, fn_0066d470(Ptr)),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0066d470_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0066_d410, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0066_d470, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0066_d410), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_0066b5b0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_b5b0, &args![0x1000u32]).u32(), 0x011d_5e70);
        assert_eq!(e.call(0x0066_b5b0, &args![0u32]).u32(), 0x011d_5e70);
    }

    #[test]
    fn fn_0066d400_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d400, &args![0x1000u32]).u32(), 0x0126_8168);
        assert_eq!(e.call(0x0066_d400, &args![0u32]).u32(), 0x0126_8168);
    }

    #[test]
    fn fn_0066d430_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d430, &args![0x1000u32]).u32(), 0x0126_83d0);
        assert_eq!(e.call(0x0066_d430, &args![0u32]).u32(), 0x0126_83d0);
    }

    #[test]
    fn fn_0066d510_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d510, &args![0x1000u32]).u32(), 0x0126_83e0);
        assert_eq!(e.call(0x0066_d510, &args![0u32]).u32(), 0x0126_83e0);
    }

    #[test]
    fn fn_0066d610_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d610, &args![0x1000u32]).u32(), 0x0126_81b4);
        assert_eq!(e.call(0x0066_d610, &args![0u32]).u32(), 0x0126_81b4);
    }

    #[test]
    fn fn_0066d720_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0066_d720, &args![0x1000u32]).u32(), 0x0126_832c);
        assert_eq!(e.call(0x0066_d720, &args![0u32]).u32(), 0x0126_832c);
    }
}
