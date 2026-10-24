//! `BSSystem/run_00b006b0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00afeac0` (`thiscall`, `RET 4`): opens the file named by its argument for the object and
/// returns the word stored at +0.
const OPEN_FOR: u32 = 0x00af_eac0;
/// `00aff090` (`thiscall`, no stack word): closes the handle at +0x10.
const CLOSE_FOR: u32 = 0x00af_f090;

// Translated from 00b00900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): a constructor. Clears the word at +0,
/// stores `second` at +4, `third` at +8 and the low byte of `fourth` at +0xc, then stores at
/// +0 what `00afeac0` returns for `first`. Returns `this`. (What `00afeac0` does with `first`
/// is not confirmed here.)
pub fn fn_00b00900(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, second);
    e.mem.set_u32(base + 8, third);
    e.mem.set_u8(base + 0xc, fourth as u8);
    let result = e.call(OPEN_FOR, &args![this, first]).u32();
    e.mem.set_u32(base, result);
    this
}

// Translated from 00b00950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `00aff090` on `this`.
pub fn fn_00b00950(e: &mut Engine, this: Ptr) {
    e.call(CLOSE_FOR, &args![this]);
}

/// Import slots (KERNEL32): `FindFirstFileA` and `FindNextFileA`.
const FIND_FIRST_FILE: u32 = 0x00fd_f070;
const FIND_NEXT_FILE: u32 = 0x00fd_f068;
/// `00b006b0` (`thiscall`, `RET 4`): what [`fn_00b00820`] runs on its argument after
/// clearing its words (not translated here; what it does with the argument is not
/// confirmed).
const SET_UP_SEARCH: u32 = 0x00b0_06b0;
/// The object's words: the search handle (+0), the `WIN32_FIND_DATAA` buffer (+4), the
/// search pattern (+0x144) and the status word (+0x248).
const FIND_DATA: u32 = 0x4;
const PATTERN: u32 = 0x144;
const STATUS: u32 = 0x248;

// Translated from 00b00720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): one step of a directory search. With no
/// handle yet (+0 is 0) it starts the search (`FindFirstFileA` on the pattern at +0x144 into
/// the buffer at +4), stores the handle and sets the status (+0x248) to 4 when the handle is
/// `INVALID_HANDLE_VALUE`, else 0. With an invalid handle it sets the status to 4. Otherwise
/// it asks for the next file (`FindNextFileA`) and stores at `finished` 1 when that fails,
/// 0 when it succeeds, leaving the status. Returns the status.
pub fn fn_00b00720(e: &mut Engine, this: Ptr, finished: Ptr) -> u32 {
    let base = this.addr();
    let handle = e.mem.u32(base);
    if handle == 0 {
        let found = e
            .call(FIND_FIRST_FILE, &args![base + PATTERN, base + FIND_DATA])
            .u32();
        e.mem.set_u32(base, found);
        let status = if e.mem.u32(base) == u32::MAX { 4 } else { 0 };
        e.mem.set_u32(base + STATUS, status);
        e.mem.u32(base + STATUS)
    } else if handle == u32::MAX {
        e.mem.set_u32(base + STATUS, 4);
        e.mem.u32(base + STATUS)
    } else {
        let handle = e.mem.u32(base);
        let next = e
            .call(FIND_NEXT_FILE, &args![handle, base + FIND_DATA])
            .u32();
        e.mem.set_u8(finished.addr(), u8::from(next == 0));
        e.mem.u32(base + STATUS)
    }
}

// Translated from 00b00820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a constructor. Clears the words at
/// +0x248, +0x250 and +0x254, then runs `00b006b0` on `this` with `argument`. Returns
/// `this`.
pub fn fn_00b00820(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + STATUS, 0);
    e.mem.set_u32(base + 0x250, 0);
    e.mem.set_u32(base + 0x254, 0);
    e.call(SET_UP_SEARCH, &args![this, argument]);
    this
}

/// `_strcpy_s` and `_strcat_s` (`00ec65a6`, `00ec6bd2`; cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `"\*"` (`010a47b0`) and the empty string just after it (`010a47b3`).
const ALL_FILES_SUFFIX: u32 = 0x010a_47b0;
const EMPTY_TEXT: u32 = 0x010a_47b3;
/// Import slot: `CreateDirectoryA` (KERNEL32).
const CREATE_DIRECTORY: u32 = 0x00fd_f0b8;
/// The word `00b00970` stores (`011f8250`).
const STORED_VALUE_011F8250: u32 = 0x011f_8250;

// Translated from 00b006b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a search's set-up. Answers 2 for a null
/// `directory`; otherwise the pattern at +0x144 becomes `directory` followed by `"\*"`
/// (`strcpy_s`, `strcat_s`, 0x104 each), the handle (+0) and the status (+0x248) are cleared
/// and the status (0) is answered.
pub fn fn_00b006b0(e: &mut Engine, this: Ptr, directory: u32) -> u32 {
    if directory == 0 {
        return 2;
    }
    let base = this.addr();
    e.call(STRCPY_S, &args![base + PATTERN, 0x104u32, directory]);
    e.call(STRCAT_S, &args![base + PATTERN, 0x104u32, ALL_FILES_SUFFIX]);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + STATUS, 0);
    e.mem.u32(base + STATUS)
}

// Translated from 00b00800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): `CreateDirectoryA(path, null)`; answers 0
/// when it succeeds, 4 when it fails.
pub fn fn_00b00800(e: &mut Engine, path: u32) -> u32 {
    let made = e.call(CREATE_DIRECTORY, &args![path, 0u32]).u32();
    if made != 0 {
        0
    } else {
        4
    }
}

// Translated from 00b00880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, no callers found): clears the byte at +4 and
/// answers `this`.
pub fn fn_00b00880(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr().wrapping_add(4), 0);
    this
}

// Translated from 00b008a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, called from `00af2a60`): does nothing.
pub fn fn_00b008a0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00b008b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): stores `value` at +0 and answers `this`.
pub fn fn_00b008b0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 00b008d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall): answers the word at +0.
pub fn fn_00b008d0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr())
}

// Translated from 00b008e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `this` not read): answers the empty string at
/// `010a47b3`.
pub fn fn_00b008e0(_e: &mut Engine, _this: Ptr) -> u32 {
    EMPTY_TEXT
}

// Translated from 00b008f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `this` not read, no callers found): answers 0.
pub fn fn_00b008f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00b00970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word, no callers found): stores `value` in
/// `011f8250` and answers it.
pub fn fn_00b00970(e: &mut Engine, value: u32) -> u32 {
    e.set_global(STORED_VALUE_011F8250, value);
    value
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_06b0, fn_00b006b0(Ptr, u32) -> u32),
        entry!(0x00b0_0800, fn_00b00800(u32) -> u32),
        entry!(0x00b0_0880, fn_00b00880(Ptr) -> Ptr),
        entry!(0x00b0_08a0, fn_00b008a0(Ptr)),
        entry!(0x00b0_08b0, fn_00b008b0(Ptr, u32) -> Ptr),
        entry!(0x00b0_08d0, fn_00b008d0(Ptr) -> u32),
        entry!(0x00b0_08e0, fn_00b008e0(Ptr) -> u32),
        entry!(0x00b0_08f0, fn_00b008f0(Ptr) -> u32),
        entry!(0x00b0_0970, fn_00b00970(u32) -> u32),
        entry!(0x00b0_0900, fn_00b00900(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00b0_0950, fn_00b00950(Ptr)),
        entry!(0x00b0_0720, fn_00b00720(Ptr, Ptr) -> u32),
        entry!(0x00b0_0820, fn_00b00820(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_words_then_the_open_result() {
        let mut e = Engine::new();
        e.register(OPEN_FOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0);
            0x99u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, 0x55);
        e.call_log = Some(vec![]);
        let made = e
            .call(
                0x00b0_0900,
                &args![this, 0x10u32, 0x20u32, 0x30u32, 0x1234_5601u32],
            )
            .u32();
        assert_eq!(made, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(OPEN_FOR, vec![this, 0x10])]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)],
            [0x99, 0x20, 0x30]
        );
        assert_eq!(e.mem.u8(this + 0xc), 1);
    }

    #[test]
    fn fn_00b00950_closes_through_00aff090() {
        let mut e = Engine::new();
        e.register(CLOSE_FOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b0_0950, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(CLOSE_FOR, vec![0x7000])]);
    }

    fn search_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FIND_FIRST_FILE, |e, a| {
            // An invalid handle when the pattern starts with the word 1, else 0x44.
            (if e.mem.u32(a[0]) == 1 {
                u32::MAX
            } else {
                0x44u32
            })
            .into_ret()
        });
        e.register(FIND_NEXT_FILE, |_, a| u32::from(a[0] == 0x44).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn search_step_starts_continues_and_fails() {
        let mut e = search_engine();
        let this = e.mem.alloc(0x258);
        let finished = e.mem.alloc(4);
        e.mem.set_u32(this + STATUS, 9);
        assert_eq!(e.call(0x00b0_0720, &args![this, finished]).u32(), 0);
        assert_eq!(e.mem.u32(this), 0x44);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FIND_FIRST_FILE, vec![this + 0x144, this + 4]));
        // The next step succeeds: `finished` 0, the status kept.
        e.mem.set_u8(finished, 7);
        e.mem.set_u32(this + STATUS, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_0720, &args![this, finished]).u32(), 6);
        assert_eq!(e.mem.u8(finished), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FIND_NEXT_FILE, vec![0x44, this + 4]));
        // A failing step sets `finished`.
        e.mem.set_u32(this, 0x45);
        assert_eq!(e.call(0x00b0_0720, &args![this, finished]).u32(), 6);
        assert_eq!(e.mem.u8(finished), 1);
        // An invalid handle: status 4, nothing asked.
        e.mem.set_u32(this, u32::MAX);
        e.mem.set_u8(finished, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_0720, &args![this, finished]).u32(), 4);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u8(finished), 7);
    }

    #[test]
    fn search_start_with_an_invalid_handle_sets_status_4() {
        let mut e = search_engine();
        let this = e.mem.alloc(0x258);
        e.mem.set_u32(this + PATTERN, 1);
        let finished = e.mem.alloc(4);
        assert_eq!(e.call(0x00b0_0720, &args![this, finished]).u32(), 4);
        assert_eq!(e.mem.u32(this), u32::MAX);
        assert_eq!(e.mem.u32(this + STATUS), 4);
    }

    #[test]
    fn search_constructor_clears_then_sets_up() {
        let mut e = Engine::new();
        e.register(SET_UP_SEARCH, |_, _| Ret::default());
        let this = e.mem.alloc(0x258);
        for offset in [0x248u32, 0x24c, 0x250, 0x254] {
            e.mem.set_u32(this + offset, 0x99);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_0820, &args![this, 0x1234u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SET_UP_SEARCH, vec![this, 0x1234])]);
        assert_eq!(
            [0x248u32, 0x24c, 0x250, 0x254].map(|o| e.mem.u32(this + o)),
            [0, 0x99, 0, 0]
        );
    }
}

#[cfg(test)]
mod n083_tests {
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
    fn search_set_up_builds_the_pattern() {
        let mut e = Engine::new();
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(STRCAT_S, |_, _| Ret::default());
        let this = e.mem.alloc(0x258);
        e.mem.set_u32(this, 7);
        e.mem.set_u32(this + STATUS, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_06b0, &args![this, 0u32]).u32(), 2);
        assert!(calls_to(&e, STRCPY_S).is_empty());
        assert_eq!(e.mem.u32(this), 7);
        assert_eq!(e.call(0x00b0_06b0, &args![this, 0x5000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, STRCPY_S),
            vec![vec![this + PATTERN, 0x104, 0x5000]]
        );
        assert_eq!(
            calls_to(&e, STRCAT_S),
            vec![vec![this + PATTERN, 0x104, ALL_FILES_SUFFIX]]
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + STATUS), 0);
    }

    #[test]
    fn create_directory_answers_0_or_4() {
        let mut e = Engine::new();
        e.register(CREATE_DIRECTORY, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b0_0800, &args![0x5000u32]).u32(), 0);
        assert_eq!(calls_to(&e, CREATE_DIRECTORY), vec![vec![0x5000, 0]]);
        e.register(CREATE_DIRECTORY, |_, _| Ret::default());
        assert_eq!(e.call(0x00b0_0800, &args![0x5000u32]).u32(), 4);
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        assert_eq!(e.call(0x00b0_0880, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff00);
        e.call(0x00b0_08a0, &args![this]);
        assert_eq!(e.call(0x00b0_08b0, &args![this, 0x77u32]).u32(), this);
        assert_eq!(e.call(0x00b0_08d0, &args![this]).u32(), 0x77);
        assert_eq!(e.call(0x00b0_08e0, &args![this]).u32(), EMPTY_TEXT);
        assert_eq!(e.call(0x00b0_08f0, &args![this]).u32(), 0);
        e.map(STORED_VALUE_011F8250, 4);
        assert_eq!(e.call(0x00b0_0970, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(STORED_VALUE_011F8250), 0x1234);
    }
}
