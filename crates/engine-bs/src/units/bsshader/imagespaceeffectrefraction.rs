//! `BSShader/imagespaceeffectrefraction.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ImageSpaceEffectRefraction` (Xbox PDB), 0x68 bytes: an `ImageSpaceEffect` (0x58)
    /// and the normals texture; the PC code reads the same offsets.
    pub struct ImageSpaceEffectRefraction: 0x68 {
        /// `ImageSpaceEffect::pEffectParamList` (Xbox PDB): `NiTPrimitiveArray<ImageSpaceEffectParam *>`.
        0x18 pEffectParamList: Inline<crate::types::NiTArray>,
        /// `spRefractionNormalBuffer` (Xbox PDB): an `ImageSpaceTexture`.
        0x58 spRefractionNormalBuffer: Inline<crate::units::bsshader::imagespacetexture::ImageSpaceTexture>,
    }
}

/// `ImageSpaceEffectRefraction`'s vtable (`010bc560`), stored by the destructor.
const VTABLE: u32 = 0x010b_c560;
/// `00ba3aa0` (`thiscall`, two words: an index and the address of the value): stores the
/// value in that slot of an `NiTArray`.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `ImageSpaceTexture::GetTexture` (`thiscall`).
const TEXTURE_GET_TEXTURE: u32 = 0x00ba_3780;
/// `ImageSpaceTexture::GetRenderedTexture` (`thiscall`).
const TEXTURE_GET_RENDERED_TEXTURE: u32 = 0x00ba_3770;
/// `ImageSpaceTexture::SetTexture` (`thiscall`, one word).
const TEXTURE_SET_TEXTURE: u32 = 0x00ba_37a0;
/// `ImageSpaceTexture::ClearTexture` (`thiscall`).
const TEXTURE_CLEAR_TEXTURE: u32 = 0x00ba_38b0;
/// `ImageSpaceTexture::~ImageSpaceTexture` (`thiscall`).
const TEXTURE_DESTRUCTOR: u32 = 0x00ba_3a00;
/// `ImageSpaceEffect::~ImageSpaceEffect` (`thiscall`).
const IMAGE_SPACE_EFFECT_DESTRUCTOR: u32 = 0x00ba_3d30;
/// `BSTextureManager::ReturnRenderedTexture` (`thiscall`, one word).
const RETURN_RENDERED_TEXTURE: u32 = 0x00b6_da10;
/// The texture manager's borrow (`00b6e110`, `thiscall`, five words).
const BORROW_RENDERED_TEXTURE_00B6E110: u32 = 0x00b6_e110;
/// The global holding the texture manager (`BSTextureManager *`).
const TEXTURE_MANAGER: u32 = 0x011f_91a8;
/// The word passed first to `00b6e110` (read from `011f4748`).
const BORROW_ARGUMENT_011F4748: u32 = 0x011f_4748;
/// A word the borrow is bracketed by: 0 before it, 1 after (`011bf400`).
const BORROW_FLAG_011BF400: u32 = 0x011b_f400;

fn normals(this: Ptr<ImageSpaceEffectRefraction>) -> u32 {
    this.at(ImageSpaceEffectRefraction::spRefractionNormalBuffer)
        .addr()
}

// Translated from 00bd9160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectRefraction::~ImageSpaceEffectRefraction` (engine map; `thiscall`):
/// stores the class's vtable; when the first `pEffectParamList` element is not null it
/// deletes it (its slot 0 with 1) and clears slot 0 (`00ba3aa0` with a zero word). When
/// the normals texture holds a texture, it is given back to the texture manager
/// (`ReturnRenderedTexture` with its rendered texture) and cleared. Then the normals
/// texture's destructor and `ImageSpaceEffect`'s run.
///
/// The exception frame (`FS:[0]`) is not translated.
// Translation incomplete: 00bd9160 - the FS:[0] exception frame (unwinding on a throw) is not translated
pub fn image_space_effect_refraction_destructor(
    e: &mut Engine,
    this: Ptr<ImageSpaceEffectRefraction>,
) {
    e.mem.set_u32(this.addr(), VTABLE);
    let params = this.at(ImageSpaceEffectRefraction::pEffectParamList).addr();
    let elements = e.mem.u32(params + 4);
    if e.mem.u32(elements) != 0 {
        let param = e.mem.u32(elements);
        if param != 0 {
            e.vcall(param, 0, &args![1u32]);
        }
        e.with_stack(4, |e, zero| {
            e.call(ARRAY_SET_AT, &args![params, 0u32, zero]);
        });
    }
    let texture = normals(this);
    if e.call(TEXTURE_GET_TEXTURE, &args![texture]).u32() != 0 {
        let manager = e.mem.u32(TEXTURE_MANAGER);
        let rendered = e.call(TEXTURE_GET_RENDERED_TEXTURE, &args![texture]).u32();
        e.call(RETURN_RENDERED_TEXTURE, &args![manager, rendered]);
        e.call(TEXTURE_CLEAR_TEXTURE, &args![texture]);
    }
    e.call(TEXTURE_DESTRUCTOR, &args![texture]);
    e.call(IMAGE_SPACE_EFFECT_DESTRUCTOR, &args![this]);
}

// Translated from 00bd9230 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (Ghidra made no function of it; `thiscall`, plain `RET`):
/// 1 when the normals texture (+0x58) holds a texture (`ImageSpaceTexture::GetTexture`
/// non-zero), else 0.
pub fn fn_00bd9230(e: &mut Engine, this: Ptr<ImageSpaceEffectRefraction>) -> u32 {
    let texture = e.call(TEXTURE_GET_TEXTURE, &args![normals(this)]).u32();
    u32::from(texture != 0)
}

// Translated from 00bd9380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectRefraction::GetNormalsTexture` (engine map; `thiscall`, `RET 4`: a
/// create flag): when the normals texture holds no texture and the flag is set, borrows
/// one from the texture manager (`00b6e110` with the word at `011f4748`, 0x1f and three
/// zeros, with the word at `011bf400` set to 0 before and 1 after) and stores it
/// (`ImageSpaceTexture::SetTexture`). Returns the rendered texture
/// (`ImageSpaceTexture::GetRenderedTexture`).
pub fn image_space_effect_refraction_get_normals_texture(
    e: &mut Engine,
    this: Ptr<ImageSpaceEffectRefraction>,
    create: bool,
) -> u32 {
    let texture = normals(this);
    let held = e.call(TEXTURE_GET_TEXTURE, &args![texture]).u32();
    if held == 0 && create {
        let manager = e.mem.u32(TEXTURE_MANAGER);
        e.mem.set_u32(BORROW_FLAG_011BF400, held);
        let argument = e.mem.u32(BORROW_ARGUMENT_011F4748);
        let borrowed = e
            .call(
                BORROW_RENDERED_TEXTURE_00B6E110,
                &args![manager, argument, 0x1fu32, held, held, held],
            )
            .u32();
        e.call(TEXTURE_SET_TEXTURE, &args![texture, borrowed]);
        e.mem.set_u32(BORROW_FLAG_011BF400, 1);
    }
    e.call(TEXTURE_GET_RENDERED_TEXTURE, &args![texture]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00bd_9160,
            image_space_effect_refraction_destructor(Ptr<ImageSpaceEffectRefraction>)
        ),
        entry!(
            0x00bd_9230,
            fn_00bd9230(Ptr<ImageSpaceEffectRefraction>) -> u32
        ),
        entry!(
            0x00bd_9380,
            image_space_effect_refraction_get_normals_texture(
                Ptr<ImageSpaceEffectRefraction>,
                bool,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(texture: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(TEXTURE_GET_TEXTURE, move |_, _| texture.into_ret());
        e.register(TEXTURE_GET_RENDERED_TEXTURE, |_, _| 0x4444u32.into_ret());
        e.register(TEXTURE_SET_TEXTURE, |_, _| Ret::default());
        e.register(TEXTURE_CLEAR_TEXTURE, |_, _| Ret::default());
        e.register(TEXTURE_DESTRUCTOR, |_, _| Ret::default());
        e.register(IMAGE_SPACE_EFFECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(RETURN_RENDERED_TEXTURE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.register(BORROW_RENDERED_TEXTURE_00B6E110, |e, _| {
            assert_eq!(e.mem.u32(BORROW_FLAG_011BF400), 0);
            0x5555u32.into_ret()
        });
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x6000);
        e.mem.set_u32(TEXTURE_MANAGER, 0x6666);
        e.mem.set_u32(BORROW_ARGUMENT_011F4748, 0x7777);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    #[test]
    fn destructor_deletes_the_first_param_and_returns_the_texture() {
        let mut e = engine(0x1234);
        let delete = 0x0000_8000u32;
        e.register(delete, |_, _| Ret::default());
        let param = e.mem.alloc(8);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, delete);
        e.mem.set_u32(param, vtable);
        let this = e.mem.alloc(0x68);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(elements, param);
        e.mem.set_u32(this + 0x1c, elements);
        e.call(0x00bd_9160, &[this]);
        let log = calls(&mut e);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x00bd_9160,
                delete,
                ARRAY_SET_AT,
                TEXTURE_GET_TEXTURE,
                TEXTURE_GET_RENDERED_TEXTURE,
                RETURN_RENDERED_TEXTURE,
                TEXTURE_CLEAR_TEXTURE,
                TEXTURE_DESTRUCTOR,
                IMAGE_SPACE_EFFECT_DESTRUCTOR,
            ]
        );
        assert_eq!(log[1].1, vec![param, 1]);
        assert_eq!(log[2].1[..2], [this + 0x18, 0]);
        assert_eq!(log[5].1, vec![0x6666, 0x4444]);
        assert_eq!(log[7].1, vec![this + 0x58]);
        assert_eq!(log[8].1, vec![this]);
        assert_eq!(e.mem.u32(this), VTABLE);
    }

    #[test]
    fn destructor_without_param_or_texture() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x68);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(this + 0x1c, elements);
        e.call(0x00bd_9160, &[this]);
        let order: Vec<u32> = calls(&mut e).iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![
                0x00bd_9160,
                TEXTURE_GET_TEXTURE,
                TEXTURE_DESTRUCTOR,
                IMAGE_SPACE_EFFECT_DESTRUCTOR
            ]
        );
    }

    #[test]
    fn fn_00bd9230_answers_whether_a_texture_is_held() {
        for (texture, expected) in [(0u32, 0u32), (0x10, 1), (0x8000_0000, 1)] {
            let mut e = engine(texture);
            let this = e.mem.alloc(0x68);
            assert_eq!(e.call(0x00bd_9230, &[this]).u32(), expected);
            let log = calls(&mut e);
            assert_eq!(log[1], (TEXTURE_GET_TEXTURE, vec![this + 0x58]));
        }
    }

    #[test]
    fn get_normals_texture_borrows_only_when_empty_and_asked() {
        // Empty and asked: borrows and stores.
        let mut e = engine(0);
        let this = e.mem.alloc(0x68);
        assert_eq!(e.call(0x00bd_9380, &[this, 1]).u32(), 0x4444);
        let log = calls(&mut e);
        let borrow = log
            .iter()
            .find(|(a, _)| *a == BORROW_RENDERED_TEXTURE_00B6E110)
            .unwrap();
        assert_eq!(borrow.1, vec![0x6666, 0x7777, 0x1f, 0, 0, 0]);
        let set = log.iter().find(|(a, _)| *a == TEXTURE_SET_TEXTURE).unwrap();
        assert_eq!(set.1, vec![this + 0x58, 0x5555]);
        assert_eq!(e.mem.u32(BORROW_FLAG_011BF400), 1);

        // Not asked (the low byte is 0), or already holding one: no borrow.
        for (texture, create) in [(0u32, 0x100u32), (0x10, 1)] {
            let mut e = engine(texture);
            let this = e.mem.alloc(0x68);
            assert_eq!(e.call(0x00bd_9380, &[this, create]).u32(), 0x4444);
            let log = calls(&mut e);
            assert!(!log
                .iter()
                .any(|(a, _)| *a == BORROW_RENDERED_TEXTURE_00B6E110));
        }
    }
}
