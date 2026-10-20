//! `BSShader/run_00b9b380` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010b7e6c`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010B7E6C: u32 = 0x010b_7e6c;
/// The global that keeps the effect (`011ffe44`), the 256-byte table it copies twice
/// (`011ad900`) and the fifteen `float`s it clears (`011ffe4c` to `011ffe94`, three of every
/// four).
const EFFECT_011FFE44: u32 = 0x011f_fe44;
const TABLE_011AD900: u32 = 0x011a_d900;
const FLOATS_011FFE4C: u32 = 0x011f_fe4c;

// Translated from 00b9b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b7e6c`, clears four 16-byte blocks at +0x58, +0x68, +0x78 and +0x88 (three bytes, then
/// three words each), keeps the effect in `011ffe44`, clears the byte +0x2b8, copies the 256
/// bytes at `011ad900` to +0x198 and to +0x98 (byte by byte, both copies per byte) and stores
/// 0.0 in the `float`s `011ffe4c`, `011ffe50`, `011ffe54`, `011ffe5c` .. `011ffe94` (three of
/// each four words). Answers `this`.
pub fn fn_00b9b830(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7E6C);
    for block in [0x58u32, 0x68, 0x78, 0x88] {
        for byte in 0..3u32 {
            e.mem.set_u8(base + block + byte, 0);
        }
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(base + block + word, 0);
        }
    }
    e.set_global(EFFECT_011FFE44, base);
    e.mem.set_u8(base + 0x2b8, 0);
    for index in 0..0x100u32 {
        let byte = e.mem.u8(TABLE_011AD900 + index);
        e.mem.set_u8(base + 0x198 + index, byte);
        e.mem.set_u8(base + 0x98 + index, byte);
    }
    for group in 0..5u32 {
        for word in 0..3u32 {
            e.mem.set_u32(FLOATS_011FFE4C + 0x10 * group + 4 * word, 0);
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b9_b830, fn_00b9b830(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b9b830_builds_the_effect_and_its_tables() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let table: Vec<u8> = (0..=255u8).map(|b| b ^ 0x5a).collect();
        e.mem.write(TABLE_011AD900, &table);
        e.mem.write(FLOATS_011FFE4C - 4, &[0xff; 0x50]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2bc);
        e.mem.write(this, &[0xcc; 0x2bc]);
        assert_eq!(e.call(0x00b9_b830, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7E6C);
        for block in [0x58u32, 0x68, 0x78, 0x88] {
            assert_eq!(e.mem.bytes(this + block, 3), vec![0; 3]);
            assert_eq!(e.mem.u8(this + block + 3), 0xcc);
            assert_eq!(e.mem.bytes(this + block + 4, 0xc), vec![0; 0xc]);
        }
        assert_eq!(e.mem.bytes(this + 0x98, 0x100), table);
        assert_eq!(e.mem.bytes(this + 0x198, 0x100), table);
        assert_eq!(e.mem.u8(this + 0x2b8), 0);
        assert_eq!(e.mem.u8(this + 0x2b9), 0xcc);
        assert_eq!(e.global::<u32>(EFFECT_011FFE44), this);
        for group in 0..5u32 {
            let first = FLOATS_011FFE4C + 0x10 * group;
            assert_eq!(e.mem.bytes(first, 0xc), vec![0; 0xc]);
            assert_eq!(e.mem.u32(first - 4), 0xffff_ffff);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EFFECT_CONSTRUCT_00BA4080, vec![this]));
    }
}
