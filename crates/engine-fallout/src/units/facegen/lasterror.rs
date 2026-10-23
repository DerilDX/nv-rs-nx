//! `FaceGen/lasterror.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::facegen::binaryfile::{
    string_chars, with_string_arg, StringArg, INVALID_PARAMETER, STRING_LENGTH, STRING_TIDY,
};

/// The last error code (`011f6c88`, 0: none yet) and its message buffer
/// (`011f6c8c`, 0x100 bytes).
const LAST_ERROR: u32 = 0x011f_6c88;
const LAST_ERROR_MESSAGE: u32 = 0x011f_6c8c;
/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and this
/// unit's source file name (`010a2de0`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const LAST_ERROR_SOURCE: u32 = 0x010a_2de0;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00ac6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `futSetLastError` (Xbox PDB; `cdecl`, the code and a string by value):
/// keeps the first error only: when no error is set yet, the code and the
/// string's characters (at most 0xFF, with `fr2Assert` for a longer one;
/// the library's check of a negative length can never fail) are copied to
/// the last-error globals, with a terminating zero. The string argument is
/// destroyed (`0044a730(1, 0)`) either way.
// Translation incomplete: 00ac6430 - the C++ exception-unwinding frame (FS:[0] chain, handler 00f20638, state variable) and the security-cookie bookkeeping are not translated.
pub fn fut_set_last_error(e: &mut Engine, code: u32, message: StringArg) {
    with_string_arg(e, message, |e, string| {
        if e.global::<u32>(LAST_ERROR) == 0 {
            let length = e.mem.u32(string + STRING_LENGTH);
            e.set_global(LAST_ERROR, code);
            if length >= 0x100 {
                e.call(FR2_ASSERT, &args![LAST_ERROR_SOURCE, 0x41u32]);
            }
            let length = if length < 0x100 { length } else { 0xff };
            // `CMP [length],0; JNC`: an unsigned length is never below 0.
            #[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
            let negative = e.mem.u32(string + STRING_LENGTH) < 0;
            if negative {
                e.call(INVALID_PARAMETER, &[]);
            }
            let source = string_chars(e, string);
            e.call(MEMCPY, &args![LAST_ERROR_MESSAGE, source, length]);
            e.mem.set_u8(LAST_ERROR_MESSAGE + length, 0);
        }
        e.call(STRING_TIDY, &args![string, 1u32, 0u32]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ac_6430, fut_set_last_error(u32, StringArg))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRING_TIDY, |_, _| Ret::default());
        e.register(FR2_ASSERT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    /// A short string (inline characters) by value.
    fn short(text: &[u8]) -> StringArg {
        let mut words = [0u32; 7];
        let mut bytes = [0u8; 16];
        bytes[..text.len()].copy_from_slice(text);
        for i in 0..4 {
            words[1 + i] = u32::from_le_bytes([
                bytes[4 * i],
                bytes[4 * i + 1],
                bytes[4 * i + 2],
                bytes[4 * i + 3],
            ]);
        }
        words[5] = text.len() as u32;
        words[6] = 0xf;
        StringArg(words)
    }

    #[test]
    fn keeps_the_first_error_and_destroys_the_argument() {
        let mut e = rig();
        let mut words = vec![7u32];
        words.extend_from_slice(&short(b"bad file").0);
        e.call(0x00ac_6430, &words);
        assert_eq!(e.global::<u32>(LAST_ERROR), 7);
        assert_eq!(e.mem.cstr(LAST_ERROR_MESSAGE), b"bad file".to_vec());
        let mut words = vec![9u32];
        words.extend_from_slice(&short(b"other").0);
        e.call(0x00ac_6430, &words);
        assert_eq!(e.global::<u32>(LAST_ERROR), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == STRING_TIDY).count(), 2);
        assert_eq!(log.iter().filter(|c| c.0 == MEMCPY).count(), 1);
    }

    #[test]
    fn long_messages_are_cut_to_0xff_bytes_with_an_assert() {
        let mut e = rig();
        let text = e.mem.alloc(0x200);
        e.mem.write(text, &[b'x'; 0x180]);
        let words = vec![3u32, 0, text, 0, 0, 0, 0x180, 0x1ff];
        e.call(0x00ac_6430, &words);
        assert_eq!(e.mem.cstr(LAST_ERROR_MESSAGE).len(), 0xff);
        let log = e.call_log.take().unwrap();
        let asserts: Vec<_> = log.iter().filter(|c| c.0 == FR2_ASSERT).collect();
        assert_eq!(asserts[0].1, vec![LAST_ERROR_SOURCE, 0x41]);
    }
}
