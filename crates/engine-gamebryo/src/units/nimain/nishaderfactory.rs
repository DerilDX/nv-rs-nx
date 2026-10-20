//! `NiMain/nishaderfactory.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderFactory::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00aa05a0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x24 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_shader_factory_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00aa_05a0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x24u32]);
    }
    this
}

/// The shader factory instance (`011f6044`, what `NiShaderFactory::GetInstance` `00a9fff0`
/// answers), or null. Its +8 is the global-constant map, a `NiTFixedStringMap` of
/// `NiGlobalConstantEntry` pointers (map +4 bucket count, +8 bucket array; each item: next,
/// key handle, entry).
const SHADER_FACTORY: u32 = 0x011f_6044;
/// The empty `NiFixedString` handle's holder (`0109b220`).
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `InterlockedIncrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa0250` (`thiscall` on the map, one stack word: the address of a key handle), unnamed
/// in the engine map; it takes the key's item out of the map.
const MAP_REMOVE_00AA0250: u32 = 0x00aa_0250;

// Translated from 00aa04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderFactory::ReleaseGlobalShaderConstant` (Xbox PDB), `cdecl` with one word, the
/// address of the constant's `NiFixedString` name. Without the shader factory, or when the
/// name's bucket in the factory's map (handle modulo the bucket count) holds no item with that
/// handle or the item's entry is null: false. Otherwise: holds the name's string
/// (`InterlockedIncrement` of handle - 8, unless it is the empty string), copies the name's
/// handle into a local; when the entry's reference count (+4) is 1 the map's `00aa0250` is
/// given that local; releases one reference of the entry (its vtable slot +4 when the count
/// reaches 0) and of the local's string; true.
pub fn ni_shader_factory_release_global_shader_constant(e: &mut Engine, name: Ptr) -> bool {
    let mut factory = e.global::<u32>(SHADER_FACTORY);
    if factory == 0 {
        return false;
    }
    let handle = e.mem.u32(name.addr());
    // DIV dword ptr [factory + 0xc] (unsigned remainder; a zero count faults as the exe does).
    let bucket = handle % e.mem.u32(factory + 0xc);
    let mut item = e.mem.u32(
        e.mem
            .u32(factory + 0x10)
            .wrapping_add(bucket.wrapping_mul(4)),
    );
    loop {
        if item == 0 {
            return false;
        }
        if handle == e.mem.u32(item + 4) {
            break;
        }
        item = e.mem.u32(item);
    }
    let entry = e.mem.u32(item + 8);
    if entry == 0 {
        return false;
    }
    if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
        factory = e.global::<u32>(SHADER_FACTORY);
    }
    let kept = e.mem.u32(name.addr());
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), kept);
        if e.mem.u32(entry + 4) == 1 {
            e.call(MAP_REMOVE_00AA0250, &args![factory.wrapping_add(8), local]);
        }
    });
    if e.call(INTERLOCKED_DECREMENT, &args![entry + 4]).u32() == 0 {
        e.vcall(entry, 4, &args![]);
    }
    if kept != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![kept.wrapping_sub(8)]);
    }
    true
}

// Translated from 00a9fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderFactory::GetInstance` (Xbox PDB), no arguments: the factory singleton word at
/// `011f6044`.
pub fn ni_shader_factory_get_instance(e: &mut Engine) -> u32 {
    e.mem.u32(SHADER_FACTORY)
}

// Translated from 00aa0000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): when there is a factory
/// (`011f6044`), jumps to its vtable slot `+0x20` (no arguments; the callers
/// do not use a result).
pub fn fn_00aa0000(e: &mut Engine) {
    let factory = e.global::<u32>(SHADER_FACTORY);
    if factory != 0 {
        e.vcall(factory, 0x20, &args![]);
    }
}

/// `NiShaderFactory`'s vtable (`010a2164`, RTTI `.?AVNiShaderFactory@@`).
const NI_SHADER_FACTORY_VTABLE: u32 = 0x010a_2164;
/// The vtables the map at `+8` gets while it is taken down: `010a2154`
/// (RTTI `NiTFixedStringMap<NiGlobalConstantEntry *>`) and `010a2138` (RTTI
/// `NiTFixedStringMapBase<..>`).
const CONSTANT_MAP_VTABLE: u32 = 0x010a_2154;
const CONSTANT_MAP_BASE_VTABLE: u32 = 0x010a_2138;
/// `NiTFixedStringMapBase<..>::GetNext` (`00a8f770`, `thiscall` on the map,
/// `RET 0xc`: the position, the key and the value, all by address).
const MAP_GET_NEXT: u32 = 0x00a8_f770;
/// `NiTFixedStringMapBase<..>::RemoveAll` (`00aa01f0`, `thiscall`, no stack
/// words).
const MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// The Gamebryo free (`00aa10f0`, `cdecl`, one word).
const NI_FREE: u32 = 0x00aa_10f0;
/// `NiRefObject`'s vtable (`0101dce4`) and its object count (`011f4400`).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;

// Translated from 00aa05a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShaderFactory::~NiShaderFactory` (engine map name), `thiscall`, no stack
/// words. Stores the factory's vtable; when there is a factory in `011f6044`
/// its slot `+0x20` runs, then (when there still is one) its slot `+0x34`. The
/// global-constant map at `+8` (hash size `+0xc`, buckets `+0x10`) is walked
/// from its first item: for each `GetNext` (key starting as the empty name,
/// value 0) with a value, the constant is released
/// (`ReleaseGlobalShaderConstant` with the key's address), and the key gives
/// up its reference (unless it is the empty name). When `011f6044` (as last
/// read) is this factory it is cleared. Then the map is emptied under each of
/// its two vtables (`RemoveAll`), its buckets freed, `NiRefObject`'s vtable
/// stored and the object count decremented.
pub fn ni_shader_factory_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, NI_SHADER_FACTORY_VTABLE);
    let mut factory = e.global::<u32>(SHADER_FACTORY);
    if factory != 0 {
        e.vcall(factory, 0x20, &args![]);
        factory = e.global::<u32>(SHADER_FACTORY);
        if factory != 0 {
            e.vcall(factory, 0x34, &args![]);
            factory = e.global::<u32>(SHADER_FACTORY);
        }
    }
    let map = t + 8;
    let size = e.mem.u32(map + 4);
    let buckets = e.mem.u32(map + 8);
    let mut first = 0;
    let mut index = 0;
    while index < size {
        let item = e.mem.u32(buckets.wrapping_add(4 * index));
        if item != 0 {
            first = item;
            break;
        }
        index += 1;
    }
    if first != 0 {
        e.with_stack(12, |e, frame| {
            // The frame: the key (+0), the value (+4), the position (+8).
            let key = frame.addr();
            let value = key + 4;
            let position = key + 8;
            e.mem.set_u32(position, first);
            loop {
                let empty = e.global::<u32>(FIXED_STRING_EMPTY);
                e.mem.set_u32(key, empty);
                e.mem.set_u32(value, 0);
                e.call(MAP_GET_NEXT, &args![map, position, key, value]);
                if e.mem.u32(value) != 0 {
                    ni_shader_factory_release_global_shader_constant(e, Ptr::new(key));
                }
                let name = e.mem.u32(key);
                if name != e.global::<u32>(FIXED_STRING_EMPTY) {
                    e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
                }
                if e.mem.u32(position) == 0 {
                    break;
                }
            }
        });
        factory = e.global::<u32>(SHADER_FACTORY);
    }
    if factory == t {
        e.set_global::<u32>(SHADER_FACTORY, 0);
    }
    e.mem.set_u32(map, CONSTANT_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    e.mem.set_u32(map, CONSTANT_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![map]);
    let buckets = e.mem.u32(map + 8);
    e.call(NI_FREE, &args![buckets]);
    e.mem.set_u32(t, NI_REF_OBJECT_VTABLE);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa06c0, ni_shader_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00aa_04e0, ni_shader_factory_release_global_shader_constant(Ptr) -> bool),
        entry!(0x00a9_fff0, ni_shader_factory_get_instance() -> u32),
        entry!(0x00aa_0000, fn_00aa0000()),
        entry!(0x00aa_05a0, ni_shader_factory_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_shader_factory_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00aa_05a0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00aa06c0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00aa06c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x24]]);
        assert_eq!(calls_to(&e, 0x00aa_05a0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_1000;
    const ENTRY_RELEASE: u32 = 0x0f00_1004;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn count(e: &mut Engine, target: u32, step: i32) -> Ret {
        let v = e.mem.i32(target).wrapping_add(step);
        e.mem.set_i32(target, v);
        (v as u32).into_ret()
    }

    /// A map of 4 buckets; bucket 1 holds an item of handle `0x8001` and behind it the item
    /// of handle `0x8005` with `entry` (reference count `refs`).
    fn rig(refs: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x100);
        e.map(0x0109_b000, 0x400);
        e.set_global(FIXED_STRING_EMPTY, 0x0000_9000u32);
        e.register(INTERLOCKED_INCREMENT, |e, a| count(e, a[0], 1));
        e.register(INTERLOCKED_DECREMENT, |e, a| count(e, a[0], -1));
        e.register(MAP_REMOVE_00AA0250, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, ENTRY_RELEASE]);
        e.register(ENTRY_RELEASE, |_, _| Ret::default());
        e.map(0x7ff0, 0x20);
        e.mem.set_u32(0x8005 - 8, 3);
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 4);
        e.mem.set_u32(map + 0x10, buckets);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, VTABLE);
        e.mem.set_u32(entry + 4, refs);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, 0x8005);
        e.mem.set_u32(item + 8, entry);
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(other, item);
        e.mem.set_u32(other + 4, 0x8001);
        e.mem.set_u32(buckets + 4, other);
        e.set_global(SHADER_FACTORY, map);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x8005);
        e.call_log = Some(vec![]);
        (e, map, entry, name)
    }

    #[test]
    fn fn_00aa04e0_releases_the_last_reference_and_removes_the_entry() {
        let (mut e, map, entry, name) = rig(1);
        assert!(e.call(0x00aa_04e0, &args![name]).bool());
        let removed = calls(&e, MAP_REMOVE_00AA0250);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], map + 8);
        assert_eq!(e.mem.u32(entry + 4), 0);
        assert_eq!(calls(&e, ENTRY_RELEASE), vec![vec![entry]]);
        // Held once and released once: unchanged.
        assert_eq!(e.mem.u32(0x8005 - 8), 3);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![0x8005 - 8]]);
    }

    #[test]
    fn fn_00aa04e0_with_more_references_only_releases_one() {
        let (mut e, _, entry, name) = rig(2);
        assert!(e.call(0x00aa_04e0, &args![name]).bool());
        assert!(calls(&e, MAP_REMOVE_00AA0250).is_empty());
        assert!(calls(&e, ENTRY_RELEASE).is_empty());
        assert_eq!(e.mem.u32(entry + 4), 1);
    }

    #[test]
    fn fn_00aa04e0_with_the_empty_string_holds_nothing() {
        let (mut e, _, entry, name) = rig(2);
        // The name's handle is the empty string's.
        e.set_global(FIXED_STRING_EMPTY, 0x8005u32);
        assert!(e.call(0x00aa_04e0, &args![name]).bool());
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![entry + 4]]);
    }

    #[test]
    fn fn_00aa04e0_answers_false_without_map_item_or_entry() {
        let (mut e, map, entry, name) = rig(1);
        // A handle of bucket 1 that no item holds.
        e.mem.set_u32(name, 0x8009);
        assert!(!e.call(0x00aa_04e0, &args![name]).bool());
        // An empty bucket.
        e.mem.set_u32(name, 0x8002);
        assert!(!e.call(0x00aa_04e0, &args![name]).bool());
        // A null entry.
        e.mem.set_u32(name, 0x8005);
        let other = e.mem.u32(e.mem.u32(map + 0x10) + 4);
        let item = e.mem.u32(other);
        e.mem.set_u32(item + 8, 0);
        assert!(!e.call(0x00aa_04e0, &args![name]).bool());
        e.set_global(SHADER_FACTORY, 0u32);
        assert!(!e.call(0x00aa_04e0, &args![name]).bool());
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.mem.u32(entry + 4), 1);
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn ni_shader_factory_get_instance_reads_the_global() {
        let mut e = Engine::new();
        e.map(SHADER_FACTORY, 4);
        e.set_global::<u32>(SHADER_FACTORY, 0x4321);
        assert_eq!(e.call(0x00a9_fff0, &args![]).u32(), 0x4321);
    }

    #[test]
    fn fn_00aa0000_calls_the_factory_slot_when_there_is_one() {
        const SLOT: u32 = 0x0124_0000;
        let mut e = Engine::new();
        e.map(SHADER_FACTORY, 4);
        e.register(SLOT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_0000, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, SLOT);
        let factory = e.mem.alloc(8);
        e.mem.set_u32(factory, vtable);
        e.set_global::<u32>(SHADER_FACTORY, factory);
        e.call_log = Some(vec![]);
        e.call(0x00aa_0000, &args![]);
        assert_eq!(e.call_log.take().unwrap()[1], (SLOT, vec![factory]));
    }

    const SLOT_20: u32 = 0x0124_1000;
    const SLOT_34: u32 = 0x0124_1010;

    /// A factory with the map buckets `buckets` (item addresses or 0); `GetNext`
    /// hands out `entries` (key, value, next position) in turn.
    fn factory(e: &mut Engine, buckets: &[u32], entries: Vec<(u32, u32, u32)>) -> u32 {
        e.map(SHADER_FACTORY, 4);
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global::<u32>(FIXED_STRING_EMPTY, 0x4444);
        for f in [
            SLOT_20,
            SLOT_34,
            MAP_REMOVE_ALL,
            NI_FREE,
            INTERLOCKED_DECREMENT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let mut entries = entries.into_iter();
        e.register_double(MAP_GET_NEXT, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x4444);
            assert_eq!(e.mem.u32(a[3]), 0);
            let (key, value, next) = entries.next().unwrap();
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            e.mem.set_u32(a[1], next);
            Ret::default()
        });
        // The destructor stores the factory's own vtable before calling its slots.
        e.map(NI_SHADER_FACTORY_VTABLE, 0x40);
        e.mem.set_u32(NI_SHADER_FACTORY_VTABLE + 0x20, SLOT_20);
        e.mem.set_u32(NI_SHADER_FACTORY_VTABLE + 0x34, SLOT_34);
        let this = e.mem.alloc(0x20);
        let table = e.mem.alloc(4 * buckets.len() as u32 + 4);
        for (i, b) in buckets.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *b);
        }
        e.mem.set_u32(this + 0xc, buckets.len() as u32);
        e.mem.set_u32(this + 0x10, table);
        this
    }

    #[test]
    fn ni_shader_factory_destructor_with_an_empty_map() {
        let mut e = Engine::new();
        let this = factory(&mut e, &[0, 0], vec![]);
        e.set_global::<u32>(SHADER_FACTORY, this);
        e.call_log = Some(vec![]);
        e.call(0x00aa_05a0, &args![this]);
        let log = e.call_log.take().unwrap();
        let table = e.mem.u32(this + 0x10);
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (SLOT_20, vec![this]),
                (SLOT_34, vec![this]),
                (MAP_REMOVE_ALL, vec![this + 8]),
                (MAP_REMOVE_ALL, vec![this + 8]),
                (NI_FREE, vec![table]),
                (INTERLOCKED_DECREMENT, vec![NI_REF_OBJECT_COUNT]),
            ]
        );
        assert_eq!(e.global::<u32>(SHADER_FACTORY), 0);
        assert_eq!(e.mem.u32(this), NI_REF_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 8), CONSTANT_MAP_BASE_VTABLE);
    }

    #[test]
    fn ni_shader_factory_destructor_releases_each_entry() {
        let mut e = Engine::new();
        let this = factory(
            &mut e,
            &[0, 0x9000],
            vec![(0x8008, 1, 0x9100), (0x4444, 0, 0)],
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa_05a0, &args![this]);
        let log = e.call_log.take().unwrap();
        let next: Vec<_> = log.iter().filter(|c| c.0 == MAP_GET_NEXT).collect();
        assert_eq!(next.len(), 2);
        assert_eq!(next[0].1[0], this + 8);
        let decrements: Vec<_> = log
            .iter()
            .filter(|c| c.0 == INTERLOCKED_DECREMENT)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(decrements, vec![0x8000, NI_REF_OBJECT_COUNT]);
        // No factory in the global: neither slot runs and the global stays.
        assert!(log.iter().all(|c| c.0 != SLOT_20 && c.0 != SLOT_34));
        assert_eq!(e.global::<u32>(SHADER_FACTORY), 0);
    }
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    #[test]
    fn get_instance_reads_the_singleton_word() {
        let mut e = Engine::new();
        e.map(SHADER_FACTORY, 4);
        e.mem.set_u32(SHADER_FACTORY, 0x1234_5678);
        assert_eq!(e.call(0x00a9_fff0, &args![]).u32(), 0x1234_5678);
    }
}
