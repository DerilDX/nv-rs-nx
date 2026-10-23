//! `fallout/interface/menus/sleepwaitmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The menu's six tile slots start at +0x28 (one word each).
const TILES: u32 = 0x28;

// Translated from 007a8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::AttachTileByID` (Xbox PDB): stores `tile` in slot `id`
/// of the six tile slots at +0x28; ids outside 0..6 (signed) are ignored.
pub fn sleep_wait_menu_attach_tile_by_id(e: &mut Engine, this: Ptr, id: i32, tile: u32) {
    if (0..6).contains(&id) {
        let slot = this
            .addr()
            .wrapping_add(TILES)
            .wrapping_add((id as u32).wrapping_mul(4));
        e.mem.set_u32(slot, tile);
    }
}

/// The object in `011dea0c` whose `00877720` (thiscall, no stack words) answers the controls
/// object, and the controls' state query (`00a24660`, thiscall, two words: control and mode;
/// mode 2 "went up", 1 "went down").
const CONTROLS_HOLDER: u32 = 0x011d_ea0c;
const CONTROLS_OF: u32 = 0x0087_7720;
const CONTROL_STATE: u32 = 0x00a2_4660;
/// The control both queries ask about (0xF).
const CONTROL_0F: u32 = 0xf;
/// The player (`PlayerCharacter *`, `011dea3c`), `PlayerCharacter::IsSleepingorResting`
/// (engine map, `0094df60`), `004d1360` (thiscall: a byte), `PlayerCharacter::
/// UpdateHardcoreMode` (engine map, `00969c30`), `0094df80` and `005cde20` (thiscall: a
/// word); none takes stack words.
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
const PLAYER_TEST_004D1360: u32 = 0x004d_1360;
const PLAYER_UPDATE_HARDCORE_MODE: u32 = 0x0096_9c30;
const PLAYER_STEP_0094DF80: u32 = 0x0094_df80;
const PLAYER_VALUE_005CDE20: u32 = 0x005c_de20;
/// `Interface::IsConsoleVisible` (engine map, `00703d50`; no arguments).
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
/// The frame timer (`011f6394`) and its elapsed seconds (`0084d030`, thiscall, `ST0`).
const FRAME_TIMER: u32 = 0x011f_6394;
const ELAPSED_SECONDS: u32 = 0x0084_d030;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// The object in `011dea10` and its `00455490` (thiscall, no stack words).
const OBJECT_011DEA10: u32 = 0x011d_ea10;
const OBJECT_STEP_00455490: u32 = 0x0045_5490;
/// `Tile::TextToTrait` (engine map, `00a01860`; cdecl, one word), the trait setter from an
/// integer (`00700320`, thiscall on a tile: trait, value) and `Tile::Poke` (engine map,
/// `00a01290`, thiscall on a tile: trait, `float`).
const TEXT_TO_TRAIT: u32 = 0x00a0_1860;
const SET_TRAIT_FROM_INT: u32 = 0x0070_0320;
const TILE_POKE: u32 = 0x00a0_1290;
/// `"_current_value"` and `"_SetInCode"`.
const CURRENT_VALUE_TEXT: u32 = 0x0107_02d8;
const SET_IN_CODE_TEXT: u32 = 0x0107_0250;
/// `SleepWaitMenu::SetTimeText` (engine map, `007c0000`; thiscall, no stack words).
const SET_TIME_TEXT: u32 = 0x007c_0000;
/// `pTiles[1]`, `fElapsedSecs`, `bRestingActive` and `bButtonReleased` (Xbox PDB offsets,
/// matching the PC code).
const TILE_1: u32 = 0x2c;
const ELAPSED_SECS: u32 = 0x40;
const RESTING_ACTIVE: u32 = 0x44;
const BUTTON_RELEASED: u32 = 0x46;

/// The state of control 0xF in `mode` (`00877720` on `011dea0c`'s object, then `00a24660`).
fn control_state(e: &mut Engine, mode: u32) -> u32 {
    let holder: u32 = e.global(CONTROLS_HOLDER);
    let controls = e.call(CONTROLS_OF, &args![holder]).u32();
    e.call(CONTROL_STATE, &args![controls, CONTROL_0F, mode])
        .u32()
}

// Translated from 007c0580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SleepWaitMenu::UpdateSleeping` (engine map; `ECX` only, the answer in `AL`). Control 0xF
/// going up sets `bButtonReleased`. When that is set and the control then goes down, or
/// when resting is active and the player is no longer sleeping or resting, the menu closes
/// unless the console is open: its vtable +0xC with (5, 0), answer false. Otherwise, while
/// resting is active, the frame's seconds (`0084d030` on `011f6394`) are added to
/// `fElapsedSecs`; once that reaches 1.0 (ordered) it restarts at 0, an hour passes
/// (`00455490` on `011dea10`'s object; `UpdateHardcoreMode` when the player's `004d1360`
/// answers true; the player's `0094df80`), the tile `pTiles[1]` gets `_current_value` =
/// the player's `005cde20` minus 1 and `_SetInCode` = 1.0, the time text is updated and the
/// answer is true. Every other path answers false.
pub fn sleep_wait_menu_update_sleeping(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if control_state(e, 2) != 0 {
        e.mem.set_u8(base + BUTTON_RELEASED, 1);
    }
    let close = if e.mem.u8(base + BUTTON_RELEASED) != 0 && control_state(e, 1) != 0 {
        true
    } else if e.mem.u8(base + RESTING_ACTIVE) == 0 {
        false
    } else {
        let player: u32 = e.global(PLAYER);
        !e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool()
    };
    if close && !e.call(IS_CONSOLE_VISIBLE, &args![]).bool() {
        e.vcall(base, 0xc, &args![5u32, 0u32]);
        return false;
    }
    if e.mem.u8(base + RESTING_ACTIVE) == 0 {
        return false;
    }
    let seconds = e.call(ELAPSED_SECONDS, &args![FRAME_TIMER]).st0;
    let sum = x87_add(seconds, x87_m32(e.mem.u32(base + ELAPSED_SECS)));
    e.mem.set_u32(base + ELAPSED_SECS, x87_store32(sum));
    let one: f64 = e.global(ONE);
    // FCOMP; TEST AH,1: goes on only when not below (C0 clear: ordered and >=).
    if !(x87_load32(e.mem.u32(base + ELAPSED_SECS)) >= one) {
        return false;
    }
    e.mem.set_u32(base + ELAPSED_SECS, 0);
    let object: u32 = e.global(OBJECT_011DEA10);
    e.call(OBJECT_STEP_00455490, &args![object]);
    let player: u32 = e.global(PLAYER);
    if e.call(PLAYER_TEST_004D1360, &args![player]).bool() {
        let player: u32 = e.global(PLAYER);
        e.call(PLAYER_UPDATE_HARDCORE_MODE, &args![player]);
    }
    let player: u32 = e.global(PLAYER);
    e.call(PLAYER_STEP_0094DF80, &args![player]);
    let player: u32 = e.global(PLAYER);
    let value = e
        .call(PLAYER_VALUE_005CDE20, &args![player])
        .u32()
        .wrapping_sub(1);
    let trait_id = e.call(TEXT_TO_TRAIT, &args![CURRENT_VALUE_TEXT]).u32();
    let tile = e.mem.u32(base + TILE_1);
    e.call(SET_TRAIT_FROM_INT, &args![tile, trait_id, value]);
    let trait_id = e.call(TEXT_TO_TRAIT, &args![SET_IN_CODE_TEXT]).u32();
    let tile = e.mem.u32(base + TILE_1);
    e.call(TILE_POKE, &args![tile, trait_id, 1.0f32]);
    e.call(SET_TIME_TEXT, &args![this]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x007a_8ab0,
            sleep_wait_menu_attach_tile_by_id(Ptr, i32, u32)
        ),
        entry!(0x007c_0580, sleep_wait_menu_update_sleeping(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attaches_only_ids_zero_to_five() {
        let mut e = Engine::new();
        let menu = e.mem.alloc(0x48);
        e.call(0x007a_8ab0, &args![menu, 0i32, 0x11u32]);
        e.call(0x007a_8ab0, &args![menu, 5i32, 0x55u32]);
        e.call(0x007a_8ab0, &args![menu, 6i32, 0x66u32]);
        e.call(0x007a_8ab0, &args![menu, -1i32, 0x99u32]);
        assert_eq!(e.mem.u32(menu + 0x28), 0x11);
        assert_eq!(e.mem.u32(menu + 0x3c), 0x55);
        assert_eq!(e.mem.u32(menu + 0x40), 0);
        assert_eq!(e.mem.u32(menu + 0x24), 0);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The menu's close slot (vtable +0xC) as the tests place it.
    const CLOSE_SLOT: u32 = 0x0090_000c;

    struct Rig {
        e: Engine,
        menu: u32,
    }

    /// `went_up` and `went_down` are the control state answers for modes 2 and 1,
    /// `sleeping` the player's sleeping-or-resting answer, `console` whether the console is
    /// open, `seconds` the frame's elapsed seconds.
    fn rig(went_up: u32, went_down: u32, sleeping: bool, console: bool, seconds: f64) -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.set_global(CONTROLS_HOLDER, 0x0c00_0000u32);
        e.set_global(PLAYER, 0x0c10_0000u32);
        e.set_global(OBJECT_011DEA10, 0x0c20_0000u32);
        e.register(CONTROLS_OF, |_, _| 0x0c30_0000u32.into_ret());
        e.register_double(CONTROL_STATE, move |_, a| {
            assert_eq!(a[..2], [0x0c30_0000, CONTROL_0F]);
            (if a[2] == 2 { went_up } else { went_down }).into_ret()
        });
        e.register_double(PLAYER_IS_SLEEPING_OR_RESTING, move |_, _| {
            u32::from(sleeping).into_ret()
        });
        e.register_double(IS_CONSOLE_VISIBLE, move |_, _| {
            u32::from(console).into_ret()
        });
        e.register_double(ELAPSED_SECONDS, move |_, a| {
            assert_eq!(a[0], FRAME_TIMER);
            seconds.into_ret()
        });
        e.register(PLAYER_TEST_004D1360, |_, _| 1u32.into_ret());
        e.register(PLAYER_VALUE_005CDE20, |_, _| 8u32.into_ret());
        e.register(TEXT_TO_TRAIT, |_, a| (a[0] & 0xffff).into_ret());
        for callee in [
            CLOSE_SLOT,
            OBJECT_STEP_00455490,
            PLAYER_UPDATE_HARDCORE_MODE,
            PLAYER_STEP_0094DF80,
            SET_TRAIT_FROM_INT,
            TILE_POKE,
            SET_TIME_TEXT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_1000, &[0, 0, 0, CLOSE_SLOT]);
        let menu = e.mem.alloc(0x4c);
        e.mem.set_u32(menu, 0x0090_1000);
        e.mem.set_u32(menu + TILE_1, 0x0d00_0000);
        Rig { e, menu }
    }

    fn update(rig: &mut Rig) -> (bool, Vec<(u32, Vec<u32>)>) {
        rig.e.call_log = Some(vec![]);
        let answer = rig.e.call(0x007c_0580, &args![rig.menu]).bool();
        (answer, rig.e.call_log.take().unwrap())
    }

    fn called(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn released_then_pressed_closes_the_menu() {
        let mut r = rig(1, 1, true, false, 0.0);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(r.e.mem.u8(r.menu + BUTTON_RELEASED), 1);
        assert_eq!(called(&log, CLOSE_SLOT), vec![vec![r.menu, 5, 0]]);
        assert!(called(&log, ELAPSED_SECONDS).is_empty());
    }

    #[test]
    fn the_console_keeps_the_menu_open() {
        let mut r = rig(1, 1, true, true, 0.25);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert!(called(&log, CLOSE_SLOT).is_empty());
        // Not resting: nothing more.
        assert!(called(&log, ELAPSED_SECONDS).is_empty());
    }

    #[test]
    fn resting_stops_when_the_player_no_longer_sleeps() {
        let mut r = rig(0, 0, false, false, 0.25);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(called(&log, CLOSE_SLOT).len(), 1);
        // Without the release, the went-down state is not asked for.
        let modes: Vec<u32> = called(&log, CONTROL_STATE).iter().map(|w| w[2]).collect();
        assert_eq!(modes, vec![2]);
    }

    #[test]
    fn resting_accumulates_below_one_second() {
        let mut r = rig(0, 0, true, false, 0.25);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        r.e.mem.set_f32(r.menu + ELAPSED_SECS, 0.5);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert_eq!(r.e.mem.f32(r.menu + ELAPSED_SECS), 0.75);
        assert!(called(&log, OBJECT_STEP_00455490).is_empty());
        assert!(called(&log, CLOSE_SLOT).is_empty());
    }

    #[test]
    fn a_full_second_passes_an_hour() {
        let mut r = rig(0, 0, true, false, 0.5);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        r.e.mem.set_f32(r.menu + ELAPSED_SECS, 0.5);
        let (answer, log) = update(&mut r);
        assert!(answer);
        assert_eq!(r.e.mem.f32(r.menu + ELAPSED_SECS), 0.0);
        assert_eq!(called(&log, OBJECT_STEP_00455490), vec![vec![0x0c20_0000]]);
        assert_eq!(
            called(&log, PLAYER_UPDATE_HARDCORE_MODE),
            vec![vec![0x0c10_0000]]
        );
        assert_eq!(called(&log, PLAYER_STEP_0094DF80), vec![vec![0x0c10_0000]]);
        assert_eq!(
            called(&log, SET_TRAIT_FROM_INT),
            vec![vec![0x0d00_0000, CURRENT_VALUE_TEXT & 0xffff, 7]]
        );
        assert_eq!(
            called(&log, TILE_POKE),
            vec![vec![
                0x0d00_0000,
                SET_IN_CODE_TEXT & 0xffff,
                1.0f32.to_bits()
            ]]
        );
        assert_eq!(called(&log, SET_TIME_TEXT), vec![vec![r.menu]]);
    }

    #[test]
    fn a_nan_sum_does_not_pass_an_hour() {
        let mut r = rig(0, 0, true, false, f64::NAN);
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let (answer, log) = update(&mut r);
        assert!(!answer);
        assert!(r.e.mem.f32(r.menu + ELAPSED_SECS).is_nan());
        assert!(called(&log, OBJECT_STEP_00455490).is_empty());
    }

    #[test]
    fn without_the_hardcore_byte_the_mode_is_not_updated() {
        let mut r = rig(0, 0, true, false, 1.0);
        r.e.register(PLAYER_TEST_004D1360, |_, _| 0x100u32.into_ret());
        r.e.mem.set_u8(r.menu + RESTING_ACTIVE, 1);
        let seen = Rc::new(RefCell::new(0u32));
        let count = seen.clone();
        r.e.register_double(PLAYER_UPDATE_HARDCORE_MODE, move |_, _| {
            *count.borrow_mut() += 1;
            Ret::default()
        });
        let (answer, _) = update(&mut r);
        assert!(answer);
        assert_eq!(*seen.borrow(), 0);
    }
}
