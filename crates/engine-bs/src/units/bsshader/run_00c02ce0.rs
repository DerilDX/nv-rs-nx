//! `BSShader/run_00c02ce0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The two effects' vtables (`010bc8a4`, `010bc8d0`; no RTTI locator before them).
const VTABLE_010BC8A4: u32 = 0x010b_c8a4;
const VTABLE_010BC8D0: u32 = 0x010b_c8d0;

// Translated from 00c033c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc8a4` and clears the 16-byte block at +0x58 (the bytes +0x58 to +0x5a, then the words
/// +0x5c to +0x64). Answers `this`.
pub fn fn_00c033c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC8A4);
    for offset in [0x58u32, 0x59, 0x5a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x5cu32, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00c03810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc8d0`, constructs the two 16-byte textures at +0x5c (`eh vector constructor iterator`
/// with `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`) and sizes the array at +0x28 to
/// 4 (`00a90180`). Answers `this`.
// Translation incomplete: 00c03810 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the textures) is not translated.
pub fn fn_00c03810(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC8D0);
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x5c,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_33c0, fn_00c033c0(Ptr) -> Ptr),
        entry!(0x00c0_3810, fn_00c03810(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00c033c0_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00c0_33c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8A4);
        assert_eq!(e.mem.bytes(this + 0x58, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x5b), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }

    #[test]
    fn fn_00c03810_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x7c);
        assert_eq!(e.call(0x00c0_3810, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8D0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x5c,
                        0x10,
                        2,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}
