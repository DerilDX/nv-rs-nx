//! `fallout/interface/menus/hudmainmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `HUDMainMenu` (Xbox PDB), 0x278 bytes. The offsets used here are
    /// confirmed by the PC instructions in this unit.
    pub struct HUDMainMenu: 0x278 {
        0x028 pTiles: u32,
        0x184 pActive: Ptr,
        0x188 fSeenLevel: f32,
        0x194 uiMessageStartTime: u32,
        0x1a8 pCurrSubtitle: Ptr,
        0x1b8 pLastObject: Ptr,
        0x1bc iHUDVisibilityMenuModeFlags: i32,
        0x1c0 iHUDVisibilityUserSettings: i32,
        0x1c4 eMenuMode: i32,
        0x1fc bScopeVisible: bool,
        0x204 bXPMeterFinishedMoving: bool,
        0x205 bForceObjectiveReminders: bool,
        0x208 uiKillRegionDisplay: u32,
        0x20c pQuestList: Ptr,
        0x210 iCompassImageWidth: i32,
        0x214 iCompassDisplayWidth: i32,
        0x218 iCompassHalfAngle: i32,
        0x21c pCurrentEnemy: Ptr,
        0x220 bWasSneaking: bool,
        0x244 iEnemyHealthTimer: u32,
        0x248 uiQuestReminderTimer: u32,
        0x24c uiDDTdisplayTimer: i32,
        0x250 uiEnemyDDTdisplayTimer: i32,
        0x254 bKillQuestUpdates: bool,
        0x260 bQuestNameShowing: bool,
        0x261 bObjectiveNameShowing: bool,
        0x274 fShake: f32,
    }
}

fn tile(this: Ptr<HUDMainMenu>, id: u32) -> u32 {
    this.addr() + 0x28 + id * 4
}

fn current_hud(e: &Engine) -> u32 {
    e.mem.u32(0x011d_96c0)
}

fn hud_field(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(current_hud(e) + offset)
}

fn set_hud_field(e: &mut Engine, offset: u32, value: u32) {
    let menu = current_hud(e);
    e.mem.set_u32(menu + offset, value);
}

fn create_lookup(e: &mut Engine, offset: u32, name: u32) -> u32 {
    let parent = e.call(0x00a0_9030, &args![0x3ec_u32]).u32();
    let found = e.call(0x00a0_8b20, &args![parent, name]).u32();
    set_hud_field(e, offset, found);
    found
}

fn create_child(e: &mut Engine, offset: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    let found = e.call(0x00a0_8b20, &args![parent, name]).u32();
    set_hud_field(e, offset, found);
    found
}

fn create_render(e: &mut Engine, menu: u32, offset: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    let rendered = e.call(0x00a1_ddb0, &args![menu, parent, name, 0_u32]).u32();
    set_hud_field(e, offset, rendered);
    rendered
}

fn create_render_discard(e: &mut Engine, menu: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    e.call(0x00a1_ddb0, &args![menu, parent, name, 0_u32]).u32()
}

fn create_brightness(e: &mut Engine, offset: u32, level: u32) {
    let current_tile = hud_field(e, offset);
    hud_main_menu_set_brightness(e, Ptr::new(current_tile), level);
}

fn create_trait(e: &mut Engine, offset: u32, trait_id: u32, value: u32) {
    let current_tile = hud_field(e, offset);
    hud_trait(e, current_tile, trait_id, value);
}

fn create_float(e: &mut Engine, offset: u32, trait_id: u32, value: f32) {
    let current_tile = hud_field(e, offset);
    hud_set_float(e, current_tile, trait_id, value);
}

fn create_get_float(e: &mut Engine, offset: u32, trait_id: u32) -> f64 {
    let current_tile = hud_field(e, offset);
    e.call(0x00a0_11b0, &args![current_tile, trait_id]).f64()
}

#[derive(Clone, Copy)]
struct HudDimensions {
    screen_width: i32,
    screen_height: i32,
    bracket_width: i32,
    bracket_height: i32,
    original_x: u32,
    total_width: u32,
    image_width: u32,
}

fn create_action_points(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x114, 0x0106_7858);
    let x = (f64::from(d.screen_width.wrapping_sub(d.bracket_width.wrapping_mul(2)))
        - create_get_float(e, 0x114, 0xfb1)
        + e.global::<f64>(0x0101_db88)) as f32;
    create_float(e, 0x114, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x114, 0xfb0)) as f32;
    create_float(e, 0x114, 0xfa2, y);
    create_trait(e, 0x114, 0xfa8, 1);
    create_render(e, menu, 0xd4, 0x114, 0x0107_3400);
    create_brightness(e, 0xd4, 2);
    create_trait(e, 0xd4, 0xfa1, (-0x25_i32) as u32);
    create_render(e, menu, 0x44, 0x114, 0x0107_33f0);
    create_trait(e, 0x44, d.original_x, 0x146);
    let meter_height: f32 = e.global(0x0107_33ec);
    create_float(e, 0x44, 0xfa2, meter_height);
    create_brightness(e, 0x44, 3);
    create_render(e, menu, 0x48, 0x114, 0x0107_33d0);
    create_trait(e, 0x48, 0xfa1, 0x13b);
    create_trait(e, 0x48, 0xfa2, 8);
    create_brightness(e, 0x48, 0);
    create_render(e, menu, 0x58, 0x114, 0x0107_33d0);
    create_trait(e, 0x58, 0xfa1, 0x13b);
    create_trait(e, 0x58, 0xfa2, 0x50);
    create_trait(e, 0x58, 0xfb9, 7);
    create_brightness(e, 0x58, 0);
    create_render(e, menu, 0x4c, 0x114, 0x0107_33d0);
    create_trait(e, 0x4c, 0xfa1, 0x50);
    let y = create_get_float(e, 0x58, 0xfa2) as f32;
    create_float(e, 0x4c, 0xfa2, y);
    create_trait(e, 0x4c, 0xfb9, 7);
    let label = e.call(0x0040_3df0, &args![0x011d_4da4_u32]).u32();
    let current_tile = hud_field(e, 0x4c);
    hud_set_text(e, current_tile, label);
    create_brightness(e, 0x4c, 0);
    create_render(e, menu, 0x50, 0x114, 0x0107_33f0);
    create_trait(e, 0x50, d.original_x, 0x5a);
    let y = (create_get_float(e, 0x58, 0xfa2) + e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x50, 0xfa2, y);
    let current_tile = hud_field(e, 0x50);
    e.call(
        0x00a0_1350,
        &args![current_tile, 0xfcc_u32, 0x0107_33b4_u32, 1_u32],
    );
    create_trait(e, 0x50, d.total_width, 0x3c);
    create_trait(e, 0x50, d.image_width, 1);
    create_trait(e, 0x50, 0xfb8, 200);
    create_brightness(e, 0x50, 3);
    create_render(e, menu, 0x54, 0x114, 0x0107_3398);
    let y = (create_get_float(e, 0x58, 0xfa2) + e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x54, 0xfa2, y);
    create_brightness(e, 0x54, 4);
    create_lookup(e, 0x180, 0x0106_ed0c);
    let first = create_get_float(e, 0x114, 0xfa1);
    let second = create_get_float(e, 0x4c, 0xfa1);
    let third = create_get_float(e, 0x50, 0xfa1);
    let fourth = create_get_float(e, 0x54, 0xfb1);
    let x = (first
        + second
        + third
        + fourth * e.global::<f64>(0x0101_de30)
        + e.global::<f64>(0x0102_40b0)) as f32;
    create_float(e, 0x180, 0xfa1, x);
    let y = (create_get_float(e, 0x114, 0xfa2) + create_get_float(e, 0x50, 0xfa2)) as f32;
    create_float(e, 0x180, 0xfa2, y);
    let height = create_get_float(e, 0x54, 0xfb0) as f32;
    create_float(e, 0x180, 0xfb0, height);
    create_trait(e, 0x180, 0xfa3, 0);
}

fn create_hit_points(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x118, 0x0107_338c);
    create_trait(
        e,
        0x118,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(10) as u32,
    );
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x118, 0xfb0)) as f32;
    create_float(e, 0x118, 0xfa2, y);
    create_trait(e, 0x118, 0xfa8, 1);
    create_render(e, menu, 0xd0, 0x118, 0x0107_3374);
    create_brightness(e, 0xd0, 2);
    create_render(e, menu, 0x2c, 0x118, 0x0107_33f0);
    create_trait(e, 0x2c, d.original_x, 0x14);
    let meter_height: f32 = e.global(0x0107_33ec);
    create_float(e, 0x2c, 0xfa2, meter_height);
    create_brightness(e, 0x2c, 3);
    create_render(e, menu, 0x30, 0x118, 0x0107_33d0);
    create_trait(e, 0x30, 0xfa1, 0x35);
    create_trait(e, 0x30, 0xfa2, 8);
    create_brightness(e, 0x30, 0);
    create_render(e, menu, 0x40, 0x118, 0x0107_335c);
    create_render(e, menu, 0x34, 0x40, 0x0107_3340);
    create_trait(e, 0x34, 0xfa2, 0x1e);
    create_trait(e, 0x34, 0xfad, 4);
    create_render(e, menu, 0x38, 0x40, 0x0107_3340);
    create_trait(e, 0x38, 0xfa2, 0x1e);
    create_trait(e, 0x38, 0xfad, 3);
    create_render(e, menu, 0x3c, 0x40, 0x0107_3340);
    create_trait(e, 0x3c, 0xfa2, 10);
    create_trait(e, 0x3c, 0xfad, 2);
    let mut index = 0_i32;
    loop {
        let setting = e.call(0x0043_d4d0, &args![0x011d_32d0_u32]).u32();
        if index >= e.mem.i32(setting) {
            break;
        }
        create_render_discard(e, menu, 0x3c, 0x0107_332c);
        index = index.wrapping_add(1);
    }
    for _ in 0..10 {
        create_render_discard(e, menu, 0x38, 0x0107_3310);
    }
    create_render_discard(e, menu, 0x38, 0x0107_32f0);
    for _ in 0..10 {
        create_render_discard(e, menu, 0x34, 0x0107_32d0);
    }
}

fn create_radiation_enemy_quest(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x11c, 0x0107_32c0);
    let x = (f64::from(d.screen_width.wrapping_sub(d.bracket_width.wrapping_mul(2)))
        - create_get_float(e, 0x11c, 0xfb1)
        - e.global::<f64>(0x0102_fc70)) as f32;
    create_float(e, 0x11c, 0xfa1, x);
    create_trait(e, 0x11c, 0xfa2, d.bracket_height.wrapping_mul(2) as u32);
    create_child(e, 0x6c, 0x11c, 0x0107_32ac);
    create_child(e, 0x70, 0x11c, 0x0107_3298);
    create_child(e, 0x78, 0x11c, 0x0107_3288);
    create_child(e, 0x74, 0x11c, 0x0107_3270);
    let current_tile = hud_field(e, 0x74);
    hud_set_text(e, current_tile, 0x0107_326c);
    let text = e.call(0x0040_3df0, &args![0x011d_21d8_u32]).u32();
    let current_tile = hud_field(e, 0x78);
    hud_set_text(e, current_tile, text);

    create_lookup(e, 0x120, 0x0107_3260);
    let half_screen = d.screen_width / 2;
    let enemy_width = create_get_float(e, 0x120, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x = (f64::from(half_screen) - enemy_width / half) as f32;
    create_float(e, 0x120, 0xfa1, x);
    let y = (create_get_float(e, 0x118, 0xfa2) + e.global::<f64>(0x0103_5810)) as f32;
    create_float(e, 0x120, 0xfa2, y);
    create_trait(e, 0x120, 0xfa8, 1);
    create_trait(e, 0x120, 0xff4, 2);
    create_render(e, menu, 0x7c, 0x120, 0x0107_3240);
    create_brightness(e, 0x7c, 1);
    create_trait(e, 0x7c, 0xfa3, 1);
    create_render(e, menu, 0x80, 0x120, 0x0107_33f0);
    create_trait(e, 0x80, d.original_x, 0xab);
    create_trait(e, 0x80, d.total_width, 0xf0);
    create_trait(e, 0x80, 0xfa2, 3);
    create_brightness(e, 0x80, 3);
    create_trait(e, 0x80, 0xfa3, 1);
    create_render(e, menu, 0x84, 0x120, 0x0107_3220);
    create_trait(e, 0x84, 0xfa1, 0xaa);
    create_trait(e, 0x84, 0xfa2, 0x23);
    create_trait(e, 0x84, 0xfba, 0x15e);
    create_brightness(e, 0x84, 0);

    let quest_y = d.bracket_height.wrapping_mul(2).wrapping_add(0xaa);
    create_lookup(e, 0x124, 0x0107_3210);
    create_trait(
        e,
        0x124,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(0xf) as u32,
    );
    create_trait(e, 0x124, 0xfa2, quest_y as u32);
    create_child(e, 0x5c, 0x124, 0x0107_3204);
    create_child(e, 0x60, 0x124, 0x0107_31f8);
    for _ in 0..0x40 {
        create_render_discard(e, menu, 0x60, 0x0107_31dc);
    }
    for _ in 0..0x20 {
        create_render_discard(e, menu, 0x5c, 0x0107_31c0);
    }
    for _ in 0..0x20 {
        create_render_discard(e, menu, 0x5c, 0x0107_31dc);
    }
}

fn create_text_height(e: &mut Engine, text_offset: u32) -> f32 {
    let font_size = create_get_float(e, text_offset, 0xfba) as f32;
    let font_raw = create_get_float(e, text_offset, 0xfb9);
    let font = x87_to_int(e, font_raw);
    e.with_stack(12, |e, dimensions| {
        let manager = e.call(0x005b_d5b0, &args![]).u32();
        let measured = e
            .call(
                0x00a1_b020,
                &args![manager, dimensions, 0x0107_3198_u32, font, font_size, 0_u32],
            )
            .u32();
        e.mem.f32(measured + 4)
    })
}

fn create_region_reticle_sneak_messages(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x128, 0x0107_31b0);
    create_trait(
        e,
        0x128,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(0xf) as u32,
    );
    let y = (create_get_float(e, 0x118, 0xfa2) - e.global::<f64>(0x0107_31a8)) as f32;
    create_float(e, 0x128, 0xfa2, y);
    create_render(e, menu, 0x64, 0x128, 0x0107_31dc);
    create_brightness(e, 0x64, 0);
    create_trait(e, 0x64, 0xfa3, 1);
    let text_height = create_text_height(e, 0x64);
    create_float(e, 0x64, 0xfa2, quiet_f32(text_height));
    create_render(e, menu, 0x68, 0x128, 0x0107_31dc);
    create_brightness(e, 0x68, 0);
    create_trait(e, 0x68, 0xfa3, 1);
    create_float(e, 0x68, 0xfa2, quiet_f32(text_height));

    create_lookup(e, 0x12c, 0x0107_3188);
    let center_width = create_get_float(e, 0x12c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x =
        (f64::from(d.screen_width / 2) - center_width / half - e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x12c, 0xfa1, x);
    let center_height = create_get_float(e, 0x12c, 0xfb0);
    let half: f64 = e.global(0x0101_1590);
    let y = (f64::from(d.screen_height / 2) - center_height / half - e.global::<f64>(0x0102_1928))
        as f32;
    create_float(e, 0x12c, 0xfa2, y);
    create_trait(e, 0x12c, 0xfa8, 1);
    create_trait(e, 0x12c, 0xfa3, 1);
    create_render(e, menu, 0xf4, 0x12c, 0x0107_3170);
    create_trait(e, 0xf4, 0xff4, 1);
    create_trait(e, 0xf4, 0xfa3, 1);

    create_lookup(e, 0x130, 0x0107_3164);
    create_trait(e, 0x130, 0xfa1, (d.screen_width / 2) as u32);
    create_trait(e, 0x130, 0xfa2, d.bracket_height.wrapping_mul(2) as u32);
    create_trait(e, 0x130, 0xfa8, 1);
    create_trait(e, 0x130, 0xfa3, 0);
    create_child(e, 0x88, 0x130, 0x0107_3158);
    create_trait(e, 0x88, 0xfa9, 0);

    create_lookup(e, 0x134, 0x0107_314c);
    create_trait(
        e,
        0x134,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_sub(5) as u32,
    );
    create_trait(
        e,
        0x134,
        0xfa2,
        d.bracket_height.wrapping_mul(2).wrapping_add(0x19) as u32,
    );
    create_trait(e, 0x134, 0xfa8, 1);
    create_render(e, menu, 0x8c, 0x134, 0x0107_3134);
    create_brightness(e, 0x8c, 6);
    create_render(e, menu, 0x90, 0x134, 0x0107_31dc);
    create_brightness(e, 0x90, 0);
    for (trait_id, value) in [
        (0xfa1, 0x82),
        (0xfa2, 5),
        (0xfba, 0x140),
        (0xfad, 2),
        (0xfa9, 0),
        (0xfa3, 1),
    ] {
        create_trait(e, 0x90, trait_id, value);
    }
    create_render(e, menu, 0x94, 0x134, 0x0107_3118);
    create_brightness(e, 0x94, 1);
}

fn create_centered_text(e: &mut Engine, menu: u32, offset: u32, y: u32, label: Option<u32>) {
    create_render(e, menu, offset, 0x138, 0x0107_3220);
    create_brightness(e, offset, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, offset, 0xfa1, (width / half) as f32);
    let width = create_get_float(e, 0x138, 0xfb1) as f32;
    create_float(e, offset, 0xfba, width);
    create_trait(e, offset, 0xfa2, y);
    if let Some(label_setting) = label {
        create_trait(e, offset, 0xfa9, 0);
        let label = e.call(0x0040_3df0, &args![label_setting]).u32();
        let current_tile = hud_field(e, offset);
        hud_set_text(e, current_tile, label);
        create_trait(e, offset, 0xfa3, 0);
    }
}

fn create_info_subtitles(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x138, 0x0107_3110);
    let center_width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x = (f64::from(d.screen_width / 2) - center_width / half) as f32;
    create_float(e, 0x138, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x138, 0xfb0)
        - e.global::<f64>(0x0101_db88)) as f32;
    create_float(e, 0x138, 0xfa2, y);
    create_trait(e, 0x138, 0xfa8, 1);
    create_child(e, 0x9c, 0x138, 0x0107_30f8);
    create_child(e, 0xa0, 0x9c, 0x0106_f46c);
    create_child(e, 0xa4, 0x9c, 0x0106_f47c);
    create_brightness(e, 0x9c, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let half_width = (width / half) as f32;
    let trait_id = e.call(0x00a0_1860, &args![0x0106_ec90_u32]).u32();
    create_float(e, 0x9c, trait_id, half_width);

    create_centered_text(e, menu, 0xa8, 0, None);
    let height = create_text_height(e, 0xa8);
    create_float(e, 0xa8, 0x100c, quiet_f32(height));
    create_centered_text(e, menu, 0xac, 0x41, Some(0x011d_2454));
    create_centered_text(e, menu, 0xb0, 0x41, Some(0x011d_1f68));
    create_render(e, menu, 0xb4, 0x138, 0x0107_33d0);
    create_brightness(e, 0xb4, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xb4, 0xfa1, (width / half - 25.0) as f32);
    create_trait(e, 0xb4, 0xfa2, 0x3c);
    let alpha = quiet_f32(e.mem.f32(0x011d_979c));
    create_float(e, 0xb4, 0xfa9, alpha);
    create_render(e, menu, 0xb8, 0x138, 0x0107_31dc);
    create_brightness(e, 0xb8, 0);
    create_trait(e, 0xb8, 0xfa1, 10);
    create_trait(e, 0xb8, 0xfa2, 0x3c);
    create_render(e, menu, 0xbc, 0x138, 0x0107_33d0);
    create_brightness(e, 0xbc, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    create_float(e, 0xbc, 0xfa1, (width - 30.0) as f32);
    create_trait(e, 0xbc, 0xfa2, 0x3c);
    let alpha = quiet_f32(e.mem.f32(0x011d_979c));
    create_float(e, 0xbc, 0xfa9, alpha);
    create_render(e, menu, 0xc0, 0x138, 0x0107_31dc);
    create_brightness(e, 0xc0, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xc0, 0xfa1, (width / half + 5.0) as f32);
    create_trait(e, 0xc0, 0xfa2, 0x3c);
    create_render(e, menu, 0xc4, 0x138, 0x0107_30e0);
    create_brightness(e, 0xc4, 2);

    create_lookup(e, 0x13c, 0x0107_30d4);
    let width = create_get_float(e, 0x13c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(
        e,
        0x13c,
        0xfa1,
        (f64::from(d.screen_width / 2) - width / half) as f32,
    );
    let top = create_get_float(e, 0x13c, 0xfa2);
    let height = create_get_float(e, 0x13c, 0xfb0);
    create_float(e, 0x13c, 0xfa2, (top - height - 50.0) as f32);
    create_trait(e, 0x13c, 0xfa8, 1);
    create_render(e, menu, 0x98, 0x13c, 0x0107_3220);
    create_brightness(e, 0x98, 0);
    let width = create_get_float(e, 0x13c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0x98, 0xfa1, (width / half) as f32);
    let width = create_get_float(e, 0x13c, 0xfb1) as f32;
    create_float(e, 0x98, 0xfba, width);
}

fn create_hotkeys_xp_breath_explosives(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x140, 0x0107_30cc);
    let x = (create_get_float(e, 0x114, 0xfa1) + e.global::<f64>(0x0101_5a38)) as f32;
    create_float(e, 0x140, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x118, 0xfb0)
        - create_get_float(e, 0x140, 0xfb0)
        - e.global::<f64>(0x0102_fc70)) as f32;
    create_float(e, 0x140, 0xfa2, y);
    create_trait(e, 0x140, 0xfa8, 1);
    create_render(e, menu, 0xc8, 0x140, 0x0107_30b0);
    let current_menu = current_hud(e);
    let selector = hud_field(e, 0xc8);
    e.call(0x0070_1610, &args![current_menu + 0x1c8, selector]);
    create_render(e, menu, 0xcc, 0x140, 0x0107_3220);
    let width = create_get_float(e, 0x140, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xcc, 0xfa1, (width / half) as f32);
    create_trait(e, 0xcc, 0xfa2, (-0x32_i32) as u32);
    create_trait(e, 0xcc, 0xfa3, 0);

    create_lookup(e, 0x144, 0x0107_30a8);
    let x = d
        .screen_width
        .wrapping_sub(d.bracket_width.wrapping_mul(2))
        .wrapping_sub(0x186);
    let y = d
        .screen_height
        .wrapping_sub(d.bracket_height.wrapping_mul(2))
        .wrapping_sub(0x118);
    create_trait(e, 0x144, 0xfa1, x as u32);
    create_trait(e, 0x144, 0xfa2, y as u32);
    create_trait(e, 0x144, 0x1007, y as u32);
    for (offset, name) in [
        (0xdc, 0x0107_309c),
        (0xe0, 0x0107_3094),
        (0xe4, 0x0107_3088),
        (0xd8, 0x0107_307c),
        (0xe8, 0x0107_3070),
        (0xec, 0x0107_3064),
        (0xf0, 0x0107_3058),
    ] {
        create_child(e, offset, 0x144, name);
    }

    create_lookup(e, 0x148, 0x0107_304c);
    create_trait(e, 0x148, 0xfa1, (d.screen_width / 2) as u32);
    let y = (create_get_float(e, 0x130, 0xfa2) + e.global::<f64>(0x0107_2310)) as f32;
    create_float(e, 0x148, 0xfa2, y);
    create_lookup(e, 0x14c, 0x0107_3030);
    create_trait(e, 0x14c, 0xfa1, (d.screen_width / 2) as u32);
    create_trait(e, 0x14c, 0xfa2, (d.screen_height / 2) as u32);
    for _ in 0..6 {
        create_render_discard(e, menu, 0x14c, 0x0107_3010);
    }
}

fn cripple_scale(e: &Engine, size: f32, numerator: u32) -> f32 {
    (f64::from(size) * e.global::<f64>(numerator) / e.global::<f64>(0x0101_7a40)) as f32
}

fn create_crippled_part(
    e: &mut Engine,
    offset: u32,
    name: u32,
    size: f32,
    width: u32,
    height: u32,
) {
    create_child(e, offset, 0x150, name);
    create_float(e, offset, 0xfb8, quiet_f32(size));
    let scaled_width = cripple_scale(e, size, width);
    create_float(e, offset, 0xfb1, scaled_width);
    let scaled_height = cripple_scale(e, size, height);
    create_float(e, offset, 0xfb0, scaled_height);
}

fn create_crippled_limbs(e: &mut Engine, d: HudDimensions) -> f32 {
    create_lookup(e, 0x150, 0x0107_2ff8);
    let size: f32 = quiet_f32(e.global(0x0102_f0f8));
    create_trait(
        e,
        0x150,
        0xfa1,
        d.screen_width
            .wrapping_sub(d.bracket_width.wrapping_mul(2))
            .wrapping_sub(200) as u32,
    );
    create_trait(
        e,
        0x150,
        0xfa2,
        d.bracket_height.wrapping_mul(2).wrapping_add(100) as u32,
    );

    create_crippled_part(e, 0xfc, 0x0101_f604, size, 0x0107_2ff0, 0x0107_2fe8);
    create_trait(e, 0xfc, 0xfa1, 0);
    create_trait(e, 0xfc, 0xfa2, 0x28);
    create_crippled_part(e, 0xf8, 0x0105_c2c8, size, 0x0107_2fe0, 0x0102_40c0);
    let head_x = create_get_float(e, 0xfc, 0xfa1);
    let head_width = create_get_float(e, 0xfc, 0xfb1);
    let torso_width = create_get_float(e, 0xf8, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(
        e,
        0xf8,
        0xfa1,
        (head_x + (head_width - torso_width) / half) as f32,
    );
    let head_y = create_get_float(e, 0xfc, 0xfa2);
    let head_height = create_get_float(e, 0xfc, 0xfb0);
    let torso_height = create_get_float(e, 0xf8, 0xfb0);
    let half: f64 = e.global(0x0101_1590);
    let offset = f64::from(cripple_scale(e, size, 0x0102_40b8));
    create_float(
        e,
        0xf8,
        0xfa2,
        (head_y + (head_height - torso_height) / half + offset) as f32,
    );

    create_crippled_part(e, 0x100, 0x0107_2fd8, size, 0x0107_04e8, 0x0102_e430);
    let x = create_get_float(e, 0xfc, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fd0));
    create_float(e, 0x100, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0xfc, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fc8));
    create_float(e, 0x100, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x104, 0x0107_2fb8, size, 0x0101_7a40, 0x0107_2fb0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0106_6808));
    create_float(e, 0x104, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2) as f32;
    create_float(e, 0x104, 0xfa2, y);

    create_crippled_part(e, 0x108, 0x0107_2fa0, size, 0x0107_2f98, 0x0107_2f90);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f88));
    create_float(e, 0x108, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f80));
    create_float(e, 0x108, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x10c, 0x0107_2f70, size, 0x0107_2f68, 0x0106_ebd0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0103_57a8));
    create_float(e, 0x10c, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fc8));
    create_float(e, 0x10c, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x110, 0x0107_2f58, size, 0x0107_2f50, 0x0106_ebd0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f48));
    create_float(e, 0x110, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f40));
    create_float(e, 0x110, 0xfa2, (y + shift) as f32);
    size
}

fn create_ddt_and_hardcore(e: &mut Engine, icon_size: f32) {
    for (offset, name, visible) in [
        (0x154, 0x0107_2f38, 0_u32),
        (0x158, 0x0107_2f28, 1),
        (0x170, 0x0107_2f38, 1),
        (0x174, 0x0107_2f18, 1),
    ] {
        create_lookup(e, offset, name);
        if matches!(offset, 0x154 | 0x170) {
            let first_x = create_get_float(e, 0x118, 0xfa1);
            let second_x = create_get_float(e, 0x118, 0xfb1);
            let shift: f64 = e.global(0x0102_0998);
            create_float(e, offset, 0xfa1, (first_x + second_x + shift) as f32);
            let first_y = create_get_float(e, 0x114, 0xfa2);
            let second_y = create_get_float(e, 0x40, 0xfa2);
            let shift: f64 = e.global(0x0102_40b8);
            create_float(e, offset, 0xfa2, (first_y + second_y - shift) as f32);
        } else {
            let first_x = create_get_float(e, 0x120, 0xfa1);
            let second_x = create_get_float(e, 0x80, 0xfa1);
            let shift: f64 = e.global(0x0102_f070);
            create_float(e, offset, 0xfa1, (first_x + second_x - shift) as f32);
            let first_y = create_get_float(e, 0x120, 0xfa2);
            let second_y = create_get_float(e, 0x80, 0xfa2);
            create_float(e, offset, 0xfa2, (first_y + second_y) as f32);
        }
        create_brightness(e, offset, 0);
        create_trait(e, offset, 0xfa3, visible);
        if matches!(offset, 0x158 | 0x174) {
            create_trait(e, offset, 0xff4, 2);
        }
    }
    create_lookup(e, 0x15c, 0x0107_2f08);
    let first_x = create_get_float(e, 0x114, 0xfa1);
    let second_x = create_get_float(e, 0x58, 0xfa1);
    let shift: f64 = e.global(0x0107_2f00);
    create_float(e, 0x15c, 0xfa1, (first_x + second_x - shift) as f32);
    let first_y = create_get_float(e, 0x114, 0xfa2);
    let second_y = create_get_float(e, 0x58, 0xfa2);
    create_float(e, 0x15c, 0xfa2, (first_y + second_y) as f32);
    create_brightness(e, 0x15c, 0);
    create_trait(e, 0x15c, 0xfa3, 1);

    create_lookup(e, 0x160, 0x0107_2eec);
    let x = (create_get_float(e, 0x118, 0xfa1) + e.global::<f64>(0x0106_c948)) as f32;
    create_float(e, 0x160, 0xfa1, x);
    let y = create_get_float(e, 0x118, 0xfa2) as f32;
    create_float(e, 0x160, 0xfa2, y);
    create_brightness(e, 0x160, 0);
    create_trait(e, 0x160, 0xfa3, 1);
    create_trait(e, 0x160, 0xfa8, 1);
    create_child(e, 0x164, 0x160, 0x0106_75c0);
    create_trait(e, 0x164, 0xfa1, 0);
    create_trait(e, 0x164, 0xfa2, 0);
    create_float(e, 0x164, 0xfb8, quiet_f32(icon_size));
    create_trait(e, 0x164, 0xfa9, 0);
    for (offset, name, predecessors) in [
        (0x16c, 0x0106_75b8, &[0x164_u32][..]),
        (0x168, 0x0106_37d4, &[0x16c, 0x164][..]),
        (0x178, 0x0105_afd0, &[0x16c, 0x164, 0x168][..]),
        (0x17c, 0x0107_2ee4, &[0x16c, 0x164, 0x168, 0x178][..]),
    ] {
        create_child(e, offset, 0x160, name);
        let mut x = 0.0;
        for &previous in predecessors {
            x += create_get_float(e, previous, 0xfb1);
            x += e.global::<f64>(0x0102_0758);
        }
        create_float(e, offset, 0xfa1, x as f32);
        create_trait(e, offset, 0xfa2, 0);
        create_float(e, offset, 0xfb8, quiet_f32(icon_size));
        create_trait(e, offset, 0xfa9, 0);
    }
}

fn create_scene(e: &mut Engine, menu: u32) {
    // The PC body at 0076fbd3-0076ff9a builds the compass texture,
    // camera, and optional forward animation after the XML tiles exist.
    e.with_stack(0x380, |e, storage| {
        let cancellation = storage.addr();
        let texture_name = storage.addr() + 0x80;
        let matrix_x = storage.addr() + 0x100;
        let matrix_y = storage.addr() + 0x160;
        let matrix_product = storage.addr() + 0x1c0;
        let vector = storage.addr() + 0x200;
        let second_vector = storage.addr() + 0x220;
        let controller_name = storage.addr() + 0x240;

        e.call(0x00a0_4200, &args![0_u32]);
        e.call(0x0063_3c90, &args![cancellation, 0_u32]);
        let name = e
            .call(0x0043_8170, &args![texture_name, 0x0107_2e40_u32])
            .u32();
        let texture = e
            .call(0x00a5_fd70, &args![name, 0x011a_9598_u32, 1_u32, 0_u32])
            .u32();
        e.call(0x0066_b0d0, &args![cancellation, texture]);
        e.call(0x0043_81b0, &args![texture_name]);

        let image = e.call(0x0055_9450, &args![cancellation]).u32();
        let tile = hud_field(e, 0x40);
        let object = e.vcall(tile, 0x20, &args![]).u32();
        fn_007700b0(e, Ptr::new(object), Ptr::new(image));
        let tile = hud_field(e, 0x40);
        let object = e.vcall(tile, 0x20, &args![]).u32();
        fn_007700d0(e, Ptr::new(object), 1);

        let allocation = e.call(0x00aa_13e0, &args![0xac_u32]).u32();
        let attachment = if allocation == 0 {
            0
        } else {
            e.call(0x00a5_ecb0, &args![allocation, 0_u32]).u32()
        };
        e.call(0x0066_b0d0, &args![0x011d_9770_u32, attachment]);
        e.call(0x0068_15c0, &args![matrix_x]);
        e.call(0x0068_15c0, &args![matrix_y]);
        let rotation = quiet_f32(e.global(0x0101_ff38));
        e.call(0x0052_4ac0, &args![matrix_x, rotation]);
        let rotation = quiet_f32(e.global(0x0101_ff38));
        e.call(0x004a_0c90, &args![matrix_y, rotation]);
        let product = e
            .call(0x0043_f8d0, &args![matrix_x, matrix_product, matrix_y])
            .u32();
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x0043_fa80, &args![scene, product]);
        let distance = quiet_f32(e.global(0x0101_e2bc));
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x0044_0490, &args![scene, distance]);
        let field_of_view = quiet_f32(e.global(0x0107_2e3c));
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x004b_c1f0, &args![scene, field_of_view, 0.0_f32, 0.0_f32]);
        e.call(0x0043_d410, &args![vector, 0.0_f32, 0_u32, 0_u32]);
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x00a5_9c60, &args![scene, vector]);

        let current_menu = current_hud(e);
        let camera = e.call(0x0077_ee50, &args![current_menu]).u32();
        e.call(0x0066_b0d0, &args![0x011d_975c_u32, camera]);
        let heap_object = e.call(0x0040_1000, &args![1_u32]).u32();
        e.mem.set_u32(0x011d_96bc, heap_object);
        let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.vcall(audio, 0xdc, &args![scene, 1_u32]);
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        let bound = e.call(0x0043_d450, &args![scene]).u32();
        let distance = e.call(0x0084_d030, &args![bound]).f64();
        let shadow_distance = (distance * e.global::<f64>(0x0102_1928)) as f32;
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
        e.call(0x00b5_ca70, &args![audio, scene, shadow_distance]);

        let tile = hud_field(e, 0x148);
        let node = e.call(0x0056_c7f0, &args![tile]).u32();
        let object = e.call(0x0043_b4a0, &args![node, 0_u32]).u32();
        let controller = e.call(0x00a5_c570, &args![object, 0x011f_36ac_u32]).u32();
        let left = quiet_f32(e.global(0x0101_6b78));
        e.call(0x0064_04b0, &args![node, left, 1.0_f32, 0.0_f32, 0.0_f32]);
        e.call(0x0043_d410, &args![second_vector, 0.0_f32, 1_u32, 0_u32]);
        let object = e.call(0x0043_b4a0, &args![node, 0_u32]).u32();
        e.call(0x00a5_9c60, &args![object, second_vector]);
        if controller != 0 {
            let name = e
                .call(0x0043_8170, &args![controller_name, 0x0104_8458_u32])
                .u32();
            let forward = e.call(0x0047_a520, &args![controller, name]).u32();
            set_hud_field(e, 0x200, forward);
            e.call(0x0043_81b0, &args![controller_name]);
            e.call(0x0048_fef0, &args![controller, 0.0_f32]);
            if hud_field(e, 0x200) != 0 {
                let forward = hud_field(e, 0x200);
                e.call(
                    0x0047_aab0,
                    &args![controller, forward, 0_u32, 1_u32, 1.0_f32, 0.0_f32, 0_u32],
                );
            }
        }
        e.call(0x00a1_dc20, &args![menu, 1_u32]);
        let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
        let alpha = quiet_f32(e.mem.f32(setting));
        e.call(0x0077_e430, &args![alpha]);
        hud_main_menu_set_menu_mode(e, 5);
        let critical = e.call(0x0071_3d70, &args![]).u32();
        e.call(0x0045_38c0, &args![critical]);
        e.call(0x0045_cec0, &args![cancellation]);
    });
}

// Translated from 0076bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::Create` (Xbox PDB): loads the XML menu, resolves its tiles,
/// lays out HUD elements, and builds the compass scene.
// Translation incomplete: the compiler's SEH unwind paths are omitted.
pub fn hud_main_menu_create(e: &mut Engine) -> Ptr {
    if e.call(0x004b_7210, &args![]).u32() == 0 {
        return Ptr::NULL;
    }
    let menu_class = e.call(0x0070_6f20, &args![]).u32();
    let previous = e.call(0x00a0_9030, &args![menu_class]).u32();
    if previous != 0 {
        e.vcall(previous, 0, &args![1_u32]);
    }
    let resource = e.call(0x004b_7210, &args![]).u32();
    let maximum_depth = e.call(0x00a0_1530, &args![]).f64() as f32;
    let critical = e.call(0x0071_3d70, &args![]).u32();
    e.call(0x0045_38a0, &args![critical, 0_u32]);
    let path = e.call(0x0058_6150, &args![resource, 0x0107_343c_u32]).u32();
    let xml = e.call(0x00a0_1b00, &args![path]).u32();
    let menu = e.call(0x00a0_3c90, &args![xml]).u32();
    let valid_menu = if menu == 0 {
        false
    } else {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        let expected = e.call(0x0070_6f20, &args![]).u32();
        class == expected
    };
    if !valid_menu {
        if menu != 0 && e.call(0x0072_6070, &args![menu]).u32() != 0 {
            e.vcall(menu, 0, &args![1_u32]);
        }
        let critical = e.call(0x0071_3d70, &args![]).u32();
        e.call(0x0045_38c0, &args![critical]);
        return Ptr::NULL;
    }
    e.call(0x00a1_dc70, &args![menu, xml, 0_u32]);
    e.mem.set_u32(0x011d_96c0, menu);
    let screen_width = e.call(0x0071_5d40, &args![]).f64();
    let screen_width = x87_to_int(e, screen_width);
    let screen_height = e.call(0x0071_5da0, &args![]).f64();
    let screen_height = x87_to_int(e, screen_height);
    let bracket_width = e.call(0x0071_77c0, &args![]).f64();
    let bracket_width = x87_to_int(e, bracket_width);
    let bracket_height = e.call(0x0071_7820, &args![]).f64();
    let bracket_height = x87_to_int(e, bracket_height);
    let original_x = e.call(0x00a0_0940, &args![0x0107_3430_u32, u32::MAX]).u32();
    let total_width = e.call(0x00a0_0940, &args![0x0107_3424_u32, u32::MAX]).u32();
    let image_width = e.call(0x00a0_0940, &args![0x0107_3418_u32, u32::MAX]).u32();
    let dimensions = HudDimensions {
        screen_width,
        screen_height,
        bracket_width,
        bracket_height,
        original_x,
        total_width,
        image_width,
    };
    create_action_points(e, menu, dimensions);
    create_hit_points(e, menu, dimensions);
    create_radiation_enemy_quest(e, menu, dimensions);
    create_region_reticle_sneak_messages(e, menu, dimensions);
    create_info_subtitles(e, menu, dimensions);
    create_hotkeys_xp_breath_explosives(e, menu, dimensions);
    let icon_size = create_crippled_limbs(e, dimensions);
    create_ddt_and_hardcore(e, icon_size);

    let current_menu = current_hud(e);
    if !fn_00770070(e, Ptr::new(current_menu)) {
        e.call(0x0040_fbe0, &args![0x0107_2e70_u32]);
        e.call(0x0070_37e0, &args![]);
    }
    let kind = create_get_float(e, 0x40, 0xfa7);
    let first: f64 = e.global(0x0107_04f0);
    let second_match = if kind == first {
        false
    } else {
        create_get_float(e, 0x40, 0xfa7) == e.global::<f64>(0x0107_04e8)
    };
    if kind == first || second_match {
        create_float(e, 0x40, 0xfad, quiet_f32(maximum_depth));
    }
    for (tile_offset, setting) in [
        (0x58, 0x011d_4918),
        (0x48, 0x011d_3e8c),
        (0x30, 0x011d_300c),
    ] {
        let text = e.call(0x0040_3df0, &args![setting]).u32();
        let current_tile = hud_field(e, tile_offset);
        hud_set_text(e, current_tile, text);
    }
    set_hud_field(e, 0x210, 0x358);
    let compass = create_get_float(e, 0x40, 0xfb1);
    let display_width = x87_to_int(e, compass / e.global::<f64>(0x0101_1590)).wrapping_sub(0x23);
    set_hud_field(e, 0x214, display_width as u32);
    set_hud_field(e, 0x218, 0x46);
    hud_trait(e, xml, 0xfa3, 1);
    let current_menu = current_hud(e);
    e.call(0x0072_6070, &args![current_menu]);
    create_scene(e, menu);
    Ptr::new(xml)
}

fn hud_tile(e: &Engine, id: u32) -> u32 {
    e.mem.u32(e.mem.u32(0x011d_96c0) + 0x28 + id * 4)
}

fn hud_trait(e: &mut Engine, hud_tile: u32, trait_id: u32, value: u32) {
    e.call(0x0070_0320, &args![hud_tile, trait_id, value]);
}

fn hud_float(e: &mut Engine, hud_tile: u32, trait_id: u32) -> f32 {
    e.call(0x00a0_11b0, &args![hud_tile, trait_id]).f32()
}

fn hud_set_float(e: &mut Engine, hud_tile: u32, trait_id: u32, value: f32) {
    e.call(0x00a0_12d0, &args![hud_tile, trait_id, value, 1_u32]);
}

fn hud_set_text(e: &mut Engine, hud_tile: u32, text: u32) {
    e.call(0x00a0_1350, &args![hud_tile, 0xfc4_u32, text, 1_u32]);
}

fn add_hud_fade(e: &mut Engine, hud_tile: u32) {
    let fade_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(fade_setting)) * e.global::<f64>(0x0101_e568)) as f32;
    let duration: f32 = e.global(0x0101_62c0);
    e.call(
        0x00a0_7c60,
        &args![hud_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
    );
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    e.call(0x0071_5c60, &args![hud_tile, 0x1004_u32, now]);
}

// Translated from 00709430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetAmmoRegenRate` (Xbox PDB): reads the base regeneration
/// rate and optionally adds weapon-mod effect 6 before rounding to a float.
pub fn tes_object_weap_get_ammo_regen_rate(e: &mut Engine, weapon: Ptr, include_mod: bool) -> f64 {
    let base = e.mem.f32(weapon.addr() + 0x1a4);
    if include_mod {
        let modifier = e.call(0x004b_cf60, &args![weapon, 6_u32, 0_u32]).f64();
        (modifier + f64::from(base)) as f32 as f64
    } else {
        f64::from(base)
    }
}

// Translated from 0076b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a custom quest-text update and queues it according to priority.
/// Zero font IDs become -1 after the game's warning; invalid priorities
/// warn and enter the normal queue.
#[allow(clippy::too_many_arguments)] // Native ABI has seven parameters.
pub fn fn_0076b960(
    e: &mut Engine,
    title: Ptr,
    subtitle: Ptr,
    priority: i32,
    display_time: u32,
    mut title_font: i32,
    mut subtitle_font: i32,
    extra_text: Ptr,
) {
    let update = e.call(0x0040_1000, &args![0x324_u32]).u32();
    e.mem.set_u32(update + 4, 3);
    e.call(0x0040_6d30, &args![update + 8, 0x104_u32, title]);
    e.call(0x0040_6d30, &args![update + 0x10c, 0x104_u32, subtitle]);
    e.mem.set_u32(update + 0x210, display_time);
    e.mem.set_u32(update + 0x214, priority as u32);
    if title_font == 0 {
        e.call(0x005b_5e40, &args![7_u32, 0x0107_2d78_u32]);
        title_font = -1;
    }
    e.mem.set_u32(update + 0x218, title_font as u32);
    if subtitle_font == 0 {
        e.call(0x005b_5e40, &args![7_u32, 0x0107_2cf8_u32]);
        subtitle_font = -1;
    }
    e.mem.set_u32(update + 0x21c, subtitle_font as u32);
    e.call(0x0040_6d30, &args![update + 0x220, 0x104_u32, extra_text]);
    if priority == 0 {
        e.call(0x0076_baa0, &args![update]);
    } else {
        if !(1..=2).contains(&priority) {
            e.call(0x005b_5e40, &args![7_u32, 0x0107_2c88_u32, priority]);
        }
        e.call(0x0076_bad0, &args![update]);
    }
}

// Translated from 0076bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::GetClass` (Xbox PDB): menu class ID 0x3ec.
pub fn hud_main_menu_get_class(_e: &mut Engine, _this: Ptr<HUDMainMenu>) -> u32 {
    0x3ec
}

// Translated from 0076bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::AttachTileByID` (Xbox PDB): stores tile IDs 1 through 86.
pub fn hud_main_menu_attach_tile_by_id(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    id: i32,
    attached_tile: Ptr,
) {
    if (1..=0x56).contains(&id) {
        e.mem.set_u32(tile(this, id as u32), attached_tile.addr());
    }
}

// Translated from 0076be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu` scalar deleting destructor (Xbox PDB): destroys the menu
/// and optionally frees its storage.
pub fn hud_main_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    flags: u32,
) -> Ptr<HUDMainMenu> {
    fn_0076be30(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0076be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the HUD menu and its owned tiles, messages, arrays and base menu.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn fn_0076be30(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    e.mem.set_u32(this.addr(), 0x0107_2df4);
    let menu_tile = e.call(0x0072_6070, &args![e.mem.u32(0x011d_96c0)]).u32();
    if menu_tile != 0 {
        e.vcall(menu_tile, 0, &args![1_u32]);
    }
    let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
    let fade = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    e.vcall(audio, 0xe8, &args![fade]);
    e.call(0x0066_b0d0, &args![0x011d_9770_u32, 0_u32]);
    e.call(0x0066_b0d0, &args![0x011d_975c_u32, 0_u32]);
    e.mem.set_u32(0x011d_96bc, 0);
    loop {
        let node = e.call(0x0068_15c0, &args![this.addr() + 0x18c]).u32();
        let message = e.mem.u32(node);
        if message == 0 {
            break;
        }
        e.call(0x0040_1030, &args![message]);
        e.call(0x0063_f7b0, &args![this.addr() + 0x18c]);
    }
    e.mem.set_u32(0x011d_96c0, 0);
    for (offset, callee) in [
        (0x26c, 0x0046_ffb0),
        (0x264, 0x0046_ffb0),
        (0x258, 0x0046_ffb0),
        (0x1c8, 0x0070_1520),
        (0x1ac, 0x0048_3710),
        (0x198, 0x0077_f9a0),
        (0x18c, 0x0046_ffb0),
    ] {
        e.call(callee, &args![this.addr() + offset]);
    }
    e.call(0x00a1_c520, &args![this]);
}

// Translated from 00770070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks whether the HUD's tile slots 1 through 85 are populated.
pub fn fn_00770070(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    (1..0x56).all(|id| e.mem.u32(tile(this, id)) != 0)
}

// Translated from 007700b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the `NiPointer` at menu offset 0x64.
pub fn fn_007700b0(e: &mut Engine, this: Ptr<HUDMainMenu>, object: Ptr) {
    e.call(0x0066_b0d0, &args![this.addr() + 0x64, object]);
}

// Translated from 007700d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a visibility flag at menu offset 0x91.
pub fn fn_007700d0(e: &mut Engine, this: Ptr<HUDMainMenu>, visible: u8) {
    e.mem.set_u8(this.addr() + 0x91, visible);
}

// Translated from 007700f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes a target's displayed damage threshold against the player, using
/// the game's process-acquire form and the two applicable perk entry points.
pub fn fn_007700f0(e: &mut Engine, player: Ptr, target: Ptr) -> f64 {
    if player.is_null() || target.is_null() {
        return 0.0;
    }
    let base = e.vcall(target.addr() + 0xa4, 0x0c, &args![0x4c_u32]).f32();
    let base = e.call(0x0040_4010, &args![base, 0.0_f32]).f32();
    let process = e.call(0x008d_8520, &args![player]).u32();
    let acquired = e.vcall(process, 0x148, &args![]).u32();
    let form = if acquired == 0 {
        e.mem.u32(0x011c_a278)
    } else {
        e.call(0x0044_ddc0, &args![acquired]).u32()
    };
    e.with_stack(8, |e, storage| {
        e.mem.set_f32(storage.addr(), 0.0);
        e.call(0x005e_58f0, &args![0x3a_u32, player, form, target, storage]);
        e.mem.set_f32(storage.addr() + 4, 0.0);
        e.call(
            0x005e_58f0,
            &args![0x38_u32, target, player, form, storage.addr() + 4],
        );
        let adjusted = ((f64::from(base) - f64::from(e.mem.f32(storage.addr())))
            + f64::from(e.mem.f32(storage.addr() + 4))) as f32;
        let clamped = e.call(0x0040_4010, &args![0.0_f32, adjusted]).f32();
        f64::from(clamped)
    })
}

// Translated from 007701e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateEnemyHealth` (Xbox PDB): updates the enemy meter and
/// its damage threshold text from actor values and perk entry points.
pub fn hud_main_menu_update_enemy_health(e: &mut Engine, this: Ptr<HUDMainMenu>, enemy: Ptr) {
    let actor_values = enemy.addr() + 0xa4;
    let maximum = e.vcall(actor_values, 0, &args![0x10_u32]).i32() as f32;
    let current = e.vcall(actor_values, 8, &args![0x10_u32]).i32() as f32;
    let player: Ptr = Ptr::new(e.mem.u32(0x011d_ea3c));
    let threshold = fn_007700f0(e, player, enemy) as f32;
    let zero: f64 = e.global(0x0101_2060);
    let ratio = if f64::from(maximum) > zero {
        (f64::from(current) / f64::from(maximum)) as f32
    } else {
        0.0
    };
    if e.mem.u32(0x011d_97bc) & 1 == 0 {
        let flags = e.mem.u32(0x011d_97bc);
        e.mem.set_u32(0x011d_97bc, flags | 1);
        e.mem.set_f32(0x011d_97b8, ratio);
    }
    if ratio != e.mem.f32(0x011d_97b8) {
        let timer = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
        e.set(this, HUDMainMenu::iEnemyHealthTimer, timer);
        e.mem.set_f32(0x011d_97b8, ratio);
    }
    if e.call(0x0047_7ba0, &args![enemy]).bool() {
        let enemy_meter = e.mem.u32(this.addr() + 0x80);
        hud_main_menu_update_meter(e, Ptr::new(enemy_meter), 0.0, 2, 0, 0);
    } else {
        let show = !e.vcall(enemy.addr(), 0x22c, &args![0_u32]).bool();
        let enemy_meter = e.mem.u32(this.addr() + 0x80);
        hud_main_menu_update_meter(e, Ptr::new(enemy_meter), ratio, 2, i32::from(show), 0);
    }
    e.with_stack(0x108, |e, storage| {
        e.mem.set_f32(storage.addr(), 0.0);
        let current_player = e.mem.u32(0x011d_ea3c);
        e.call(0x005e_58f0, &args![0x2f_u32, current_player, storage]);
        if f64::from(e.mem.f32(storage.addr())) > zero {
            let damage_label = e.call(0x0040_3df0, &args![0x011d_396c_u32]).u32();
            let threshold_as_int = e.call(0x00ec_62c0, &args![f64::from(current)]).i32();
            let health_label = e.call(0x0040_3df0, &args![0x011d_50b0_u32]).u32();
            let name = e.call(0x0055_d520, &args![enemy]).u32();
            e.call(
                0x00ec_623a,
                &args![
                    storage.addr() + 4,
                    0x0107_3460_u32,
                    name,
                    health_label,
                    threshold_as_int,
                    damage_label,
                    f64::from(threshold)
                ],
            );
            let current_menu = e.mem.u32(0x011d_96c0);
            let hud_text = e.mem.u32(current_menu + 0x84);
            e.call(
                0x00a0_1350,
                &args![hud_text, 0xfc4_u32, storage.addr() + 4, 1_u32],
            );
        } else {
            let name = e.call(0x0055_d520, &args![enemy]).u32();
            let current_menu = e.mem.u32(0x011d_96c0);
            let hud_text = e.mem.u32(current_menu + 0x84);
            e.call(0x00a0_1350, &args![hud_text, 0xfc4_u32, name, 1_u32]);
        }
    });
}

fn current_hud_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(e.mem.u32(0x011d_96c0) + offset)
}

fn hud_visible(e: &mut Engine, offset: u32) -> bool {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_1230, &args![current_tile, 0xfa3_u32]).u8() != 0
}

fn fade_finished(e: &mut Engine, offset: u32) -> bool {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_7fc0, &args![current_tile, 0xfa9_u32]).u8() != 0
}

fn hud_alpha(e: &mut Engine, offset: u32) -> f64 {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_11b0, &args![current_tile, 0xfa9_u32]).f64()
}

fn hud_time(e: &mut Engine, object: u32) -> u32 {
    e.call(0x0082_5c00, &args![object]).u32()
}

fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(
        if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        },
    )
}

fn fistp_i64_low(value: f64) -> u32 {
    let converted = if value.is_finite()
        && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    converted as u32
}

fn do_idle_geometry(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let width = e.call(0x0071_5da0, &args![]).f64();
    let width = x87_to_int(e, width);
    let screen_width = e.call(0x0071_5d40, &args![]).f64();
    let screen_width = x87_to_int(e, screen_width);
    let compass_width = e.call(0x0071_77c0, &args![]).f64();
    let compass_width = x87_to_int(e, compass_width);
    let inset = e.call(0x0071_7820, &args![]).f64();
    let inset = x87_to_int(e, inset);
    let current_tile = current_hud_tile(e, 0x118);
    hud_trait(
        e,
        current_tile,
        0xfa1,
        compass_width.wrapping_mul(2).wrapping_add(10) as u32,
    );
    let current_tile = current_hud_tile(e, 0x118);
    let current_width = e.call(0x00a0_11b0, &args![current_tile, 0xfb0_u32]).f64();
    let new_width = (f64::from(width.wrapping_sub(inset.wrapping_mul(2))) - current_width) as f32;
    let current_tile = current_hud_tile(e, 0x118);
    hud_set_float(e, current_tile, 0xfa2, new_width);
    let current_tile = current_hud_tile(e, 0x114);
    let current_x = e.call(0x00a0_11b0, &args![current_tile, 0xfb1_u32]).f64();
    let horizontal_adjustment: f64 = e.global(0x0101_db88);
    let new_x = (f64::from(screen_width.wrapping_sub(compass_width.wrapping_mul(2))) - current_x
        + horizontal_adjustment) as f32;
    let current_tile = current_hud_tile(e, 0x114);
    hud_set_float(e, current_tile, 0xfa1, new_x);
    let current_tile = current_hud_tile(e, 0x114);
    let current_width = e.call(0x00a0_11b0, &args![current_tile, 0xfb0_u32]).f64();
    let new_width = (f64::from(width.wrapping_sub(inset.wrapping_mul(2))) - current_width) as f32;
    let current_tile = current_hud_tile(e, 0x114);
    hud_set_float(e, current_tile, 0xfa2, new_width);
    for offset in [0x118, 0x114] {
        let current_tile = current_hud_tile(e, offset);
        e.call(0x00a0_7530, &args![current_tile, 1_u32]);
    }
    hud_main_menu_update_actor_value(e, this, 0x10);
    hud_main_menu_update_actor_value(e, this, 0x0c);
    e.call(0x0077_e4f0, &args![this]);
    for offset in [0x180, 0x160, 0x154, 0x170, 0x174, 0x158, 0x15c] {
        let alpha = quiet_f32(e.mem.f32(0x011d_979c));
        let current_tile = current_hud_tile(e, offset);
        hud_set_float(e, current_tile, 0xfa9, alpha);
    }
}

fn do_idle_fades(e: &mut Engine) {
    for (offset, from_setting) in [(0x64_u32, false), (0x68, true)] {
        if hud_alpha(e, offset) > e.global::<f64>(0x0101_2060) && fade_finished(e, offset) {
            let now = hud_time(e, 0x011f_6394);
            let current_tile = current_hud_tile(e, offset);
            let start = e.call(0x00a0_11b0, &args![current_tile, 0x1004_u32]).f64();
            let timeout: f64 = e.global(0x0107_3480);
            if f64::from(now) - start > timeout {
                let alpha = if from_setting {
                    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
                    (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32
                } else {
                    hud_alpha(e, offset) as f32
                };
                let duration: f32 = quiet_f32(e.global(0x0101_62c0));
                let current_tile = current_hud_tile(e, offset);
                e.call(
                    0x00a0_7c60,
                    &args![current_tile, 0xfa9_u32, alpha, 0.0_f32, duration, 0_u32],
                );
            }
        }
    }
    let hold_trait = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
    let text_tile = current_hud_tile(e, 0x64);
    if e.call(0x00a0_1230, &args![text_tile, hold_trait]).u8() != 0 {
        let compass = current_hud_tile(e, 0x128);
        if e.call(0x00a0_11b0, &args![compass, 0xfa3_u32]).f64() != e.global::<f64>(0x0101_2060) {
            let subtitle = current_hud_tile(e, 0x68);
            if e.call(0x00a0_11b0, &args![subtitle, 0xfa9_u32]).f64()
                == e.global::<f64>(0x0101_2060)
                && fade_finished(e, 0x68)
            {
                let duration: f32 = quiet_f32(e.global(0x0101_62c0));
                let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
                let alpha = (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32;
                let current_tile = current_hud_tile(e, 0x64);
                e.call(
                    0x00a0_7c60,
                    &args![current_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
                );
                let now = hud_time(e, 0x011f_6394);
                let current_tile = current_hud_tile(e, 0x64);
                e.call(0x0071_5c60, &args![current_tile, 0x1004_u32, now]);
                let hold_trait = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
                let current_tile = current_hud_tile(e, 0x64);
                hud_trait(e, current_tile, hold_trait, 0);
            }
        }
    }
    let player = e.mem.u32(0x011d_ea3c);
    if e.call(0x0049_97b0, &args![player]).u8() != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        hud_main_menu_update_sneak(e, Ptr::new(menu));
    } else if hud_alpha(e, 0x88) > e.global::<f64>(0x0101_2060) && fade_finished(e, 0x88) {
        let duration: f32 = quiet_f32(e.global(0x0101_6248));
        let alpha = hud_alpha(e, 0x88) as f32;
        let current_tile = current_hud_tile(e, 0x88);
        e.call(
            0x00a0_7c60,
            &args![current_tile, 0xfa9_u32, alpha, 0.0_f32, duration, 0_u32],
        );
    }
}

fn do_idle_target(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    e.mem.set_u8(0x011d_96c5, 0);
    if e.mem.u32(this.addr() + 0x21c) != 0 {
        let player = e.mem.u32(0x011d_ea3c);
        let mut entry = e.call(0x0059_5d10, &args![player]).u32();
        while entry != 0 && e.call(0x0082_56d0, &args![entry]).u8() == 0 {
            let first = e.call(0x0068_15c0, &args![entry]).u32();
            let object = e.mem.u32(first);
            if e.mem.u32(object) == e.mem.u32(this.addr() + 0x21c)
                && e.mem.u8(object + 4) != 0
                && e.mem.u8(object + 5) == 0
            {
                e.mem.set_u8(0x011d_96c5, 1);
                break;
            }
            entry = e.call(0x0072_6070, &args![entry]).u32();
        }
    }
    let player = e.mem.u32(0x011d_ea3c);
    let heading = e.vcall(player, 0x2bc, &args![0_u32]).f64();
    let degrees_per_radian: f64 = e.global(0x0102_f248);
    let heading = (heading * degrees_per_radian) as f32;
    let mut target_heading = 0.0_f32;
    let target = e.mem.u32(this.addr() + 0x21c);
    if target != 0 {
        let target_location = e.vcall(target, 0x1f4, &args![]).u32();
        let player = e.mem.u32(0x011d_ea3c);
        let player_location = e.vcall(player, 0x1f4, &args![]).u32();
        target_heading = hud_main_menu_heading_between_locations(
            e,
            Ptr::new(player_location),
            Ptr::new(target_location),
        ) as f32;
    }
    let now = hud_time(e, this.addr() + 0x224);
    let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x244));
    let timing_setting = e.call(0x0040_3e20, &args![0x011c_f87c_u32]).u32();
    let timeout =
        fistp_i64_low(f64::from(e.mem.f32(timing_setting)) * e.global::<f64>(0x0101_7b70));
    let close_enough = if elapsed <= timeout {
        true
    } else {
        let difference = (f64::from(heading) - f64::from(target_heading)) as f32;
        let angle = e.call(0x0040_8840, &args![difference]).f64();
        angle < e.global::<f64>(0x0103_57e8)
    };
    let mut show_enemy = false;
    if e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4 {
        let actor = e.mem.u32(0x011f_21cc);
        if actor != 0 && e.vcall(actor, 0x100, &args![]).u8() != 0 {
            let actor = e.mem.u32(0x011f_21cc);
            if e.vcall(actor, 0x22c, &args![0_u32]).u8() == 0 {
                show_enemy = true;
                let actor = e.mem.u32(0x011f_21cc);
                hud_main_menu_set_enemy_actor(e, this, Ptr::new(actor));
            }
        }
    } else {
        let target = e.mem.u32(this.addr() + 0x21c);
        let mut show_ref = false;
        if target != 0 && e.vcall(target, 0x22c, &args![0_u32]).u8() == 0 {
            let target = e.mem.u32(this.addr() + 0x21c);
            if e.call(0x0047_7ba0, &args![target]).u8() == 0
                && e.mem.u8(0x011d_96c5) != 0
                && !hud_visible(e, 0x9c)
                && !hud_visible(e, 0xa8)
            {
                let current_tile = current_hud_tile(e, 0x120);
                let state = e.call(0x00a0_11b0, &args![current_tile, 0xff4_u32]).f64();
                if state == e.global::<f64>(0x0101_1590) && close_enough {
                    show_ref = true;
                }
            }
        }
        if show_ref {
            show_enemy = true;
            let last = e.mem.u32(this.addr() + 0x1b8);
            e.call(0x0077_5a00, &args![last, 0_u32, 1_u32]);
        } else {
            let current_tile = current_hud_tile(e, 0x120);
            let state = e.call(0x00a0_11b0, &args![current_tile, 0xff4_u32]).f64();
            if state == e.global::<f64>(0x0101_1590) {
                for offset in [0x7c, 0x80, 0x84] {
                    let current_tile = e.mem.u32(this.addr() + offset);
                    hud_trait(e, current_tile, 0xfa3, 0);
                }
            }
        }
    }
    if show_enemy {
        let mut perceived = false;
        let target = e.mem.u32(this.addr() + 0x21c);
        if target != 0 {
            let player = e.mem.u32(0x011d_ea3c);
            let actor = e.call(0x0096_7090, &args![player, target]).u32();
            if actor != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                let target = e.mem.u32(this.addr() + 0x21c);
                let actor = e.call(0x0096_7090, &args![player, target]).u32();
                perceived = e.mem.u8(actor + 4) != 0;
            }
            if !perceived {
                let target = e.mem.u32(this.addr() + 0x21c);
                let player = e.mem.u32(0x011d_ea3c);
                perceived = e.call(0x008b_c700, &args![target, player]).u8() != 0;
            }
        }
        for offset in [0x7c, 0x80, 0x84] {
            let current_tile = e.mem.u32(this.addr() + offset);
            hud_trait(e, current_tile, 0xfa3, 1);
        }
        let current_tile = current_hud_tile(e, 0x120);
        hud_trait(e, current_tile, 0xff4, u32::from(perceived) + 1);
        for offset in [0x7c, 0x80, 0x84, 0x84] {
            let current_tile = current_hud_tile(e, offset);
            e.call(0x00a0_7530, &args![current_tile, 8_u32]);
        }
    }
    let target = e.mem.u32(this.addr() + 0x21c);
    if target != 0 {
        let qualifies = e.call(0x0049_3bb0, &args![target]).u8() != 0
            || e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4;
        if qualifies {
            let target = e.mem.u32(this.addr() + 0x21c);
            hud_main_menu_update_enemy_health(e, this, Ptr::new(target));
        }
    }
    let last = e.mem.u32(this.addr() + 0x1b8);
    if last != 0 {
        let previous = e.mem.u8(0x011d_97c0);
        let current = e.call(0x0057_9690, &args![last]).u8();
        if previous != current {
            let last = e.mem.u32(this.addr() + 0x1b8);
            let current = e.call(0x0057_9690, &args![last]).u8();
            e.mem.set_u8(0x011d_97c0, current);
            let last = e.mem.u32(this.addr() + 0x1b8);
            e.call(0x0077_5a00, &args![last, 0_u32, 1_u32]);
        }
    }
}

fn do_idle_quests(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let mut display_reminder = false;
    if e.mem.u8(this.addr() + 0x205) != 0 {
        let now = hud_time(e, 0x011f_6394);
        let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x248));
        let setting = e.call(0x0043_d4d0, &args![0x011d_2220_u32]).u32();
        let delay = e.mem.u32(setting).wrapping_mul(1000);
        if elapsed > delay {
            let mode = e.mem.u32(e.mem.u32(0x011d_96c0) + 0x1c4);
            if mode != 0x12 && mode != 0x13 {
                display_reminder = true;
            }
        }
    }
    if display_reminder {
        let now = hud_time(e, 0x011f_6394);
        e.mem.set_u32(this.addr() + 0x248, now);
        let player = e.mem.u32(0x011d_ea3c);
        let quest = e.call(0x005c_bb50, &args![player]).u32();
        hud_main_menu_display_quest_reminder(e, Ptr::new(quest));
    } else if e.mem.u32(this.addr() + 0x248) == 0 && e.call(0x0045_8030, &args![]).u32() == 2 {
        let now = hud_time(e, 0x011f_6394);
        e.mem.set_u32(this.addr() + 0x248, now);
        let player = e.mem.u32(0x011d_ea3c);
        let quest = e.call(0x005c_bb50, &args![player]).u32();
        hud_main_menu_display_quest_reminder(e, Ptr::new(quest));
        e.mem.set_u8(this.addr() + 0x254, 0);
    }
    if e.mem.u8(this.addr() + 0x254) != 0 {
        e.call(0x0077_f500, &args![this]);
        e.call(0x0077_f5e0, &args![this]);
        e.mem.set_u8(this.addr() + 0x254, 0);
        return;
    }
    if e.call(0x0076_b610, &args![this.addr() + 0x198]).u8() != 0 {
        return;
    }
    let mut index = 0_u32;
    loop {
        let count = e.call(0x0044_ddc0, &args![this.addr() + 0x198]).u32();
        if index >= count {
            break;
        }
        let now = hud_time(e, this.addr() + 0x224);
        let entry = e
            .call(0x0087_7a30, &args![this.addr() + 0x198, index])
            .u32();
        let message = e.mem.u32(entry);
        let elapsed = now.wrapping_sub(e.mem.u32(message + 0x104));
        let entry = e
            .call(0x0087_7a30, &args![this.addr() + 0x198, index])
            .u32();
        let message = e.mem.u32(entry);
        let duration = f64::from(e.mem.f32(message + 0x108)) * e.global::<f64>(0x0101_7b70);
        if duration <= f64::from(elapsed) {
            let entry = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            if e.mem.u32(entry) == e.mem.u32(this.addr() + 0x1a8) {
                e.call(0x0077_5670, &args![this]);
            }
            let entry = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            let message = e.mem.u32(entry);
            e.call(0x0040_1030, &args![message]);
            e.call(0x006b_f8f0, &args![this.addr() + 0x198, index, 1_u32]);
        } else {
            index = index.wrapping_add(1);
        }
    }
    if e.mem.u32(this.addr() + 0x1a8) == 0 {
        let text = e.call(0x0077_5230, &args![this]).u32();
        e.mem.set_u32(this.addr() + 0x1a8, text);
        if e.mem.u32(this.addr() + 0x1a8) != 0 {
            let text = e.mem.u32(this.addr() + 0x1a8);
            let current_tile = current_hud_tile(e, 0x98);
            hud_set_text(e, current_tile, text);
            let current_tile = current_hud_tile(e, 0x98);
            hud_trait(e, current_tile, 0xfa3, 1);
        }
    }
}

fn do_idle_messages(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if e.call(0x005a_e380, &args![this.addr() + 0x18c]).u32() == 0 || !hud_visible(e, 0x134) {
        return;
    }
    if hud_alpha(e, 0x94) == e.global::<f64>(0x0101_2060) {
        for offset in [0x94, 0x8c, 0x90] {
            let alpha = quiet_f32(e.mem.f32(0x011d_979c));
            let duration: f32 = quiet_f32(e.global(0x0102_1f44));
            let current_tile = current_hud_tile(e, offset);
            e.call(
                0x00a0_7c60,
                &args![current_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
            );
        }
    }
    if fade_finished(e, 0x94) {
        let first = e.call(0x0068_15c0, &args![this.addr() + 0x18c]).u32();
        let message = e.mem.u32(first);
        if e.mem.u32(this.addr() + 0x194) == 0 {
            let now = hud_time(e, this.addr() + 0x224);
            e.mem.set_u32(this.addr() + 0x194, now);
        }
        let now = hud_time(e, this.addr() + 0x224);
        let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x194));
        let duration = f64::from(e.mem.f32(message + 0x30c)) * e.global::<f64>(0x0101_7b70);
        if duration <= f64::from(elapsed) {
            e.call(0x0077_56e0, &args![this]);
        }
    }
}

// Translated from 00770430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DoIdle` (Xbox PDB): advances the visible HUD, target,
/// reminder and message state on the game's timed UI cadence.
// Translation incomplete: the compiler's SEH unwind frame is omitted.
pub fn hud_main_menu_do_idle(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let player = e.mem.u32(0x011d_ea3c);
    if player == 0 || e.call(0x008d_6f30, &args![player]).u32() == 0 {
        return;
    }
    fn_00771520(e, Ptr::new(this.addr() + 0x224));
    let scene = e.mem.u32(this.addr() + 0x40);
    let a = e.mem.u32(this.addr() + 0x34);
    let b = e.mem.u32(this.addr() + 0x38);
    let c = e.mem.u32(this.addr() + 0x3c);
    e.call(0x0077_9070, &args![scene, a, b, c]);
    e.call(0x0077_da60, &args![this]);
    fn_007721c0(e, this);
    e.call(0x0077_e750, &args![this]);
    e.call(0x0077_f700, &args![this]);
    if e.mem.u32(0x011d_97c8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_97c8);
        e.mem.set_u32(0x011d_97c8, flags | 1);
        let now = hud_time(e, this.addr() + 0x224);
        e.mem.set_u32(0x011d_97c4, now);
    }
    let now = hud_time(e, this.addr() + 0x224);
    if now.wrapping_sub(e.mem.u32(0x011d_97c4)) < 200 {
        return;
    }
    for (timer_offset, first_tile, second_tile) in [(0x24c, 0x154, 0x170), (0x250, 0x158, 0x174)] {
        let timer = e.mem.u32(this.addr() + timer_offset);
        let expired = timer == u32::MAX || hud_time(e, 0x011f_6394).wrapping_sub(timer) > 1000;
        if expired {
            e.mem.set_u32(this.addr() + timer_offset, u32::MAX);
            for offset in [first_tile, second_tile] {
                let current_tile = current_hud_tile(e, offset);
                hud_trait(e, current_tile, 0xfa3, 0);
            }
        }
    }
    let now = hud_time(e, this.addr() + 0x224);
    e.mem.set_u32(0x011d_97c4, now);
    do_idle_geometry(e, this);
    do_idle_fades(e);
    let menu = e.mem.u32(0x011d_96c0);
    if !matches!(e.mem.u32(menu + 0x1c4), 0x12 | 0x13) {
        e.call(0x0077_a650, &args![this]);
    }
    e.call(0x0077_c4e0, &args![this]);
    hud_main_menu_update_radiation_meter(e, this);
    let player = e.mem.u32(0x011d_ea3c);
    let status = e.call(0x004d_1360, &args![player]).u8() != 0;
    hud_main_menu_update_status_mode_ui(e, this, status);
    do_idle_target(e, this);
    do_idle_quests(e, this);
    do_idle_messages(e, this);
    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32;
    e.mem.set_f32(0x011d_979c, alpha);
}

// Translated from 00771520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts a scene object and passes the result to its attached controller.
pub fn fn_00771520(e: &mut Engine, object: Ptr) {
    let converted = e.call(0x0045_7fe0, &args![object]).u32();
    e.call(0x00aa_4ee0, &args![object, converted]);
}

// Translated from 00771540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateActorValue` (Xbox PDB): refreshes the action-point
/// or hit-point meter for actor values 12 and 16.
pub fn hud_main_menu_update_actor_value(e: &mut Engine, this: Ptr<HUDMainMenu>, value: u32) {
    let player = e.mem.u32(0x011d_ea3c);
    let actor_values = player + 0xa4;
    match value {
        0x0c => {
            let maximum = e.vcall(actor_values, 0x20, &args![value]).f32();
            let maximum = e.call(0x0040_4010, &args![0.0_f32, maximum]).f32();
            let ratio = if maximum != 0.0 {
                let current = e.vcall(actor_values, 0x0c, &args![value]).f32();
                let current = e.call(0x0040_4010, &args![0.0_f32, current]).f32();
                (f64::from(current) / f64::from(maximum)) as f32
            } else {
                0.0
            };
            let ratio = e.call(0x0040_ebd0, &args![ratio, 1.0_f32]).f32();
            let meter = e.mem.u32(tile(this, 7));
            hud_main_menu_update_meter(e, Ptr::new(meter), ratio, 1, 0, 0);
        }
        0x10 => {
            let maximum = e.vcall(actor_values, 0x20, &args![value]).f32();
            let mut ratio = if maximum != 0.0 {
                let current = e.vcall(actor_values, 0x0c, &args![value]).f32();
                let current = e.call(0x0040_4010, &args![0.0_f32, current]).f32();
                (f64::from(current) / f64::from(maximum)) as f32
            } else {
                0.0
            };
            if ratio > 1.0 {
                ratio = 1.0;
            }
            let should_animate = !e.vcall(player, 0x22c, &args![0_u32]).bool();
            let menu = e.mem.u32(0x011d_96c0);
            let meter = e.mem.u32(menu + 0x2c);
            hud_main_menu_update_meter(e, Ptr::new(meter), ratio, 0, i32::from(should_animate), 0);
        }
        _ => {}
    }
}

fn set_visibility_trait(e: &mut Engine, offset: u32, mask: u32) {
    let menu = e.mem.u32(0x011d_96c0);
    let value = e.mem.u32(menu + 0x1bc) & mask;
    let current_tile = e.mem.u32(e.mem.u32(0x011d_96c0) + offset);
    hud_trait(e, current_tile, 0xfa3, value);
}

// Translated from 00771700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetMenuMode` (Xbox PDB): selects the HUD visibility mask for
/// the current menu and player state, updates each tile, and controls its sound.
pub fn hud_main_menu_set_menu_mode(e: &mut Engine, requested_mode: i32) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let mut mode = requested_mode;
    let menu = e.mem.u32(0x011d_96c0);
    if e.call(0x0059_bb30, &args![menu]).i32() != 1 {
        let menu = e.mem.u32(0x011d_96c0);
        e.call(0x00a1_db20, &args![menu]);
    }
    if mode == 1 {
        let top_menu = e.call(0x0070_23c0, &args![]).u32();
        mode = match top_menu {
            0 | 0x3ec => {
                let player = e.mem.u32(0x011d_ea3c);
                if e.vcall(player, 0x214, &args![]).u32() == 4 {
                    0xb
                } else {
                    let player = e.mem.u32(0x011d_ea3c);
                    if e.vcall(player, 0x214, &args![]).u32() == 9 {
                        0xb
                    } else {
                        let player = e.mem.u32(0x011d_ea3c);
                        if e.vcall(player, 0x214, &args![]).u32() != 0 {
                            10
                        } else if e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4 {
                            8
                        } else {
                            let player = e.mem.u32(0x011d_ea3c);
                            if e.call(0x005a_03f0, &args![player, 1_u32]).u8() != 0 {
                                hud_main_menu_set_menu_mode(e, 0xc);
                                1
                            } else {
                                let player = e.mem.u32(0x011d_ea3c);
                                if e.call(0x008b_bc10, &args![player]).u8() != 0 {
                                    0x16
                                } else {
                                    2
                                }
                            }
                        }
                    }
                }
            }
            0x3ea | 0x3eb | 0x3f6 | 0x3ff | 0x40b | 0x425 => 3,
            0x3ef => 5,
            0x3f0 => 9,
            0x3f1 => 6,
            0x3f4 => 0xb,
            0x3f5 => 4,
            0x41f | 0x421 => 0x10,
            0x420 => 7,
            _ => 1,
        };
    }
    if mode == 0x16 {
        let weapon = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        if weapon != 0 {
            let weapon = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
            if e.call(0x0045_bc00, &args![weapon, 0_u32]).u32() != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                if e.call(0x008a_16d0, &args![player]).u8() != 0 {
                    mode = 0x17;
                }
            }
        }
    }
    if mode != 0x18 && mode != 4 && mode != 5 {
        let player = e.mem.u32(0x011d_ea3c);
        if e.call(0x005a_03f0, &args![player, 1_u32]).u8() != 0 {
            let selected = e.call(0x004b_7210, &args![]).u32();
            if e.call(0x0071_9ae0, &args![selected]).u8() != 0 {
                mode = 2;
            } else if mode != 6 {
                mode = 0xc;
            }
        }
    }
    let flags = match mode {
        2 | 0xd | 0x14 | 0x16 => {
            let player = e.mem.u32(0x011d_ea3c);
            let flags = if e.call(0x005a_03f0, &args![player, 0x20_u32]).u8() != 0 {
                0x1fdff
            } else {
                0x1ffff
            };
            let menu = e.mem.u32(0x011d_96c0);
            e.mem.set_u32(menu + 0x1bc, flags);
            e.call(0x0077_8b10, &args![]);
            None
        }
        3 | 7 => Some(0x100),
        4 | 5 | 0xe | 0x11 | 0x12 | 0x15 | 0x18 | 0x13 | 0x19 => Some(0),
        6 => Some(0x1110),
        8 => Some(0x108),
        9 => Some(0x1000),
        10 => Some(0x11dbf),
        0xb => Some(0x11fff),
        0xc => Some(0x1514),
        0xf | 0x10 => Some(0x1010),
        0x17 => Some(8),
        _ => None,
    };
    if let Some(flags) = flags {
        if !matches!(mode, 10..=0xc) {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x0077_5670, &args![menu]);
        }
        let menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u32(menu + 0x1bc, flags);
    }
    let menu = e.mem.u32(0x011d_96c0);
    let flags = e.mem.u32(menu + 0x1bc) & e.mem.u32(menu + 0x1c0);
    e.mem.set_u32(menu + 0x1bc, flags);
    if e.mem.u8(0x011d_96bb) != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        let flags = e.mem.u32(menu + 0x1bc);
        e.mem.set_u32(menu + 0x1bc, flags & !8);
    }
    let player = e.mem.u32(0x011d_ea3c);
    if player != 0 && fn_007721a0(e, Ptr::new(player)).addr() != 0 {
        let player = e.mem.u32(0x011d_ea3c);
        if e.call(0x0052_4d10, &args![player]).u8() == 0 {
            let menu = e.mem.u32(0x011d_96c0);
            let flags = e.mem.u32(menu + 0x1bc);
            e.mem.set_u32(menu + 0x1bc, flags & !0x40);
        }
    }
    let meter_tile = hud_tile(e, 0x47);
    let value = e.call(0x00a0_11b0, &args![meter_tile, 0x1007_u32]).f64() as f32;
    let value = if mode == 6 {
        (f64::from(value) - e.global::<f64>(0x0107_3488)) as f32
    } else {
        value
    };
    let meter_tile = hud_tile(e, 0x47);
    hud_set_float(e, meter_tile, 0xfa2, value);
    let menu = e.mem.u32(0x011d_96c0);
    let prior_mode = e.mem.u32(menu + 0x1c4);
    if prior_mode != 0x17 && mode == 0x17 {
        e.call(0x0077_f3c0, &args![1_u32]);
    } else if prior_mode == 0x17 && mode != 0x17 {
        e.call(0x0077_f3c0, &args![0_u32]);
    }
    let menu = e.mem.u32(0x011d_96c0);
    e.mem.set_u32(menu + 0x1c4, mode as u32);
    if !matches!(
        mode,
        0x14 | 6 | 2 | 5 | 0xe | 0x11 | 9 | 0xb | 0x12 | 0x13 | 0x19
    ) {
        let menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u8(menu + 0x254, 1);
    }
    for (offset, bit) in [
        (0x114, 1),
        (0x118, 2),
        (0x11c, 4),
        (0x120, 8),
        (0x124, 0x10),
        (0x128, 0x20),
        (0x12c, 0x40),
        (0x130, 0x80),
        (0x134, 0x100),
        (0x138, 0x200),
        (0x13c, 0x400),
        (0x140, 0x800),
        (0x144, 0x1000),
        (0x148, 0x2000),
        (0x14c, 0x4000),
        (0x150, 0x8000),
        (0x15c, 1),
        (0x160, 0x10000),
        (0x180, 1),
    ] {
        set_visibility_trait(e, offset, bit);
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.mem.u32(menu + 0x1bc) & 1 == 0 {
        for offset in [0x154, 0x170] {
            let menu = e.mem.u32(0x011d_96c0);
            let current_tile = e.mem.u32(menu + offset);
            hud_trait(e, current_tile, 0xfa3, 0);
        }
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.mem.u32(menu + 0x1bc) & 8 == 0 {
        for offset in [0x158, 0x174] {
            let menu = e.mem.u32(0x011d_96c0);
            let current_tile = e.mem.u32(menu + offset);
            hud_trait(e, current_tile, 0xfa3, 0);
        }
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.call(0x00ad_8ce0, &args![menu + 0x1ac]).u8() != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        if e.mem.u32(menu + 0x1bc) & 4 != 0 {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x00ad_8830, &args![menu + 0x1ac, 0_u32]);
        } else {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x00ad_88b0, &args![menu + 0x1ac]);
        }
    }
}

// Translated from 007721a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the object's pointer at PC offset 0xe34.
pub fn fn_007721a0(e: &mut Engine, object: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(object.addr() + 0xe34))
}

// Translated from 00772820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the value held by the object's inline pointer at +0xcc.
pub fn fn_00772820(e: &mut Engine, object: Ptr) -> Ptr {
    e.call(0x0055_9450, &args![object.addr() + 0xcc]).ptr()
}

// Translated from 00772840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HeadingBetweenLocations` (Xbox PDB): asks the source
/// location for a relative 2D vector, resolves its angle, clamps it, and
/// converts radians to degrees.
pub fn hud_main_menu_heading_between_locations(e: &mut Engine, from: Ptr, to: Ptr) -> f64 {
    e.with_stack(12, |e, relative| {
        e.call(0x0043_9ef0, &args![to, relative, from]);
        let x = e.mem.f32(relative.addr());
        let y = e.mem.f32(relative.addr() + 4);
        let mut radians = 0.0_f32;
        if y != 0.0 {
            let ratio = (f64::from(x) / f64::from(y)) as f32;
            radians = e.call(0x004b_1460, &args![ratio]).f32();
            if y < 0.0 {
                let pi: f64 = e.global(0x0101_ff40);
                radians = (f64::from(radians) + pi) as f32;
            }
        } else if x < 0.0 {
            radians = e.global(0x0102_b3c8);
        }
        let clamped = e.call(0x004b_1480, &args![radians]).f32();
        let degrees_per_radian: f64 = e.global(0x0102_f248);
        f64::from((f64::from(clamped) * degrees_per_radian) as f32)
    })
}

// Translated from 007728f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetRegionText` (Xbox PDB): updates the region/location
/// text when its tile is clear, with a fade when it replaces prior text.
pub fn hud_main_menu_set_region_text(e: &mut Engine, region: Ptr) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let enabled = e.mem.u8(region.addr() + 0x108) != 0;
    let region_id = e.mem.u32(region.addr());
    if enabled && region_id == e.mem.u32(0x011d_9aec) {
        return;
    }
    if enabled {
        e.mem.set_u32(0x011d_9aec, region_id);
    }
    let region_tile = hud_tile(e, 15);
    let display_tile = hud_tile(e, 16);
    let has_previous_text = if hud_float(e, region_tile, 0xfa9) != 0.0 {
        let old_text = e.call(0x00a0_11f0, &args![region_tile, 0xfc4_u32]).u32();
        e.call(0x0040_8b20, &args![region.addr() + 4, old_text])
            .i32()
            != 0
    } else {
        false
    };
    if e.mem.u8(region.addr() + 4) != 0
        && hud_float(e, display_tile, 0xfa9) == 0.0
        && e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
    {
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        hud_trait(e, region_tile, hold, 0);
        hud_set_text(e, region_tile, region.addr() + 4);
        if has_previous_text {
            add_hud_fade(e, region_tile);
        }
    }
}

// Translated from 00772aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the HUD's location text when a newly active reference has map
/// marker data, and clears its fade control while the region text resets.
pub fn fn_00772aa0(e: &mut Engine, reference: Ptr, state: Ptr) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let active = e.mem.u8(state.addr() + 4) != 0;
    let id = e.mem.u32(state.addr());
    if active && id != e.mem.u32(0x011d_9af0) {
        e.mem.set_u32(0x011d_9af0, id);
        if id == 0 {
            return;
        }
        let marker = e.call(0x0056_9060, &args![id]).u32();
        if marker == 0 {
            return;
        }
        let display_tile = hud_tile(e, 16);
        if e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
            && hud_float(e, display_tile, 0xfa9) == 0.0
        {
            let marker = e.call(0x0056_9060, &args![id]).u32();
            let name = e.call(0x0043_8eb0, &args![marker]).u32();
            hud_set_text(e, display_tile, name);
            add_hud_fade(e, display_tile);
        }
        let region_tile = hud_tile(e, 15);
        e.call(0x00a0_7dc0, &args![region_tile, 0xfa9_u32]);
        hud_set_float(e, region_tile, 0xfa9, 0.0);
    } else if !active && id == e.mem.u32(0x011d_9af0) {
        e.mem.set_u32(0x011d_9af0, 0);
    } else if e.mem.u8(state.addr() + 5) != 0 {
        e.mem.set_u32(0x011d_9af0, id);
    }
    let _ = reference; // ECX is not read by the PC body.
}

// Translated from 00772c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetWorldspaceText` (Xbox PDB): shows the worldspace name
/// after the location tile has faded completely. SEH bookkeeping is omitted.
pub fn hud_main_menu_set_worldspace_text(e: &mut Engine, text: Ptr) {
    if e.mem.u32(0x011d_9af8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9af8);
        e.mem.set_u32(0x011d_9af8, flags | 1);
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        e.mem.set_u32(0x011d_9af4, hold);
    }
    if text.is_null() {
        return;
    }
    let display_tile = hud_tile(e, 16);
    if hud_float(e, display_tile, 0xfa9) == 0.0
        && e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
    {
        let region_tile = hud_tile(e, 15);
        hud_set_text(e, region_tile, text.addr());
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        hud_trait(e, region_tile, hold, 1);
    }
}

// Translated from 00773050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the current enemy, quest timer, experience state and associated
/// tile visibility when the HUD menu exists.
pub fn fn_00773050(e: &mut Engine) {
    let class = e.call(0x0070_6f20, &args![]).u32();
    let menu_tile = e.call(0x00a0_9030, &args![class]).u32();
    if menu_tile == 0 {
        return;
    }
    let menu = e.call(0x00a0_3c90, &args![menu_tile]).u32();
    if menu == 0 {
        return;
    }
    let menu = e.call(0x00a0_3c90, &args![menu_tile]).u32();
    e.mem.set_u32(menu + 0x21c, 0);
    e.mem.set_u32(menu + 0x248, 0);
    e.mem.set_u32(menu + 0x24c, u32::MAX);
    e.mem.set_u32(menu + 0x250, u32::MAX);
    e.mem.set_u8(menu + 0x205, 0);
    let hotkey = e.mem.u32(menu + 0xf0);
    hud_trait(e, hotkey, 0xfa3, 0);
    e.mem.set_u8(menu + 0x204, 0);
    for id in 0x2c..=0x31 {
        let tile = hud_tile(e, id);
        hud_trait(e, tile, 0xfa3, 0);
    }
    e.mem.set_u32(0x011a_010c, 1);
    let singleton = e.mem.u32(0x011d_96c0);
    e.mem.set_u8(singleton + 0x254, 1);
}

// Translated from 00773190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetEnemyActor` (Xbox PDB): chooses the enemy shown by the
/// HUD, updates its name, and starts the health timer.
pub fn hud_main_menu_set_enemy_actor(e: &mut Engine, _this: Ptr<HUDMainMenu>, enemy: Ptr) {
    let blocked = e.call(0x0040_8d60, &args![0x011e_0988_u32]).u32();
    if e.mem.u8(blocked) != 0
        || (!enemy.is_null() && e.vcall(enemy.addr(), 0x22c, &args![0_u32]).bool())
    {
        return;
    }
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 {
        return;
    }
    let mode = e.call(0x0044_ddc0, &args![0x011f_2250_u32]).i32();
    let chosen = if mode == 4 {
        let alternative = e.mem.u32(0x011f_21cc);
        if alternative != 0 && e.vcall(alternative, 0x100, &args![]).bool() {
            alternative
        } else {
            0
        }
    } else {
        enemy.addr()
    };
    e.mem.set_u32(menu + 0x21c, chosen);
    if chosen != 0 {
        let name = e.call(0x0055_d520, &args![chosen]).u32();
        let current_menu = e.mem.u32(0x011d_96c0);
        let name_tile = e.mem.u32(current_menu + 0x84);
        hud_set_text(e, name_tile, name);
        let current_menu = e.mem.u32(0x011d_96c0);
        fn_00771520(e, Ptr::new(current_menu + 0x224));
        let current_menu = e.mem.u32(0x011d_96c0);
        let now = e.call(0x0082_5c00, &args![current_menu + 0x224]).u32();
        let current_menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u32(current_menu + 0x244, now);
    }
}

// Translated from 007732b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the HUD's current enemy, if the HUD exists.
pub fn fn_007732b0(e: &mut Engine) -> Ptr {
    let menu = e.mem.u32(0x011d_96c0);
    Ptr::new(if menu == 0 {
        0
    } else {
        e.mem.u32(menu + 0x21c)
    })
}

// Translated from 007735b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the player state byte at +0x5f8.
pub fn fn_007735b0(e: &mut Engine, player: Ptr) -> u8 {
    e.mem.u8(player.addr() + 0x5f8)
}

fn quest_text_showing(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    flag_offset: u32,
    tile_id: u32,
) -> bool {
    if e.mem.u8(this.addr() + flag_offset) == 0 {
        return false;
    }
    let singleton = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(singleton + 0x28 + tile_id * 4);
    e.with_stack(4, |e, cursor| {
        let first = e.call(0x0055_9450, &args![tile + 4]).u32();
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 {
            let item = e.call(0x0057_cbe0, &args![tile + 4, cursor]).u32();
            let text_tile = e.mem.u32(item);
            if e.call(0x00a0_1230, &args![text_tile, 0xfa3_u32]).bool() {
                return true;
            }
        }
        e.mem.set_u8(this.addr() + flag_offset, 0);
        false
    })
}

// Translated from 007735d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::IsQuestNameShowing` (Xbox PDB): tests quest-name child
/// tiles and clears the cached showing flag once none is visible.
pub fn hud_main_menu_is_quest_name_showing(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    quest_text_showing(e, this, 0x260, 0x0e)
}

// Translated from 00773650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::AreObjectivesShowing` (Xbox PDB): tests objective child
/// tiles and clears the cached showing flag once none is visible.
pub fn hud_main_menu_are_objectives_showing(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    quest_text_showing(e, this, 0x261, 0x0d)
}

// Translated from 007737a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests the requested flag in the object at +0x20.
pub fn fn_007737a0(e: &mut Engine, object: Ptr, flag: u32) -> bool {
    e.mem.u32(object.addr() + 0x20) & flag != 0
}

// Translated from 007736d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DisplayQuestReminder` (Xbox PDB): queues incomplete,
/// eligible objectives from the quest's target list while reminders are free.
pub fn hud_main_menu_display_quest_reminder(e: &mut Engine, quest: Ptr) {
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 || quest.is_null() {
        return;
    }
    if hud_main_menu_are_objectives_showing(e, Ptr::new(menu))
        || !e.call(0x0082_56d0, &args![menu + 0x258]).bool()
    {
        return;
    }
    let mut node = e.call(0x005d_43e0, &args![quest]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let element = e.call(0x0068_15c0, &args![node]).u32();
        let objective = e.mem.u32(element);
        if fn_007737a0(e, Ptr::new(objective), 1) {
            let element = e.call(0x0068_15c0, &args![node]).u32();
            let objective = e.mem.u32(element);
            if !fn_007737a0(e, Ptr::new(objective), 2) {
                let element = e.call(0x0068_15c0, &args![node]).u32();
                let objective = e.mem.u32(element);
                let data = e.call(0x005e_c330, &args![objective]).u32();
                e.call(0x0077_a5b0, &args![data, 0_u32, 1_u32]);
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 007737c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateRadiationMeterConditions` (Xbox PDB): sets the
/// radiation amount and postpones the meter's expiry when radiation rises.
pub fn hud_main_menu_update_radiation_meter_conditions(
    e: &mut Engine,
    _this: Ptr<HUDMainMenu>,
    radiation: f32,
    accumulated: f32,
) {
    let sum = (f64::from(radiation) + f64::from(accumulated)) as f32;
    let meter = hud_tile(e, 0x13);
    if radiation > 0.0 {
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        let delay: f64 = e.global(0x0107_3490);
        hud_set_float(e, meter, 0x100a, (now as f64 + delay) as f32);
        hud_set_float(e, meter, 0x1009, sum);
        hud_trait(e, meter, 0x100b, 1);
    } else if accumulated > 0.0 {
        let expiry = hud_float(e, meter, 0x100a);
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        if f64::from(now) >= f64::from(expiry) {
            hud_set_float(e, meter, 0x1009, sum);
        }
    }
}

// Translated from 007738c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007738c0 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::SetRadiationLevel` (Xbox PDB): starts or fades the
/// radiation sound and updates the meter for positive radiation.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn hud_main_menu_set_radiation_level(e: &mut Engine, radiation: f32, accumulated: f32) {
    let sum = (f64::from(radiation) + f64::from(accumulated)) as f32;
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 {
        return;
    }
    let sound = menu + 0x1ac;
    if sum <= 0.0 {
        if e.call(0x00ad_8ce0, &args![sound]).bool() {
            e.call(0x00ad_8da0, &args![sound, 500_u32]);
        }
    } else {
        let valid = e.call(0x00ad_8ce0, &args![sound]).bool();
        if !valid && accumulated > 0.0 {
            e.with_stack(12, |e, handle| {
                let instance = e.call(0x0045_3a70, &args![]).u32();
                let sample = e
                    .call(
                        0x00ad_7550,
                        &args![instance, handle, 0x0107_3498_u32, 0x11_u32],
                    )
                    .u32();
                e.call(0x0041_8900, &args![sound, sample]);
                e.call(0x0048_3710, &args![handle]);
            });
            e.call(0x00ad_8830, &args![sound, 0_u32]);
        }
        hud_main_menu_update_radiation_meter_conditions(e, Ptr::new(menu), radiation, accumulated);
    }
}

// Translated from 00774800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetBrightness` (Xbox PDB): sets a tile's brightness for
/// the requested HUD role. Role 4 writes both 44 and 100 in that order.
pub fn hud_main_menu_set_brightness(e: &mut Engine, target: Ptr, role: u32) {
    match role {
        1 | 3 => hud_trait(e, target.addr(), 0xff5, 0xaf),
        2 => hud_trait(e, target.addr(), 0xff5, 0xff),
        4 => {
            hud_trait(e, target.addr(), 0xff5, 0x2c);
            hud_trait(e, target.addr(), 0xff5, 100);
        }
        5 => hud_trait(e, target.addr(), 0xff5, 100),
        _ => hud_trait(e, target.addr(), 0xff5, 0xff),
    }
}

fn x87_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(0x00ec_62c0, &args![value]).i32()
}

// Translated from 007748b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateMeter` (Xbox PDB): fits the requested fraction into
/// an integral number of image widths, then positions the meter for its
/// left-, right- or center-anchored mode.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn hud_main_menu_update_meter(
    e: &mut Engine,
    target: Ptr,
    mut fraction: f32,
    anchor: i32,
    minimum_images: i32,
    x_offset: i32,
) {
    for (bit, name, slot) in [
        (1_u32, 0x0107_3424_u32, 0x011d_9c14_u32),
        (2, 0x0107_3418, 0x011d_9c10),
        (4, 0x0107_3430, 0x011d_9c0c),
    ] {
        let flags = e.mem.u32(0x011d_9c18);
        if flags & bit == 0 {
            e.mem.set_u32(0x011d_9c18, flags | bit);
            let trait_id = e.call(0x00a0_0940, &args![name, u32::MAX]).u32();
            e.mem.set_u32(slot, trait_id);
        }
    }
    if target.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let one: f64 = e.global(0x0101_2070);
    if f64::from(fraction) > one {
        fraction = 1.0;
    } else if f64::from(fraction) < e.global::<f64>(0x0101_2060) {
        fraction = 0.0;
    }
    let width_trait = e.mem.u32(0x011d_9c14);
    let width_raw = e.call(0x00a0_11b0, &args![target, width_trait]).f64();
    let width = x87_to_int(e, width_raw);
    let image_trait = e.mem.u32(0x011d_9c10);
    let image_raw = e.call(0x00a0_11b0, &args![target, image_trait]).f64();
    let image = x87_to_int(e, image_raw);
    let zero: f64 = e.global(0x0101_2060);
    let positive = if f64::from(fraction) >= zero {
        fraction
    } else {
        0.0
    };
    let scale = if anchor == 2 {
        image.wrapping_shl(1)
    } else {
        image
    };
    let quotient = x87_to_int(e, f64::from(width) * f64::from(positive) / f64::from(scale));
    let mut displayed_width = quotient.wrapping_mul(scale);
    let minimum_width = image.wrapping_mul(minimum_images);
    if displayed_width < minimum_width {
        displayed_width = minimum_width;
    }
    let x_trait = e.mem.u32(0x011d_9c0c);
    let x_raw = e.call(0x00a0_11b0, &args![target, x_trait]).f64();
    let mut position = x87_to_int(e, x_raw);
    if anchor == 1 {
        position = position.wrapping_sub(displayed_width);
    } else if anchor == 2 {
        position = position.wrapping_sub(displayed_width / 2);
    }
    position = position.wrapping_add(x_offset);
    hud_trait(e, target.addr(), 0xfb1, displayed_width as u32);
    hud_trait(e, target.addr(), 0xfa1, position as u32);
}

// Translated from 00774b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00774b00 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::UpdateMeter_ov2` (Xbox PDB): sizes and positions two
/// overlapping meter tiles, including the center anchor's child tile and
/// alpha fades.
pub fn hud_main_menu_update_meter_ov2(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    mut first_fraction: f32,
    mut second_fraction: f32,
    anchor: i32,
    minimum_images: i32,
) {
    for (bit, name, slot) in [
        (1_u32, 0x0107_3424_u32, 0x011d_9c28_u32),
        (2, 0x0107_3418, 0x011d_9c24),
        (4, 0x0107_3430, 0x011d_9c20),
        (8, 0x0107_34b8, 0x011d_9c1c),
    ] {
        let flags = e.mem.u32(0x011d_9c2c);
        if flags & bit == 0 {
            e.mem.set_u32(0x011d_9c2c, flags | bit);
            let trait_id = e.call(0x00a0_0940, &args![name, u32::MAX]).u32();
            e.mem.set_u32(slot, trait_id);
        }
    }
    if first.is_null() || second.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    for fraction in [&mut first_fraction, &mut second_fraction] {
        if f64::from(*fraction) > e.global::<f64>(0x0101_2070) {
            *fraction = 1.0;
        } else if f64::from(*fraction) < e.global::<f64>(0x0101_2060) {
            *fraction = 0.0;
        }
    }
    let total_trait = e.mem.u32(0x011d_9c28);
    let total_raw = e.call(0x00a0_11b0, &args![first, total_trait]).f64();
    let total_width = x87_to_int(e, total_raw);
    let image_trait = e.mem.u32(0x011d_9c24);
    let image_raw = e.call(0x00a0_11b0, &args![first, image_trait]).f64();
    let image_width = x87_to_int(e, image_raw);
    let scale = if anchor == 2 {
        image_width.wrapping_shl(1)
    } else {
        image_width
    };
    let first_positive = if f64::from(first_fraction) >= e.global::<f64>(0x0101_2060) {
        first_fraction
    } else {
        0.0
    };
    let _first_width_before_second_conversion = x87_to_int(
        e,
        f64::from(total_width) * f64::from(first_positive) / f64::from(scale),
    )
    .wrapping_mul(scale);
    let mut second_width = x87_to_int(
        e,
        f64::from(total_width) * f64::from(second_fraction) / f64::from(scale),
    )
    .wrapping_mul(scale);
    // The PC body recalculates the first width after both conversions, so
    // preserve this callback-visible ordering rather than reuse first_width.
    let first_positive = if f64::from(first_fraction) >= e.global::<f64>(0x0101_2060) {
        first_fraction
    } else {
        0.0
    };
    let first_width = x87_to_int(
        e,
        f64::from(total_width) * f64::from(first_positive) / f64::from(scale),
    )
    .wrapping_mul(scale);
    let original_x_trait = e.mem.u32(0x011d_9c20);
    let original_x_raw = e.call(0x00a0_11b0, &args![first, original_x_trait]).f64();
    let mut first_x = x87_to_int(e, original_x_raw);
    if anchor == 1 {
        first_x = first_x.wrapping_sub(first_width);
    } else if anchor == 2 {
        first_x = first_x.wrapping_sub(first_width / 2);
    }
    let original_x_trait = e.mem.u32(0x011d_9c20);
    let original_x_raw = e.call(0x00a0_11b0, &args![second, original_x_trait]).f64();
    let mut second_x = x87_to_int(e, original_x_raw);
    if anchor == 1 {
        let width_difference = second_width.wrapping_sub(first_width);
        second_x = first_x.wrapping_sub(width_difference);
        let original_x_trait = e.mem.u32(0x011d_9c20);
        let original_x_raw = e.call(0x00a0_11b0, &args![second, original_x_trait]).f64();
        let original_x = x87_to_int(e, original_x_raw);
        second_width = e
            .call(
                0x004a_8f20,
                &args![width_difference, original_x.wrapping_sub(second_x)],
            )
            .i32();
    } else if anchor == 0 {
        second_x = second_x.wrapping_add(first_width);
    } else if anchor == 2 {
        second_x = second_x.wrapping_sub(second_width / 2);
        second_width = first_x.wrapping_sub(second_x);
        let child_storage = e.call(0x0073_ac90, &args![second.addr() + 4]).u32();
        let child = e.mem.u32(child_storage);
        hud_trait(e, child, 0xfb1, second_width as u32);
        hud_trait(e, child, 0xfa1, first_x.wrapping_add(first_width) as u32);
        if !e.call(0x00a0_7ed0, &args![child, 0xfa9_u32]).bool() {
            let duration: f32 = quiet_f32(e.global(0x0102_3cd8));
            e.call(
                0x00a0_7c60,
                &args![child, 0xfa9_u32, 0.0_f32, duration, 1.0_f32, 1_u32],
            );
        }
    }
    if first_width == 0 && second_width < image_width.wrapping_mul(minimum_images) {
        second_width = image_width.wrapping_mul(minimum_images);
    }
    hud_trait(e, first.addr(), 0xfb1, first_width as u32);
    hud_trait(e, first.addr(), 0xfa1, first_x as u32);
    hud_trait(e, second.addr(), 0xfb1, second_width as u32);
    hud_trait(e, second.addr(), 0xfa1, second_x as u32);
    if !e.call(0x00a0_7ed0, &args![second, 0xfa9_u32]).bool() {
        let duration: f32 = quiet_f32(e.global(0x0102_3cd8));
        e.call(
            0x00a0_7c60,
            &args![second, 0xfa9_u32, 0.0_f32, duration, 1.0_f32, 1_u32],
        );
    }
}

// Translated from 00774fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00774fd0 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::ShowSubtitle` (Xbox PDB): queues a subtitle when the text
/// and actor/setting gate allow it, deriving its duration from the sound
/// handle or the configured per-character fallback.
#[allow(clippy::too_many_arguments)]
pub fn hud_main_menu_show_subtitle(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    text: Ptr,
    sound_0: u32,
    sound_1: u32,
    sound_2: u32,
    point_0: u32,
    point_1: u32,
    point_2: u32,
    speaker: Ptr,
    force_priority: u8,
) -> bool {
    e.with_stack(12, |e, sound| {
        e.mem.set_u32(sound.addr(), sound_0);
        e.mem.set_u32(sound.addr() + 4, sound_1);
        e.mem.set_u32(sound.addr() + 8, sound_2);
        let mut shown = false;
        let actor = if speaker.is_null() || !e.vcall(speaker.addr(), 0x100, &args![]).bool() {
            0
        } else {
            e.call(
                0x00ec_43fb,
                &args![speaker, 0_u32, 0x0118_41cc_u32, 0x0118_46d4_u32, 0_u32],
            )
            .u32()
        };
        if !text.is_null() {
            let subtitles_setting = e.call(0x0040_8d60, &args![0x011d_8928_u32]).u32();
            let allowed = e.mem.u8(subtitles_setting) != 0
                || (actor != 0 && e.call(0x008c_1bc0, &args![actor]).bool());
            if allowed {
                let allocated = e.call(0x0040_1000, &args![0x11c_u32]).u32();
                let subtitle = if allocated == 0 {
                    0
                } else {
                    fn_00775210(e, Ptr::new(allocated)).addr()
                };
                e.call(0x0040_6d30, &args![subtitle, 0x104_u32, text]);
                for (offset, value) in [(0x10c, point_0), (0x110, point_1), (0x114, point_2)] {
                    e.mem.set_u32(subtitle + offset, value);
                }
                let duration_ms = e.call(0x00ad_8b30, &args![sound]).i32();
                let duration = (f64::from(duration_ms) / e.global::<f64>(0x0101_7b70)) as f32;
                e.mem.set_f32(subtitle + 0x108, duration);
                if f64::from(duration) == e.global::<f64>(0x0101_2060) {
                    let length = e.call(0x00ec_6130, &args![text]).u32();
                    let rate_setting = e.call(0x0040_3e20, &args![0x011d_2178_u32]).u32();
                    let fallback = (f64::from(e.mem.f32(rate_setting)) * f64::from(length)) as f32;
                    e.mem.set_f32(subtitle + 0x108, fallback);
                }
                let priority = if force_priority != 0 {
                    i32::MAX
                } else if speaker.addr() == e.mem.u32(0x011d_ea3c) {
                    0
                } else if speaker.is_null() {
                    1
                } else {
                    2
                };
                e.mem.set_i32(subtitle + 0x118, priority);
                let now = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
                e.mem.set_u32(subtitle + 0x104, now);
                e.with_stack(4, |e, pointer| {
                    e.mem.set_u32(pointer.addr(), subtitle);
                    e.call(0x007c_b2e0, &args![this.addr() + 0x198, pointer]);
                });
                shown = true;
            }
        }
        e.call(0x0048_3710, &args![sound]);
        shown
    })
}

// Translated from 00775210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the three-word point member at +0x10c in the newly
/// allocated subtitle record, then returns that record.
pub fn fn_00775210(e: &mut Engine, subtitle: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![subtitle.addr() + 0x10c]);
    subtitle
}

// Translated from 00775230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the subtitle record by squared distance from the player,
/// priority and the forced-priority sentinel. The returned pointer remains
/// in the menu's subtitle array.
pub fn fn_00775230(e: &mut Engine, this: Ptr<HUDMainMenu>) -> Ptr {
    let mut selected = 0;
    let mut best_distance: f32 = quiet_f32(e.global(0x0101_6970));
    let player = e.mem.u32(0x011d_ea3c);
    let player_position = e.vcall(player, 0x1f4, &args![]).u32();
    e.with_stack(24, |e, scratch| {
        let difference = scratch.addr();
        let copied_position = difference + 12;
        for offset in [0_u32, 4, 8] {
            e.mem.set_u32(
                copied_position + offset,
                e.mem.u32(player_position + offset),
            );
        }
        let mut index = 0_u32;
        loop {
            let count = e.call(0x0044_ddc0, &args![this.addr() + 0x198]).u32();
            if index >= count {
                break;
            }
            let element = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            let subtitle = e.mem.u32(element);
            let vector = e
                .call(
                    0x0043_9ef0,
                    &args![subtitle + 0x10c, difference, copied_position],
                )
                .u32();
            let distance = e.call(0x004a_7290, &args![vector]).f64() as f32;
            let in_range = if selected == 0 {
                let threshold: f32 = quiet_f32(e.global(0x0101_3d84));
                let threshold = e.call(0x0068_5d20, &args![threshold, 2_u32]).f64();
                f64::from(distance) <= threshold
            } else {
                true
            };
            let mut choose = selected == 0 && in_range;
            if selected != 0 && distance < best_distance {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                let subtitle = e.mem.u32(element);
                choose = e.mem.i32(subtitle + 0x118) >= e.mem.i32(selected + 0x118);
            }
            if !choose {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                let subtitle = e.mem.u32(element);
                choose = e.mem.i32(subtitle + 0x118) == i32::MAX;
            }
            if choose {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                selected = e.mem.u32(element);
                best_distance = distance;
            }
            index = index.wrapping_add(1);
        }
    });
    Ptr::new(selected)
}

// Translated from 00775380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00775380 - the compiler's SEH unwind frame is omitted.
/// Queues a HUD corner message, choosing its default icon from the PC jump
/// table when no path is supplied, and replacing or appending the current
/// message according to the list state.
#[allow(clippy::too_many_arguments)]
pub fn fn_00775380(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    text: Ptr,
    icon_kind: u32,
    icon_path: Ptr,
    sound_name: Ptr,
    duration: f32,
    urgent: u8,
) -> bool {
    if text.is_null() {
        return false;
    }
    let record = e.call(0x0040_1000, &args![0x310_u32]).u32();
    e.call(0x0040_6d30, &args![record, 0x104_u32, text]);
    e.mem.set_f32(record + 0x30c, quiet_f32(duration));
    if !icon_path.is_null() && e.mem.u8(icon_path.addr()) != 0 {
        e.call(0x0040_6d30, &args![record + 0x104, 0x104_u32, icon_path]);
    } else {
        // The four cases are ordered by the jump table at 00775658.
        let default_icon = match icon_kind {
            0 => Some(0x0102_08e0_u32),
            1 => Some(0x0103_a830),
            2 => Some(0x0102_08a0),
            3 => Some(0x0104_bfe8),
            _ => None,
        };
        if let Some(path) = default_icon {
            e.call(0x0040_6d30, &args![record + 0x104, 0x104_u32, path]);
        }
    }
    if sound_name.is_null() {
        e.mem.set_u8(record + 0x208, 0);
    } else {
        e.call(0x0040_6d30, &args![record + 0x208, 0x104_u32, sound_name]);
    }
    e.with_stack(4, |e, record_pointer| {
        e.mem.set_u32(record_pointer.addr(), record);
        let list = this.addr() + 0x18c;
        if e.call(0x0082_56d0, &args![list]).bool() {
            e.call(0x0090_5820, &args![list, record_pointer]);
            let first_node = e.call(0x0068_15c0, &args![list]).u32();
            let first_record = e.mem.u32(first_node);
            let menu = e.mem.u32(0x011d_96c0);
            e.call(
                0x00a0_1350,
                &args![e.mem.u32(menu + 0x90), 0xfc4_u32, first_record, 1_u32],
            );
            let first_node = e.call(0x0068_15c0, &args![list]).u32();
            let first_record = e.mem.u32(first_node);
            let menu = e.mem.u32(0x011d_96c0);
            e.call(
                0x00a0_1350,
                &args![
                    e.mem.u32(menu + 0x8c),
                    0xfcc_u32,
                    first_record + 0x104,
                    1_u32
                ],
            );
            if !sound_name.is_null() {
                e.with_stack(12, |e, sound| {
                    let instance = e.call(0x0045_3a70, &args![]).u32();
                    let handle = e
                        .call(0x00ad_7550, &args![instance, sound, sound_name, 0x121_u32])
                        .u32();
                    e.call(0x00ad_8830, &args![handle, 0_u32]);
                    e.call(0x0048_3710, &args![sound]);
                });
            }
        } else {
            let mut tail = list;
            loop {
                let next = e.call(0x0072_6070, &args![tail]).u32();
                if next == 0 {
                    break;
                }
                tail = e.call(0x0072_6070, &args![tail]).u32();
            }
            let node = e.call(0x0068_15c0, &args![tail]).u32();
            let old_record = e.mem.u32(node);
            let duplicate = e.call(0x0040_8b20, &args![old_record, record]).i32() == 0;
            if duplicate {
                e.call(0x0040_1030, &args![record]);
            } else if urgent == 0 {
                e.call(0x0090_5820, &args![list, record_pointer]);
            } else {
                let first_node = e.call(0x0068_15c0, &args![list]).u32();
                let first_record = e.mem.u32(first_node);
                e.mem.set_f32(first_record + 0x30c, 0.0);
                e.call(0x005a_e3d0, &args![list, record_pointer]);
            }
        }
    });
    true
}

// Translated from 00775670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HideSubtitle` (Xbox PDB): clears the visible subtitle tile
/// and the menu's current subtitle pointer, updating its text when the
/// dialogue menu is not visible.
pub fn hud_main_menu_hide_subtitle(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if !e.call(0x0070_2680, &args![0x3f5_u32, 0_u32]).bool() {
        let hud = e.mem.u32(0x011d_96c0);
        let subtitle_tile = e.mem.u32(hud + 0x98);
        e.call(
            0x00a0_1350,
            &args![subtitle_tile, 0xfc4_u32, 0x0102_0770_u32, 1_u32],
        );
    }
    let hud = e.mem.u32(0x011d_96c0);
    let subtitle_tile = e.mem.u32(hud + 0x98);
    hud_trait(e, subtitle_tile, 0xfa3, 0);
    e.mem.set_u32(this.addr() + 0x1a8, 0);
}

// Translated from 007756e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HideMessage` (Xbox PDB): removes the currently displayed
/// message from the linked list, then shows the next one or fades the three
/// message tiles when the list becomes empty.
pub fn hud_main_menu_hide_message(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let list = this.addr() + 0x18c;
    if e.call(0x0082_56d0, &args![list]).bool() {
        return;
    }
    let mut node = list;
    while node != 0 {
        let node_value = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(node_value);
        if record != 0 {
            let menu = e.mem.u32(0x011d_96c0);
            let tile = e.mem.u32(menu + 0x90);
            let displayed = e.call(0x00a0_11f0, &args![tile, 0xfc4_u32]).u32();
            if e.call(0x0040_8b20, &args![record, displayed]).i32() == 0 {
                e.with_stack(4, |e, record_pointer| {
                    e.mem.set_u32(record_pointer.addr(), record);
                    e.call(0x0090_5330, &args![list, record_pointer]);
                });
                e.call(0x0040_1030, &args![record]);
                if !e.call(0x0082_56d0, &args![list]).bool() {
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    if e.mem.u8(first_record + 0x208) != 0 {
                        let first_node = e.call(0x0068_15c0, &args![list]).u32();
                        let first_record = e.mem.u32(first_node);
                        e.with_stack(12, |e, sound| {
                            let player = e.mem.u32(0x011d_ea3c);
                            e.call(
                                0x0093_3270,
                                &args![
                                    player,
                                    sound,
                                    first_record + 0x208,
                                    0_u32,
                                    0x121_u32,
                                    0_u32
                                ],
                            );
                            e.call(0x0048_3710, &args![sound]);
                        });
                    }
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    let menu = e.mem.u32(0x011d_96c0);
                    let tile = e.mem.u32(menu + 0x90);
                    e.call(0x00a0_1350, &args![tile, 0xfc4_u32, first_record, 1_u32]);
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    let menu = e.mem.u32(0x011d_96c0);
                    let tile = e.mem.u32(menu + 0x8c);
                    e.call(
                        0x00a0_1350,
                        &args![tile, 0xfcc_u32, first_record + 0x104, 1_u32],
                    );
                    let now = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
                    e.mem.set_u32(this.addr() + 0x194, now);
                } else {
                    for offset in [0x90_u32, 0x8c, 0x94] {
                        let start = quiet_f32(e.mem.f32(0x011d_979c));
                        let duration: f32 = quiet_f32(e.global(0x0102_1f44));
                        let menu = e.mem.u32(0x011d_96c0);
                        let tile = e.mem.u32(menu + offset);
                        e.call(
                            0x00a0_7c60,
                            &args![tile, 0xfa9_u32, start, 0.0_f32, duration, 0_u32],
                        );
                    }
                    e.mem.set_u32(this.addr() + 0x194, 0);
                }
                return;
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 00775920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every queued HUD message, clears the message tiles and hides the
/// subtitle in the order used by the PC menu.
pub fn fn_00775920(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let list = this.addr() + 0x18c;
    let mut node = list;
    while node != 0 {
        if e.call(0x0082_56d0, &args![node]).bool() {
            break;
        }
        let node_value = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(node_value);
        e.call(0x0040_1030, &args![record]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![list]);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x94);
    hud_set_float(e, tile, 0xfa9, 0.0);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x90);
    hud_set_text(e, tile, 0x0102_0770);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x90);
    hud_set_float(e, tile, 0xfa9, 0.0);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x8c);
    hud_set_float(e, tile, 0xfa9, 0.0);
    hud_main_menu_hide_subtitle(e, this);
}

// Translated from 00773e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Displays a status icon and its alpha according to four configured
/// thresholds, then advances its shared blinking alpha when requested.
pub fn fn_00773e90(
    e: &mut Engine,
    _this: Ptr<HUDMainMenu>,
    target: Ptr,
    value: f32,
    advance_blink: bool,
) {
    let threshold = |e: &mut Engine, setting| {
        let address = e.call(0x0040_3e20, &args![setting]).u32();
        e.mem.f32(address)
    };
    let high = threshold(e, 0x011d_07fc_u32);
    if value >= high {
        hud_trait(e, target.addr(), 0xff4, 2);
        let alpha = e.mem.f32(0x011d_9b04);
        hud_set_float(e, target.addr(), 0xfa9, alpha);
    } else if value >= threshold(e, 0x011d_132c_u32) {
        hud_trait(e, target.addr(), 0xff4, 2);
        hud_trait(e, target.addr(), 0xfa9, 0xff);
    } else if value >= threshold(e, 0x011d_0c84_u32) {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0xff);
    } else if value >= threshold(e, 0x011d_05a4_u32) {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0x7c);
    } else {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0);
    }
    if !advance_blink {
        return;
    }
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    let elapsed = now.wrapping_sub(e.mem.u32(0x011d_9b00));
    e.mem.set_u32(0x011d_9b00, now);
    let rate: f32 = e.global(0x0101_6248);
    let alpha = e.mem.f32(0x011d_9b04);
    let change = f64::from(elapsed) * f64::from(rate);
    let next = if e.mem.u8(0x011d_9afc) != 0 {
        (f64::from(alpha) - change) as f32
    } else {
        (f64::from(alpha) + change) as f32
    };
    e.mem.set_f32(0x011d_9b04, next);
    if next >= e.global::<f64>(0x0101_e568) as f32 {
        e.mem.set_u8(0x011d_9afc, 1);
    }
    if next <= e.global::<f64>(0x0102_e430) as f32 {
        e.mem.set_u8(0x011d_9afc, 0);
    }
    let minimum: f32 = e.global(0x0101_e704);
    let maximum: f32 = e.global(0x0102_3cd8);
    e.call(0x0053_30e0, &args![0x011d_9b04_u32, minimum, maximum]);
}

fn radiation_bar_position(e: &mut Engine, amount: f32) -> f32 {
    let limit: f32 = e.global(0x0101_3974);
    let bounded = e.call(0x0040_ebd0, &args![amount, limit]).f32();
    let bounded = e.call(0x0040_4010, &args![0.0_f32, bounded]).f32();
    let low: f32 = e.global(0x0101_e580);
    let high: f32 = e.global(0x0107_34b4);
    e.call(0x004b_3ab0, &args![low, high, 0.0_f32, limit, bounded])
        .f32()
}

fn radiation_fade(e: &mut Engine, target: u32, alpha: f32) {
    // The original reads the start alpha from the first radiation tile for
    // every tile in the group, including the other three.
    let start_tile = hud_tile(e, 0x11);
    let start = hud_float(e, start_tile, 0xfa9);
    let duration: f32 = e.global(0x0101_622c);
    e.call(
        0x00a0_7c60,
        &args![target, 0xfa9_u32, start, alpha, duration, 0_u32],
    );
}

// Translated from 00774090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateRadiationMeter` (Xbox PDB): moves and fades the
/// radiation gauge during and after a radiation event.
pub fn hud_main_menu_update_radiation_meter(e: &mut Engine, _this: Ptr<HUDMainMenu>) {
    let meter = hud_tile(e, 0x13);
    let expiry = hud_float(e, meter, 0x100a);
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    let pending = f64::from(now) < f64::from(expiry);
    let status = e.call(0x0050_98e0, &args![0x011e_0e80_u32]).bool();
    if !status && !pending && e.mem.u32(0x011a_0120) == 4 {
        return;
    }
    let mut transition = false;
    let zero: f64 = e.global(0x0101_2060);
    let mut all_hidden = true;
    for id in 0x11..=0x14 {
        let item = hud_tile(e, id);
        if !e.call(0x00a0_7fc0, &args![item, 0xfa9_u32]).bool()
            || f64::from(hud_float(e, item, 0xfa9)) != zero
        {
            all_hidden = false;
            break;
        }
    }
    if all_hidden {
        e.mem.set_u32(0x011a_0120, 4);
    }
    let state = e.mem.u32(0x011a_0120);
    if (state == 4 || state == 2) && (status || pending) {
        e.mem.set_u32(0x011a_0120, 8);
        transition = true;
    }
    let meter = hud_tile(e, 0x13);
    let marked = e.call(0x00a0_1230, &args![meter, 0x100b_u32]).bool();
    let pending_gauge = marked || pending;
    let mut difference = 0.0_f32;
    let mut actual_position = 0.0_f32;
    let mut difference_position = 0.0_f32;
    if status || pending || transition {
        let player = e.mem.u32(0x011d_ea3c);
        let raw = e.vcall(player + 0xa4, 8, &args![0x36_u32]).i32() as f32;
        let mut display_text = false;
        if status || transition || pending_gauge {
            display_text = true;
            let meter = hud_tile(e, 0x13);
            difference = hud_float(e, meter, 0x1009);
            difference_position =
                radiation_bar_position(e, (f64::from(raw) - f64::from(difference)) as f32);
        }
        if (pending && transition) || pending_gauge || status {
            display_text = true;
            actual_position = radiation_bar_position(e, raw);
        }
        if display_text {
            let amount = x87_to_int(e, f64::from(difference));
            let text_number = e.call(0x0064_7b70, &args![1_u32, amount]).u32();
            e.call(
                0x0040_6d00,
                &args![0x011d_9b08_u32, 0x104_u32, 0x0107_34b0_u32, text_number],
            );
            let meter = hud_tile(e, 0x13);
            hud_set_text(e, meter, 0x011d_9b08);
        }
    }
    let upper: f32 = e.global(0x0107_34b4);
    if transition {
        e.mem.set_u32(0x011a_0120, 8);
        let selected = if status {
            actual_position
        } else {
            difference_position
        };
        let limited = e.call(0x0040_ebd0, &args![selected, upper]).f32();
        let target = hud_tile(e, 0x12);
        hud_set_float(e, target, 0xfa1, limited);
        let indicator = hud_tile(e, 0x3d);
        hud_trait(e, indicator, 0xfa3, 1);
        let alpha: f32 = e.global(0x011d_979c);
        for id in 0x11..=0x14 {
            let target = hud_tile(e, id);
            radiation_fade(e, target, alpha);
        }
    } else if status {
        let limited = e.call(0x0040_ebd0, &args![actual_position, upper]).f32();
        let target = hud_tile(e, 0x12);
        hud_set_float(e, target, 0xfa1, limited);
    }
    if pending_gauge {
        let meter = hud_tile(e, 0x13);
        hud_trait(e, meter, 0x100b, 0);
        let limited = e.call(0x0040_ebd0, &args![actual_position, upper]).f32();
        let gauge = hud_tile(e, 0x12);
        hud_set_float(e, gauge, 0x1004, limited);
        let fifty: f32 = e.global(0x0101_b268);
        let limited_amount = e.call(0x0040_ebd0, &args![fifty, difference]).f32();
        let start_time: f32 = e.global(0x0101_6088);
        let end_time: f32 = e.global(0x0101_7718);
        let duration = e
            .call(
                0x004b_3ab0,
                &args![start_time, end_time, 0.0_f32, fifty, limited_amount],
            )
            .f32();
        let gauge = hud_tile(e, 0x12);
        let start = hud_float(e, gauge, 0xfa1);
        let position = hud_float(e, gauge, 0x1004);
        let floor: f32 = e.global(0x0101_e580);
        let end = e.call(0x0040_4010, &args![position, floor]).f32();
        if !status {
            let gauge = hud_tile(e, 0x12);
            e.call(
                0x00a0_7c60,
                &args![gauge, 0xfa1_u32, start, end, duration, 0_u32],
            );
        }
    }
    if !status && !pending {
        e.mem.set_u32(0x011a_0120, 2);
        for id in 0x11..=0x14 {
            let target = hud_tile(e, id);
            radiation_fade(e, target, 0.0);
        }
    }
}

fn sneak_alpha(e: &mut Engine) -> f32 {
    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32
}

fn sneak_set_label(e: &mut Engine, setting: u32) {
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let tile = hud_tile(e, 0x18);
    hud_set_text(e, tile, text);
}

// Translated from 007732d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateSneak` (Xbox PDB): selects the sneak status text,
/// image and alpha fade from combat, discovery and the player state.
pub fn hud_main_menu_update_sneak(e: &mut Engine, _this: Ptr<HUDMainMenu>) {
    e.with_stack(1, |e, combat_detail| {
        e.mem.set_u8(combat_detail.addr(), 0);
        let player = e.mem.u32(0x011d_ea3c);
        let combat = e.call(0x0095_3c50, &args![player, combat_detail]).bool();
        let detail = e.mem.u8(combat_detail.addr()) != 0;
        let mut alert_fade = false;
        if combat || detail {
            let tile = hud_tile(e, 0x18);
            hud_trait(e, tile, 0xff4, 2);
            alert_fade = !detail;
            sneak_set_label(e, if detail { 0x011d_4804 } else { 0x011d_4eb8 });
        } else {
            let player = e.mem.u32(0x011d_ea3c);
            if fn_007735b0(e, Ptr::new(player)) != 0 {
                let tile = hud_tile(e, 0x18);
                hud_trait(e, tile, 0xff4, 2);
                sneak_set_label(e, 0x011d_4804);
            } else {
                let player = e.mem.u32(0x011d_ea3c);
                let detected = e
                    .call(0x0097_3710, &args![0x011e_0e80_u32, player, 0_u32])
                    .i32();
                let tile = hud_tile(e, 0x18);
                hud_trait(e, tile, 0xff4, 1);
                if detected <= 0 {
                    sneak_set_label(e, 0x011d_27b4);
                } else {
                    let player = e.mem.u32(0x011d_ea3c);
                    if fn_007735b0(e, Ptr::new(player)) == 0 {
                        sneak_set_label(e, 0x011d_2490);
                    }
                }
            }
        }
        let tile = hud_tile(e, 0x18);
        if alert_fade && !e.call(0x00a0_1230, &args![tile, 0x1004_u32]).bool() {
            let alpha = sneak_alpha(e);
            let duration: f32 = e.global(0x0101_6248);
            let tile = hud_tile(e, 0x18);
            e.call(
                0x00a0_7c60,
                &args![tile, 0xfa9_u32, 0.0_f32, alpha, duration, 2_u32],
            );
        } else {
            let fade_type = if alert_fade {
                -1
            } else {
                let tile = hud_tile(e, 0x18);
                e.call(0x00a0_8070, &args![tile, 0xfa9_u32]).i32()
            };
            let needs_fade = if !alert_fade && fade_type == 2 {
                true
            } else {
                let tile = hud_tile(e, 0x18);
                if !e.call(0x00a0_7fc0, &args![tile, 0xfa9_u32]).bool() {
                    false
                } else {
                    let tile = hud_tile(e, 0x18);
                    let current = hud_float(e, tile, 0xfa9);
                    let alpha = sneak_alpha(e);
                    current != alpha
                }
            };
            if needs_fade {
                let alpha = sneak_alpha(e);
                let duration: f32 = e.global(0x0101_6248);
                let tile = hud_tile(e, 0x18);
                let current = hud_float(e, tile, 0xfa9);
                let tile = hud_tile(e, 0x18);
                e.call(
                    0x00a0_7c60,
                    &args![tile, 0xfa9_u32, current, alpha, duration, 0_u32],
                );
            }
        }
        let tile = hud_tile(e, 0x18);
        hud_trait(e, tile, 0x1004, u32::from(alert_fade));
    });
}

// Translated from 00772d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetTargetType` (Xbox PDB): decides whether the aimed actor
/// can become the health target, then updates the enemy tiles and cached
/// perceived/essential flags.
pub fn hud_main_menu_set_target_type(e: &mut Engine, target: Ptr, enabled: bool) {
    let mut perceived = false;
    let mut actor_blocked = true;
    let blocked = if target.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        true
    } else {
        let blocked_ptr = e.call(0x0040_8d60, &args![0x011d_8a0c_u32]).u32();
        if e.mem.u8(blocked_ptr) != 0 || !enabled {
            true
        } else {
            let iron_ptr = e.call(0x0040_8d60, &args![0x011d_898c_u32]).u32();
            if e.mem.u8(iron_ptr) != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                e.call(0x008b_bc10, &args![player]).bool()
            } else {
                false
            }
        }
    };
    if blocked {
        let icon = hud_tile(e, 0x3e);
        let one: f64 = e.global(0x0101_2070);
        if e.call(0x00a0_11b0, &args![icon, 0xff4_u32]).f64() == one {
            for id in [0x15_u32, 0x16, 0x17] {
                let item = hud_tile(e, id);
                hud_trait(e, item, 0xfa3, 0);
            }
        }
    } else {
        e.call(0x0057_9280, &args![target]);
        let actor = if e.vcall(target.addr(), 0x100, &args![]).bool() {
            target.addr()
        } else {
            0
        };
        actor_blocked = actor != 0 && e.vcall(actor, 0x22c, &args![0_u32]).bool();
        if actor != 0 {
            let player = e.mem.u32(0x011d_ea3c);
            let observation = e.call(0x0096_7090, &args![player, actor]).u32();
            if observation != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                let observation = e.call(0x0096_7090, &args![player, actor]).u32();
                if e.mem.u8(observation + 4) != 0 {
                    perceived = true;
                }
            }
            if !perceived {
                let player = e.mem.u32(0x011d_ea3c);
                perceived = e.call(0x008b_c700, &args![actor, player]).bool();
            }
        }
        let mut show = false;
        if actor != 0 && !actor_blocked && !e.vcall(actor, 0x22c, &args![0_u32]).bool() {
            let current = e.vcall(actor + 0xa4, 8, &args![0x10_u32]).i32();
            let maximum = e.vcall(actor + 0xa4, 0, &args![0x10_u32]).i32();
            if current < maximum {
                let meter = hud_tile(e, 0x20);
                show = !e.call(0x00a0_1230, &args![meter, 0xfa3_u32]).bool();
            }
        }
        if show {
            let menu = e.mem.u32(0x011d_96c0);
            hud_main_menu_set_enemy_actor(e, Ptr::new(menu), Ptr::new(actor));
            for id in [0x15_u32, 0x16, 0x17, 0x17] {
                let item = hud_tile(e, id);
                hud_trait(e, item, 0xfa3, 1);
            }
            let indicator = hud_tile(e, 0x3e);
            hud_trait(e, indicator, 0xff4, u32::from(perceived) + 1);
            for id in [0x15_u32, 0x16, 0x17, 0x17] {
                let item = hud_tile(e, id);
                e.call(0x00a0_7530, &args![item, 8_u32]);
            }
            let menu = e.mem.u32(0x011d_96c0);
            hud_main_menu_update_enemy_health(e, Ptr::new(menu), Ptr::new(actor));
        }
    }
    e.mem
        .set_u8(0x011d_96ba, u8::from(!actor_blocked && perceived));
    e.mem.set_u8(0x011d_96b9, u8::from(enabled));
}

fn inventory_count_for(e: &mut Engine, player: u32, item: u32) -> i32 {
    let changes = e.call(0x004b_f220, &args![player]).u32();
    e.call(0x004c_8f30, &args![changes, item]).i32()
}

// Translated from 007721c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the equipped weapon's ammunition and condition indicators.
/// The PC body reads the selected item from `MiddleHighProcess`, including
/// a separate branch for form types 10 through 13.
pub fn fn_007721c0(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let player = e.mem.u32(0x011d_ea3c);
    if player == 0 {
        return;
    }
    let process = e.call(0x008d_8520, &args![player]).u32();
    if process == 0 {
        return;
    }
    let alpha_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(alpha_setting)) * e.global::<f64>(0x0101_e568)) as f32;
    let player = e.mem.u32(0x011d_ea3c);
    let process = e.call(0x008d_8520, &args![player]).u32();
    let item_change = e.vcall(process, 0x148, &args![]).u32();
    if e.mem.u32(0x011d_9ae8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9ae8);
        e.mem.set_u32(0x011d_9ae8, flags | 1);
        e.mem.set_f32(0x011d_9ae4, alpha);
    }
    e.call(
        0x0040_6d00,
        &args![0x011d_99e0_u32, 0x104_u32, 0x0101_9f08_u32, 0x0101_1584_u32],
    );
    let ammo_label = e.mem.u32(this.addr() + 0x15c);
    hud_set_text(e, ammo_label, 0x011d_99e0);
    if item_change == 0 {
        let sentinel: f32 = e.global(0x0102_efc4);
        for address in [0x011a_011c_u32, 0x011a_0118, 0x011a_0110] {
            e.mem.set_f32(address, sentinel);
        }
        e.mem.set_u32(0x011d_98d4, 0);
        for address in [
            hud_tile(e, 0x0c),
            e.mem.u32(this.addr() + 0x50),
            e.mem.u32(this.addr() + 0x54),
            hud_tile(e, 0x09),
            e.mem.u32(this.addr() + 0x180),
        ] {
            hud_trait(e, address, 0xfa3, 0);
        }
        return;
    }
    let weapon = e.call(0x0044_ddc0, &args![item_change]).u32();
    let mut loaded = 0.0_f32;
    let mut reserve = 0.0_f32;
    let form_type = e.call(0x0044_6390, &args![weapon]).i32();
    if (10..=13).contains(&form_type) {
        loaded = e.call(0x0052_4b60, &args![weapon]).u8() as f32;
        let player = e.mem.u32(0x011d_ea3c);
        let total = inventory_count_for(e, player, weapon);
        reserve = (f64::from(total) - f64::from(loaded)) as f32;
    } else {
        let player = e.mem.u32(0x011d_ea3c);
        let process = e.call(0x008d_8520, &args![player]).u32();
        let acquired = e.vcall(process, 0x14c, &args![]).u32();
        let mut converted = 0;
        if acquired != 0 {
            let acquired_form = e.call(0x0044_ddc0, &args![acquired]).u32();
            if acquired_form != 0 {
                let acquired_form = e.call(0x0044_ddc0, &args![acquired]).u32();
                converted = e
                    .call(
                        0x00ec_43fb,
                        &args![
                            acquired_form,
                            0_u32,
                            0x0118_3108_u32,
                            0x0118_40c4_u32,
                            0_u32
                        ],
                    )
                    .u32();
            }
        }
        if converted != 0 {
            let attachment = fn_00772820(e, Ptr::new(converted)).addr();
            let name = if attachment == 0 {
                0x0101_1584
            } else {
                attachment
            };
            e.call(
                0x0040_6d00,
                &args![0x011d_98d8_u32, 0x104_u32, 0x0101_9f08_u32, name],
            );
            let ammo_label = e.mem.u32(this.addr() + 0x15c);
            hud_set_text(e, ammo_label, 0x011d_98d8);
            let shifted = e.mem.u8(0x011d_96b8) != 0;
            if attachment == 0 && shifted {
                e.call(0x0077_f890, &args![this, 0x32_i32]);
                e.mem.set_u8(0x011d_96b8, 0);
            } else if attachment != 0 && !shifted {
                e.call(0x0077_f890, &args![this, -0x32_i32]);
                e.mem.set_u8(0x011d_96b8, 1);
            }
        }
        let modification = e.call(0x004b_da70, &args![item_change, 6_u32]).bool();
        let rate = tes_object_weap_get_ammo_regen_rate(e, Ptr::new(weapon), modification);
        let mut subtract_loaded = true;
        if rate > e.global::<f64>(0x0101_2060) {
            let modification = e.call(0x004b_da70, &args![item_change, 2_u32]).bool();
            reserve = e.call(0x004f_e160, &args![weapon, modification]).i32() as f32;
            subtract_loaded = false;
        }
        let player = e.mem.u32(0x011d_ea3c);
        let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
        if ammo != 0 && acquired != 0 {
            loaded = e.call(0x0072_6070, &args![acquired]).i32() as f32;
            if subtract_loaded {
                let player = e.mem.u32(0x011d_ea3c);
                let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
                let player = e.mem.u32(0x011d_ea3c);
                let count = inventory_count_for(e, player, ammo);
                reserve = (f64::from(count) - f64::from(loaded)) as f32;
            }
        } else if e.call(0x0047_4a80, &args![weapon + 0xa4]).bool() {
            let player = e.mem.u32(0x011d_ea3c);
            loaded = e.call(0x0096_9040, &args![player]).i32() as f32;
            reserve = loaded;
        }
    }
    if e.call(0x0064_50c0, &args![weapon]).bool() {
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_trait(e, ammo_count_tile, 0xfa3, 0);
    } else if loaded != e.mem.f32(0x011a_011c)
        || reserve != e.mem.f32(0x011a_0118)
        || weapon != e.mem.u32(0x011d_98d4)
    {
        let reserve_int = x87_to_int(e, f64::from(reserve));
        let loaded_int = x87_to_int(e, f64::from(loaded));
        e.call(
            0x0040_6d00,
            &args![
                0x011d_97d0_u32,
                0x104_u32,
                0x0107_14ec_u32,
                loaded_int,
                reserve_int
            ],
        );
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_set_text(e, ammo_count_tile, 0x011d_97d0);
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_trait(e, ammo_count_tile, 0xfa3, 1);
        e.mem.set_f32(0x011a_011c, loaded);
        e.mem.set_f32(0x011a_0118, reserve);
    }
    if e.mem.u32(0x011d_98d4) != weapon {
        for offset in [0x4c_u32, 0x50, 0x54, 0x180] {
            let item = e.mem.u32(this.addr() + offset);
            hud_trait(e, item, 0xfa3, 1);
        }
        e.mem.set_u32(0x011d_98d4, weapon);
        if e.mem.u8(0x011d_96b8) != 0 {
            e.call(0x0077_f890, &args![this, 0x32_i32]);
        }
        e.mem.set_u8(0x011d_96b8, 0);
    }
    let _first_health = e.call(0x004b_cdb0, &args![item_change, 0_u32]).f32();
    let condition = e.call(0x004b_cdb0, &args![item_change, 1_u32]).f32();
    let warning_limit: f64 = e.global(0x0104_f2f0);
    if f64::from(condition) < warning_limit {
        e.call(0x0053_30e0, &args![0x011d_9ae4_u32, 0.0_f32, alpha]);
        let current_alpha = e.mem.f32(0x011d_9ae4);
        if current_alpha == 0.0 || current_alpha == alpha {
            let old = e.mem.u8(0x011a_0114);
            e.mem.set_u8(0x011a_0114, u8::from(old == 0));
        }
        let scale: f64 = e.global(0x0102_0758);
        let maximum: f64 = e.global(0x0101_e568);
        let change = f64::from(alpha) / maximum * scale;
        let next = if e.mem.u8(0x011a_0114) != 0 {
            (f64::from(current_alpha) - change) as f32
        } else {
            (f64::from(current_alpha) + change) as f32
        };
        e.mem.set_f32(0x011d_9ae4, next);
    } else {
        e.mem.set_f32(0x011d_9ae4, alpha);
    }
    let condition_tile = e.mem.u32(this.addr() + 0x50);
    let condition_alpha = e.mem.f32(0x011d_9ae4);
    hud_set_float(e, condition_tile, 0xfa9, condition_alpha);
    if condition != e.mem.f32(0x011a_0110) {
        let health = e.call(0x004b_cdb0, &args![item_change, 0_u32]).f32();
        let has_mod = e.call(0x004b_da70, &args![item_change, 10_u32]).bool();
        let maximum = e.call(0x004b_cf00, &args![weapon, has_mod]).i32() as f32;
        let ratio = (f64::from(health) / f64::from(maximum)) as f32;
        let ratio = e.call(0x0040_4010, &args![0.0_f32, ratio]).f32();
        let ratio = e.call(0x0040_ebd0, &args![ratio, 1.0_f32]).f32();
        let offset = if e.mem.u8(0x011d_96b8) != 0 { -50 } else { 0 };
        let health_tile = e.mem.u32(this.addr() + 0x50);
        hud_main_menu_update_meter(e, Ptr::new(health_tile), ratio, 0, 0, offset);
        e.mem.set_f32(0x011a_0110, condition);
    }
}

// Translated from 007739f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateStatusModeUI` (Xbox PDB): sets the five status
/// labels, counts fully damaged body parts, and updates the condition icons.
pub fn hud_main_menu_update_status_mode_ui(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    status_mode: bool,
) {
    let menu = e.mem.u32(0x011d_96c0);
    for (setting, offset) in [
        (0x011d_4d74_u32, 0x17c_u32),
        (0x011d_3e74, 0x178),
        (0x011d_1f08, 0x164),
        (0x011d_37c8, 0x16c),
        (0x011d_246c, 0x168),
    ] {
        let label = e.call(0x0040_3df0, &args![setting]).u32();
        let target = e.mem.u32(menu + offset);
        hud_set_text(e, target, label);
    }
    let player = e.mem.u32(0x011d_ea3c);
    let values = if player == 0 { 0 } else { player + 0xa4 };
    let mut destroyed = 0_u8;
    for part in [0x1b_u32, 0x1d, 0x1e, 0x1c, 0x19, 0x1a] {
        let condition = e.call(0x0064_6800, &args![values, part, 0_u32]).f32();
        if condition == 0.0 {
            destroyed = destroyed.wrapping_add(1);
        }
    }
    let condition = if destroyed == 0 {
        0.0
    } else {
        let setting = match destroyed {
            1 => 0x011d_05a4,
            2 => 0x011d_0c84,
            3 => 0x011d_132c,
            _ => 0x011d_07fc,
        };
        let ptr = e.call(0x0040_3e20, &args![setting]).u32();
        (f64::from(e.mem.f32(ptr)) + 1.0) as f32
    };
    let body_part_value = |e: &mut Engine, id: u32, argument: u32| {
        e.vcall(values, 8, &args![id, argument]).i32() as f32
    };
    let radiation = body_part_value(e, 0x36, 0);
    let radiation_tile = e.mem.u32(menu + 0x178);
    fn_00773e90(e, this, Ptr::new(radiation_tile), radiation, false);
    let damaged_limb_tile = e.mem.u32(menu + 0x17c);
    fn_00773e90(
        e,
        this,
        Ptr::new(damaged_limb_tile),
        condition,
        !status_mode,
    );
    if !status_mode {
        for offset in [0x164_u32, 0x168, 0x16c] {
            let target = e.mem.u32(menu + offset);
            hud_set_float(e, target, 0xfa9, 0.0);
        }
    } else {
        for (offset, id, argument) in [
            (0x164_u32, 0x49_u32, 0_u32),
            (0x168, 0x4b, 0),
            (0x16c, 0x4a, 1),
        ] {
            let value = body_part_value(e, id, argument);
            let item = e.mem.u32(menu + offset);
            fn_00773e90(e, this, Ptr::new(item), value, false);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_9430, tes_object_weap_get_ammo_regen_rate(Ptr, bool) -> f64),
        entry!(0x0076_b960, fn_0076b960(Ptr, Ptr, i32, u32, i32, i32, Ptr)),
        entry!(
            0x0076_bdc0,
            hud_main_menu_get_class(Ptr<HUDMainMenu>) -> u32
        ),
        entry!(
            0x0076_bdd0,
            hud_main_menu_attach_tile_by_id(Ptr<HUDMainMenu>, i32, Ptr)
        ),
        entry!(
            0x0076_be00,
            hud_main_menu_scalar_deleting_destructor(Ptr<HUDMainMenu>, u32) -> Ptr<HUDMainMenu>
        ),
        entry!(0x0076_be30, fn_0076be30(Ptr<HUDMainMenu>)),
        entry!(0x0076_bfe0, hud_main_menu_create() -> Ptr),
        entry!(0x0077_0070, fn_00770070(Ptr<HUDMainMenu>) -> bool),
        entry!(0x0077_00b0, fn_007700b0(Ptr<HUDMainMenu>, Ptr)),
        entry!(0x0077_00d0, fn_007700d0(Ptr<HUDMainMenu>, u8)),
        entry!(0x0077_00f0, fn_007700f0(Ptr, Ptr) -> f64),
        entry!(
            0x0077_01e0,
            hud_main_menu_update_enemy_health(Ptr<HUDMainMenu>, Ptr)
        ),
        entry!(0x0077_0430, hud_main_menu_do_idle(Ptr<HUDMainMenu>)),
        entry!(0x0077_1520, fn_00771520(Ptr)),
        entry!(
            0x0077_1540,
            hud_main_menu_update_actor_value(Ptr<HUDMainMenu>, u32)
        ),
        entry!(0x0077_1700, hud_main_menu_set_menu_mode(i32)),
        entry!(0x0077_21a0, fn_007721a0(Ptr) -> Ptr),
        entry!(0x0077_21c0, fn_007721c0(Ptr<HUDMainMenu>)),
        entry!(0x0077_2820, fn_00772820(Ptr) -> Ptr),
        entry!(0x0077_2840, hud_main_menu_heading_between_locations(Ptr, Ptr) -> f64),
        entry!(0x0077_28f0, hud_main_menu_set_region_text(Ptr)),
        entry!(0x0077_2aa0, fn_00772aa0(Ptr, Ptr)),
        entry!(0x0077_2c30, hud_main_menu_set_worldspace_text(Ptr)),
        entry!(0x0077_2d10, hud_main_menu_set_target_type(Ptr, bool)),
        entry!(0x0077_3050, fn_00773050()),
        entry!(
            0x0077_3190,
            hud_main_menu_set_enemy_actor(Ptr<HUDMainMenu>, Ptr)
        ),
        entry!(0x0077_32b0, fn_007732b0() -> Ptr),
        entry!(0x0077_32d0, hud_main_menu_update_sneak(Ptr<HUDMainMenu>)),
        entry!(0x0077_35b0, fn_007735b0(Ptr) -> u8),
        entry!(
            0x0077_35d0,
            hud_main_menu_is_quest_name_showing(Ptr<HUDMainMenu>) -> bool
        ),
        entry!(
            0x0077_3650,
            hud_main_menu_are_objectives_showing(Ptr<HUDMainMenu>) -> bool
        ),
        entry!(0x0077_36d0, hud_main_menu_display_quest_reminder(Ptr)),
        entry!(0x0077_37a0, fn_007737a0(Ptr, u32) -> bool),
        entry!(
            0x0077_37c0,
            hud_main_menu_update_radiation_meter_conditions(Ptr<HUDMainMenu>, f32, f32)
        ),
        entry!(0x0077_38c0, hud_main_menu_set_radiation_level(f32, f32)),
        entry!(
            0x0077_39f0,
            hud_main_menu_update_status_mode_ui(Ptr<HUDMainMenu>, bool)
        ),
        entry!(0x0077_3e90, fn_00773e90(Ptr<HUDMainMenu>, Ptr, f32, bool)),
        entry!(
            0x0077_4090,
            hud_main_menu_update_radiation_meter(Ptr<HUDMainMenu>)
        ),
        entry!(0x0077_4800, hud_main_menu_set_brightness(Ptr, u32)),
        entry!(
            0x0077_48b0,
            hud_main_menu_update_meter(Ptr, f32, i32, i32, i32)
        ),
        entry!(
            0x0077_4b00,
            hud_main_menu_update_meter_ov2(Ptr, Ptr, f32, f32, i32, i32)
        ),
        entry!(
            0x0077_4fd0,
            hud_main_menu_show_subtitle(
                Ptr<HUDMainMenu>,
                Ptr,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                Ptr,
                u8,
            ) -> bool
        ),
        entry!(0x0077_5210, fn_00775210(Ptr) -> Ptr),
        entry!(0x0077_5230, fn_00775230(Ptr<HUDMainMenu>) -> Ptr),
        entry!(
            0x0077_5380,
            fn_00775380(Ptr<HUDMainMenu>, Ptr, u32, Ptr, Ptr, f32, u8) -> bool
        ),
        entry!(0x0077_5670, hud_main_menu_hide_subtitle(Ptr<HUDMainMenu>)),
        entry!(0x0077_56e0, hud_main_menu_hide_message(Ptr<HUDMainMenu>)),
        entry!(0x0077_5920, fn_00775920(Ptr<HUDMainMenu>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ammo_regen_includes_mod_effect_only_when_requested() {
        let mut e = Engine::new();
        let weapon: Ptr = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_f32(weapon.addr() + 0x1a4, 2.5);
        e.register_double(0x004b_cf60, |_, words| {
            assert_eq!(words[1..], [6, 0]);
            Ret {
                st0: 1.25,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            tes_object_weap_get_ammo_regen_rate(&mut e, weapon, false),
            2.5
        );
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(
            tes_object_weap_get_ammo_regen_rate(&mut e, weapon, true),
            3.75
        );
    }

    #[test]
    fn hud_class_matches_xml_menu_id() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert_eq!(hud_main_menu_get_class(&mut e, menu), 0x3ec);
    }

    #[test]
    fn attach_tile_accepts_only_ids_one_through_eighty_six() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        for id in [0, 87, -1] {
            hud_main_menu_attach_tile_by_id(&mut e, menu, id, Ptr::new(0x9999));
        }
        assert_eq!(e.mem.u32(menu.addr() + 0x28), 0);
        hud_main_menu_attach_tile_by_id(&mut e, menu, 1, Ptr::new(0x1111));
        hud_main_menu_attach_tile_by_id(&mut e, menu, 86, Ptr::new(0x8686));
        assert_eq!(e.mem.u32(tile(menu, 1)), 0x1111);
        assert_eq!(e.mem.u32(tile(menu, 86)), 0x8686);
    }

    #[test]
    fn validate_hud_requires_every_tile_slot_one_through_eighty_five() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        for id in 1..0x56 {
            e.mem.set_u32(tile(menu, id), 1);
        }
        assert!(fn_00770070(&mut e, menu));
        e.mem.set_u32(tile(menu, 0x55), 0);
        assert!(!fn_00770070(&mut e, menu));
    }

    #[test]
    fn replace_compass_pointer_passes_embedded_slot_to_reference_setter() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        e.register_double(0x0066_b0d0, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x64, 0x3333]);
            Ret::default()
        });
        fn_007700b0(&mut e, menu, Ptr::new(0x3333));
    }

    #[test]
    fn compass_visibility_writes_one_byte_at_menu_offset_ninety_one() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        fn_007700d0(&mut e, menu, 0x7f);
        assert_eq!(e.mem.u8(menu.addr() + 0x91), 0x7f);
    }

    #[test]
    fn controller_attachment_converts_before_assigning() {
        let mut e = Engine::new();
        e.register_double(0x0045_7fe0, |_, words| Ret {
            eax: words[0] + 4,
            ..Ret::default()
        });
        e.register_double(0x00aa_4ee0, |_, words| {
            assert_eq!(words, &[0x4000, 0x4004]);
            Ret::default()
        });
        fn_00771520(&mut e, Ptr::new(0x4000));
    }

    #[test]
    fn embedded_object_getter_reads_pc_e34_offset() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0xe40));
        e.mem.set_u32(object.addr() + 0xe34, 0x5555);
        assert_eq!(fn_007721a0(&mut e, object).addr(), 0x5555);
    }

    #[test]
    fn inline_pointer_getter_uses_cc_offset() {
        let mut e = Engine::new();
        e.register_double(0x0055_9450, |_, words| {
            assert_eq!(words, &[0x40cc]);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        assert_eq!(fn_00772820(&mut e, Ptr::new(0x4000)).addr(), 0x5000);
    }

    #[test]
    fn current_enemy_getter_handles_missing_hud() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        assert_eq!(fn_007732b0(&mut e).addr(), 0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0x21c, 0x7777);
        assert_eq!(fn_007732b0(&mut e).addr(), 0x7777);
    }

    #[test]
    fn player_state_getter_reads_byte_at_five_f8() {
        let mut e = Engine::new();
        let player: Ptr = Ptr::new(e.mem.alloc(0x600));
        e.mem.set_u8(player.addr() + 0x5f8, 0x7d);
        assert_eq!(fn_007735b0(&mut e, player), 0x7d);
    }

    #[test]
    fn quest_name_visibility_stops_when_cached_flag_is_clear() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert!(!hud_main_menu_is_quest_name_showing(&mut e, menu));
    }

    #[test]
    fn objective_visibility_stops_when_cached_flag_is_clear() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert!(!hud_main_menu_are_objectives_showing(&mut e, menu));
    }

    #[test]
    fn object_flag_predicate_uses_requested_mask() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(object.addr() + 0x20, 0b1010);
        assert!(fn_007737a0(&mut e, object, 0b1000));
        assert!(!fn_007737a0(&mut e, object, 0b0100));
    }

    #[test]
    fn brightness_role_four_writes_both_values_in_native_order() {
        let mut e = Engine::new();
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_set_brightness(&mut e, Ptr::new(0x4000), 4);
        let writes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            writes,
            vec![
                args![0x4000_u32, 0xff5_u32, 0x2c_u32],
                args![0x4000_u32, 0xff5_u32, 100_u32]
            ]
        );
    }

    #[test]
    fn actor_value_ignores_unrecognized_ids_before_virtual_calls() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.call_log = Some(vec![]);
        hud_main_menu_update_actor_value(&mut e, menu, 0xff);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn hotkey_update_exits_when_player_or_process_is_missing() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        fn_007721c0(&mut e, menu);
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.register_double(0x008d_8520, |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_007721c0(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn region_text_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let region: Ptr = Ptr::new(e.mem.alloc(0x110));
        e.mem.set_u8(region.addr() + 0x108, 1);
        e.call_log = Some(vec![]);
        hud_main_menu_set_region_text(&mut e, region);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn map_marker_update_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let state: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u8(state.addr() + 4, 1);
        e.call_log = Some(vec![]);
        fn_00772aa0(&mut e, Ptr::new(0x2222), state);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn worldspace_text_initializes_hold_trait_once_for_null_text() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.register_double(0x00a0_1860, |_, words| {
            assert_eq!(words, &[0x0107_3474]);
            Ret {
                eax: 0xfeed,
                ..Ret::default()
            }
        });
        hud_main_menu_set_worldspace_text(&mut e, Ptr::NULL);
        assert_eq!(e.mem.u32(0x011d_9af4), 0xfeed);
        assert_eq!(e.mem.u32(0x011d_9af8) & 1, 1);
        e.call_log = Some(vec![]);
        hud_main_menu_set_worldspace_text(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn clear_hud_state_exits_when_menu_class_has_no_tile() {
        let mut e = Engine::new();
        e.register_double(0x0070_6f20, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_9030, |_, words| {
            assert_eq!(words, &[0x3ec]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00773050(&mut e);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn set_enemy_actor_honors_global_block_before_touching_enemy() {
        let mut e = Engine::new();
        let blocked = e.mem.alloc(1);
        e.mem.set_u8(blocked, 1);
        e.register_double(0x0040_8d60, move |_, words| {
            assert_eq!(words, &[0x011e_0988]);
            Ret {
                eax: blocked,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        hud_main_menu_set_enemy_actor(&mut e, Ptr::NULL, Ptr::new(0xdead));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn quest_reminder_ignores_missing_hud_or_quest() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        hud_main_menu_display_quest_reminder(&mut e, Ptr::new(0x4040));
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.call_log = Some(vec![]);
        hud_main_menu_display_quest_reminder(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn radiation_sound_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.call_log = Some(vec![]);
        hud_main_menu_set_radiation_level(&mut e, 1.0, 2.0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn radiation_sound_passes_pending_words_to_sound_factory() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.register_double(0x00ad_8ce0, |_, _| Ret::default());
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x6200,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, |_, words| {
            assert_eq!(words.len(), 4);
            assert_eq!(words[0], 0x6200);
            assert_ne!(words[1], 0);
            assert_eq!(words[2..], [0x0107_3498, 0x11]);
            Ret {
                eax: 0x6300,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_8900, |_, words| {
            assert_eq!(words[1], 0x6300);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words[1], 0);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1_000_000.0,
            ..Ret::default()
        });
        e.register_double(0x0082_5c00, |_, _| Ret::default());
        hud_main_menu_set_radiation_level(&mut e, 0.0, 1.0);
    }

    #[test]
    fn target_type_clears_tracking_flags_for_null_target() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x3e), 0x4000);
        e.mem.set_f64(0x0101_2070, 1.0);
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        hud_main_menu_set_target_type(&mut e, Ptr::NULL, true);
        assert_eq!(e.mem.u8(0x011d_96ba), 0);
        assert_eq!(e.mem.u8(0x011d_96b9), 1);
    }

    #[test]
    fn positive_radiation_sets_expiry_sum_and_activity_marker() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0107_3000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.mem.set_f64(0x0107_3490, 250.0);
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 100,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_radiation_meter_conditions(&mut e, menu, 2.0, 3.0);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x5130_u32, 0x100a_u32, 350.0_f32, 1_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x5130_u32, 0x1009_u32, 5.0_f32, 1_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x5130_u32, 0x100b_u32, 1_u32]));
    }

    #[test]
    fn status_icon_high_tier_uses_shared_blink_alpha_without_advancing() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let threshold = e.mem.alloc(4);
        e.mem.set_f32(threshold, 5.0);
        e.mem.set_f32(0x011d_9b04, 0.75);
        e.register_double(0x0040_3e20, move |_, words| {
            assert_eq!(words, &[0x011d_07fc]);
            Ret {
                eax: threshold,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00773e90(&mut e, Ptr::NULL, Ptr::new(0x4444), 6.0, false);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x4444_u32, 0xff4_u32, 2_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x4444_u32, 0xfa9_u32, 0.75_f32, 1_u32]));
        assert!(!calls.iter().any(|(address, _)| *address == 0x0082_5c00));
    }

    #[test]
    fn radiation_meter_skips_finished_idle_state() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011a_0000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.mem.set_u32(0x011a_0120, 4);
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.register_double(0x0050_98e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_radiation_meter(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn quest_text_update_queues_normal_priority_with_stored_fonts() {
        let mut e = Engine::new();
        let update = e.mem.alloc(0x324);
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x324]);
            Ret {
                eax: update,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0076_baa0, move |_, words| {
            assert_eq!(words, &[update]);
            Ret::default()
        });
        fn_0076b960(&mut e, Ptr::new(1), Ptr::new(2), 0, 750, 5, 6, Ptr::new(3));
        assert_eq!(e.mem.u32(update + 0x210), 750);
        assert_eq!(e.mem.u32(update + 0x214), 0);
        assert_eq!(e.mem.u32(update + 0x218), 5);
        assert_eq!(e.mem.u32(update + 0x21c), 6);
    }

    #[test]
    fn heading_between_locations_uses_three_float_output_and_degrees() {
        let mut e = Engine::new();
        e.map(0x0102_f000, 0x1000);
        e.mem.set_f64(0x0102_f248, 57.295_776_367_187_5);
        e.register_double(0x0043_9ef0, |e, words| {
            assert_eq!(words.len(), 3);
            assert_eq!(words[0], 0x2222);
            assert_eq!(words[2], 0x1111);
            e.mem.set_f32(words[1], 0.0);
            e.mem.set_f32(words[1] + 4, 1.0);
            e.mem.set_f32(words[1] + 8, 9.0);
            Ret::default()
        });
        e.register_double(0x004b_1460, |_, words| {
            assert_eq!(words, &[0.0_f32.to_bits()]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        e.register_double(0x004b_1480, |_, words| Ret {
            st0: f64::from(f32::from_bits(words[0])),
            ..Ret::default()
        });
        let heading =
            hud_main_menu_heading_between_locations(&mut e, Ptr::new(0x1111), Ptr::new(0x2222));
        assert_eq!(heading, 57.295_776_f32 as f64);
    }

    #[test]
    fn status_mode_refreshes_five_labels_and_all_three_limb_icons() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        let player = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(player + 0xa4, table);
        e.mem.set_u32(table + 8, 0x0f00_0010);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, player);
        let threshold = e.mem.alloc(4);
        e.mem.set_f32(threshold, 0.0);
        e.register_double(0x0040_3df0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0064_6800, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x0f00_0010, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, _| Ret {
            eax: threshold,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_status_mode_ui(&mut e, menu, true);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0040_3df0)
                .count(),
            5
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0064_6800)
                .count(),
            6
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0f00_0010)
                .count(),
            4
        );
    }

    #[test]
    fn enemy_health_reloads_hud_tile_after_name_callback() {
        let (mut e, meter) = meter_engine();
        e.map(0x011d_e000, 0x1000);
        let original_menu = e.new_object::<HUDMainMenu>();
        let replacement_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, original_menu.addr());
        e.mem.set_u32(original_menu.addr() + 0x80, meter.addr());
        e.mem.set_u32(original_menu.addr() + 0x84, 0x3000);
        e.mem.set_u32(replacement_menu.addr() + 0x84, 0x4000);
        let enemy: Ptr = Ptr::new(e.mem.alloc(0x200));
        let values_table = e.mem.alloc(0x20);
        e.mem.set_u32(enemy.addr() + 0xa4, values_table);
        e.mem.set_u32(values_table, 0x0f00_0020);
        e.mem.set_u32(values_table + 8, 0x0f00_0021);
        e.register_double(0x0f00_0020, |_, words| {
            assert_eq!(words[1], 0x10);
            Ret {
                eax: 100,
                ..Ret::default()
            }
        });
        e.register_double(0x0f00_0021, |_, words| {
            assert_eq!(words[1], 0x10);
            Ret {
                eax: 25,
                ..Ret::default()
            }
        });
        e.register_double(0x0047_7ba0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x005e_58f0, |e, words| {
            assert_eq!(words[0..2], [0x2f, 0]);
            e.mem.set_f32(words[2], 0.0);
            Ret::default()
        });
        e.register_double(0x0055_d520, move |e, words| {
            assert_eq!(words, &[enemy.addr()]);
            e.mem.set_u32(0x011d_96c0, replacement_menu.addr());
            Ret {
                eax: 0x9090,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, &[0x4000, 0xfc4, 0x9090, 1]);
            Ret::default()
        });
        hud_main_menu_update_enemy_health(&mut e, original_menu, enemy);
        assert_eq!(e.mem.f32(0x011d_97b8), 0.25);
    }

    fn destructor_engine() -> (Engine, Ptr<HUDMainMenu>) {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let audio = e.mem.alloc(4);
        let table = e.mem.alloc(0xec);
        e.mem.set_u32(audio, table);
        e.mem.set_u32(table + 0xe8, 0x0f00_0030);
        let empty_list_item = e.mem.alloc(4);
        e.register_double(0x0072_6070, |_, _| Ret::default());
        e.register_double(0x0045_0b80, move |_, _| Ret {
            eax: audio,
            ..Ret::default()
        });
        e.register_double(0x0055_9450, |_, _| Ret::default());
        e.register_double(0x0f00_0030, |_, _| Ret::default());
        e.register_double(0x0066_b0d0, |_, _| Ret::default());
        e.register_double(0x0068_15c0, move |_, _| Ret {
            eax: empty_list_item,
            ..Ret::default()
        });
        for address in [
            0x0046_ffb0,
            0x0070_1520,
            0x0048_3710,
            0x0077_f9a0,
            0x00a1_c520,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        (e, menu)
    }

    #[test]
    fn hud_destructor_clears_global_and_releases_owned_containers() {
        let (mut e, menu) = destructor_engine();
        e.call_log = Some(vec![]);
        fn_0076be30(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr()), 0x0107_2df4);
        assert_eq!(e.mem.u32(0x011d_96c0), 0);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0046_ffb0)
                .count(),
            4
        );
        assert_eq!(calls.last().unwrap().0, 0x00a1_c520);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_flag_is_set() {
        let (mut e, menu) = destructor_engine();
        e.register_double(0x0040_1030, move |_, words| {
            assert_eq!(words, &[menu.addr()]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            hud_main_menu_scalar_deleting_destructor(&mut e, menu, 1),
            menu
        );
        assert_eq!(e.call_log.as_ref().unwrap().last().unwrap().0, 0x0040_1030);
        let (mut retained, retained_menu) = destructor_engine();
        retained.call_log = Some(vec![]);
        assert_eq!(
            hud_main_menu_scalar_deleting_destructor(&mut retained, retained_menu, 0),
            retained_menu
        );
        assert!(!retained
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x0040_1030));
    }

    #[test]
    fn sneak_update_uses_detail_label_without_starting_alert_fade() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.mem.set_u32(tile(menu, 0x18), 0x1818);
        e.register_double(0x0095_3c50, |e, words| {
            assert_eq!(words[0], 0x1234);
            e.mem.set_u8(words[1], 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, &[0x011d_4804]);
            Ret {
                eax: 0x9090,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, &[0x1818, 0xfc4, 0x9090, 1]);
            Ret::default()
        });
        e.register_double(0x00a0_8070, |_, _| Ret::default());
        e.register_double(0x00a0_7fc0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_sneak(&mut e, menu);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x1818_u32, 0xff4_u32, 2_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x1818_u32, 0x1004_u32, 0_u32]));
        assert!(!calls.iter().any(|(address, _)| *address == 0x00a0_7c60));
    }

    #[test]
    fn idle_timeout_uses_x87_invalid_conversion_and_quiets_fade_inputs() {
        assert_eq!(fistp_i64_low(f64::INFINITY), 0);
        assert_eq!(fistp_i64_low(f64::NEG_INFINITY), 0);
        assert_eq!(fistp_i64_low(f64::NAN), 0);
        assert_eq!(fistp_i64_low(1.9), 1);
        assert_eq!(fistp_i64_low(-1.9), u32::MAX);
        assert_eq!(
            quiet_f32(f32::from_bits(0x7f80_0001)).to_bits(),
            0x7fc0_0001
        );
    }

    fn meter_engine() -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_2070, 1.0);
        e.mem.set_u32(0x011d_96c0, 1);
        e.mem.set_u32(0x011d_9c18, 7);
        e.mem.set_u32(0x011d_9c14, 0x101);
        e.mem.set_u32(0x011d_9c10, 0x102);
        e.mem.set_u32(0x011d_9c0c, 0x103);
        e.register_double(0x00ec_62c0, |_, words| {
            let bits = u64::from(words[0]) | (u64::from(words[1]) << 32);
            Ret {
                eax: f64::from_bits(bits).round_ties_even() as i32 as u32,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let target = Ptr::new(e.mem.alloc(0x20));
        (e, target)
    }

    #[test]
    fn meter_nan_uses_zero_and_preserves_st0_precision() {
        let (mut e, target) = meter_engine();
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match words[1] {
                0x101 => 15.999_999_8,
                0x102 => 4.0,
                _ => 30.0,
            },
            ..Ret::default()
        });
        hud_main_menu_update_meter(&mut e, target, f32::NAN, 0, 0, 0);
        let changes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            changes,
            vec![
                args![target, 0xfb1_u32, 0_u32],
                args![target, 0xfa1_u32, 30_u32]
            ]
        );
        let conversions: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00ec_62c0)
            .map(|(_, words)| f64::from_bits(u64::from(words[0]) | (u64::from(words[1]) << 32)))
            .collect();
        assert_eq!(conversions[0], 15.999_999_8);
    }

    #[test]
    fn meter_reloads_zero_after_image_callback() {
        let (mut e, target) = meter_engine();
        e.register_double(0x00a0_11b0, |e, words| {
            if words[1] == 0x102 {
                e.mem.set_f64(0x0101_2060, 1.0);
            }
            Ret {
                st0: match words[1] {
                    0x101 => 16.0,
                    0x102 => 4.0,
                    _ => 30.0,
                },
                ..Ret::default()
            }
        });
        hud_main_menu_update_meter(&mut e, target, 0.99, 0, 0, 0);
        let changes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(changes[0], args![target, 0xfb1_u32, 0_u32]);
    }

    fn meter_pair_engine() -> (Engine, Ptr, Ptr, u32) {
        let (mut e, first) = meter_engine();
        let second: Ptr = Ptr::new(e.mem.alloc(0x20));
        let child = e.mem.alloc(0x20);
        let child_storage = e.mem.alloc(4);
        e.mem.set_u32(child_storage, child);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f32(0x0102_3cd8, 255.0);
        e.mem.set_u32(0x011d_9c2c, 15);
        e.mem.set_u32(0x011d_9c28, 0x101);
        e.mem.set_u32(0x011d_9c24, 0x102);
        e.mem.set_u32(0x011d_9c20, 0x103);
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match words[1] {
                0x101 => 20.0,
                0x102 => 4.0,
                0x103 => 100.0,
                _ => panic!("unexpected trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x004a_8f20, |_, words| Ret {
            eax: (words[0] as i32).min(words[1] as i32) as u32,
            ..Ret::default()
        });
        e.register_double(0x0073_ac90, move |_, words| {
            assert_eq!(words, &[second.addr() + 4]);
            Ret {
                eax: child_storage,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7ed0, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |_, _| Ret::default());
        (e, first, second, child)
    }

    #[test]
    fn meter_pair_positions_both_tiles_and_center_child() {
        for (anchor, expected) in [
            (0, vec![(0xfb1, 8), (0xfa1, 100), (0xfb1, 12), (0xfa1, 108)]),
            (1, vec![(0xfb1, 8), (0xfa1, 92), (0xfb1, 4), (0xfa1, 88)]),
            (2, vec![(0xfb1, 8), (0xfa1, 96), (0xfb1, 4), (0xfa1, 92)]),
        ] {
            let (mut e, first, second, child) = meter_pair_engine();
            hud_main_menu_update_meter_ov2(&mut e, first, second, 0.4, 0.6, anchor, 0);
            let writes: Vec<_> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x0070_0320)
                .map(|(_, words)| (words[0], words[1], words[2]))
                .collect();
            let mut expected_writes = Vec::new();
            if anchor == 2 {
                expected_writes.extend([(child, 0xfb1, 4), (child, 0xfa1, 104)]);
            }
            expected_writes.extend(expected.into_iter().enumerate().map(
                |(index, (trait_id, value))| {
                    (
                        if index < 2 {
                            first.addr()
                        } else {
                            second.addr()
                        },
                        trait_id,
                        value,
                    )
                },
            ));
            assert_eq!(writes, expected_writes, "anchor {anchor}");
            let fades: Vec<_> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x00a0_7c60)
                .map(|(_, words)| words.clone())
                .collect();
            let targets = if anchor == 2 {
                vec![child, second.addr()]
            } else {
                vec![second.addr()]
            };
            assert_eq!(
                fades,
                targets
                    .into_iter()
                    .map(|target| args![target, 0xfa9_u32, 0.0_f32, 255.0_f32, 1.0_f32, 1_u32])
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn meter_pair_initializes_traits_before_null_guard_and_keeps_minimum() {
        let (mut e, first, second, _) = meter_pair_engine();
        e.mem.set_u32(0x011d_9c2c, 0);
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        hud_main_menu_update_meter_ov2(&mut e, Ptr::new(0), second, 0.0, 0.1, 0, 2);
        let initializers: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_0940)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            initializers,
            vec![
                args![0x0107_3424_u32, u32::MAX],
                args![0x0107_3418_u32, u32::MAX],
                args![0x0107_3430_u32, u32::MAX],
                args![0x0107_34b8_u32, u32::MAX],
            ]
        );
        assert_eq!(e.mem.u32(0x011d_9c2c), 15);

        e.mem.set_u32(0x011d_9c28, 0x101);
        e.mem.set_u32(0x011d_9c24, 0x102);
        e.mem.set_u32(0x011d_9c20, 0x103);
        e.call_log = Some(vec![]);
        hud_main_menu_update_meter_ov2(&mut e, first, second, 0.0, 0.1, 0, 2);
        let writes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(writes[2], args![second, 0xfb1_u32, 8_u32]);
    }

    #[test]
    fn subtitle_constructor_initializes_embedded_point() {
        let mut e = Engine::new();
        let subtitle: Ptr = Ptr::new(e.mem.alloc(0x11c));
        e.register_double(0x0068_15c0, |e, words| {
            e.mem.set_u32(words[0], 0x1234);
            Ret::default()
        });
        assert_eq!(fn_00775210(&mut e, subtitle), subtitle);
        assert_eq!(e.mem.u32(subtitle.addr() + 0x10c), 0x1234);
    }

    #[test]
    fn subtitle_gate_and_zero_sound_duration_fallback() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x011d_2000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_7b70, 1000.0);
        e.mem.set_u32(0x011d_ea3c, 0x5000);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        let rate = e.mem.alloc(4);
        e.mem.set_f32(rate, 0.5);
        let subtitle = e.mem.alloc(0x11c);
        let menu = e.new_object::<HUDMainMenu>();
        let text: Ptr = Ptr::new(e.mem.alloc(8));
        e.register_double(0x0048_3710, |e, words| {
            assert_eq!(e.mem.u32(words[0]), 0xfeed);
            Ret::default()
        });
        e.register_double(0x0040_8d60, move |_, words| {
            assert_eq!(words, &[0x011d_8928]);
            Ret {
                eax: setting,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x11c]);
            Ret {
                eax: subtitle,
                ..Ret::default()
            }
        });
        e.register_double(0x0068_15c0, |_, _| Ret::default());
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x00ad_8b30, |_, _| Ret::default());
        e.register_double(0x00ec_6130, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, words| {
            assert_eq!(words, &[0x011d_2178]);
            Ret {
                eax: rate,
                ..Ret::default()
            }
        });
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 42,
            ..Ret::default()
        });
        e.register_double(0x007c_b2e0, move |e, words| {
            assert_eq!(e.mem.u32(words[1]), subtitle);
            Ret::default()
        });
        assert!(!hud_main_menu_show_subtitle(
            &mut e,
            menu,
            Ptr::new(0),
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert!(hud_main_menu_show_subtitle(
            &mut e,
            menu,
            text,
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert_eq!(e.mem.f32(subtitle + 0x108), 2.5);
        assert_eq!(e.mem.u32(subtitle + 0x104), 42);
        assert_eq!(e.mem.u32(subtitle + 0x10c), 1);
        assert_eq!(e.mem.u32(subtitle + 0x110), 2);
        assert_eq!(e.mem.u32(subtitle + 0x114), 3);
        assert_eq!(e.mem.i32(subtitle + 0x118), 1);

        e.mem.set_u8(setting, 0);
        e.call_log = Some(vec![]);
        assert!(!hud_main_menu_show_subtitle(
            &mut e,
            menu,
            text,
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x0040_1000));
    }

    #[test]
    fn subtitle_selection_respects_priority_and_forced_sentinel() {
        let mut e = Engine::new();
        e.map(0x0101_3000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x0101_3d84, 0x7f80_0001);
        e.mem.set_f32(0x0101_6970, f32::MAX);
        let menu = e.new_object::<HUDMainMenu>();
        let player = e.mem.alloc(4);
        let position = e.mem.alloc(12);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut slots = [0_u32; 0x7e];
        slots[0x1f4 / 4] = 0x0300_1000;
        e.put_vtable(0x0300_0000, &slots);
        e.mem.set_u32(player, 0x0300_0000);
        e.register_double(0x0300_1000, move |_, _| Ret {
            eax: position,
            ..Ret::default()
        });
        let records: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x11c)).collect();
        for (record, distance, priority) in [
            (records[0], 10.0, 1),
            (records[1], 5.0, 0),
            (records[2], 7.0, i32::MAX),
        ] {
            e.mem.set_f32(record + 0x10c, distance);
            e.mem.set_i32(record + 0x118, priority);
        }
        let elements = e.mem.alloc(12);
        for (index, record) in records.iter().enumerate() {
            e.mem.set_u32(elements + index as u32 * 4, *record);
        }
        e.register_double(0x0044_ddc0, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(0x0087_7a30, move |_, words| Ret {
            eax: elements + words[1] * 4,
            ..Ret::default()
        });
        e.register_double(0x0043_9ef0, |e, words| {
            e.mem.set_f32(words[1], e.mem.f32(words[0]));
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        e.register_double(0x004a_7290, |e, words| Ret {
            st0: f64::from(e.mem.f32(words[0])),
            ..Ret::default()
        });
        e.register_double(0x0068_5d20, |_, words| {
            assert_eq!(words, &args![f32::from_bits(0x7fc0_0001), 2_u32]);
            Ret {
                st0: 20.0,
                ..Ret::default()
            }
        });
        assert_eq!(fn_00775230(&mut e, menu).addr(), records[2]);
    }

    #[test]
    fn hud_message_uses_pc_icon_table_and_shows_first_record() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0x90, 0x9000);
        e.mem.set_u32(menu.addr() + 0x8c, 0x9004);
        let text: Ptr = Ptr::new(e.mem.alloc(12));
        let record = e.mem.alloc(0x310);
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x310]);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0082_56d0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0090_5820, |e, words| {
            e.mem.set_u32(words[0], e.mem.u32(words[1]));
            Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!fn_00775380(
            &mut e,
            menu,
            Ptr::new(0),
            0,
            Ptr::new(0),
            Ptr::new(0),
            1.0,
            0,
        ));
        assert!(fn_00775380(
            &mut e,
            menu,
            text,
            0,
            Ptr::new(0),
            Ptr::new(0),
            1.5,
            0,
        ));
        assert_eq!(e.mem.f32(record + 0x30c), 1.5);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0040_6d30 && words == &args![record + 0x104, 0x104_u32, 0x0102_08e0_u32]
        }));
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350 && words == &args![0x9000_u32, 0xfc4_u32, record, 1_u32]
        }));

        let sound_name: Ptr = Ptr::new(e.mem.alloc(8));
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x6200,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, move |_, words| {
            assert_eq!(words.len(), 4);
            assert_eq!(words[0], 0x6200);
            assert_eq!(words[2], sound_name.addr());
            assert_eq!(words[3], 0x121);
            Ret {
                eax: 0x6300,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words, &[0x6300, 0]);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        assert!(fn_00775380(
            &mut e,
            menu,
            text,
            0,
            Ptr::new(0),
            sound_name,
            f32::from_bits(0x7f80_0001),
            0,
        ));
        assert_eq!(e.mem.u32(record + 0x30c), 0x7fc0_0001);
    }

    #[test]
    fn urgent_hud_message_ends_current_timer_before_insertion() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        let current = e.mem.alloc(0x310);
        let incoming = e.mem.alloc(0x310);
        let text: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(menu.addr() + 0x18c, current);
        e.mem.set_u32(menu.addr() + 0x190, 0);
        e.mem.set_f32(current + 0x30c, 9.0);
        e.register_double(0x0040_1000, move |_, _| Ret {
            eax: incoming,
            ..Ret::default()
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0082_56d0, |_, _| Ret::default());
        e.register_double(0x0072_6070, |e, words| Ret {
            eax: e.mem.u32(words[0] + 4),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0040_8b20, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x005a_e3d0, move |e, words| {
            assert_eq!(e.mem.f32(current + 0x30c), 0.0);
            assert_eq!(e.mem.u32(words[1]), incoming);
            Ret::default()
        });
        fn_00775380(&mut e, menu, text, 3, Ptr::new(0), Ptr::new(0), 4.0, 1);
        assert_eq!(e.mem.f32(current + 0x30c), 0.0);
        assert_eq!(e.mem.f32(incoming + 0x30c), 4.0);
    }

    #[test]
    fn hide_subtitle_clears_text_only_outside_dialogue() {
        for dialogue_visible in [false, true] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x1000);
            let menu = e.new_object::<HUDMainMenu>();
            e.mem.set_u32(0x011d_96c0, menu.addr());
            e.mem.set_u32(menu.addr() + 0x98, 0x2345);
            e.mem.set_u32(menu.addr() + 0x1a8, 0x6789);
            e.register_double(0x0070_2680, move |_, _| Ret {
                eax: u32::from(dialogue_visible),
                ..Ret::default()
            });
            e.register_double(0x00a0_1350, |_, _| Ret::default());
            e.register_double(0x0070_0320, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            hud_main_menu_hide_subtitle(&mut e, menu);
            assert_eq!(e.mem.u32(menu.addr() + 0x1a8), 0);
            let calls = e.call_log.as_ref().unwrap();
            assert_eq!(
                calls.iter().any(|(address, _)| *address == 0x00a0_1350),
                !dialogue_visible
            );
            assert!(calls.iter().any(|(address, words)| {
                *address == 0x0070_0320 && words == &args![0x2345_u32, 0xfa3_u32, 0_u32]
            }));
        }
    }

    #[test]
    fn hide_message_fades_tiles_after_last_record() {
        let mut e = Engine::new();
        e.map(0x0102_1000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.mem.set_f32(0x0102_1f44, 0.35);
        e.mem.set_f32(0x011d_979c, 200.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let record = e.mem.alloc(0x310);
        e.mem.set_u32(menu.addr() + 0x18c, record);
        e.mem.set_u32(menu.addr() + 0x194, 99);
        for (offset, tile) in [(0x90, 0x10), (0x8c, 0x11), (0x94, 0x12)] {
            e.mem.set_u32(menu.addr() + offset, tile);
        }
        e.register_double(0x0082_56d0, |e, words| Ret {
            eax: u32::from(e.mem.u32(words[0]) == 0),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_11f0, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register_double(0x0040_8b20, |_, _| Ret::default());
        e.register_double(0x0090_5330, |e, words| {
            assert_eq!(e.mem.u32(words[1]), e.mem.u32(words[0]));
            e.mem.set_u32(words[0], 0);
            Ret::default()
        });
        e.register_double(0x0040_1030, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_hide_message(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr() + 0x194), 0);
        let fades: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_7c60)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(fades.len(), 3);
        assert_eq!(
            fades[0],
            args![0x10_u32, 0xfa9_u32, 200.0_f32, 0.0_f32, 0.35_f32, 0_u32]
        );
    }

    #[test]
    fn clear_messages_frees_records_then_clears_tiles_and_subtitle() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let record = e.mem.alloc(0x310);
        e.mem.set_u32(menu.addr() + 0x18c, record);
        e.mem.set_u32(menu.addr() + 0x190, 0);
        e.mem.set_u32(menu.addr() + 0x1a8, 0x8000);
        for (offset, tile) in [(0x94, 0x11), (0x90, 0x12), (0x8c, 0x13), (0x98, 0x14)] {
            e.mem.set_u32(menu.addr() + offset, tile);
        }
        e.register_double(0x0082_56d0, |e, words| Ret {
            eax: u32::from(e.mem.u32(words[0]) == 0),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0072_6070, |e, words| Ret {
            eax: e.mem.u32(words[0] + 4),
            ..Ret::default()
        });
        e.register_double(0x0040_1030, |_, _| Ret::default());
        e.register_double(0x0047_0470, |e, words| {
            e.mem.set_u32(words[0], 0);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0070_2680, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00775920(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr() + 0x1a8), 0);
        assert_eq!(e.mem.u32(menu.addr() + 0x18c), 0);
        let calls = e.call_log.as_ref().unwrap();
        let free_at = calls
            .iter()
            .position(|(address, words)| *address == 0x0040_1030 && words == &args![record])
            .unwrap();
        let clear_at = calls
            .iter()
            .position(|(address, _)| *address == 0x0047_0470)
            .unwrap();
        assert!(free_at < clear_at);
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x00a0_12d0)
                .count(),
            3
        );
    }

    #[test]
    fn menu_mode_reloads_hud_after_subtitle_callback() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let old_menu = e.new_object::<HUDMainMenu>();
        let current_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, old_menu.addr());
        e.mem.set_u32(current_menu.addr() + 0x1c0, u32::MAX);
        e.mem.set_u32(current_menu.addr() + 0x144, 0x5050);
        e.mem.set_u32(current_menu.addr() + 0x114, 0x5114);
        e.register_double(0x0059_bb30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x005a_03f0, |_, _| Ret::default());
        e.register_double(0x0077_5670, move |e, words| {
            assert_eq!(words, &[old_menu.addr()]);
            e.mem.set_u32(0x011d_96c0, current_menu.addr());
            Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00ad_8ce0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_set_menu_mode(&mut e, 3);
        assert_eq!(e.mem.u32(current_menu.addr() + 0x1bc), 0x100);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls
            .iter()
            .any(|(addr, words)| *addr == 0x0070_0320
                && words == &args![0x5114_u32, 0xfa3_u32, 0_u32]));
        assert_eq!(
            calls
                .iter()
                .filter(|(addr, _)| *addr == 0x0070_0320)
                .count(),
            23
        );
    }

    #[test]
    fn do_idle_stops_before_updating_when_player_has_no_process() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        hud_main_menu_do_idle(&mut e, menu);
        assert!(e.call_log.as_ref().map_or(true, Vec::is_empty));

        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.register_double(0x008d_6f30, |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_do_idle(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_idle_message_fades_reload_alpha_after_each_callback() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        for (offset, current_tile) in [(0x94, 0x4094), (0x8c, 0x408c), (0x90, 0x4090)] {
            e.mem.set_u32(menu.addr() + offset, current_tile);
        }
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0102_1f44, 2.0);
        e.mem.set_f32(0x011d_979c, 1.0);
        e.register_double(0x005a_e380, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_1230, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        e.register_double(0x00a0_7fc0, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |e, _| {
            let next = e.mem.f32(0x011d_979c) + 1.0;
            e.mem.set_f32(0x011d_979c, next);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        do_idle_messages(&mut e, menu);
        let fade_calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_7c60)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(fade_calls.len(), 3);
        for (index, (tile, words)) in [
            (0x4094, &fade_calls[0]),
            (0x408c, &fade_calls[1]),
            (0x4090, &fade_calls[2]),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(
                words,
                &args![tile, 0xfa9_u32, 0.0_f32, index as f32 + 1.0, 2.0_f32, 0_u32]
            );
        }
    }

    #[test]
    fn create_unlocks_when_xml_does_not_produce_menu() {
        let mut e = Engine::new();
        e.register_double(0x004b_7210, |_, _| Ret {
            eax: 0x1010,
            ..Ret::default()
        });
        e.register_double(0x0070_6f20, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_9030, |_, _| Ret::default());
        e.register_double(0x00a0_1530, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.register_double(0x0071_3d70, |_, _| Ret {
            eax: 0x2020,
            ..Ret::default()
        });
        e.register_double(0x0045_38a0, |_, words| {
            assert_eq!(words, &[0x2020, 0]);
            Ret::default()
        });
        e.register_double(0x0058_6150, |_, words| {
            assert_eq!(words, &[0x1010, 0x0107_343c]);
            Ret {
                eax: 0x3030,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1b00, |_, words| {
            assert_eq!(words, &[0x3030]);
            Ret {
                eax: 0x4040,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_3c90, |_, words| {
            assert_eq!(words, &[0x4040]);
            Ret::default()
        });
        e.register_double(0x0045_38c0, |_, words| {
            assert_eq!(words, &[0x2020]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(hud_main_menu_create(&mut e), Ptr::NULL);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0071_3d70)
                .count(),
            2
        );
        assert_eq!(calls.last().unwrap().0, 0x0045_38c0);
    }

    #[test]
    fn damage_threshold_uses_exact_clamp_arity_and_clears_second_perk_output_late() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.mem.set_u32(0x011c_a278, 0x7000);
        let player: Ptr = Ptr::new(e.mem.alloc(0x200));
        let target: Ptr = Ptr::new(e.mem.alloc(0x200));
        let process = e.mem.alloc(0x20);
        let target_table = e.mem.alloc(0x20);
        let process_table = e.mem.alloc(0x200);
        e.mem.set_u32(target.addr() + 0xa4, target_table);
        e.mem.set_u32(target_table + 0x0c, 0x0f00_0001);
        e.mem.set_u32(process, process_table);
        e.mem.set_u32(process_table + 0x148, 0x0f00_0002);
        e.register_double(0x0f00_0001, move |_, words| {
            assert_eq!(words, &[target.addr() + 0xa4, 0x4c]);
            Ret {
                st0: 10.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0f00_0002, move |_, words| {
            assert_eq!(words, &[process]);
            Ret::default()
        });
        e.register_double(0x0040_4010, |_, words| {
            assert_eq!(words.len(), 2);
            let value = if words[0] == 0 {
                f32::from_bits(words[1])
            } else {
                assert_eq!(words[1], 0.0_f32.to_bits());
                f32::from_bits(words[0])
            };
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        e.register_double(0x008d_8520, move |_, words| {
            assert_eq!(words, &[player.addr()]);
            Ret {
                eax: process,
                ..Ret::default()
            }
        });
        e.register_double(0x005e_58f0, move |e, words| {
            assert_eq!(words.len(), 5);
            if words[0] == 0x3a {
                assert_eq!(words[1], player.addr());
                assert_eq!(words[2], 0x7000);
                assert_eq!(words[3], target.addr());
                assert_eq!(e.mem.f32(words[4]), 0.0);
                e.mem.set_f32(words[4], 2.0);
                e.mem.set_f32(words[4] + 4, 9.0);
            } else {
                assert_eq!(words[0], 0x38);
                assert_eq!(words[1], target.addr());
                assert_eq!(words[2], player.addr());
                assert_eq!(words[3], 0x7000);
                assert_eq!(e.mem.f32(words[4]), 0.0);
                e.mem.set_f32(words[4], 3.0);
            }
            Ret::default()
        });
        assert_eq!(fn_007700f0(&mut e, player, target), 11.0);
    }

    #[test]
    fn create_ddt_icons_use_distinct_offsets_and_hardcore_child_order() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0080_0000);
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        for (address, value) in [
            (0x0102_0998, 5.0),
            (0x0102_40b8, 16.0),
            (0x0102_f070, 32.0),
            (0x0107_2f00, 165.0),
            (0x0106_c948, 150.0),
            (0x0102_0758, 10.0),
        ] {
            e.mem.set_f64(address, value);
        }
        e.register_double(0x00a0_9030, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_8b20, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        create_ddt_and_hardcore(&mut e, 75.0);
        let calls = e.call_log.as_ref().unwrap();
        let x = |tile, expected: f32| {
            let value = calls
                .iter()
                .find(|(address, words)| {
                    *address == 0x00a0_12d0 && words[0] == tile && words[1] == 0xfa1
                })
                .unwrap()
                .1[2];
            assert_eq!(f32::from_bits(value), expected);
        };
        x(0x0107_2f38, 7.0);
        x(0x0107_2f28, -30.0);
        x(0x0107_2f08, -163.0);
        x(0x0107_2eec, 151.0);
        let children: Vec<_> = calls
            .iter()
            .filter(|(address, words)| *address == 0x00a0_8b20 && words[0] == 0x0107_2eec)
            .map(|(_, words)| words[1])
            .collect();
        assert_eq!(
            children,
            vec![
                0x0106_75c0,
                0x0106_75b8,
                0x0106_37d4,
                0x0105_afd0,
                0x0107_2ee4
            ]
        );
    }

    #[test]
    fn do_idle_runs_immediate_updates_then_respects_200_ms_cadence() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, 0x5050);
        e.mem.set_u32(menu.addr() + 0x40, 0x4040);
        e.mem.set_u32(menu.addr() + 0x34, 1);
        e.mem.set_u32(menu.addr() + 0x38, 2);
        e.mem.set_u32(menu.addr() + 0x3c, 3);
        e.register_double(0x008d_6f30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0045_7fe0, |_, _| Ret::default());
        e.register_double(0x00aa_4ee0, |_, _| Ret::default());
        e.register_double(0x0077_9070, |_, words| {
            assert_eq!(words, &[0x4040, 1, 2, 3]);
            Ret::default()
        });
        e.register_double(0x0077_da60, |_, _| Ret::default());
        e.register_double(0x008d_8520, |_, _| Ret::default());
        e.register_double(0x0077_e750, |_, _| Ret::default());
        e.register_double(0x0077_f700, |_, _| Ret::default());
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_do_idle(&mut e, menu);
        assert_eq!(e.mem.u32(0x011d_97c8) & 1, 1);
        assert_eq!(e.mem.u32(0x011d_97c4), 100);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0082_5c00)
                .count(),
            2
        );
        assert!(!calls.iter().any(|(address, _)| *address == 0x0071_5da0));
    }
}
