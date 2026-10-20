//! `BSShader/run_00bcc570` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The constructors of the three shaders made here (unnamed, `thiscall`): `00bd12c0` and
/// `00bcf7f0` (`RET 8`: two declarations), `00bcd390` (`RET 4`: one declaration).
const SHADER_CONSTRUCT_00BD12C0: u32 = 0x00bd_12c0;
const SHADER_CONSTRUCT_00BCF7F0: u32 = 0x00bc_f7f0;
const SHADER_CONSTRUCT_00BCD390: u32 = 0x00bc_d390;

/// The six entries (vtable slot +0x8c, five words each) of the six-entry declarations here.
const SIX_ENTRIES: [[u32; 5]; 6] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 5, 4, 3, 0],
    [3, 7, 5, 1, 0],
    [4, 0xe, 0xd, 2, 0],
    [5, 0xf, 0xe, 2, 0],
];
/// The eight entries of the second declaration of `00bd14b0` and `00bcfb00`.
const EIGHT_ENTRIES: [[u32; 5]; 8] = [
    [0, 0, 0, 2, 0],
    [1, 1, 1, 3, 0],
    [2, 2, 2, 4, 0],
    [3, 3, 3, 2, 0],
    [4, 5, 4, 3, 0],
    [5, 7, 5, 1, 0],
    [6, 0xe, 0xd, 2, 0],
    [7, 0xf, 0xe, 2, 0],
];

/// The shared body of `00bd14b0` and `00bcfb00` (each a separate function in the exe, with
/// the same code but for the size and the constructor): two declarations (6 and 8 entries,
/// 1 stream each) made one after the other, then filled and ended (slot +0xb4) in that
/// order; the shader made with both, its slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0
/// with the first declaration.
fn two_declaration_shader(e: &mut Engine, size: u32, construct: u32) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let first = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let second = e
        .call(DECLARATION_CREATE, &args![renderer, 8u32, 1u32])
        .u32();
    for entry in SIX_ENTRIES {
        e.vcall(first, 0x8c, &entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in EIGHT_ENTRIES {
        e.vcall(second, 0x8c, &entry);
    }
    e.vcall(second, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![size]).u32();
    let shader = if block != 0 {
        e.call(construct, &args![block, first, second]).u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

// Translated from 00bd14b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xd. No arguments. Makes two declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508`: 6 entries and 8 entries, 1 stream each), sets the first's entries
/// through its vtable slot +0x8c ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0), (3, 7,
/// 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0)) and ends it (slot +0xb4), then the
/// second's ((0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0), (4, 5, 4, 3,
/// 0), (5, 7, 5, 1, 0), (6, 0xe, 0xd, 2, 0), (7, 0xf, 0xe, 2, 0)) and ends it. Allocates 0xec
/// bytes and runs the constructor `00bd12c0` with both (a null allocation is used as it is),
/// calls the shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the first
/// declaration, and answers the shader.
// Translation incomplete: 00bd14b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bd14b0(e: &mut Engine) -> Ptr {
    two_declaration_shader(e, 0xec, SHADER_CONSTRUCT_00BD12C0)
}

// Translated from 00bcfb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xe. The same steps as `00bd14b0` (two declarations of 6 and 8 entries with the
/// same entries), with 0x128 bytes and the constructor `00bcf7f0`.
// Translation incomplete: 00bcfb00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcfb00(e: &mut Engine) -> Ptr {
    two_declaration_shader(e, 0x128, SHADER_CONSTRUCT_00BCF7F0)
}

// Translated from 00bcd680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the creator `GetShader` (`00b55560`) calls for
/// shader 0xf. No arguments. Makes a declaration (`NiXenonShaderDeclaration::Create` with the
/// renderer `011f9508`, 6 entries, 1 stream), sets its entries through its vtable slot +0x8c
/// (the six of `00bd14b0`'s first declaration) and calls slot +0xb4. Then allocates 0x160
/// bytes and runs the constructor `00bcd390` with the declaration (a null allocation is used
/// as it is), calls the shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the
/// declaration, and answers the shader.
// Translation incomplete: 00bcd680 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bcd680(e: &mut Engine) -> Ptr {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let declaration = e
        .call(DECLARATION_CREATE, &args![renderer, 6u32, 1u32])
        .u32();
    for entry in SIX_ENTRIES {
        e.vcall(declaration, 0x8c, &entry);
    }
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x160u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BCD390, &args![block, declaration])
            .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![declaration]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shared shader constructor `00b79b00` (unnamed; `thiscall`, four stack words, each an
/// object kept by an `NiPointer`).
const SHADER_BASE_CONSTRUCT_00B79B00: u32 = 0x00b7_9b00;
/// The vtables of the three shaders (`010bb7a8`, `010bb980`, `010bbb50`; no RTTI locators
/// before them).
const VTABLE_010BB7A8: u32 = 0x010b_b7a8;
const VTABLE_010BB980: u32 = 0x010b_b980;
const VTABLE_010BBB50: u32 = 0x010b_bb50;

// Translated from 00bcd390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bcd680` runs,
/// `thiscall` with one stack word (the declaration). Runs `00b79b00` with (the declaration,
/// 0, 0, 0), stores the vtable `010bb7a8`, constructs the `NiPointer` arrays at +0x8c (0x14)
/// and +0xdc (0x21; both destructor `00b66f20`), sets +0x68 to 3 and clears the byte +0x64,
/// assigns the declaration to the `NiPointer` at +0x6c, clears the `NiPointer` at +0x70 and
/// the word +0x60. Answers `this`.
// Translation incomplete: 00bcd390 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bcd390(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, declaration, 0u32, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BB7A8);
    construct_pointer_array(e, base + 0x8c, 0x14, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xdc, 0x21, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 3);
    e.mem.set_u8(base + 0x64, 0);
    ni_pointer_assign(e, base + 0x6c, declaration.addr());
    ni_pointer_clear(e, base + 0x70);
    e.mem.set_u32(base + 0x60, 0);
    this
}

// Translated from 00bcf7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bcfb00` runs,
/// `thiscall` with two stack words (the two declarations). Runs `00b79b00` with (first,
/// second, 0, 0), stores the vtable `010bb980`, constructs the `NiPointer` arrays at +0x8c
/// (0x1a) and +0xf4 (0xd; both destructor `00b66f20`), sets +0x68 to 4 and clears the byte
/// +0x64, assigns the declarations to the `NiPointer`s at +0x6c and +0x70 and clears the word
/// +0x60. Answers `this`.
// Translation incomplete: 00bcf7f0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the arrays) is not translated.
pub fn fn_00bcf7f0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, first, second, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BB980);
    construct_pointer_array(e, base + 0x8c, 0x1a, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xf4, 0xd, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0x68, 4);
    e.mem.set_u8(base + 0x64, 0);
    ni_pointer_assign(e, base + 0x6c, first.addr());
    ni_pointer_assign(e, base + 0x70, second.addr());
    e.mem.set_u32(base + 0x60, 0);
    this
}

// Translated from 00bd12c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bcc570`): the constructor the creator `00bd14b0` runs,
/// `thiscall` with two stack words (the two declarations). Runs `00b79b00` with (first,
/// second, 0, 0), stores the vtable `010bbb50`, constructs the `NiPointer` arrays at +0x8c (2,
/// destructor `00b79930`), +0x94 (7), +0xb0 (3), +0xbc (7) and +0xd8 (3; destructor
/// `00b66f20`), clears the words +0xe4 and +0xe8, sets +0x68 to 2, assigns the declarations to
/// the `NiPointer`s at +0xe4 and +0xe8, then the object +0xe8 holds to the `NiPointer` at
/// +0x28, and clears the word +0x60. Answers `this`.
// Translation incomplete: 00bd12c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the base and the arrays) is not translated.
pub fn fn_00bd12c0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let base = this.addr();
    e.call(
        SHADER_BASE_CONSTRUCT_00B79B00,
        &args![base, first, second, 0u32, 0u32],
    );
    e.mem.set_u32(base, VTABLE_010BBB50);
    construct_pointer_array(e, base + 0x8c, 2, DESTRUCT_00B79930);
    construct_pointer_array(e, base + 0x94, 7, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xb0, 3, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xbc, 7, DESTRUCT_00B66F20);
    construct_pointer_array(e, base + 0xd8, 3, DESTRUCT_00B66F20);
    e.mem.set_u32(base + 0xe4, 0);
    e.mem.set_u32(base + 0xe8, 0);
    e.mem.set_u32(base + 0x68, 2);
    ni_pointer_assign(e, base + 0xe4, first.addr());
    ni_pointer_assign(e, base + 0xe8, second.addr());
    let kept = e.mem.u32(base + 0xe8);
    ni_pointer_assign(e, base + 0x28, kept);
    e.mem.set_u32(base + 0x60, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_d390, fn_00bcd390(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_f7f0, fn_00bcf7f0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bd_12c0, fn_00bd12c0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bd_14b0, fn_00bd14b0() -> Ptr),
        entry!(0x00bc_fb00, fn_00bcfb00() -> Ptr),
        entry!(0x00bc_d680, fn_00bcd680() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    fn entries(declaration: u32, list: &[[u32; 5]]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|entry| {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                (declaration_slot(0x8c), words)
            })
            .collect()
    }

    fn two_declarations(creator: u32, construct: u32, size: u32) {
        let mut e = rig(construct, &[]);
        let (shader, log) = run(&mut e, creator);
        let (a, b) = (DECLARATION_0, DECLARATION_1);
        assert_eq!(shader, SHADER);
        let mut want = vec![
            (DECLARATION_CREATE, vec![RENDERER, 6, 1]),
            (DECLARATION_CREATE, vec![RENDERER, 8, 1]),
        ];
        want.extend(entries(a, &SIX_ENTRIES));
        want.push((declaration_slot(0xb4), vec![a]));
        want.extend(entries(b, &EIGHT_ENTRIES));
        want.push((declaration_slot(0xb4), vec![b]));
        want.push((NI_ALLOCATE, vec![size]));
        want.push((construct, vec![SHADER, a, b]));
        for slot in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, a]));
        assert_eq!(log, want);
    }

    #[test]
    fn fn_00bd14b0_makes_two_declarations_then_the_shader() {
        two_declarations(0x00bd_14b0, SHADER_CONSTRUCT_00BD12C0, 0xec);
    }

    #[test]
    fn fn_00bcfb00_makes_two_declarations_then_the_shader() {
        two_declarations(0x00bc_fb00, SHADER_CONSTRUCT_00BCF7F0, 0x128);
    }

    #[test]
    fn fn_00bcd680_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BCD390, &[]);
        let (shader, log) = run(&mut e, 0x00bc_d680);
        let d = DECLARATION_0;
        assert_eq!(shader, SHADER);
        let mut want = vec![(DECLARATION_CREATE, vec![RENDERER, 6, 1])];
        want.extend(entries(d, &SIX_ENTRIES));
        want.push((declaration_slot(0xb4), vec![d]));
        want.push((NI_ALLOCATE, vec![0x160]));
        want.push((SHADER_CONSTRUCT_00BCD390, vec![SHADER, d]));
        for slot in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, d]));
        assert_eq!(log, want);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn engine() -> (Engine, u32) {
        let mut e = rig();
        e.register(SHADER_BASE_CONSTRUCT_00B79B00, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        for offset in [0x28u32, 0x6c, 0x70, 0xe4, 0xe8] {
            e.mem.set_u32(this + offset, 0);
        }
        (e, this)
    }

    #[test]
    fn fn_00bcd390_builds_the_shader() {
        let (mut e, this) = engine();
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_d390, &args![this, declaration]).u32(), this);
        assert_eq!(
            calls(&e, SHADER_BASE_CONSTRUCT_00B79B00),
            vec![vec![this, declaration, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010BB7A8);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 0x14, DESTRUCT_00B66F20),
                pointer_array(this + 0xdc, 0x21, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 3);
        assert_eq!(e.mem.u8(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x6c), declaration);
        assert_eq!(e.mem.u32(declaration + 4), 1);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }

    #[test]
    fn fn_00bcf7f0_builds_the_shader() {
        let (mut e, this) = engine();
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_f7f0, &args![this, first, second]).u32(), this);
        assert_eq!(
            calls(&e, SHADER_BASE_CONSTRUCT_00B79B00),
            vec![vec![this, first, second, 0, 0]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_010BB980);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 0x1a, DESTRUCT_00B66F20),
                pointer_array(this + 0xf4, 0xd, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 4);
        assert_eq!(e.mem.u32(this + 0x6c), first);
        assert_eq!(e.mem.u32(this + 0x70), second);
        assert_eq!(e.mem.u32(second + 4), 1);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }

    #[test]
    fn fn_00bd12c0_builds_the_shader_and_shares_the_second_declaration() {
        let (mut e, this) = engine();
        let old = object(&mut e, 1);
        e.mem.set_u32(this + 0x28, old);
        let first = object(&mut e, 0);
        let second = object(&mut e, 0);
        assert_eq!(e.call(0x00bd_12c0, &args![this, first, second]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BBB50);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![
                pointer_array(this + 0x8c, 2, DESTRUCT_00B79930),
                pointer_array(this + 0x94, 7, DESTRUCT_00B66F20),
                pointer_array(this + 0xb0, 3, DESTRUCT_00B66F20),
                pointer_array(this + 0xbc, 7, DESTRUCT_00B66F20),
                pointer_array(this + 0xd8, 3, DESTRUCT_00B66F20),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x68), 2);
        assert_eq!(e.mem.u32(this + 0xe4), first);
        assert_eq!(e.mem.u32(this + 0xe8), second);
        assert_eq!(e.mem.u32(this + 0x28), second);
        assert_eq!(e.mem.u32(second + 4), 2);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![old]]);
        assert_eq!(e.mem.u32(this + 0x60), 0);
    }
}
