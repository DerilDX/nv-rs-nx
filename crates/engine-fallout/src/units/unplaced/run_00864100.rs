//! `(unplaced)/run_00864100` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004ede70` (`cdecl`, one word): frees the block held at +4.
const FREE_HELD_BLOCK: u32 = 0x004e_de70;

// Translated from 00864100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `01081fc8` and passes the word at +4 to `004ede70`.
pub fn fn_00864100(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_1fc8);
    let block = e.mem.u32(this.addr() + 4);
    e.call(FREE_HELD_BLOCK, &args![block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0086_4100, fn_00864100(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_and_frees_the_block() {
        let mut e = Engine::new();
        e.register(FREE_HELD_BLOCK, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0086_4100, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(FREE_HELD_BLOCK, vec![0x6000])]);
        assert_eq!(e.mem.u32(this), 0x0108_1fc8);
    }
}
