//! `BSShader/run_00bb04b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bb04b0` (this unit's first function, unnamed, `thiscall`, no
/// words).
const SHADER_CONSTRUCT_00BB04B0: u32 = 0x00bb_04b0;

// Translated from 00bb0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the creator `GetShader` (`00b55560`) calls for
/// shader 0x1e. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`, 3 entries, 2 streams), sets its entries through its vtable slot
/// +0x8c (five words each: (0, 0, 0, 2, 0), (1, 7, 5, 1, 0)), calls slot +0x94 with (1, 0,
/// 6, 0x80000000, 2, 5, 1, 0) and slot +0xb4. Then allocates 0xac bytes and runs the
/// constructor `00bb04b0` (a null allocation is used as it is), calls the shader's slots
/// +0x4c, +0x13c, +0x148, +0x11c, then +0xc0 with the declaration, and answers the shader.
// Translation incomplete: 00bb0660 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bb0660(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 3u32, 2u32])
        .u32();
    for entry in [[0u32, 0, 0, 2, 0], [1, 7, 5, 1, 0]] {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(
        declaration,
        0x94,
        &args![1u32, 0u32, 6u32, 0x8000_0000u32, 2u32, 5u32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0xacu32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BB04B0, &args![block]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x13c, 0x148, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

/// `BSShader::BSShader` (engine map, `00be2200`, `thiscall`, no stack words).
const BS_SHADER_CONSTRUCT: u32 = 0x00be_2200;
/// The shader's vtable (`010b8e80`; no RTTI locator before it).
const SHADER_VTABLE_010B8E80: u32 = 0x010b_8e80;
/// The three words copied to +0xa0 (`011a9b7c`, `011a9b80`, `011a9b84`).
const WORDS_011A9B7C: u32 = 0x011a_9b7c;

// Translated from 00bb04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb04b0`): the constructor of shader 0x1e, `thiscall`, no
/// stack words. Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010b8e80`, clears
/// the word +0x9c, stores 0.0 (`FLDZ`) at +0xa0, +0xa4, +0xa8, sets +0x68 to 0x12, stores
/// 0.0 at +0x90, +0x94, +0x98, then copies the words at `011a9b7c`, `011a9b80`, `011a9b84`
/// to +0xa0, +0xa4, +0xa8 and sets the byte +0x24 to 1. Answers `this`.
pub fn fn_00bb04b0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, SHADER_VTABLE_010B8E80);
    e.mem.set_u32(base + 0x9c, 0);
    for offset in [0xa0u32, 0xa4, 0xa8] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0x12);
    for offset in [0x90u32, 0x94, 0x98] {
        e.mem.set_f32(base + offset, 0.0);
    }
    for index in 0..3u32 {
        let word = e.global::<u32>(WORDS_011A9B7C + 4 * index);
        e.mem.set_u32(base + 0xa0 + 4 * index, word);
    }
    e.mem.set_u8(base + 0x24, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_04b0, fn_00bb04b0(Ptr) -> Ptr),
        entry!(0x00bb_0660, fn_00bb0660() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bb0660_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BB04B0, &[]);
        let (shader, log) = run(&mut e, 0x00bb_0660);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE, vec![RENDERER, 3, 2]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (
                    declaration_slot(0x94),
                    vec![d, 1, 0, 6, 0x8000_0000, 2, 5, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0xac]),
                (SHADER_CONSTRUCT_00BB04B0, vec![SHADER]),
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

    #[test]
    fn fn_00bb04b0_sets_the_fields() {
        let mut e = Engine::new();
        e.register(BS_SHADER_CONSTRUCT, |_, a| a[0].into_ret());
        e.map(0x011a_9000, 0x1000);
        for index in 0..3u32 {
            e.set_global::<u32>(WORDS_011A9B7C + 4 * index, 0x100 + index);
        }
        let this = e.mem.alloc(0xac);
        e.mem.write(this, &[0xcc; 0xac]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bb_04b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010B8E80);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x68), 0x12);
        for offset in [0x90u32, 0x94, 0x98, 0x9c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0xa0), 0x100);
        assert_eq!(e.mem.u32(this + 0xa4), 0x101);
        assert_eq!(e.mem.u32(this + 0xa8), 0x102);
        let log = e.call_log.take().unwrap();
        let constructed: Vec<_> = log.iter().filter(|c| c.0 == BS_SHADER_CONSTRUCT).collect();
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0].1, vec![this]);
    }
}
