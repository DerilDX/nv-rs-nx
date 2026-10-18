//! `fallout/ai/lipsyncbackgroundmanager.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00905e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor that hands `this` (`ECX`) to `00906160` with the three words 10, 0x25 and 0xc
/// (pushed 0xc, 0x25, 0xa, so 10 is the first argument), and returns `this`. The Ghidra
/// signature shows one word; the function ends in a plain `RET`.
pub fn fn_00905e30(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00906160(e, this, 10, 0x25, 0xc);
    this
}

// Translated from 00906160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the object `00905e30` builds (the engine map names it only by its source
/// unit). Parameters: three words after `this`, `RET 0xc`: `rows` (the first), `slots` (the
/// second) and `extra` (the third). Order of events: `0044d7f0(this)` (vtable `01017458`),
/// the vtable `0108864c` at `+0`, `0096a2d0` on the member at `+0x20`, then, between the scope
/// guard `00404eb0` .. `00404ee0` (id 6, mode 1, source file and line 0x390 of
/// `ThreadSafeStructures.inl`): `+0x18` = 0, `+8` = `slots`, `+0xc` = an array of `slots`
/// words (size `slots * 4`, `0xFFFFFFFF` on overflow; each word cleared by the vector
/// constructor iterator `00401050` with the constructor `0044dee0`, none if the allocation
/// failed), `+4` = a block of `rows * 3 * 4` bytes (the product of the first two factors
/// wraps at 32 bits, the last multiplication saturates), `+0x10` = `extra`, and `+0x14` = a
/// 16-byte object built by `006c73b0(object, rows)` (0 if the allocation failed). The
/// exception-unwinding frame is not translated. Returns `this`.
pub fn fn_00906160(e: &mut Engine, this: Ptr, rows: u32, slots: u32, extra: u32) -> Ptr {
    let base = this.addr();
    e.call(0x0044_d7f0, &args![this]);
    e.mem.set_u32(base, 0x0108_864c);
    e.call(0x0096_a2d0, &args![base + 0x20]);
    e.with_stack(4, |e, scope| {
        e.call(
            0x0040_4eb0,
            &args![scope, 6u32, 1u32, 0x0101_73f8u32, 0x390u32],
        );
        e.mem.set_u32(base + 0x18, 0);
        e.mem.set_u32(base + 8, slots);
        let slot_count = e.mem.u32(base + 8);
        let slot_bytes = slot_count.saturating_mul(4);
        let slot_block = e.call(0x0040_1000, &args![slot_bytes]).u32();
        if slot_block != 0 {
            e.call(
                0x0040_1050,
                &args![slot_block, 4u32, slot_count, 0x0044_dee0u32],
            );
        }
        e.mem.set_u32(base + 0xc, slot_block);
        let row_bytes = rows.wrapping_mul(3).saturating_mul(4);
        let row_block = e.call(0x0040_1000, &args![row_bytes]).u32();
        e.mem.set_u32(base + 4, row_block);
        e.mem.set_u32(base + 0x10, extra);
        let object = e.call(0x0040_1000, &args![0x10u32]).u32();
        let built = if object != 0 {
            e.call(0x006c_73b0, &args![object, rows]).u32()
        } else {
            0
        };
        e.mem.set_u32(base + 0x14, built);
        e.call(0x0040_4ee0, &args![scope]);
    });
    this
}

// Translated from 00905c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00c3cea0`, which stores the vtable `010c1554` at
/// `+0`, calls `00aa4060` on the word at `+0x28` and ends with `00c3c620`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00c3cea0` ends with `CALL 00c3c620` and no result of its
/// own (Ghidra types both void).
pub fn fn_00905c30(e: &mut Engine, this: Ptr) {
    e.call(0x00c3_cea0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0090_5c30, fn_00905c30(Ptr)),
        entry!(0x0090_5e30, fn_00905e30(Ptr) -> Ptr),
        entry!(0x0090_6160, fn_00906160(Ptr, u32, u32, u32) -> Ptr),
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
    fn fwd_a03_00905c30_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x00c3_cea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0090_5c30, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x00c3_cea0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod process_list_tests {
    use super::*;

    const NEW: u32 = 0x0040_1000;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Doubles for every callee outside this file; `NEW` hands out 0x100-byte blocks.
    fn doubles(e: &mut Engine) {
        for address in [
            0x0044_d7f0,
            0x0096_a2d0,
            0x0040_4eb0,
            0x0040_4ee0,
            0x0040_1050,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x006c_73b0, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.register_double(NEW, |e, _| Ret {
            eax: e.mem.alloc(0x100),
            ..Ret::default()
        });
    }

    #[test]
    fn constructor_forwards_ten_0x25_and_0xc() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        let result = e.call(0x0090_5e30, &args![this]).u32();
        assert_eq!(result, this);
        // 0x25 slots of four bytes, ten rows of three words.
        assert_eq!(
            calls_to(&e, NEW).iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![0x25 * 4, 10 * 3 * 4, 0x10]
        );
        assert_eq!(e.mem.u32(this + 8), 0x25);
        assert_eq!(e.mem.u32(this + 0x10), 0xc);
        assert_eq!(calls_to(&e, 0x006c_73b0)[0][1], 10);
    }

    #[test]
    fn body_fills_the_fields_between_the_scope_guard() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x0090_6160, &args![this, 2u32, 3u32, 7u32]);
        assert_eq!(e.mem.u32(this), 0x0108_864c);
        assert_eq!(calls_to(&e, 0x0044_d7f0), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0096_a2d0), vec![vec![this + 0x20]]);
        let guard = calls_to(&e, 0x0040_4eb0);
        assert_eq!(guard[0][1..], [6, 1, 0x0101_73f8, 0x390]);
        assert_eq!(calls_to(&e, 0x0040_4ee0), vec![vec![guard[0][0]]]);
        let slot_block = e.mem.u32(this + 0xc);
        assert_eq!(
            calls_to(&e, 0x0040_1050),
            vec![vec![slot_block, 4, 3, 0x0044_dee0]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        let object = e.mem.u32(this + 0x14);
        assert_eq!(calls_to(&e, 0x006c_73b0), vec![vec![object - 0x1000, 2]]);
    }

    #[test]
    fn sizes_saturate_and_failed_allocations_skip_the_dependents() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(NEW, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0090_6160,
            &args![this, 0x4000_0000u32, 0x4000_0000u32, 1u32],
        );
        // 0x4000_0000 * 3 wraps to 0xC000_0000 and times four overflows: saturated.
        assert_eq!(
            calls_to(&e, NEW).iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![0xffff_ffff, 0xffff_ffff, 0x10]
        );
        assert!(calls_to(&e, 0x0040_1050).is_empty());
        assert!(calls_to(&e, 0x006c_73b0).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }
}
