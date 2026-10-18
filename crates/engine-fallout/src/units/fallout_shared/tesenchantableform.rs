//! `fallout shared/tesenchantableform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00483210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEnchantableForm::InitItem` (Xbox PDB): turns the enchantment form id stored at `+0x4` of
/// `this` into the form. `owner` is the form that owns the enchantable part (it may be null).
///
/// The id gets the owner file's compile index (`TESForm::GetFile` `00484e60` with -1, then
/// `TESForm::AddCompileIndex` `00485d50`), is looked up (`004839c0`) and cast with
/// `__RTDynamicCast` from the type at `0x01183028` to the type at `0x01183594`. When nothing is
/// found, one of three `MASTERFILE: Unable to find enchanting ...` messages goes to `005b5e40`
/// (no owner; owner for which `00474cb0` answers non-zero, which prints the owner's name through
/// virtual slot `+0x130`; any other owner, which prints `0084e3a0`'s answer). When an enchantment
/// was found for an owner but `004094f0` (on the enchantment, with the owner) answers false, the
/// `has invalid effect trigger type or effect` message is printed and the field is cleared.
pub fn tesenchantableform_init_item(e: &mut Engine, this: Ptr, owner: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        return;
    }
    let slot = e.mem.alloc(4);
    let id_in = e.mem.u32(base + 4);
    e.mem.set_u32(slot, id_in);
    if owner.addr() != 0 {
        let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
        e.call(0x0048_5d50, &args![slot, file]);
    }
    let id = e.mem.u32(slot);
    let form = e.call(0x0048_39c0, &args![id]).u32();
    let found = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_3028u32, 0x0118_3594u32, 0u32],
        )
        .u32();
    e.mem.set_u32(base + 4, found);
    if found == 0 {
        if owner.addr() == 0 {
            e.call(0x005b_5e40, &args![0x0101_c1c4u32, id]);
        } else if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(0x005b_5e40, &args![0x0101_c250u32, id, name]);
        } else {
            let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
            e.call(0x005b_5e40, &args![0x0101_c208u32, id, owner_id]);
        }
    }
    let enchantment = e.mem.u32(base + 4);
    if enchantment != 0
        && owner.addr() != 0
        && !e.call(0x0040_94f0, &args![enchantment, owner]).bool()
    {
        let owner_name = e.vcall(owner.addr(), 0x130, &args![]).u32();
        let owner_id = e.call(0x0084_e3a0, &args![owner]).u32();
        let enchantment = e.mem.u32(base + 4);
        let enchantment_name = e.vcall(enchantment, 0x130, &args![]).u32();
        let enchantment = e.mem.u32(base + 4);
        let enchantment_id = e.call(0x0084_e3a0, &args![enchantment]).u32();
        e.call(
            0x005b_5e40,
            &args![
                0x0101_af00u32,
                enchantment_id,
                enchantment_name,
                owner_id,
                owner_name
            ],
        );
        e.mem.set_u32(base + 4, 0);
    }
    e.mem.free(slot);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_3210, tesenchantableform_init_item(Ptr, Ptr))]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{calls_to, run, stub};

    const OWNER: u32 = 0x0800_0000;
    const ENCHANTMENT: u32 = 0x0800_1000;
    const FILE: u32 = 0x0a11_e000;
    const OWNER_NAME: u32 = 0xaaaa_0001;
    const ENCHANTMENT_NAME: u32 = 0xbbbb_0002;

    /// An engine where the lookup (`004839c0`) answers `ENCHANTMENT` when `found`, the owner's
    /// `00474cb0` answers `named`, and `004094f0` answers `valid`. Returns the engine and `this`.
    fn setup(id: u32, found: bool, named: u32, valid: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, id);
        // Both forms have a virtual slot `+0x130` that answers their name.
        for (object, vtable, target, name) in [
            (OWNER, 0x0900_0000, 0x0090_0300, OWNER_NAME),
            (ENCHANTMENT, 0x0900_1000, 0x0090_0400, ENCHANTMENT_NAME),
        ] {
            e.map(object, 0x40);
            e.map(vtable, 0x200);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(vtable + 0x130, target);
            e.register_double(target, move |_, _| Ret {
                eax: name,
                ..Ret::default()
            });
        }
        stub(&mut e, &[0x005b_5e40]);
        e.register(0x0048_4e60, |_, _| Ret {
            eax: FILE,
            ..Ret::default()
        });
        // The compile index goes into the top byte of the id.
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(0x0048_39c0, |_, a| Ret {
            eax: a[0] ^ 0x1000_0000,
            ..Ret::default()
        });
        e.register_double(0x00ec_43fb, move |_, _| Ret {
            eax: if found { ENCHANTMENT } else { 0 },
            ..Ret::default()
        });
        e.register_double(0x0047_4cb0, move |_, _| Ret {
            eax: named,
            ..Ret::default()
        });
        e.register(0x0084_e3a0, |_, a| Ret {
            eax: a[0] + 0x77,
            ..Ret::default()
        });
        e.register_double(0x0040_94f0, move |_, _| Ret {
            eax: u32::from(valid),
            ..Ret::default()
        });
        (e, this)
    }

    #[test]
    fn nothing_happens_without_an_enchantment_id() {
        let (mut e, this) = setup(0, true, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert!(calls.is_empty());
    }

    #[test]
    fn a_found_enchantment_is_stored_after_the_compile_index() {
        let (mut e, this) = setup(0x0000_1234, true, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(e.mem.u32(this + 4), ENCHANTMENT);
        assert_eq!(calls_to(&calls, 0x0048_4e60), [vec![OWNER, 0xffff_ffff]]);
        assert_eq!(calls_to(&calls, 0x0048_39c0), [vec![0x0100_1234]]);
        // The cast takes the lookup's answer.
        assert_eq!(
            calls_to(&calls, 0x00ec_43fb),
            [vec![0x1100_1234, 0, 0x0118_3028, 0x0118_3594, 0]]
        );
        assert_eq!(calls_to(&calls, 0x0040_94f0), [vec![ENCHANTMENT, OWNER]]);
        assert!(calls_to(&calls, 0x005b_5e40).is_empty());
    }

    #[test]
    fn a_missing_enchantment_reports_the_owner_it_has() {
        // No owner: no compile index, the short message.
        let (mut e, this) = setup(0x1234, false, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, 0]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert!(calls_to(&calls, 0x0048_4e60).is_empty());
        assert_eq!(calls_to(&calls, 0x005b_5e40), [vec![0x0101_c1c4, 0x1234]]);
        assert!(calls_to(&calls, 0x0040_94f0).is_empty());

        // An owner `00474cb0` answers for: its name.
        let (mut e, this) = setup(0x1234, false, 1, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_c250, 0x0100_1234, OWNER_NAME]]
        );

        // Any other owner: its `0084e3a0` answer.
        let (mut e, this) = setup(0x1234, false, 0, true);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![0x0101_c208, 0x0100_1234, OWNER + 0x77]]
        );
    }

    #[test]
    fn an_enchantment_the_owner_rejects_is_dropped_with_a_message() {
        let (mut e, this) = setup(0x1234, true, 1, false);
        let (_, calls) = run(&mut e, 0x0048_3210, &[this, OWNER]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(
            calls_to(&calls, 0x005b_5e40),
            [vec![
                0x0101_af00,
                ENCHANTMENT + 0x77,
                ENCHANTMENT_NAME,
                OWNER + 0x77,
                OWNER_NAME
            ]]
        );
    }
}
