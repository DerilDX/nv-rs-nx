//! `fallout shared/run_005a6150` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::types::BSStringT;

/// `TESForm::TESForm` (`00483370`).
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `TESGlobal`'s vtable.
const TES_GLOBAL_VTABLE: u32 = 0x0103_6524;
/// The form type of a `TESGlobal` (`GLOB`).
const FORM_TYPE_GLOBAL: u8 = 6;
/// `TESGlobal::cType` of a new global: `'s'`, a short.
const GLOBAL_TYPE_SHORT: u8 = 0x73;

layout! {
    /// `TESGlobal` (Xbox PDB), 0x28 bytes; the same offsets on PC.
    pub struct TESGlobal: 0x28 {
        /// `cFormEditorID` (Xbox PDB): a [`BSStringT`] at +0x18.
        0x18 cFormEditorID: Inline<BSStringT>,
        /// `cType` (Xbox PDB): `'s'`, `'l'` or `'f'`.
        0x20 cType: u8,
        /// `fValue` (Xbox PDB).
        0x24 fValue: f32,
    }
}

// Translated from 005a6320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESGlobal` (the engine map gives this address no name;
/// the body fits the Xbox PDB class: `CreateFormOfType` allocates 0x28 bytes
/// for form type 6 and calls it, `cType` is at +0x20 and `fValue` at +0x24). It
/// constructs the `TESForm` base, sets the
/// vtable, constructs the editor ID string, makes the global a short
/// (`cType` `'s'`) of value 0.0 and sets the form type to 6. Returns `this`.
// Translation incomplete: 005a6320 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_005a6320(e: &mut Engine, this: Ptr<TESGlobal>) -> Ptr<TESGlobal> {
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_GLOBAL_VTABLE);
    let editor_id = this.at(TESGlobal::cFormEditorID);
    e.call(STRING_CONSTRUCT, &args![editor_id]);
    e.set(this, TESGlobal::cType, GLOBAL_TYPE_SHORT);
    e.set(this, TESGlobal::fValue, 0.0);
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_GLOBAL as u32]);
    this
}

// Translated from 005a63c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a63f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a63c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_63f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a6320, fn_005a6320(Ptr<TESGlobal>) -> Ptr<TESGlobal>),
        entry!(0x005a_63c0, fn_005a63c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TESForm;

    #[test]
    fn constructs_a_zero_valued_short_global_in_the_games_order() {
        let mut e = Engine::new();
        e.register(TES_FORM_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_FORM_TYPE, |_, _| Ret::default());
        let global: Ptr<TESGlobal> = e.new_object();
        // Dirty memory the constructor must overwrite.
        e.set(global, TESGlobal::cType, 0x66);
        e.set(global, TESGlobal::fValue, 9.0);
        e.call_log = Some(vec![]);
        let back = e.call(0x005a_6320, &args![global]).ptr::<TESGlobal>();
        assert_eq!(back, global);
        assert_eq!(e.mem.u32(global.addr()), TES_GLOBAL_VTABLE);
        assert_eq!(e.get(global, TESGlobal::cType), 0x73);
        assert_eq!(e.get(global, TESGlobal::fValue), 0.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (TES_FORM_CONSTRUCT, vec![global.addr()]),
                (STRING_CONSTRUCT, vec![global.addr() + 0x18]),
                (SET_FORM_TYPE, vec![global.addr(), 6]),
            ]
        );
    }

    #[test]
    fn the_form_type_is_written_by_the_game_function() {
        // With the real 004f15a0 in place the type byte lands at +4.
        let mut e = Engine::new();
        e.register(TES_FORM_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let global: Ptr<TESGlobal> = e.new_object();
        e.call(0x005a_6320, &args![global]);
        assert_eq!(e.get(global.cast::<TESForm>(), TESForm::cFormType), 6);
    }
    #[test]
    fn at_005a63c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_63f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_63c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_63c0)
                .collect();
            let mut expected = vec![(0x005a_63f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
