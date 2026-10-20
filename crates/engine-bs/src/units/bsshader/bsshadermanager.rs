//! `BSShader/bsshadermanager.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `sin` (`00eca0a0`): the argument is the x87 `ST0`, so a leading `f64`; the result is `ST0`.
const CRT_SIN: u32 = 0x00ec_a0a0;
/// CRT `cos` (`00ec9f70`), same convention.
const CRT_COS: u32 = 0x00ec_9f70;
/// The first `double` the argument is multiplied by.
const FIRST_FACTOR: u32 = 0x0101_1588;
/// The second `double` the argument is multiplied by.
const SECOND_FACTOR: u32 = 0x0102_3128;
/// The `double` angle the ratio is divided by `tan` of.
const REFERENCE_ANGLE: u32 = 0x010a_d6d8;
/// The `float` the result is stored in.
const RATIO_RESULT: u32 = 0x011a_dcc8;

// Translated from 00b54000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`). `cdecl`, one `float` stack word: scales it
/// by the doubles at `01011588` and `01023128` (stored as a `float`), takes
/// `sin / cos` of the result and of the double at `010ad6d8` (each `sin` and `cos` stored as
/// a `float` first) and stores the first quotient divided by the second in the `float` at
/// `011adcc8`. Computed on the x87 as the exe does (53-bit precision, NaN choice of the x87).
pub fn fn_00b54000(e: &mut Engine, value: f32) {
    let first_factor: f64 = e.global(FIRST_FACTOR);
    let second_factor: f64 = e.global(SECOND_FACTOR);
    let scaled = x87_store32(x87_mul(
        x87_mul(x87_load32(value.to_bits()), first_factor),
        second_factor,
    ));
    let angle = x87_load32(scaled);
    let sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![angle]).f64()));
    let cosine = x87_store32(e.call(CRT_COS, &args![angle]).f64());
    let quotient = x87_quiet_bits(x87_store32(x87_div(x87_m32(sine), x87_load32(cosine))));

    let reference: f64 = e.global(REFERENCE_ANGLE);
    let reference_sine = x87_quiet_bits(x87_store32(e.call(CRT_SIN, &args![reference]).f64()));
    let reference_cosine = x87_store32(e.call(CRT_COS, &args![reference]).f64());
    let reference_quotient = x87_div(x87_m32(reference_sine), x87_load32(reference_cosine));
    let ratio = x87_div(x87_m32(quotient), reference_quotient);
    e.set_global(RATIO_RESULT, f32::from_bits(x87_store32(ratio)));
}

/// The table of shadow scene nodes, one word per index (`011f91c8`).
const SHADOW_SCENE_NODES: u32 = 0x011f_91c8;
/// The word `BSShaderManager::GetAccumulator` answers (`011f95ec`).
const ACCUMULATOR: u32 = 0x011f_95ec;
/// `ShadowSceneNode::SetFogProperty` (engine map, `00b54110`): `thiscall`, `RET 4`.
const SHADOW_SCENE_NODE_SET_FOG_PROPERTY: u32 = 0x00b5_4110;

/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`,
/// size) and deallocate (`00aa4060`, `thiscall`, block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocation `00aa13e0` (`cdecl`, size).
const NEW_OBJECT: u32 = 0x00aa_13e0;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The thread block's word at `+0x2b4` (the allocation context), set to 5 during the set-up.
const TLS_CONTEXT: u32 = 0x2b4;
/// The set-up's first-run byte (`011f91c6`).
const SET_UP_DONE_011F91C6: u32 = 0x011f_91c6;
/// The objects the first run creates and the globals it reads.
const OBJECT_011F91A8: u32 = 0x011f_91a8;
const OBJECT_011F9498: u32 = 0x011f_9498;
const OBJECT_011F91AC: u32 = 0x011f_91ac;
const HOLDER_011F94BC: u32 = 0x011f_94bc;
const GLOBAL_011F9508: u32 = 0x011f_9508;
/// The four floats set to (0.5, 0.5, 0.5, 1.0) on the first run, and the `float` 0.5 the
/// code loads (`01016248`).
const FLOATS_011FA090: u32 = 0x011f_a090;
const HALF_01016248: u32 = 0x0101_6248;
/// The string the first run's holder is made from (`010ada3c`, passed to `00a5b690`).
const HOLDER_NAME_010ADA3C: u32 = 0x010a_da3c;
/// The accumulator array: count (`011f916c`) and pointer (`011f9170`); the word cleared at
/// the end (`011f9168`); the word copied (`011f94f0` into `011a9608`).
const ACCUMULATOR_COUNT_011F916C: u32 = 0x011f_916c;
const ACCUMULATOR_ARRAY_011F9170: u32 = 0x011f_9170;
const GLOBAL_011F9168: u32 = 0x011f_9168;
const GLOBAL_011F94F0: u32 = 0x011f_94f0;
const GLOBAL_011A9608: u32 = 0x011a_9608;
const GLOBAL_011FFE2C: u32 = 0x011f_fe2c;
const GLOBAL_011FFE30: u32 = 0x011f_fe30;

/// The function-pointer words `00b57420` clears, in the exe's order.
const CLEARED_WORDS_00B57420: [u32; 32] = [
    0x011f_9f40,
    0x011f_9f68,
    0x011f_9f70,
    0x011f_9f74,
    0x011f_9f78,
    0x011f_9f80,
    0x011f_9fa0,
    0x011f_9fa4,
    0x011f_9fa8,
    0x011f_9fac,
    0x011f_9fb4,
    0x011f_9fb8,
    0x011f_9f44,
    0x011f_9f48,
    0x011f_9f4c,
    0x011f_9f50,
    0x011f_9f54,
    0x011f_9f58,
    0x011f_9f5c,
    0x011f_9f60,
    0x011f_9f64,
    0x011f_9f6c,
    0x011f_9f7c,
    0x011f_9f84,
    0x011f_9f88,
    0x011f_9f8c,
    0x011f_9f90,
    0x011f_9f94,
    0x011f_9f98,
    0x011f_9f9c,
    0x011f_9fb0,
    0x011f_9fbc,
];

/// The (word, code address) stores `00b57420` makes after clearing, in the exe's order
/// (immediates of its `MOV` instructions).
const FUNCTION_WORDS_00B57420: [(u32, u32); 14] = [
    (0x011f_9f80, 0x00b6_4020),
    (0x011f_9f3c, 0x00b6_4020),
    (0x011f_9fa8, 0x00b6_5a90),
    (0x011f_9fac, 0x00b6_06e0),
    (0x011f_9fb4, 0x00b6_3f90),
    (0x011f_9fa4, 0x00b6_4440),
    (0x011f_9fb8, 0x00b6_44b0),
    (0x011f_9fa0, 0x00b6_4490),
    (0x011f_9f40, 0x00b6_65a0),
    (0x011f_9f70, 0x00b6_5dc0),
    (0x011f_9f68, 0x00b6_5e80),
    (0x011f_9f74, 0x00a2_9680),
    (0x011f_9f78, 0x00b6_4530),
    (0x011f_9f38, 0x00b6_65a0),
];

// Translated from 0070b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSShaderManager::SetShadowSceneNode` in the engine map (Xbox PDB), but the body is a
/// `thiscall` on the node itself (`RET 4`): it stores the index byte at `+0x120` of `this` and
/// then puts `this` into the shadow-scene-node table at that index (`00b4f2f0`).
pub fn fn_0070b760(e: &mut Engine, this: Ptr, index: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x120), index);
    fn_00b4f2f0(e, u32::from(index), this);
}

// Translated from 00b4f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`). `cdecl` (index, node): stores the node in
/// the shadow-scene-node table at `011f91c8 + 4 * index` (no bounds check).
pub fn fn_00b4f2f0(e: &mut Engine, index: u32, node: Ptr) {
    e.mem.set_u32(
        SHADOW_SCENE_NODES.wrapping_add(index.wrapping_mul(4)),
        node.addr(),
    );
}

// Translated from 00b4f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetAccumulator` (Xbox PDB): the word at `011f95ec`.
pub fn bs_shader_manager_get_accumulator(e: &mut Engine) -> Ptr {
    Ptr::new(e.global::<u32>(ACCUMULATOR))
}

// Translated from 00b55540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::SetFogProperty` (Xbox PDB). `cdecl` (index, fog property): calls
/// `ShadowSceneNode::SetFogProperty` (`00b54110`) on the shadow scene node at that index of the
/// table (`011f91c8`) with the fog property.
pub fn bs_shader_manager_set_fog_property(e: &mut Engine, index: u32, fog: Ptr) {
    let node = e
        .mem
        .u32(SHADOW_SCENE_NODES.wrapping_add(index.wrapping_mul(4)));
    e.call(SHADOW_SCENE_NODE_SET_FOG_PROPERTY, &args![node, fog]);
}

/// `MemoryManager::Allocate` on the singleton.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// The first-run object at `011f9498` released: `00b7fd50` on it, then the memory manager
/// frees it and the global is cleared.
fn release_object_011f9498(e: &mut Engine, object: u32) {
    e.call(0x00b7_fd50, &args![object]);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, object]);
    e.set_global(OBJECT_011F9498, 0u32);
}

// Translated from 00b57420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`); no arguments. The shader manager's
/// set-up, with the thread block's allocation context (`+0x2b4`) at 5 and restored at the end:
///
/// * On the first run (byte `011f91c6` clear): makes the 0xb0-byte object at `011f91a8`
///   (`00b6dd80`) and the 0x1c-byte one at `011f9498` (`00b7ff40` with `00b7faf0`'s value),
///   runs `00b7fe40` on the latter with its first word (read even when the allocation
///   failed) and releases it (`00b7fd50`, free) when its word at `+4` is 0; runs `00b55bb0`;
///   sets the floats at `011fa090` to 0.5, 0.5, 0.5 (the `float` at `01016248`) and 1.0; when
///   `011f9508` is not 0, gets shaders 1 to 0x22 (`BSShaderManager::GetShader`, `00b55560`),
///   makes the 0x31c-byte object at `011f91ac` (`00b97a80`) and releases `011f9498` if it is
///   still there; registers the streamables (`00b4f1a0`), runs `00b6e450` on `011f91a8` with
///   `011f9508`, makes a one-word holder of `00a5b690(010ada3c)` at `011f94bc` and sets the
///   first-run byte.
/// * Every run: clears 32 function-pointer words of the `011f9f38` block and stores 14 code
///   addresses into it, clears `011ffe2c` and `011ffe30`, runs `MTRenderingSystem::CreateThreads`
///   (`00ba3720`) and `00b97c10`, copies `011f94f0` to `011a9608`, then makes the array of
///   `011f916c` accumulator pointers (a count word before the elements, the elements
///   constructed by `00ec782f` with `00b54620`/`0045cec0`) at `011f9170` and fills each slot
///   with a new `BSShaderAccumulator` (`00b660d0(0x63, 1, 0x2f7)` on 0x280 bytes) through the
///   `NiPointer` assignment (the old one released, deleted through its vtable `+4` when its
///   count reaches 0), and clears `011f9168`.
///
/// The `ECX` the exe loads before `CreateThreads` (`01200088`) is not read by it.
// Translation incomplete: 00b57420 - the C++ exception frame (FS:[0] chain, unwind states 1 to 6) is not translated.
pub fn fn_00b57420(e: &mut Engine) {
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), 5);

    if e.global::<u8>(SET_UP_DONE_011F91C6) == 0 {
        let memory = allocate(e, 0xb0);
        let object = if memory != 0 {
            e.call(0x00b6_dd80, &args![memory]).u32()
        } else {
            0
        };
        e.set_global(OBJECT_011F91A8, object);

        let memory = allocate(e, 0x1c);
        let object = if memory != 0 {
            let value = e.call(0x00b7_faf0, &args![]).u32();
            e.call(0x00b7_ff40, &args![memory, value]).u32()
        } else {
            0
        };
        e.set_global(OBJECT_011F9498, object);
        let first = e.mem.u32(object);
        e.call(0x00b7_fe40, &args![object, first]);
        let object = e.global::<u32>(OBJECT_011F9498);
        if e.mem.u32(object.wrapping_add(4)) == 0 {
            release_object_011f9498(e, object);
        }

        e.call(0x00b5_5bb0, &args![]);
        let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
        e.set_global(FLOATS_011FA090, half);
        e.set_global(FLOATS_011FA090 + 4, half);
        e.set_global(FLOATS_011FA090 + 8, half);
        e.set_global(FLOATS_011FA090 + 0xc, 1.0f32.to_bits());

        if e.global::<u32>(GLOBAL_011F9508) != 0 {
            for shader in 1u32..0x23 {
                e.call(0x00b5_5560, &args![shader]);
            }
            let memory = allocate(e, 0x31c);
            let object = if memory != 0 {
                e.call(0x00b9_7a80, &args![memory]).u32()
            } else {
                0
            };
            e.set_global(OBJECT_011F91AC, object);
            let object = e.global::<u32>(OBJECT_011F9498);
            if object != 0 {
                release_object_011f9498(e, object);
            }
        }

        e.call(0x00b4_f1a0, &args![]);
        let word = e.global::<u32>(GLOBAL_011F9508);
        let object = e.global::<u32>(OBJECT_011F91A8);
        e.call(0x00b6_e450, &args![object, word]);

        let holder = allocate(e, 4);
        let holder = if holder != 0 {
            let text = e.call(0x00a5_b690, &args![HOLDER_NAME_010ADA3C]).u32();
            e.mem.set_u32(holder, text);
            holder
        } else {
            0
        };
        e.set_global(HOLDER_011F94BC, holder);
        e.set_global(SET_UP_DONE_011F91C6, 1u8);
    }

    for word in CLEARED_WORDS_00B57420 {
        e.set_global(word, 0u32);
    }
    for (word, code) in FUNCTION_WORDS_00B57420 {
        e.set_global(word, code);
    }
    e.set_global(GLOBAL_011FFE2C, 0u32);
    e.set_global(GLOBAL_011FFE30, 0u32);
    e.call(0x00ba_3720, &args![]);
    e.call(0x00b9_7c10, &args![]);
    let copied = e.global::<u32>(GLOBAL_011F94F0);
    let count = e.global::<u32>(ACCUMULATOR_COUNT_011F916C);
    e.set_global(GLOBAL_011A9608, copied);

    // `count * 4 + 4` for the count word, all ones on overflow at either step
    // (`MUL; SETO; NEG; OR` then `ADD; SETC; NEG; OR`).
    let size = count
        .checked_mul(4)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let memory = allocate(e, size);
    let array = if memory != 0 {
        e.mem.set_u32(memory, count);
        let elements = memory.wrapping_add(4);
        e.call(
            0x00ec_782f,
            &args![elements, 4u32, count, 0x00b5_4620u32, 0x0045_cec0u32],
        );
        elements
    } else {
        0
    };
    e.set_global(ACCUMULATOR_ARRAY_011F9170, array);

    let mut index = 0u32;
    if e.global::<u32>(ACCUMULATOR_COUNT_011F916C) != 0 {
        loop {
            let memory = e.call(NEW_OBJECT, &args![0x280u32]).u32();
            let accumulator = if memory != 0 {
                e.call(0x00b6_60d0, &args![memory, 0x63u32, 1u32, 0x2f7u32])
                    .u32()
            } else {
                0
            };
            let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
            let slot = array.wrapping_add(index.wrapping_mul(4));
            let old = e.mem.u32(slot);
            if old != accumulator {
                if old != 0 {
                    let left = e
                        .call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
                        .u32();
                    if left == 0 {
                        e.vcall(old, 4, &args![]);
                    }
                }
                e.mem.set_u32(slot, accumulator);
                if accumulator != 0 {
                    e.call(
                        IMPORT_INTERLOCKED_INCREMENT,
                        &args![accumulator.wrapping_add(4)],
                    );
                }
            }
            index = index.wrapping_add(1);
            if index >= e.global::<u32>(ACCUMULATOR_COUNT_011F916C) {
                break;
            }
        }
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    e.set_global(GLOBAL_011F9168, 0u32);
}

/// The shader table `BSShaderManager::GetShader` fills, one `NiPointer` per index 0 to 0x22
/// (`011f9548`).
const SHADER_TABLE_011F9548: u32 = 0x011f_9548;
/// The highest shader index `GetShader` accepts.
const LAST_SHADER_INDEX: u32 = 0x22;
/// The byte that selects shader 0x1d's creator (`011f94a6`).
const SHADER_1D_SELECT_011F94A6: u32 = 0x011f_94a6;
/// The `NiPointer` assignment `00a5f860` (`thiscall` on the slot, `RET 4`: the new object).
const NI_POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;
/// Shader 4's creator (`00bd4750`), which runs after shader 1 is made, and shader 0x1d's
/// when the byte at `011f94a6` is set (`00bbd0d0`).
const SHADER_4_CREATE: u32 = 0x00bd_4750;
const SHADER_1D_CREATE: u32 = 0x00bb_d0d0;

/// The creator `GetShader` calls for a shader index (no stack words, the shader in `EAX`):
/// the arms of its switch, read from the jump table at `00b55748` in Ghidra. Indices without
/// a creator (0, 0xb, 0xc, 0x12 to 0x17, 0x1b, 0x1c, 0x1f) make nothing; 4 and 0x1d are
/// handled by the caller.
fn shader_creator(index: u32) -> Option<u32> {
    match index {
        1 => Some(0x00b7_a380),
        2 => Some(0x00ba_cf00),
        3 => Some(0x00bd_5920),
        5 => Some(0x00bd_2e70),
        6 => Some(0x00bb_2c00),
        7 => Some(0x00b8_0e70),
        8 => Some(0x00bb_6e30),
        9 => Some(0x00bd_23f0),
        0xa => Some(0x00b8_a390),
        0xd => Some(0x00bd_14b0),
        0xe => Some(0x00bc_fb00),
        0xf => Some(0x00bc_d680),
        0x10 => Some(0x00bc_c0a0),
        0x11 => Some(0x00b8_6de0),
        0x18 => Some(0x00ba_58b0),
        0x19 => Some(0x00bc_10e0),
        0x1a => Some(0x00bc_4af0),
        0x1e => Some(0x00bb_0660),
        0x20 => Some(0x00bc_ae90),
        0x21 => Some(0x00bc_93c0),
        0x22 => Some(0x00bc_59b0),
        _ => None,
    }
}

/// An `NiPointer`'s release: the count at +4 decremented, the object deleted through its
/// vtable slot +4 when it reaches 0.
fn release_ni_pointer(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00b55560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::GetShader` (Xbox PDB). `cdecl`, one word (the shader index). Answers 0
/// when the word at `011f9508` is 0 or the index is above 0x22; otherwise the shader in the
/// table at `011f9548`, making it first when the slot is empty: with the thread block's
/// allocation context (`+0x2b4`) at 5, the index's creator runs (shader 4 first gets shader
/// 1 through this function) and its result is assigned to the slot (`00a5f860`); for index
/// 0x1d the creator is `00bbd0d0` when the byte at `011f94a6` is set, else the slot's
/// shader is released and the slot cleared (the slot is empty on that path, so nothing is
/// released). The context is restored before answering the slot's word.
// Translation incomplete: 00b55560 - the C++ exception frame (FS:[0] chain, unwind state 0 around the creation) is not translated.
pub fn bs_shader_manager_get_shader(e: &mut Engine, index: u32) -> Ptr {
    if e.global::<u32>(GLOBAL_011F9508) == 0 || index > LAST_SHADER_INDEX {
        return Ptr::new(0);
    }
    let slot = SHADER_TABLE_011F9548.wrapping_add(index.wrapping_mul(4));
    if e.mem.u32(slot) != 0 {
        return Ptr::new(e.mem.u32(slot));
    }
    let tls = e.tls();
    let saved_context = e.mem.u32(tls.wrapping_add(TLS_CONTEXT));
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), 5);
    let created = match index {
        4 => {
            bs_shader_manager_get_shader(e, 1);
            Some(e.call(SHADER_4_CREATE, &args![]).u32())
        }
        0x1d => {
            if e.global::<u8>(SHADER_1D_SELECT_011F94A6) != 0 {
                Some(e.call(SHADER_1D_CREATE, &args![]).u32())
            } else {
                let held = e.mem.u32(slot);
                if held != 0 {
                    release_ni_pointer(e, held);
                    e.mem.set_u32(slot, 0);
                    let shader = e.mem.u32(slot);
                    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
                    return Ptr::new(shader);
                }
                None
            }
        }
        _ => shader_creator(index).map(|create| e.call(create, &args![]).u32()),
    };
    if let Some(shader) = created {
        e.call(NI_POINTER_ASSIGN_00A5F860, &args![slot, shader]);
    }
    e.mem.set_u32(tls.wrapping_add(TLS_CONTEXT), saved_context);
    Ptr::new(e.mem.u32(slot))
}

/// `NiStream::RegisterLoader` (engine map, `00a64900`, `cdecl`: class name, loader).
const NI_STREAM_REGISTER_LOADER: u32 = 0x00a6_4900;

/// The (class name, loader) pairs `RegisterStreamables` registers, in the exe's order
/// (immediates of its `PUSH` instructions; the names are the exe's strings at those
/// addresses, read in Ghidra).
const STREAMABLES_00B4F1A0: [(u32, u32); 20] = [
    // "BSShaderProperty"
    (0x010a_93a0, 0x00ba_9250),
    // "BSShaderLightingProperty"
    (0x010a_9384, 0x00b7_1920),
    // "BSShaderPPLightingProperty"
    (0x010a_9368, 0x00b6_8d50),
    // "Lighting30ShaderProperty"
    (0x010a_934c, 0x00bb_45d0),
    // "HairShaderProperty"
    (0x010a_9338, 0x00bc_4340),
    // "NoFaderShaderProperty"
    (0x010a_9320, 0x00bb_45d0),
    // "WaterShaderProperty"
    (0x010a_930c, 0x00b6_ae60),
    // "BSShaderNoLightingProperty"
    (0x010a_92f0, 0x00b6_fe80),
    // "VolumetricFogShaderProperty"
    (0x010a_92d4, 0x00bc_38c0),
    // "SkyShaderProperty"
    (0x010a_92c0, 0x00ba_ee40),
    // "TallGrassShaderProperty"
    (0x010a_92a8, 0x00b6_0f50),
    // "BSTextureData"
    (0x010a_9298, 0x00c5_eed0),
    // "BSShaderTextureSet"
    (0x010a_9284, 0x00ba_8a00),
    // "BSRefractionStrengthController"
    (0x010a_9264, 0x00bc_36a0),
    // "BSRefractionFirePeriodController"
    (0x010a_9240, 0x00bc_33f0),
    // "TileShaderProperty"
    (0x010a_922c, 0x00bb_7ef0),
    // "BSMaterialEmittanceMultController"
    (0x010a_9208, 0x00bc_3190),
    // "BSFadeNode"
    (0x010a_9194, 0x00b4_eaa0),
    // "BSOrderedNode"
    (0x010a_91f8, 0x00bc_2cb0),
    // "BSDistantTreeShaderProperty"
    (0x010a_91dc, 0x00bb_7300),
];

// Translated from 00b4f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderManager::RegisterStreamables` (Xbox PDB). No arguments: registers 20 stream
/// loaders with `NiStream::RegisterLoader` (`00a64900`), each a class name and the code
/// address of its loader (the same loader `00bb45d0` serves two names).
pub fn bs_shader_manager_register_streamables(e: &mut Engine) {
    for (name, loader) in STREAMABLES_00B4F1A0 {
        e.call(NI_STREAM_REGISTER_LOADER, &args![name, loader]);
    }
}

/// `BSFogProperty::BSFogProperty` (engine map, `00bb8180`, `thiscall`, no stack words).
const BS_FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
/// The `float` the default fog's four distances are loaded from (`010ad6e0`).
const FOG_DISTANCE_010AD6E0: u32 = 0x010a_d6e0;

// Translated from 00b54110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShadowSceneNode::SetFogProperty` (Xbox PDB). `thiscall`, `RET 4`, one word (the fog
/// property). With a fog property: assigns it to the node's `spFog` (+0x134; the old one
/// released, the new one's count incremented, nothing when they are the same). Without one:
/// makes a 0x64-byte `BSFogProperty` (`00aa13e0`, then its constructor `00bb8180` when the
/// allocation succeeded), assigns it the same way (its count incremented when not null),
/// then sets on the node's fog: `fStartDistance`, `fEndDistance` and the floats at +0x34 and
/// +0x38 (`fStartWaterDistance`, `fEndWaterDistance` in the Xbox PDB) from the `float` at
/// `010ad6e0`, `fPower` 1.0, and the colours at +0x20 (`m_kColor`) and +0x54 (`kWaterColor`)
/// to the `float` at `01016248` in each of their three words, in the exe's order.
// Translation incomplete: 00b54110 - the C++ exception frame (FS:[0] chain, unwind state 0 around the constructor) is not translated.
pub fn shadow_scene_node_set_fog_property(e: &mut Engine, this: Ptr, fog: Ptr) {
    use crate::units::bsshader::bsfogproperty::BSFogProperty;
    // `ShadowSceneNode::spFog` (Xbox PDB name, PC offset): `NiPointer<BSFogProperty>`.
    let field = this.addr().wrapping_add(0x134);
    let fog = fog.addr();
    if fog != 0 {
        let old = e.mem.u32(field);
        if old != fog {
            if old != 0 {
                release_ni_pointer(e, old);
            }
            e.mem.set_u32(field, fog);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![fog.wrapping_add(4)]);
        }
        return;
    }
    let memory = e.call(NEW_OBJECT, &args![0x64u32]).u32();
    let made = if memory != 0 {
        e.call(BS_FOG_PROPERTY_CONSTRUCT, &args![memory]).u32()
    } else {
        0
    };
    let old = e.mem.u32(field);
    if old != made {
        if old != 0 {
            release_ni_pointer(e, old);
        }
        e.mem.set_u32(field, made);
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
        }
    }
    let property = Ptr::<BSFogProperty>::new(e.mem.u32(field));
    let distance = f32::from_bits(x87_quiet_bits(e.global::<u32>(FOG_DISTANCE_010AD6E0)));
    e.set(property, BSFogProperty::fStartDistance, distance);
    e.set(property, BSFogProperty::fEndDistance, distance);
    e.set(
        property,
        BSFogProperty::fPower,
        f32::from_bits(x87_store32(1.0)),
    );
    // `fStartWaterDistance`, `fEndWaterDistance` (Xbox PDB, `BSFogProperty`).
    e.mem.set_f32(property.addr().wrapping_add(0x34), distance);
    e.mem.set_f32(property.addr().wrapping_add(0x38), distance);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    // `m_kColor` (Xbox PDB, `NiFogProperty`), three floats.
    for word in 0..3u32 {
        e.mem
            .set_u32(property.addr().wrapping_add(0x20 + 4 * word), half);
    }
    // `kWaterColor` (Xbox PDB, `BSFogProperty`), three floats; the node's word read again.
    let property = e.mem.u32(field);
    for word in 0..3u32 {
        e.mem.set_u32(property.wrapping_add(0x54 + 4 * word), half);
    }
}

/// `NiPixelData::NiPixelData_ov2` (engine map, `00a7c190`): `thiscall`, five stack words
/// (width, height, the address of a pixel format, mipmap levels, faces).
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// `NiPixelData` fields (Xbox PDB names, the PC offsets the code uses): `m_pucPixels`
/// (+0x50), `m_puiOffsetInBytes` (+0x5c), `m_uiMipmapLevels` (+0x60), `m_uiRevID` (+0x68),
/// `bNoConvert` (+0x70).
const PIXELS: u32 = 0x50;
const OFFSETS_IN_BYTES: u32 = 0x5c;
const MIPMAP_LEVELS: u32 = 0x60;
const REVISION: u32 = 0x68;
const NO_CONVERT: u32 = 0x70;
/// The pixel format copied onto the stack for the cube (0x11 words at `011aaac8`) and the
/// two formats of the small textures (`011aa2a0`, four bytes a pixel in the fills;
/// `011aa2e8`, three).
const CUBE_PIXEL_FORMAT_011AAAC8: u32 = 0x011a_aac8;
const PIXEL_FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const PIXEL_FORMAT_011AA2E8: u32 = 0x011a_a2e8;
/// The texture format preferences passed to the texture constructors (`011a9598`).
const FORMAT_PREFS_011A9598: u32 = 0x011a_9598;
/// The `NiFixedString` maker (`00a5b690`, `cdecl`: text) and the word a string is compared
/// with before its count (8 bytes before its text) is decremented (`0109b220`).
const FIXED_STRING_MAKE: u32 = 0x00a5_b690;
const FIXED_STRING_NONE: u32 = 0x0109_b220;
/// `NiSourceTexture::Create_ov2` (engine map, `00a5ff10`, `cdecl`: pixel data, name,
/// format preferences) and `NiSourceTexture::Create` (`00a5fd70`, `cdecl`: the address of
/// the name, format preferences, two words).
const SOURCE_TEXTURE_FROM_PIXELS: u32 = 0x00a5_ff10;
const SOURCE_TEXTURE_FROM_FILE: u32 = 0x00a5_fd70;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words).
const BS_SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;
/// The cube map maker of `NiMain/nisourcecubemap.cpp` (`00a61120`, `cdecl`: six file
/// names, the renderer, the address of the name, format preferences).
const SOURCE_CUBE_MAP_CREATE: u32 = 0x00a6_1120;
/// The formatter `00406d00` (`cdecl`: buffer, size, format, then the format's words).
const FORMAT_00406D00: u32 = 0x0040_6d00;
/// The CRT square root `00ec6040` (argument and result in `ST0`) and the CRT function
/// `00ec6940` (`cdecl`, one `double`; result in `ST0`).
const CRT_SQRT_00EC6040: u32 = 0x00ec_6040;
const CRT_00EC6940: u32 = 0x00ec_6940;
/// `004dc060` (engine map: `fallout shared/renderer.cpp`, `cdecl`, one word).
const RENDERER_QUERY_004DC060: u32 = 0x004d_c060;

/// The constants of the generated textures, read from the exe as the code reads them:
/// the `double`s 64.0 (`010240c0`), -64.0 (`0106b9f0`), 4096.0 (`01017a10`), 0.5
/// (`01011588`), 255.0 (`0101e568`), 63.5 (`010ad890`) and 0.0 (`01012060`), and the
/// `float` 128.0 the loops count to (`0101e704`); the values are what the exe holds today.
const CUBE_HALF_SIZE_010240C0: u32 = 0x0102_40c0;
const CUBE_MINUS_HALF_SIZE_0106B9F0: u32 = 0x0106_b9f0;
const CUBE_DEPTH_SQUARED_01017A10: u32 = 0x0101_7a10;
const HALF_01011588: u32 = 0x0101_1588;
const BYTE_SCALE_0101E568: u32 = 0x0101_e568;
const ATTENUATION_CENTRE_010AD890: u32 = 0x010a_d890;
const ZERO_01012060: u32 = 0x0101_2060;
const LOOP_END_0101E704: u32 = 0x0101_e704;

/// The texture globals `00b55bb0` fills (each an `NiPointer`).
const TEXTURE_011F9510: u32 = 0x011f_9510;
const TEXTURE_011F9514: u32 = 0x011f_9514;
const TEXTURE_011F9518: u32 = 0x011f_9518;
const TEXTURE_011F951C: u32 = 0x011f_951c;
const TEXTURE_011F9520: u32 = 0x011f_9520;
const TEXTURE_011F9524: u32 = 0x011f_9524;
const TEXTURE_011F9528: u32 = 0x011f_9528;
const TEXTURE_011F952C: u32 = 0x011f_952c;
const TEXTURE_011F9530: u32 = 0x011f_9530;
const TEXTURE_011F9534: u32 = 0x011f_9534;
const TEXTURE_011F9538: u32 = 0x011f_9538;
const TEXTURE_011F953C: u32 = 0x011f_953c;
const TEXTURE_011F9540: u32 = 0x011f_9540;
const TEXTURE_011F9544: u32 = 0x011f_9544;
const TEXTURE_011F95D4: u32 = 0x011f_95d4;

/// `00b55bb0`'s stack block, by the offsets the exe uses after its prologue: the fixed
/// string whose address the cube maps and file textures get (+0x20), the format
/// preferences of the last texture (+0x44), the cube's pixel format (+0x6c, 0x44 bytes) and
/// the 0x104-byte file name buffer (+0xb4).
const FRAME_NAME: u32 = 0x20;
const FRAME_PREFS: u32 = 0x44;
const FRAME_PIXEL_FORMAT: u32 = 0x6c;
const FRAME_BUFFER: u32 = 0xb4;
const FRAME_SIZE: u32 = 0x1b8;

/// `FISTP` with the rounding control set to truncate (the code ORs 0xc00 into the control
/// word around it): toward zero, the integer indefinite 0x80000000 for NaN and values out
/// of range.
fn fistp_truncate(value: f64) -> i32 {
    let truncated = value.trunc();
    if truncated.is_nan() || truncated < -2_147_483_648.0 || truncated > 2_147_483_647.0 {
        i32::MIN
    } else {
        truncated as i32
    }
}

/// One colour byte of the cube: `(value + 1) * 0.5 * 255` truncated, the low byte kept;
/// `one_first` for the third byte, whose addition the code makes as `1 + value`.
fn cube_byte(value_bits: u32, one_first: bool, half: f64, scale: f64) -> u8 {
    let value = x87_load32(value_bits);
    let sum = if one_first {
        x87_add(1.0, value)
    } else {
        x87_add(value, 1.0)
    };
    fistp_truncate(x87_mul(x87_mul(sum, half), scale)) as u8
}

/// The constants one cube face needs.
struct CubeConstants {
    half_size: f64,
    minus_half_size: f64,
    depth_squared: f64,
    half: f64,
    scale: f64,
}

/// One 128x128 face of the normalization cube `00b55bb0` computes: for each row and column
/// (counted as floats to 128.0), the direction from the cube's centre to the texel, made
/// unit length (`1 / sqrt` of the squared length through `00ec6040`), each component mapped
/// to a byte, then a zero byte. The six faces differ in which offsets they use, the order
/// they add the squares in and where each component goes; each arm follows its loop in the
/// exe (`00b55c57`, `00b55df4`, `00b55fa4`, `00b56145`, `00b562f6`, `00b56493`).
fn normal_cube_face(e: &mut Engine, face: u32, mut pixel: u32, k: &CubeConstants) {
    let k64 = k.half_size;
    let mut row = 0i32;
    let mut row_bits = x87_store32(0.0);
    loop {
        let y = x87_load32(row_bits);
        // The row's offset, stored as a float once per row.
        let row_value = match face {
            2 => x87_store32(x87_sub(y, k64)),
            _ => x87_store32(x87_sub(k64, y)),
        };
        let mut column = 0i32;
        let mut column_bits = x87_store32(0.0);
        loop {
            let x = x87_load32(column_bits);
            let column_value = match face {
                0 | 5 => x87_store32(x87_sub(k64, x)),
                _ => x87_store32(x87_sub(x, k64)),
            };
            let row_copy = x87_quiet_bits(row_value);
            // (first, second): the offset the code loads first and the other.
            let (first, second) = match face {
                0 | 1 | 4 | 5 => (row_copy, column_value),
                _ => (column_value, row_copy),
            };
            let first_value = x87_load32(first);
            let second_value = x87_load32(second);
            let first_square = x87_mul(first_value, first_value);
            let second_square = x87_mul(second_value, second_value);
            let length_squared = if face >= 4 {
                x87_add(x87_add(first_square, second_square), k.depth_squared)
            } else {
                x87_add(second_square, x87_add(first_square, k.depth_squared))
            };
            let length_squared = x87_store32(length_squared);
            let length = e
                .call(CRT_SQRT_00EC6040, &args![x87_load32(length_squared)])
                .f64();
            let length = x87_load32(x87_store32(length));
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, length)));
            let depth = x87_store32(match face {
                1 | 3 => x87_mul(k.minus_half_size, reciprocal),
                5 => x87_mul(reciprocal, k.minus_half_size),
                _ => x87_mul(reciprocal, k64),
            });
            let first_scaled = x87_store32(x87_mul(reciprocal, first_value));
            let second_scaled = x87_store32(x87_mul(reciprocal, second_value));
            let bytes = match face {
                0 | 1 => [second_scaled, first_scaled, depth],
                2 | 3 => [second_scaled, depth, first_scaled],
                _ => [depth, first_scaled, second_scaled],
            };
            for (index, value) in bytes.iter().enumerate() {
                let byte = cube_byte(*value, index == 2, k.half, k.scale);
                e.mem.set_u8(pixel.wrapping_add(index as u32), byte);
            }
            e.mem.set_u8(pixel.wrapping_add(3), 0);
            pixel = pixel.wrapping_add(4);
            column = column.wrapping_add(1);
            column_bits = x87_store32(f64::from(column));
            if !(x87_load32(column_bits) < x87_m32(e.global::<u32>(LOOP_END_0101E704))) {
                break;
            }
        }
        row = row.wrapping_add(1);
        row_bits = x87_store32(f64::from(row));
        if !(x87_load32(row_bits) < x87_m32(e.global::<u32>(LOOP_END_0101E704))) {
            break;
        }
    }
}

/// The address of face `face` of a pixel data's first mipmap level: its pixels plus the
/// level's offset plus `face` times the offset after the last level (face 0 reads neither
/// the level count nor that offset).
fn cube_face_pixels(e: &Engine, pixel_data: u32, face: u32) -> u32 {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let first = e.mem.u32(offsets);
    let pixels = e.mem.u32(pixel_data.wrapping_add(PIXELS));
    if face == 0 {
        return first.wrapping_add(pixels);
    }
    let levels = e.mem.u32(pixel_data.wrapping_add(MIPMAP_LEVELS));
    let face_size = e.mem.u32(offsets.wrapping_add(levels.wrapping_mul(4)));
    pixels
        .wrapping_add(first)
        .wrapping_add(face_size.wrapping_mul(face))
}

/// A new pixel data (`00aa13e0(0x74)`, then its constructor with one level and one face
/// when the allocation succeeded) and its reference count incremented when not null.
fn new_pixel_data(e: &mut Engine, width: u32, height: u32, format: u32) -> u32 {
    let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
    let pixel_data = if memory != 0 {
        e.call(
            NI_PIXEL_DATA_CONSTRUCT,
            &args![memory, width, height, format, 1u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    if pixel_data != 0 {
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![pixel_data.wrapping_add(4)],
        );
    }
    pixel_data
}

/// Fills `width * height` pixels from the first level's start with `pattern`.
fn fill_pixels(e: &mut Engine, pixel_data: u32, width: u32, height: u32, pattern: &[u8]) {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let mut pixel = e
        .mem
        .u32(offsets)
        .wrapping_add(e.mem.u32(pixel_data.wrapping_add(PIXELS)));
    for _ in 0..height {
        for _ in 0..width {
            for byte in pattern {
                e.mem.set_u8(pixel, *byte);
                pixel = pixel.wrapping_add(1);
            }
        }
    }
}

/// `bNoConvert` set and `m_uiRevID` incremented, in the order the code makes them.
fn mark_pixels_changed(e: &mut Engine, pixel_data: u32, revision_first: bool) {
    let revision = pixel_data.wrapping_add(REVISION);
    if revision_first {
        let value = e.mem.u32(revision).wrapping_add(1);
        e.mem.set_u32(revision, value);
        e.mem.set_u8(pixel_data.wrapping_add(NO_CONVERT), 1);
    } else {
        e.mem.set_u8(pixel_data.wrapping_add(NO_CONVERT), 1);
        let value = e.mem.u32(revision).wrapping_add(1);
        e.mem.set_u32(revision, value);
    }
}

/// The inlined `NiPointer` assignment of a global: nothing when the same, else the old one
/// released, the new one stored and its count incremented when not null.
fn assign_global(e: &mut Engine, global: u32, object: u32) {
    let old = e.global::<u32>(global);
    if old != object {
        if old != 0 {
            release_ni_pointer(e, old);
        }
        e.set_global(global, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
}

/// The inlined `NiFixedString` release: unless it is the word at `0109b220`, the count 8
/// bytes before the text is decremented.
fn release_fixed_string(e: &mut Engine, name: u32) {
    if name != e.global::<u32>(FIXED_STRING_NONE) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
}

/// A source texture named `text` made from `pixel_data` with the format preferences at
/// `prefs` (`00a5ff10`), assigned to `global`, the name released.
fn texture_from_pixels(e: &mut Engine, pixel_data: u32, text: u32, prefs: u32, global: u32) {
    let name = e.call(FIXED_STRING_MAKE, &args![text]).u32();
    let texture = e
        .call(SOURCE_TEXTURE_FROM_PIXELS, &args![pixel_data, name, prefs])
        .u32();
    assign_global(e, global, texture);
    release_fixed_string(e, name);
}

/// A filled solid texture: new pixel data, `pattern` in every pixel, marked changed, made
/// into a texture for `global` with the default preferences. Answers the pixel data.
fn solid_texture(
    e: &mut Engine,
    size: u32,
    format: u32,
    pattern: &[u8],
    revision_first: bool,
    text: u32,
    global: u32,
) -> u32 {
    let pixel_data = new_pixel_data(e, size, size, format);
    fill_pixels(e, pixel_data, size, size, pattern);
    mark_pixels_changed(e, pixel_data, revision_first);
    texture_from_pixels(e, pixel_data, text, FORMAT_PREFS_011A9598, global);
    pixel_data
}

/// Formats a file name of `Data\Textures\Effects\` (`010ad924`) into the frame's 0x104-byte
/// buffer with `format` (`00406d00`).
fn format_effects_path(e: &mut Engine, frame: u32, format: u32) {
    e.call(
        FORMAT_00406D00,
        &args![
            frame.wrapping_add(FRAME_BUFFER),
            0x104u32,
            format,
            EFFECTS_FOLDER_010AD924
        ],
    );
}

/// `"Data\\Textures\\Effects\\"` (`010ad924`).
const EFFECTS_FOLDER_010AD924: u32 = 0x010a_d924;

// Translated from 00b55bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsshadermanager.cpp`); no arguments. Makes the shader
/// manager's built-in textures, in the exe's order:
///
/// * a 128x128 six-face pixel data in the pixel format copied from `011aaac8` (revision
///   incremented at once) filled with a normalization cube (see `normal_cube_face`); it is
///   only released at the end;
/// * 16x16 solid textures: `BSShader_DefErrorMap` (ff 00 00 ff) into `011f9514`,
///   `BSShader_DefNormalMap` (80 80 ff, three-byte format) into `011f9510`,
///   `BSShader_DefNormalGlossMap` (80 80 ff 40) into `011f9518`, and `BSShader_DefHeightMap`
///   (00 00 00) into `011f951c`, whose pixel data replaces the normal map's in the same
///   `NiPointer` (the old one released without a null check, as the exe does);
/// * `BSShaderManager::GetTexture` (`00b55840`) of the two window environment maps into
///   `011f9520` and `011f9524`;
/// * 4x4 solid textures `BSShader_HideSecretMap` (00 00 00 40) into `011f9528` and
///   `BSShader_DefTexEffectMap` (00 00 00 ff) into `011f952c`;
/// * the cube maps `EyeReflectionCubeMap` (`%seyereflection.dds`) into `011f9544` and
///   `ArmorReflectionCubeMap` (`%sreflection.dds`) into `011f95d4` (`00a61120`, the same
///   file for all six faces), and the file textures `%saniso.dds` into `011f9530` and
///   `%sNoiseMap.dds` into `011f953c` (`00a5fd70`), all in `Data\Textures\Effects\`;
/// * the 4x4 `BSShader_EmptyNoiseMap` (8f 8f 8f ff) into `011f9540`;
/// * the 128x128 three-byte `BSShader_AttenuationXYZ` into `011f9534` and `011f9538`: for
///   each texel the distances of its column and row from 63.5 as fractions of 63.5 (`1 - v`
///   below the centre), the sum of their squares capped at 1 and raised to 0, inverted when
///   `004dc060(0)` answers 1, scaled by 255, passed through `00ec6940` and truncated into
///   all three bytes; its format preferences are a stack copy of (2, 0, 0).
///
/// Then the pixel data are released in reverse order (no null checks, as the exe).
// Translation incomplete: 00b55bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 0x1c) and the /GS cookie check (00ec408c) are not translated.
pub fn fn_00b55bb0(e: &mut Engine) {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        for word in 0..0x11u32 {
            let value = e.global::<u32>(CUBE_PIXEL_FORMAT_011AAAC8 + 4 * word);
            e.mem
                .set_u32(frame.wrapping_add(FRAME_PIXEL_FORMAT + 4 * word), value);
        }
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let cube = if memory != 0 {
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![
                    memory,
                    0x80u32,
                    0x80u32,
                    frame.wrapping_add(FRAME_PIXEL_FORMAT),
                    1u32,
                    6u32
                ],
            )
            .u32()
        } else {
            0
        };
        if cube != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![cube.wrapping_add(4)]);
        }
        let revision = e.mem.u32(cube.wrapping_add(REVISION)).wrapping_add(1);
        e.mem.set_u32(cube.wrapping_add(REVISION), revision);
        let constants = CubeConstants {
            half_size: e.global::<f64>(CUBE_HALF_SIZE_010240C0),
            minus_half_size: e.global::<f64>(CUBE_MINUS_HALF_SIZE_0106B9F0),
            depth_squared: e.global::<f64>(CUBE_DEPTH_SQUARED_01017A10),
            half: e.global::<f64>(HALF_01011588),
            scale: e.global::<f64>(BYTE_SCALE_0101E568),
        };
        for face in 0..6u32 {
            let pixels = cube_face_pixels(e, cube, face);
            normal_cube_face(e, face, pixels, &constants);
        }

        let prefs = frame.wrapping_add(FRAME_PREFS);
        e.mem.set_u32(prefs, 2);
        e.mem.set_u32(prefs.wrapping_add(8), 0);
        e.mem.set_u32(prefs.wrapping_add(4), 0);

        let error_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2A0,
            &[0xff, 0, 0, 0xff],
            false,
            0x010a_da24, // "BSShader_DefErrorMap"
            TEXTURE_011F9514,
        );
        let normal_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2E8,
            &[0x80, 0x80, 0xff],
            false,
            0x010a_da0c, // "BSShader_DefNormalMap"
            TEXTURE_011F9510,
        );
        let gloss_map = solid_texture(
            e,
            0x10,
            PIXEL_FORMAT_011AA2A0,
            &[0x80, 0x80, 0xff, 0x40],
            false,
            0x010a_d9f0, // "BSShader_DefNormalGlossMap"
            TEXTURE_011F9518,
        );

        // The height map's pixel data goes into the normal map's `NiPointer`.
        let memory = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let made = if memory != 0 {
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![memory, 0x10u32, 0x10u32, PIXEL_FORMAT_011AA2E8, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let held = if normal_map != made {
            release_ni_pointer(e, normal_map);
            if made != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![made.wrapping_add(4)]);
            }
            made
        } else {
            normal_map
        };
        fill_pixels(e, held, 0x10, 0x10, &[0, 0, 0]);
        mark_pixels_changed(e, held, false);
        texture_from_pixels(
            e,
            held,
            0x010a_d9d8, // "BSShader_DefHeightMap"
            FORMAT_PREFS_011A9598,
            TEXTURE_011F951C,
        );

        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![
                0x010a_d9a4u32, // "Data\\Textures\\Effects\\WindowEnvironmentMap01.dds"
                1u32,
                TEXTURE_011F9520,
                1u32,
                0u32
            ],
        );
        e.call(
            BS_SHADER_MANAGER_GET_TEXTURE,
            &args![
                0x010a_d970u32, // "Data\\Textures\\Effects\\WindowEnvironmentMap02.dds"
                1u32,
                TEXTURE_011F9524,
                1u32,
                0u32
            ],
        );

        let hide_secret_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0, 0, 0, 0x40],
            true,
            0x010a_d958, // "BSShader_HideSecretMap"
            TEXTURE_011F9528,
        );
        let effect_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0, 0, 0, 0xff],
            false,
            0x010a_d93c, // "BSShader_DefTexEffectMap"
            TEXTURE_011F952C,
        );

        let name_slot = frame.wrapping_add(FRAME_NAME);
        let buffer = frame.wrapping_add(FRAME_BUFFER);
        for (format, text, global) in [
            // "%seyereflection.dds", "EyeReflectionCubeMap"
            (0x010a_d910u32, 0x010a_d8f8u32, TEXTURE_011F9544),
            // "%sreflection.dds", "ArmorReflectionCubeMap"
            (0x010a_d8e4, 0x010a_d8cc, TEXTURE_011F95D4),
        ] {
            format_effects_path(e, frame, format);
            let name = e.call(FIXED_STRING_MAKE, &args![text]).u32();
            e.mem.set_u32(name_slot, name);
            let renderer = e.global::<u32>(GLOBAL_011F9508);
            let cube_map = e
                .call(
                    SOURCE_CUBE_MAP_CREATE,
                    &args![
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        buffer,
                        renderer,
                        name_slot,
                        FORMAT_PREFS_011A9598
                    ],
                )
                .u32();
            assign_global(e, global, cube_map);
            let name = e.mem.u32(name_slot);
            release_fixed_string(e, name);
        }
        for (format, global) in [
            // "%saniso.dds"
            (0x010a_d8c0u32, TEXTURE_011F9530),
            // "%sNoiseMap.dds"
            (0x010a_d8b0, TEXTURE_011F953C),
        ] {
            format_effects_path(e, frame, format);
            let name = e.call(FIXED_STRING_MAKE, &args![buffer]).u32();
            e.mem.set_u32(name_slot, name);
            let texture = e
                .call(
                    SOURCE_TEXTURE_FROM_FILE,
                    &args![name_slot, FORMAT_PREFS_011A9598, 1u32, 0u32],
                )
                .u32();
            assign_global(e, global, texture);
            let name = e.mem.u32(name_slot);
            release_fixed_string(e, name);
        }

        let empty_noise_map = solid_texture(
            e,
            4,
            PIXEL_FORMAT_011AA2A0,
            &[0x8f, 0x8f, 0x8f, 0xff],
            false,
            0x010a_d898, // "BSShader_EmptyNoiseMap"
            TEXTURE_011F9540,
        );

        let attenuation = new_pixel_data(e, 0x80, 0x80, PIXEL_FORMAT_011AA2E8);
        attenuation_pixels(e, attenuation);
        e.mem.set_u32(prefs, 2);
        e.mem.set_u32(prefs.wrapping_add(8), 0);
        e.mem.set_u32(prefs.wrapping_add(4), 0);
        mark_pixels_changed(e, attenuation, false);
        texture_from_pixels(
            e,
            attenuation,
            0x010a_d874, // "BSShader_AttenuationXYZ"
            prefs,
            TEXTURE_011F9534,
        );
        let texture = e.global::<u32>(TEXTURE_011F9534);
        assign_global(e, TEXTURE_011F9538, texture);

        for pixel_data in [
            attenuation,
            empty_noise_map,
            effect_map,
            hide_secret_map,
            gloss_map,
            held,
            error_map,
            cube,
        ] {
            release_ni_pointer(e, pixel_data);
        }
    });
}

/// The pixels of `BSShader_AttenuationXYZ` (`00b570d2` to `00b5721a`): 128 rows of 128
/// three-byte texels, each byte the same value.
fn attenuation_pixels(e: &mut Engine, pixel_data: u32) {
    let offsets = e.mem.u32(pixel_data.wrapping_add(OFFSETS_IN_BYTES));
    let mut pixel = e
        .mem
        .u32(offsets)
        .wrapping_add(e.mem.u32(pixel_data.wrapping_add(PIXELS)));
    // The fraction of 63.5 a coordinate lies from the centre: `1 - v / c` below it, else
    // `(v - c) / c` (also for a NaN comparison).
    let distance = |value: f64, centre: f64| -> u32 {
        if value < centre {
            x87_store32(x87_sub(1.0, x87_div(value, centre)))
        } else {
            x87_store32(x87_div(x87_sub(value, centre), centre))
        }
    };
    let mut row = 0i32;
    loop {
        let row_bits = x87_store32(f64::from(row));
        let mut column = 0i32;
        loop {
            // The centre is loaded again for every texel.
            let centre = e.global::<f64>(ATTENUATION_CENTRE_010AD890);
            let column_bits = x87_store32(f64::from(column));
            let across = distance(x87_load32(column_bits), centre);
            let down = distance(x87_load32(row_bits), centre);
            let across = x87_load32(across);
            let down = x87_load32(down);
            let sum = x87_add(x87_mul(down, down), x87_mul(across, across));
            let sum = x87_load32(x87_store32(sum));
            // Capped at 1 when above it (ordered), raised to 0 when below 0.
            let capped = if sum > 1.0 { 1.0 } else { sum };
            let capped = x87_load32(x87_store32(capped));
            let zero = e.global::<f64>(ZERO_01012060);
            let floored = if capped < zero { 0.0 } else { capped };
            let stored = x87_store32(floored);
            let inverted = e.call(RENDERER_QUERY_004DC060, &args![0u32]).u32();
            let value = x87_load32(stored);
            let value = if inverted == 1 {
                x87_sub(1.0, value)
            } else {
                value
            };
            let value = x87_store32(value);
            let scale = e.global::<f64>(BYTE_SCALE_0101E568);
            let scaled = x87_store32(x87_mul(x87_load32(value), scale));
            let result = e.call(CRT_00EC6940, &args![x87_load32(scaled)]).f64();
            let result = x87_load32(x87_store32(result));
            let byte = fistp_truncate(result) as u8;
            for index in 0..3u32 {
                e.mem.set_u8(pixel.wrapping_add(index), byte);
            }
            pixel = pixel.wrapping_add(3);
            column += 1;
            if column >= 0x80 {
                break;
            }
        }
        row += 1;
        if row >= 0x80 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_5bb0, fn_00b55bb0()),
        entry!(0x00b5_5560, bs_shader_manager_get_shader(u32) -> Ptr),
        entry!(0x00b4_f1a0, bs_shader_manager_register_streamables()),
        entry!(0x00b5_4110, shadow_scene_node_set_fog_property(Ptr, Ptr)),
        entry!(0x00b5_4000, fn_00b54000(f32)),
        entry!(0x0070_b760, fn_0070b760(Ptr, u8)),
        entry!(0x00b4_f2f0, fn_00b4f2f0(u32, Ptr)),
        entry!(0x00b4_f5c0, bs_shader_manager_get_accumulator() -> Ptr),
        entry!(0x00b5_5540, bs_shader_manager_set_fog_property(u32, Ptr)),
        entry!(0x00b5_7420, fn_00b57420()),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(FIRST_FACTOR, 8);
        e.map(SECOND_FACTOR, 8);
        e.map(REFERENCE_ANGLE, 8);
        e.map(RATIO_RESULT, 4);
        e.set_global(FIRST_FACTOR, 0.5f64);
        e.set_global(SECOND_FACTOR, 2.0f64);
        e.set_global(REFERENCE_ANGLE, 3.0f64);
        // sin(x) = x + 1, cos(x) = 2: the quotients are exact in binary.
        e.register(CRT_SIN, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: x + 1.0,
                ..Ret::default()
            }
        });
        e.register(CRT_COS, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e
    }

    #[test]
    fn at_00b54000_divides_tan_of_the_scaled_angle_by_tan_of_the_reference() {
        let mut e = rig();
        // 0.5 * 0.5 * 2.0 = 0.5; (0.5 + 1) / 2 = 0.75; (3 + 1) / 2 = 2; 0.75 / 2 = 0.375.
        e.call(0x00b5_4000, &args![0.5f32]);
        assert_eq!(e.global::<f32>(RATIO_RESULT), 0.375);
        let log = e.call_log.take().unwrap();
        let sines: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CRT_SIN)
            .map(|c| &c.1)
            .collect();
        let half = 0.5f64.to_bits();
        let three = 3.0f64.to_bits();
        assert_eq!(
            sines,
            vec![
                &vec![half as u32, (half >> 32) as u32],
                &vec![three as u32, (three >> 32) as u32]
            ]
        );
    }

    #[test]
    fn at_00b54000_rounds_each_stage_to_a_float() {
        let mut e = rig();
        // The scaled angle (0.2 * 0.5 * 1.0) is a float before sin sees it.
        e.set_global(SECOND_FACTOR, 1.0f64);
        e.call(0x00b5_4000, &args![0.2f32]);
        let scaled = f64::from(0.2f32 / 2.0);
        let sine = f64::from((scaled + 1.0) as f32);
        let quotient = f64::from((sine / 2.0) as f32);
        let expected = (quotient / 2.0) as f32;
        assert_eq!(e.global::<f32>(RATIO_RESULT), expected);
    }

    #[test]
    fn at_00b54000_gives_a_nan_for_a_nan_argument() {
        let mut e = rig();
        e.call(0x00b5_4000, &args![f32::from_bits(0x7f80_0001)]);
        assert!(e.global::<f32>(RATIO_RESULT).is_nan());
    }
}

#[cfg(test)]
mod b000_tests {
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

    #[test]
    fn fn_0070b760_stores_the_index_and_fills_the_table() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        let node = e.mem.alloc(0x130);
        e.call(0x0070_b760, &args![node, 0xffff_ff03u32]);
        assert_eq!(e.mem.u8(node + 0x120), 3);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 12), node);
    }

    #[test]
    fn fn_00b4f2f0_stores_the_node_at_its_index() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        e.call(0x00b4_f2f0, &args![2u32, 0x1234_5678u32]);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 8), 0x1234_5678);
        assert_eq!(e.mem.u32(SHADOW_SCENE_NODES + 4), 0);
    }

    #[test]
    fn get_accumulator_reads_its_global() {
        let mut e = Engine::new();
        e.map(ACCUMULATOR, 4);
        e.set_global(ACCUMULATOR, 0x0bad_f00du32);
        assert_eq!(e.call(0x00b4_f5c0, &args![]).u32(), 0x0bad_f00d);
    }

    #[test]
    fn set_fog_property_calls_the_node_of_the_index() {
        let mut e = Engine::new();
        e.map(SHADOW_SCENE_NODES, 0x40);
        e.mem.set_u32(SHADOW_SCENE_NODES + 4, 0x5000);
        e.register(SHADOW_SCENE_NODE_SET_FOG_PROPERTY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b5_5540, &args![1u32, 0x6000u32]);
        assert_eq!(
            calls(&e, SHADOW_SCENE_NODE_SET_FOG_PROPERTY),
            vec![vec![0x5000, 0x6000]]
        );
    }

    /// Doubles for every callee of `00b57420`; constructors answer `this`. The vector
    /// constructor fills the slots with `old` (0 for none).
    fn set_up_rig(old: u32) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_6000,
            0x010a_d000,
            0x011a_9000,
            0x011f_9000,
            0x011f_a000,
            0x011f_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1].max(8)).into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        for this_returning in [0x00b6_dd80u32, 0x00b9_7a80, 0x00b6_60d0] {
            e.register(this_returning, |_, a| a[0].into_ret());
        }
        // `00b7ff40` keeps `this`, with its word at +4 as 1 (kept by the first check).
        e.register(0x00b7_ff40, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            a[0].into_ret()
        });
        e.register(0x00b7_faf0, |_, _| 7u32.into_ret());
        e.register(0x00a5_b690, |_, _| 0x0777_0000u32.into_ret());
        for plain in [
            0x00b7_fe40u32,
            0x00b7_fd50,
            0x00b5_5bb0,
            0x00b5_5560,
            0x00b4_f1a0,
            0x00b6_e450,
            0x00ba_3720,
            0x00b9_7c10,
        ] {
            e.register(plain, |_, _| Ret::default());
        }
        e.register_double(0x00ec_782f, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u32(a[0] + 4 * i, old);
            }
            Ret::default()
        });
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
        e
    }

    #[test]
    fn fn_00b57420_first_run_builds_the_manager_and_the_accumulators() {
        let mut e = set_up_rig(0);
        e.set_global(GLOBAL_011F9508, 1u32);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 2u32);
        e.set_global(GLOBAL_011F94F0, 0x4444u32);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register_double(0x00b5_5bb0, move |e, _| {
            // The context is 5 while the set-up runs.
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);

        let shaders: Vec<u32> = calls(&e, 0x00b5_5560).iter().map(|w| w[0]).collect();
        assert_eq!(shaders, (1..0x23).collect::<Vec<u32>>());
        // Kept by the first check, released after the shaders.
        assert_eq!(calls(&e, 0x00b7_fd50).len(), 1);
        assert_eq!(e.global::<u32>(OBJECT_011F9498), 0);
        assert_ne!(e.global::<u32>(OBJECT_011F91A8), 0);
        assert_ne!(e.global::<u32>(OBJECT_011F91AC), 0);
        let first = e.global::<u32>(OBJECT_011F91A8);
        assert_eq!(calls(&e, 0x00b6_e450), vec![vec![first, 1]]);
        assert_eq!(calls(&e, 0x00a5_b690), vec![vec![HOLDER_NAME_010ADA3C]]);
        let holder = e.global::<u32>(HOLDER_011F94BC);
        assert_eq!(e.mem.u32(holder), 0x0777_0000);
        assert_eq!(e.global::<u8>(SET_UP_DONE_011F91C6), 1);
        for (i, want) in [0.5f32, 0.5, 0.5, 1.0].iter().enumerate() {
            assert_eq!(e.global::<f32>(FLOATS_011FA090 + 4 * i as u32), *want);
        }
        for (word, code) in FUNCTION_WORDS_00B57420 {
            assert_eq!(e.global::<u32>(word), code);
        }
        assert_eq!(e.global::<u32>(0x011f_9f44), 0);
        assert_eq!(e.global::<u32>(GLOBAL_011A9608), 0x4444);

        // Two accumulators, each held once, after the count word.
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_eq!(e.mem.u32(array - 4), 2);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE).iter().map(|w| w[1]).collect();
        assert_eq!(sizes, vec![0xb0, 0x1c, 0x31c, 4, 12]);
        let vector = calls(&e, 0x00ec_782f);
        assert_eq!(vector, vec![vec![array, 4, 2, 0x00b5_4620, 0x0045_cec0]]);
        let accumulators = calls(&e, 0x00b6_60d0);
        assert_eq!(accumulators.len(), 2);
        assert_eq!(&accumulators[0][1..], &[0x63, 1, 0x2f7]);
        for (i, call) in accumulators.iter().enumerate() {
            let accumulator = e.mem.u32(array + 4 * i as u32);
            assert_eq!(accumulator, call[0]);
            assert_eq!(e.mem.u32(accumulator + 4), 1);
        }
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn fn_00b57420_first_run_without_shaders_releases_an_empty_object_at_once() {
        let mut e = set_up_rig(0);
        e.register(0x00b7_ff40, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);
        assert!(calls(&e, 0x00b5_5560).is_empty());
        assert!(calls(&e, 0x00b9_7a80).is_empty());
        assert_eq!(calls(&e, 0x00b7_fd50).len(), 1);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
        assert_eq!(e.global::<u32>(OBJECT_011F9498), 0);
        // No accumulators: the array has only its count word.
        assert!(calls(&e, NEW_OBJECT).is_empty());
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_eq!(e.mem.u32(array - 4), 0);
    }

    #[test]
    fn fn_00b57420_later_runs_skip_the_set_up_and_release_old_slots() {
        let mut e = set_up_rig(0);
        let old = e.mem.alloc(8);
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        e.mem.set_u32(old, 0x0f00_1000);
        e.mem.set_u32(old + 4, 1);
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.register_double(0x00ec_782f, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u32(a[0] + 4 * i, old);
            }
            Ret::default()
        });
        e.set_global(SET_UP_DONE_011F91C6, 1u8);
        e.set_global(ACCUMULATOR_COUNT_011F916C, 1u32);
        e.call_log = Some(vec![]);
        e.call(0x00b5_7420, &args![]);
        assert!(calls(&e, 0x00b6_dd80).is_empty());
        assert!(calls(&e, 0x00b4_f1a0).is_empty());
        // The old pointer's count went to 0: deleted through its vtable +4.
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![old]]);
        let array = e.global::<u32>(ACCUMULATOR_ARRAY_011F9170);
        assert_ne!(e.mem.u32(array), old);
        assert_eq!(calls(&e, 0x00ba_3720).len(), 1);
        assert_eq!(e.global::<u32>(GLOBAL_011F9168), 0);
    }
}

#[cfg(test)]
mod b000c_tests {
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

    /// The renderer word set, each creator answering `0x5000 + index`, and the assignment
    /// storing into the slot (as the `NiPointer` assignment does).
    fn shader_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(GLOBAL_011F9508, 1u32);
        for index in 0..=LAST_SHADER_INDEX {
            if let Some(create) = shader_creator(index) {
                e.register_double(create, move |_, _| (0x5000 + index).into_ret());
            }
        }
        e.register(SHADER_4_CREATE, |_, _| 0x5004u32.into_ret());
        e.register(SHADER_1D_CREATE, |_, _| 0x501du32.into_ret());
        e.register(NI_POINTER_ASSIGN_00A5F860, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e
    }

    #[test]
    fn get_shader_makes_a_missing_shader_once_with_the_context_at_5() {
        let mut e = shader_rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.register_double(0x00b7_a380, move |e, _| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 5);
            0x5001u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0x5001);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0x5001);
        assert_eq!(calls(&e, 0x00b7_a380).len(), 1);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_00A5F860),
            vec![vec![SHADER_TABLE_011F9548 + 4, 0x5001]]
        );
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
    }

    #[test]
    fn get_shader_calls_each_index_creator() {
        let mut e = shader_rig();
        for index in [
            2u32, 3, 5, 6, 7, 8, 9, 0xa, 0xd, 0xe, 0xf, 0x10, 0x11, 0x18, 0x19, 0x1a, 0x1e, 0x20,
            0x21, 0x22,
        ] {
            assert_eq!(e.call(0x00b5_5560, &args![index]).u32(), 0x5000 + index);
        }
        assert_eq!(shader_creator(7), Some(0x00b8_0e70));
        assert_eq!(shader_creator(0x22), Some(0x00bc_59b0));
    }

    #[test]
    fn get_shader_4_makes_shader_1_first() {
        let mut e = shader_rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![4u32]).u32(), 0x5004);
        assert_eq!(e.mem.u32(SHADER_TABLE_011F9548 + 4), 0x5001);
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_00A5F860),
            vec![
                vec![SHADER_TABLE_011F9548 + 4, 0x5001],
                vec![SHADER_TABLE_011F9548 + 16, 0x5004]
            ]
        );
    }

    #[test]
    fn get_shader_without_a_creator_or_out_of_range_answers_0() {
        let mut e = shader_rig();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 9);
        e.call_log = Some(vec![]);
        for index in [
            0u32,
            0xb,
            0xc,
            0x12,
            0x17,
            0x1b,
            0x1c,
            0x1f,
            0x23,
            0xffff_ffff,
        ] {
            assert_eq!(e.call(0x00b5_5560, &args![index]).u32(), 0);
        }
        assert!(calls(&e, NI_POINTER_ASSIGN_00A5F860).is_empty());
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 9);
        // Without the renderer word nothing is looked at.
        e.mem.set_u32(SHADER_TABLE_011F9548 + 4, 0x7777);
        e.set_global(GLOBAL_011F9508, 0u32);
        assert_eq!(e.call(0x00b5_5560, &args![1u32]).u32(), 0);
    }

    #[test]
    fn get_shader_1d_depends_on_its_byte() {
        let mut e = shader_rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b5_5560, &args![0x1du32]).u32(), 0);
        assert!(calls(&e, SHADER_1D_CREATE).is_empty());
        e.set_global(SHADER_1D_SELECT_011F94A6, 1u8);
        assert_eq!(e.call(0x00b5_5560, &args![0x1du32]).u32(), 0x501d);
        assert_eq!(calls(&e, SHADER_1D_CREATE).len(), 1);
    }

    const PIXEL_DATA_VTABLE: u32 = 0x0f00_7000;
    const PIXEL_DATA_DELETE: u32 = 0x0f00_7004;
    const TEXTURE_VTABLE: u32 = 0x0f00_8000;
    const TEXTURE_DELETE: u32 = 0x0f00_8004;

    /// An object of `vtable` with reference count 0.
    fn object(e: &mut Engine, vtable: u32, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The exe's constants, a pixel data constructor laying out `faces` faces of four-byte
    /// texels, a real square root, `00ec6940` answering its argument, and `004dc060`
    /// answering `inverted`.
    fn textures_rig(inverted: u32) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_7000,
            0x0101_e000,
            0x0102_4000,
            0x0106_b000,
            0x0109_b000,
            0x010a_d000,
            0x011a_a000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(CUBE_HALF_SIZE_010240C0, 64.0f64);
        e.set_global(CUBE_MINUS_HALF_SIZE_0106B9F0, -64.0f64);
        e.set_global(CUBE_DEPTH_SQUARED_01017A10, 4096.0f64);
        e.set_global(HALF_01011588, 0.5f64);
        e.set_global(BYTE_SCALE_0101E568, 255.0f64);
        e.set_global(ATTENUATION_CENTRE_010AD890, 63.5f64);
        e.set_global(ZERO_01012060, 0.0f64);
        e.set_global(LOOP_END_0101E704, 128.0f32);
        e.set_global(GLOBAL_011F9508, 0x0777_0000u32);
        for word in 0..0x11u32 {
            e.set_global(CUBE_PIXEL_FORMAT_011AAAC8 + 4 * word, 0x100 + word);
        }
        counting(&mut e);
        e.put_vtable(PIXEL_DATA_VTABLE, &[0, PIXEL_DATA_DELETE]);
        e.put_vtable(TEXTURE_VTABLE, &[0, TEXTURE_DELETE]);
        e.register(PIXEL_DATA_DELETE, |_, _| Ret::default());
        e.register(TEXTURE_DELETE, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |e, a| {
            let (this, face) = (a[0], a[1] * a[2] * 4);
            let pixels = e.mem.alloc(face * a[5]);
            let offsets = e.mem.alloc(8);
            e.mem.set_u32(offsets + 4, face);
            e.mem.set_u32(this, PIXEL_DATA_VTABLE);
            e.mem.set_u32(this + PIXELS, pixels);
            e.mem.set_u32(this + OFFSETS_IN_BYTES, offsets);
            e.mem.set_u32(this + MIPMAP_LEVELS, a[4]);
            this.into_ret()
        });
        e.register(CRT_SQRT_00EC6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(CRT_00EC6940, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32),
            ..Ret::default()
        });
        e.register_double(RENDERER_QUERY_004DC060, move |_, _| inverted.into_ret());
        // A fixed string: its text 8 bytes after a count of 1.
        e.register(FIXED_STRING_MAKE, |e, _| {
            let block = e.mem.alloc(16);
            e.mem.set_u32(block, 1);
            (block + 8).into_ret()
        });
        for maker in [
            SOURCE_TEXTURE_FROM_PIXELS,
            SOURCE_TEXTURE_FROM_FILE,
            SOURCE_CUBE_MAP_CREATE,
        ] {
            e.register(maker, |e, _| object(e, TEXTURE_VTABLE, 8).into_ret());
        }
        e.register(BS_SHADER_MANAGER_GET_TEXTURE, |_, _| Ret::default());
        e.register(FORMAT_00406D00, |_, _| Ret::default());
        e
    }

    fn texel(e: &Engine, pixel_data: u32, face: u32, column: u32, row: u32) -> [u8; 4] {
        texel_of(e, pixel_data, 128, face, column, row)
    }

    fn texel_of(
        e: &Engine,
        pixel_data: u32,
        width: u32,
        face: u32,
        column: u32,
        row: u32,
    ) -> [u8; 4] {
        let offsets = e.mem.u32(pixel_data + OFFSETS_IN_BYTES);
        let base = e.mem.u32(pixel_data + PIXELS) + face * e.mem.u32(offsets + 4);
        let at = base + (row * width + column) * 4;
        [
            e.mem.u8(at),
            e.mem.u8(at + 1),
            e.mem.u8(at + 2),
            e.mem.u8(at + 3),
        ]
    }

    #[test]
    fn fn_00b55bb0_makes_every_built_in_texture() {
        let mut e = textures_rig(0);
        // The last texture gets the stack's preferences (2, 0, 0).
        e.register(SOURCE_TEXTURE_FROM_PIXELS, |e, a| {
            if a[2] != FORMAT_PREFS_011A9598 {
                assert_eq!(e.mem.bytes(a[2], 12), [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            }
            object(e, TEXTURE_VTABLE, 8).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_5bb0, &args![]);

        let constructed = calls(&e, NI_PIXEL_DATA_CONSTRUCT);
        let shapes: Vec<&[u32]> = constructed.iter().map(|c| &c[1..]).collect();
        assert_eq!(shapes.len(), 9);
        assert_eq!(shapes[0][..2], [0x80, 0x80]);
        assert_eq!(shapes[0][3..], [1, 6]);
        let cube = constructed[0][0];
        // The pixel format was copied onto the stack.
        assert_eq!(e.mem.u32(shapes[0][2]), 0x100);
        assert_eq!(shapes[1], &[0x10, 0x10, PIXEL_FORMAT_011AA2A0, 1, 1]);
        assert_eq!(shapes[4], &[0x10, 0x10, PIXEL_FORMAT_011AA2E8, 1, 1]);
        assert_eq!(shapes[5], &[4, 4, PIXEL_FORMAT_011AA2A0, 1, 1]);
        assert_eq!(shapes[8], &[0x80, 0x80, PIXEL_FORMAT_011AA2E8, 1, 1]);

        // The normalization cube: the centre of each face points along its axis.
        assert_eq!(texel(&e, cube, 0, 64, 64), [127, 127, 255, 0]);
        assert_eq!(texel(&e, cube, 1, 64, 64), [127, 127, 0, 0]);
        assert_eq!(texel(&e, cube, 2, 64, 64), [127, 255, 127, 0]);
        assert_eq!(texel(&e, cube, 3, 64, 64), [127, 0, 127, 0]);
        assert_eq!(texel(&e, cube, 4, 64, 64), [255, 127, 127, 0]);
        assert_eq!(texel(&e, cube, 5, 64, 64), [0, 127, 127, 0]);
        // A corner: every component 1/sqrt(3) in size.
        assert_eq!(texel(&e, cube, 0, 0, 0), [201, 201, 201, 0]);
        assert_eq!(texel(&e, cube, 1, 0, 0), [53, 201, 53, 0]);
        assert_eq!(texel(&e, cube, 2, 0, 0), [53, 201, 53, 0]);
        assert_eq!(e.mem.u32(cube + REVISION), 1);

        // The solid textures.
        let error_map = constructed[1][0];
        assert_eq!(texel_of(&e, error_map, 16, 0, 15, 15), [0xff, 0, 0, 0xff]);
        assert_eq!(e.mem.u8(error_map + NO_CONVERT), 1);
        assert_eq!(e.mem.u32(error_map + REVISION), 1);
        let effect_map = constructed[6][0];
        assert_eq!(texel_of(&e, effect_map, 4, 0, 3, 3), [0, 0, 0, 0xff]);
        let height_map = constructed[4][0];
        let pixels = e.mem.u32(height_map + PIXELS);
        assert_eq!(e.mem.bytes(pixels, 0x300), vec![0; 0x300]);

        // Every texture global got its texture, held once (0x9534's twice: 0x9538 too).
        let made = calls(&e, SOURCE_TEXTURE_FROM_PIXELS);
        assert_eq!(made.len(), 8);
        for call in &made[..7] {
            assert_eq!(call[2], FORMAT_PREFS_011A9598);
        }
        assert_ne!(made[7][2], FORMAT_PREFS_011A9598);
        for global in [
            TEXTURE_011F9510,
            TEXTURE_011F9514,
            TEXTURE_011F9518,
            TEXTURE_011F951C,
            TEXTURE_011F9528,
            TEXTURE_011F952C,
            TEXTURE_011F9530,
            TEXTURE_011F953C,
            TEXTURE_011F9540,
            TEXTURE_011F9544,
            TEXTURE_011F95D4,
        ] {
            let texture = e.global::<u32>(global);
            assert_ne!(texture, 0, "{global:#x}");
            assert_eq!(e.mem.u32(texture + 4), 1, "{global:#x}");
        }
        let attenuation = e.global::<u32>(TEXTURE_011F9534);
        assert_eq!(e.global::<u32>(TEXTURE_011F9538), attenuation);
        assert_eq!(e.mem.u32(attenuation + 4), 2);

        assert_eq!(
            calls(&e, BS_SHADER_MANAGER_GET_TEXTURE),
            vec![
                vec![0x010a_d9a4, 1, TEXTURE_011F9520, 1, 0],
                vec![0x010a_d970, 1, TEXTURE_011F9524, 1, 0]
            ]
        );
        let formats: Vec<u32> = calls(&e, FORMAT_00406D00).iter().map(|c| c[2]).collect();
        assert_eq!(
            formats,
            vec![0x010a_d910, 0x010a_d8e4, 0x010a_d8c0, 0x010a_d8b0]
        );
        let cube_maps = calls(&e, SOURCE_CUBE_MAP_CREATE);
        assert_eq!(cube_maps.len(), 2);
        let buffer = cube_maps[0][0];
        assert_eq!(
            cube_maps[0][..7],
            [buffer, buffer, buffer, buffer, buffer, buffer, 0x0777_0000]
        );
        assert_eq!(cube_maps[0][8], FORMAT_PREFS_011A9598);
        let from_file = calls(&e, SOURCE_TEXTURE_FROM_FILE);
        assert_eq!(from_file[0][1..], [FORMAT_PREFS_011A9598, 1, 0]);

        // The attenuation map: full at the corners, empty at the centre.
        let attenuation_data = constructed[8][0];
        let pixels = e.mem.u32(attenuation_data + PIXELS);
        let at = |column: u32, row: u32| pixels + (row * 128 + column) * 3;
        assert_eq!(e.mem.bytes(at(0, 0), 3), vec![255; 3]);
        assert_eq!(e.mem.bytes(at(127, 127), 3), vec![255; 3]);
        assert_eq!(e.mem.bytes(at(63, 63), 3), vec![0; 3]);

        // Every pixel data deleted at the end (the normal map's when the height map
        // replaced it), every name's count back to 0.
        let deleted: Vec<u32> = calls(&e, PIXEL_DATA_DELETE).iter().map(|c| c[0]).collect();
        assert_eq!(deleted.len(), 9);
        assert_eq!(deleted[0], constructed[2][0]);
        assert_eq!(*deleted.last().unwrap(), cube);
        let names: Vec<u32> = made.iter().map(|c| c[1]).collect();
        for name in names {
            assert_eq!(e.mem.u32(name - 8), 0);
        }
    }

    #[test]
    fn fn_00b55bb0_inverts_the_attenuation_when_asked() {
        let mut e = textures_rig(1);
        e.call_log = Some(vec![]);
        e.call(0x00b5_5bb0, &args![]);
        let constructed = calls(&e, NI_PIXEL_DATA_CONSTRUCT);
        let pixels = e.mem.u32(constructed[8][0] + PIXELS);
        assert_eq!(e.mem.bytes(pixels, 3), vec![0; 3]);
        let centre = pixels + (63 * 128 + 63) * 3;
        assert_eq!(e.mem.bytes(centre, 3), vec![254; 3]);
    }

    #[test]
    fn register_streamables_registers_the_twenty_loaders_in_order() {
        let mut e = Engine::new();
        e.register(NI_STREAM_REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b4_f1a0, &args![]);
        let registered = calls(&e, NI_STREAM_REGISTER_LOADER);
        assert_eq!(registered.len(), 20);
        assert_eq!(registered[0], vec![0x010a_93a0, 0x00ba_9250]);
        assert_eq!(registered[5], vec![0x010a_9320, 0x00bb_45d0]);
        assert_eq!(registered[19], vec![0x010a_91dc, 0x00bb_7300]);
    }

    fn fog_rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x010a_d000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(FOG_DISTANCE_010AD6E0, 500_000.0f32);
        e.set_global(HALF_01016248, 0.5f32);
        counting(&mut e);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BS_FOG_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        e
    }

    fn held(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x64);
        e.mem.set_u32(object, 0x0f00_1000);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn set_fog_property_without_one_makes_the_default_fog() {
        let mut e = fog_rig();
        let node = e.mem.alloc(0x200);
        let old = held(&mut e, 1);
        e.mem.set_u32(node + 0x134, old);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4110, &args![node, 0u32]);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![0x64]]);
        let fog = e.mem.u32(node + 0x134);
        assert_eq!(calls(&e, BS_FOG_PROPERTY_CONSTRUCT), vec![vec![fog]]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![old]]);
        assert_eq!(e.mem.u32(fog + 4), 1);
        for offset in [0x2c, 0x30, 0x34, 0x38] {
            assert_eq!(e.mem.f32(fog + offset), 500_000.0, "+{offset:#x}");
        }
        assert_eq!(e.mem.f32(fog + 0x60), 1.0);
        for offset in [0x20, 0x24, 0x28, 0x54, 0x58, 0x5c] {
            assert_eq!(e.mem.f32(fog + offset), 0.5, "+{offset:#x}");
        }
    }

    #[test]
    fn set_fog_property_assigns_a_given_fog_once() {
        let mut e = fog_rig();
        let node = e.mem.alloc(0x200);
        let fog = held(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b5_4110, &args![node, fog]);
        e.call(0x00b5_4110, &args![node, fog]);
        assert_eq!(e.mem.u32(node + 0x134), fog);
        assert_eq!(e.mem.u32(fog + 4), 1);
        assert!(calls(&e, NEW_OBJECT).is_empty());
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.mem.f32(fog + 0x2c), 0.0);
        // A second fog releases the first, which is deleted at 0.
        let other = held(&mut e, 0);
        e.call(0x00b5_4110, &args![node, other]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![fog]]);
        assert_eq!(e.mem.u32(node + 0x134), other);
    }
}
