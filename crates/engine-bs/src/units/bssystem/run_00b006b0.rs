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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
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
