//! `fallout shared/facegen/bsfacegenmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `tolower` (LIBCMT, `00ec67aa`).
const TOLOWER: u32 = 0x00ec_67aa;

// Translated from 00600b20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<..>::KeyToHashIndex` (Xbox PDB; `thiscall`,
/// `RET 4`): the bucket of a key. Each byte up to the terminator, lowered
/// (`tolower` on the sign-extended byte), is folded in by [`fn_00659210`];
/// the hash modulo the map's bucket count (+4, unsigned).
pub fn key_to_hash_index(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut p = key;
    loop {
        let c = e.mem.u8(p) as i8 as i32;
        if c == 0 {
            break;
        }
        let lowered = e.call(TOLOWER, &args![c as u32]).u32() as u8;
        hash = fn_00659210(e, this, lowered, hash);
        p = p.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

// Translated from 00659210 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// One step of the string hash (`thiscall`, `RET 8`; `this` unused):
/// `hash * 33 + c`, the byte sign-extended (`MOVSX`).
pub fn fn_00659210(_e: &mut Engine, _this: Ptr, c: u8, hash: u32) -> u32 {
    (hash << 5)
        .wrapping_add(hash)
        .wrapping_add(c as i8 as i32 as u32)
}

/// The `__stricmp` wrapper (`00404dc0`, `cdecl`).
const STRICMP_WRAPPER: u32 = 0x0040_4dc0;

// Translated from 00486b50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<..>::IsKeysEqual` (Xbox PDB; `thiscall`,
/// `RET 8`; `this` unused): whether the keys are equal ignoring case
/// (`00404dc0` answers 0).
pub fn is_keys_equal(e: &mut Engine, _this: Ptr, a: u32, b: u32) -> bool {
    e.call(STRICMP_WRAPPER, &args![a, b]).u32() == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00486b50, is_keys_equal(Ptr, u32, u32) -> bool),
        entry!(0x00600b20, key_to_hash_index(Ptr, u32) -> u32),
        entry!(0x00659210, fn_00659210(Ptr, u8, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_equal_when_the_compare_answers_zero() {
        let mut e = Engine::new();
        e.register(STRICMP_WRAPPER, |_, a| u32::from(a[0] != a[1]).into_ret());
        assert_eq!(e.call(0x00486b50, &[0, 5, 5]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00486b50, &[0, 5, 6]).u32() & 0xff, 0);
    }

    #[test]
    fn hashes_the_lowered_key_modulo_the_bucket_count() {
        let mut e = Engine::new();
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, 37);
        let key = e.mem.alloc(8);
        e.mem.write(key, b"fAb\0");
        let mut h = 0u32;
        for c in b"fab" {
            h = h.wrapping_mul(33).wrapping_add(u32::from(*c));
        }
        assert_eq!(e.call(0x00600b20, &[map, key]).u32(), h % 37);
    }

    #[test]
    fn a_high_byte_is_sign_extended() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00659210, &[0, 0xe9, 1]).u32(),
            33u32.wrapping_sub(0x17)
        );
    }
}
