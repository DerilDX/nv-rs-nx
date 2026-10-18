//! `fallout shared/magic/effectitemlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00402c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00402c90 - the exception-unwinding frame (FS:[0]) is not translated.
/// `EffectItemList::EffectItemList` (the Xbox PDB name the engine map gives
/// it). Runs the constructor `00402d00` on the object (it sets a base vtable
/// and constructs a string at `+4`, see `run_00402d00.rs`), `004056a0` on
/// the member at `+0xc`, then stores the vtable pointers `01011d0c` at `+0`
/// and `01011cf4` at `+0xc` (the RTTI name of both is `.?AVMagicItem@@`).
/// Returns `this`. The exception-unwinding frame is not translated.
pub fn effect_item_list_effect_item_list(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d00, &args![this]);
    e.call(0x0040_56a0, &args![this.byte_add(0xc)]);
    e.mem.set_u32(this.addr(), 0x0101_1d0c);
    e.mem.set_u32(this.addr() + 0xc, 0x0101_1cf4);
    this
}

// Translated from 004059d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00403d50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004059d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_3d50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00406e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `StatsMenu::StatusDataList` scalar deleting destructor (engine map name): runs the destructor body `00406de0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn stats_menu_status_data_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0040_6de0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402c90, effect_item_list_effect_item_list(Ptr) -> Ptr),
        entry!(0x0040_59d0, fn_004059d0(Ptr, u32) -> Ptr),
        entry!(0x0040_6e40, stats_menu_status_data_list_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn effect_item_list_effect_item_list_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_2c90,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d00, &[A::T(0x0)]), (0x0040_56a0, &[A::T(0xc)])],
            stores: &[(0x0, 4, 0x0101_1d0c), (0xc, 4, 0x0101_1cf4)],
        });
    }
    #[test]
    fn at_004059d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_3d50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_59d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_59d0)
                .collect();
            let mut expected = vec![(0x0040_3d50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00406e40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_6de0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_6e40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_6e40)
                .collect();
            let mut expected = vec![(0x0040_6de0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
