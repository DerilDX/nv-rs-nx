//! `NiSystem/nipath.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const SLASH: u8 = b'/';
const BACKSLASH: u8 = b'\\';

// Translated from 00aa2a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPath::Standardize` (engine map): rewrites the C string at `path` in place
/// (`cdecl`, one stack word). Every `/` becomes a backslash, and a backslash
/// that follows a backslash is dropped, except that the first two characters
/// are never dropped (so a leading double backslash of a network path stays).
/// Returns true when it changed a character or dropped one (`AL`, set by a `/`
/// or a dropped backslash), false for a null pointer or an unchanged path.
pub fn ni_path_standardize(e: &mut Engine, path: Ptr) -> bool {
    if path.is_null() {
        return false;
    }
    let text = e.mem.cstr(path.addr());
    let length = text.len() as u32;
    let mut changed = false;
    let mut out = 0u32;
    let mut after_backslash = false;
    let mut index = 0u32;
    while index < length {
        let mut c = text[index as usize];
        if c == SLASH {
            changed = true;
            c = BACKSLASH;
        }
        let is_backslash = c == BACKSLASH;
        if is_backslash && after_backslash && index > 1 {
            changed = true;
        } else {
            e.mem.set_u8(path.addr().wrapping_add(out), c);
            after_backslash = is_backslash;
            out += 1;
        }
        index += 1;
    }
    e.mem.set_u8(path.addr().wrapping_add(out), 0);
    changed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_2a00, ni_path_standardize(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standardize(text: &[u8]) -> (bool, Vec<u8>) {
        let mut e = Engine::new();
        let buffer = e.mem.alloc(0x40);
        e.mem.set_cstr(buffer, text);
        let changed = e.call(0x00aa_2a00, &args![buffer]).bool();
        (changed, e.mem.cstr(buffer))
    }

    #[test]
    fn a_null_path_gives_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00aa_2a00, &args![0u32]).bool());
    }

    #[test]
    fn a_clean_path_is_unchanged() {
        assert_eq!(
            standardize(b"meshes\\a.nif"),
            (false, b"meshes\\a.nif".to_vec())
        );
        assert_eq!(standardize(b""), (false, Vec::new()));
    }

    #[test]
    fn slashes_become_backslashes() {
        assert_eq!(
            standardize(b"meshes/a/b.nif"),
            (true, b"meshes\\a\\b.nif".to_vec())
        );
    }

    #[test]
    fn repeated_backslashes_collapse_after_the_first_two_characters() {
        assert_eq!(standardize(b"a\\\\b\\\\\\c"), (true, b"a\\b\\c".to_vec()));
        assert_eq!(standardize(b"x//y"), (true, b"x\\y".to_vec()));
    }

    #[test]
    fn a_leading_double_backslash_is_kept() {
        assert_eq!(
            standardize(b"\\\\srv\\\\share"),
            (true, b"\\\\srv\\share".to_vec())
        );
        assert_eq!(standardize(b"//srv"), (true, b"\\\\srv".to_vec()));
        assert_eq!(standardize(b"\\\\srv"), (false, b"\\\\srv".to_vec()));
    }
}
