//! `(unplaced)/run_0076b6a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0057bd10` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_0057BD10: u32 = 0x0057_bd10;

// Translated from 0076b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `0057bd10(this, 0)`, then clears the word at `+0xc`.
/// Returns `this`.
pub fn fn_0076b930(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0057BD10, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

/// `0076b870` (`thiscall`, `RET 4`): run on each chained object with 1.
const CHAINED_DESTROY: u32 = 0x0076_b870;
/// `00418900` (`thiscall`, `RET 4`; unit `fallout shared/extradatalist.cpp`).
const ASSIGN_FROM: u32 = 0x0041_8900;
/// `00483710` (`thiscall`, no stack word; unit `fallout shared/tesform.cpp`): the base
/// destructor (also of the local).
const BASE_DESTRUCTOR: u32 = 0x0048_3710;

// Translated from 0076b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0076b7a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f0b368, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): destroys the chain at +0xc (each
/// object's +0xc is the next one: unlinked, then `0076b870(1)` on it), then builds a local
/// with `0057bd10(0)`, passes it to `00418900` on `this`, and destroys the local
/// (`00483710`).
pub fn fn_0076b7a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    while e.mem.u32(base + 0xc) != 0 {
        let head = e.mem.u32(base + 0xc);
        let next = e.mem.u32(head + 0xc);
        e.mem.set_u32(head + 0xc, 0);
        let object = e.mem.u32(base + 0xc);
        if object != 0 {
            e.call(CHAINED_DESTROY, &args![object, 1u32]);
        }
        e.mem.set_u32(base + 0xc, next);
    }
    e.with_stack(0xc, |e, local| {
        let made = e.call(BASE_CONSTRUCTOR_0057BD10, &args![local, 0u32]).u32();
        e.call(ASSIGN_FROM, &args![this, made]);
        e.call(BASE_DESTRUCTOR, &args![local]);
    });
}

// Translated from 0076b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0076b8a0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f005f8, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word; Ghidra matches the body to a library
/// destructor pattern): [`fn_0076b7a0`], then the base destructor `00483710`.
pub fn fn_0076b8a0(e: &mut Engine, this: Ptr) {
    fn_0076b7a0(e, this);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0076_b930, fn_0076b930(Ptr) -> Ptr),
        entry!(0x0076_b7a0, fn_0076b7a0(Ptr)),
        entry!(0x0076_b8a0, fn_0076b8a0(Ptr)),
    ]
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
    fn calls_the_base_then_clears_the_word() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_0057BD10, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0x5555);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0076_b930, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_0057BD10),
            vec![vec![this, 0]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_0076b8a0_destroys_the_chain_then_resets_through_a_local() {
        let mut e = Engine::new();
        e.register(CHAINED_DESTROY, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 0xc), 0);
            Ret::default()
        });
        e.register(BASE_CONSTRUCTOR_0057BD10, |_, a| a[0].into_ret());
        e.register(ASSIGN_FROM, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(first + 0xc, second);
        e.call_log = Some(vec![]);
        e.call(0x0076_b8a0, &args![this]);
        let log = e.call_log.take().unwrap();
        let local = log
            .iter()
            .find(|c| c.0 == BASE_CONSTRUCTOR_0057BD10)
            .unwrap()
            .1[0];
        assert_eq!(
            log[1..],
            [
                (CHAINED_DESTROY, vec![first, 1]),
                (CHAINED_DESTROY, vec![second, 1]),
                (BASE_CONSTRUCTOR_0057BD10, vec![local, 0]),
                (ASSIGN_FROM, vec![this, local]),
                (BASE_DESTRUCTOR, vec![local]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}
