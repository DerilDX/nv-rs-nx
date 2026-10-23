//! `(unplaced)/run_0084fbe0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BSStringT` constructor (`004037b0`, `__thiscall`).
const STRING_CTOR: u32 = 0x0040_37b0;
/// The global pointer `0084fc70` fills with the object `0084fda0` builds
/// (`011de134`). The save and load code reads it (`SaveGame`, `LoadData`,
/// `UpdateQueuedSaves`, `IsAutosave` in Ghidra's names); its class is not
/// named in the exe.
const SAVE_LOAD_OBJECT: u32 = 0x011d_e134;
/// A function-local static singleton getter in `BSSystemUtilities`
/// (`00af2e50`, no arguments): builds the object at `011f72a0` with
/// `00af2ed0` on its first call and returns `011f72a0`.
const SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2e50;
/// The string `"-SAVE-"` (`0107fabc`).
const SAVE_OPTION: u32 = 0x0107_fabc;
/// Stores a word at `+0x44` (`008d8040`, `RET 4`; the map names it
/// `LowProcess::SetGenericLocation`, a shared one-word setter body).
const SET_WORD_AT_44: u32 = 0x008d_8040;
/// The function `0084fc70` stores at `+0x44` of the singleton (`008527d0`).
const CALLBACK_008527D0: u32 = 0x0085_27d0;

// Translated from 0084fc70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0084fc70 - the C++ exception frame (FS:[0] handler 00eed64b, state 0 that frees the 0x38-byte block when its constructor throws) is not translated.
/// Called from `TES::TES` (no arguments, plain `RET`): allocates the 0x38-byte
/// object, constructs it with [`fn_0084fda0`] (null when the allocation
/// fails) and stores it in the global at `011de134`. Then asks the
/// `00af2e50` singleton, through its vtable slot `+0xC` with
/// (`"-SAVE-"`, 0, 0), and stores whether the answer is non-zero in the
/// byte at `+0x1C` of that object (`SETA` after `TEST EAX,EAX`). Last, it
/// gives the singleton the function `008527d0` through `008d8040` (the
/// word at its `+0x44`).
pub fn fn_0084fc70(e: &mut Engine) {
    let block = e.call(OPERATOR_NEW, &args![0x38u32]).u32();
    let object = if block == 0 {
        0
    } else {
        fn_0084fda0(e, Ptr::new(block)).addr()
    };
    e.mem.set_u32(SAVE_LOAD_OBJECT, object);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
    let answer = e
        .vcall(utility, 0x0c, &args![SAVE_OPTION, 0u32, 0u32])
        .u32();
    // The exe reads the global again; a null object faults there as in the
    // game (the write goes to address 0x1C).
    let object = e.mem.u32(SAVE_LOAD_OBJECT);
    e.mem
        .set_u8(object.wrapping_add(0x1c), u8::from(answer != 0));
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
    e.call(SET_WORD_AT_44, &args![utility, CALLBACK_008527D0]);
}

// Translated from 0084fda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x38-byte object `0084fc70` makes (`__thiscall`,
/// plain `RET`; no vtable, class not named in the exe): zeroes the words at
/// +0, +4, +8, +0x14, +0x20, +0x28, +0x2C and the bytes at +0xC..+0x11,
/// +0x1C, +0x25, +0x26; sets the word at +0x18 to -1 and the byte at +0x24
/// to 1; constructs the `BSStringT` at +0x30 (`004037b0`). Returns `this`.
pub fn fn_0084fda0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 0x4, 0);
    e.mem.set_u32(t + 0x8, 0);
    e.mem.set_u8(t + 0xc, 0);
    e.mem.set_u8(t + 0xd, 0);
    e.mem.set_u8(t + 0xe, 0);
    e.mem.set_u8(t + 0xf, 0);
    e.mem.set_u8(t + 0x10, 0);
    e.mem.set_u8(t + 0x11, 0);
    e.mem.set_u32(t + 0x14, 0);
    e.mem.set_u32(t + 0x18, 0xffff_ffff);
    e.mem.set_u8(t + 0x1c, 0);
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u8(t + 0x24, 1);
    e.mem.set_u8(t + 0x25, 0);
    e.mem.set_u8(t + 0x26, 0);
    e.mem.set_u32(t + 0x28, 0);
    e.mem.set_u32(t + 0x2c, 0);
    e.call(STRING_CTOR, &args![t + 0x30]);
    this
}

/// `0084fd70` (thiscall, one word: the delete flags; `RET 4`): the object's deleting
/// destructor (`0084fe50`, then `operator delete` when bit 0 is set).
const OBJECT_DELETE: u32 = 0x0084_fd70;

// Translated from 0084fd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the counterpart of [`fn_0084fc70`]. Deletes the
/// object in `011de134` (`0084fd70` with 1) when there is one, then clears the global.
pub fn fn_0084fd30(e: &mut Engine) {
    let object = e.mem.u32(SAVE_LOAD_OBJECT);
    if object != 0 {
        e.call(OBJECT_DELETE, &args![object, 1u32]);
    }
    e.mem.set_u32(SAVE_LOAD_OBJECT, 0);
}

/// The base path (`004dc110`, no arguments: answers `01202fa0`).
const BASE_PATH: u32 = 0x004d_c110;
/// `strcpy_s` and `strcat_s` wrappers (`00406d30`, `00406d50`; cdecl: buffer, size, text).
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
/// The `sLocalSavePath:General` setting (`011c3f74`; its initializer passes that name and
/// `"Saves\"`), its string through `00403df0` (thiscall, no stack words) and `00464f30`
/// (cdecl, two words: answers the first).
const LOCAL_SAVE_PATH_SETTING: u32 = 0x011c_3f74;
const SETTING_STRING: u32 = 0x0040_3df0;
const FIRST_ARGUMENT: u32 = 0x0046_4f30;
/// `CreateDirectoryA` (KERNEL32 import slot; stdcall: path, security attributes).
const CREATE_DIRECTORY: u32 = 0x00fd_f0b8;
/// The size of the path buffer (`MAX_PATH`).
const PATH_SIZE: u32 = 0x104;

// Translated from 0084ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word: the path buffer; `RET 4`; `this` is not
/// read): writes the base path (`004dc110`) into `path` with `strcpy_s`, appends the
/// `sLocalSavePath:General` string with `strcat_s` (both with size 0x104), and creates that
/// directory (`CreateDirectoryA(path, null)`).
pub fn fn_0084ff30(e: &mut Engine, _this: Ptr, path: u32) {
    let base = e.call(BASE_PATH, &args![]).u32();
    e.call(STRING_COPY, &args![path, PATH_SIZE, base]);
    let text = e
        .call(SETTING_STRING, &args![LOCAL_SAVE_PATH_SETTING])
        .u32();
    let text = e.call(FIRST_ARGUMENT, &args![text, 0u32]).u32();
    e.call(STRING_APPEND, &args![path, PATH_SIZE, text]);
    e.call(CREATE_DIRECTORY, &args![path, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084_fd30, fn_0084fd30()),
        entry!(0x0084_ff30, fn_0084ff30(Ptr, u32)),
        entry!(0x0084fc70, fn_0084fc70()),
        entry!(0x0084fda0, fn_0084fda0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Vtable slot `+0xC` of the singleton, as the tests place it.
    const QUERY: u32 = 0x7000_0000;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn returning(e: &mut Engine, log: &Log, addr: u32, eax: u32) {
        let l = log.clone();
        e.register_double(addr, move |_, a| {
            l.borrow_mut().push((addr, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// Doubles `operator new` (returning a fresh 0x38-byte block, or null
    /// when `allocate` is false), the string constructor, the singleton
    /// getter (whose slot `+0xC` answers `answer`) and the `+0x44` setter.
    /// Returns the engine, the block, the singleton and the call log.
    fn setup(allocate: bool, answer: u32) -> (Engine, u32, u32, Log) {
        let mut e = Engine::new();
        e.mem.map(SAVE_LOAD_OBJECT, 4);
        let block = if allocate { e.mem.alloc(0x38) } else { 0 };
        let utility = e.mem.alloc(0x48);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(utility, vtable);
        e.mem.set_u32(vtable + 0xc, QUERY);
        let log: Log = Rc::new(RefCell::new(vec![]));
        returning(&mut e, &log, OPERATOR_NEW, block);
        returning(&mut e, &log, SYSTEM_UTILITY_INSTANCE, utility);
        returning(&mut e, &log, QUERY, answer);
        returning(&mut e, &log, SET_WORD_AT_44, 0);
        let l = log.clone();
        e.register_double(STRING_CTOR, move |_, a| {
            l.borrow_mut().push((STRING_CTOR, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        (e, block, utility, log)
    }

    #[test]
    fn makes_the_object_and_records_the_save_option() {
        let (mut e, block, utility, log) = setup(true, 0x100);
        e.call(0x0084_fc70, &args![]);
        assert_eq!(calls(&log, OPERATOR_NEW), [vec![0x38]]);
        assert_eq!(e.mem.u32(SAVE_LOAD_OBJECT), block);
        assert_eq!(calls(&log, STRING_CTOR), [vec![block + 0x30]]);
        assert_eq!(calls(&log, SYSTEM_UTILITY_INSTANCE).len(), 2);
        assert_eq!(calls(&log, QUERY), [vec![utility, SAVE_OPTION, 0, 0]]);
        // 0x100 has a zero low byte but is non-zero: the flag is set.
        assert_eq!(e.mem.u8(block + 0x1c), 1);
        assert_eq!(
            calls(&log, SET_WORD_AT_44),
            [vec![utility, CALLBACK_008527D0]]
        );
    }

    #[test]
    fn a_zero_answer_clears_the_flag() {
        let (mut e, block, _, _) = setup(true, 0);
        e.call(0x0084_fc70, &args![]);
        assert_eq!(e.mem.u8(block + 0x1c), 0);
    }

    #[test]
    fn a_failed_allocation_stores_null_and_constructs_nothing() {
        let (mut e, _, _, log) = setup(false, 1);
        // The byte write then goes to address 0x1C, as in the game.
        e.mem.map(0, 0x20);
        e.call(0x0084_fc70, &args![]);
        assert_eq!(e.mem.u32(SAVE_LOAD_OBJECT), 0);
        assert!(calls(&log, STRING_CTOR).is_empty());
        assert_eq!(e.mem.u8(0x1c), 1);
    }

    #[test]
    fn the_constructor_sets_every_field() {
        let (mut e, this, _, log) = setup(true, 0);
        for offset in (0..0x38).step_by(4) {
            e.mem.set_u32(this + offset, 0xcccc_cccc);
        }
        let back = e.call(0x0084_fda0, &args![this]).u32();
        assert_eq!(back, this);
        for offset in [0u32, 4, 8, 0x14, 0x20, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), 0, "word at {offset:#x}");
        }
        for offset in [0xcu32, 0xd, 0xe, 0xf, 0x10, 0x11, 0x1c, 0x25, 0x26] {
            assert_eq!(e.mem.u8(this + offset), 0, "byte at {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        // Bytes the constructor does not touch keep their value.
        assert_eq!(e.mem.u16(this + 0x12), 0xcccc);
        assert_eq!(e.mem.u8(this + 0x27), 0xcc);
        assert_eq!(calls(&log, STRING_CTOR), [vec![this + 0x30]]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_0084fd30_deletes_and_clears_the_object() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.register(OBJECT_DELETE, |_, _| Ret::default());
        e.mem.set_u32(SAVE_LOAD_OBJECT, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0084_fd30, &args![]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|(a, _)| *a == OBJECT_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![0x5000, 1]);
        assert_eq!(e.mem.u32(SAVE_LOAD_OBJECT), 0);
        // Without an object only the clear.
        e.call_log = Some(vec![]);
        e.call(0x0084_fd30, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_0084ff30_builds_and_creates_the_save_directory() {
        let mut e = Engine::new();
        e.register(BASE_PATH, |_, _| 0x0120_2fa0u32.into_ret());
        e.register(SETTING_STRING, |_, _| 0x0300_0000u32.into_ret());
        e.register(FIRST_ARGUMENT, |_, a| a[0].into_ret());
        for callee in [STRING_COPY, STRING_APPEND, CREATE_DIRECTORY] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0084_ff30, &args![0x1111u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (BASE_PATH, vec![]),
                (STRING_COPY, vec![0x7000, 0x104, 0x0120_2fa0]),
                (SETTING_STRING, vec![LOCAL_SAVE_PATH_SETTING]),
                (FIRST_ARGUMENT, vec![0x0300_0000, 0]),
                (STRING_APPEND, vec![0x7000, 0x104, 0x0300_0000]),
                (CREATE_DIRECTORY, vec![0x7000, 0]),
            ]
        );
    }
}
