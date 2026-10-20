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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa06c0, ni_shader_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00aa_04e0, ni_shader_factory_release_global_shader_constant(Ptr) -> bool),
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
