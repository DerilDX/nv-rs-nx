//! `fallout shared/teshealthform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESHealthForm::GetFormAsHealthForm` (Xbox PDB, `004872e0`): `cdecl`, one stack word; the
/// form's health part, or null.
const GET_FORM_AS_HEALTH_FORM: u32 = 0x0048_72e0;
/// The virtual slot (byte offset) of the health part that answers the health.
const SLOT_GET_HEALTH: u32 = 0x10;

// Translated from 004873d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESHealthForm::GetFormHealth` (Xbox PDB): `cdecl`, one stack word. Zero when the form has no
/// health part (`GetFormAsHealthForm`); otherwise what the part's virtual slot +0x10 answers.
pub fn tes_health_form_get_form_health(e: &mut Engine, form: Ptr) -> u32 {
    let part = e.call(GET_FORM_AS_HEALTH_FORM, &args![form]).u32();
    if part == 0 {
        0
    } else {
        e.vcall(part, SLOT_GET_HEALTH, &args![]).u32()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_73d0, tes_health_form_get_form_health(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004873d0_asks_the_health_part_for_the_health() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 8];
        slots[SLOT_GET_HEALTH as usize / 4] = 0x0f00_1000;
        e.put_vtable(0x0f00_1100, &slots);
        e.register(0x0f00_1000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        let part = e.mem.alloc(0x10);
        e.mem.set_u32(part, 0x0f00_1100);
        e.mem.set_u32(part + 8, 250);
        e.register_double(GET_FORM_AS_HEALTH_FORM, move |_, _| part.into_ret());
        assert_eq!(e.call(0x0048_73d0, &args![0x4000u32]).u32(), 250);
        let asked: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GET_FORM_AS_HEALTH_FORM)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(asked, vec![vec![0x4000]]);
    }

    #[test]
    fn at_004873d0_answers_zero_without_a_health_part() {
        let mut e = Engine::new();
        e.register(GET_FORM_AS_HEALTH_FORM, |_, _| Ret::default());
        assert_eq!(e.call(0x0048_73d0, &args![0x4000u32]).u32(), 0);
    }
}
