//! `fallout/magic/run_00822780` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00822780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00822780 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Builds `count` 4-byte elements in place (the engine map gives it no name). `ECX` is not read.
/// Parameters: `block` and `count`, `RET 8`. For each index below `count` (unsigned compare) it
/// calls the cdecl `006e6da0(4, block + index * 4)` and, when that returns a non-null object,
/// runs the constructor `00633c90(object, 0)` on it. No result. The exception-unwinding frame
/// is not translated.
pub fn fn_00822780(e: &mut Engine, block: u32, count: u32) {
    for index in 0..count {
        let object = e
            .call(
                0x006e_6da0,
                &args![4u32, block.wrapping_add(index.wrapping_mul(4))],
            )
            .u32();
        if object != 0 {
            e.call(0x0063_3c90, &args![object, 0u32]);
        }
    }
}

// Translated from 00822860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an array object (the engine map gives it no name) whose vtable slot `+4` takes
/// a count and returns a block. Parameters: `size` and `used`, `RET 8`, `this` in `ECX`. It clears
/// `+4` (block), `+8` (used count) and `+0xc` (capacity); `size` is raised to `used` when `used`
/// is larger (unsigned); when `size` is non-zero the block is `vtable[+4](size)` and the capacity
/// is `size`; when `used` is non-zero the block's elements are built with `00822780(block, used)`
/// and the used count is `used`. The Ghidra signature returns nothing and the disassembly does
/// not set `EAX`.
pub fn fn_00822860(e: &mut Engine, this: Ptr, size: u32, used: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let size = size.max(used);
    if size != 0 {
        let block = e.vcall(base, 4, &args![size]).u32();
        e.mem.set_u32(base + 4, block);
        e.mem.set_u32(base + 0xc, size);
    }
    if used != 0 {
        let block = e.mem.u32(base + 4);
        fn_00822780(e, block, used);
        e.mem.set_u32(base + 8, used);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_2780, fn_00822780(u32, u32)),
        entry!(0x0082_2860, fn_00822860(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod process_list_tests {
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
    fn elements_are_built_only_when_the_placement_call_gives_an_object() {
        let mut e = Engine::new();
        e.register(0x006e_6da0, |_, a| Ret {
            eax: if a[1] == 0x1004 { 0 } else { a[1] },
            ..Ret::default()
        });
        e.register(0x0063_3c90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_2780, &[0x1000, 3]);
        assert_eq!(
            calls_to(&e, 0x006e_6da0),
            vec![vec![4, 0x1000], vec![4, 0x1004], vec![4, 0x1008]]
        );
        assert_eq!(
            calls_to(&e, 0x0063_3c90),
            vec![vec![0x1000, 0], vec![0x1008, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0082_2780, &[0x1000, 0]);
        assert!(calls_to(&e, 0x006e_6da0).is_empty());
    }

    fn array_object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x00aa_0000);
        e.register(0x00aa_0000, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.write(this + 4, &[0xee; 12]);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn size_is_raised_to_used_and_the_elements_are_built() {
        let mut e = Engine::new();
        e.register(0x006e_6da0, |_, _| Ret::default());
        let this = array_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0082_2860, &[this, 2, 5]);
        assert_eq!(calls_to(&e, 0x00aa_0000)[0][1..], [5]);
        assert_eq!(e.mem.u32(this + 4), 0x7000);
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        assert_eq!(calls_to(&e, 0x006e_6da0).len(), 5);
    }

    #[test]
    fn capacity_without_used_builds_nothing() {
        let mut e = Engine::new();
        e.register(0x006e_6da0, |_, _| Ret::default());
        let this = array_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0082_2860, &[this, 8, 0]);
        assert_eq!(e.mem.u32(this + 4), 0x7000);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert!(calls_to(&e, 0x006e_6da0).is_empty());
    }

    #[test]
    fn zero_size_and_used_allocates_nothing() {
        let mut e = Engine::new();
        let this = array_object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0082_2860, &[this, 0, 0]);
        assert!(calls_to(&e, 0x00aa_0000).is_empty());
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0, 0, 0)
        );
    }
}
