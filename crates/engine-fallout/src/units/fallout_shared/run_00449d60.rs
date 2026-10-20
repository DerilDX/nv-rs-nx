//! `fallout shared/run_00449d60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00449e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00449d60` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00449e70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9d60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0044c480` (`thiscall`, `RET 4`): run with 1 first.
const CLEAR_ENTRIES: u32 = 0x0044_c480;
/// `0044c3f0` (`thiscall`, no stack word): the base destructor.
const BASE_DESTRUCTOR_0044C3F0: u32 = 0x0044_c3f0;

// Translated from 00449d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449d60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eece28, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `0101732c`, calls `0044c480(1)` on `this`, then the base destructor `0044c3f0`.
pub fn fn_00449d60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_732c);
    e.call(CLEAR_ENTRIES, &args![this, 1u32]);
    e.call(BASE_DESTRUCTOR_0044C3F0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_9e70, fn_00449e70(Ptr, u32) -> Ptr),
        entry!(0x0044_9d60, fn_00449d60(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00449e70_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_9d60, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_9e70, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_9e70)
                .collect();
            let mut expected = vec![(0x0044_9d60, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00449d60_clears_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(CLEAR_ENTRIES, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_0044C3F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0044_9d60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CLEAR_ENTRIES, vec![this, 1]),
                (BASE_DESTRUCTOR_0044C3F0, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0101_732c);
    }
}
