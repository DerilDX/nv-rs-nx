//! `NiXenonRenderer/run_00e87980` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e88c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with one word: stores its low byte
/// at +0x18.
pub fn fn_00e88c20(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x18), value);
}

/// What the render state's base constructor uses: the `NiRefObject` table and global object
/// count (`011f4400`), the interlocked imports, its table (`010ef60c`), the member set-up
/// `00e68810` (unnamed; `thiscall`, no stack words), the renderer hook-up `00e87af0` (unnamed;
/// `thiscall`, the renderer), the allocation `00aa13e0`, the object constructor `00a5c200`
/// (`niobjectnet.cpp`; `thiscall`, no stack words) and the table it then gets (`010162dc`),
/// the default colour words (`011f4998` to `011f49a0`), the `double` 255.0 (`0101e568`),
/// `memset` and the state tables it fills (`0126f918` to `0126f988`).
const NI_REF_OBJECT_VTABLE_0101DCE4: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const INTERLOCKED_UP: u32 = 0x00fd_f1c0;
const INTERLOCKED_DOWN: u32 = 0x00fd_f1c4;
const RENDER_STATE_BASE_VTABLE: u32 = 0x010e_f60c;
const MEMBER_SET_UP_00E68810: u32 = 0x00e6_8810;
const HOOK_UP_00E87AF0: u32 = 0x00e8_7af0;
const NEW_OBJECT_00AA13E0: u32 = 0x00aa_13e0;
const OBJECT_CONSTRUCT_00A5C200: u32 = 0x00a5_c200;
const MEMBER_74_VTABLE_010162DC: u32 = 0x0101_62dc;
const DEFAULT_COLOR_011F4998: u32 = 0x011f_4998;
const SCALE_0101E568: u32 = 0x0101_e568;
const MEMSET_00EC61C0: u32 = 0x00ec_61c0;
const STATE_TABLE_0126F958: u32 = 0x0126_f958;

/// The low byte of a colour channel: `FISTP` with the rounding control set to chop.
fn channel_byte(value: f64) -> u32 {
    (crate::x87::fistp_chop(value) as u32) & 0xff
}

// Translated from 00e881a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`): the render state's base constructor,
/// `thiscall` with the renderer. The `NiRefObject` start and its table, +0x74 cleared and
/// three `float`s +0x8c to +0x94 0.0, the member at +0xf8 (`00e68810`), the 256, 128 and 80
/// state pairs from +0x120, +0xa20 and +0xe20 marked unset (0x7fffffff), +0x10f0, +0x10f8 and
/// +0x10fc cleared, the renderer hooked up (`00e87af0`), many state words given their
/// defaults, an object (0x1c bytes, `00a5c200`, its table, 16-bit +0x18 0xec, byte +0x1a 0)
/// held at +0x74 (`NiPointer` assignment), four `float`s 0.0 and +0x88 1.0, the colour words
/// from `011f4998` at +0x8c and their packed form (alpha 0xff, each channel times 255.0
/// truncated) at +0x98, and the state tables at `0126f958` (0x42 bytes of 0xff, then its
/// 16-bit entries) and `0126f918` to `0126f944`. Answers `this`.
pub fn fn_00e881a0(e: &mut Engine, this: Ptr, renderer: u32) -> Ptr {
    let base = this.addr();
    let set = |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(base + offset, value);
    let zero = x87_store32(0.0);
    let one = x87_store32(1.0);
    set(e, 0, NI_REF_OBJECT_VTABLE_0101DCE4);
    set(e, 4, 0);
    e.call(INTERLOCKED_UP, &args![OBJECT_COUNT_011F4400]);
    set(e, 0, RENDER_STATE_BASE_VTABLE);
    set(e, 0x74, 0);
    for offset in [0x8cu32, 0x90, 0x94] {
        set(e, offset, zero);
    }
    e.call(MEMBER_SET_UP_00E68810, &args![base + 0xf8]);
    for (start, count) in [(0x120u32, 0x100u32), (0xa20, 0x80), (0xe20, 0x50)] {
        for pair in 0..count {
            set(e, start + 8 * pair, 0x7fff_ffff);
        }
    }
    set(e, 0x10f0, 0);
    set(e, 0x10f8, 0);
    set(e, 0x10fc, 0);
    e.call(HOOK_UP_00E87AF0, &args![base, renderer]);
    set(e, 0xc, 0);
    set(e, 0x10, 1);
    set(e, 0x14, 1);
    set(e, 0x18, 2);
    set(e, 0x1c, 3);
    set(e, 0x6c, 0);
    set(e, 0x70, 0);
    let block = e.call(NEW_OBJECT_00AA13E0, &args![0x1cu32]).u32();
    let object = if block != 0 {
        e.call(OBJECT_CONSTRUCT_00A5C200, &args![block]);
        e.mem.set_u32(block, MEMBER_74_VTABLE_010162DC);
        e.mem.set_u16(block + 0x18, 0xec);
        e.mem.set_u8(block + 0x1a, 0);
        block
    } else {
        0
    };
    let held = e.mem.u32(base + 0x74);
    if held != object {
        if held != 0 && e.call(INTERLOCKED_DOWN, &args![held + 4]).u32() == 0 {
            e.vcall(held, 4, &args![]);
        }
        set(e, 0x74, object);
        if object != 0 {
            e.call(INTERLOCKED_UP, &args![object + 4]);
        }
    }
    for offset in [0x78u32, 0x7c, 0x80, 0x84] {
        set(e, offset, zero);
    }
    for (offset, value) in [
        (0x2cu32, 4u32),
        (0x58, 4),
        (0xa8, 4),
        (0xc8, 4),
        (0x38, 5),
        (0x5c, 5),
        (0xac, 5),
        (0xcc, 5),
        (0x20, 2),
        (0x24, 1),
        (0x28, 3),
        (0x30, 9),
        (0x34, 0xa),
        (0x3c, 6),
        (0x40, 7),
        (0x44, 8),
        (0x48, 0xb),
        (0x4c, 8),
        (0x50, 2),
        (0x54, 3),
        (0x60, 6),
        (0x64, 7),
        (0x68, 1),
        (0xb8, 8),
        (0xa0, 2),
        (0xa4, 3),
        (0xb0, 6),
        (0xb4, 7),
        (0x9c, 1),
        (0xbc, 1),
        (0xc0, 2),
        (0xc4, 3),
        (0xd0, 6),
        (0xd4, 2),
        (0xdc, 2),
        (0xe4, 3),
        (0xec, 1),
        (0xd8, 3),
        (0xe0, 3),
        (0xe8, 2),
        (0xf0, 1),
        (0xf4, 0),
    ] {
        set(e, offset, value);
    }
    set(e, 0x88, one);
    for word in 0..3 {
        let value = e.mem.u32(DEFAULT_COLOR_011F4998 + 4 * word);
        set(e, 0x8c + 4 * word, value);
    }
    let scale = x87_load64(e.mem.u64(SCALE_0101E568));
    // FMUL ST(1),ST(0) (bytes DC C9), then FMUL ST(0),ST(1) (D8 C9), then FMUL dword.
    let red = x87_mul(x87_load32(e.mem.u32(base + 0x8c)), scale);
    let mut packed = (0xffff_ff00 | channel_byte(red)) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(base + 0x90)), scale);
    packed = (packed | channel_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(base + 0x94)));
    set(e, 8, 0);
    for (offset, value) in [
        (0x100u32, 8u32),
        (0x104, 2),
        (0x108, 3),
        (0x10c, 4),
        (0x110, 5),
        (0x114, 6),
        (0x118, 7),
        (0x11c, 1),
    ] {
        set(e, offset, value);
    }
    packed |= channel_byte(blue);
    set(e, 0x98, packed);
    e.mem.set_u8(base + 0x10f4, 0);
    e.mem.set_u8(base + 0x10f5, 0);
    e.call(
        MEMSET_00EC61C0,
        &args![STATE_TABLE_0126F958, 0xffffu32, 0x42u32],
    );
    let half = |e: &mut Engine, at: u32, value: u16| e.mem.set_u16(at, value);
    for (at, value) in [
        (0x0126_f95au32, 0u16),
        (0x0126_f95e, 2),
        (0x0126_f95c, 1),
        (0x0126_f964, 5),
        (0x0126_f960, 3),
        (0x0126_f962, 4),
        (0x0126_f948, 1),
        (0x0126_f966, 6),
        (0x0126_f988, 7),
        (0x0126_f94e, 4),
        (0x0126_f94a, 2),
        (0x0126_f94c, 3),
        (0x0126_f954, 7),
        (0x0126_f950, 5),
        (0x0126_f952, 6),
    ] {
        half(e, at, value);
    }
    for at in [0x0126_f92cu32, 0x0126_f938, 0x0126_f930, 0x0126_f934] {
        e.mem.set_u32(at, 0xffff_ffff);
    }
    half(e, 0x0126_f956, 0x18);
    for at in [0x0126_f93cu32, 0x0126_f940, 0x0126_f944] {
        e.mem.set_u32(at, 0xffff_ffff);
    }
    for (at, value) in [
        (0x0126_f92eu32, 0u16),
        (0x0126_f930, 1),
        (0x0126_f936, 2),
        (0x0126_f938, 3),
        (0x0126_f93a, 4),
    ] {
        half(e, at, value);
    }
    for (at, value) in [
        (0x0126_f918u32, 1u32),
        (0x0126_f91c, 2),
        (0x0126_f920, 5),
        (0x0126_f924, 6),
        (0x0126_f928, 7),
    ] {
        e.mem.set_u32(at, value);
    }
    this
}

// Translated from 00e87af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`): the render state's renderer hook-up,
/// `thiscall` with one word (the renderer). Stores the renderer at +0x10fc. With a renderer,
/// the object at its +0x288 replaces the one held at +0x10f8: the old one gets its vtable
/// slot 8 call (release) first, then the new one is stored and gets its slot 4 call (add
/// reference) when not null. Without one, the held object gets its slot 8 call and +0x10f8 is
/// cleared.
pub fn fn_00e87af0(e: &mut Engine, this: Ptr, renderer: u32) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x10fc), renderer);
    if renderer != 0 {
        let object = e.mem.u32(renderer.wrapping_add(0x288));
        let held = e.mem.u32(base.wrapping_add(0x10f8));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(base.wrapping_add(0x10f8), object);
        if object != 0 {
            e.vcall(object, 4, &args![]);
        }
        return;
    }
    let held = e.mem.u32(base.wrapping_add(0x10f8));
    if held != 0 {
        e.vcall(held, 8, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(0x10f8), 0);
}

/// The render state's own virtual slot 0x68 (`SetRenderState`-like: state, value, two more
/// words) and its device word (+0x10f8).
const SET_STATE_SLOT: u32 = 0x68;
const STATE_DEVICE: u32 = 0x10f8;

// Translated from 00e879b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with an index and a word: its slot
/// 0x68 with (0x16, the word at +0xd4 + 4 * (the word at +0xf4 + 2 * index), 0, the word).
pub fn fn_00e879b0(e: &mut Engine, this: Ptr, index: u32, word: u32) {
    let base = this.addr();
    let row = e
        .mem
        .u32(base.wrapping_add(0xf4))
        .wrapping_add(index.wrapping_mul(2));
    let value = e
        .mem
        .u32(base.wrapping_add(0xd4).wrapping_add(row.wrapping_mul(4)));
    e.vcall(base, SET_STATE_SLOT, &args![0x16u32, value, 0u32, word]);
}

/// The `double` 0.99 (`0106d508`) and the `float` 1.01 (`0106b9e0`) the alpha test compares
/// with.
const ALPHA_LOW_0106D508: u32 = 0x0106_d508;
const ALPHA_HIGH_0106B9E0: u32 = 0x0106_b9e0;

// Translated from 00e879e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with an object: its slot 0x68 with
/// state 0x8f and value 0 when the object's `float` at +0x30 is at least the `double` at
/// `0106d508` (0.99) and at most the `float` at `0106b9e0` (1.01) (x87 `FCOMP` flag tests: an
/// unordered compare fails them) and the bytes +0x10f5 and +0x10f4 are clear; else value 1.
pub fn fn_00e879e0(e: &mut Engine, this: Ptr, object: u32) {
    let base = this.addr();
    let bits = e.mem.u32(object.wrapping_add(0x30));
    let value = x87_load32(bits);
    let low = f64::from_bits(e.mem.u64(ALPHA_LOW_0106D508));
    // `FCOMP`: C0 is set when ST0 is below the operand or the compare is unordered.
    let mut off = false;
    let below_low = value < low || value.is_nan() || low.is_nan();
    if !below_low {
        let high = x87_load32(e.mem.u32(ALPHA_HIGH_0106B9E0));
        let operand = x87_m32(e.mem.u32(object.wrapping_add(0x30)));
        let above_high = high < operand || high.is_nan() || operand.is_nan();
        if !above_high
            && e.mem.u8(base.wrapping_add(0x10f5)) == 0
            && e.mem.u8(base.wrapping_add(0x10f4)) == 0
        {
            off = true;
        }
    }
    let enable = u32::from(!off);
    e.vcall(base, SET_STATE_SLOT, &args![0x8fu32, enable, 0u32, 0u32]);
}

/// The member at +0xf8's setter `00e94070` (unnamed; `thiscall`, one word).
const MEMBER_SET_00E94070: u32 = 0x00e9_4070;

// Translated from 00e87a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall` with one word: a
/// tail jump to `00e94070` on the member at +0xf8 with the same word.
pub fn fn_00e87a40(e: &mut Engine, this: Ptr, word: u32) {
    e.call(
        MEMBER_SET_00E94070,
        &args![this.addr().wrapping_add(0xf8), word],
    );
}

// Translated from 00e87a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: clears the current
/// vertex and pixel shaders (+0x10e0, +0x10e8) and sets the kept ones (+0x10e4, +0x10ec) to 1.
pub fn fn_00e87a50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(0x10e0), 0);
    e.mem.set_u32(base.wrapping_add(0x10e4), 1);
    e.mem.set_u32(base.wrapping_add(0x10e8), 0);
    e.mem.set_u32(base.wrapping_add(0x10ec), 1);
}

/// The (state, value) pairs of the render states (`011bfce8`), the sampler states
/// (`011c04f0`) and the texture stage states (`011c0538`), each ended by state -1; the sampler
/// and stage counts (`011bf558`, `011bf560`); the stage state slots (`0126f92c`, 16-bit); and
/// the device's `SetRenderState` (slot 0xe4), `SetSamplerState` (0x114) and
/// `SetTextureStageState` (0x10c) (by the interface's order; the arguments match).
const RENDER_STATE_PAIRS_011BFCE8: u32 = 0x011b_fce8;
const SAMPLER_PAIRS_011C04F0: u32 = 0x011c_04f0;
const STAGE_PAIRS_011C0538: u32 = 0x011c_0538;
const SAMPLER_COUNT_011BF558: u32 = 0x011b_f558;
const STAGE_COUNT_011BF560: u32 = 0x011b_f560;
const STAGE_SLOTS_0126F92C: u32 = 0x0126_f92c;
const SET_RENDER_STATE: u32 = 0xe4;
const SET_SAMPLER_STATE: u32 = 0x114;
const SET_TEXTURE_STAGE_STATE: u32 = 0x10c;

// Translated from 00e88000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: for each (state,
/// value) pair at `011bfce8` (until state -1) the value goes to both words of the state's
/// cache pair (+0x120 + 8 * state, then +0x124) and to the device's `SetRenderState`.
pub fn fn_00e88000(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut pair = RENDER_STATE_PAIRS_011BFCE8;
    let mut state = e.mem.u32(pair);
    if state == 0xffff_ffff {
        return;
    }
    loop {
        let value = e.mem.u32(pair.wrapping_add(4));
        e.mem.set_u32(
            base.wrapping_add(0x124).wrapping_add(state.wrapping_mul(8)),
            value,
        );
        let state_again = e.mem.u32(pair);
        let value = e.mem.u32(pair.wrapping_add(4));
        e.mem.set_u32(
            base.wrapping_add(0x120)
                .wrapping_add(state_again.wrapping_mul(8)),
            value,
        );
        let value = e.mem.u32(pair.wrapping_add(4));
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        let state_now = e.mem.u32(pair);
        e.vcall(device, SET_RENDER_STATE, &args![state_now, value]);
        state = e.mem.u32(pair.wrapping_add(8));
        pair = pair.wrapping_add(8);
        if state == 0xffff_ffff {
            break;
        }
    }
}

// Translated from 00e88060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: sets the default
/// sampler and texture stage states. For each sampler below the count at `011bf558`, each pair
/// at `011c04f0` (until state -1): state 0xb's value becomes the sampler, states 1 and 4 get
/// value 1 for samplers after the first (written back into the table); the value is cached at
/// +0xa24 + 8 * (the state's slot at `0126f958` + 8 * sampler) and goes to the device's
/// `SetTextureStageState` (sampler, state, value). Then for each stage below the count at
/// `011bf560`, each pair at `011c0538`: the value is cached at +0xe24 + 8 * (the state's slot
/// at `0126f92c` + 5 * stage), goes to its own slot 0xcc (stage, state, value, 0) and to the
/// device's `SetSamplerState` (stage, state, value).
pub fn fn_00e88060(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut sampler = 0u32;
    while sampler < e.mem.u32(SAMPLER_COUNT_011BF558) {
        let mut pair = SAMPLER_PAIRS_011C04F0;
        let mut state = e.mem.u32(pair);
        if state != 0xffff_ffff {
            loop {
                if state == 0xb {
                    e.mem.set_u32(pair.wrapping_add(4), sampler);
                } else if (state == 1 || state == 4) && sampler != 0 {
                    e.mem.set_u32(pair.wrapping_add(4), 1);
                }
                let slot = u32::from(
                    e.mem
                        .u16(STATE_TABLE_0126F958.wrapping_add(state.wrapping_mul(2))),
                );
                let index = slot.wrapping_add(sampler.wrapping_mul(8));
                let value = e.mem.u32(pair.wrapping_add(4));
                e.mem.set_u32(
                    base.wrapping_add(0xa24).wrapping_add(index.wrapping_mul(8)),
                    value,
                );
                let value = e.mem.u32(pair.wrapping_add(4));
                let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
                e.vcall(
                    device,
                    SET_TEXTURE_STAGE_STATE,
                    &args![sampler, state, value],
                );
                state = e.mem.u32(pair.wrapping_add(8));
                pair = pair.wrapping_add(8);
                if state == 0xffff_ffff {
                    break;
                }
            }
        }
        sampler = sampler.wrapping_add(1);
    }
    let mut stage = 0u32;
    let mut offset = 0u32;
    while stage < e.mem.u32(STAGE_COUNT_011BF560) {
        let mut pair = STAGE_PAIRS_011C0538;
        let mut state = e.mem.u32(pair);
        if state != 0xffff_ffff {
            loop {
                let slot = u32::from(
                    e.mem
                        .u16(STAGE_SLOTS_0126F92C.wrapping_add(state.wrapping_mul(2))),
                );
                let value = e.mem.u32(pair.wrapping_add(4));
                e.mem.set_u32(
                    base.wrapping_add(0xe24)
                        .wrapping_add(slot.wrapping_add(offset).wrapping_mul(8)),
                    value,
                );
                let value = e.mem.u32(pair.wrapping_add(4));
                e.vcall(base, 0xcc, &args![stage, state, value, 0u32]);
                let value = e.mem.u32(pair.wrapping_add(4));
                let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
                e.vcall(device, SET_SAMPLER_STATE, &args![stage, state, value]);
                state = e.mem.u32(pair.wrapping_add(8));
                pair = pair.wrapping_add(8);
                if state == 0xffff_ffff {
                    break;
                }
            }
        }
        offset = offset.wrapping_add(5);
        stage = stage.wrapping_add(1);
    }
}

// Translated from 00e88690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with a 16-bit index: its slot 0x68
/// with (0x97, the word at +0xc + 4 * index, 0, 0).
pub fn fn_00e88690(e: &mut Engine, this: Ptr, index: u16) {
    let base = this.addr();
    let value = e
        .mem
        .u32(base.wrapping_add(0xc).wrapping_add(4 * u32::from(index)));
    e.vcall(base, SET_STATE_SLOT, &args![0x97u32, value, 0u32, 0u32]);
}

// Translated from 00e88730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with two `float`s: stores them at
/// +0x78 and +0x7c (through the x87 stack, `FST`) and their difference, second minus first
/// (`FSUBRP`), at +0x80.
pub fn fn_00e88730(e: &mut Engine, this: Ptr, first: f32, second: f32) {
    let base = this.addr();
    let first = x87_load32(first.to_bits());
    e.mem.set_u32(base.wrapping_add(0x78), x87_store32(first));
    let second = x87_load32(second.to_bits());
    e.mem.set_u32(base.wrapping_add(0x7c), x87_store32(second));
    let difference = x87_sub(second, first);
    e.mem
        .set_u32(base.wrapping_add(0x80), x87_store32(difference));
}

// Translated from 00e88750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: whether the word at
/// +0xf4 is nonzero.
pub fn fn_00e88750(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0xf4)) != 0
}

// Translated from 00e88780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with four words, the third not read:
/// the state, the value and a byte. When the state's lock byte (+0x920 + state) is clear it
/// gets the byte, and a value different from the cached one (+0x120 + 8 * state) goes to the
/// device's `SetRenderState` and is cached.
pub fn fn_00e88780(e: &mut Engine, this: Ptr, state: u32, value: u32, _unused_2: u32, lock: u8) {
    let base = this.addr();
    let locked = base.wrapping_add(0x920).wrapping_add(state);
    if e.mem.u8(locked) != 0 {
        return;
    }
    e.mem.set_u8(locked, lock);
    let cached = base.wrapping_add(0x120).wrapping_add(state.wrapping_mul(8));
    if e.mem.u32(cached) == value {
        return;
    }
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_RENDER_STATE, &args![state, value]);
    e.mem.set_u32(cached, value);
}

/// The device's `SetPixelShader` (slot 0x1ac, by the interface's order).
const SET_PIXEL_SHADER: u32 = 0x1ac;

// Translated from 00e88870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall` with the pixel shader
/// and a byte: with the byte set the current one (+0x10e8) is kept at +0x10ec; a different
/// shader becomes current and goes to the device's `SetPixelShader` (a tail jump).
pub fn fn_00e88870(e: &mut Engine, this: Ptr, shader: u32, keep: u8) {
    let base = this.addr();
    if keep != 0 {
        let current = e.mem.u32(base.wrapping_add(0x10e8));
        e.mem.set_u32(base.wrapping_add(0x10ec), current);
    }
    if e.mem.u32(base.wrapping_add(0x10e8)) != shader {
        e.mem.set_u32(base.wrapping_add(0x10e8), shader);
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.vcall(device, SET_PIXEL_SHADER, &args![shader]);
    }
}

// Translated from 00e888b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: the current pixel
/// shader (+0x10e8).
pub fn fn_00e888b0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x10e8))
}

// Translated from 00e888e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with a pixel shader that goes away:
/// when it is current (+0x10e8) that is cleared and the device's `SetPixelShader` gets null;
/// when it is the kept one (+0x10ec) that is cleared.
pub fn fn_00e888e0(e: &mut Engine, this: Ptr, shader: u32) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x10e8)) == shader {
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.mem.set_u32(base.wrapping_add(0x10e8), 0);
        e.vcall(device, SET_PIXEL_SHADER, &args![0u32]);
    }
    if e.mem.u32(base.wrapping_add(0x10ec)) == shader {
        e.mem.set_u32(base.wrapping_add(0x10ec), 0);
    }
}

// Translated from 00e88930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with four words, the last not read:
/// the stage, the state and the value. A state whose slot (`0126f958`) is below 8 is cached at
/// +0xa20 (8 bytes per slot, 8 slots per stage): a value different from the cached one goes
/// to the device's `SetTextureStageState`; the value is cached either way.
pub fn fn_00e88930(e: &mut Engine, this: Ptr, stage: u32, state: u32, value: u32, _unused_3: u32) {
    let slot = u32::from(
        e.mem
            .u16(STATE_TABLE_0126F958.wrapping_add(state.wrapping_mul(2))),
    );
    if slot >= 8 {
        return;
    }
    let base = this.addr();
    let cached = base
        .wrapping_add(0xa20)
        .wrapping_add(slot.wrapping_add(stage.wrapping_mul(8)).wrapping_mul(8));
    if e.mem.u32(cached) != value {
        let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
        e.vcall(device, SET_TEXTURE_STAGE_STATE, &args![stage, state, value]);
    }
    e.mem.set_u32(cached, value);
}

// Translated from 00e889b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall`: clears the sixteen
/// texture words +0x10a0 to +0x10dc.
pub fn fn_00e889b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for stage in 0..0x10u32 {
        e.mem.set_u32(base.wrapping_add(0x10a0 + 4 * stage), 0);
    }
}

/// The device's `SetTexture` (slot 0x104, by the interface's order).
const SET_TEXTURE: u32 = 0x104;

// Translated from 00e88a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with a stage and a texture: a texture
/// different from the stage's word (+0x10a0 + 4 * stage) is stored and goes to the device's
/// `SetTexture`.
pub fn fn_00e88a20(e: &mut Engine, this: Ptr, stage: u32, texture: u32) {
    let base = this.addr();
    let slot = base
        .wrapping_add(0x10a0)
        .wrapping_add(stage.wrapping_mul(4));
    if e.mem.u32(slot) == texture {
        return;
    }
    e.mem.set_u32(slot, texture);
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.vcall(device, SET_TEXTURE, &args![stage, texture]);
}

// Translated from 00e88a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`), `thiscall` with a texture that goes away: when
/// it is not null, each of the sixteen stages holding it (+0x10a0) is cleared and the device's
/// `SetTexture` gets null for it.
pub fn fn_00e88a60(e: &mut Engine, this: Ptr, texture: u32) {
    if texture == 0 {
        return;
    }
    let base = this.addr();
    for stage in 0..0x10u32 {
        let slot = base.wrapping_add(0x10a0 + 4 * stage);
        if e.mem.u32(slot) == texture {
            e.mem.set_u32(slot, 0);
            let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
            e.vcall(device, SET_TEXTURE, &args![stage, 0u32]);
        }
    }
}

// Translated from 00e88ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`; a virtual slot), `thiscall` with one word:
/// stores its low byte at +0x10f5.
pub fn fn_00e88ad0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x10f5), value);
}

/// The member at +0xf8's destructor (`00e940a0`, unnamed; `thiscall`).
const MEMBER_DOWN_00E940A0: u32 = 0x00e9_40a0;

/// An `NiPointer` let go: the object's count (+4) down through `InterlockedDecrement` and, at
/// zero, its virtual slot 4 (delete).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DOWN, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00e88b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e87980`): the `NiD3DRenderState` destructor (the RTTI of
/// the table it stores), `thiscall`. Its table; the object at +0x74 let go (count down,
/// deleted at zero) and cleared; the renderer (+0x10fc) cleared; the device (+0x10f8) released
/// (slot 8) when set and cleared; the object at +0x10f0 let go (not cleared); the member at
/// +0xf8 destroyed (`00e940a0`); the (now null) +0x74 let go again when set; then the
/// `NiRefObject` table and the global object count down.
pub fn fn_00e88b00(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, RENDER_STATE_BASE_VTABLE);
    let held = e.mem.u32(base.wrapping_add(0x74));
    if held != 0 {
        release_reference(e, held);
        e.mem.set_u32(base.wrapping_add(0x74), 0);
    }
    let device = e.mem.u32(base.wrapping_add(STATE_DEVICE));
    e.mem.set_u32(base.wrapping_add(0x10fc), 0);
    if device != 0 {
        e.vcall(device, 8, &args![]);
    }
    e.mem.set_u32(base.wrapping_add(STATE_DEVICE), 0);
    let object = e.mem.u32(base.wrapping_add(0x10f0));
    release_reference(e, object);
    e.call(MEMBER_DOWN_00E940A0, &args![base.wrapping_add(0xf8)]);
    let held = e.mem.u32(base.wrapping_add(0x74));
    release_reference(e, held);
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE_0101DCE4);
    e.call(INTERLOCKED_DOWN, &args![OBJECT_COUNT_011F4400]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_8c20, fn_00e88c20(Ptr, u8)),
        entry!(0x00e8_81a0, fn_00e881a0(Ptr, u32) -> Ptr),
        entry!(0x00e8_7af0, fn_00e87af0(Ptr, u32)),
        entry!(0x00e8_79b0, fn_00e879b0(Ptr, u32, u32)),
        entry!(0x00e8_79e0, fn_00e879e0(Ptr, u32)),
        entry!(0x00e8_7a40, fn_00e87a40(Ptr, u32)),
        entry!(0x00e8_7a50, fn_00e87a50(Ptr)),
        entry!(0x00e8_8000, fn_00e88000(Ptr)),
        entry!(0x00e8_8060, fn_00e88060(Ptr)),
        entry!(0x00e8_8690, fn_00e88690(Ptr, u16)),
        entry!(0x00e8_8730, fn_00e88730(Ptr, f32, f32)),
        entry!(0x00e8_8750, fn_00e88750(Ptr) -> bool),
        entry!(0x00e8_8780, fn_00e88780(Ptr, u32, u32, u32, u8)),
        entry!(0x00e8_8870, fn_00e88870(Ptr, u32, u8)),
        entry!(0x00e8_88b0, fn_00e888b0(Ptr) -> u32),
        entry!(0x00e8_88e0, fn_00e888e0(Ptr, u32)),
        entry!(0x00e8_8930, fn_00e88930(Ptr, u32, u32, u32, u32)),
        entry!(0x00e8_89b0, fn_00e889b0(Ptr)),
        entry!(0x00e8_8a20, fn_00e88a20(Ptr, u32, u32)),
        entry!(0x00e8_8a60, fn_00e88a60(Ptr, u32)),
        entry!(0x00e8_8ad0, fn_00e88ad0(Ptr, u8)),
        entry!(0x00e8_8b00, fn_00e88b00(Ptr)),
    ]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e88c20_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0xffff_ffff);
        e.call(0x00e8_8c20, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0xffff_ff01);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e881a0_sets_the_render_state_defaults() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(SCALE_0101E568, 8);
        e.set_global(SCALE_0101E568, 255.0f64);
        for (i, value) in [1.0f32, 0.5, 0.0].iter().enumerate() {
            e.mem
                .set_u32(DEFAULT_COLOR_011F4998 + 4 * i as u32, value.to_bits());
        }
        for no_op in [
            INTERLOCKED_UP,
            MEMBER_SET_UP_00E68810,
            HOOK_UP_00E87AF0,
            OBJECT_CONSTRUCT_00A5C200,
            MEMSET_00EC61C0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(NEW_OBJECT_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x1248);
        assert_eq!(e.call(0x00e8_81a0, &args![this, 0x4000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), RENDER_STATE_BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x120), 0x7fff_ffff);
        assert_eq!(e.mem.u32(this + 0x120 + 8 * 0xff), 0x7fff_ffff);
        assert_eq!(e.mem.u32(this + 0xe20 + 8 * 0x4f), 0x7fff_ffff);
        assert_eq!(e.mem.u32(this + 0xe20 + 8 * 0x50), 0);
        let object = e.mem.u32(this + 0x74);
        assert_eq!(e.mem.u32(object), MEMBER_74_VTABLE_010162DC);
        assert_eq!(e.mem.u16(object + 0x18), 0xec);
        // 1.0 -> 255, 0.5 -> 127 (127.5 chopped), 0.0 -> 0.
        assert_eq!(e.mem.u32(this + 0x98), 0xffff_7f00);
        assert_eq!(e.mem.f32(this + 0x88), 1.0);
        assert_eq!(e.mem.u32(this + 0x30), 9);
        assert_eq!(e.mem.u32(this + 0x11c), 1);
        // The overlapping stores leave these words.
        assert_eq!(e.mem.u32(0x0126_f92c), 0x0000_ffff);
        assert_eq!(e.mem.u32(0x0126_f930), 0xffff_0001);
        assert_eq!(e.mem.u32(0x0126_f934), 0x0002_ffff);
        assert_eq!(e.mem.u32(0x0126_f938), 0x0004_0003);
        assert_eq!(e.mem.u32(0x0126_f928), 7);
        assert_eq!(e.mem.u16(0x0126_f988), 7);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .contains(&(MEMSET_00EC61C0, vec![STATE_TABLE_0126F958, 0xffff, 0x42])));
    }
}

#[cfg(test)]
mod b000x_tests {
    use super::*;

    /// An object whose vtable slots 4 and 8 are doubles at 0x0e10_0004 and 0x0e10_0008.
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0e10_0004);
        e.mem.set_u32(vtable + 8, 0x0e10_0008);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    fn calls(e: &mut Engine) -> Vec<(u32, u32)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0e10_0004 || *a == 0x0e10_0008)
            .map(|(a, words)| (a, words[0]))
            .collect()
    }

    #[test]
    fn fn_00e87af0_swaps_the_renderer_object() {
        let mut e = Engine::new();
        e.register(0x0e10_0004, |_, _| Ret::default());
        e.register(0x0e10_0008, |_, _| Ret::default());
        let this = e.mem.alloc(0x1100);
        let old = object(&mut e);
        let new = object(&mut e);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + 0x288, new);
        e.mem.set_u32(this + 0x10f8, old);
        e.call_log = Some(vec![]);
        e.call(HOOK_UP_00E87AF0, &args![this, renderer]);
        assert_eq!(e.mem.u32(this + 0x10fc), renderer);
        assert_eq!(e.mem.u32(this + 0x10f8), new);
        assert_eq!(calls(&mut e), vec![(0x0e10_0008, old), (0x0e10_0004, new)]);
        // A renderer without an object: nothing held, nothing added.
        let bare = e.mem.alloc(0x290);
        e.call_log = Some(vec![]);
        e.call(HOOK_UP_00E87AF0, &args![this, bare]);
        assert_eq!(e.mem.u32(this + 0x10f8), 0);
        assert_eq!(calls(&mut e), vec![(0x0e10_0008, new)]);
        // No renderer: released (none held now) and cleared.
        e.mem.set_u32(this + 0x10f8, old);
        e.call_log = Some(vec![]);
        e.call(HOOK_UP_00E87AF0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x10fc), 0);
        assert_eq!(e.mem.u32(this + 0x10f8), 0);
        assert_eq!(calls(&mut e), vec![(0x0e10_0008, old)]);
        e.call_log = Some(vec![]);
        e.call(HOOK_UP_00E87AF0, &args![this, 0u32]);
        assert!(calls(&mut e).is_empty());
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const STATE_TABLE: u32 = 0x8000;
    const DEVICE_TABLE: u32 = 0x8400;

    /// A render state whose own table sends every slot to `0x7000 + slot` and whose device's
    /// table to `0x7400 + slot`; all of them answer 0.
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011b_f000, 0x2000);
        e.map(0x0106_b000, 0x3000);
        let own: Vec<u32> = (0..0x80).map(|i| 0x7000 + 4 * i).collect();
        let device: Vec<u32> = (0..0x80).map(|i| 0x7400 + 4 * i).collect();
        e.put_vtable(STATE_TABLE, &own);
        e.put_vtable(DEVICE_TABLE, &device);
        for slot in (0..0x200u32).step_by(4) {
            e.register(0x7000 + slot, |_, _| Ret::default());
            e.register(0x7400 + slot, |_, _| Ret::default());
        }
        for no_op in [MEMBER_SET_00E94070, MEMBER_DOWN_00E940A0, INTERLOCKED_DOWN] {
            e.register(no_op, |_, _| Ret::default());
        }
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, DEVICE_TABLE);
        let state = e.mem.alloc(0x1248);
        e.mem.set_u32(state, STATE_TABLE);
        e.mem.set_u32(state + STATE_DEVICE, device);
        e.call_log = Some(vec![]);
        (e, state, device)
    }

    #[test]
    fn fn_00e879b0_sets_state_0x16_from_its_table() {
        let (mut e, state, _) = engine();
        e.mem.set_u32(state + 0xf4, 1);
        e.mem.set_u32(state + 0xd4 + 4 * 5, 0x55);
        e.call(0x00e8_79b0, &args![state, 2u32, 0x99u32]);
        assert_eq!(calls(&e, 0x7068), vec![vec![state, 0x16, 0x55, 0, 0x99]]);
    }

    #[test]
    fn fn_00e879e0_switches_state_0x8f_by_the_alpha() {
        let (mut e, state, _) = engine();
        e.mem
            .write(ALPHA_LOW_0106D508, &f64::from(0.99f32).to_le_bytes());
        e.mem.set_u32(ALPHA_HIGH_0106B9E0, 1.01f32.to_bits());
        let object = e.mem.alloc(0x34);
        let mut values = vec![];
        for alpha in [1.0f32, 0.5, 1.5, f32::NAN, 0.99, 1.01] {
            e.mem.set_u32(object + 0x30, alpha.to_bits());
            e.call(0x00e8_79e0, &args![state, object]);
            values.push(calls(&e, 0x7068).last().unwrap()[2]);
        }
        assert_eq!(values, vec![0, 1, 1, 1, 0, 0]);
        e.mem.set_u32(object + 0x30, 1.0f32.to_bits());
        e.mem.set_u8(state + 0x10f4, 1);
        e.call(0x00e8_79e0, &args![state, object]);
        assert_eq!(
            calls(&e, 0x7068).last().unwrap(),
            &vec![state, 0x8f, 1, 0, 0]
        );
    }

    #[test]
    fn shader_words_and_member_calls() {
        let (mut e, state, device) = engine();
        e.call(0x00e8_7a40, &args![state, 7u32]);
        assert_eq!(calls(&e, MEMBER_SET_00E94070), vec![vec![state + 0xf8, 7]]);
        e.mem.set_u32(state + 0x10e0, 3);
        e.call(0x00e8_7a50, &args![state]);
        assert_eq!(e.mem.u32(state + 0x10e0), 0);
        assert_eq!(e.mem.u32(state + 0x10e4), 1);
        assert_eq!(e.mem.u32(state + 0x10ec), 1);
        e.call(0x00e8_8870, &args![state, 0x11u32, 0u32]);
        e.call(0x00e8_8870, &args![state, 0x11u32, 0u32]);
        assert_eq!(calls(&e, 0x75ac), vec![vec![device, 0x11]]);
        e.call(0x00e8_8870, &args![state, 0x22u32, 1u32]);
        assert_eq!(e.mem.u32(state + 0x10ec), 0x11);
        assert_eq!(e.call(0x00e8_88b0, &args![state]).u32(), 0x22);
        e.call(0x00e8_88e0, &args![state, 0x11u32]);
        assert_eq!(e.mem.u32(state + 0x10ec), 0);
        assert_eq!(e.mem.u32(state + 0x10e8), 0x22);
        e.call(0x00e8_88e0, &args![state, 0x22u32]);
        assert_eq!(e.mem.u32(state + 0x10e8), 0);
        assert_eq!(calls(&e, 0x75ac).last().unwrap(), &vec![device, 0]);
    }

    #[test]
    fn fn_00e88000_caches_and_sets_every_render_state() {
        let (mut e, state, device) = engine();
        for (i, word) in [7u32, 1, 9, 2, 0xffff_ffff].iter().enumerate() {
            e.mem
                .set_u32(RENDER_STATE_PAIRS_011BFCE8 + 4 * i as u32, *word);
        }
        e.call(0x00e8_8000, &args![state]);
        assert_eq!(
            calls(&e, 0x74e4),
            vec![vec![device, 7, 1], vec![device, 9, 2]]
        );
        assert_eq!(e.mem.u32(state + 0x120 + 8 * 7), 1);
        assert_eq!(e.mem.u32(state + 0x124 + 8 * 9), 2);
        e.mem.set_u32(RENDER_STATE_PAIRS_011BFCE8, 0xffff_ffff);
        e.call(0x00e8_8000, &args![state]);
        assert_eq!(calls(&e, 0x74e4).len(), 2);
    }

    #[test]
    fn fn_00e88060_sets_the_sampler_and_stage_defaults() {
        let (mut e, state, device) = engine();
        e.mem.set_u32(SAMPLER_COUNT_011BF558, 2);
        e.mem.set_u32(STAGE_COUNT_011BF560, 2);
        for (i, word) in [0xbu32, 5, 1, 0, 3, 6, 0xffff_ffff].iter().enumerate() {
            e.mem.set_u32(SAMPLER_PAIRS_011C04F0 + 4 * i as u32, *word);
        }
        for (i, word) in [2u32, 4, 0xffff_ffff].iter().enumerate() {
            e.mem.set_u32(STAGE_PAIRS_011C0538 + 4 * i as u32, *word);
        }
        e.mem.set_u16(STATE_TABLE_0126F958 + 2 * 0xb, 1);
        e.mem.set_u16(STATE_TABLE_0126F958 + 2, 2);
        e.mem.set_u16(STATE_TABLE_0126F958 + 2 * 3, 3);
        e.mem.set_u16(STAGE_SLOTS_0126F92C + 2 * 2, 4);
        e.call(0x00e8_8060, &args![state]);
        assert_eq!(
            calls(&e, 0x750c),
            vec![
                vec![device, 0, 0xb, 0],
                vec![device, 0, 1, 0],
                vec![device, 0, 3, 6],
                vec![device, 1, 0xb, 1],
                vec![device, 1, 1, 1],
                vec![device, 1, 3, 6]
            ]
        );
        assert_eq!(e.mem.u32(SAMPLER_PAIRS_011C04F0 + 4), 1);
        assert_eq!(e.mem.u32(state + 0xa24 + 8 * (1 + 8)), 1);
        assert_eq!(e.mem.u32(state + 0xa24 + 8 * 3), 6);
        assert_eq!(
            calls(&e, 0x70cc),
            vec![vec![state, 0, 2, 4, 0], vec![state, 1, 2, 4, 0]]
        );
        assert_eq!(
            calls(&e, 0x7514),
            vec![vec![device, 0, 2, 4], vec![device, 1, 2, 4]]
        );
        assert_eq!(e.mem.u32(state + 0xe24 + 8 * (4 + 5)), 4);
    }

    #[test]
    fn small_setters() {
        let (mut e, state, _) = engine();
        e.mem.set_u32(state + 0xc + 4 * 3, 0x33);
        e.call(0x00e8_8690, &args![state, 0x1_0003u32]);
        assert_eq!(calls(&e, 0x7068), vec![vec![state, 0x97, 0x33, 0, 0]]);
        e.call(0x00e8_8730, &args![state, 0.25f32, 1.0f32]);
        assert_eq!(e.mem.u32(state + 0x78), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(state + 0x7c), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(state + 0x80), 0.75f32.to_bits());
        assert!(!e.call(0x00e8_8750, &args![state]).bool());
        e.mem.set_u32(state + 0xf4, 2);
        assert!(e.call(0x00e8_8750, &args![state]).bool());
        e.call(0x00e8_8ad0, &args![state, 0x101u32]);
        assert_eq!(e.mem.u8(state + 0x10f5), 1);
        for stage in 0..0x10 {
            e.mem.set_u32(state + 0x10a0 + 4 * stage, 9);
        }
        e.call(0x00e8_89b0, &args![state]);
        assert_eq!(e.mem.bytes(state + 0x10a0, 0x40), vec![0; 0x40]);
    }

    #[test]
    fn fn_00e88780_sets_unlocked_render_states() {
        let (mut e, state, device) = engine();
        e.call(0x00e8_8780, &args![state, 7u32, 3u32, 0u32, 1u32]);
        assert_eq!(calls(&e, 0x74e4), vec![vec![device, 7, 3]]);
        assert_eq!(e.mem.u32(state + 0x120 + 8 * 7), 3);
        assert_eq!(e.mem.u8(state + 0x920 + 7), 1);
        // Locked now: nothing more.
        e.call(0x00e8_8780, &args![state, 7u32, 4u32, 0u32, 0u32]);
        assert_eq!(calls(&e, 0x74e4).len(), 1);
        // Unlocked with the cached value: only the lock byte.
        e.call(0x00e8_8780, &args![state, 8u32, 0u32, 0u32, 1u32]);
        assert_eq!(calls(&e, 0x74e4).len(), 1);
        assert_eq!(e.mem.u8(state + 0x920 + 8), 1);
    }

    #[test]
    fn fn_00e88930_caches_texture_stage_states() {
        let (mut e, state, device) = engine();
        e.mem.set_u16(STATE_TABLE_0126F958 + 2 * 4, 2);
        e.mem.set_u16(STATE_TABLE_0126F958 + 2 * 5, 8);
        e.call(0x00e8_8930, &args![state, 1u32, 4u32, 6u32, 0u32]);
        e.call(0x00e8_8930, &args![state, 1u32, 4u32, 6u32, 0u32]);
        e.call(0x00e8_8930, &args![state, 1u32, 5u32, 6u32, 0u32]);
        assert_eq!(calls(&e, 0x750c), vec![vec![device, 1, 4, 6]]);
        assert_eq!(e.mem.u32(state + 0xa20 + 8 * (2 + 8)), 6);
    }

    #[test]
    fn textures_follow_the_device() {
        let (mut e, state, device) = engine();
        e.call(0x00e8_8a20, &args![state, 2u32, 0x44u32]);
        e.call(0x00e8_8a20, &args![state, 2u32, 0x44u32]);
        e.call(0x00e8_8a20, &args![state, 5u32, 0x44u32]);
        assert_eq!(
            calls(&e, 0x7504),
            vec![vec![device, 2, 0x44], vec![device, 5, 0x44]]
        );
        e.call(0x00e8_8a60, &args![state, 0u32]);
        assert_eq!(calls(&e, 0x7504).len(), 2);
        e.call(0x00e8_8a60, &args![state, 0x44u32]);
        assert_eq!(
            calls(&e, 0x7504)[2..],
            [vec![device, 2, 0], vec![device, 5, 0]]
        );
        assert_eq!(e.mem.u32(state + 0x10a0 + 8), 0);
    }

    #[test]
    fn fn_00e88b00_takes_the_render_state_down() {
        let (mut e, state, device) = engine();
        e.register(INTERLOCKED_DOWN, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, STATE_TABLE);
        e.mem.set_u32(held + 4, 1);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, STATE_TABLE);
        e.mem.set_u32(object + 4, 2);
        e.mem.set_u32(state + 0x74, held);
        e.mem.set_u32(state + 0x10f0, object);
        e.mem.set_u32(state + 0x10fc, 0x99);
        e.call(0x00e8_8b00, &args![state]);
        assert_eq!(calls(&e, 0x7004), vec![vec![held]]);
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(e.mem.u32(state + 0x74), 0);
        assert_eq!(e.mem.u32(state + 0x10f0), object);
        assert_eq!(e.mem.u32(state + 0x10fc), 0);
        assert_eq!(e.mem.u32(state + STATE_DEVICE), 0);
        assert_eq!(calls(&e, 0x7408), vec![vec![device]]);
        assert_eq!(calls(&e, MEMBER_DOWN_00E940A0), vec![vec![state + 0xf8]]);
        assert_eq!(e.mem.u32(state), NI_REF_OBJECT_VTABLE_0101DCE4);
        assert_eq!(
            calls(&e, INTERLOCKED_DOWN).last().unwrap(),
            &vec![OBJECT_COUNT_011F4400]
        );
    }
}
