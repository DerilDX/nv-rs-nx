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

/// The first node of a pointer map (`NiTMapBase`: vtable +0, bucket count +4, buckets +8): the
/// first non-null bucket, or 0.
fn map_first(e: &mut Engine, map: u32) -> u32 {
    let count = e.mem.u32(map.wrapping_add(4));
    let buckets = e.mem.u32(map.wrapping_add(8));
    for index in 0..count {
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
    }
    0
}

/// The node after `node` (next +0, key +4): its next, or the first non-null bucket after the
/// key's bucket (the map's virtual slot 4, `thiscall` with the key), or 0.
fn map_next(e: &mut Engine, map: u32, node: u32) -> u32 {
    let next = e.mem.u32(node);
    if next != 0 {
        return next;
    }
    let key = e.mem.u32(node.wrapping_add(4));
    let mut index = e.vcall(map, 4, &args![key]).u32().wrapping_add(1);
    while index < e.mem.u32(map.wrapping_add(4)) {
        let buckets = e.mem.u32(map.wrapping_add(8));
        let node = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
        if node != 0 {
            return node;
        }
        index += 1;
    }
    0
}

/// What the destructor `00e8ceb0` uses besides the constructor's: the interlocked decrement
/// import, `DeleteCriticalSection` (import slot), the free `00aa10f0` (`cdecl`, one word), the
/// Ni free `00aa1460` (`cdecl`: block, size) and the buffer entry clean-up `00e95b80` (unnamed;
/// `thiscall`, no stack words).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;
const ENTRY_CLEAN_UP_00E95B80: u32 = 0x00e9_5b80;

// Translated from 00e8ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the `NiDX9VertexBufferManager` destructor,
/// `thiscall`, no stack words. Stores the class's table; then for each of the three maps
/// (+0xc, +0x1c, +0x2c) walks its nodes (the next node found before the current one's value is
/// freed): a value (a 0x1c-byte record) has five entries at +8 to +0x18, each one not null
/// releasing its +0x20 (slot 8, `stdcall`) when set, cleaned up (`00e95b80`) and freed (0x3c
/// bytes); then the record is freed. Then `00b601a0` on each map, the device (+8) released
/// (slot 8, no null check) and cleared, +0x40 freed (`00aa10f0`), the critical section (+0x80)
/// deleted, and each map from the last undone: its two tables with `00b601a0` after each and
/// its buckets freed; finally the `NiRefObject` table and the global object count counted down.
pub fn fn_00e8ceb0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_DX9_VERTEX_BUFFER_MANAGER_VTABLE);
    for offset in [0xcu32, 0x1c, 0x2c] {
        let map = base.wrapping_add(offset);
        let mut node = map_first(e, map);
        while node != 0 {
            let value = e.mem.u32(node.wrapping_add(8));
            node = map_next(e, map, node);
            if value == 0 {
                continue;
            }
            for slot in 0..5u32 {
                let entry = e.mem.u32(value.wrapping_add(8 + 4 * slot));
                if entry == 0 {
                    continue;
                }
                let buffer = e.mem.u32(entry.wrapping_add(0x20));
                if buffer != 0 {
                    let buffer = e.mem.u32(entry.wrapping_add(0x20));
                    e.vcall(buffer, 8, &args![]);
                }
                e.call(ENTRY_CLEAN_UP_00E95B80, &args![entry]);
                e.call(NI_FREE_00AA1460, &args![entry, 0x3cu32]);
            }
            e.call(NI_FREE_00AA1460, &args![value, 0x1cu32]);
        }
    }
    for offset in [0xcu32, 0x1c, 0x2c] {
        e.call(MAP_00B601A0, &args![base.wrapping_add(offset)]);
    }
    let device = e.mem.u32(base.wrapping_add(8));
    e.vcall(device, 8, &args![]);
    let block = e.mem.u32(base.wrapping_add(0x40));
    e.mem.set_u32(base.wrapping_add(8), 0);
    e.call(FREE_00AA10F0, &args![block]);
    e.call(
        IMPORT_DELETE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    for offset in [0x2cu32, 0x1c, 0xc] {
        let map = base.wrapping_add(offset);
        e.mem.set_u32(map, VB_MAP_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        e.mem.set_u32(map, VB_MAP_BASE_VTABLE);
        e.call(MAP_00B601A0, &args![map]);
        let buckets = e.mem.u32(map.wrapping_add(8));
        e.call(FREE_00AA10F0, &args![buckets]);
    }
    e.mem.set_u32(base, NI_REF_OBJECT_VTABLE);
    e.call(IMPORT_INTERLOCKED_DECREMENT, &args![OBJECT_COUNT_011F4400]);
}

// Translated from 00e8d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e8c570`): the scalar deleting destructor, `thiscall` with
/// one word (the flags): the destructor `00e8ceb0`, then with bit 0 set the 0x100 bytes freed
/// (`00aa1460`). Answers `this`.
pub fn fn_00e8d200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e8ceb0(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_00AA1460, &args![this, 0x100u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e8_d0e0, fn_00e8d0e0(Ptr, u32) -> Ptr),
        entry!(0x00e8_ceb0, fn_00e8ceb0(Ptr)),
        entry!(0x00e8_d200, fn_00e8d200(Ptr, u32) -> Ptr),
    ]
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

#[cfg(test)]
mod b052_tests {
    use super::*;

    const KEY_TO_INDEX: u32 = 0x0e30_0004;
    const RELEASE: u32 = 0x0e30_0008;
    const MAP_VTABLE: u32 = 0x0e31_0000;
    const DEVICE_VTABLE: u32 = 0x0e32_0000;

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A manager whose first map holds two nodes in buckets 1 and 3 (keys 1 and 3), the first
    /// value with one entry holding a buffer at +0x20.
    fn rig() -> (Engine, u32, [u32; 4]) {
        let mut e = Engine::new();
        for no_op in [
            IMPORT_INTERLOCKED_DECREMENT,
            IMPORT_DELETE_CRITICAL_SECTION,
            FREE_00AA10F0,
            NI_FREE_00AA1460,
            ENTRY_CLEAN_UP_00E95B80,
            MAP_00B601A0,
            RELEASE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(KEY_TO_INDEX, |_, a| a[1].into_ret());
        e.put_vtable(MAP_VTABLE, &[0, KEY_TO_INDEX]);
        e.put_vtable(DEVICE_VTABLE, &[0, 0, RELEASE]);
        let this = e.mem.alloc(0x100);
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, DEVICE_VTABLE);
        e.mem.set_u32(this + 8, device);
        e.mem.set_u32(this + 0x40, 0x4040);
        for offset in [0xc, 0x1c, 0x2c] {
            let buckets = e.mem.alloc(0x10);
            e.mem.set_u32(this + offset, MAP_VTABLE);
            e.mem.set_u32(this + offset + 4, 4);
            e.mem.set_u32(this + offset + 8, buckets);
        }
        let buckets = e.mem.u32(this + 0xc + 8);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first + 4, 1);
        e.mem.set_u32(second + 4, 3);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(buckets + 0xc, second);
        let value = e.mem.alloc(0x1c);
        e.mem.set_u32(first + 8, value);
        let entry = e.mem.alloc(0x3c);
        let buffer = e.mem.alloc(4);
        e.mem.set_u32(buffer, DEVICE_VTABLE);
        e.mem.set_u32(entry + 0x20, buffer);
        e.mem.set_u32(value + 0x10, entry);
        (e, this, [device, value, entry, buffer])
    }

    #[test]
    fn fn_00e8ceb0_frees_the_records_and_undoes_the_maps() {
        let (mut e, this, [device, value, entry, buffer]) = rig();
        e.call_log = Some(vec![]);
        e.call(0x00e8_ceb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, NI_FREE_00AA1460),
            vec![vec![entry, 0x3c], vec![value, 0x1c]]
        );
        assert_eq!(calls(&log, ENTRY_CLEAN_UP_00E95B80), vec![vec![entry]]);
        assert_eq!(calls(&log, RELEASE), vec![vec![buffer], vec![device]]);
        // The walk asked for the bucket of each node without a next.
        assert_eq!(
            calls(&log, KEY_TO_INDEX),
            vec![vec![this + 0xc, 1], vec![this + 0xc, 3]]
        );
        let maps: Vec<u32> = calls(&log, MAP_00B601A0).iter().map(|w| w[0]).collect();
        assert_eq!(
            maps,
            vec![
                this + 0xc,
                this + 0x1c,
                this + 0x2c,
                this + 0x2c,
                this + 0x2c,
                this + 0x1c,
                this + 0x1c,
                this + 0xc,
                this + 0xc
            ]
        );
        assert_eq!(calls(&log, FREE_00AA10F0)[0], vec![0x4040]);
        assert_eq!(calls(&log, FREE_00AA10F0).len(), 4);
        assert!(log.contains(&(IMPORT_DELETE_CRITICAL_SECTION, vec![this + 0x80])));
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), VB_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert!(log.contains(&(IMPORT_INTERLOCKED_DECREMENT, vec![OBJECT_COUNT_011F4400])));
    }

    #[test]
    fn fn_00e8d200_frees_with_bit_0() {
        let (mut e, this, _) = rig();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_d200, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, NI_FREE_00AA1460).last().unwrap(),
            &vec![this, 0x100]
        );
        let (mut e, this, _) = rig();
        e.call_log = Some(vec![]);
        e.call(0x00e8_d200, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_FREE_00AA1460).len(), 2);
    }
}
