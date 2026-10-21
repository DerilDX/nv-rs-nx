//! `BSShader/bsshaderutil.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB, `00ba91a0`): `cdecl`, three stack words (the
/// object, the flag index, the value).
const SHADER_PROPERTY_SET_FLAG_RECURSE: u32 = 0x00ba_91a0;

// Translated from 00b6b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyWireframe` (Xbox PDB): `cdecl`, two stack words (the
/// object, the value): `BSShaderProperty::SetFlagRecurse` with flag index `0x30`.
pub fn bs_shader_util_recursive_set_property_wireframe(e: &mut Engine, object: Ptr, value: u32) {
    e.call(
        SHADER_PROPERTY_SET_FLAG_RECURSE,
        &args![object, 0x30u32, value],
    );
}

/// The lock `006503b0` holds while it walks its map: `00652140` takes it, `00652190` gives
/// it back (no arguments; both work on the lock at `011d5a9c` and the count at `011d59dc`).
const MAP_LOCK_ENTER: u32 = 0x0065_2140;
const MAP_LOCK_LEAVE: u32 = 0x0065_2190;
/// `004b9ba0` (`thiscall`, no stack words): the map's first position (0 when empty).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// `00633c90` (`thiscall`, one word): `NiPointer` constructor from a raw pointer.
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiTMapBase<..>::GetNext` (`00559120`, `thiscall`, three words: &position, &key, &value).
const MAP_GET_NEXT: u32 = 0x0055_9120;
/// `00559450` (`thiscall`, no stack words): the word at `this` (a smart pointer's object).
const POINTER_GET: u32 = 0x0055_9450;
/// `00726070` (`thiscall`, no stack words): the word at `this + 4`.
const WORD_AT_4: u32 = 0x0072_6070;
/// `NiTMapBase<..>::RemoveAt` (`00405430`, `thiscall`, one word: the key).
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// `0066b0d0` (`thiscall`, one word): `NiPointer` assignment from a raw pointer.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0045cec0` (`thiscall`, no stack words): `NiPointer` destructor.
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;

// Translated from 006503b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `BSShaderUtil::UpdateDebugBoundVis` (a call-graph guess the body does not
/// confirm), so named by address: `thiscall`, no stack words. Under the lock `00652140` /
/// `00652190`, walks the map at `this + 4` from its first position with `GetNext` (position,
/// key and an `NiPointer` value in locals) and removes (`RemoveAt`, by key) every entry whose
/// value's object at +8 has the word 1 at +4; the value pointer is cleared after each entry
/// and destroyed at the end.
// Translation incomplete: 006503b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
pub fn fn_006503b0(e: &mut Engine, this: Ptr) {
    let map = this.addr().wrapping_add(4);
    e.call(MAP_LOCK_ENTER, &args![]);
    let first = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    e.with_stack(0xc, |e, locals| {
        // EBP-0x10: position, EBP-0x14: key, EBP-0x18: the NiPointer value.
        let position = locals.addr();
        let key = position + 4;
        let value = position + 8;
        e.mem.set_u32(position, first);
        e.call(NI_POINTER_CONSTRUCT, &args![value, 0u32]);
        while e.mem.u32(position) != 0 {
            e.call(MAP_GET_NEXT, &args![map, position, key, value]);
            let object = e.call(POINTER_GET, &args![value]).u32();
            let inner = e.call(POINTER_GET, &args![object.wrapping_add(8)]).u32();
            if e.call(WORD_AT_4, &args![inner]).u32() == 1 {
                let k = e.mem.u32(key);
                e.call(MAP_REMOVE_AT, &args![map, k]);
            }
            e.call(NI_POINTER_ASSIGN, &args![value, 0u32]);
        }
        e.call(MAP_LOCK_LEAVE, &args![]);
        e.call(NI_POINTER_DESTRUCT, &args![value]);
    });
}

// Translated from 00b6b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader/bsshaderutil.cpp`): `cdecl`, three stack words (its
/// one caller, `00873200`, passes an animation data pointer, an object and 1). Jumps to the
/// object's virtual at +0xAC with the caller's stack as it is, so that method sees the three
/// words as its stack arguments.
pub fn fn_00b6b930(e: &mut Engine, first: u32, object: Ptr, third: u32) {
    e.vcall(object.addr(), 0xac, &args![first, object, third]);
}

/// The global `00b6ba20`, `AccumulateScene` and `AccumulateSceneList` test first
/// (`011f4748`).
const SCENE_GLOBAL_011F4748: u32 = 0x011f_4748;
/// The global `00b6ba20` stores its first argument in (`011f917c`).
const LAST_OBJECT_011F917C: u32 = 0x011f_917c;
/// `004e9bb0` (`thiscall` on the global at `011f4748`, one word).
const SCENE_SET_OBJECT_004E9BB0: u32 = 0x004e_9bb0;
/// `00b54b10` (`cdecl`, one word).
const PREPARE_OBJECT_00B54B10: u32 = 0x00b5_4b10;

// Translated from 00b6ba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader/bsshaderutil.cpp`): `cdecl`, three stack words.
/// With the global at `011f4748` set and a first argument: calls `004e9bb0` on the global
/// with it, stores it at `011f917c`, calls `00b54b10` with the second argument, then the
/// second argument's virtual +0x8C with the first, and jumps to its virtual +0xA8 with the
/// caller's stack as it is (the three words as that method's stack arguments).
pub fn fn_00b6ba20(e: &mut Engine, first: u32, object: Ptr, third: u32) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || first == 0 {
        return;
    }
    e.call(SCENE_SET_OBJECT_004E9BB0, &args![scene, first]);
    e.set_global(LAST_OBJECT_011F917C, first);
    e.call(PREPARE_OBJECT_00B54B10, &args![object]);
    e.vcall(object.addr(), 0x8c, &args![first]);
    e.vcall(object.addr(), 0xa8, &args![first, object, third]);
}

/// `MemoryManager` singleton and `MemoryManager::Allocate` (`thiscall`, one word: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00a5b690` (`cdecl`, one word: a C string): an `NiFixedString` handle for the string.
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// `BSShaderUtil::pConvertedTextureName` and `pBGConvertedTextureName` (Xbox PDB statics):
/// `NiFixedString*`.
const CONVERTED_TEXTURE_NAME: u32 = 0x011f_a050;
const BG_CONVERTED_TEXTURE_NAME: u32 = 0x011f_a054;
/// `"ConvertedTexture"` and `"BackgroundConvertedTexture"`.
const CONVERTED_TEXTURE_TEXT: u32 = 0x010a_e658;
const BG_CONVERTED_TEXTURE_TEXT: u32 = 0x010a_e63c;

/// A 4-byte `NiFixedString` from the allocator holding `text`'s handle (0 when the
/// allocation failed).
fn new_fixed_string(e: &mut Engine, text: u32) -> u32 {
    let string = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    if string != 0 {
        let handle = e.call(FIXED_STRING_FROM_TEXT, &args![text]).u32();
        e.mem.set_u32(string, handle);
    }
    string
}

// Translated from 00b6ba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::InitSDM` (Xbox PDB): no arguments. Allocates the two texture-name strings
/// (`"ConvertedTexture"`, `"BackgroundConvertedTexture"`) into `pConvertedTextureName` and
/// `pBGConvertedTextureName` (0 where the allocation failed).
// Translation incomplete: 00b6ba70 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around each allocation) is not translated.
pub fn bs_shader_util_init_sdm(e: &mut Engine) {
    let name = new_fixed_string(e, CONVERTED_TEXTURE_TEXT);
    e.set_global(CONVERTED_TEXTURE_NAME, name);
    let background = new_fixed_string(e, BG_CONVERTED_TEXTURE_TEXT);
    e.set_global(BG_CONVERTED_TEXTURE_NAME, background);
}

// Translated from 00b6bb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyFadeAlpha` (Xbox PDB): `cdecl`, two stack words (the
/// node, the alpha as a float). Only for a node with a world bound (+0x20) whose radius
/// (+0xC) is not 0 (or is NaN): for each child slot (`m_kChildren`, elements at +0xA0, count
/// `m_usSize` at +0xA6, read again each step) holding a child, a child that is geometry
/// (virtual +0x18) gets the alpha in its shade property's `fFadeAlpha` (+0x2C) when the
/// property (`m_PropertyState` slot 3, +0xA8) is set and its word at +0x1C is not -1; then the
/// function recurses on what the child's virtual +0xC answers (its node, or 0).
pub fn bs_shader_util_recursive_set_property_fade_alpha(e: &mut Engine, node: Ptr, alpha: f32) {
    if node.is_null() {
        return;
    }
    let bound = e.mem.u32(node.addr() + 0x20);
    if bound == 0 {
        return;
    }
    // FLDZ; FCOMP [bound + 0xC]; TEST AH,0x44; JNP: leave when the radius equals 0.
    let radius = x87_m32(e.mem.u32(bound + 0xc));
    if radius == 0.0 {
        return;
    }
    let mut index = 0u32;
    if e.mem.u16(node.addr() + 0xa6) == 0 {
        return;
    }
    loop {
        let children = e.mem.u32(node.addr() + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index * 4));
        if child != 0 {
            let geometry = e.vcall(child, 0x18, &args![]).u32();
            // LEA ECX,[EAX+0x9C]; TEST ECX,ECX: the property state's address.
            if geometry != 0 && geometry.wrapping_add(0x9c) != 0 {
                let property = e.mem.u32(geometry.wrapping_add(0xa8));
                if property != 0 && e.mem.u32(property.wrapping_add(0x1c)).wrapping_add(1) != 0 {
                    e.mem
                        .set_u32(property.wrapping_add(0x2c), x87_quiet_bits(alpha.to_bits()));
                }
            }
            let next = e.vcall(child, 0xc, &args![]).ptr();
            bs_shader_util_recursive_set_property_fade_alpha(e, next, alpha);
        }
        index += 1;
        if index >= u32::from(e.mem.u16(node.addr() + 0xa6)) {
            break;
        }
    }
}

/// `004dc540` (`thiscall` on the global at `011f4748`, one word).
const SCENE_SET_ACCUMULATOR_004DC540: u32 = 0x004d_c540;
/// `BSShaderManager::SetAccumulator` (`00b54ac0`, `cdecl`, one word).
const SHADER_MANAGER_SET_ACCUMULATOR: u32 = 0x00b5_4ac0;
/// The thread's TLS word `AccumulateScene` tests (`+0x2BC` in the block of `_tls_index`).
const TLS_ACCUMULATE_WORD: u32 = 0x2bc;
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00b6bee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::AccumulateScene` (Xbox PDB): `cdecl`, three stack words (two objects and
/// the culling process). With the global at `011f4748` and both objects set: unless the
/// thread's TLS word +0x2BC is set, holds a reference to the global's object at +8 (an
/// `NiPointer` local), calls `004dc540` on the global with the process's accumulator
/// (+0xC4) and releases the reference (`0045cec0`); then `BSShaderManager::SetAccumulator`
/// with the accumulator and the process's virtual +0x48 with (first, second, 0).
// Translation incomplete: 00b6bee0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the NiPointer local) is not translated.
pub fn bs_shader_util_accumulate_scene(e: &mut Engine, first: u32, second: u32, process: Ptr) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || first == 0 || second == 0 {
        return;
    }
    let tls = e.tls();
    if e.mem.u32(tls + TLS_ACCUMULATE_WORD) == 0 {
        let held = e.mem.u32(scene.wrapping_add(8));
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), held);
            if held != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
            }
            let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
            let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
            e.call(SCENE_SET_ACCUMULATOR_004DC540, &args![scene, accumulator]);
            e.call(NI_POINTER_DESTRUCT, &args![local]);
        });
    }
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SHADER_MANAGER_SET_ACCUMULATOR, &args![accumulator]);
    e.vcall(process.addr(), 0x48, &args![first, second, 0u32]);
}

// Translated from 00b6bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::AccumulateSceneList` (Xbox PDB): `cdecl`, three stack words (an object, a
/// list whose first word is its head node, the culling process). With the global at
/// `011f4748` and the object set: holds a reference to the global's object at +8, calls
/// `004dc540` on the global and `BSShaderManager::SetAccumulator` with the process's
/// accumulator (+0xC4); for a non-empty list calls the process's virtual +0x48 with (object,
/// first node's element, 0), then its virtual +0x44 with each following node's element
/// (nodes: next at +0, element at +8), stopping after the element before a null one; then
/// releases the reference (`InterlockedDecrement`, `DeleteThis` at 0).
// Translation incomplete: 00b6bfc0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the held reference) is not translated.
pub fn bs_shader_util_accumulate_scene_list(e: &mut Engine, object: u32, list: Ptr, process: Ptr) {
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    if scene == 0 || object == 0 {
        return;
    }
    let held = e.mem.u32(scene.wrapping_add(8));
    if held != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
    }
    let scene = e.global::<u32>(SCENE_GLOBAL_011F4748);
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SCENE_SET_ACCUMULATOR_004DC540, &args![scene, accumulator]);
    let accumulator = e.mem.u32(process.addr().wrapping_add(0xc4));
    e.call(SHADER_MANAGER_SET_ACCUMULATOR, &args![accumulator]);
    let head = e.mem.u32(list.addr());
    if head != 0 {
        let second = e.mem.u32(head);
        let element = e.mem.u32(head.wrapping_add(8));
        e.vcall(process.addr(), 0x48, &args![object, element, 0u32]);
        if second != 0 {
            let mut node = e.mem.u32(second);
            let mut element = e.mem.u32(second.wrapping_add(8));
            loop {
                let current = element;
                if node != 0 {
                    element = e.mem.u32(node.wrapping_add(8));
                    node = e.mem.u32(node);
                } else {
                    element = 0;
                }
                e.vcall(process.addr(), 0x44, &args![current]);
                if element == 0 {
                    break;
                }
            }
        }
    }
    if held != 0
        && e.call(INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(held, 4, &args![]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b6_b910,
            bs_shader_util_recursive_set_property_wireframe(Ptr, u32)
        ),
        entry!(0x0065_03b0, fn_006503b0(Ptr)),
        entry!(0x00b6_b930, fn_00b6b930(u32, Ptr, u32)),
        entry!(0x00b6_ba20, fn_00b6ba20(u32, Ptr, u32)),
        entry!(0x00b6_ba70, bs_shader_util_init_sdm()),
        entry!(
            0x00b6_bb30,
            bs_shader_util_recursive_set_property_fade_alpha(Ptr, f32)
        ),
        entry!(0x00b6_bee0, bs_shader_util_accumulate_scene(u32, u32, Ptr)),
        entry!(
            0x00b6_bfc0,
            bs_shader_util_accumulate_scene_list(u32, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00b6b910_sets_flag_0x30_recursively() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ba_91a0, |_, _| Ret::default());
        e.call(0x00b6_b910, &args![0x8000u32, 1u32]);
        e.call(0x00b6_b910, &args![0x9000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x00ba_91a0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(sets, vec![&vec![0x8000, 0x30, 1], &vec![0x9000, 0x30, 0]]);
    }
}

#[cfg(test)]
mod b048_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A vtable at `table` whose slot `n` calls the double at `base + 4 * n`.
    fn vtable(e: &mut Engine, table: u32, base: u32, slots: u32) {
        let targets: Vec<u32> = (0..slots).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
    }

    #[test]
    fn fn_006503b0_removes_entries_whose_value_word_is_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        // Two values: objects whose +8 points at a block with word +4 = 1 and = 0.
        let removable = e.mem.alloc(0x10);
        let kept = e.mem.alloc(0x10);
        let flag_one = e.mem.alloc(8);
        let flag_zero = e.mem.alloc(8);
        e.mem.set_u32(flag_one + 4, 1);
        e.mem.set_u32(removable + 8, flag_one);
        e.mem.set_u32(kept + 8, flag_zero);
        e.register(MAP_LOCK_ENTER, nop);
        e.register(MAP_LOCK_LEAVE, nop);
        e.register(MAP_FIRST_POSITION, |_, _| 0x77u32.into_ret());
        e.register(NI_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_DESTRUCT, nop);
        let entries = Rc::new(RefCell::new(vec![
            (0x88u32, 11u32, removable),
            (0, 12, kept),
        ]));
        let feed = entries.clone();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            let (next, key, value) = feed.borrow_mut().remove(0);
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(WORD_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(MAP_REMOVE_AT, nop);
        e.call_log = Some(vec![]);
        e.call(0x0065_03b0, &args![this]);
        assert!(entries.borrow().is_empty());
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![this + 4, 11]]);
        assert_eq!(calls(&e, MAP_FIRST_POSITION), vec![vec![this + 4]]);
        assert_eq!(calls(&e, MAP_LOCK_LEAVE).len(), 1);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN).len(), 2);
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00b6b930_passes_the_callers_stack_to_virtual_0xac() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        vtable(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object, 0x6000);
        e.register(0x6100_00ac, nop);
        e.call_log = Some(vec![]);
        e.call(0x00b6_b930, &args![0x1234u32, object, 1u32]);
        assert_eq!(
            calls(&e, 0x6100_00ac),
            vec![vec![object, 0x1234, object, 1]]
        );
    }

    #[test]
    fn fn_00b6ba20_needs_the_global_and_the_first_argument() {
        let mut e = Engine::new();
        e.map(SCENE_GLOBAL_011F4748, 4);
        e.map(LAST_OBJECT_011F917C, 4);
        let object = e.mem.alloc(8);
        vtable(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object, 0x6000);
        for a in [
            SCENE_SET_OBJECT_004E9BB0,
            PREPARE_OBJECT_00B54B10,
            0x6100_008c,
            0x6100_00a8,
        ] {
            e.register(a, nop);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b6_ba20, &args![0x1234u32, object, 3u32]);
        e.set_global(SCENE_GLOBAL_011F4748, 0x9000u32);
        e.call(0x00b6_ba20, &args![0u32, object, 3u32]);
        assert!(calls(&e, SCENE_SET_OBJECT_004E9BB0).is_empty());
        e.call(0x00b6_ba20, &args![0x1234u32, object, 3u32]);
        assert_eq!(
            calls(&e, SCENE_SET_OBJECT_004E9BB0),
            vec![vec![0x9000, 0x1234]]
        );
        assert_eq!(e.global::<u32>(LAST_OBJECT_011F917C), 0x1234);
        assert_eq!(calls(&e, PREPARE_OBJECT_00B54B10), vec![vec![object]]);
        assert_eq!(calls(&e, 0x6100_008c), vec![vec![object, 0x1234]]);
        assert_eq!(
            calls(&e, 0x6100_00a8),
            vec![vec![object, 0x1234, object, 3]]
        );
    }

    #[test]
    fn init_sdm_allocates_both_texture_names() {
        let mut e = Engine::new();
        e.map(CONVERTED_TEXTURE_NAME, 8);
        let first = Rc::new(RefCell::new(0u32));
        let seen = first.clone();
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER, 4]);
            let mut b = seen.borrow_mut();
            // The first allocation succeeds, the second fails.
            let p = if *b == 0 { e.mem.alloc(4) } else { 0 };
            if *b == 0 {
                *b = p;
            }
            p.into_ret()
        });
        e.register(FIXED_STRING_FROM_TEXT, |_, a| (a[0] + 1).into_ret());
        e.set_global(BG_CONVERTED_TEXTURE_NAME, 0x55u32);
        e.call(0x00b6_ba70, &args![]);
        let first = *first.borrow();
        assert_eq!(e.global::<u32>(CONVERTED_TEXTURE_NAME), first);
        assert_eq!(e.mem.u32(first), CONVERTED_TEXTURE_TEXT + 1);
        assert_eq!(e.global::<u32>(BG_CONVERTED_TEXTURE_NAME), 0);
    }

    #[test]
    fn fade_alpha_reaches_geometry_properties_and_recurses() {
        let mut e = Engine::new();
        vtable(&mut e, 0x6000, 0x6100_0000, 0x10);
        // Slot 0x18 answers the object itself when its word +0x40 is 1 (geometry);
        // slot 0x0c answers the word +0x44 (its node, for the recursion).
        e.register(0x6100_0018, |e, a| {
            if e.mem.u32(a[0] + 0x40) == 1 { a[0] } else { 0 }.into_ret()
        });
        e.register(0x6100_000c, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, 2.0f32.to_bits());
        let make_node = |e: &mut Engine, children: &[u32]| {
            let node = e.mem.alloc(0xb0);
            e.mem.set_u32(node, 0x6000);
            e.mem.set_u32(node + 0x20, bound);
            let array = e.mem.alloc(16);
            for (i, c) in children.iter().enumerate() {
                e.mem.set_u32(array + 4 * i as u32, *c);
            }
            e.mem.set_u32(node + 0xa0, array);
            e.mem.set_u16(node + 0xa6, children.len() as u16);
            node
        };
        let property = e.mem.alloc(0x30);
        let skipped = e.mem.alloc(0x30);
        e.mem.set_u32(skipped + 0x1c, 0xffff_ffff);
        let geometry = e.mem.alloc(0xb0);
        e.mem.set_u32(geometry, 0x6000);
        e.mem.set_u32(geometry + 0x40, 1);
        e.mem.set_u32(geometry + 0xa8, property);
        let other = e.mem.alloc(0xb0);
        e.mem.set_u32(other, 0x6000);
        e.mem.set_u32(other + 0x40, 1);
        e.mem.set_u32(other + 0xa8, skipped);
        let deep = e.mem.alloc(0x30);
        let deep_geometry = e.mem.alloc(0xb0);
        e.mem.set_u32(deep_geometry, 0x6000);
        e.mem.set_u32(deep_geometry + 0x40, 1);
        e.mem.set_u32(deep_geometry + 0xa8, deep);
        let inner = make_node(&mut e, &[other, deep_geometry]);
        let holder = e.mem.alloc(0xb0);
        e.mem.set_u32(holder, 0x6000);
        e.mem.set_u32(holder + 0x44, inner);
        let root = make_node(&mut e, &[geometry, 0, holder]);
        e.call(0x00b6_bb30, &args![root, 0.25f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(deep + 0x2c), 0.25f32.to_bits());
        assert_eq!(e.mem.u32(skipped + 0x2c), 0);
        // A zero radius (also -0) stops at the root.
        e.mem.set_u32(property + 0x2c, 0);
        e.mem.set_u32(bound + 0xc, 0x8000_0000);
        e.call(0x00b6_bb30, &args![root, 0.5f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0);
        // A NaN radius does not.
        e.mem.set_u32(bound + 0xc, 0x7fc0_0000);
        e.call(0x00b6_bb30, &args![root, 0.5f32]);
        assert_eq!(e.mem.u32(property + 0x2c), 0.5f32.to_bits());
    }

    fn accumulate_rig(e: &mut Engine) -> (u32, u32) {
        e.map(SCENE_GLOBAL_011F4748, 4);
        let scene = e.mem.alloc(0x10);
        let held = e.mem.alloc(8);
        e.mem.set_u32(held, 0x6000);
        e.mem.set_u32(scene + 8, held);
        e.set_global(SCENE_GLOBAL_011F4748, scene);
        vtable(e, 0x6000, 0x6100_0000, 0x20);
        let process = e.mem.alloc(0xd0);
        e.mem.set_u32(process, 0x6000);
        e.mem.set_u32(process + 0xc4, 0xacc);
        for a in [
            SCENE_SET_ACCUMULATOR_004DC540,
            SHADER_MANAGER_SET_ACCUMULATOR,
            NI_POINTER_DESTRUCT,
            0x6100_0048,
            0x6100_0044,
            0x6100_0004,
        ] {
            e.register(a, nop);
        }
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        (scene, process)
    }

    #[test]
    fn accumulate_scene_skips_the_scene_call_when_the_tls_word_is_set() {
        let mut e = Engine::new();
        let (scene, process) = accumulate_rig(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b6_bee0, &args![1u32, 0u32, process]);
        assert!(calls(&e, 0x6100_0048).is_empty());
        e.call(0x00b6_bee0, &args![1u32, 2u32, process]);
        assert_eq!(
            calls(&e, SCENE_SET_ACCUMULATOR_004DC540),
            vec![vec![scene, 0xacc]]
        );
        assert_eq!(calls(&e, NI_POINTER_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, SHADER_MANAGER_SET_ACCUMULATOR), vec![vec![0xacc]]);
        assert_eq!(calls(&e, 0x6100_0048), vec![vec![process, 1, 2, 0]]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ACCUMULATE_WORD, 1);
        e.call(0x00b6_bee0, &args![1u32, 2u32, process]);
        assert_eq!(calls(&e, SCENE_SET_ACCUMULATOR_004DC540).len(), 1);
        assert_eq!(calls(&e, 0x6100_0048).len(), 2);
    }

    #[test]
    fn accumulate_scene_list_walks_until_a_null_element() {
        let mut e = Engine::new();
        let (scene, process) = accumulate_rig(&mut e);
        let held = e.mem.u32(scene + 8);
        // Nodes: next at +0, element at +8. Elements 0xa, 0xb, 0xc, then a node with 0.
        let n4 = e.mem.alloc(12);
        let n3 = e.mem.alloc(12);
        let n2 = e.mem.alloc(12);
        let n1 = e.mem.alloc(12);
        e.mem.set_u32(n1, n2);
        e.mem.set_u32(n1 + 8, 0xa);
        e.mem.set_u32(n2, n3);
        e.mem.set_u32(n2 + 8, 0xb);
        e.mem.set_u32(n3, n4);
        e.mem.set_u32(n3 + 8, 0xc);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, n1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_bfc0, &args![7u32, list, process]);
        assert_eq!(calls(&e, 0x6100_0048), vec![vec![process, 7, 0xa, 0]]);
        assert_eq!(
            calls(&e, 0x6100_0044),
            vec![vec![process, 0xb], vec![process, 0xc]]
        );
        // The reference went up and back down to 0: DeleteThis ran.
        assert_eq!(e.mem.u32(held + 4), 0);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![held]]);
        // An empty list only sets the accumulator.
        e.mem.set_u32(list, 0);
        e.call(0x00b6_bfc0, &args![7u32, list, process]);
        assert_eq!(calls(&e, 0x6100_0048).len(), 1);
        assert_eq!(calls(&e, SHADER_MANAGER_SET_ACCUMULATOR).len(), 2);
    }
}
