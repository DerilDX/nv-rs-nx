//! `BSHavok/bhkworldobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The key used when the caller passes none (an address in `.data`, used as
/// the key value itself).
const DEFAULT_KEY_011F4280: u32 = 0x011f_4280;
/// `00c6d840`: thiscall on the Havok world object, (output pair, key, flag)
/// with `RET 0xc`; returns the output pair, whose first word is the value.
const GET_PROPERTY: u32 = 0x00c6_d840;

// Translated from 00c85c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWorldObject::GetAVObject` (map name). Thiscall, one word (`RET 4`).
/// Looks `key` (the address `011f4280` when null) up in the property table
/// of the Havok world object at `this + 8` (array at `+0x78`, count at
/// `+0x7c`, 0x10 bytes each, the key in the first word); when found, asks
/// `00c6d840` for the property and returns its first word, otherwise 0.
pub fn bhk_world_object_get_av_object(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let key = if key == 0 { DEFAULT_KEY_011F4280 } else { key };
    let world_object = e.mem.u32(this.addr() + 8);
    if world_object == 0 {
        return 0;
    }
    let count = e.mem.i32(world_object + 0x7c);
    if count <= 0 {
        return 0;
    }
    let table = e.mem.u32(world_object + 0x78);
    let found = (0..count as u32).any(|i| e.mem.u32(table + 0x10 * i) == key);
    if !found {
        return 0;
    }
    e.with_stack(8, |e, out| {
        let property = e
            .call(GET_PROPERTY, &args![world_object, out, key, 0u32])
            .u32();
        e.mem.u32(property)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c85c80, bhk_world_object_get_av_object(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world_object_00c85c80(e: &mut Engine, keys: &[u32]) -> Ptr {
        let table = e.mem.alloc(0x10 * keys.len() as u32 + 0x10);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_u32(table + 0x10 * i as u32, *key);
        }
        let world_object = e.mem.alloc(0x100);
        e.mem.set_u32(world_object + 0x78, table);
        e.mem.set_i32(world_object + 0x7c, keys.len() as i32);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, world_object);
        Ptr::<()>::new(this)
    }

    #[test]
    fn bhk_world_object_get_av_object_returns_the_found_propertys_value() {
        let mut e = Engine::new();
        e.register(GET_PROPERTY, |e, a| {
            e.mem.set_u32(a[1], a[2] + 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = world_object_00c85c80(&mut e, &[0x10, 0x20, DEFAULT_KEY_011F4280]);
        assert_eq!(e.call(0x00c8_5c80, &args![this, 0x20u32]).u32(), 0x21);
        // A null key means the default key.
        assert_eq!(
            e.call(0x00c8_5c80, &args![this, 0u32]).u32(),
            DEFAULT_KEY_011F4280 + 1
        );
    }

    #[test]
    fn bhk_world_object_get_av_object_is_zero_when_nothing_matches() {
        let mut e = Engine::new();
        let this = world_object_00c85c80(&mut e, &[0x10, 0x20]);
        assert_eq!(e.call(0x00c8_5c80, &args![this, 0x30u32]).u32(), 0);
        let empty = world_object_00c85c80(&mut e, &[]);
        assert_eq!(e.call(0x00c8_5c80, &args![empty, 0x30u32]).u32(), 0);
        let none = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00c8_5c80, &args![Ptr::<()>::new(none), 0x30u32])
                .u32(),
            0
        );
    }
}
