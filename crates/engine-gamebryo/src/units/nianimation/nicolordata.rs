//! `NiAnimation/nicolordata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiColorData` (Xbox PDB, 0x18 bytes; PC offsets as the code uses them):
//! `NiObject`, then `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c, `m_eType` +0x10
//! and `m_ucKeySize` +0x14. The colour keys use the animation key tables in
//! `.data` at the colour rows, indexed by the key type: `011f38e8` (a
//! `cdecl` function deleting a key array) and `011f376a` (the key size, one
//! byte per type). Their Xbox PDB names are not confirmed.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiColorData` (RTTI `.?AVNiColorData@@`).
const NI_COLOR_DATA_VTABLE: u32 = 0x0102_db4c;
/// `m_uiNumKeys`, `m_pkKeys`, `m_eType`, `m_ucKeySize` (Xbox PDB).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;
/// The colour rows of the animation key tables (see the module comment).
const KEY_DELETE_TABLE: u32 = 0x011f_38e8;
const KEY_SIZE_TABLE: u32 = 0x011f_376a;
/// `NiObject::~NiObject` (`00a5d3d0`, `NiMain/niobject.cpp`).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;

/// Deletes the key array (when there is one) through the delete function
/// of its current key type.
fn delete_keys(e: &mut Engine, this: u32) {
    let keys = e.mem.u32(this.wrapping_add(KEYS));
    if keys != 0 {
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let delete = e
            .mem
            .u32(KEY_DELETE_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

// Translated from 00a3d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorData::~NiColorData` (unnamed in the engine map; the destructor
/// its scalar deleting destructor `0052d13a` calls): stores the vtable,
/// deletes the key array through the delete function of its key type, then
/// the `NiObject` destructor (`00a5d3d0`, a tail jump).
pub fn ni_color_data_destruct(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, NI_COLOR_DATA_VTABLE);
    delete_keys(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a3d210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorData::ReplaceAnim` (Xbox PDB): deletes the current key array
/// (through its type's delete function), then takes `keys`, `num_keys` and
/// `key_type` with the type's key size when `keys` and `num_keys` are
/// non-zero (the type is not checked, unlike `NiFloatData`'s), and clears
/// all four fields otherwise.
pub fn ni_color_data_replace_anim(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    num_keys: u32,
    key_type: u32,
) {
    let this = this.addr();
    delete_keys(e, this);
    if keys != 0 && num_keys != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_d120, ni_color_data_destruct(Ptr)),
        entry!(0x00a3_d210, ni_color_data_replace_anim(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE: u32 = 0x00ee_0100;

    fn fixture() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.mem.set_u8(KEY_SIZE_TABLE, 0x14);
        e.mem.set_u8(KEY_SIZE_TABLE + 2, 0x38);
        e.mem.set_u32(KEY_DELETE_TABLE, DELETE);
        e.mem.set_u32(KEY_DELETE_TABLE + 8, DELETE);
        for address in [DELETE, NI_OBJECT_DESTRUCT] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0x18);
        (e, log, this)
    }

    #[test]
    fn destructor_deletes_the_keys_then_runs_the_object_destructor() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + KEYS, 0x5000);
        e.mem.set_u32(this + KEY_TYPE, 2);
        e.call(0x00a3_d120, &args![this]);
        assert_eq!(e.mem.u32(this), NI_COLOR_DATA_VTABLE);
        assert_eq!(
            *log.borrow(),
            [(DELETE, vec![0x5000]), (NI_OBJECT_DESTRUCT, vec![this])]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + KEYS, 0);
        e.call(0x00a3_d120, &args![this]);
        assert_eq!(*log.borrow(), [(NI_OBJECT_DESTRUCT, vec![this])]);
    }

    #[test]
    fn replace_anim_takes_keys_with_any_type_and_clears_without_keys() {
        let (mut e, log, this) = fixture();
        // Type 0 is taken (no type check).
        e.call(0x00a3_d210, &args![this, 0x6000u32, 4u32, 0u32]);
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 4);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 0);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x14);
        e.call(0x00a3_d210, &args![this, 0x6100u32, 2u32, 2u32]);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x38);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 2);
        for (keys, count) in [(0u32, 3u32), (0x6200, 0)] {
            e.mem.set_u32(this + KEYS, 0x6100);
            e.mem.set_u32(this + KEY_TYPE, 2);
            log.borrow_mut().clear();
            e.call(0x00a3_d210, &args![this, keys, count, 2u32]);
            assert_eq!(*log.borrow(), [(DELETE, vec![0x6100])]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
    }
}
