//! `NiXenonRenderer/run_00e8c570` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiRefObject` virtual table and the global object count it counts up (`011f4400`,
/// `InterlockedIncrement`), the class's virtual table (RTTI `NiDX9VertexBufferManager`), the
/// maps' virtual tables (RTTI `NiTMapBase<..., unsigned int,
/// NiDX9VertexBufferManager::NiDX9VBInfo *>`, then `NiTPointerMap<...>`), the bucket allocation
/// `00aa1070`, `memset`, `InitializeCriticalSection` (import slot) and the map set-up
/// `00b601a0` (unnamed; `thiscall`, no stack words).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const OBJECT_COUNT_011F4400: u32 = 0x011f_4400;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE: u32 = 0x010e_fe94;
const VB_MAP_BASE_VTABLE: u32 = 0x010e_fe54;
const VB_MAP_VTABLE: u32 = 0x010e_fe74;
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x00ec_61c0;
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const MAP_00B601A0: u32 = 0x00b6_01a0;

// Translated from 00e8d0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the `NiDX9VertexBufferManager` constructor (the
/// RTTI of the virtual table it stores), `thiscall` with the device. The `NiRefObject` start
/// (its table, count 0, the global object count up), the class's table, three maps at +0xc,
/// +0x1c, +0x2c (0x25 buckets each, cleared), +0x40 to +0x4c cleared, the critical section at
/// +0x80, the device at +8 (counted: its slot 4), `00b601a0` on each map and +0x3c cleared.
/// Answers `this`.
pub fn fn_00e8d0e0(e: &mut Engine, this: Ptr, device: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), 0);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![OBJECT_COUNT_011F4400]);
    e.mem.set_u32(base, NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
    for offset in [0xcu32, 0x1c, 0x2c] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, VB_MAP_BASE_VTABLE);
        e.mem.set_u32(map.wrapping_add(4), 0x25);
        e.mem.set_u32(map.wrapping_add(0xc), 0);
        let buckets = e.call(ALLOCATE_00AA1070, &args![0x94u32]).u32();
        e.mem.set_u32(map.wrapping_add(8), buckets);
        let size = e.mem.u32(map.wrapping_add(4)).wrapping_mul(4);
        e.call(MEMSET, &args![buckets, 0u32, size]);
        e.mem.set_u32(map, VB_MAP_VTABLE);
    }
    for offset in [0x40u32, 0x44, 0x48, 0x4c] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base.wrapping_add(8), device);
    e.vcall(device, 4, &args![]);
    for offset in [0xcu32, 0x1c, 0x2c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    e.mem.set_u32(base.wrapping_add(0x3c), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_d0e0, fn_00e8d0e0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn fn_00e8d0e0_builds_the_vertex_buffer_manager() {
        let mut e = Engine::new();
        for no_op in [
            IMPORT_INTERLOCKED_INCREMENT,
            MEMSET,
            IMPORT_INITIALIZE_CRITICAL_SECTION,
            MAP_00B601A0,
            0x6100,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.put_vtable(0x6000, &[0, 0x6100]);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x3c, 7);
        assert_eq!(e.call(0x00e8_d0e0, &args![this, device]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
        for offset in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(this + offset), VB_MAP_VTABLE);
            assert_eq!(e.mem.u32(this + offset + 4), 0x25);
        }
        assert_eq!(e.mem.u32(this + 8), device);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        let log = e.call_log.clone().unwrap();
        let set_ups: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == MAP_00B601A0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(set_ups, vec![this + 0xc, this + 0x1c, this + 0x2c]);
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        assert!(log.contains(&(0x6100, vec![device])));
    }
}
