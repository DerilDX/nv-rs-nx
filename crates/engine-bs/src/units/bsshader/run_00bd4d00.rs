//! `BSShader/run_00bd4d00` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bd5670` (unnamed, `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BD5670: u32 = 0x00bd_5670;

// Translated from 00bd5920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the creator `GetShader` (`00b55560`) calls for
/// shader 3. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 4 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (five words each: (0, 0, 0, 2, 0), (1, 5, 4, 3, 0), (2, 7, 5, 1, 0)), calls slot +0x94
/// with (0, 3, 6, 0x80000000, 0, 5, 1, 0) and slot +0xb4. Then allocates 0x100 bytes and runs
/// the constructor `00bd5670` with the declaration (a null allocation is used as it is),
/// calls the shader's slots +0x4c, +0x13c, +0x11c, +0x140, then +0xc0 with the declaration,
/// and answers the shader.
// Translation incomplete: 00bd5920 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd5920(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 4u32, 1u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 5, 4, 3, 0], [2, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![0u32, 3u32, 6u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x100u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BD5670, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x11c, 0x140] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010bc260`; no RTTI locator before it).
const VTABLE_010BC260: u32 = 0x010b_c260;
/// The four words copied to +0xa0 (`011a9bd0`) and the shader model setting (`011f91c0`).
const COLOR_011A9BD0: u32 = 0x011a_9bd0;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bd5670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd4d00`): the constructor the creator `00bd5970` runs,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010bc260`, constructs the `NiPointer` arrays at +0x6c (1, destructor
/// `00b79930`), +0x7c (4) and +0x8c (2; destructor `00b66f20`), stores 0.0 at +0xa0 to +0xfc,
/// sets +0x68 to 7, clears (releasing what they hold) the `NiPointer`s at +0x7c to +0x88 and
/// +0x8c, +0x90, assigns the declaration to the `NiPointer` at +0x28, stores 0.0 at +0x70,
/// copies the four words at `011a9bd0` to +0xa0, sets the 16-bit count at +0x9c to 0xe4 (0x50
/// when the setting at `011f91c0` is below 2, signed), allocates that many 0x10-byte elements
/// from the memory manager into +0x94 and clears +0x98. Answers `this`.
// Translation incomplete: 00bd5670 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn fn_00bd5670(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC260);
    construct_pointer_array(e, base + 0x6c, 1, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x7c, 4, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x8c, 2, DESTRUCT_00B66F20);
    for offset in (0xa0u32..=0xfc).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 7);
    for index in 0..4u32 {
        ni_pointer_clear(e, base + 0x7c + 4 * index);
    }
    for index in 0..2u32 {
        ni_pointer_clear(e, base + 0x8c + 4 * index);
    }
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    e.mem.set_f32(base + 0x70, 0.0);
    for index in 0..4u32 {
        let word = e.global::<u32>(COLOR_011A9BD0 + 4 * index);
        e.mem.set_u32(base + 0xa0 + 4 * index, word);
    }
    let count: u16 = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
        0x50
    } else {
        0xe4
    };
    e.mem.set_u16(base + 0x9c, count);
    let count = e.mem.u16(base + 0x9c);
    let elements = allocate_array(e, u32::from(count), 0x10);
    e.mem.set_u32(base + 0x94, elements);
    e.mem.set_u32(base + 0x98, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_5670, fn_00bd5670(Ptr, Ptr) -> Ptr),
        entry!(0x00bd_5920, fn_00bd5920() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bd5920_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BD5670, &[]);
        let (shader, log) = run(&mut e, 0x00bd_5920);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 4, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 5, 4, 3, 0]),
                (declaration_slot(0x8c), vec![d, 2, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 0, 3, 6, 0x8000_0000, 0, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x100]),
                (SHADER_CONSTRUCT_00BD5670, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BD0 + 4 * index, 0x40 + index);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bd_5670, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bd5670_builds_the_shader() {
        let (e, this, declaration) = construct(2);
        assert_eq!(e.mem.u32(this), VTABLE_010BC260);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 1, DESTRUCT_00B79930),
                pointer_array(this + 0x7c, 4, DESTRUCT_00B66F20),
                pointer_array(this + 0x8c, 2, DESTRUCT_00B66F20),
            ]
        );
        for index in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0xa0 + 4 * index), 0x40 + index);
        }
        for offset in (0xb0u32..=0xfc).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x70), 0);
        assert_eq!(e.mem.u32(this + 0x68), 7);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u16(this + 0x9c), 0xe4);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40]]
        );
        assert_eq!(e.mem.u32(this + 0x98), 0);
    }

    #[test]
    fn fn_00bd5670_small_count_below_model_2() {
        let (e, this, _) = construct(1);
        assert_eq!(e.mem.u16(this + 0x9c), 0x50);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x500]]
        );
    }
}
