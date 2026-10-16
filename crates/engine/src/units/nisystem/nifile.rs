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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_1a20, ni_file_get_file(Ptr, u32, u32) -> Ptr)]
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
