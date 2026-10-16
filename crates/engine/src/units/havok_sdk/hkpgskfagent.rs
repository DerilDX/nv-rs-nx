//! `Havok SDK/hkpgskfagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpGskBaseAgent::hkpGskBaseAgent` (Xbox PDB, `00dabf10`, `this`, three words).
const GSK_BASE_AGENT_CONSTRUCT: u32 = 0x00da_bf10;
/// `hkpGskfAgent`'s vtable (RTTI confirmed).
const GSKF_AGENT_VTABLE: u32 = 0x010d_5224;

// Translated from 00dacbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskfAgent::hkpGskfAgent` (Xbox PDB): the base agent constructor with the three
/// words, then the vtable and a zero word at +0x30. Returns `this`.
pub fn hkp_gskf_agent_construct(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) -> Ptr {
    e.call(GSK_BASE_AGENT_CONSTRUCT, &args![this, first, second, third]);
    e.mem.set_u32(this.addr(), GSKF_AGENT_VTABLE);
    e.mem.set_u32(this.addr() + 0x30, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00da_cbd0, hkp_gskf_agent_construct(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn construct_runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(GSK_BASE_AGENT_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 0xdead);
        assert_eq!(
            e.call(0x00da_cbd0, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(*seen.borrow(), vec![vec![this, 1, 2, 3]]);
        assert_eq!(e.mem.u32(this), GSKF_AGENT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}
