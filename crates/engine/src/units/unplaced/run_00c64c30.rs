//! `(unplaced)/run_00c64c30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c6ba30`: the jump target of `00c657f0`.
const JUMP_TARGET_00C657F0: u32 = 0x00c6_ba30;

// Translated from 00c657f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00c6ba30`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00c657f0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00C657F0, &args![this, first, second]);
}

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c65160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c39bc (unit `run_00c64c30`). Forwards `this` and
/// the stream to `00a7b3b0` (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c65160(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

// Translated from 00c657e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043f0 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00c657e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43f0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_57e0, fn_00c657e0(Ptr) -> u32),
        entry!(0x00c6_57f0, fn_00c657f0(Ptr, Ptr, Ptr)),
        entry!(0x00c6_5160, fn_00c65160(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c657f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_57f0;
        const TARGET: u32 = 0x00c6_ba30;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn forwarder_fn_00c65160_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5160, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c657e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_57e0, &args![0x1000u32]).u32(), 0x0120_43f0);
        assert_eq!(e.call(0x00c6_57e0, &args![0u32]).u32(), 0x0120_43f0);
    }
}
