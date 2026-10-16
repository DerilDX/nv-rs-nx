//! `fallout shared/bgsimpactdataset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058e810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0058e810 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x5f (0x4c bytes); `ECX` is the object.
/// The engine map names it `BGSImpactDataSet::BGSImpactDataSet` (Xbox PDB); the RTTI name of the vtable it stores at `+0` (`0x0103_323c`) is `BGSImpactDataSet`, which agrees.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0058e810(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x004f_9d90, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_323c);
    e.mem.set_u32(base + 0x18, 0x0103_3224);
    e.call(0x004f_15a0, &args![this, 0x5fu32]);
    e.call(0x0058_eb10, &args![this]);
    this
}

// Translated from 0058f850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f900` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f900, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0058f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<BGSImpactData_P_BGSImpactData_P>` scalar deleting destructor (engine map name): runs the destructor body `0058f960` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0058_f960, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_e810, fn_0058e810(Ptr) -> Ptr),
        entry!(0x0058_f850, bs_map_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0058_f9a0, bs_map_base_bgs_impact_data_p_bgs_impact_data_p_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0058e810_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_e810,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x004f_9d90, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x5f)]),
                (0x0058_eb10, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_323c), (0x18, 4, 0x0103_3224)],
        });
    }
    #[test]
    fn at_0058f850_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f900, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f850, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f850)
                .collect();
            let mut expected = vec![(0x0058_f900, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0058f9a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_f960, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_f9a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_f9a0)
                .collect();
            let mut expected = vec![(0x0058_f960, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
