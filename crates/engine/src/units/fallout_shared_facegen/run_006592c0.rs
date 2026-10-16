//! `fallout shared/facegen/run_006592c0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CreateSemaphoreA` import slot (`kernel32`, stdcall: attributes, initial count, maximum
/// count, name).
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;

// Translated from 00659500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the two words at `+0` and `+4`, then creates a
/// semaphore with `CreateSemaphoreA(null, [+0], [+4], null)` (so the first word is the initial
/// count and the second the maximum count) and stores its handle at `+8`. Returns `this`.
pub fn fn_00659500(e: &mut Engine, this: Ptr, initial_count: u32, maximum_count: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, initial_count);
    e.mem.set_u32(base + 4, maximum_count);
    let initial = e.mem.u32(base);
    let maximum = e.mem.u32(base + 4);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, initial, maximum, 0u32])
        .u32();
    e.mem.set_u32(base + 8, handle);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_9500, fn_00659500(Ptr, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn creates_the_semaphore_from_the_two_words() {
        let mut e = Engine::new();
        e.register(CREATE_SEMAPHORE_A, |_, _| Ret {
            eax: 0xabc,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9500, &args![this, 1u32, 5u32]).u32(), this);
        assert_eq!(calls_to(&mut e, CREATE_SEMAPHORE_A), vec![vec![0, 1, 5, 0]]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 8), 0xabc);
    }
}
