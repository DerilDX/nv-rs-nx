//! `fallout/interface/fopipboymanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007fa200 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `007f9840` (Ghidra name
/// `FOPipboyManager::InitLight`), which creates the menu light `Omni01` and sets its
/// colours. The engine map names this address `FOPipboyManager::ReInitialize` from the call
/// graph only (row `cg`), which this translation does not confirm, and returns nothing.
/// Parameters: one word, `this`; the disassembly of the function ends in a plain `RET` and
/// passes only `ECX` on. `007f9840`'s disassembly ends with `CALL 007f99b0` (which stores a
/// float and ends `RET 4`) and no result is computed (Ghidra types it void).
pub fn fn_007fa200(e: &mut Engine, this: Ptr) {
    e.call(0x007f_9840, &args![this]);
}

/// The player (`PlayerCharacter *` global).
const PLAYER: u32 = 0x011d_ea3c;
/// The player's 3D root (`this` = player, flag).
const PLAYER_3D: u32 = 0x0095_0bb0;
/// `NiFixedString` constructor (`this`, text; answers the string) and
/// destructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// The checked cast to an RTTI type (cdecl: type record, object).
const NI_CAST: u32 = 0x0065_3270;
/// The RTTI records the code casts with (`NiNode` and `NiGeometry` by how
/// the results are used; not confirmed).
const RTTI_011F4428: u32 = 0x011f_4428;
const RTTI_011F4ACC: u32 = 0x011f_4acc;
/// A child of a node (`this`, index).
const CHILD_AT: u32 = 0x0045_bc00;
/// The object's parent (`this` only).
const PARENT_OF: u32 = 0x0096_11e0;
/// `NiPointer` assignment (`this` = the pointer, object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The pointer's object (`this` = the pointer).
const POINTER_GET: u32 = 0x0055_9450;
/// `NiAVObject::SetAppCulled`-shaped call (`this`, flag).
const SET_CULLED: u32 = 0x0045_0f90;
/// The Pip-Boy state test (`this` only; a byte).
const PIPBOY_STATE: u32 = 0x0097_4d90;
/// A setting's bool address (`ECX` = the setting).
const SETTING_BOOL_ADDRESS: u32 = 0x0040_8d60;
/// A setting's value address (`ECX` = the setting).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// `FORenderedMenu::Initialize`, `PrepareGeometry` and `CreateMenuLight`.
const RENDERED_MENU_INITIALIZE: u32 = 0x007f_c150;
const RENDERED_MENU_PREPARE_GEOMETRY: u32 = 0x007f_c2d0;
const RENDERED_MENU_CREATE_LIGHT: u32 = 0x007f_c400;
/// The interface manager singleton getter and the object `007fa950`
/// answers for it.
const INTERFACE_SINGLETON: u32 = 0x004b_7210;
const INTERFACE_OBJECT_007FA950: u32 = 0x007f_a950;
/// Stores an object at +0x14 of the rendered menu (`run_00705fc0`).
const SET_MEMBER_0X14: u32 = 0x0070_5fc0;
/// A call with the player's 3D root (`this`, root).
const USE_PLAYER_ROOT: u32 = 0x004d_c540;
/// `NiUpdateData` constructor (`this`, time, two flags).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// The node's update (`this`, update data).
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiAVObject::UpdateProperties`.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `NiColorA` constructor (`this`, four floats).
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
/// `SystemColorManager::GetInstance` (no arguments) and `GetColor`
/// (`this`, index, colour).
const SYSTEM_COLORS: u32 = 0x0071_8b60;
const SYSTEM_COLOR: u32 = 0x0071_9060;
/// A float setter of the menu (`this`, float).
const SETTER_00444020: u32 = 0x0044_4020;
/// A byte setter of the menu (`this`, flag).
const SET_FLAG_00810530: u32 = 0x0081_0530;
/// A geometry's property list entry (`this`, index).
const PROPERTY_AT: u32 = 0x0043_b4a0;
/// The shared body that answers `this` (here constructing two `NiMatrix3`
/// temporaries; it does not write them).
const ANSWER_THIS: u32 = 0x0068_15c0;
/// The local rotation of an object (`this` only; nine floats).
const LOCAL_ROTATION: u32 = 0x006a_9540;
/// `NiMatrix3` product (`this`, result, other).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// Sets an object's rotation (`this`, matrix).
const SET_ROTATION: u32 = 0x0043_fa80;
/// Sets an angle about an axis (`this`, angle, x, y, z).
const SET_AXIS_ANGLE: u32 = 0x0064_04b0;
/// A float of the interface (`007ddef0`, no arguments; ST0).
const INTERFACE_VALUE_007DDEF0: u32 = 0x007d_def0;
/// `NiPoint3` constructor (`this`, x, y, z).
const POINT_CONSTRUCT: u32 = 0x0041_6870;
/// The menu light (`this` only).
const MENU_LIGHT: u32 = 0x0068_38b0;
/// The light's setters.
const LIGHT_SET_AMBIENT: u32 = 0x004b_c2e0;
const LIGHT_SET_DIFFUSE: u32 = 0x004b_c280;
const LIGHT_SET_0050C630: u32 = 0x0050_c630;
const LIGHT_SET_0050C660: u32 = 0x0050_c660;
const LIGHT_SET_0050C690: u32 = 0x0050_c690;
const LIGHT_SET_DIMMER: u32 = 0x0050_dd20;
const LIGHT_SET_0050DD50: u32 = 0x0050_dd50;
/// The menu's screen light colour and intensity setters.
const SET_SCREEN_LIGHT_COLOR: u32 = 0x006b_0570;
const SET_SCREEN_LIGHT_BASE: u32 = 0x0096_5580;
/// Tests and property calls of `ShowPipboyLightEffect`.
const GEOMETRY_TEST_00453470: u32 = 0x0045_3470;
const PROPERTY_KIND_004AF350: u32 = 0x004a_f350;
const PROPERTY_KIND_00438220: u32 = 0x0043_8220;
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_SET_0068C9F0: u32 = 0x0068_c9f0;
const SHADER_PROPERTY_SET_ALPHA: u32 = 0x00ba_8ab0;

/// The node names `Initialize`, `InitButtons`, `InitKnobs`, `InitLight`
/// and `ShowPipboyLightEffect` look up.
const PIPBOY_NAME_ONE: u32 = 0x0107_8140;
const PIPBOY_NAME_TWO: u32 = 0x0107_8130;
const LIGHT_NAME_ONE: u32 = 0x0107_778c;
const LIGHT_NAME_TWO: u32 = 0x0107_826c;
const MENU_LIGHT_NAME_ONE: u32 = 0x0107_8210;
const MENU_LIGHT_NAME_TWO: u32 = 0x0107_8204;
/// The settings `Initialize` reads.
const SETTING_ENABLED: u32 = 0x011d_b2cc;
const SETTING_BLUR: u32 = 0x011d_b2dc;
const SETTING_00444020: u32 = 0x011d_b328;
const SETTING_SCANLINES: u32 = 0x011d_b274;
const SETTING_LIGHT_RANGE: u32 = 0x011d_b514;
const SETTING_FIRST_TAB: u32 = 0x011d_b33c;
const SETTING_LAST_TAB: u32 = 0x011d_b34c;
/// `double` constants of `InitKnobs`.
const TAB_STEPS: u32 = 0x0101_db80;
const WHEEL_SCALE: u32 = 0x0101_ff40;
const WHEEL_OFFSET: u32 = 0x0106_4f50;

layout! {
    /// `FOPipboyManager` (Xbox PDB), 0x170 bytes; the fields this unit
    /// uses, at the PC offsets the code reads (the PDB's).
    pub struct FOPipboyManager: 0x170 {
        /// `ScreenLightColor`'s neighbour `fScreenLightFlickerIntensity`
        /// (Xbox PDB `FORenderedMenu`).
        0x3C fScreenLightFlickerIntensity: u32,
        /// `fBlurIntensity` (Xbox PDB `FORenderedMenu`).
        0x44 fBlurIntensity: u32,
        /// `fScanlineScale` (Xbox PDB `FORenderedMenu`).
        0x48 fScanlineScale: u32,
        /// `bDoScanlines` (Xbox PDB `FORenderedMenu`).
        0x4C bDoScanlines: u8,
        /// `b1stPersonLightOn` (Xbox PDB).
        0x160 b1stPersonLightOn: u8,
        /// `b3rdPersonLightOn` (Xbox PDB).
        0x161 b3rdPersonLightOn: u8,
    }
}

/// Offsets of `BackgroundColor`, `TintColor` (Xbox PDB `FORenderedMenu`),
/// `spPipboyRoot`, `sp3rdPersonPipboyRoot`, `sp3rdPersonScreen`,
/// `spButtons`, `spButtonGlow`, `spKnobs`, `fKnobPositions`,
/// `fKnobTargets`, `fKnobRates` and `fTabKnobPositions` (Xbox PDB; the
/// arrays hold three entries, five for the tab positions).
const BACKGROUND_COLOR: u32 = 0xa8;
const TINT_COLOR: u32 = 0xc0;
const PIPBOY_ROOT: u32 = 0xd8;
const THIRD_PERSON_ROOT: u32 = 0xdc;
const THIRD_PERSON_SCREEN: u32 = 0xe0;
const BUTTONS: u32 = 0xe8;
const BUTTON_GLOW: u32 = 0xf4;
const KNOBS: u32 = 0x100;
const KNOB_POSITIONS: u32 = 0x10c;
const KNOB_TARGETS: u32 = 0x118;
const KNOB_RATES: u32 = 0x124;
const TAB_KNOB_POSITIONS: u32 = 0x130;

/// The object named `name` under the player's 3D root (its slot `+0x9c`
/// with a temporary `NiFixedString`), handed to `f` while the string
/// lives, as the code orders its calls.
fn with_named<R>(e: &mut Engine, name: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    let player = e.global::<u32>(PLAYER);
    let root = e.call(PLAYER_3D, &args![player, 1u32]).u32();
    e.with_stack(4, |e, text| {
        let key = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let found = e.vcall(root, 0x9c, &args![key]).u32();
        let result = f(e, found);
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
        result
    })
}

/// [`with_named`] answering the object.
fn named(e: &mut Engine, name: u32) -> u32 {
    with_named(e, name, |_, found| found)
}

/// [`with_named`] answering the object cast to the RTTI record.
fn named_cast(e: &mut Engine, name: u32, rtti: u32) -> u32 {
    with_named(e, name, |e, found| {
        e.call(NI_CAST, &args![rtti, found]).u32()
    })
}

/// [`with_named`] storing the object in the pointer at `slot`.
fn assign_named(e: &mut Engine, slot: u32, name: u32) {
    with_named(e, name, |e, found| {
        e.call(POINTER_ASSIGN, &args![slot, found]);
    });
}

/// The object of the pointer at `slot`.
fn pointer(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// A setting's float, as the bit pattern an `FLD`/`FSTP` copy gives.
fn setting_float_bits(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    x87_quiet_bits(e.mem.u32(address))
}

fn copy_color(e: &mut Engine, this: Ptr<FOPipboyManager>, offset: u32, color: u32) {
    for i in 0..4 {
        let word = e.mem.u32(color.wrapping_add(4 * i));
        e.mem
            .set_u32(this.addr().wrapping_add(offset + 4 * i), word);
    }
}

// Translated from 007f7fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words of the colour at `color` into `TintColor` (+0xc0,
/// `FORenderedMenu`, Xbox PDB).
pub fn fn_007f7fb0(e: &mut Engine, this: Ptr<FOPipboyManager>, color: u32) {
    copy_color(e, this, TINT_COLOR, color);
}

// Translated from 007f7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words of the colour at `color` into `BackgroundColor`
/// (+0xa8, `FORenderedMenu`, Xbox PDB).
pub fn fn_007f7fe0(e: &mut Engine, this: Ptr<FOPipboyManager>, color: u32) {
    copy_color(e, this, BACKGROUND_COLOR, color);
}

// Translated from 007f86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets knob `index`'s target (`fKnobTargets`, +0x118) and rate
/// (`fKnobRates`, +0x124) from the two floats (passed as bit patterns).
pub fn fn_007f86f0(e: &mut Engine, this: Ptr<FOPipboyManager>, index: u32, target: u32, rate: u32) {
    let at = this.addr().wrapping_add(index.wrapping_mul(4));
    e.mem
        .set_u32(at.wrapping_add(KNOB_TARGETS), x87_quiet_bits(target));
    e.mem
        .set_u32(at.wrapping_add(KNOB_RATES), x87_quiet_bits(rate));
}

// Translated from 007f8ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::Initialize` (Xbox PDB). With the setting `011db2cc`
/// set, the Pip-Boy state test false and a player with a 3D root: finds the
/// Pip-Boy node (`01078140`, else `01078130`, as `011f4428`), initializes
/// the rendered menu, stores the interface's object, takes the Pip-Boy
/// root and screen (from the node's first child, or the `011f4acc` object
/// named `01078130`), the third-person root and screen, prepares both
/// screens' geometry, initializes the buttons, knobs and light, updates
/// both roots (time 0.0), takes the background and tint colours from
/// system colour 4 when there is one, reads three settings into the blur,
/// `00444020` and scanline values, calls `00810530(1)` and turns both
/// Pip-Boy lights off without storing the state.
// Translation incomplete: 007f8ba0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_initialize(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let enabled = e.call(SETTING_BOOL_ADDRESS, &args![SETTING_ENABLED]).u32();
    if e.mem.u8(enabled) == 0 {
        return;
    }
    if e.call(PIPBOY_STATE, &args![this]).u8() != 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    if player == 0 || e.call(PLAYER_3D, &args![player, 1u32]).u32() == 0 {
        return;
    }
    let mut third_screen = 0u32;
    let mut pipboy = named_cast(e, PIPBOY_NAME_ONE, RTTI_011F4428);
    if pipboy == 0 {
        pipboy = named_cast(e, PIPBOY_NAME_TWO, RTTI_011F4428);
    }
    e.call(RENDERED_MENU_INITIALIZE, &args![this]);
    let interface = e.call(INTERFACE_SINGLETON, &args![]).u32();
    let object = e.call(INTERFACE_OBJECT_007FA950, &args![interface]).u32();
    e.call(SET_MEMBER_0X14, &args![this, object]);
    let screen = if pipboy != 0 {
        let screen = e.call(CHILD_AT, &args![pipboy, 0u32]).u32();
        let parent = e.call(PARENT_OF, &args![pipboy]).u32();
        e.call(POINTER_ASSIGN, &args![this.addr() + PIPBOY_ROOT, parent]);
        screen
    } else {
        let screen = named_cast(e, PIPBOY_NAME_TWO, RTTI_011F4ACC);
        if screen != 0 {
            let parent = e.call(PARENT_OF, &args![screen]).u32();
            e.call(POINTER_ASSIGN, &args![this.addr() + PIPBOY_ROOT, parent]);
        }
        screen
    };
    let player = e.global::<u32>(PLAYER);
    let root = e.call(PLAYER_3D, &args![player, 1u32]).u32();
    e.call(USE_PLAYER_ROOT, &args![this, root]);
    if pipboy != 0 {
        let child = e.call(CHILD_AT, &args![pipboy, 0u32]).u32();
        third_screen = e.call(NI_CAST, &args![RTTI_011F4ACC, child]).u32();
        let parent = e.call(PARENT_OF, &args![pipboy]).u32();
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_ROOT, parent],
        );
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_SCREEN, third_screen],
        );
    } else if screen != 0 {
        third_screen = screen;
        let parent = e.call(PARENT_OF, &args![screen]).u32();
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_ROOT, parent],
        );
        e.call(
            POINTER_ASSIGN,
            &args![this.addr() + THIRD_PERSON_SCREEN, third_screen],
        );
    }
    if screen != 0 && third_screen != 0 {
        e.call(RENDERED_MENU_PREPARE_GEOMETRY, &args![this, screen]);
        e.call(RENDERED_MENU_PREPARE_GEOMETRY, &args![this, third_screen]);
        fn_007f9010(e, this, screen);
    }
    fo_pipboy_manager_init_buttons(e, this);
    fo_pipboy_manager_init_knobs(e, this);
    fo_pipboy_manager_init_light(e, this);
    for slot in [PIPBOY_ROOT, THIRD_PERSON_ROOT] {
        if pointer(e, this.addr() + slot) != 0 {
            e.with_stack(0xc, |e, update| {
                e.call(UPDATE_DATA_CONSTRUCT, &args![update, 0.0f32, 0u32, 0u32]);
                let node = pointer(e, this.addr() + slot);
                e.call(NODE_UPDATE, &args![node, update]);
                let node = pointer(e, this.addr() + slot);
                e.call(UPDATE_PROPERTIES, &args![node]);
            });
        }
    }
    e.with_stack(0x10, |e, color| {
        e.call(
            COLOR_CONSTRUCT,
            &args![color, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let colors = e.call(SYSTEM_COLORS, &args![]).u32();
        if e.call(SYSTEM_COLOR, &args![colors, 4u32, color]).bool() {
            fn_007f7fe0(e, this, color.addr());
            fn_007f7fb0(e, this, color.addr());
        }
    });
    let blur = setting_float_bits(e, SETTING_BLUR);
    fn_007f9030(e, this, blur);
    let value = setting_float_bits(e, SETTING_00444020);
    e.call(SETTER_00444020, &args![this, value]);
    let scale = setting_float_bits(e, SETTING_SCANLINES);
    fn_007f9050(e, this, 1, scale);
    e.call(SET_FLAG_00810530, &args![this, 1u32]);
    fo_pipboy_manager_show_pipboy_light_effect(e, this, 0, 0, 1);
    fo_pipboy_manager_show_pipboy_light_effect(e, this, 1, 0, 1);
}

// Translated from 007f9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `object` in the pointer at +4 (`spScreenTriShape`, Xbox PDB
/// `FORenderedMenu`) through the `NiPointer` assignment `0066b0d0`.
pub fn fn_007f9010(e: &mut Engine, this: Ptr<FOPipboyManager>, object: u32) {
    e.call(POINTER_ASSIGN, &args![this.byte_add(4), object]);
}

// Translated from 007f9030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fBlurIntensity` (+0x44, Xbox PDB `FORenderedMenu`) from the float
/// (passed as its bit pattern).
pub fn fn_007f9030(e: &mut Engine, this: Ptr<FOPipboyManager>, value: u32) {
    e.set(this, FOPipboyManager::fBlurIntensity, x87_quiet_bits(value));
}

// Translated from 007f9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bDoScanlines` (+0x4c) and `fScanlineScale` (+0x48, Xbox PDB
/// `FORenderedMenu`; the float passed as its bit pattern).
pub fn fn_007f9050(e: &mut Engine, this: Ptr<FOPipboyManager>, on: u8, scale: u32) {
    e.set(this, FOPipboyManager::bDoScanlines, on);
    e.set(this, FOPipboyManager::fScanlineScale, x87_quiet_bits(scale));
}

// Translated from 007f9070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitButtons` (Xbox PDB): with the buttons node
/// (`010781f4`, else `010781e0`) present, stores the first child of each
/// of the three button nodes (each name tried, then its fallback, cast as
/// `011f4428`) in `spButtons`; with the glow node (`0107818c`, else
/// `01078180`) present, stores the three glow objects in `spButtonGlow`
/// (each with a fallback name) and hides them (`007fa010(0)`).
// Translation incomplete: 007f9070 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_init_buttons(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let mut found = named(e, 0x0107_81f4);
    if found == 0 {
        found = named(e, 0x0107_81e0);
    }
    if found != 0 {
        for (index, (name, fallback)) in [
            (0x0107_81f4u32, 0x0107_81e0u32),
            (0x0107_81d0, 0x0107_81bc),
            (0x0107_81ac, 0x0107_8198),
        ]
        .into_iter()
        .enumerate()
        {
            let mut node = named_cast(e, name, RTTI_011F4428);
            if node == 0 {
                node = named_cast(e, fallback, RTTI_011F4428);
            }
            let child = e.call(CHILD_AT, &args![node, 0u32]).u32();
            let slot = this.addr() + BUTTONS + 4 * index as u32;
            e.call(POINTER_ASSIGN, &args![slot, child]);
        }
    }
    let mut glow = named(e, 0x0107_818c);
    if glow == 0 {
        glow = named(e, 0x0107_8180);
    }
    if glow == 0 {
        return;
    }
    for (index, (name, fallback)) in [
        (0x0107_818cu32, 0x0107_8180u32),
        (0x0107_8174, 0x0107_8168),
        (0x0107_815c, 0x0107_8150),
    ]
    .into_iter()
    .enumerate()
    {
        let slot = this.addr() + BUTTON_GLOW + 4 * index as u32;
        assign_named(e, slot, name);
        if pointer(e, slot) == 0 {
            assign_named(e, slot, fallback);
        }
    }
    fn_007fa010(e, this, 0);
}

// Translated from 007f9840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitLight` (Xbox PDB): creates the menu light
/// (`01078210`, else `01078204`) and, when there is one, zeroes its two
/// colours and three values, sets its dimmer to 1.0 and its point
/// (`0050dd50`) to (setting `011db514`, 0, 0), sets the screen light colour
/// from system colour 4 and both screen light intensities to 1.0.
pub fn fo_pipboy_manager_init_light(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    if !e
        .call(
            RENDERED_MENU_CREATE_LIGHT,
            &args![this, MENU_LIGHT_NAME_ONE, 0u32],
        )
        .bool()
    {
        e.call(
            RENDERED_MENU_CREATE_LIGHT,
            &args![this, MENU_LIGHT_NAME_TWO, 0u32],
        );
    }
    let light = e.call(MENU_LIGHT, &args![this]).u32();
    if light == 0 {
        return;
    }
    e.with_stack(0x40, |e, frame| {
        let first = frame.addr();
        let second = frame.addr() + 0xc;
        let third = frame.addr() + 0x18;
        let colour = frame.addr() + 0x24;
        let point = frame.addr() + 0x34;
        let zero = e
            .call(POINT_CONSTRUCT, &args![first, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_AMBIENT, &args![light, zero]);
        let zero = e
            .call(POINT_CONSTRUCT, &args![second, 0.0f32, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_DIFFUSE, &args![light, zero]);
        e.call(LIGHT_SET_0050C630, &args![light, 0.0f32]);
        e.call(LIGHT_SET_0050C660, &args![light, 0.0f32]);
        e.call(LIGHT_SET_0050C690, &args![light, 0.0f32]);
        e.call(LIGHT_SET_DIMMER, &args![light, 1.0f32]);
        let range = setting_float_bits(e, SETTING_LIGHT_RANGE);
        let at = e
            .call(POINT_CONSTRUCT, &args![third, range, 0.0f32, 0.0f32])
            .u32();
        e.call(LIGHT_SET_0050DD50, &args![light, at]);
        e.call(
            COLOR_CONSTRUCT,
            &args![colour, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        let colors = e.call(SYSTEM_COLORS, &args![]).u32();
        e.call(SYSTEM_COLOR, &args![colors, 4u32, colour]);
        let red = x87_quiet_bits(e.mem.u32(colour));
        let green = x87_quiet_bits(e.mem.u32(colour + 4));
        let blue = x87_quiet_bits(e.mem.u32(colour + 8));
        e.call(POINT_CONSTRUCT, &args![point, red, green, blue]);
        e.call(SET_SCREEN_LIGHT_COLOR, &args![this, point]);
    });
    e.call(SET_SCREEN_LIGHT_BASE, &args![this, 1.0f32]);
    fn_007f99b0(e, this, 1.0f32.to_bits());
}

// Translated from 007f99b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fScreenLightFlickerIntensity` (+0x3c, Xbox PDB `FORenderedMenu`)
/// from the float (passed as its bit pattern).
pub fn fn_007f99b0(e: &mut Engine, this: Ptr<FOPipboyManager>, value: u32) {
    e.set(
        this,
        FOPipboyManager::fScreenLightFlickerIntensity,
        x87_quiet_bits(value),
    );
}

// Translated from 007f99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::InitKnobs` (Xbox PDB): takes the tab knob (the first
/// property of the geometry of `01078254`, else `01078244`) and the two
/// other knobs (`01078238`/`0107822c`, `01078224`/`01078218`); spreads the
/// five tab knob positions evenly from the setting `011db33c` to the
/// setting `011db34c` (x87), starts the third knob at the first position,
/// the second at a value from the player's actor value `0x36` and
/// `007ddef0` (0.0 without a player), clears two rates and turns each knob
/// to its position about its axis (the tab knob's rotation first composed
/// with its parent's).
// Translation incomplete: 007f99d0 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_init_knobs(e: &mut Engine, this: Ptr<FOPipboyManager>) {
    let me_addr = this.addr();
    let mut tab_knob = named(e, 0x0107_8254);
    if tab_knob == 0 {
        tab_knob = named(e, 0x0107_8244);
    }
    if tab_knob != 0 && e.vcall(tab_knob, 0xc, &args![]).u32() != 0 {
        let geometry = e.vcall(tab_knob, 0xc, &args![]).u32();
        let property = e.call(PROPERTY_AT, &args![geometry, 0u32]).u32();
        e.call(POINTER_ASSIGN, &args![me_addr + KNOBS, property]);
    }
    for (index, (name, fallback)) in [(0x0107_8238u32, 0x0107_822cu32), (0x0107_8224, 0x0107_8218)]
        .into_iter()
        .enumerate()
    {
        let slot = me_addr + KNOBS + 4 * (index as u32 + 1);
        assign_named(e, slot, name);
        if pointer(e, slot) == 0 {
            assign_named(e, slot, fallback);
        }
    }
    let first = setting_float_bits(e, SETTING_FIRST_TAB);
    e.mem.set_u32(me_addr + TAB_KNOB_POSITIONS, first);
    let last = setting_float_bits(e, SETTING_LAST_TAB);
    e.mem.set_u32(me_addr + TAB_KNOB_POSITIONS + 0x10, last);
    let span = x87_sub(
        x87_m32(e.mem.u32(me_addr + TAB_KNOB_POSITIONS + 0x10)),
        x87_m32(e.mem.u32(me_addr + TAB_KNOB_POSITIONS)),
    );
    let step = x87_store32(x87_div(span, e.global::<f64>(TAB_STEPS)));
    for index in 1..4u32 {
        let previous = e.mem.u32(me_addr + TAB_KNOB_POSITIONS + 4 * (index - 1));
        let value = x87_store32(x87_add(x87_m32(previous), x87_m32(step)));
        e.mem
            .set_u32(me_addr + TAB_KNOB_POSITIONS + 4 * index, value);
    }
    let first = x87_quiet_bits(e.mem.u32(me_addr + TAB_KNOB_POSITIONS));
    e.mem.set_u32(me_addr + KNOB_POSITIONS + 8, first);
    let first = x87_quiet_bits(e.mem.u32(me_addr + TAB_KNOB_POSITIONS));
    e.mem.set_u32(me_addr + KNOB_TARGETS + 8, first);
    e.mem.set_u32(me_addr + KNOB_RATES + 8, 0);
    let player = e.global::<u32>(PLAYER);
    let wheel = if player != 0 {
        let player = e.global::<u32>(PLAYER);
        // The player's object at +0xa4, slot +0xc, asked for actor value 0x36.
        let value = e
            .vcall(player.wrapping_add(0xa4), 0xc, &args![0x36u32])
            .f64();
        let divisor = e.call(INTERFACE_VALUE_007DDEF0, &args![]).f64();
        let ratio = x87_div(value, divisor);
        let scaled = x87_mul(ratio, e.global::<f64>(WHEEL_SCALE));
        x87_sub(e.global::<f64>(WHEEL_OFFSET), scaled)
    } else {
        0.0
    };
    e.mem
        .set_u32(me_addr + KNOB_TARGETS + 4, x87_store32(wheel));
    let target = x87_quiet_bits(e.mem.u32(me_addr + KNOB_TARGETS + 4));
    e.mem.set_u32(me_addr + KNOB_POSITIONS + 4, target);
    e.mem.set_u32(me_addr + KNOB_RATES + 4, 0);
    e.with_stack(0x6c, |e, frame| {
        // [EBP-0x54] and [EBP-0x30]: two matrices; [EBP-0x9c]: the product.
        let own = frame.addr();
        let parent = frame.addr() + 0x24;
        let product = frame.addr() + 0x48;
        e.call(ANSWER_THIS, &args![own]);
        e.call(ANSWER_THIS, &args![parent]);
        if pointer(e, me_addr + KNOBS) != 0 {
            let knob = pointer(e, me_addr + KNOBS);
            let rotation = e.call(LOCAL_ROTATION, &args![knob]).u32();
            let words = e.mem.bytes(rotation, 0x24);
            e.mem.write(own, &words);
            let knob = pointer(e, me_addr + KNOBS);
            let up = e.call(PARENT_OF, &args![knob]).u32();
            let rotation = e.call(LOCAL_ROTATION, &args![up]).u32();
            let words = e.mem.bytes(rotation, 0x24);
            e.mem.write(parent, &words);
            let result = e.call(MATRIX_MULTIPLY, &args![parent, product, own]).u32();
            let knob = pointer(e, me_addr + KNOBS);
            let up = e.call(PARENT_OF, &args![knob]).u32();
            e.call(SET_ROTATION, &args![up, result]);
            let angle = x87_quiet_bits(e.mem.u32(me_addr + KNOB_POSITIONS));
            let knob = pointer(e, me_addr + KNOBS);
            e.call(SET_AXIS_ANGLE, &args![knob, angle, 1.0f32, 0.0f32, 0.0f32]);
        }
    });
    for (index, offset) in [(1u32, 4u32), (2, 8)] {
        let slot = me_addr + KNOBS + 4 * index;
        if pointer(e, slot) != 0 {
            let angle = x87_quiet_bits(e.mem.u32(me_addr + KNOB_POSITIONS + offset));
            let knob = pointer(e, slot);
            e.call(SET_AXIS_ANGLE, &args![knob, angle, 0.0f32, 0.0f32, 1.0f32]);
        }
    }
}

// Translated from 007fa010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Shows the glow of button `button` (1 to 3; 0 shows none): unless the
/// Pip-Boy state test fails, hides the three button glows that exist and
/// shows glow `button - 1` (`spButtonGlow`, +0xf4; the index is not
/// checked against the array).
pub fn fn_007fa010(e: &mut Engine, this: Ptr<FOPipboyManager>, button: u32) {
    if e.call(PIPBOY_STATE, &args![this]).u8() == 0 {
        return;
    }
    for index in 0..3 {
        let slot = this.addr() + BUTTON_GLOW + 4 * index;
        if pointer(e, slot) != 0 {
            let glow = pointer(e, slot);
            e.call(SET_CULLED, &args![glow, 1u32]);
        }
    }
    if button != 0 {
        let slot = this
            .addr()
            .wrapping_add(button.wrapping_mul(4))
            .wrapping_add(0xf0);
        if pointer(e, slot) != 0 {
            let glow = pointer(e, slot);
            e.call(SET_CULLED, &args![glow, 0u32]);
        }
    }
}

// Translated from 007fa310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOPipboyManager::ShowPipboyLightEffect` (Xbox PDB name from the call
/// graph): finds the light node (`0107778c`, else `0107826c`) under the
/// first- or third-person Pip-Boy root and shows it when `on` (hides it
/// otherwise). With `store`, the state goes to `b1stPersonLightOn` when
/// `third_person` is set (as the code does) and to `b3rdPersonLightOn`
/// otherwise; in the latter case, when the node has geometry passing
/// `00453470` with a property list, the property of kind `004af350` (or the
/// shader property of kind `00438220`) gets `on` as a float and the
/// properties are updated. Without the root, only the state is stored.
// Translation incomplete: 007fa310 - the C++ exception-unwinding frame (FS:[0]) that destroys the temporary strings is not translated
pub fn fo_pipboy_manager_show_pipboy_light_effect(
    e: &mut Engine,
    this: Ptr<FOPipboyManager>,
    third_person: u8,
    on: u8,
    store: u8,
) {
    let slot = if third_person != 0 {
        this.addr() + THIRD_PERSON_ROOT
    } else {
        this.addr() + PIPBOY_ROOT
    };
    let root = pointer(e, slot);
    if root == 0 {
        if store != 0 {
            store_light_state(e, this, third_person, on);
        }
        return;
    }
    let mut node = find_under(e, root, LIGHT_NAME_ONE);
    if node == 0 {
        node = find_under(e, root, LIGHT_NAME_TWO);
    }
    if node == 0 {
        return;
    }
    e.call(SET_CULLED, &args![node, u32::from(on == 0)]);
    if store == 0 {
        return;
    }
    if third_person != 0 {
        e.set(this, FOPipboyManager::b1stPersonLightOn, on);
        return;
    }
    e.set(this, FOPipboyManager::b3rdPersonLightOn, on);
    let geometry = e.vcall(node, 0xc, &args![]).u32();
    if geometry == 0 || e.call(GEOMETRY_TEST_00453470, &args![geometry]).u32() == 0 {
        return;
    }
    let entry = e.call(PROPERTY_AT, &args![geometry, 0u32]).u32();
    let object = e.vcall(entry, 0x18, &args![]).u32();
    if object == 0 {
        return;
    }
    let value = x87_store32(f64::from(on));
    let kind = e.call(PROPERTY_KIND_004AF350, &args![]).u32();
    let property = e.call(GET_PROPERTY, &args![object, kind]).u32();
    if property != 0 {
        e.call(PROPERTY_SET_0068C9F0, &args![property, value]);
    } else {
        let kind = e.call(PROPERTY_KIND_00438220, &args![]).u32();
        let shader = e.call(GET_PROPERTY, &args![object, kind]).u32();
        e.call(SHADER_PROPERTY_SET_ALPHA, &args![shader, value]);
    }
    e.call(UPDATE_PROPERTIES, &args![object]);
}

/// The object named `name` under `root` (its slot `+0x9c` with a temporary
/// `NiFixedString`).
fn find_under(e: &mut Engine, root: u32, name: u32) -> u32 {
    e.with_stack(4, |e, text| {
        let key = e.call(FIXED_STRING_CONSTRUCT, &args![text, name]).u32();
        let found = e.vcall(root, 0x9c, &args![key]).u32();
        e.call(FIXED_STRING_DESTRUCT, &args![text]);
        found
    })
}

/// The stored light state of [`fo_pipboy_manager_show_pipboy_light_effect`]
/// without a root.
fn store_light_state(e: &mut Engine, this: Ptr<FOPipboyManager>, third_person: u8, on: u8) {
    if third_person != 0 {
        e.set(this, FOPipboyManager::b1stPersonLightOn, on);
    } else {
        e.set(this, FOPipboyManager::b3rdPersonLightOn, on);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007f_a200, fn_007fa200(Ptr)),
        entry!(0x007f_7fb0, fn_007f7fb0(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_7fe0, fn_007f7fe0(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_86f0,
            fn_007f86f0(Ptr<FOPipboyManager>, u32, u32, u32)
        ),
        entry!(
            0x007f_8ba0,
            fo_pipboy_manager_initialize(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_9010, fn_007f9010(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_9030, fn_007f9030(Ptr<FOPipboyManager>, u32)),
        entry!(0x007f_9050, fn_007f9050(Ptr<FOPipboyManager>, u8, u32)),
        entry!(
            0x007f_9070,
            fo_pipboy_manager_init_buttons(Ptr<FOPipboyManager>)
        ),
        entry!(
            0x007f_9840,
            fo_pipboy_manager_init_light(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_99b0, fn_007f99b0(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_99d0,
            fo_pipboy_manager_init_knobs(Ptr<FOPipboyManager>)
        ),
        entry!(0x007f_a010, fn_007fa010(Ptr<FOPipboyManager>, u32)),
        entry!(
            0x007f_a310,
            fo_pipboy_manager_show_pipboy_light_effect(Ptr<FOPipboyManager>, u8, u8, u8)
        ),
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
    fn fwd_a03_007fa200_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x007f_9840, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x007f_a200, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x007f_9840), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b070_tests {
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

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots are `base + slot`.
    fn object_with_vtable(e: &mut Engine, size: u32, base: u32) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
        }
        object
    }

    /// A player whose 3D root answers names through its slot `+0x9c`
    /// (double at `00c1_009c`): the fixed string's word is the name.
    fn player_with_root(e: &mut Engine) -> u32 {
        e.map(PLAYER & !0xfff, 0x1000);
        let player = e.mem.alloc(0x100);
        e.set_global(PLAYER, player);
        let root = object_with_vtable(e, 0x10, 0x00c1_0000);
        answer(e, PLAYER_3D, root);
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        stub(e, &[FIXED_STRING_DESTRUCT]);
        // The pointer of a slot answers the word in it.
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        player
    }

    #[test]
    fn colour_copies_and_float_setters() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let color = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(color + 4 * i, 0x10 + i);
        }
        e.call(0x007f_7fb0, &args![this, color]);
        e.call(0x007f_7fe0, &args![this, color]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this.addr() + 0xc0 + 4 * i), 0x10 + i);
            assert_eq!(e.mem.u32(this.addr() + 0xa8 + 4 * i), 0x10 + i);
        }
        e.call(0x007f_86f0, &args![this, 2u32, 1.5f32, 2.5f32]);
        assert_eq!(e.mem.u32(this.addr() + 0x118 + 8), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(this.addr() + 0x124 + 8), 2.5f32.to_bits());
        e.call(0x007f_9030, &args![this, 0.75f32]);
        assert_eq!(
            e.get(this, FOPipboyManager::fBlurIntensity),
            0.75f32.to_bits()
        );
        e.call(0x007f_9050, &args![this, 1u32, 3.0f32]);
        assert_eq!(e.get(this, FOPipboyManager::bDoScanlines), 1);
        assert_eq!(
            e.get(this, FOPipboyManager::fScanlineScale),
            3.0f32.to_bits()
        );
        e.call(0x007f_99b0, &args![this, 0x7f80_0001u32]);
        assert_eq!(
            e.get(this, FOPipboyManager::fScreenLightFlickerIntensity),
            0x7fc0_0001,
            "an FLD/FSTP copy quiets a signalling NaN"
        );
    }

    #[test]
    fn screen_pointer_store() {
        let mut e = Engine::new();
        stub(&mut e, &[POINTER_ASSIGN]);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_9010, &args![this, 0x900u32]);
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN),
            vec![vec![this.addr() + 4, 0x900]]
        );
    }

    #[test]
    fn button_glows_hide_all_and_show_one() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        e.mem.set_u32(this.addr() + 0xf4, 0x901);
        e.mem.set_u32(this.addr() + 0xfc, 0x903);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        answer(&mut e, PIPBOY_STATE, 1);
        stub(&mut e, &[SET_CULLED]);
        e.call_log = Some(vec![]);
        e.call(0x007f_a010, &args![this, 3u32]);
        assert_eq!(
            calls_to(&e, SET_CULLED),
            vec![vec![0x901, 1], vec![0x903, 1], vec![0x903, 0]]
        );
        answer(&mut e, PIPBOY_STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_a010, &args![this, 3u32]);
        assert!(calls_to(&e, SET_CULLED).is_empty());
    }

    #[test]
    fn light_effect_with_a_root_stores_the_state() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.mem.set_u32(this.addr() + 0xd8, root);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FIXED_STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let node = object_with_vtable(&mut e, 0x10, 0x00c2_0000);
        e.register_double(0x00c1_009c, move |e, a| Ret {
            eax: if e.mem.u32(a[1]) == LIGHT_NAME_TWO {
                node
            } else {
                0
            },
            ..Ret::default()
        });
        answer(&mut e, 0x00c2_000c, 0);
        stub(&mut e, &[FIXED_STRING_DESTRUCT, SET_CULLED]);
        e.call_log = Some(vec![]);
        e.call(0x007f_a310, &args![this, 0u32, 1u32, 1u32]);
        assert_eq!(calls_to(&e, 0x00c1_009c).len(), 2);
        assert_eq!(calls_to(&e, SET_CULLED), vec![vec![node, 0]]);
        assert_eq!(e.get(this, FOPipboyManager::b3rdPersonLightOn), 1);
        // Without the third-person root only the state is stored.
        e.call(0x007f_a310, &args![this, 1u32, 1u32, 1u32]);
        assert_eq!(e.get(this, FOPipboyManager::b1stPersonLightOn), 1);
    }

    #[test]
    fn light_effect_sets_the_property_value() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        e.mem.set_u32(this.addr() + 0xd8, root);
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let node = object_with_vtable(&mut e, 0x10, 0x00c2_0000);
        let geometry = e.mem.alloc(4);
        let entry = object_with_vtable(&mut e, 0x10, 0x00c3_0000);
        answer(&mut e, 0x00c1_009c, node);
        answer(&mut e, 0x00c2_000c, geometry);
        answer(&mut e, GEOMETRY_TEST_00453470, 1);
        answer(&mut e, PROPERTY_AT, entry);
        answer(&mut e, 0x00c3_0018, 0x950);
        answer(&mut e, PROPERTY_KIND_004AF350, 7);
        answer(&mut e, GET_PROPERTY, 0);
        answer(&mut e, PROPERTY_KIND_00438220, 8);
        stub(
            &mut e,
            &[
                FIXED_STRING_CONSTRUCT,
                FIXED_STRING_DESTRUCT,
                SET_CULLED,
                SHADER_PROPERTY_SET_ALPHA,
                UPDATE_PROPERTIES,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_a310, &args![this, 0u32, 1u32, 1u32]);
        assert_eq!(
            calls_to(&e, GET_PROPERTY),
            vec![vec![0x950, 7], vec![0x950, 8]]
        );
        assert_eq!(
            calls_to(&e, SHADER_PROPERTY_SET_ALPHA),
            vec![vec![0, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, UPDATE_PROPERTIES), vec![vec![0x950]]);
    }

    #[test]
    fn init_buttons_takes_buttons_and_glows() {
        let mut e = Engine::new();
        player_with_root(&mut e);
        let this = e.new_object::<FOPipboyManager>();
        // Every name is found except the glow names 0107818c (fallback
        // 01078180 is used); objects are named by their name's low word.
        e.register(0x00c1_009c, |e, a| {
            let name = e.mem.u32(a[1]);
            Ret {
                eax: if name == 0x0107_818c {
                    0
                } else {
                    name & 0xffff
                },
                ..Ret::default()
            }
        });
        e.register(NI_CAST, |_, a| Ret {
            eax: a[1] + 0x1_0000,
            ..Ret::default()
        });
        e.register(CHILD_AT, |_, a| Ret {
            eax: a[0] + 0x2_0000,
            ..Ret::default()
        });
        answer(&mut e, PIPBOY_STATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x007f_9070, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0xe8), 0x3_81f4);
        assert_eq!(e.mem.u32(me_addr + 0xec), 0x3_81d0);
        assert_eq!(e.mem.u32(me_addr + 0xf0), 0x3_81ac);
        assert_eq!(e.mem.u32(me_addr + 0xf4), 0x8180);
        assert_eq!(e.mem.u32(me_addr + 0xf8), 0x8174);
        assert_eq!(e.mem.u32(me_addr + 0xfc), 0x815c);
        assert_eq!(calls_to(&e, PIPBOY_STATE), vec![vec![me_addr]]);
    }

    #[test]
    fn init_light_sets_up_the_menu_light() {
        let mut e = Engine::new();
        let this = e.new_object::<FOPipboyManager>();
        answer(&mut e, RENDERED_MENU_CREATE_LIGHT, 0);
        answer(&mut e, MENU_LIGHT, 0x700);
        e.register(POINT_CONSTRUCT, |e, a| {
            e.mem
                .write(a[0], &[a[1], a[2], a[3]].map(u32::to_le_bytes).concat());
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let range = e.mem.alloc(4);
        e.mem.set_f32(range, 96.0);
        answer(&mut e, SETTING_VALUE_ADDRESS, range);
        stub(
            &mut e,
            &[
                LIGHT_SET_AMBIENT,
                LIGHT_SET_DIFFUSE,
                LIGHT_SET_0050C630,
                LIGHT_SET_0050C660,
                LIGHT_SET_0050C690,
                LIGHT_SET_DIMMER,
                LIGHT_SET_0050DD50,
                COLOR_CONSTRUCT,
                SYSTEM_COLORS,
                SET_SCREEN_LIGHT_COLOR,
                SET_SCREEN_LIGHT_BASE,
            ],
        );
        e.register(SYSTEM_COLOR, |e, a| {
            e.mem.set_f32(a[2], 0.25);
            e.mem.set_f32(a[2] + 4, 0.5);
            e.mem.set_f32(a[2] + 8, 0.75);
            Ret::default()
        });
        e.register(LIGHT_SET_0050DD50, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 96.0);
            Ret::default()
        });
        e.register(SET_SCREEN_LIGHT_COLOR, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 8), 0.75);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x007f_9840, &args![this]);
        assert_eq!(
            calls_to(&e, RENDERED_MENU_CREATE_LIGHT),
            vec![
                vec![this.addr(), MENU_LIGHT_NAME_ONE, 0],
                vec![this.addr(), MENU_LIGHT_NAME_TWO, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, LIGHT_SET_DIMMER),
            vec![vec![0x700, 1.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, SET_SCREEN_LIGHT_COLOR).len(), 1);
        assert_eq!(
            e.get(this, FOPipboyManager::fScreenLightFlickerIntensity),
            1.0f32.to_bits()
        );
    }

    #[test]
    fn init_knobs_spreads_the_tab_positions() {
        let mut e = Engine::new();
        let player = player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0106_4000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        e.set_global(WHEEL_SCALE, 2.0f64);
        e.set_global(WHEEL_OFFSET, 10.0f64);
        let this = e.new_object::<FOPipboyManager>();
        // No knob objects: every name answers 0.
        answer(&mut e, 0x00c1_009c, 0);
        let first = e.mem.alloc(4);
        e.mem.set_f32(first, 10.0);
        let last = e.mem.alloc(4);
        e.mem.set_f32(last, 30.0);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, a| Ret {
            eax: if a[0] == SETTING_FIRST_TAB {
                first
            } else {
                last
            },
            ..Ret::default()
        });
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c5_000c);
        e.register(0x00c5_000c, |_, _| Ret {
            st0: 6.0,
            ..Ret::default()
        });
        e.register(INTERFACE_VALUE_007DDEF0, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        stub(&mut e, &[ANSWER_THIS]);
        e.call(0x007f_99d0, &args![this]);
        let me_addr = this.addr();
        let positions: Vec<f32> = (0..5).map(|i| e.mem.f32(me_addr + 0x130 + 4 * i)).collect();
        assert_eq!(positions, vec![10.0, 15.0, 20.0, 25.0, 30.0]);
        assert_eq!(e.mem.f32(me_addr + 0x114), 10.0);
        assert_eq!(e.mem.f32(me_addr + 0x120), 10.0);
        // 10 - (6 / 3) * 2 = 6.
        assert_eq!(e.mem.f32(me_addr + 0x11c), 6.0);
        assert_eq!(e.mem.f32(me_addr + 0x110), 6.0);
    }

    #[test]
    fn init_knobs_turns_the_knobs() {
        let mut e = Engine::new();
        player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        e.set_global(PLAYER, 0u32);
        let this = e.new_object::<FOPipboyManager>();
        let tab = object_with_vtable(&mut e, 0x10, 0x00c6_0000);
        e.register_double(0x00c1_009c, move |e, a| Ret {
            eax: match e.mem.u32(a[1]) {
                0x0107_8254 => tab,
                0x0107_8238 => 0x904,
                0x0107_8224 => 0x908,
                _ => 0,
            },
            ..Ret::default()
        });
        answer(&mut e, 0x00c6_000c, 0x600);
        answer(&mut e, PROPERTY_AT, 0x900);
        let zero = e.mem.alloc(4);
        answer(&mut e, SETTING_VALUE_ADDRESS, zero);
        let rotation = e.mem.alloc(0x24);
        e.mem.set_f32(rotation, 1.0);
        answer(&mut e, LOCAL_ROTATION, rotation);
        answer(&mut e, PARENT_OF, 0x980);
        e.register(MATRIX_MULTIPLY, |e, a| {
            assert_eq!(e.mem.f32(a[0]), 1.0);
            assert_eq!(e.mem.f32(a[2]), 1.0);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        stub(&mut e, &[ANSWER_THIS, SET_ROTATION, SET_AXIS_ANGLE]);
        e.call_log = Some(vec![]);
        e.call(0x007f_99d0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0x100), 0x900);
        assert_eq!(e.mem.u32(me_addr + 0x104), 0x904);
        assert_eq!(e.mem.u32(me_addr + 0x108), 0x908);
        assert_eq!(calls_to(&e, SET_ROTATION).len(), 1);
        assert_eq!(calls_to(&e, SET_ROTATION)[0][0], 0x980);
        assert_eq!(
            calls_to(&e, SET_AXIS_ANGLE),
            vec![
                vec![0x900, 0, 1.0f32.to_bits(), 0, 0],
                vec![0x904, 0, 0, 0, 1.0f32.to_bits()],
                vec![0x908, 0, 0, 0, 1.0f32.to_bits()]
            ]
        );
    }

    #[test]
    fn initialize_stops_when_disabled() {
        let mut e = Engine::new();
        let flag = e.mem.alloc(4);
        answer(&mut e, SETTING_BOOL_ADDRESS, flag);
        let this = e.new_object::<FOPipboyManager>();
        e.call_log = Some(vec![]);
        e.call(0x007f_8ba0, &args![this]);
        assert_eq!(
            calls_to(&e, SETTING_BOOL_ADDRESS),
            vec![vec![SETTING_ENABLED]]
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn initialize_builds_the_pipboy() {
        let mut e = Engine::new();
        let player = player_with_root(&mut e);
        e.map(0x0101_d000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0106_4000, 0x1000);
        e.set_global(TAB_STEPS, 4.0f64);
        let owner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xa4, owner_vtable);
        e.mem.set_u32(owner_vtable + 0xc, 0x00c5_000c);
        stub(&mut e, &[0x00c5_000c, INTERFACE_VALUE_007DDEF0]);
        // The roots answer names through the same slot double.
        let root = object_with_vtable(&mut e, 0x10, 0x00c1_0000);
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, 1);
        answer(&mut e, SETTING_BOOL_ADDRESS, flag);
        answer(&mut e, PIPBOY_STATE, 0);
        let this = e.new_object::<FOPipboyManager>();
        // The Pip-Boy node is found under its first name.
        e.register(0x00c1_009c, |e, a| Ret {
            eax: if e.mem.u32(a[1]) == PIPBOY_NAME_ONE {
                0x700
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(NI_CAST, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        answer(&mut e, CHILD_AT, 0x710);
        answer(&mut e, PARENT_OF, root);
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 0.5);
        answer(&mut e, SETTING_VALUE_ADDRESS, setting);
        answer(&mut e, SYSTEM_COLOR, 0);
        answer(&mut e, RENDERED_MENU_CREATE_LIGHT, 1);
        answer(&mut e, MENU_LIGHT, 0);
        stub(
            &mut e,
            &[
                RENDERED_MENU_INITIALIZE,
                INTERFACE_SINGLETON,
                INTERFACE_OBJECT_007FA950,
                SET_MEMBER_0X14,
                USE_PLAYER_ROOT,
                RENDERED_MENU_PREPARE_GEOMETRY,
                ANSWER_THIS,
                UPDATE_DATA_CONSTRUCT,
                NODE_UPDATE,
                UPDATE_PROPERTIES,
                COLOR_CONSTRUCT,
                SYSTEM_COLORS,
                SETTER_00444020,
                SET_FLAG_00810530,
                SET_SCREEN_LIGHT_BASE,
                SET_CULLED,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x007f_8ba0, &args![this]);
        let me_addr = this.addr();
        assert_eq!(e.mem.u32(me_addr + 0xd8), root);
        assert_eq!(e.mem.u32(me_addr + 0xdc), root);
        assert_eq!(e.mem.u32(me_addr + 0xe0), 0x710);
        assert_eq!(
            e.mem.u32(me_addr + 4),
            0x710,
            "the screen stored by 007f9010"
        );
        assert_eq!(
            calls_to(&e, RENDERED_MENU_PREPARE_GEOMETRY),
            vec![vec![me_addr, 0x710], vec![me_addr, 0x710]]
        );
        assert_eq!(calls_to(&e, NODE_UPDATE).len(), 2);
        assert_eq!(
            e.get(this, FOPipboyManager::fBlurIntensity),
            0.5f32.to_bits()
        );
        assert_eq!(e.get(this, FOPipboyManager::bDoScanlines), 1);
        assert_eq!(calls_to(&e, SET_FLAG_00810530), vec![vec![me_addr, 1]]);
    }
}
