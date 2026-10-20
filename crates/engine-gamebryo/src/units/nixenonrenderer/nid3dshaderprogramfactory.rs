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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_3d80,
            ni_d3d_shader_program_factory_create_shader_creator_map()
        ),
        entry!(0x00e9_3df0, fn_00e93df0()),
        entry!(0x00e9_3e70, ni_d3d_shader_program_factory_register_shader_creator(Ptr, u32) -> bool),
    ]
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
