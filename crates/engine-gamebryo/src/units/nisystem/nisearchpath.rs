//! `NiSystem/nisearchpath.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSearchPath::m_acFilePath` (Xbox PDB): the 0x104-byte file path at +8 (the same on PC).
const FILE_PATH: u32 = 0x8;
/// `_strncpy_s` (`00ec8d5f`, `cdecl`: destination, its size, source, count) and
/// `NiPath::Standardize` (`00aa2a00`, `cdecl`: the path).
const STRNCPY_S: u32 = 0x00ec_8d5f;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;

// Translated from 00aa2540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSearchPath::SetFilePath` (Xbox PDB), `thiscall`, one stack word (the path). A path that
/// is not null and not empty is copied into the file path at +8 (`_strncpy_s` with size 0x104
/// and count 0x103) and standardized there (`NiPath::Standardize`); otherwise the file path
/// is made empty.
pub fn ni_search_path_set_file_path(e: &mut Engine, this: Ptr, path: Ptr) {
    let path = path.addr();
    let file_path = this.addr().wrapping_add(FILE_PATH);
    if path != 0 && e.mem.u8(path) != 0 {
        e.call(STRNCPY_S, &args![file_path, 0x104u32, path, 0x103u32]);
        e.call(PATH_STANDARDIZE, &args![file_path]);
        return;
    }
    e.mem.set_u8(file_path, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_2540, ni_search_path_set_file_path(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(STRNCPY_S, |_, _| Ret::default());
        e.register(PATH_STANDARDIZE, |_, _| Ret::default());
        let this = e.mem.alloc(0x210);
        e.mem.set_u8(this + FILE_PATH, b'x');
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn ni_search_path_set_file_path_copies_and_standardizes_a_path() {
        let (mut e, this) = engine();
        let path = e.mem.alloc(8);
        e.mem.write(path, b"a\\b\0");
        e.call(0x00aa_2540, &args![this, path]);
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![this + 8, 0x104, path, 0x103]]
        );
        assert_eq!(calls(&e, PATH_STANDARDIZE), vec![vec![this + 8]]);
        assert_eq!(e.mem.u8(this + FILE_PATH), b'x');
    }

    #[test]
    fn ni_search_path_set_file_path_empties_for_a_null_or_empty_path() {
        let (mut e, this) = engine();
        e.call(0x00aa_2540, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + FILE_PATH), 0);
        e.mem.set_u8(this + FILE_PATH, b'x');
        let empty = e.mem.alloc(4);
        e.mem.set_u8(empty, 0);
        e.call(0x00aa_2540, &args![this, empty]);
        assert_eq!(e.mem.u8(this + FILE_PATH), 0);
        assert!(calls(&e, STRNCPY_S).is_empty());
        assert!(calls(&e, PATH_STANDARDIZE).is_empty());
    }
}
