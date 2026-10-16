//! `fallout shared/magic/run_00402bc0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0048e270` (a `BSStringT` at `+4`, per the Xbox PDB's
/// `TESTexture`).
const BASE_CONSTRUCT: u32 = 0x0048_e270;
/// This class's vtable.
const VTABLE: u32 = 0x0101_1cd4;

// Translated from 00402c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESIcon` (the engine map gives it no name; the Xbox
/// PDB's `TESIcon` is a `TESTexture` and `TESQuest` constructs it at +0x24):
/// runs the base constructor `0048e270`, then sets its own vtable. Returns
/// `this`.
pub fn fn_00402c20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

// Translated from 00402c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00402bc0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00402c60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_2bc0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00402c20, fn_00402c20(Ptr) -> Ptr),
        entry!(0x0040_2c60, fn_00402c60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_base_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        // The base constructor sets its own vtable first; the derived one wins.
        e.register(BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let object = e.mem.alloc(0x10);
        let back = e.call(0x0040_2c20, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
    }
    #[test]
    fn at_00402c60_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_2bc0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_2c60, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_2c60)
                .collect();
            let mut expected = vec![(0x0040_2bc0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
