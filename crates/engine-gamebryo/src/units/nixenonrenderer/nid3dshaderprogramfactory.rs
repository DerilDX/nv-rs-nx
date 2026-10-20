//! `NiXenonRenderer/nid3dshaderprogramfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a642a0` (`thiscall`, `RET 8`, no unit): stores a creator under a name in a map.
const MAP_SET_AT: u32 = 0x00a6_42a0;
/// `0126fa14`: the creator map object.
const CREATOR_MAP_SLOT: u32 = 0x0126_fa14;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00ec7690` (`cdecl`, `strchr`: text, character).
const STRCHR: u32 = 0x00ec_7690;
/// `00ec67aa` (`cdecl`, `tolower`: character).
const TOLOWER: u32 = 0x00ec_67aa;

// Translated from 00e93d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramFactory::CreateShaderCreatorMap` (Xbox PDB; no arguments): allocates
/// 0x14 bytes from the memory manager and, when that succeeds, sets the vtable
/// `010f0eb0`, the size 0x25 at `+4`, the word at `+0xc` to 0, a zeroed 0x94-byte slot
/// block at `+8` (`00aa1070`, `memset`), the byte at `+0x10` to 1 and the vtable
/// `010f0f50`, keeping the object in `0126fa14` (null when the allocation fails).
pub fn ni_d3d_shader_program_factory_create_shader_creator_map(e: &mut Engine) {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        return;
    }
    e.mem.set_u32(map, 0x010f_0eb0);
    e.mem.set_u32(map + 4, 0x25);
    e.mem.set_u32(map + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![0x94u32]).u32();
    let bytes = e.mem.u32(map + 4).wrapping_mul(4);
    e.mem.set_u32(map + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(map + 0x10, 1);
    e.mem.set_u32(map, 0x010f_0f50);
    e.mem.set_u32(CREATOR_MAP_SLOT, map);
}

// Translated from 00e93df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderprogramfactory.cpp`; no arguments): when the
/// creator map `0126fa14` is null, makes it with `00e93d80` (a jump in the exe).
pub fn fn_00e93df0(e: &mut Engine) {
    if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
        ni_d3d_shader_program_factory_create_shader_creator_map(e);
    }
}

// Translated from 00e93e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00e93e70 - /GS stack cookie check (00ec408c) not modelled
// (the 0x100-byte name buffer is a zeroed heap block)
/// `NiD3DShaderProgramFactory::RegisterShaderCreator` (Xbox PDB; `cdecl`, two words):
/// answers `false` for a null `name` or one containing a '.'; otherwise copies the name
/// lower-cased (`tolower` per character, up to 0x100 characters including the terminator)
/// into a local buffer, makes the creator map when it is missing (`00e93d80`) and stores
/// `creator` in it under the copy (`00a642a0`); answers `true`. A name of 0x100 characters
/// or more leaves the copy without terminator (the exe's buffer is followed by its
/// cookie; here by zero bytes).
pub fn ni_d3d_shader_program_factory_register_shader_creator(
    e: &mut Engine,
    name: Ptr,
    creator: u32,
) -> bool {
    if name.is_null() {
        return false;
    }
    if !e.call(STRCHR, &args![name, 0x2eu32]).ptr::<u8>().is_null() {
        return false;
    }
    e.with_stack(0x104, |e, buffer| {
        let mut index = 0u32;
        let mut source = name.addr();
        loop {
            let character = e.mem.u8(source);
            let lower = e.call(TOLOWER, &args![character as i8 as i32]).u8();
            e.mem.set_u8(buffer.addr() + index, lower);
            if character == 0 {
                break;
            }
            index += 1;
            source = source.wrapping_add(1);
            if index >= 0x100 {
                break;
            }
        }
        if e.mem.u32(CREATOR_MAP_SLOT) == 0 {
            ni_d3d_shader_program_factory_create_shader_creator_map(e);
        }
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        e.call(MAP_SET_AT, &args![map, buffer, creator]);
    });
    true
}

/// The program factory's renderer globals (`0126fa1c` the renderer, `0126fa18` its device,
/// counted with the device's `AddRef`/`Release`, slots 4 and 8, `0126fa20` its shader
/// library word, the renderer's +0x8b8).
const FACTORY_RENDERER_0126FA1C: u32 = 0x0126_fa1c;
const FACTORY_DEVICE_0126FA18: u32 = 0x0126_fa18;
const FACTORY_LIBRARY_0126FA20: u32 = 0x0126_fa20;

// Translated from 00e92f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`), `cdecl`, the renderer: keeps
/// it at `0126fa1c`. With a renderer, the device held at `0126fa18` is released (slot 8) and
/// replaced by the renderer's device (+0x288; counted, slot 4, when not null), and `0126fa20`
/// gets the renderer's +0x8b8. Without one, `0126fa20` is cleared, the held device released
/// and `0126fa18` cleared.
pub fn fn_00e92f70(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(FACTORY_RENDERER_0126FA1C, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(FACTORY_RENDERER_0126FA1C);
        let library = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, library);
    } else {
        let held = e.mem.u32(FACTORY_DEVICE_0126FA18);
        e.mem.set_u32(FACTORY_LIBRARY_0126FA20, 0);
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(FACTORY_DEVICE_0126FA18, 0);
    }
}

/// The `NiRefObject` table and the global object count (`011f4400`, counted up with
/// `InterlockedIncrement`), the factory's table (`010f0f70`) and its two maps' base and
/// final tables (`010f0df0` then `010f0f10`, `010f0e50` then `010f0f30`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const PROGRAM_FACTORY_VTABLE: u32 = 0x010f_0f70;
const FIRST_MAP_BASE_VTABLE: u32 = 0x010f_0df0;
const FIRST_MAP_VTABLE: u32 = 0x010f_0f10;
const SECOND_MAP_BASE_VTABLE: u32 = 0x010f_0e50;
const SECOND_MAP_VTABLE: u32 = 0x010f_0f30;

/// One of the factory's maps (0x14 bytes from the memory manager): base table, 0x3b buckets
/// (0xec bytes of `00aa1070`, cleared), count 0, byte +0x10 cleared, final table; null when
/// the allocation fails.
fn program_map(e: &mut Engine, base_vtable: u32, vtable: u32) -> u32 {
    let map = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if map == 0 {
        return 0;
    }
    e.mem.set_u32(map, base_vtable);
    e.mem.set_u32(map + 4, 0x3b);
    e.mem.set_u32(map + 0xc, 0);
    let buckets = e.call(ALLOCATE_BLOCK, &args![0xecu32]).u32();
    e.mem.set_u32(map + 8, buckets);
    let size = e.mem.u32(map + 4).wrapping_mul(4);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u8(map + 0x10, 0);
    e.mem.set_u32(map, vtable);
    map
}

// Translated from 00e93ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderprogramfactory.cpp`): the shader program factory
/// constructor, `thiscall`, no stack words. The `NiRefObject` start, the factory's table, +8,
/// +0xc and +0x10 cleared, and its two maps at +0x14 and +0x18. Answers `this`.
pub fn fn_00e93ad0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base + 4, 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, PROGRAM_FACTORY_VTABLE);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let first = program_map(e, FIRST_MAP_BASE_VTABLE, FIRST_MAP_VTABLE);
    e.mem.set_u32(base + 0x14, first);
    let second = program_map(e, SECOND_MAP_BASE_VTABLE, SECOND_MAP_VTABLE);
    e.mem.set_u32(base + 0x18, second);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_2f70, fn_00e92f70(u32)),
        entry!(0x00e9_3ad0, fn_00e93ad0(Ptr) -> Ptr),
        entry!(
            0x00e9_3d80,
            ni_d3d_shader_program_factory_create_shader_creator_map()
        ),
        entry!(0x00e9_3df0, fn_00e93df0()),
        entry!(0x00e9_3e70, ni_d3d_shader_program_factory_register_shader_creator(Ptr, u32) -> bool),
        entry!(
            0x00b7_1300,
            ni_t_pointer_list_base_remove_all(Ptr<crate::types::NiTPointerList>)
        ),
    ]
}

/// The node pool's lock object (`this` of the lock enter `0040fbf0`, thiscall,
/// one word); its owner word.
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
/// The lock's entry count, which the inlined leave decrements.
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
/// The lock enter (`0040fbf0`; thiscall, one word).
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// Head of the node pool's free list.
const NODE_POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00b71300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,char *>::RemoveAll`
/// (engine map): returns every node of the list to the shared node pool:
/// for each node clears its element (+0x08), enters the pool's lock
/// (`0040fbf0(0)`), clears its previous link (+0x04), pushes it on the free
/// list at `011c5f58` and leaves the lock (the count at `011c5f84` drops by
/// one; at zero the owner word `011c5f80` is cleared). Then clears the
/// count, head and tail.
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr<crate::types::NiTPointerList>) {
    use crate::types::NiTPointerList;
    let mut next = e.get(this, NiTPointerList::m_pkHead);
    while next != 0 {
        let node = next;
        next = e.mem.u32(node);
        e.mem.set_u32(node + 8, 0);
        e.call(LOCK_ENTER, &[NODE_POOL_LOCK, 0]);
        e.mem.set_u32(node + 4, 0);
        let free = e.mem.u32(NODE_POOL_FREE_LIST);
        e.mem.set_u32(node, free);
        let count = e.mem.u32(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, count);
        e.mem.set_u32(NODE_POOL_FREE_LIST, node);
        if count == 0 {
            e.mem.set_u32(NODE_POOL_LOCK, 0);
        }
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

#[cfg(test)]
mod remove_all_tests {
    use super::*;

    #[test]
    fn remove_all_returns_each_node_to_the_pool_and_empties_the_list() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        // The lock enter double takes the lock: owner 1, count + 1.
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        // Already held once by the caller: the owner stays after each leave.
        e.mem.set_u32(NODE_POOL_LOCK_COUNT, 1);
        e.mem.set_u32(NODE_POOL_FREE_LIST, 0x0099_0000);
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0);
        e.mem.set_u32(first + 8, 0x11);
        e.mem.set_u32(second, 0);
        e.mem.set_u32(second + 4, first);
        e.mem.set_u32(second + 8, 0x22);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        let locks: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == LOCK_ENTER)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(locks, vec![vec![NODE_POOL_LOCK, 0]; 2]);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), second);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first), 0x0099_0000);
        for node in [first, second] {
            assert_eq!(e.mem.u32(node + 4), 0);
            assert_eq!(e.mem.u32(node + 8), 0);
        }
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 1);
        assert_eq!(e.mem.bytes(list, 0xc), vec![0; 0xc]);
    }

    #[test]
    fn remove_all_clears_the_owner_when_the_lock_count_reaches_zero() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register(LOCK_ENTER, |e, _| {
            e.mem.set_u32(NODE_POOL_LOCK, 1);
            let count = e.mem.u32(NODE_POOL_LOCK_COUNT);
            e.mem.set_u32(NODE_POOL_LOCK_COUNT, count + 1);
            0u32.into_ret()
        });
        let list = e.mem.alloc(0xc);
        let node = e.mem.alloc(0xc);
        e.mem.set_u32(list, node);
        e.mem.set_u32(list + 4, node);
        e.mem.set_u32(list + 8, 1);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(NODE_POOL_LOCK), 0);
        assert_eq!(e.mem.u32(NODE_POOL_FREE_LIST), node);
        // An empty list only clears its words.
        e.call_log = Some(vec![]);
        e.call(0x00b7_1300, &[list]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn creator_map_is_made_once_with_a_zeroed_slot_block() {
        let mut e = engine();
        e.call(0x00e9_3df0, &args![]);
        let map = e.mem.u32(CREATOR_MAP_SLOT);
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), 0x010f_0f50);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(e.mem.u8(map + 0x10), 1);
        let slots = e.mem.u32(map + 8);
        assert_eq!(e.mem.bytes(slots, 0x94), vec![0; 0x94]);
        e.call(0x00e9_3df0, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), map);
    }

    #[test]
    fn failed_allocation_stores_null() {
        let mut e = engine();
        e.mem.set_u32(CREATOR_MAP_SLOT, 0);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3d80, &args![]);
        assert_eq!(e.mem.u32(CREATOR_MAP_SLOT), 0);
    }

    fn register(name: &[u8]) -> (bool, Vec<(Vec<u8>, u32)>) {
        let mut e = engine();
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().position(|c| u32::from(*c) == a[1]) {
                Some(at) => (a[0] + at as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(TOLOWER, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        let stored: Rc<RefCell<Vec<(Vec<u8>, u32)>>> = Rc::default();
        let log = stored.clone();
        e.register_double(MAP_SET_AT, move |e, a| {
            log.borrow_mut().push((e.mem.cstr(a[1]), a[2]));
            Ret::default()
        });
        let text = e.mem.alloc(0x40);
        e.mem.set_cstr(text, name);
        let accepted = e.call(0x00e9_3e70, &args![text, 0x1234u32]).bool();
        let entries = stored.borrow().clone();
        (accepted, entries)
    }

    #[test]
    fn registers_the_lower_case_copy() {
        assert_eq!(
            register(b"NiD3DShader"),
            (true, vec![(b"nid3dshader".to_vec(), 0x1234)])
        );
    }

    #[test]
    fn refuses_names_with_a_dot() {
        assert_eq!(register(b"Shader.vsh"), (false, vec![]));
    }

    #[test]
    fn refuses_a_null_name() {
        let mut e = engine();
        assert!(!e.call(0x00e9_3e70, &args![0u32, 1u32]).bool());
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e92f70_swaps_the_held_device() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.put_vtable(0x6000, &[0, 0x6104, 0x6108]);
        e.register(0x6104, |_, _| Ret::default());
        e.register(0x6108, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let renderer = e.mem.alloc(0x900);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x66);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2f70, &args![renderer]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), device);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0x66);
        e.call(0x00e9_2f70, &args![0u32]);
        assert_eq!(e.mem.u32(FACTORY_DEVICE_0126FA18), 0);
        assert_eq!(e.mem.u32(FACTORY_LIBRARY_0126FA20), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6104, vec![device])));
        assert!(log.contains(&(0x6108, vec![device])));
    }

    #[test]
    fn fn_00e93ad0_makes_the_two_maps() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00e9_3ad0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROGRAM_FACTORY_VTABLE);
        let first = e.mem.u32(this + 0x14);
        let second = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(first), FIRST_MAP_VTABLE);
        assert_eq!(e.mem.u32(second), SECOND_MAP_VTABLE);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e9_3ad0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }
}
