//! `NiXenonRenderer/nid3dgeometrygroupmanager.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The allocation `00aa13e0` (`cdecl`, size), the base constructor `00a7f810` (unnamed;
/// `thiscall`, no stack words) and the class's virtual table (RTTI
/// `NiD3DGeometryGroupManager`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const BASE_CONSTRUCT_00A7F810: u32 = 0x00a7_f810;
const NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE: u32 = 0x010e_f718;

// Translated from 00e88eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dgeometrygroupmanager.cpp`): makes an
/// `NiD3DGeometryGroupManager` (0x1c bytes; the RTTI of the virtual table it stores), `cdecl`:
/// the device and a second word. A made object gets the base constructor, the class's virtual
/// table and +4 to +0x14 and the byte +0x18 cleared; null when the allocation fails, and the
/// stores below still go to it. The device (+0x10) is counted (`AddRef`, its virtual slot 4)
/// and the second word kept at +0x14. Answers the object.
pub fn fn_00e88eb0(e: &mut Engine, device: u32, second: u32) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0x1cu32]).u32();
    let manager = if block != 0 {
        e.call(BASE_CONSTRUCT_00A7F810, &args![block]);
        e.mem.set_u32(block, NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
        for offset in [4u32, 8, 0xc, 0x10, 0x14] {
            e.mem.set_u32(block + offset, 0);
        }
        e.mem.set_u8(block + 0x18, 0);
        block
    } else {
        0
    };
    e.mem.set_u32(manager.wrapping_add(0x10), device);
    e.vcall(device, 4, &args![]);
    e.mem.set_u32(manager.wrapping_add(0x14), second);
    manager
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_8eb0, fn_00e88eb0(u32, u32) -> u32)]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e88eb0_makes_the_manager_and_counts_the_device() {
        let mut e = Engine::new();
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A7F810, |_, a| a[0].into_ret());
        e.put_vtable(0x6000, &[0, 0x6100]);
        e.register(0x6100, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let manager = e.call(0x00e8_8eb0, &args![device, 0x77u32]).u32();
        assert_eq!(e.mem.u32(manager), NI_D3D_GEOMETRY_GROUP_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(manager + 0x10), device);
        assert_eq!(e.mem.u32(manager + 0x14), 0x77);
        assert_eq!(e.mem.u8(manager + 0x18), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6100, vec![device])));
    }
}
