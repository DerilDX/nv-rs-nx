//! `fallout/ai/run_008caa70` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008cab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0044fb00`, which calls the import slot
/// `00fdf058` (Ghidra: `DeleteCriticalSection`) on `this`, and returns nothing. Parameters:
/// one word, `this`; the disassembly of the function ends in a plain `RET` and passes only
/// `ECX` on. `0044fb00` ends right after that call; `DeleteCriticalSection` returns nothing
/// and Ghidra types `0044fb00` void.
pub fn fn_008cab00(e: &mut Engine, this: Ptr) {
    e.call(0x0044_fb00, &args![this]);
}

/// The base constructor `0044fae0` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_0044FAE0: u32 = 0x0044_fae0;

// Translated from 008caab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `0044fae0` on `this`, then clears the words at `+0x44`
/// and `+0x40`. Returns `this`.
pub fn fn_008caab0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0044FAE0, &args![this]);
    e.mem.set_u32(this.addr() + 0x44, 0);
    e.mem.set_u32(this.addr() + 0x40, 0);
    this
}

// Translated from 008caae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the word at `+0xc`. Returns `this`.
pub fn fn_008caae0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008c_ab00, fn_008cab00(Ptr)),
        entry!(0x008c_aab0, fn_008caab0(Ptr) -> Ptr),
        entry!(0x008c_aae0, fn_008caae0(Ptr) -> Ptr),
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
    fn fwd_a03_008cab00_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0044_fb00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008c_ab00, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0044_fb00), vec![vec![0x5000]]);
    }
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
    fn fn_008caab0_runs_the_base_then_clears_the_two_words() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0044FAE0, |e, a| {
            e.mem.set_u32(a[0] + 0x40, 0x11);
            e.mem.set_u32(a[0] + 0x44, 0x22);
            Ret::default()
        });
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008c_aab0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0044FAE0),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn fn_008caae0_clears_the_word_at_0c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x77);
        e.mem.set_u32(this + 0x10, 0x88);
        assert_eq!(e.call(0x008c_aae0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x88);
    }
}
