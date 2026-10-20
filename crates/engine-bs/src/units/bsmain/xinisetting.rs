//! `BSMain/xinisetting.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `INISettingCollection`'s open handle (`+0x108`): set to the collection
/// itself while open, 0 when closed.
const OPEN_HANDLE_OFFSET: u32 = 0x108;

// Translated from 00c43d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INISettingCollection::Open` (Xbox PDB, engine map): marks the
/// collection open (its handle at `+0x108` becomes `this`) and returns
/// true. The argument is not read.
pub fn ini_setting_collection_open(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    e.mem
        .set_u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET), this.addr());
    true
}

// Translated from 00c43d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INISettingCollection::Close` (Xbox PDB, engine map): clears the open
/// handle at `+0x108` and returns true.
pub fn ini_setting_collection_close(e: &mut Engine, this: Ptr) -> bool {
    e.mem
        .set_u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET), 0);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3d60, ini_setting_collection_open(Ptr, u32) -> bool),
        entry!(0x00c4_3d80, ini_setting_collection_close(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_close_set_the_handle() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10c);
        assert!(e.call(0x00c4_3d60, &args![this, 7u32]).bool());
        assert_eq!(e.mem.u32(this + 0x108), this);
        assert!(e.call(0x00c4_3d80, &args![this]).bool());
        assert_eq!(e.mem.u32(this + 0x108), 0);
    }
}
