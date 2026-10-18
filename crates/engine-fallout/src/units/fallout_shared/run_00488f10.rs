//! `fallout shared/run_00488f10` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00488ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00489620`, which frees the block held at `+4`
/// (through `00401030`) when it is non-null, zeroes `+4`, clears the byte at `+0` and
/// answers whether a block was freed, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`, AL) is from `00489620`'s disassembly, which ends `MOV
/// AL,[EBP-1]` (the flag it sets to 1 only on the freeing path); the wrapper leaves AL as it
/// came back.
pub fn fn_00488ff0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0048_9620, &args![this]).bool()
}

// Translated from 00488f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00488f50 - the exception-unwinding frame (FS:[0]) is not translated.
/// The constructor of the form component `TESModel` (the engine map gives it
/// no name; the RTTI name of the vtable it stores, `0101cf44`, is
/// `.?AVTESModel@@`): runs the base constructor (`00402d30`), stores the
/// vtable at `+0`, constructs the string at `+4` (`004037b0`), runs `00488fd0`
/// on the member at `+0xc`, then `TESModel::InitializeDataComponent` (Xbox PDB
/// name of `00489040`). Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00488f50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_cf44);
    e.call(0x0040_37b0, &args![this.byte_add(4)]);
    e.call(0x0048_8fd0, &args![this.byte_add(0xc)]);
    e.call(0x0048_9040, &args![this]);
    this
}

// Translated from 00489010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00489070` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00489010(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0048_9070, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_8ff0, fn_00488ff0(Ptr) -> bool),
        entry!(0x00488f50, fn_00488f50(Ptr) -> Ptr),
        entry!(0x0048_9010, fn_00489010(Ptr, u32) -> Ptr),
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
    fn fwd_a03_00488ff0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x0048_9620, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_8ff0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x0048_9620), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x0048_9620, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x0048_8ff0, &args![0x0000_5000u32]).bool());
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00488f50_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_8f50,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0040_37b0, &[A::T(0x4)]),
                (0x0048_8fd0, &[A::T(0xc)]),
                (0x0048_9040, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_cf44)],
        });
    }
    #[test]
    fn at_00489010_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_9070, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_9010, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_9010)
                .collect();
            let mut expected = vec![(0x0048_9070, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
