//! `fallout/magic/run_008229d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00822a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0081de70` (Ghidra name
/// `MagicProjectile::~MagicProjectile`), a destructor with an exception frame, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `0081de70`'s disassembly ends with its exception-frame
/// unlink and returns no value (Ghidra types it void).
pub fn fn_00822a00(e: &mut Engine, this: Ptr) {
    e.call(0x0081_de70, &args![this]);
}

// Translated from 00822a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `MagicTarget` (the RTTI name of the vtable `0107bc28` it
/// installs; not named in the engine map): the vtable, the dispel list at +8
/// (constructor `0096a2d0`, which zeroes its two words), `bShowTargetStats` (+4) and
/// `bUpdating` (+5) cleared. Returns `this`.
pub fn fn_00822a40(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_bc28);
    e.call(0x0096_a2d0, &args![this.byte_add(8)]);
    // MagicTarget::bShowTargetStats (Xbox PDB) +0x04
    e.mem.set_u8(this.addr() + 4, 0);
    // MagicTarget::bUpdating (Xbox PDB) +0x05
    e.mem.set_u8(this.addr() + 5, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_2a00, fn_00822a00(Ptr)),
        entry!(0x00822a40, fn_00822a40(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00822a00_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0081_de70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_2a00, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0081_de70), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_target_constructor_installs_the_vtable_and_clears_its_flags() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0096_a2d0, |_, _| Ret::default());
        let this = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u8(this.addr() + 4, 1);
        e.mem.set_u8(this.addr() + 5, 1);
        assert_eq!(e.call(0x0082_2a40, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), 0x0107_bc28);
        assert_eq!(e.mem.u8(this.addr() + 4), 0);
        assert_eq!(e.mem.u8(this.addr() + 5), 0);
        let list_calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0096_a2d0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(list_calls, [vec![this.addr() + 8]]);
    }
}
