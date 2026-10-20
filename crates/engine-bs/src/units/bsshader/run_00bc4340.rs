//! `BSShader/run_00bc4340` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bc4830` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BC4830: u32 = 0x00bc_4830;

// Translated from 00bc4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x1a. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`, 5 entries, 1 stream), sets its entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1,
/// 0)) and calls slot +0xb4. Then allocates 0xc0 bytes and runs the constructor `00bc4830`
/// with the declaration (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x140, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bc4af0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bc4af0(e: &mut Engine) -> Ptr {
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
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xc0u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BC4830, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x140, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bad98`; no RTTI locator before it).
const SHADER_VTABLE_010BAD98: u32 = 0x010b_ad98;
/// CRT `_memset` (`00ec61c0`, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;

// Translated from 00bc4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc4340`): the constructor of shader 0x1a, `thiscall` with
/// one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`), stores the vtable
/// `010bad98`, clears the words +0xb4, +0xb8, +0xbc, sets +0x68 to 0x10, the byte +0x24 to 1
/// and +0x6c to 0x243; then the pass pointer at +0xb4 is cleared (its count at +0x64 dropped,
/// `00e80190` at 0), the declaration assigned to the `NiPointer` at +0x28, the `NiPointer`s
/// at +0xb8 and +0xbc cleared (each just set to 0, so these releases find nothing), 0.0
/// stored at +0x70 and 0x40 bytes from +0x74 cleared (`memset`). Answers `this`.
// Translation incomplete: 00bc4830 - the C++ exception frame (FS:[0] chain, unwind state 3 for the base and the pointers) is not translated.
pub fn fn_00bc4830(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_VTABLE_010BAD98);
    e.mem.set_u32(base + 0xb4, 0);
    e.mem.set_u32(base + 0xb8, 0);
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0x68, 0x10);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x243);
    pass_pointer_clear(e, base + 0xb4);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0xb8);
    ni_pointer_clear(e, base + 0xbc);
    e.mem.set_f32(base + 0x70, 0.0);
    e.call(MEMSET, &args![base + 0x74, 0u32, 0x40u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_4830, fn_00bc4830(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_4af0, fn_00bc4af0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bc4af0_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BC4830, &[]);
        let (shader, log) = run(&mut e, 0x00bc_4af0);
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
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xc0]),
                (SHADER_CONSTRUCT_00BC4830, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
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
    fn fn_00bc4830_builds_the_shader() {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xcc; 0xc0]);
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x28, old);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_4830, &args![this, declaration]).u32(), this);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BAD98);
        assert_eq!(e.mem.u32(this + 0x68), 0x10);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x243);
        assert_eq!(e.mem.u32(this + 0x70), 0);
        for offset in [0xb4u32, 0xb8, 0xbc] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert!(calls(&e, PASS_DELETE_00E80190).is_empty());
        assert_eq!(calls(&e, MEMSET), vec![vec![this + 0x74, 0, 0x40]]);
    }
}
