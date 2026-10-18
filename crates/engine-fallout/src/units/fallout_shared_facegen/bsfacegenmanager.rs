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

/// `006532c0`: thiscall (`this`, one word), returns a bool in AL.
const CHECK_006532C0: u32 = 0x0065_32c0;

// Translated from 00653290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bsfacegenmanager.cpp`). Thiscall
/// (`RET 4`): `this` when `006532c0(this, argument)` is true, otherwise null.
pub fn fn_00653290(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    if e.call(CHECK_006532C0, &args![this, argument]).bool() {
        this
    } else {
        Ptr::NULL
    }
}

/// The function `fn_00657fd0` calls on the member (`00658100`).
const CALLEE_00658100: u32 = 0x0065_8100;

/// The function `fn_00658040` calls on the member (`00658130`).
const CALLEE_00658130: u32 = 0x0065_8130;

// Translated from 00657fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00658100` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_00657fd0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00658100, &args![this.byte_add(0x8)]).u32()
}

// Translated from 00658040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00658130` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_00658040(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00658130, &args![this.byte_add(0x8)]).u32()
}

/// `BSFaceGenManager::GetInstance` (Xbox PDB name of `00651b30`; no
/// arguments): the manager singleton.
const FACE_GEN_GET_INSTANCE: u32 = 0x0065_1b30;
/// `009611e0` (`ECX` = matrix): the word at `+0x18`.
const MATRIX_FIRST_DIMENSION: u32 = 0x0096_11e0;
/// `PathingLocation::GetWorldspace` (Xbox PDB name of `00441110`; `ECX` =
/// matrix): the word at `+0x1c`.
const MATRIX_SECOND_DIMENSION: u32 = 0x0044_1110;
/// `std::vector<float>::resize` (engine map name of `0060b340`; `ECX` =
/// matrix, two words): stores them at `+0x18` and `+0x1c` and sizes the
/// storage for their product.
const MATRIX_RESIZE: u32 = 0x0060_b340;
/// `FR2MatrixVTC<float>::operator=` (Xbox PDB name of `006581e0`; `ECX` =
/// destination matrix, source matrix): copies the dimensions and the values.
const MATRIX_ASSIGN: u32 = 0x0065_81e0;

// Translated from 006521e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::GetFaceGenCoord` (Xbox PDB; no arguments): the address
/// of the coordinate block, 8 bytes into the manager singleton
/// (`BSFaceGenManager::GetInstance`, `00651b30`).
pub fn bs_face_gen_manager_get_face_gen_coord(e: &mut Engine) -> u32 {
    e.call(FACE_GEN_GET_INSTANCE, &[]).u32() + 8
}

// Translated from 006529a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenManager::CopyFaceGenCoord` (Xbox PDB; `cdecl`: source block,
/// destination block, two bytes). Both blocks hold two rows of two 0x20-byte
/// matrices (row `i`, column `j` at `i * 0x40 + j * 0x20`). Does nothing for
/// a null block. For each matrix: when the source has both dimensions
/// non-zero, the destination takes the source's dimensions, and its values
/// too when `first` is non-zero and the row is 0, `second` is non-zero and the
/// row is 1, or the two bytes are equal; otherwise the destination is sized
/// 0 by 0.
pub fn bs_face_gen_manager_copy_face_gen_coord(
    e: &mut Engine,
    source: u32,
    destination: u32,
    first: u8,
    second: u8,
) {
    if source == 0 || destination == 0 {
        return;
    }
    for row in 0..2u32 {
        for column in 0..2u32 {
            let from = source + row * 0x40 + column * 0x20;
            let to = destination + row * 0x40 + column * 0x20;
            if e.call(MATRIX_FIRST_DIMENSION, &args![from]).u32() != 0
                && e.call(MATRIX_SECOND_DIMENSION, &args![from]).u32() != 0
            {
                let columns = e.call(MATRIX_SECOND_DIMENSION, &args![from]).u32();
                let rows = e.call(MATRIX_FIRST_DIMENSION, &args![from]).u32();
                e.call(MATRIX_RESIZE, &args![to, rows, columns]);
                if (first != 0 && row == 0) || (second != 0 && row == 1) || first == second {
                    e.call(MATRIX_ASSIGN, &args![to, from]);
                }
            } else {
                e.call(MATRIX_RESIZE, &args![to, 0u32, 0u32]);
            }
        }
    }
}

// Translated from 00658bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01065870` (RTTI `.?AV?$BSTCaseInsensitiveStringMap@V?$NiPointer@VEntry@BSFaceGenModelMap@@@@@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00658d30`. One word, plain `RET`, returns nothing.
pub fn fn_00658bb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_5870);
    e.call(0x0065_8d30, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_8bb0, fn_00658bb0(Ptr)),
        entry!(0x00653290, fn_00653290(Ptr, u32) -> Ptr),
        entry!(0x00486b50, is_keys_equal(Ptr, u32, u32) -> bool),
        entry!(0x00600b20, key_to_hash_index(Ptr, u32) -> u32),
        entry!(0x00659210, fn_00659210(Ptr, u8, u32) -> u32),
        entry!(0x0065_7fd0, fn_00657fd0(Ptr) -> u32),
        entry!(0x0065_8040, fn_00658040(Ptr) -> u32),
        entry!(0x006521e0, bs_face_gen_manager_get_face_gen_coord() -> u32),
        entry!(
            0x006529a0,
            bs_face_gen_manager_copy_face_gen_coord(u32, u32, u8, u8)
        ),
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

    #[test]
    fn fn_00653290_returns_this_only_when_the_check_passes() {
        let mut e = Engine::new();
        e.register(CHECK_006532C0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let this = Ptr::<()>::new(0x5000);
        assert_eq!(e.call(0x0065_3290, &args![this, 1u32]).ptr::<()>(), this);
        assert_eq!(
            e.call(0x0065_3290, &args![this, 0u32]).ptr::<()>(),
            Ptr::<()>::NULL
        );
    }

    #[test]
    fn l68d_00657fd0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00658100, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_7fd0, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00658100).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }

    #[test]
    fn l68d_00658040_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00658130, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_8040, &args![0x4200u32]).u32(), 0x175d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00658130).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x8]);
    }
}

#[cfg(test)]
mod coord_tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_coordinate_block_is_8_bytes_into_the_manager() {
        let mut e = Engine::new();
        e.register(FACE_GEN_GET_INSTANCE, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        assert_eq!(e.call(0x006521e0, &[]).u32(), 0x5008);
    }

    /// Runs the copy on blocks at 0x10000 (source) and 0x20000 (destination).
    /// The source matrix at offset `index * 0x20` has the dimensions
    /// `dimensions[index]` (rows, columns).
    fn copy(dimensions: [(u32, u32); 4], first: u32, second: u32) -> CallLog {
        let mut e = Engine::new();
        e.map(0x10000, 0x100);
        for (index, (rows, columns)) in dimensions.iter().enumerate() {
            e.mem.set_u32(0x10000 + index as u32 * 0x20 + 0x18, *rows);
            e.mem
                .set_u32(0x10000 + index as u32 * 0x20 + 0x1c, *columns);
        }
        e.register(MATRIX_FIRST_DIMENSION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x18),
            ..Ret::default()
        });
        e.register(MATRIX_SECOND_DIMENSION, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x1c),
            ..Ret::default()
        });
        e.register(MATRIX_RESIZE, |_, _| Ret::default());
        e.register(MATRIX_ASSIGN, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006529a0, &[0x10000, 0x20000, first, second]);
        e.call_log.take().unwrap()
    }

    #[test]
    fn null_blocks_do_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x006529a0, &[0, 0x20000, 1, 1]);
        e.call(0x006529a0, &[0x10000, 0, 1, 1]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn equal_flags_copy_every_matrix_with_dimensions() {
        let log = copy([(2, 3), (4, 5), (6, 7), (8, 9)], 0, 0);
        assert_eq!(
            calls_to(&log, MATRIX_RESIZE),
            vec![
                vec![0x20000, 2, 3],
                vec![0x20020, 4, 5],
                vec![0x20040, 6, 7],
                vec![0x20060, 8, 9],
            ]
        );
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![
                vec![0x20000, 0x10000],
                vec![0x20020, 0x10020],
                vec![0x20040, 0x10040],
                vec![0x20060, 0x10060],
            ]
        );
    }

    #[test]
    fn unequal_flags_copy_only_the_row_they_name() {
        let dims = [(2, 3); 4];
        let first = copy(dims, 1, 0);
        assert_eq!(
            calls_to(&first, MATRIX_ASSIGN),
            vec![vec![0x20000, 0x10000], vec![0x20020, 0x10020]]
        );
        let second = copy(dims, 0, 1);
        assert_eq!(
            calls_to(&second, MATRIX_ASSIGN),
            vec![vec![0x20040, 0x10040], vec![0x20060, 0x10060]]
        );
        // Only the low byte of each flag counts, and unequal bytes of two
        // set flags still copy both rows.
        let both = copy(dims, 2, 1);
        assert_eq!(calls_to(&both, MATRIX_ASSIGN).len(), 4);
        let masked = copy(dims, 0x100, 0);
        assert_eq!(calls_to(&masked, MATRIX_ASSIGN).len(), 4);
    }

    #[test]
    fn a_source_matrix_without_both_dimensions_empties_the_destination() {
        let log = copy([(2, 0), (0, 5), (0, 0), (1, 1)], 0, 0);
        assert_eq!(
            calls_to(&log, MATRIX_RESIZE),
            vec![
                vec![0x20000, 0, 0],
                vec![0x20020, 0, 0],
                vec![0x20040, 0, 0],
                vec![0x20060, 1, 1],
            ]
        );
        assert_eq!(calls_to(&log, MATRIX_ASSIGN), vec![vec![0x20060, 0x10060]]);
    }

    #[test]
    fn fn_00658bb0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0065_8d30, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0065_8bb0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0065_8d30)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0106_5870);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0106_5870,
            "the vtable was stored before the call"
        );
    }
}
