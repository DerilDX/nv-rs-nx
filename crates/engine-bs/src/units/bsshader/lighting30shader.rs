//! `BSShader/lighting30shader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The shader's constructor `00bbc640` (engine map unit `lighting30shader.cpp`, unnamed,
/// `thiscall`, `RET 0x10`: four declarations).
const SHADER_CONSTRUCT_00BBC640: u32 = 0x00bb_c640;

/// The entries (vtable slot +0x8c, five words each) of the first two declarations (5
/// entries made), the next two (7 of 8), the two after (6 of 6) and the last two (8 of 8).
const FIVE_ENTRIES: [[u32; 5]; 5] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 7, 5, 1, 0],
    [3, 0xe, 0xd, 2, 0],
    [4, 0xf, 0xe, 2, 0],
];
const SEVEN_ENTRIES: [[u32; 5]; 7] = [
    [0, 0, 0, 2, 0],
    [1, 1, 1, 3, 0],
    [2, 2, 2, 4, 0],
    [3, 3, 3, 2, 0],
    [4, 7, 5, 1, 0],
    [5, 0xe, 0xd, 2, 0],
    [6, 0xf, 0xe, 2, 0],
];
const SIX_ENTRIES: [[u32; 5]; 6] = [
    [0, 0, 0, 2, 0],
    [1, 3, 3, 2, 0],
    [2, 5, 4, 3, 0],
    [3, 7, 5, 1, 0],
    [4, 0xe, 0xd, 2, 0],
    [5, 0xf, 0xe, 2, 0],
];
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

/// The shader's `NiPointer` members the code assigns after the constructor (+0x80, +0x88,
/// +0x90, +0x98), inlined: when the member does not hold `object` already, the old one is
/// released (count at +4 through `InterlockedDecrement`, deleted through vtable slot +4 at
/// 0) when not null, the member set and `object` held (`InterlockedIncrement` on its +4; the
/// code makes no null test on this side).
fn assign_member(e: &mut Engine, member: u32, object: u32) {
    let old = e.mem.u32(member);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(member, object);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

// Translated from 00bbd0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the creator `GetShader`
/// (`00b55560`) calls for shader 0x1d when the byte at `011f94a6` is set. No arguments.
///
/// Makes eight declarations (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`
/// read each time, 1 stream each): two of 6 entries (`a1`, `a2`), two of 8 (`b1`, `b2`), two
/// of 6 (`c1`, `c2`), two of 8 (`d1`, `d2`), in that order. Fills them through vtable slot
/// +0x8c and ends each pair (slot +0xb4, first then second) in the order a, c, b, d: the a
/// pair gets 5 entries ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 7, 5, 1, 0), (3, 0xe, 0xd, 2,
/// 0), (4, 0xf, 0xe, 2, 0)), the c pair 6 ((0, 0, 0, 2, 0), (1, 3, 3, 2, 0), (2, 5, 4, 3, 0),
/// (3, 7, 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0)), the b pair 7 ((0, 0, 0, 2, 0),
/// (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2, 0), (4, 7, 5, 1, 0), (5, 0xe, 0xd, 2, 0),
/// (6, 0xf, 0xe, 2, 0)), the d pair 8 (the b list with (4, 5, 4, 3, 0) and (5, 7, 5, 1, 0) in
/// place of its fifth entry, then the last two). Allocates 0xa4 bytes and runs the
/// constructor `00bbc640` with (a1, b1, c1, d1) (a null allocation is used as it is), calls
/// the shader's slot +0x4c, assigns a2, c2, b2, d2 to its `NiPointer`s at +0x80, +0x88,
/// +0x90, +0x98, calls slots +0x148, +0x11c, +0x154, then +0xc0 with a1, and answers the
/// shader.
// Translation incomplete: 00bbd0d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bbd0d0(e: &mut Engine) -> Ptr {
    let make = |e: &mut Engine, entries: u32| {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call(DECLARATION_CREATE, &args![renderer, entries, 1u32])
            .u32()
    };
    let a1 = make(e, 6);
    let a2 = make(e, 6);
    let b1 = make(e, 8);
    let b2 = make(e, 8);
    let c1 = make(e, 6);
    let c2 = make(e, 6);
    let d1 = make(e, 8);
    let d2 = make(e, 8);
    let fill = |e: &mut Engine, first: u32, second: u32, entries: &[[u32; 5]]| {
        for declaration in [first, second] {
            for entry in entries {
                e.vcall(declaration, 0x8c, entry);
            }
        }
        e.vcall(first, 0xb4, &args![]);
        e.vcall(second, 0xb4, &args![]);
    };
    fill(e, a1, a2, &FIVE_ENTRIES);
    fill(e, c1, c2, &SIX_ENTRIES);
    fill(e, b1, b2, &SEVEN_ENTRIES);
    fill(e, d1, d2, &EIGHT_ENTRIES);
    let block = e.call(NI_ALLOCATE, &args![0xa4u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BBC640, &args![block, a1, b1, c1, d1])
            .u32()
    } else {
        0
    };
    e.vcall(shader, 0x4c, &args![]);
    assign_member(e, shader.wrapping_add(0x80), a2);
    assign_member(e, shader.wrapping_add(0x88), c2);
    assign_member(e, shader.wrapping_add(0x90), b2);
    assign_member(e, shader.wrapping_add(0x98), d2);
    for slot in [0x148u32, 0x11c, 0x154] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![a1]);
    Ptr::new(shader)
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The lighting 3.0 shader's vtable (`010ba0f8`; no RTTI locator before it).
const LIGHTING30_VTABLE_010BA0F8: u32 = 0x010b_a0f8;
/// The shader model setting (`011f91c0`) and the word set to 5 below model 7 (`011ae60c`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const WORD_SETTING_011AE60C: u32 = 0x011a_e60c;

// Translated from 00bbc640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lighting30shader.cpp`): the lighting 3.0 shader's
/// constructor (the creator `00bbd0d0` runs it), `thiscall` with four stack words (four
/// declarations). Runs `BSShader::BSShader` (`00be2200`), stores the vtable `010ba0f8`, clears
/// +0x6c to +0x98, sets +0x68 to 0x11; assigns the first declaration to the `NiPointer`s at
/// +0x28, +0x7c and +0x80, the second to +0x8c and +0x90, the third to +0x84 and +0x88 and the
/// fourth to +0x94 and +0x98; clears the `NiPointer`s at +0x6c, +0x70, +0x74 and +0x78; clears
/// the words `01200854` to `01200860`, the byte `01200864`, the words `01200848` and
/// `0120084c` and the 16-bit `01200850`; sets `011ae60c` to 5 when the shader model setting
/// (`011f91c0`) is below 7 (signed); clears +0x9c and sets +0xa0 to 1. Answers `this`.
// Translation incomplete: 00bbc640 - the C++ exception frame (FS:[0] chain, unwind state 0xc for the base and the pointers) is not translated.
pub fn fn_00bbc640(
    e: &mut Engine,
    this: Ptr,
    first: Ptr,
    second: Ptr,
    third: Ptr,
    fourth: Ptr,
) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, LIGHTING30_VTABLE_010BA0F8);
    for offset in (0x6cu32..=0x98).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x68, 0x11);
    for (declaration, slots) in [
        (first, &[0x28u32, 0x7c, 0x80][..]),
        (second, &[0x8c, 0x90][..]),
        (third, &[0x84, 0x88][..]),
        (fourth, &[0x94, 0x98][..]),
    ] {
        for slot in slots {
            ni_pointer_assign(e, base + slot, declaration.addr());
        }
    }
    for offset in [0x6cu32, 0x70, 0x74, 0x78] {
        ni_pointer_clear(e, base + offset);
    }
    for word in [0x0120_0854u32, 0x0120_0858, 0x0120_085c, 0x0120_0860] {
        e.set_global::<u32>(word, 0);
    }
    e.set_global::<u8>(0x0120_0864, 0);
    e.set_global::<u32>(0x0120_0848, 0);
    e.set_global::<u32>(0x0120_084c, 0);
    e.set_global::<u16>(0x0120_0850, 0);
    if e.global::<i32>(SHADER_MODEL_011F91C0) < 7 {
        e.set_global::<u32>(WORD_SETTING_011AE60C, 5);
    }
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_c640, fn_00bbc640(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00bb_d0d0, fn_00bbd0d0() -> Ptr),
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

    fn counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn fn_00bbd0d0_makes_eight_declarations_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BBC640, &[]);
        counting(&mut e);
        let (shader, log) = run(&mut e, 0x00bb_d0d0);
        assert_eq!(shader, SHADER);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let mut want: Vec<(u32, Vec<u32>)> = [6, 6, 8, 8, 6, 6, 8, 8]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        for (first, second, list) in [
            (0u32, 1u32, &FIVE_ENTRIES[..]),
            (4, 5, &SIX_ENTRIES[..]),
            (2, 3, &SEVEN_ENTRIES[..]),
            (6, 7, &EIGHT_ENTRIES[..]),
        ] {
            want.extend(entries(d(first), list));
            want.extend(entries(d(second), list));
            want.push((declaration_slot(0xb4), vec![d(first)]));
            want.push((declaration_slot(0xb4), vec![d(second)]));
        }
        want.push((NI_ALLOCATE, vec![0xa4]));
        want.push((
            SHADER_CONSTRUCT_00BBC640,
            vec![SHADER, d(0), d(2), d(4), d(6)],
        ));
        want.push((shader_slot(0x4c), vec![SHADER]));
        for n in [1, 5, 3, 7] {
            want.push((IMPORT_INTERLOCKED_INCREMENT, vec![d(n) + 4]));
        }
        for slot in [0x148, 0x11c, 0x154] {
            want.push((shader_slot(slot), vec![SHADER]));
        }
        want.push((shader_slot(0xc0), vec![SHADER, d(0)]));
        assert_eq!(log, want);
        assert_eq!(e.mem.u32(SHADER + 0x80), d(1));
        assert_eq!(e.mem.u32(SHADER + 0x88), d(5));
        assert_eq!(e.mem.u32(SHADER + 0x90), d(3));
        assert_eq!(e.mem.u32(SHADER + 0x98), d(7));
    }

    #[test]
    fn assign_member_releases_the_old_object_and_keeps_the_same_one() {
        let mut e = Engine::new();
        counting(&mut e);
        e.put_vtable(0x0f90_0000, &[0, 0x0f90_1004]);
        e.register(0x0f90_1004, |_, _| Ret::default());
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, 0x0f90_0000);
        e.mem.set_u32(old + 4, 1);
        let new = e.mem.alloc(8);
        let member = e.mem.alloc(4);
        e.mem.set_u32(member, old);
        e.call_log = Some(vec![]);
        assign_member(&mut e, member, new);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (IMPORT_INTERLOCKED_DECREMENT, vec![old + 4]),
                (0x0f90_1004, vec![old]),
                (IMPORT_INTERLOCKED_INCREMENT, vec![new + 4]),
            ]
        );
        assert_eq!(e.mem.u32(member), new);
        e.call_log = Some(vec![]);
        assign_member(&mut e, member, new);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.mem.u32(new + 4), 1);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(model: i32) -> (Engine, u32, [u32; 4]) {
        let mut e = rig();
        for page in [0x0120_0000u32, 0x011f_9000, 0x011a_e000] {
            e.map(page, 0x1000);
        }
        e.mem.write(0x0120_0848, &[0xee; 0x20]);
        e.set_global::<u32>(WORD_SETTING_011AE60C, 9);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0xa4);
        e.mem.write(this, &[0xcc; 0xa4]);
        e.mem.set_u32(this + 0x28, 0);
        let declarations = [
            object(&mut e, 0),
            object(&mut e, 0),
            object(&mut e, 0),
            object(&mut e, 0),
        ];
        let words = [
            this,
            declarations[0],
            declarations[1],
            declarations[2],
            declarations[3],
        ];
        assert_eq!(e.call(0x00bb_c640, &words).u32(), this);
        (e, this, declarations)
    }

    #[test]
    fn fn_00bbc640_builds_the_shader() {
        let (e, this, d) = construct(3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), LIGHTING30_VTABLE_010BA0F8);
        assert_eq!(e.mem.u32(this + 0x68), 0x11);
        for (offset, declaration) in [
            (0x28u32, d[0]),
            (0x7c, d[0]),
            (0x80, d[0]),
            (0x8c, d[1]),
            (0x90, d[1]),
            (0x84, d[2]),
            (0x88, d[2]),
            (0x94, d[3]),
            (0x98, d[3]),
        ] {
            assert_eq!(e.mem.u32(this + offset), declaration, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(d[0] + 4), 3);
        assert_eq!(e.mem.u32(d[1] + 4), 2);
        for offset in [0x6cu32, 0x70, 0x74, 0x78, 0x9c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0xa0), 1);
        assert_eq!(e.mem.bytes(0x0120_0848, 0xa), vec![0; 0xa]);
        assert_eq!(e.mem.bytes(0x0120_0854, 0x11), vec![0; 0x11]);
        assert_eq!(e.global::<u16>(0x0120_0852), 0xeeee);
        assert_eq!(e.global::<u32>(WORD_SETTING_011AE60C), 5);
    }

    #[test]
    fn fn_00bbc640_keeps_the_word_from_model_7() {
        let (e, _, _) = construct(7);
        assert_eq!(e.global::<u32>(WORD_SETTING_011AE60C), 9);
    }
}
