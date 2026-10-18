//! `fallout shared/tesattributes.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memcpy` wrapper (`00401460`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x0040_1460;
/// The folded `return 7` (`009d2510`, `ECX` = attributes): the size of the
/// attribute data.
const ATTRIBUTES_SIZE: u32 = 0x009d_2510;

// Translated from 00480130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `TESAttributes` data from a buffer (unnamed in the map; `ECX`,
/// one stack word, `RET 4`): copies 7 bytes from the source to `+4`
/// (`00401460`) and answers the size `009d2510` gives, the number of bytes
/// used (`TESForm::LoadData` advances by it).
pub fn fn_00480130(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    e.call(MEMCPY, &args![this.byte_add(4), source, 7u32]);
    e.call(ATTRIBUTES_SIZE, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00480130, fn_00480130(Ptr, Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_seven_bytes_and_answers_the_size() {
        let mut e = Engine::new();
        e.register(MEMCPY, |_, _| Ret::default());
        e.register(ATTRIBUTES_SIZE, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(0x0400_0000);
        let source = Ptr::<()>::new(0x0500_0000);
        let used = e.call(0x0048_0130, &args![this, source]).u32();
        assert_eq!(used, 9);
        let calls: Vec<(u32, Vec<u32>)> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MEMCPY || c.0 == ATTRIBUTES_SIZE)
            .collect();
        assert_eq!(
            calls,
            vec![
                (MEMCPY, vec![0x0400_0004, 0x0500_0000, 7]),
                (ATTRIBUTES_SIZE, vec![0x0400_0000]),
            ]
        );
    }
}
