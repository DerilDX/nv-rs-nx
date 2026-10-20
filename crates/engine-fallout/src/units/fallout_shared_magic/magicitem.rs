//! `fallout shared/magic/magicitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ModelLoader` singleton (pointer global).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `MagicItem::GetAVEffect` as the decompiler names `0040a300` (`thiscall`, `RET 4`).
const MAGIC_ITEM_GET_AV_EFFECT: u32 = 0x0040_a300;
/// `0040b470` (unnamed in the engine map): `thiscall`, whether the item is loaded, in `AL`.
const MAGIC_ITEM_IS_LOADED: u32 = 0x0040_b470;
/// `0040b6b0` (unnamed in the engine map): `thiscall`, whether the item is loading, in `AL`.
const MAGIC_ITEM_IS_LOADING: u32 = 0x0040_b6b0;
/// `00444f10` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 0xc`.
const MODEL_LOADER_QUEUE_ITEM: u32 = 0x0044_4f10;
/// `0048cee0` (unnamed in the engine map): `thiscall`; the length of the model path of a
/// `TESModel` (zero when it has none).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `ModelLoader::LoadFile` (`00447080`, `thiscall`, `RET 0x18`): the model path then
/// `0, 1, 0, 0, 0`.
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// `004088c0` (unnamed in the engine map): `thiscall`, adjusts a counter at `+0xa8`.
const FORM_COUNTER_A8: u32 = 0x0040_88c0;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, whether the node is empty
/// (no item and no next).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: returns `this` (a node's address is the address of its item pointer).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// `00726070`: the next-node pointer at `this + 4`.
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `00825c00` (unnamed in the engine map): the dword at `this + 0x14` (the effect setting of an
/// effect item).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// `004064b0` (unnamed in the engine map): `thiscall`; whether the setting qualifies (the
/// result of `00403c80` on a field of the setting).
const EFFECT_SETTING_QUALIFIES: u32 = 0x0040_64b0;
/// `005f5f80` (unnamed in the engine map): the dword at `this + 0x60` (the setting's associated
/// form).
const EFFECT_SETTING_ASSOCIATED_FORM: u32 = 0x005f_5f80;
/// `00403e60` (unnamed in the engine map): `thiscall`, `RET 4`; whether the flag mask is set on
/// the form.
const FORM_HAS_FLAG: u32 = 0x0040_3e60;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five stack words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `RTTI` type descriptor the associated form is cast from.
const RTTI_SOURCE_TYPE: u32 = 0x0118_3028;
/// The cast target for the `0x10000` flag.
const RTTI_CAST_FOR_10000: u32 = 0x0118_3998;
/// The cast target for the `0x20000` flag.
const RTTI_CAST_FOR_20000: u32 = 0x0118_3a34;
/// The first cast target for the `0x40000` flag.
const RTTI_CAST_FOR_40000_FIRST: u32 = 0x0118_3a1c;
/// The second cast target for the `0x40000` flag.
const RTTI_CAST_FOR_40000_SECOND: u32 = 0x0118_3a00;
/// `004811e0` (unnamed in the engine map): `thiscall`, `RET 4`; the model path of one of the two
/// biped models (index 0 or 1), through the model's virtual slot +0x14.
const BIPED_MODEL_BY_INDEX: u32 = 0x0048_11e0;
/// `0060af90` (unnamed in the engine map): `thiscall`, `RET 8`; the decompiler calls it
/// `TESNPC::BuildDefaultModelList(1, 1)`. Builds a list of model paths.
const BUILD_DEFAULT_MODEL_LIST: u32 = 0x0060_af90;
/// `00470470` (unnamed in the engine map): `thiscall`; empties a model-path list.
const FILENAME_LIST_CLEAR: u32 = 0x0047_0470;
/// `004702f0` (unnamed in the engine map): `thiscall`, `RET 4`; destroys a model-path list (frees
/// it when the argument's bit 0 is set).
const FILENAME_LIST_DESTROY: u32 = 0x0047_02f0;
/// `00717e50`: `this + 4`.
const MEMBER_LIST_AT_4: u32 = 0x0071_7e50;
/// `00406d30` (unnamed in the engine map): `cdecl`, (destination, size, source): copies a string.
const STRING_COPY: u32 = 0x0040_6d30;
/// `00406d50` (unnamed in the engine map): `cdecl`, (destination, size, source): appends a
/// string.
const STRING_APPEND: u32 = 0x0040_6d50;
/// The CRT `strrchr` (`00ec6e30`, `cdecl`): the last occurrence of a character.
const CRT_STRRCHR: u32 = 0x00ec_6e30;
/// `_strnicmp` (`00ec7ec0`, `cdecl`).
const CRT_STRNICMP: u32 = 0x00ec_7ec0;
/// `00447330` (unnamed in the engine map): `thiscall` on the `ModelLoader`; the decompiler calls
/// it `ModelLoader::BuildKFFileList(path, 0, 0, 0)`, which makes a list of animation files.
const MODEL_LOADER_BUILD_KF_LIST: u32 = 0x0044_7330;
/// `004471c0` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 4`; loads one
/// animation file by path.
const MODEL_LOADER_LOAD_KF: u32 = 0x0044_71c0;
/// `00401030` (unnamed in the engine map): `cdecl`, frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `0047fd90` (unnamed in the engine map): `thiscall`; whether a `TESAnimation` has animation
/// files, in `AL`.
const ANIMATION_HAS_KF_FILES: u32 = 0x0047_fd90;
/// `00408880` (unnamed in the engine map): `thiscall`; whether the counter at `+0xac` is above
/// zero.
const SETTING_COUNTER_POSITIVE: u32 = 0x0040_8880;
/// `004088a0` (unnamed in the engine map): `thiscall`; whether the counter at `+0xac` is below
/// zero.
const SETTING_COUNTER_NEGATIVE: u32 = 0x0040_88a0;
/// `0040ab50` (unnamed in the engine map): no arguments; adds one to the global at `011c3438`.
const COUNT_PRELOADED: u32 = 0x0040_ab50;
/// `004086e0` (unnamed in the engine map): `thiscall`; adjusts the counter at `+0xac`.
const SETTING_COUNTER_ADJUST: u32 = 0x0040_86e0;
/// The `TESModel` virtual slot (byte offset) that answers the model path.
const SLOT_MODEL_PATH: u32 = 0x14;
/// `"Skeleton"`.
const SKELETON_TEXT: u32 = 0x0101_3458;
/// `"SpecialAnims"`.
const SPECIAL_ANIMS_TEXT: u32 = 0x0101_3448;
/// The text appended after `"SpecialAnims"`.
const SPECIAL_ANIMS_SEPARATOR_TEXT: u32 = 0x0101_3444;
/// Size of the path buffer the function keeps on its stack.
const PATH_BUFFER_SIZE: u32 = 0x104;

/// `ModelLoader::LoadFile(path, 0, 1, 0, 0, 0)` on the singleton.
fn load_model_file(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(
        MODEL_LOADER_LOAD_FILE,
        &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
    );
}

/// `ModelLoader::LoadKF(path)` on the singleton.
fn load_animation_file(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(MODEL_LOADER_LOAD_KF, &args![loader, path]);
}

/// `__RTDynamicCast(form, 0, <source type>, target, 0)`.
fn dynamic_cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_SOURCE_TYPE, target, 0u32],
    )
    .u32()
}

/// Loads the model path of a `TESModel` when it has one (`0048cee0` non-zero): the path comes
/// from the model's virtual slot +0x14.
fn load_model_with_path(e: &mut Engine, model: u32) {
    if e.call(MODEL_PATH_LENGTH, &args![model]).u32() != 0 {
        let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
        load_model_file(e, path);
    }
}

/// The part of the loop body that depends on the setting's associated form: three arms chosen by
/// the flag masks `0x10000`, `0x20000` and `0x40000` of the setting (each tested only when there
/// is an associated form, in this order).
fn preload_associated_form(e: &mut Engine, setting: u32, associated: u32) {
    if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0001_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_10000);
        if form != 0 {
            load_model_with_path(e, form.wrapping_add(0x3c));
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0002_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_20000);
        if form != 0 {
            let biped = form.wrapping_add(0x70);
            let first = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 0u32]).u32();
            load_model_file(e, first);
            let second = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 1u32]).u32();
            load_model_file(e, second);
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_FIRST);
        if form != 0 {
            let list = e
                .call(BUILD_DEFAULT_MODEL_LIST, &args![form, 1u32, 1u32])
                .u32();
            let mut node = list;
            while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let path = e.mem.u32(slot);
                load_model_file(e, path);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
            e.call(FILENAME_LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(FILENAME_LIST_DESTROY, &args![list, 1u32]);
            }
        }
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_SECOND);
        if form != 0 {
            preload_model_with_animations(e, form);
        }
    }
}

/// The last arm: the form (cast to the class of `01183a00`) has a model at `+0xdc` and a
/// `TESAnimation` at `+0xc4`. The model's path is loaded (without the length test), then the
/// files named by the list at `+0x114` next to the model's folder, the `Skeleton` animation
/// files when the model's file name starts with `Skeleton`, and the `SpecialAnims` files the
/// animation lists. The path buffer is a 0x104-byte stack array in the exe.
fn preload_model_with_animations(e: &mut Engine, form: u32) {
    let model = form.wrapping_add(0xdc);
    let path_buffer = e.mem.alloc(PATH_BUFFER_SIZE);
    let first_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
    load_model_file(e, first_path);
    let model_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
    let list = e
        .call(MEMBER_LIST_AT_4, &args![form.wrapping_add(0x114)])
        .u32();
    e.call(
        STRING_COPY,
        &args![path_buffer, PATH_BUFFER_SIZE, model_path],
    );
    let mut folder_end = fn_0040ab30(e, Ptr::new(path_buffer), 0x5c).addr();
    let mut node = list;
    while folder_end != 0 && node != 0 {
        e.mem.set_u8(folder_end.wrapping_add(1), 0);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let name = e.mem.u32(slot);
        if name != 0 {
            e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
            load_model_file(e, path_buffer);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let last_slash = e.call(CRT_STRRCHR, &args![model_path, 0x5cu32]).u32();
    if last_slash != 0
        && e.call(
            CRT_STRNICMP,
            &args![last_slash.wrapping_add(1), SKELETON_TEXT, 8u32],
        )
        .u32()
            == 0
    {
        let loader = e.global::<u32>(MODEL_LOADER);
        let files = e
            .call(
                MODEL_LOADER_BUILD_KF_LIST,
                &args![loader, model_path, 0u32, 0u32, 0u32],
            )
            .u32();
        let mut node = files;
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let file = e.mem.u32(slot);
            if file != 0 {
                load_animation_file(e, file);
            }
            e.call(FREE_BLOCK, &args![file]);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        e.call(FILENAME_LIST_CLEAR, &args![files]);
        if files != 0 {
            e.call(FILENAME_LIST_DESTROY, &args![files, 1u32]);
        }
    }
    let animation = form.wrapping_add(0xc4);
    if e.call(ANIMATION_HAS_KF_FILES, &args![animation]).bool() {
        e.mem.set_u8(folder_end.wrapping_add(1), 0);
        e.call(
            STRING_APPEND,
            &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_TEXT],
        );
        e.call(
            STRING_APPEND,
            &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_SEPARATOR_TEXT],
        );
        folder_end = fn_0040ab30(e, Ptr::new(path_buffer), 0x5c).addr();
        let mut node = e.call(MEMBER_LIST_AT_4, &args![animation]).u32();
        while node != 0 {
            e.mem.set_u8(folder_end.wrapping_add(1), 0);
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let name = e.mem.u32(slot);
            if name != 0 {
                e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
                load_animation_file(e, path_buffer);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
    e.mem.free(path_buffer);
}

// Translated from 0040ab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The CRT `strrchr` as a `cdecl` wrapper: passes its two stack words (the string and the
/// character) to `00ec6e30` and answers its result, a pointer to the last occurrence or null.
pub fn fn_0040ab30(e: &mut Engine, text: Ptr, character: u32) -> Ptr {
    e.call(CRT_STRRCHR, &args![text, character]).ptr()
}

// Translated from 0040a420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040a420 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// `MagicItem::Preload` (Xbox PDB): `thiscall`, `RET 4` (the stack word is `force`, a byte).
/// Finds the item's first effect setting (`0040a300` with zero). When `force` is non-zero or the
/// item is loaded (`0040b470`), it is ready and counts as loaded; when it is only loading
/// (`0040b6b0`) it is ready but not loaded; otherwise the `ModelLoader` is only told about the
/// item (`00444f10`) and nothing else happens. A ready item with an effect loads the effect's
/// model (if the item counts as loaded and the model has a path) and counts the effect
/// (`004088c0`). Then each effect item of the list at `this + 0x10` (the walked pointer is 4
/// before the node) is looked at: when its setting qualifies (`004064b0`) and the item counts as
/// loaded, the setting's associated form is cast by the setting's flag masks and its models and
/// animation files are loaded ([`preload_associated_form`]); then, unless the setting's counter
/// at `+0xac` is positive or negative, a setting with the `0x40000` flag is counted globally
/// (`0040ab50`), and `004086e0` adjusts the counter.
pub fn magic_item_preload(e: &mut Engine, this: Ptr, force: u8) {
    let effect = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![this, 0u32]).u32();
    let (ready, loaded) = if force != 0 || e.call(MAGIC_ITEM_IS_LOADED, &args![this]).bool() {
        (true, true)
    } else if e.call(MAGIC_ITEM_IS_LOADING, &args![this]).bool() {
        (true, false)
    } else {
        (false, false)
    };
    if !ready {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(MODEL_LOADER_QUEUE_ITEM, &args![loader, this, effect, 0u32]);
        return;
    }
    if effect != 0 {
        if loaded {
            load_model_with_path(e, effect.wrapping_add(0x18));
        }
        e.call(FORM_COUNTER_A8, &args![effect]);
    }
    let mut walked = if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(0xc)
    };
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return;
    }
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let effect_item = e.mem.u32(slot);
        let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
        if e.call(EFFECT_SETTING_QUALIFIES, &args![setting]).bool() {
            if loaded {
                let associated = e
                    .call(EFFECT_SETTING_ASSOCIATED_FORM, &args![setting])
                    .u32();
                preload_associated_form(e, setting, associated);
            }
            if !e.call(SETTING_COUNTER_POSITIVE, &args![setting]).bool()
                && !e.call(SETTING_COUNTER_NEGATIVE, &args![setting]).bool()
                && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
                    .bool()
            {
                e.call(COUNT_PRELOADED, &args![]);
            }
            e.call(SETTING_COUNTER_ADJUST, &args![setting]);
        }
        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
        walked = if next != 0 { next.wrapping_sub(4) } else { 0 };
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_a420, magic_item_preload(Ptr, u8)),
        entry!(0x0040_ab30, fn_0040ab30(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const LOADER: u32 = 0x00c0_ffee;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text.as_bytes());
        address
    }

    /// A `TESModel`-shaped object at `address`: vtable (slot +0x14 answers the path), the path
    /// length at +4 and the path pointer at +8.
    fn put_model(e: &mut Engine, address: u32, path: &str) {
        let table = 0x0f00_8000;
        let mut slots = [0u32; 8];
        slots[SLOT_MODEL_PATH as usize / 4] = 0x0f00_8100;
        e.put_vtable(table, &slots);
        e.mem.set_u32(address, table);
        let pointer = if path.is_empty() {
            0
        } else {
            put_text(e, path)
        };
        e.mem.set_u32(address + 4, path.len() as u32);
        e.mem.set_u32(address + 8, pointer);
    }

    /// An engine with every callee of the function doubled over simple objects, and the model
    /// loader's `LoadFile` and `LoadKF` recording the paths they are given.
    fn rig(effect: u32) -> (Engine, Log, Log) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, LOADER);
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        e.register(MAGIC_ITEM_IS_LOADED, |_, _| Ret::default());
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| Ret::default());
        e.register(MODEL_LOADER_QUEUE_ITEM, |_, _| Ret::default());
        e.register(MODEL_PATH_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0f00_8100, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FORM_COUNTER_A8, |_, _| Ret::default());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_ITEM_SETTING, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(EFFECT_SETTING_QUALIFIES, |_, a| {
            u32::from(a[0] != 0).into_ret()
        });
        e.register(EFFECT_SETTING_ASSOCIATED_FORM, |e, a| {
            e.mem.u32(a[0] + 0x60).into_ret()
        });
        e.register(FORM_HAS_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & a[1] != 0).into_ret()
        });
        e.register(SETTING_COUNTER_POSITIVE, |_, _| Ret::default());
        e.register(SETTING_COUNTER_NEGATIVE, |_, _| Ret::default());
        e.register(COUNT_PRELOADED, |_, _| Ret::default());
        e.register(SETTING_COUNTER_ADJUST, |_, _| Ret::default());
        let loaded: Log = Rc::new(RefCell::new(Vec::new()));
        let animations: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = loaded.clone();
        e.register_double(MODEL_LOADER_LOAD_FILE, move |e, a| {
            assert_eq!(a[0], LOADER);
            assert_eq!(a[2..], [0, 1, 0, 0, 0]);
            sink.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        let sink = animations.clone();
        e.register_double(MODEL_LOADER_LOAD_KF, move |e, a| {
            assert_eq!(a[0], LOADER);
            sink.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        (e, loaded, animations)
    }

    /// A magic item with one effect item whose setting has the given flags (at +8) and
    /// associated form (at +0x60); the list head is at `this + 0x10`.
    fn item_with_setting(e: &mut Engine, flags: u32, associated: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        let effect_item = e.mem.alloc(0x20);
        let setting = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x10, effect_item);
        e.mem.set_u32(this + 0x14, 0);
        e.mem.set_u32(effect_item + 0x14, setting);
        e.mem.set_u32(setting + 8, flags);
        e.mem.set_u32(setting + 0x60, associated);
        this
    }

    #[test]
    fn at_0040a420_only_tells_the_loader_when_the_item_is_not_ready() {
        let (mut e, loaded, _) = rig(0x9000);
        let this = item_with_setting(&mut e, 0, 0);
        e.call(0x0040_a420, &args![this, 0u8]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_ITEM),
            vec![vec![LOADER, this, 0x9000, 0]]
        );
        assert_eq!(calls(&e, MAGIC_ITEM_GET_AV_EFFECT), vec![vec![this, 0]]);
        assert_eq!(calls(&e, MAGIC_ITEM_IS_LOADED), vec![vec![this]]);
        assert_eq!(calls(&e, MAGIC_ITEM_IS_LOADING), vec![vec![this]]);
        assert!(calls(&e, FORM_COUNTER_A8).is_empty());
        assert!(calls(&e, LIST_NODE_IS_EMPTY).is_empty());
        assert!(loaded.borrow().is_empty());
    }

    #[test]
    fn at_0040a420_force_skips_the_state_checks_and_loads_the_effect_model() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "effect.nif");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        let this = e.mem.alloc(0x20);
        // An empty list head: nothing is walked.
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(calls(&e, MAGIC_ITEM_IS_LOADED).is_empty());
        assert!(calls(&e, MAGIC_ITEM_IS_LOADING).is_empty());
        assert_eq!(*loaded.borrow(), vec!["effect.nif".to_string()]);
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
        assert_eq!(calls(&e, LIST_NODE_IS_EMPTY), vec![vec![this + 0x10]]);
        assert!(calls(&e, MODEL_LOADER_QUEUE_ITEM).is_empty());
    }

    #[test]
    fn at_0040a420_a_model_without_a_path_is_not_loaded() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        let this = e.mem.alloc(0x20);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
    }

    #[test]
    fn at_0040a420_an_item_that_is_only_loading_counts_the_effect_without_loading() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "effect.nif");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| 1u32.into_ret());
        let this = item_with_setting(&mut e, 0x0004_0000, 0);
        e.call(0x0040_a420, &args![this, 0u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
        // The setting still goes through the counters: the 0x40000 flag counts it globally.
        let setting = e.mem.u32(e.mem.u32(this + 0x10) + 0x14);
        assert_eq!(calls(&e, SETTING_COUNTER_POSITIVE), vec![vec![setting]]);
        assert_eq!(calls(&e, SETTING_COUNTER_NEGATIVE), vec![vec![setting]]);
        assert_eq!(calls(&e, COUNT_PRELOADED).len(), 1);
        assert_eq!(calls(&e, SETTING_COUNTER_ADJUST), vec![vec![setting]]);
        // No associated form is looked at for an item that is not loaded.
        assert!(calls(&e, EFFECT_SETTING_ASSOCIATED_FORM).is_empty());
    }

    #[test]
    fn at_0040a420_counters_decide_whether_the_setting_is_counted() {
        for (positive, negative, counted) in [(0u32, 0u32, true), (1, 0, false), (0, 1, false)] {
            let (mut e, _, _) = rig(0);
            e.register_double(SETTING_COUNTER_POSITIVE, move |_, _| positive.into_ret());
            e.register_double(SETTING_COUNTER_NEGATIVE, move |_, _| negative.into_ret());
            let this = item_with_setting(&mut e, 0x0004_0000, 0);
            e.call(0x0040_a420, &args![this, 1u8]);
            assert_eq!(calls(&e, COUNT_PRELOADED).len(), usize::from(counted));
            assert_eq!(calls(&e, SETTING_COUNTER_ADJUST).len(), 1);
        }
        // Without the 0x40000 flag nothing is counted globally.
        let (mut e, _, _) = rig(0);
        let this = item_with_setting(&mut e, 0x0001_0000, 0);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(calls(&e, COUNT_PRELOADED).is_empty());
    }

    #[test]
    fn at_0040a420_a_setting_that_does_not_qualify_is_skipped() {
        let (mut e, loaded, _) = rig(0);
        e.register(EFFECT_SETTING_QUALIFIES, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, 0x1234);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert!(calls(&e, SETTING_COUNTER_ADJUST).is_empty());
        assert!(calls(&e, EFFECT_SETTING_ASSOCIATED_FORM).is_empty());
    }

    #[test]
    fn at_0040a420_walks_the_whole_list_and_checks_it_is_not_empty_once() {
        let (mut e, _, _) = rig(0);
        let this = item_with_setting(&mut e, 0, 0);
        // A second effect item hangs off the first node: its `next` points 4 past the next
        // container (the list head sits at +4 of the walked pointer).
        let second_container = e.mem.alloc(0x20);
        let second_item = e.mem.alloc(0x20);
        let second_setting = e.mem.alloc(0x80);
        e.mem.set_u32(second_container + 4, second_item);
        e.mem.set_u32(second_item + 0x14, second_setting);
        e.mem.set_u32(this + 0x14, second_container + 4);
        e.call(0x0040_a420, &args![this, 1u8]);
        let first_setting = e.mem.u32(e.mem.u32(this + 0x10) + 0x14);
        assert_eq!(
            calls(&e, SETTING_COUNTER_ADJUST),
            vec![vec![first_setting], vec![second_setting]]
        );
        assert_eq!(calls(&e, LIST_NODE_IS_EMPTY).len(), 1);
    }

    #[test]
    fn at_0040a420_flag_0x10000_loads_the_model_of_the_cast_form() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        put_model(&mut e, form + 0x3c, "weapon.nif");
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[1..3], [0, RTTI_SOURCE_TYPE]);
            assert_eq!(a[4], 0);
            if a[3] == RTTI_CAST_FOR_10000 { a[0] } else { 0 }.into_ret()
        });
        let this = item_with_setting(&mut e, 0x0001_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(*loaded.borrow(), vec!["weapon.nif".to_string()]);
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 1);
        // The other masks are not tested once the first one matched.
        assert_eq!(calls(&e, FORM_HAS_FLAG).len(), 2);
    }

    #[test]
    fn at_0040a420_a_failed_cast_loads_nothing() {
        let (mut e, loaded, _) = rig(0);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0007_0000, 0x5555);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 1);
    }

    #[test]
    fn at_0040a420_flag_0x20000_loads_both_biped_models() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_20000 { a[0] } else { 0 }.into_ret()
        });
        let male = put_text(&mut e, "male.nif");
        let female = put_text(&mut e, "female.nif");
        e.register_double(BIPED_MODEL_BY_INDEX, move |_, a| {
            assert_eq!(a[0], form + 0x70);
            [male, female][a[1] as usize].into_ret()
        });
        let this = item_with_setting(&mut e, 0x0002_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["male.nif".to_string(), "female.nif".to_string()]
        );
    }

    #[test]
    fn at_0040a420_flag_0x40000_loads_the_default_models_and_frees_the_list() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_FIRST {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let first_path = put_text(&mut e, "head.nif");
        let second_path = put_text(&mut e, "hair.nif");
        e.mem.set_u32(first, first_path);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, second_path);
        e.register_double(BUILD_DEFAULT_MODEL_LIST, move |_, a| {
            assert_eq!(a[1..], [1, 1]);
            first.into_ret()
        });
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["head.nif".to_string(), "hair.nif".to_string()]
        );
        assert_eq!(calls(&e, BUILD_DEFAULT_MODEL_LIST), vec![vec![form, 1, 1]]);
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR), vec![vec![first]]);
        assert_eq!(calls(&e, FILENAME_LIST_DESTROY), vec![vec![first, 1]]);
        // The second cast is tried as well (and fails here).
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 2);
    }

    #[test]
    fn at_0040a420_flag_0x40000_without_a_list_still_clears_it() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_FIRST {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        e.register(BUILD_DEFAULT_MODEL_LIST, |_, _| Ret::default());
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR), vec![vec![0]]);
        assert!(calls(&e, FILENAME_LIST_DESTROY).is_empty());
    }

    /// A node list whose items are the given strings.
    fn string_list(e: &mut Engine, items: &[&str]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            let pointer = put_text(e, item);
            e.mem.set_u32(node, pointer);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// Everything the last arm calls: the string functions work on game memory, the lists are
    /// the given ones. Returns the form (model at +0xdc, extra files at +0x114, animations at
    /// +0xc4) and the log of the paths `FREE_BLOCK` was given.
    fn rig_last_arm(
        e: &mut Engine,
        model_path: &str,
        extra: &[&str],
        kf_files: &[&str],
        animations: &[&str],
        has_special_animations: bool,
    ) -> (u32, Log) {
        for (address, content) in [
            (SKELETON_TEXT, "Skeleton"),
            (SPECIAL_ANIMS_TEXT, "SpecialAnims"),
            (SPECIAL_ANIMS_SEPARATOR_TEXT, "\\"),
        ] {
            e.map(address, 16);
            e.mem.set_cstr(address, content.as_bytes());
        }
        e.register(STRING_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        e.register(CRT_STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| u32::from(*b) == a[1]) {
                Some(index) => (a[0] + index as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(CRT_STRNICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            let n = a[2] as usize;
            let left = &left[..left.len().min(n)];
            let right = &right[..right.len().min(n)];
            u32::from(left != right).into_ret()
        });
        let form = e.mem.alloc(0x140);
        put_model(e, form + 0xdc, model_path);
        let extra_list = string_list(e, extra);
        let animation_list = string_list(e, animations);
        e.register_double(MEMBER_LIST_AT_4, move |_, a| {
            if a[0] == form + 0x114 {
                extra_list.into_ret()
            } else {
                assert_eq!(a[0], form + 0xc4);
                animation_list.into_ret()
            }
        });
        let kf_list = string_list(e, kf_files);
        e.register_double(MODEL_LOADER_BUILD_KF_LIST, move |_, a| {
            assert_eq!(a[0], LOADER);
            assert_eq!(a[2..], [0, 0, 0]);
            kf_list.into_ret()
        });
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        e.register_double(ANIMATION_HAS_KF_FILES, move |_, a| {
            assert_eq!(a[0], form + 0xc4);
            u32::from(has_special_animations).into_ret()
        });
        let freed: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = freed.clone();
        e.register_double(FREE_BLOCK, move |e, a| {
            sink.borrow_mut().push(text(e, a[0]));
            Ret::default()
        });
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_SECOND {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        (form, freed)
    }

    #[test]
    fn at_0040a420_the_last_arm_loads_the_model_its_files_and_the_animations() {
        let (mut e, loaded, animations) = rig(0);
        let (form, freed) = rig_last_arm(
            &mut e,
            "actors\\skeleton\\Skeleton.nif",
            &["a.nif", "b.nif"],
            &["idle.kf", "walk.kf"],
            &["x.kf", "y.kf"],
            true,
        );
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec![
                "actors\\skeleton\\Skeleton.nif".to_string(),
                "actors\\skeleton\\a.nif".to_string(),
                "actors\\skeleton\\b.nif".to_string(),
            ]
        );
        assert_eq!(
            *animations.borrow(),
            vec![
                "idle.kf".to_string(),
                "walk.kf".to_string(),
                "actors\\skeleton\\SpecialAnims\\x.kf".to_string(),
                "actors\\skeleton\\SpecialAnims\\y.kf".to_string(),
            ]
        );
        assert_eq!(
            *freed.borrow(),
            vec!["idle.kf".to_string(), "walk.kf".to_string()]
        );
        // The skeleton file list is emptied and destroyed.
        let list = calls(&e, MODEL_LOADER_BUILD_KF_LIST).len();
        assert_eq!(list, 1);
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR).len(), 1);
        assert_eq!(calls(&e, FILENAME_LIST_DESTROY).len(), 1);
    }

    #[test]
    fn at_0040a420_the_last_arm_skips_what_does_not_apply() {
        let (mut e, loaded, animations) = rig(0);
        let (form, freed) = rig_last_arm(
            &mut e,
            "actors\\skeleton\\Other.nif",
            &[],
            &["idle.kf"],
            &["x.kf"],
            false,
        );
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["actors\\skeleton\\Other.nif".to_string()]
        );
        assert!(animations.borrow().is_empty());
        assert!(freed.borrow().is_empty());
        assert!(calls(&e, MODEL_LOADER_BUILD_KF_LIST).is_empty());
        assert_eq!(calls(&e, ANIMATION_HAS_KF_FILES).len(), 1);
    }
}
