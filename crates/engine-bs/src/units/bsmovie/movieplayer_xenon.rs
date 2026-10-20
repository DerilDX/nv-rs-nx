//! `BSMovie/movieplayer_xenon.obj` (Xbox PDB source unit), subsystem `BSMovie`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The movie player drives Bink through its imports (`BinkWait`,
//! `BinkNextFrame`, called by their import slots) and draws each frame with
//! Direct3D 9 device calls (COM vtable slots on the device). Fields of the
//! `Movie` object (read at their offsets, unnamed in the Xbox PDB here):
//! +0 the Bink handle (its frame count at +8), +4 the device, +8 the frame
//! textures, +0xC the frames shown, +0x10 the frames skipped, +0x14 the
//! scale, +0x18 and +0x1C the position, +0x20 the play state, +0x24 a busy
//! byte.

#[allow(unused_imports)]
use crate::prelude::*;

/// The movie settings object (`0126fac4`, a pointer) whose vtable the
/// player asks (+0x40 with one byte: go on; +0x30: no scene begin/end) and
/// notifies (+4/+8 and +0xC/+0x10 around a playback).
const MOVIE_SETTINGS: u32 = 0x0126_fac4;
/// The renderer pointer (`011f4748`): vtable +0xC8 (the default render
/// target group, then with 0 its first target), +0xD4 (the back buffer
/// description: width +8, height +0xC).
const RENDERER: u32 = 0x011f_4748;
/// The `NiRTTI` record the render target's class chain is checked against
/// (`0126f59c`).
const TARGET_RTTI: u32 = 0x0126_f59c;
/// The byte that turns on the letterbox viewport (`011f9426`), and the
/// viewport's left, right, bottom and top fractions (`011ad840`, `011ad844`,
/// `011ad848`, `011ad84c`).
const LETTERBOX: u32 = 0x011f_9426;
const VIEW_LEFT: u32 = 0x011a_d840;
const VIEW_RIGHT: u32 = 0x011a_d844;
const VIEW_BOTTOM: u32 = 0x011a_d848;
const VIEW_TOP: u32 = 0x011a_d84c;
/// Imports, by slot: `BinkWait` (`00fdf3ac`), `BinkNextFrame` (`00fdf3a8`),
/// `Sleep` (`00fdf104`), the tick count import (`00fdf060`),
/// `GetExitCodeThread` (`00fdf168`) and `CloseHandle` (`00fdf07c`).
const BINK_WAIT: u32 = 0x00fd_f3ac;
const BINK_NEXT_FRAME: u32 = 0x00fd_f3a8;
const SLEEP: u32 = 0x00fd_f104;
const TICK_COUNT: u32 = 0x00fd_f060;
const GET_EXIT_CODE_THREAD: u32 = 0x00fd_f168;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// `STILL_ACTIVE`.
const STILL_ACTIVE: u32 = 0x103;
/// `0.0` and `1000.0` (`double`).
const ZERO: u32 = 0x0101_2060;
const THOUSAND: u32 = 0x0101_7b70;
/// `Error` (`0040fbe0`, `cdecl`) and `"Bink playback: Skipped frame #%i.
/// Total skipped: %i"` (`010f1be4`).
const ERROR: u32 = 0x0040_fbe0;
const SKIPPED_FRAME_MESSAGE: u32 = 0x010f_1be4;
/// The movie's open (`00ec0cd0`, `thiscall`, four words), close
/// (`00ec1020`, `thiscall`), frame copy (`00ec14d0`, `cdecl`: Bink handle,
/// textures, 0), the draw state set-up (`00ec0460`, `cdecl`: device) and the
/// quad draw (`00ec0280`, `cdecl`: device, x, y, width, height, two sizes).
const MOVIE_OPEN: u32 = 0x00ec_0cd0;
const MOVIE_CLOSE: u32 = 0x00ec_1020;
const COPY_FRAME: u32 = 0x00ec_14d0;
const DRAW_SETUP: u32 = 0x00ec_0460;
const DRAW_QUAD: u32 = 0x00ec_0280;
/// The player's stop (`00ec25e0`, `MoviePlayer::Stop` by the decompiler's
/// name; `thiscall`) and the member clean-up before the map (`00ec2e50`).
const PLAYER_STOP: u32 = 0x00ec_25e0;
const PLAYER_CLEAN_UP: u32 = 0x00ec_2e50;
/// The vtables the player's destructor stores: its own (`010f1c44`) and the
/// name map's (`010f1c20`); the map destructor's (`010f1ce4`, `010f1cc4`,
/// `010f1ca4`).
const PLAYER_VTABLE: u32 = 0x010f_1c44;
const NAME_MAP_VTABLE: u32 = 0x010f_1c20;
const STRING_MAP_VTABLE: u32 = 0x010f_1ce4;
const MAP_BASE_VTABLE: u32 = 0x010f_1cc4;
const MAP_ROOT_VTABLE: u32 = 0x010f_1ca4;
/// `00438af0` (`thiscall`): the map base destructor body, run twice.
const MAP_BASE_DESTRUCT: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`): frees a block.
const NI_FREE: u32 = 0x00aa_10f0;
/// `00ec67aa` (`cdecl`, one character): the lower-case form used by the
/// hash.
const TO_LOWER: u32 = 0x00ec_67aa;

/// Direct3D 9 device slots used here.
const SET_RENDER_STATE: u32 = 0xe4;
const SET_TEXTURE: u32 = 0x104;
const CLEAR: u32 = 0xac;
const SET_VIEWPORT: u32 = 0xbc;
const BEGIN_SCENE: u32 = 0xa4;
const END_SCENE: u32 = 0xa8;
const PRESENT: u32 = 0x44;
const SET_RENDER_TARGET: u32 = 0x94;

/// `x * step` for an unsigned word: `FILD qword` (the word zero-extended),
/// `FMUL dword`, `FSTP dword`.
fn word_times(count: u32, factor: u32) -> u32 {
    x87_store32(x87_mul(f64::from(count), x87_m32(factor)))
}

/// `value + step` through the x87 stack (`FLD`, `FADD dword`, `FSTP`).
fn advance(value: u32, step: u32) -> u32 {
    x87_store32(x87_add(x87_load32(value), x87_m32(step)))
}

// Translated from 00ebff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the frame texture set, x, y, tile
/// width and height as floats): draws a movie frame tile by tile. The set
/// holds the device (+0), the tile grid (+0x24 columns, +0x28 rows), the
/// last column's and last row's partial sizes (+0x2C, +0x30), the tile size
/// (+0x3C) and the texture array (+0x40). After the draw set-up
/// (`00ec0460`) it turns render states 7 and 0xE off and sets state 0xA8 to
/// 0xF; then, row by row from `y`, each full tile and a partial last column
/// gets its texture (`SetTexture(0, ..)`) and a quad (`00ec0280`), the x
/// position advancing by `tile size * width` and the y by
/// `tile size * height`; a partial last row is drawn the same way. Finally
/// states 7 and 0xE back on and texture stage 0 cleared.
pub fn fn_00ebff30(e: &mut Engine, set: Ptr, x: f32, y: f32, width: f32, height: f32) {
    let set = set.addr();
    let device = |e: &Engine| e.mem.u32(set);
    let call_device = |e: &mut Engine, slot: u32, words: &[u32]| {
        let device = e.mem.u32(set);
        e.vcall(device, slot, words);
    };
    let d = device(e);
    e.call(DRAW_SETUP, &args![d]);
    call_device(e, SET_RENDER_STATE, &args![7u32, 0u32]);
    call_device(e, SET_RENDER_STATE, &args![0xeu32, 0u32]);
    call_device(e, SET_RENDER_STATE, &args![0xa8u32, 0xfu32]);
    let mut texture = e.mem.u32(set + 0x40);
    // Two products the code computes into locals and never reads: the tile
    // size times +8, and +0x1C times the tile size minus +0x24 times it
    // times +8 (plain loads, nothing written).
    let tile = e.mem.u32(set + 0x3c);
    let step_x = word_times(tile, width.to_bits());
    let step_y = word_times(e.mem.u32(set + 0x3c), height.to_bits());
    let mut row_y = x87_quiet_bits(y.to_bits());
    let width = x87_quiet_bits(width.to_bits());
    let height = x87_quiet_bits(height.to_bits());
    let x = x87_quiet_bits(x.to_bits());
    let draw = |e: &mut Engine, texture: &mut u32, at_x: u32, at_y: u32, w: u32, h: u32| {
        let current = e.mem.u32(*texture);
        call_device(e, SET_TEXTURE, &args![0u32, current]);
        *texture = texture.wrapping_add(4);
        let d = device(e);
        e.call(DRAW_QUAD, &args![d, at_x, at_y, width, height, w, h]);
    };
    let mut row = 0u32;
    while row < e.mem.u32(set + 0x28) {
        let mut column_x = x;
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let size = e.mem.u32(set + 0x3c);
            draw(e, &mut texture, column_x, row_y, size, size);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let (w, h) = (e.mem.u32(set + 0x2c), e.mem.u32(set + 0x3c));
            draw(e, &mut texture, column_x, row_y, w, h);
        }
        row_y = advance(row_y, step_y);
        row += 1;
    }
    if e.mem.u32(set + 0x30) != 0 {
        let mut column_x = x;
        let mut column = 0u32;
        while column < e.mem.u32(set + 0x24) {
            let (w, h) = (e.mem.u32(set + 0x3c), e.mem.u32(set + 0x30));
            draw(e, &mut texture, column_x, row_y, w, h);
            column_x = advance(column_x, step_x);
            column += 1;
        }
        if e.mem.u32(set + 0x2c) != 0 {
            let (w, h) = (e.mem.u32(set + 0x2c), e.mem.u32(set + 0x30));
            draw(e, &mut texture, column_x, row_y, w, h);
        }
    }
    call_device(e, SET_RENDER_STATE, &args![7u32, 1u32]);
    call_device(e, SET_RENDER_STATE, &args![0xeu32, 1u32]);
    call_device(e, SET_TEXTURE, &args![0u32, 0u32]);
}

/// The `0.0` test of `Play`'s duration (`FCOMP qword`, `TEST AH,0x44`,
/// `JNP`): true when it is not equal (a NaN is not equal).
fn has_duration(e: &Engine, duration: u32) -> bool {
    x87_load32(duration) != f64::from_bits(e.mem.u64(ZERO))
}

// Translated from 00ec0e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Movie::Play` (Xbox PDB; `thiscall`, nine words, `RET 0x24`): marks the
/// movie playing (+0x20 = 1) and busy (+0x24) and opens it (`00ec0cd0` with
/// `name`, `open_1`, `open_2`, `open_3`); a failed open clears busy and
/// answers false. Otherwise clears busy, notifies the settings object
/// (`notify_a`: slots +4/+8, `notify_b`: +0xC/+0x10, before and after),
/// points the device's render target 0 (slot +0x94) at the renderer's first
/// target's surface (+0x14) when that target's class chain reaches the
/// `NiRTTI` at `0126f59c`, then runs `Update(go_on, keep_going, open_3)`
/// until it answers false. With a non-zero `duration` (seconds) `go_on`
/// starts at 0, and once more than `duration * 1000` ticks (truncated, low
/// word) have passed it becomes 1 and the duration 0. Answers whether every
/// frame was shown (`+0xC >= the Bink frame count`), after closing the
/// movie (`00ec1020`).
#[allow(clippy::too_many_arguments)]
pub fn movie_play(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    go_on: u8,
    open_1: u8,
    keep_going: u8,
    open_2: u8,
    open_3: u8,
    notify_a: u8,
    notify_b: u8,
    duration: f32,
) -> bool {
    let at = this.addr();
    e.mem.set_u32(at + 0x20, 1);
    e.mem.set_u8(at + 0x24, 1);
    let opened = e
        .call(
            MOVIE_OPEN,
            &args![
                at,
                name,
                u32::from(open_1),
                u32::from(open_2),
                u32::from(open_3)
            ],
        )
        .u8();
    if opened == 0 {
        e.mem.set_u8(at + 0x24, 0);
        return false;
    }
    e.mem.set_u8(at + 0x24, 0);
    if notify_a != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x4, &[]);
    }
    if notify_b != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0xc, &[]);
    }
    let renderer = e.mem.u32(RENDERER);
    let group = e.vcall(renderer, 0xc8, &[]).u32();
    let target = e.vcall(group, 0xc8, &args![0u32]).u32();
    let matching = if target == 0 {
        0
    } else {
        let mut rtti = e.vcall(target, 0xc, &[]).u32();
        let mut found = false;
        while rtti != 0 {
            if rtti == TARGET_RTTI {
                found = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
        if found {
            target
        } else {
            0
        }
    };
    if matching != 0 {
        let surface = e.mem.u32(matching + 0x14);
        let device = e.mem.u32(at + 4);
        e.vcall(device, SET_RENDER_TARGET, &args![0u32, surface]);
    }
    let mut go_on = go_on;
    let mut duration = duration.to_bits();
    if has_duration(e, duration) {
        go_on = 0;
    }
    let start = e.call(TICK_COUNT, &[]).u32();
    while movie_update(e, this, go_on, keep_going, open_3) {
        if has_duration(e, duration) {
            let now = e.call(TICK_COUNT, &[]).u32();
            let elapsed = now.wrapping_sub(start);
            let thousand = f64::from_bits(e.mem.u64(THOUSAND));
            let limit = x87_fistp64_chop(X87Ext::from(x87_mul(x87_load32(duration), thousand)));
            if elapsed > limit as u32 {
                go_on = 1;
                duration = 0.0f32.to_bits();
            }
        }
    }
    let bink = e.mem.u32(at);
    let shown_all = e.mem.u32(at + 0xc) >= e.mem.u32(bink + 8);
    e.call(MOVIE_CLOSE, &args![at]);
    if notify_a != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x8, &[]);
    }
    if notify_b != 0 {
        let settings = e.mem.u32(MOVIE_SETTINGS);
        e.vcall(settings, 0x10, &[]);
    }
    shown_all
}

/// The letterbox viewport `Movie::Update` sets: the back buffer size
/// (renderer slot +0xD4: width +8, height +0xC) times the fractions
/// (`FILD qword`, `FMUL`, truncating `FISTP qword`, low word).
fn letterbox_viewport(e: &mut Engine, viewport: u32) {
    let renderer = e.mem.u32(RENDERER);
    let description = e.vcall(renderer, 0xd4, &[]).u32();
    let width = e.mem.u32(description + 8);
    let renderer = e.mem.u32(RENDERER);
    let description = e.vcall(renderer, 0xd4, &[]).u32();
    let height = e.mem.u32(description + 0xc);
    let left = e.mem.u32(VIEW_LEFT);
    let right = e.mem.u32(VIEW_RIGHT);
    let bottom = e.mem.u32(VIEW_BOTTOM);
    let top = e.mem.u32(VIEW_TOP);
    let chop = |value: f64| x87_fistp64_chop(X87Ext::from(value)) as u32;
    let x = chop(x87_mul(f64::from(width), x87_m32(left)));
    let w = chop(x87_mul(
        x87_sub(x87_load32(right), x87_m32(left)),
        f64::from(width),
    ));
    let y = chop(x87_mul(f64::from(height), x87_m32(top)));
    let h = chop(x87_mul(
        x87_sub(x87_load32(bottom), x87_m32(top)),
        f64::from(height),
    ));
    for (offset, word) in [x, y, w, h, 0.0f32.to_bits(), 1.0f32.to_bits()]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(viewport + 4 * offset as u32, word);
    }
}

// Translated from 00ec1060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Movie::Update` (Xbox PDB; `thiscall`, three bytes, `RET 0xC`): false
/// when the settings object's slot +0x40 (with `go_on`) answers false.
/// Unless the settings' byte +0x19 is set, it waits on Bink: a frame not
/// ready sleeps 1 ms and answers true (unless the state +0x20 is 2, which
/// draws anyway); a ready frame is copied (`00ec14d0`; a skipped copy counts
/// +0x10 and reports it through `Error`), advanced (`BinkNextFrame`) and
/// counted (+0xC). Then it clears the target (black, z 1.0), sets the
/// letterbox viewport (unless `no_viewport`, when `011f9426` is set),
/// draws the frame (`00ebff30` with the position and the scale as both tile
/// sizes) inside a scene unless the settings' slot +0x30 says not to,
/// presents unless the state is 2, restores the full viewport the same way,
/// and answers whether frames remain or `keep_going` is set.
pub fn movie_update(e: &mut Engine, this: Ptr, go_on: u8, keep_going: u8, no_viewport: u8) -> bool {
    let at = this.addr();
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x40, &args![u32::from(go_on)]).bool() {
        return false;
    }
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if e.mem.u8(settings + 0x19) == 0 {
        let bink = e.mem.u32(at);
        if e.call(BINK_WAIT, &args![bink]).u32() == 0 {
            let bink = e.mem.u32(at);
            let textures = e.mem.u32(at + 8);
            let skipped = e.call(COPY_FRAME, &args![bink, textures, 0u32]).u8();
            if skipped != 0 {
                let count = e.mem.u32(at + 0x10).wrapping_add(1);
                e.mem.set_u32(at + 0x10, count);
                let frame = e.mem.u32(at + 0xc);
                e.call(ERROR, &args![SKIPPED_FRAME_MESSAGE, frame, count]);
            }
            let bink = e.mem.u32(at);
            e.call(BINK_NEXT_FRAME, &args![bink]);
            let shown = e.mem.u32(at + 0xc).wrapping_add(1);
            e.mem.set_u32(at + 0xc, shown);
        } else if e.mem.u32(at + 0x20) != 2 {
            e.call(SLEEP, &args![1u32]);
            return true;
        }
    }
    let device = e.mem.u32(at + 4);
    e.vcall(
        device,
        CLEAR,
        &args![0u32, 0u32, 1u32, 0xff00_0000u32, 1.0f32, 0u32],
    );
    if no_viewport == 0 && e.mem.u8(LETTERBOX) != 0 {
        e.with_stack(0x18, |e, viewport| {
            letterbox_viewport(e, viewport.addr());
            let device = e.mem.u32(at + 4);
            e.vcall(device, SET_VIEWPORT, &args![viewport]);
        });
    }
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x30, &[]).bool() {
        let device = e.mem.u32(at + 4);
        e.vcall(device, BEGIN_SCENE, &[]);
    }
    let textures = e.mem.u32(at + 8);
    let x = x87_quiet_bits(e.mem.u32(at + 0x18));
    let y = x87_quiet_bits(e.mem.u32(at + 0x1c));
    let scale = x87_quiet_bits(e.mem.u32(at + 0x14));
    fn_00ebff30(
        e,
        Ptr::new(textures),
        f32::from_bits(x),
        f32::from_bits(y),
        f32::from_bits(scale),
        f32::from_bits(scale),
    );
    let settings = e.mem.u32(MOVIE_SETTINGS);
    if !e.vcall(settings, 0x30, &[]).bool() {
        let device = e.mem.u32(at + 4);
        e.vcall(device, END_SCENE, &[]);
    }
    if e.mem.u32(at + 0x20) != 2 {
        let device = e.mem.u32(at + 4);
        e.vcall(device, PRESENT, &args![0u32, 0u32, 0u32, 0u32]);
    }
    if no_viewport == 0 && e.mem.u8(LETTERBOX) != 0 {
        let renderer = e.mem.u32(RENDERER);
        let description = e.vcall(renderer, 0xd4, &[]).u32();
        let width = e.mem.u32(description + 8);
        let renderer = e.mem.u32(RENDERER);
        let description = e.vcall(renderer, 0xd4, &[]).u32();
        let height = e.mem.u32(description + 0xc);
        e.with_stack(0x18, |e, viewport| {
            let viewport = viewport.addr();
            for (offset, word) in [0, 0, width, height, 0.0f32.to_bits(), 1.0f32.to_bits()]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(viewport + 4 * offset as u32, word);
            }
            let device = e.mem.u32(at + 4);
            e.vcall(device, SET_VIEWPORT, &args![viewport]);
        });
    }
    let bink = e.mem.u32(at);
    e.mem.u32(at + 0xc) < e.mem.u32(bink + 8) || keep_going != 0
}

// Translation incomplete: 00ec15e0 - the FS:[0] exception frame (unwinding the name map when the clean-up throws) is not translated
// Translated from 00ec15e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body the `MoviePlayer` destructor
/// (`thiscall`): stores its vtable, stops movies (`00ec25e0`) while the
/// count at +0xC is non-zero, cleans up (`00ec2e50`), destroys the name map
/// at +0x40 (its vtable, then `00ec1a90`) and closes the handle at +0x34.
pub fn fn_00ec15e0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, PLAYER_VTABLE);
    while e.mem.u32(at + 0xc) != 0 {
        e.call(PLAYER_STOP, &args![at]);
    }
    e.call(PLAYER_CLEAN_UP, &args![at]);
    e.mem.set_u32(at + 0x40, NAME_MAP_VTABLE);
    fn_00ec1a90(e, Ptr::new(at + 0x40));
    let handle = e.mem.u32(at + 0x34);
    e.call(CLOSE_HANDLE, &args![handle]);
}

// Translated from 00ec17c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::GetPlayingSequence` (engine map; `thiscall`): false without
/// a thread handle (+8), else whether `GetExitCodeThread` reports the
/// thread still active (`0x103`).
pub fn movie_player_get_playing_sequence(e: &mut Engine, this: Ptr) -> bool {
    let thread = e.mem.u32(this.addr() + 8);
    if thread == 0 {
        return false;
    }
    e.with_stack(4, |e, code| {
        e.call(GET_EXIT_CODE_THREAD, &args![thread, code]);
        e.mem.u32(code.addr()) == STILL_ACTIVE
    })
}

// Translation incomplete: 00ec1a90 - the FS:[0] exception frame (unwinding the base destructors when one throws) is not translated
// Translated from 00ec1a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char*, char*>, char*>::~NiTStringTemplateMap`
/// (engine map): stores its vtable; when the map owns its keys (byte +0x10)
/// frees each item's key (+4) in every bucket (+8, count +4) walking the
/// chains (next at +0); then the base vtables with the base destructor
/// body (`00438af0`) twice, and frees the bucket array.
pub fn ni_t_string_template_map_destruct(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, STRING_MAP_VTABLE);
    if e.mem.u8(at + 0x10) != 0 {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(at + 4) {
            let buckets = e.mem.u32(at + 8);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let next = e.mem.u32(item);
                let key = e.mem.u32(item + 4);
                e.call(NI_FREE, &args![key]);
                item = next;
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(at, MAP_BASE_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    e.mem.set_u32(at, MAP_ROOT_VTABLE);
    e.call(MAP_BASE_DESTRUCT, &args![at]);
    let buckets = e.mem.u32(at + 8);
    e.call(NI_FREE, &args![buckets]);
}

/// The address `00ec15e0` calls the map destructor at.
fn fn_00ec1a90(e: &mut Engine, this: Ptr) {
    ni_t_string_template_map_destruct(e, this);
}

// Translated from 00ec1d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<char*>::KeyToHashIndex` (engine map;
/// `thiscall`, one word, `RET 4`): `hash = hash * 33 + lower(c)` over the
/// key's characters (each sign-extended, lowered by `00ec67aa`, the answer's
/// low byte sign-extended), modulo the bucket count (+4).
pub fn bst_case_insensitive_string_map_key_to_hash_index(
    e: &mut Engine,
    this: Ptr,
    key: u32,
) -> u32 {
    let mut hash = 0u32;
    let mut at = key;
    loop {
        let c = e.mem.u8(at) as i8;
        if c == 0 {
            break;
        }
        let lower = e.call(TO_LOWER, &args![i32::from(c)]).u8() as i8;
        hash = (hash << 5)
            .wrapping_add(hash)
            .wrapping_add(i32::from(lower) as u32);
        at = at.wrapping_add(1);
    }
    hash % e.mem.u32(this.addr() + 4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ebff30, fn_00ebff30(Ptr, f32, f32, f32, f32)),
        entry!(
            0x00ec0e00,
            movie_play(Ptr, u32, u8, u8, u8, u8, u8, u8, u8, f32) -> bool
        ),
        entry!(0x00ec1060, movie_update(Ptr, u8, u8, u8) -> bool),
        entry!(0x00ec15e0, fn_00ec15e0(Ptr)),
        entry!(0x00ec17c0, movie_player_get_playing_sequence(Ptr) -> bool),
        entry!(0x00ec1a90, ni_t_string_template_map_destruct(Ptr)),
        entry!(
            0x00ec1d50,
            bst_case_insensitive_string_map_key_to_hash_index(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A device whose every slot up to +0x108 is a recording no-op at
    /// `0x0020_0000 + slot`.
    fn device(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x120);
        for slot in (0..0x120u32).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0020_0000 + slot);
            e.register(0x0020_0000 + slot, |_, _| Ret::default());
        }
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, vtable);
        device
    }

    #[test]
    fn hash_index_lowers_and_wraps() {
        let mut e = Engine::new();
        e.register(TO_LOWER, |_, a| {
            let c = a[0] as u8;
            u32::from(c.to_ascii_lowercase()).into_ret()
        });
        let key = e.mem.alloc(4);
        e.mem.write(key, b"Ab\0");
        let map = e.mem.alloc(0x14);
        e.mem.set_u32(map + 4, 37);
        let expected = (u32::from(b'a') * 33 + u32::from(b'b')) % 37;
        assert_eq!(e.call(0x00ec_1d50, &args![map, key]).u32(), expected);
    }

    #[test]
    fn playing_sequence_asks_the_thread() {
        let mut e = Engine::new();
        e.register(GET_EXIT_CODE_THREAD, |e, a| {
            e.mem
                .set_u32(a[1], if a[0] == 1 { STILL_ACTIVE } else { 0 });
            1u32.into_ret()
        });
        let player = e.mem.alloc(0x10);
        assert!(!e.call(0x00ec_17c0, &args![player]).bool());
        e.mem.set_u32(player + 8, 1);
        assert!(e.call(0x00ec_17c0, &args![player]).bool());
        e.mem.set_u32(player + 8, 2);
        assert!(!e.call(0x00ec_17c0, &args![player]).bool());
    }

    #[test]
    fn map_destructor_frees_owned_keys() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(MAP_BASE_DESTRUCT, |_, _| Ret::default());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x111);
        e.mem.set_u32(second + 4, 0x222);
        e.mem.set_u32(buckets + 4, first);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1a90, &args![map]);
        assert_eq!(
            calls_to(&e, NI_FREE),
            vec![vec![0x111], vec![0x222], vec![buckets]]
        );
        assert_eq!(calls_to(&e, MAP_BASE_DESTRUCT).len(), 2);
        assert_eq!(e.mem.u32(map), MAP_ROOT_VTABLE);
        // Keys not owned: only the buckets are freed.
        e.mem.set_u8(map + 0x10, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1a90, &args![map]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![buckets]]);
    }

    #[test]
    fn player_destructor_stops_then_closes() {
        let mut e = Engine::new();
        e.register(PLAYER_STOP, |e, a| {
            let count = e.mem.u32(a[0] + 0xc) - 1;
            e.mem.set_u32(a[0] + 0xc, count);
            Ret::default()
        });
        for address in [PLAYER_CLEAN_UP, MAP_BASE_DESTRUCT, NI_FREE, CLOSE_HANDLE] {
            e.register(address, |_, _| Ret::default());
        }
        let player = e.mem.alloc(0x60);
        e.mem.set_u32(player + 0xc, 2);
        e.mem.set_u32(player + 0x34, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00ec_15e0, &args![player]);
        assert_eq!(calls_to(&e, PLAYER_STOP).len(), 2);
        assert_eq!(e.mem.u32(player), PLAYER_VTABLE);
        assert_eq!(e.mem.u32(player + 0x40), MAP_ROOT_VTABLE);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x99]]);
    }

    #[test]
    fn frame_draw_tiles_the_grid() {
        let mut e = Engine::new();
        e.register(DRAW_SETUP, |_, _| Ret::default());
        e.register(DRAW_QUAD, |_, _| Ret::default());
        let device = device(&mut e);
        let set = e.mem.alloc(0x44);
        let textures = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(textures + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(set, device);
        e.mem.set_u32(set + 0x24, 2);
        e.mem.set_u32(set + 0x28, 1);
        e.mem.set_u32(set + 0x2c, 5);
        e.mem.set_u32(set + 0x30, 7);
        e.mem.set_u32(set + 0x3c, 64);
        e.mem.set_u32(set + 0x40, textures);
        e.call_log = Some(vec![]);
        e.call(0x00eb_ff30, &args![set, 10.0f32, 20.0f32, 0.5f32, 2.0f32]);
        let quads = calls_to(&e, DRAW_QUAD);
        let f = |v: f32| v.to_bits();
        let size = (f(0.5), f(2.0));
        let quad = |x: f32, y: f32, w: u32, h: u32| vec![device, f(x), f(y), size.0, size.1, w, h];
        assert_eq!(
            quads,
            vec![
                quad(10.0, 20.0, 64, 64),
                quad(42.0, 20.0, 64, 64),
                quad(74.0, 20.0, 5, 64),
                quad(10.0, 148.0, 64, 7),
                quad(42.0, 148.0, 64, 7),
                quad(74.0, 148.0, 5, 7),
            ]
        );
        let textures_set: Vec<u32> = calls_to(&e, 0x0020_0000 + SET_TEXTURE)
            .into_iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(textures_set, [0x100, 0x101, 0x102, 0x103, 0x104, 0x105, 0]);
        let states = calls_to(&e, 0x0020_0000 + SET_RENDER_STATE);
        assert_eq!(states[2], vec![device, 0xa8, 0xf]);
        assert_eq!(states[4], vec![device, 0xe, 1]);
    }

    /// A movie on a device, a settings object answering `go_on` from slot
    /// +0x40 and false from +0x30, and a renderer with a 1000x500 back
    /// buffer.
    fn movie_engine(frames: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let device = device(&mut e);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x6000);
        e.map(0x011a_d000, 0x1000);
        let settings_vtable = e.mem.alloc(0x50);
        for slot in (0..0x50u32).step_by(4) {
            e.mem.set_u32(settings_vtable + slot, 0x0021_0000 + slot);
            e.register(0x0021_0000 + slot, |_, _| Ret::default());
        }
        e.register(0x0021_0040, |_, a| a[1].into_ret());
        let settings = e.mem.alloc(0x20);
        e.mem.set_u32(settings, settings_vtable);
        e.set_global(MOVIE_SETTINGS, settings);
        let description = e.mem.alloc(0x10);
        e.mem.set_u32(description + 8, 1000);
        e.mem.set_u32(description + 0xc, 500);
        let renderer_vtable = e.mem.alloc(0xd8);
        e.mem.set_u32(renderer_vtable + 0xd4, 0x0022_00d4);
        e.mem.set_u32(renderer_vtable + 0xc8, 0x0022_00c8);
        e.register_double(0x0022_00d4, move |_, _| description.into_ret());
        // The group is the renderer itself; it has no first target.
        e.register(0x0022_00c8, |_, a| {
            if a.len() == 1 {
                a[0].into_ret()
            } else {
                Ret::default()
            }
        });
        let renderer = e.mem.alloc(8);
        e.mem.set_u32(renderer, renderer_vtable);
        e.set_global(RENDERER, renderer);
        e.set_global(VIEW_LEFT, 0.25f32);
        e.set_global(VIEW_RIGHT, 0.75f32);
        e.set_global(VIEW_TOP, 0.25f32);
        e.set_global(VIEW_BOTTOM, 0.75f32);
        e.set_global(LETTERBOX, 1u8);
        for address in [BINK_NEXT_FRAME, SLEEP, ERROR, DRAW_SETUP, DRAW_QUAD] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(BINK_WAIT, |_, _| Ret::default());
        e.register(COPY_FRAME, |_, _| 1u32.into_ret());
        let bink = e.mem.alloc(0x10);
        e.mem.set_u32(bink + 8, frames);
        let set = e.mem.alloc(0x44);
        e.mem.set_u32(set, device);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(movie, bink);
        e.mem.set_u32(movie + 4, device);
        e.mem.set_u32(movie + 8, set);
        (e, movie, device)
    }

    #[test]
    fn update_shows_a_frame_with_the_letterbox() {
        let (mut e, movie, device) = movie_engine(2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 0u32]).bool());
        assert_eq!(e.mem.u32(movie + 0xc), 1);
        assert_eq!(e.mem.u32(movie + 0x10), 1);
        assert_eq!(calls_to(&e, ERROR)[0][1..], [0, 1]);
        let clear = calls_to(&e, 0x0020_0000 + CLEAR);
        assert_eq!(
            clear,
            vec![vec![device, 0, 0, 1, 0xff00_0000, 1.0f32.to_bits(), 0]]
        );
        assert_eq!(calls_to(&e, 0x0020_0000 + SET_VIEWPORT).len(), 2);
        assert_eq!(calls_to(&e, 0x0020_0000 + BEGIN_SCENE).len(), 1);
        assert_eq!(calls_to(&e, 0x0020_0000 + PRESENT).len(), 1);
        // The last frame: no frames remain.
        assert!(!e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 1u32]).bool());
        // ... unless kept going.
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 1u32, 1u32]).bool());
        // The settings refuse: false straight away.
        assert!(!e.call(0x00ec_1060, &args![movie, 0u32, 1u32, 1u32]).bool());
    }

    #[test]
    fn update_sleeps_on_a_frame_not_ready() {
        let (mut e, movie, _) = movie_engine(2);
        e.register(BINK_WAIT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ec_1060, &args![movie, 1u32, 0u32, 0u32]).bool());
        assert_eq!(calls_to(&e, SLEEP), vec![vec![1]]);
        assert!(calls_to(&e, 0x0020_0000 + CLEAR).is_empty());
    }

    #[test]
    fn letterbox_uses_the_fractions() {
        let (mut e, _, _) = movie_engine(1);
        let viewport = e.mem.alloc(0x18);
        letterbox_viewport(&mut e, viewport);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(viewport + 4 * i)).collect();
        assert_eq!(words, [250, 125, 500, 250]);
        assert_eq!(e.mem.f32(viewport + 0x14), 1.0);
    }

    #[test]
    fn play_runs_updates_until_done() {
        let (mut e, movie, _) = movie_engine(2);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_u64(THOUSAND, 1000.0f64.to_bits());
        e.register(MOVIE_OPEN, |_, _| 1u32.into_ret());
        e.register(MOVIE_CLOSE, |_, _| Ret::default());
        e.register(TICK_COUNT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let played = e
            .call(
                0x00ec_0e00,
                &args![movie, 0x55u32, 1u32, 2u32, 0u32, 3u32, 4u32, 1u32, 0u32, 0.0f32],
            )
            .bool();
        assert!(played);
        assert_eq!(calls_to(&e, MOVIE_OPEN), vec![vec![movie, 0x55, 2, 3, 4]]);
        assert_eq!(e.mem.u32(movie + 0xc), 2);
        assert_eq!(e.mem.u32(movie + 0x20), 1);
        assert_eq!(calls_to(&e, 0x0021_0004).len(), 1);
        assert_eq!(calls_to(&e, 0x0021_0008).len(), 1);
        assert!(calls_to(&e, 0x0021_000c).is_empty());
        assert_eq!(calls_to(&e, MOVIE_CLOSE), vec![vec![movie]]);
        // A failed open answers false and clears busy.
        e.register(MOVIE_OPEN, |_, _| Ret::default());
        e.mem.set_u8(movie + 0x24, 0);
        let played = e
            .call(
                0x00ec_0e00,
                &args![movie, 0x55u32, 1u32, 2u32, 0u32, 3u32, 4u32, 1u32, 0u32, 0.0f32],
            )
            .bool();
        assert!(!played);
        assert_eq!(e.mem.u8(movie + 0x24), 0);
    }
}
