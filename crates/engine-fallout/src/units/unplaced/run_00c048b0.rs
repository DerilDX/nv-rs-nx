//! `(unplaced)/run_00c048b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c1a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a89760`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0xd8 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00c1a450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_9760;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xd8u32]);
    }
    this
}

/// The `ImageSpaceEffect` base constructor `00ba4080` (unnamed in the engine map; `thiscall`,
/// no stack words) and the `NiTArray` resize `00a90180` (`thiscall` on the array, `RET 4`).
const IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// `ImageSpaceEffect::pTextures` (Xbox PDB, +0x28; the same on PC).
const EFFECT_TEXTURES: u32 = 0x28;
/// The `float` `00c049c0` stores at +0x5c (`01016248`, through `FLD`).
const VALUE_01016248: u32 = 0x0101_6248;
/// `_eh_vector_constructor_iterator_` (`00ec782f`: array, element size, count, constructor,
/// destructor) with the element constructor `004ebb70` and `ImageSpaceTexture`'s destructor
/// (`00ba3a00`, as Ghidra names it) that `00c05420` gives it.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;

// Translated from 00c049c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x78-byte
/// image-space effect `00b94520` stores at index 0xc of the effect array (`EFFECT_MOTION_BLUR`
/// in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is
/// unconfirmed). Runs the base constructor `00ba4080`, sets the vtable `010bcaac`, stores 0.0
/// in the `float`s +0x58 to +0x64, clears the bytes +0x68 to +0x6a and the words +0x6c to
/// +0x74, sizes the texture array (+0x28) to three (`00a90180`), then stores 0.0 at +0x58,
/// the `float` at `01016248` (through the FPU, a signalling NaN quieted) at +0x5c and 0.0 at
/// +0x60 and +0x64. Answers `this`.
// Translation incomplete: 00c049c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the array) is not translated.
pub fn fn_00c049c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let zero = 0.0f32.to_bits();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base, 0x010b_caac);
    for offset in [0x58u32, 0x5c, 0x60, 0x64] {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [0x68u32, 0x69, 0x6a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x6cu32, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 3u32],
    );
    let value = x87_quiet_bits(e.global::<u32>(VALUE_01016248));
    e.mem.set_u32(base + 0x58, zero);
    e.mem.set_u32(base + 0x5c, value);
    e.mem.set_u32(base + 0x60, zero);
    e.mem.set_u32(base + 0x64, zero);
    this
}

// Translated from 00c04d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0xc8-byte
/// image-space effect `00b94520` stores at index 3 of the effect array (`EFFECT_CINEMATIC` in
/// the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed).
/// Runs the base constructor `00ba4080`, clears the byte +0x58 and the word +0x64, sets the
/// vtable `010bcae8` and stores the addresses `011ae684` at +0x5c and `011ae694` at +0x60.
/// Answers `this`.
pub fn fn_00c04d20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u8(base + 0x58, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base, 0x010b_cae8);
    e.mem.set_u32(base + 0x5c, 0x011a_e684);
    e.mem.set_u32(base + 0x60, 0x011a_e694);
    this
}

// Translated from 00c05420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x7c-byte
/// image-space effect `00b94520` stores at index 0 of the effect array (`EFFECT_REFRACTION`
/// in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is
/// unconfirmed). Runs the base constructor `00ba4080`, clears +0x58, sets the vtable
/// `010bcb18`, constructs two 0x10-byte elements at +0x5c (`00ec782f` with `004ebb70` and
/// `00ba3a00`) and sizes the texture array (+0x28) to four (`00a90180`). Answers `this`.
// Translation incomplete: 00c05420 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the elements) is not translated.
pub fn fn_00c05420(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base, 0x010b_cb18);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x5c,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 4u32],
    );
    this
}

// Translated from 00c05680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x68-byte
/// image-space effect `00b94520` stores at index 0xd of the effect array
/// (`EFFECT_VOLUMETRIC_FOG` in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the
/// class name is unconfirmed). Runs the base constructor `00ba4080`, sets the vtable
/// `010bcb3c` and stores 0.0 in the `float`s +0x58 to +0x64. Answers `this`.
pub fn fn_00c05680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base, 0x010b_cb3c);
    for offset in [0x58u32, 0x5c, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1a450, fn_00c1a450(Ptr, u32) -> Ptr),
        entry!(0x00c0_49c0, fn_00c049c0(Ptr) -> Ptr),
        entry!(0x00c0_4d20, fn_00c04d20(Ptr) -> Ptr),
        entry!(0x00c0_5420, fn_00c05420(Ptr) -> Ptr),
        entry!(0x00c0_5680, fn_00c05680(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c1a450_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_9760, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c1a450, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c1a450, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xd8]]);
        assert_eq!(calls_to(&e, 0x00a8_9760), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod effect_constructor_tests {
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

    /// The base constructor fills the object with 0xee bytes and a marker vtable.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, |e, a| {
            for offset in 0..0xc8u32 {
                e.mem.set_u8(a[0] + offset, 0xee);
            }
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(ARRAY_SET_SIZE_00A90180, |e, a| {
            // The vtable is the class's own by then.
            assert_ne!(e.mem.u32(a[0] - EFFECT_TEXTURES), 0x1111);
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xc8);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00c049c0_constructs_the_effect() {
        let (mut e, this) = engine();
        e.map(0x0101_6000, 0x1000);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global::<u32>(VALUE_01016248, 0x7f80_0003);
        assert_eq!(e.call(0x00c0_49c0, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + EFFECT_TEXTURES, 3]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_caac);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 0x7fc0_0003);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.bytes(this + 0x68, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x6b), 0xee);
        for offset in [0x6cu32, 0x70, 0x74] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn fn_00c04d20_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_4d20, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cae8);
        assert_eq!(e.mem.u8(this + 0x58), 0);
        assert_eq!(e.mem.u8(this + 0x59), 0xee);
        assert_eq!(e.mem.u32(this + 0x5c), 0x011a_e684);
        assert_eq!(e.mem.u32(this + 0x60), 0x011a_e694);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert!(calls(&e, ARRAY_SET_SIZE_00A90180).is_empty());
    }

    #[test]
    fn fn_00c05420_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_5420, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c0_5420,
                IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080,
                VECTOR_CONSTRUCT,
                ARRAY_SET_SIZE_00A90180
            ]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![
                this + 0x5c,
                0x10,
                2,
                TEXTURE_CONSTRUCT_004EBB70,
                TEXTURE_DESTRUCT_00BA3A00
            ]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + EFFECT_TEXTURES, 4]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cb18);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    #[test]
    fn fn_00c05680_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_5680, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cb3c);
        for offset in [0x58u32, 0x5c, 0x60, 0x64] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x68), 0xee);
        assert!(calls(&e, ARRAY_SET_SIZE_00A90180).is_empty());
    }
}
