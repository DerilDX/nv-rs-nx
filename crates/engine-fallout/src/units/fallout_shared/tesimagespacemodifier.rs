//! `fallout shared/tesimagespacemodifier.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0052b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0052b8f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x54 (0x730 bytes); `ECX` is the object.
/// The engine map names it `TESImageSpaceModifier::TESImageSpaceModifier` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0102_d97c`) is `TESImageSpaceModifier`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It builds the 0x2a-element array of 0x18-byte members at `+0x114` with the compiler's vector constructor (`00ec782f`), and, when the object is the one at `011c9750`, also calls `00484490` on it at the end.
/// The exception-unwinding frame is not translated.
pub fn fn_0052b8f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_d97c);
    e.call(0x0052_bad0, &args![base + 0x20]);
    e.call(
        0x00ec_782f,
        &args![
            base + 0x114,
            0x18u32,
            0x2au32,
            0x0052_baf0u32,
            0x0052_bb10u32
        ],
    );
    e.call(0x00a3_c340, &args![base + 0x504, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x51c, 0x0u32]);
    e.call(0x00a3_bd10, &args![base + 0x534, 0x0u32]);
    e.call(0x00a3_bd10, &args![base + 0x558, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x57c, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x594, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5ac, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5c4, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5dc, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x5f4, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x60c, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x624, 0x0u32]);
    e.call(0x00a3_c340, &args![base + 0x63c, 0x0u32]);
    e.call(0x004f_15a0, &args![this, 0x54u32]);
    e.call(0x0052_bd40, &args![this]);
    if base == 0x011c_9750 {
        e.call(0x0048_4490, &args![this]);
    }
    this
}

// Translated from 0052bad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `006815c0` on the member at `+0xcc` of `this` (`ECX`) and returns `this`. The disassembly
/// shows no other work; `006815c0` takes only `ECX`.
pub fn fn_0052bad0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this.addr() + 0xcc]);
    this
}

// Translated from 0052d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0052d240`, which only forwards `this` to
/// `00483710`, an empty function, and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `00483710`'s
/// disassembly is `PUSH EBP; MOV EBP,ESP; PUSH ECX; MOV [EBP-4],ECX; MOV ESP,EBP; POP EBP;
/// RET`, so nothing is returned.
pub fn fn_0052d220(e: &mut Engine, this: Ptr) {
    e.call(0x0052_d240, &args![this]);
}

// Translated from 0052baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0052bbd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0052baa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0052_bbd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_d220, fn_0052d220(Ptr)),
        entry!(0x0052_b8f0, fn_0052b8f0(Ptr) -> Ptr),
        entry!(0x0052_bad0, fn_0052bad0(Ptr) -> Ptr),
        entry!(0x0052_baa0, fn_0052baa0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0052b8f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0052_b8f0,
            size: 0x730,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0052_bad0, &[A::T(0x20)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x114),
                        A::V(0x18),
                        A::V(0x2a),
                        A::V(0x0052_baf0),
                        A::V(0x0052_bb10),
                    ],
                ),
                (0x00a3_c340, &[A::T(0x504), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x51c), A::V(0x0)]),
                (0x00a3_bd10, &[A::T(0x534), A::V(0x0)]),
                (0x00a3_bd10, &[A::T(0x558), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x57c), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x594), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5ac), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5c4), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5dc), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x5f4), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x60c), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x624), A::V(0x0)]),
                (0x00a3_c340, &[A::T(0x63c), A::V(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x54)]),
                (0x0052_bd40, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0102_d97c)],
        });
    }

    #[test]
    fn fn_0052b8f0_runs_the_extra_call_on_the_default_object() {
        // The object at `011c9750` also gets `00484490` called on it, last.
        let mut e = Engine::new();
        e.mem.map(0x011c_9750, 0x730);
        for a in [
            0x0048_3370u32,
            0x0052_bad0,
            0x00ec_782f,
            0x00a3_c340,
            0x00a3_bd10,
            0x004f_15a0,
            0x0052_bd40,
            0x0048_4490,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0052_b8f0, &args![0x011c_9750u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(0x0048_4490, vec![0x011c_9750]));
    }
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
    fn fwd_a03_0052d220_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0052_d240, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0052_d220, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0052_d240), vec![vec![0x5000]]);
    }

    #[test]
    fn at_0052baa0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0052_bbd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0052_baa0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0052_baa0)
                .collect();
            let mut expected = vec![(0x0052_bbd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod process_list_tests {
    use crate::prelude::*;

    #[test]
    fn fn_0052bad0_runs_the_member_constructor_and_returns_this() {
        let mut e = Engine::new();
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0052_bad0, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        let members: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0068_15c0).collect();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].1, vec![0x50cc]);
    }
}
