//! `(unplaced)/run_00b79de0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
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
/// The shader's constructor `00b79b00` (unnamed, `thiscall`, `RET 0x10`: four declarations).
const SHADER_CONSTRUCT_00B79B00: u32 = 0x00b7_9b00;

/// The entries (vtable slot +0x8c, five words each) of the first and fourth declarations
/// (six) and of the second (eight).
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

// Translated from 00b7a380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b79de0`): the creator `GetShader` (`00b55560`) calls for
/// shader 1. No arguments. Makes four declarations (`NiXenonShaderDeclaration::Create` with
/// the renderer `011f9508` read each time, 1 stream each: 6, 8, 3 and 8 entries), then
/// through vtable slot +0x8c (five words each): the first gets (0, 0, 0, 2, 0), (1, 3, 3, 2,
/// 0), (2, 5, 4, 3, 0), (3, 7, 5, 1, 0), (4, 0xe, 0xd, 2, 0), (5, 0xf, 0xe, 2, 0) and ends
/// (slot +0xb4); the second (0, 0, 0, 2, 0), (1, 1, 1, 3, 0), (2, 2, 2, 4, 0), (3, 3, 3, 2,
/// 0), (4, 5, 4, 3, 0), (5, 7, 5, 1, 0), (6, 0xe, 0xd, 2, 0), (7, 0xf, 0xe, 2, 0) and ends; the
/// third (0, 0, 0, 2, 0), (1, 3, 5, 1, 0), slot +0x94 with (0, 2, 5, 0x80000000, 0, 5, 1, 0)
/// and ends; the fourth the first's six entries, slot +0x94 with (0, 6, 8, 0x80000000, 3, 5,
/// 1, 0) and with (0, 7, 9, 0x80000001, 3, 5, 2, 0) and ends. Allocates 0x8c bytes and runs
/// the constructor `00b79b00` with the four (a null allocation is used as it is), calls the
/// shader's slots +0x4c, +0x144, +0x150, +0x11c, then +0xc0 with the first declaration, and
/// answers the shader.
// Translation incomplete: 00b7a380 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
pub fn fn_00b7a380(e: &mut Engine) -> Ptr {
    let mut made = [0u32; 4];
    for (slot, count) in made.iter_mut().zip([6u32, 8, 3, 8]) {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        *slot = e
            .call(DECLARATION_CREATE, &args![renderer, count, 1u32])
            .u32();
    }
    let [first, second, third, fourth] = made;
    for entry in &SIX_ENTRIES {
        e.vcall(first, 0x8c, entry);
    }
    e.vcall(first, 0xb4, &args![]);
    for entry in &EIGHT_ENTRIES {
        e.vcall(second, 0x8c, entry);
    }
    e.vcall(second, 0xb4, &args![]);
    e.vcall(third, 0x8c, &args![0u32, 0u32, 0u32, 2u32, 0u32]);
    e.vcall(third, 0x8c, &args![1u32, 3u32, 5u32, 1u32, 0u32]);
    e.vcall(
        third,
        0x94,
        &args![0u32, 2u32, 5u32, 0x8000_0000u32, 0u32, 5u32, 1u32, 0u32],
    );
    e.vcall(third, 0xb4, &args![]);
    for entry in &SIX_ENTRIES {
        e.vcall(fourth, 0x8c, entry);
    }
    e.vcall(
        fourth,
        0x94,
        &args![0u32, 6u32, 8u32, 0x8000_0000u32, 3u32, 5u32, 1u32, 0u32],
    );
    e.vcall(
        fourth,
        0x94,
        &args![0u32, 7u32, 9u32, 0x8000_0001u32, 3u32, 5u32, 2u32, 0u32],
    );
    e.vcall(fourth, 0xb4, &args![]);
    let block = e.call(NI_ALLOCATE, &args![0x8cu32]).u32();
    let shader = if block != 0 {
        e.call(
            SHADER_CONSTRUCT_00B79B00,
            &args![block, first, second, third, fourth],
        )
        .u32()
    } else {
        0
    };
    for slot in [0x4cu32, 0x144, 0x150, 0x11c] {
        e.vcall(shader, slot, &args![]);
    }
    e.vcall(shader, 0xc0, &args![first]);
    Ptr::new(shader)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b7_a380, fn_00b7a380() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const RENDERER: u32 = 0x0f50_0000;
    const DECLARATION_0: u32 = 0x0f60_0000;
    const DECLARATION_VTABLE: u32 = 0x0f10_0000;
    const DECLARATION_SLOTS: u32 = 0x0f30_0000;
    const SHADER: u32 = 0x0f70_0000;
    const SHADER_VTABLE: u32 = 0x0f20_0000;
    const SHADER_SLOTS: u32 = 0x0f40_0000;

    /// Every slot of both vtables is a no-op double at `<slots> + offset`; `Create` answers
    /// declarations 0x100 bytes apart; the allocation answers `SHADER`.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(RENDERER_011F9508, RENDERER);
        e.map(DECLARATION_0, 0x1000);
        e.map(SHADER, 0x1000);
        let declaration_slots: Vec<u32> = (0..0x60).map(|i| DECLARATION_SLOTS + 4 * i).collect();
        let shader_slots: Vec<u32> = (0..0x60).map(|i| SHADER_SLOTS + 4 * i).collect();
        e.put_vtable(DECLARATION_VTABLE, &declaration_slots);
        e.put_vtable(SHADER_VTABLE, &shader_slots);
        for target in declaration_slots.iter().chain(&shader_slots) {
            e.register(*target, |_, _| Ret::default());
        }
        let mut made = 0u32;
        e.register_double(DECLARATION_CREATE, move |e, _| {
            let declaration = DECLARATION_0 + 0x100 * made;
            made += 1;
            e.mem.set_u32(declaration, DECLARATION_VTABLE);
            declaration.into_ret()
        });
        e.register(NI_ALLOCATE, |_, _| SHADER.into_ret());
        e.register(SHADER_CONSTRUCT_00B79B00, |e, a| {
            e.mem.set_u32(a[0], SHADER_VTABLE);
            a[0].into_ret()
        });
        e
    }

    fn entries(declaration: u32, list: &[[u32; 5]]) -> Vec<(u32, Vec<u32>)> {
        list.iter()
            .map(|entry| {
                let mut words = vec![declaration];
                words.extend_from_slice(entry);
                (DECLARATION_SLOTS + 0x8c, words)
            })
            .collect()
    }

    #[test]
    fn fn_00b7a380_makes_four_declarations_then_the_shader() {
        let mut e = rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b7_a380, &[]).u32(), SHADER);
        let mut log = e.call_log.take().unwrap();
        log.remove(0);
        let d = |n: u32| DECLARATION_0 + 0x100 * n;
        let slot = |offset: u32| DECLARATION_SLOTS + offset;
        let mut want: Vec<(u32, Vec<u32>)> = [6, 8, 3, 8]
            .iter()
            .map(|&n| (DECLARATION_CREATE, vec![RENDERER, n, 1]))
            .collect();
        want.extend(entries(d(0), &SIX_ENTRIES));
        want.push((slot(0xb4), vec![d(0)]));
        want.extend(entries(d(1), &EIGHT_ENTRIES));
        want.push((slot(0xb4), vec![d(1)]));
        want.push((slot(0x8c), vec![d(2), 0, 0, 0, 2, 0]));
        want.push((slot(0x8c), vec![d(2), 1, 3, 5, 1, 0]));
        want.push((slot(0x94), vec![d(2), 0, 2, 5, 0x8000_0000, 0, 5, 1, 0]));
        want.push((slot(0xb4), vec![d(2)]));
        want.extend(entries(d(3), &SIX_ENTRIES));
        want.push((slot(0x94), vec![d(3), 0, 6, 8, 0x8000_0000, 3, 5, 1, 0]));
        want.push((slot(0x94), vec![d(3), 0, 7, 9, 0x8000_0001, 3, 5, 2, 0]));
        want.push((slot(0xb4), vec![d(3)]));
        want.push((NI_ALLOCATE, vec![0x8c]));
        want.push((
            SHADER_CONSTRUCT_00B79B00,
            vec![SHADER, d(0), d(1), d(2), d(3)],
        ));
        for offset in [0x4c, 0x144, 0x150, 0x11c] {
            want.push((SHADER_SLOTS + offset, vec![SHADER]));
        }
        want.push((SHADER_SLOTS + 0xc0, vec![SHADER, d(0)]));
        assert_eq!(log, want);
    }
}
