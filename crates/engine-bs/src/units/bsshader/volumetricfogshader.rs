//! `BSShader/volumetricfogshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `VolumetricFogShader::VolumetricFogShader` (engine map, `00bc58a0`, `thiscall`, no words).
const CONSTRUCT: u32 = 0x00bc_58a0;

// Translated from 00bc59b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VolumetricFogShader::CreateShader` (engine map): the creator `GetShader` (`00b55560`)
/// calls for shader 0x22. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 2 entries, 1 stream),
/// sets its one entry through its vtable slot +0x8c (five words: (0, 0, 0, 2, 0)) and calls
/// slot +0xb4. Then allocates 0xa8 bytes and runs the constructor (`00bc58a0`; a null
/// allocation is used as it is), calls the shader's slots +0x4c, +0x13c, +0x148, +0x11c, then
/// +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bc59b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn volumetric_fog_shader_create_shader(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 2u32, 1u32])
        .u32();
    e.vcall(declaration, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xa8u32]).u32();
    let shader = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The volumetric fog shader's vtable (`010baf60`; no RTTI locator before it).
const VOLUMETRIC_FOG_VTABLE_010BAF60: u32 = 0x010b_af60;

// Translated from 00bc58a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VolumetricFogShader::VolumetricFogShader` (Xbox PDB), `thiscall`, no stack words. Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010baf60`, constructs the `NiPointer`
/// arrays at +0x6c (3, destructor `00b79930`), +0x78 and +0x80 (2 each, destructor
/// `00b66f20`), stores 0.0 at +0x88 to +0xa4 and sets +0x68 to 0x13. Answers `this`.
// Translation incomplete: 00bc58a0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn volumetric_fog_shader_volumetric_fog_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VOLUMETRIC_FOG_VTABLE_010BAF60);
    construct_pointer_array(e, base + 0x6c, 3, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x78, 2, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x80, 2, DESTRUCT_00B66F20);
    for offset in (0x88u32..=0xa4).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0x13);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_58a0, volumetric_fog_shader_volumetric_fog_shader(Ptr) -> Ptr),
        entry!(
            0x00bc_59b0,
            volumetric_fog_shader_create_shader() -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn volumetric_fog_shader_create_shader_makes_the_declaration_then_the_shader() {
        let mut e = rig(CONSTRUCT, &[]);
        let (shader, log) = run(&mut e, 0x00bc_59b0);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xa8]),
                (CONSTRUCT, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x148), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn volumetric_fog_shader_constructor_builds_the_shader() {
        let mut e = rig();
        let this = e.mem.alloc(0xa8);
        e.mem.write(this, &[0xcc; 0xa8]);
        assert_eq!(e.call(CONSTRUCT, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VOLUMETRIC_FOG_VTABLE_010BAF60);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 3, DESTRUCT_00B79930),
                pointer_array(this + 0x78, 2, DESTRUCT_00B66F20),
                pointer_array(this + 0x80, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x88u32..=0xa4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 0x13);
    }
}
