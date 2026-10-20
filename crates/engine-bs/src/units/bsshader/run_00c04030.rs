//! `BSShader/run_00c04030` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010bca64`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BCA64: u32 = 0x010b_ca64;

// Translated from 00c04600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, the vtable `010bca64`, +0x58
/// cleared. Answers `this`.
pub fn fn_00c04600(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BCA64);
    e.mem.set_u32(base + 0x58, 0);
    this
}

/// `BSShader::BSShader` (`00be2200`, `thiscall`), the image-space effect base constructor
/// `00ba4080` (`thiscall`, on the effect part at +0x6c), and the two set-ups the constructor
/// ends with: `00be0870` (unnamed; `thiscall`, no stack words) and `00c04030` (this unit,
/// unnamed; `thiscall`, no stack words).
const BS_SHADER_CONSTRUCT_00BE2200: u32 = 0x00be_2200;
const SET_UP_00BE0870: u32 = 0x00be_0870;
const SET_UP_00C04030: u32 = 0x00c0_4030;
/// The image-space shader's vtable (`010bc8f8`) and its effect part's (`010b0318`) (no RTTI
/// locators before them).
const SHADER_VTABLE_010BC8F8: u32 = 0x010b_c8f8;
const EFFECT_PART_VTABLE_010B0318: u32 = 0x010b_0318;

// Translated from 00c04570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): the image-space shader base constructor,
/// `thiscall`, no stack words. Runs `BSShader::BSShader` (`00be2200`) and the image-space
/// effect base constructor `00ba4080` on +0x6c, stores the vtables `010bc8f8` at +0 and
/// `010b0318` at +0x6c, clears +0xc4 and +0xc8, sets +0x68 to 0x18 and runs `00be0870` and
/// `00c04030` on the shader. Answers `this`.
// Translation incomplete: 00c04570 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the two bases) is not translated.
pub fn fn_00c04570(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT_00BE2200, &args![base]);
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base + 0x6c]);
    e.mem.set_u32(base, SHADER_VTABLE_010BC8F8);
    e.mem.set_u32(base + 0x6c, EFFECT_PART_VTABLE_010B0318);
    e.mem.set_u32(base + 0xc4, 0);
    e.mem.set_u32(base + 0xc8, 0);
    e.mem.set_u32(base + 0x68, 0x18);
    e.call(SET_UP_00BE0870, &args![base]);
    e.call(SET_UP_00C04030, &args![base]);
    this
}

/// The renderer (`011f9508`) and `NiXenonShaderDeclaration::Create` (engine map, `00e76700`,
/// `cdecl`: renderer, entry count, stream count).
const RENDERER_011F9508: u32 = 0x011f_9508;
const DECLARATION_CREATE_00E76700: u32 = 0x00e7_6700;

// Translated from 00c04030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c04030`): the image-space shader's declaration set-up,
/// `thiscall`, no stack words. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer, 2 entries, 1 stream), sets its entries through its vtable slot +0x8c ((0, 0, 0, 2,
/// 0) and (1, 7, 5, 1, 0)), calls its slot +0xb4 and hands it to the shader's slot +0xc0. The
/// declaration is used without a null check.
pub fn fn_00c04030(e: &mut Engine, this: Ptr) {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE_00E76700, &args![renderer, 2u32, 1u32])
        .u32();
    e.vcall(declaration, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(declaration, 0x8c, &args![1u32, 7u32, 5u32, 1u32, 0u32]);
    e.vcall(declaration, 0xb4, &args![]);
    e.vcall(this.addr(), 0xc0, &args![declaration]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_4600, fn_00c04600(Ptr) -> Ptr),
        entry!(0x00c0_4570, fn_00c04570(Ptr) -> Ptr),
        entry!(0x00c0_4030, fn_00c04030(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c04600_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x5c);
        e.mem.write(this, &[0xcc; 0x5c]);
        assert_eq!(e.call(0x00c0_4600, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BCA64);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }
}

#[cfg(test)]
mod shader_base_tests {
    use super::*;

    #[test]
    fn fn_00c04570_builds_both_bases_and_runs_the_set_ups() {
        let mut e = Engine::new();
        for target in [
            BS_SHADER_CONSTRUCT_00BE2200,
            EFFECT_CONSTRUCT_00BA4080,
            SET_UP_00BE0870,
            SET_UP_00C04030,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xcc);
        e.mem.write(this, &[0xcc; 0xcc]);
        assert_eq!(e.call(0x00c0_4570, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010BC8F8);
        assert_eq!(e.mem.u32(this + 0x6c), EFFECT_PART_VTABLE_010B0318);
        assert_eq!(e.mem.u32(this + 0x68), 0x18);
        assert_eq!(e.mem.u32(this + 0xc4), 0);
        assert_eq!(e.mem.u32(this + 0xc8), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BS_SHADER_CONSTRUCT_00BE2200, vec![this]),
                (EFFECT_CONSTRUCT_00BA4080, vec![this + 0x6c]),
                (SET_UP_00BE0870, vec![this]),
                (SET_UP_00C04030, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod declaration_tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    /// The address the rig's constructor double sits at (it gives the shader its vtable).
    const RIG_CONSTRUCTOR: u32 = 0x0f9f_0000;

    #[test]
    fn fn_00c04030_declares_two_entries_and_hands_them_over() {
        let mut e = rig(RIG_CONSTRUCTOR, &[]);
        e.call(RIG_CONSTRUCTOR, &args![SHADER]);
        e.call_log = Some(vec![]);
        e.call(0x00c0_4030, &args![SHADER]);
        let log: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        let d = DECLARATION_0;
        assert_eq!(
            log,
            vec![
                (DECLARATION_CREATE_00E76700, vec![RENDERER, 2, 1]),
                (declaration_slot(0x8c), vec![d, 0, 0, 0, 2, 0]),
                (declaration_slot(0x8c), vec![d, 1, 7, 5, 1, 0]),
                (declaration_slot(0xb4), vec![d]),
                (shader_slot(0xc0), vec![SHADER, d]),
            ]
        );
    }
}
