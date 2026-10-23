//! `NiXenonRenderer/nistaticgeometrygroup.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeometryGroup::AddObject` (engine map; thiscall, one word;
/// `nigeometrygroup.cpp`).
const GEOMETRY_GROUP_ADD_OBJECT: u32 = 0x00e9_8730;
/// `NiGeometryGroup::RemoveObject` (engine map; thiscall, one word;
/// `nigeometrygroup.cpp`).
const GEOMETRY_GROUP_REMOVE_OBJECT: u32 = 0x00e9_8750;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiGeometryBufferData::NiGeometryBufferData` (engine map; thiscall, no
/// stack words), run on a 0x54-byte block.
const BUFFER_DATA_CONSTRUCTOR: u32 = 0x00e9_47c0;
/// The record `00e948f0` looks for in the geometry's chain (each record's
/// word +4 is the next; the shape of an `NiRTTI` base-class chain, not
/// confirmed).
const RECORD_011F4A88: u32 = 0x011f_4a88;
/// The record `00e948f0` passes to the check `0043b300`.
const RECORD_011F4A70: u32 = 0x011f_4a70;
/// The check `0043b300` (`fallout shared/modelloader.cpp`; cdecl: record,
/// object; answers in AL).
const CHECK_0043B300: u32 = 0x0043_b300;
/// The cap on the geometry's low flag bit (`011bf558`, a word).
const FLAG_CAP_011BF558: u32 = 0x011b_f558;

/// The removal both `RemoveObject` overloads share: takes the buffer data
/// in `geometry`'s word `slot` out of the group, deletes it (its virtual
/// slot 0 with 1) and clears the word.
fn remove_buffer_data(e: &mut Engine, this: Ptr, geometry: Ptr, slot: u32) {
    let data = e.mem.u32(geometry.addr() + slot);
    if data != 0 {
        e.call(GEOMETRY_GROUP_REMOVE_OBJECT, &[this.addr(), data]);
        e.vcall(data, 0, &[1]);
        e.mem.set_u32(geometry.addr() + slot, 0);
    }
}

// Translated from 00e94740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::RemoveObject` (Xbox PDB; thiscall, one word):
/// for the geometry's buffer data at +0x34, when set: removes it from the
/// group (`NiGeometryGroup::RemoveObject`), deletes it and clears +0x34.
pub fn ni_static_geometry_group_remove_object(e: &mut Engine, this: Ptr, geometry: Ptr) {
    remove_buffer_data(e, this, geometry, 0x34);
}

// Translated from 00e94d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStaticGeometryGroup::RemoveObject` (Xbox PDB, the engine map's
/// `_ov2`; thiscall, one word): the same for the buffer data at +0x28 of
/// its argument.
pub fn ni_static_geometry_group_remove_object_ov2(e: &mut Engine, this: Ptr, part: Ptr) {
    remove_buffer_data(e, this, part, 0x28);
}

/// A new buffer data (0x54 bytes, constructed), or 0 when the allocation
/// fails.
fn new_buffer_data(e: &mut Engine) -> u32 {
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x54]).u32();
    if block != 0 {
        e.call(BUFFER_DATA_CONSTRUCTOR, &[block]).u32()
    } else {
        0
    }
}

// Translated from 00e948f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, three words; the second is never
/// read): gives `geometry` (or, when `part` is set, `part`) a new buffer
/// data and adds it to the group. Nothing happens when the target already
/// has one (`part` +0x28, `geometry` +0x34).
///
/// For `part`, the data's +0x3C is 5 when its 16-bit +0x22 is not 0,
/// else 4. For `geometry`, +0x3C becomes 4 when the record `011f4a88` is in
/// the chain the geometry's virtual slot +8 returns, else 5 when
/// `0043b300(011f4a70, geometry)` answers true (otherwise the constructor's
/// value stays). The data's +0x04 gets the geometry's flags: bit 22 when
/// its +0x28 is set, bit 23 when +0x24 is set, bit 21 when +0x24 is set and
/// the 16-bit +0x0C has a bit of 0xF000, and in bit 24 up its bit 0 capped
/// by the word at `011bf558`. Then `NiGeometryGroup::AddObject`. A failed
/// allocation leaves the data null and the exe writes through it anyway.
pub fn fn_00e948f0(e: &mut Engine, this: Ptr, geometry: Ptr, _unused_1: u32, part: Ptr) {
    let geometry = geometry.addr();
    let part = part.addr();
    let data;
    if part != 0 {
        if e.mem.u32(part + 0x28) != 0 {
            return;
        }
        data = new_buffer_data(e);
        let kind = u32::from(e.mem.u16(part + 0x22) != 0) + 4;
        e.mem.set_u32(data.wrapping_add(0x3c), kind);
        e.mem.set_u32(part + 0x28, data);
    } else {
        if e.mem.u32(geometry + 0x34) != 0 {
            return;
        }
        data = new_buffer_data(e);
        let mut record = e.vcall(geometry, 8, &[]).u32();
        let mut found = false;
        while record != 0 {
            if record == RECORD_011F4A88 {
                found = true;
                break;
            }
            record = e.mem.u32(record + 4);
        }
        if found {
            e.mem.set_u32(data.wrapping_add(0x3c), 4);
        } else if e.call(CHECK_0043B300, &[RECORD_011F4A70, geometry]).u8() != 0 {
            e.mem.set_u32(data.wrapping_add(0x3c), 5);
        }
        e.mem.set_u32(geometry + 0x34, data);
    }
    let has_28 = e.mem.u32(geometry + 0x28) != 0;
    let has_24 = e.mem.u32(geometry + 0x24) != 0;
    let high_flags = has_24 && e.mem.u16(geometry + 0xc) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(geometry + 0xc)) & 1;
    let cap = e.mem.u32(FLAG_CAP_011BF558);
    if low > cap {
        low = cap;
    }
    let mut bits = 0;
    if has_28 {
        bits = 0x40_0000;
    }
    if has_24 {
        bits |= 0x80_0000;
    }
    if high_flags {
        bits |= 0x20_0000;
    }
    e.mem.set_u32(data.wrapping_add(4), (low << 24) | bits);
    e.call(GEOMETRY_GROUP_ADD_OBJECT, &[this.addr(), data]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_4740,
            ni_static_geometry_group_remove_object(Ptr, Ptr)
        ),
        entry!(
            0x00e9_4d00,
            ni_static_geometry_group_remove_object_ov2(Ptr, Ptr)
        ),
        entry!(0x00e9_48f0, fn_00e948f0(Ptr, Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE: u32 = 0x0066_0000;
    const GET_CHAIN: u32 = 0x0066_0008;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011b_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.register(GEOMETRY_GROUP_ADD_OBJECT, |_, _| Ret::default());
        e.register(GEOMETRY_GROUP_REMOVE_OBJECT, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(BUFFER_DATA_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x3c, 0x77);
            a[0].into_ret()
        });
        e.register(CHECK_0043B300, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn buffer_data(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, DELETE);
        let data = e.mem.alloc(0x54);
        e.mem.set_u32(data, vtable);
        data
    }

    #[test]
    fn remove_object_deletes_the_data_at_34() {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0x34, data);
        e.call(0x00e9_4740, &[0x9000, geometry]);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_REMOVE_OBJECT),
            vec![vec![0x9000, data]]
        );
        assert_eq!(calls_to(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(geometry + 0x34), 0);
        e.call(0x00e9_4740, &[0x9000, geometry]);
        assert_eq!(calls_to(&e, DELETE).len(), 1);
    }

    #[test]
    fn remove_object_ov2_deletes_the_data_at_28() {
        let mut e = engine();
        let data = buffer_data(&mut e);
        let part = e.mem.alloc(0x40);
        e.mem.set_u32(part + 0x28, data);
        e.mem.set_u32(part + 0x34, 0x1234);
        e.call(0x00e9_4d00, &[0x9000, part]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![data, 1]]);
        assert_eq!(e.mem.u32(part + 0x28), 0);
        assert_eq!(e.mem.u32(part + 0x34), 0x1234);
    }

    fn geometry(e: &mut Engine, chain: u32) -> u32 {
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, GET_CHAIN);
        e.register_double(GET_CHAIN, move |_, _| chain.into_ret());
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u32(geometry, vtable);
        geometry
    }

    #[test]
    fn add_finds_the_record_and_sets_the_flags() {
        let mut e = engine();
        let other = e.mem.alloc(8);
        e.mem.set_u32(other + 4, RECORD_011F4A88);
        let geometry = geometry(&mut e, other);
        e.mem.set_u32(geometry + 0x24, 1);
        e.mem.set_u32(geometry + 0x28, 1);
        e.mem.set_u16(geometry + 0xc, 0x1001);
        e.mem.set_u32(FLAG_CAP_011BF558, 1);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0xdead, 0]);
        let data = e.mem.u32(geometry + 0x34);
        assert_ne!(data, 0);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
        assert_eq!(e.mem.u32(data + 4), 0x01e0_0000);
        assert_eq!(
            calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT),
            vec![vec![0x9000, data]]
        );
        assert!(calls_to(&e, CHECK_0043B300).is_empty());
        // Already has data: nothing more.
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, 0]);
        assert_eq!(calls_to(&e, GEOMETRY_GROUP_ADD_OBJECT).len(), 1);
    }

    #[test]
    fn add_without_the_record_asks_the_check_and_caps_the_bit() {
        let mut e = engine();
        e.register(CHECK_0043B300, |_, _| 0x0101u32.into_ret());
        let geometry = geometry(&mut e, 0);
        e.mem.set_u16(geometry + 0xc, 0xf001);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, 0]);
        let data = e.mem.u32(geometry + 0x34);
        assert_eq!(
            calls_to(&e, CHECK_0043B300),
            vec![vec![RECORD_011F4A70, geometry]]
        );
        assert_eq!(e.mem.u32(data + 0x3c), 5);
        // +0x24 clear: no flag bits; the cap (0) clears bit 0.
        assert_eq!(e.mem.u32(data + 4), 0);
    }

    #[test]
    fn add_for_a_part_uses_its_word_22() {
        let mut e = engine();
        let geometry = geometry(&mut e, 0);
        e.mem.set_u32(geometry + 0x28, 1);
        let part = e.mem.alloc(0x2c);
        e.mem.set_u16(part + 0x22, 3);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, part]);
        let data = e.mem.u32(part + 0x28);
        assert_eq!(e.mem.u32(data + 0x3c), 5);
        assert_eq!(e.mem.u32(data + 4), 0x40_0000);
        assert_eq!(e.mem.u32(geometry + 0x34), 0);
        assert!(calls_to(&e, GET_CHAIN).is_empty());
        let part = e.mem.alloc(0x2c);
        e.call(0x00e9_48f0, &[0x9000, geometry, 0, part]);
        let data = e.mem.u32(part + 0x28);
        assert_eq!(e.mem.u32(data + 0x3c), 4);
    }
}
