//! `(unplaced)/run_00aff060` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of `CloseHandle` (`stdcall`, one word).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;

// Translated from 00aff090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `CloseHandle` on the handle at
/// +0x10; its result (`EAX`) is passed on.
pub fn fn_00aff090(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr() + 0x10);
    e.call(CLOSE_HANDLE, &args![handle]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00af_f090, fn_00aff090(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closes_the_handle_at_0x10() {
        let mut e = Engine::new();
        e.register(CLOSE_HANDLE, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, 0x44);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_f090, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(CLOSE_HANDLE, vec![0x44])]);
    }
}
