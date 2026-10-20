//! `BSShader/tallgrassshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread block's word at +0x2b4 (the allocation context), 0x1c while the shader is made.
const TLS_CONTEXT: u32 = 0x2b4;
const CONTEXT_TALL_GRASS: u32 = 0x1c;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `NiXenonShaderDeclaration::Create` (engine map, `00e76700`, `cdecl`: renderer, entry count,
/// stream count).
const DECLARATION_CREATE: u32 = 0x00e7_6700;
/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader's constructor `00bacd00` (engine map unit `tallgrassshader.cpp`, `thiscall`,
/// `RET 4`: the declaration).
const TALL_GRASS_SHADER_CONSTRUCT_00BACD00: u32 = 0x00ba_cd00;

// Translated from 00bacf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 2. No arguments. With the thread's allocation context
/// (+0x2b4 of the thread block) at 0x1c (put back at the end): makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 5 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0)), calls slot +0x94 with (0, 4, 6, 0x80000000,
/// 0, 5, 1, 0) and slot +0xb4. Then allocates 0x1e0 bytes and runs the constructor `00bacd00`
/// with the declaration (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x11c, +0x140, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bacf00 - the C++ exception frame (FS:[0] chain, unwind state for the allocation) is not translated.
pub fn fn_00bacf00(e: &mut Engine) -> Ptr {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem
        .set_u32(tls.wrapping_add(TLS_CONTEXT), CONTEXT_TALL_GRASS);
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
        &args![0u32, 4u32, 6u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x1e0u32]).u32();
    let shader = if block != 0 {
        e.call(
            TALL_GRASS_SHADER_CONSTRUCT_00BACD00,
            &args![block, declaration],
        )
        .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x11c, 0x140] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The tall grass shader's vtable (`010b8980`; no RTTI locator before it).
const TALL_GRASS_VTABLE_010B8980: u32 = 0x010b_8980;
/// The four words copied (`011a9bd0`..`011a9bdc` into `01200570`..`0120057c`).
const WORDS_011A9BD0: u32 = 0x011a_9bd0;
const WORDS_01200570: u32 = 0x0120_0570;
/// The shader model setting (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bacd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `tallgrassshader.cpp`): the tall grass shader's constructor,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010b8980`, constructs the `NiPointer` arrays at +0x6c (0x2c, destructor
/// `00b79930`), +0x128 (0x18) and +0x188 (7, both destructor `00b66f20`), stores 0.0 at
/// +0x1b0 to +0x1dc, sets +0x68 to 6, assigns the declaration to the `NiPointer` at +0x28,
/// stores 0.0 at +0x11c, copies the four words at `011a9bd0` to `01200570`, sets the 16-bit
/// count at +0x1ac to 0xe4 (0x50 when the setting at `011f91c0` is below 2, signed), allocates
/// that many 0x10-byte elements from the memory manager into +0x1a4 and clears +0x1a8.
/// Answers `this`.
// Translation incomplete: 00bacd00 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the base and the arrays) is not translated.
pub fn fn_00bacd00(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, TALL_GRASS_VTABLE_010B8980);
    construct_pointer_array(e, base + 0x6c, 0x2c, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x128, 0x18, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0x188, 7, DESTRUCT_00B66F20);
    for offset in (0x1b0u32..=0x1dc).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 6);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    e.mem.set_f32(base + 0x11c, 0.0);
    for index in 0..4u32 {
        let word = e.global::<u32>(WORDS_011A9BD0 + 4 * index);
        e.set_global::<u32>(WORDS_01200570 + 4 * index, word);
    }
    let count: u16 = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
        0x50
    } else {
        0xe4
    };
    e.mem.set_u16(base + 0x1ac, count);
    let count = e.mem.u16(base + 0x1ac);
    let elements = allocate_array(e, u32::from(count), 0x10);
    e.mem.set_u32(base + 0x1a4, elements);
    e.mem.set_u32(base + 0x1a8, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_cd00, fn_00bacd00(Ptr, Ptr) -> Ptr),
        entry!(0x00ba_cf00, fn_00bacf00() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bacf00_makes_the_declaration_then_the_shader() {
        let mut e = rig(TALL_GRASS_SHADER_CONSTRUCT_00BACD00, &[]);
        let (shader, log) = run(&mut e, 0x00ba_cf00);
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
                    vec![d, 0, 4, 6, 0x8000_0000, 0, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x1e0]),
                (TALL_GRASS_SHADER_CONSTRUCT_00BACD00, vec![SHADER, d]),
                (shader_slot(0x4c), vec![SHADER]),
                (shader_slot(0x13c), vec![SHADER]),
                (shader_slot(0x11c), vec![SHADER]),
                (shader_slot(0x140), vec![SHADER]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
        assert_eq!(e.mem.u32(CONTEXT_SEEN), CONTEXT_TALL_GRASS);
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
        e.map(0x0120_0000, 0x1000);
        for index in 0..4u32 {
            e.set_global::<u32>(WORDS_011A9BD0 + 4 * index, 0x70 + index);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x1e0);
        e.mem.write(this, &[0xcc; 0x1e0]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 1);
        assert_eq!(e.call(0x00ba_cd00, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bacd00_builds_the_shader() {
        let (e, this, declaration) = construct(3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), TALL_GRASS_VTABLE_010B8980);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x6c, 0x2c, DESTRUCT_00B79930),
                pointer_array(this + 0x128, 0x18, DESTRUCT_00B66F20),
                pointer_array(this + 0x188, 7, DESTRUCT_00B66F20),
            ]
        );
        for offset in (0x1b0u32..=0x1dc).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x68), 6);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 2);
        assert_eq!(e.mem.u32(this + 0x11c), 0);
        for index in 0..4u32 {
            assert_eq!(e.global::<u32>(WORDS_01200570 + 4 * index), 0x70 + index);
        }
        assert_eq!(e.mem.u16(this + 0x1ac), 0xe4);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xe40]]
        );
        assert_ne!(e.mem.u32(this + 0x1a4), 0);
        assert_eq!(e.mem.u32(this + 0x1a8), 0);
    }

    #[test]
    fn fn_00bacd00_small_count_below_model_2() {
        let (e, this, _) = construct(1);
        assert_eq!(e.mem.u16(this + 0x1ac), 0x50);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x500]]
        );
        let (e, this, _) = construct(-7);
        assert_eq!(e.mem.u16(this + 0x1ac), 0x50);
    }
}
