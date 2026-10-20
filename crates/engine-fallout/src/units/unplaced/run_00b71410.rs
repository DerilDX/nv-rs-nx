//! `(unplaced)/run_00b71410` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The list clear run twice on `this + 0x60` (`00b71300`; `thiscall`) and the base
/// destructor the function ends with (`00baa0e0`, a jump).
const LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const BASE_DESTRUCTOR: u32 = 0x00ba_a0e0;
/// The virtual table the destructor stores (`010ae7b0`).
const DESTRUCTED_VTABLE: u32 = 0x010a_e7b0;

// Translated from 00b71410 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor (unnamed in the engine map; `thiscall`): stores the virtual table
/// `010ae7b0`, runs `00b71300` twice on the list at `this + 0x60`, then the base destructor
/// (`00baa0e0`).
pub fn fn_00b71410(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), DESTRUCTED_VTABLE);
    let list = this.addr() + 0x60;
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(LIST_REMOVE_ALL, &args![list]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b7_1410, fn_00b71410(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_the_list_twice_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(LIST_REMOVE_ALL, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x70);
        e.call_log = Some(vec![]);
        e.call(0x00b7_1410, &args![this]);
        assert_eq!(e.mem.u32(this), DESTRUCTED_VTABLE);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.iter().skip(1).map(|c| (c.0, c.1.clone())).collect();
        assert_eq!(
            order,
            vec![
                (LIST_REMOVE_ALL, vec![this + 0x60]),
                (LIST_REMOVE_ALL, vec![this + 0x60]),
                (BASE_DESTRUCTOR, vec![this]),
            ]
        );
    }
}
