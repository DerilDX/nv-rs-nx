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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_8c20, fn_00e88c20(Ptr, u8)),
        entry!(0x00e8_81a0, fn_00e881a0(Ptr, u32) -> Ptr),
        entry!(0x00e8_7af0, fn_00e87af0(Ptr, u32)),
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
