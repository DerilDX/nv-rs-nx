//! `BSShader/run_00b97900` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa1070` (`cdecl`, size).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// CRT `_memset` (`00ec61c0`, `cdecl`: destination, byte value word, size).
const MEMSET: u32 = 0x00ec_61c0;
/// `ImageSpaceManager::Initialize` (engine map, `00b977d0`): `thiscall`, no stack words.
const IMAGE_SPACE_MANAGER_INITIALIZE: u32 = 0x00b9_77d0;
/// The vtable `00b97a80` puts at +4 first (`010b9d24`) and the one that replaces it after
/// the allocation (`010b8004`); neither has an RTTI locator before it.
const VTABLE_010B9D24: u32 = 0x010b_9d24;
const VTABLE_010B8004: u32 = 0x010b_8004;

/// The words `00b97a80` clears with `FLDZ`/`FST` (0.0 as a float), in the exe's order.
const ZERO_FLOATS_00B97A80: [u32; 24] = [
    0x214, 0x218, 0x21c, 0x220, 0x224, 0x228, 0x22c, 0x230, 0x238, 0x23c, 0x240, 0x244, 0x248,
    0x24c, 0x250, 0x254, 0x2f8, 0x2fc, 0x300, 0x304, 0x308, 0x30c, 0x310, 0x314,
];

// Translated from 00b97a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b97900`): the constructor of the 0x31c-byte object the
/// shader manager's set-up (`00b57420`) keeps at `011f91ac`. `thiscall`, no stack words.
/// Sets the 16-bit words at +0xc (0x74), +0x12 (1), +0xe and +0x10 (0) and the vtable
/// `010b9d24` at +4; stores the result of `00aa1070(0x1d0)` at +8; puts the vtable `010b8004`
/// at +4; clears the words and bytes from +0xac to +0x210 that the exe clears (the word at
/// +0x1ec is set to -1) and 24 floats from +0x214 to +0x314 (not +0x234); clears 0x98 bytes
/// at +0x14 (`_memset`); then runs `ImageSpaceManager::Initialize` (`00b977d0`) on it.
/// Answers `this`.
// Translation incomplete: 00b97a80 - the C++ exception frame (FS:[0] chain, unwind state 5 around the field set-up) is not translated.
pub fn fn_00b97a80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let at = |offset: u32| base.wrapping_add(offset);
    e.mem.set_u16(at(0xc), 0x74);
    e.mem.set_u32(at(4), VTABLE_010B9D24);
    e.mem.set_u16(at(0x12), 1);
    e.mem.set_u16(at(0xe), 0);
    e.mem.set_u16(at(0x10), 0);
    let block = e.call(ALLOCATE_00AA1070, &args![0x1d0u32]).u32();
    e.mem.set_u32(at(8), block);
    e.mem.set_u32(at(4), VTABLE_010B8004);
    e.mem.set_u32(at(0xac), 0);
    e.mem.set_u32(at(0xb0), 0);
    e.mem.set_u8(at(0x1e4), 0);
    e.mem.set_u32(at(0x1ec), 0xffff_ffff);
    e.mem.set_u32(at(0x1f0), 0);
    e.mem.set_u32(at(0x1f4), 0);
    e.mem.set_u32(at(0x1f8), 0);
    e.mem.set_u8(at(0x1fc), 0);
    e.mem.set_u32(at(0x200), 0);
    e.mem.set_u8(at(0x204), 0);
    e.mem.set_u8(at(0x205), 0);
    e.mem.set_u8(at(0x206), 0);
    e.mem.set_u32(at(0x208), 0);
    e.mem.set_u32(at(0x20c), 0);
    e.mem.set_u32(at(0x210), 0);
    let zero = x87_store32(0.0);
    for offset in ZERO_FLOATS_00B97A80 {
        e.mem.set_u32(at(offset), zero);
    }
    e.call(MEMSET, &args![at(0x14), 0u32, 0x98u32]);
    e.call(IMAGE_SPACE_MANAGER_INITIALIZE, &args![this]);
    this
}

/// The two blocks `00b97c10` fills: 0x3fc bytes of 0xff at `011ffa30` and 0x54 bytes of 0
/// at `011ff9d8`.
const BLOCK_011FFA30: u32 = 0x011f_fa30;
const BLOCK_011FF9D8: u32 = 0x011f_f9d8;

/// The (word, value) stores `00b97c10` makes after the two fills, in the exe's order.
const STORES_00B97C10: [(u32, u32); 27] = [
    (0x011f_fa94, 5),
    (0x011f_fa90, 5),
    (0x011f_fa7c, 7),
    (0x011f_fa80, 7),
    (0x011f_fb04, 10),
    (0x011f_fb08, 10),
    (0x011f_fb0c, 10),
    (0x011f_fb10, 0xb),
    (0x011f_fb14, 0xb),
    (0x011f_fb18, 0xb),
    (0x011f_fa4c, 0),
    (0x011f_fa68, 1),
    (0x011f_fa8c, 2),
    (0x011f_fd3c, 3),
    (0x011f_fa6c, 4),
    (0x011f_fa9c, 6),
    (0x011f_fcdc, 8),
    (0x011f_fb00, 9),
    (0x011f_fb1c, 0xc),
    (0x011f_fce8, 0xd),
    (0x011f_fc90, 0xf),
    (0x011f_fcd0, 0x10),
    (0x011f_fd28, 0x11),
    (0x011f_fd2c, 0x11),
    (0x011f_fd30, 0x11),
    (0x011f_fa50, 0x12),
    (0x011f_fa88, 0x13),
];

// Translated from 00b97c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b97900`). No arguments: fills 0x3fc bytes at `011ffa30`
/// with 0xff and 0x54 bytes at `011ff9d8` with 0 (`_memset`, the fill value passed as the
/// word the exe pushes), then stores 27 small numbers into words of the `011ffa30` table (the
/// immediates of its `MOV` instructions, in the exe's order).
pub fn fn_00b97c10(e: &mut Engine) {
    e.call(MEMSET, &args![BLOCK_011FFA30, 0xffff_ffffu32, 0x3fcu32]);
    e.call(MEMSET, &args![BLOCK_011FF9D8, 0u32, 0x54u32]);
    for (word, value) in STORES_00B97C10 {
        e.set_global(word, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_7a80, fn_00b97a80(Ptr) -> Ptr),
        entry!(0x00b9_7c10, fn_00b97c10()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// `_memset` double that fills like the CRT (low byte of the value word).
    fn memset(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..a[2] {
            e.mem.set_u8(a[0] + i, a[1] as u8);
        }
        a[0].into_ret()
    }

    #[test]
    fn fn_00b97a80_sets_its_fields_then_initializes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x31c);
        for offset in (0..0x31c).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.register(ALLOCATE_00AA1070, |_, _| 0x0abc_0000u32.into_ret());
        e.register(MEMSET, memset);
        e.register(IMAGE_SPACE_MANAGER_INITIALIZE, |e, a| {
            // Everything is set before Initialize runs.
            assert_eq!(e.mem.u32(a[0] + 0x1ec), 0xffff_ffff);
            assert_eq!(e.mem.u32(a[0] + 0x14), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b9_7a80, &args![this]).u32(), this);

        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![0x1d0]]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x14, 0, 0x98]]);
        assert_eq!(calls(&e, IMAGE_SPACE_MANAGER_INITIALIZE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), VTABLE_010B8004);
        assert_eq!(e.mem.u32(this + 8), 0x0abc_0000);
        assert_eq!(e.mem.u32(this + 0xc), 0x0000_0074);
        assert_eq!(e.mem.u32(this + 0x10), 0x0001_0000);
        assert_eq!(e.mem.bytes(this + 0x14, 0x98), vec![0; 0x98]);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u32(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x1e4), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x1e8), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x1fc), 0xdead_be00);
        assert_eq!(e.mem.u32(this + 0x204), 0xde00_0000);
        for offset in [0x1f0, 0x1f4, 0x1f8, 0x200, 0x208, 0x20c, 0x210] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        for offset in ZERO_FLOATS_00B97A80 {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x234), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x258), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x318), 0xdead_beef);
    }

    #[test]
    fn fn_00b97c10_fills_the_tables_then_stores_the_numbers() {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        e.register(MEMSET, memset);
        e.call_log = Some(vec![]);
        e.call(0x00b9_7c10, &args![]);
        assert_eq!(
            calls(&e, MEMSET),
            vec![
                vec![BLOCK_011FFA30, 0xffff_ffff, 0x3fc],
                vec![BLOCK_011FF9D8, 0, 0x54]
            ]
        );
        for (word, value) in STORES_00B97C10 {
            assert_eq!(e.global::<u32>(word), value, "{word:#x}");
        }
        // Untouched words of the table keep the fill.
        assert_eq!(e.global::<u32>(0x011f_fa30), 0xffff_ffff);
        assert_eq!(e.global::<u32>(0x011f_fe28), 0xffff_ffff);
        assert_eq!(e.global::<u32>(0x011f_fe2c), 0);
        assert_eq!(e.global::<u32>(0x011f_f9d8), 0);
    }
}
