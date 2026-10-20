//! `NiMain/ninifimagereader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00fdf054`: import slot of `InitializeCriticalSection` (`stdcall`, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// `00a66150` (`thiscall`, engine map unit `nistream.cpp`): constructs the part at `+0x100`.
const PART_CONSTRUCTOR: u32 = 0x00a6_6150;

// Translated from 00a8ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `ninifimagereader.cpp`; the body of a constructor,
/// `thiscall`, no stack word): sets the vtable `0109f3d0`, initializes the critical
/// section at `this + 0x80`, sets the vtable `0109f3e4`, constructs the part at
/// `this + 0x100` with `00a66150` and returns `this`.
pub fn fn_00a8ae50(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0109_f3d0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![this.addr() + 0x80]);
    e.mem.set_u32(this.addr(), 0x0109_f3e4);
    e.call(PART_CONSTRUCTOR, &args![this.addr() + 0x100]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_ae50, fn_00a8ae50(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_the_lock_then_the_part_and_ends_on_the_second_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        e.register_double(INITIALIZE_CRITICAL_SECTION, move |e, a| {
            assert_eq!(a, &[this + 0x80]);
            assert_eq!(e.mem.u32(this), 0x0109_f3d0);
            Ret::default()
        });
        e.register_double(PART_CONSTRUCTOR, move |e, a| {
            assert_eq!(a, &[this + 0x100]);
            assert_eq!(e.mem.u32(this), 0x0109_f3e4);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_ae50, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.unwrap().into_iter().map(|(a, _)| a).collect();
        assert_eq!(
            order,
            vec![0x00a8_ae50, INITIALIZE_CRITICAL_SECTION, PART_CONSTRUCTOR]
        );
        assert_eq!(e.mem.u32(this), 0x0109_f3e4);
    }
}
