//! `NiMain/nistring.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ecbf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed body (the map puts `NiString::FindOneOf` on it, but it is the CRT's `strpbrk`,
/// cdecl, two words): the first character of `string` that occurs in `characters`, or null.
/// The code marks each byte of `characters` in a 256-bit set on its stack (the closing 0 is
/// never marked), then scans `string` until a marked byte or its end.
pub fn fn_00ecbf20(e: &mut Engine, string: Ptr, characters: Ptr) -> Ptr {
    let mut set = [false; 256];
    let mut p = characters.addr();
    loop {
        let byte = e.mem.u8(p);
        if byte == 0 {
            break;
        }
        p = p.wrapping_add(1);
        set[usize::from(byte)] = true;
    }
    let mut p = string.addr();
    loop {
        let byte = e.mem.u8(p);
        if byte == 0 {
            return Ptr::new(0);
        }
        p = p.wrapping_add(1);
        if set[usize::from(byte)] {
            return Ptr::new(p.wrapping_sub(1));
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ecbf20, fn_00ecbf20(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn string(e: &mut Engine, bytes: &[u8]) -> u32 {
        let p = e.mem.alloc(bytes.len() as u32 + 1);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(p + i as u32, *b);
        }
        p
    }

    #[test]
    fn fn_00ecbf20_finds_the_first_listed_character() {
        let mut e = Engine::new();
        let s = string(&mut e, b"textures\\a/b.dds");
        let separators = string(&mut e, b"/\\");
        let none = string(&mut e, b"#");
        let empty = string(&mut e, b"");
        assert_eq!(e.call(0x00ecbf20, &args![s, separators]).u32(), s + 8);
        assert_eq!(e.call(0x00ecbf20, &args![s, none]).u32(), 0);
        assert_eq!(e.call(0x00ecbf20, &args![s, empty]).u32(), 0);
        assert_eq!(e.call(0x00ecbf20, &args![empty, separators]).u32(), 0);
    }
}
