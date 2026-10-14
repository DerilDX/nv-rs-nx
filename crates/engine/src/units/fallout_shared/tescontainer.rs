//! `fallout shared/tescontainer.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable offset, source
/// type, target type, is-reference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESForm@@` and `.?AVTESScriptableForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_SCRIPTABLE_FORM: u32 = 0x0118_3254;
/// `00726070` (`ECX` = object): the word at `+4`; on a
/// `TESScriptableForm`, its script.
const WORD_AT_4: u32 = 0x0072_6070;

// Translated from 004826d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESScriptableForm::GetFormScript` (Xbox PDB; `cdecl`, one word): casts
/// the form from `TESForm` to `TESScriptableForm` (`__RTDynamicCast`) and
/// answers the component's word at `+4` (`00726070`), or 0 when the form
/// has no scriptable component.
pub fn tes_scriptable_form_get_form_script(e: &mut Engine, form: Ptr) -> Ptr {
    let scriptable = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0u32],
        )
        .u32();
    if scriptable == 0 {
        Ptr::new(0)
    } else {
        e.call(WORD_AT_4, &args![scriptable]).ptr()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x004826d0,
        tes_scriptable_form_get_form_script(Ptr) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORM: u32 = 0x0300_0000;
    const COMPONENT: u32 = 0x0300_0040;

    fn engine(cast_result: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(RT_DYNAMIC_CAST, move |_, _| Ret {
            eax: cast_result,
            ..Ret::default()
        });
        e.register(WORD_AT_4, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn answers_the_script_of_a_scriptable_form() {
        let mut e = engine(COMPONENT);
        let script = e.call(0x0048_26d0, &args![Ptr::<()>::new(FORM)]).u32();
        assert_eq!(script, COMPONENT + 0x1000);
        let log = e.call_log.unwrap();
        let casts: Vec<_> = log.iter().filter(|c| c.0 == RT_DYNAMIC_CAST).collect();
        assert_eq!(
            casts[0].1,
            vec![FORM, 0, TYPE_TES_FORM, TYPE_TES_SCRIPTABLE_FORM, 0]
        );
        let reads: Vec<_> = log.iter().filter(|c| c.0 == WORD_AT_4).collect();
        assert_eq!(reads[0].1, vec![COMPONENT]);
    }

    #[test]
    fn answers_zero_when_the_cast_fails() {
        let mut e = engine(0);
        let script = e.call(0x0048_26d0, &args![Ptr::<()>::new(FORM)]).u32();
        assert_eq!(script, 0);
        let log = e.call_log.unwrap();
        assert!(log.iter().all(|c| c.0 != WORD_AT_4));
    }
}
