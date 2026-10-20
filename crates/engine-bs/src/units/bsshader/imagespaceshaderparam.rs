//! `BSShader/imagespaceshaderparam.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The clean-up `00b8a7e0` (this unit, unnamed; `thiscall`, no stack words) the function
/// starts with.
const CLEAN_UP_00B8A7E0: u32 = 0x00b8_a7e0;
/// The Ni allocation `00aa13e0` (`cdecl`, size) and
/// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map, `00b7e330`, `thiscall`: the
/// renderer and a kind word).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const CONSTANT_MAP_CONSTRUCT: u32 = 0x00b7_e330;
/// The renderer word (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// The memory manager singleton (`011f6238`), its allocate (`00aa3e40`) and deallocate
/// (`00aa4060`) (`thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `"Constant Group"` (`010aff90`) and `""` (`01011584`), the strings the maps get.
const CONSTANT_GROUP_010AFF90: u32 = 0x010a_ff90;
const EMPTY_STRING_01011584: u32 = 0x0101_1584;

/// `NiPointer<T>::operator=(T *)` expanded inline.
fn assign_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        let left = e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32();
        if left == 0 {
            e.vcall(old, 4, &args![]);
        }
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

/// A constant map of the given kind (0x44 bytes, `00b7e330` with the renderer `011f9508`; null
/// when the allocation fails) assigned to the `NiPointer` at `slot`.
fn make_constant_map(e: &mut Engine, slot: u32, kind: u32) {
    let block = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
    let map = if block != 0 {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        e.call(CONSTANT_MAP_CONSTRUCT, &args![block, renderer, kind])
            .u32()
    } else {
        0
    };
    assign_pointer(e, slot, map);
}

/// A group of `count` registers: the count stored at `count_slot`; when it is above 0
/// (signed) the old buffer at `buffer_slot` is freed (when set), a new one of `count * 4`
/// four-byte words (`operator new[]` size check) allocated into it and the map at `map_slot`
/// told through its vtable slot +8: ("Constant Group", 0x10000000, 0, 0, count, "", count *
/// 16, 4, buffer, 0).
fn make_constant_group(e: &mut Engine, base: u32, count: u32, slots: (u32, u32, u32)) {
    let (count_slot, buffer_slot, map_slot) = slots;
    e.mem.set_u32(base + count_slot, count);
    if (count as i32) <= 0 {
        return;
    }
    let old = e.mem.u32(base + buffer_slot);
    if old != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, old]);
    }
    let elements = e.mem.u32(base + count_slot).wrapping_mul(4);
    let product = u64::from(elements) * 4;
    let bytes = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    let buffer = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let count = e.mem.u32(base + count_slot);
    let map = e.mem.u32(base + map_slot);
    e.mem.set_u32(base + buffer_slot, buffer);
    e.vcall(
        map,
        8,
        &args![
            CONSTANT_GROUP_010AFF90,
            0x1000_0000u32,
            0u32,
            0u32,
            count,
            EMPTY_STRING_01011584,
            count.wrapping_shl(4),
            4u32,
            buffer,
            0u32
        ],
    );
}

// Translated from 00b8ab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceShaderParam::InitConstantMap` (Xbox PDB), `thiscall` with two stack words: the
/// vertex and pixel register counts. Runs the clean-up `00b8a7e0`, makes the vertex constant
/// map (kind 0) into the `NiPointer` at +0x4c and the pixel one (kind 2) into +0x50
/// (`NiD3DShaderConstantMap::NiD3DShaderConstantMap` with the renderer `011f9508` on 0x44
/// bytes). Then the vertex group: +0x40 = first + 3, buffer at +0x3c, map +0x4c; and the pixel
/// group: +0x48 = second, buffer at +0x44, map +0x50; a group with a count above 0 gets a new
/// buffer of 16 bytes per register (the old one freed) and is added to its map as
/// `"Constant Group"` (vtable slot +8).
// Translation incomplete: 00b8ab10 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn image_space_shader_param_init_constant_map(
    e: &mut Engine,
    this: Ptr,
    vertex_count: u32,
    pixel_count: u32,
) {
    let base = this.addr();
    e.call(CLEAN_UP_00B8A7E0, &args![base]);
    make_constant_map(e, base + 0x4c, 0);
    make_constant_map(e, base + 0x50, 2);
    make_constant_group(e, base, vertex_count.wrapping_add(3), (0x40, 0x3c, 0x4c));
    make_constant_group(e, base, pixel_count, (0x48, 0x44, 0x50));
}

// Translated from 00b8a7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceshaderparam.cpp`): the clean-up the constant map
/// set-up (`00b8ab10`) starts with, `thiscall`, no stack words. The constant maps held at +0x4c
/// and +0x50 are released (`InterlockedDecrement` on their +4, deleted through vtable slot +4
/// at 0) and cleared; the register buffers at +0x3c and +0x44 are given back to the memory
/// manager (`00aa4060` on `011f6238`) and cleared when set.
pub fn fn_00b8a7e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for slot in [0x4cu32, 0x50] {
        let map = e.mem.u32(base + slot);
        if map != 0 {
            let left = e
                .call(IMPORT_INTERLOCKED_DECREMENT, &args![map.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(map, 4, &args![]);
            }
            e.mem.set_u32(base + slot, 0);
        }
    }
    for slot in [0x3cu32, 0x44] {
        let buffer = e.mem.u32(base + slot);
        if buffer != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
            e.mem.set_u32(base + slot, 0);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b8_ab10,
            image_space_shader_param_init_constant_map(Ptr, u32, u32)
        ),
        entry!(0x00b8_a7e0, fn_00b8a7e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADD_GROUP: u32 = 0x0f00_0000;
    const DELETE: u32 = 0x0f00_0010;
    const MAP_VTABLE: u32 = 0x0f01_0000;
    const RENDERER: u32 = 0x0f02_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(CLEAN_UP_00B8A7E0, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            a[0].into_ret()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(ADD_GROUP, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(MAP_VTABLE, &[0, DELETE, ADD_GROUP]);
        e.map(0x011f_9000, 0x1000);
        e.set_global::<u32>(RENDERER_011F9508, RENDERER);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn init_constant_map_makes_both_maps_and_groups() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.mem.set_u32(this + 0x3c, 0x1234);
        e.call(0x00b8_ab10, &args![this, 5u32, 2u32]);
        assert_eq!(calls(&e, CLEAN_UP_00B8A7E0), vec![vec![this]]);
        let vertex_map = e.mem.u32(this + 0x4c);
        let pixel_map = e.mem.u32(this + 0x50);
        assert_eq!(
            calls(&e, CONSTANT_MAP_CONSTRUCT),
            vec![vec![vertex_map, RENDERER, 0], vec![pixel_map, RENDERER, 2]]
        );
        assert_eq!(e.mem.u32(vertex_map + 4), 1);
        assert_eq!(e.mem.u32(this + 0x40), 8);
        assert_eq!(e.mem.u32(this + 0x48), 2);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1234]]
        );
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x80], vec![MEMORY_MANAGER, 0x20]]
        );
        let vertex_buffer = e.mem.u32(this + 0x3c);
        let pixel_buffer = e.mem.u32(this + 0x44);
        assert_eq!(
            calls(&e, ADD_GROUP),
            vec![
                vec![
                    vertex_map,
                    CONSTANT_GROUP_010AFF90,
                    0x1000_0000,
                    0,
                    0,
                    8,
                    EMPTY_STRING_01011584,
                    0x80,
                    4,
                    vertex_buffer,
                    0
                ],
                vec![
                    pixel_map,
                    CONSTANT_GROUP_010AFF90,
                    0x1000_0000,
                    0,
                    0,
                    2,
                    EMPTY_STRING_01011584,
                    0x20,
                    4,
                    pixel_buffer,
                    0
                ],
            ]
        );
    }

    #[test]
    fn init_constant_map_skips_empty_groups() {
        let mut e = engine();
        let this = e.mem.alloc(0x54);
        e.call(0x00b8_ab10, &args![this, (-3i32) as u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, ADD_GROUP).is_empty());
        // A failed allocation leaves the map pointer null.
        let mut e = engine();
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x54);
        e.call(0x00b8_ab10, &args![this, (-3i32) as u32, 0u32]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert!(calls(&e, CONSTANT_MAP_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod clean_up_tests {
    use super::*;

    const DELETE: u32 = 0x0f10_0004;
    const VTABLE: u32 = 0x0f11_0000;

    #[test]
    fn fn_00b8a7e0_releases_the_maps_and_frees_the_buffers() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x54);
        let kept = e.mem.alloc(8);
        e.mem.set_u32(kept, VTABLE);
        e.mem.set_u32(kept + 4, 2);
        let dropped = e.mem.alloc(8);
        e.mem.set_u32(dropped, VTABLE);
        e.mem.set_u32(dropped + 4, 1);
        e.mem.set_u32(this + 0x4c, kept);
        e.mem.set_u32(this + 0x50, dropped);
        e.mem.set_u32(this + 0x3c, 0x0e00_1000);
        e.mem.set_u32(this + 0x44, 0);
        e.call(0x00b8_a7e0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(e.mem.u32(kept + 4), 1);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<_> = log.iter().filter(|c| c.0 == DELETE).collect();
        assert_eq!(deleted, vec![&(DELETE, vec![dropped])]);
        let freed: Vec<_> = log.iter().filter(|c| c.0 == MEMORY_DEALLOCATE).collect();
        assert_eq!(
            freed,
            vec![&(MEMORY_DEALLOCATE, vec![MEMORY_MANAGER, 0x0e00_1000])]
        );
    }
}
