//! `fallout shared/run_0048ea40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable (RTTI `.?AVTESWeightForm@@`).
const VTABLE: u32 = 0x0101_d874;
/// The `TESWeightForm::InitializeDataComponent` body the constructor calls (Xbox PDB name of `00910b10`).
const INITIALIZE_DATA: u32 = 0x0091_0b10;

// Translated from 0048eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESWeightForm` (the engine map gives it
/// no name; the RTTI name of the vtable it stores is `.?AVTESWeightForm@@`): runs the
/// base constructor, stores the vtable at `+0`, then calls the component's
/// `InitializeDataComponent` body directly. Returns `this`.
pub fn fn_0048eac0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(INITIALIZE_DATA, &args![this]);
    this
}

// Translated from 0048eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101d874` (RTTI `.?AVTESWeightForm@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_0048eaf0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_d874);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_eaf0, fn_0048eaf0(Ptr)),
        entry!(0x0048eac0, fn_0048eac0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0048eac0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048eac0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0091_0b10, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_d874)],
        });
    }

    #[test]
    fn fn_0048eaf0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_eaf0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_d874);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_d874,
            "the vtable was stored before the call"
        );
    }
}
