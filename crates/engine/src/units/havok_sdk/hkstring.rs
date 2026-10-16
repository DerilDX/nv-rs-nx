//! `Havok SDK/hkstring.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkString::memCpy`, `memMove` and `memSet` are each a single `JMP` to the C runtime
//! (`memcpy` `00ec44d0`, `memmove` `00ec7230`, `memset` `00ec61c0`), so they forward their
//! arguments and return the runtime's result.

#[allow(unused_imports)]
use crate::prelude::*;

/// The C runtime's `memcpy`, `memmove` and `memset`.
const CRT_MEMCPY: u32 = 0x00ec_44d0;
const CRT_MEMMOVE: u32 = 0x00ec_7230;
const CRT_MEMSET: u32 = 0x00ec_61c0;

// Translated from 00cb10d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strLen` (Xbox PDB, cdecl): the number of bytes before the terminating zero.
pub fn hk_string_str_len(e: &mut Engine, string: Ptr) -> i32 {
    let mut length = 0u32;
    while e.mem.u8(string.addr().wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length as i32
}

// Translated from 00cb1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memCpy` (Xbox PDB, cdecl): a jump to the runtime's `memcpy`
/// `(destination, source, size)`; returns its result.
pub fn hk_string_mem_cpy(e: &mut Engine, destination: Ptr, source: Ptr, size: u32) -> Ptr {
    e.call(CRT_MEMCPY, &args![destination, source, size]).ptr()
}

// Translated from 00cb1270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memMove` (Xbox PDB, cdecl): a jump to the runtime's `memmove`
/// `(destination, source, size)`; returns its result.
pub fn hk_string_mem_move(e: &mut Engine, destination: Ptr, source: Ptr, size: u32) -> Ptr {
    e.call(CRT_MEMMOVE, &args![destination, source, size]).ptr()
}

// Translated from 00cb1280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::memSet` (Xbox PDB, cdecl): a jump to the runtime's `memset`
/// `(destination, value, size)`; returns its result.
pub fn hk_string_mem_set(e: &mut Engine, destination: Ptr, value: i32, size: u32) -> Ptr {
    e.call(CRT_MEMSET, &args![destination, value, size]).ptr()
}

/// `__cdecl` character mapper `strCasecmp` applies to each byte (`00cb0f30`:
/// one word in, the mapped byte in AL).
const MAP_CHARACTER: u32 = 0x00cb_0f30;
/// `__cdecl` copy of a string with an allocator (`00cb1160`: string, allocator).
const COPY_STRING: u32 = 0x00cb_1160;
/// `hkMemoryRouter::hkMemoryRouter` (`00c85770`, `__thiscall`, no stack arguments).
const MEMORY_ROUTER_CONSTRUCTOR: u32 = 0x00c8_5770;
/// The calling thread's memory router (`00c85750`).
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The CRT `strrchr` (`00ec6e30`, `__cdecl`).
const CRT_STRRCHR: u32 = 0x00ec_6e30;
/// Size of the `hkMemoryRouter` that `strDup` keeps on its stack.
const MEMORY_ROUTER_SIZE: u32 = 0x24;

// Translated from 00cb0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strCasecmp` (Xbox PDB): compares two strings byte by byte
/// through the mapper `00cb0f30`, as signed bytes; -1, 0 or 1. Returns 0
/// when both strings end at the same position.
pub fn hk_string_str_casecmp(e: &mut Engine, left: u32, right: u32) -> i32 {
    let mut index = 0;
    loop {
        let left_byte = e.mem.u8(left + index);
        if left_byte == 0 && e.mem.u8(right + index) == 0 {
            return 0;
        }
        let mapped_left = e.call(MAP_CHARACTER, &args![left_byte as u32]).u32() as u8 as i8;
        let right_byte = e.mem.u8(right + index);
        let mapped_right = e.call(MAP_CHARACTER, &args![right_byte as u32]).u32() as u8 as i8;
        if mapped_left < mapped_right {
            return -1;
        }
        if mapped_left > mapped_right {
            return 1;
        }
        index += 1;
    }
}

// Translated from 00cb1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strRchr` (Xbox PDB): a jump to the CRT `strrchr`
/// (`00ec6e30`) with the same two arguments; returns its result.
pub fn hk_string_str_rchr(e: &mut Engine, text: u32, character: u32) -> u32 {
    e.call(CRT_STRRCHR, &args![text, character]).u32()
}

// Translated from 00cb1410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkString::strDup` (Xbox PDB): constructs a temporary `hkMemoryRouter`
/// on the stack (`00c85770`), takes the calling thread's router (`00c85750`)
/// and copies the string with the word at router `+0x10` (`00cb1160`);
/// returns the copy.
pub fn hk_string_str_dup(e: &mut Engine, text: u32) -> u32 {
    e.with_stack(MEMORY_ROUTER_SIZE, |e, router| {
        e.call(MEMORY_ROUTER_CONSTRUCTOR, &args![router]);
        let thread_router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        // hkMemoryRouter +0x10: the allocator word `strDup` hands to the copy.
        let allocator = e.mem.u32(thread_router + 0x10);
        e.call(COPY_STRING, &args![text, allocator]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_10d0, hk_string_str_len(Ptr) -> i32),
        entry!(0x00cb_1260, hk_string_mem_cpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb_1270, hk_string_mem_move(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb_1280, hk_string_mem_set(Ptr, i32, u32) -> Ptr),
        entry!(0x00cb0fe0, hk_string_str_casecmp(u32, u32) -> i32),
        entry!(0x00cb1150, hk_string_str_rchr(u32, u32) -> u32),
        entry!(0x00cb1410, hk_string_str_dup(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_len_counts_bytes_before_the_zero() {
        let mut e = Engine::new();
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"hello");
        assert_eq!(e.call(0x00cb_10d0, &args![text]).i32(), 5);
        e.mem.set_u8(text, 0);
        assert_eq!(e.call(0x00cb_10d0, &args![text]).i32(), 0);
    }

    /// Each wrapper passes its three words on unchanged and returns the runtime's result.
    fn forwards(wrapper: u32, runtime: u32) {
        let mut e = Engine::new();
        e.register(runtime, |_, a| Ret {
            eax: a[0] ^ a[1] ^ a[2],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let got = e.call(wrapper, &args![0x1000u32, 0x20u32, 0x300u32]).u32();
        assert_eq!(got, 0x1000 ^ 0x20 ^ 0x300);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == runtime).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x1000, 0x20, 0x300]);
    }

    #[test]
    fn mem_functions_forward_to_the_runtime() {
        forwards(0x00cb_1260, CRT_MEMCPY);
        forwards(0x00cb_1270, CRT_MEMMOVE);
        forwards(0x00cb_1280, CRT_MEMSET);
    }
}

#[cfg(test)]
mod tests_b {
    use super::*;

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        for (i, byte) in text.iter().enumerate() {
            e.mem.set_u8(address + i as u32, *byte);
        }
        address
    }

    #[test]
    fn str_casecmp_orders_by_mapped_bytes() {
        let mut e = Engine::new();
        e.register(MAP_CHARACTER, |_, a| Ret {
            eax: (a[0] as u8).to_ascii_lowercase() as u32,
            ..Ret::default()
        });
        let hello = put_string(&mut e, b"Hello");
        let shout = put_string(&mut e, b"HELLO");
        let longer = put_string(&mut e, b"HELLOS");
        let bigger = put_string(&mut e, b"hellp");
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, shout]).i32(), 0);
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, longer]).i32(), -1);
        assert_eq!(e.call(0x00cb_0fe0, &args![longer, hello]).i32(), 1);
        assert_eq!(e.call(0x00cb_0fe0, &args![bigger, hello]).i32(), 1);
        assert_eq!(e.call(0x00cb_0fe0, &args![hello, bigger]).i32(), -1);
    }

    #[test]
    fn str_casecmp_compares_mapped_bytes_as_signed() {
        let mut e = Engine::new();
        // 'a' maps to 0x80 (signed -128), anything else to 1.
        e.register(MAP_CHARACTER, |_, a| Ret {
            eax: if a[0] == b'a' as u32 { 0x80 } else { 1 },
            ..Ret::default()
        });
        let a = put_string(&mut e, b"a");
        let b = put_string(&mut e, b"b");
        assert_eq!(e.call(0x00cb_0fe0, &args![a, b]).i32(), -1);
    }

    #[test]
    fn str_rchr_forwards_to_the_crt() {
        let mut e = Engine::new();
        e.register(CRT_STRRCHR, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00cb_1150, &args![0x1000u32, 0x2fu32]).u32(),
            0x102f
        );
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(CRT_STRRCHR, vec![0x1000, 0x2f])));
    }

    #[test]
    fn str_dup_copies_with_the_thread_routers_word() {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register(MEMORY_ROUTER_CONSTRUCTOR, |_, _| Ret::default());
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(COPY_STRING, |_, a| Ret {
            eax: a[0] ^ a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00cb_1410, &args![0x5000u32]).u32(),
            0x5000 ^ 0xa110c
        );
        let log = e.call_log.clone().unwrap();
        assert!(log.iter().any(|c| c.0 == MEMORY_ROUTER_CONSTRUCTOR));
        assert!(log.contains(&(COPY_STRING, vec![0x5000, 0xa110c])));
    }
}
