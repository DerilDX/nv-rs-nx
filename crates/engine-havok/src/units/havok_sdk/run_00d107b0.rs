//! `Havok SDK/run_00d107b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d107b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed): sets the `u16` at `+6` to 1 (the reference count
/// of the `hkReferencedObject` layout), the word at `+8` to 0 and the vtable word at `+0` to
/// 0x010c_d54c. Returns `this`.
pub fn fn_00d107b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr(), 0x010c_d54c);
    this
}

// Translated from 00d107d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (class unconfirmed): sets the `u16` at `+6` to 1 (the reference count of the
/// `hkReferencedObject` layout), the word at `+8` to `0x010c_63bc` and then to `0x010c_d588`, the
/// vtable word at `+0` to `0x010c_d598` and the word at `+0xc` to `link`. Returns `this`.
pub fn fn_00d107d0(e: &mut Engine, this: Ptr, link: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0x010c_63bc);
    e.mem.set_u32(base, 0x010c_d598);
    e.mem.set_u32(base + 8, 0x010c_d588);
    e.mem.set_u32(base + 0xc, link);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d107b0, fn_00d107b0(Ptr) -> Ptr),
        entry!(0x00d1_07d0, fn_00d107d0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00d107b0_sets_count_link_and_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e.call(0x00d1_07b0, &args![this]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010c_d54c);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00d107d0_sets_count_vtables_and_link() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in (0..0x20).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        let result = e.call(0x00d1_07d0, &args![this, 0x1234u32]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x010c_d598);
        assert_eq!(e.mem.u16(this + 4), 0xffff);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x010c_d588);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
    }
}
