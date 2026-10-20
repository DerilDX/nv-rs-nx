//! `BSSystem/filefinder.cpp` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Archive lookup: `(path, extra)`, cdecl, returns whether the file is in an
/// archive (the engine map places it in `BSSystem/archive.cpp`).
const ARCHIVE_CONTAINS: u32 = 0x00af_6320;
/// Thiscall on the finder with `(flags, extra)`: whether the loose-file
/// search applies (`00b01bc0`, 26 bytes).
const LOOKS_IN_LOOSE_FILES: u32 = 0x00b0_1bc0;
/// Thiscall on the finder with `(flags, extra)`: whether the search path
/// list applies (`00b01be0`, 26 bytes).
const LOOKS_IN_SEARCH_PATHS: u32 = 0x00b0_1be0;
/// `_access(path, mode)` (cdecl), -1 on failure.
const ACCESS: u32 = 0x00ec_bf05;
/// `strlen` (cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `strcpy_s(dest, size, source)` (cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `strcat_s(dest, size, source)` (cdecl).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `_strnicmp(a, b, count)` (cdecl, folded body `00ec7ec0`).
const STRNICMP: u32 = 0x00ec_7ec0;
/// A byte global; the search paths are only used when it is set and
/// [`SEARCH_PATHS_DISABLED`] is clear.
const SEARCH_PATHS_ENABLED: u32 = 0x011a_cdf8;
/// A byte global; when set the flags are cleared before the lookups.
const SEARCH_PATHS_DISABLED: u32 = 0x011a_cdf9;
/// Size of the path buffer the function builds a candidate in.
const PATH_BUFFER_SIZE: u32 = 0x104;

layout! {
    /// `FileFinder` (Xbox PDB), 0x10 bytes: the search-path array
    /// (`PathA`, a `BSSimpleArray<char const *,1024>`) at +0.
    pub struct FileFinder: 0x10 {
        /// `PathA.pBuffer` (PC +4): the array of path prefixes.
        0x04 pBuffer: u32,
        /// `PathA.iSize` (PC +8): number of entries.
        0x08 iSize: u32,
    }
}

// Translated from 00afe220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00afe220 - the /GS stack cookie check (00ec408c) at the exit is not translated
/// File lookup on a finder (the engine map gives no name): `path` is a
/// `char[0x104]` buffer, `flags` and `extra` are passed on to the lookup
/// helpers. Returns 2 when the archive lookup knows the path (skipped when
/// `flags` has bit 0), 1 when the path exists as given or, after a prefix
/// from the search-path array was prepended, as one of those (the path
/// buffer is then replaced by the prefixed path), 0 when nothing is found.
///
/// The decompiler drops the return values; they are read from the
/// disassembly.
pub fn fn_00afe220(
    e: &mut Engine,
    this: Ptr<FileFinder>,
    path: Ptr,
    flags: u32,
    extra: u32,
) -> u32 {
    if flags & 1 == 0 && e.call(ARCHIVE_CONTAINS, &args![path, extra]).bool() {
        return 2;
    }
    let mut flags = flags;
    if e.global::<u8>(SEARCH_PATHS_ENABLED) == 0 || e.global::<u8>(SEARCH_PATHS_DISABLED) != 0 {
        flags = 0;
    }
    if e.call(LOOKS_IN_LOOSE_FILES, &args![this, flags, extra])
        .bool()
        && e.call(ACCESS, &args![path, 0u32]).i32() != -1
    {
        return 1;
    }
    if !e
        .call(LOOKS_IN_SEARCH_PATHS, &args![this, flags, extra])
        .bool()
    {
        return 0;
    }
    let count = e.get(this, FileFinder::iSize);
    let mut index = 0u32;
    while index < count {
        let buffer = e.get(this, FileFinder::pBuffer);
        let prefix = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        if prefix != 0 {
            let length = e.call(STRLEN, &args![prefix]).u32();
            if e.call(STRNICMP, &args![path, prefix, length]).i32() != 0 {
                let found = e.with_stack(PATH_BUFFER_SIZE, |e, candidate| {
                    e.call(STRCPY_S, &args![candidate, PATH_BUFFER_SIZE, prefix]);
                    e.call(STRCAT_S, &args![candidate, PATH_BUFFER_SIZE, path]);
                    if e.call(ACCESS, &args![candidate, 0u32]).i32() != -1 {
                        e.call(STRCPY_S, &args![path, PATH_BUFFER_SIZE, candidate]);
                        true
                    } else {
                        false
                    }
                });
                if found {
                    return 1;
                }
            }
        }
        index += 1;
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00afe220,
        fn_00afe220(Ptr<FileFinder>, Ptr, u32, u32) -> u32
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A finder with the given prefixes, a path buffer holding `path`, and
    /// doubles for every callee: the archive knows `archive_hit`, the loose
    /// files `loose`, the search paths `searched`, and `_access` succeeds for
    /// the strings in `existing`. The log records `(address, first word)`.
    struct Scene {
        e: Engine,
        finder: Ptr<FileFinder>,
        path: Ptr,
        log: Rc<RefCell<Vec<u32>>>,
    }

    fn scene(
        prefixes: &[&[u8]],
        path: &[u8],
        archive_hit: bool,
        loose: bool,
        searched: bool,
        existing: &'static [&'static [u8]],
    ) -> Scene {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        let finder: Ptr<FileFinder> = e.new_object();
        let table = e.mem.alloc(4 * prefixes.len() as u32 + 4);
        for (i, prefix) in prefixes.iter().enumerate() {
            let text = e.mem.alloc(0x40);
            e.mem.set_cstr(text, prefix);
            e.mem.set_u32(table + 4 * i as u32, text);
        }
        e.set(finder, FileFinder::pBuffer, table);
        e.set(finder, FileFinder::iSize, prefixes.len() as u32);
        let path_buffer = e.mem.alloc(PATH_BUFFER_SIZE);
        e.mem.set_cstr(path_buffer, path);
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = log.clone();
        e.register_double(ARCHIVE_CONTAINS, move |_, _| {
            seen.borrow_mut().push(ARCHIVE_CONTAINS);
            Ret {
                eax: archive_hit as u32,
                ..Ret::default()
            }
        });
        let seen = log.clone();
        e.register_double(LOOKS_IN_LOOSE_FILES, move |_, a| {
            seen.borrow_mut().push(LOOKS_IN_LOOSE_FILES + a[1]);
            Ret {
                eax: loose as u32,
                ..Ret::default()
            }
        });
        let seen = log.clone();
        e.register_double(LOOKS_IN_SEARCH_PATHS, move |_, a| {
            seen.borrow_mut().push(LOOKS_IN_SEARCH_PATHS + a[1]);
            Ret {
                eax: searched as u32,
                ..Ret::default()
            }
        });
        e.register_double(ACCESS, move |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = existing.iter().any(|name| name[..] == text[..]);
            Ret {
                eax: if found { 0 } else { u32::MAX },
                ..Ret::default()
            }
        });
        e.register(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRNICMP, |e, a| {
            let first = e.mem.cstr(a[0]);
            let second = e.mem.cstr(a[1]);
            let n = a[2] as usize;
            let lower = |text: &[u8]| {
                text.iter()
                    .take(n)
                    .map(u8::to_ascii_lowercase)
                    .collect::<Vec<_>>()
            };
            Ret {
                eax: (lower(&first) != lower(&second)) as u32,
                ..Ret::default()
            }
        });
        Scene {
            e,
            finder,
            path: Ptr::new(path_buffer),
            log,
        }
    }

    fn find(s: &mut Scene, flags: u32) -> u32 {
        let (finder, path) = (s.finder, s.path);
        s.e.call(0x00af_e220, &args![finder, path, flags, 7u32])
            .u32()
    }

    #[test]
    fn an_archive_hit_returns_two_without_touching_the_disk() {
        let mut s = scene(&[], b"meshes\\a.nif", true, true, true, &[b"meshes\\a.nif"]);
        assert_eq!(find(&mut s, 0), 2);
        assert_eq!(*s.log.borrow(), vec![ARCHIVE_CONTAINS]);
    }

    #[test]
    fn flag_one_skips_the_archive_lookup() {
        let mut s = scene(&[], b"a.nif", true, true, false, &[b"a.nif"]);
        // Bit 0 of the flags skips the archive; the search paths are enabled,
        // so the flags are handed on unchanged.
        assert_eq!(find(&mut s, 1), 1);
        assert_eq!(*s.log.borrow(), vec![LOOKS_IN_LOOSE_FILES + 1]);
    }

    #[test]
    fn the_flags_survive_when_the_search_paths_are_enabled() {
        let mut s = scene(&[], b"a.nif", false, true, false, &[b"a.nif"]);
        s.e.set_global(SEARCH_PATHS_ENABLED, 1u8);
        assert_eq!(find(&mut s, 6), 1);
        assert_eq!(
            *s.log.borrow(),
            vec![ARCHIVE_CONTAINS, LOOKS_IN_LOOSE_FILES + 6]
        );
        s.e.set_global(SEARCH_PATHS_DISABLED, 1u8);
        s.log.borrow_mut().clear();
        assert_eq!(find(&mut s, 6), 1);
        assert_eq!(
            *s.log.borrow(),
            vec![ARCHIVE_CONTAINS, LOOKS_IN_LOOSE_FILES]
        );
    }

    #[test]
    fn a_prefix_is_prepended_and_the_path_replaced() {
        let mut s = scene(
            &[b"", b"data\\"],
            b"a.nif",
            false,
            false,
            true,
            &[b"data\\a.nif"],
        );
        assert_eq!(find(&mut s, 0), 1);
        assert_eq!(s.e.mem.cstr(s.path.addr()), b"data\\a.nif");
    }

    #[test]
    fn a_path_already_starting_with_the_prefix_is_not_prefixed_again() {
        let mut s = scene(
            &[b"data\\"],
            b"data\\a.nif",
            false,
            false,
            true,
            &[b"data\\data\\a.nif"],
        );
        assert_eq!(find(&mut s, 0), 0);
        assert_eq!(s.e.mem.cstr(s.path.addr()), b"data\\a.nif");
    }

    #[test]
    fn nothing_found_returns_zero() {
        let mut s = scene(&[b"x\\"], b"a.nif", false, false, true, &[]);
        assert_eq!(find(&mut s, 0), 0);
        let mut s = scene(&[b"x\\"], b"a.nif", false, false, false, &[]);
        assert_eq!(find(&mut s, 0), 0);
    }
}
