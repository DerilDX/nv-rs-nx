//! `(unplaced)/run_0066dab0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0066db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): calls `this`'s vtable slot +0x10 with
/// `first` and `count * 16` (`count << 4`); its `EAX` is passed on.
pub fn fn_0066db70(e: &mut Engine, this: Ptr, first: u32, count: u32) -> u32 {
    e.vcall(this.addr(), 0x10, &args![first, count << 4]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0066_db70, fn_0066db70(Ptr, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_count_in_sixteen_byte_units() {
        let mut e = Engine::new();
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| 0x31u32.into_ret());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0150_0000);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0066_db70, &args![this, 0x7000u32, 3u32]).u32(),
            0x31
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![this, 0x7000, 0x30])]);
    }
}
