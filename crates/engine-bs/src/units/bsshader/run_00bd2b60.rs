//! `BSShader/run_00bd2b60` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1f while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_SHADER_5: u32 = 0x1f;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bd2b60` (this unit's first function, unnamed, `thiscall`, no
/// words) and the shader method `00bd2d50` (`thiscall`, no words) called directly.
const SHADER_CONSTRUCT_00BD2B60: u32 = 0x00bd_2b60;
const SHADER_METHOD_00BD2D50: u32 = 0x00bd_2d50;

// Translated from 00bd2e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): the creator `GetShader` (`00b55560`) calls for
/// shader 5. No arguments. With the thread's allocation context (+0x2b4 of the thread block)
/// at 0x1f (put back at the end): makes a declaration (`NiXenonShaderDeclaration::Create`
/// with the renderer `011f9508`, 5 entries, 1 stream), sets its entries through its vtable
/// slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5,
/// 1, 0)), calls slot +0x94 with (0, 4, 8, 0x80000000, 3, 2, 0, 0) and slot +0xb4. Then
/// allocates 0xa8 bytes and runs the constructor `00bd2b60` (a null allocation is used as it
/// is), calls the shader's slots +0x4c and +0x13c, the method `00bd2d50`, slot +0x11c, then
/// +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bd2e70 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn fn_00bd2e70(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_SHADER_5);
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 5u32, 1u32])
        .u32();
    for entry in [
        [0u32, 0, 0, 2, 0],
        [1, 3, 3, 2, 0],
        [2, 5, 4, 3, 0],
        [3, 7, 5, 1, 0],
    ] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 4u32, 8u32, 0x8000_0000u32, 3u32, 2u32, 0u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xa8u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD2B60, &args![block]).u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    e.vcall(shader, 0x13c, &args![]);
    e.call(SHADER_METHOD_00BD2D50, &args![shader]);
    e.vcall(shader, 0x11c, &args![]);
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bbeb8`; no RTTI locator before it).
const VTABLE_010BBEB8: u32 = 0x010b_beb8;

// Translated from 00bd2b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): the constructor of shader 5, `thiscall`, no
/// stack words. Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010bbeb8`,
/// constructs the `NiPointer` arrays at +0x6c (4) and +0x7c (2; both destructor `00b66f20`),
/// clears the word +0x84, stores 0.0 at +0x88 to +0xa4 and sets +0x68 to 8. Answers `this`.
// Translation incomplete: 00bd2b60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the arrays) is not translated.
pub fn fn_00bd2b60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010BBEB8);
    construct_pointer_array(e, base + 0x6c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x7c, 2, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x84, 0);
    for offset in (0x88u32..=0xa4).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 8);
    this
}

// Translated from 00bd2d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd2b60`): shader 5's stages and passes (the creator
/// `00bd2e70` calls it after the vtable slot +0x13c), `thiscall`, no stack words. Gives the
/// pass pointer at +0x84 a new pass (`00e804d0`); makes one texture stage (`CreateNewStage`
/// into a stack pointer, `SetupDefaultStage` with index 0 and 3, 2, 1, no filter mode) and puts
/// it on the pass (`NiD3DPass::SetStage` with the pass's word at +0x14); assigns the shader's
/// +0x6c to the pass's vertex shader `NiPointer` (+0x5c) and +0x7c to its pixel shader
/// `NiPointer` (+0x44); then drops the stage reference.
// Translation incomplete: 00bd2d50 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage pointer) is not translated.
pub fn fn_00bd2d50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let slot = base + 0x84;
    e.call(NEW_PASS_00E804D0, &args![slot]);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.call(CREATE_NEW_STAGE, &args![local]);
        let stage = e.mem.u32(local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 3u32, 2u32, 1u32]);
        let pass = e.mem.u32(slot);
        let stage = e.mem.u32(local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
        let pass = e.mem.u32(slot);
        let vertex = e.mem.u32(base + 0x6c);
        ni_pointer_assign(e, pass + 0x5c, vertex);
        let pass = e.mem.u32(slot);
        let pixel = e.mem.u32(base + 0x7c);
        ni_pointer_assign(e, pass + 0x44, pixel);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_2d50, fn_00bd2d50(Ptr)),
        entry!(0x00bd_2b60, fn_00bd2b60(Ptr) -> Ptr),
        entry!(0x00bd_2e70, fn_00bd2e70() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd2e70_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD2B60, &[SHADER_METHOD_00BD2D50]);
        let (shader, log) = run(&mut e, 0x00bd_2e70);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 5, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 3, 3, 2, 0]),
                (declaration_slot(0x8c), vec![d, 2, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 3, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 4, 8, 0x8000_0000, 3, 2, 0, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xa8]),
                (SHADER_CONSTRUCT_00BD2B60, vec![SHADER]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (SHADER_METHOD_00BD2D50, vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_SHADER_5);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bd2b60_builds_the_shader() {
        let mut e = rig();
        let this = e.mem.alloc(0xa8);
        e.mem.write(this, &[0xcc; 0xa8]);
        assert_eq!(e.call(0x00bd_2b60, &args![this]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010BBEB8);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x7c, 2, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x84u32..=0xa4).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x68), 8);
    }
}

#[cfg(test)]
mod stages_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00bd2d50_builds_one_pass_with_one_stage() {
        let mut e = pass_rig();
        let this = e.mem.alloc(0xa8);
        let vertex = object(&mut e, 1);
        let pixel = object(&mut e, 1);
        e.mem.set_u32(this + 0x6c, vertex);
        e.mem.set_u32(this + 0x7c, pixel);
        e.call(SHADER_METHOD_00BD2D50, &args![this]);
        assert_eq!(calls(&e, NEW_PASS_00E804D0), vec![vec![this + 0x84]]);
        let pass = e.mem.u32(this + 0x84);
        let set = calls(&e, PASS_SET_STAGE);
        assert_eq!(set.len(), 1);
        assert_eq!(set[0][..2], [pass, 0]);
        let stage = set[0][2];
        assert_eq!(
            calls(&e, SETUP_DEFAULT_STAGE),
            vec![vec![stage, 0, 3, 2, 1]]
        );
        assert!(calls(&e, SET_FILTER_MODE).is_empty());
        assert_eq!(e.mem.u32(stage + 0x60), 1);
        assert_eq!(e.mem.u32(pass + 0x5c), vertex);
        assert_eq!(e.mem.u32(pass + 0x44), pixel);
        assert_eq!(e.mem.u32(vertex + 4), 2);
    }
}
