//! The C/C++ runtime library (MSVC LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), replaced by Rust at the game's own addresses so
//! translations call it like any other function. Lead-owned.
//!
//! These are `library` code in the ledger; each is marked
//! `Platform: replaces <addr>` so the ledger counts it as stood in for.
//! Semantics are the C standard's (C locale), which is what the game relies
//! on; where MSVC's behaviour is more specific it is noted.
//!
//! Floating point: x87 transcendentals (`FSIN`, `FCOS`) and Rust's `sin`,
//! `cos` can differ in the last bit; values that must be bit-exact need an
//! `nv-call` check.

#[allow(unused_imports)]
use crate::prelude::*;

// ---- Memory ---------------------------------------------------------------

// Platform: replaces 00ec44d0 (_memcpy, LIBCMT)
pub fn memcpy(e: &mut Engine, dst: Ptr, src: Ptr, n: u32) -> Ptr {
    let b = e.mem.bytes(src.addr(), n);
    e.mem.write(dst.addr(), &b);
    dst
}

// Platform: replaces 00ec7230 (memmove, LIBCMT)
pub fn memmove(e: &mut Engine, dst: Ptr, src: Ptr, n: u32) -> Ptr {
    memcpy(e, dst, src, n)
}

// Platform: replaces 00ec61c0 (_memset, LIBCMT)
pub fn memset(e: &mut Engine, dst: Ptr, c: i32, n: u32) -> Ptr {
    e.mem.write(dst.addr(), &vec![c as u8; n as usize]);
    dst
}

// Platform: replaces 00ec4835 (_memcmp, LIBCMT)
pub fn memcmp(e: &mut Engine, a: Ptr, b: Ptr, n: u32) -> i32 {
    let (x, y) = (e.mem.bytes(a.addr(), n), e.mem.bytes(b.addr(), n));
    sign(x.cmp(&y))
}

fn sign(o: std::cmp::Ordering) -> i32 {
    o as i32
}

// Platform: replaces 00ec3bad (_byteswap_ulong, LIBCMT)
/// Reverses the byte order of a 32-bit value (cdecl, one word).
pub fn byteswap_ulong(_e: &mut Engine, value: u32) -> u32 {
    value.swap_bytes()
}

// Platform: replaces 00ec3bf0 (_byteswap_uint64, LIBCMT)
/// Reverses the byte order of a 64-bit value (cdecl, two words, low word
/// first; the result in `EDX:EAX`).
pub fn byteswap_uint64(_e: &mut Engine, value: u64) -> u64 {
    value.swap_bytes()
}

// Translated from 00401000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `operator new`: allocates through the `MemoryManager` singleton.
pub fn operator_new(e: &mut Engine, size: u32) -> Ptr {
    let mm: Ptr = crate::units::platform::get_memory_manager(e);
    e.call_as(0x00aa_3e40, &args![mm, size])
}

// Translated from 00401460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `memcpy` wrapper: forwards to the runtime's `_memcpy`.
pub fn memcpy_wrapper(e: &mut Engine, dst: Ptr, src: Ptr, n: u32) -> Ptr {
    e.call_as(0x00ec_44d0, &args![dst, src, n])
}

// Translated from 00403d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `memset` wrapper: forwards to the runtime's `_memset`.
pub fn memset_wrapper(e: &mut Engine, dst: Ptr, c: i32, n: u32) -> Ptr {
    e.call_as(0x00ec_61c0, &args![dst, c, n])
}

// Platform: replaces 00ed0cdf (malloc, LIBCMT)
pub fn malloc(e: &mut Engine, n: u32) -> Ptr {
    Ptr::new(e.mem.alloc(n))
}

// Platform: replaces 00ecd291 (free, LIBCMT)
pub fn free(e: &mut Engine, p: Ptr) {
    e.mem.free(p.addr())
}

// Platform: replaces 00ed0d24 (__calloc_crt, LIBCMT)
pub fn calloc_crt(e: &mut Engine, count: u32, size: u32) -> Ptr {
    Ptr::new(e.mem.alloc(count.saturating_mul(size)))
}

// ---- Strings --------------------------------------------------------------

// Platform: replaces 00ec6130 (_strlen, LIBCMT)
pub fn strlen(e: &mut Engine, s: Ptr) -> u32 {
    e.mem.cstr(s.addr()).len() as u32
}

// Platform: replaces 00ec6370 (strcpy, LIBCMT)
pub fn strcpy(e: &mut Engine, dst: Ptr, src: Ptr) -> Ptr {
    let s = e.mem.cstr(src.addr());
    e.mem.set_cstr(dst.addr(), &s);
    dst
}

/// The `_s` functions' failure: MSVC calls the invalid-parameter handler,
/// which in a release build ends the program.
fn invalid_parameter(what: &str) -> ! {
    panic!("CRT invalid parameter: {what}")
}

// Platform: replaces 00ec65a6 (_strcpy_s, LIBCMT)
pub fn strcpy_s(e: &mut Engine, dst: Ptr, size: u32, src: Ptr) -> i32 {
    let s = e.mem.cstr(src.addr());
    if dst.is_null() || s.len() as u32 >= size {
        invalid_parameter("strcpy_s");
    }
    e.mem.set_cstr(dst.addr(), &s);
    0
}

// Platform: replaces 00ec6bd2 (_strcat_s, LIBCMT)
pub fn strcat_s(e: &mut Engine, dst: Ptr, size: u32, src: Ptr) -> i32 {
    let mut d = e.mem.cstr(dst.addr());
    d.extend(e.mem.cstr(src.addr()));
    if d.len() as u32 >= size {
        invalid_parameter("strcat_s");
    }
    e.mem.set_cstr(dst.addr(), &d);
    0
}

/// `_TRUNCATE` for the `count` of `strncpy_s`.
pub const TRUNCATE: u32 = 0xffff_ffff;

// Platform: replaces 00ec8d5f (_strncpy_s, LIBCMT)
pub fn strncpy_s(e: &mut Engine, dst: Ptr, size: u32, src: Ptr, count: u32) -> i32 {
    let mut s = e.mem.cstr(src.addr());
    if count != TRUNCATE {
        s.truncate(count as usize);
    }
    if s.len() as u32 >= size {
        if count == TRUNCATE && size > 0 {
            s.truncate(size as usize - 1);
            e.mem.set_cstr(dst.addr(), &s);
            return 80; // STRUNCATE
        }
        invalid_parameter("strncpy_s");
    }
    e.mem.set_cstr(dst.addr(), &s);
    0
}

fn cmp_bytes(a: &[u8], b: &[u8]) -> i32 {
    sign(a.cmp(b))
}

// Platform: replaces 00ec8a19 (strncmp, LIBCMT)
pub fn strncmp(e: &mut Engine, a: Ptr, b: Ptr, n: u32) -> i32 {
    let mut x = e.mem.cstr(a.addr());
    let mut y = e.mem.cstr(b.addr());
    x.truncate(n as usize);
    y.truncate(n as usize);
    cmp_bytes(&x, &y)
}

// Platform: replaces 00ec68e4 (__stricmp, LIBCMT; C locale)
pub fn stricmp(e: &mut Engine, a: Ptr, b: Ptr) -> i32 {
    let x: Vec<u8> = e
        .mem
        .cstr(a.addr())
        .iter()
        .map(u8::to_ascii_lowercase)
        .collect();
    let y: Vec<u8> = e
        .mem
        .cstr(b.addr())
        .iter()
        .map(u8::to_ascii_lowercase)
        .collect();
    cmp_bytes(&x, &y)
}

// Platform: replaces 00ec7ec0 (__strnicmp, LIBCMT; C locale)
pub fn strnicmp(e: &mut Engine, a: Ptr, b: Ptr, n: u32) -> i32 {
    let low = |s: Vec<u8>| -> Vec<u8> {
        s.iter()
            .take(n as usize)
            .map(u8::to_ascii_lowercase)
            .collect()
    };
    let x = low(e.mem.cstr(a.addr()));
    let y = low(e.mem.cstr(b.addr()));
    cmp_bytes(&x, &y)
}

// Platform: replaces 00ec7690 (strchr, LIBCMT)
pub fn strchr(e: &mut Engine, s: Ptr, c: i32) -> Ptr {
    let bytes = e.mem.cstr(s.addr());
    let c = c as u8;
    if c == 0 {
        return Ptr::new(s.addr() + bytes.len() as u32);
    }
    match bytes.iter().position(|&b| b == c) {
        Some(i) => Ptr::new(s.addr() + i as u32),
        None => Ptr::NULL,
    }
}

// Platform: replaces 00ec67aa (tolower, LIBCMT; C locale)
pub fn tolower(_e: &mut Engine, c: i32) -> i32 {
    if (b'A' as i32..=b'Z' as i32).contains(&c) {
        c + 32
    } else {
        c
    }
}

// Platform: replaces 00eca7f4 (toupper, LIBCMT; C locale)
pub fn toupper(_e: &mut Engine, c: i32) -> i32 {
    if (b'a' as i32..=b'z' as i32).contains(&c) {
        c - 32
    } else {
        c
    }
}

/// C `atol`/`strtol` base 10: optional space, sign, digits; wraps like
/// MSVC's `atol` on overflow is not relied on by the game.
fn parse_long(s: &[u8]) -> i32 {
    let mut i = 0;
    while i < s.len() && (s[i] == b' ' || (9..=13).contains(&s[i])) {
        i += 1;
    }
    let mut neg = false;
    if i < s.len() && (s[i] == b'-' || s[i] == b'+') {
        neg = s[i] == b'-';
        i += 1;
    }
    let mut v: i64 = 0;
    while i < s.len() && s[i].is_ascii_digit() {
        v = (v * 10 + (s[i] - b'0') as i64).min(i64::from(i32::MAX) + 1);
        i += 1;
    }
    let v = if neg { -v } else { v };
    v.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}

// Platform: replaces 00eca6d3 (atol, LIBCMT)
pub fn atol(e: &mut Engine, s: Ptr) -> i32 {
    parse_long(&e.mem.cstr(s.addr()))
}

// Platform: replaces 00eca6bd (_atol's body, strtol(s, NULL, 10), LIBCMT; 00eca6d3 jumps here)
pub fn atol_body(e: &mut Engine, s: Ptr) -> i32 {
    parse_long(&e.mem.cstr(s.addr()))
}

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the
/// locale or null): fills the 0x10-byte object (locale info, multibyte
/// info, thread data, "updated" byte at +0x0C).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// `__isctype_l` (`00ed41e7`, cdecl: character, mask, locale object).
const ISCTYPE_L: u32 = 0x00ed_41e7;
/// `__locale_changed`: 0 while the C locale is the global one.
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// The initial locale object `strtol` passes while the locale is unchanged.
const INITIAL_LOCALE: u32 = 0x011c_2400;

/// The inlined `_LocaleUpdate` destructor: when the constructor set the
/// "updated" byte (+0x0C), clears bit 1 of the thread data's `_ownlocale`
/// (+0x70).
fn locale_update_release(e: &mut Engine, update: u32) {
    if e.mem.u8(update + 0xc) != 0 {
        let thread = e.mem.u32(update + 8);
        let own = e.mem.u32(thread + 0x70);
        e.mem.set_u32(thread + 0x70, own & 0xffff_fffd);
    }
}

// Translated from 00ec8ad9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strtoxl` (LIBCMT/strtol.c, engine map; cdecl: locale, text, end, base,
/// flags; flag 1 is unsigned, 2 negative, 4 overflow, 8 a digit read).
/// Takes the locale (`_LocaleUpdate`), stores `text` in `*end`; a null text
/// or a base outside 0 and 2..36 sets `errno` to `EINVAL` (0x16), reports
/// an invalid parameter and returns 0. Otherwise skips characters of type
/// 8 (space, through `__isctype_l` when the locale's `mb_cur_max` (+0xAC)
/// is above 1, else the ctype table at +0xC8), takes a sign, picks base 8,
/// 10 or 16 for base 0 and skips a `0x`/`0X` prefix for base 16, then
/// accumulates digits (type 4) and letters (types 0x103) below the base.
/// With no digit `*end` is `text` and the result 0; on overflow (unsigned,
/// or past `0x7FFFFFFF`/`0x80000000` signed) `errno` is `ERANGE` (0x22)
/// and the result `0xFFFFFFFF`, `0x7FFFFFFF` or `0x80000000`. `*end` is the
/// first unread character; a negative result is negated.
pub fn strtoxl(e: &mut Engine, locale: u32, text: u32, end: u32, base: i32, flags: u32) -> u32 {
    // The `_LocaleUpdate` object at EBP-0x14 and the value at EBP-0x4.
    e.with_stack(0x14, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        if end != 0 {
            e.mem.set_u32(end, text);
        }
        let invalid = text == 0 || (base != 0 && !(2..=0x24).contains(&base));
        if invalid {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            locale_update_release(e, update);
            return 0;
        }
        let mut flags = flags;
        let mut c = e.mem.u8(text);
        let mut value: u32 = 0;
        let mut p = text + 1;
        loop {
            let info = e.mem.u32(update);
            let space = if (e.mem.u32(info + 0xac) as i32) > 1 {
                e.call(ISCTYPE_L, &[u32::from(c), 8, update]).u32()
            } else {
                let table = e.mem.u32(info + 0xc8);
                u32::from(e.mem.u16(table + u32::from(c) * 2)) & 8
            };
            if space == 0 {
                break;
            }
            c = e.mem.u8(p);
            p += 1;
        }
        if c == b'-' {
            flags |= 2;
            c = e.mem.u8(p);
            p += 1;
        } else if c == b'+' {
            c = e.mem.u8(p);
            p += 1;
        }
        // Checked again after the sign (never true after the check above).
        if base < 0 || base == 1 || base > 0x24 {
            if end != 0 {
                e.mem.set_u32(end, text);
            }
            locale_update_release(e, update);
            return 0;
        }
        let mut base = base;
        let mut check_prefix = false;
        if base == 0 {
            if c != b'0' {
                base = 10;
            } else {
                let next = e.mem.u8(p);
                if next == b'x' || next == b'X' {
                    base = 16;
                    check_prefix = true;
                } else {
                    base = 8;
                }
            }
        } else if base == 16 && c == b'0' {
            check_prefix = true;
        }
        if check_prefix {
            let next = e.mem.u8(p);
            if next == b'x' || next == b'X' {
                p += 1;
                c = e.mem.u8(p);
                p += 1;
            }
        }
        let base = base as u32;
        let info = e.mem.u32(update);
        let table = e.mem.u32(info + 0xc8);
        let limit = u32::MAX / base;
        let last_digit = u32::MAX % base;
        loop {
            let kind = u32::from(e.mem.u16(table + u32::from(c) * 2));
            let digit = if kind & 4 != 0 {
                (i32::from(c as i8) - 0x30) as u32
            } else if kind & 0x103 != 0 {
                let mut letter = i32::from(c as i8);
                if c.wrapping_sub(0x61) <= 0x19 {
                    letter -= 0x20;
                }
                (letter - 0x37) as u32
            } else {
                break;
            };
            if digit >= base {
                break;
            }
            flags |= 8;
            if value < limit || (value == limit && digit <= last_digit) {
                value = value.wrapping_mul(base).wrapping_add(digit);
            } else {
                flags |= 4;
                if end == 0 {
                    break;
                }
            }
            c = e.mem.u8(p);
            p += 1;
        }
        p -= 1;
        if flags & 8 == 0 {
            if end != 0 {
                p = text;
            }
            value = 0;
        } else {
            let overflow = if flags & 4 != 0 {
                true
            } else if flags & 1 != 0 {
                false
            } else if flags & 2 != 0 {
                value > 0x8000_0000
            } else {
                value > 0x7fff_ffff
            };
            if overflow {
                let slot = errno(e);
                e.mem.set_u32(slot.addr(), 0x22);
                value = if flags & 1 != 0 {
                    u32::MAX
                } else {
                    0x7fff_ffff + u32::from(flags & 2 != 0)
                };
            }
        }
        if end != 0 {
            e.mem.set_u32(end, p);
        }
        if flags & 2 != 0 {
            value = value.wrapping_neg();
        }
        locale_update_release(e, update);
        value
    })
}

// Translated from 00ec8d08 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strtol` (LIBCMT/strtol.c; cdecl): `strtoxl(locale, text, end, base, 0)`
/// with the initial locale object `011c2400` while `__locale_changed`
/// (`0126fdf4`) is 0, else a null locale (the thread's).
pub fn strtol(e: &mut Engine, text: u32, end: u32, base: i32) -> i32 {
    let locale = if e.mem.u32(LOCALE_CHANGED) == 0 {
        INITIAL_LOCALE
    } else {
        0
    };
    strtoxl(e, locale, text, end, base, 0) as i32
}

// ---- Per-thread data and locale (LIBCMT tidtable.c, setlocal.c, mbctype.c) ----

/// `GetLastError` (import slot).
const IMPORT_GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `SetLastError` (import slot).
const IMPORT_SET_LAST_ERROR: u32 = 0x00fd_f1dc;
/// `GetCurrentThreadId` (import slot).
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `___set_flsgetvalue` (`00ed100d`, cdecl, no arguments): the
/// `FlsGetValue` function to call.
const SET_FLSGETVALUE: u32 = 0x00ed_100d;
/// `__flsindex`: the FLS slot of the thread data block (-1 until the CRT's
/// start-up allocates it).
const FLS_INDEX: u32 = 0x011c_1cbc;
/// The encoded `FlsSetValue` pointer.
const ENCODED_FLS_SET_VALUE: u32 = 0x0126_fe08;
/// `_initptd` (`00ed107e`, cdecl: block, locale info).
const INIT_PTD: u32 = 0x00ed_107e;
/// `_amsg_exit` (`00eccc55`, cdecl: run-time error number).
const AMSG_EXIT: u32 = 0x00ec_cc55;
/// `_lock` (`00eddf39`, cdecl: lock number).
const CRT_LOCK: u32 = 0x00ed_df39;
/// `_unlock` (`00edde5f`, cdecl: lock number).
const CRT_UNLOCK: u32 = 0x00ed_de5f;
/// `__updatetlocinfoEx_nolock` (`00ecf704`): register arguments, EAX the
/// slot to update and EDI the new locale info, passed as those two words.
const UPDATE_TLOCINFO_EX_NOLOCK: u32 = 0x00ec_f704;
/// `__globallocalestatus`: the `_ownlocale` bits that keep a thread's own
/// locale.
const GLOBAL_LOCALE_STATUS: u32 = 0x011c_2310;
/// `__ptlocinfo`: the global locale info.
const GLOBAL_LOCALE_INFO: u32 = 0x011c_23f8;
/// `__ptmbcinfo`: the global multibyte info.
const GLOBAL_MBC_INFO: u32 = 0x011c_2218;
/// The CRT's initial (static) multibyte info, never freed.
const INITIAL_MBC_INFO: u32 = 0x011c_1df0;
/// `__crtGetStringTypeA_stat` (`00ee11e9`): the locale object in ECX and
/// seven stack words, passed as eight words (the object first).
const GET_STRING_TYPE_A_STAT: u32 = 0x00ee_11e9;

// Translated from 00ed1165 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__getptd_noexit` (LIBCMT/tidtable.c, Ghidra's function ID; cdecl): the
/// calling thread's CRT data block. Keeps the last error, reads the block
/// from the FLS slot (`FlsGetValue` as `___set_flsgetvalue` returns it); a
/// thread without one gets a zeroed 0x214-byte block stored in the slot
/// (the decoded `FlsSetValue`) and set up by `_initptd(block, 0)` with its
/// thread id and handle -1; a failed store frees it and returns 0. Restores
/// the last error.
pub fn getptd_noexit(e: &mut Engine) -> u32 {
    let last_error = e.call(IMPORT_GET_LAST_ERROR, &[]).u32();
    let index = e.mem.u32(FLS_INDEX);
    let get_value = e.call(SET_FLSGETVALUE, &[]).u32();
    let mut block = e.call(get_value, &[index]).u32();
    if block == 0 {
        block = calloc_crt(e, 1, 0x214).addr();
        if block != 0 {
            let encoded = e.mem.u32(ENCODED_FLS_SET_VALUE);
            let index = e.mem.u32(FLS_INDEX);
            let set_value = e.call(DECODE_POINTER, &[encoded]).u32();
            if e.call(set_value, &[index, block]).u32() != 0 {
                e.call(INIT_PTD, &[block, 0]);
                let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32();
                e.mem.set_u32(block + 4, u32::MAX);
                e.mem.set_u32(block, thread);
            } else {
                free(e, Ptr::new(block));
                block = 0;
            }
        }
    }
    e.call(IMPORT_SET_LAST_ERROR, &[last_error]);
    block
}

// Translated from 00ed11de (decompiled, FalloutNV.exe 1.4.0.525)
/// `_getptd` (LIBCMT/tidtable.c; cdecl): [`getptd_noexit`], ending the
/// program through `_amsg_exit(0x10)` (`_RT_THREAD`) when there is no
/// block.
pub fn getptd(e: &mut Engine) -> u32 {
    let block = getptd_noexit(e);
    if block == 0 {
        e.call(AMSG_EXIT, &[0x10]);
    }
    block
}

// Translated from 00ecf742 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetlocinfo` (LIBCMT/setlocal.c, Ghidra's function ID; cdecl): the
/// thread's locale info: the thread's own (+0x6C) when its `_ownlocale`
/// (+0x70) has a bit of `__globallocalestatus` and it has one; otherwise,
/// under lock 0xC, `__updatetlocinfoEx_nolock(&ptd->ptlocinfo,
/// __ptlocinfo)` (the `__finally` block at `00ecf7ac`, which unlocks and
/// takes the result, is this function's own code). A null result ends the
/// program (`_amsg_exit(0x20)`).
// Translation incomplete: 00ecf742 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn update_tlocinfo(e: &mut Engine) -> u32 {
    let thread = getptd(e);
    let status = e.mem.u32(GLOBAL_LOCALE_STATUS);
    let info = if e.mem.u32(thread + 0x70) & status != 0 && e.mem.u32(thread + 0x6c) != 0 {
        let thread = getptd(e);
        e.mem.u32(thread + 0x6c)
    } else {
        e.call(CRT_LOCK, &[0xc]);
        let global = e.mem.u32(GLOBAL_LOCALE_INFO);
        let result = e
            .call(UPDATE_TLOCINFO_EX_NOLOCK, &[thread + 0x6c, global])
            .u32();
        e.call(CRT_UNLOCK, &[0xc]);
        result
    };
    if info == 0 {
        e.call(AMSG_EXIT, &[0x20]);
    }
    info
}

// Translated from 00ed3cdf (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetmbcinfo` (LIBCMT/mbctype.c, Ghidra's function ID; cdecl): the
/// thread's multibyte info (+0x68). When the thread does not keep its own
/// locale (the `_ownlocale` test, then its locale info +0x6C), under lock
/// 0xD: a block other than `__ptmbcinfo` is released (`InterlockedDecrement`
/// on its count; freed at zero unless it is the static initial one) and
/// replaced by `__ptmbcinfo`, whose count goes up (the `__finally` block at
/// `00ed3d7a`, the unlock, is this function's own code). A null result ends
/// the program (`_amsg_exit(0x20)`).
// Translation incomplete: 00ed3cdf - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn update_tmbcinfo(e: &mut Engine) -> u32 {
    let thread = getptd(e);
    let status = e.mem.u32(GLOBAL_LOCALE_STATUS);
    let info = if e.mem.u32(thread + 0x70) & status != 0 && e.mem.u32(thread + 0x6c) != 0 {
        e.mem.u32(thread + 0x68)
    } else {
        e.call(CRT_LOCK, &[0xd]);
        let mut info = e.mem.u32(thread + 0x68);
        if info != e.mem.u32(GLOBAL_MBC_INFO) {
            if info != 0
                && e.call(IMPORT_INTERLOCKED_DECREMENT, &[info]).u32() == 0
                && info != INITIAL_MBC_INFO
            {
                free(e, Ptr::new(info));
            }
            let global = e.mem.u32(GLOBAL_MBC_INFO);
            e.mem.set_u32(thread + 0x68, global);
            info = e.mem.u32(GLOBAL_MBC_INFO);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &[info]);
        }
        e.call(CRT_UNLOCK, &[0xd]);
        info
    };
    if info == 0 {
        e.call(AMSG_EXIT, &[0x20]);
    }
    info
}

// Translated from 00ec660e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_LocaleUpdate::_LocaleUpdate` (Ghidra's function ID; thiscall, one word):
/// fills the 0x10-byte object: with a locale, its locale and multibyte info;
/// without one, the thread's (`_getptd`, +0x6C and +0x68) brought up to date
/// (`__updatetlocinfo` / `__updatetmbcinfo`) unless they are the globals or
/// the thread keeps its own locale, the thread block at +0x08 and, when bit
/// 1 of its `_ownlocale` (+0x70) was clear, that bit set and the "updated"
/// byte (+0x0C) set. Returns `this`.
pub fn locale_update(e: &mut Engine, this: u32, locale: u32) -> u32 {
    e.mem.set_u8(this + 0xc, 0);
    if locale != 0 {
        let info = e.mem.u32(locale);
        e.mem.set_u32(this, info);
        let mbc = e.mem.u32(locale + 4);
        e.mem.set_u32(this + 4, mbc);
        return this;
    }
    let thread = getptd(e);
    e.mem.set_u32(this + 8, thread);
    let info = e.mem.u32(thread + 0x6c);
    e.mem.set_u32(this, info);
    let mbc = e.mem.u32(thread + 0x68);
    e.mem.set_u32(this + 4, mbc);
    if e.mem.u32(this) != e.mem.u32(GLOBAL_LOCALE_INFO)
        && e.mem.u32(thread + 0x70) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0
    {
        let info = update_tlocinfo(e);
        e.mem.set_u32(this, info);
    }
    if e.mem.u32(this + 4) != e.mem.u32(GLOBAL_MBC_INFO) {
        let thread = e.mem.u32(this + 8);
        if e.mem.u32(thread + 0x70) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0 {
            let mbc = update_tmbcinfo(e);
            e.mem.set_u32(this + 4, mbc);
        }
    }
    let thread = e.mem.u32(this + 8);
    let own = e.mem.u32(thread + 0x70);
    if own & 2 == 0 {
        e.mem.set_u32(thread + 0x70, own | 2);
        e.mem.set_u8(this + 0xc, 1);
    }
    this
}

// Translated from 00ed419c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isleadbyte_l` (Ghidra's function ID; cdecl: character, locale): bit
/// 0x8000 (`_LEADBYTE`) of the low byte's entry in the locale's ctype table
/// (+0xC8).
pub fn isleadbyte_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        locale_update(e, update, locale);
        let info = e.mem.u32(update);
        let table = e.mem.u32(info + 0xc8);
        let kind = u32::from(e.mem.u16(table + (c & 0xff) * 2)) & 0x8000;
        locale_update_release(e, update);
        kind
    })
}

// Translated from 00ee13a3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtGetStringTypeA` (Ghidra's function ID; cdecl, eight words): takes
/// the locale and passes the seven other words to `__crtGetStringTypeA_stat`
/// (`00ee11e9`) with the locale object, returning its result.
#[allow(clippy::too_many_arguments)]
pub fn crt_get_string_type_a(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
    error: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        locale_update(e, update, locale);
        let result = e
            .call(
                GET_STRING_TYPE_A_STAT,
                &[
                    update, info_type, text, length, types, code_page, lcid, error,
                ],
            )
            .u32();
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ed41e7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isctype_l` (Ghidra's function ID; cdecl: character, mask, locale): for
/// -1..255 the character's entry in the locale's ctype table (+0xC8);
/// otherwise the type `__crtGetStringTypeA(.., CT_CTYPE1, ..)` gives for
/// its one or two bytes (two when the high byte is a lead byte), 0 when
/// that fails. Returns the type masked by `mask`.
pub fn isctype_l(e: &mut Engine, c: i32, mask: u32, locale: u32) -> u32 {
    // The frame from EBP-0x18: the `_LocaleUpdate` object, the 3-byte
    // text at EBP-0x8 and the 16-bit type at EBP-0x4.
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let text = update + 0x10;
        let kind_slot = update + 0x14;
        locale_update(e, update, locale);
        let kind = if (c as u32).wrapping_add(1) <= 0x100 {
            let info = e.mem.u32(update);
            let table = e.mem.u32(info + 0xc8);
            u32::from(e.mem.u16(table.wrapping_add((c as u32).wrapping_mul(2))))
        } else {
            let high = c >> 8;
            let length = if isleadbyte_l(e, (high as u32) & 0xff, update) != 0 {
                e.mem.set_u8(text, high as u8);
                e.mem.set_u8(text + 1, c as u8);
                e.mem.set_u8(text + 2, 0);
                2
            } else {
                e.mem.set_u8(text, c as u8);
                e.mem.set_u8(text + 1, 0);
                1
            };
            let info = e.mem.u32(update);
            let code_page = e.mem.u32(info + 4);
            let lcid = e.mem.u32(info + 0x14);
            let ok =
                crt_get_string_type_a(e, update, 1, text, length, kind_slot, code_page, lcid, 1);
            if ok == 0 {
                locale_update_release(e, update);
                return 0;
            }
            u32::from(e.mem.u16(kind_slot))
        };
        let result = kind & mask;
        locale_update_release(e, update);
        result
    })
}

// ---- Thread start-up: __mtinit and its chain (LIBCMT tidtable.c, mlock.c) ----

/// `GetModuleHandleW` (import slot).
const IMPORT_GET_MODULE_HANDLE_W: u32 = 0x00fd_f198;
/// `GetProcAddress` (import slot).
const IMPORT_GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// `TlsAlloc` (import slot).
const IMPORT_TLS_ALLOC: u32 = 0x00fd_f048;
/// `TlsGetValue` (import slot).
const IMPORT_TLS_GET_VALUE: u32 = 0x00fd_f044;
/// `TlsSetValue` (import slot).
const IMPORT_TLS_SET_VALUE: u32 = 0x00fd_f04c;
/// `TlsFree` (import slot).
const IMPORT_TLS_FREE: u32 = 0x00fd_f050;
/// `Sleep` (import slot).
const IMPORT_SLEEP: u32 = 0x00fd_f104;
/// `EnterCriticalSection` (import slot).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` (import slot).
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InitializeCriticalSectionAndSpinCount` (import slot).
const IMPORT_INIT_CRITICAL_SECTION_SPIN: u32 = 0x00fd_f130;
/// `GetStringTypeW` (import slot).
const IMPORT_GET_STRING_TYPE_W: u32 = 0x00fd_f230;
/// `GetStringTypeA` (import slot).
const IMPORT_GET_STRING_TYPE_A: u32 = 0x00fd_f234;
/// `MultiByteToWideChar` (import slot).
const IMPORT_MULTI_BYTE_TO_WIDE_CHAR: u32 = 0x00fd_f128;
/// The wide string `KERNEL32.DLL`.
const KERNEL32_NAME: u32 = 0x010f_29e8;
/// The strings `FlsAlloc`, `FlsGetValue`, `FlsSetValue`, `FlsFree`,
/// `EncodePointer` and `DecodePointer`.
const FLS_ALLOC_NAME: u32 = 0x010f_2a34;
const FLS_GET_VALUE_NAME: u32 = 0x010f_2a28;
const FLS_SET_VALUE_NAME: u32 = 0x010f_2a1c;
const FLS_FREE_NAME: u32 = 0x010f_2a14;
const ENCODE_POINTER_NAME: u32 = 0x010f_29d8;
const DECODE_POINTER_NAME: u32 = 0x010f_2a04;
/// The four FLS function pointers `__mtinit` fills (encoded once set):
/// `FlsAlloc`, `FlsGetValue`, `FlsSetValue`, `FlsFree`.
const FLS_ALLOC_POINTER: u32 = 0x0126_fe00;
const FLS_GET_VALUE_POINTER: u32 = 0x0126_fe04;
const FLS_SET_VALUE_POINTER: u32 = 0x0126_fe08;
const FLS_FREE_POINTER: u32 = 0x0126_fe0c;
/// `__tlsindex`: the TLS slot holding the `FlsGetValue` pointer (-1 until
/// `__mtinit`).
const TLS_INDEX: u32 = 0x011c_1cc0;
/// `__crtTlsAlloc` (`00ed1004`): the `FlsAlloc` stand-in when the system
/// has no FLS (`TlsAlloc`, its callback ignored).
const CRT_TLS_ALLOC: u32 = 0x00ed_1004;
/// `_freefls` (`00ed11f8`), the FLS destructor `FlsAlloc` is given.
const FREE_FLS: u32 = 0x00ed_11f8;
/// `__mtdeletelocks` (`00edde08`), where `__mtterm` ends.
const MT_DELETE_LOCKS: u32 = 0x00ed_de08;
/// The lock table: 0x24 entries of (critical section, kind), kind 1 for
/// the locks `__mtinitlocks` sets up from the static block at `012705c0`.
const LOCK_TABLE: u32 = 0x011c_28a8;
const STATIC_LOCKS: u32 = 0x0127_05c0;
/// `_crtheap`, the CRT heap's handle (0 before the heap's start-up: too
/// early, `__mtinitlocknum` and `_malloc` end the program).
const LOCKS_READY: u32 = 0x0127_05bc;
/// `_FF_MSGBANNER` (`00ed2ab7`, no arguments) and `_NMSG_WRITE`
/// (`00ed290c`, cdecl: message number).
const FF_MSGBANNER: u32 = 0x00ed_2ab7;
const NMSG_WRITE: u32 = 0x00ed_290c;
/// The encoded exit routine `_amsg_exit` calls with 0xFF.
const ENCODED_EXIT: u32 = 0x011c_19c0;
/// `___addlocaleref`, `___removelocaleref`, `___freetlocinfo` (cdecl, one
/// word each).
const ADD_LOCALE_REF: u32 = 0x00ec_f5b6;
const REMOVE_LOCALE_REF: u32 = 0x00ec_f645;
const FREE_TLOCINFO: u32 = 0x00ec_f46d;
/// The CRT's initial (static) locale info, never freed.
const INITIAL_LOCALE_INFO: u32 = 0x011c_2320;
/// `_malloc` (`00ecd1c7`) and `__freea` (`00ec8e14`), cdecl.
const MALLOC_BODY: u32 = 0x00ec_d1c7;
const FREEA: u32 = 0x00ec_8e14;
/// `___ansicp` (`00ee13e5`, cdecl: locale id) and `___convertcp`
/// (`00ee142e`, cdecl, six words).
const ANSI_CODE_PAGE: u32 = 0x00ee_13e5;
const CONVERT_CODE_PAGE: u32 = 0x00ee_142e;
/// Which string-type API works: 0 unknown, 1 the wide one, 2 the ANSI one.
const STRING_TYPE_MODE: u32 = 0x0127_07c0;
/// The wide empty string the probe passes.
const EMPTY_WIDE_STRING: u32 = 0x0101_1d78;

// Translated from 00eccc25 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crt_waiting_on_module_handle` (engine map; cdecl: module name): sleeps
/// 1000 ms, then 2000, ... while `GetModuleHandleW` finds nothing, giving
/// up past 60000. Returns the handle (0 when it gave up).
pub fn crt_waiting_on_module_handle(e: &mut Engine, name: u32) -> u32 {
    let mut wait = 1000u32;
    loop {
        e.call(IMPORT_SLEEP, &[wait]);
        let handle = e.call(IMPORT_GET_MODULE_HANDLE_W, &[name]).u32();
        wait += 1000;
        if wait > 60000 || handle != 0 {
            return handle;
        }
    }
}

/// The function `name` of `KERNEL32.DLL` (`GetModuleHandleW`, waiting for
/// it when needed, then `GetProcAddress`), 0 without the module: the
/// shared tail of `__encode_pointer` and `__decode_pointer`.
fn kernel32_function(e: &mut Engine, name: u32) -> u32 {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
        if module == 0 {
            return 0;
        }
    }
    e.call(IMPORT_GET_PROC_ADDRESS, &[module, name]).u32()
}

/// `EncodePointer`/`DecodePointer` as the thread block caches them
/// (`ptd + offset`), else from `KERNEL32.DLL` (`name`); the value passed
/// through it, or unchanged when there is none.
fn code_pointer(e: &mut Engine, value: u32, offset: u32, name: u32) -> u32 {
    let mut function = 0;
    let mut cached = false;
    let tls_index = e.mem.u32(TLS_INDEX);
    if e.call(IMPORT_TLS_GET_VALUE, &[tls_index]).u32() != 0 {
        let fls_index = e.mem.u32(FLS_INDEX);
        if fls_index != u32::MAX {
            let tls_index = e.mem.u32(TLS_INDEX);
            let get_value = e.call(IMPORT_TLS_GET_VALUE, &[tls_index]).u32();
            let block = e.call(get_value, &[fls_index]).u32();
            if block != 0 {
                function = e.mem.u32(block + offset);
                cached = true;
            }
        }
    }
    if !cached {
        function = kernel32_function(e, name);
    }
    if function != 0 {
        e.call(function, &[value]).u32()
    } else {
        value
    }
}

// Translated from 00ed0f17 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__encode_pointer` (engine map; cdecl: pointer): through the thread
/// block's `EncodePointer` (+0x1F8) when the TLS slot and FLS slot give one,
/// else `KERNEL32.DLL`'s; unchanged without either.
pub fn encode_pointer(e: &mut Engine, value: u32) -> u32 {
    code_pointer(e, value, 0x1f8, ENCODE_POINTER_NAME)
}

// Translated from 00ed0f92 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__decode_pointer` (Ghidra's function ID; the engine map's `doexit` name
/// is folded code; cdecl: pointer): as [`encode_pointer`] with
/// `DecodePointer` (+0x1FC).
pub fn decode_pointer(e: &mut Engine, value: u32) -> u32 {
    code_pointer(e, value, 0x1fc, DECODE_POINTER_NAME)
}

// Translated from 00ed0f89 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__encoded_null` (engine map): `__encode_pointer(0)`.
pub fn encoded_null(e: &mut Engine) -> u32 {
    encode_pointer(e, 0)
}

// Translated from 00ed100d (decompiled, FalloutNV.exe 1.4.0.525)
/// `___set_flsgetvalue` (engine map; cdecl): the TLS slot's value (the
/// `FlsGetValue` function); when empty, the decoded `FlsGetValue` pointer,
/// stored in the slot.
pub fn set_flsgetvalue(e: &mut Engine) -> u32 {
    let index = e.mem.u32(TLS_INDEX);
    let mut value = e.call(IMPORT_TLS_GET_VALUE, &[index]).u32();
    if value == 0 {
        let encoded = e.mem.u32(FLS_GET_VALUE_POINTER);
        value = decode_pointer(e, encoded);
        let index = e.mem.u32(TLS_INDEX);
        e.call(IMPORT_TLS_SET_VALUE, &[index, value]);
    }
    value
}

// Translated from 00ed1004 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtTlsAlloc` (Ghidra made no function here; read from the bytes;
/// stdcall, one word): `TlsAlloc()`, the callback ignored.
pub fn crt_tls_alloc(e: &mut Engine, _unused_1: u32) -> u32 {
    e.call(IMPORT_TLS_ALLOC, &[]).u32()
}

// Translated from 00ede1fc (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `0127072c`.
pub fn fn_00ede1fc(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_072c, value);
}

// Translated from 00ede18d (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270728`.
pub fn fn_00ede18d(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0728, value);
}

// Translated from 00ed2c6f (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270464`.
pub fn fn_00ed2c6f(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0464, value);
}

// Translated from 00ede17e (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270724`.
pub fn fn_00ede17e(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0724, value);
}

// Translated from 00eddf6c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__initp_misc_winsig` (engine map): stores its argument in the four
/// signal-handler words `01270710`..`0127071c`.
pub fn initp_misc_winsig(e: &mut Engine, value: u32) {
    for slot in [0x0127_0710, 0x0127_0714, 0x0127_0718, 0x0127_071c] {
        e.mem.set_u32(slot, value);
    }
}

// Translated from 00ec5eec (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): an empty function (`RET`);
/// its one word is not read.
pub fn fn_00ec5eec(_e: &mut Engine, _unused_1: u32) {}

// Translated from 00ed1538 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__initp_eh_hooks` (engine map; its word is not read): stores the
/// encoded `00ed14b4` in `0126fe10`.
pub fn initp_eh_hooks(e: &mut Engine, _unused_1: u32) {
    let encoded = encode_pointer(e, 0x00ed_14b4);
    e.mem.set_u32(0x0126_fe10, encoded);
}

/// `00ec7af9`, the third of `__init_pointers`' set-ups
/// (`units/unplaced/run_00ec7ad2.rs`).
const INITP_00EC7AF9: u32 = 0x00ec_7af9;

// Translated from 00eccf0f (decompiled, FalloutNV.exe 1.4.0.525)
/// `__init_pointers` (engine map; cdecl): hands the encoded null to the
/// CRT's eight pointer set-ups and keeps the encoded `00eccedb` at
/// `011c19c0` (the exit routine `_amsg_exit` calls).
pub fn init_pointers(e: &mut Engine) {
    let null = encoded_null(e);
    fn_00ede1fc(e, null);
    fn_00ede18d(e, null);
    e.call(INITP_00EC7AF9, &[null]);
    fn_00ed2c6f(e, null);
    fn_00ede17e(e, null);
    initp_misc_winsig(e, null);
    fn_00ec5eec(e, null);
    initp_eh_hooks(e, null);
    let exit = encode_pointer(e, 0x00ec_cedb);
    e.mem.set_u32(ENCODED_EXIT, exit);
}

// Translated from 00ede19c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtInitCritSecAndSpinCount` (engine map; cdecl: section, spin count):
/// `InitializeCriticalSectionAndSpinCount`'s result.
// Translation incomplete: 00ede19c - the SEH frame and its __except handler (__SEH_prolog4/__SEH_epilog4) are not translated.
pub fn crt_init_crit_sec_and_spin_count(e: &mut Engine, section: u32, spin: u32) -> u32 {
    e.call(IMPORT_INIT_CRITICAL_SECTION_SPIN, &[section, spin])
        .u32()
}

// Translated from 00edddbd (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinitlocks` (engine map; cdecl): gives each lock of kind 1 its
/// critical section from the static block (0x18 bytes each, in order) and
/// initializes it (spin count 0xFA0); a failure clears that entry and
/// returns 0. Returns 1.
pub fn mtinitlocks(e: &mut Engine) -> u32 {
    let mut section = STATIC_LOCKS;
    for index in 0..0x24u32 {
        let entry = LOCK_TABLE + index * 8;
        if e.mem.u32(entry + 4) == 1 {
            e.mem.set_u32(entry, section);
            section += 0x18;
            let current = e.mem.u32(entry);
            if crt_init_crit_sec_and_spin_count(e, current, 0xfa0) == 0 {
                e.mem.set_u32(entry, 0);
                return 0;
            }
        }
    }
    1
}

// Translated from 00eddf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__mtinitlocknum` (its own function; it uses
/// no frame): `_unlock(0xA)`.
pub fn fn_00eddf30(e: &mut Engine) {
    unlock(e, 0xa);
}

// Translated from 00edde76 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinitlocknum` (engine map; cdecl: lock number): before the CRT's
/// start-up is done (`012705bc` 0) writes the banner and message 0x1E and
/// exits with 0xFF. An entry that has its section returns 1; otherwise a
/// 0x18-byte section is allocated (`ENOMEM` and 0 when that fails) and,
/// under lock 0xA, initialized into the entry unless another thread did it
/// first (then freed); a failed initialization frees it, sets `ENOMEM` and
/// returns 0.
// Translation incomplete: 00edde76 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn mtinitlocknum(e: &mut Engine, number: u32) -> u32 {
    let mut result = 1;
    if e.mem.u32(LOCKS_READY) == 0 {
        e.call(FF_MSGBANNER, &[]);
        e.call(NMSG_WRITE, &[0x1e]);
        // `___crtExitProcess` by address: it does not return.
        e.call(0x00ec_cca9, &[0xff]);
    }
    let entry = LOCK_TABLE.wrapping_add(number.wrapping_mul(8));
    if e.mem.u32(entry) != 0 {
        return 1;
    }
    let section = malloc(e, 0x18).addr();
    if section == 0 {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
    lock(e, 0xa);
    if e.mem.u32(entry) == 0 {
        if crt_init_crit_sec_and_spin_count(e, section, 0xfa0) == 0 {
            free(e, Ptr::new(section));
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0xc);
            result = 0;
        } else {
            e.mem.set_u32(entry, section);
        }
    } else {
        free(e, Ptr::new(section));
    }
    fn_00eddf30(e);
    result
}

// Translated from 00eddf39 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lock` (engine map; cdecl: lock number): sets up the lock's section on
/// first use (`__mtinitlocknum`; failing that, `_amsg_exit(0x11)`) and
/// enters it.
pub fn lock(e: &mut Engine, number: u32) {
    let entry = LOCK_TABLE.wrapping_add(number.wrapping_mul(8));
    if e.mem.u32(entry) == 0 && mtinitlocknum(e, number) == 0 {
        amsg_exit(e, 0x11);
    }
    let section = e.mem.u32(entry);
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[section]);
}

// Translated from 00edde5f (decompiled, FalloutNV.exe 1.4.0.525)
/// `_unlock` (cdecl: lock number): leaves the lock's section.
pub fn unlock(e: &mut Engine, number: u32) {
    let section = e.mem.u32(LOCK_TABLE.wrapping_add(number.wrapping_mul(8)));
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
}

// Translated from 00eccc55 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_amsg_exit` (engine map; cdecl: run-time error number): writes the
/// banner and the message, then calls the decoded exit routine
/// (`011c19c0`) with 0xFF.
pub fn amsg_exit(e: &mut Engine, number: u32) {
    e.call(FF_MSGBANNER, &[]);
    e.call(NMSG_WRITE, &[number]);
    let encoded = e.mem.u32(ENCODED_EXIT);
    let exit = decode_pointer(e, encoded);
    e.call(exit, &[0xff]);
}

// Translated from 00ed1153 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first `__finally` block of `_initptd` (its own function; it uses no
/// frame): `_unlock(0xD)`.
pub fn fn_00ed1153(e: &mut Engine) {
    unlock(e, 0xd);
}

// Translated from 00ed115c (decompiled, FalloutNV.exe 1.4.0.525)
/// The second `__finally` block of `_initptd` (its own function; it uses
/// no frame): `_unlock(0xC)`.
pub fn fn_00ed115c(e: &mut Engine) {
    unlock(e, 0xc);
}

// Translated from 00ed107e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_initptd` (engine map; cdecl: thread block, locale info): sets the
/// block's `_pxcptacttab` (+0x5C, `010fdca0`), `_holdrand` (+0x14, 1),
/// `EncodePointer`/`DecodePointer` (+0x1F8/+0x1FC, from `KERNEL32.DLL` when
/// it is found), `_ownlocale` (+0x70, 1), the two `'C'` bytes (+0xC8,
/// +0x14B), the multibyte info (+0x68, the static one, its count raised
/// under lock 0xD) and the locale info (+0x6C, `locale` or `__ptlocinfo`,
/// its references added under lock 0xC).
// Translation incomplete: 00ed107e - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn initptd(e: &mut Engine, block: u32, locale: u32) {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
    }
    e.mem.set_u32(block + 0x5c, 0x010f_dca0);
    e.mem.set_u32(block + 0x14, 1);
    if module != 0 {
        let encode = e
            .call(IMPORT_GET_PROC_ADDRESS, &[module, ENCODE_POINTER_NAME])
            .u32();
        e.mem.set_u32(block + 0x1f8, encode);
        let decode = e
            .call(IMPORT_GET_PROC_ADDRESS, &[module, DECODE_POINTER_NAME])
            .u32();
        e.mem.set_u32(block + 0x1fc, decode);
    }
    e.mem.set_u32(block + 0x70, 1);
    e.mem.set_u8(block + 0xc8, 0x43);
    e.mem.set_u8(block + 0x14b, 0x43);
    e.mem.set_u32(block + 0x68, INITIAL_MBC_INFO);
    lock(e, 0xd);
    let mbc = e.mem.u32(block + 0x68);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &[mbc]);
    fn_00ed1153(e);
    lock(e, 0xc);
    e.mem.set_u32(block + 0x6c, locale);
    if locale == 0 {
        let global = e.mem.u32(GLOBAL_LOCALE_INFO);
        e.mem.set_u32(block + 0x6c, global);
    }
    let info = e.mem.u32(block + 0x6c);
    e.call(ADD_LOCALE_REF, &[info]);
    fn_00ed115c(e);
}

// Translated from 00ed1041 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtterm` (engine map; cdecl): frees the FLS slot (the decoded
/// `FlsFree`) and the TLS slot (`TlsFree`) that are set, marking them -1,
/// then jumps to `__mtdeletelocks` (`00edde08`).
pub fn mtterm(e: &mut Engine) {
    let fls_index = e.mem.u32(FLS_INDEX);
    if fls_index != u32::MAX {
        let encoded = e.mem.u32(FLS_FREE_POINTER);
        let fls_free = decode_pointer(e, encoded);
        e.call(fls_free, &[fls_index]);
        e.mem.set_u32(FLS_INDEX, u32::MAX);
    }
    let tls_index = e.mem.u32(TLS_INDEX);
    if tls_index != u32::MAX {
        e.call(IMPORT_TLS_FREE, &[tls_index]);
        e.mem.set_u32(TLS_INDEX, u32::MAX);
    }
    e.call(MT_DELETE_LOCKS, &[]);
}

// Translated from 00ed1327 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinit` (engine map; cdecl): the CRT's thread start-up. Finds
/// `KERNEL32.DLL`'s `FlsAlloc`/`FlsGetValue`/`FlsSetValue`/`FlsFree`; when
/// one is missing uses `__crtTlsAlloc` and the import table's `TlsGetValue`,
/// `TlsSetValue` and `TlsFree` words instead. Allocates the TLS slot and
/// stores the `FlsGetValue` pointer in it, sets up the CRT's pointers,
/// encodes the four FLS pointers, sets up the locks, allocates the FLS slot
/// (with `_freefls`) and gives the calling thread its 0x214-byte block
/// (stored in the slot, `_initptd`, thread id, handle -1). Returns 1; a
/// failure returns 0 (after `__mtterm`, except when the TLS slot could not
/// be allocated or set).
pub fn mtinit(e: &mut Engine) -> u32 {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
    }
    if module == 0 {
        mtterm(e);
        return 0;
    }
    for (slot, name) in [
        (FLS_ALLOC_POINTER, FLS_ALLOC_NAME),
        (FLS_GET_VALUE_POINTER, FLS_GET_VALUE_NAME),
        (FLS_SET_VALUE_POINTER, FLS_SET_VALUE_NAME),
        (FLS_FREE_POINTER, FLS_FREE_NAME),
    ] {
        let function = e.call(IMPORT_GET_PROC_ADDRESS, &[module, name]).u32();
        e.mem.set_u32(slot, function);
    }
    let found = [
        FLS_ALLOC_POINTER,
        FLS_GET_VALUE_POINTER,
        FLS_SET_VALUE_POINTER,
        FLS_FREE_POINTER,
    ]
    .iter()
    .all(|slot| e.mem.u32(*slot) != 0);
    if !found {
        // The import table's words, as the code reads them.
        let get_value = e.mem.u32(IMPORT_TLS_GET_VALUE);
        e.mem.set_u32(FLS_GET_VALUE_POINTER, get_value);
        let free_slot = e.mem.u32(IMPORT_TLS_FREE);
        e.mem.set_u32(FLS_ALLOC_POINTER, CRT_TLS_ALLOC);
        let set_value = e.mem.u32(IMPORT_TLS_SET_VALUE);
        e.mem.set_u32(FLS_SET_VALUE_POINTER, set_value);
        e.mem.set_u32(FLS_FREE_POINTER, free_slot);
    }
    let tls_index = e.call(IMPORT_TLS_ALLOC, &[]).u32();
    e.mem.set_u32(TLS_INDEX, tls_index);
    if tls_index == u32::MAX {
        return 0;
    }
    let get_value = e.mem.u32(FLS_GET_VALUE_POINTER);
    if e.call(IMPORT_TLS_SET_VALUE, &[tls_index, get_value]).u32() == 0 {
        return 0;
    }
    init_pointers(e);
    for slot in [
        FLS_ALLOC_POINTER,
        FLS_GET_VALUE_POINTER,
        FLS_SET_VALUE_POINTER,
        FLS_FREE_POINTER,
    ] {
        let plain = e.mem.u32(slot);
        let encoded = encode_pointer(e, plain);
        e.mem.set_u32(slot, encoded);
    }
    if mtinitlocks(e) == 0 {
        mtterm(e);
        return 0;
    }
    let encoded = e.mem.u32(FLS_ALLOC_POINTER);
    let fls_alloc = decode_pointer(e, encoded);
    let fls_index = e.call(fls_alloc, &[FREE_FLS]).u32();
    e.mem.set_u32(FLS_INDEX, fls_index);
    if fls_index == u32::MAX {
        mtterm(e);
        return 0;
    }
    let block = calloc_crt(e, 1, 0x214).addr();
    if block == 0 {
        mtterm(e);
        return 0;
    }
    let encoded = e.mem.u32(FLS_SET_VALUE_POINTER);
    let fls_set = decode_pointer(e, encoded);
    let fls_index = e.mem.u32(FLS_INDEX);
    if e.call(fls_set, &[fls_index, block]).u32() == 0 {
        mtterm(e);
        return 0;
    }
    initptd(e, block, 0);
    let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32();
    e.mem.set_u32(block + 4, u32::MAX);
    e.mem.set_u32(block, thread);
    1
}

// Translated from 00ecf704 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetlocinfoEx_nolock` (engine map; register arguments: `slot` in
/// EAX, `info` in EDI, passed as these two words): 0 when either is null;
/// otherwise, when the slot holds another info, stores `info`, adds its
/// references and removes the old one's, freeing the old one when its count
/// is 0 and it is not the static initial info. Returns `info`.
pub fn update_tlocinfo_ex_nolock(e: &mut Engine, slot: u32, info: u32) -> u32 {
    if info == 0 || slot == 0 {
        return 0;
    }
    let old = e.mem.u32(slot);
    if old != info {
        e.mem.set_u32(slot, info);
        e.call(ADD_LOCALE_REF, &[info]);
        if old != 0 {
            e.call(REMOVE_LOCALE_REF, &[old]);
            if e.mem.u32(old) == 0 && old != INITIAL_LOCALE_INFO {
                e.call(FREE_TLOCINFO, &[old]);
            }
        }
    }
    info
}

// Translated from 00ee11e9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtGetStringTypeA_stat` (Ghidra's function ID; the locale object in
/// ECX, then seven cdecl words: info type, text, length, types, code page,
/// locale id, error flag). Probes once which API works (`GetStringTypeW` on
/// an empty wide string; `ERROR_CALL_NOT_IMPLEMENTED` 0x78 picks the ANSI
/// one). Wide: converts the text with `MultiByteToWideChar` (code page from
/// the locale when 0; `MB_ERR_INVALID_CHARS` with the error flag) into a
/// zeroed buffer (on the stack up to 0x400 bytes, marked 0xCCCC, else
/// `_malloc`'d, marked 0xDDDD) and returns `GetStringTypeW`'s result (0 on
/// a failure). ANSI: with the locale's id and code page when 0, converts the
/// text to the ANSI code page when it differs (`___convertcp`) and returns
/// `GetStringTypeA`'s result. The code-page conversion takes the length by
/// address; here it is a stack word.
// Translation incomplete: 00ee11e9 - the /GS stack cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn crt_get_string_type_a_stat(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
    error: u32,
) -> u32 {
    // EBP-0x8: the probe's out word and the wide result.
    e.with_stack(4, |e, result_slot| {
        let result_slot = result_slot.addr();
        let mut mode = e.mem.u32(STRING_TYPE_MODE);
        let mut wide = false;
        if mode == 0 {
            let probed = e
                .call(
                    IMPORT_GET_STRING_TYPE_W,
                    &[1, EMPTY_WIDE_STRING, 1, result_slot],
                )
                .u32();
            if probed != 0 {
                e.mem.set_u32(STRING_TYPE_MODE, 1);
                wide = true;
            } else if e.call(IMPORT_GET_LAST_ERROR, &[]).u32() == 0x78 {
                mode = 2;
                e.mem.set_u32(STRING_TYPE_MODE, 2);
            } else {
                mode = e.mem.u32(STRING_TYPE_MODE);
            }
        }
        if !wide {
            if mode == 2 || mode == 0 {
                return string_type_ansi(
                    e, locale, info_type, text, length, types, code_page, lcid,
                );
            }
            if mode != 1 {
                return 0;
            }
        }
        e.mem.set_u32(result_slot, 0);
        let code_page = if code_page == 0 {
            let info = e.mem.u32(locale);
            e.mem.u32(info + 4)
        } else {
            code_page
        };
        let flags = u32::from(error != 0) * 8 + 1;
        let count = e
            .call(
                IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                &[code_page, flags, text, length, 0, 0],
            )
            .u32();
        if count == 0 {
            return 0;
        }
        let size = count.wrapping_mul(2).wrapping_add(8);
        let fits = (count as i32) > 0 && count <= 0x7fff_fff0;
        if fits && size <= 0x400 {
            // `_alloca` through the probe `00ec5ec0`: a stack block here.
            e.with_stack(size, |e, block| {
                let block = block.addr();
                e.mem.set_u32(block, 0xcccc);
                string_type_wide(
                    e,
                    block + 8,
                    info_type,
                    text,
                    length,
                    types,
                    code_page,
                    count,
                    result_slot,
                )
            })
        } else {
            let mut buffer = 0;
            if fits {
                let block = e.call(MALLOC_BODY, &[size]).u32();
                if block != 0 {
                    e.mem.set_u32(block, 0xdddd);
                    buffer = block + 8;
                }
            }
            if buffer == 0 {
                return 0;
            }
            string_type_wide(
                e,
                buffer,
                info_type,
                text,
                length,
                types,
                code_page,
                count,
                result_slot,
            )
        }
    })
}

/// The wide path's tail: clears the buffer, converts into it and asks
/// `GetStringTypeW`, then `__freea`.
#[allow(clippy::too_many_arguments)]
fn string_type_wide(
    e: &mut Engine,
    buffer: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    count: u32,
    result_slot: u32,
) -> u32 {
    memset(e, Ptr::new(buffer), 0, count.wrapping_mul(2));
    let converted = e
        .call(
            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
            &[code_page, 1, text, length, buffer, count],
        )
        .u32();
    if converted != 0 {
        let result = e
            .call(
                IMPORT_GET_STRING_TYPE_W,
                &[info_type, buffer, converted, types],
            )
            .u32();
        e.mem.set_u32(result_slot, result);
    }
    e.call(FREEA, &[buffer]);
    e.mem.u32(result_slot)
}

/// The ANSI path of [`crt_get_string_type_a_stat`].
#[allow(clippy::too_many_arguments)]
fn string_type_ansi(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
) -> u32 {
    let lcid = if lcid == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info + 0x14)
    } else {
        lcid
    };
    let code_page = if code_page == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info + 4)
    } else {
        code_page
    };
    let ansi = e.call(ANSI_CODE_PAGE, &[lcid]).u32();
    if ansi == u32::MAX {
        return 0;
    }
    // The length argument's slot, which `___convertcp` updates.
    e.with_stack(4, |e, length_slot| {
        let length_slot = length_slot.addr();
        e.mem.set_u32(length_slot, length);
        let mut source = text;
        let mut converted = 0;
        if ansi != code_page {
            converted = e
                .call(
                    CONVERT_CODE_PAGE,
                    &[code_page, ansi, text, length_slot, 0, 0],
                )
                .u32();
            if converted == 0 {
                return 0;
            }
            source = converted;
        }
        let length = e.mem.u32(length_slot);
        let result = e
            .call(
                IMPORT_GET_STRING_TYPE_A,
                &[lcid, info_type, source, length, types],
            )
            .u32();
        if converted != 0 {
            free(e, Ptr::new(converted));
        }
        result
    })
}

// ---- Locks' end, run-time messages, locale references, heap (LIBCMT) ----

/// `DeleteCriticalSection` (import slot).
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `GetModuleFileNameA`, `GetStdHandle`, `WriteFile` (import slots).
const IMPORT_GET_MODULE_FILE_NAME_A: u32 = 0x00fd_f150;
const IMPORT_GET_STD_HANDLE: u32 = 0x00fd_f1e0;
const IMPORT_WRITE_FILE: u32 = 0x00fd_f0dc;
/// `GetLocaleInfoA`, `GetCPInfo`, `WideCharToMultiByte`, `HeapAlloc`
/// (import slots).
const IMPORT_GET_LOCALE_INFO_A: u32 = 0x00fd_f268;
const IMPORT_GET_CP_INFO: u32 = 0x00fd_f1d8;
const IMPORT_WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const IMPORT_HEAP_ALLOC: u32 = 0x00fd_f1a0;
/// The end of the lock table (0x24 entries of 8 bytes).
const LOCK_TABLE_END: u32 = 0x011c_29c8;
/// `__set_error_mode` (`00ee1dac`, cdecl: mode; 3 asks for the current one).
const SET_ERROR_MODE: u32 = 0x00ee_1dac;
/// `__app_type` (1: a console program).
const APP_TYPE: u32 = 0x011c_19b0;
/// The run-time message table: 0x17 entries of (number, text).
const RUNTIME_MESSAGES: u32 = 0x011c_1d30;
/// The 0x314-byte message buffer and the program name inside it.
const MESSAGE_BUFFER: u32 = 0x0127_0148;
const MESSAGE_PROGRAM_NAME: u32 = 0x0127_0161;
const MESSAGE_PROGRAM_NAME_END: u32 = 0x0127_0265;
const MESSAGE_BUFFER_END: u32 = 0x0127_045c;
/// The strings the message is built from: `Runtime Error!\n\nProgram: `,
/// `<program name unknown>`, `...`, `\n\n` and the box caption
/// `Microsoft Visual C++ Runtime Library`.
const MESSAGE_HEADER: u32 = 0x010f_3070;
const MESSAGE_UNKNOWN_PROGRAM: u32 = 0x010f_3058;
const MESSAGE_ELLIPSIS: u32 = 0x0107_2308;
const MESSAGE_BREAK: u32 = 0x0109_5188;
const MESSAGE_CAPTION: u32 = 0x010f_3030;
/// `strcpy_s`, `strcat_s`, `strncpy_s`, `strlen` (this file's stand-ins,
/// called by address as the code does).
const STRCPY_S_ADDR: u32 = 0x00ec_65a6;
const STRCAT_S_ADDR: u32 = 0x00ec_6bd2;
const STRNCPY_S_ADDR: u32 = 0x00ec_8d5f;
const STRLEN_ADDR: u32 = 0x00ec_6130;
/// `__invoke_watson` (`00ec7b08`; it does not return).
const INVOKE_WATSON_ADDR: u32 = 0x00ec_7b08;
/// `__crtMessageBoxA` (`00ee1c43`, cdecl: text, caption, type).
const CRT_MESSAGE_BOX_A: u32 = 0x00ee_1c43;
/// `doexit` (`00eccd99`, cdecl: code, quick, return to caller).
const DOEXIT_ADDR: u32 = 0x00ec_cd99;
/// The C locale's static category names, never released.
const C_LOCALE_NAMES: u32 = 0x011c_2318;
/// The C locale's static `lconv` and its pieces, never freed.
const C_LCONV: u32 = 0x011c_19d4;
const C_LC_TIME: u32 = 0x011c_2a98;
/// The `lconv` and `lc_time` frees (`00ee06e6`, `00ee04d7`, `00ee02c9`;
/// cdecl, one word).
const FREE_LCONV_MONETARY: u32 = 0x00ee_06e6;
const FREE_LCONV_NUMERIC: u32 = 0x00ee_04d7;
const FREE_LC_TIME: u32 = 0x00ee_02c9;
/// `__active_heap` (1 the system heap, 3 the small-block heap) and
/// `_newmode`.
const ACTIVE_HEAP: u32 = 0x0127_0948;
const NEW_MODE: u32 = 0x0127_0730;
/// `__sbh_alloc_block`-style small-block allocation (`00ecd178`, cdecl:
/// size) and `_callnewh` (`00ede20b`, cdecl: size).
const SMALL_BLOCK_ALLOC: u32 = 0x00ec_d178;
const CALL_NEW_HANDLER: u32 = 0x00ed_e20b;

// Translated from 00edde08 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtdeletelocks` (engine map; cdecl): deletes and frees every lock
/// section that is set and not of kind 1 (clearing its entry), then deletes
/// the kind-1 (static) sections.
pub fn mtdeletelocks(e: &mut Engine) {
    let mut entry = LOCK_TABLE;
    while entry < LOCK_TABLE_END {
        let section = e.mem.u32(entry);
        if section != 0 && e.mem.u32(entry + 4) != 1 {
            e.call(IMPORT_DELETE_CRITICAL_SECTION, &[section]);
            free(e, Ptr::new(section));
            e.mem.set_u32(entry, 0);
        }
        entry += 8;
    }
    let mut entry = LOCK_TABLE;
    while entry < LOCK_TABLE_END {
        let section = e.mem.u32(entry);
        if section != 0 && e.mem.u32(entry + 4) == 1 {
            e.call(IMPORT_DELETE_CRITICAL_SECTION, &[section]);
        }
        entry += 8;
    }
}

// Translated from 00ed2ab7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_FF_MSGBANNER` (LIBCMT/crt0msg.c; no arguments): for a console program
/// (error mode 1, or the default mode with `__app_type` 1) writes the
/// messages 0xFC (a new line) and 0xFF (the banner).
pub fn ff_msgbanner(e: &mut Engine) {
    let console = e.call(SET_ERROR_MODE, &[3]).u32() == 1
        || (e.call(SET_ERROR_MODE, &[3]).u32() == 0 && e.mem.u32(APP_TYPE) == 1);
    if console {
        nmsg_write(e, 0xfc);
        nmsg_write(e, 0xff);
    }
}

/// `__invoke_watson(0, 0, 0, 0, 0)` after a failed string call (it does
/// not return).
fn watson_unless_zero(e: &mut Engine, result: u32) {
    if result != 0 {
        e.call(INVOKE_WATSON_ADDR, &[0, 0, 0, 0, 0]);
    }
}

// Translated from 00ed290c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_NMSG_WRITE` (LIBCMT/crt0msg.c; cdecl: message number): for a number in
/// the run-time message table, a console program writes the text to the
/// standard error handle (`WriteFile`); otherwise (except for 0xFC) builds
/// `Runtime Error!\n\nProgram: <name, cut to "...">\n\n<text>` in the
/// message buffer and shows it with `__crtMessageBoxA` (type 0x12010).
pub fn nmsg_write(e: &mut Engine, number: u32) {
    let mut index = 0u32;
    while index < 0x17 {
        if number == e.mem.u32(RUNTIME_MESSAGES + index * 8) {
            break;
        }
        index += 1;
    }
    if index >= 0x17 {
        return;
    }
    let console = e.call(SET_ERROR_MODE, &[3]).u32() == 1
        || (e.call(SET_ERROR_MODE, &[3]).u32() == 0 && e.mem.u32(APP_TYPE) == 1);
    if console {
        let handle = e.call(IMPORT_GET_STD_HANDLE, &[(-12i32) as u32]).u32();
        if handle != 0 && handle != u32::MAX {
            let text = e.mem.u32(RUNTIME_MESSAGES + index * 8 + 4);
            e.with_stack(4, |e, written| {
                let length = e.call(STRLEN_ADDR, &[text]).u32();
                e.call(
                    IMPORT_WRITE_FILE,
                    &[handle, text, length, written.addr(), 0],
                );
            });
        }
        return;
    }
    if number == 0xfc {
        return;
    }
    let r = e
        .call(STRCPY_S_ADDR, &[MESSAGE_BUFFER, 0x314, MESSAGE_HEADER])
        .u32();
    watson_unless_zero(e, r);
    e.mem.set_u8(MESSAGE_PROGRAM_NAME_END, 0);
    let named = e
        .call(
            IMPORT_GET_MODULE_FILE_NAME_A,
            &[0, MESSAGE_PROGRAM_NAME, 0x104],
        )
        .u32();
    if named == 0 {
        let r = e
            .call(
                STRCPY_S_ADDR,
                &[MESSAGE_PROGRAM_NAME, 0x2fb, MESSAGE_UNKNOWN_PROGRAM],
            )
            .u32();
        watson_unless_zero(e, r);
    }
    let length = e.call(STRLEN_ADDR, &[MESSAGE_PROGRAM_NAME]).u32();
    if length.wrapping_add(1) > 0x3c {
        let length = e.call(STRLEN_ADDR, &[MESSAGE_PROGRAM_NAME]).u32();
        let start = length.wrapping_add(MESSAGE_PROGRAM_NAME - 0x3b);
        let r = e
            .call(
                STRNCPY_S_ADDR,
                &[
                    start,
                    MESSAGE_BUFFER_END.wrapping_sub(start),
                    MESSAGE_ELLIPSIS,
                    3,
                ],
            )
            .u32();
        watson_unless_zero(e, r);
    }
    let r = e
        .call(STRCAT_S_ADDR, &[MESSAGE_BUFFER, 0x314, MESSAGE_BREAK])
        .u32();
    watson_unless_zero(e, r);
    let text = e.mem.u32(RUNTIME_MESSAGES + index * 8 + 4);
    let r = e.call(STRCAT_S_ADDR, &[MESSAGE_BUFFER, 0x314, text]).u32();
    watson_unless_zero(e, r);
    e.call(
        CRT_MESSAGE_BOX_A,
        &[MESSAGE_BUFFER, MESSAGE_CAPTION, 0x12010],
    );
}

// Translated from 00eccedb (decompiled, FalloutNV.exe 1.4.0.525)
/// `__exit` (engine map; cdecl: exit code): `doexit(code, 1, 0)`, the quick
/// exit `_amsg_exit` reaches through its encoded pointer.
pub fn exit_quick(e: &mut Engine, code: u32) {
    e.call(DOEXIT_ADDR, &[code, 1, 0]);
}

/// Applies `step` (`InterlockedIncrement` or `InterlockedDecrement`) to every
/// count of the locale info `info`, in the order the CRT walks them: the
/// info, its `lconv` counts (+0xB0, +0xB8, +0xB4, +0xC0), the six
/// categories' name and wide-name counts that are not the C locale's, and
/// the `lc_time` count (+0xD4's +0xB4).
fn locale_counts(e: &mut Engine, info: u32, step: u32) {
    e.call(step, &[info]);
    for offset in [0xb0, 0xb8, 0xb4, 0xc0] {
        let count = e.mem.u32(info + offset);
        if count != 0 {
            e.call(step, &[count]);
        }
    }
    let mut category = info + 0x50;
    for _ in 0..6 {
        if e.mem.u32(category - 8) != C_LOCALE_NAMES {
            let count = e.mem.u32(category);
            if count != 0 {
                e.call(step, &[count]);
            }
        }
        if e.mem.u32(category - 4) != 0 {
            let count = e.mem.u32(category + 4);
            if count != 0 {
                e.call(step, &[count]);
            }
        }
        category += 0x10;
    }
    let time = e.mem.u32(info + 0xd4);
    e.call(step, &[time.wrapping_add(0xb4)]);
}

// Translated from 00ecf5b6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___addlocaleref` (engine map; cdecl: locale info): raises every
/// reference count of the info (`InterlockedIncrement`).
pub fn addlocaleref(e: &mut Engine, info: u32) {
    locale_counts(e, info, IMPORT_INTERLOCKED_INCREMENT);
}

// Translated from 00ecf645 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___removelocaleref` (engine map; cdecl: locale info): lowers every
/// reference count of a non-null info (`InterlockedDecrement`). Returns
/// the info.
pub fn removelocaleref(e: &mut Engine, info: u32) -> u32 {
    if info != 0 {
        locale_counts(e, info, IMPORT_INTERLOCKED_DECREMENT);
    }
    info
}

// Translated from 00ecf46d (decompiled, FalloutNV.exe 1.4.0.525)
/// `___freetlocinfo` (engine map; cdecl: locale info): frees what has no
/// reference left: the `lconv` (+0xBC, unless it is the C locale's) with
/// its monetary (+0xB8) and numeric (+0xB4) parts, the ctype tables (+0xC0
/// count; the blocks before +0xC4, +0xCC, +0xD0), the `lc_time` (+0xD4,
/// unless the C locale's), the categories' names (not the C locale's), and
/// the info itself.
pub fn freetlocinfo(e: &mut Engine, info: u32) {
    let lconv = e.mem.u32(info + 0xbc);
    if lconv != 0 && lconv != C_LCONV {
        let count = e.mem.u32(info + 0xb0);
        if count != 0 && e.mem.u32(count) == 0 {
            let monetary = e.mem.u32(info + 0xb8);
            if monetary != 0 && e.mem.u32(monetary) == 0 {
                free(e, Ptr::new(monetary));
                let lconv = e.mem.u32(info + 0xbc);
                e.call(FREE_LCONV_MONETARY, &[lconv]);
            }
            let numeric = e.mem.u32(info + 0xb4);
            if numeric != 0 && e.mem.u32(numeric) == 0 {
                free(e, Ptr::new(numeric));
                let lconv = e.mem.u32(info + 0xbc);
                e.call(FREE_LCONV_NUMERIC, &[lconv]);
            }
            let count = e.mem.u32(info + 0xb0);
            free(e, Ptr::new(count));
            let lconv = e.mem.u32(info + 0xbc);
            free(e, Ptr::new(lconv));
        }
    }
    let ctype = e.mem.u32(info + 0xc0);
    if ctype != 0 && e.mem.u32(ctype) == 0 {
        let table = e.mem.u32(info + 0xc4).wrapping_sub(0xfe);
        free(e, Ptr::new(table));
        let lower = e.mem.u32(info + 0xcc).wrapping_sub(0x80);
        free(e, Ptr::new(lower));
        let upper = e.mem.u32(info + 0xd0).wrapping_sub(0x80);
        free(e, Ptr::new(upper));
        let ctype = e.mem.u32(info + 0xc0);
        free(e, Ptr::new(ctype));
    }
    let time = e.mem.u32(info + 0xd4);
    if time != C_LC_TIME && e.mem.u32(time + 0xb4) == 0 {
        e.call(FREE_LC_TIME, &[time]);
        let time = e.mem.u32(info + 0xd4);
        free(e, Ptr::new(time));
    }
    let mut category = info + 0x50;
    for _ in 0..6 {
        if e.mem.u32(category - 8) != C_LOCALE_NAMES {
            let name = e.mem.u32(category);
            if name != 0 && e.mem.u32(name) == 0 {
                free(e, Ptr::new(name));
            }
        }
        if e.mem.u32(category - 4) != 0 {
            let wide = e.mem.u32(category + 4);
            if wide != 0 && e.mem.u32(wide) == 0 {
                free(e, Ptr::new(wide));
            }
        }
        category += 0x10;
    }
    free(e, Ptr::new(info));
}

// Translated from 00ee13e5 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___ansicp` (engine map; cdecl: locale id): the locale's ANSI code page
/// (`GetLocaleInfoA(lcid, LOCALE_IDEFAULTANSICODEPAGE, buffer, 6)`, read
/// with `atol`), -1 when it is not known.
// Translation incomplete: 00ee13e5 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ansicp(e: &mut Engine, lcid: u32) -> u32 {
    e.with_stack(8, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u8(buffer + 6, 0);
        if e.call(IMPORT_GET_LOCALE_INFO_A, &[lcid, 0x1004, buffer, 6])
            .u32()
            == 0
        {
            u32::MAX
        } else {
            atol_body(e, Ptr::new(buffer)) as u32
        }
    })
}

// Translated from 00ee142e (decompiled, FalloutNV.exe 1.4.0.525)
/// `___convertcp` (engine map; cdecl: from code page, to code page, text,
/// length by address, output, output size): converts the text through
/// wide characters (`MultiByteToWideChar` / `WideCharToMultiByte`; a length
/// of -1 means up to the terminator). The wide buffer is on the stack up to
/// 0x400 bytes (marked 0xCCCC), else `_malloc`'d (0xDDDD), released with
/// `__freea`. With an output buffer the result is that buffer; without one
/// a new block (`_calloc_crt`) sized by the conversion, or by the wide
/// count for two single-byte code pages, whose length is stored back
/// (unless it was -1). Returns 0 for the same code page or any failure.
// Translation incomplete: 00ee142e - the /GS stack cookie check (00ec408c) is not translated.
pub fn convertcp(
    e: &mut Engine,
    from: u32,
    to: u32,
    text: u32,
    length_slot: u32,
    output: u32,
    output_size: u32,
) -> u32 {
    let length = e.mem.u32(length_slot);
    if from == to {
        return 0;
    }
    // CPINFO at EBP-0x18 (MaxCharSize first).
    let single_byte = e.with_stack(0x14, |e, info| {
        let info = info.addr();
        e.call(IMPORT_GET_CP_INFO, &[from, info]).u32() != 0
            && e.mem.u32(info) == 1
            && e.call(IMPORT_GET_CP_INFO, &[to, info]).u32() != 0
            && e.mem.u32(info) == 1
    });
    let count = if single_byte {
        if length == u32::MAX {
            e.call(STRLEN_ADDR, &[text]).u32().wrapping_add(1)
        } else {
            length
        }
    } else {
        let count = e
            .call(
                IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                &[from, 1, text, length, 0, 0],
            )
            .u32();
        if count == 0 {
            return 0;
        }
        count
    };
    let fits = (count as i32) > 0 && count <= 0x7fff_fff0;
    let size = count.wrapping_mul(2).wrapping_add(8);
    let convert = |e: &mut Engine, wide: u32| {
        convert_wide(
            e,
            wide,
            ConvertArgs {
                from,
                to,
                text,
                length,
                length_slot,
                output,
                output_size,
                count,
                single_byte,
            },
        )
    };
    if fits && size <= 0x400 {
        // `_alloca` through the probe `00ec5ec0`: a stack block here.
        e.with_stack(size, |e, block| {
            let block = block.addr();
            e.mem.set_u32(block, 0xcccc);
            convert(e, block + 8)
        })
    } else {
        let mut wide = 0;
        if fits {
            let block = e.call(MALLOC_BODY, &[size]).u32();
            if block != 0 {
                e.mem.set_u32(block, 0xdddd);
                wide = block + 8;
            }
        }
        if wide == 0 {
            return 0;
        }
        convert(e, wide)
    }
}

/// The values [`convertcp`]'s second half works with.
struct ConvertArgs {
    from: u32,
    to: u32,
    text: u32,
    length: u32,
    length_slot: u32,
    output: u32,
    output_size: u32,
    count: u32,
    single_byte: bool,
}

/// The second half of [`convertcp`]: into wide characters at `wide`, then
/// out to the target code page; releases the wide buffer.
fn convert_wide(e: &mut Engine, wide: u32, a: ConvertArgs) -> u32 {
    let mut result = 0;
    memset(e, Ptr::new(wide), 0, a.count.wrapping_mul(2));
    let converted = e
        .call(
            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
            &[a.from, 1, a.text, a.length, wide, a.count],
        )
        .u32();
    if converted != 0 {
        if a.output != 0 {
            let written = e
                .call(
                    IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                    &[a.to, 0, wide, a.count, a.output, a.output_size, 0, 0],
                )
                .u32();
            if written != 0 {
                result = a.output;
            }
        } else {
            let mut size = a.count;
            let mut sized = true;
            if !a.single_byte {
                size = e
                    .call(
                        IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                        &[a.to, 0, wide, a.count, 0, 0, 0, 0],
                    )
                    .u32();
                sized = size != 0;
            }
            if sized {
                result = calloc_crt(e, 1, size).addr();
                if result != 0 {
                    let written = e
                        .call(
                            IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                            &[a.to, 0, wide, size, result, size, 0, 0],
                        )
                        .u32();
                    if written == 0 {
                        free(e, Ptr::new(result));
                        result = 0;
                    } else if a.length != u32::MAX {
                        e.mem.set_u32(a.length_slot, written);
                    }
                }
            }
        }
    }
    freea(e, wide);
    result
}

// Translated from 00ecd1c7 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `_malloc` (engine map; cdecl: size). Above 0xFFFFFFE0 bytes
/// it calls the new handler, sets `ENOMEM` and returns 0. Otherwise, with
/// the CRT heap (`_crtheap`; none: banner, message 0x1E, exit 0xFF): from
/// the system heap (`HeapAlloc`, at least one byte) when `__active_heap` is
/// 1; else the small-block heap first when it is 3, then the system heap
/// with the size rounded up to 16. A failure retries while `_newmode` is
/// set and the new handler succeeds; otherwise `ENOMEM` and 0.
pub fn malloc_body(e: &mut Engine, size: u32) -> u32 {
    if size > 0xffff_ffe0 {
        e.call(CALL_NEW_HANDLER, &[size]);
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
    let mut request = size;
    loop {
        if e.mem.u32(LOCKS_READY) == 0 {
            ff_msgbanner(e);
            nmsg_write(e, 0x1e);
            e.call(0x00ec_cca9, &[0xff]);
        }
        let mode = e.mem.u32(ACTIVE_HEAP);
        let mut block = 0;
        let mut allocated = false;
        if mode == 1 {
            let bytes = if request != 0 { request } else { 1 };
            let heap = e.mem.u32(LOCKS_READY);
            block = e.call(IMPORT_HEAP_ALLOC, &[heap, 0, bytes]).u32();
            allocated = true;
        } else if mode == 3 {
            block = e.call(SMALL_BLOCK_ALLOC, &[request]).u32();
            allocated = block != 0;
        }
        if !allocated {
            if request == 0 {
                request = 1;
            }
            request = request.wrapping_add(0xf) & 0xffff_fff0;
            let heap = e.mem.u32(LOCKS_READY);
            block = e.call(IMPORT_HEAP_ALLOC, &[heap, 0, request]).u32();
        }
        if block != 0 {
            return block;
        }
        if e.mem.u32(NEW_MODE) == 0 {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0xc);
        } else if e.call(CALL_NEW_HANDLER, &[size]).u32() != 0 {
            request = size;
            continue;
        }
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
}

// Translated from 00ec8e14 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__freea` (engine map; cdecl: buffer): frees a non-null `_malloca`
/// buffer whose marker (8 bytes before it) is 0xDDDD (a heap block); a
/// stack block (0xCCCC) is left alone.
pub fn freea(e: &mut Engine, buffer: u32) {
    if buffer != 0 {
        let block = buffer - 8;
        if e.mem.u32(block) == 0xdddd {
            free(e, Ptr::new(block));
        }
    }
}

// Translated from 00ed11f8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_freefls` (the FLS destructor `__mtinit` registers; Ghidra made no
/// function here, read from the bytes; stdcall: thread block): for a
/// non-null block frees its buffers (+0x24, +0x2C, +0x34, +0x3C, +0x40,
/// +0x44, +0x48 when set, +0x5C unless the static table), releases its
/// multibyte info under lock 0xD (freed at a zero count unless the static
/// one) and its locale info under lock 0xC (freed when unreferenced unless
/// it is the global or the static one; the `__finally` blocks at
/// `00ed1312`/`00ed131e`, the unlocks, are this function's own code), then
/// frees the block.
// Translation incomplete: 00ed11f8 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn freefls(e: &mut Engine, block: u32) {
    if block == 0 {
        return;
    }
    for offset in [0x24, 0x2c, 0x34, 0x3c, 0x40, 0x44, 0x48] {
        let buffer = e.mem.u32(block + offset);
        if buffer != 0 {
            free(e, Ptr::new(buffer));
        }
    }
    let table = e.mem.u32(block + 0x5c);
    if table != 0x010f_dca0 {
        free(e, Ptr::new(table));
    }
    lock(e, 0xd);
    let mbc = e.mem.u32(block + 0x68);
    if mbc != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &[mbc]).u32() == 0
        && mbc != INITIAL_MBC_INFO
    {
        free(e, Ptr::new(mbc));
    }
    unlock(e, 0xd);
    lock(e, 0xc);
    let info = e.mem.u32(block + 0x6c);
    if info != 0 {
        removelocaleref(e, info);
        if info != e.mem.u32(GLOBAL_LOCALE_INFO)
            && info != INITIAL_LOCALE_INFO
            && e.mem.u32(info) == 0
        {
            freetlocinfo(e, info);
        }
    }
    unlock(e, 0xc);
    free(e, Ptr::new(block));
}

/// C `atof`: the longest prefix that is a decimal floating-point number.
fn parse_double(s: &[u8]) -> f64 {
    let t: Vec<u8> = s
        .iter()
        .copied()
        .skip_while(|c| *c == b' ' || (9..=13).contains(c))
        .collect();
    let mut end = 0;
    let mut best = 0.0;
    for i in 1..=t.len() {
        if let Ok(v) = std::str::from_utf8(&t[..i]).unwrap_or("x").parse::<f64>() {
            best = v;
            end = i;
        }
    }
    let _ = end;
    best
}

// Platform: replaces 00eca573 (atof, LIBCMT)
pub fn atof(e: &mut Engine, s: Ptr) -> f64 {
    parse_double(&e.mem.cstr(s.addr()))
}

// ---- Numbers --------------------------------------------------------------

// Platform: replaces 00ec62c0 (_ftol2, LIBCMT)
/// Float to 64-bit integer, truncating toward zero. The value comes in
/// x87 `ST0`, so in the uniform form it is a leading `f64`.
pub fn ftol2(_e: &mut Engine, v: f64) -> u64 {
    if v.is_nan() || v >= 9_223_372_036_854_775_808.0 || v < -9_223_372_036_854_775_808.0 {
        0x8000_0000_0000_0000 // the x87 "integer indefinite"
    } else {
        v.trunc() as i64 as u64
    }
}

// Platform: replaces 00ec6040 (_CIsqrt, LIBCMT): argument in ST0.
pub fn ci_sqrt(_e: &mut Engine, v: f64) -> f64 {
    v.sqrt()
}

// Platform: replaces 00ec6054 (sqrt, LIBCMT): argument on the stack.
pub fn sqrt(_e: &mut Engine, v: f64) -> f64 {
    v.sqrt()
}

// Platform: replaces 00ec782f (`eh vector constructor iterator', MSVC runtime;
// stdcall: array, element size, count, constructor, destructor): runs the
// constructor (`thiscall`) on each element in order. The destructor is only
// for unwinding after an exception, which the engine doesn't raise.
pub fn eh_vector_constructor_iterator(
    e: &mut Engine,
    array: u32,
    size: u32,
    count: i32,
    constructor: u32,
    _destructor: u32,
) {
    let mut element = array;
    for _ in 0..count.max(0) {
        e.call(constructor, &[element]);
        element = element.wrapping_add(size);
    }
}

// Platform: replaces 00ec9f70 (_CIcos, LIBCMT): argument in ST0.
pub fn ci_cos(_e: &mut Engine, v: f64) -> f64 {
    v.cos()
}

// Platform: replaces 00eca0a0 (_CIsin, LIBCMT): argument in ST0.
pub fn ci_sin(_e: &mut Engine, v: f64) -> f64 {
    v.sin()
}

// Platform: replaces 00ec9f30 (cos, LIBCMT): argument on the stack.
pub fn cos(_e: &mut Engine, v: f64) -> f64 {
    v.cos()
}

// Platform: replaces 00eca060 (sin, LIBCMT): argument on the stack.
pub fn sin(_e: &mut Engine, v: f64) -> f64 {
    v.sin()
}

// Platform: replaces 00ec9b00 (pow, LIBCMT)
pub fn pow(_e: &mut Engine, x: f64, y: f64) -> f64 {
    x.powf(y)
}

// Platform: replaces 00eca1d0 (acos, LIBCMT)
pub fn acos(_e: &mut Engine, x: f64) -> f64 {
    x.acos()
}

/// `_CIpow` (`00ec9b00`, base in `ST1`, exponent in `ST0`) and `_CIacos` (`00eca1d0`,
/// argument in `ST0`): the register-argument entries that stand in for the shared bodies.
const CI_POW: u32 = 0x00ec_9b00;
const CI_ACOS: u32 = 0x00ec_a1d0;

// Translated from 00ec9ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `pow` (LIBCMT, `cdecl`, the base then the exponent as doubles on the stack, the result in
/// `ST0`). It makes the same choice as `_CIpow` (`00ec9b00`): with `__use_sse2_mathfcns`
/// (`01270a64`) set and every MXCSR and x87 exception masked it jumps to the SSE2 body
/// (`00ed87a9`, the stack form of `_CIpow`'s `00ed8790`), otherwise it loads the exponent from
/// the stack (`00ed27c5`) and runs the x87 body `00ec9b5d`, the one `_CIpow` calls after storing
/// its two registers to the same stack slots. Only where the arguments come from differs, so
/// the bodies are reached through `_CIpow`, which stands in for them (`Platform`).
pub fn pow_cdecl(e: &mut Engine, x: f64, y: f64) -> f64 {
    e.call(CI_POW, &args![x, y]).f64()
}

// Translated from 00eca190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `acos` (LIBCMT, `cdecl`, one double on the stack, the result in `ST0`). It makes the same
/// choice as `_CIacos` (`00eca1d0`): with `__use_sse2_mathfcns` (`01270a64`) set and every
/// MXCSR and x87 exception masked it jumps to the SSE2 body (`00ed9b18`, which loads the stack
/// argument and falls into `_CIacos`'s `00ed9b1e`), otherwise it loads the argument from the
/// stack (`00ed27c5`) and falls into the x87 body `00eca228` that `_CIacos` calls. Only where the
/// argument comes from differs, so the bodies are reached through `_CIacos`, which stands in
/// for them (`Platform`).
pub fn acos_cdecl(e: &mut Engine, x: f64) -> f64 {
    e.call(CI_ACOS, &args![x]).f64()
}

/// Where the engine keeps the running thread's x87 control word (`FSTCW`/`FLDCW`): a word of
/// the thread's block past the exe's `.tls` data, holding `0x10000 | cw` once set; zero means
/// the control word every thread starts with, `0x027F` (all exceptions masked, 53-bit
/// precision, round to nearest: `crate::x87::DEFAULT_CONTROL_WORD`), the value the CRT's x87
/// code checks for. The FPU register the x87 arithmetic helpers round with
/// (`crate::x87::control_word`) always holds the same word: [`set_x87_control_word`] sets
/// both. A thread made by `CreateThread` keeps its word in its kernel object while it does
/// not run (`units::platform::run_as_thread` swaps it in).
const X87_CONTROL_WORD: u32 = 0x0e04;

/// The x87 control word as `FSTCW` reads it.
pub fn x87_control_word(e: &mut Engine) -> u16 {
    let at = e.tls() + X87_CONTROL_WORD;
    let slot = e.mem.u32(at);
    if slot & 0x1_0000 == 0 {
        crate::x87::DEFAULT_CONTROL_WORD
    } else {
        slot as u16
    }
}

/// `FLDCW`: loads the running thread's x87 control word, its word in the thread block and the
/// FPU register the x87 arithmetic helpers read (`crate::x87::set_control_word`), so precision
/// control (bits 8-9) applies to them from here on.
pub fn set_x87_control_word(e: &mut Engine, word: u16) {
    let at = e.tls() + X87_CONTROL_WORD;
    e.mem.set_u32(at, 0x1_0000 | u32::from(word));
    crate::x87::set_control_word(word);
}

// Translated from 00ed5abb (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ctrlfp(newctrl, mask)` (LIBCMT, `cdecl`; `fabs` `00ec6cde` calls it): reads the x87
/// control word (`FSTCW`), loads `(old & !mask) | (newctrl & mask)` (its low 16 bits,
/// `FLDCW`) and returns the old control word sign-extended (`MOVSX`).
pub fn ctrlfp(e: &mut Engine, newctrl: u32, mask: u32) -> i32 {
    let old = x87_control_word(e);
    let new = (!mask & u32::from(old)) | (newctrl & mask);
    set_x87_control_word(e, new as u16);
    i32::from(old as i16)
}

// Platform: replaces 00ec9050 (x87 arctangent, LIBCMT; the name is unconfirmed): argument
// in ST0. Dispatches on the SSE2 flag (`01270a64`) and the masked exception state to the
// SSE2 routine `00ed5d80`, otherwise computes `FPATAN` of the argument and 1 through
// `00ec90a8`; the error handling for NaN and infinite arguments is not modelled.
pub fn ci_atan(_e: &mut Engine, v: f64) -> f64 {
    v.atan()
}

// Platform: replaces 00ec9760 (atan2, LIBCMT): `cdecl`, `y` then `x` as doubles on the stack, the
// result in ST0. The function is `MOV EDX,0x11c26a0; JMP 0x00ed60eb`: the shared dispatcher that
// picks the SSE2 body (`__use_sse2_mathfcns`, `01270a64`) or the x87 body `00ed7cbc`, which
// computes `FPATAN` of the two stack operands; the descriptor handlers for NaN, infinite and zero
// operands and the error report (`_matherr`, `atan2`) are not modelled.
pub fn atan2(_e: &mut Engine, y: f64, x: f64) -> f64 {
    y.atan2(x)
}

// Platform: replaces 00ec8660 (__alloca_probe, LIBCMT)
/// The stack probe the compiler calls before a large frame. It takes its inputs in
/// registers, so they come first: `size` is `EAX` (the bytes wanted) and `esp` the value
/// of `ESP` at entry (it points at the return address). It touches each page between the
/// current stack page and the new top (a read of `[page]`), moves the return address to the
/// new top and returns. The return value is the `ESP` the caller continues with
/// (`esp - size + 4`; when `size` is above `esp` the new top is address 0 and the return
/// address is stored there); `EAX` on return holds the return address (the word at `esp`).
pub fn alloca_probe(e: &mut Engine, size: u32, esp: u32) -> u32 {
    // `LEA ECX, [ESP + 4]` after the `PUSH ECX`, minus `EAX`; zero when that borrows.
    let top = esp.saturating_sub(size);
    let mut page = esp.wrapping_sub(4) & 0xffff_f000;
    while top < page {
        page = page.wrapping_sub(0x1000);
        // `TEST [EAX], EAX`: a read that commits the guard page.
        let _ = e.mem.u32(page);
    }
    let return_address = e.mem.u32(esp);
    e.mem.set_u32(top, return_address);
    top.wrapping_add(4)
}

/// The import slot of KERNEL32 `GetSystemTimeAsFileTime` (`00fdf184`) and `__aulldiv`
/// (`00ecc590`, four words `(low, high, divisor low, divisor high)`, 64-bit unsigned
/// quotient in `EDX:EAX`).
const GET_SYSTEM_TIME_AS_FILE_TIME_SLOT: u32 = 0x00fd_f184;
const AULLDIV: u32 = 0x00ec_c590;

// Platform: replaces 00ecadda (__time64, LIBCMT)
/// `__time64(__time64_t *)` (cdecl): the seconds since 1970-01-01. The `FILETIME` from
/// `GetSystemTimeAsFileTime` (100 ns units since 1601) has 116444736000000000 added (as the
/// exe does, `0xfe624e212ac18000` modulo 2^64) and is divided by 10000000 through
/// `__aulldiv`. A result above 0x793406fff (the year 3000) becomes -1. The result is stored
/// to `out` when it is not null and returned (`EDX:EAX`).
pub fn time64(e: &mut Engine, out: Ptr) -> u64 {
    let (low, high) = e.with_stack(8, |e, filetime| {
        let function = e.mem.u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT);
        e.call(function, &args![filetime]);
        (e.mem.u32(filetime.addr()), e.mem.u32(filetime.addr() + 4))
    });
    let (low, carry) = low.overflowing_add(0x2ac1_8000);
    let high = high
        .wrapping_add(0xfe62_4e21)
        .wrapping_add(u32::from(carry));
    let quotient = e
        .call(AULLDIV, &args![low, high, 0x0098_9680u32, 0u32])
        .u64();
    let (quotient_low, quotient_high) = (quotient as u32, (quotient >> 32) as u32);
    let result = if (quotient_high as i32) < 7 {
        quotient
    } else if (quotient_high as i32) > 7 || quotient_low > 0x9340_6fff {
        u64::MAX
    } else {
        quotient
    };
    if out.addr() != 0 {
        e.mem.set_u32(out.addr(), result as u32);
        e.mem.set_u32(out.addr() + 4, (result >> 32) as u32);
    }
    result
}

/// The state of `rand`, per engine (MSVC keeps it per thread; one thread
/// here). Starts at 1 like MSVC's `holdrand`.
const RAND_STATE: u32 = 0x0e00;

// Platform: replaces 00ecadb8 (_rand, LIBCMT)
/// MSVC's linear congruential generator: `holdrand * 214013 + 2531011`,
/// bits 16 to 30 returned.
pub fn rand(e: &mut Engine) -> i32 {
    let at = e.tls() + RAND_STATE;
    let mut s = e.mem.u32(at);
    if s == 0 {
        s = 1;
    }
    s = s.wrapping_mul(214_013).wrapping_add(2_531_011);
    e.mem.set_u32(at, s);
    ((s >> 16) & 0x7fff) as i32
}

// ---- Formatted output ------------------------------------------------------

/// Reads the arguments of a `va_list` (a pointer to the pushed words: an
/// `int` is one word, a `double` or `__int64` two, low word first).
struct VaArgs {
    at: u32,
}

impl VaArgs {
    fn word(&mut self, e: &Engine) -> u32 {
        let v = e.mem.u32(self.at);
        self.at = self.at.wrapping_add(4);
        v
    }
    fn dword(&mut self, e: &Engine) -> u64 {
        let v = e.mem.u64(self.at);
        self.at = self.at.wrapping_add(8);
        v
    }
}

/// The flags of one conversion specification.
#[derive(Default, Clone, Copy)]
struct Spec {
    left: bool,
    plus: bool,
    space: bool,
    alt: bool,
    zero: bool,
}

/// Pads `body` (sign or prefix already split off as `head`) to `width`.
fn pad(out: &mut Vec<u8>, head: &[u8], body: &[u8], width: usize, spec: Spec, zero_ok: bool) {
    let len = head.len() + body.len();
    let fill = width.saturating_sub(len);
    if spec.left {
        out.extend_from_slice(head);
        out.extend_from_slice(body);
        out.resize(out.len() + fill, b' ');
    } else if spec.zero && zero_ok {
        out.extend_from_slice(head);
        out.resize(out.len() + fill, b'0');
        out.extend_from_slice(body);
    } else {
        out.resize(out.len() + fill, b' ');
        out.extend_from_slice(head);
        out.extend_from_slice(body);
    }
}

/// An integer conversion (`d i u o x X p`).
fn format_integer(
    out: &mut Vec<u8>,
    conv: u8,
    value: u64,
    negative: bool,
    spec: Spec,
    width: usize,
    precision: Option<usize>,
) {
    let mut digits = match conv {
        b'o' => format!("{value:o}"),
        b'x' => format!("{value:x}"),
        b'X' | b'p' => format!("{value:X}"),
        _ => format!("{value}"),
    }
    .into_bytes();
    let zero_ok = precision.is_none();
    let precision = precision.unwrap_or(1);
    if value == 0 && precision == 0 {
        digits.clear();
    }
    while digits.len() < precision {
        digits.insert(0, b'0');
    }
    if conv == b'o' && spec.alt && digits.first() != Some(&b'0') {
        digits.insert(0, b'0');
    }
    let mut head: Vec<u8> = Vec::new();
    match conv {
        b'd' | b'i' => {
            if negative {
                head.push(b'-');
            } else if spec.plus {
                head.push(b'+');
            } else if spec.space {
                head.push(b' ');
            }
        }
        b'x' if spec.alt && value != 0 => head.extend_from_slice(b"0x"),
        b'X' if spec.alt && value != 0 => head.extend_from_slice(b"0X"),
        _ => {}
    }
    pad(out, &head, &digits, width, spec, zero_ok);
}

/// The decimal digits of a double the way MSVC's runtime of 2008 produces
/// them: 17 significant digits (`$I10_OUTPUT`), which later conversions
/// round with `_fptostr` (round half up on the digit string). Returns the
/// digit string and the decimal point position (`decpt`: the number of
/// digits before the point). Infinities and NaNs yield MSVC's `1#INF`,
/// `1#QNAN`, `1#SNAN` and `1#IND` digit strings.
fn double_digits(v: f64) -> (Vec<u8>, i32) {
    let bits = v.to_bits();
    if v.is_infinite() {
        return (b"1#INF".to_vec(), 1);
    }
    if v.is_nan() {
        let text: &[u8] = if bits == 0xfff8_0000_0000_0000 {
            b"1#IND"
        } else if bits & (1 << 51) != 0 {
            b"1#QNAN"
        } else {
            b"1#SNAN"
        };
        return (text.to_vec(), 1);
    }
    if v == 0.0 {
        return (b"0".to_vec(), 1);
    }
    let s = format!("{:.16e}", v.abs());
    let (mantissa, exp) = s.split_once('e').unwrap_or((&s, "0"));
    let digits: Vec<u8> = mantissa.bytes().filter(u8::is_ascii_digit).collect();
    (digits, exp.parse::<i32>().unwrap_or(0) + 1)
}

/// `_fptostr`: the first `count` digits of `digits`, zero padded, rounded up
/// when the next digit is `'5'` or above (a carry out of the first digit
/// moves the decimal point). A negative `count` gives no digits and no
/// rounding. Returns the digits and the new `decpt`.
fn round_digits(digits: &[u8], decpt: i32, count: i32) -> (Vec<u8>, i32) {
    if count < 0 {
        return (Vec::new(), decpt);
    }
    let count = count as usize;
    // Slot 0 receives a carry.
    let mut buf: Vec<u8> = vec![b'0'];
    for i in 0..count {
        buf.push(*digits.get(i).unwrap_or(&b'0'));
    }
    if *digits.get(count).unwrap_or(&b'0') >= b'5' {
        let mut i = buf.len();
        loop {
            i -= 1;
            if buf[i] == b'9' {
                buf[i] = b'0';
            } else {
                buf[i] += 1;
                break;
            }
        }
    }
    if buf[0] == b'1' {
        (buf, decpt + 1)
    } else {
        (buf[1..].to_vec(), decpt)
    }
}

fn exponent_text(out: &mut Vec<u8>, upper: bool, exp: i32) {
    out.push(if upper { b'E' } else { b'e' });
    out.push(if exp < 0 { b'-' } else { b'+' });
    out.extend_from_slice(format!("{:03}", exp.abs()).as_bytes());
}

/// `%f` body (no sign): `digits`/`decpt` already rounded to the precision.
/// Digit `i` has the place value `10^(decpt - 1 - i)`.
fn fixed_body(digits: &[u8], decpt: i32, precision: usize, alt: bool) -> Vec<u8> {
    let mut body = Vec::new();
    if decpt <= 0 {
        body.push(b'0');
    } else {
        for i in 0..decpt {
            body.push(*digits.get(i as usize).unwrap_or(&b'0'));
        }
    }
    if precision > 0 || alt {
        body.push(b'.');
    }
    for k in 1..=precision as i32 {
        let i = decpt + k - 1;
        body.push(if i < 0 {
            b'0'
        } else {
            *digits.get(i as usize).unwrap_or(&b'0')
        });
    }
    body
}

/// `%e`/`%E` body from rounded digits.
fn exp_body(digits: &[u8], exp: i32, precision: usize, alt: bool, upper: bool) -> Vec<u8> {
    let mut body = vec![digits[0]];
    if precision > 0 || alt {
        body.push(b'.');
    }
    for k in 1..=precision {
        body.push(*digits.get(k).unwrap_or(&b'0'));
    }
    exponent_text(&mut body, upper, exp);
    body
}

/// A floating-point conversion (`e E f g G`).
fn format_float(
    out: &mut Vec<u8>,
    conv: u8,
    value: f64,
    spec: Spec,
    width: usize,
    precision: Option<usize>,
) {
    let head: &[u8] = if value.is_sign_negative() {
        b"-"
    } else if spec.plus {
        b"+"
    } else if spec.space {
        b" "
    } else {
        b""
    };
    let (digits, decpt) = double_digits(value);
    let precision = precision.unwrap_or(6);
    let body = match conv {
        b'f' => {
            let (d, dp) = round_digits(&digits, decpt, decpt + precision as i32);
            fixed_body(&d, dp, precision, spec.alt)
        }
        b'e' | b'E' => {
            let (mut d, dp) = round_digits(&digits, decpt, precision as i32 + 1);
            d.truncate(precision + 1);
            exp_body(&d, dp - 1, precision, spec.alt, conv == b'E')
        }
        _ => {
            let p = precision.max(1);
            let (mut d, dp) = round_digits(&digits, decpt, p as i32);
            d.truncate(p);
            let x = dp - 1;
            if !spec.alt {
                while d.len() > 1 && d.last() == Some(&b'0') {
                    d.pop();
                }
            }
            if x < -4 || x >= p as i32 {
                let frac = if spec.alt { p - 1 } else { d.len() - 1 };
                exp_body(&d, x, frac, spec.alt, conv == b'G')
            } else {
                let frac = if spec.alt {
                    (p as i32 - 1 - x) as usize
                } else {
                    (d.len() as i32 - dp).max(0) as usize
                };
                fixed_body(&d, dp, frac, spec.alt)
            }
        }
    };
    pad(out, head, &body, width, spec, true);
}

/// The size prefix of a conversion.
#[derive(Clone, Copy, PartialEq)]
enum Size {
    Default,
    Short,
    /// `l`, `w`: wide for `c` and `s`.
    Long,
    /// `ll`, `I64`.
    Quad,
}

/// MSVC `printf` formatting (`_output_l` of LIBCMT, C locale): `%d %i %u %o
/// %x %X %p %c %C %s %S %f %e %E %g %G %n %%`, flags `- + space # 0`, width
/// and precision (also `*`), size prefixes `h l ll L w I I32 I64` (`F` and
/// `N` ignored). Differences of the 2008 runtime from the C standard are
/// kept: three-digit exponents (`1.000000e+000`), floating-point digits
/// beyond the 17th are zeros, `1.#INF`/`1.#QNAN`/`1.#IND` for infinities
/// and NaNs, `%p` as 8 upper-case hex digits, `%05s` padded with zeros, and
/// `(null)` for a null string. A wide character or string is narrowed to
/// the low byte; a character above 0xff is dropped (the C locale's
/// `wctomb` fails). A conversion character that is none of these is written
/// as itself (the runtime's default state; unconfirmed in the disassembly).
/// Returns the formatted bytes.
pub fn format_va(e: &mut Engine, format: &[u8], va_list: u32) -> Vec<u8> {
    let mut va = VaArgs { at: va_list };
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0;
    while i < format.len() {
        let c = format[i];
        i += 1;
        if c != b'%' {
            out.push(c);
            continue;
        }
        let mut spec = Spec::default();
        while let Some(&f) = format.get(i) {
            match f {
                b'-' => spec.left = true,
                b'+' => spec.plus = true,
                b' ' => spec.space = true,
                b'#' => spec.alt = true,
                b'0' => spec.zero = true,
                _ => break,
            }
            i += 1;
        }
        let mut width = 0usize;
        if format.get(i) == Some(&b'*') {
            i += 1;
            let w = va.word(e) as i32;
            if w < 0 {
                spec.left = true;
            }
            width = w.unsigned_abs() as usize;
        } else {
            while let Some(d) = format.get(i).filter(|d| d.is_ascii_digit()) {
                width = width.saturating_mul(10).saturating_add((d - b'0') as usize);
                i += 1;
            }
        }
        let mut precision: Option<usize> = None;
        if format.get(i) == Some(&b'.') {
            i += 1;
            if format.get(i) == Some(&b'*') {
                i += 1;
                let p = va.word(e) as i32;
                precision = (p >= 0).then_some(p as usize); // negative: ignored
            } else {
                let mut p = 0usize;
                while let Some(d) = format.get(i).filter(|d| d.is_ascii_digit()) {
                    p = p.saturating_mul(10).saturating_add((d - b'0') as usize);
                    i += 1;
                }
                precision = Some(p);
            }
        }
        let mut size = Size::Default;
        loop {
            match format.get(i) {
                Some(b'h') => size = Size::Short,
                Some(b'l') => {
                    size = if format.get(i + 1) == Some(&b'l') {
                        i += 1;
                        Size::Quad
                    } else {
                        Size::Long
                    }
                }
                Some(b'w') => size = Size::Long,
                Some(b'L') | Some(b'F') | Some(b'N') => {}
                Some(b'I') => {
                    if format[i + 1..].starts_with(b"64") {
                        i += 2;
                        size = Size::Quad;
                    } else if format[i + 1..].starts_with(b"32") {
                        i += 2;
                    }
                }
                _ => break,
            }
            i += 1;
        }
        let Some(&conv) = format.get(i) else { break };
        i += 1;
        match conv {
            b'd' | b'i' => {
                let (v, neg) = match size {
                    Size::Quad => {
                        let v = va.dword(e) as i64;
                        (v.unsigned_abs(), v < 0)
                    }
                    Size::Short => {
                        let v = va.word(e) as u16 as i16 as i64;
                        (v.unsigned_abs(), v < 0)
                    }
                    _ => {
                        let v = va.word(e) as i32 as i64;
                        (v.unsigned_abs(), v < 0)
                    }
                };
                format_integer(&mut out, conv, v, neg, spec, width, precision);
            }
            b'u' | b'o' | b'x' | b'X' => {
                let v = match size {
                    Size::Quad => va.dword(e),
                    Size::Short => va.word(e) as u16 as u64,
                    _ => va.word(e) as u64,
                };
                format_integer(&mut out, conv, v, false, spec, width, precision);
            }
            b'p' => {
                let v = va.word(e) as u64;
                // MSVC: precision becomes 2 * sizeof(void *), upper-case hex.
                format_integer(&mut out, conv, v, false, spec, width, Some(8));
            }
            b'c' | b'C' => {
                let wide = conv == b'C' || size == Size::Long;
                let v = va.word(e);
                let byte = if wide {
                    ((v & 0xffff) <= 0xff).then_some(v as u8)
                } else {
                    Some(v as u8)
                };
                let body: Vec<u8> = byte.into_iter().collect();
                pad(&mut out, b"", &body, width, spec, true);
            }
            b's' | b'S' => {
                let wide = conv == b'S' || size == Size::Long;
                let p = va.word(e);
                let limit = precision.unwrap_or(usize::MAX);
                let body: Vec<u8> = if p == 0 {
                    b"(null)".iter().copied().take(limit).collect()
                } else if wide {
                    let mut text = Vec::new();
                    let mut at = p;
                    while text.len() < limit {
                        let ch = e.mem.u16(at);
                        if ch == 0 {
                            break;
                        }
                        at += 2;
                        text.push(ch);
                    }
                    text.iter()
                        .filter(|c| **c <= 0xff)
                        .map(|c| *c as u8)
                        .collect()
                } else {
                    let mut text = e.mem.cstr(p);
                    text.truncate(limit);
                    text
                };
                pad(&mut out, b"", &body, width, spec, true);
            }
            b'f' | b'e' | b'E' | b'g' | b'G' => {
                let v = f64::from_bits(va.dword(e));
                format_float(&mut out, conv, v, spec, width, precision);
            }
            b'n' => {
                let target = va.word(e);
                if size == Size::Short {
                    e.mem.set_u16(target, out.len() as u16);
                } else {
                    e.mem.set_u32(target, out.len() as u32);
                }
            }
            other => out.push(other),
        }
    }
    out
}

// Platform: replaces 00ec6cc4 (_vsprintf, LIBCMT)
/// `vsprintf(dest, format, va_list)`: formats into `dest` (no bound, as the
/// C function) and returns the number of characters written, without the
/// terminating zero. The game's callers: `ScriptError`, `AddNoteVarArgs`,
/// `IdvFormatString` (via `00407000`).
pub fn vsprintf(e: &mut Engine, dest: Ptr, format: Ptr, va_list: u32) -> i32 {
    if dest.is_null() || format.is_null() {
        invalid_parameter("vsprintf");
    }
    let text = e.mem.cstr(format.addr());
    let out = format_va(e, &text, va_list);
    e.mem.set_cstr(dest.addr(), &out);
    out.len() as i32
}

/// Copies variable-argument words to game memory and formats them there as
/// the `va_list` ([`format_va`]).
fn format_words(e: &mut Engine, format: &[u8], args: &[u32]) -> Vec<u8> {
    let size = 4 * (args.len() as u32).max(1);
    e.with_stack(size, |e, list| {
        for (i, word) in args.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *word);
        }
        format_va(e, format, list.addr())
    })
}

// Platform: replaces 00ec623a (sprintf, LIBCMT)
/// `sprintf(buffer, format, ...)` (cdecl, variable arguments): the 2008
/// runtime's string stream with room `0x7fffffff`, so the formatted text is
/// written with its terminating zero and its length returned. A null
/// `buffer` or `format` is an invalid parameter (MSVC sets `errno` to
/// `EINVAL` and calls the handler, which in a release build ends the
/// program). The formatting is [`format_va`]'s.
pub fn sprintf(e: &mut Engine, buffer: Ptr, format: Ptr, args: &[u32]) -> i32 {
    if format.is_null() || buffer.is_null() {
        invalid_parameter("sprintf");
    }
    let text = e.mem.cstr(format.addr());
    let out = format_words(e, &text, args);
    e.mem.set_cstr(buffer.addr(), &out);
    out.len() as i32
}

/// The registry entry of [`sprintf`]: buffer, format, then the variable
/// arguments.
fn sprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    sprintf(e, Ptr::new(words[0]), Ptr::new(words[1]), &words[2..]).into_ret()
}

/// `_snprintf`'s room: `count`, at most `INT_MAX` (`0x7fffffff`).
const SNPRINTF_MAX_COUNT: u32 = 0x7fff_ffff;

// Platform: replaces 00eca38d (_snprintf, LIBCMT)
/// `_snprintf(buffer, count, format, ...)` (cdecl, variable arguments) as
/// the 2008 runtime's string stream (`_IOWRT | _IOSTRG`, room `count` capped
/// at `0x7fffffff`) does it:
/// - the formatted text fits with room to spare: written with its
///   terminating zero, returns its length;
/// - it fills the room exactly: written without a terminating zero,
///   returns its length;
/// - it is longer: the first `count` characters are written, no
///   terminating zero, returns -1;
/// - a null `buffer` with `count` 0 only counts (`write_char` and
///   `write_string`, `00ed2ec8`/`00ed4638`, skip the store for a string
///   stream without a base): returns the length.
///
/// A null `format`, or a null `buffer` with a nonzero `count`, is an invalid
/// parameter (MSVC sets `errno` to `EINVAL` and calls the handler, which in a
/// release build ends the program). `errno` (`ERANGE` from `_flsbuf` on
/// overflow) is not modeled. The formatting is [`format_va`]'s, on the
/// argument words copied to game memory as the `va_list`.
pub fn snprintf(e: &mut Engine, buffer: Ptr, count: u32, format: Ptr, args: &[u32]) -> i32 {
    if format.is_null() || (count != 0 && buffer.is_null()) {
        invalid_parameter("_snprintf");
    }
    let text = e.mem.cstr(format.addr());
    let out = format_words(e, &text, args);
    if buffer.is_null() {
        return out.len() as i32;
    }
    let room = count.min(SNPRINTF_MAX_COUNT) as usize;
    if out.len() > room {
        e.mem.write(buffer.addr(), &out[..room]);
        return -1;
    }
    e.mem.write(buffer.addr(), &out);
    if out.len() < room {
        e.mem.set_u8(buffer.addr() + out.len() as u32, 0);
    }
    out.len() as i32
}

/// The registry entry of [`snprintf`]: buffer, count, format, then the
/// variable arguments.
fn snprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    snprintf(
        e,
        Ptr::new(words[0]),
        words[1],
        Ptr::new(words[2]),
        &words[3..],
    )
    .into_ret()
}

// ---- Program ---------------------------------------------------------------

// Platform: replaces 00ec658f (_atexit, LIBCMT) with its callees 00ec6553 (__onexit) and 00ec6468 (__onexit_nolock)
/// Registers a function to run at exit: appends it to the CRT's table of
/// exit functions (`ONEXIT_BEGIN`, `ONEXIT_END`), which [`doexit`] runs.
/// Answers 0, or -1 when the table could not take it (`(f != 0) - 1` of
/// `__onexit`'s answer, which is `f` when it was stored and 0 when not).
///
/// What the exe does and what differs here:
/// - `__onexit_nolock` keeps the pointers in a heap block of words
///   (`__encode_pointer`'d: the OS obfuscates pointers kept in writable
///   memory; `EncodePointer` is the identity here, so the words are plain
///   addresses and a null entry is 0). It grows the block by
///   `min(size, 0x800)` bytes when full, or by 0x10 if that fails.
/// - The table itself is made by the first C initializer, `00ec6522`
///   (32 zeroed words from `__calloc_crt`, begin = end = the block); it has
///   no Ghidra function, so the engine's start-up leaves it open and the
///   first `atexit` makes the table the same way instead.
/// - `_lock(8)` / `_unlock(8)` around it are no-ops: one thread.
pub fn atexit(e: &mut Engine, f: Ptr) -> i32 {
    let stored = onexit(e, f.addr()).unwrap_or(0);
    (stored != 0) as i32 - 1
}

/// The words at `01271a8c` and `01271a88`: the start and the end (one past the last
/// word) of the table of exit functions. The names (`__onexitbegin`,
/// `__onexitend` in the CRT's source) are not confirmed in Ghidra.
const ONEXIT_BEGIN: u32 = 0x0127_1a8c;
const ONEXIT_END: u32 = 0x0127_1a88;

/// `__onexit_nolock`'s work for `f`: its address when stored, `None` when
/// not (the exe answers 0).
fn onexit(e: &mut Engine, f: u32) -> Option<u32> {
    e.mem.map(ONEXIT_END, 8);
    if e.mem.u32(ONEXIT_BEGIN) == 0 {
        // 00ec6522: `__calloc_crt(32, 4)`; begin = end = the block; `AND
        // [block],0`. (The exe answers 24, _RT_SPACEINIT, when the heap
        // gives nothing; `Mem`'s heap stops with a panic instead.)
        let table = calloc_crt(e, 32, 4).addr();
        e.mem.set_u32(ONEXIT_BEGIN, table);
        e.mem.set_u32(ONEXIT_END, table);
        e.mem.set_u32(table, 0);
    }
    let begin = e.mem.u32(ONEXIT_BEGIN);
    let end = e.mem.u32(ONEXIT_END);
    // `JC`: the end below the start; then the byte count plus one word
    // wrapping around.
    if end < begin {
        return None;
    }
    let used = end - begin;
    if used.wrapping_add(4) < 4 {
        return None;
    }
    let size = e
        .mem
        .block_size(begin)
        .unwrap_or_else(|| panic!("the exit function table {begin:08x} is not a heap block"));
    let mut at = end;
    if size < used.wrapping_add(4) {
        let grow = size.min(0x800);
        let block = match size
            .checked_add(grow)
            .and_then(|n| onexit_realloc(e, begin, n))
        {
            Some(b) => b,
            None => size
                .checked_add(0x10)
                .and_then(|n| onexit_realloc(e, begin, n))?,
        };
        // `used >> 2` words (SAR) from the new start.
        at = block.wrapping_add((((used as i32) >> 2) as u32).wrapping_mul(4));
        e.mem.set_u32(ONEXIT_BEGIN, block);
    }
    e.mem.set_u32(at, f);
    e.mem.set_u32(ONEXIT_END, at.wrapping_add(4));
    Some(f)
}

/// `__realloc_crt` (`00ed0d70`) for the table: `Mem`'s heap, which does not
/// fail; the old block is freed and its bytes are copied.
fn onexit_realloc(e: &mut Engine, block: u32, size: u32) -> Option<u32> {
    let old = e.mem.block_size(block)?;
    let new = e.mem.alloc(size);
    let kept = e.mem.bytes(block, old.min(size));
    e.mem.write(new, &kept);
    e.mem.free(block);
    Some(new)
}

/// The `_initterm` ranges `doexit` runs, as the exe passes them (`PUSH end;
/// MOV EAX,start; CALL 00ecccd3`, at `00ecce65` and `00ecce75`): the
/// pre-terminators (`__xp_a`..`__xp_z` in the CRT's source) and the
/// terminators (`__xt_a`..`__xt_z`). The exe holds 3 pre-terminator slots:
/// `010113f4` empty, `010113f8` = `00ee628e` and `010113fc` = `00ecdffb`
/// (neither has a Ghidra function); and one terminator slot, empty.
pub const PRE_TERMINATORS: (u32, u32) = (0x0101_13f4, 0x0101_1400);
pub const TERMINATORS: (u32, u32) = (0x0101_1404, 0x0101_1408);

/// The words at `0126fde8` (set once the exit has finished: a second
/// `doexit` runs no handlers) and `0126fde4` (set when it starts), and the
/// byte at `0126fde0` (the caller's `retcaller` argument).
const EXIT_DONE: u32 = 0x0126_fde8;
const EXIT_STARTED: u32 = 0x0126_fde4;
const EXIT_RETURNS: u32 = 0x0126_fde0;

/// What a run of [`doexit`] did. The exe has no such report; it is how the
/// engine accounts for handlers it cannot run.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExitReport {
    /// The handlers run, in order: exit functions, pre-terminators,
    /// terminators.
    pub ran: Vec<u32>,
    /// Handlers skipped because nobody has translated them.
    pub open: Vec<u32>,
    /// Handlers that stopped part-way in a function nobody has translated
    /// (handler, the stop); what they did before stays.
    pub stopped: Vec<(u32, String)>,
    /// `Some(code)` when the exe now ends the process with `___crtExitProcess`
    /// (`00eccca9`, `ExitProcess`): not run here, the host decides.
    pub exit_process: Option<i32>,
}

/// Runs one handler (`CALL f`, no arguments), or skips and counts it if it
/// isn't translated.
fn exit_handler(e: &mut Engine, f: u32, report: &mut ExitReport) {
    if !e.is_translated(f) {
        report.open.push(f);
        return;
    }
    match crate::static_init::guarded(e, |e| e.call(f, &[])) {
        Ok(_) => report.ran.push(f),
        Err(stop) => report.stopped.push((f, stop)),
    }
}

/// `_initterm` (`00ecccd3`): every non-null word of `start..end`, in order,
/// is a function called without arguments.
fn run_exit_table(e: &mut Engine, (start, end): (u32, u32), report: &mut ExitReport) {
    e.mem.map(start, end - start);
    let mut slot = start;
    while slot < end {
        let f = e.mem.u32(slot);
        if f != 0 {
            exit_handler(e, f, report);
        }
        slot = slot.wrapping_add(4);
    }
}

/// The part of `doexit` that runs the exit functions: from the last
/// registered to the first, each cleared before it runs, and the walk
/// restarted from the new end if a handler changed the table.
fn run_onexit(e: &mut Engine, report: &mut ExitReport) {
    e.mem.map(ONEXIT_END, 8);
    let mut begin = e.mem.u32(ONEXIT_BEGIN);
    if begin == 0 {
        return;
    }
    // The table as the walk last saw it; its start is the walk's lower bound.
    let mut end = e.mem.u32(ONEXIT_END);
    let mut at = end;
    loop {
        at = at.wrapping_sub(4);
        if at < begin {
            break;
        }
        if e.mem.u32(at) == 0 {
            continue;
        }
        // The exe compares `at` with the start a second time here (`JC`);
        // it cannot differ from the test above.
        if at < begin {
            break;
        }
        let f = e.mem.u32(at);
        e.mem.set_u32(at, 0);
        exit_handler(e, f, report);
        let (b, n) = (e.mem.u32(ONEXIT_BEGIN), e.mem.u32(ONEXIT_END));
        if begin != b || end != n {
            (begin, end, at) = (b, n, n);
        }
    }
}

// Translation incomplete: 00eccd99 - the SEH frame (__SEH_prolog4) is not modelled; with `retcaller` 0 the exe ends in ___crtExitProcess (00eccca9, ExitProcess), reported in `exit_process` and not run; _lock(8) and _unlock(8) (00eddf39, 00edde5f) are no-ops (one thread).
// Translated from 00eccd99 (decompiled, FalloutNV.exe 1.4.0.525)
/// `doexit(code, quick, retcaller)`, the CRT's exit path (Ghidra: `doexit`;
/// called by `00eccec5` as `(code, 0, 0)`, by `00eccedb` as `(code, 1, 0)`
/// and by `00eccef1` as `(0, 0, 1)`; Ghidra labels those `_exit`, `__exit`
/// and `__cexit`). Unless an earlier call finished the exit (`0126fde8`
/// is 1), it sets `0126fde4` and `0126fde0` (the low byte of `retcaller`),
/// and, when `quick` is 0, runs
/// 1. the exit functions registered with [`atexit`], from the last to the
///    first (each cleared before it runs),
/// 2. the pre-terminators ([`PRE_TERMINATORS`]),
///
/// and always then 3. the terminators ([`TERMINATORS`]). With `retcaller` 0
/// it then sets `0126fde8` and ends the process; that last step is only
/// reported (`exit_process`).
///
/// A handler nobody has translated is skipped and counted in the report; one
/// that stops part-way is recorded with its stop. The registered form (the
/// address `00eccd99`, three words) discards the report.
pub fn doexit(e: &mut Engine, code: i32, quick: i32, retcaller: i32) -> ExitReport {
    let mut report = ExitReport::default();
    e.mem.map(EXIT_RETURNS, 0x10);
    // __lock(8): one thread.
    if e.mem.u32(EXIT_DONE) != 1 {
        e.mem.set_u32(EXIT_STARTED, 1);
        e.mem.set_u8(EXIT_RETURNS, retcaller as u8);
        if quick == 0 {
            run_onexit(e, &mut report);
            run_exit_table(e, PRE_TERMINATORS, &mut report);
        }
        run_exit_table(e, TERMINATORS, &mut report);
    }
    // 00ecceb0: _unlock(8) when `retcaller` is not 0 (no-op).
    if retcaller == 0 {
        e.mem.set_u32(EXIT_DONE, 1);
        // _unlock(8); then ___crtExitProcess(code): not run.
        report.exit_process = Some(code);
    }
    report
}

/// The registered form of [`doexit`]: `(code, quick, retcaller)`.
fn doexit_entry(e: &mut Engine, words: &[u32]) -> Ret {
    doexit(e, words[0] as i32, words[1] as i32, words[2] as i32);
    Ret::default()
}

// Platform: replaces 00ec7c56 (the invalid-parameter path, LIBCMT)
pub fn invalid_parameter_noinfo(_e: &mut Engine) {
    invalid_parameter("noinfo")
}

// ---- RTTI ------------------------------------------------------------------

/// Whether two `TypeDescriptor`s are the same type: the same address or
/// the same decorated name (MSVC compares names after the address).
fn same_type(e: &Engine, a: u32, b: u32) -> bool {
    a == b || (a != 0 && b != 0 && e.mem.cstr(a + 8) == e.mem.cstr(b + 8))
}

// Platform: replaces 00ec43fb (__RTDynamicCast, LIBCMT)
/// `dynamic_cast` through the game's own RTTI in mapped memory: from the
/// object's vtable to its complete object locator, then the class
/// hierarchy's base class array. A target that is a base (or the complete
/// type) gives the address of that subobject; otherwise null, or a panic
/// for a reference cast (`std::bad_cast`). For a class with multiple
/// inheritance MSVC also checks that the source and target subobjects are
/// unambiguous and public; here the first visible match is taken, which is
/// the same result for every cast that MSVC allows.
pub fn rt_dynamic_cast(
    e: &mut Engine,
    inptr: Ptr,
    _vf_delta: i32,
    _src_type: Ptr,
    target_type: Ptr,
    is_reference: i32,
) -> Ptr {
    if inptr.is_null() {
        return Ptr::NULL;
    }
    let vft = e.mem.u32(inptr.addr());
    let col = e.mem.u32(vft.wrapping_sub(4));
    let offset = e.mem.u32(col + 4);
    let cd_offset = e.mem.u32(col + 8);
    let mut complete = inptr.addr().wrapping_sub(offset);
    if cd_offset != 0 {
        complete = complete.wrapping_sub(e.mem.u32(inptr.addr().wrapping_sub(cd_offset)));
    }
    let chd = e.mem.u32(col + 16);
    let count = e.mem.u32(chd + 8);
    let bases = e.mem.u32(chd + 12);
    for i in 0..count {
        let bcd = e.mem.u32(bases + 4 * i);
        let td = e.mem.u32(bcd);
        let attributes = e.mem.u32(bcd + 20);
        if attributes & 1 != 0 || !same_type(e, td, target_type.addr()) {
            continue; // BCD_NOTVISIBLE, or another type
        }
        let mdisp = e.mem.i32(bcd + 8);
        let pdisp = e.mem.i32(bcd + 12);
        let vdisp = e.mem.i32(bcd + 16);
        let mut off = mdisp;
        if pdisp >= 0 {
            let vbtable = e.mem.u32(complete.wrapping_add(pdisp as u32));
            off += pdisp + e.mem.i32(vbtable.wrapping_add(vdisp as u32));
        }
        return Ptr::new(complete.wrapping_add(off as u32));
    }
    if is_reference != 0 {
        panic!("std::bad_cast from {:08x}", inptr.addr());
    }
    Ptr::NULL
}

// ---- The physics step's remaining runtime functions ----------------------------------------
//
// The x87 helpers below take and give what the exe passes in registers and flags: the uniform
// form lists register inputs first, in the order EAX, ECX, EDX, then `ST0` (two words), then the
// stack words from the one at `[ESP]` on; a flag input (`ZF`) is a `bool` word before them. The
// engine computes in `f64`, so `ST0` is an `f64`, and the control word is the one `ctrlfp` keeps.

/// `__use_sse2_mathfcns` (`01270a64`): nonzero when the CRT's SSE2 math routines may be used.
const USE_SSE2_MATHFCNS: u32 = 0x0127_0a64;
/// The CRT's math-error switch (`0126fd90`): nonzero skips the error handling and returns the
/// result as it is.
const MATH_ERROR_SWITCH: u32 = 0x0126_fd90;
/// `MXCSR` as the game runs with it (every exception masked, `0x1f80`). The SSE2 routines test
/// `STMXCSR`; the engine does not track the register.
const MXCSR: u32 = 0x1f80;

/// `FLD m64`: a double loaded onto the x87 stack. A signalling NaN becomes quiet (the quiet bit
/// is set, the payload kept); every other value is exact. There is no shared helper for
/// doubles (`x87_quiet` is for floats).
fn fld64(bits: u64) -> f64 {
    let exponent = 0x7ff0_0000_0000_0000u64;
    if bits & exponent == exponent && bits & 0x000f_ffff_ffff_ffff != 0 {
        f64::from_bits(bits | 0x0008_0000_0000_0000)
    } else {
        f64::from_bits(bits)
    }
}

/// `FLD tbyte` then `FST double` of an extended NaN or infinity at `address` (the CRT's default
/// "indefinite" NaN at `011c1d00` is one): the sign is kept, the fraction is the extended
/// significand without its integer bit, shifted to 52 bits.
fn fld_extended_special(e: &Engine, address: u32) -> f64 {
    let significand = e.mem.u64(address);
    let sign_exponent = e.mem.u16(address + 8);
    assert!(
        sign_exponent & 0x7fff == 0x7fff,
        "{address:08x} is not an extended NaN or infinity"
    );
    let sign = u64::from(sign_exponent >> 15) << 63;
    f64::from_bits(sign | 0x7ff0_0000_0000_0000 | ((significand << 1) >> 12))
}

/// Whether the SSE2 math routines run: `__use_sse2_mathfcns` is set and `MXCSR` and the x87
/// control word have their exception masks set (`MXCSR & 0x1f80 == 0x1f80`, control word
/// `& 0x7f == 0x7f`), the test every `_CIxxx` entry makes before it jumps to its SSE2 body.
fn sse2_math_available(e: &mut Engine) -> bool {
    e.global::<u32>(USE_SSE2_MATHFCNS) != 0
        && MXCSR & 0x1f80 == 0x1f80
        && x87_control_word(e) & 0x7f == 0x7f
}

/// `__sptype` (`00ed5981`, cdecl, the double on the stack) and its siblings `ceil`'s x87 body
/// calls: `__handle_qnan1` (`00ed5837`: function code, the double, the control word),
/// `__except1` (`00ed588c`: operation, function code, argument, result, control word),
/// `__frnd` (`00ee23bc`: the double, result in `ST0`) and `___libm_error_support` (`00ed429f`:
/// three pointers and a code).
const SPTYPE: u32 = 0x00ed_5981;
const HANDLE_QNAN1: u32 = 0x00ed_5837;
const EXCEPT1: u32 = 0x00ed_588c;
const FRND: u32 = 0x00ee_23bc;
const LIBM_ERROR_SUPPORT: u32 = 0x00ed_429f;
/// The control word `ceil`'s x87 body loads (`011c278c`, round up) and the constant it adds to
/// an infinity or NaN for the error report (`01012070`, 1.0).
const CEIL_CONTROL_WORD: u32 = 0x011c_278c;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The SSE2 constants of `ceil`: the shift base (`010f2000`, `0x433` per qword), the 1.0 mask
/// (`010f1ff0`), zero (`010f2010`), the exponent mask (`010f2020`, `0x7ff`) and minus zero
/// (`010f2030`).
const CEIL_SHIFT_BASE: u32 = 0x010f_2000;
const CEIL_ONE_MASK: u32 = 0x010f_1ff0;
const CEIL_ZERO: u32 = 0x010f_2010;
const CEIL_EXPONENT_MASK: u32 = 0x010f_2020;
const CEIL_MINUS_ZERO: u32 = 0x010f_2030;

// Translated from 00ec9e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ceil` (LIBCMT; no label in the map: named from the body, which rounds toward +infinity, and
/// from `011c278c`, the round-up control word its x87 path loads), cdecl, the double on the stack,
/// the result in `ST0`. With the SSE2 routines available (`sse2_math_available`) it computes the
/// result with integer shifts on the bit pattern: the shift count is `0x433 - exponent`, a
/// positive number below 1 gives 1.0 (zero stays +0), a negative one below 1 gives -0.0, up to
/// the exponent `0x432` the fraction bits are cleared (and 1.0 is added when that lowered a
/// positive value), and beyond that the value is returned as it is, a NaN after
/// `___libm_error_support(&x, &x, &x, 0x3ec)` has had its say about the result slot. Without them
/// it loads the round-up control word through `ctrlfp` (keeping the old one). A finite value is
/// rounded by `__frnd`; the old control word is put back and the rounded value returned when it
/// equals the argument or the old word has bit 5 (the precision-exception mask) set, otherwise
/// the inexact result is reported through `__except1` (operation `0x10`). An infinity or NaN goes
/// through `__sptype`: classes 1 and 2 put the old control word back and return the value,
/// class 3 is `__handle_qnan1`, anything else `__except1` operation 8 with the value plus 1.0.
pub fn ceil(e: &mut Engine, x: f64) -> f64 {
    if sse2_math_available(e) {
        ceil_sse2(e, x)
    } else {
        ceil_x87(e, x)
    }
}

/// The SSE2 body of `ceil` (`00ec9e50`).
fn ceil_sse2(e: &mut Engine, x: f64) -> f64 {
    let bits = x.to_bits();
    let shifted = bits >> 52; // PSRLQ XMM0, 0x34
    let sign_and_exponent = shifted as u32; // MOVD EAX, XMM0
    let exponent_mask = e.mem.u64(CEIL_EXPONENT_MASK);
    let exponent = shifted & exponent_mask; // ANDPD XMM0, [010f2020]
    let base = e.mem.u64(CEIL_SHIFT_BASE);
    // PSUBD: the two dwords of the low qword subtract separately; the qword is the shift count.
    let count_low = (base as u32).wrapping_sub(exponent as u32);
    let count_high = ((base >> 32) as u32).wrapping_sub((exponent >> 32) as u32);
    let count = u64::from(count_low) | (u64::from(count_high) << 32);
    // PSRLQ / PSLLQ by a count above 63 give zero.
    let shift_right = |value: u64| if count > 63 { 0 } else { value >> count };
    let shift_left = |value: u64| if count > 63 { 0 } else { value << count };
    let one_mask = e.mem.u64(CEIL_ONE_MASK);
    if sign_and_exponent & 0x800 == 0 {
        // 00ec9ed2: a positive value.
        let truncated = f64::from_bits(shift_left(shift_right(bits)));
        let not_le = !(x <= truncated); // CMPNLEPD XMM0, XMM1
        if (sign_and_exponent as i32) < 0x3ff {
            // 00ec9f11
            let zero = f64::from_bits(e.mem.u64(CEIL_ZERO));
            let mask = if !(x <= zero) { u64::MAX } else { 0 };
            return fld64(mask & one_mask);
        }
        if (sign_and_exponent as i32) > 0x432 {
            return ceil_out_of_range(e, x);
        }
        let addend = if not_le { one_mask } else { 0 };
        return fld64((truncated + f64::from_bits(addend)).to_bits()); // ADDSD XMM1, XMM0
    }
    if (sign_and_exponent as i32) < 0xbff {
        return fld64(e.mem.u64(CEIL_MINUS_ZERO)); // 00ec9f0a
    }
    let truncated = shift_left(shift_right(bits));
    if (sign_and_exponent as i32) > 0xc32 {
        return ceil_out_of_range(e, x);
    }
    fld64(truncated)
}

/// `ceil`'s exit for a value whose exponent is at least `0x433` (`00ec9ea3`): a number comes back
/// as it is; a NaN is first given to `___libm_error_support`, which may rewrite the slot the
/// value sits in (all three pointers are that slot), and the slot is the result.
fn ceil_out_of_range(e: &mut Engine, x: f64) -> f64 {
    if !x.is_nan() {
        return fld64(x.to_bits());
    }
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        e.call(LIBM_ERROR_SUPPORT, &args![slot, slot, slot, 0x3ecu32]);
        fld64(e.mem.u64(slot.addr()))
    })
}

/// The x87 body of `ceil` (`00ed969d`).
fn ceil_x87(e: &mut Engine, x: f64) -> f64 {
    let new_control = e.mem.u32(CEIL_CONTROL_WORD);
    let old_control = ctrlfp(e, new_control, 0xffff) as u32; // EBX
    let bits = x.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    // The exponent word of the double, at `[EBP + 0xe]` (the bytes above it are masked away).
    let exponent_word = ((bits >> 48) as u32) & 0x7ff0;
    let quiet = fld64(bits).to_bits();
    let (quiet_low, quiet_high) = (quiet as u32, (quiet >> 32) as u32);
    if exponent_word == 0x7ff0 {
        let class = e.call(SPTYPE, &args![low, high]).i32();
        if class > 0 {
            if class <= 2 {
                // 00ed96fc
                ctrlfp(e, old_control, 0xffff);
                return fld64(bits);
            }
            if class == 3 {
                return e
                    .call(
                        HANDLE_QNAN1,
                        &args![0xcu32, quiet_low, quiet_high, old_control],
                    )
                    .f64();
            }
        }
        // 00ed970a: the value plus 1.0, reported as an error of operation 8.
        let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
        let result = x87_add(fld64(bits), one).to_bits();
        return e
            .call(
                EXCEPT1,
                &args![
                    8u32,
                    0xcu32,
                    quiet_low,
                    quiet_high,
                    result as u32,
                    (result >> 32) as u32,
                    old_control
                ],
            )
            .f64();
    }
    // 00ed9727
    let rounded = e.call(FRND, &args![low, high]).f64();
    let (rounded_low, rounded_high) = (rounded.to_bits() as u32, (rounded.to_bits() >> 32) as u32);
    // FCOMP and the `TEST AH, 0x44` / `JP`: equal only when ordered and equal.
    if fld64(bits) == rounded || old_control & 0x20 != 0 {
        ctrlfp(e, old_control, 0xffff);
        return rounded;
    }
    e.call(
        EXCEPT1,
        &args![
            0x10u32,
            0xcu32,
            quiet_low,
            quiet_high,
            rounded_low,
            rounded_high,
            old_control
        ],
    )
    .f64()
}

/// `_CIasin`'s SSE2 body (`00ed7720`, `ST0`) and its x87 helpers: `00ed2808` (stack double in,
/// `EAX` = the exponent bits, or the whole high word for an infinity or NaN; `ZF` set exactly in
/// that case) and the x87 body `00ec96a8` (`ZF`, `EAX`, `ST0`, the stack double).
const ASIN_SSE2: u32 = 0x00ed_7720;
const LOAD_FLAGS_HELPER: u32 = 0x00ed_2808;
const ASIN_X87: u32 = 0x00ec_96a8;

// Translated from 00ec9650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_CIasin` (no label in the map: named from the body, the x87 arcsine at `00ec96a8` with
/// `FPATAN` of `x / sqrt((1 + x)(1 - x))` and the error string `"asin"` at `011c1930`; called by
/// the Euler-angle conversions): `ST0` in, `ST0` out. With the SSE2 routines available
/// (`sse2_math_available`) it jumps to the SSE2 body `00ed7720`. Otherwise it stores the double
/// on the stack, runs `00ed2808` (which leaves `ZF` set when the exponent is all ones), and runs
/// the x87 body `00ec96a8` on it, returning its `ST0`.
pub fn ci_asin(e: &mut Engine, st0: f64) -> f64 {
    if sse2_math_available(e) {
        return e.call(ASIN_SSE2, &args![st0]).f64();
    }
    let bits = st0.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let eax = e.call(LOAD_FLAGS_HELPER, &args![low, high]).u32();
    // `ZF` is the result of `00ed2808`'s `CMP EAX, 0x7ff00000`.
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    e.call(ASIN_X87, &args![zf, eax, st0, low, high]).f64()
}

/// The name `sqrt`'s error report is given (`"sqrt"`, `011c1740`).
const SQRT_NAME: u32 = 0x011c_1740;
/// The x87 square root's helpers: `00ed2795` (loads the control word `(old & 0x300) | 0x7f`; the
/// saved control word is its stack word), `00ed27ac` (`EAX` = high word, `ST0`: `EAX` 7 for a
/// quiet NaN, else 1 with `ST0 + 1.0`), `00ed282b` (the exit: `EDX` = operation, `ECX` = name,
/// `ST0`, the saved control word, the caller's return address, the double) and `00ed2737` (the
/// error handler: `EAX` = type, `ECX` = name, `EDX` = operation, `ST0`, the saved control word,
/// the caller's return address, the double).
const SET_CONTROL_WORD_HELPER: u32 = 0x00ed_2795;
const NAN_TYPE_HELPER: u32 = 0x00ed_27ac;
const MATH_EXIT: u32 = 0x00ed_282b;
const ONE_ARG_ERROR_HANDLING: u32 = 0x00ed_2737;
/// The indefinite NaN the CRT returns for a domain error (`011c1d00`, extended).
const INDEFINITE_NAN: u32 = 0x011c_1d00;

/// `00ec607e`: the end of the square-root paths that have a result. With the error switch set the
/// control word is restored (`00ed281e`: loaded back unless it is `0x027f`) and `ST0` returned;
/// otherwise the exit `00ed282b` runs with operation 5 and the name `"sqrt"`.
fn sqrt_exit(e: &mut Engine, saved_control: u16, st0: f64, low: u32, high: u32) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        if saved_control != 0x027f {
            set_x87_control_word(e, saved_control);
        }
        return st0;
    }
    e.call(
        MATH_EXIT,
        &args![
            5u32,
            SQRT_NAME,
            st0,
            u32::from(saved_control),
            0u32,
            low,
            high
        ],
    )
    .f64()
}

/// `00ec60db`: the end of the error paths, with the error type in `EAX`. With the error switch
/// set it restores the control word and returns `ST0`; otherwise `00ed2737` reports the error
/// (operation 5, name `"sqrt"`) and returns the result `ST0`.
fn sqrt_error(
    e: &mut Engine,
    saved_control: u16,
    st0: f64,
    error_type: u32,
    low: u32,
    high: u32,
) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        if saved_control != 0x027f {
            set_x87_control_word(e, saved_control);
        }
        return st0;
    }
    e.call(
        ONE_ARG_ERROR_HANDLING,
        &args![
            error_type,
            SQRT_NAME,
            5u32,
            st0,
            u32::from(saved_control),
            0u32,
            low,
            high
        ],
    )
    .f64()
}

// Translated from 00ec605d (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec605d - FSQRT's x87 status flags (read later by the exit 00ed282b through FSTSW) are not modelled
/// The shared x87 body of `_CIsqrt` (`00ec6040`, which stores `ST0` to the stack and calls
/// `00ed2808` first) and `sqrt` (`00ec6054`, which loads the stack double with `__fload_withFB`
/// and falls into this code); no label in the map, named from those two callers. Its inputs are
/// `ZF` (set exactly when the double's exponent is all ones), `ST0` and the double on the stack
/// (`low`, `high`), returning `ST0`. It saves the control word (`FSTCW`). A finite argument:
/// with a control word other than `0x027f` it first loads `(cw & 0x300) | 0x7f` (`00ed2795`);
/// a non-negative one takes `FSQRT` and ends in the exit (`sqrt_exit`); `-0.0` skips the `FSQRT`
/// and ends there with `-0.0`; any other negative one (a negative denormal included) is a domain
/// error: `ST0` becomes the indefinite NaN (`011c1d00`) with error type 1. An infinity or NaN
/// (`ZF` set): a NaN goes to `00ed27ac` for the error type and the new `ST0`, a positive infinity
/// ends in the exit unchanged, a negative one is the domain error. Error paths end in
/// `sqrt_error`.| 0x7f`
/// (`00ed2795`); a positive or zero-exponent-and-mantissa (`-0.0`) argument takes `FSQRT` (the
/// -0.0 skips it, keeping -0.0) and ends in the exit (`sqrt_exit`); any other negative argument is
/// a domain error: `ST0` becomes the indefinite NaN (`011c1d00`) with error type 1. An infinity
/// or NaN: a NaN goes to `00ed27ac` for the error type (and `ST0`), a positive infinity ends in
/// the exit unchanged, a negative one is the domain error. Error paths end in `sqrt_error`.
pub fn fn_00ec605d(e: &mut Engine, zf: bool, st0: f64, low: u32, high: u32) -> f64 {
    let saved_control = x87_control_word(e);
    if zf {
        // 00ec60b9
        if high & 0xfffff != 0 || low != 0 {
            let result = e.call(NAN_TYPE_HELPER, &args![high, st0]);
            return sqrt_error(e, saved_control, result.st0, result.eax, low, high);
        }
        if high & 0x8000_0000 == 0 {
            return sqrt_exit(e, saved_control, st0, low, high);
        }
    } else {
        if saved_control != 0x027f {
            e.call(SET_CONTROL_WORD_HELPER, &args![u32::from(saved_control)]);
        }
        if high & 0x8000_0000 == 0 {
            // FSQRT
            return sqrt_exit(e, saved_control, st0.sqrt(), low, high);
        }
        // 00ec609b: a negative argument; only -0.0 has no error.
        if high & 0x7ff0_0000 == 0 && high & 0xfffff == 0 && low == 0 {
            return sqrt_exit(e, saved_control, st0, low, high);
        }
    }
    // 00ec60ce: FSTP ST0, FLD the indefinite NaN, EAX = 1
    let indefinite = fld_extended_special(e, INDEFINITE_NAN);
    sqrt_error(e, saved_control, indefinite, 1, low, high)
}

/// `__getptd_noexit` (`00ed1165`): the calling thread's CRT data block, or 0 when it has none.
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
/// The errno slot the CRT uses when a thread has no data block (`011c18c0`).
const ERRNO_FALLBACK: u32 = 0x011c_18c0;

// Translated from 00ec85e3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_errno` (LIBCMT/dosmap.c; Ghidra: `__errno`), cdecl: the address of the thread's `errno`:
/// the `_terrno` field at `+8` of the thread's CRT data block (`__getptd_noexit`, `00ed1165`),
/// or the static slot `011c18c0` when the thread has no block. `__getptd_noexit` is
/// translated ([`getptd_noexit`]); it finds the block through the TLS/FLS slots that `__mtinit`
/// ([`mtinit`]) sets up, so the block exists once the host has run `__mtinit`.
pub fn errno(e: &mut Engine) -> Ptr {
    let block = e.call(GETPTD_NOEXIT, &args![]).u32();
    if block == 0 {
        Ptr::new(ERRNO_FALLBACK)
    } else {
        Ptr::new(block.wrapping_add(8))
    }
}

/// `__decode_pointer` (`00ed0f92`, cdecl, one word) and the encoded handler `_set_invalid_parameter_handler`
/// stores (`0126fd9c`).
const DECODE_POINTER: u32 = 0x00ed_0f92;
const INVALID_PARAMETER_HANDLER: u32 = 0x0126_fd9c;
/// `00ed5bae` (its body is `AND [01270a60], 0`, whatever argument it is given; the CRT pushes
/// 1 or 2 here) and `__invoke_watson` (`00ec7b08`).
const CLEAR_01270A60: u32 = 0x00ed_5bae;
const INVOKE_WATSON: u32 = 0x00ec_7b08;

// Translated from 00ec7c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra labels the body `__invalid_parameter` (function ID); the engine map names it
/// `_invalid_parameter_noinfo` (LIBCMT/invarg.c), but it takes the five arguments of
/// `_invalid_parameter` and the no-argument form is the caller at `00ec7c56`, so it is named
/// by address. Cdecl, five words that it only passes on: it decodes the handler stored by
/// `_set_invalid_parameter_handler` (`__decode_pointer(0126fd9c)`) and, if there is one, jumps to
/// it with the same arguments; without one it calls `00ed5bae(2)` and jumps to `__invoke_watson`
/// (`00ec7b08`) with the same arguments, which ends the process.
pub fn fn_00ec7c30(
    e: &mut Engine,
    expression: u32,
    function: u32,
    file: u32,
    line: u32,
    reserved: u32,
) {
    let encoded = e.global::<u32>(INVALID_PARAMETER_HANDLER);
    let handler = e.call(DECODE_POINTER, &args![encoded]).u32();
    if handler != 0 {
        e.call(handler, &args![expression, function, file, line, reserved]);
        return;
    }
    e.call(CLEAR_01270A60, &args![2u32]);
    e.call(
        INVOKE_WATSON,
        &args![expression, function, file, line, reserved],
    );
}

/// `__vsnprintf_l` (`00ecdd3e`, cdecl: destination, count, format, locale, argument list).
const VSNPRINTF_L: u32 = 0x00ec_dd3e;

// Translated from 00ecddf1 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_vsnprintf` (LIBCMT/vsnprint.c), cdecl: `_vsnprintf_l` with a null locale. The words go on in
/// the order destination, count, format, locale (0), argument list, and the result comes back.
pub fn vsnprintf(e: &mut Engine, dest: Ptr, count: u32, format: Ptr, va_list: u32) -> i32 {
    e.call(VSNPRINTF_L, &args![dest, count, format, 0u32, va_list])
        .i32()
}

// Translated from 00ecc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__aulldiv` (Ghidra's function-ID label), stdcall (`RET 0x10`): the 64-bit unsigned quotient
/// `dividend / divisor` in `EDX:EAX`, the dividend and the divisor each as a low and a high word.
/// A divisor below 2^32 divides the high word first (`DIV`) and then the remainder with the low
/// word. Otherwise both operands are shifted right until the divisor's high word is zero, the
/// quotient estimate is `DIV` of the shifted values, and it is corrected by one: the divisor times
/// the estimate is formed from two `MUL`s, and the estimate is decremented when that product
/// carries or is greater than the dividend. A zero divisor stops with the `DIV` fault (#DE).
pub fn aulldiv(
    _e: &mut Engine,
    dividend_low: u32,
    dividend_high: u32,
    divisor_low: u32,
    divisor_high: u32,
) -> u64 {
    if divisor_high == 0 {
        assert!(divisor_low != 0, "00ecc590 __aulldiv: divide by zero (#DE)");
        // DIV ECX on 0:high, then on remainder:low.
        let high_quotient = dividend_high / divisor_low;
        let remainder = dividend_high % divisor_low;
        let low_quotient =
            ((u64::from(remainder) << 32) | u64::from(dividend_low)) / u64::from(divisor_low);
        return (u64::from(high_quotient) << 32) | (low_quotient & 0xffff_ffff);
    }
    let (mut shifted_divisor_high, mut shifted_divisor_low) = (divisor_high, divisor_low);
    let (mut shifted_high, mut shifted_low) = (dividend_high, dividend_low);
    loop {
        // SHR ECX, 1 / RCR EBX, 1 and SHR EDX, 1 / RCR EAX, 1
        let carry = shifted_divisor_high & 1;
        shifted_divisor_high >>= 1;
        shifted_divisor_low = (shifted_divisor_low >> 1) | (carry << 31);
        let carry = shifted_high & 1;
        shifted_high >>= 1;
        shifted_low = (shifted_low >> 1) | (carry << 31);
        if shifted_divisor_high == 0 {
            break;
        }
    }
    let estimate =
        ((u64::from(shifted_high) << 32) | u64::from(shifted_low)) / u64::from(shifted_divisor_low);
    assert!(
        estimate <= u64::from(u32::MAX),
        "00ecc590 __aulldiv: DIV overflow (#DE)"
    );
    let mut quotient = estimate as u32; // ESI
                                        // MUL [divisor high] (the low word of the product in ECX), then MUL ESI by the divisor low.
    let high_product = (u64::from(divisor_high) * u64::from(quotient)) as u32;
    let low_product = u64::from(divisor_low) * u64::from(quotient);
    let (sum, carry) = ((low_product >> 32) as u32).overflowing_add(high_product);
    let low_word = low_product as u32;
    // JC, then JA / JC / JBE on the comparisons with the dividend's words.
    let too_big = carry || sum > dividend_high || (sum == dividend_high && low_word > dividend_low);
    if too_big {
        quotient = quotient.wrapping_sub(1);
    }
    u64::from(quotient)
}

/// `___crtCorExitProcess` (`00eccc7e`, cdecl, the exit code) and the import slot of KERNEL32
/// `ExitProcess` (`00fdf0ec`, stdcall).
const CRT_COR_EXIT_PROCESS: u32 = 0x00ec_cc7e;
const EXIT_PROCESS: u32 = 0x00fd_f0ec;

// Translated from 00eccca9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___crtExitProcess` (Ghidra's function-ID label), cdecl, the exit code: gives it to
/// `___crtCorExitProcess` (`00eccc7e`, which tells the .NET runtime if `mscoree.dll` is loaded)
/// and then to `ExitProcess`. `ExitProcess` does not return, so the engine stops with a panic
/// naming this address and the exit code.
pub fn crt_exit_process(e: &mut Engine, code: i32) {
    e.call(CRT_COR_EXIT_PROCESS, &args![code]);
    e.call(EXIT_PROCESS, &args![code]);
    panic!("00eccca9 ___crtExitProcess: ExitProcess({code}) ended the process");
}

// Translated from 00ecc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strncpy` (LIBCMT/strncpy.c), cdecl: copies up to `count` bytes of the string at `source` to
/// `dest`, padding with zero bytes up to `count` once the terminator is copied, and returns
/// `dest`. It is the exe's word-at-a-time routine, run label by label: bytes until `source` is
/// 4-aligned, then 4-byte words (a word holding a zero byte is detected with the
/// `(w ^ 0xffffffff) ^ (w + 0x7efefeff)` mask `0x81010100` and stored cut at the zero, the rest
/// zero-filled by words), then the last `count % 4` bytes. It reads whole aligned words, so it
/// may read up to three bytes past the terminator, as the exe does.
pub fn strncpy(e: &mut Engine, dest: Ptr, source: Ptr, count: u32) -> Ptr {
    /// The labels of the routine, by address.
    #[derive(Clone, Copy)]
    enum At {
        L7dc,
        L7fe,
        L803,
        L81e,
        L826,
        L83c,
        L843,
        L848,
        L855,
        L85f,
        L8ad,
        L8b7,
        L8b9,
        L8c3,
    }
    if count == 0 {
        return dest; // 00ecc7b7: JZ 00ecc84f
    }
    let mut ecx = count;
    let mut ebx = count;
    let mut esi = source.addr();
    let mut edi = dest.addr();
    let mut eax = 0u32;
    let mut edx = 0u32;
    let mut at = if esi & 3 != 0 {
        At::L7dc
    } else {
        ecx >>= 2;
        if ecx != 0 {
            At::L85f
        } else {
            At::L803
        }
    };
    loop {
        match at {
            At::L7dc => {
                eax = (eax & !0xff) | u32::from(e.mem.u8(esi));
                esi = esi.wrapping_add(1);
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                ecx = ecx.wrapping_sub(1);
                if ecx == 0 {
                    return dest;
                }
                if eax as u8 == 0 {
                    at = At::L81e;
                } else if esi & 3 != 0 {
                    at = At::L7dc;
                } else {
                    ebx = ecx;
                    ecx >>= 2;
                    at = if ecx != 0 { At::L85f } else { At::L7fe };
                }
            }
            At::L7fe => {
                ebx &= 3;
                if ebx == 0 {
                    return dest;
                }
                at = At::L803;
            }
            At::L803 => {
                eax = (eax & !0xff) | u32::from(e.mem.u8(esi));
                esi = esi.wrapping_add(1);
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                if eax as u8 == 0 {
                    at = At::L848;
                } else {
                    ebx = ebx.wrapping_sub(1);
                    if ebx == 0 {
                        return dest;
                    }
                }
            }
            At::L81e => {
                at = if edi & 3 == 0 { At::L83c } else { At::L826 };
            }
            At::L826 => {
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                ecx = ecx.wrapping_sub(1);
                if ecx == 0 {
                    return dest;
                }
                at = if edi & 3 != 0 { At::L826 } else { At::L83c };
            }
            At::L83c => {
                ebx = ecx;
                ecx >>= 2;
                at = if ecx != 0 { At::L8b7 } else { At::L843 };
            }
            At::L843 => {
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                at = At::L848;
            }
            At::L848 => {
                ebx = ebx.wrapping_sub(1);
                if ebx == 0 {
                    return dest;
                }
                at = At::L843;
            }
            At::L855 => {
                e.mem.set_u32(edi, edx);
                edi = edi.wrapping_add(4);
                ecx = ecx.wrapping_sub(1);
                at = if ecx == 0 { At::L7fe } else { At::L85f };
            }
            At::L85f => {
                edx = 0x7efe_feff;
                eax = e.mem.u32(esi);
                edx = edx.wrapping_add(eax);
                eax ^= 0xffff_ffff;
                eax ^= edx;
                edx = e.mem.u32(esi);
                esi = esi.wrapping_add(4);
                if eax & 0x8101_0100 == 0 {
                    at = At::L855;
                } else if edx as u8 == 0 {
                    // 00ecc8a9
                    edx = 0;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if (edx >> 8) as u8 == 0 {
                    // 00ecc89f
                    edx &= 0xff;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if edx & 0x00ff_0000 == 0 {
                    // 00ecc895
                    edx &= 0xffff;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if edx & 0xff00_0000 != 0 {
                    at = At::L855;
                } else {
                    // 00ecc891
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                }
            }
            At::L8ad => {
                edi = edi.wrapping_add(4);
                eax = 0;
                ecx = ecx.wrapping_sub(1);
                at = if ecx == 0 { At::L8c3 } else { At::L8b7 };
            }
            At::L8b7 => {
                eax = 0;
                at = At::L8b9;
            }
            At::L8b9 => {
                e.mem.set_u32(edi, eax);
                edi = edi.wrapping_add(4);
                ecx = ecx.wrapping_sub(1);
                at = if ecx != 0 { At::L8b9 } else { At::L8c3 };
            }
            At::L8c3 => {
                ebx &= 3;
                if ebx == 0 {
                    return dest;
                }
                at = At::L843;
            }
        }
    }
}

// Translated from 00ed2ec8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `write_char` (LIBCMT/outputs.c), a register-argument helper of the `printf` core: `eax` holds the
/// character in its low byte, `stream` is `ECX` (a `FILE`: `_ptr` +0, `_cnt` +4, `_base` +8,
/// `_flag` +0xc) and `count` is `ESI` (the number of characters written, `-1` on error). Unless
/// the stream is a string stream with no buffer (`_flag & 0x40` and `_base == 0`: nothing is
/// stored), it counts the stream down (`_cnt - 1`) and stores the byte through `_ptr` (which then
/// advances) while the count stays non-negative, otherwise `_flsbuf(char, stream)` (`00ed2d64`,
/// cdecl, the byte sign-extended). A result of -1 sets `*count` to -1 (`OR [ESI], EAX`); any other
/// adds 1 to `*count`. The result is `EAX`: the byte or the `_flsbuf` result, or the incoming
/// `EAX` when nothing was stored.
pub fn write_char(e: &mut Engine, eax: u32, stream: Ptr, count: Ptr) -> u32 {
    let (ptr_at, cnt_at, base_at, flag_at) = (
        stream.addr(),
        stream.addr().wrapping_add(4),
        stream.addr().wrapping_add(8),
        stream.addr().wrapping_add(0xc),
    );
    // TEST byte [ECX + 0xc], 0x40 / JZ; CMP [ECX + 8], 0 / JZ 00ed2ef8
    let skip = e.mem.u8(flag_at) & 0x40 != 0 && e.mem.u32(base_at) == 0;
    if skip {
        e.mem
            .set_u32(count.addr(), e.mem.u32(count.addr()).wrapping_add(1));
        return eax;
    }
    let remaining = e.mem.i32(cnt_at).wrapping_sub(1);
    e.mem.set_i32(cnt_at, remaining);
    let result = if remaining < 0 {
        // MOVSX EAX, AL, then _flsbuf(EAX, ECX).
        let character = i32::from(eax as u8 as i8) as u32;
        e.call(FLSBUF, &args![character, stream]).u32()
    } else {
        let at = e.mem.u32(ptr_at);
        e.mem.set_u8(at, eax as u8);
        e.mem.set_u32(ptr_at, at.wrapping_add(1));
        u32::from(eax as u8)
    };
    if result == 0xffff_ffff {
        let written = e.mem.u32(count.addr());
        e.mem.set_u32(count.addr(), written | result);
    } else {
        e.mem
            .set_u32(count.addr(), e.mem.u32(count.addr()).wrapping_add(1));
    }
    result
}

/// `_flsbuf` (`00ed2d64`, cdecl: the character, the stream).
const FLSBUF: u32 = 0x00ed_2d64;

// Translated from 00ed4638 (decompiled, FalloutNV.exe 1.4.0.525)
/// `write_string` (LIBCMT/outputs.c), a helper of the `printf` core with register arguments:
/// `count` is `EAX` (the characters-written counter, `ESI` inside), `string` `ECX`, `stream` `EDI`
/// and `length` the one stack word. For a string stream with no buffer (`_flag & 0x40` set,
/// `_base` zero) it just adds `length` to `*count`. Otherwise it writes the bytes through
/// `write_char` while `length` is positive; when a write leaves `*count` at -1 and `errno` is
/// `0x2a` (`EILSEQ`) it writes a `?` instead and goes on, with any other `errno` it stops.
pub fn write_string(e: &mut Engine, count: Ptr, string: Ptr, stream: Ptr, length: i32) {
    let mut eax = count.addr();
    let mut remaining = length;
    let mut at = string.addr();
    let flag = e.mem.u8(stream.addr().wrapping_add(0xc));
    if flag & 0x40 != 0 && e.mem.u32(stream.addr().wrapping_add(8)) == 0 {
        let written = e.mem.u32(count.addr());
        e.mem
            .set_u32(count.addr(), written.wrapping_add(length as u32));
        return;
    }
    while remaining > 0 {
        eax = (eax & !0xff) | u32::from(e.mem.u8(at));
        remaining -= 1;
        eax = write_char(e, eax, stream, count);
        at = at.wrapping_add(1);
        if e.mem.i32(count.addr()) != -1 {
            continue;
        }
        let slot = errno(e);
        if e.mem.u32(slot.addr()) != 0x2a {
            return;
        }
        eax = (eax & !0xff) | 0x3f;
        eax = write_char(e, eax, stream, count);
    }
}

// Translated from 00ecdf44 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__iob_func` (LIBCMT/_file.c), cdecl: the address of the CRT's table of the standard streams
/// (`stdin` first), `011c1a08`; reads nothing.
pub fn iob_func(_e: &mut Engine) -> Ptr {
    Ptr::new(0x011c_1a08)
}

// Translated from 00ed27c5 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__fload_withFB` (Ghidra's function-ID label), a fastcall helper (`ECX` unused, `EDX` = the
/// address of a double) that returns `ST0` and, in `EAX`, the double's exponent bits
/// (`high & 0x7ff00000`, which is not `0x7ff00000`) after `FLD double [EDX]`. For an exponent of
/// all ones (an infinity or NaN) it builds the extended-precision value by hand instead (exponent
/// `0x7fff`, the significand shifted up by 11 with its integer bit set, the sign kept) and loads
/// that, which does not quiet a signalling NaN; `EAX` is then the whole high word. It sets `ZF`
/// exactly in that case (`TEST EAX, 0`), clear otherwise (the `CMP`'s result). In the engine's
/// `f64` model both loads give the same bits.
pub fn fload_with_fb(e: &mut Engine, _unused_ecx: u32, edx: Ptr) -> Ret {
    let high = e.mem.u32(edx.addr().wrapping_add(4));
    let exponent = high & 0x7ff0_0000;
    let bits = e.mem.u64(edx.addr());
    if exponent != 0x7ff0_0000 {
        return Ret {
            eax: exponent,
            st0: f64::from_bits(bits),
            ..Ret::default()
        };
    }
    Ret {
        eax: high,
        st0: f64::from_bits(bits),
        ..Ret::default()
    }
}

/// The registered form of [`fload_with_fb`]: `(ECX, EDX)`, giving `EAX` and `ST0`.
fn fload_with_fb_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fload_with_fb(e, words[0], Ptr::new(words[1]))
}

/// The CRT's exception record, context and debugger flag for `___report_gsfailure`:
/// `ExceptionCode` `0126fe18`, `ExceptionFlags` `0126fe1c`, `ContextFlags` `0126fe70` and the
/// `IsDebuggerPresent` result `0126fe68`. The pointer pair handed to the filter is the static
/// `010f2a40`.
const GS_EXCEPTION_CODE: u32 = 0x0126_fe18;
const GS_EXCEPTION_FLAGS: u32 = 0x0126_fe1c;
const GS_CONTEXT_FLAGS: u32 = 0x0126_fe70;
const GS_DEBUGGER_PRESENT: u32 = 0x0126_fe68;
const GS_EXCEPTION_POINTERS: u32 = 0x010f_2a40;
const IS_DEBUGGER_PRESENT: u32 = 0x00fd_f0f0;
const SET_UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f180;
const UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f17c;
const GET_CURRENT_PROCESS: u32 = 0x00fd_f110;
const TERMINATE_PROCESS: u32 = 0x00fd_f178;
/// `STATUS_STACK_BUFFER_OVERRUN`, the exception code and exit code of a failed cookie check.
const STACK_BUFFER_OVERRUN: u32 = 0xc000_0409;

// Translated from 00ed159c (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed159c - the caller's registers (EAX, ECX, EDX, EBX, ESI, EDI, EBP), segment registers, EFLAGS, return address and stack pointer that it saves into the context at 0126fefc..0126ff38 are machine state the engine does not have and are not written
/// `___report_gsfailure` (Ghidra's function-ID label), cdecl, no arguments: what the stack cookie
/// check (`00ec408c`) calls on a mismatch. It fills the CRT's static exception record and context
/// (`ExceptionCode` `0xc0000409`, `ExceptionFlags` 1, `ContextFlags` `0x10001`; the register
/// fields come from machine state and are skipped), keeps `IsDebuggerPresent()` in `0126fe68`,
/// calls `00ed5bae(1)`, `SetUnhandledExceptionFilter(NULL)` and
/// `UnhandledExceptionFilter(010f2a40)` (the record and context pointers), calls `00ed5bae(1)`
/// again when no debugger is present, and ends with `TerminateProcess(GetCurrentProcess(),
/// 0xc0000409)`. Nothing returns from that, so the engine stops with a panic naming this address
/// and the reason.
pub fn report_gsfailure(e: &mut Engine) {
    e.mem.set_u32(GS_CONTEXT_FLAGS, 0x0001_0001);
    e.mem.set_u32(GS_EXCEPTION_CODE, STACK_BUFFER_OVERRUN);
    e.mem.set_u32(GS_EXCEPTION_FLAGS, 1);
    let debugger = e.call(IS_DEBUGGER_PRESENT, &args![]).u32();
    e.mem.set_u32(GS_DEBUGGER_PRESENT, debugger);
    e.call(CLEAR_01270A60, &args![1u32]);
    e.call(SET_UNHANDLED_EXCEPTION_FILTER, &args![0u32]);
    e.call(UNHANDLED_EXCEPTION_FILTER, &args![GS_EXCEPTION_POINTERS]);
    if e.mem.u32(GS_DEBUGGER_PRESENT) == 0 {
        e.call(CLEAR_01270A60, &args![1u32]);
    }
    let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
    e.call(TERMINATE_PROCESS, &args![process, STACK_BUFFER_OVERRUN]);
    panic!(
        "00ed159c ___report_gsfailure: TerminateProcess(GetCurrentProcess(), 0xc0000409): \
         a stack buffer overrun (cookie mismatch) ended the process"
    );
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00401000, operator_new(u32) -> Ptr),
        entry!(0x00401460, memcpy_wrapper(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00403d30, memset_wrapper(Ptr, i32, u32) -> Ptr),
        entry!(0x00ec44d0, memcpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ec7230, memmove(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ec61c0, memset(Ptr, i32, u32) -> Ptr),
        entry!(0x00ec4835, memcmp(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec3bad, byteswap_ulong(u32) -> u32),
        entry!(0x00ec3bf0, byteswap_uint64(u64) -> u64),
        entry!(0x00ed0cdf, malloc(u32) -> Ptr),
        entry!(0x00ecd291, free(Ptr)),
        entry!(0x00ed0d24, calloc_crt(u32, u32) -> Ptr),
        entry!(0x00ec6130, strlen(Ptr) -> u32),
        entry!(0x00ec6370, strcpy(Ptr, Ptr) -> Ptr),
        entry!(0x00ec65a6, strcpy_s(Ptr, u32, Ptr) -> i32),
        entry!(0x00ec6bd2, strcat_s(Ptr, u32, Ptr) -> i32),
        entry!(0x00ec8d5f, strncpy_s(Ptr, u32, Ptr, u32) -> i32),
        entry!(0x00ec8a19, strncmp(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec68e4, stricmp(Ptr, Ptr) -> i32),
        entry!(0x00ec7ec0, strnicmp(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec7690, strchr(Ptr, i32) -> Ptr),
        entry!(0x00ec67aa, tolower(i32) -> i32),
        entry!(0x00eca7f4, toupper(i32) -> i32),
        entry!(0x00eca6d3, atol(Ptr) -> i32),
        entry!(0x00eca6bd, atol_body(Ptr) -> i32),
        entry!(0x00ec8d08, strtol(u32, u32, i32) -> i32),
        entry!(0x00ec8ad9, strtoxl(u32, u32, u32, i32, u32) -> u32),
        entry!(0x00ed1165, getptd_noexit() -> u32),
        entry!(0x00ed11de, getptd() -> u32),
        entry!(0x00ecf742, update_tlocinfo() -> u32),
        entry!(0x00ed3cdf, update_tmbcinfo() -> u32),
        entry!(0x00ec660e, locale_update(u32, u32) -> u32),
        entry!(0x00ed419c, isleadbyte_l(u32, u32) -> u32),
        entry!(0x00ee13a3, crt_get_string_type_a(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed41e7, isctype_l(i32, u32, u32) -> u32),
        entry!(0x00eccc25, crt_waiting_on_module_handle(u32) -> u32),
        entry!(0x00ed0f17, encode_pointer(u32) -> u32),
        entry!(0x00ed0f92, decode_pointer(u32) -> u32),
        entry!(0x00ed0f89, encoded_null() -> u32),
        entry!(0x00ed100d, set_flsgetvalue() -> u32),
        entry!(0x00ed1004, crt_tls_alloc(u32) -> u32),
        entry!(0x00ede1fc, fn_00ede1fc(u32)),
        entry!(0x00ede18d, fn_00ede18d(u32)),
        entry!(0x00ed2c6f, fn_00ed2c6f(u32)),
        entry!(0x00ede17e, fn_00ede17e(u32)),
        entry!(0x00eddf6c, initp_misc_winsig(u32)),
        entry!(0x00ec5eec, fn_00ec5eec(u32)),
        entry!(0x00ed1538, initp_eh_hooks(u32)),
        entry!(0x00eccf0f, init_pointers()),
        entry!(0x00ede19c, crt_init_crit_sec_and_spin_count(u32, u32) -> u32),
        entry!(0x00edddbd, mtinitlocks() -> u32),
        entry!(0x00eddf30, fn_00eddf30()),
        entry!(0x00edde76, mtinitlocknum(u32) -> u32),
        entry!(0x00eddf39, lock(u32)),
        entry!(0x00edde5f, unlock(u32)),
        entry!(0x00eccc55, amsg_exit(u32)),
        entry!(0x00ed1153, fn_00ed1153()),
        entry!(0x00ed115c, fn_00ed115c()),
        entry!(0x00ed107e, initptd(u32, u32)),
        entry!(0x00ed1041, mtterm()),
        entry!(0x00ed1327, mtinit() -> u32),
        entry!(0x00ecf704, update_tlocinfo_ex_nolock(u32, u32) -> u32),
        entry!(0x00ee11e9, crt_get_string_type_a_stat(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00edde08, mtdeletelocks()),
        entry!(0x00ed2ab7, ff_msgbanner()),
        entry!(0x00ed290c, nmsg_write(u32)),
        entry!(0x00eccedb, exit_quick(u32)),
        entry!(0x00ecf5b6, addlocaleref(u32)),
        entry!(0x00ecf645, removelocaleref(u32) -> u32),
        entry!(0x00ecf46d, freetlocinfo(u32)),
        entry!(0x00ee13e5, ansicp(u32) -> u32),
        entry!(0x00ee142e, convertcp(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ecd1c7, malloc_body(u32) -> u32),
        entry!(0x00ec8e14, freea(u32)),
        entry!(0x00ed11f8, freefls(u32)),
        entry!(0x00eca573, atof(Ptr) -> f64),
        entry!(0x00ec62c0, ftol2(f64) -> u64),
        entry!(0x00ec6040, ci_sqrt(f64) -> f64),
        entry!(0x00ec6054, sqrt(f64) -> f64),
        entry!(
            0x00ec782f,
            eh_vector_constructor_iterator(u32, u32, i32, u32, u32)
        ),
        entry!(0x00ec9f70, ci_cos(f64) -> f64),
        entry!(0x00eca0a0, ci_sin(f64) -> f64),
        entry!(0x00ec9f30, cos(f64) -> f64),
        entry!(0x00eca060, sin(f64) -> f64),
        entry!(0x00ec9b00, pow(f64, f64) -> f64),
        entry!(0x00eca1d0, acos(f64) -> f64),
        entry!(0x00ec9ac0, pow_cdecl(f64, f64) -> f64),
        entry!(0x00eca190, acos_cdecl(f64) -> f64),
        entry!(0x00ed5abb, ctrlfp(u32, u32) -> i32),
        entry!(0x00ec9050, ci_atan(f64) -> f64),
        entry!(0x00ec9760, atan2(f64, f64) -> f64),
        entry!(0x00ec8660, alloca_probe(u32, u32) -> u32),
        entry!(0x00ecadda, time64(Ptr) -> u64),
        entry!(0x00ecadb8, rand() -> i32),
        entry!(0x00ec658f, atexit(Ptr) -> i32),
        (0x00eccd99, doexit_entry as AbiFn),
        entry!(0x00ec6cc4, vsprintf(Ptr, Ptr, u32) -> i32),
        (0x00ec623a, sprintf_entry as AbiFn),
        (0x00eca38d, snprintf_entry as AbiFn),
        entry!(0x00ec7c56, invalid_parameter_noinfo()),
        entry!(0x00ec43fb, rt_dynamic_cast(Ptr, i32, Ptr, Ptr, i32) -> Ptr),
        entry!(0x00ec9e10, ceil(f64) -> f64),
        entry!(0x00ec605d, fn_00ec605d(bool, f64, u32, u32) -> f64),
        entry!(0x00ec85e3, errno() -> Ptr),
        entry!(0x00ec9650, ci_asin(f64) -> f64),
        entry!(0x00ed159c, report_gsfailure()),
        entry!(0x00ecddf1, vsnprintf(Ptr, u32, Ptr, u32) -> i32),
        entry!(0x00ec7c30, fn_00ec7c30(u32, u32, u32, u32, u32)),
        entry!(0x00ecc590, aulldiv(u32, u32, u32, u32) -> u64),
        entry!(0x00eccca9, crt_exit_process(i32)),
        entry!(0x00ecc7b0, strncpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed2ec8, write_char(u32, Ptr, Ptr) -> u32),
        entry!(0x00ed4638, write_string(Ptr, Ptr, Ptr, i32)),
        entry!(0x00ecdf44, iob_func() -> Ptr),
        (0x00ed27c5, fload_with_fb_entry as AbiFn),
    ]
}

/// Test support for the constructors of the forms (`CreateFormOfType`'s
/// table): a constructor is a run of calls into the component and base
/// constructors and stores of vtable pointers and zeros. [`run`] builds
/// the object, doubles every callee, runs the constructor by its address
/// and compares the calls (with the object's address substituted for
/// [`A::T`]) and the stores; every other byte of the object must be left
/// as it was. Other engine crates' tests reach it through the core's
/// `test-support` feature.
#[cfg(any(test, feature = "test-support"))]
pub mod ctor_test {
    use crate::prelude::*;

    /// An expected argument word: the object's address plus an offset, or
    /// a plain value.
    #[derive(Clone, Copy, Debug)]
    pub enum A {
        T(u32),
        V(u32),
    }

    pub struct Case<'a> {
        pub addr: u32,
        pub size: u32,
        /// (address, value) of the globals the constructor reads.
        pub globals: &'a [(u32, u32)],
        /// (address, value) a callee answers; other callees answer 0.
        pub returns: &'a [(u32, u32)],
        pub calls: &'a [(u32, &'a [A])],
        /// (offset, width in bytes, value).
        pub stores: &'a [(u32, u32, u32)],
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    pub fn run(c: &Case) {
        let mut e = Engine::new();
        let this = e.mem.alloc(c.size);
        e.mem.write(this, &vec![0xcd; c.size as usize]);
        for (a, v) in c.globals {
            e.mem.map(*a, 4);
            e.mem.set_u32(*a, *v);
        }
        for (a, _) in c.calls {
            e.register(*a, noop);
        }
        for (a, v) in c.returns {
            let v = *v;
            e.register_double(*a, move |_, _| Ret {
                eax: v,
                ..Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        let ret = e.call(c.addr, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(ret, this, "{:08x} returns this", c.addr);
        let log = e.call_log.take().unwrap();
        let seen: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        let want: Vec<(u32, Vec<u32>)> = c
            .calls
            .iter()
            .map(|(a, args)| {
                (
                    *a,
                    args.iter()
                        .map(|x| match x {
                            A::T(o) => this + o,
                            A::V(v) => *v,
                        })
                        .collect(),
                )
            })
            .collect();
        assert_eq!(seen.len(), want.len(), "{:08x}: calls {:x?}", c.addr, seen);
        for (s, w) in seen.iter().zip(&want) {
            assert_eq!(s.0, w.0, "{:08x}: callee order", c.addr);
            assert_eq!(
                s.1[..w.1.len().min(s.1.len())],
                w.1[..],
                "{:08x}: arguments of {:08x}",
                c.addr,
                w.0
            );
        }
        let mut expected = vec![0xcd_u8; c.size as usize];
        for (off, width, v) in c.stores {
            let bytes = v.to_le_bytes();
            expected[*off as usize..(*off + *width) as usize]
                .copy_from_slice(&bytes[..*width as usize]);
        }
        assert_eq!(
            e.mem.bytes(this, c.size),
            expected,
            "{:08x}: object bytes",
            c.addr
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cstr(e: &mut Engine, s: &[u8]) -> Ptr {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        Ptr::new(p)
    }

    #[test]
    fn the_vector_constructor_iterator_constructs_each_element_in_order() {
        let mut e = Engine::new();
        e.register(0x0090_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec782f, &[0x1000, 0xc, 3, 0x0090_0000, 0x0090_1000]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log[1..].iter().map(|(a, w)| (*a, w.clone())).collect();
        assert_eq!(
            calls,
            [
                (0x0090_0000, vec![0x1000]),
                (0x0090_0000, vec![0x100c]),
                (0x0090_0000, vec![0x1018]),
            ]
        );
    }

    #[test]
    fn byteswap_uint64_reverses_the_bytes() {
        let mut e = Engine::new();
        let r = e.call(0x00ec3bf0, &[0x0506_0708, 0x0102_0304]);
        assert_eq!((r.eax, r.edx), (0x0403_0201, 0x0807_0605));
    }

    #[test]
    fn byteswap_ulong_reverses_the_bytes() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec3bad, &[0x1234_5678]).u32(), 0x7856_3412);
        assert_eq!(e.call(0x00ec3bad, &[0xff00_0001]).u32(), 0x0100_00ff);
    }

    #[test]
    fn strings() {
        let mut e = Engine::new();
        let a = cstr(&mut e, b"Goodsprings");
        let b = cstr(&mut e, b"GOODSPRINGS");
        assert_eq!(e.call(0x00ec_6130, &args![a]).u32(), 11);
        assert_eq!(e.call(0x00ec_68e4, &args![a, b]).i32(), 0);
        assert_eq!(e.call(0x00ec_8a19, &args![a, b, 1u32]).i32(), 0);
        assert!(e.call(0x00ec_8a19, &args![a, b, 2u32]).i32() > 0);
        assert_eq!(e.call(0x00ec_7ec0, &args![a, b, 11u32]).i32(), 0);
        let o = e.call(0x00ec_7690, &args![a, b'p' as i32]).u32();
        assert_eq!(o, a.addr() + 5);
        let buf: Ptr = Ptr::new(e.mem.alloc(8));
        assert_eq!(
            e.call(0x00ec_8d5f, &args![buf, 8u32, a, TRUNCATE]).i32(),
            80
        );
        assert_eq!(e.mem.cstr(buf.addr()), b"Goodspr");
    }

    #[test]
    #[should_panic(expected = "strcpy_s")]
    fn strcpy_s_overflow_ends_the_program() {
        let mut e = Engine::new();
        let a = cstr(&mut e, b"Primm");
        let buf: Ptr = Ptr::new(e.mem.alloc(4));
        e.call(0x00ec_65a6, &args![buf, 4u32, a]);
    }

    #[test]
    fn numbers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec_62c0, &args![-2.75f64]).u64() as i64, -2);
        assert_eq!(e.call(0x00ec_62c0, &args![f64::NAN]).u64(), 1 << 63);
        let s = cstr(&mut e, b"  -42xyz");
        assert_eq!(e.call(0x00ec_a6d3, &args![s]).i32(), -42);
        let f = cstr(&mut e, b"1.5e2 rest");
        assert_eq!(e.call(0x00ec_a573, &args![f]).f64(), 150.0);
        // MSVC rand from the default seed: 41, 18467, 6334.
        let r: Vec<i32> = (0..3).map(|_| e.call(0x00ec_adb8, &[]).i32()).collect();
        assert_eq!(r, vec![41, 18467, 6334]);
    }

    #[test]
    fn memory_and_operator_new() {
        let mut e = Engine::new();
        let p: Ptr = e.call_as(0x0040_1000, &args![12u32]);
        e.call(0x0040_3d30, &args![p, 0xab_i32, 12u32]);
        assert_eq!(e.mem.u32(p.addr() + 8), 0xabab_abab);
        let q: Ptr = e.call_as(0x0040_1000, &args![12u32]);
        e.call(0x0040_1460, &args![q, p, 12u32]);
        assert_eq!(e.call(0x00ec_4835, &args![p, q, 12u32]).i32(), 0);
    }

    /// A class `D : B` with RTTI laid out as MSVC does (COL, CHD, base
    /// class array with D then B), and a `D` object.
    #[test]
    fn dynamic_cast_to_a_base_and_back() {
        let mut e = Engine::new();
        let r = e.mem.alloc(0x200);
        let (td_d, td_b, td_x) = (r, r + 0x20, r + 0x40);
        e.mem.set_cstr(td_d + 8, b".?AVD@@");
        e.mem.set_cstr(td_b + 8, b".?AVB@@");
        e.mem.set_cstr(td_x + 8, b".?AVX@@");
        let (bcd_d, bcd_b) = (r + 0x60, r + 0x80);
        for (bcd, td) in [(bcd_d, td_d), (bcd_b, td_b)] {
            e.mem.set_u32(bcd, td);
            e.mem.set_i32(bcd + 12, -1); // pdisp: not a virtual base
        }
        let array = r + 0xa0;
        e.mem.set_u32(array, bcd_d);
        e.mem.set_u32(array + 4, bcd_b);
        let chd = r + 0xb0;
        e.mem.set_u32(chd + 8, 2);
        e.mem.set_u32(chd + 12, array);
        let col = r + 0xc0;
        e.mem.set_u32(col + 12, td_d);
        e.mem.set_u32(col + 16, chd);
        let vft = r + 0xe0;
        e.mem.set_u32(vft - 4, col);
        let obj = e.mem.alloc(8);
        e.mem.set_u32(obj, vft);
        let cast = |e: &mut Engine, to: u32| -> u32 {
            e.call(
                0x00ec_43fb,
                &args![
                    Ptr::<()>::new(obj),
                    0i32,
                    Ptr::<()>::new(td_b),
                    Ptr::<()>::new(to),
                    0i32
                ],
            )
            .u32()
        };
        assert_eq!(cast(&mut e, td_d), obj);
        assert_eq!(cast(&mut e, td_b), obj);
        assert_eq!(cast(&mut e, td_x), 0);
    }

    /// `vsprintf(format, words)` through the registered entry.
    fn vsp(e: &mut Engine, format: &[u8], words: &[u32]) -> String {
        let f = cstr(e, format);
        let list = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *w);
        }
        let dest: Ptr = Ptr::new(e.mem.alloc(512));
        let n = e.call(0x00ec_6cc4, &args![dest, f, list]).i32();
        let s = e.mem.cstr(dest.addr());
        assert_eq!(n as usize, s.len());
        String::from_utf8(s).unwrap()
    }

    fn dbl(v: f64) -> [u32; 2] {
        let b = v.to_bits();
        [b as u32, (b >> 32) as u32]
    }

    #[test]
    fn vsprintf_integers_and_strings() {
        let mut e = Engine::new();
        let s = cstr(&mut e, b"Doc Mitchell");
        assert_eq!(
            vsp(&mut e, b"%d|%i|%u", &[(-5i32) as u32, 42, (-1i32) as u32]),
            "-5|42|4294967295"
        );
        assert_eq!(
            vsp(
                &mut e,
                b"%08X|%x|%X|%o|%#x|%#o",
                &[0xbeef, 255, 255, 8, 255, 8]
            ),
            "0000BEEF|ff|FF|10|0xff|010"
        );
        assert_eq!(
            vsp(&mut e, b"%03i|%10i|%-6d|%+d|% d", &[7, 7, 7, 7, 7]),
            "007|         7|7     |+7| 7"
        );
        assert_eq!(
            vsp(&mut e, b"%.3d|%5.3d|%-5.3d|%.0d", &[5, 5, 5, 0]),
            "005|  005|005  |"
        );
        assert_eq!(
            vsp(
                &mut e,
                b"%s|%.4s|%10s|%-10s|%s",
                &[s.addr(), s.addr(), s.addr(), s.addr(), 0]
            ),
            "Doc Mitchell|Doc |Doc Mitchell|Doc Mitchell|(null)"
        );
        assert_eq!(
            vsp(
                &mut e,
                b"%c%c|100%%|%p|%hd|%lu",
                &[b'N' as u32, b'V' as u32, 0x1234, 0x1_0005, 9]
            ),
            "NV|100%|00001234|5|9"
        );
        assert_eq!(
            vsp(&mut e, b"%*d|%-*d|%.*s", &[4, 9, 3, 9, 2, s.addr()]),
            "   9|9  |Do"
        );
        assert_eq!(
            vsp(&mut e, b"%I64d|%llx", &[0xffff_ffff, 1, 0, 3]),
            "8589934591|300000000"
        );
        assert_eq!(vsp(&mut e, b"%5%|%q|%H", &[]), "%|q|H");
        assert_eq!(vsp(&mut e, b"%d%", &[1]), "1");
    }

    #[test]
    fn vsprintf_wide_text_and_count() {
        let mut e = Engine::new();
        let w = e.mem.alloc(16);
        for (i, c) in "Vault".encode_utf16().enumerate() {
            e.mem.set_u16(w + 2 * i as u32, c);
        }
        assert_eq!(
            vsp(&mut e, b"%S|%ls|%C|%lc", &[w, w, b'a' as u32, 0x100]),
            "Vault|Vault|a|"
        );
        let n = e.mem.alloc(4);
        assert_eq!(vsp(&mut e, b"abc%n", &[n]), "abc");
        assert_eq!(e.mem.u32(n), 3);
    }

    #[test]
    fn vsprintf_floats_as_the_2008_runtime() {
        let mut e = Engine::new();
        let f = |e: &mut Engine, fmt: &[u8], v: f64| vsp(e, fmt, &dbl(v));
        assert_eq!(f(&mut e, b"%f", 1.5), "1.500000");
        assert_eq!(f(&mut e, b"%0.2f", 3.45678), "3.46");
        assert_eq!(f(&mut e, b"%.2f", 2.675), "2.67"); // the 17th digits are 2.6749999999999998
        assert_eq!(f(&mut e, b"%.0f", 0.5), "1");
        assert_eq!(f(&mut e, b"%.0f", 2.5), "3");
        assert_eq!(f(&mut e, b"%5.3f|", -0.0004), "-0.000|");
        assert_eq!(f(&mut e, b"%.2f", 0.006), "0.01");
        assert_eq!(f(&mut e, b"%.2f", 9.999), "10.00");
        assert_eq!(f(&mut e, b"%f", 0.0), "0.000000");
        assert_eq!(f(&mut e, b"%08.2f", -1.5), "-0001.50");
        assert_eq!(f(&mut e, b"%+.1f", 1.25), "+1.3");
        assert_eq!(f(&mut e, b"%.20f", 0.1), "0.10000000000000001000");
        assert_eq!(f(&mut e, b"%e", 1.0), "1.000000e+000");
        assert_eq!(f(&mut e, b"%E", 12345.678), "1.234568E+004");
        assert_eq!(f(&mut e, b"%.2e", 0.000123), "1.23e-004");
        assert_eq!(f(&mut e, b"%e", 0.0), "0.000000e+000");
        assert_eq!(f(&mut e, b"%g", 100000.0), "100000");
        assert_eq!(f(&mut e, b"%g", 1000000.0), "1e+006");
        assert_eq!(f(&mut e, b"%g", 0.0001), "0.0001");
        assert_eq!(f(&mut e, b"%g", 0.00001), "1e-005");
        assert_eq!(f(&mut e, b"%g", 1.5), "1.5");
        assert_eq!(f(&mut e, b"%g", 0.0), "0");
        assert_eq!(f(&mut e, b"%G", 1.5e20), "1.5E+020");
        assert_eq!(f(&mut e, b"%#g", 1.5), "1.50000");
        assert_eq!(f(&mut e, b"%.3g", 1234.5), "1.23e+003");
        assert_eq!(f(&mut e, b"%LG", 0.25), "0.25");
        assert_eq!(f(&mut e, b"%10.1f|", 2.25), "       2.3|");
        assert_eq!(f(&mut e, b"%-10.1f|", 2.25), "2.3       |");
    }

    #[test]
    fn vsprintf_infinities_and_nans() {
        let mut e = Engine::new();
        let f = |e: &mut Engine, fmt: &[u8], v: f64| vsp(e, fmt, &dbl(v));
        assert_eq!(f(&mut e, b"%f", f64::INFINITY), "1.#INF00");
        assert_eq!(f(&mut e, b"%f", f64::NEG_INFINITY), "-1.#INF00");
        assert_eq!(f(&mut e, b"%.2f", f64::INFINITY), "1.#J");
        assert_eq!(
            f(&mut e, b"%f", f64::from_bits(0xfff8_0000_0000_0000)),
            "-1.#IND00"
        );
        assert_eq!(
            f(&mut e, b"%f", f64::from_bits(0x7ff8_0000_0000_0001)),
            "1.#QNAN0"
        );
        assert_eq!(f(&mut e, b"%g", f64::INFINITY), "1.#INF");
    }

    #[test]
    fn vsprintf_game_messages() {
        // A script error line: "%s at line %d, offset %d" style text and a hex id.
        let mut e = Engine::new();
        let name = cstr(&mut e, b"MQ101Script");
        assert_eq!(
            vsp(
                &mut e,
                b"Script '%s' line %i: form %08X, %.2f",
                &[name.addr(), 12, 0x0001_f2c0, 0, 0x4000_0000]
            ),
            "Script 'MQ101Script' line 12: form 0001F2C0, 2.00"
        );
    }

    #[test]
    fn snprintf_fits_fills_overflows_and_counts() {
        let mut e = Engine::new();
        let f = cstr(&mut e, b"Topic %s (%08x)");
        let s = cstr(&mut e, b"GREETING");
        let buffer = e.mem.alloc(32);
        let fill = |e: &mut Engine| e.mem.write(buffer, &[0xaa; 32]);
        // Room to spare: written with its zero.
        fill(&mut e);
        let n = e.call(0x00eca38d, &args![buffer, 32u32, f, s, 0x0001_2345u32]);
        assert_eq!(n.i32(), 25);
        assert_eq!(e.mem.cstr(buffer), b"Topic GREETING (00012345)".to_vec());
        assert_eq!(e.mem.u8(buffer + 25), 0);
        // Exactly full: no zero, the length.
        let short = cstr(&mut e, b"abc%d");
        fill(&mut e);
        let n = e.call(0x00eca38d, &args![buffer, 4u32, short, 7u32]);
        assert_eq!(n.i32(), 4);
        assert_eq!(e.mem.bytes(buffer, 5), b"abc7\xaa".to_vec());
        // Too long: the first `count` characters, no zero, -1.
        fill(&mut e);
        let n = e.call(0x00eca38d, &args![buffer, 3u32, short, 7u32]);
        assert_eq!(n.i32(), -1);
        assert_eq!(e.mem.bytes(buffer, 4), b"abc\xaa".to_vec());
        // A count of 0 writes nothing.
        fill(&mut e);
        let n = e.call(0x00eca38d, &args![buffer, 0u32, short, 7u32]);
        assert_eq!(n.i32(), -1);
        assert_eq!(e.mem.u8(buffer), 0xaa);
        // A null buffer with a zero count only counts.
        let n = e.call(0x00eca38d, &args![0u32, 0u32, short, 12u32]);
        assert_eq!(n.i32(), 5);
    }

    #[test]
    fn sprintf_writes_the_text_and_its_zero() {
        let mut e = Engine::new();
        let f = cstr(&mut e, b"Form %s %s (%08X)");
        let a = cstr(&mut e, b"QUST");
        let b = cstr(&mut e, b"VMS49");
        let buffer = e.mem.alloc(32);
        e.mem.write(buffer, &[0xaa; 32]);
        let n = e.call(0x00ec623a, &args![buffer, f, a, b, 0x0010_abcdu32]);
        assert_eq!(n.i32(), 26);
        assert_eq!(
            e.mem.bytes(buffer, 28),
            b"Form QUST VMS49 (0010ABCD)\0\xaa".to_vec()
        );
    }

    #[test]
    #[should_panic(expected = "CRT invalid parameter: sprintf")]
    fn sprintf_null_buffer_is_invalid() {
        let mut e = Engine::new();
        let f = cstr(&mut e, b"x");
        e.call(0x00ec623a, &args![0u32, f]);
    }

    #[test]
    #[should_panic(expected = "CRT invalid parameter: _snprintf")]
    fn snprintf_null_buffer_with_room_is_invalid() {
        let mut e = Engine::new();
        let f = cstr(&mut e, b"x");
        e.call(0x00eca38d, &args![0u32, 4u32, f]);
    }

    #[test]
    fn the_arctangent_takes_its_argument_in_st0() {
        let mut e = Engine::new();
        let r = e.call(0x00ec9050, &args![1.0f64]).f64();
        assert!((r - std::f64::consts::FRAC_PI_4).abs() < 1e-15);
        assert!(e.call(0x00ec9050, &args![f64::NAN]).f64().is_nan());
    }

    #[test]
    fn the_stack_probe_returns_the_new_stack_pointer_and_moves_the_return_address() {
        let mut e = Engine::new();
        let stack = e.mem.alloc(0x3000);
        // ESP at entry in the top page, 0x2400 bytes wanted: the probe walks two pages down.
        let esp = stack + 0x2f00;
        e.mem.set_u32(esp, 0x00c0_ffee);
        let new_esp = e.call(0x00ec8660, &args![0x2400u32, esp]).u32();
        assert_eq!(new_esp, esp - 0x2400 + 4);
        assert_eq!(e.mem.u32(esp - 0x2400), 0x00c0_ffee);
    }

    #[test]
    #[should_panic]
    fn the_stack_probe_stores_at_address_zero_when_the_size_borrows() {
        let mut e = Engine::new();
        let stack = e.mem.alloc(0x100);
        e.mem.set_u32(stack, 1);
        e.call(0x00ec8660, &args![stack + 1, stack]);
    }

    #[test]
    fn time64_adds_the_epoch_offset_divides_and_stores() {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        e.mem
            .set_u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT, 0x0900_0010);
        // The FILETIME for 2020-01-01 00:00:00 UTC: 132223104000000000.
        e.register(0x0900_0010, |e, a| {
            e.mem.set_u32(a[0], 0x6905_0000);
            e.mem.set_u32(a[0] + 4, 0x01d5_c036);
            Ret::default()
        });
        e.register(AULLDIV, |_, a| {
            let dividend = a[0] as u64 | (a[1] as u64) << 32;
            let divisor = a[2] as u64 | (a[3] as u64) << 32;
            let q = dividend / divisor;
            Ret {
                eax: q as u32,
                edx: (q >> 32) as u32,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(8);
        let now = e.call(0x00ecadda, &args![out]).u64();
        assert_eq!(now, 1_577_836_800);
        assert_eq!(e.mem.u32(out), 1_577_836_800);
        assert_eq!(e.mem.u32(out + 4), 0);
        // A null pointer is allowed.
        assert_eq!(e.call(0x00ecadda, &args![0u32]).u64(), 1_577_836_800);
    }

    #[test]
    fn time64_gives_minus_one_beyond_the_year_3000() {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        e.mem
            .set_u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT, 0x0900_0010);
        e.register(0x0900_0010, |_, _| Ret::default());
        // The quotient is the high word and low word the double returns.
        for (high, low, expected) in [
            (6u32, 0xffff_ffffu32, (6u64 << 32) | 0xffff_ffff),
            (7, 0x9340_6fff, (7u64 << 32) | 0x9340_6fff),
            (7, 0x9340_7000, u64::MAX),
            (8, 0, u64::MAX),
        ] {
            // Plain functions cannot capture the case: drive it through memory.
            e.mem.set_u32(0x00fd_f400, high);
            e.mem.set_u32(0x00fd_f404, low);
            e.register(AULLDIV, |e, _| Ret {
                eax: e.mem.u32(0x00fd_f404),
                edx: e.mem.u32(0x00fd_f400),
                ..Ret::default()
            });
            assert_eq!(e.call(0x00ecadda, &args![0u32]).u64(), expected);
        }
    }
}

#[cfg(test)]
mod exit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Order = Rc<RefCell<Vec<u32>>>;

    /// Doubles at `addrs` that append their own address to the returned log.
    fn recorders(e: &mut Engine, addrs: &[u32]) -> Order {
        let order: Order = Rc::default();
        for &a in addrs {
            let o = order.clone();
            e.register_double(a, move |_, _| {
                o.borrow_mut().push(a);
                Ret::default()
            });
        }
        order
    }

    fn table_words(e: &Engine) -> Vec<u32> {
        let (begin, end) = (e.mem.u32(ONEXIT_BEGIN), e.mem.u32(ONEXIT_END));
        (begin..end).step_by(4).map(|a| e.mem.u32(a)).collect()
    }

    #[test]
    fn atexit_records_the_function_in_registration_order() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec658f, &[0x0090_0010]).i32(), 0);
        assert_eq!(e.call(0x00ec658f, &[0x0090_0020]).i32(), 0);
        assert_eq!(table_words(&e), [0x0090_0010, 0x0090_0020]);
        // The exe's first table: 32 words of zeros, filled from the start.
        let begin = e.mem.u32(ONEXIT_BEGIN);
        assert_eq!(e.mem.block_size(begin), Some(128));
        assert_eq!(e.mem.u32(begin + 8), 0);
    }

    #[test]
    fn atexit_of_a_null_function_stores_it_and_answers_minus_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec658f, &[0]).i32(), -1);
        assert_eq!(table_words(&e), [0]);
    }

    #[test]
    fn the_table_grows_by_its_size_and_keeps_its_entries() {
        let mut e = Engine::new();
        for i in 0..100u32 {
            assert_eq!(atexit(&mut e, Ptr::new(0x0090_0000 + i)), 0);
        }
        let expect: Vec<u32> = (0..100).map(|i| 0x0090_0000 + i).collect();
        assert_eq!(table_words(&e), expect);
        // 128 bytes, then 256 (full at 64 words), then 512 (128 words).
        let begin = e.mem.u32(ONEXIT_BEGIN);
        assert_eq!(e.mem.block_size(begin), Some(512));
    }

    #[test]
    fn doexit_runs_the_exit_functions_last_first_then_the_two_tables() {
        let mut e = Engine::new();
        let order = recorders(
            &mut e,
            &[
                0x0090_0010,
                0x0090_0020,
                0x0090_0030,
                0x0090_1000,
                0x0090_1010,
                0x0090_2000,
            ],
        );
        for f in [0x0090_0010, 0x0090_0020, 0x0090_0030] {
            atexit(&mut e, Ptr::new(f));
        }
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        // Slots: pre-terminators 3 (one empty), then the terminator slot.
        e.mem.set_u32(0x0101_13f8, 0x0090_1000);
        e.mem.set_u32(0x0101_13fc, 0x0090_1010);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        // A word past each table's end is not run.
        e.mem.set_u32(0x0101_1400, 0x0090_3000);
        e.mem.set_u32(0x0101_1408, 0x0090_3000);
        let report = doexit(&mut e, 5, 0, 0);
        assert_eq!(
            *order.borrow(),
            [
                0x0090_0030,
                0x0090_0020,
                0x0090_0010,
                0x0090_1000,
                0x0090_1010,
                0x0090_2000
            ]
        );
        assert_eq!(report.ran, *order.borrow());
        assert!(report.open.is_empty() && report.stopped.is_empty());
        assert_eq!(report.exit_process, Some(5));
        // Each exit function was cleared before it ran.
        assert_eq!(table_words(&e), [0, 0, 0]);
        assert_eq!(e.mem.u32(EXIT_STARTED), 1);
        assert_eq!(e.mem.u32(EXIT_DONE), 1);
        assert_eq!(e.mem.u8(EXIT_RETURNS), 0);
    }

    #[test]
    fn a_quick_exit_runs_only_the_terminators() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_0010, 0x0090_1000, 0x0090_2000]);
        atexit(&mut e, Ptr::new(0x0090_0010));
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_13f8, 0x0090_1000);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        let report = doexit(&mut e, 1, 1, 0);
        assert_eq!(*order.borrow(), [0x0090_2000]);
        assert_eq!(report.exit_process, Some(1));
        // The exit function is still registered.
        assert_eq!(table_words(&e), [0x0090_0010]);
    }

    #[test]
    fn a_second_exit_after_the_process_exit_began_runs_nothing() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_2000]);
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        doexit(&mut e, 0, 0, 0);
        let again = doexit(&mut e, 3, 0, 0);
        assert_eq!(*order.borrow(), [0x0090_2000]);
        assert!(again.ran.is_empty());
        assert_eq!(again.exit_process, Some(3));
    }

    #[test]
    fn a_returning_exit_keeps_going_and_records_the_caller_flag() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_2000]);
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.exit_process, None);
        assert_eq!(e.mem.u32(EXIT_DONE), 0);
        assert_eq!(e.mem.u8(EXIT_RETURNS), 1);
        doexit(&mut e, 0, 0, 1);
        assert_eq!(*order.borrow(), [0x0090_2000, 0x0090_2000]);
    }

    #[test]
    fn a_handler_that_registers_another_restarts_the_walk_at_the_new_end() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_0020, 0x0090_0040]);
        let o = order.clone();
        e.register_double(0x0090_0030, move |e, _| {
            o.borrow_mut().push(0x0090_0030);
            atexit(e, Ptr::new(0x0090_0040));
            Ret::default()
        });
        atexit(&mut e, Ptr::new(0x0090_0020));
        atexit(&mut e, Ptr::new(0x0090_0030));
        doexit(&mut e, 0, 0, 1);
        // 30 runs first and registers 40, which runs next, then 20.
        assert_eq!(*order.borrow(), [0x0090_0030, 0x0090_0040, 0x0090_0020]);
    }

    #[test]
    fn handlers_nobody_translated_are_skipped_and_counted() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_0010]);
        // 00f00f00 is not translated; the handler at 00900030 stops in it.
        e.register(0x0090_0030, |e, _| e.call(0x00f0_0f00, &[]));
        for f in [0x0090_0010, 0x00f0_0f00, 0x0090_0030] {
            atexit(&mut e, Ptr::new(f));
        }
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.open, [0x00f0_0f00]);
        assert_eq!(report.stopped.len(), 1);
        assert_eq!(report.stopped[0].0, 0x0090_0030);
        assert!(report.stopped[0].1.contains("open function 00f00f00"));
        assert_eq!(*order.borrow(), [0x0090_0010]);
        assert_eq!(report.ran, [0x0090_0010]);
    }

    #[test]
    fn destructor_rows_run_through_the_exit_functions() {
        // 00fc7a70: call 00404a00 on 011c3264 (data/atexit_destructors.tsv).
        let mut e = Engine::new();
        let seen: Order = Rc::default();
        let s = seen.clone();
        e.register_double(0x0040_4a00, move |_, a| {
            s.borrow_mut().extend_from_slice(a);
            Ret::default()
        });
        assert_eq!(e.call(0x00ec658f, &[0x00fc_7a70]).i32(), 0);
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.ran, [0x00fc_7a70]);
        assert_eq!(*seen.borrow(), [0x011c_3264]);
    }

    #[test]
    fn doexit_is_callable_at_its_address() {
        let mut e = Engine::new();
        let order = recorders(&mut e, &[0x0090_0010]);
        atexit(&mut e, Ptr::new(0x0090_0010));
        e.call(0x00eccd99, &args![0u32, 0u32, 1u32]);
        assert_eq!(*order.borrow(), [0x0090_0010]);
    }

    #[test]
    fn the_stack_forms_of_pow_and_acos_reach_the_register_forms_bodies() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec9ac0, &args![2.0f64, 10.0f64]).f64(), 1024.0);
        assert_eq!(e.call(0x00eca190, &args![0.5f64]).f64(), 0.5f64.acos());
        let log = e.call_log.take().unwrap();
        let words = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        };
        let two = 2.0f64.to_bits();
        let ten = 10.0f64.to_bits();
        let half = 0.5f64.to_bits();
        assert_eq!(
            words(CI_POW),
            [vec![
                two as u32,
                (two >> 32) as u32,
                ten as u32,
                (ten >> 32) as u32
            ]]
        );
        assert_eq!(words(CI_ACOS), [vec![half as u32, (half >> 32) as u32]]);
    }

    #[test]
    fn atan2_takes_y_first_and_keeps_the_quadrant() {
        let mut e = Engine::new();
        let quarter_turn = std::f64::consts::FRAC_PI_2;
        assert_eq!(
            e.call(0x00ec9760, &args![1.0f64, 1.0f64]).f64(),
            1.0f64.atan2(1.0)
        );
        assert_eq!(
            e.call(0x00ec9760, &args![1.0f64, 0.0f64]).f64(),
            quarter_turn
        );
        assert_eq!(
            e.call(0x00ec9760, &args![0.0f64, -1.0f64]).f64(),
            std::f64::consts::PI
        );
        assert_eq!(
            e.call(0x00ec9760, &args![-1.0f64, -1.0f64]).f64(),
            (-1.0f64).atan2(-1.0)
        );
        assert!(e.call(0x00ec9760, &args![-1.0f64, -1.0f64]).f64() < -quarter_turn);
    }

    #[test]
    fn ctrlfp_merges_the_masked_bits_and_returns_the_old_control_word() {
        let mut e = Engine::new();
        // The game's control word until something loads another.
        assert_eq!(ctrlfp(&mut e, 0x133f, 0xffff), 0x027f);
        assert_eq!(x87_control_word(&mut e), 0x133f);
        // Only the masked bits change.
        assert_eq!(ctrlfp(&mut e, 0x0c00, 0x0c00), 0x133f);
        assert_eq!(x87_control_word(&mut e), 0x1f3f);
        // FLDCW takes the low 16 bits; the old word comes back sign-extended.
        assert_eq!(
            e.call(0x00ed5abb, &args![0x0001_8000u32, 0xffff_ffffu32])
                .i32(),
            0x1f3f
        );
        assert_eq!(ctrlfp(&mut e, 0x027f, 0xffff), -0x8000);
        assert_eq!(x87_control_word(&mut e), 0x027f);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double at `addr` that records its call in `log` and answers `ret`.
    fn recorder(e: &mut Engine, log: &Log, addr: u32, ret: Ret) {
        let log = log.clone();
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            ret
        });
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn panic_text(result: std::thread::Result<Ret>) -> String {
        let payload = result.expect_err("the engine stops with a panic");
        match payload.downcast_ref::<String>() {
            Some(text) => text.clone(),
            None => (*payload.downcast_ref::<&str>().unwrap()).to_string(),
        }
    }

    fn words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    // ---- _errno, _vsnprintf, __iob_func ------------------------------------------------

    #[test]
    fn errno_is_the_thread_blocks_terrno_or_the_static_slot() {
        let mut e = Engine::new();
        e.register(GETPTD_NOEXIT, |_, _| eax(0x7000));
        assert_eq!(e.call(0x00ec_85e3, &args![]).u32(), 0x7008);
        e.register(GETPTD_NOEXIT, |_, _| eax(0));
        assert_eq!(e.call(0x00ec_85e3, &args![]).u32(), 0x011c_18c0);
    }

    #[test]
    fn vsnprintf_passes_a_null_locale_to_the_locale_version() {
        let mut e = Engine::new();
        let log = Log::default();
        recorder(&mut e, &log, VSNPRINTF_L, eax(17));
        let result = e.call(0x00ec_ddf1, &args![0x100u32, 64u32, 0x200u32, 0x300u32]);
        assert_eq!(result.i32(), 17);
        assert_eq!(
            calls_to(&log, VSNPRINTF_L),
            vec![vec![0x100, 64, 0x200, 0, 0x300]]
        );
    }

    #[test]
    fn iob_func_is_the_address_of_the_stream_table() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec_df44, &args![]).u32(), 0x011c_1a08);
    }

    // ---- the invalid-parameter path ------------------------------------------------------

    #[test]
    fn a_registered_handler_gets_the_five_words() {
        let mut e = Engine::new();
        e.map(INVALID_PARAMETER_HANDLER, 4);
        e.set_global(INVALID_PARAMETER_HANDLER, 0xaaaa_bbbbu32);
        let log = Log::default();
        recorder(&mut e, &log, DECODE_POINTER, eax(0x0050_0000));
        recorder(&mut e, &log, 0x0050_0000, Ret::default());
        recorder(&mut e, &log, INVOKE_WATSON, Ret::default());
        e.call(0x00ec_7c30, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(calls_to(&log, DECODE_POINTER), vec![vec![0xaaaa_bbbb]]);
        assert_eq!(calls_to(&log, 0x0050_0000), vec![vec![1, 2, 3, 4, 5]]);
        assert!(calls_to(&log, INVOKE_WATSON).is_empty());
        assert!(calls_to(&log, CLEAR_01270A60).is_empty());
    }

    #[test]
    fn without_a_handler_it_clears_the_flag_and_invokes_watson() {
        let mut e = Engine::new();
        e.map(INVALID_PARAMETER_HANDLER, 4);
        let log = Log::default();
        recorder(&mut e, &log, DECODE_POINTER, eax(0));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, INVOKE_WATSON, Ret::default());
        e.call(0x00ec_7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![DECODE_POINTER, CLEAR_01270A60, INVOKE_WATSON]);
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![2]]);
        assert_eq!(calls_to(&log, INVOKE_WATSON), vec![vec![0, 0, 0, 0, 0]]);
    }

    // ---- the exits -------------------------------------------------------------------------

    #[test]
    fn crt_exit_process_tells_the_runtime_then_exits_and_stops_the_engine() {
        let mut e = Engine::new();
        let log = Log::default();
        recorder(&mut e, &log, CRT_COR_EXIT_PROCESS, Ret::default());
        recorder(&mut e, &log, EXIT_PROCESS, Ret::default());
        let text = panic_text(catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00ec_cca9, &args![3u32])
        })));
        assert!(text.contains("00eccca9"));
        assert!(text.contains("ExitProcess(3)"));
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![CRT_COR_EXIT_PROCESS, EXIT_PROCESS]);
        assert_eq!(calls_to(&log, CRT_COR_EXIT_PROCESS), vec![vec![3]]);
        assert_eq!(calls_to(&log, EXIT_PROCESS), vec![vec![3]]);
    }

    #[test]
    fn report_gsfailure_fills_the_record_and_ends_the_process() {
        let mut e = Engine::new();
        e.map(0x0126_fe00, 0x200);
        let log = Log::default();
        recorder(&mut e, &log, IS_DEBUGGER_PRESENT, eax(0));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, SET_UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, GET_CURRENT_PROCESS, eax(0xffff_ffff));
        recorder(&mut e, &log, TERMINATE_PROCESS, Ret::default());
        let text = panic_text(catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00ed_159c, &args![])
        })));
        assert!(text.contains("00ed159c"));
        assert!(text.contains("0xc0000409"));
        assert_eq!(e.mem.u32(0x0126_fe18), 0xc000_0409);
        assert_eq!(e.mem.u32(0x0126_fe1c), 1);
        assert_eq!(e.mem.u32(0x0126_fe70), 0x0001_0001);
        assert_eq!(e.mem.u32(0x0126_fe68), 0);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                IS_DEBUGGER_PRESENT,
                CLEAR_01270A60,
                SET_UNHANDLED_EXCEPTION_FILTER,
                UNHANDLED_EXCEPTION_FILTER,
                CLEAR_01270A60,
                GET_CURRENT_PROCESS,
                TERMINATE_PROCESS,
            ]
        );
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![1], vec![1]]);
        assert_eq!(
            calls_to(&log, UNHANDLED_EXCEPTION_FILTER),
            vec![vec![0x010f_2a40]]
        );
        assert_eq!(
            calls_to(&log, TERMINATE_PROCESS),
            vec![vec![0xffff_ffff, 0xc000_0409]]
        );
    }

    #[test]
    fn report_gsfailure_with_a_debugger_skips_the_second_hook_call() {
        let mut e = Engine::new();
        e.map(0x0126_fe00, 0x200);
        let log = Log::default();
        recorder(&mut e, &log, IS_DEBUGGER_PRESENT, eax(1));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, SET_UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, GET_CURRENT_PROCESS, eax(1));
        recorder(&mut e, &log, TERMINATE_PROCESS, Ret::default());
        let _ = catch_unwind(AssertUnwindSafe(|| e.call(0x00ed_159c, &args![])));
        assert_eq!(e.mem.u32(0x0126_fe68), 1);
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![1]]);
    }

    // ---- __aulldiv -------------------------------------------------------------------------

    fn divide(e: &mut Engine, n: u64, d: u64) -> u64 {
        e.call(
            0x00ec_c590,
            &args![n as u32, (n >> 32) as u32, d as u32, (d >> 32) as u32],
        )
        .u64()
    }

    #[test]
    fn aulldiv_agrees_with_64_bit_division() {
        let mut e = Engine::new();
        let mut state = 0x1234_5678_9abc_def1u64;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state
        };
        let edge = [
            0u64,
            1,
            2,
            0xffff_ffff,
            0x1_0000_0000,
            0x1_0000_0001,
            0x7fff_ffff_ffff_ffff,
            0x8000_0000_0000_0000,
            0xffff_ffff_ffff_ffff,
            0xfedc_ba98_7654_3210,
        ];
        for &n in &edge {
            for &d in edge.iter().filter(|d| **d != 0) {
                assert_eq!(divide(&mut e, n, d), n / d, "{n:#x} / {d:#x}");
            }
        }
        for round in 0..4000 {
            let n = next();
            // Divisors of every width: shift a random word down by 0..63 bits.
            let d = (next() >> (round % 64)).max(1);
            assert_eq!(divide(&mut e, n, d), n / d, "{n:#x} / {d:#x}");
        }
    }

    #[test]
    fn aulldiv_needs_its_correction_step_for_a_wide_divisor() {
        let mut e = Engine::new();
        // The shifted estimate is one too big here: the corrected quotient is 0.
        let n = 0x1_ffff_ffffu64;
        let d = 0x2_0000_0000u64;
        assert_eq!(divide(&mut e, n, d), 0);
        // And exactly on the boundary: the estimate stands.
        assert_eq!(divide(&mut e, 0x2_0000_0000, d), 1);
        assert_eq!(divide(&mut e, 0x3_ffff_ffff, d), 1);
        assert_eq!(divide(&mut e, 0x4_0000_0000, d), 2);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn aulldiv_by_zero_faults() {
        let mut e = Engine::new();
        divide(&mut e, 5, 0);
    }

    // ---- strncpy ---------------------------------------------------------------------------

    /// The C `strncpy` the exe's routine must agree with: bytes of the string up to `count`,
    /// zero-padded, nothing written past `count`.
    fn reference(source: &[u8], count: usize) -> Vec<u8> {
        let mut out = vec![0xaau8; count + 8];
        let mut ended = false;
        for i in 0..count {
            let byte = if ended { 0 } else { source[i] };
            if byte == 0 {
                ended = true;
            }
            out[i] = byte;
        }
        out
    }

    #[test]
    fn strncpy_matches_the_c_function_for_every_alignment_length_and_count() {
        let mut e = Engine::new();
        for source_align in 0..4u32 {
            for dest_align in 0..4u32 {
                for length in 0..14usize {
                    for count in 0..20usize {
                        let source_block = e.mem.alloc(64);
                        let dest_block = e.mem.alloc(64);
                        let mut text: Vec<u8> = (0..length).map(|i| b'A' + i as u8).collect();
                        text.push(0);
                        // Bytes after the terminator are not zero: the copy must not use them.
                        text.extend_from_slice(&[0x55; 24]);
                        let source = source_block + source_align;
                        e.mem.write(source, &text);
                        let dest = dest_block + dest_align;
                        e.mem.write(dest, &[0xaa; 40]);
                        let result = e
                            .call(0x00ec_c7b0, &args![dest, source, count as u32])
                            .u32();
                        assert_eq!(result, dest);
                        let expected = reference(&text, count);
                        let got = e.mem.bytes(dest, count as u32 + 8);
                        assert_eq!(
                            got, expected,
                            "source+{source_align} dest+{dest_align} length {length} count {count}"
                        );
                        e.mem.free(source_block);
                        e.mem.free(dest_block);
                    }
                }
            }
        }
    }

    #[test]
    fn strncpy_stores_a_word_cut_at_each_zero_byte_position() {
        let mut e = Engine::new();
        // An aligned source of exactly 8 bytes copied: the zero is in byte 0, 1, 2 or 3 of the
        // second word, and the first word is full (the (w ^ !0) ^ (w + 0x7efefeff) test fires
        // only for the second).
        for zero_at in 0..4usize {
            let source = e.mem.alloc(16);
            let dest = e.mem.alloc(16);
            let mut text = *b"WXYZabcd";
            text[4 + zero_at] = 0;
            e.mem.write(source, &text);
            e.mem.write(dest, &[0xaa; 16]);
            e.call(0x00ec_c7b0, &args![dest, source, 8u32]);
            let mut expected = text.to_vec();
            for byte in expected[4 + zero_at..].iter_mut() {
                *byte = 0;
            }
            assert_eq!(e.mem.bytes(dest, 8), expected, "zero at {zero_at}");
            assert_eq!(e.mem.bytes(dest + 8, 8), vec![0xaa; 8]);
        }
    }

    // ---- write_char / write_string ---------------------------------------------------------

    /// A `FILE` at a fresh block: `_ptr`, `_cnt`, `_base`, `_flag`, and a 16-byte buffer.
    fn stream(e: &mut Engine, cnt: i32, base: bool, flag: u32) -> (Ptr, u32) {
        let file = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(file, buffer);
        e.mem.set_i32(file + 4, cnt);
        e.mem.set_u32(file + 8, if base { buffer } else { 0 });
        e.mem.set_u32(file + 0xc, flag);
        (Ptr::new(file), buffer)
    }

    #[test]
    fn write_char_stores_the_byte_and_counts_it() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 3, true, 0);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 5);
        let result = e.call(
            0x00ed_2ec8,
            &args![0x1234_5641u32, file, Ptr::<()>::new(count)],
        );
        assert_eq!(result.u32(), 0x41, "the byte, zero-extended");
        assert_eq!(e.mem.u8(buffer), 0x41);
        assert_eq!(e.mem.u32(file.addr()), buffer + 1);
        assert_eq!(e.mem.i32(file.addr() + 4), 2);
        assert_eq!(e.mem.u32(count), 6);
    }

    #[test]
    fn write_char_calls_flsbuf_when_the_buffer_is_full() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 0, true, 0);
        let count = e.mem.alloc(4);
        let log = Log::default();
        recorder(&mut e, &log, FLSBUF, eax(0x80));
        let result = e.call(0x00ed_2ec8, &args![0xf0u32, file, Ptr::<()>::new(count)]);
        assert_eq!(result.u32(), 0x80);
        assert_eq!(e.mem.i32(file.addr() + 4), -1);
        assert_eq!(e.mem.u8(buffer), 0, "nothing stored by write_char");
        assert_eq!(
            calls_to(&log, FLSBUF),
            vec![vec![0xffff_fff0, file.addr()]],
            "the byte sign-extended, then the stream"
        );
        assert_eq!(e.mem.u32(count), 1);
    }

    #[test]
    fn write_char_with_a_failed_flsbuf_sets_the_count_to_minus_one() {
        let mut e = Engine::new();
        let (file, _) = stream(&mut e, 0, true, 0);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 7);
        e.register(FLSBUF, |_, _| eax(0xffff_ffff));
        let result = e.call(0x00ed_2ec8, &args![0x41u32, file, Ptr::<()>::new(count)]);
        assert_eq!(result.u32(), 0xffff_ffff);
        assert_eq!(e.mem.i32(count), -1);
    }

    #[test]
    fn write_char_to_a_string_stream_without_a_buffer_only_counts() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 5, false, 0x40);
        let count = e.mem.alloc(4);
        let result = e.call(
            0x00ed_2ec8,
            &args![0xabcd_ef41u32, file, Ptr::<()>::new(count)],
        );
        assert_eq!(result.u32(), 0xabcd_ef41, "EAX is untouched");
        assert_eq!(e.mem.u32(count), 1);
        assert_eq!(e.mem.u8(buffer), 0);
        assert_eq!(e.mem.i32(file.addr() + 4), 5);
        // The flag alone is not enough: with a buffer the byte is stored.
        let (file, buffer) = stream(&mut e, 5, true, 0x40);
        e.call(0x00ed_2ec8, &args![0x42u32, file, Ptr::<()>::new(count)]);
        assert_eq!(e.mem.u8(buffer), 0x42);
    }

    #[test]
    fn write_string_writes_every_byte() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 8, true, 0);
        let count = e.mem.alloc(4);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"hello");
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 5i32],
        );
        assert_eq!(e.mem.bytes(buffer, 5), b"hello");
        assert_eq!(e.mem.u32(count), 5);
        assert_eq!(e.mem.i32(file.addr() + 4), 3);
        // A length of zero or less writes nothing.
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, -2i32],
        );
        assert_eq!(e.mem.u32(count), 5);
    }

    #[test]
    fn write_string_to_a_string_stream_without_a_buffer_adds_the_length() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 8, false, 0x40);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 3);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"hello");
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 5i32],
        );
        assert_eq!(e.mem.u32(count), 8);
        assert_eq!(e.mem.u8(buffer), 0);
    }

    #[test]
    fn write_string_stops_on_an_error_but_writes_a_question_mark_for_eilseq() {
        let mut e = Engine::new();
        let count = e.mem.alloc(4);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc");
        let errno_slot = e.mem.alloc(4);
        e.register_double(GETPTD_NOEXIT, move |_, _| eax(errno_slot - 8));
        // Every flush fails; with errno EILSEQ (0x2a) the string goes on with a '?' each time.
        let (file, _) = stream(&mut e, 0, true, 0);
        let log = Log::default();
        recorder(&mut e, &log, FLSBUF, eax(0xffff_ffff));
        e.mem.set_u32(errno_slot, 0x2a);
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 3i32],
        );
        // For each of the three bytes: the byte (fails), then '?' (fails again).
        let flushed: Vec<u32> = calls_to(&log, FLSBUF).iter().map(|c| c[0]).collect();
        assert_eq!(flushed, vec![0x61, 0x3f, 0x62, 0x3f, 0x63, 0x3f]);
        // With any other errno it stops after the first failure.
        let (file, _) = stream(&mut e, 0, true, 0);
        e.mem.set_u32(errno_slot, 9);
        log.borrow_mut().clear();
        e.mem.set_u32(count, 0);
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 3i32],
        );
        let flushed: Vec<u32> = calls_to(&log, FLSBUF).iter().map(|c| c[0]).collect();
        assert_eq!(flushed, vec![0x61]);
        assert_eq!(e.mem.i32(count), -1);
    }

    // ---- __fload_withFB --------------------------------------------------------------------

    #[test]
    fn fload_with_fb_loads_a_finite_double_and_returns_its_exponent_bits() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        e.mem.set_f64(slot, 6.5);
        let result = e.call(0x00ed_27c5, &[0, slot]);
        assert_eq!(result.st0, 6.5);
        assert_eq!(result.eax, 0x4010_0000);
        e.mem.set_f64(slot, -0.0);
        let result = e.call(0x00ed_27c5, &[0, slot]);
        assert_eq!(result.st0.to_bits(), (-0.0f64).to_bits());
        assert_eq!(result.eax, 0);
    }

    #[test]
    fn fload_with_fb_keeps_infinities_and_signalling_nans_as_they_are() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        for bits in [
            0x7ff0_0000_0000_0000u64,
            0xfff0_0000_0000_0000,
            0x7ff0_0000_0000_0001, // signalling: not quieted by this load
            0x7ff8_0000_0000_1234,
            0xfff8_0000_0000_0000,
        ] {
            e.mem.set_u64(slot, bits);
            let result = e.call(0x00ed_27c5, &[0xdead, slot]);
            assert_eq!(result.st0.to_bits(), bits, "{bits:#x}");
            assert_eq!(result.eax, (bits >> 32) as u32, "the whole high word");
        }
    }

    // ---- ceil ------------------------------------------------------------------------------

    /// The constants of `ceil`'s SSE2 body and the flags its dispatch reads.
    fn ceil_setup(e: &mut Engine, sse2: bool) {
        e.map(0x010f_1fe0, 0x60);
        e.mem.set_u64(CEIL_SHIFT_BASE, 0x433);
        e.mem.set_u64(CEIL_SHIFT_BASE + 8, 0x433);
        e.mem.set_u64(CEIL_ONE_MASK, 1.0f64.to_bits());
        e.mem.set_u64(CEIL_ONE_MASK + 8, 1.0f64.to_bits());
        e.mem.set_u64(CEIL_ZERO, 0);
        e.mem.set_u64(CEIL_EXPONENT_MASK, 0x7ff);
        e.mem.set_u64(CEIL_MINUS_ZERO, (-0.0f64).to_bits());
        e.map(USE_SSE2_MATHFCNS, 4);
        e.set_global(USE_SSE2_MATHFCNS, u32::from(sse2));
    }

    fn ceil_via_registry(e: &mut Engine, x: f64) -> f64 {
        e.call(0x00ec_9e10, &args![x]).f64()
    }

    #[test]
    fn ceil_sse2_matches_the_host_ceil_bit_for_bit() {
        let mut e = Engine::new();
        ceil_setup(&mut e, true);
        let values = [
            0.0f64,
            -0.0,
            0.25,
            0.5,
            0.999_999_999_999,
            1.0,
            1.000_000_000_001,
            1.5,
            2.0,
            2.000_000_1,
            123.456,
            4_503_599_627_370_495.5, // 2^52 - 0.5
            4_503_599_627_370_496.0, // 2^52
            9_007_199_254_740_991.0,
            1.0e300,
            f64::MAX,
            f64::INFINITY,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            -f64::from_bits(1),
            -0.25,
            -0.5,
            -0.999_999_999_999,
            -1.0,
            -1.000_000_000_001,
            -1.5,
            -2.5,
            -123.456,
            -4_503_599_627_370_495.5,
            -4_503_599_627_370_496.0,
            -1.0e300,
            f64::MIN,
            f64::NEG_INFINITY,
        ];
        for x in values {
            let got = ceil_via_registry(&mut e, x);
            assert_eq!(
                got.to_bits(),
                x.ceil().to_bits(),
                "ceil({x:e}) = {got:e}, host {:e}",
                x.ceil()
            );
        }
    }

    #[test]
    fn ceil_sse2_of_a_nan_lets_the_error_support_rewrite_the_result_slot() {
        let mut e = Engine::new();
        ceil_setup(&mut e, true);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(LIBM_ERROR_SUPPORT, move |e, a| {
            seen.borrow_mut().push((LIBM_ERROR_SUPPORT, a.to_vec()));
            assert_eq!(a[0], a[1]);
            assert_eq!(a[1], a[2]);
            // The slot holds the argument; the support routine replaces it.
            assert_eq!(e.mem.u64(a[2]), 0x7ff0_0000_0000_0123);
            e.mem.set_u64(a[2], 0x7ff8_0000_0000_0777);
            Ret::default()
        });
        let nan = f64::from_bits(0x7ff0_0000_0000_0123); // signalling
        let got = ceil_via_registry(&mut e, nan);
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0777);
        let calls = calls_to(&log, LIBM_ERROR_SUPPORT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][3], 0x3ec);
        // A support routine that leaves the slot alone: the NaN comes back quieted by FLD.
        e.register(LIBM_ERROR_SUPPORT, |_, _| Ret::default());
        let got = ceil_via_registry(&mut e, nan);
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0123);
    }

    #[test]
    fn ceil_takes_the_x87_body_without_the_sse2_flag_or_with_unmasked_control_bits() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        let log = Log::default();
        // __frnd rounds up here (the round-up control word is loaded).
        let seen = log.clone();
        e.register_double(FRND, move |e, a| {
            seen.borrow_mut().push((FRND, a.to_vec()));
            assert_eq!(x87_control_word(e), 0x1b3f, "round up while it runs");
            st0(f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).ceil())
        });
        let got = ceil_via_registry(&mut e, 2.25);
        assert_eq!(got, 3.0);
        assert_eq!(
            x87_control_word(&mut e),
            0x027f,
            "the old control word is back"
        );
        assert_eq!(calls_to(&log, FRND), vec![words(2.25).to_vec()]);
        // SSE2 flag set but the control word's exception masks not all set: x87 body again.
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        ctrlfp(&mut e, 0x027e, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(ceil_via_registry(&mut e, -2.25), -2.0);
        assert_eq!(calls_to(&log, FRND).len(), 1);
    }

    #[test]
    fn ceil_x87_reports_an_inexact_result_only_when_the_precision_mask_is_clear() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        e.register(FRND, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).ceil())
        });
        let log = Log::default();
        recorder(&mut e, &log, EXCEPT1, st0(42.0));
        // Exact: no report, whatever the mask.
        ctrlfp(&mut e, 0x025f, 0xffff); // precision exception unmasked (bit 5 clear)
        assert_eq!(ceil_via_registry(&mut e, 3.0), 3.0);
        assert!(calls_to(&log, EXCEPT1).is_empty());
        // Inexact with the mask clear: __except1(0x10, 0xc, x, result, old control word).
        let got = ceil_via_registry(&mut e, 2.5);
        assert_eq!(got, 42.0, "the report's result is the result");
        let x = words(2.5);
        let r = words(3.0);
        assert_eq!(
            calls_to(&log, EXCEPT1),
            vec![vec![0x10, 0xc, x[0], x[1], r[0], r[1], 0x025f]]
        );
        // Inexact with the mask set: the rounded value, no report.
        ctrlfp(&mut e, 0x027f, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(ceil_via_registry(&mut e, 2.5), 3.0);
        assert!(calls_to(&log, EXCEPT1).is_empty());
    }

    #[test]
    fn ceil_x87_sorts_infinities_and_nans_by_sptype() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        e.map(ONE_DOUBLE, 8);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        let log = Log::default();
        let class = Rc::new(RefCell::new(0u32));
        let c = class.clone();
        e.register_double(SPTYPE, move |_, _| eax(*c.borrow()));
        recorder(&mut e, &log, HANDLE_QNAN1, st0(7.0));
        recorder(&mut e, &log, EXCEPT1, st0(9.0));
        let x = f64::from_bits(0x7ff0_0000_0000_0042); // a signalling NaN
                                                       // Classes 1 and 2: the value (quieted by FLD) and the control word restored.
        for value in [1u32, 2] {
            *class.borrow_mut() = value;
            let got = ceil_via_registry(&mut e, x);
            assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0042);
            assert_eq!(x87_control_word(&mut e), 0x027f);
        }
        // Class 3: __handle_qnan1(0xc, x quieted, old control word).
        *class.borrow_mut() = 3;
        ctrlfp(&mut e, 0x027f, 0xffff);
        assert_eq!(ceil_via_registry(&mut e, x), 7.0);
        let q = words(f64::from_bits(0x7ff8_0000_0000_0042));
        assert_eq!(
            calls_to(&log, HANDLE_QNAN1),
            vec![vec![0xc, q[0], q[1], 0x027f]]
        );
        // Other classes: __except1(8, 0xc, x, x + 1.0 (NaN quieted), old control word).
        for value in [0u32, 4, 0xffff_ffff] {
            *class.borrow_mut() = value;
            log.borrow_mut().clear();
            ctrlfp(&mut e, 0x027f, 0xffff);
            assert_eq!(ceil_via_registry(&mut e, x), 9.0);
            assert_eq!(
                calls_to(&log, EXCEPT1),
                vec![vec![8, 0xc, q[0], q[1], q[0], q[1], 0x027f]],
                "class {value:#x}"
            );
        }
        // An infinity of another kind: x + 1.0 is the infinity.
        *class.borrow_mut() = 0;
        log.borrow_mut().clear();
        ctrlfp(&mut e, 0x027f, 0xffff);
        ceil_via_registry(&mut e, f64::NEG_INFINITY);
        let inf = words(f64::NEG_INFINITY);
        assert_eq!(
            calls_to(&log, EXCEPT1),
            vec![vec![8, 0xc, inf[0], inf[1], inf[0], inf[1], 0x027f]]
        );
    }

    // ---- _CIasin ---------------------------------------------------------------------------

    #[test]
    fn ci_asin_jumps_to_the_sse2_body_when_available() {
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        let log = Log::default();
        recorder(&mut e, &log, ASIN_SSE2, st0(0.5));
        recorder(&mut e, &log, LOAD_FLAGS_HELPER, Ret::default());
        recorder(&mut e, &log, ASIN_X87, st0(0.25));
        assert_eq!(e.call(0x00ec_9650, &args![0.3f64]).f64(), 0.5);
        assert_eq!(calls_to(&log, ASIN_SSE2), vec![words(0.3).to_vec()]);
        assert!(calls_to(&log, ASIN_X87).is_empty());
    }

    #[test]
    fn ci_asin_runs_the_x87_body_with_zf_from_the_exponent() {
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        let log = Log::default();
        recorder(&mut e, &log, ASIN_SSE2, st0(0.5));
        recorder(&mut e, &log, LOAD_FLAGS_HELPER, eax(0x3fd0_0000));
        recorder(&mut e, &log, ASIN_X87, st0(0.25));
        assert_eq!(e.call(0x00ec_9650, &args![0.3f64]).f64(), 0.25);
        let w = words(0.3);
        assert_eq!(calls_to(&log, LOAD_FLAGS_HELPER), vec![vec![w[0], w[1]]]);
        assert_eq!(
            calls_to(&log, ASIN_X87),
            vec![vec![0, 0x3fd0_0000, w[0], w[1], w[0], w[1]]],
            "ZF clear, EAX from the helper, ST0, the stack double"
        );
        assert!(calls_to(&log, ASIN_SSE2).is_empty());
        // An infinity or NaN sets ZF.
        log.borrow_mut().clear();
        e.call(0x00ec_9650, &args![f64::NAN]);
        let w = words(f64::NAN);
        assert_eq!(
            calls_to(&log, ASIN_X87),
            vec![vec![1, 0x3fd0_0000, w[0], w[1], w[0], w[1]]]
        );
        // The SSE2 flag set but the control word's masks not: still the x87 body.
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        ctrlfp(&mut e, 0x0200, 0xffff);
        log.borrow_mut().clear();
        e.call(0x00ec_9650, &args![0.1f64]);
        assert_eq!(calls_to(&log, ASIN_X87).len(), 1);
        assert!(calls_to(&log, ASIN_SSE2).is_empty());
    }

    // ---- the shared square-root body -------------------------------------------------------

    fn sqrt_setup(e: &mut Engine, error_switch: u32) {
        e.map(MATH_ERROR_SWITCH, 4);
        e.set_global(MATH_ERROR_SWITCH, error_switch);
        e.map(INDEFINITE_NAN, 16);
        e.mem.set_u64(INDEFINITE_NAN, 0xc000_0000_0000_0000);
        e.mem.set_u16(INDEFINITE_NAN + 8, 0xffff);
    }

    fn sqrt_body(e: &mut Engine, zf: bool, x: f64) -> f64 {
        let w = words(x);
        e.call(0x00ec_605d, &args![zf, x, w[0], w[1]]).f64()
    }

    #[test]
    fn the_sqrt_body_takes_the_square_root_of_a_positive_value() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        assert_eq!(sqrt_body(&mut e, false, 6.25), 2.5);
        assert_eq!(sqrt_body(&mut e, false, 0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(
            sqrt_body(&mut e, false, f64::from_bits(1)),
            f64::from_bits(1).sqrt(),
            "a denormal"
        );
        // A positive infinity (ZF set, no fraction, sign clear) is returned unchanged.
        assert_eq!(sqrt_body(&mut e, true, f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn the_sqrt_body_keeps_minus_zero_and_flags_other_negatives_as_domain_errors() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        assert_eq!(
            sqrt_body(&mut e, false, -0.0).to_bits(),
            (-0.0f64).to_bits()
        );
        for x in [-1.0f64, -4.0e-320, -1.0e300] {
            assert_eq!(
                sqrt_body(&mut e, false, x).to_bits(),
                0xfff8_0000_0000_0000,
                "{x:e}: the indefinite NaN"
            );
        }
        assert_eq!(
            sqrt_body(&mut e, true, f64::NEG_INFINITY).to_bits(),
            0xfff8_0000_0000_0000
        );
    }

    #[test]
    fn the_sqrt_body_asks_the_nan_helper_for_the_type_and_value_of_a_nan() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(NAN_TYPE_HELPER, move |_, a| {
            seen.borrow_mut().push((NAN_TYPE_HELPER, a.to_vec()));
            Ret {
                eax: 1,
                st0: 3.5,
                ..Ret::default()
            }
        });
        let nan = f64::from_bits(0x7ff0_0000_0000_0007);
        // With the error switch set, the helper's ST0 is the result.
        assert_eq!(sqrt_body(&mut e, true, nan), 3.5);
        let w = words(nan);
        let st0_words = words(nan);
        assert_eq!(
            calls_to(&log, NAN_TYPE_HELPER),
            vec![vec![w[1], st0_words[0], st0_words[1]]],
            "EAX is the high word, then ST0"
        );
    }

    #[test]
    fn the_sqrt_body_ends_in_the_exit_or_the_error_handler_without_the_error_switch() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 0);
        let log = Log::default();
        recorder(&mut e, &log, MATH_EXIT, st0(11.0));
        recorder(&mut e, &log, ONE_ARG_ERROR_HANDLING, st0(13.0));
        // A result: the exit (operation 5, the name "sqrt", ST0, the saved control word, the
        // return address slot, the double).
        assert_eq!(sqrt_body(&mut e, false, 9.0), 11.0);
        let w = words(9.0);
        let r = words(3.0);
        assert_eq!(
            calls_to(&log, MATH_EXIT),
            vec![vec![5, SQRT_NAME, r[0], r[1], 0x027f, 0, w[0], w[1]]]
        );
        // A domain error: EAX = 1, the indefinite NaN in ST0.
        assert_eq!(sqrt_body(&mut e, false, -9.0), 13.0);
        let w = words(-9.0);
        let nan = words(f64::from_bits(0xfff8_0000_0000_0000));
        assert_eq!(
            calls_to(&log, ONE_ARG_ERROR_HANDLING),
            vec![vec![1, SQRT_NAME, 5, nan[0], nan[1], 0x027f, 0, w[0], w[1]]]
        );
    }

    #[test]
    fn the_sqrt_body_loads_a_masked_control_word_and_restores_the_old_one() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(SET_CONTROL_WORD_HELPER, move |e, a| {
            seen.borrow_mut()
                .push((SET_CONTROL_WORD_HELPER, a.to_vec()));
            // The real helper loads (old & 0x300) | 0x7f.
            set_x87_control_word(e, ((a[0] & 0x300) | 0x7f) as u16);
            Ret::default()
        });
        ctrlfp(&mut e, 0x0b7f, 0xffff);
        assert_eq!(sqrt_body(&mut e, false, 16.0), 4.0);
        assert_eq!(calls_to(&log, SET_CONTROL_WORD_HELPER), vec![vec![0x0b7f]]);
        assert_eq!(x87_control_word(&mut e), 0x0b7f, "restored on the way out");
        // With the usual control word nothing is loaded.
        ctrlfp(&mut e, 0x027f, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(sqrt_body(&mut e, false, 16.0), 4.0);
        assert!(calls_to(&log, SET_CONTROL_WORD_HELPER).is_empty());
        // The special path (ZF set) does not load it either.
        ctrlfp(&mut e, 0x037f, 0xffff);
        assert_eq!(sqrt_body(&mut e, true, f64::INFINITY), f64::INFINITY);
        assert!(calls_to(&log, SET_CONTROL_WORD_HELPER).is_empty());
        assert_eq!(x87_control_word(&mut e), 0x037f);
    }
}

#[cfg(test)]
mod strtol_tests {
    use super::*;

    /// The C locale's ctype bits for `c` (upper 1, lower 2, digit 4,
    /// space 8, hex 0x80).
    fn c_type(c: u8) -> u16 {
        let mut bits = 0;
        if c.is_ascii_uppercase() {
            bits |= 1;
        }
        if c.is_ascii_lowercase() {
            bits |= 2;
        }
        if c.is_ascii_digit() {
            bits |= 4;
        }
        if c == b' ' || (9..=13).contains(&c) {
            bits |= 8;
        }
        if c.is_ascii_hexdigit() {
            bits |= 0x80;
        }
        bits
    }

    const HANDLER: u32 = 0x0077_0000;

    /// An engine whose `_LocaleUpdate` double fills the object with a locale
    /// info block (`mb_cur_max` at +0xAC, the C ctype table at +0xC8), a
    /// thread block (`_ownlocale` 3 at +0x70) and the "updated" byte
    /// `updated`; `errno` falls back to its static slot and the invalid
    /// parameter handler is a double. Returns the engine and the thread
    /// block.
    fn engine(mb_cur_max: u32, updated: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        let table = e.mem.alloc(0x200);
        for c in 0..=255u8 {
            e.mem.set_u16(table + u32::from(c) * 2, c_type(c));
        }
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xac, mb_cur_max);
        e.mem.set_u32(info + 0xc8, table);
        let thread = e.mem.alloc(0x80);
        e.mem.set_u32(thread + 0x70, 3);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, updated);
            a[0].into_ret()
        });
        e.register(ISCTYPE_L, |e, a| {
            let info = e.mem.u32(a[2]);
            let table = e.mem.u32(info + 0xc8);
            (u32::from(e.mem.u16(table + a[0] * 2)) & a[1]).into_ret()
        });
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        e.register(HANDLER, |_, _| 0u32.into_ret());
        (e, thread)
    }

    /// `strtol(text, &end, base)`: the value, how many bytes it read and
    /// errno (0 when unset).
    fn parse(e: &mut Engine, text: &[u8], base: i32) -> (i32, u32, u32) {
        let s = e.mem.alloc(64);
        e.mem.write(s, text);
        e.mem.set_u8(s + text.len() as u32, 0);
        let end = e.mem.alloc(4);
        e.mem.set_u32(end, 0xdead_beef);
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        let value = e.call(0x00ec_8d08, &[s, end, base as u32]).u32() as i32;
        (value, e.mem.u32(end) - s, e.mem.u32(ERRNO_FALLBACK))
    }

    #[test]
    fn strtol_reads_bases_signs_prefixes_and_limits() {
        let (mut e, thread) = engine(1, 0);
        e.call_log = Some(vec![]);
        let mut p = |text: &[u8], base| parse(&mut e, text, base);
        assert_eq!(p(b"1F", 16), (0x1f, 2, 0));
        assert_eq!(p(b"0x1fz", 16), (0x1f, 4, 0));
        assert_eq!(p(b"  -42abc", 10), (-42, 5, 0));
        assert_eq!(p(b"\t+017", 0), (0o17, 5, 0));
        assert_eq!(p(b"0X10", 0), (16, 4, 0));
        assert_eq!(p(b"129", 0), (129, 3, 0));
        assert_eq!(p(b"0", 0), (0, 1, 0));
        assert_eq!(p(b"zz", 36), (35 * 36 + 35, 2, 0));
        assert_eq!(p(b"102", 2), (2, 2, 0));
        assert_eq!(p(b"7fffffff", 16), (i32::MAX, 8, 0));
        assert_eq!(p(b"-80000000", 16), (i32::MIN, 9, 0));
        // Out of range: ERANGE, the limit, and every digit still read.
        assert_eq!(p(b"80000000", 16), (i32::MAX, 8, 0x22));
        assert_eq!(p(b"-80000001", 16), (i32::MIN, 9, 0x22));
        assert_eq!(p(b"123456789012", 10), (i32::MAX, 12, 0x22));
        // No digits: 0 and the end at the start (also after "0x").
        assert_eq!(p(b"", 16), (0, 0, 0));
        assert_eq!(p(b" -x", 10), (0, 0, 0));
        assert_eq!(p(b"0x", 16), (0, 0, 0));
        // A bad base: EINVAL, 0, the end at the start.
        assert_eq!(p(b"12", 1), (0, 0, 0x16));
        assert_eq!(p(b"12", 37), (0, 0, 0x16));
        assert_eq!(p(b"12", -1), (0, 0, 0x16));
        assert_eq!(e.mem.u32(thread + 0x70), 3);
        let locales: Vec<u32> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == LOCALE_UPDATE)
            .map(|(_, w)| w[1])
            .collect();
        assert!(locales.iter().all(|l| *l == INITIAL_LOCALE));
    }

    #[test]
    fn strtoxl_paths_through_the_locale_and_flags() {
        // A multibyte locale asks __isctype_l; an updated locale is released;
        // a changed locale passes null.
        let (mut e, thread) = engine(2, 1);
        e.mem.set_u32(LOCALE_CHANGED, 1);
        e.call_log = Some(vec![]);
        assert_eq!(parse(&mut e, b"  77", 8), (0o77, 4, 0));
        let log = e.call_log.take().unwrap();
        let isctype = log.iter().filter(|(a, _)| *a == ISCTYPE_L).count();
        assert_eq!(isctype, 3);
        assert_eq!(
            log.iter().find(|(a, _)| *a == LOCALE_UPDATE).unwrap().1[1],
            0
        );
        assert_eq!(e.mem.u32(thread + 0x70), 1);
        // Unsigned (flag 1): no signed limit; overflow gives 0xFFFFFFFF.
        let s = e.mem.alloc(16);
        e.mem.write(s, b"ffffffff\0");
        assert_eq!(e.call(0x00ec_8ad9, &[0, s, 0, 16, 1]).u32(), u32::MAX);
        e.mem.write(s, b"100000000\0");
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        assert_eq!(e.call(0x00ec_8ad9, &[0, s, 0, 16, 1]).u32(), u32::MAX);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0x22);
        // A null text: EINVAL, the invalid parameter handler, 0.
        e.call_log = Some(vec![]);
        let end = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_8ad9, &[0, 0, end, 10, 0]).u32(), 0);
        assert_eq!(e.mem.u32(end), 0);
        let handler = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == HANDLER)
            .count();
        assert_eq!(handler, 1);
        // A null end pointer is allowed.
        e.mem.write(s, b"-9\0");
        assert_eq!(e.call(0x00ec_8d08, &[s, 0, 10]).u32() as i32, -9);
    }
}

#[cfg(test)]
mod thread_data_tests {
    use super::*;

    const FLS_GET_VALUE: u32 = 0x0077_0000;
    const FLS_SET_VALUE: u32 = 0x0077_0010;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with the platform's imports at the slots these functions
    /// call, the CRT globals' pages mapped and the thread block `block`
    /// (0: none) behind the FLS doubles; `FlsSetValue` succeeds when
    /// `store_ok`.
    fn engine(block: u32, store_ok: bool) -> Engine {
        let mut e = Engine::new();
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_GET_LAST_ERROR, "GetLastError"),
            (IMPORT_SET_LAST_ERROR, "SetLastError"),
            (IMPORT_GET_CURRENT_THREAD_ID, "GetCurrentThreadId"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        e.map(0x011c_1000, 0x2000);
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(FLS_INDEX, 3);
        e.register(SET_FLSGETVALUE, |_, _| FLS_GET_VALUE.into_ret());
        e.register_double(FLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [3]);
            block.into_ret()
        });
        e.register(DECODE_POINTER, |_, _| FLS_SET_VALUE.into_ret());
        e.register_double(FLS_SET_VALUE, move |_, _| u32::from(store_ok).into_ret());
        e.register(INIT_PTD, |_, _| 0u32.into_ret());
        e.register(AMSG_EXIT, |_, _| 0u32.into_ret());
        e.register(CRT_LOCK, |_, _| 0u32.into_ret());
        e.register(CRT_UNLOCK, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn getptd_returns_the_thread_block_and_keeps_the_last_error() {
        let mut e = engine(0x0055_0000, true);
        e.call(IMPORT_SET_LAST_ERROR, &[7]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_11de, &[]).u32(), 0x0055_0000);
        assert_eq!(calls_to(&e, IMPORT_SET_LAST_ERROR), vec![vec![7]]);
        assert!(calls_to(&e, FLS_SET_VALUE).is_empty());
        assert!(calls_to(&e, AMSG_EXIT).is_empty());
    }

    #[test]
    fn getptd_noexit_makes_a_block_for_a_new_thread() {
        let mut e = engine(0, true);
        e.call_log = Some(vec![]);
        let block = e.call(0x00ed_1165, &[]).u32();
        assert_ne!(block, 0);
        assert_eq!(calls_to(&e, FLS_SET_VALUE), vec![vec![3, block]]);
        assert_eq!(calls_to(&e, INIT_PTD), vec![vec![block, 0]]);
        assert_eq!(e.mem.u32(block), crate::units::platform::MAIN_THREAD_ID);
        assert_eq!(e.mem.u32(block + 4), u32::MAX);
        assert!(e.mem.bytes(block + 8, 0x20c).iter().all(|b| *b == 0));
        // The store fails: the block is freed, 0, and _getptd ends the program.
        let mut e = engine(0, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_11de, &[]).u32(), 0);
        assert!(calls_to(&e, INIT_PTD).is_empty());
        assert_eq!(calls_to(&e, AMSG_EXIT), vec![vec![0x10]]);
    }

    /// A thread block with `_ownlocale` `own` and the given locale and
    /// multibyte info.
    fn thread(e: &mut Engine, own: u32, info: u32, mbc: u32) -> u32 {
        let block = e.mem.alloc(0x214);
        e.mem.set_u32(block + 0x70, own);
        e.mem.set_u32(block + 0x6c, info);
        e.mem.set_u32(block + 0x68, mbc);
        block
    }

    #[test]
    fn locale_update_copies_a_locale_or_takes_the_threads() {
        let mut e = engine(0, true);
        let locale = e.mem.alloc(8);
        e.mem.set_u32(locale, 0x11);
        e.mem.set_u32(locale + 4, 0x22);
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 0xc, 9);
        assert_eq!(e.call(0x00ec_660e, &[this, locale]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x11);
        assert_eq!(e.mem.u32(this + 4), 0x22);
        assert_eq!(e.mem.u8(this + 0xc), 0);

        // The thread's globals: nothing to update; bit 1 set and noted.
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_INFO, 0x100);
        e.mem.set_u32(GLOBAL_MBC_INFO, 0x200);
        let block = thread(&mut e, 0, 0x100, 0x200);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(e.mem.u32(this), 0x100);
        assert_eq!(e.mem.u32(this + 4), 0x200);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(block + 0x70), 2);
        assert!(calls_to(&e, CRT_LOCK).is_empty());
        // Bit 1 already set: not noted again.
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(e.mem.u8(this + 0xc), 0);
    }

    #[test]
    fn locale_update_refreshes_stale_thread_locales() {
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_INFO, 0x100);
        let global_mbc = e.mem.alloc(4);
        e.mem.set_u32(global_mbc, 1);
        e.mem.set_u32(GLOBAL_MBC_INFO, global_mbc);
        let old_mbc = e.mem.alloc(4);
        e.mem.set_u32(old_mbc, 1);
        let block = thread(&mut e, 0, 0x300, old_mbc);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        e.register(UPDATE_TLOCINFO_EX_NOLOCK, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[1].into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(
            calls_to(&e, UPDATE_TLOCINFO_EX_NOLOCK),
            vec![vec![block + 0x6c, 0x100]]
        );
        assert_eq!(e.mem.u32(this), 0x100);
        // The old multibyte info's count fell to zero; the global's rose.
        assert_eq!(e.mem.u32(this + 4), global_mbc);
        assert_eq!(e.mem.u32(block + 0x68), global_mbc);
        assert_eq!(e.mem.u32(global_mbc), 2);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![old_mbc]]
        );
        assert_eq!(calls_to(&e, CRT_LOCK), vec![vec![0xc], vec![0xd]]);
        assert_eq!(calls_to(&e, CRT_UNLOCK), vec![vec![0xc], vec![0xd]]);
    }

    #[test]
    fn update_functions_keep_a_threads_own_locale_and_stop_on_null() {
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_STATUS, 1);
        let block = thread(&mut e, 1, 0x400, 0x500);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f742, &[]).u32(), 0x400);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), 0x500);
        assert!(calls_to(&e, CRT_LOCK).is_empty());
        assert_eq!(calls_to(&e, SET_FLSGETVALUE).len(), 3);
        // Not kept, and the update gives null: _amsg_exit(0x20).
        e.mem.set_u32(GLOBAL_LOCALE_STATUS, 0);
        e.register(UPDATE_TLOCINFO_EX_NOLOCK, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f742, &[]).u32(), 0);
        assert_eq!(calls_to(&e, AMSG_EXIT), vec![vec![0x20]]);
        // The multibyte info equal to the global: kept as it is.
        e.mem.set_u32(GLOBAL_MBC_INFO, 0x500);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), 0x500);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // The static initial info is not freed when its count reaches 0.
        e.mem.set_u32(INITIAL_MBC_INFO, 1);
        e.mem.set_u32(block + 0x68, INITIAL_MBC_INFO);
        let global = e.mem.alloc(4);
        e.mem.set_u32(GLOBAL_MBC_INFO, global);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), global);
        assert_eq!(e.mem.u32(INITIAL_MBC_INFO), 0);
    }

    /// A locale object (`_locale_tstruct`) whose locale info has a ctype
    /// table where `A` is upper case (1) and 0x81 a lead byte (0x8000), and
    /// code page 932 at +4, locale id 0x411 at +0x14.
    fn locale(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x202);
        e.mem.set_u16(table + 2 * 0x41, 1);
        e.mem.set_u16(table + 2 * 0x81, 0x8000);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xc8, table);
        e.mem.set_u32(info + 4, 932);
        e.mem.set_u32(info + 0x14, 0x411);
        let locale = e.mem.alloc(8);
        e.mem.set_u32(locale, info);
        locale
    }

    /// The byte strings the string-type double received.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>;

    #[test]
    fn character_types_read_the_table_or_ask_the_string_types() {
        let mut e = engine(0, true);
        let l = locale(&mut e);
        assert_eq!(e.call(0x00ed_419c, &[0x181, l]).u32(), 0x8000);
        assert_eq!(e.call(0x00ed_419c, &[0x41, l]).u32(), 0);
        assert_eq!(e.call(0x00ed_41e7, &[0x41, 1, l]).u32(), 1);
        assert_eq!(e.call(0x00ed_41e7, &[0x41, 2, l]).u32(), 0);
        // Above 255: the string types of one or two bytes.
        let seen: Seen = Default::default();
        let record = seen.clone();
        e.register_double(GET_STRING_TYPE_A_STAT, move |e, a| {
            assert_eq!(a[1], 1);
            assert_eq!((a[5], a[6], a[7]), (932, 0x411, 1));
            record.borrow_mut().push(e.mem.bytes(a[2], a[3]));
            e.mem.set_u16(a[4], 0x0104);
            u32::from(a[3] == 2).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_41e7, &[0x8142, 0xffff, l]).u32(), 0x0104);
        assert_eq!(e.call(0x00ed_41e7, &[0x4142, 0xffff, l]).u32(), 0);
        assert_eq!(*seen.borrow(), vec![vec![0x81, 0x42], vec![0x42]]);
        assert_eq!(calls_to(&e, GET_STRING_TYPE_A_STAT).len(), 2);
        // __crtGetStringTypeA passes the locale object and its seven words.
        let text = e.mem.alloc(4);
        e.mem.write(text, &[0x81, 0x42]);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ee_13a3, &[l, 1, text, 2, out, 932, 0x411, 1])
            .u32();
        assert_eq!(result, 1);
        let call = &calls_to(&e, GET_STRING_TYPE_A_STAT)[0];
        assert_eq!(&call[1..], &[1, text, 2, out, 932, 0x411, 1]);
    }
}

#[cfg(test)]
mod thread_start_up_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const MODULE: u32 = 0x7700_0000;
    const FLS_ALLOC: u32 = 0x0077_1000;
    const FLS_GET: u32 = 0x0077_1010;
    const FLS_SET: u32 = 0x0077_1020;
    const FLS_FREE: u32 = 0x0077_1030;
    const ENCODE: u32 = 0x0077_1040;
    const DECODE: u32 = 0x0077_1050;
    const EXIT: u32 = 0x0077_1060;
    const KEY: u32 = 0x5a5a_5a5a;

    /// Slot values of the doubles' TLS and FLS.
    type Slots = Rc<RefCell<HashMap<u32, u32>>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with doubles for the imports `__mtinit`'s chain calls:
    /// `KERNEL32.DLL` at `MODULE` whose `GetProcAddress` knows the FLS
    /// functions (when `fls`) and `EncodePointer`/`DecodePointer` (XOR with
    /// `KEY`); TLS and FLS slots kept in maps; critical sections that
    /// initialize when `sections_ok`. Lock entries 0xA, 0xC and 0xD are of
    /// kind 1. Returns the engine and the TLS and FLS slots.
    fn engine(fls: bool, sections_ok: bool) -> (Engine, Slots, Slots) {
        let mut e = Engine::new();
        for page in [
            0x00fd_f000,
            0x0101_1000,
            0x011c_1000,
            0x011c_2000,
            0x0126_f000,
            0x0127_0000,
        ] {
            e.map(page, 0x1000);
        }
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_GET_LAST_ERROR, "GetLastError"),
            (IMPORT_SET_LAST_ERROR, "SetLastError"),
            (IMPORT_GET_CURRENT_THREAD_ID, "GetCurrentThreadId"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        e.register(IMPORT_GET_MODULE_HANDLE_W, |_, _| MODULE.into_ret());
        e.register_double(IMPORT_GET_PROC_ADDRESS, move |_, a| {
            assert_eq!(a[0], MODULE);
            let found = match a[1] {
                FLS_ALLOC_NAME => FLS_ALLOC,
                FLS_GET_VALUE_NAME => FLS_GET,
                FLS_SET_VALUE_NAME => FLS_SET,
                FLS_FREE_NAME => FLS_FREE,
                ENCODE_POINTER_NAME => return ENCODE.into_ret(),
                DECODE_POINTER_NAME => return DECODE.into_ret(),
                other => panic!("GetProcAddress({other:08x})"),
            };
            (if fls { found } else { 0 }).into_ret()
        });
        e.register(ENCODE, |_, a| (a[0] ^ KEY).into_ret());
        e.register(DECODE, |_, a| (a[0] ^ KEY).into_ret());
        let tls: Slots = Rc::default();
        let fls_slots: Slots = Rc::default();
        e.register(IMPORT_TLS_ALLOC, |_, _| 5u32.into_ret());
        let slots = tls.clone();
        e.register_double(IMPORT_TLS_GET_VALUE, move |_, a| {
            slots.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let slots = tls.clone();
        e.register_double(IMPORT_TLS_SET_VALUE, move |_, a| {
            slots.borrow_mut().insert(a[0], a[1]);
            1u32.into_ret()
        });
        e.register(IMPORT_TLS_FREE, |_, _| 1u32.into_ret());
        e.register(FLS_ALLOC, |_, a| {
            assert_eq!(a, [FREE_FLS]);
            7u32.into_ret()
        });
        let slots = fls_slots.clone();
        e.register_double(FLS_GET, move |_, a| {
            slots.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let slots = fls_slots.clone();
        e.register_double(FLS_SET, move |_, a| {
            slots.borrow_mut().insert(a[0], a[1]);
            1u32.into_ret()
        });
        e.register(FLS_FREE, |_, _| 1u32.into_ret());
        e.register(INITP_00EC7AF9, |_, _| 0u32.into_ret());
        e.register_double(IMPORT_INIT_CRITICAL_SECTION_SPIN, move |_, _| {
            u32::from(sections_ok).into_ret()
        });
        e.register(IMPORT_ENTER_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(IMPORT_LEAVE_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(ADD_LOCALE_REF, |_, _| 0u32.into_ret());
        e.register(MT_DELETE_LOCKS, |_, _| 0u32.into_ret());
        e.register(IMPORT_SLEEP, |_, _| 0u32.into_ret());
        e.register(FF_MSGBANNER, |_, _| 0u32.into_ret());
        e.register(NMSG_WRITE, |_, _| 0u32.into_ret());
        e.register(EXIT, |_, _| 0u32.into_ret());
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        for number in [0xa, 0xc, 0xd] {
            e.mem.set_u32(LOCK_TABLE + number * 8 + 4, 1);
        }
        e.mem.set_u32(LOCKS_READY, 1);
        e.mem.set_u32(FLS_INDEX, u32::MAX);
        e.mem.set_u32(TLS_INDEX, u32::MAX);
        (e, tls, fls_slots)
    }

    #[test]
    fn mtinit_sets_up_fls_locks_and_the_first_thread_block() {
        let (mut e, tls, fls) = engine(true, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 1);
        assert_eq!(e.mem.u32(TLS_INDEX), 5);
        assert_eq!(tls.borrow()[&5], FLS_GET);
        assert_eq!(e.mem.u32(FLS_INDEX), 7);
        // The four FLS pointers are kept encoded.
        assert_eq!(e.mem.u32(FLS_ALLOC_POINTER), FLS_ALLOC ^ KEY);
        assert_eq!(e.mem.u32(FLS_GET_VALUE_POINTER), FLS_GET ^ KEY);
        assert_eq!(e.mem.u32(FLS_SET_VALUE_POINTER), FLS_SET ^ KEY);
        assert_eq!(e.mem.u32(FLS_FREE_POINTER), FLS_FREE ^ KEY);
        // __init_pointers: the encoded null everywhere, the exit routine.
        assert_eq!(e.mem.u32(0x0127_072c), KEY);
        assert_eq!(e.mem.u32(0x0127_071c), KEY);
        assert_eq!(e.mem.u32(0x0126_fe10), 0x00ed_14b4 ^ KEY);
        assert_eq!(e.mem.u32(ENCODED_EXIT), 0x00ec_cedb ^ KEY);
        assert_eq!(calls_to(&e, INITP_00EC7AF9), vec![vec![KEY]]);
        // The locks of kind 1 got the static sections in order.
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xa * 8), STATIC_LOCKS);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xc * 8), STATIC_LOCKS + 0x18);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xd * 8), STATIC_LOCKS + 0x30);
        // The thread block.
        let block = fls.borrow()[&7];
        assert_eq!(e.mem.u32(block), crate::units::platform::MAIN_THREAD_ID);
        assert_eq!(e.mem.u32(block + 4), u32::MAX);
        assert_eq!(e.mem.u32(block + 0x5c), 0x010f_dca0);
        assert_eq!(e.mem.u32(block + 0x14), 1);
        assert_eq!(e.mem.u32(block + 0x1f8), ENCODE);
        assert_eq!(e.mem.u32(block + 0x1fc), DECODE);
        assert_eq!(e.mem.u32(block + 0x70), 1);
        assert_eq!(e.mem.u8(block + 0xc8), 0x43);
        assert_eq!(e.mem.u8(block + 0x14b), 0x43);
        assert_eq!(e.mem.u32(block + 0x68), INITIAL_MBC_INFO);
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![STATIC_LOCKS + 0x30], vec![STATIC_LOCKS + 0x18]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![STATIC_LOCKS + 0x30], vec![STATIC_LOCKS + 0x18]]
        );
        assert!(calls_to(&e, MT_DELETE_LOCKS).is_empty());
        // Afterwards the pointers decode through the thread block.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_0f92, &[0x1234 ^ KEY]).u32(), 0x1234);
        assert!(calls_to(&e, IMPORT_GET_PROC_ADDRESS).is_empty());
        assert_eq!(e.call(0x00ed_100d, &[]).u32(), FLS_GET);
    }

    #[test]
    fn mtinit_falls_back_to_tls_and_stops_on_failures() {
        let (mut e, tls, _) = engine(false, true);
        e.mem.set_u32(IMPORT_TLS_GET_VALUE, 0x0077_2044);
        e.mem.set_u32(IMPORT_TLS_SET_VALUE, 0x0077_204c);
        e.mem.set_u32(IMPORT_TLS_FREE, 0x0077_2050);
        // The FLS stand-ins: __crtTlsAlloc, then the import table's words.
        e.register(0x0077_204c, |_, _| 1u32.into_ret());
        e.register(0x0077_2044, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 1);
        assert_eq!(e.mem.u32(FLS_ALLOC_POINTER), CRT_TLS_ALLOC ^ KEY);
        assert_eq!(e.mem.u32(FLS_GET_VALUE_POINTER), 0x0077_2044 ^ KEY);
        assert_eq!(e.mem.u32(FLS_SET_VALUE_POINTER), 0x0077_204c ^ KEY);
        assert_eq!(e.mem.u32(FLS_FREE_POINTER), 0x0077_2050 ^ KEY);
        assert_eq!(tls.borrow()[&5], 0x0077_2044);
        // __crtTlsAlloc gave the FLS slot through TlsAlloc.
        assert_eq!(e.mem.u32(FLS_INDEX), 5);
        assert_eq!(calls_to(&e, 0x0077_204c).len(), 1);

        // TlsAlloc fails: 0 without __mtterm.
        let (mut e, _, _) = engine(true, true);
        e.register(IMPORT_TLS_ALLOC, |_, _| u32::MAX.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 0);
        assert!(calls_to(&e, MT_DELETE_LOCKS).is_empty());

        // A lock that cannot be initialized: __mtterm frees the TLS slot.
        let (mut e, _, _) = engine(true, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 0);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xa * 8), 0);
        assert_eq!(calls_to(&e, IMPORT_TLS_FREE), vec![vec![5]]);
        assert_eq!(e.mem.u32(TLS_INDEX), u32::MAX);
        assert_eq!(calls_to(&e, MT_DELETE_LOCKS).len(), 1);
    }

    #[test]
    fn mtterm_frees_the_slots_that_are_set() {
        let (mut e, _, _) = engine(true, true);
        e.mem.set_u32(FLS_INDEX, 7);
        e.mem.set_u32(TLS_INDEX, 5);
        e.mem.set_u32(FLS_FREE_POINTER, FLS_FREE ^ KEY);
        e.call_log = Some(vec![]);
        e.call(0x00ed_1041, &[]);
        assert_eq!(calls_to(&e, FLS_FREE), vec![vec![7]]);
        assert_eq!(calls_to(&e, IMPORT_TLS_FREE), vec![vec![5]]);
        assert_eq!(e.mem.u32(FLS_INDEX), u32::MAX);
        assert_eq!(e.mem.u32(TLS_INDEX), u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x00ed_1041, &[]);
        assert!(calls_to(&e, FLS_FREE).is_empty());
        assert!(calls_to(&e, IMPORT_TLS_FREE).is_empty());
        assert_eq!(calls_to(&e, MT_DELETE_LOCKS).len(), 1);
    }

    #[test]
    fn locks_are_made_on_first_use_and_failures_exit() {
        let (mut e, _, _) = engine(true, true);
        // Lock 0xA (which __mtinitlocks sets up) guards the set-up.
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0100);
        e.call_log = Some(vec![]);
        e.call(0x00ed_df39, &[3]);
        let section = e.mem.u32(LOCK_TABLE + 3 * 8);
        assert_ne!(section, 0);
        assert_eq!(
            calls_to(&e, IMPORT_INIT_CRITICAL_SECTION_SPIN),
            vec![vec![section, 0xfa0]]
        );
        let lock_a = 0x0055_0100;
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![lock_a], vec![section]]
        );
        e.call(0x00ed_de5f, &[3]);
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION).last().unwrap(),
            &vec![section]
        );
        // Initialization fails: ENOMEM and _amsg_exit(0x11) through the
        // decoded exit routine.
        let (mut e, _, _) = engine(true, false);
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0000);
        e.mem.set_u32(ENCODED_EXIT, EXIT ^ KEY);
        e.call_log = Some(vec![]);
        e.call(0x00ed_df39, &[4]);
        assert_eq!(e.mem.u32(LOCK_TABLE + 4 * 8), 0);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
        assert_eq!(calls_to(&e, NMSG_WRITE), vec![vec![0x11]]);
        assert_eq!(calls_to(&e, EXIT), vec![vec![0xff]]);
        // Too early in start-up: banner, message 0x1E, exit 0xFF.
        let (mut e, _, _) = engine(true, true);
        e.mem.set_u32(LOCKS_READY, 0);
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0100);
        e.register(0x00ec_cca9, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ed_de76, &[6]);
        assert_eq!(calls_to(&e, NMSG_WRITE), vec![vec![0x1e]]);
        assert_eq!(calls_to(&e, 0x00ec_cca9), vec![vec![0xff]]);
    }

    #[test]
    fn waiting_for_a_module_sleeps_longer_each_time() {
        let (mut e, _, _) = engine(true, true);
        e.register(IMPORT_GET_MODULE_HANDLE_W, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_cc25, &[KERNEL32_NAME]).u32(), 0);
        let sleeps = calls_to(&e, IMPORT_SLEEP);
        assert_eq!(sleeps.len(), 60);
        assert_eq!(sleeps[0], vec![1000]);
        assert_eq!(sleeps[59], vec![60000]);
        // Without the module the pointers pass unchanged.
        assert_eq!(e.call(0x00ed_0f17, &[0x99]).u32(), 0x99);
    }

    #[test]
    fn locale_info_update_swaps_references() {
        let (mut e, _, _) = engine(true, true);
        e.register(REMOVE_LOCALE_REF, |e, a| {
            e.mem.set_u32(a[0], 0);
            0u32.into_ret()
        });
        e.register(FREE_TLOCINFO, |_, _| 0u32.into_ret());
        let slot = e.mem.alloc(4);
        let old = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0x0066_0000]).u32(), 0x0066_0000);
        assert_eq!(e.mem.u32(slot), 0x0066_0000);
        assert_eq!(calls_to(&e, ADD_LOCALE_REF), vec![vec![0x0066_0000]]);
        assert_eq!(calls_to(&e, REMOVE_LOCALE_REF), vec![vec![old]]);
        assert_eq!(calls_to(&e, FREE_TLOCINFO), vec![vec![old]]);
        // The same info, or a null argument: nothing changes.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0x0066_0000]).u32(), 0x0066_0000);
        assert_eq!(e.call(0x00ec_f704, &[0, 0x0066_0000]).u32(), 0);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0]).u32(), 0);
        assert!(calls_to(&e, ADD_LOCALE_REF).is_empty());
    }

    /// The locale object for the string types: code page 1252 at info +4,
    /// locale id 0x409 at +0x14.
    fn string_locale(e: &mut Engine) -> u32 {
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 1252);
        e.mem.set_u32(info + 0x14, 0x409);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, info);
        object
    }

    #[test]
    fn string_types_go_through_the_wide_api_when_it_works() {
        let (mut e, _, _) = engine(true, true);
        let locale = string_locale(&mut e);
        e.register(IMPORT_GET_STRING_TYPE_W, |e, a| {
            if a[1] != EMPTY_WIDE_STRING {
                assert_eq!(e.mem.u16(a[1]), u16::from(b'h'));
                e.mem.set_u16(a[3], 0x0102);
            }
            1u32.into_ret()
        });
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            a[3].into_ret()
        });
        e.register(FREEA, |_, _| 0u32.into_ret());
        let text = e.mem.alloc(4);
        e.mem.write(text, b"hi");
        let types = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let args = [locale, 1, text, 2, types, 0, 0, 1];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 1);
        assert_eq!(e.mem.u32(STRING_TYPE_MODE), 1);
        assert_eq!(e.mem.u16(types), 0x0102);
        let conversions = calls_to(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR);
        assert_eq!(conversions[0], vec![1252, 9, text, 2, 0, 0]);
        assert_eq!(conversions[1][..4], [1252, 1, text, 2]);
        assert_eq!(calls_to(&e, FREEA).len(), 1);
        // A long text takes a _malloc'd buffer (marked 0xDDDD).
        let long = e.mem.alloc(0x300);
        e.mem.write(long, &[b'h'; 0x300]);
        e.register(MALLOC_BODY, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let args = [locale, 1, long, 0x300, types, 0, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 1);
        let block = calls_to(&e, MALLOC_BODY);
        assert_eq!(block, vec![vec![0x300 * 2 + 8]]);
        let freed = calls_to(&e, FREEA)[0][0];
        assert_eq!(e.mem.u32(freed - 8), 0xdddd);
        // A failed conversion gives 0.
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00ee_11e9, &[locale, 1, text, 2, types, 0, 0, 0])
                .u32(),
            0
        );
    }

    #[test]
    fn string_types_fall_back_to_the_ansi_api() {
        let (mut e, _, _) = engine(true, true);
        let locale = string_locale(&mut e);
        e.register(IMPORT_GET_STRING_TYPE_W, |e, _| {
            e.call(IMPORT_SET_LAST_ERROR, &[0x78]);
            0u32.into_ret()
        });
        e.register(ANSI_CODE_PAGE, |_, a| {
            assert_eq!(a[0], 0x409);
            1250u32.into_ret()
        });
        e.register(CONVERT_CODE_PAGE, |e, a| {
            assert_eq!(a[..2], [1252, 1250]);
            e.mem.set_u32(a[3], 5);
            e.mem.alloc(8).into_ret()
        });
        e.register(IMPORT_GET_STRING_TYPE_A, |_, _| 7u32.into_ret());
        let types = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let args = [locale, 1, 0x0055_0000, 3, types, 0, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 7);
        assert_eq!(e.mem.u32(STRING_TYPE_MODE), 2);
        let converted = calls_to(&e, CONVERT_CODE_PAGE);
        let ansi = calls_to(&e, IMPORT_GET_STRING_TYPE_A);
        assert_eq!(ansi.len(), 1);
        assert_eq!(ansi[0][0], 0x409);
        assert_eq!(ansi[0][3], 5);
        assert_ne!(ansi[0][2], 0x0055_0000);
        assert_eq!(converted.len(), 1);
        // The same code page: no conversion; an unknown locale: 0.
        e.call_log = Some(vec![]);
        let args = [locale, 1, 0x0055_0000, 3, types, 1250, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 7);
        assert!(calls_to(&e, CONVERT_CODE_PAGE).is_empty());
        assert_eq!(calls_to(&e, IMPORT_GET_STRING_TYPE_A)[0][2], 0x0055_0000);
        e.register(ANSI_CODE_PAGE, |_, _| u32::MAX.into_ret());
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 0);
    }
}

#[cfg(test)]
mod runtime_support_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn freed(e: &Engine, block: u32) -> bool {
        e.mem.block_size(block).is_none()
    }

    /// An engine with the CRT's data pages mapped, the Interlocked imports
    /// at their slots and doubles for the section imports.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x00fd_f000,
            0x0107_2000,
            0x0109_5000,
            0x010f_3000,
            0x011c_1000,
            0x011c_2000,
            0x0126_f000,
            0x0127_0000,
        ] {
            e.map(page, 0x1000);
        }
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        for slot in [
            IMPORT_DELETE_CRITICAL_SECTION,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
        ] {
            e.register(slot, |_, _| 0u32.into_ret());
        }
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn mtdeletelocks_frees_dynamic_sections_and_deletes_static_ones() {
        let mut e = engine();
        let dynamic = e.mem.alloc(0x18);
        e.mem.set_u32(LOCK_TABLE + 3 * 8, dynamic);
        e.mem.set_u32(LOCK_TABLE + 5 * 8, STATIC_LOCKS);
        e.mem.set_u32(LOCK_TABLE + 5 * 8 + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ed_de08, &[]);
        assert_eq!(
            calls_to(&e, IMPORT_DELETE_CRITICAL_SECTION),
            vec![vec![dynamic], vec![STATIC_LOCKS]]
        );
        assert!(freed(&e, dynamic));
        assert_eq!(e.mem.u32(LOCK_TABLE + 3 * 8), 0);
        assert_eq!(e.mem.u32(LOCK_TABLE + 5 * 8), STATIC_LOCKS);
    }

    /// The message table with 0xFC, 0xFF and 0x1E, their texts in the heap.
    fn messages(e: &mut Engine) -> [u32; 3] {
        let mut texts = [0; 3];
        for (i, (number, text)) in [
            (0xfcu32, &b"\r\n\0"[..]),
            (0xff, b"banner\0"),
            (0x1e, b"R6030\0"),
        ]
        .into_iter()
        .enumerate()
        {
            let s = e.mem.alloc(16);
            e.mem.write(s, text);
            e.mem.set_u32(RUNTIME_MESSAGES + i as u32 * 8, number);
            e.mem.set_u32(RUNTIME_MESSAGES + i as u32 * 8 + 4, s);
            texts[i] = s;
        }
        for i in 3..0x17 {
            e.mem.set_u32(RUNTIME_MESSAGES + i * 8, 0xdead);
        }
        texts
    }

    #[test]
    fn console_messages_go_to_standard_error() {
        let mut e = engine();
        let texts = messages(&mut e);
        e.register(SET_ERROR_MODE, |_, _| 1u32.into_ret());
        e.register(IMPORT_GET_STD_HANDLE, |_, a| {
            assert_eq!(a[0], (-12i32) as u32);
            0x30u32.into_ret()
        });
        e.register(IMPORT_WRITE_FILE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ed_2ab7, &[]);
        let writes: Vec<(u32, u32, u32)> = calls_to(&e, IMPORT_WRITE_FILE)
            .iter()
            .map(|w| (w[0], w[1], w[2]))
            .collect();
        assert_eq!(writes, vec![(0x30, texts[0], 2), (0x30, texts[1], 6)]);
        // A number outside the table writes nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ed_290c, &[0x1234]);
        assert!(calls_to(&e, SET_ERROR_MODE).is_empty());
        // No console and the default mode with a GUI app: no banner.
        e.register(SET_ERROR_MODE, |_, _| 0u32.into_ret());
        e.mem.set_u32(APP_TYPE, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ed_2ab7, &[]);
        assert!(calls_to(&e, IMPORT_WRITE_FILE).is_empty());
    }

    #[test]
    fn window_messages_build_the_runtime_error_box() {
        let mut e = engine();
        messages(&mut e);
        for (at, text) in [
            (MESSAGE_HEADER, &b"Runtime Error!\n\nProgram: \0"[..]),
            (MESSAGE_UNKNOWN_PROGRAM, b"<program name unknown>\0"),
            (MESSAGE_ELLIPSIS, b"...\0"),
            (MESSAGE_BREAK, b"\n\n\0"),
            (MESSAGE_CAPTION, b"Microsoft Visual C++ Runtime Library\0"),
        ] {
            e.mem.write(at, text);
        }
        e.register(SET_ERROR_MODE, |_, _| 2u32.into_ret());
        let name = "C:\\Games\\Fallout New Vegas\\a rather long folder name\\FalloutNV.exe";
        e.register(IMPORT_GET_MODULE_FILE_NAME_A, |e, a| {
            let name = "C:\\Games\\Fallout New Vegas\\a rather long folder name\\FalloutNV.exe";
            e.mem.write(a[1], name.as_bytes());
            e.mem.set_u8(a[1] + name.len() as u32, 0);
            (name.len() as u32).into_ret()
        });
        let shown = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let record = shown.clone();
        e.register_double(CRT_MESSAGE_BOX_A, move |e, a| {
            assert_eq!(a[1], MESSAGE_CAPTION);
            assert_eq!(a[2], 0x12010);
            *record.borrow_mut() = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            1u32.into_ret()
        });
        e.call(0x00ed_290c, &[0x1e]);
        let kept = &name[..name.len() - 0x3b];
        assert_eq!(
            *shown.borrow(),
            format!("Runtime Error!\n\nProgram: {kept}...\n\nR6030")
        );
        // An unknown program name, and 0xFC shows nothing.
        e.register(IMPORT_GET_MODULE_FILE_NAME_A, |_, _| 0u32.into_ret());
        e.call(0x00ed_290c, &[0x1e]);
        assert_eq!(
            *shown.borrow(),
            "Runtime Error!\n\nProgram: <program name unknown>\n\nR6030"
        );
        shown.borrow_mut().clear();
        e.call(0x00ed_290c, &[0xfc]);
        assert!(shown.borrow().is_empty());
    }

    #[test]
    fn exit_quick_is_doexit_quick() {
        let mut e = engine();
        e.register(DOEXIT_ADDR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ec_cedb, &[3]);
        assert_eq!(calls_to(&e, DOEXIT_ADDR), vec![vec![3, 1, 0]]);
    }

    /// A locale info with every count at 1 (category 2 uses the C names and
    /// category 4 has no wide name); returns it and its count words.
    fn locale_info(e: &mut Engine) -> (u32, Vec<u32>) {
        let info = e.mem.alloc(0xd8);
        let mut counts = vec![info];
        e.mem.set_u32(info, 1);
        let count = |e: &mut Engine| {
            let c = e.mem.alloc(4);
            e.mem.set_u32(c, 1);
            c
        };
        for offset in [0xb0, 0xb8, 0xb4, 0xc0] {
            let c = count(e);
            e.mem.set_u32(info + offset, c);
            counts.push(c);
        }
        for category in 0..6u32 {
            let at = info + 0x50 + category * 0x10;
            e.mem
                .set_u32(at - 8, if category == 2 { C_LOCALE_NAMES } else { 0 });
            let name = count(e);
            e.mem.set_u32(at, name);
            e.mem.set_u32(at - 4, u32::from(category != 4));
            let wide = count(e);
            e.mem.set_u32(at + 4, wide);
            if category != 2 {
                counts.push(name);
            }
            if category != 4 {
                counts.push(wide);
            }
        }
        let time = e.mem.alloc(0xb8);
        e.mem.set_u32(time + 0xb4, 1);
        e.mem.set_u32(info + 0xd4, time);
        counts.push(time + 0xb4);
        (info, counts)
    }

    #[test]
    fn locale_references_go_up_and_down_together() {
        let mut e = engine();
        let (info, counts) = locale_info(&mut e);
        e.call(0x00ec_f5b6, &[info]);
        assert!(counts.iter().all(|c| e.mem.u32(*c) == 2));
        assert_eq!(e.call(0x00ec_f645, &[info]).u32(), info);
        assert!(counts.iter().all(|c| e.mem.u32(*c) == 1));
        // The skipped names keep their counts.
        let c_names = e.mem.u32(info + 0x50 + 2 * 0x10);
        assert_eq!(e.mem.u32(c_names), 1);
        assert_eq!(e.call(0x00ec_f645, &[0]).u32(), 0);
    }

    #[test]
    fn freetlocinfo_frees_the_unreferenced_pieces() {
        let mut e = engine();
        e.register(FREE_LCONV_MONETARY, |_, _| 0u32.into_ret());
        e.register(FREE_LCONV_NUMERIC, |_, _| 0u32.into_ret());
        e.register(FREE_LC_TIME, |_, _| 0u32.into_ret());
        let (info, counts) = locale_info(&mut e);
        for c in &counts {
            e.mem.set_u32(*c, 0);
        }
        let lconv = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0xbc, lconv);
        let tables: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x200)).collect();
        e.mem.set_u32(info + 0xc4, tables[0] + 0xfe);
        e.mem.set_u32(info + 0xcc, tables[1] + 0x80);
        e.mem.set_u32(info + 0xd0, tables[2] + 0x80);
        let time = e.mem.u32(info + 0xd4);
        let c_names = e.mem.u32(info + 0x50 + 2 * 0x10);
        let no_wide = e.mem.u32(info + 0x50 + 4 * 0x10 + 4);
        e.call_log = Some(vec![]);
        e.call(0x00ec_f46d, &[info]);
        assert_eq!(calls_to(&e, FREE_LCONV_MONETARY), vec![vec![lconv]]);
        assert_eq!(calls_to(&e, FREE_LCONV_NUMERIC), vec![vec![lconv]]);
        assert_eq!(calls_to(&e, FREE_LC_TIME), vec![vec![time]]);
        for block in [info, lconv, time, tables[0], tables[1], tables[2]] {
            assert!(freed(&e, block), "{block:08x}");
        }
        for c in counts.iter().filter(|c| **c != info && **c != time + 0xb4) {
            assert!(freed(&e, *c), "{c:08x}");
        }
        assert!(!freed(&e, c_names));
        assert!(!freed(&e, no_wide));
    }

    #[test]
    fn freetlocinfo_keeps_the_c_locales_pieces() {
        let mut e = engine();
        let (info, _) = locale_info(&mut e);
        e.mem.set_u32(info + 0xbc, C_LCONV);
        e.mem.set_u32(info + 0xd4, C_LC_TIME);
        let ctype = e.mem.u32(info + 0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_f46d, &[info]);
        assert!(!freed(&e, ctype));
        assert!(freed(&e, info));
        assert!(e.call_log.as_ref().unwrap().len() == 1);
    }

    #[test]
    fn ansicp_reads_the_locales_code_page() {
        let mut e = engine();
        e.register(IMPORT_GET_LOCALE_INFO_A, |e, a| {
            assert_eq!((a[0], a[1], a[3]), (0x409, 0x1004, 6));
            e.mem.write(a[2], b"1252\0");
            5u32.into_ret()
        });
        assert_eq!(e.call(0x00ee_13e5, &[0x409]).u32(), 1252);
        e.register(IMPORT_GET_LOCALE_INFO_A, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_13e5, &[0x409]).u32(), u32::MAX);
    }

    /// Doubles for the code-page imports: code page 932 is double-byte,
    /// the others single-byte; conversions copy bytes to and from 16-bit
    /// characters and give their counts.
    fn code_pages(e: &mut Engine) {
        e.register(IMPORT_GET_CP_INFO, |e, a| {
            e.mem.set_u32(a[1], if a[0] == 932 { 2 } else { 1 });
            1u32.into_ret()
        });
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            a[3].into_ret()
        });
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u16(a[2] + i * 2);
                    e.mem.set_u8(a[4] + i, c as u8 + 1);
                }
            }
            a[3].into_ret()
        });
        e.register(MALLOC_BODY, |e, a| e.mem.alloc(a[0]).into_ret());
    }

    #[test]
    fn convertcp_converts_through_wide_characters() {
        let mut e = engine();
        code_pages(&mut e);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc\0");
        let length = e.mem.alloc(4);
        e.mem.set_u32(length, 3);
        // The same code page: nothing.
        assert_eq!(
            e.call(0x00ee_142e, &[1252, 1252, text, length, 0, 0]).u32(),
            0
        );
        // Single-byte to single-byte, a new block sized by the text.
        let out = e.call(0x00ee_142e, &[1252, 1250, text, length, 0, 0]).u32();
        assert_eq!(e.mem.bytes(out, 3), b"bcd");
        assert_eq!(e.mem.u32(length), 3);
        // Double-byte source, into a given buffer; -1 counts the terminator.
        e.mem.set_u32(length, u32::MAX);
        let buffer = e.mem.alloc(8);
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            let count = if a[3] == u32::MAX { 4 } else { a[3] };
            if a[4] != 0 {
                for i in 0..count {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ee_142e, &[932, 1252, text, length, buffer, 8])
            .u32();
        assert_eq!(result, buffer);
        assert_eq!(e.mem.bytes(buffer, 4), b"bcd\x01");
        assert_eq!(e.mem.u32(length), u32::MAX);
        let wide = calls_to(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(wide.len(), 1);
        assert_eq!(wide[0][3], 4);
        // A failed first conversion gives 0.
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00ee_142e, &[932, 1252, text, length, 0, 0]).u32(),
            0
        );
    }

    #[test]
    fn convertcp_long_texts_use_a_heap_buffer_that_is_freed() {
        let mut e = engine();
        code_pages(&mut e);
        let text = e.mem.alloc(0x300);
        e.mem.write(text, &[b'a'; 0x300]);
        let length = e.mem.alloc(4);
        e.mem.set_u32(length, 0x300);
        e.call_log = Some(vec![]);
        let out = e.call(0x00ee_142e, &[932, 1252, text, length, 0, 0]).u32();
        assert_ne!(out, 0);
        assert_eq!(e.mem.u32(length), 0x300);
        let block = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == MALLOC_BODY)
            .unwrap()
            .1[0];
        assert_eq!(block, 0x300 * 2 + 8);
        // Sized by the conversion for a double-byte source: two calls out.
        assert_eq!(calls_to(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE).len(), 2);
    }

    #[test]
    fn malloc_body_picks_the_heap_and_reports_failures() {
        let mut e = engine();
        e.mem.set_u32(LOCKS_READY, 0x0044_0000);
        e.register(IMPORT_HEAP_ALLOC, |_, a| {
            assert_eq!(&a[..2], &[0x0044_0000, 0]);
            (0x0010_0000 + a[2]).into_ret()
        });
        e.mem.set_u32(ACTIVE_HEAP, 1);
        assert_eq!(e.call(0x00ec_d1c7, &[0]).u32(), 0x0010_0001);
        assert_eq!(e.call(0x00ec_d1c7, &[0x21]).u32(), 0x0010_0021);
        // Other heaps round up to 16 bytes; the small-block heap goes first.
        e.mem.set_u32(ACTIVE_HEAP, 3);
        e.register(SMALL_BLOCK_ALLOC, |_, a| u32::from(a[0] == 8).into_ret());
        assert_eq!(e.call(0x00ec_d1c7, &[8]).u32(), 1);
        assert_eq!(e.call(0x00ec_d1c7, &[0x21]).u32(), 0x0010_0030);
        e.mem.set_u32(ACTIVE_HEAP, 2);
        assert_eq!(e.call(0x00ec_d1c7, &[0]).u32(), 0x0010_0010);
        // Failures: ENOMEM; with _newmode the new handler gets a retry.
        e.register(IMPORT_HEAP_ALLOC, |_, _| 0u32.into_ret());
        e.register_double(CALL_NEW_HANDLER, |e, _| {
            // The handler frees memory: the next attempt succeeds.
            e.register(IMPORT_HEAP_ALLOC, |_, _| 0x0020_0000u32.into_ret());
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00ec_d1c7, &[4]).u32(), 0);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
        e.mem.set_u32(NEW_MODE, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d1c7, &[4]).u32(), 0x0020_0000);
        assert_eq!(calls_to(&e, CALL_NEW_HANDLER), vec![vec![4]]);
        // Too large: the new handler, ENOMEM, 0.
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d1c7, &[0xffff_fff0]).u32(), 0);
        assert_eq!(calls_to(&e, CALL_NEW_HANDLER), vec![vec![0xffff_fff0]]);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
    }

    #[test]
    fn freea_frees_only_heap_buffers() {
        let mut e = engine();
        let heap = e.mem.alloc(0x20);
        e.mem.set_u32(heap, 0xdddd);
        let stack = e.mem.alloc(0x20);
        e.mem.set_u32(stack, 0xcccc);
        e.call(0x00ec_8e14, &[heap + 8]);
        e.call(0x00ec_8e14, &[stack + 8]);
        e.call(0x00ec_8e14, &[0]);
        assert!(freed(&e, heap));
        assert!(!freed(&e, stack));
    }

    #[test]
    fn freefls_releases_a_thread_block() {
        let mut e = engine();
        for number in [0xc, 0xd] {
            e.mem.set_u32(LOCK_TABLE + number * 8, 0x0055_0000 + number);
        }
        let block = e.mem.alloc(0x214);
        let buffers: Vec<u32> = [0x24, 0x2c, 0x34, 0x3c, 0x40, 0x44, 0x48, 0x5c]
            .iter()
            .map(|offset| {
                let b = e.mem.alloc(8);
                e.mem.set_u32(block + offset, b);
                b
            })
            .collect();
        let mbc = e.mem.alloc(4);
        e.mem.set_u32(mbc, 1);
        e.mem.set_u32(block + 0x68, mbc);
        let (info, _) = locale_info(&mut e);
        e.mem.set_u32(info, 2);
        e.mem.set_u32(block + 0x6c, info);
        e.call_log = Some(vec![]);
        e.call(0x00ed_11f8, &[block]);
        assert!(buffers.iter().all(|b| freed(&e, *b)));
        assert!(freed(&e, mbc));
        assert!(freed(&e, block));
        // The locale info is still referenced: kept.
        assert!(!freed(&e, info));
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![0x0055_000d], vec![0x0055_000c]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![0x0055_000d], vec![0x0055_000c]]
        );
        // A null block does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ed_11f8, &[0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}
