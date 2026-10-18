//! `NiMain/nigpuprogramcache.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ee9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGPUProgramCache::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ee9a60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x3c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_gpu_program_cache_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ee_9a60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

/// The slot `00aa1ab0` jumps through.
const POINTER_CELL_00AA1AB0: u32 = 0x011a_c230;

// Translated from 00aa1ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011ac230]`: `NiGPUProgramCache::ValidateDirectory` (engine map). The slot holds a function pointer (`0x011ac230` is a data cell, not an import slot); the 1 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 00ee8d2f (`ADD ESP,0x4`); the cell starts out holding `00aa1670`, a function that returns `AL`.
pub fn fn_00aa1ab0(e: &mut Engine, path: Ptr) -> bool {
    let target = e.global::<u32>(POINTER_CELL_00AA1AB0);
    e.call(target, &args![path]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee9ca0, ni_gpu_program_cache_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00aa_1ab0, fn_00aa1ab0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_gpu_program_cache_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ee_9a60, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ee9ca0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ee9ca0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x3c]]);
        assert_eq!(calls_to(&e, 0x00ee_9a60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod a06_thunk_tests {
    use super::*;

    /// Calls `thunk` with `words`, with the data cell `cell` holding the
    /// address of a double that returns `0xdeadbeef`; checks that the double
    /// ran once with exactly those words and that the thunk returned `expected`.
    fn pointer(thunk: u32, cell: u32, words: &[u32], expected: u32) {
        const TARGET: u32 = 0x00aa_1b60;
        let mut e = Engine::new();
        e.map(cell & !0xfff, 0x1000);
        e.set_global(cell, TARGET);
        e.register(TARGET, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_00aa1ab0() {
        pointer(0x00aa_1ab0, POINTER_CELL_00AA1AB0, &[0x1001], 1);
    }
}
