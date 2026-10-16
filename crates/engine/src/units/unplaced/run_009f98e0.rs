//! `(unplaced)/run_009f98e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009f9c80`: the jump target of `009f9d50`.
const JUMP_TARGET_009F9D50: u32 = 0x009f_9c80;

// Translated from 009f9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `009f9c80`, passing on `this` and its one argument; returns the target result word. Return type (u32): the target `009f9c80` returns EAX at both returns (0, or `0x8007000e`); its meaning is unconfirmed.
pub fn fn_009f9d50(e: &mut Engine, this: Ptr, arg: Ptr) -> u32 {
    e.call(JUMP_TARGET_009F9D50, &args![this, arg]).u32()
}

// Translated from 009fdd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `009fddd0`. `009fddd0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_009fdd50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x009f_ddd0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// The slot `009f9968` jumps through.
const IMPORT_SLOT_009F9968: u32 = 0x00fd_f3c8;

// Translated from 009f9968 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3c8]` (the import slot; Ghidra labels the import `D3DPERF_SetOptions`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Argument count from the caller at 004dad63 (`PUSH 1`, no stack cleanup after the call).
pub fn fn_009f9968(e: &mut Engine, options: u32) {
    e.call(IMPORT_SLOT_009F9968, &args![options]);
}

/// The slot `009fead0` jumps through.
const POINTER_CELL_009FEAD0: u32 = 0x011a_6238;

// Translated from 009fead0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6238]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6238` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db36f (`ADD ESP,0x8`); the second word is the address of a local, the first is 0; the result is tested against zero.
pub fn fn_009fead0(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAD0);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009fead6` jumps through.
const POINTER_CELL_009FEAD6: u32 = 0x011a_6240;

// Translated from 009fead6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6240]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6240` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db3e4 (`ADD ESP,0x8`); the first word is a loop index, the second the address of a local array element; the result is tested against zero.
pub fn fn_009fead6(e: &mut Engine, word_0: u32, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAD6);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009feae2` jumps through.
const POINTER_CELL_009FEAE2: u32 = 0x011a_6250;

// Translated from 009feae2 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6250]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6250` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db444 (`ADD ESP,0x8`); both words are addresses of locals; the result is tested against zero.
pub fn fn_009feae2(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAE2);
    e.call(target, &args![word_0, word_1]).u32()
}

/// The slot `009feae8` jumps through.
const POINTER_CELL_009FEAE8: u32 = 0x011a_6258;

// Translated from 009feae8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011a6258]`: unnamed in the engine map. The slot holds a function pointer (`0x011a6258` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 004db41a (`ADD ESP,0x8`); both words are addresses of locals; the result is tested against zero.
pub fn fn_009feae8(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> u32 {
    let target = e.global::<u32>(POINTER_CELL_009FEAE8);
    e.call(target, &args![word_0, word_1]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_9d50, fn_009f9d50(Ptr, Ptr) -> u32),
        entry!(0x009f_dd50, fn_009fdd50(Ptr, u32) -> u32),
        entry!(0x009f_9968, fn_009f9968(u32)),
        entry!(0x009f_ead0, fn_009fead0(u32, Ptr) -> u32),
        entry!(0x009f_ead6, fn_009fead6(u32, Ptr) -> u32),
        entry!(0x009f_eae2, fn_009feae2(Ptr, Ptr) -> u32),
        entry!(0x009f_eae8, fn_009feae8(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_009f9d50_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x009f_9d50;
        const TARGET: u32 = 0x009f_9c80;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 0x8007_000e,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(),
            0x8007_000e
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_009fdd50() {
        assert_eq!(
            check(0x009f_dd50, 0x009f_ddd0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod a06_thunk_tests {
    use super::*;

    /// Calls `thunk` with `words`, with a double over the import `slot` that
    /// returns `0xdeadbeef`; checks that the double ran once with exactly
    /// those words and that the thunk returned `expected`.
    fn import(thunk: u32, slot: u32, words: &[u32], expected: u32) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == slot).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

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
    fn thunk_009f9968() {
        import(0x009f_9968, IMPORT_SLOT_009F9968, &[0x1001], 0);
    }

    #[test]
    fn thunk_009fead0() {
        pointer(
            0x009f_ead0,
            POINTER_CELL_009FEAD0,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009fead6() {
        pointer(
            0x009f_ead6,
            POINTER_CELL_009FEAD6,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009feae2() {
        pointer(
            0x009f_eae2,
            POINTER_CELL_009FEAE2,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_009feae8() {
        pointer(
            0x009f_eae8,
            POINTER_CELL_009FEAE8,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }
}
