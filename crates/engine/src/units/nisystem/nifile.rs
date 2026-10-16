//! `NiSystem/nifile.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function pointer `NiFile::GetFile` jumps through (`011ac224`): `00aa1b90`
/// sets it to `00aa1b60`, or to the function it is given.
const GET_FILE_POINTER: u32 = 0x011a_c224;

// Translated from 00aa1a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFile::GetFile` (engine map): `cdecl`, `JMP [011ac224]`, so the three stack
/// words (`name`, `mode`, `buffer_size`) go unchanged to the function the
/// pointer holds. Returns that function's result, a pointer to the file object
/// (or null).
pub fn ni_file_get_file(e: &mut Engine, name: Ptr, mode: u32, buffer_size: u32) -> Ptr {
    let target = e.global::<u32>(GET_FILE_POINTER);
    e.call(target, &args![name, mode, buffer_size]).ptr()
}

/// The slot `00aa1a30` jumps through.
const POINTER_CELL_00AA1A30: u32 = 0x011a_c228;

// Translated from 00aa1a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `JMP dword ptr [0x011ac228]`: `NiFile::Access` (engine map). The slot holds a function pointer (`0x011ac228` is a data cell, not an import slot); the 2 stack word(s) (cdecl (the caller pops its words)) go unchanged to the function it holds. Argument count from the caller at 00a60dd7 (`ADD ESP,0x8`); the cell starts out holding `00aa1a40`, a function that returns `AL`; the meaning of the second word is unconfirmed.
pub fn fn_00aa1a30(e: &mut Engine, path: Ptr, mode: u32) -> bool {
    let target = e.global::<u32>(POINTER_CELL_00AA1A30);
    e.call(target, &args![path, mode]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_1a20, ni_file_get_file(Ptr, u32, u32) -> Ptr),
        entry!(0x00aa_1a30, fn_00aa1a30(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_jumps_through_the_pointer_with_its_three_words() {
        const TARGET: u32 = 0x00aa_1b60;
        let mut e = Engine::new();
        e.map(GET_FILE_POINTER & !0xfff, 0x1000);
        e.set_global(GET_FILE_POINTER, TARGET);
        e.register(TARGET, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_1a20, &args![0x1000u32, 1u32, 0x8000u32])
                .u32(),
            0x5000
        );
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 1u32, 0x8000u32]);
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
    fn thunk_00aa1a30() {
        pointer(0x00aa_1a30, POINTER_CELL_00AA1A30, &[0x1001, 0x1002], 1);
    }
}
