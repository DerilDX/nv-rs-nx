//! `fallout/interface/run_007001f0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTPrimitiveArray` constructor `BGSMenuPacker`'s constructor runs
/// first (`00700480`, thiscall: two words).
const PACKED_MENUS_CONSTRUCTOR: u32 = 0x0070_0480;
/// Reads the word at `this` (`00559450`, thiscall).
const READ_WORD: u32 = 0x0055_9450;
/// Sets a tile's float trait (`00a012d0`, thiscall: trait, value, flag).
const TILE_SET_FLOAT: u32 = 0x00a0_12d0;

// Translated from 007002d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BGSMenuPacker` constructor (no PDB name; the Xbox PDB's
/// `BGSMenuPacker` has `PackedMenus` at +0 and `bPackFailed` at +0x21):
/// constructs the packed-menu array (`00700480` with 0 and 1) and clears
/// `bPackFailed`. Returns `this`.
pub fn fn_007002d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PACKED_MENUS_CONSTRUCTOR, &args![this, 0u32, 1u32]);
    e.mem.set_u8(this.addr() + 0x21, 0);
    this
}

// Translated from 00700300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x20` (through `00559450`; for a `BGSMenuPacker`,
/// `dump_info_flag` and the bytes after it).
pub fn fn_00700300(e: &mut Engine, this: Ptr) -> u32 {
    e.call(READ_WORD, &args![this.byte_add(0x20)]).u32()
}

// Translated from 00700320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the tile `this`'s trait `trait_id` to the integer `value` as a
/// float (flag word 1).
pub fn fn_00700320(e: &mut Engine, this: Ptr, trait_id: i32, value: i32) {
    let value = x87_store32(f64::from(value));
    e.call(TILE_SET_FLOAT, &args![this, trait_id, value, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_02d0, fn_007002d0(Ptr) -> Ptr),
        entry!(0x0070_0300, fn_00700300(Ptr) -> u32),
        entry!(0x0070_0320, fn_00700320(Ptr, i32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_007002d0_constructs_the_array_and_clears_the_failure() {
        let mut e = Engine::new();
        e.register(PACKED_MENUS_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        e.mem.set_u8(this + 0x21, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0070_02d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this + 0x21), 0);
        assert_eq!(
            calls_to(&e, PACKED_MENUS_CONSTRUCTOR),
            vec![vec![this, 0, 1]]
        );
    }

    #[test]
    fn fn_00700300_reads_the_word_at_0x20() {
        let mut e = Engine::new();
        e.register(READ_WORD, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0070_0300, &args![0x5000u32]).u32(), 0x5021);
    }

    #[test]
    fn fn_00700320_sets_the_integer_as_a_float() {
        let mut e = Engine::new();
        e.register(TILE_SET_FLOAT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0070_0320, &args![0x5000u32, 0xfb1u32, -7i32]);
        assert_eq!(
            calls_to(&e, TILE_SET_FLOAT),
            vec![vec![0x5000, 0xfb1, (-7.0f32).to_bits(), 1]]
        );
    }
}
