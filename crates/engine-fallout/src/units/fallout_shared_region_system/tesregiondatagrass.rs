//! `fallout shared/region system/tesregiondatagrass.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f1690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataGrass` scalar deleting destructor (engine map name): runs the destructor body `004f1730` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_data_grass_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_1730, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004f14a0` (`thiscall`, no stack word; unit `fallout shared/region system/tesregiondata.cpp`):
/// the base destructor.
const REGION_DATA_DESTRUCTOR: u32 = 0x004f_14a0;

// Translated from 004f1730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1730 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef2f08, state variable) is not translated
/// Unnamed in the engine map (unit `tesregiondatagrass.cpp`, `thiscall`, no stack word): the
/// `TESRegionDataGrass` destructor run by its scalar deleting destructor. Sets the vtable
/// `01023b24`; when the object at +8 is set, calls its vtable slot 0 with 1; then the base
/// destructor `004f14a0`. The exe tests the pointer twice (the second test is the same and
/// cannot fail).
pub fn fn_004f1730(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_3b24);
    if e.mem.u32(this.addr() + 8) != 0 {
        let list = e.mem.u32(this.addr() + 8);
        if list != 0 {
            e.vcall(list, 0, &args![1u32]);
        }
    }
    e.call(REGION_DATA_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_1690, tes_region_data_grass_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_1730, fn_004f1730(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f1690_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_1730, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_1690, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_1690)
                .collect();
            let mut expected = vec![(0x004f_1730, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_004f1730_destroys_the_held_list_then_the_base() {
        let mut e = Engine::new();
        e.register(REGION_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x0150_0000);
        e.mem.set_u32(this + 8, list);
        e.call_log = Some(vec![]);
        e.call(0x004f_1730, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0150_1000, vec![list, 1]),
                (REGION_DATA_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_3b24);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_1730, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(REGION_DATA_DESTRUCTOR, vec![this])]);
    }
}
