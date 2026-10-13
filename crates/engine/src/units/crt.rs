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

// ---- Program ---------------------------------------------------------------

// Platform: replaces 00ec658f (_atexit, LIBCMT)
/// Registers a function to run at exit. The engine does not run exit
/// handlers (the game's own run them as it quits); always succeeds.
pub fn atexit(_e: &mut Engine, _f: Ptr) -> i32 {
    0
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

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00401000, operator_new(u32) -> Ptr),
        entry!(0x00401460, memcpy_wrapper(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00403d30, memset_wrapper(Ptr, i32, u32) -> Ptr),
        entry!(0x00ec44d0, memcpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ec7230, memmove(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ec61c0, memset(Ptr, i32, u32) -> Ptr),
        entry!(0x00ec4835, memcmp(Ptr, Ptr, u32) -> i32),
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
        entry!(0x00eca573, atof(Ptr) -> f64),
        entry!(0x00ec62c0, ftol2(f64) -> u64),
        entry!(0x00ec6040, ci_sqrt(f64) -> f64),
        entry!(0x00ec9f70, ci_cos(f64) -> f64),
        entry!(0x00eca0a0, ci_sin(f64) -> f64),
        entry!(0x00ec9f30, cos(f64) -> f64),
        entry!(0x00eca060, sin(f64) -> f64),
        entry!(0x00ec9b00, pow(f64, f64) -> f64),
        entry!(0x00eca1d0, acos(f64) -> f64),
        entry!(0x00ecadb8, rand() -> i32),
        entry!(0x00ec658f, atexit(Ptr) -> i32),
        entry!(0x00ec7c56, invalid_parameter_noinfo()),
        entry!(0x00ec43fb, rt_dynamic_cast(Ptr, i32, Ptr, Ptr, i32) -> Ptr),
    ]
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
}
