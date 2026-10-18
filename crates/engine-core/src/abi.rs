//! The uniform form every translated function also has, so that any
//! translation can call any other by its exe address (directly, or through
//! a vtable) without knowing whether it has been translated yet.
//!
//! Arguments are 32-bit words in declaration order, `this` first for
//! `__thiscall` methods (the register conventions `__thiscall` and
//! `__fastcall` are an x86 detail and do not matter here). An 8-byte value
//! (`f64`, `u64`) takes two words, low word first. A struct returned by
//! value is a hidden pointer argument, written out explicitly. A float
//! result (x87 `ST0`) is in [`Ret::st0`]; an integer or pointer result in
//! `eax` (and `edx` for 64-bit results).

use crate::ptr::Ptr;
use crate::Engine;

/// Result of a call in the uniform form.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ret {
    pub eax: u32,
    pub edx: u32,
    pub st0: f64,
}

impl Ret {
    pub fn u32(self) -> u32 {
        self.eax
    }
    pub fn i32(self) -> i32 {
        self.eax as i32
    }
    /// The low byte, as MSVC returns `bool` (`AL`).
    pub fn bool(self) -> bool {
        self.eax as u8 != 0
    }
    pub fn u8(self) -> u8 {
        self.eax as u8
    }
    pub fn u16(self) -> u16 {
        self.eax as u16
    }
    pub fn u64(self) -> u64 {
        self.eax as u64 | (self.edx as u64) << 32
    }
    pub fn f32(self) -> f32 {
        self.st0 as f32
    }
    pub fn f64(self) -> f64 {
        self.st0
    }
    pub fn ptr<T>(self) -> Ptr<T> {
        Ptr::new(self.eax)
    }
}

/// A function in the uniform form.
pub type AbiFn = fn(&mut Engine, &[u32]) -> Ret;

/// A parameter type: how it is read from the argument words.
pub trait Arg: Sized {
    const WORDS: usize;
    fn take(words: &[u32], i: &mut usize) -> Self;
    fn put(self, out: &mut Vec<u32>);
}

fn word(words: &[u32], i: &mut usize) -> u32 {
    let w = *words
        .get(*i)
        .unwrap_or_else(|| panic!("call has {} argument words, needs more", words.len()));
    *i += 1;
    w
}

macro_rules! word_arg {
    ($($t:ty => |$w:ident| $from:expr, |$v:ident| $to:expr;)*) => {$(
        impl Arg for $t {
            const WORDS: usize = 1;
            fn take(words: &[u32], i: &mut usize) -> Self {
                let $w = word(words, i);
                $from
            }
            fn put(self, out: &mut Vec<u32>) {
                let $v = self;
                out.push($to);
            }
        }
    )*};
}
word_arg! {
    u32 => |w| w, |v| v;
    i32 => |w| w as i32, |v| v as u32;
    u16 => |w| w as u16, |v| v as u32;
    i16 => |w| w as i16, |v| v as i16 as i32 as u32;
    u8 => |w| w as u8, |v| v as u32;
    i8 => |w| w as i8, |v| v as i8 as i32 as u32;
    bool => |w| w as u8 != 0, |v| v as u32;
    f32 => |w| f32::from_bits(w), |v| v.to_bits();
}

impl<T> Arg for Ptr<T> {
    const WORDS: usize = 1;
    fn take(words: &[u32], i: &mut usize) -> Self {
        Ptr::new(word(words, i))
    }
    fn put(self, out: &mut Vec<u32>) {
        out.push(self.0)
    }
}

impl Arg for u64 {
    const WORDS: usize = 2;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let lo = word(words, i) as u64;
        let hi = word(words, i) as u64;
        lo | hi << 32
    }
    fn put(self, out: &mut Vec<u32>) {
        out.push(self as u32);
        out.push((self >> 32) as u32);
    }
}

impl Arg for f64 {
    const WORDS: usize = 2;
    fn take(words: &[u32], i: &mut usize) -> Self {
        f64::from_bits(u64::take(words, i))
    }
    fn put(self, out: &mut Vec<u32>) {
        self.to_bits().put(out)
    }
}

/// A result type: how it is put into a [`Ret`] and read back.
pub trait RetVal: Sized {
    fn into_ret(self) -> Ret;
    fn from_ret(r: Ret) -> Self;
}

macro_rules! int_ret {
    ($($t:ty),*) => {$(
        impl RetVal for $t {
            fn into_ret(self) -> Ret {
                let mut v = Vec::with_capacity(1);
                self.put(&mut v);
                Ret { eax: v[0], ..Ret::default() }
            }
            fn from_ret(r: Ret) -> Self {
                <$t as Arg>::take(&[r.eax], &mut 0)
            }
        }
    )*};
}
int_ret!(u32, i32, u16, i16, u8, i8, bool);

impl<T> RetVal for Ptr<T> {
    fn into_ret(self) -> Ret {
        Ret {
            eax: self.0,
            ..Ret::default()
        }
    }
    fn from_ret(r: Ret) -> Self {
        Ptr::new(r.eax)
    }
}
impl RetVal for () {
    fn into_ret(self) -> Ret {
        Ret::default()
    }
    fn from_ret(_: Ret) -> Self {}
}
impl RetVal for u64 {
    fn into_ret(self) -> Ret {
        Ret {
            eax: self as u32,
            edx: (self >> 32) as u32,
            st0: 0.0,
        }
    }
    fn from_ret(r: Ret) -> Self {
        r.u64()
    }
}
impl RetVal for f32 {
    fn into_ret(self) -> Ret {
        Ret {
            st0: self as f64,
            ..Ret::default()
        }
    }
    fn from_ret(r: Ret) -> Self {
        r.st0 as f32
    }
}
impl RetVal for f64 {
    fn into_ret(self) -> Ret {
        Ret {
            st0: self,
            ..Ret::default()
        }
    }
    fn from_ret(r: Ret) -> Self {
        r.st0
    }
}

/// Builds argument words: `args![this, 1.5f32, true]`.
#[macro_export]
macro_rules! args {
    ($($a:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut v: Vec<u32> = Vec::new();
        $( $crate::abi::Arg::put($a, &mut v); )*
        v
    }};
}

/// One entry of a unit's function table: the exe address and the uniform
/// form of a typed translation.
///
/// ```ignore
/// pub fn funcs() -> Vec<(u32, AbiFn)> {
///     vec![entry!(0x00d87410, get_iterator(Ptr<HkCachedHashMap>) -> i32)]
/// }
/// ```
#[macro_export]
macro_rules! entry {
    ($addr:literal, $($f:ident)::+ ( $($t:ty),* $(,)? ) $(-> $r:ty)?) => {
        ($addr, (|e: &mut $crate::Engine, a: &[u32]| {
            let _ = a;
            #[allow(unused_mut, unused_variables)]
            let mut i = 0usize;
            let r = $($f)::+(e, $( <$t as $crate::abi::Arg>::take(a, &mut i) ),*);
            $crate::abi::RetVal::into_ret(r)
        }) as $crate::abi::AbiFn)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_round_trip() {
        let w = args![7u32, -2i32, 1.5f32, 2.25f64, true, Ptr::<()>::new(0x1234)];
        assert_eq!(w.len(), 7);
        let mut i = 0;
        assert_eq!(u32::take(&w, &mut i), 7);
        assert_eq!(i32::take(&w, &mut i), -2);
        assert_eq!(f32::take(&w, &mut i), 1.5);
        assert_eq!(f64::take(&w, &mut i), 2.25);
        assert!(bool::take(&w, &mut i));
        assert_eq!(Ptr::<()>::take(&w, &mut i).0, 0x1234);
    }

    #[test]
    fn bool_is_the_low_byte() {
        let r = Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        };
        assert!(!r.bool());
    }
}
