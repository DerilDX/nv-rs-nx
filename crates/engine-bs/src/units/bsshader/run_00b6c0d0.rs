//! `BSShader/run_00b6c0d0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00b6c200` (unnamed, `cdecl`, one word), asked with the word at `011f9490` for the last
/// out word's first value and by kinds 4 and 0x1f.
const QUERY_00B6C200: u32 = 0x00b6_c200;
const QUERY_ARGUMENT_011F9490: u32 = 0x011f_9490;
/// The renderer's width and height helpers `00b6c1a0` and `00b6c1d0` (unnamed, `thiscall` on
/// the renderer, no stack words).
const RENDERER_WIDTH_00B6C1A0: u32 = 0x00b6_c1a0;
const RENDERER_HEIGHT_00B6C1D0: u32 = 0x00b6_c1d0;
/// The byte that makes the code take the two sizes at `011f9430`/`011f9434` in place of the
/// renderer's.
const SIZE_OVERRIDE_011F9426: u32 = 0x011f_9426;
const WIDTH_011F9430: u32 = 0x011f_9430;
const HEIGHT_011F9434: u32 = 0x011f_9434;
/// The other words and bytes the arms read.
const SIZE_01200054: u32 = 0x0120_0054;
const WIDTH_011AD814: u32 = 0x011a_d814;
const HEIGHT_011AD818: u32 = 0x011a_d818;
const EXTRA_011AD810: u32 = 0x011a_d810;
const WIDTH_011AD81C: u32 = 0x011a_d81c;
const HEIGHT_011AD820: u32 = 0x011a_d820;
const SIZE_011ADBC8: u32 = 0x011a_dbc8;
const SIZE_011AD830: u32 = 0x011a_d830;
const FORMAT_011AD870: u32 = 0x011a_d870;
const BYTE_SETTING_011F941E: u32 = 0x011f_941e;
const BYTE_SETTING_011AD80E: u32 = 0x011a_d80e;
const BYTE_SETTING_012004FC: u32 = 0x0120_04fc;
/// The `float` 2^32 the code adds to a negative `FILD` (`01099378`) and the `double` 0.5 it
/// multiplies by (`01011588`).
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const HALF_01011588: u32 = 0x0101_1588;

/// The renderer's slot +0xc8, then that object's slot `slot` (+0x8c width, +0x90 height)
/// with 0.
fn target_size(e: &mut Engine, renderer: Ptr, slot: u32) -> u32 {
    let target = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    e.vcall(target, slot, &args![0u32]).u32()
}

/// `CDQ; AND EDX,3; ADD EAX,EDX; SAR EAX,2`: the signed word divided by 4, toward zero.
fn signed_quarter(value: u32) -> u32 {
    let value = value as i32;
    (value.wrapping_add((value >> 31) & 3) >> 2) as u32
}

/// `CDQ; SUB EAX,EDX; SAR EAX,1`: the signed word divided by 2, toward zero.
fn signed_half(value: u32) -> u32 {
    let value = value as i32;
    (value.wrapping_sub(value >> 31) >> 1) as u32
}

/// `FILD dword` of an unsigned word: loaded signed, then 2^32 added (the `float` at
/// `01099378`, through the FPU) when the sign bit is set.
fn load_unsigned(e: &Engine, value: u32) -> f64 {
    let loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)))
    } else {
        loaded
    }
}

/// `FISTP qword` with the rounding control set to truncate (the code ORs 0xc00 into the
/// control word around it): toward zero, the integer indefinite 0x8000000000000000 for NaN
/// and values out of range. The code keeps the low word.
fn fistp_qword_truncate(value: f64) -> u64 {
    let truncated = value.trunc();
    if truncated.is_nan()
        || truncated < -9_223_372_036_854_775_808.0
        || truncated >= 9_223_372_036_854_775_808.0
    {
        0x8000_0000_0000_0000
    } else {
        truncated as i64 as u64
    }
}

/// The width and height from the renderer's helpers (`00b6c1a0`, `00b6c1d0`) or, with the
/// byte at `011f9426` set, the words at `011f9430`/`011f9434`; the width stored before the
/// height is asked.
fn helper_or_override_size(e: &mut Engine, renderer: Ptr, width: Ptr, height: Ptr) {
    if e.global::<u8>(SIZE_OVERRIDE_011F9426) == 0 {
        let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
        e.mem.set_u32(width.addr(), w);
        let h = e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32();
        e.mem.set_u32(height.addr(), h);
    } else {
        e.mem.set_u32(width.addr(), e.global::<u32>(WIDTH_011F9430));
        e.mem
            .set_u32(height.addr(), e.global::<u32>(HEIGHT_011F9434));
    }
}

/// The same with the renderer's slot +0xc8 target sizes (+0x8c, +0x90) in place of the
/// helpers.
fn target_or_override_size(e: &mut Engine, renderer: Ptr, width: Ptr, height: Ptr) {
    if e.global::<u8>(SIZE_OVERRIDE_011F9426) == 0 {
        let w = target_size(e, renderer, 0x8c);
        e.mem.set_u32(width.addr(), w);
        let h = target_size(e, renderer, 0x90);
        e.mem.set_u32(height.addr(), h);
    } else {
        e.mem.set_u32(width.addr(), e.global::<u32>(WIDTH_011F9430));
        e.mem
            .set_u32(height.addr(), e.global::<u32>(HEIGHT_011F9434));
    }
}

/// The arms of kinds 0x2d, 0x31 (`flag_word` 4) and 0x30 (0x40): the width set to 0x500,
/// the renderer's slot +0xc8 asked twice; the first object's height (+0x90) and the second's
/// width (+0x8c), each loaded as unsigned, give `height / width` (`FDIVR`), times the width
/// word read back as unsigned (`FMULP`), truncated into the height (low word of a `FISTP
/// qword`); then the flags and a format of 0.
fn aspect_size(
    e: &mut Engine,
    renderer: Ptr,
    width: Ptr,
    height: Ptr,
    format: Ptr,
    flags: Ptr,
    flag_word: u32,
) {
    e.mem.set_u32(width.addr(), 0x500);
    let first = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let second = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    let target_height = e.vcall(first, 0x90, &args![0u32]).u32();
    let target_height = load_unsigned(e, target_height);
    let target_width = e.vcall(second, 0x8c, &args![0u32]).u32();
    let target_width = load_unsigned(e, target_width);
    let ratio = x87_div(target_height, target_width);
    let width_back = e.mem.u32(width.addr());
    let product = x87_mul(ratio, load_unsigned(e, width_back));
    e.mem
        .set_u32(height.addr(), fistp_qword_truncate(product) as u32);
    e.mem.set_u32(flags.addr(), flag_word);
    e.mem.set_u32(format.addr(), 0);
}

// Translated from 00b6c2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`): the description of a rendered-texture kind.
/// `stdcall`, `RET 0x20`, eight words: the renderer, the kind, and the addresses of the
/// width, height, format, a word it clears, the flags and a last word. Clears the fifth word
/// and sets the last to `00b6c200(word at 011f9490)`; for kinds 0 to 0x33 (the jump table at
/// `00b6d048`, read in Ghidra) stores the kind's width, height, flags and format (each arm in
/// the exe's order; see the arms below); kind 0xb and kinds above 0x33 store nothing. At the
/// end the last word is cleared unless the flags (as they then are) have bit 0x40.
///
/// Sizes come from constants, from words of the exe's data, from the renderer (its slot
/// +0xc8 object's +0x8c/+0x90 with 0, or its helpers `00b6c1a0`/`00b6c1d0`) or, with the
/// byte at `011f9426` set, from the words at `011f9430`/`011f9434`; some arms take a
/// quarter or a half (logical shifts of the renderer's sizes, signed divisions toward zero
/// of the words). Kind 1 takes half the helper width through the FPU (unsigned load,
/// `FMUL` by 0.5, truncation), at least 0x100.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b6c2c0(
    e: &mut Engine,
    renderer: Ptr,
    kind: u32,
    width: Ptr,
    height: Ptr,
    format: Ptr,
    cleared: Ptr,
    flags: Ptr,
    last: Ptr,
) {
    let put = |e: &mut Engine, at: Ptr, value: u32| e.mem.set_u32(at.addr(), value);
    put(e, cleared, 0);
    let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
    let first = e.call(QUERY_00B6C200, &args![word]).u32();
    put(e, last, first);
    let overridden = |e: &Engine| e.global::<u8>(SIZE_OVERRIDE_011F9426) != 0;
    match kind {
        0 | 2 => {
            put(e, flags, 0x26);
            if target_size(e, renderer, 0x8c) <= 0x500 {
                put(e, height, 0x100);
                put(e, width, 0x100);
                put(e, format, 0x71);
            } else {
                let w = if overridden(e) {
                    signed_quarter(e.global::<u32>(WIDTH_011F9430))
                } else {
                    target_size(e, renderer, 0x8c) >> 2
                };
                put(e, width, w);
                let h = if overridden(e) {
                    signed_quarter(e.global::<u32>(HEIGHT_011F9434))
                } else {
                    target_size(e, renderer, 0x90) >> 2
                };
                put(e, height, h);
                put(e, format, 0x71);
            }
        }
        1 => {
            put(e, flags, 0x26);
            let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
            let half = f64::from_bits(e.global::<u64>(HALF_01011588));
            let product = x87_mul(load_unsigned(e, w), half);
            let mut value = fistp_qword_truncate(product) as u32;
            if value < 0x100 {
                value = 0x100;
            }
            put(e, width, value);
            put(e, height, 0x100);
            put(e, format, 0x71);
        }
        3 => {
            if overridden(e) {
                put(e, width, e.global::<u32>(WIDTH_011F9430));
                put(e, height, e.global::<u32>(HEIGHT_011F9434));
            } else {
                let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32();
                put(e, width, w);
                let h = e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32();
                put(e, height, h);
            }
            put(e, flags, 0x26);
            put(e, format, 0);
        }
        4 => {
            helper_or_override_size(e, renderer, width, height);
            let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
            let answer = e.call(QUERY_00B6C200, &args![word]).i32();
            put(e, flags, if answer <= 0 { 0x22 } else { 0x62 });
            let byte = e.global::<u8>(BYTE_SETTING_011F941E) != 0;
            put(e, format, if byte { 0x71 } else { 0 });
        }
        5 => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, 0x22);
            put(e, format, 0x71);
        }
        6 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 6);
            put(e, format, 0);
        }
        7 | 0xf | 0x10 | 0x15 => {
            put(e, height, e.global::<u32>(SIZE_01200054));
            put(e, width, e.global::<u32>(SIZE_01200054));
            let (flag_word, format_word) = match kind {
                7 => (6, 0x74),
                0xf => (0xc, 0x72),
                0x10 => (0xc, 0x74),
                _ => (4, 0),
            };
            put(e, flags, flag_word);
            put(e, format, format_word);
        }
        8 => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 2);
            put(e, format, 0x24);
        }
        9 | 0x16 => {
            put(e, width, e.global::<u32>(WIDTH_011AD814));
            put(e, height, e.global::<u32>(HEIGHT_011AD818));
            put(e, flags, 0x40);
            put(e, format, 0);
            put(e, last, e.global::<u32>(EXTRA_011AD810));
            let value = e.mem.u32(flags.addr()) | 0x20;
            put(e, flags, value);
        }
        0xa => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 0x40);
            put(e, format, 0);
        }
        0xb => {}
        0xc => {
            put(e, height, 0x80);
            put(e, width, 0x80);
            put(e, flags, 6);
            put(e, format, 0x24);
        }
        0xd | 0xe => {
            put(e, width, e.global::<u32>(SIZE_01200054));
            put(e, height, if kind == 0xd { 1 } else { 0x10 });
            put(e, flags, 0xc);
            put(e, format, if kind == 0xd { 0x51 } else { 0x74 });
        }
        0x11 => {
            put(e, width, 0x200);
            put(e, height, 0x200);
            put(e, flags, 0x122);
            put(e, format, 0x15);
        }
        0x12 => {
            put(e, width, e.global::<u32>(WIDTH_011AD81C));
            put(e, height, e.global::<u32>(HEIGHT_011AD820));
            put(e, flags, 0xc);
            put(e, format, 0x15);
        }
        0x13 => {
            put(e, width, e.global::<u32>(WIDTH_011AD81C));
            put(e, height, e.global::<u32>(HEIGHT_011AD820));
            put(e, flags, 4);
            put(e, format, 0x15);
            let value = e.mem.u32(flags.addr()) | 0x20;
            put(e, flags, value);
        }
        0x14 => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 6);
            put(e, format, 0x71);
        }
        0x17 | 0x29 | 0x2c | 0x32 => {
            put(e, height, 0x100);
            put(e, width, 0x100);
            let flag_word = match kind {
                0x17 => 6,
                0x29 => 2,
                0x2c => 0x10,
                _ => 4,
            };
            put(e, flags, flag_word);
            put(e, format, 0);
        }
        0x18 => {
            let h = if overridden(e) {
                put(e, width, signed_quarter(e.global::<u32>(WIDTH_011F9430)));
                signed_quarter(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = target_size(e, renderer, 0x8c) >> 2;
                put(e, width, w);
                target_size(e, renderer, 0x90) >> 2
            };
            put(e, height, h);
            put(e, flags, 6);
            put(e, format, 0);
        }
        0x19 | 0x22 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 0x26);
            put(e, format, 0);
        }
        0x1a | 0x1b => {
            put(e, height, e.global::<u32>(SIZE_011ADBC8));
            put(e, width, e.global::<u32>(SIZE_011ADBC8));
            put(e, flags, 6);
            put(e, format, 0);
        }
        0x1c..=0x1e => {
            let size = if e.global::<u8>(BYTE_SETTING_012004FC) != 0 {
                0x200
            } else {
                0x80
            };
            put(e, height, size);
            put(e, width, size);
            put(e, flags, if kind == 0x1c { 4 } else { 0 });
            put(e, format, 0x17);
        }
        0x1f => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, 6);
            let word = e.global::<u32>(QUERY_ARGUMENT_011F9490);
            if e.call(QUERY_00B6C200, &args![word]).i32() > 0 {
                put(e, flags, 0x46);
            }
            put(e, format, 0);
        }
        0x20 | 0x21 => {
            if kind == 0x20 {
                target_or_override_size(e, renderer, width, height);
            } else {
                helper_or_override_size(e, renderer, width, height);
            }
            put(e, flags, if kind == 0x20 { 0x42 } else { 2 });
            let byte = e.global::<u8>(BYTE_SETTING_011AD80E) != 0;
            let format_word = match (kind, byte) {
                (0x20, true) => 0x71,
                (_, true) => 0x72,
                _ => 0x15,
            };
            put(e, format, format_word);
        }
        0x23 | 0x24 => {
            target_or_override_size(e, renderer, width, height);
            put(e, flags, 0x42);
            put(e, format, 0x15);
        }
        0x25 | 0x26 | 0x28 => {
            let h = if overridden(e) {
                put(e, width, signed_quarter(e.global::<u32>(WIDTH_011F9430)));
                signed_quarter(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = e.call(RENDERER_WIDTH_00B6C1A0, &args![renderer]).u32() >> 2;
                put(e, width, w);
                e.call(RENDERER_HEIGHT_00B6C1D0, &args![renderer]).u32() >> 2
            };
            put(e, height, h);
            put(e, flags, 6);
            put(e, format, 0x15);
        }
        0x27 | 0x2f => {
            helper_or_override_size(e, renderer, width, height);
            put(e, flags, if kind == 0x27 { 2 } else { 0x40 });
            put(e, format, if kind == 0x27 { 0x15 } else { 0 });
        }
        0x2a => {
            put(e, height, 0x200);
            put(e, width, 0x200);
            put(e, flags, 6);
            put(e, format, 0x72);
        }
        0x2b => {
            let size = u32::from(e.global::<u16>(SIZE_011AD830));
            put(e, height, size);
            put(e, width, size);
            put(e, flags, 6);
            put(e, format, e.global::<u32>(FORMAT_011AD870));
        }
        0x2d | 0x31 => aspect_size(e, renderer, width, height, format, flags, 4),
        0x30 => aspect_size(e, renderer, width, height, format, flags, 0x40),
        0x2e => {
            let h = if overridden(e) {
                put(e, width, signed_half(e.global::<u32>(WIDTH_011F9430)));
                signed_half(e.global::<u32>(HEIGHT_011F9434))
            } else {
                let w = target_size(e, renderer, 0x8c) >> 1;
                put(e, width, w);
                target_size(e, renderer, 0x90) >> 1
            };
            put(e, height, h);
            put(e, flags, 4);
            put(e, format, 0);
        }
        0x33 => {
            put(e, height, 8);
            put(e, width, 8);
            put(e, flags, 0);
            put(e, format, 0);
        }
        _ => {}
    }
    if e.mem.u32(flags.addr()) & 0x40 == 0 {
        put(e, last, 0);
    }
}

/// The renderer's target through its vtable slot +0xc8, then that target's slot +0xb0 with 0;
/// the word at `offset` of what it answers, 0 when either answers null.
fn renderer_surface_word(e: &mut Engine, renderer: Ptr, offset: u32) -> u32 {
    let target = e.vcall(renderer.addr(), 0xc8, &args![]).u32();
    if target == 0 {
        return 0;
    }
    let surface = e.vcall(target, 0xb0, &args![0u32]).u32();
    if surface == 0 {
        return 0;
    }
    e.mem.u32(surface + offset)
}

// Translated from 00b6c1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`), `thiscall` on the renderer, no stack words: the
/// renderer's slot +0xc8, that object's slot +0xb0 with 0, and the word at +0x8 of the answer
/// (the width its callers use); 0 when either call answers null.
pub fn fn_00b6c1a0(e: &mut Engine, renderer: Ptr) -> u32 {
    renderer_surface_word(e, renderer, 0x8)
}

// Translated from 00b6c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`): as `00b6c1a0`, with the word at +0xc (the
/// height its callers use).
pub fn fn_00b6c1d0(e: &mut Engine, renderer: Ptr) -> u32 {
    renderer_surface_word(e, renderer, 0xc)
}

// Translated from 00b6c200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`), `cdecl`, one word: 1 for a negative value; 0 for
/// 0, 1 and values above 0x10; the value itself for 2 to 0x10 (the arms of the switch, jump
/// table at `00b6c278` read in Ghidra).
pub fn fn_00b6c200(value: i32) -> u32 {
    if value < 0 {
        return 1;
    }
    match value {
        2..=0x10 => value as u32,
        _ => 0,
    }
}

/// `fn_00b6c200` registered with the engine's argument form.
fn fn_00b6c200_entry(_e: &mut Engine, value: i32) -> u32 {
    fn_00b6c200(value)
}

/// Bytes and words `00b6d170` sets around the texture's making and puts back after it.
const MAKING_FLAG_011F4AC0: u32 = 0x011f_4ac0;
const MAKING_FORMAT_011AB3C8: u32 = 0x011a_b3c8;
const FLAG_011AB3C4: u32 = 0x011a_b3c4;
const SETTING_011BF400: u32 = 0x011b_f400;
/// The byte that keeps the asked width and height (`011f9183`); without it both become the
/// smallest power of two not below the larger one.
const KEEP_SIZE_011F9183: u32 = 0x011f_9183;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// The texture makers: `00ee8330` (`cdecl`: width, renderer, the three-word block) and the
/// 0x40-byte object's constructor `00b6b380` (`thiscall`, three words), or `00b6b610`
/// (`cdecl`, nine words) with a name from `00a5b690` (`cdecl`, the string `01011584`).
const MAKE_SURFACE_00EE8330: u32 = 0x00ee_8330;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const TEXTURE_CONSTRUCT_00B6B380: u32 = 0x00b6_b380;
const MAKE_TEXTURE_00B6B610: u32 = 0x00b6_b610;
const MAKE_FIXED_STRING_00A5B690: u32 = 0x00a5_b690;
const EMPTY_NAME_01011584: u32 = 0x0101_1584;
/// The word the name is compared with before its release (`0109b220`).
const NULL_NAME_0109B220: u32 = 0x0109_b220;
/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b6d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6c0d0`): the rendered-texture maker
/// `BorrowRenderedTexture` (`00b6d870`) calls; `thiscall` on the texture manager with ten
/// stack words (`RET 0x28`), of which the first, fifth and seventh are not read.
///
/// The width and height are kept when the byte at `011f9183` is set; otherwise both become the
/// smallest power of two (from 1) not below the larger of them. A three-word block (2, 2, 2)
/// goes to the maker. A nonzero `format` sets the byte `011f4ac0` and is stored in
/// `011ab3c8`; flag 8 clears the byte `011ab3c4`; flag 0x80 clears the word `011bf400`;
/// without flag 0x100 the manager's word at +0xa8 is passed (else 0); with flag 0x40 the
/// `multisample` count 2, 4 or 8 becomes 1, 2 or 3 (others 0; 0 without the flag). With flag
/// 0x10 `00ee8330(width, renderer 011f9508, block)` makes a surface and, when it is not null, a
/// 0x40-byte object is allocated and constructed on it (`00b6b380(surface, 1, word)`; a null
/// allocation answers 0). Otherwise `00b6b610` gets the address of a name from
/// `00a5b690("")`, the width, height, the block, the multisample value, the asked height's
/// upper bytes with the low byte `!(flags >> 2) & 1` (the exe rewrites only the low byte of
/// that argument's slot), the word and the last two arguments; the name is then released
/// (`InterlockedDecrement` on its word at -8) unless it equals the word at `0109b220`.
/// Afterwards the byte `011f4ac0` is cleared when `format` is nonzero, `011ab3c4` set for flag
/// 8 and `011bf400` set to 1 for flag 0x80. Answers the object (0 when nothing was made).
// Translation incomplete: 00b6d170 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocation and the name) is not translated.
pub fn fn_00b6d170(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    width: u32,
    height: u32,
    format: u32,
    _unused_5: u32,
    flags: u32,
    _unused_7: u32,
    multisample: u32,
    last_but_one: u32,
    last: u32,
) -> u32 {
    let (made_width, made_height) = if e.global::<u8>(KEEP_SIZE_011F9183) != 0 {
        (width, height)
    } else {
        let larger = if width > height { width } else { height };
        let mut size = 1u32;
        while size < larger {
            size = size.wrapping_add(size);
        }
        (size, size)
    };
    if format != 0 {
        e.set_global::<u8>(MAKING_FLAG_011F4AC0, 1);
        e.set_global::<u32>(MAKING_FORMAT_011AB3C8, format);
    }
    let flag_8 = flags & 8;
    if flag_8 != 0 {
        e.set_global::<u8>(FLAG_011AB3C4, 0);
    }
    let flag_80 = flags & 0x80;
    if flag_80 != 0 {
        e.set_global::<u32>(SETTING_011BF400, 0);
    }
    let word = if flags & 0x100 == 0 {
        e.mem.u32(this.addr() + 0xa8)
    } else {
        0
    };
    let samples = if flags & 0x40 != 0 {
        match multisample {
            2 => 1u32,
            4 => 2,
            8 => 3,
            _ => 0,
        }
    } else {
        0
    };
    let bit_word = (height & 0xffff_ff00) | (u32::from(!((flags >> 2) as u8)) & 1);
    let made = e.with_stack(0x10, |e, frame| {
        let block = frame.addr();
        for offset in [0u32, 4, 8] {
            e.mem.set_u32(block + offset, 2);
        }
        let name_slot = block + 0xc;
        if flags & 0x10 != 0 {
            let renderer = e.global::<u32>(RENDERER_011F9508);
            let surface = e
                .call(MAKE_SURFACE_00EE8330, &args![made_width, renderer, block])
                .u32();
            if surface == 0 {
                return 0;
            }
            let object = e.call(NI_ALLOCATE, &args![0x40u32]).u32();
            if object == 0 {
                return 0;
            }
            e.call(
                TEXTURE_CONSTRUCT_00B6B380,
                &args![object, surface, 1u32, word],
            )
            .u32()
        } else {
            let name = e
                .call(MAKE_FIXED_STRING_00A5B690, &args![EMPTY_NAME_01011584])
                .u32();
            e.mem.set_u32(name_slot, name);
            let made = e
                .call(
                    MAKE_TEXTURE_00B6B610,
                    &args![
                        name_slot,
                        made_width,
                        made_height,
                        block,
                        samples,
                        bit_word,
                        word,
                        last_but_one,
                        last
                    ],
                )
                .u32();
            let name = e.mem.u32(name_slot);
            if name != e.global::<u32>(NULL_NAME_0109B220) {
                e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
            }
            made
        }
    });
    if format != 0 {
        e.set_global::<u8>(MAKING_FLAG_011F4AC0, 0);
    }
    if flag_8 != 0 {
        e.set_global::<u8>(FLAG_011AB3C4, 1);
    }
    if flag_80 != 0 {
        e.set_global::<u32>(SETTING_011BF400, 1);
    }
    made
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_c1a0, fn_00b6c1a0(Ptr) -> u32),
        entry!(0x00b6_c1d0, fn_00b6c1d0(Ptr) -> u32),
        entry!(0x00b6_c200, fn_00b6c200_entry(i32) -> u32),
        entry!(0x00b6_d170, fn_00b6d170(Ptr, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00b6_c2c0,
            fn_00b6c2c0(Ptr, u32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: u32 = 0x0f00_1000;
    const TARGET_WIDTH: u32 = 0x0f00_208c;
    const TARGET_HEIGHT: u32 = 0x0f00_2090;

    /// The renderer: slot +0xc8 answers a target sized `target` (its words at +4, +8); the
    /// helpers answer `helpers`. `00b6c200` answers `query`. The data words are distinct.
    struct Rig {
        e: Engine,
        renderer: u32,
        out: u32,
    }

    fn rig(target: (u32, u32), helpers: (u32, u32), query: i32) -> Rig {
        let mut e = Engine::new();
        e.map(0x0109_9000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        e.set_global(HALF_01011588, 0.5f64.to_bits());
        e.set_global(WIDTH_011F9430, 0xffff_fffdu32);
        e.set_global(HEIGHT_011F9434, 0x0000_0402u32);
        e.set_global(SIZE_01200054, 0x54u32);
        e.set_global(WIDTH_011AD814, 0x814u32);
        e.set_global(HEIGHT_011AD818, 0x818u32);
        e.set_global(EXTRA_011AD810, 0x810u32);
        e.set_global(WIDTH_011AD81C, 0x81cu32);
        e.set_global(HEIGHT_011AD820, 0x820u32);
        e.set_global(SIZE_011ADBC8, 0xbc8u32);
        e.set_global(SIZE_011AD830, 0x1234_0830u32);
        e.set_global(FORMAT_011AD870, 0x870u32);
        let mut renderer_slots = vec![0u32; 0x33];
        renderer_slots[0x32] = 0x0f00_00c8;
        e.put_vtable(0x0f00_0000, &renderer_slots);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, 0x0f00_0000);
        e.register(0x0f00_00c8, |_, _| TARGET.into_ret());
        let mut target_slots = vec![0u32; 0x25];
        target_slots[0x23] = TARGET_WIDTH;
        target_slots[0x24] = TARGET_HEIGHT;
        e.put_vtable(0x0f00_2000, &target_slots);
        e.map(TARGET, 0x10);
        e.mem.set_u32(TARGET, 0x0f00_2000);
        e.mem.set_u32(TARGET + 4, target.0);
        e.mem.set_u32(TARGET + 8, target.1);
        e.register(TARGET_WIDTH, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TARGET_HEIGHT, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register_double(RENDERER_WIDTH_00B6C1A0, move |_, _| helpers.0.into_ret());
        e.register_double(RENDERER_HEIGHT_00B6C1D0, move |_, _| helpers.1.into_ret());
        e.register_double(QUERY_00B6C200, move |_, _| (query as u32).into_ret());
        let out = e.mem.alloc(0x20);
        Rig { e, renderer, out }
    }

    /// Runs the kind and answers (width, height, format, cleared, flags, last).
    fn describe(r: &mut Rig, kind: u32) -> [u32; 6] {
        let o = r.out;
        for i in 0..6 {
            r.e.mem.set_u32(o + 4 * i, 0xdead_0000 + i);
        }
        r.e.call(
            0x00b6_c2c0,
            &args![
                r.renderer,
                kind,
                o,
                o + 4,
                o + 8,
                o + 0xc,
                o + 0x10,
                o + 0x14
            ],
        );
        [0, 1, 2, 3, 4, 5].map(|i| r.e.mem.u32(o + 4 * i))
    }

    #[test]
    fn fn_00b6c2c0_kind_0_takes_a_quarter_of_a_large_target() {
        let mut r = rig((1920, 1080), (0, 0), 7);
        assert_eq!(describe(&mut r, 0), [480, 270, 0x71, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 2), [480, 270, 0x71, 0, 0x26, 0]);
        // A target of at most 0x500 wide: 0x100 square.
        let mut r = rig((0x500, 720), (0, 0), 7);
        assert_eq!(describe(&mut r, 0), [0x100, 0x100, 0x71, 0, 0x26, 0]);
        // With the override: signed quarters of the words (toward zero).
        let mut r = rig((1920, 1080), (0, 0), 7);
        r.e.set_global(SIZE_OVERRIDE_011F9426, 1u8);
        assert_eq!(describe(&mut r, 0), [0, 0x100, 0x71, 0, 0x26, 0]);
    }

    #[test]
    fn fn_00b6c2c0_kind_1_halves_the_helper_width_through_the_fpu() {
        let mut r = rig((0, 0), (1001, 0), 0);
        assert_eq!(describe(&mut r, 1), [500, 0x100, 0x71, 0, 0x26, 0]);
        let mut r = rig((0, 0), (0x1ff, 0), 0);
        assert_eq!(describe(&mut r, 1), [0x100, 0x100, 0x71, 0, 0x26, 0]);
        // Above 2^31: loaded unsigned.
        let mut r = rig((0, 0), (0xffff_fffe, 0), 0);
        assert_eq!(describe(&mut r, 1)[0], 0x7fff_ffff);
    }

    #[test]
    fn fn_00b6c2c0_helper_and_target_sizes_follow_the_override() {
        let mut r = rig((1920, 1080), (800, 600), 0);
        assert_eq!(describe(&mut r, 3), [800, 600, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 6), [1920, 1080, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x18), [480, 270, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x25), [200, 150, 0x15, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2e), [960, 540, 0, 0, 4, 0]);
        r.e.set_global(SIZE_OVERRIDE_011F9426, 1u8);
        assert_eq!(describe(&mut r, 3), [0xffff_fffd, 0x402, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 0x19), [0xffff_fffd, 0x402, 0, 0, 0x26, 0]);
        assert_eq!(describe(&mut r, 0x18), [0, 0x100, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2e), [0xffff_ffff, 0x201, 0, 0, 4, 0]);
    }

    #[test]
    fn fn_00b6c2c0_flags_with_bit_0x40_keep_the_last_word() {
        // Kind 4: flags 0x62 when 00b6c200 answers above 0; the last word kept.
        let mut r = rig((0, 0), (64, 32), 1);
        assert_eq!(describe(&mut r, 4), [64, 32, 0, 0, 0x62, 1]);
        let mut r2 = rig((0, 0), (64, 32), 0);
        r2.e.set_global(BYTE_SETTING_011F941E, 1u8);
        assert_eq!(describe(&mut r2, 4), [64, 32, 0x71, 0, 0x22, 0]);
        // Kind 9 takes the last word from 011ad810 and keeps it (flags 0x60).
        assert_eq!(describe(&mut r, 9), [0x814, 0x818, 0, 0, 0x60, 0x810]);
        assert_eq!(describe(&mut r, 0x16), [0x814, 0x818, 0, 0, 0x60, 0x810]);
        // Kind 0x1f: 0x46 with a positive answer.
        assert_eq!(describe(&mut r, 0x1f), [64, 32, 0, 0, 0x46, 1]);
        assert_eq!(describe(&mut r2, 0x1f), [64, 32, 0, 0, 6, 0]);
    }

    #[test]
    fn fn_00b6c2c0_constant_and_data_kinds() {
        let mut r = rig((0, 0), (0, 0), 5);
        assert_eq!(describe(&mut r, 7), [0x54, 0x54, 0x74, 0, 6, 0]);
        assert_eq!(describe(&mut r, 8), [0x200, 0x200, 0x24, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0xa), [0x200, 0x200, 0, 0, 0x40, 5]);
        assert_eq!(describe(&mut r, 0xc), [0x80, 0x80, 0x24, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0xd), [0x54, 1, 0x51, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0xe), [0x54, 0x10, 0x74, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0xf), [0x54, 0x54, 0x72, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x10), [0x54, 0x54, 0x74, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x11), [0x200, 0x200, 0x15, 0, 0x122, 0]);
        assert_eq!(describe(&mut r, 0x12), [0x81c, 0x820, 0x15, 0, 0xc, 0]);
        assert_eq!(describe(&mut r, 0x13), [0x81c, 0x820, 0x15, 0, 0x24, 0]);
        assert_eq!(describe(&mut r, 0x14), [0x200, 0x200, 0x71, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x15), [0x54, 0x54, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x17), [0x100, 0x100, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x1a), [0xbc8, 0xbc8, 0, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x1c), [0x80, 0x80, 0x17, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x1d), [0x80, 0x80, 0x17, 0, 0, 0]);
        assert_eq!(describe(&mut r, 0x29), [0x100, 0x100, 0, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x2a), [0x200, 0x200, 0x72, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2b), [0x830, 0x830, 0x870, 0, 6, 0]);
        assert_eq!(describe(&mut r, 0x2c), [0x100, 0x100, 0, 0, 0x10, 0]);
        assert_eq!(describe(&mut r, 0x32), [0x100, 0x100, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x33), [8, 8, 0, 0, 0, 0]);
        r.e.set_global(BYTE_SETTING_012004FC, 1u8);
        assert_eq!(describe(&mut r, 0x1e), [0x200, 0x200, 0x17, 0, 0, 0]);
    }

    #[test]
    fn fn_00b6c2c0_kinds_0x20_0x21_take_the_format_byte() {
        let mut r = rig((1920, 1080), (800, 600), 0);
        assert_eq!(describe(&mut r, 0x20), [1920, 1080, 0x15, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x21), [800, 600, 0x15, 0, 2, 0]);
        r.e.set_global(BYTE_SETTING_011AD80E, 1u8);
        assert_eq!(describe(&mut r, 0x20), [1920, 1080, 0x71, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x21), [800, 600, 0x72, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x23), [1920, 1080, 0x15, 0, 0x42, 0]);
        assert_eq!(describe(&mut r, 0x27), [800, 600, 0x15, 0, 2, 0]);
        assert_eq!(describe(&mut r, 0x2f), [800, 600, 0, 0, 0x40, 0]);
        assert_eq!(describe(&mut r, 5), [800, 600, 0x71, 0, 0x22, 0]);
    }

    #[test]
    fn fn_00b6c2c0_aspect_kinds_scale_the_height_to_0x500_wide() {
        let mut r = rig((1920, 1080), (0, 0), 3);
        assert_eq!(describe(&mut r, 0x2d), [0x500, 720, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x31), [0x500, 720, 0, 0, 4, 0]);
        assert_eq!(describe(&mut r, 0x30), [0x500, 720, 0, 0, 0x40, 3]);
        // A zero-wide target: infinity, the indefinite integer's low word.
        let mut r = rig((0, 1080), (0, 0), 3);
        assert_eq!(describe(&mut r, 0x2d)[1], 0);
    }

    #[test]
    fn fn_00b6c2c0_kind_0xb_and_out_of_range_kinds_store_nothing_but_the_ends() {
        let mut r = rig((0, 0), (0, 0), 9);
        let o = r.out;
        let result = describe(&mut r, 0xb);
        // The flags word is the caller's (0xdead0004: bit 0x40 clear), so the last is cleared.
        assert_eq!(
            result,
            [0xdead_0000, 0xdead_0001, 0xdead_0002, 0, 0xdead_0004, 0]
        );
        r.e.mem.set_u32(o + 0x10, 0x40);
        r.e.call(
            0x00b6_c2c0,
            &args![
                r.renderer,
                0x34u32,
                o,
                o + 4,
                o + 8,
                o + 0xc,
                o + 0x10,
                o + 0x14
            ],
        );
        assert_eq!(r.e.mem.u32(o + 0x14), 9);
        assert_eq!(fistp_qword_truncate(f64::NAN), 0x8000_0000_0000_0000);
        assert_eq!(signed_quarter(0xffff_fffb), 0xffff_ffff);
        assert_eq!(signed_half(0xffff_fffd), 0xffff_ffff);
    }
}

#[cfg(test)]
mod b000h_tests {
    use super::*;

    const TARGET_SLOT_C8: u32 = 0x0e10_0000;
    const SURFACE_SLOT_B0: u32 = 0x0e10_0010;

    /// A renderer whose slot +0xc8 answers the target (or null) and a target whose slot +0xb0
    /// answers a surface (or null) with the words 7 and 9 at +8 and +0xc.
    fn surfaces(target: bool, surface: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let renderer = e.mem.alloc(4);
        let target_object = e.mem.alloc(4);
        let surface_object = e.mem.alloc(0x10);
        e.mem.set_u32(surface_object + 8, 7);
        e.mem.set_u32(surface_object + 0xc, 9);
        e.put_vtable(0x0e00_0000, &vec![TARGET_SLOT_C8; 0x33]);
        e.put_vtable(0x0e00_1000, &vec![SURFACE_SLOT_B0; 0x2d]);
        e.mem.set_u32(renderer, 0x0e00_0000);
        e.mem.set_u32(target_object, 0x0e00_1000);
        let t = if target { target_object } else { 0 };
        let s = if surface { surface_object } else { 0 };
        e.register_double(TARGET_SLOT_C8, move |_, _| t.into_ret());
        e.register_double(SURFACE_SLOT_B0, move |_, a| {
            assert_eq!(a[1], 0);
            s.into_ret()
        });
        (e, renderer)
    }

    #[test]
    fn fn_00b6c1a0_and_00b6c1d0_read_the_surface_words() {
        let (mut e, renderer) = surfaces(true, true);
        assert_eq!(e.call(0x00b6_c1a0, &args![renderer]).u32(), 7);
        assert_eq!(e.call(0x00b6_c1d0, &args![renderer]).u32(), 9);
        let (mut e, renderer) = surfaces(true, false);
        assert_eq!(e.call(0x00b6_c1a0, &args![renderer]).u32(), 0);
        assert_eq!(e.call(0x00b6_c1d0, &args![renderer]).u32(), 0);
        let (mut e, renderer) = surfaces(false, true);
        assert_eq!(e.call(0x00b6_c1a0, &args![renderer]).u32(), 0);
        assert_eq!(e.call(0x00b6_c1d0, &args![renderer]).u32(), 0);
    }

    #[test]
    fn fn_00b6c200_maps_the_switch() {
        let mut e = Engine::new();
        let mut ask = |v: i32| e.call(0x00b6_c200, &args![v]).u32();
        assert_eq!(ask(-5), 1);
        assert_eq!(ask(i32::MIN), 1);
        assert_eq!(ask(0), 0);
        assert_eq!(ask(1), 0);
        for v in 2..=0x10 {
            assert_eq!(ask(v), v as u32);
        }
        assert_eq!(ask(0x11), 0);
        assert_eq!(ask(0x7fff_ffff), 0);
    }

    const MANAGER_WORD: u32 = 0x0abc_0000;
    const SURFACE: u32 = 0x0d00_0000;
    const OBJECT: u32 = 0x0d10_0000;
    const NAME: u32 = 0x0d20_0008;
    const RENDERER: u32 = 0x0d30_0000;
    const SEEN: u32 = 0x0d40_0000;

    /// Copies the three-word block and the globals the maker changes into `SEEN`.
    fn record(e: &mut Engine, block: u32) {
        for i in 0..3 {
            let w = e.mem.u32(block + 4 * i);
            e.mem.set_u32(SEEN + 4 * i, w);
        }
        let flag = e.global::<u8>(MAKING_FLAG_011F4AC0);
        e.mem.set_u8(SEEN + 0xc, flag);
        let format = e.global::<u32>(MAKING_FORMAT_011AB3C8);
        e.mem.set_u32(SEEN + 0x10, format);
        let byte = e.global::<u8>(FLAG_011AB3C4);
        e.mem.set_u8(SEEN + 0x14, byte);
        let word = e.global::<u32>(SETTING_011BF400);
        e.mem.set_u32(SEEN + 0x18, word);
    }

    /// The texture manager, the globals and doubles for every maker.
    fn maker(keep_size: bool, surface: u32, allocation: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x011f_4000u32,
            0x011a_b000,
            0x011b_f000,
            0x011f_9000,
            0x0109_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(SEEN, 0x100);
        e.map(NAME - 8, 0x10);
        e.set_global::<u8>(KEEP_SIZE_011F9183, u8::from(keep_size));
        e.set_global::<u32>(RENDERER_011F9508, RENDERER);
        e.set_global::<u8>(FLAG_011AB3C4, 1);
        e.set_global::<u32>(SETTING_011BF400, 1);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0xa8, MANAGER_WORD);
        e.register_double(MAKE_SURFACE_00EE8330, move |e, a| {
            record(e, a[2]);
            surface.into_ret()
        });
        e.register_double(NI_ALLOCATE, move |_, _| allocation.into_ret());
        e.register(TEXTURE_CONSTRUCT_00B6B380, |_, a| a[0].into_ret());
        e.register(MAKE_FIXED_STRING_00A5B690, |_, _| NAME.into_ret());
        e.register(MAKE_TEXTURE_00B6B610, |e, a| {
            record(e, a[3]);
            assert_eq!(e.mem.u32(a[0]), NAME);
            OBJECT.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Runs `00b6d170` with (width, height, format, flags, multisample) and fillers elsewhere.
    fn make(e: &mut Engine, this: u32, words: [u32; 5]) -> u32 {
        let [width, height, format, flags, multisample] = words;
        e.call(
            0x00b6_d170,
            &args![
                this,
                1u32,
                width,
                height,
                format,
                5u32,
                flags,
                7u32,
                multisample,
                0x99u32,
                0x77u32
            ],
        )
        .u32()
    }

    #[test]
    fn fn_00b6d170_named_path_rounds_the_size_and_passes_the_words() {
        let (mut e, this) = maker(false, 0, 0);
        let made = make(&mut e, this, [0x300, 0x1_0120, 0, 0x40, 4]);
        assert_eq!(made, OBJECT);
        let named = calls(&e, MAKE_TEXTURE_00B6B610);
        assert_eq!(named.len(), 1);
        let a = &named[0];
        assert_eq!(&a[1..3], &[0x2_0000, 0x2_0000]);
        assert_eq!(a[4], 2);
        // Flag 4 clear: the low byte is 1, the upper bytes are the asked height's.
        assert_eq!(a[5], 0x1_0101);
        assert_eq!(&a[6..], &[MANAGER_WORD, 0x99, 0x77]);
        assert_eq!(e.mem.u32(SEEN), 2);
        assert_eq!(e.mem.u32(SEEN + 4), 2);
        assert_eq!(e.mem.u32(SEEN + 8), 2);
        assert_eq!(
            calls(&e, MAKE_FIXED_STRING_00A5B690),
            vec![vec![EMPTY_NAME_01011584]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![NAME - 8]]
        );
        assert!(calls(&e, MAKE_SURFACE_00EE8330).is_empty());
    }

    #[test]
    fn fn_00b6d170_flags_change_the_globals_only_while_making() {
        let (mut e, this) = maker(true, 0, 0);
        let made = make(
            &mut e,
            this,
            [0x300, 0x200, 0x15, 0x100 | 0x80 | 0x8 | 0x4, 3],
        );
        assert_eq!(made, OBJECT);
        let a = &calls(&e, MAKE_TEXTURE_00B6B610)[0];
        assert_eq!(&a[1..3], &[0x300, 0x200]);
        assert_eq!(a[4], 0);
        assert_eq!(a[5], 0x200);
        assert_eq!(a[6], 0);
        assert_eq!(e.mem.u8(SEEN + 0xc), 1);
        assert_eq!(e.mem.u32(SEEN + 0x10), 0x15);
        assert_eq!(e.mem.u8(SEEN + 0x14), 0);
        assert_eq!(e.mem.u32(SEEN + 0x18), 0);
        assert_eq!(e.global::<u8>(MAKING_FLAG_011F4AC0), 0);
        assert_eq!(e.global::<u8>(FLAG_011AB3C4), 1);
        assert_eq!(e.global::<u32>(SETTING_011BF400), 1);
    }

    #[test]
    fn fn_00b6d170_name_equal_to_the_null_word_is_not_released() {
        let (mut e, this) = maker(true, 0, 0);
        e.set_global::<u32>(NULL_NAME_0109B220, NAME);
        make(&mut e, this, [4, 4, 0, 0x40, 8]);
        assert_eq!(calls(&e, MAKE_TEXTURE_00B6B610)[0][4], 3);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn fn_00b6d170_surface_path() {
        let (mut e, this) = maker(false, SURFACE, OBJECT);
        assert_eq!(make(&mut e, this, [3, 2, 0, 0x10, 2]), OBJECT);
        assert_eq!(calls(&e, MAKE_SURFACE_00EE8330)[0][..2], [4, RENDERER]);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x40]]);
        assert_eq!(
            calls(&e, TEXTURE_CONSTRUCT_00B6B380),
            vec![vec![OBJECT, SURFACE, 1, MANAGER_WORD]]
        );
        assert!(calls(&e, MAKE_TEXTURE_00B6B610).is_empty());
        // No surface: nothing allocated; a failed allocation answers 0.
        let (mut e, this) = maker(false, 0, OBJECT);
        assert_eq!(make(&mut e, this, [3, 2, 0, 0x10, 2]), 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        let (mut e, this) = maker(false, SURFACE, 0);
        assert_eq!(make(&mut e, this, [3, 2, 0, 0x10, 2]), 0);
        assert!(calls(&e, TEXTURE_CONSTRUCT_00B6B380).is_empty());
    }
}
