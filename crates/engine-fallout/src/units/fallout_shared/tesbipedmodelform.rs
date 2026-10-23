//! `fallout shared/tesbipedmodelform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401170` (unnamed in the engine map): `thiscall` getter, the form type byte at `this + 4`.
const FORM_TYPE_BYTE: u32 = 0x0040_1170;
/// `00726070` (unnamed in the engine map): `thiscall`; the dword at `this + 4` (a list node's
/// next).
const LIST_NEXT: u32 = 0x0072_6070;
/// `00500940` (unnamed in the engine map): `thiscall`; `this + 0x18` (the first node of a
/// container's list).
const LIST_FIRST_NODE_AT_18: u32 = 0x0050_0940;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, whether the node is empty
/// (no item and no next).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: returns `this` (the address of a node's item pointer).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;

// Translated from 00480db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetFormAsBipedModel` (Xbox PDB): `cdecl`, one stack word. For a non-null
/// form, answers its biped model part by the form's type byte (`00401170`): type 0x18 at
/// `form + 0x70`, type 0x1a at `form + 0x68`, type 0x60 at `form + 0x70`; null for any other type
/// or a null form. (The exe tests the form for null again inside each case; those tests cannot
/// fail and are kept.)
pub fn tes_biped_model_form_get_form_as_biped_model(e: &mut Engine, form: Ptr) -> Ptr {
    let mut result = 0;
    if !form.is_null() {
        let part_at = |offset: u32| {
            if form.is_null() {
                0
            } else {
                form.addr().wrapping_add(offset)
            }
        };
        match e.call(FORM_TYPE_BYTE, &args![form]).u32() {
            0x18 => result = part_at(0x70),
            0x1a => result = part_at(0x68),
            0x60 => result = part_at(0x70),
            _ => {}
        }
    }
    Ptr::new(result)
}

// Translated from 00480af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::FillsBipedSlot` (Xbox PDB): `thiscall`, `RET 0xc` (a slot number, a flag
/// byte, a container). False when the flag is set, the slot is 1 and bit 0 of the flags dword
/// at `this + 4` is set. Otherwise true when bit `slot & 31` of that dword is set; if it is
/// not and a container was given, walks the container's list (its first node comes from the
/// container's own list head `00726070`, then `00500940` on that) and asks every item that is a
/// biped model form (`GetFormAsBipedModel`) the same question, without a container, until one
/// answers true or the list ends.
pub fn tes_biped_model_form_fills_biped_slot(
    e: &mut Engine,
    this: Ptr,
    slot: u32,
    flag: u8,
    container: Ptr,
) -> bool {
    let flags = this.addr().wrapping_add(4);
    if flag != 0 && slot == 1 && e.mem.u32(flags) & 1 != 0 {
        return false;
    }
    let mut filled = (1u32.wrapping_shl(slot & 0x1f) & e.mem.u32(flags)) != 0;
    if !filled && !container.is_null() {
        let head = e.call(LIST_NEXT, &args![container]).u32();
        if head != 0 {
            let mut node = e.call(LIST_FIRST_NODE_AT_18, &args![head]).u32();
            while !filled && node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let item_slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let form = e.mem.u32(item_slot);
                let part = tes_biped_model_form_get_form_as_biped_model(e, Ptr::new(form));
                if !part.is_null() {
                    filled = tes_biped_model_form_fills_biped_slot(e, part, slot, flag, Ptr::NULL);
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
        }
    }
    filled
}

/// The components `InitializeDataComponent` and `ClearDataComponent` run,
/// in the exe's order: `bipedModel[0]`, `worldModel[0]`, `inventoryIcon[0]`,
/// `bipedModel[1]`, `worldModel[1]`, `inventoryIcon[1]`, `ConstraintTemplate`
/// (Xbox PDB names; the PC offsets are the same).
const DATA_COMPONENTS: [u32; 7] = [0x0c, 0x4c, 0x8c, 0x2c, 0x6c, 0x98, 0xc4];
/// Whether a model has a file name (`0048cee0`).
const MODEL_HAS_NAME: u32 = 0x0048_cee0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@` and the biped model form's
/// (`01183978`), as the code passes them.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_BIPED_MODEL_FORM: u32 = 0x0118_3978;
/// The model loader (a pointer at `011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;

// Translated from 004804d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::InitializeDataComponent` (Xbox PDB): clears the
/// biped model data (`+4` word, `+8` byte), then runs vtable slot 0 of
/// each data component (`DATA_COMPONENTS`).
pub fn tes_biped_model_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
    for offset in DATA_COMPONENTS {
        e.vcall(this.addr() + offset, 0, &args![]);
    }
}

// Translated from 00480570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::ClearDataComponent` (Xbox PDB): vtable slot 4 of
/// each data component (`DATA_COMPONENTS`).
pub fn tes_biped_model_form_clear_data_component(e: &mut Engine, this: Ptr) {
    for offset in DATA_COMPONENTS {
        e.vcall(this.addr() + offset, 4, &args![]);
    }
}

// Translated from 00480750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::InitItem` (Xbox PDB): `TESModelTextureSwap::InitItem`
/// (Xbox PDB, `0048aa80`) with `form` on `bipedModel[0]` (`+0xC`),
/// `worldModel[0]` (`+0x4C`), `bipedModel[1]` (`+0x2C`) and `worldModel[1]`
/// (`+0x6C`); `RET 4`.
pub fn tes_biped_model_form_init_item(e: &mut Engine, this: Ptr, form: u32) {
    for offset in [0x0c, 0x4c, 0x2c, 0x6c] {
        e.call(0x0048_aa80, &args![this.addr() + offset, form]);
    }
}

// Translated from 00480bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00480bd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESBipedModelForm::AddToBiped` (Xbox PDB), `RET 0xC`: `biped`, the
/// `sex` (1 falls back to 0 when `bipedModel[1]` has no file name,
/// `0048cee0`) and a third word. Gives `004abad0` (on `biped`) the form
/// (`this` cast from `01183978` to `TESForm`), `bipedModel[sex]`
/// (`this + 0xC + sex * 0x20`) and the third word. Then, when the
/// constraint template (`+0xC4`) has a file name, the model it names
/// (`00480ce0`) is looked up (`ModelLoader::FindModel`, `004472a0`, on the
/// loader at `011c3b3c`, into a stack `NiPointer` made by `00633c90` with
/// 0 and released by `0040c110`); a model found (`00559450`) has its
/// object (`0043b230`) applied (`Actor::ApplyConstrTemplate`, `008bbc40`)
/// to what `0043f010` answers for `biped`.
pub fn tes_biped_model_form_add_to_biped(
    e: &mut Engine,
    this: Ptr,
    biped: u32,
    sex: u32,
    third: u32,
) {
    let base = this.addr();
    let mut sex = sex;
    if sex == 1 && e.call(MODEL_HAS_NAME, &args![base + 0x2c]).u32() == 0 {
        sex = 0;
    }
    let model = base.wrapping_add(sex.wrapping_shl(5)).wrapping_add(0xc);
    let form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, TYPE_BIPED_MODEL_FORM, TYPE_TES_FORM, 0u32],
        )
        .u32();
    e.call(0x004a_bad0, &args![biped, form, model, third]);
    if e.call(MODEL_HAS_NAME, &args![base + 0xc4]).u32() == 0 {
        return;
    }
    let name = tes_biped_model_form_constraint_template_name(e, this);
    e.with_stack(4, |e, pointer| {
        e.call(0x0063_3c90, &args![pointer, 0u32]);
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(0x0044_72a0, &args![loader, name, pointer]);
        if e.call(0x0055_9450, &args![pointer]).u32() != 0 {
            let found = e.call(0x0055_9450, &args![pointer]).u32();
            let object = e.call(0x0043_b230, &args![found]).u32();
            let actor = e.call(0x0043_f010, &args![biped]).u32();
            e.call(0x008b_bc40, &args![actor, object]);
        }
        e.call(0x0040_c110, &args![pointer]);
    });
}

// Translated from 00480ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constraint template's model name: vtable slot 0x14 of the
/// `TESModelRDT` at `+0xC4`.
pub fn tes_biped_model_form_constraint_template_name(e: &mut Engine, this: Ptr) -> u32 {
    e.vcall(this.addr() + 0xc4, 0x14, &args![]).u32()
}

// Translated from 00480d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::IsPowerArmor` (Xbox PDB): bit 0x20 of the biped
/// model flags byte (`+8`).
pub fn tes_biped_model_form_is_power_armor(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 0x20 != 0
}

// Translated from 00480d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bit 4 of the biped model flags byte (`+8`).
pub fn fn_00480d30(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 4 != 0
}

// Translated from 00480d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` true) or clears bit 0x10 of the biped model flags byte
/// (`+8`); `RET 4`.
pub fn fn_00480d50(e: &mut Engine, this: Ptr, set: bool) {
    let flags = e.mem.u8(this.addr() + 8);
    let flags = if set { flags | 0x10 } else { flags & 0xef };
    e.mem.set_u8(this.addr() + 8, flags);
}

// Translated from 00480d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetPlayable` (Xbox PDB): true unless bit 0x40 of the
/// biped model flags byte (`+8`) is set.
pub fn tes_biped_model_form_get_playable(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 8) & 0x40 == 0
}

// Translated from 00481010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetWorldTESModel` (Xbox PDB), cdecl with two words:
/// `form` cast from `TESForm` to the biped model form (`01183978`); null
/// when that fails; `worldModel[0]` (`+0x4C`) without a reference;
/// otherwise the world model for the sex of the reference's owner
/// (`TESObjectREFR::GetOwner`, `00567790`): `TESActorBase::GetSex`
/// (`005f0cc0`) when the owner has form type 0x2A (`00401170`), else 0,
/// through `GetWorldTESModel_ov2`.
pub fn tes_biped_model_form_get_world_tes_model(e: &mut Engine, form: u32, reference: u32) -> u32 {
    let biped = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_BIPED_MODEL_FORM, 0u32],
        )
        .u32();
    if biped == 0 {
        return 0;
    }
    if reference == 0 {
        return biped + 0x4c;
    }
    let owner = e.call(0x0056_7790, &args![reference]).u32();
    let mut sex = 0;
    if owner != 0 && e.call(FORM_TYPE_BYTE, &args![owner]).u32() == 0x2a {
        sex = e.call(0x005f_0cc0, &args![owner]).u32();
    }
    tes_biped_model_form_get_world_tes_model_ov2(e, Ptr::new(biped), sex)
}

// Translated from 00481110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetWorldTESModel_ov2` (Xbox PDB): `worldModel[sex]`
/// (`this + 0x4C + sex * 0x20`), sex 1 falling back to 0 when that model
/// has no file name (`0048cee0`); `RET 4`.
pub fn tes_biped_model_form_get_world_tes_model_ov2(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    this.addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0x4c)
}

// Translated from 00481150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESBipedModelForm::GetBipedTESModel_ov2` (engine map): `bipedModel[sex]`
/// (`this + 0xC + sex * 0x20`), sex 1 falling back to 0 when that model has no file name
/// (`0048cee0`); `RET 4`.
pub fn tes_biped_model_form_get_biped_tes_model_ov2(e: &mut Engine, this: Ptr, sex: u32) -> u32 {
    let mut sex = sex;
    let model = this
        .addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc);
    if e.call(MODEL_HAS_NAME, &args![model]).u32() == 0 && sex == 1 {
        sex = 0;
    }
    this.addr()
        .wrapping_add(sex.wrapping_shl(5))
        .wrapping_add(0xc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_1150, tes_biped_model_form_get_biped_tes_model_ov2(Ptr, u32) -> u32),
        entry!(0x0048_0db0, tes_biped_model_form_get_form_as_biped_model(Ptr) -> Ptr),
        entry!(0x0048_0af0, tes_biped_model_form_fills_biped_slot(Ptr, u32, u8, Ptr) -> bool),
        entry!(
            0x0048_04d0,
            tes_biped_model_form_initialize_data_component(Ptr)
        ),
        entry!(0x0048_0570, tes_biped_model_form_clear_data_component(Ptr)),
        entry!(0x0048_0750, tes_biped_model_form_init_item(Ptr, u32)),
        entry!(
            0x0048_0bd0,
            tes_biped_model_form_add_to_biped(Ptr, u32, u32, u32)
        ),
        entry!(0x0048_0ce0, tes_biped_model_form_constraint_template_name(Ptr) -> u32),
        entry!(0x0048_0d10, tes_biped_model_form_is_power_armor(Ptr) -> bool),
        entry!(0x0048_0d30, fn_00480d30(Ptr) -> bool),
        entry!(0x0048_0d50, fn_00480d50(Ptr, bool)),
        entry!(0x0048_0d90, tes_biped_model_form_get_playable(Ptr) -> bool),
        entry!(0x0048_1010, tes_biped_model_form_get_world_tes_model(u32, u32) -> u32),
        entry!(0x0048_1110, tes_biped_model_form_get_world_tes_model_ov2(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod b023_tests {
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

    /// A form whose components share one vtable: slot 0 is `0x7000_0000`,
    /// slot 4 `0x7000_0004`, slot 0x14 `0x7000_0014`.
    fn form(e: &mut Engine) -> u32 {
        let mut slots = vec![0u32; 6];
        slots[0] = 0x7000_0000;
        slots[1] = 0x7000_0004;
        slots[5] = 0x7000_0014;
        e.put_vtable(0x0600_0000, &slots);
        let form = e.mem.alloc(0xdc);
        for offset in DATA_COMPONENTS {
            e.mem.set_u32(form + offset, 0x0600_0000);
        }
        form
    }

    #[test]
    fn data_components_are_initialized_and_cleared_in_order() {
        let mut e = Engine::new();
        e.register(0x7000_0000, |_, _| Ret::default());
        e.register(0x7000_0004, |_, _| Ret::default());
        let f = form(&mut e);
        e.mem.set_u32(f + 4, 0xffff_ffff);
        e.mem.set_u8(f + 8, 0xff);
        e.mem.set_u8(f + 9, 0xee);
        e.call_log = Some(vec![]);
        e.call(0x0048_04d0, &[f]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = calls_to(&log, 0x7000_0000)
            .iter()
            .map(|w| w[0] - f)
            .collect();
        assert_eq!(order, DATA_COMPONENTS.to_vec());
        assert_eq!(e.mem.u32(f + 4), 0);
        assert_eq!(e.mem.u8(f + 8), 0);
        assert_eq!(e.mem.u8(f + 9), 0xee);
        e.call_log = Some(vec![]);
        e.call(0x0048_0570, &[f]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = calls_to(&log, 0x7000_0004)
            .iter()
            .map(|w| w[0] - f)
            .collect();
        assert_eq!(order, DATA_COMPONENTS.to_vec());
    }

    #[test]
    fn init_item_and_flag_bits() {
        let mut e = Engine::new();
        e.register(0x0048_aa80, |_, _| Ret::default());
        let f = e.mem.alloc(0xdc);
        e.call_log = Some(vec![]);
        e.call(0x0048_0750, &[f, 0x99]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x0048_aa80),
            vec![
                vec![f + 0xc, 0x99],
                vec![f + 0x4c, 0x99],
                vec![f + 0x2c, 0x99],
                vec![f + 0x6c, 0x99]
            ]
        );
        for (flags, power, four, playable) in [
            (0u8, false, false, true),
            (0x20, true, false, true),
            (0x04, false, true, true),
            (0x40, false, false, false),
        ] {
            e.mem.set_u8(f + 8, flags);
            assert_eq!(e.call(0x0048_0d10, &[f]).bool(), power);
            assert_eq!(e.call(0x0048_0d30, &[f]).bool(), four);
            assert_eq!(e.call(0x0048_0d90, &[f]).bool(), playable);
        }
        e.mem.set_u8(f + 8, 0x81);
        e.call(0x0048_0d50, &[f, 0x100]);
        assert_eq!(e.mem.u8(f + 8), 0x81);
        e.call(0x0048_0d50, &[f, 1]);
        assert_eq!(e.mem.u8(f + 8), 0x91);
        e.call(0x0048_0d50, &[f, 0]);
        assert_eq!(e.mem.u8(f + 8), 0x81);
    }

    #[test]
    fn world_models_by_sex() {
        let mut e = Engine::new();
        // Model at +0x6c has no name; any other has one.
        e.register_double(MODEL_HAS_NAME, |_, a| ret(u32::from(a[0] & 0xff != 0x6c)));
        let f = 0x0010_0000u32;
        assert_eq!(e.call(0x0048_1110, &[f, 0]).u32(), f + 0x4c);
        assert_eq!(e.call(0x0048_1110, &[f, 1]).u32(), f + 0x4c);
        assert_eq!(e.call(0x0048_1110, &[f, 2]).u32(), f + 0x8c);
        let g = 0x0010_0100u32 - 0x4c + 0x20;
        // A sex-1 model with a name stays.
        assert_eq!(e.call(0x0048_1110, &[g, 1]).u32(), g + 0x6c);

        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[0] == 0x55 { 0 } else { a[0] })
        });
        e.register(0x0056_7790, |_, a| ret(a[0] + 1));
        e.register(FORM_TYPE_BYTE, |_, a| {
            ret(if a[0] == 0x301 { 0x2a } else { 0x2b })
        });
        e.register(0x005f_0cc0, |_, _| ret(2));
        assert_eq!(e.call(0x0048_1010, &[0x55, 0]).u32(), 0);
        assert_eq!(e.call(0x0048_1010, &[f, 0]).u32(), f + 0x4c);
        // An NPC owner (type 0x2a) gives its sex.
        assert_eq!(e.call(0x0048_1010, &[f, 0x300]).u32(), f + 0x8c);
        // Another owner, or none, gives sex 0.
        assert_eq!(e.call(0x0048_1010, &[f, 0x400]).u32(), f + 0x4c);
        e.register(0x0056_7790, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0048_1010, &[f, 0x300]).u32(), f + 0x4c);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_TYPE_BYTE).is_empty());
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![f, 0, TYPE_TES_FORM, TYPE_BIPED_MODEL_FORM, 0]]
        );
    }

    #[test]
    fn add_to_biped_and_the_constraint_template() {
        let mut e = Engine::new();
        let f = form(&mut e);
        e.register(0x7000_0014, |_, _| ret(0x0123_0000));
        // Female model (+0x2c) has no name; the template (+0xc4) has one.
        e.register_double(MODEL_HAS_NAME, move |_, a| ret(u32::from(a[0] != f + 0x2c)));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        for a in [0x004a_bad0, 0x0063_3c90, 0x0040_c110, 0x008b_bc40] {
            e.register(a, |_, _| Ret::default());
        }
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER, 0x0777_0000u32);
        e.register(0x0044_72a0, |e, a| {
            e.mem.set_u32(a[2], 0x0888_0000);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_b230, |_, a| ret(a[0] + 2));
        e.register(0x0043_f010, |_, a| ret(a[0] + 3));
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 1, 0x42]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![f, 0, TYPE_BIPED_MODEL_FORM, TYPE_TES_FORM, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x004a_bad0),
            vec![vec![0x0500_0000, f + 1, f + 0xc, 0x42]]
        );
        let pointer = calls_to(&log, 0x0063_3c90)[0][0];
        assert_eq!(calls_to(&log, 0x0063_3c90)[0][1], 0);
        assert_eq!(
            calls_to(&log, 0x0044_72a0),
            vec![vec![0x0777_0000, 0x0123_0000, pointer]]
        );
        assert_eq!(
            calls_to(&log, 0x008b_bc40),
            vec![vec![0x0500_0003, 0x0888_0002]]
        );
        assert_eq!(calls_to(&log, 0x0040_c110), vec![vec![pointer]]);

        // Sex 2 is used as given; no model found: nothing applied; no
        // template name: no lookup.
        e.register(0x0044_72a0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 2, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_bad0)[0][2], f + 0x4c);
        assert!(calls_to(&log, 0x008b_bc40).is_empty());
        assert_eq!(calls_to(&log, 0x0040_c110).len(), 1);
        e.register_double(MODEL_HAS_NAME, move |_, a| ret(u32::from(a[0] != f + 0xc4)));
        e.call_log = Some(vec![]);
        e.call(0x0048_0bd0, &[f, 0x0500_0000, 1, 0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x004a_bad0)[0][2], f + 0x2c);
        assert!(calls_to(&log, 0x0063_3c90).is_empty());
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(FORM_TYPE_BYTE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_FIRST_NODE_AT_18, |_, a| (a[0] + 0x18).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e
    }

    fn form_of_type(e: &mut Engine, kind: u32) -> u32 {
        let form = e.mem.alloc(0x80);
        e.mem.set_u32(form + 4, kind);
        form
    }

    #[test]
    fn at_00480db0_answers_the_part_each_type_selects() {
        let mut e = rig();
        for (kind, offset) in [(0x18u32, 0x70u32), (0x1a, 0x68), (0x60, 0x70)] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(
                e.call(0x0048_0db0, &args![form]).u32(),
                form + offset,
                "{kind:#x}"
            );
        }
        for kind in [0u32, 0x17, 0x19, 0x61, 0xff] {
            let form = form_of_type(&mut e, kind);
            assert_eq!(e.call(0x0048_0db0, &args![form]).u32(), 0, "{kind:#x}");
        }
        assert_eq!(e.call(0x0048_0db0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, FORM_TYPE_BYTE).len(), 8);
    }

    #[test]
    fn at_00480af0_tests_the_slot_bit() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b1010);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, 0u32]).bool());
        assert!(e.call(0x0048_0af0, &args![this, 3u32, 0u8, 0u32]).bool());
        // The shift count is taken modulo 32.
        assert!(e.call(0x0048_0af0, &args![this, 33u32, 0u8, 0u32]).bool());
        assert!(!e.call(0x0048_0af0, &args![this, 34u32, 0u8, 0u32]).bool());
    }

    #[test]
    fn at_00480af0_the_flag_with_slot_1_and_bit_0_means_false() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0b11);
        assert!(!e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
        // Without the flag the slot-1 bit is true as usual.
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 0u8, 0u32]).bool());
        // Another slot is unaffected by the flag.
        assert!(e.call(0x0048_0af0, &args![this, 0u32, 1u8, 0u32]).bool());
        // Slot 1 with bit 0 clear: the slot bit decides.
        e.mem.set_u32(this + 4, 0b10);
        assert!(e.call(0x0048_0af0, &args![this, 1u32, 1u8, 0u32]).bool());
    }

    /// A container whose list head sits at `container + 4` (`00726070`), its nodes from
    /// `head + 0x18`: (item, next).
    fn container_of(e: &mut Engine, forms: &[u32]) -> u32 {
        let container = e.mem.alloc(0x10);
        let head = e.mem.alloc(0x40);
        e.mem.set_u32(container + 4, head);
        let mut next = 0;
        let mut nodes = Vec::new();
        for form in forms.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *form);
            e.mem.set_u32(node + 4, next);
            next = node;
            nodes.push(node);
        }
        // The first node is the head's list head: `00500940` answers `head + 0x18`, so the
        // first node is stored there.
        let first = nodes.last().copied().unwrap_or(0);
        if first != 0 {
            e.mem.set_u32(head + 0x18, e.mem.u32(first));
            e.mem.set_u32(head + 0x1c, e.mem.u32(first + 4));
        }
        container
    }

    #[test]
    fn at_00480af0_asks_the_container_s_forms_when_the_slot_is_not_set_here() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0);
        let unrelated = form_of_type(&mut e, 0x19);
        let biped_form = form_of_type(&mut e, 0x18);
        // The biped part of the form (at +0x70) has the slot bit set.
        e.mem.set_u32(biped_form + 0x70 + 4, 0b100);
        let container = container_of(&mut e, &[unrelated, biped_form]);
        assert!(e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        let types = calls(&e, FORM_TYPE_BYTE);
        assert_eq!(types, vec![vec![unrelated], vec![biped_form]]);
    }

    #[test]
    fn at_00480af0_answers_false_when_no_form_of_the_container_fills_the_slot() {
        let mut e = rig();
        let this = e.mem.alloc(0x10);
        let biped_form = form_of_type(&mut e, 0x18);
        let container = container_of(&mut e, &[biped_form]);
        assert!(!e
            .call(0x0048_0af0, &args![this, 2u32, 0u8, container])
            .bool());
        // An empty container (no list head) ends the search at once.
        let bare = e.mem.alloc(0x10);
        assert!(!e.call(0x0048_0af0, &args![this, 2u32, 0u8, bare]).bool());
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn get_biped_tes_model_falls_back_to_the_male_model() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        // The female model (+0x2C) has a name only when its word +4 is set.
        e.register(MODEL_HAS_NAME, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let at = |e: &mut Engine, sex: u32| e.call(0x0048_1150, &args![this, sex]).u32();
        assert_eq!(at(&mut e, 0), this + 0xc);
        assert_eq!(at(&mut e, 1), this + 0xc);
        e.mem.set_u32(this + 0x2c + 4, 1);
        assert_eq!(at(&mut e, 1), this + 0x2c);
        // Other indices are not checked: 2 without a name stays 2.
        assert_eq!(at(&mut e, 2), this + 0x4c);
    }
}
