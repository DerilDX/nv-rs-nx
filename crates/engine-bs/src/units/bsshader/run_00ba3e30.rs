//! `BSShader/run_00ba3e30` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010b80a8`; no RTTI locator before it).
const VTABLE_010B80A8: u32 = 0x010b_80a8;
/// The shared element count (`012003ec`), set on first use from the shader model setting
/// (`011f91c0`): 0x1a below model 2, else 0x4e.
const ELEMENT_COUNT_012003EC: u32 = 0x0120_03ec;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00ba53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the constructor the creator `00ba58b0` runs,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010b80a8`, clears +0x74 to +0x80, stores 0.0 at +0xec to +0x128, sets
/// +0x68 to 0xe, the byte +0x24 to 1 and +0x6c to 0x241; clears the pass pointers at +0x74 and
/// +0x78 (their counts at +0x64 dropped, `00e80190` at 0), assigns the declaration to the
/// `NiPointer` at +0x28, clears the `NiPointer`s at +0x7c and +0x80, stores 0.0 at +0xd4 to
/// +0x128; the element count at `012003ec` becomes 0x1a (shader model below 2, signed) or
/// 0x4e when it is 0; that many 0x30-byte elements (`operator new[]` size check) from the
/// memory manager go to +0x70 and are cleared, and 0x50 bytes at +0x84 are cleared
/// (`memset`). Answers `this`.
// Translation incomplete: 00ba53f0 - the C++ exception frame (FS:[0] chain, unwind state 4 for the base and the pointers) is not translated.
pub fn fn_00ba53f0(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010B80A8);
    for offset in [0x74u32, 0x78, 0x7c, 0x80] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in (0xecu32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0xe);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x241);
    pass_pointer_clear(e, base + 0x74);
    pass_pointer_clear(e, base + 0x78);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0x7c);
    ni_pointer_clear(e, base + 0x80);
    for offset in (0xd4u32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    let mut count = e.global::<u32>(ELEMENT_COUNT_012003EC);
    if count == 0 {
        count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x1a
        } else {
            0x4e
        };
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
    }
    let buffer = allocate_array(e, count, 0x30);
    e.mem.set_u32(base + 0x70, buffer);
    e.call(MEMSET, &args![buffer, 0u32, count.wrapping_mul(0x30)]);
    e.call(MEMSET, &args![base + 0x84, 0u32, 0x50u32]);
    this
}

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The element constructor `004ebb70` (unit `teswater.cpp`, unnamed) and
/// `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`) the texture array passes to the
/// `eh vector constructor iterator`.
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The two effects' vtables (`010b8008`, `010b8030`; no RTTI locator before them).
const VTABLE_010B8008: u32 = 0x010b_8008;
const VTABLE_010B8030: u32 = 0x010b_8030;
/// The three words `00ba4880` copies (`011a9490` to `011a9498`, to `012003d4` to `012003dc`).
const WORDS_011A9490: u32 = 0x011a_9490;
const WORDS_012003D4: u32 = 0x0120_03d4;

// Translated from 00ba4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the constructor of the 0x60-byte effects
/// `00b94520` makes in pairs, `thiscall`, no stack words. Runs the base constructor
/// `00ba4080`, stores the vtable `010b8008`, sizes the array at +0x28 to 4 (`00a90180`) and
/// sets +0x58 to -1. Answers `this`.
// Translation incomplete: 00ba4640 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00ba4640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8008);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    e.mem.set_u32(base + 0x58, 0xffff_ffff);
    this
}

// Translated from 00ba4880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010b8030`,
/// constructs the three 16-byte textures at +0x58 (`eh vector constructor iterator` with
/// `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 4
/// (`00a90180`) and copies the three words at `011a9490` to `012003d4` (stored last word
/// first). Answers `this`.
// Translation incomplete: 00ba4880 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the textures) is not translated.
pub fn fn_00ba4880(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8030);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x58,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    let third = e.global::<u32>(WORDS_011A9490 + 8);
    let first = e.global::<u32>(WORDS_011A9490);
    let second = e.global::<u32>(WORDS_011A9490 + 4);
    e.set_global(WORDS_012003D4 + 8, third);
    e.set_global(WORDS_012003D4, first);
    e.set_global(WORDS_012003D4 + 4, second);
    this
}

/// The image-space effect's vtable (`010b7fe4`) and the vtables of its five arrays
/// (`010b8004` for the two at +8 and +0x18, `010b9d24` for the three at +0x28, +0x38 and
/// +0x48; no RTTI locators before them).
const EFFECT_VTABLE_010B7FE4: u32 = 0x010b_7fe4;
const ARRAY_VTABLE_010B8004: u32 = 0x010b_8004;
const ARRAY_VTABLE_010B9D24: u32 = 0x010b_9d24;

// Translated from 00ba4080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the image-space effect base constructor every
/// image-space effect's constructor starts with, `thiscall`, no stack words. Stores the vtable
/// `010b7fe4`, sets the byte +5 and clears the byte +4, then makes five empty `NiTArray`s at
/// +8, +0x18, +0x28, +0x38 and +0x48 (each: max size 0, grow-by 1, size and element count 0,
/// no elements, its vtable). Answers `this`.
// Translation incomplete: 00ba4080 - the C++ exception frame (FS:[0] chain set up and removed, no unwind state used) is not translated.
pub fn fn_00ba4080(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, EFFECT_VTABLE_010B7FE4);
    e.mem.set_u8(base + 5, 1);
    e.mem.set_u8(base + 4, 0);
    for (array, vtable) in [
        (8u32, ARRAY_VTABLE_010B8004),
        (0x18, ARRAY_VTABLE_010B8004),
        (0x28, ARRAY_VTABLE_010B9D24),
        (0x38, ARRAY_VTABLE_010B9D24),
        (0x48, ARRAY_VTABLE_010B9D24),
    ] {
        let array = base + array;
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 1);
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u16(array + 0xc, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array, vtable);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_53f0, fn_00ba53f0(Ptr, Ptr) -> Ptr),
        entry!(0x00ba_4640, fn_00ba4640(Ptr) -> Ptr),
        entry!(0x00ba_4880, fn_00ba4880(Ptr) -> Ptr),
        entry!(0x00ba_4080, fn_00ba4080(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.map(0x0120_0000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x130);
        e.mem.write(this, &[0xcc; 0x130]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00ba_53f0, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00ba53f0_builds_the_shader() {
        let (e, this, declaration) = construct(0, 2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010B80A8);
        for offset in [0x74u32, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in (0xd4u32..=0x128).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xd0), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0x68), 0xe);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x241);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x4e);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xea0]]
        );
        let buffer = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, MEMSET),
            vec![vec![buffer, 0, 0xea0], vec![this + 0x84, 0, 0x50]]
        );
    }

    #[test]
    fn fn_00ba53f0_count_follows_the_model_once() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x1a);
        let (e, _, _) = construct(3, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 3);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x90]]);
    }
}

#[cfg(test)]
mod effect_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00ba4640_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00ba_4640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8008);
        assert_eq!(e.mem.u32(this + 0x58), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }

    #[test]
    fn fn_00ba4880_builds_the_effect_and_copies_the_words() {
        let mut e = engine();
        for index in 0..3 {
            e.set_global(WORDS_011A9490 + 4 * index, 0x0e00_0000 + index);
        }
        let this = e.mem.alloc(0x88);
        assert_eq!(e.call(0x00ba_4880, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8030);
        for index in 0..3 {
            assert_eq!(
                e.global::<u32>(WORDS_012003D4 + 4 * index),
                0x0e00_0000 + index
            );
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT,
                    vec![
                        this + 0x58,
                        0x10,
                        3,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}

#[cfg(test)]
mod effect_base_tests {
    use super::*;

    #[test]
    fn fn_00ba4080_makes_the_five_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 0x58]);
        assert_eq!(e.call(0x00ba_4080, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), EFFECT_VTABLE_010B7FE4);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(e.mem.u16(this + 6), 0xcccc);
        let vtables = [
            ARRAY_VTABLE_010B8004,
            ARRAY_VTABLE_010B8004,
            ARRAY_VTABLE_010B9D24,
            ARRAY_VTABLE_010B9D24,
            ARRAY_VTABLE_010B9D24,
        ];
        for (index, vtable) in vtables.iter().enumerate() {
            let array = this + 8 + 0x10 * index as u32;
            assert_eq!(e.mem.u32(array), *vtable);
            assert_eq!(e.mem.bytes(array + 4, 0xa), vec![0; 0xa]);
            assert_eq!(e.mem.u16(array + 0xe), 1);
        }
    }
}
