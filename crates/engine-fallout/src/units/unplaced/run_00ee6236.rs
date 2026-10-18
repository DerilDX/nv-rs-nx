//! `(unplaced)/run_00ee6236` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The slot `00ee6d44` jumps through.
const IMPORT_SLOT_00EE6D44: u32 = 0x00fd_f33c;

// Translated from 00ee6d44 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf33c]` (the import slot; Ghidra labels the import `closesocket`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d44(e: &mut Engine, s: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D44, &args![s]).i32()
}

/// The slot `00ee6d4a` jumps through.
const IMPORT_SLOT_00EE6D4A: u32 = 0x00fd_f340;

// Translated from 00ee6d4a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf340]` (the import slot; Ghidra labels the import `socket`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d4a(e: &mut Engine, af: u32, socket_type: u32, protocol: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6D4A, &args![af, socket_type, protocol])
        .u32()
}

/// The slot `00ee6d50` jumps through.
const IMPORT_SLOT_00EE6D50: u32 = 0x00fd_f344;

// Translated from 00ee6d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf344]` (the import slot; Ghidra labels the import `WSAGetLastError`, unconfirmed beyond that label). stdcall (the callee pops its words): the 0 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d50(e: &mut Engine) -> i32 {
    e.call(IMPORT_SLOT_00EE6D50, &args![]).i32()
}

/// The slot `00ee6d56` jumps through.
const IMPORT_SLOT_00EE6D56: u32 = 0x00fd_f348;

// Translated from 00ee6d56 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf348]` (the import slot; Ghidra labels the import `recv`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d56(e: &mut Engine, s: u32, buf: Ptr, len: u32, flags: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D56, &args![s, buf, len, flags])
        .i32()
}

/// The slot `00ee6d5c` jumps through.
const IMPORT_SLOT_00EE6D5C: u32 = 0x00fd_f34c;

// Translated from 00ee6d5c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf34c]` (the import slot; Ghidra labels the import `send`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d5c(e: &mut Engine, s: u32, buf: Ptr, len: u32, flags: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D5C, &args![s, buf, len, flags])
        .i32()
}

/// The slot `00ee6d62` jumps through.
const IMPORT_SLOT_00EE6D62: u32 = 0x00fd_f350;

// Translated from 00ee6d62 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf350]` (the import slot; Ghidra labels the import `connect`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d62(e: &mut Engine, s: u32, name: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D62, &args![s, name, namelen]).i32()
}

/// The slot `00ee6d68` jumps through.
const IMPORT_SLOT_00EE6D68: u32 = 0x00fd_f354;

// Translated from 00ee6d68 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf354]` (the import slot; Ghidra labels the import `gethostbyname`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d68(e: &mut Engine, name: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6D68, &args![name]).ptr()
}

/// The slot `00ee6d6e` jumps through.
const IMPORT_SLOT_00EE6D6E: u32 = 0x00fd_f358;

// Translated from 00ee6d6e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf358]` (the import slot; Ghidra labels the import `inet_addr`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d6e(e: &mut Engine, cp: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE6D6E, &args![cp]).u32()
}

/// The slot `00ee6d74` jumps through.
const IMPORT_SLOT_00EE6D74: u32 = 0x00fd_f35c;

// Translated from 00ee6d74 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf35c]` (the import slot; Ghidra labels the import `htons`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra; the result is the low 16 bits of `EAX`.
pub fn fn_00ee6d74(e: &mut Engine, hostshort: u16) -> u16 {
    e.call(IMPORT_SLOT_00EE6D74, &args![hostshort]).u16()
}

/// The slot `00ee6d7a` jumps through.
const IMPORT_SLOT_00EE6D7A: u32 = 0x00fd_f38c;

// Translated from 00ee6d7a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf38c]` (the import slot; Ghidra labels the import `WSAAsyncSelect`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d7a(e: &mut Engine, s: u32, hwnd: u32, msg: u32, event: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D7A, &args![s, hwnd, msg, event])
        .i32()
}

/// The slot `00ee6d86` jumps through.
const IMPORT_SLOT_00EE6D86: u32 = 0x00fd_f364;

// Translated from 00ee6d86 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf364]` (the import slot; Ghidra labels the import `WSAStartup`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d86(e: &mut Engine, version_requested: u16, wsa_data: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6D86, &args![version_requested, wsa_data])
        .i32()
}

/// The slot `00ee6d8c` jumps through.
const IMPORT_SLOT_00EE6D8C: u32 = 0x00fd_f368;

// Translated from 00ee6d8c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf368]` (the import slot; Ghidra labels the import `gethostname`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d8c(e: &mut Engine, name: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D8C, &args![name, namelen]).i32()
}

/// The slot `00ee6d92` jumps through.
const IMPORT_SLOT_00EE6D92: u32 = 0x00fd_f36c;

// Translated from 00ee6d92 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf36c]` (the import slot; Ghidra labels the import `listen`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d92(e: &mut Engine, s: u32, backlog: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D92, &args![s, backlog]).i32()
}

/// The slot `00ee6d98` jumps through.
const IMPORT_SLOT_00EE6D98: u32 = 0x00fd_f370;

// Translated from 00ee6d98 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf370]` (the import slot; Ghidra labels the import `bind`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d98(e: &mut Engine, s: u32, addr: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D98, &args![s, addr, namelen]).i32()
}

/// The slot `00ee6d9e` jumps through.
const IMPORT_SLOT_00EE6D9E: u32 = 0x00fd_f374;

// Translated from 00ee6d9e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf374]` (the import slot; Ghidra labels the import `setsockopt`, unconfirmed beyond that label). stdcall (the callee pops its words): the 5 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d9e(
    e: &mut Engine,
    s: u32,
    level: u32,
    optname: u32,
    optval: Ptr,
    optlen: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6D9E,
        &args![s, level, optname, optval, optlen],
    )
    .i32()
}

/// The slot `00ee6da4` jumps through.
const IMPORT_SLOT_00EE6DA4: u32 = 0x00fd_f378;

// Translated from 00ee6da4 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf378]` (the import slot; Ghidra labels the import `inet_ntoa`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra (the `in_addr` is one word).
pub fn fn_00ee6da4(e: &mut Engine, in_addr: u32) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DA4, &args![in_addr]).ptr()
}

/// The slot `00ee6daa` jumps through.
const IMPORT_SLOT_00EE6DAA: u32 = 0x00fd_f388;

// Translated from 00ee6daa (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf388]` (the import slot; Ghidra labels the import `ntohs`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra; the result is the low 16 bits of `EAX`.
pub fn fn_00ee6daa(e: &mut Engine, netshort: u16) -> u16 {
    e.call(IMPORT_SLOT_00EE6DAA, &args![netshort]).u16()
}

/// The slot `00ee6db0` jumps through.
const IMPORT_SLOT_00EE6DB0: u32 = 0x00fd_f384;

// Translated from 00ee6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf384]` (the import slot; Ghidra labels the import `accept`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6db0(e: &mut Engine, s: u32, addr: Ptr, addrlen: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE6DB0, &args![s, addr, addrlen]).u32()
}

/// The slot `00ee6db6` jumps through.
const IMPORT_SLOT_00EE6DB6: u32 = 0x00fd_f380;

// Translated from 00ee6db6 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf380]` (the import slot; Ghidra labels the import `__WSAFDIsSet`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra (its parameters are unnamed there).
pub fn fn_00ee6db6(e: &mut Engine, s: u32, set: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6DB6, &args![s, set]).i32()
}

/// The slot `00ee6dbc` jumps through.
const IMPORT_SLOT_00EE6DBC: u32 = 0x00fd_f37c;

// Translated from 00ee6dbc (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf37c]` (the import slot; Ghidra labels the import `select`, unconfirmed beyond that label). stdcall (the callee pops its words): the 5 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6dbc(
    e: &mut Engine,
    nfds: u32,
    readfds: Ptr,
    writefds: Ptr,
    exceptfds: Ptr,
    timeout: Ptr,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6DBC,
        &args![nfds, readfds, writefds, exceptfds, timeout],
    )
    .i32()
}

/// The slot `00ee6dc2` jumps through.
const IMPORT_SLOT_00EE6DC2: u32 = 0x00fd_f3d4;

// Translated from 00ee6dc2 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3d4]` (the import slot; Ghidra labels the import `D3DXSaveTextureToFileA`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Argument count from the only caller at 007a0797: four words (the first pushed is 0, then a texture word from a virtual call with no stack argument, 3, and a path buffer); the result is not used there.
pub fn fn_00ee6dc2(e: &mut Engine, word_0: Ptr, word_1: u32, word_2: u32, word_3: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6DC2, &args![word_0, word_1, word_2, word_3])
        .u32()
}

/// The slot `00ee6dc8` jumps through.
const IMPORT_SLOT_00EE6DC8: u32 = 0x00fd_f418;

// Translated from 00ee6dc8 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf418]` (the import slot; Ghidra labels the import `D3DXPlaneTransform`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Argument count from the callers (three address pushes at 004ed0b1; the result is not used there).
pub fn fn_00ee6dc8(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DC8, &args![word_0, word_1, word_2])
        .ptr()
}

/// The slot `00ee6dce` jumps through.
const IMPORT_SLOT_00EE6DCE: u32 = 0x00fd_f430;

// Translated from 00ee6dce (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf430]` (the import slot; Ghidra labels the import `D3DXPlaneNormalize`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the callers (two address pushes at 004ed097; the result is not used there).
pub fn fn_00ee6dce(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DCE, &args![word_0, word_1]).ptr()
}

/// The import slot of KERNEL32 `CreateFileA` (`00fdf088`, stdcall, seven words).
const CREATE_FILE_A: u32 = 0x00fd_f088;
/// `"CONOUT$"` in `.rdata`, the file name `___initconout` opens.
const CONOUT_NAME: u32 = 0x0110_08b4;
/// The CRT's console output handle, written by `___initconout`.
const CONSOLE_OUTPUT_HANDLE: u32 = 0x011c_2f74;

// Translated from 00ee626f (decompiled, FalloutNV.exe 1.4.0.525)
/// `___initconout` (Ghidra label from its function ID; `__putwch_nolock` calls it,
/// `00ee46d5`): opens the console output device and keeps the handle. It calls
/// `CreateFileA("CONOUT$", GENERIC_WRITE (0x40000000), FILE_SHARE_READ | FILE_SHARE_WRITE (3),
/// NULL, OPEN_EXISTING (3), 0, NULL)` and stores the result (`INVALID_HANDLE_VALUE` when
/// there is no console) in `011c2f74`. No arguments, nothing returned.
pub fn fn_00ee626f(e: &mut Engine) {
    let handle = e
        .call(
            CREATE_FILE_A,
            &args![CONOUT_NAME, 0x4000_0000u32, 3u32, 0u32, 3u32, 0u32, 0u32],
        )
        .u32();
    e.set_global(CONSOLE_OUTPUT_HANDLE, handle);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_626f, fn_00ee626f()),
        entry!(0x00ee_6d44, fn_00ee6d44(u32) -> i32),
        entry!(0x00ee_6d4a, fn_00ee6d4a(u32, u32, u32) -> u32),
        entry!(0x00ee_6d50, fn_00ee6d50() -> i32),
        entry!(0x00ee_6d56, fn_00ee6d56(u32, Ptr, u32, u32) -> i32),
        entry!(0x00ee_6d5c, fn_00ee6d5c(u32, Ptr, u32, u32) -> i32),
        entry!(0x00ee_6d62, fn_00ee6d62(u32, Ptr, u32) -> i32),
        entry!(0x00ee_6d68, fn_00ee6d68(Ptr) -> Ptr),
        entry!(0x00ee_6d6e, fn_00ee6d6e(Ptr) -> u32),
        entry!(0x00ee_6d74, fn_00ee6d74(u16) -> u16),
        entry!(0x00ee_6d7a, fn_00ee6d7a(u32, u32, u32, u32) -> i32),
        entry!(0x00ee_6d86, fn_00ee6d86(u16, Ptr) -> i32),
        entry!(0x00ee_6d8c, fn_00ee6d8c(Ptr, u32) -> i32),
        entry!(0x00ee_6d92, fn_00ee6d92(u32, u32) -> i32),
        entry!(0x00ee_6d98, fn_00ee6d98(u32, Ptr, u32) -> i32),
        entry!(0x00ee_6d9e, fn_00ee6d9e(u32, u32, u32, Ptr, u32) -> i32),
        entry!(0x00ee_6da4, fn_00ee6da4(u32) -> Ptr),
        entry!(0x00ee_6daa, fn_00ee6daa(u16) -> u16),
        entry!(0x00ee_6db0, fn_00ee6db0(u32, Ptr, Ptr) -> u32),
        entry!(0x00ee_6db6, fn_00ee6db6(u32, Ptr) -> i32),
        entry!(0x00ee_6dbc, fn_00ee6dbc(u32, Ptr, Ptr, Ptr, Ptr) -> i32),
        entry!(0x00ee_6dc2, fn_00ee6dc2(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ee_6dc8, fn_00ee6dc8(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6dce, fn_00ee6dce(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls `thunk` with `words`, with a double over the import `slot` that
    /// returns `0xdeadbeef`; checks that the double ran once with exactly
    /// those words and that the thunk returned `expected`.
    fn import(thunk: u32, slot: u32, words: &[u32], expected: u32) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == slot).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_00ee6d44() {
        import(0x00ee_6d44, IMPORT_SLOT_00EE6D44, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d4a() {
        import(
            0x00ee_6d4a,
            IMPORT_SLOT_00EE6D4A,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d50() {
        import(0x00ee_6d50, IMPORT_SLOT_00EE6D50, &[], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d56() {
        import(
            0x00ee_6d56,
            IMPORT_SLOT_00EE6D56,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d5c() {
        import(
            0x00ee_6d5c,
            IMPORT_SLOT_00EE6D5C,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d62() {
        import(
            0x00ee_6d62,
            IMPORT_SLOT_00EE6D62,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d68() {
        import(0x00ee_6d68, IMPORT_SLOT_00EE6D68, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d6e() {
        import(0x00ee_6d6e, IMPORT_SLOT_00EE6D6E, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d74() {
        import(0x00ee_6d74, IMPORT_SLOT_00EE6D74, &[0x1001], 0xbeef);
    }

    #[test]
    fn thunk_00ee6d7a() {
        import(
            0x00ee_6d7a,
            IMPORT_SLOT_00EE6D7A,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d86() {
        import(
            0x00ee_6d86,
            IMPORT_SLOT_00EE6D86,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d8c() {
        import(
            0x00ee_6d8c,
            IMPORT_SLOT_00EE6D8C,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d92() {
        import(
            0x00ee_6d92,
            IMPORT_SLOT_00EE6D92,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d98() {
        import(
            0x00ee_6d98,
            IMPORT_SLOT_00EE6D98,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d9e() {
        import(
            0x00ee_6d9e,
            IMPORT_SLOT_00EE6D9E,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6da4() {
        import(0x00ee_6da4, IMPORT_SLOT_00EE6DA4, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6daa() {
        import(0x00ee_6daa, IMPORT_SLOT_00EE6DAA, &[0x1001], 0xbeef);
    }

    #[test]
    fn thunk_00ee6db0() {
        import(
            0x00ee_6db0,
            IMPORT_SLOT_00EE6DB0,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6db6() {
        import(
            0x00ee_6db6,
            IMPORT_SLOT_00EE6DB6,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dbc() {
        import(
            0x00ee_6dbc,
            IMPORT_SLOT_00EE6DBC,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dc2() {
        import(
            0x00ee_6dc2,
            IMPORT_SLOT_00EE6DC2,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dc8() {
        import(
            0x00ee_6dc8,
            IMPORT_SLOT_00EE6DC8,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dce() {
        import(
            0x00ee_6dce,
            IMPORT_SLOT_00EE6DCE,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    #[test]
    fn initconout_opens_conout_and_keeps_the_handle() {
        let mut e = Engine::new();
        e.map(CONSOLE_OUTPUT_HANDLE, 4);
        e.register(CREATE_FILE_A, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ee_626f, &args![]);
        let log = e.call_log.take().unwrap();
        let opens: Vec<_> = log.iter().filter(|c| c.0 == CREATE_FILE_A).collect();
        assert_eq!(opens.len(), 1);
        assert_eq!(
            opens[0].1,
            vec![0x0110_08b4, 0x4000_0000, 3, 0, 3, 0, 0],
            "name, GENERIC_WRITE, share read|write, no security, OPEN_EXISTING, no flags, no template"
        );
        assert_eq!(e.global::<u32>(CONSOLE_OUTPUT_HANDLE), 0x1234);
    }

    #[test]
    fn initconout_keeps_an_invalid_handle_too() {
        let mut e = Engine::new();
        e.map(CONSOLE_OUTPUT_HANDLE, 4);
        e.set_global(CONSOLE_OUTPUT_HANDLE, 0xfffe_fffeu32);
        e.register(CREATE_FILE_A, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call(0x00ee_626f, &args![]);
        assert_eq!(e.global::<u32>(CONSOLE_OUTPUT_HANDLE), 0xffff_ffff);
    }
}
