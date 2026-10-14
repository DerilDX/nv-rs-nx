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
        entry!(0x00ecadb8, rand() -> i32),
        entry!(0x00ec658f, atexit(Ptr) -> i32),
        entry!(0x00ec6cc4, vsprintf(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec7c56, invalid_parameter_noinfo()),
        entry!(0x00ec43fb, rt_dynamic_cast(Ptr, i32, Ptr, Ptr, i32) -> Ptr),
    ]
}

/// Test support for the constructors of the forms (`CreateFormOfType`'s
/// table): a constructor is a run of calls into the component and base
/// constructors and stores of vtable pointers and zeros. [`run`] builds
/// the object, doubles every callee, runs the constructor by its address
/// and compares the calls (with the object's address substituted for
/// [`A::T`]) and the stores; every other byte of the object must be left
/// as it was.
#[cfg(test)]
pub(crate) mod ctor_test {
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
}
