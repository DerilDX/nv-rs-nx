//! `(unplaced)/run_00406cc0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `vsprintf_s(buffer, size, format, va_list)` (LIBCMT, `cdecl`).
const VSPRINTF_S: u32 = 0x00ec_6bb5;
/// `strcpy_s(destination, size, source)` (LIBCMT, `cdecl`).
const STRCPY_S: u32 = 0x00ec_65a6;

// Translated from 00406d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sprintf_s(buffer, size, format, ...)` (`cdecl`): hands the variable
/// arguments, as a `va_list` that points at them, to `vsprintf_s`, and returns
/// its result (left in `EAX`). `variable_arguments` are the words the caller
/// pushed after `format`, in order; they are copied to memory because the
/// `va_list` is the address of the first one.
pub fn fn_00406d00(
    e: &mut Engine,
    buffer: Ptr,
    size: u32,
    format: Ptr,
    variable_arguments: &[u32],
) -> i32 {
    let list = e.mem.alloc(4 * variable_arguments.len().max(1) as u32);
    for (i, word) in variable_arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = e.call(VSPRINTF_S, &args![buffer, size, format, list]).i32();
    e.mem.free(list);
    result
}

/// The uniform form of [`fn_00406d00`]: it takes any number of argument
/// words, so it cannot go through `entry!`. The first three words are the
/// buffer, its size and the format; the rest are the variable arguments.
fn fn_00406d00_uniform(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00406d00(e, Ptr::new(a[0]), a[1], Ptr::new(a[2]), &a[3..]).into_ret()
}

// Translated from 00406d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strcpy_s(destination, size, source)` (`cdecl`): a thin wrapper, its
/// result (left in `EAX`) is `strcpy_s`'s.
pub fn fn_00406d30(e: &mut Engine, destination: Ptr, size: u32, source: Ptr) -> i32 {
    e.call(STRCPY_S, &args![destination, size, source]).i32()
}

/// `strcat_s(destination, size, source)` (LIBCMT, `cdecl`).
const STRCAT_S: u32 = 0x00ec_6bd2;

// Translated from 00406d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strcat_s(destination, size, source)` (`cdecl`): a thin wrapper, its
/// result (left in `EAX`) is `strcat_s`'s (`00ec6bd2`).
pub fn fn_00406d50(e: &mut Engine, destination: Ptr, size: u32, source: Ptr) -> i32 {
    e.call(STRCAT_S, &args![destination, size, source]).i32()
}

// Translated from 00406d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rounds a `float` to the nearest integer (`fld` / `fistp`), ties to even
/// as the x87 does with its default rounding mode. A value that does not fit
/// (or NaN) gives the "integer indefinite" `0x80000000`.
pub fn fn_00406d90(_e: &mut Engine, value: f32) -> i32 {
    // Round half to even by hand (`f64::round_ties_even` needs a newer Rust
    // than the crate supports).
    let value = value as f64;
    let floor = value.floor();
    let fraction = value - floor;
    let rounded = if fraction < 0.5 || (fraction == 0.5 && floor % 2.0 == 0.0) {
        floor
    } else {
        floor + 1.0
    };
    if rounded.is_nan() || !(-2147483648.0..2147483648.0).contains(&rounded) {
        i32::MIN
    } else {
        rounded as i32
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00406d00, fn_00406d00_uniform as AbiFn),
        entry!(0x00406d30, fn_00406d30(Ptr, u32, Ptr) -> i32),
        entry!(0x00406d50, fn_00406d50(Ptr, u32, Ptr) -> i32),
        entry!(0x00406d90, fn_00406d90(f32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprintf_passes_a_va_list_over_the_extra_words() {
        let mut e = Engine::new();
        // The double reads the words the va_list points at.
        e.register(VSPRINTF_S, |e, a| {
            let first = e.mem.u32(a[3]);
            let second = e.mem.u32(a[3] + 4);
            Ret {
                eax: first + second + a[1],
                ..Ret::default()
            }
        });
        let r = e.call(
            0x0040_6d00,
            &args![0x1000u32, 7u32, 0x2000u32, 10u32, 20u32],
        );
        assert_eq!(r.i32(), 37);
    }

    #[test]
    fn sprintf_without_variable_arguments_still_passes_a_list() {
        let mut e = Engine::new();
        e.register(VSPRINTF_S, |_, a| Ret {
            eax: a[3],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let list = e.call(0x0040_6d00, &args![1u32, 2u32, 3u32]).u32();
        assert_ne!(list, 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, VSPRINTF_S);
        assert_eq!(&log[1].1[..3], &[1, 2, 3]);
    }

    #[test]
    fn strcpy_s_forwards_its_three_words_and_result() {
        let mut e = Engine::new();
        e.register(STRCPY_S, |_, a| Ret {
            eax: a[0] + a[1] + a[2],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0040_6d30, &args![100u32, 20u32, 3u32]).i32(), 123);
    }

    #[test]
    fn strcat_s_forwards_its_three_words_and_result() {
        let mut e = Engine::new();
        e.register(STRCAT_S, |_, a| Ret {
            eax: a[0] * 100 + a[1] * 10 + a[2],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_6d50, &args![4u32, 5u32, 6u32]).i32(), 456);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == STRCAT_S).collect();
        assert_eq!(calls[0].1, vec![4, 5, 6]);
    }

    #[test]
    fn rounds_to_nearest_even_and_flags_overflow() {
        let mut e = Engine::new();
        let round = |e: &mut Engine, v: f32| e.call(0x0040_6d90, &args![v]).i32();
        assert_eq!(round(&mut e, 2.5), 2);
        assert_eq!(round(&mut e, 3.5), 4);
        assert_eq!(round(&mut e, -1.7), -2);
        assert_eq!(round(&mut e, -0.4), 0);
        assert_eq!(round(&mut e, 3.0e10), i32::MIN);
        assert_eq!(round(&mut e, f32::NAN), i32::MIN);
    }
}
