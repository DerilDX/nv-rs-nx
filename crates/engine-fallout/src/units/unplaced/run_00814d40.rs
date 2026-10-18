//! `(unplaced)/run_00814d40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004534f0(this, 0)` (`fallout shared/tes.cpp`): the call `00815300` makes
/// twice, with a zero stack word.
const CLEAR_STEP: u32 = 0x0045_34f0;
/// The vtable `00815300` stores.
const VTABLE: u32 = 0x0107_ac5c;

// Translated from 00815300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; on the player construction path):
/// stores the vtable `0107ac5c`, calls `004534f0(this, 0)` twice, zeroes the
/// words at +8 and +4 and returns `this`. The class is not confirmed from
/// the exe.
pub fn fn_00815300(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(CLEAR_STEP, &args![this, 0u32]);
    e.call(CLEAR_STEP, &args![this, 0u32]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00815300, fn_00815300(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_vtable_steps_twice_and_clears_the_words() {
        let mut e = Engine::new();
        e.register(CLEAR_STEP, |_, _| Ret::default());
        let this = e.mem.alloc(16);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u32(this + 8, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00815300, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CLEAR_STEP, vec![this, 0]));
        assert_eq!(log[2], (CLEAR_STEP, vec![this, 0]));
        assert_eq!(log.len(), 3);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 8)), (0, 0));
    }
}
