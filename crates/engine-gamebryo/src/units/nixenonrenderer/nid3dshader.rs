//! `NiXenonRenderer/nid3dshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

/// The five words `fn_00e81100` stores at `0126f738`.
const SHADER_FUNCTIONS: [u32; 5] = [
    0x00e8_0860,
    0x00e8_0a50,
    0x00e8_0c50,
    0x00e8_0de0,
    0x00e8_0f70,
];

// Translated from 00e81100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShader::_SDMInit` (Xbox PDB; run, by a jump, at the end of the start-up
/// `00e91dc0`; no arguments): sets the static fixed string at `0126f750` from the literal
/// at `010ef37c` when it does not hold the literal's address already (releasing the old
/// handle), then stores five function addresses at `0126f738` to `0126f748`.
pub fn ni_d3d_shader_sdm_init(e: &mut Engine) {
    let old = e.mem.u32(0x0126_f750);
    if old != 0x010e_f37c {
        let handle = e.call(GET_FIXED_STRING, &args![0x010e_f37cu32]).u32();
        e.mem.set_u32(0x0126_f750, handle);
        release_fixed_string(e, old);
    }
    for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
        e.mem.set_u32(0x0126_f738 + 4 * index as u32, function);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_1100, ni_d3d_shader_sdm_init()),
        entry!(0x00aa_1ac0, fn_00aa1ac0(Ptr) -> u32),
        entry!(0x00e8_0690, fn_00e80690()),
        entry!(0x00e8_06c0, fn_00e806c0(Ptr)),
        entry!(
            0x00e8_07d0,
            fn_00e807d0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

// Translated from 00aa1ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiD3DShader::GetShaderDecl` in the engine map, `thiscall`, no stack words: answers
/// the word at +0x20. The linker shares this body with other getters of a word at +0x20 (the
/// archive code calls it as a file's buffer getter), so the name is not the only user.
pub fn fn_00aa1ac0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x20))
}

// Translated from 00e80690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`), no arguments: the shut-down of the
/// static fixed string `_SDMInit` sets. When `0126f750` holds a handle it gets the handle for
/// a null string (`00a5b690` with 0) and the old one is released (its count at -8 down,
/// unless it is the empty string's handle).
pub fn fn_00e80690(e: &mut Engine) {
    let old = e.mem.u32(0x0126_f750);
    if old == 0 {
        return;
    }
    let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
    e.mem.set_u32(0x0126_f750, handle);
    release_fixed_string(e, old);
}

/// What the destructor `00e806c0` uses: the shader's table (`010eeda4`), the empty fixed
/// string (`0109b220`), the shader factory getter `00e91f10` (no arguments), the program's name
/// getter (`00cc0760`, engine map `NiD3DShaderProgram::GetName`; `thiscall`, no stack words),
/// the renderer's release of the shader `00e75310` (unit `nixenonrenderer.cpp`; `thiscall`, the
/// shader), the render-state block release `00e7f6b0` (unit `run_00e7ef80`; `cdecl`), the
/// current shader global (`0126f74c`), the interlocked decrement import and the
/// `NiD3DShaderInterface` destructor `00e81a50`.
const SHADER_VTABLE_010EEDA4: u32 = 0x010e_eda4;
const EMPTY_FIXED_STRING_0109B220: u32 = 0x0109_b220;
const SHADER_FACTORY_00E91F10: u32 = 0x00e9_1f10;
const GET_NAME_00CC0760: u32 = 0x00cc_0760;
const RENDERER_RELEASE_SHADER_00E75310: u32 = 0x00e7_5310;
const RELEASE_STATE_BLOCK_00E7F6B0: u32 = 0x00e7_f6b0;
const CURRENT_SHADER_0126F74C: u32 = 0x0126_f74c;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SHADER_INTERFACE_DESTRUCT_00E81A50: u32 = 0x00e8_1a50;

// Translated from 00e806c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`): the `NiD3DShader` destructor,
/// `thiscall`, no stack words. Stores the class's table. With a name (+8) that is not the empty
/// string and whose length word (-4) is not 0: the shader factory (`00e91f10`) looks it up
/// (slot 0xc: the name and the name `00cc0760` answers for the shader) and removes it (slot
/// 0x1c with the same) when it finds this shader; with the byte +0x20 set the same with -1.
/// With a renderer (+0x18) it releases the shader (`00e75310`). Then the byte +0x10 is
/// cleared, the render-state block (+0x2c) released (`00e7f6b0`), +0x38 and the current shader
/// global `0126f74c` cleared, the objects at +0x34, +0x30 and +0x28 released (their count at +4
/// down, deleted through their slot 4 at 0), and the `NiD3DShaderInterface` destructor runs
/// (a tail jump).
pub fn fn_00e806c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SHADER_VTABLE_010EEDA4);
    let empty = e.mem.u32(EMPTY_FIXED_STRING_0109B220);
    // The name is compared with the empty string twice, read again for the second time.
    let named = e.mem.u32(base.wrapping_add(8)) != empty && {
        let name = e.mem.u32(base.wrapping_add(8));
        name != empty && e.mem.u32(name.wrapping_sub(4)) != 0
    };
    if named {
        let factory = e.call(SHADER_FACTORY_00E91F10, &args![]).u32();
        let name = e.mem.u32(base.wrapping_add(8));
        let other = e.call(GET_NAME_00CC0760, &args![base]).u32();
        if e.vcall(factory, 0xc, &args![name, other]).u32() == base {
            let name = e.mem.u32(base.wrapping_add(8));
            let other = e.call(GET_NAME_00CC0760, &args![base]).u32();
            e.vcall(factory, 0x1c, &args![name, other]);
        }
        if e.mem.u8(base.wrapping_add(0x20)) != 0 {
            let name = e.mem.u32(base.wrapping_add(8));
            if e.vcall(factory, 0xc, &args![name, 0xffff_ffffu32]).u32() == base {
                let name = e.mem.u32(base.wrapping_add(8));
                e.vcall(factory, 0x1c, &args![name, 0xffff_ffffu32]);
            }
        }
    }
    let renderer = e.mem.u32(base.wrapping_add(0x18));
    if renderer != 0 {
        e.call(RENDERER_RELEASE_SHADER_00E75310, &args![renderer, base]);
    }
    let block = e.mem.u32(base.wrapping_add(0x2c));
    e.mem.set_u8(base.wrapping_add(0x10), 0);
    e.call(RELEASE_STATE_BLOCK_00E7F6B0, &args![block]);
    e.mem.set_u32(base.wrapping_add(0x38), 0);
    e.mem.set_u32(CURRENT_SHADER_0126F74C, 0);
    for offset in [0x34u32, 0x30, 0x28] {
        let object = e.mem.u32(base.wrapping_add(offset));
        if object != 0
            && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(object, 4, &args![]);
        }
    }
    e.call(SHADER_INTERFACE_DESTRUCT_00E81A50, &args![base]);
}

/// The renderer's (+0x18) calls `00e807d0` makes: `00e6fe30` (unnamed; `thiscall`, five
/// words), `00e6fda0` (unnamed; `thiscall`, three words) and `00e6fbb0` (unnamed; `thiscall`,
/// two words).
const RENDERER_00E6FE30: u32 = 0x00e6_fe30;
const RENDERER_00E6FDA0: u32 = 0x00e6_fda0;
const RENDERER_00E6FBB0: u32 = 0x00e6_fbb0;

// Translated from 00e807d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshader.cpp`): a shader's set-up of the skinning
/// state, `thiscall` with eight words (only the second, third and seventh are read). With a
/// skin partition (the third): unless +0x38 is set, the renderer's (+0x18) `00e6fe30` with the
/// second and seventh words, 0, 4 and 0; then the object at +0x1c gets the partition's 16-bit
/// +0x24 (its slot 0x2c) and the renderer's `00e6fda0` runs with the second word, the partition
/// and the seventh. Without one: when +0x38 is clear or the renderer's byte +0x209 is set, the
/// object at +0x1c gets 0 (slot 0x2c) and the renderer's `00e6fbb0` runs with the seventh word
/// and 1. Answers 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e807d0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    second: u32,
    partition: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    seventh: u32,
    _unused_8: u32,
) -> u32 {
    let base = this.addr();
    if partition != 0 {
        if e.mem.u32(base.wrapping_add(0x38)) == 0 {
            let renderer = e.mem.u32(base.wrapping_add(0x18));
            e.call(
                RENDERER_00E6FE30,
                &args![renderer, second, seventh, 0u32, 4u32, 0u32],
            );
        }
        let target = e.mem.u32(base.wrapping_add(0x1c));
        let bones = u32::from(e.mem.u16(partition.wrapping_add(0x24)));
        e.vcall(target, 0x2c, &args![bones]);
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        e.call(
            RENDERER_00E6FDA0,
            &args![renderer, second, partition, seventh],
        );
        return 0;
    }
    let renderer = e.mem.u32(base.wrapping_add(0x18));
    if e.mem.u32(base.wrapping_add(0x38)) == 0 || e.mem.u8(renderer.wrapping_add(0x209)) != 0 {
        let target = e.mem.u32(base.wrapping_add(0x1c));
        e.vcall(target, 0x2c, &args![0u32]);
        let renderer = e.mem.u32(base.wrapping_add(0x18));
        e.call(RENDERER_00E6FBB0, &args![renderer, seventh, 1u32]);
    }
    0
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_the_string_once_and_the_five_functions() {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.mem.set_u32(0x0126_f750, empty);
        e.call_log = Some(vec![]);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(e.mem.u32(0x0126_f750), 0x010e_f37c + 0x1000);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x010e_f37c]]);
        for (index, function) in SHADER_FUNCTIONS.into_iter().enumerate() {
            assert_eq!(e.mem.u32(0x0126_f738 + 4 * index as u32), function);
        }
        // The word already equals the literal's address: no lookup.
        e.mem.set_u32(0x0126_f750, 0x010e_f37c);
        e.call(0x00e8_1100, &args![]);
        assert_eq!(calls_to(&e, GET_FIXED_STRING).len(), 1);
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    const LOOK_UP: u32 = 0x0eb0_000c;
    const REMOVE: u32 = 0x0eb0_001c;
    const SET_BONES: u32 = 0x0eb0_002c;
    const DELETE: u32 = 0x0eb0_0004;
    const TABLE: u32 = 0x0eb1_0000;

    #[test]
    fn fn_00aa1ac0_reads_the_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x20, 0x55);
        assert_eq!(e.call(0x00aa_1ac0, &args![this]).u32(), 0x55);
    }

    #[test]
    fn fn_00e80690_releases_the_static_string() {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x1000);
        e.register(GET_FIXED_STRING, |_, _| 0x7770u32.into_ret());
        let handle = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(0x0126_f750, handle);
        e.call_log = Some(vec![]);
        e.call(0x00e8_0690, &args![]);
        assert_eq!(e.mem.u32(0x0126_f750), 0x7770);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]]);
        // Nothing held: nothing done.
        e.mem.set_u32(0x0126_f750, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_0690, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e806c0_unregisters_and_releases() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING_0109B220, 0x0eee_0000);
        let mut slots = vec![0u32; 8];
        slots[1] = DELETE;
        slots[3] = LOOK_UP;
        slots[7] = REMOVE;
        e.put_vtable(TABLE, &slots);
        let this = e.mem.alloc(0x3c);
        let factory = e.mem.alloc(4);
        e.mem.set_u32(factory, TABLE);
        e.register_double(SHADER_FACTORY_00E91F10, move |_, _| factory.into_ret());
        e.register_double(LOOK_UP, move |_, _| this.into_ret());
        e.register(GET_NAME_00CC0760, |_, _| 0x4e00u32.into_ret());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        for no_op in [
            REMOVE,
            DELETE,
            RENDERER_RELEASE_SHADER_00E75310,
            RELEASE_STATE_BLOCK_00E7F6B0,
            SHADER_INTERFACE_DESTRUCT_00E81A50,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 4, 3);
        e.mem.set_u32(this + 8, name);
        e.mem.set_u8(this + 0x20, 1);
        e.mem.set_u32(this + 0x18, 0x1800);
        e.mem.set_u32(this + 0x2c, 0x2c00);
        e.mem.set_u32(this + 0x38, 9);
        e.mem.set_u32(CURRENT_SHADER_0126F74C, 9);
        let objects: Vec<u32> = (0..3)
            .map(|_| {
                let object = e.mem.alloc(8);
                e.mem.set_u32(object, TABLE);
                object
            })
            .collect();
        for (offset, object) in [0x34u32, 0x30, 0x28].iter().zip(&objects) {
            e.mem.set_u32(this + offset, *object);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e8_06c0, &args![this]);
        assert_eq!(
            calls(&e, LOOK_UP),
            vec![
                vec![factory, name, 0x4e00],
                vec![factory, name, 0xffff_ffff]
            ]
        );
        assert_eq!(
            calls(&e, REMOVE),
            vec![
                vec![factory, name, 0x4e00],
                vec![factory, name, 0xffff_ffff]
            ]
        );
        assert_eq!(
            calls(&e, RENDERER_RELEASE_SHADER_00E75310),
            vec![vec![0x1800, this]]
        );
        assert_eq!(calls(&e, RELEASE_STATE_BLOCK_00E7F6B0), vec![vec![0x2c00]]);
        assert_eq!(
            calls(&e, DELETE),
            vec![vec![objects[0]], vec![objects[1]], vec![objects[2]]]
        );
        assert_eq!(
            calls(&e, SHADER_INTERFACE_DESTRUCT_00E81A50),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), SHADER_VTABLE_010EEDA4);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(CURRENT_SHADER_0126F74C), 0);
        // The empty name: no lookup; no renderer: no release.
        e.mem.set_u32(this + 8, 0x0eee_0000);
        e.mem.set_u32(this + 0x18, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_06c0, &args![this]);
        assert!(calls(&e, LOOK_UP).is_empty());
        assert!(calls(&e, RENDERER_RELEASE_SHADER_00E75310).is_empty());
    }

    #[test]
    fn fn_00e807d0_sets_up_skinning() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0xc];
        slots[0xb] = SET_BONES;
        e.put_vtable(TABLE, &slots);
        for no_op in [
            SET_BONES,
            RENDERER_00E6FE30,
            RENDERER_00E6FDA0,
            RENDERER_00E6FBB0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, TABLE);
        let renderer = e.mem.alloc(0x210);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x18, renderer);
        e.mem.set_u32(this + 0x1c, target);
        let partition = e.mem.alloc(0x28);
        e.mem.set_u16(partition + 0x24, 4);
        let run = |e: &mut Engine, partition: u32| {
            e.call(
                0x00e8_07d0,
                &args![this, 1u32, 2u32, partition, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32()
        };
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, partition), 0);
        assert_eq!(
            calls(&e, RENDERER_00E6FE30),
            vec![vec![renderer, 2, 7, 0, 4, 0]]
        );
        assert_eq!(calls(&e, SET_BONES), vec![vec![target, 4]]);
        assert_eq!(
            calls(&e, RENDERER_00E6FDA0),
            vec![vec![renderer, 2, partition, 7]]
        );
        // +0x38 set: no 00e6fe30.
        e.mem.set_u32(this + 0x38, 1);
        e.call_log = Some(vec![]);
        run(&mut e, partition);
        assert!(calls(&e, RENDERER_00E6FE30).is_empty());
        // No partition, +0x38 set and the renderer's byte clear: nothing.
        e.call_log = Some(vec![]);
        run(&mut e, 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.mem.set_u8(renderer + 0x209, 1);
        e.call_log = Some(vec![]);
        run(&mut e, 0);
        assert_eq!(calls(&e, SET_BONES), vec![vec![target, 0]]);
        assert_eq!(calls(&e, RENDERER_00E6FBB0), vec![vec![renderer, 7, 1]]);
    }
}
