//! `fallout shared/tesscriptableform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESScriptableForm` (Xbox PDB), a `BaseFormComponent`; 0xC bytes.
    pub struct TESScriptableForm: 0x0C {
        /// `pFormScript` (Xbox PDB): the `Script`; a form id until
        /// `InitItem` resolves it.
        0x04 pFormScript: u32,
        /// `bIsInitialized` (Xbox PDB).
        0x08 bIsInitialized: bool,
    }
}

/// `TESForm::GetFile` (Xbox PDB, `00484e60`): `thiscall (form; index)`.
const FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`): `cdecl (&form_id,
/// file)`.
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The form lookup by form id in the form map (`004839c0`, `cdecl (form_id)`).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor of `TESForm` (`.?AVTESForm@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor of `Script` (`.?AVScript@@`).
const TYPE_SCRIPT: u32 = 0x0118_46bc;
/// The owner's name length (`00474cb0`, `thiscall (form)`): 0 when the form
/// has no name.
const OWNER_NAME_LENGTH: u32 = 0x0047_4cb0;
/// The owner's name: the form's virtual slot `+0x130`.
const OWNER_NAME_SLOT: u32 = 0x130;
/// The owner's word at `+0xC`, its `TESForm` form id (`0084e3a0`, `thiscall (form)`).
const OWNER_FORM_ID: u32 = 0x0084_e3a0;
/// The warning printer (`005b5e40`, cdecl, format and its arguments).
const PRINT_WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find script (%08X) on unknown owner."
const MISSING_SCRIPT_NO_OWNER: u32 = 0x0101_d528;
/// "MASTERFILE: Unable to find script (%08X) on owner object (%08X)."
const MISSING_SCRIPT_OWNER_ID: u32 = 0x0101_d568;
/// "MASTERFILE: Unable to find script (%08X) on owner object \"%s\"."
const MISSING_SCRIPT_OWNER_NAME: u32 = 0x0101_d5ac;

// Translated from 0048cd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESScriptableForm::InitItem` (Xbox PDB), `thiscall (this; owner)`: once
/// (`bIsInitialized`), resolves the script form id in `pFormScript`. The id
/// gets the owner's file compile index (`TESForm::GetFile(owner, -1)`,
/// `TESForm::AddCompileIndex`), is looked up and cast to `Script`, and the
/// result (null when missing) replaces it. A missing script is reported with
/// the owner's name when it has one, else its form id, or as on an unknown
/// owner when there is none. A zero id is left as it is.
pub fn tes_scriptable_form_init_item(e: &mut Engine, this: Ptr<TESScriptableForm>, owner: Ptr) {
    if e.get(this, TESScriptableForm::bIsInitialized) {
        return;
    }
    if e.get(this, TESScriptableForm::pFormScript) != 0 {
        e.with_stack(4, |e, form_id| {
            let id = e.get(this, TESScriptableForm::pFormScript);
            e.mem.set_u32(form_id.addr(), id);
            let file = e.call(FORM_GET_FILE, &args![owner, -1i32]).u32();
            e.call(FORM_ADD_COMPILE_INDEX, &args![form_id, file]);
            let id = e.mem.u32(form_id.addr());
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            let script = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_SCRIPT, 0u32],
                )
                .u32();
            e.set(this, TESScriptableForm::pFormScript, script);
            if e.get(this, TESScriptableForm::pFormScript) != 0 {
                return;
            }
            if owner.addr() == 0 {
                e.call(PRINT_WARNING, &args![MISSING_SCRIPT_NO_OWNER, id]);
            } else if e.call(OWNER_NAME_LENGTH, &args![owner]).u32() != 0 {
                let name = e.vcall(owner.addr(), OWNER_NAME_SLOT, &args![]).u32();
                e.call(PRINT_WARNING, &args![MISSING_SCRIPT_OWNER_NAME, id, name]);
            } else {
                let owner_id = e.call(OWNER_FORM_ID, &args![owner]).u32();
                e.call(PRINT_WARNING, &args![MISSING_SCRIPT_OWNER_ID, id, owner_id]);
            }
        });
    }
    e.set(this, TESScriptableForm::bIsInitialized, true);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0048cd90,
        tes_scriptable_form_init_item(Ptr<TESScriptableForm>, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    const OWNER_VTABLE: u32 = 0x0170_0000;
    const OWNER_NAME: u32 = 0x0170_1000;
    const FILE: u32 = 0x0000_7000;
    const SCRIPT: u32 = 0x0000_8000;

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Runs `InitItem` on a component holding `id`; `found` says whether the
    /// lookup finds a `Script`, `named` whether the owner has a name.
    fn run(
        id: u32,
        initialized: bool,
        owner: bool,
        found: bool,
        named: bool,
    ) -> (Engine, Ptr<TESScriptableForm>, CallLog) {
        let mut e = Engine::new();
        let this = e.new_object::<TESScriptableForm>();
        e.set(this, TESScriptableForm::pFormScript, id);
        e.set(this, TESScriptableForm::bIsInitialized, initialized);
        let owner_form = if owner {
            let p = e.mem.alloc(0x20);
            let mut slots = vec![0u32; 0x130 / 4 + 1];
            slots[0x130 / 4] = OWNER_NAME;
            e.put_vtable(OWNER_VTABLE, &slots);
            e.mem.set_u32(p, OWNER_VTABLE);
            p
        } else {
            0
        };
        e.register(FORM_GET_FILE, |_, _| FILE.into_ret());
        // The compile index goes into the top byte.
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], (id & 0x00ff_ffff) | 0x0200_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] ^ 0x5000_0000).into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| {
            if found { SCRIPT } else { 0 }.into_ret()
        });
        e.register_double(OWNER_NAME_LENGTH, move |_, _| {
            if named { 4u32 } else { 0 }.into_ret()
        });
        e.register(OWNER_NAME, |_, _| 0x0000_9000u32.into_ret());
        e.register(OWNER_FORM_ID, |_, _| 0x0012_3456u32.into_ret());
        e.register(PRINT_WARNING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_cd90, &args![this, owner_form]);
        let log = e.call_log.take().unwrap();
        (e, this, log)
    }

    #[test]
    fn resolves_the_script() {
        let (e, this, log) = run(0x0000_1234, false, true, true, true);
        assert_eq!(e.get(this, TESScriptableForm::pFormScript), SCRIPT);
        assert!(e.get(this, TESScriptableForm::bIsInitialized));
        assert_eq!(calls_to(&log, FORM_GET_FILE)[0][1], u32::MAX);
        assert_eq!(calls_to(&log, FORM_ADD_COMPILE_INDEX)[0][1], FILE);
        assert_eq!(calls_to(&log, LOOKUP_FORM_BY_ID), vec![vec![0x0200_1234]]);
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x5200_1234, 0, TYPE_TES_FORM, TYPE_SCRIPT, 0]]
        );
        assert!(calls_to(&log, PRINT_WARNING).is_empty());
    }

    #[test]
    fn reports_a_missing_script_by_owner_name_id_or_unknown_owner() {
        let (e, this, log) = run(0x0000_1234, false, true, false, true);
        assert_eq!(e.get(this, TESScriptableForm::pFormScript), 0);
        assert!(e.get(this, TESScriptableForm::bIsInitialized));
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![vec![MISSING_SCRIPT_OWNER_NAME, 0x0200_1234, 0x0000_9000]]
        );
        let (_, _, log) = run(0x0000_1234, false, true, false, false);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![vec![MISSING_SCRIPT_OWNER_ID, 0x0200_1234, 0x0012_3456]]
        );
        let (_, _, log) = run(0x0000_1234, false, false, false, false);
        assert_eq!(
            calls_to(&log, PRINT_WARNING),
            vec![vec![MISSING_SCRIPT_NO_OWNER, 0x0200_1234]]
        );
        assert!(calls_to(&log, OWNER_NAME_LENGTH).is_empty());
    }

    #[test]
    fn a_zero_id_or_initialized_component_is_left_alone() {
        let (e, this, log) = run(0, false, true, true, true);
        assert_eq!(e.get(this, TESScriptableForm::pFormScript), 0);
        assert!(e.get(this, TESScriptableForm::bIsInitialized));
        assert!(calls_to(&log, FORM_GET_FILE).is_empty());
        let (e, this, log) = run(0x0000_1234, true, true, true, true);
        assert_eq!(e.get(this, TESScriptableForm::pFormScript), 0x0000_1234);
        assert!(calls_to(&log, FORM_GET_FILE).is_empty());
    }
}
