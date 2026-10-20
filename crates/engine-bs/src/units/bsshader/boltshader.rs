//! `BSShader/boltshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The shader's constructor `00bc0c90` (engine map unit `boltshader.cpp`, unnamed,
/// `thiscall`, `RET 4`: the declaration).
const SHADER_CONSTRUCT_00BC0C90: u32 = 0x00bc_0c90;

// Translated from 00bc10e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the creator `GetShader` (`00b55560`)
/// calls for shader 0x19. No arguments. Makes a declaration
/// (`NiXenonShaderDeclaration::Create` with the renderer `011f9508`, 5 entries, 1 stream),
/// sets its entries through its vtable slot +0x8c (five words each: (0, 0, 0, 2, 0), (1, 3,
/// 3, 2, 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0)), calls slot +0x94 with (0, 4, 6, 0x80000000,
/// 0, 0xa, 1, 0) and slot +0xb4. Then allocates 0x194 bytes and runs the constructor
/// `00bc0c90` with the declaration (a null allocation is used as it is), calls the shader's
/// slots +0x4c, +0x13c, +0x140, +0x11c, then +0xc0 with the declaration, and answers the
/// shader.
// Translation incomplete: 00bc10e0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00bc10e0(e: &mut Engine) -> Ptr {
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
        &args![0u32, 4u32, 6u32, 0x8000_0000u32, 0u32, 0xau32, 1u32, 0u32],
    );
    e.vcall(declaration, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x194u32]).u32();
    let shader = if block != 0 {
        e.call(SHADER_CONSTRUCT_00BC0C90, &args![block, declaration])
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

/// The bolt shader's vtable (`010ba3b0`; no RTTI locator before it).
const BOLT_VTABLE_010BA3B0: u32 = 0x010b_a3b0;
/// The four words copied twice (`011a9be0`), the shared segment count (`01201f50`, set on
/// first use from the shader model setting `011f91c0`), the `float` 2^32 added to a negative
/// `FILD` (`01099378`) and the `double` the count is scaled by (`010290b0`).
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const SEGMENT_COUNT_01201F50: u32 = 0x0120_1f50;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const QUARTER_010290B0: u32 = 0x0102_90b0;
/// CRT `ceil` (`00ec9e10`, the `double` on the stack, the result in `ST0`).
const CEIL: u32 = 0x00ec_9e10;

// Translated from 00bc0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `boltshader.cpp`): the bolt shader's constructor (the creator
/// `00bc0fb0` runs it), `thiscall` with one stack word (the declaration). Runs
/// `BSShader::BSShader` (`00be2200`), stores the vtable `010ba3b0`, 0.0 at +0x118 to +0x124 and
/// +0x16c to +0x188, clears +0x168, +0x18c, +0x190, sets +0x68 to 0xf, the byte +0x24 to 1 and
/// +0x6c to 0x242; clears the pass pointer at +0x168, assigns the declaration to the `NiPointer`
/// at +0x28, clears the `NiPointer`s at +0x18c and +0x190, stores 0.0 at +0x74 and +0x118 to
/// +0x124 and copies the four words at `011a9be0` to +0x16c and +0x17c. The segment count at
/// `01201f50` becomes 0x4b (shader model below 2, signed) or 0xeb when it is 0; `ceil` of the
/// count (as unsigned, through the FPU) times the `double` at `010290b0`, rounded to a `float`
/// before and after, truncated (`FISTP qword` with truncation, low word) gives n: 64 * n bytes
/// (the `operator new[]` size check on 4 * n elements of 16) from the memory manager go to +0x70
/// and are cleared, as are 0x40 bytes at +0x128 and 0xa0 bytes at +0x78 (`memset`). Answers
/// `this`.
// Translation incomplete: 00bc0c90 - the C++ exception frame (FS:[0] chain, unwind state 3 for the base and the pointers) is not translated.
pub fn fn_00bc0c90(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, BOLT_VTABLE_010BA3B0);
    for offset in [0x118u32, 0x11c, 0x120, 0x124] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x168, 0);
    for offset in (0x16cu32..=0x188).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x18c, 0);
    e.mem.set_u32(base + 0x190, 0);
    e.mem.set_u32(base + 0x68, 0xf);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x242);
    pass_pointer_clear(e, base + 0x168);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0x18c);
    ni_pointer_clear(e, base + 0x190);
    e.mem.set_f32(base + 0x74, 0.0);
    for offset in [0x118u32, 0x11c, 0x120, 0x124] {
        e.mem.set_f32(base + offset, 0.0);
    }
    for copy in [0x16cu32, 0x17c] {
        for index in 0..4u32 {
            let word = e.global::<u32>(COLOR_011A9BE0 + 4 * index);
            e.mem.set_u32(base + copy + 4 * index, word);
        }
    }
    if e.global::<u32>(SEGMENT_COUNT_01201F50) == 0 {
        let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x4b
        } else {
            0xeb
        };
        e.set_global::<u32>(SEGMENT_COUNT_01201F50, count);
    }
    let count = e.global::<u32>(SEGMENT_COUNT_01201F50);
    let mut loaded = f64::from(count as i32);
    if (count as i32) < 0 {
        loaded = x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    let quarter = f64::from_bits(e.global::<u64>(QUARTER_010290B0));
    let scaled = x87_load32(x87_store32(x87_mul(loaded, quarter)));
    let rounded = e.call(CEIL, &args![scaled]).f64();
    let rounded = x87_load32(x87_store32(rounded));
    // FISTP qword with the rounding control at truncate: the integer indefinite (low word 0)
    // for NaN and values out of range.
    let truncated = rounded.trunc();
    let segments = if truncated.is_nan()
        || truncated < -9_223_372_036_854_775_808.0
        || truncated >= 9_223_372_036_854_775_808.0
    {
        0
    } else {
        truncated as i64 as u32
    };
    let elements = segments.wrapping_mul(4);
    let buffer = allocate_array(e, elements, 0x10);
    e.mem.set_u32(base + 0x70, buffer);
    e.call(MEMSET, &args![buffer, 0u32, elements.wrapping_shl(4)]);
    e.call(MEMSET, &args![base + 0x128, 0u32, 0x40u32]);
    e.call(MEMSET, &args![base + 0x78, 0u32, 0xa0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_0c90, fn_00bc0c90(Ptr, Ptr) -> Ptr),
        entry!(0x00bc_10e0, fn_00bc10e0() -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::bsshadermanager::creator_rig::*;

    #[test]
    fn fn_00bc10e0_makes_the_declaration_then_the_shader() {
        let mut e = rig(SHADER_CONSTRUCT_00BC0C90, &[]);
        let (shader, log) = run(&mut e, 0x00bc_10e0);
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
                    vec![d, 0, 4, 6, 0x8000_0000, 0, 0xa, 1, 0]
                ),
                (declaration_slot(0xb4), vec![d]),
                (NI_ALLOCATE, vec![0x194]),
                (SHADER_CONSTRUCT_00BC0C90, vec![SHADER, d]),
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

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].min(0x1000)).into_ret()
        });
        e.register(CEIL, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            x.ceil().into_ret()
        });
        for page in [
            0x011a_9000u32,
            0x0120_1000,
            0x011f_9000,
            0x0109_9000,
            0x0102_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f32>(TWO_TO_32_01099378, 4_294_967_296.0);
        e.set_global::<f64>(QUARTER_010290B0, 0.25);
        for index in 0..4u32 {
            e.set_global::<u32>(COLOR_011A9BE0 + 4 * index, 0x50 + index);
        }
        e.set_global::<u32>(SEGMENT_COUNT_01201F50, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x194);
        e.mem.write(this, &[0xcc; 0x194]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00bc_0c90, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00bc0c90_builds_the_bolt_shader() {
        let (e, this, declaration) = construct(0, 3);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), BOLT_VTABLE_010BA3B0);
        assert_eq!(e.mem.u32(this + 0x68), 0xf);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x242);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        for offset in [0x74u32, 0x118, 0x11c, 0x120, 0x124, 0x168, 0x18c, 0x190] {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        for index in 0..4u32 {
            assert_eq!(e.mem.u32(this + 0x16c + 4 * index), 0x50 + index);
            assert_eq!(e.mem.u32(this + 0x17c + 4 * index), 0x50 + index);
        }
        // 0xeb segments: ceil(235 * 0.25) = 59, 4 * 59 elements of 16 bytes.
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0xeb);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xec0]]
        );
        let buffer = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, MEMSET),
            vec![
                vec![buffer, 0, 0xec0],
                vec![this + 0x128, 0, 0x40],
                vec![this + 0x78, 0, 0xa0]
            ]
        );
    }

    #[test]
    fn fn_00bc0c90_segment_count_follows_the_model_and_an_unsigned_count() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0x4b);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x4c0]]
        );
        // A count with the sign bit set is loaded as unsigned: 2^31 / 4 segments, whose
        // byte count overflows the size check.
        let (e, _, _) = construct(0x8000_0000, 3);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, u32::MAX]]
        );
    }
}
