//! `fallout shared/magic/ingredientitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004013e0` (thiscall on the form, returns `AL`; unnamed): when true, nothing is
/// initialized.
const FORM_TEST_004013E0: u32 = 0x0040_13e0;
/// `TESScriptableForm::InitItem` (`0048cd90`, thiscall: the owner form).
const TES_SCRIPTABLE_FORM_INIT_ITEM: u32 = 0x0048_cd90;
/// `BGSDestructibleObjectForm::InitItem` (`00478be0`, thiscall: the owner form).
const BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM: u32 = 0x0047_8be0;
/// `TESModelTextureSwap::InitItem` (`0048aa80`, thiscall: the owner form).
const TES_MODEL_TEXTURE_SWAP_INIT_ITEM: u32 = 0x0048_aa80;
/// `MagicItemObject::InitItem` (`0040c9d0`).
const MAGIC_ITEM_OBJECT_INIT_ITEM: u32 = 0x0040_c9d0;

// Translated from 00409d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IngredientItem::InitItem` (Xbox PDB): unless `004013e0` says otherwise, initializes
/// the scriptable form component (+0x78), the destructible object component (+0x94)
/// and the model texture swap component (+0x4c), each with the form, then the
/// `MagicItemObject` part.
pub fn ingredient_item_init_item(e: &mut Engine, this: Ptr) {
    if e.call(FORM_TEST_004013E0, &args![this]).bool() {
        return;
    }
    let base = this.addr();
    e.call(TES_SCRIPTABLE_FORM_INIT_ITEM, &args![base + 0x78, this]);
    e.call(
        BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM,
        &args![base + 0x94, this],
    );
    e.call(TES_MODEL_TEXTURE_SWAP_INIT_ITEM, &args![base + 0x4c, this]);
    e.call(MAGIC_ITEM_OBJECT_INIT_ITEM, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040_9d40, ingredient_item_init_item(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_item_initializes_the_components_unless_told_not_to() {
        for (skip, expected) in [(1u32, 0usize), (0, 4)] {
            let mut e = Engine::new();
            e.register_double(FORM_TEST_004013E0, move |_, _| Ret {
                eax: skip,
                ..Ret::default()
            });
            for callee in [
                TES_SCRIPTABLE_FORM_INIT_ITEM,
                BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM,
                TES_MODEL_TEXTURE_SWAP_INIT_ITEM,
                MAGIC_ITEM_OBJECT_INIT_ITEM,
            ] {
                e.register(callee, |_, _| Ret::default());
            }
            e.call_log = Some(vec![]);
            e.call(0x0040_9d40, &args![0x5000u32]);
            let log: Vec<(u32, Vec<u32>)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 != 0x0040_9d40 && c.0 != FORM_TEST_004013E0)
                .collect();
            assert_eq!(log.len(), expected);
            if expected != 0 {
                assert_eq!(
                    log,
                    vec![
                        (TES_SCRIPTABLE_FORM_INIT_ITEM, vec![0x5078, 0x5000]),
                        (BGS_DESTRUCTIBLE_OBJECT_FORM_INIT_ITEM, vec![0x5094, 0x5000]),
                        (TES_MODEL_TEXTURE_SWAP_INIT_ITEM, vec![0x504c, 0x5000]),
                        (MAGIC_ITEM_OBJECT_INIT_ITEM, vec![0x5000]),
                    ]
                );
            }
        }
    }
}
