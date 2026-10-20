//! `fallout/ai/run_009f25f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActorPackageData`'s constructor (`0067d920`: stores the base vtable).
const ACTOR_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_d920;
/// The `GuardActorPackageData` vtable (its RTTI locator names
/// `.?AVGuardActorPackageData@@`).
const GUARD_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_33c8;

// Translated from 009f27a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GuardActorPackageData` constructor: the `ActorPackageData` base,
/// the class's vtable, then `pGuardWarnedActor` (+8), `iWarningCount`
/// (+0xC), `fGuardTimer` (+0x10, 0.0) and `eState` (+4) cleared (Xbox PDB
/// field names). Returns `this`.
pub fn fn_009f27a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ACTOR_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, GUARD_ACTOR_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_f32(base + 0x10, 0.0);
    e.mem.set_u32(base + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009f27a0, fn_009f27a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_data_constructor_clears_its_fields() {
        let mut e = Engine::new();
        e.register(ACTOR_PACKAGE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x14);
        e.mem.write(this, &[0xaa; 0x14]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_27a0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTOR_PACKAGE_DATA_CONSTRUCT, vec![this]));
        assert_eq!(e.mem.u32(this), GUARD_ACTOR_PACKAGE_DATA_VTABLE);
        for offset in [4, 8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }
}
