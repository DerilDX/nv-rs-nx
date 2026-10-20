//! `fallout/ai/run_009a6050` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00451010` (no words, unit `fallout shared/tes.cpp`): answers an object (it calls
/// `00559450` on `011ca0d8`).
const GET_OBJECT: u32 = 0x0045_1010;
/// `00560d30` (`thiscall`, one word: the index; unit `fallout shared/tesobjectrefr.cpp`):
/// answers `this + index * 4`.
const ELEMENT_ADDRESS: u32 = 0x0056_0d30;
/// `004587d0` (`cdecl`, one float; unit `fallout shared/tes.cpp`): answers the float times
/// the float at `011c3d18`, in `ST0`.
const SCALE_FLOAT: u32 = 0x0045_87d0;
/// The float `fn_009a6050` stores.
const RESULT_FLOAT: u32 = 0x011f_1c00;
/// `006219c0` (`thiscall`, no stack words): called twice by `fn_009a6080`.
const CALLED_AROUND: u32 = 0x0062_19c0;
/// `00460140` (`thiscall`, unit `fallout shared/tesdatahandler.cpp`): answers `this + 0x10`.
const MEMBER_AT_10: u32 = 0x0046_0140;
/// `00458b20` (no words, unit `fallout shared/tes.cpp`): answers `01267e30`.
const DEFAULT_OBJECT: u32 = 0x0045_8b20;

// Translated from 009a6050 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no words; called by `00990f20`): takes the object `00451010`
/// answers, its `fn_009a6080` result, the address of word 2 of that (`00560d30` with 2; the
/// `PUSH 2` before the first call is that call's argument, `RET 4`), passes the float there
/// to `004587d0` and stores the negated result (`FCHS`) as a float at `011f1c00`.
pub fn fn_009a6050(e: &mut Engine) {
    let object = e.call(GET_OBJECT, &[]).u32();
    let base = fn_009a6080(e, object);
    let address = e.call(ELEMENT_ADDRESS, &args![base, 2u32]).u32();
    let value = x87_quiet_bits(e.mem.u32(address));
    let scaled = e.call(SCALE_FLOAT, &args![value]).f64();
    e.mem.set_u32(RESULT_FLOAT, x87_store32(-scaled));
}

// Translated from 009a6080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): asks `this` for an object (vtable
/// slot +0x94), calls `006219c0` on `this`, then answers the object's `00460140` result
/// (`object + 0x10`) or, when the object is null, `00458b20`'s (`01267e30`); calls
/// `006219c0` on `this` once more before answering.
pub fn fn_009a6080(e: &mut Engine, this: u32) -> u32 {
    let object = e.vcall(this, 0x94, &[]).u32();
    e.call(CALLED_AROUND, &args![this]);
    let result = if object != 0 {
        e.call(MEMBER_AT_10, &args![object]).u32()
    } else {
        e.call(DEFAULT_OBJECT, &[]).u32()
    };
    e.call(CALLED_AROUND, &args![this]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009a_6050, fn_009a6050()),
        entry!(0x009a_6080, fn_009a6080(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn actor(e: &mut Engine, slot_94: u32) -> u32 {
        let this = e.mem.alloc(8);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x94, slot_94);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn fn_009a6080_answers_the_member_or_the_default() {
        let mut e = Engine::new();
        e.register(0x0090_1000, |_, _| 0x7000u32.into_ret());
        e.register(0x0090_2000, |_, _| Ret::default());
        e.register(CALLED_AROUND, |_, _| Ret::default());
        e.register(MEMBER_AT_10, |_, a| (a[0] + 0x10).into_ret());
        e.register(DEFAULT_OBJECT, |_, _| 0x0126_7e30u32.into_ret());
        let with = actor(&mut e, 0x0090_1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_6080, &[with]).u32(), 0x7010);
        assert_eq!(calls_to(&e, CALLED_AROUND), vec![vec![with], vec![with]]);
        assert!(calls_to(&e, DEFAULT_OBJECT).is_empty());

        let without = actor(&mut e, 0x0090_2000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009a_6080, &[without]).u32(), 0x0126_7e30);
        assert!(calls_to(&e, MEMBER_AT_10).is_empty());
        assert_eq!(calls_to(&e, CALLED_AROUND).len(), 2);
    }

    #[test]
    fn fn_009a6050_stores_the_negated_scaled_float() {
        let mut e = Engine::new();
        e.map(RESULT_FLOAT & !0xfff, 0x1000);
        e.register(0x0090_1000, |_, _| 0x7000u32.into_ret());
        let this = actor(&mut e, 0x0090_1000);
        e.register_double(GET_OBJECT, move |_, _| this.into_ret());
        e.register(CALLED_AROUND, |_, _| Ret::default());
        e.register(MEMBER_AT_10, |_, a| (a[0] + 0x10).into_ret());
        let floats = e.mem.alloc(0x10);
        e.mem.set_f32(floats + 8, 1.5);
        e.register_double(ELEMENT_ADDRESS, move |_, a| {
            assert_eq!(a, &[0x7010, 2]);
            (floats + 8).into_ret()
        });
        e.register(SCALE_FLOAT, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009a_6050, &[]);
        assert_eq!(e.mem.f32(RESULT_FLOAT), -3.0);
        assert_eq!(calls_to(&e, SCALE_FLOAT), vec![vec![1.5f32.to_bits()]]);

        // A signalling NaN is quieted by the FLD/FSTP copy before the call.
        e.mem.set_u32(floats + 8, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x009a_6050, &[]);
        assert_eq!(calls_to(&e, SCALE_FLOAT), vec![vec![0x7fc0_0001]]);
    }
}
