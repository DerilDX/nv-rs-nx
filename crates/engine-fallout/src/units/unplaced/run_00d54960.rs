//! `(unplaced)/run_00d54960` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkScaledMoppBvTreeShape` (RTTI
/// `.?AVhkScaledMoppBvTreeShape@@`).
const SCALED_MOPP_VTABLE: u32 = 0x010c_a330;
/// The referenced object at +0x40 and its release (`00c905b0`,
/// `hkreferencedobject.cpp`; thiscall, no stack words).
const REFERENCED: u32 = 0x40;
const REFERENCE_RELEASE_00C905B0: u32 = 0x00c9_05b0;
/// The base destructor body (`00d543e0`, `hkpmoppbvtreeshape.cpp`).
const BASE_DESTRUCT_00D543E0: u32 = 0x00d5_43e0;

// Translated from 00d54960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d54960 - the function's C++ exception frame (FS:[0], unwind state) is not translated
/// `hkScaledMoppBvTreeShape`'s destructor body (by the vtable it stores;
/// unnamed in the engine map): the vtable, the object at +0x40 released
/// (`00c905b0`) when non-null, then the base destructor (`00d543e0`).
pub fn fn_00d54960(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCALED_MOPP_VTABLE);
    let referenced = e.mem.u32(this.addr() + REFERENCED);
    if referenced != 0 {
        e.call(REFERENCE_RELEASE_00C905B0, &args![referenced]);
    }
    e.call(BASE_DESTRUCT_00D543E0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d5_4960, fn_00d54960(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn others(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a != 0x00d5_4960)
            .cloned()
            .collect()
    }

    #[test]
    fn releases_the_referenced_object_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(REFERENCE_RELEASE_00C905B0, |_, _| Ret::default());
        e.register(BASE_DESTRUCT_00D543E0, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + REFERENCED, 0x0700_0000);
        e.call_log = Some(vec![]);
        e.call(0x00d5_4960, &args![this]);
        assert_eq!(e.mem.u32(this), SCALED_MOPP_VTABLE);
        assert_eq!(
            others(&e),
            vec![
                (REFERENCE_RELEASE_00C905B0, vec![0x0700_0000]),
                (BASE_DESTRUCT_00D543E0, vec![this]),
            ]
        );
        e.mem.set_u32(this + REFERENCED, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d5_4960, &args![this]);
        assert_eq!(others(&e), vec![(BASE_DESTRUCT_00D543E0, vec![this])]);
    }
}
