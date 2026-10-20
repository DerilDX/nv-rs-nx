//! `BSShader/run_00b6adb0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRenderedCubeMap::ms_RTTI` (`012708e4`: its static initializer `00fc7950` gives it the
/// name `"NiRenderedCubeMap"`), the renderer (`011f9508`) and the render target maker
/// `00a9ff00` (unnamed; `cdecl`: a buffer, the renderer).
const RENDERED_CUBE_MAP_RTTI_012708E4: u32 = 0x0127_08e4;
const RENDERER_011F9508: u32 = 0x011f_9508;
const RENDER_TARGET_00A9FF00: u32 = 0x00a9_ff00;

/// The word at +0x10 of the object at +0x10 of `holder`, or 0 when there is none.
fn inner_word(e: &Engine, holder: u32) -> u32 {
    let inner = e.mem.u32(holder.wrapping_add(0x10));
    if inner == 0 {
        0
    } else {
        e.mem.u32(inner + 0x10)
    }
}

// Translated from 00b6b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): a texture's render target, `cdecl` with two
/// words: the texture and a target that may be reused. For an `NiRenderedCubeMap` texture
/// (`GetRTTI`, vtable slot +8, then each base through the RTTI's +4): the target is kept when
/// it is set and at least as wide (+8) and high (+0xc) as the cube map's buffer (texture
/// +0x4c), else `00a9ff00(buffer, renderer)` is answered. Other textures (also a null one, whose
/// vtable is then read): the target is kept when it is set, at least as wide as the texture's
/// width (slot +0x94) and high as its height (slot +0x98), and its +0x10's +0x10 matches that of
/// the texture's buffer (slot +0xa4); else `00a9ff00` of the texture's buffer (the renderer read
/// first) is answered.
pub fn fn_00b6b180(e: &mut Engine, texture: Ptr, target: Ptr) -> u32 {
    let texture = texture.addr();
    let target = target.addr();
    if texture != 0 {
        let mut rtti = e.vcall(texture, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == RENDERED_CUBE_MAP_RTTI_012708E4 {
                let buffer = e.mem.u32(texture + 0x4c);
                if target != 0
                    && e.mem.u32(target + 8) >= e.mem.u32(buffer.wrapping_add(8))
                    && e.mem.u32(target + 0xc) >= e.mem.u32(buffer.wrapping_add(0xc))
                {
                    return target;
                }
                let renderer = e.global::<u32>(RENDERER_011F9508);
                return e
                    .call(RENDER_TARGET_00A9FF00, &args![buffer, renderer])
                    .u32();
            }
            rtti = e.mem.u32(rtti + 4);
        }
    }
    if target != 0 {
        let width = e.mem.u32(target + 8);
        if width >= e.vcall(texture, 0x94, &args![]).u32() {
            let height = e.mem.u32(target + 0xc);
            if height >= e.vcall(texture, 0x98, &args![]).u32() {
                let kept = inner_word(e, target);
                let buffer = e.vcall(texture, 0xa4, &args![]).u32();
                if kept == inner_word(e, buffer) {
                    return target;
                }
            }
        }
    }
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let buffer = e.vcall(texture, 0xa4, &args![]).u32();
    e.call(RENDER_TARGET_00A9FF00, &args![buffer, renderer])
        .u32()
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00b6aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b6adb0`): an `NiPointer` constructor, `thiscall` with the
/// object: stores it at `this` and, when not null, counts its reference count (+4) up through
/// `InterlockedIncrement`. Answers `this`.
pub fn fn_00b6aed0(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_b180, fn_00b6b180(Ptr, Ptr) -> u32),
        entry!(0x00b6_aed0, fn_00b6aed0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTURE_VTABLE: u32 = 0x0e00_0000;
    const GET_RTTI: u32 = 0x0e01_0008;
    const WIDTH: u32 = 0x0e01_0094;
    const HEIGHT: u32 = 0x0e01_0098;
    const BUFFER_OF: u32 = 0x0e01_00a4;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A texture whose RTTI chain ends in `base` (one RTTI in front), 64 x 32, its buffer's
    /// +0x10 object holding 5 at +0x10; render targets answered as 0x0e20_0000.
    fn engine(base: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(RENDERER_011F9508, 0x0e10_0000u32);
        let mut slots = vec![0u32; 0x2a];
        slots[2] = GET_RTTI;
        slots[0x25] = WIDTH;
        slots[0x26] = HEIGHT;
        slots[0x29] = BUFFER_OF;
        e.put_vtable(TEXTURE_VTABLE, &slots);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, base);
        e.register_double(GET_RTTI, move |_, _| rtti.into_ret());
        e.register(WIDTH, |_, _| 64u32.into_ret());
        e.register(HEIGHT, |_, _| 32u32.into_ret());
        let buffer = e.mem.alloc(0x14);
        let inner = e.mem.alloc(0x14);
        e.mem.set_u32(inner + 0x10, 5);
        e.mem.set_u32(buffer + 0x10, inner);
        e.mem.set_u32(buffer + 8, 16);
        e.mem.set_u32(buffer + 0xc, 16);
        e.register_double(BUFFER_OF, move |_, _| buffer.into_ret());
        e.register(RENDER_TARGET_00A9FF00, |_, _| 0x0e20_0000u32.into_ret());
        let texture = e.mem.alloc(0x50);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        e.mem.set_u32(texture + 0x4c, buffer);
        e.call_log = Some(vec![]);
        (e, texture, buffer)
    }

    /// A target `width` x `height` whose +0x10 object holds `word` at +0x10.
    fn target(e: &mut Engine, width: u32, height: u32, word: u32) -> u32 {
        let target = e.mem.alloc(0x14);
        let inner = e.mem.alloc(0x14);
        e.mem.set_u32(inner + 0x10, word);
        e.mem.set_u32(target + 0x10, inner);
        e.mem.set_u32(target + 8, width);
        e.mem.set_u32(target + 0xc, height);
        target
    }

    #[test]
    fn fn_00b6b180_reuses_a_large_enough_cube_target() {
        let (mut e, texture, buffer) = engine(RENDERED_CUBE_MAP_RTTI_012708E4);
        let kept = target(&mut e, 16, 16, 0);
        assert_eq!(e.call(0x00b6_b180, &args![texture, kept]).u32(), kept);
        let small = target(&mut e, 16, 8, 0);
        assert_eq!(
            e.call(0x00b6_b180, &args![texture, small]).u32(),
            0x0e20_0000
        );
        assert_eq!(
            e.call(0x00b6_b180, &args![texture, 0u32]).u32(),
            0x0e20_0000
        );
        assert_eq!(
            calls(&e, RENDER_TARGET_00A9FF00),
            vec![vec![buffer, 0x0e10_0000], vec![buffer, 0x0e10_0000]]
        );
        assert!(calls(&e, WIDTH).is_empty());
    }

    #[test]
    fn fn_00b6b180_checks_size_and_format_of_other_textures() {
        let (mut e, texture, buffer) = engine(0);
        let kept = target(&mut e, 64, 32, 5);
        assert_eq!(e.call(0x00b6_b180, &args![texture, kept]).u32(), kept);
        for (width, height, word) in [(63, 32, 5), (64, 31, 5), (64, 32, 6)] {
            let other = target(&mut e, width, height, word);
            assert_eq!(
                e.call(0x00b6_b180, &args![texture, other]).u32(),
                0x0e20_0000
            );
        }
        assert_eq!(calls(&e, RENDER_TARGET_00A9FF00).len(), 3);
        assert_eq!(
            calls(&e, RENDER_TARGET_00A9FF00)[0],
            vec![buffer, 0x0e10_0000]
        );
        // Width fails first: no height asked; the format check asks for the buffer twice.
        assert_eq!(calls(&e, HEIGHT).len(), 3);
        assert_eq!(calls(&e, BUFFER_OF).len(), 5);
    }
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00b6aed0_holds_and_counts_the_object() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let slot = e.mem.alloc(4);
        assert_eq!(e.call(0x00b6_aed0, &args![slot, 0x500u32]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x500);
        e.call(0x00b6_aed0, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        let counted: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == IMPORT_INTERLOCKED_INCREMENT)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(counted, vec![vec![0x504]]);
    }
}
