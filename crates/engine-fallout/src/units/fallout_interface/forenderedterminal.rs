//! `fallout/interface/forenderedterminal.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007ff8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007ff8e0`, which tests the flag `0x4000` of the
/// word at `+0x30` through `00456630`, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`) is from `00456630`'s disassembly (`AND EAX,[EBP+8]; NEG EAX;
/// SBB EAX,EAX; NEG EAX`: 1 when the masked word is non-zero, else 0), which `007ff8e0`
/// returns without change.
pub fn fn_007ff8c0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x007f_f8e0, &args![this]).bool()
}

/// `00456630` (`thiscall`, `RET 4`): whether the word at +0x30 has a bit of the mask.
const FLAG_TEST_00456630: u32 = 0x0045_6630;
/// `00450b80` (`cdecl`, one word, `ECX` unused): the word at `011f91c8 + 4 * index`.
const TES_00450B80: u32 = 0x0045_0b80;
/// The holder at `011db63c`: `00559450` (`thiscall`) returns its first word, `0066b0d0`
/// (`thiscall`, `RET 4`) is called on it with 0.
const HOLDER_011DB63C: u32 = 0x011d_b63c;
const HOLDER_GET_00559450: u32 = 0x0055_9450;
const HOLDER_SET_0066B0D0: u32 = 0x0066_b0d0;
/// `ModelLoader::ReleaseModel` (`00445300`, `thiscall` on the model loader at `011c3b3c`,
/// `RET 0xc`) and the model path pointer at `011a0bb0` (the terminal interface model).
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
const MODEL_LOADER_011C3B3C: u32 = 0x011c_3b3c;
const TERMINAL_MODEL_PATH_011A0BB0: u32 = 0x011a_0bb0;

// Translated from 007ff8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`): `00456630(this, 0x4000)`, its `EAX` returned
/// unchanged (0 or 1).
pub fn fn_007ff8e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAG_TEST_00456630, &args![this, 0x4000u32]).bool()
}

// Translated from 007ffad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, plain `RET`; `this` is not read): `00450b80(1)`, whose `EAX` it
/// leaves as its own return value.
pub fn fn_007ffad0(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(TES_00450B80, &args![1u32]).u32()
}

// Translated from 007ffe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::ReleaseStaticGeometry` (Xbox PDB name from the engine map; no
/// arguments): when the holder at `011db63c` holds something (`00559450`), releases the
/// terminal model (`ModelLoader::ReleaseModel(path, 1, 1)` with the path at `011a0bb0`),
/// then `0066b0d0(holder, 0)`.
pub fn fo_rendered_terminal_release_static_geometry(e: &mut Engine) {
    if e.call(HOLDER_GET_00559450, &args![HOLDER_011DB63C]).u32() != 0 {
        let path: u32 = e.global(TERMINAL_MODEL_PATH_011A0BB0);
        let loader: u32 = e.global(MODEL_LOADER_011C3B3C);
        e.call(MODEL_LOADER_RELEASE_MODEL, &args![loader, path, 1u32, 1u32]);
    }
    e.call(HOLDER_SET_0066B0D0, &args![HOLDER_011DB63C, 0u32]);
}

// ---------------------------------------------------------------------------
// `FORenderedTerminal` (batch n051)

layout! {
    /// `FORenderedTerminal` (Xbox PDB), 0xf0 bytes: `FORenderedMenu` (0xd8) and the
    /// fields below, at the offsets the PC code uses.
    pub struct FORenderedTerminal: 0xf0 {
        /// `OriginalPos` (Xbox PDB): `NiPoint3`.
        0xD8 OriginalPos: u32,
        /// `spExitButton` (Xbox PDB): `NiPointer<NiAVObject>`.
        0xE4 spExitButton: u32,
        /// `fLastFadePercent` (Xbox PDB).
        0xE8 fLastFadePercent: f32,
        /// `bFadingOut` (Xbox PDB).
        0xEC bFadingOut: u8,
    }
}

/// `FORenderedMenu`'s functions this unit calls (`forenderedmenu.cpp`): `OnIdle`
/// (`007faf10`), `OnMenuOpen` (`007fb040`), its close counterpart (`007fb0b0`), the
/// screen pick (`007fb790`, `RET 0xc`) and the render into the texture (`007fba00`,
/// `RET 0xc`).
const RENDERED_MENU_ON_IDLE: u32 = 0x007f_af10;
const RENDERED_MENU_ON_MENU_OPEN: u32 = 0x007f_b040;
const RENDERED_MENU_ON_MENU_CLOSE: u32 = 0x007f_b0b0;
const RENDERED_MENU_PICK: u32 = 0x007f_b790;
const RENDERED_MENU_RENDER: u32 = 0x007f_ba00;
/// The menu's `NiPointer` getters (`thiscall`): `spScreenParent` (+8, `005585e0`),
/// `spMenuRoot` (+0x14, `0063d040`) and `spLight` (+0x1c, `006838b0`).
const SCREEN_PARENT_005585E0: u32 = 0x0055_85e0;
const MENU_ROOT_0063D040: u32 = 0x0063_d040;
const MENU_LIGHT_006838B0: u32 = 0x0068_38b0;
/// `00450f90` (`thiscall`, `RET 4`): sets an object's app-culled flag.
const SET_APP_CULLED_00450F90: u32 = 0x0045_0f90;
/// The update data's initialisation (`0043d410`, `RET 0xc`: a `float` and two words)
/// and the object update (`00a59c60`, `RET 4`).
const UPDATE_DATA_INIT_0043D410: u32 = 0x0043_d410;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// `BSFadeNode::SetRange` (`00b4dfd0`, `RET 8`: two `float`s), `0054b800`
/// (`thiscall`) and `00476ae0` (`thiscall`, `RET 4`) on the terminal geometry.
const FADE_NODE_SET_RANGE: u32 = 0x00b4_dfd0;
const GEOMETRY_0054B800: u32 = 0x0054_b800;
const GEOMETRY_00476AE0: u32 = 0x0047_6ae0;
/// The terminal geometry's fade (`006d2c40`, `thiscall`, `ST0`).
const GEOMETRY_FADE_006D2C40: u32 = 0x006d_2c40;
/// The fade range of an opening terminal (`01017130`) and the menu root's x translation
/// (`01078bb0`).
const OPEN_FADE_RANGE: u32 = 0x0101_7130;
const ROOT_OFFSET_01078BB0: u32 = 0x0107_8bb0;
/// The object's translation from a point (`00440460`, `RET 4`) and from three `float`s
/// (`004bc1f0`, `RET 0xc`: the local translation at +0x58).
const SET_POSITION_00440460: u32 = 0x0044_0460;
const SET_TRANSLATE_004BC1F0: u32 = 0x004b_c1f0;
/// `InterfaceManager::EnterRenderedMenu` (`007181d0`, `RET 4`) on the interface manager
/// (`004b7210`).
const INTERFACE_MANAGER: u32 = 0x004b_7210;
const ENTER_RENDERED_MENU: u32 = 0x0071_81d0;
/// The setting `011f9148`: its value address (`0043d4d0`) and its setter (`0045ce80`,
/// `RET 4`).
const SETTING_011F9148: u32 = 0x011f_9148;
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_SET_0045CE80: u32 = 0x0045_ce80;
/// `009611e0` (`thiscall`) and `00984f60` (`thiscall`, `RET 4`) on the menu.
const MENU_OBJECT_009611E0: u32 = 0x0096_11e0;
const MENU_00984F60: u32 = 0x0098_4f60;
/// `BSAudioManager::QInstance` (`00ad9060`), `BSAudioManager::GetSoundHandleByName`
/// (`00ae5680`, `RET 0x10`), `BSSoundHandle::Play` (`00ad8830`, `RET 4`) and the
/// handle's destructor (`00483710`); `"OBJComputerTerminalShutdown..."` (`01078bc8`).
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
const AUDIO_MANAGER_SOUND_BY_NAME: u32 = 0x00ae_5680;
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCT: u32 = 0x0048_3710;
const SHUTDOWN_SOUND: u32 = 0x0107_8bc8;
/// The security cookie (`.data`).
const SECURITY_COOKIE: u32 = 0x011c_16bc;
/// The pick: the point initialisation (`006815c0`), the camera (`008b6200`, else
/// `006629f0` of `0045c670()`), `NiPick::NiPick` (`00e98f20`, `RET 8`), `00458b30`
/// (`RET 4`), the target (`00705fc0`, `RET 4`), `006ecd40` (`RET 4`), the cursor
/// (`00792760`, `009a9350`, `ST0`), `_ftol2` (`00ec62c0`), the camera's ray
/// (`00a71080`, `RET 0x14`), `NiPick::PickObjects` (`00e98e20`, `RET 0xc`), the results
/// (`00500940`), a record (`00968670`, `RET 4`), its object (`00458b50`) and
/// `NiPick::~NiPick` (`00e98fa0`).
const POINT_INIT_006815C0: u32 = 0x0068_15c0;
const CAMERA_008B6200: u32 = 0x008b_6200;
const GETTER_0045C670: u32 = 0x0045_c670;
const GETTER_006629F0: u32 = 0x0066_29f0;
const NI_PICK_CONSTRUCT: u32 = 0x00e9_8f20;
const PICK_00458B30: u32 = 0x0045_8b30;
const PICK_SET_TARGET_00705FC0: u32 = 0x0070_5fc0;
const PICK_006ECD40: u32 = 0x006e_cd40;
const CURSOR_00792760: u32 = 0x0079_2760;
const CURSOR_009A9350: u32 = 0x009a_9350;
const FTOL: u32 = 0x00ec_62c0;
const CAMERA_WINDOW_POINT_TO_RAY: u32 = 0x00a7_1080;
const NI_PICK_PICK_OBJECTS: u32 = 0x00e9_8e20;
const PICK_RESULTS_00500940: u32 = 0x0050_0940;
const PICK_RECORD_00968670: u32 = 0x0096_8670;
const RECORD_OBJECT_00458B50: u32 = 0x0045_8b50;
const NI_PICK_DESTROY: u32 = 0x00e9_8fa0;

/// Inits a local update data (`0043d410` with 0.0, 0, 0) and updates `object` with it
/// (`00a59c60`).
fn update_object(e: &mut Engine, object: u32) {
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![object, data]);
    });
}

/// The terminal geometry the static holder at `011db63c` holds (`00559450`).
fn terminal_geometry(e: &mut Engine) -> u32 {
    e.call(HOLDER_GET_00559450, &args![HOLDER_011DB63C]).u32()
}

// Translated from 007ff650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::OnMenuOpen` (Xbox PDB): attaches the screen parent to the
/// shadow scene node (`00450b80(1)`, its slot +0xdc with it and 1) and updates it, shows
/// the light (`00450f90` with 0) when there is one, runs
/// `FORenderedMenu::OnMenuOpen`, clears `bFadingOut` and `fLastFadePercent`, and with a
/// terminal geometry sets its fade range to the value at `01017130` twice
/// (`BSFadeNode::SetRange`), then `0054b800` and `00476ae0(0)` on it.
pub fn fo_rendered_terminal_on_menu_open(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    let again = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    e.vcall(scene, 0xdc, &args![again, 1u32]);
    update_object(e, root);
    if e.call(MENU_LIGHT_006838B0, &args![this]).u32() != 0 {
        let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![node, 0u32]);
    }
    e.call(RENDERED_MENU_ON_MENU_OPEN, &args![this]);
    e.set(this, FORenderedTerminal::bFadingOut, 0);
    e.mem.set_u32(this.addr() + 0xe8, x87_store32(0.0));
    if terminal_geometry(e) != 0 {
        let range = x87_quiet_bits(e.global::<u32>(OPEN_FADE_RANGE));
        let geometry = terminal_geometry(e);
        e.call(FADE_NODE_SET_RANGE, &args![geometry, range, range]);
        let geometry = terminal_geometry(e);
        e.call(GEOMETRY_0054B800, &args![geometry]);
        let geometry = terminal_geometry(e);
        e.call(GEOMETRY_00476AE0, &args![geometry, 0u32]);
    }
}

// Translated from 007ff740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the terminal's menu close): hides the light (`00450f90` with 1) and
/// updates it when there is one, detaches the screen parent from the shadow scene node
/// (`00450b80(1)`, its slot +0xe8) and runs `FORenderedMenu`'s close (`007fb0b0`).
pub fn fn_007ff740(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    if e.call(MENU_LIGHT_006838B0, &args![this]).u32() != 0 {
        let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
        e.call(SET_APP_CULLED_00450F90, &args![node, 1u32]);
        e.with_stack(0xc, |e, data| {
            e.call(
                UPDATE_DATA_INIT_0043D410,
                &args![data, x87_store32(0.0), 0u32, 0u32],
            );
            let node = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
            e.call(UPDATE_00A59C60, &args![node, data]);
        });
    }
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    let root = e.call(SCREEN_PARENT_005585E0, &args![this]).u32();
    e.vcall(scene, 0xe8, &args![root]);
    e.call(RENDERED_MENU_ON_MENU_CLOSE, &args![this]);
}

// Translated from 007ff7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 0xc`; only the first word is read): puts the menu root back at
/// `OriginalPos` and updates it (when there is one), then renders the menu into the
/// texture (`007fba00` with the word, 1 and 0).
pub fn fn_007ff7c0(
    e: &mut Engine,
    this: Ptr<FORenderedTerminal>,
    texture: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root != 0 {
        e.call(SET_POSITION_00440460, &args![root, this.addr() + 0xd8]);
        update_object(e, root);
    }
    e.call(RENDERED_MENU_RENDER, &args![this, texture, 1u32, 0u32]);
}

// Translated from 007ff820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::OnIdle` (Xbox PDB): while fading out, enters the rendered menu
/// (`InterfaceManager::EnterRenderedMenu(0)`) and answers true once the terminal geometry
/// is gone, no longer fading (`007ff8c0`) or its fade (`006d2c40`) equals the last one
/// (`FUCOMPP`); otherwise stores the new fade in `fLastFadePercent`. Then (and when not
/// fading out) answers `FORenderedMenu::OnIdle`.
pub fn fo_rendered_terminal_on_idle(e: &mut Engine, this: Ptr<FORenderedTerminal>) -> bool {
    if e.get(this, FORenderedTerminal::bFadingOut) != 0 {
        let done = terminal_geometry(e) == 0 || {
            let geometry = terminal_geometry(e);
            !fn_007ff8c0(e, Ptr::new(geometry)) || {
                let geometry = terminal_geometry(e);
                let fade = e.call(GEOMETRY_FADE_006D2C40, &args![geometry]).f64();
                // FUCOMPP; TEST AH,0x44; JP: the last fade unchanged (equal, not NaN).
                x87_load32(e.mem.u32(this.addr() + 0xe8)) == fade
            }
        };
        if done {
            let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
            e.call(ENTER_RENDERED_MENU, &args![manager, 0u32]);
            return true;
        }
        let geometry = terminal_geometry(e);
        let fade = e.call(GEOMETRY_FADE_006D2C40, &args![geometry]).f64();
        e.mem.set_u32(this.addr() + 0xe8, x87_store32(fade));
    }
    e.call(RENDERED_MENU_ON_IDLE, &args![this]).bool()
}

// Translated from 007ff900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`): with a menu root, moves it to (the `float` at `01078bb0`, 0.0,
/// 0.0) (`004bc1f0`) and updates it.
pub fn fn_007ff900(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root == 0 {
        return;
    }
    let offset = x87_quiet_bits(e.global::<u32>(ROOT_OFFSET_01078BB0));
    e.call(
        SET_TRANSLATE_004BC1F0,
        &args![root, offset, x87_store32(0.0), x87_store32(0.0)],
    );
    update_object(e, root);
}

/// `FORenderedMenu::CreateMenuLight` (`007fc400`, `RET 8`), `"Omni01"` (`01078210`)
/// and its failure message (`01078a68`, printed by `005b5e40`).
const CREATE_MENU_LIGHT: u32 = 0x007f_c400;
const LIGHT_NODE_NAME: u32 = 0x0107_8210;
const NO_LIGHT_TEXT: u32 = 0x0107_8a68;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// The point/color constructor from three `float`s (`00416870`, `RET 0xc`, returns
/// `this`) and the light's setters: `004bc280` (`RET 4`, a point), `0050c630`,
/// `0050c660`, `0050c690`, `0050dd20` (`RET 4`, a `float` each) and `0050dd50` (`RET 4`,
/// a point).
const POINT_FROM_FLOATS_00416870: u32 = 0x0041_6870;
const LIGHT_004BC280: u32 = 0x004b_c280;
const LIGHT_0050C630: u32 = 0x0050_c630;
const LIGHT_0050C660: u32 = 0x0050_c660;
const LIGHT_0050C690: u32 = 0x0050_c690;
const LIGHT_0050DD20: u32 = 0x0050_dd20;
const LIGHT_0050DD50: u32 = 0x0050_dd50;
/// A setting's value address (`00403e20`, `thiscall` on the setting) and the screen
/// light settings read here (`011db644`, `011db5f8`, `011db668`, `011db608`, `011db650`).
const SETTING_FLOAT_VALUE: u32 = 0x0040_3e20;
/// The menu's light color (`006b0570`, `RET 4`), `00965580` (`RET 4`, a `float`) and
/// `007f99b0` (`RET 4`, a `float`).
const MENU_LIGHT_COLOR_006B0570: u32 = 0x006b_0570;
const MENU_00965580: u32 = 0x0096_5580;
const MENU_007F99B0: u32 = 0x007f_99b0;

// Translated from 007ff960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::InitLight` (Xbox PDB): creates the menu light at `"Omni01"` in the
/// shadow scene node (`CreateMenuLight`; prints a failure message and stops when it
/// fails); with the light: a zero point (`004bc280`), 0.0 for `0050c630`, `0050c660` and
/// `0050c690`, 1.0 for `0050dd20`, the point (setting `011db644`, 0, 0) for `0050dd50`;
/// the color (settings `011db608`, `011db668`, `011db5f8`) for the menu (`006b0570`),
/// the setting `011db650` for `00965580`, 1.0 for `007f99b0`, then updates the light.
pub fn fo_rendered_terminal_init_light(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let scene = e.call(TES_00450B80, &args![1u32]).u32();
    if !e
        .call(CREATE_MENU_LIGHT, &args![this, LIGHT_NODE_NAME, scene])
        .bool()
    {
        e.call(PRINT_MESSAGE, &args![NO_LIGHT_TEXT]);
        return;
    }
    let light = e.call(MENU_LIGHT_006838B0, &args![this]).u32();
    if light == 0 {
        return;
    }
    let zero = x87_store32(0.0);
    let setting = |e: &mut Engine, address: u32| {
        let value = e.call(SETTING_FLOAT_VALUE, &args![address]).u32();
        x87_quiet_bits(e.mem.u32(value))
    };
    // The frame below `EBP`: points at `EBP - 0x28`, `- 0x34` and `- 0x1c`, the update
    // data at `EBP - 0x10`.
    e.with_stack(0x30, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 0xc;
        let color = frame.addr() + 0x18;
        let data = frame.addr() + 0x24;
        let point = e
            .call(POINT_FROM_FLOATS_00416870, &args![first, zero, zero, zero])
            .u32();
        e.call(LIGHT_004BC280, &args![light, point]);
        e.call(LIGHT_0050C630, &args![light, zero]);
        e.call(LIGHT_0050C660, &args![light, zero]);
        e.call(LIGHT_0050C690, &args![light, zero]);
        e.call(LIGHT_0050DD20, &args![light, x87_store32(1.0)]);
        let x = setting(e, 0x011d_b644);
        let point = e
            .call(POINT_FROM_FLOATS_00416870, &args![second, x, zero, zero])
            .u32();
        e.call(LIGHT_0050DD50, &args![light, point]);
        let blue = setting(e, 0x011d_b5f8);
        let green = setting(e, 0x011d_b668);
        let red = setting(e, 0x011d_b608);
        e.call(POINT_FROM_FLOATS_00416870, &args![color, red, green, blue]);
        e.call(MENU_LIGHT_COLOR_006B0570, &args![this, color]);
        let value = setting(e, 0x011d_b650);
        e.call(MENU_00965580, &args![this, value]);
        e.call(MENU_007F99B0, &args![this, x87_store32(1.0)]);
        e.call(
            UPDATE_DATA_INIT_0043D410,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_00A59C60, &args![light, data]);
    });
}

// Translated from 007ffaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FORenderedTerminal::FadeOut` (Xbox PDB): with a terminal geometry, sets its fade range
/// to 0.0 and 0.0 with the setting at `011f9148` cleared around it (`007ffb60(0)`, then
/// the old value back); sets `bFadingOut` and `fLastFadePercent` to 1.0. The 0 pushed for
/// `007ffb60` is left on the stack and becomes the range's first `float`.
pub fn fo_rendered_terminal_fade_out(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    if terminal_geometry(e) != 0 {
        let old = fn_007ffb80(e);
        fn_007ffb60(e, 0);
        let geometry = terminal_geometry(e);
        e.call(
            FADE_NODE_SET_RANGE,
            &args![geometry, x87_store32(0.0), x87_store32(0.0)],
        );
        fn_007ffb60(e, old);
    }
    e.set(this, FORenderedTerminal::bFadingOut, 1);
    e.mem.set_u32(this.addr() + 0xe8, x87_store32(1.0));
}

// Translated from 007ffb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one word): sets the setting at `011f9148` (`0045ce80`).
pub fn fn_007ffb60(e: &mut Engine, value: u32) {
    e.call(SETTING_SET_0045CE80, &args![SETTING_011F9148, value]);
}

// Translated from 007ffb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): the word value of the setting at `011f9148` (`0043d4d0`).
pub fn fn_007ffb80(e: &mut Engine) -> u32 {
    let value = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_011F9148])
        .u32();
    e.mem.u32(value)
}

// Translated from 007ffba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ffba0 - the C++ exception-unwinding frame (FS:[0] around the local NiPick) is not translated.
/// Unnamed (`thiscall`, `RET 0xc`: a `bool` and two words; the terminal's click): with
/// an exit button and the flag, casts a pick (`NiPick(0, 8)`, `00458b30(0)`, the exit
/// button as its target, `006ecd40(1)`) along the camera's ray through the cursor
/// (`00a71080` with the truncated `009a9350` and `00792760` of the interface manager);
/// when the first record's object is the exit button, closes the terminal (`007ffd50`)
/// and answers false. Otherwise answers `FORenderedMenu`'s screen pick (`007fb790`)
/// with the three words.
pub fn fn_007ffba0(
    e: &mut Engine,
    this: Ptr<FORenderedTerminal>,
    flag: u8,
    first: u32,
    second: u32,
) -> bool {
    let base = this.addr();
    let interface = e.call(INTERFACE_MANAGER, &args![]).u32();
    let has_button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32() != 0;
    if has_button && flag != 0 {
        // The frame below `EBP`: the points at `EBP - 0x58` and `- 0x64`, the pick at
        // `EBP - 0x4c` (to `EBP - 0x18`).
        let hit = e.with_stack(0x58, |e, frame| {
            let local = |offset: u32| frame.addr() + 0x68 - offset;
            let origin = local(0x58);
            let direction = local(0x64);
            let pick = local(0x4c);
            e.call(POINT_INIT_006815C0, &args![origin]);
            e.call(POINT_INIT_006815C0, &args![direction]);
            let mut camera = e.call(CAMERA_008B6200, &args![this]).u32();
            if camera == 0 {
                let object = e.call(GETTER_0045C670, &args![]).u32();
                camera = e.call(GETTER_006629F0, &args![object]).u32();
            }
            e.call(NI_PICK_CONSTRUCT, &args![pick, 0u32, 8u32]);
            e.call(PICK_00458B30, &args![pick, 0u32]);
            let button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32();
            e.call(PICK_SET_TARGET_00705FC0, &args![pick, button]);
            e.call(PICK_006ECD40, &args![pick, 1u32]);
            let x = e.call(CURSOR_00792760, &args![interface]).f64();
            let x = e.call(FTOL, &args![x]).u32();
            let y = e.call(CURSOR_009A9350, &args![interface]).f64();
            let y = e.call(FTOL, &args![y]).u32();
            e.call(
                CAMERA_WINDOW_POINT_TO_RAY,
                &args![camera, y, x, origin, direction, 0u32],
            );
            let mut hit = false;
            if e.call(NI_PICK_PICK_OBJECTS, &args![pick, origin, direction, 0u32])
                .bool()
            {
                let results = e.call(PICK_RESULTS_00500940, &args![pick]).u32();
                let record = e.call(PICK_RECORD_00968670, &args![results, 0u32]).u32();
                if record != 0 {
                    let object = e.call(RECORD_OBJECT_00458B50, &args![record]).u32();
                    let button = e.call(HOLDER_GET_00559450, &args![base + 0xe4]).u32();
                    if object == button {
                        fn_007ffd50(e, this);
                        hit = true;
                    }
                }
            }
            e.call(NI_PICK_DESTROY, &args![pick]);
            hit
        });
        if hit {
            return false;
        }
    }
    e.call(
        RENDERED_MENU_PICK,
        &args![this, u32::from(flag), first, second],
    )
    .bool()
}

// Translated from 007ffd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the terminal's close): plays the shutdown sound (`PlayShutdownSound`),
/// deletes the menu's object (`009611e0`, slot 0 with 1) when there is one, then
/// `00984f60(0)` and `InterfaceManager::EnterRenderedMenu(0)`.
pub fn fn_007ffd50(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    fo_rendered_terminal_play_shutdown_sound(e);
    let object = e.call(MENU_OBJECT_009611E0, &args![this]).u32();
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.call(MENU_00984F60, &args![this, 0u32]);
    let manager = e.call(INTERFACE_MANAGER, &args![]).u32();
    e.call(ENTER_RENDERED_MENU, &args![manager, 0u32]);
}

// Translated from 007ffdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: with a menu root, puts it at `OriginalPos` and updates it.
pub fn fn_007ffdb0(e: &mut Engine, this: Ptr<FORenderedTerminal>) {
    let root = e.call(MENU_ROOT_0063D040, &args![this]).u32();
    if root == 0 {
        return;
    }
    e.call(SET_POSITION_00440460, &args![root, this.addr() + 0xd8]);
    update_object(e, root);
}

// Translated from 007ffe40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007ffe40 - the C++ exception-unwinding frame (FS:[0] around the sound handle) is not translated.
/// `FORenderedTerminal::PlayShutdownSound` (Xbox PDB, no arguments): a sound handle by
/// name (`BSAudioManager::GetSoundHandleByName(QInstance(), handle,
/// "OBJComputerTerminalShutdown...", 0x121, 0)`), played and destroyed.
/// `BSSoundHandle::Play` takes one word (`RET 4`) but none is pushed: it reads the word
/// at the caller's stack top, the frame's saved security cookie (`011c16bc ^ EBP`, at
/// `EBP - 0x24`).
pub fn fo_rendered_terminal_play_shutdown_sound(e: &mut Engine) {
    // The frame below `EBP`: the handle at `EBP - 0x18`, the saved cookie at `EBP - 0x24`.
    e.with_stack(0x24, |e, frame| {
        let ebp = frame.addr() + 0x24;
        let cookie = e.global::<u32>(SECURITY_COOKIE) ^ ebp;
        e.mem.set_u32(ebp - 0x24, cookie);
        let handle = ebp - 0x18;
        let manager = e.call(AUDIO_MANAGER_INSTANCE, &args![]).u32();
        let sound = e
            .call(
                AUDIO_MANAGER_SOUND_BY_NAME,
                &args![manager, handle, SHUTDOWN_SOUND, 0x121u32, 0u32],
            )
            .u32();
        let stack_word = e.mem.u32(ebp - 0x24);
        e.call(SOUND_HANDLE_PLAY, &args![sound, stack_word]);
        e.call(SOUND_HANDLE_DESTRUCT, &args![handle]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_f8c0, fn_007ff8c0(Ptr) -> bool),
        entry!(0x007f_f8e0, fn_007ff8e0(Ptr) -> bool),
        entry!(0x007f_fad0, fn_007ffad0(Ptr) -> u32),
        entry!(0x007f_fe00, fo_rendered_terminal_release_static_geometry()),
        entry!(
            0x007f_f650,
            fo_rendered_terminal_on_menu_open(Ptr<FORenderedTerminal>)
        ),
        entry!(0x007f_f740, fn_007ff740(Ptr<FORenderedTerminal>)),
        entry!(
            0x007f_f7c0,
            fn_007ff7c0(Ptr<FORenderedTerminal>, u32, u32, u32)
        ),
        entry!(
            0x007f_f820,
            fo_rendered_terminal_on_idle(Ptr<FORenderedTerminal>) -> bool
        ),
        entry!(0x007f_f900, fn_007ff900(Ptr<FORenderedTerminal>)),
        entry!(
            0x007f_f960,
            fo_rendered_terminal_init_light(Ptr<FORenderedTerminal>)
        ),
        entry!(
            0x007f_faf0,
            fo_rendered_terminal_fade_out(Ptr<FORenderedTerminal>)
        ),
        entry!(0x007f_fb60, fn_007ffb60(u32)),
        entry!(0x007f_fb80, fn_007ffb80() -> u32),
        entry!(
            0x007f_fba0,
            fn_007ffba0(Ptr<FORenderedTerminal>, u8, u32, u32) -> bool
        ),
        entry!(0x007f_fd50, fn_007ffd50(Ptr<FORenderedTerminal>)),
        entry!(0x007f_fdb0, fn_007ffdb0(Ptr<FORenderedTerminal>)),
        entry!(0x007f_fe40, fo_rendered_terminal_play_shutdown_sound()),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_007ff8c0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x007f_f8e0), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x007f_f8e0, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x007f_f8c0, &args![0x0000_5000u32]).bool());
    }
}

#[cfg(test)]
mod b069_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_007ff8e0_tests_the_flag_0x4000() {
        let mut e = Engine::new();
        e.register(FLAG_TEST_00456630, |_, a| {
            u32::from(a[1] == 0x4000).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f8e0, &args![0x5000u32]).bool());
        assert_eq!(calls_to(&e, FLAG_TEST_00456630), vec![vec![0x5000, 0x4000]]);
    }

    #[test]
    fn fn_007ffad0_returns_the_entry_at_index_1() {
        let mut e = Engine::new();
        e.register(TES_00450B80, |_, a| (0x100 + a[0]).into_ret());
        assert_eq!(e.call(0x007f_fad0, &args![0x5000u32]).u32(), 0x101);
    }

    #[test]
    fn release_static_geometry_releases_the_model_only_when_held() {
        let mut e = Engine::new();
        e.map(MODEL_LOADER_011C3B3C & !0xfff, 0x1000);
        e.map(TERMINAL_MODEL_PATH_011A0BB0 & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER_011C3B3C, 0x6000u32);
        e.set_global(TERMINAL_MODEL_PATH_011A0BB0, 0x0107_8b80u32);
        e.register(HOLDER_GET_00559450, |_, _| 1u32.into_ret());
        e.register(HOLDER_SET_0066B0D0, |_, _| Ret::default());
        e.register(MODEL_LOADER_RELEASE_MODEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![0x6000, 0x0107_8b80, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, HOLDER_SET_0066B0D0),
            vec![vec![HOLDER_011DB63C, 0]]
        );
        e.register(HOLDER_GET_00559450, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_fe00, &args![]);
        assert!(calls_to(&e, MODEL_LOADER_RELEASE_MODEL).is_empty());
        assert_eq!(calls_to(&e, HOLDER_SET_0066B0D0).len(), 1);
    }
}

#[cfg(test)]
mod n051_tests {
    use super::*;

    const STUBBED: &[u32] = &[
        RENDERED_MENU_ON_IDLE,
        RENDERED_MENU_ON_MENU_OPEN,
        RENDERED_MENU_ON_MENU_CLOSE,
        RENDERED_MENU_PICK,
        RENDERED_MENU_RENDER,
        SCREEN_PARENT_005585E0,
        MENU_ROOT_0063D040,
        MENU_LIGHT_006838B0,
        SET_APP_CULLED_00450F90,
        UPDATE_DATA_INIT_0043D410,
        UPDATE_00A59C60,
        FADE_NODE_SET_RANGE,
        GEOMETRY_0054B800,
        GEOMETRY_00476AE0,
        GEOMETRY_FADE_006D2C40,
        SET_POSITION_00440460,
        SET_TRANSLATE_004BC1F0,
        INTERFACE_MANAGER,
        ENTER_RENDERED_MENU,
        SETTING_VALUE_ADDRESS,
        SETTING_SET_0045CE80,
        MENU_OBJECT_009611E0,
        MENU_00984F60,
        AUDIO_MANAGER_INSTANCE,
        AUDIO_MANAGER_SOUND_BY_NAME,
        SOUND_HANDLE_PLAY,
        SOUND_HANDLE_DESTRUCT,
        POINT_INIT_006815C0,
        CAMERA_008B6200,
        GETTER_0045C670,
        GETTER_006629F0,
        NI_PICK_CONSTRUCT,
        PICK_00458B30,
        PICK_SET_TARGET_00705FC0,
        PICK_006ECD40,
        CURSOR_00792760,
        CURSOR_009A9350,
        FTOL,
        CAMERA_WINDOW_POINT_TO_RAY,
        NI_PICK_PICK_OBJECTS,
        PICK_RESULTS_00500940,
        PICK_RECORD_00968670,
        RECORD_OBJECT_00458B50,
        NI_PICK_DESTROY,
        TES_00450B80,
        HOLDER_GET_00559450,
        FLAG_TEST_00456630,
        CREATE_MENU_LIGHT,
        PRINT_MESSAGE,
        POINT_FROM_FLOATS_00416870,
        LIGHT_004BC280,
        LIGHT_0050C630,
        LIGHT_0050C660,
        LIGHT_0050C690,
        LIGHT_0050DD20,
        LIGHT_0050DD50,
        SETTING_FLOAT_VALUE,
        MENU_LIGHT_COLOR_006B0570,
        MENU_00965580,
        MENU_007F99B0,
    ];

    const VTABLE: u32 = 0x0f00_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x0107_8000, 0x1000);
        e.map(0x011c_1000, 0x1000);
        e.set_global(OPEN_FADE_RANGE, 100_000.0f32);
        e.set_global(ROOT_OFFSET_01078BB0, 30_490.0f32);
        for &addr in STUBBED {
            e.register(addr, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x80).map(|i| slot(4 * i)).collect();
        for &s in &slots {
            e.register(s, |_, _| Ret::default());
        }
        e.put_vtable(VTABLE, &slots);
        e
    }

    fn terminal(e: &mut Engine) -> Ptr<FORenderedTerminal> {
        let this = Ptr::new(e.mem.alloc(0xf0));
        e.mem.set_u32(this.addr(), VTABLE);
        this
    }

    #[test]
    fn opening_attaches_the_root_and_resets_the_fade() {
        let mut e = engine();
        let this = terminal(&mut e);
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, TES_00450B80, scene);
        answer(&mut e, SCREEN_PARENT_005585E0, 0x5000);
        answer(&mut e, MENU_LIGHT_006838B0, 0x5100);
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        e.set(this, FORenderedTerminal::bFadingOut, 1);
        e.call_log = Some(vec![]);
        e.call(0x007f_f650, &args![this]);
        assert_eq!(calls_to(&e, slot(0xdc)), vec![vec![scene, 0x5000, 1]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5000);
        assert_eq!(calls_to(&e, SET_APP_CULLED_00450F90), vec![vec![0x5100, 0]]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_ON_MENU_OPEN),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.get(this, FORenderedTerminal::bFadingOut), 0);
        let range = 100_000.0f32.to_bits();
        assert_eq!(
            calls_to(&e, FADE_NODE_SET_RANGE),
            vec![vec![0x5200, range, range]]
        );
        assert_eq!(calls_to(&e, GEOMETRY_00476AE0), vec![vec![0x5200, 0]]);
    }

    #[test]
    fn closing_hides_the_node_and_detaches_the_root() {
        let mut e = engine();
        let this = terminal(&mut e);
        let scene = e.mem.alloc(4);
        e.mem.set_u32(scene, VTABLE);
        answer(&mut e, TES_00450B80, scene);
        answer(&mut e, SCREEN_PARENT_005585E0, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x007f_f740, &args![this]);
        assert!(calls_to(&e, SET_APP_CULLED_00450F90).is_empty());
        answer(&mut e, MENU_LIGHT_006838B0, 0x5100);
        e.call_log = Some(vec![]);
        e.call(0x007f_f740, &args![this]);
        assert_eq!(calls_to(&e, SET_APP_CULLED_00450F90), vec![vec![0x5100, 1]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5100);
        assert_eq!(calls_to(&e, slot(0xe8)), vec![vec![scene, 0x5000]]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_ON_MENU_CLOSE),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn the_light_goes_back_and_the_menu_renders() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_f7c0, &args![this, 0x7000u32, 9u32, 9u32]);
        assert!(calls_to(&e, SET_POSITION_00440460).is_empty());
        assert_eq!(
            calls_to(&e, RENDERED_MENU_RENDER),
            vec![vec![this.addr(), 0x7000, 1, 0]]
        );
        answer(&mut e, MENU_ROOT_0063D040, 0x5300);
        e.call_log = Some(vec![]);
        e.call(0x007f_f7c0, &args![this, 0x7000u32, 9u32, 9u32]);
        assert_eq!(
            calls_to(&e, SET_POSITION_00440460),
            vec![vec![0x5300, this.addr() + 0xd8]]
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_fdb0, &args![this]);
        assert_eq!(calls_to(&e, SET_POSITION_00440460).len(), 1);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5300);
        e.call_log = Some(vec![]);
        e.call(0x007f_f900, &args![this]);
        assert_eq!(
            calls_to(&e, SET_TRANSLATE_004BC1F0),
            vec![vec![0x5300, 30_490.0f32.to_bits(), 0, 0]]
        );
    }

    #[test]
    fn idle_waits_for_the_fade_to_settle() {
        let mut e = engine();
        let this = terminal(&mut e);
        answer(&mut e, RENDERED_MENU_ON_IDLE, 1);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        e.set(this, FORenderedTerminal::bFadingOut, 1);
        // No geometry: done.
        answer(&mut e, RENDERED_MENU_ON_IDLE, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU).len(), 1);
        // Still fading and changing: the fade is kept and the menu idles.
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        answer(&mut e, FLAG_TEST_00456630, 1);
        e.register(GEOMETRY_FADE_006D2C40, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(e.get(this, FORenderedTerminal::fLastFadePercent), 0.5);
        assert!(calls_to(&e, ENTER_RENDERED_MENU).is_empty());
        // The same fade again: done.
        e.call_log = Some(vec![]);
        assert!(e.call(0x007f_f820, &args![this]).bool());
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU).len(), 1);
    }

    #[test]
    fn fading_out_clears_the_setting_around_the_range() {
        let mut e = engine();
        let this = terminal(&mut e);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x33);
        answer(&mut e, SETTING_VALUE_ADDRESS, value);
        e.call_log = Some(vec![]);
        e.call(0x007f_faf0, &args![this]);
        assert!(calls_to(&e, FADE_NODE_SET_RANGE).is_empty());
        assert_eq!(e.get(this, FORenderedTerminal::bFadingOut), 1);
        assert_eq!(e.get(this, FORenderedTerminal::fLastFadePercent), 1.0);
        answer(&mut e, HOLDER_GET_00559450, 0x5200);
        e.call_log = Some(vec![]);
        e.call(0x007f_faf0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_SET_0045CE80),
            vec![vec![SETTING_011F9148, 0], vec![SETTING_011F9148, 0x33]]
        );
        assert_eq!(calls_to(&e, FADE_NODE_SET_RANGE), vec![vec![0x5200, 0, 0]]);
        assert_eq!(e.call(0x007f_fb80, &args![]).u32(), 0x33);
    }

    #[test]
    fn a_click_on_the_exit_button_closes() {
        let mut e = engine();
        let this = terminal(&mut e);
        answer(&mut e, RENDERED_MENU_PICK, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007f_fba0, &args![this, 1u32, 0x10u32, 0x20u32])
            .bool());
        assert_eq!(
            calls_to(&e, RENDERED_MENU_PICK),
            vec![vec![this.addr(), 1, 0x10, 0x20]]
        );
        answer(&mut e, HOLDER_GET_00559450, 0x5400);
        answer(&mut e, NI_PICK_PICK_OBJECTS, 1);
        answer(&mut e, PICK_RECORD_00968670, 0x5500);
        answer(&mut e, RECORD_OBJECT_00458B50, 0x5400);
        answer(&mut e, CAMERA_008B6200, 0x5600);
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(CURSOR_00792760, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.register(CURSOR_009A9350, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x007f_fba0, &args![this, 1u32, 0x10u32, 0x20u32])
            .bool());
        let ray = calls_to(&e, CAMERA_WINDOW_POINT_TO_RAY);
        assert_eq!(ray[0][..3], [0x5600, 4, 3]);
        assert_eq!(calls_to(&e, PICK_SET_TARGET_00705FC0)[0][1], 0x5400);
        assert!(calls_to(&e, RENDERED_MENU_PICK).is_empty());
        assert_eq!(calls_to(&e, MENU_00984F60), vec![vec![this.addr(), 0]]);
        assert_eq!(calls_to(&e, NI_PICK_DESTROY).len(), 1);
        // Without the flag the menu's pick answers.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x007f_fba0, &args![this, 0u32, 0x10u32, 0x20u32])
            .bool());
        assert!(calls_to(&e, NI_PICK_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_light_is_created_and_set_up() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x007f_f960, &args![this]);
        assert_eq!(calls_to(&e, PRINT_MESSAGE), vec![vec![NO_LIGHT_TEXT]]);
        assert!(calls_to(&e, LIGHT_004BC280).is_empty());
        answer(&mut e, CREATE_MENU_LIGHT, 1);
        answer(&mut e, TES_00450B80, 0x5600);
        answer(&mut e, MENU_LIGHT_006838B0, 0x5700);
        e.register(POINT_FROM_FLOATS_00416870, |_, a| a[0].into_ret());
        // Each setting answers its own address as the value.
        e.register(SETTING_FLOAT_VALUE, |e, a| {
            let value = e.mem.alloc(4);
            e.mem.set_u32(value, a[0] & 0xffff);
            value.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x007f_f960, &args![this]);
        assert_eq!(
            calls_to(&e, CREATE_MENU_LIGHT),
            vec![vec![this.addr(), LIGHT_NODE_NAME, 0x5600]]
        );
        assert_eq!(
            calls_to(&e, LIGHT_0050DD20),
            vec![vec![0x5700, 1.0f32.to_bits()]]
        );
        let points = calls_to(&e, POINT_FROM_FLOATS_00416870);
        assert_eq!(points[1][1..], [0xb644, 0, 0]);
        assert_eq!(points[2][1..], [0xb608, 0xb668, 0xb5f8]);
        assert_eq!(
            calls_to(&e, MENU_LIGHT_COLOR_006B0570),
            vec![vec![this.addr(), points[2][0]]]
        );
        assert_eq!(calls_to(&e, MENU_00965580), vec![vec![this.addr(), 0xb650]]);
        assert_eq!(calls_to(&e, UPDATE_00A59C60)[0][0], 0x5700);
    }

    #[test]
    fn closing_plays_the_shutdown_sound_with_the_stack_word() {
        let mut e = engine();
        let this = terminal(&mut e);
        e.set_global(SECURITY_COOKIE, 0x1234_5678u32);
        answer(&mut e, AUDIO_MANAGER_SOUND_BY_NAME, 0x7700);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, VTABLE);
        answer(&mut e, MENU_OBJECT_009611E0, object);
        e.call_log = Some(vec![]);
        e.call(0x007f_fd50, &args![this]);
        let sound = calls_to(&e, AUDIO_MANAGER_SOUND_BY_NAME);
        assert_eq!(sound[0][2..], [SHUTDOWN_SOUND, 0x121, 0]);
        let played = calls_to(&e, SOUND_HANDLE_PLAY);
        assert_eq!(played.len(), 1);
        assert_eq!(played[0][0], 0x7700);
        // The word read is the saved cookie: the cookie xor the frame's `EBP`.
        assert_eq!(played[0][1] ^ 0x1234_5678, sound[0][1] + 0x18);
        assert_eq!(calls_to(&e, slot(0)), vec![vec![object, 1]]);
        assert_eq!(calls_to(&e, ENTER_RENDERED_MENU)[0][1], 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_fb60, &args![5u32]);
        assert_eq!(
            calls_to(&e, SETTING_SET_0045CE80),
            vec![vec![SETTING_011F9148, 5]]
        );
    }
}
