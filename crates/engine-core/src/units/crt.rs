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
/// or the static slot `011c18c0` when the thread has no block. The block is the platform
/// layer's thread block, which nothing provides yet, so the call to `00ed1165` stays open until
/// it is.
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
