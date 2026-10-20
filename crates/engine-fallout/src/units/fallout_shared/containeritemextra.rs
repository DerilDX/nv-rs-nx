//! `fallout shared/containeritemextra.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `ContainerItemExtra` (Xbox PDB), 0xC bytes (the same on PC).
    pub struct ContainerItemExtra: 0x0C {
        /// `pOwnerForm` (Xbox PDB): `TESForm*` (a form id until `InitItem`).
        0x00 pOwnerForm: u32,
        /// `pOwnerGlobal` / `iOwnerRank` (Xbox PDB, a union).
        0x04 iUnion: u32,
        /// `fHealthMult` (Xbox PDB), as stored bits.
        0x08 fHealthMult: u32,
    }
}

/// The `COED` chunk signature, as the word the exe compares.
const SIG_COED: u32 = 0x4445_4f43;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `RET 4`).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, cdecl, two words).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks up a form by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and `.?AVTESGlobal@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_GLOBAL: u32 = 0x0118_3ad4;
/// `TESForm`'s form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;
/// The form type byte (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// The warning printer (`005b5e40`, cdecl, variable arguments).
const WARNING: u32 = 0x005b_5e40;
/// "MASTERFILE: Failed to find owner form (%08X) in init for
/// ContainerItemExtra for form '%s' (%08X)."
const MISSING_OWNER_MESSAGE: u32 = 0x0101_4168;
/// "MASTERFILE: Failed to find owner global (%08X) in init for
/// ContainerItemExtra for form '%s' (%08X)."
const MISSING_GLOBAL_MESSAGE: u32 = 0x0101_4100;

// Translated from 0040e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerItemExtra::ContainerItemExtra` (Xbox PDB): no owner, no
/// global or rank, a health multiplier of 1.0 (`FLD1`). Returns `this`.
pub fn container_item_extra_container_item_extra(
    e: &mut Engine,
    this: Ptr<ContainerItemExtra>,
) -> Ptr<ContainerItemExtra> {
    e.set(this, ContainerItemExtra::pOwnerForm, 0);
    e.set(this, ContainerItemExtra::iUnion, 0);
    e.set(this, ContainerItemExtra::fHealthMult, 1.0f32.to_bits());
    this
}

// Translated from 0040e780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the extra from `file` (`RET 4`): with a file whose next chunk
/// (`TESFile::GetTESChunk`, `004726b0`) is `COED`, 0xC bytes are read into a
/// stack copy (`00472890`), swapped (`005a6b10`) when the file says so
/// (`00401680`), and copied in (the multiplier through `FLD`/`FSTP`, a
/// signalling NaN coming out quiet); true then, false otherwise.
pub fn fn_0040e780(e: &mut Engine, this: Ptr<ContainerItemExtra>, file: Ptr) -> bool {
    if file.is_null() || e.call(0x0047_26b0, &args![file]).u32() != SIG_COED {
        return false;
    }
    e.with_stack(0xc, |e, copy| {
        e.call(0x0047_2890, &args![file, copy, 0xcu32]);
        if e.call(0x0040_1680, &args![file]).bool() {
            e.call(0x005a_6b10, &args![copy]);
        }
        let owner = e.mem.u32(copy.addr());
        let union = e.mem.u32(copy.addr() + 4);
        let health = x87_quiet_bits(e.mem.u32(copy.addr() + 8));
        e.set(this, ContainerItemExtra::pOwnerForm, owner);
        e.set(this, ContainerItemExtra::iUnion, union);
        e.set(this, ContainerItemExtra::fHealthMult, health);
    });
    true
}

// Translated from 0040e7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ContainerItemExtra::InitItem` (Xbox PDB), `form` the form holding it
/// (`RET 4`): an owner id gets the compile index of `form`'s file
/// (`00485d50` with `TESForm::GetFile(-1)`) and is looked up (`004839c0`);
/// missing, it is reported (with the id, `form`'s vtable slot 0x130 answer
/// and form id). Without an owner the union word is cleared; with an owner
/// of form type 0x2A (`00401170`) a global id in it that is not zero is
/// resolved the same way and cast to `TESGlobal`, a missing one reported.
pub fn container_item_extra_init_item(e: &mut Engine, this: Ptr<ContainerItemExtra>, form: Ptr) {
    let resolve = |e: &mut Engine, id_at: Ptr| {
        let file = e.call(GET_FILE, &args![form, u32::MAX]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
        let id = e.mem.u32(id_at.addr());
        e.call(LOOKUP_FORM, &args![id]).u32()
    };
    let report = |e: &mut Engine, message: u32, id: u32| {
        let form_id = e.call(FORM_ID, &args![form]).u32();
        let name = e.vcall(form.addr(), 0x130, &args![]).u32();
        e.call(WARNING, &args![message, id, name, form_id]);
    };
    e.with_stack(8, |e, locals| {
        let owner_at = locals;
        let global_at = locals.byte_add(4);
        if e.get(this, ContainerItemExtra::pOwnerForm) != 0 {
            let owner = e.get(this, ContainerItemExtra::pOwnerForm);
            e.mem.set_u32(owner_at.addr(), owner);
            let found = resolve(e, owner_at);
            e.set(this, ContainerItemExtra::pOwnerForm, found);
            if e.get(this, ContainerItemExtra::pOwnerForm) == 0 {
                let id = e.mem.u32(owner_at.addr());
                report(e, MISSING_OWNER_MESSAGE, id);
            }
        }
        let owner = e.get(this, ContainerItemExtra::pOwnerForm);
        if owner == 0 {
            e.set(this, ContainerItemExtra::iUnion, 0);
        } else if e.call(FORM_TYPE, &args![owner]).u32() == 0x2a
            && e.get(this, ContainerItemExtra::iUnion) != 0
        {
            let global = e.get(this, ContainerItemExtra::iUnion);
            e.mem.set_u32(global_at.addr(), global);
            let found = resolve(e, global_at);
            let cast = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![found, 0u32, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0u32],
                )
                .u32();
            e.set(this, ContainerItemExtra::iUnion, cast);
            if e.get(this, ContainerItemExtra::iUnion) == 0 {
                let id = e.mem.u32(global_at.addr());
                report(e, MISSING_GLOBAL_MESSAGE, id);
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040_e690,
            container_item_extra_container_item_extra(
                Ptr<ContainerItemExtra>,
            ) -> Ptr<ContainerItemExtra>
        ),
        entry!(
            0x0040_e780,
            fn_0040e780(Ptr<ContainerItemExtra>, Ptr) -> bool
        ),
        entry!(
            0x0040_e7f0,
            container_item_extra_init_item(Ptr<ContainerItemExtra>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_values() {
        let mut e = Engine::new();
        let this: Ptr<ContainerItemExtra> = e.new_object();
        e.mem.write(this.addr(), &[0xcd; 0xc]);
        assert_eq!(e.call(0x0040_e690, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.f32(this.addr() + 8), 1.0);
    }

    #[test]
    fn load_reads_a_coed_chunk() {
        let mut e = Engine::new();
        let this: Ptr<ContainerItemExtra> = e.new_object();
        assert!(!e.call(0x0040_e780, &args![this, 0u32]).bool());
        e.register(0x0047_26b0, |_, _| ret(0x1234));
        assert!(!e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        e.register(0x0047_26b0, |_, _| ret(SIG_COED));
        e.register(0x0047_2890, |e, a| {
            assert_eq!(a[2], 0xc);
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u32(a[1] + 4, 0x22);
            e.mem.set_u32(a[1] + 8, 0x7f80_0003);
            Ret::default()
        });
        e.register(0x0040_1680, |_, _| ret(0));
        e.register(0x005a_6b10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        assert!(calls_to(e.call_log.as_ref().unwrap(), 0x005a_6b10).is_empty());
        assert_eq!(e.mem.u32(this.addr()), 0x11);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x22);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x7fc0_0003);
        // Swapped data: the swap runs on the stack copy before the copy in.
        e.register(0x0040_1680, |_, _| ret(1));
        e.register(0x005a_6b10, |e, a| {
            e.mem.set_u32(a[0], 0x33);
            Ret::default()
        });
        assert!(e.call(0x0040_e780, &args![this, 0x500u32]).bool());
        assert_eq!(e.mem.u32(this.addr()), 0x33);
    }

    #[test]
    fn init_item_resolves_owner_and_global() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0x18);
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0x130 / 4] = 0x7000_0000;
        e.put_vtable(0x0600_0000, &slots);
        e.mem.set_u32(form, 0x0600_0000);
        e.register(0x7000_0000, |_, _| ret(0x0102_0304));
        e.register(FORM_ID, |_, _| ret(0x00ab_cdef));
        e.register(GET_FILE, |_, _| ret(0x0400_0000));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            assert_eq!(a[1], 0x0400_0000);
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x100);
            Ret::default()
        });
        // Ids ending in 1 resolve to forms of type 0x2a, ending in 2 to
        // type 0x1c.
        e.register(LOOKUP_FORM, |_, a| {
            ret(match a[0] & 0xf {
                1 | 2 => a[0] + 0x1000,
                _ => 0,
            })
        });
        e.register(FORM_TYPE, |_, a| {
            ret(if a[0] & 0xf == 1 { 0x2a } else { 0x1c })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(WARNING, |_, _| Ret::default());
        let this: Ptr<ContainerItemExtra> = e.new_object();
        let t = this.addr();
        // An NPC owner and a global that resolves.
        e.mem.set_u32(t, 1);
        e.mem.set_u32(t + 4, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t), 0x1101);
        assert_eq!(e.mem.u32(t + 4), 0x1111);
        assert!(calls_to(&log, WARNING).is_empty());
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![0x1111, 0, TYPE_TES_FORM, TYPE_TES_GLOBAL, 0]]
        );
        // An NPC owner with a missing global: reported, null.
        e.mem.set_u32(t, 1);
        e.mem.set_u32(t + 4, 0x13);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t + 4), 0);
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![
                MISSING_GLOBAL_MESSAGE,
                0x113,
                0x0102_0304,
                0x00ab_cdef
            ]]
        );
        // Another owner type keeps the rank word.
        e.mem.set_u32(t, 2);
        e.mem.set_u32(t + 4, 5);
        e.call(0x0040_e7f0, &[t, form]);
        assert_eq!(e.mem.u32(t + 4), 5);
        // A missing owner: reported, and the union cleared.
        e.mem.set_u32(t, 3);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, WARNING),
            vec![vec![MISSING_OWNER_MESSAGE, 0x103, 0x0102_0304, 0x00ab_cdef]]
        );
        assert_eq!(e.mem.u32(t + 4), 0);
        // No owner at all: the union is cleared without a lookup.
        e.mem.set_u32(t + 4, 9);
        e.call_log = Some(vec![]);
        e.call(0x0040_e7f0, &[t, form]);
        assert!(calls_to(e.call_log.as_ref().unwrap(), LOOKUP_FORM).is_empty());
        assert_eq!(e.mem.u32(t + 4), 0);
    }
}
