//! `fallout/interface/menus/specialbookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The player (`PlayerCharacter *`, `011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 00794d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::GetTotalPointsUsed` (engine map; `thiscall`, the menu not read): the
/// sum (32-bit, wrapping) of the player's values 5 to 11 (the object at the player's
/// +0xa4, its slot +0x1c with each index, in order).
pub fn special_book_menu_get_total_points_used(e: &mut Engine, _this: Ptr) -> u32 {
    let mut total = 0u32;
    for index in 5u32..=11 {
        let player = e.mem.u32(PLAYER);
        let value = e
            .vcall(player.wrapping_add(0xa4), 0x1c, &args![index])
            .u32();
        total = total.wrapping_add(value);
    }
    total
}

/// `pSPECIALBookMenu` (Xbox PDB static of `SPECIALBookMenu`; the code reads the word at
/// `011daa38`, offsets matching the PDB's `pBookGeometry` +0x5C and `Pages` +0x60).
const SPECIAL_BOOK_MENU: u32 = 0x011d_aa38;
/// The `NiPointer` the close clears (`011daa9c`, through `0066b0d0`).
const POINTER_011DAA9C: u32 = 0x011d_aa9c;
/// The model loader (`011c3b3c`) and its release of a model by path (`0045a5e0`, thiscall,
/// one word).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_RELEASE: u32 = 0x0045_a5e0;
/// `"Meshes\Terminals\Babybook02.NIF"`.
const BOOK_MODEL_PATH: u32 = 0x0107_689c;
/// `007c6110` (no arguments): answers the menu class (0x424).
const MENU_CLASS: u32 = 0x007c_6110;
/// `Tile::GetMenuByClass` (engine map, `00a09030`; cdecl, one word).
const GET_MENU_BY_CLASS: u32 = 0x00a0_9030;
/// `00450b80` (cdecl, one word): the word at `011f91c8 + 4 * index`.
const OBJECT_BY_INDEX: u32 = 0x0045_0b80;
/// `004def90` and `00b5d180` (thiscall, no stack words) on the object `00450b80(1)` answers.
const OBJECT_CLEAR_004DEF90: u32 = 0x004d_ef90;
const OBJECT_RELEASE_00B5D180: u32 = 0x00b5_d180;
/// `NiPointer` assignment (`0066b0d0`, thiscall, one word).
const SMART_POINTER_SET: u32 = 0x0066_b0d0;
/// The page array's clear (`008454f0`, thiscall, one word: whether to free), for the
/// `BSSimpleArray`s `Buttons` (+0x00) and `TextItems` (+0x10) of each 0x20-byte
/// `SpecialBookPage` (Xbox PDB).
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// The trait setter from an integer (`00700320`, thiscall on a tile: trait, value; it hands
/// the value converted to a `float` to `00a012d0`).
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
/// `Menu::StartFadeOut` (engine map, `00a1d910`; thiscall).
const MENU_START_FADE_OUT: u32 = 0x00a1_d910;

// Translated from 007c6120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SPECIALBookMenu::Close` (engine map; no arguments). When the menu of class `007c6110()` is
/// open (`Tile::GetMenuByClass`): releases the book model (`0045a5e0` on the model loader
/// with `"Meshes\Terminals\Babybook02.NIF"`), runs `004def90` and `00b5d180` on
/// `00450b80(1)` (fetched once for each), clears `pBookGeometry` (+0x5C of the menu at
/// `011daa38`) and the pointer at `011daa9c`, frees both arrays of each of the nine pages
/// (`008454f0` with 1 on +0x60 + 0x20 * page and +0x70 + 0x20 * page); then, when the menu at
/// `011daa38` is still set, trait 0x1772 of the open menu set to 1 (`00700320`) and
/// `Menu::StartFadeOut` on `011daa38`'s menu. The global is read again at each use.
pub fn special_book_menu_close(e: &mut Engine) {
    let class = e.call(MENU_CLASS, &args![]).u32();
    let menu = e.call(GET_MENU_BY_CLASS, &args![class]).u32();
    if menu == 0 {
        return;
    }
    let loader: u32 = e.global(MODEL_LOADER);
    e.call(MODEL_LOADER_RELEASE, &args![loader, BOOK_MODEL_PATH]);
    let object = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.call(OBJECT_CLEAR_004DEF90, &args![object]);
    let object = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
    e.call(OBJECT_RELEASE_00B5D180, &args![object]);
    let book: u32 = e.global(SPECIAL_BOOK_MENU);
    e.mem.set_u32(book.wrapping_add(0x5c), 0);
    e.call(SMART_POINTER_SET, &args![POINTER_011DAA9C, 0u32]);
    let mut page = 0i32;
    while page < 9 {
        let offset = (page as u32) << 5;
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(
            ARRAY_CLEAR,
            &args![book.wrapping_add(offset).wrapping_add(0x60), 1u32],
        );
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(
            ARRAY_CLEAR,
            &args![book.wrapping_add(offset).wrapping_add(0x70), 1u32],
        );
        page += 1;
    }
    if e.global::<u32>(SPECIAL_BOOK_MENU) != 0 {
        e.call(SET_TRAIT_FROM_INT, &args![menu, 0x1772u32, 1u32]);
        let book: u32 = e.global(SPECIAL_BOOK_MENU);
        e.call(MENU_START_FADE_OUT, &args![book]);
    }
}

/// `00559450` (`thiscall`, no stack words): the word at `this` (an `NiPointer`'s object).
const POINTER_GET: u32 = 0x0055_9450;
/// `0043c4b0` (no arguments: the word at `011f4748`) and `007148c0` (`thiscall` on it, two
/// words: when `"ClearBuffer"` is enabled, its vtable +0x188 with them).
const RENDERER_OBJECT: u32 = 0x0043_c4b0;
const CLEAR_BUFFER: u32 = 0x0071_48c0;
/// `0045c670` (no arguments: a global pointer's object) and `008d80e0` (`thiscall`: the word
/// at +0xB4; the map's `DialoguePackage::GetTargetOfConversation`, a shared body): the
/// culling process the render uses.
const SCENE_OBJECT: u32 = 0x0045_c670;
const WORD_AT_B4: u32 = 0x008d_80e0;
/// `00c4f270` (`thiscall`, one word: pushes the culling process's mode, +0x90, and sets the
/// new one) and `00c4f2d0` (`thiscall`: pops it).
const PUSH_CULL_MODE: u32 = 0x00c4_f270;
const POP_CULL_MODE: u32 = 0x00c4_f2d0;
/// `00709d50` (`cdecl`, two words: a camera and a `float`) and its `float` (0.15,
/// `01030ff4`).
const CAMERA_SETUP_00709D50: u32 = 0x0070_9d50;
const CAMERA_VALUE_01030FF4: u32 = 0x0103_0ff4;
/// `004b7210` (no arguments: the interface manager) and `007fd640` (`thiscall` on it: an
/// object's address through `00559450`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const INTERFACE_OBJECT_007FD640: u32 = 0x007f_d640;
/// `00633c90` (`thiscall` on an `NiPointer`, one word: stores the object and adds a
/// reference) and `0045cec0` (the `NiPointer` destructor: releases the object).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `004a0fd0` (`thiscall` on the culling process, one word: through `0066b0d0`).
const CULLER_SET_004A0FD0: u32 = 0x004a_0fd0;
/// `BSShaderUtil::AccumulateScene` (engine map, `00b6bee0`; `cdecl`, three words) and
/// `00b6c0d0` (`cdecl`, three words: camera, accumulator, 0).
const ACCUMULATE_SCENE: u32 = 0x00b6_bee0;
const RENDER_00B6C0D0: u32 = 0x00b6_c0d0;
/// The `NiPointer` the render reads (`011daa9c`, the one `Close` clears).
const CAMERA_POINTER: u32 = POINTER_011DAA9C;

// Translated from 007c9ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c9ca0 - the compiler's exception frame (FS:[0] chain, handler 00f0c4d8, state 0 that releases the stack `NiPointer`) is not translated
/// Unnamed in the engine map (no arguments): renders the book when the menu (`011daa38`)
/// and its pointer at `011daa9c` are both set. Clears the buffer (`007148c0(0, 4)` on
/// `0043c4b0`'s object), takes the culling process (`008d80e0` of `0045c670`'s object),
/// pushes its mode 1, sets the camera up (`00709d50(camera, 0.15)`), holds the interface
/// object (`007fd640` of the interface manager) in a stack `NiPointer`, hands it to the
/// culling process (`004a0fd0`), accumulates the scene of `00450b80(1)`
/// (`BSShaderUtil::AccumulateScene(camera, object, culler)`), renders
/// (`00b6c0d0(camera, object, 0)`), clears the process's object (`004a0fd0(0)`), pops the
/// mode and releases the pointer.
pub fn fn_007c9ca0(e: &mut Engine) {
    if e.global::<u32>(SPECIAL_BOOK_MENU) == 0 {
        return;
    }
    if e.call(POINTER_GET, &args![CAMERA_POINTER]).u32() == 0 {
        return;
    }
    let renderer = e.call(RENDERER_OBJECT, &args![]).u32();
    e.call(CLEAR_BUFFER, &args![renderer, 0u32, 4u32]);
    let scene = e.call(SCENE_OBJECT, &args![]).u32();
    let culler = e.call(WORD_AT_B4, &args![scene]).u32();
    e.call(PUSH_CULL_MODE, &args![culler, 1u32]);
    let camera = e.call(POINTER_GET, &args![CAMERA_POINTER]).u32();
    let value = x87_quiet_bits(e.global::<u32>(CAMERA_VALUE_01030FF4));
    e.call(CAMERA_SETUP_00709D50, &args![camera, value]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    let object = e.call(INTERFACE_OBJECT_007FD640, &args![manager]).u32();
    e.with_stack(4, |e, held| {
        e.call(POINTER_CONSTRUCT, &args![held, object]);
        let object = e.call(POINTER_GET, &args![held]).u32();
        e.call(CULLER_SET_004A0FD0, &args![culler, object]);
        let accumulator = e.call(OBJECT_BY_INDEX, &args![1u32]).u32();
        e.call(ACCUMULATE_SCENE, &args![camera, accumulator, culler]);
        let object = e.call(POINTER_GET, &args![held]).u32();
        e.call(RENDER_00B6C0D0, &args![camera, object, 0u32]);
        e.call(CULLER_SET_004A0FD0, &args![culler, 0u32]);
        e.call(POP_CULL_MODE, &args![culler]);
        e.call(POINTER_DESTRUCT, &args![held]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007c_9ca0, fn_007c9ca0()),
        entry!(
            0x0079_4d80,
            special_book_menu_get_total_points_used(Ptr) -> u32
        ),
        entry!(0x007c_6120, special_book_menu_close()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_the_seven_values() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(0xa8);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, 0x701c);
        e.register(0x701c, |_, a| (a[1] * 10).into_ret());
        e.mem.set_u32(player + 0xa4, vtable);
        e.mem.set_u32(PLAYER, player);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0079_4d80, &[0]).u32(), 560);
        let log = e.call_log.take().unwrap();
        let indices: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x701c)
            .map(|(_, w)| {
                assert_eq!(w[0], player + 0xa4);
                w[1]
            })
            .collect();
        assert_eq!(indices, vec![5, 6, 7, 8, 9, 10, 11]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    fn close_engine(open: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_a000, 0x1000);
        e.register(MENU_CLASS, |_, _| 0x424u32.into_ret());
        e.register_double(GET_MENU_BY_CLASS, move |_, a| {
            assert_eq!(a[0], 0x424);
            open.into_ret()
        });
        e.register(OBJECT_BY_INDEX, |_, a| (0x7000 + a[0]).into_ret());
        for callee in [
            MODEL_LOADER_RELEASE,
            OBJECT_CLEAR_004DEF90,
            OBJECT_RELEASE_00B5D180,
            SMART_POINTER_SET,
            ARRAY_CLEAR,
            SET_TRAIT_FROM_INT,
            MENU_START_FADE_OUT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.set_global(MODEL_LOADER, 0x0c00_0000u32);
        e
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn close_releases_the_book_and_fades_out() {
        let mut e = close_engine(0x5000);
        let book = e.mem.alloc(0x1e4);
        e.mem.set_u32(book + 0x5c, 0x1234);
        e.set_global(SPECIAL_BOOK_MENU, book);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MODEL_LOADER_RELEASE),
            vec![vec![0x0c00_0000, BOOK_MODEL_PATH]]
        );
        assert_eq!(calls_to(&log, OBJECT_CLEAR_004DEF90), vec![vec![0x7001]]);
        assert_eq!(calls_to(&log, OBJECT_RELEASE_00B5D180), vec![vec![0x7001]]);
        assert_eq!(e.mem.u32(book + 0x5c), 0);
        assert_eq!(
            calls_to(&log, SMART_POINTER_SET),
            vec![vec![POINTER_011DAA9C, 0]]
        );
        let clears = calls_to(&log, ARRAY_CLEAR);
        let mut want = Vec::new();
        for page in 0..9u32 {
            want.push(vec![book + 0x60 + 0x20 * page, 1]);
            want.push(vec![book + 0x70 + 0x20 * page, 1]);
        }
        assert_eq!(clears, want);
        assert_eq!(
            calls_to(&log, SET_TRAIT_FROM_INT),
            vec![vec![0x5000, 0x1772, 1]]
        );
        assert_eq!(calls_to(&log, MENU_START_FADE_OUT), vec![vec![book]]);
    }

    #[test]
    fn close_without_the_open_menu_does_nothing() {
        let mut e = close_engine(0);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
        assert!(calls_to(&log, MODEL_LOADER_RELEASE).is_empty());
    }

    #[test]
    fn close_without_the_book_global_skips_the_fade() {
        // The book global is 0: the clears run on the low addresses, no fade.
        let mut e = close_engine(0x5000);
        e.map(0, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x007c_6120, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ARRAY_CLEAR).len(), 18);
        assert!(calls_to(&log, SET_TRAIT_FROM_INT).is_empty());
        assert!(calls_to(&log, MENU_START_FADE_OUT).is_empty());
    }
}

#[cfg(test)]
mod la1_render_tests {
    use super::*;

    fn render_engine(camera: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0103_0000, 0x1000);
        e.map(0x011d_a000, 0x1000);
        e.set_global(CAMERA_VALUE_01030FF4, 0.15f32);
        e.set_global(CAMERA_POINTER, camera);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(RENDERER_OBJECT, |_, _| 0x0c00_0000u32.into_ret());
        e.register(SCENE_OBJECT, |_, _| 0x0c10_0000u32.into_ret());
        e.register(WORD_AT_B4, |_, _| 0x0c20_0000u32.into_ret());
        e.register(INTERFACE_MANAGER, |_, _| 0x0c30_0000u32.into_ret());
        e.register(INTERFACE_OBJECT_007FD640, |_, _| 0x0c40_0000u32.into_ret());
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(OBJECT_BY_INDEX, |_, a| (0x7000 + a[0]).into_ret());
        for callee in [
            CLEAR_BUFFER,
            PUSH_CULL_MODE,
            POP_CULL_MODE,
            CAMERA_SETUP_00709D50,
            CULLER_SET_004A0FD0,
            ACCUMULATE_SCENE,
            RENDER_00B6C0D0,
            POINTER_DESTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn render_runs_the_scene_through_the_culler() {
        let camera = 0x0d00_0000;
        let mut e = render_engine(camera);
        e.set_global(SPECIAL_BOOK_MENU, 0x0e00_0000u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        let log = e.call_log.take().unwrap();
        let held = log.iter().find(|(a, _)| *a == POINTER_CONSTRUCT).unwrap().1[0];
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .skip(1)
            .filter(|(a, _)| *a != POINTER_GET)
            .collect();
        let culler = 0x0c20_0000;
        assert_eq!(
            calls,
            vec![
                (RENDERER_OBJECT, vec![]),
                (CLEAR_BUFFER, vec![0x0c00_0000, 0, 4]),
                (SCENE_OBJECT, vec![]),
                (WORD_AT_B4, vec![0x0c10_0000]),
                (PUSH_CULL_MODE, vec![culler, 1]),
                (CAMERA_SETUP_00709D50, vec![camera, 0.15f32.to_bits()]),
                (INTERFACE_MANAGER, vec![]),
                (INTERFACE_OBJECT_007FD640, vec![0x0c30_0000]),
                (POINTER_CONSTRUCT, vec![held, 0x0c40_0000]),
                (CULLER_SET_004A0FD0, vec![culler, 0x0c40_0000]),
                (OBJECT_BY_INDEX, vec![1]),
                (ACCUMULATE_SCENE, vec![camera, 0x7001, culler]),
                (RENDER_00B6C0D0, vec![camera, 0x0c40_0000, 0]),
                (CULLER_SET_004A0FD0, vec![culler, 0]),
                (POP_CULL_MODE, vec![culler]),
                (POINTER_DESTRUCT, vec![held]),
            ]
        );
    }

    #[test]
    fn render_needs_the_menu_and_the_camera() {
        let mut e = render_engine(0);
        e.set_global(SPECIAL_BOOK_MENU, 0x0e00_0000u32);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        let mut e = render_engine(0x0d00_0000);
        e.call_log = Some(vec![]);
        e.call(0x007c_9ca0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
