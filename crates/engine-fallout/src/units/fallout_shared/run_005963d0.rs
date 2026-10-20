//! `fallout shared/run_005963d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00598040 (decompiled, FalloutNV.exe 1.4.0.525)
/// A plain getter: the `float` at `+0x3c` of `this`, returned in `ST0`. The
/// engine map has no name for it; the translations that call it
/// (`animation.rs`, `inventorychanges.rs`) use it on a sequence object, which
/// this body does not confirm.
pub fn fn_00598040(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x3c)
}

// Translated from 00597610 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00597610 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6f (0xc4 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_42ec`) is `MediaSet`, so this is the constructor of `MediaSet`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It also clears the global at `011ca9fc`.
/// The exception-unwinding frame is not translated.
pub fn fn_00597610(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_42ec);
    e.mem.set_u32(base + 0x18, 0x0103_42d8);
    e.call(0x0040_2d00, &args![base + 0x38]);
    e.call(0x0059_77d0, &args![base + 0x48]);
    e.call(0x0059_77d0, &args![base + 0x58]);
    e.call(0x0059_77d0, &args![base + 0x68]);
    e.call(0x0059_77d0, &args![base + 0x78]);
    e.call(0x0059_77d0, &args![base + 0x88]);
    e.call(0x0059_77d0, &args![base + 0x98]);
    e.call(0x004f_15a0, &args![this, 0x6fu32]);
    e.mem.set_u32(base + 0x44, 0xffff_ffff);
    e.call(0x0059_7850, &args![base + 0x48]);
    e.call(0x0059_7850, &args![base + 0x58]);
    e.call(0x0059_7850, &args![base + 0x68]);
    e.call(0x0059_7850, &args![base + 0x78]);
    e.call(0x0059_7850, &args![base + 0x88]);
    e.call(0x0059_7850, &args![base + 0x98]);
    e.mem.set_f32(base + 0xac, 0.0);
    e.mem.set_f32(base + 0xb0, 0.0);
    e.mem.set_f32(base + 0xb4, 0.0);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u32(base + 0xbc, 0x0);
    e.mem.set_u32(base + 0xc0, 0x0);
    e.mem.set_u32(0x011c_a9fc, 0x0);
    e.mem.set_u32(base + 0x2c, 0x0);
    e.mem.set_u32(base + 0x28, 0x0);
    e.mem.set_u8(base + 0x30, 0x0);
    e.mem.set_u8(base + 0x31, 0x0);
    e.mem.set_u8(base + 0x33, 0x0);
    e.mem.set_u8(base + 0x34, 0x0);
    this
}

// Translated from 00597880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005978b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00597880(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_78b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// Empties a list (`00470470`, on the list) and adds an item to one
/// (`005ae3d0`, given the item's address).
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_ADD: u32 = 0x005a_e3d0;
/// The six form lists of the location controller (+0x88 .. +0xB0).
const CONTROLLER_LISTS: [u32; 6] = [0x88, 0x90, 0x98, 0xa0, 0xa8, 0xb0];
/// `BSStringT` set (`004037f0`: text, length).
const STRING_SET: u32 = 0x0040_37f0;

/// The audio media helpers on channel 2: whether media is playing
/// (`FalloutAudioMedia::MediaIsPlaying` in the engine map, `00830750`,
/// cdecl), its length (`00830970`), its position (`00830a20`), the fade
/// out (`00830680`, cdecl: channel, milliseconds), the start (`008300c0`,
/// cdecl, seven words) and the update after it (`008304c0`).
const MEDIA_IS_PLAYING: u32 = 0x0083_0750;
const MEDIA_LENGTH: u32 = 0x0083_0970;
const MEDIA_POSITION: u32 = 0x0083_0a20;
const MEDIA_FADE_OUT: u32 = 0x0083_0680;
const MEDIA_START: u32 = 0x0083_00c0;
const MEDIA_UPDATE: u32 = 0x0083_04c0;
/// `_ftol2_sse` (`00ec62c0`): the value in ST0 truncated (an `f64`
/// argument).
const FTOL: u32 = 0x00ec_62c0;
/// `1000.0` (`double`, milliseconds per second), `100.0` (`double`) and
/// `1000.0` (`float`), as the exe holds them.
const THOUSAND: u32 = 0x0101_7b70;
const HUNDRED: u32 = 0x0101_7a40;
const THOUSAND_FLOAT: u32 = 0x0101_3974;
/// The time (milliseconds) before which the music type does not start a
/// new track.
const NEXT_TRACK_TIME: u32 = 0x011d_d37c;
/// The time `00597990`'s mode 3 compares with.
const MEDIA_SET_TIME: u32 = 0x011c_a9fc;

// Translated from 005963d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005963d0 - the /GS stack cookie check (00ec408c) is not translated.
/// The media location controller's record reader (`RET 4`: the
/// `TESFile`; the engine map has no name; the form type is not checked):
/// `TESForm::LoadForm`, `00484ab0(0)` and each chunk: `EDID`, `OBND`,
/// `FULL` (+0x18), `NAM1`..`NAM7` (words at +0x6C, +0x70, +0x74, +0x7C,
/// +0x80, +0x84, +0x78), `FNAM`/`RNAM` (words at +0x64/+0x60) and
/// `HNAM`/`YNAM`/`WNAM`/`XNAM`/`LNAM`/`GNAM` (a form id added to the list
/// at +0x88/+0x90/+0x98/+0xA0/+0xA8/+0xB0). The form id local starts at 0
/// and keeps the last value read. Returns true.
pub fn fn_005963d0(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let id = e.mem.alloc(4);
    let copy = e.mem.alloc(4);
    e.mem.set_u32(id, 0);
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        // The `xNAM` chunks from `FNAM` on go through the exe's byte map.
        let list = match chunk {
            0x4d41_4e48 => Some(0x88u32), // HNAM
            0x4d41_4e59 => Some(0x90),    // YNAM
            0x4d41_4e57 => Some(0x98),    // WNAM
            0x4d41_4e58 => Some(0xa0),    // XNAM
            0x4d41_4e4c => Some(0xa8),    // LNAM
            0x4d41_4e47 => Some(0xb0),    // GNAM
            _ => None,
        };
        if let Some(list) = list {
            e.call(FILE_CHUNK_DATA, &args![file, id]);
            e.mem.set_u32(copy, e.mem.u32(id));
            e.call(LIST_ADD, &args![base + list, copy]);
        } else {
            match chunk {
                // FULL
                0x4c4c_5546 => {
                    let full = if this.is_null() { 0 } else { base + 0x18 };
                    e.call(0x0048_7050, &args![full, file]);
                }
                // NAM1 .. NAM7
                0x314d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x6c]);
                }
                0x324d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x70]);
                }
                0x334d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x74]);
                }
                0x344d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x7c]);
                }
                0x354d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x80]);
                }
                0x364d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x84]);
                }
                0x374d_414e => {
                    e.call(FILE_CHUNK_DATA, &args![file, base + 0x78]);
                }
                // EDID
                0x4449_4445 => {
                    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                    // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
                    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                        e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                        e.vcall(base, 0x134, &args![text]);
                    });
                }
                // OBND
                0x444e_424f => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                // RNAM
                0x4d41_4e52 => {
                    e.call(FILE_CHUNK_DATA, &args![file, id]);
                    e.mem.set_u32(base + 0x60, e.mem.u32(id));
                }
                // FNAM
                0x4d41_4e46 => {
                    e.call(FILE_CHUNK_DATA, &args![file, id]);
                    e.mem.set_u32(base + 0x64, e.mem.u32(id));
                }
                _ => {}
            }
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.mem.free(copy);
    e.mem.free(id);
    true
}

// Translated from 005974f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media location controller's defaults: three floats from the exe's
/// constants at +0x70, +0x74 and +0x7C (FLD/FSTP), 0x40 at +0x80, 0x150 at
/// +0x84, its six lists emptied, then 0, 1 and 0 at +0x60, +0x64 and
/// +0x68.
pub fn fn_005974f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, constant) in [
        (0x70u32, 0x0102_6994u32),
        (0x74, 0x0103_40a4),
        (0x7c, 0x0103_4294),
    ] {
        let value = e.global::<f32>(constant);
        e.mem.set_f32(base + offset, x87_quiet(value));
    }
    e.mem.set_u32(base + 0x80, 0x40);
    e.mem.set_u32(base + 0x84, 0x150);
    fn_005975b0(e, this);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 1);
    e.mem.set_u32(base + 0x68, 0);
}

// Translated from 005975b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the media location controller's six lists (+0x88 .. +0xB0).
pub fn fn_005975b0(e: &mut Engine, this: Ptr) {
    for list in CONTROLLER_LISTS {
        e.call(LIST_CLEAR, &args![this.addr() + list]);
    }
}

// Translated from 005977d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005977d0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Constructor of a media set's 0x10-byte track (`00597610` builds six):
/// the name string (`004037b0`, then set empty), the `float` at `01034424`
/// at +8 and the one at `01016410` at +0xC. Returns `this`.
pub fn fn_005977d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_37b0, &args![this]);
    e.call(STRING_SET, &args![this, 0u32, 0u32]);
    let value = e.global::<f32>(0x0103_4424);
    e.mem.set_f32(base + 8, x87_quiet(value));
    let value = e.global::<f32>(0x0101_6410);
    e.mem.set_f32(base + 0xc, x87_quiet(value));
    this
}

// Translated from 00597850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a media set's track: the name set empty, the `float` at
/// `0103395c` at +8 and the one at `01016410` at +0xC.
pub fn fn_00597850(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(STRING_SET, &args![this, 0u32, 0u32]);
    let value = e.global::<f32>(0x0103_395c);
    e.mem.set_f32(base + 8, x87_quiet(value));
    let value = e.global::<f32>(0x0101_6410);
    e.mem.set_f32(base + 0xc, x87_quiet(value));
}

// Translated from 005978b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005978b0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `MediaSet` (the vtables `00597610` stores): puts
/// back the vtables, destroys the six tracks from the last (`007d9ec0` on
/// +0x98 .. +0x48), the strings at +0x38 and +0x18 (`00591300`) and the
/// `TESForm` base (`00483630`).
pub fn fn_005978b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_42ec);
    e.mem.set_u32(base + 0x18, 0x0103_42d8);
    for track in [0x98u32, 0x88, 0x78, 0x68, 0x58, 0x48] {
        e.call(0x007d_9ec0, &args![base + track]);
    }
    e.call(0x0059_1300, &args![base + 0x38]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 00597990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the media set for `time` (`RET 0xC`: time, a flag byte, the
/// location object): with an object, by the set's kind at +0x44: 0
/// `00598550(flag, time, 4)`, 1 `00597a60`, 2 `00598060` (both with the
/// three words), any other false. Without one: kind 3 answers true while
/// `011ca9fc` is past `time` and otherwise runs `00598890(time)` and
/// answers false; kind 0 `00598550(flag, time, 4)`; others false.
pub fn fn_00597990(e: &mut Engine, this: Ptr, time: u32, flag: u8, location: u32) -> bool {
    let kind = e.mem.u32(this.addr() + 0x44);
    if location != 0 {
        match kind {
            0 => e
                .call(0x0059_8550, &args![this, u32::from(flag), time, 4u32])
                .bool(),
            1 => fn_00597a60(e, this, time, flag, location),
            2 => e
                .call(0x0059_8060, &args![this, time, u32::from(flag), location])
                .bool(),
            _ => false,
        }
    } else if kind == 3 {
        if e.global::<u32>(MEDIA_SET_TIME) > time {
            return true;
        }
        e.call(0x0059_8890, &args![this, time]);
        false
    } else if kind == 0 {
        e.call(0x0059_8550, &args![this, u32::from(flag), time, 4u32])
            .bool()
    } else {
        false
    }
}

/// `FISTP qword` with the rounding control set to chop: the low word of
/// the truncated value, of the integer indefinite (0) for NaN and values
/// out of range.
fn fistp_chop_low(value: f64) -> u32 {
    if value.is_nan()
        || !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// One of `00597a60`'s threshold tests: with `bit` set in the flags byte at
/// +0xA8, the `float` at `offset` above `level` and below `limit`, the
/// track `choice` is picked and `limit` lowered to it (FLD/FSTP through a
/// local, the lower of the two by the exe's flag test).
fn pick_track(
    e: &mut Engine,
    this: u32,
    bit: u8,
    offset: u32,
    level: f64,
    limit: &mut u32,
    choice: &mut u8,
    pick: u8,
) {
    if e.mem.u8(this + 0xa8) & bit == 0 {
        return;
    }
    let threshold = x87_load32(e.mem.u32(this + offset));
    // FCOMPP; TEST AH,5; JP: on only when level < threshold.
    if !(level < threshold) {
        return;
    }
    let current = x87_load32(*limit);
    // TEST AH,0x41; JNZ: on only when current > threshold.
    if !(current > threshold) {
        return;
    }
    *choice = pick;
    // TEST AH,5; JP: the threshold unless current < threshold.
    let lower = if current < threshold {
        *limit
    } else {
        e.mem.u32(this + offset)
    };
    *limit = x87_quiet_bits(lower);
}

// Translated from 00597a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The media set's music type run (`RET 0xC`: time in milliseconds, a flag
/// byte, an object (`location`; its getters `00598000`, `00598020`, `00598040` are in this unit); the engine map has no name). A set flag
/// answers false at once. Otherwise: when the current track has played its
/// length (`+0x2C` start plus channel 2's length, less the fade time at
/// +0xB4 in milliseconds, below `time`) it fades out and restarts the
/// clock, answering false unless the object's state (`00598000`) is 0.
/// Then the object's level (`00598040` times 100) picks the track: by the
/// object's byte at +0x37 either the thresholds at +0x54/+0x64/+0x74 (flag
/// bits 1/2/4) or +0x84/+0x94/+0xA4 (bits 8/0x10/0x20), each the lowest
/// threshold above the level. A changed state (not playing, the flag, the
/// object's byte or the track differing from +0x31/+0x33/+0x34) once the
/// time passed `011dd37c`, or a restart, starts the picked track
/// (+0x48/+0x58/+0x68 or +0x78/+0x88/+0x98) on channel 2 with its volume
/// (+8), the fade time and the offset into the playing media, sets the
/// next start (`time` plus +0xAC seconds) and the start time; without a
/// track it fades out what plays and answers false. The flag, the
/// object's byte and the track are kept. Answers true otherwise.
pub fn fn_00597a60(e: &mut Engine, this: Ptr, time: u32, flag: u8, location: u32) -> bool {
    let base = this.addr();
    let playing = e.call(MEDIA_IS_PLAYING, &args![2u32, 0u32]).u8();
    let mut result = true;
    let mut choice = 0u8;
    if flag != 0 {
        return false;
    }
    let thousand = e.global::<f64>(THOUSAND);
    let fade_millis = |e: &mut Engine| {
        let fade = x87_load32(e.mem.u32(base + 0xb4));
        x87_mul(fade, thousand)
    };
    let length = e.call(MEDIA_LENGTH, &args![2u32]).u32();
    let mut restarted = false;
    let start = e.mem.u32(base + 0x2c);
    if start != 0 && length != 0 {
        let end = f64::from(start.wrapping_add(length));
        let fade = fade_millis(e);
        let left = x87_sub(end, fade);
        // FCOMPP; TEST AH,5; JP: on only when left < time.
        if left < f64::from(time) {
            let fade = fade_millis(e);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE_OUT, &args![2u32, millis]);
            e.mem.set_u32(base + 0x2c, time);
            if fn_00598000(e, Ptr::new(location)) != 0 {
                return false;
            }
            restarted = true;
        }
    }
    let level = x87_load32(fn_00598040(e, Ptr::new(location)).to_bits());
    let hundred = e.global::<f64>(HUNDRED);
    let level = x87_load32(x87_store32(x87_mul(level, hundred)));
    let mut limit = x87_quiet_bits(e.global::<u32>(THOUSAND_FLOAT));
    let first_set = fn_00598020(e, Ptr::new(location)) != 0;
    let tests: [(u8, u32); 3] = if first_set {
        [(1, 0x54), (2, 0x64), (4, 0x74)]
    } else {
        [(8, 0x84), (0x10, 0x94), (0x20, 0xa4)]
    };
    for (pick, (bit, offset)) in (1u8..).zip(tests) {
        pick_track(e, base, bit, offset, level, &mut limit, &mut choice, pick);
    }
    let unchanged = playing != 0
        && e.mem.u8(base + 0x31) == flag
        && e.mem.u8(base + 0x33) == fn_00598020(e, Ptr::new(location))
        && e.mem.i8(base + 0x34) == choice as i8;
    let due = !unchanged && e.global::<u32>(NEXT_TRACK_TIME) < time;
    if !due && !restarted {
        return result;
    }
    let tracks: [u32; 3] = if fn_00598020(e, Ptr::new(location)) != 0 {
        [0x48, 0x58, 0x68]
    } else {
        [0x78, 0x88, 0x98]
    };
    let track = match choice {
        1 => base + tracks[0],
        2 => base + tracks[1],
        3 => base + tracks[2],
        _ => 0,
    };
    if track == 0 {
        if playing != 0 {
            let fade = fade_millis(e);
            let millis = e.call(FTOL, &args![fade]).u32();
            e.call(MEDIA_FADE_OUT, &args![2u32, millis]);
        }
        result = false;
    } else {
        let delay = x87_load32(e.mem.u32(base + 0xac));
        let next = x87_add(f64::from(time), x87_mul(delay, thousand));
        e.set_global(NEXT_TRACK_TIME, fistp_chop_low(next));
        let mut offset = 0u32;
        if e.call(MEDIA_POSITION, &args![2u32]).u32() != 0 && !restarted {
            let position = e.call(MEDIA_POSITION, &args![2u32]).u32();
            offset = time.wrapping_sub(position);
        }
        let volume = x87_quiet_bits(e.mem.u32(track + 8));
        let fresh = u32::from(fn_00598000(e, Ptr::new(location)) == 0);
        let fade = fistp_chop_low(fade_millis(e));
        let name = e.call(0x0055_9450, &args![track]).u32();
        e.call(
            MEDIA_START,
            &args![2u32, name, fade, fresh, 1u32, volume, offset],
        );
        e.call(MEDIA_UPDATE, &args![]);
        e.mem.set_u32(base + 0x2c, time);
    }
    e.mem.set_u8(base + 0x31, flag);
    let byte = fn_00598020(e, Ptr::new(location));
    e.mem.set_u8(base + 0x33, byte);
    e.mem.set_u8(base + 0x34, choice);
    result
}

// Translated from 00598000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bits 4 and 5 of the word at +0x6C (`SAR 4`, then `AND 3`).
pub fn fn_00598000(e: &mut Engine, this: Ptr) -> u32 {
    ((e.mem.i32(this.addr() + 0x6c) >> 4) & 3) as u32
}

// Translated from 00598020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x37.
pub fn fn_00598020(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x37)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00598040, fn_00598040(Ptr) -> f32),
        entry!(0x0059_7610, fn_00597610(Ptr) -> Ptr),
        entry!(0x0059_7880, fn_00597880(Ptr, u32) -> Ptr),
        entry!(0x0059_63d0, fn_005963d0(Ptr, Ptr) -> bool),
        entry!(0x0059_74f0, fn_005974f0(Ptr)),
        entry!(0x0059_75b0, fn_005975b0(Ptr)),
        entry!(0x0059_77d0, fn_005977d0(Ptr) -> Ptr),
        entry!(0x0059_7850, fn_00597850(Ptr)),
        entry!(0x0059_78b0, fn_005978b0(Ptr)),
        entry!(0x0059_7990, fn_00597990(Ptr, u32, u8, u32) -> bool),
        entry!(0x0059_7a60, fn_00597a60(Ptr, u32, u8, u32) -> bool),
        entry!(0x0059_8000, fn_00598000(Ptr) -> u32),
        entry!(0x0059_8020, fn_00598020(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_float_at_offset_0x3c() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_f32(object + 0x3c, 1.5);
        e.mem.set_f32(object + 0x38, 9.0);
        assert_eq!(e.call(0x0059_8040, &args![object]).f32(), 1.5);
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00597610_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_7610,
            size: 0xc4,
            globals: &[(0x011c_a9fc, 0xdead_beef)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0040_2d00, &[A::T(0x38)]),
                (0x0059_77d0, &[A::T(0x48)]),
                (0x0059_77d0, &[A::T(0x58)]),
                (0x0059_77d0, &[A::T(0x68)]),
                (0x0059_77d0, &[A::T(0x78)]),
                (0x0059_77d0, &[A::T(0x88)]),
                (0x0059_77d0, &[A::T(0x98)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6f)]),
                (0x0059_7850, &[A::T(0x48)]),
                (0x0059_7850, &[A::T(0x58)]),
                (0x0059_7850, &[A::T(0x68)]),
                (0x0059_7850, &[A::T(0x78)]),
                (0x0059_7850, &[A::T(0x88)]),
                (0x0059_7850, &[A::T(0x98)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_42ec),
                (0x18, 4, 0x0103_42d8),
                (0x44, 4, 0xffff_ffff),
                (0xac, 4, 0x0),
                (0xb0, 4, 0x0),
                (0xb4, 4, 0x0),
                (0xb8, 4, 0x0),
                (0xbc, 4, 0x0),
                (0xc0, 4, 0x0),
                (0x2c, 4, 0x0),
                (0x28, 4, 0x0),
                (0x30, 1, 0x0),
                (0x31, 1, 0x0),
                (0x33, 1, 0x0),
                (0x34, 1, 0x0),
            ],
        });
    }

    #[test]
    fn fn_00597610_clears_the_global() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc4);
        e.mem.map(0x011c_a9fc, 4);
        e.mem.set_u32(0x011c_a9fc, 0xdead_beef);
        for a in [
            0x0048_3370u32,
            0x0040_2d00,
            0x0059_77d0,
            0x0059_7850,
            0x004f_15a0,
        ] {
            e.register(a, |_, _| Ret::default());
        }
        e.call(0x0059_7610, &args![this]);
        assert_eq!(e.mem.u32(0x011c_a9fc), 0);
    }
    #[test]
    fn at_00597880_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_78b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_7880, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_7880)
                .collect();
            let mut expected = vec![(0x0059_78b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod media_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005963d0_reads_the_controller_chunks() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc0);
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0xe0, 0x7000_00e0);
        e.mem.set_u32(vtable + 0x134, 0x7000_0134);
        e.mem.set_u32(this, vtable);
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((
            vec![
                (0x4449_4445u32, 0u32), // EDID
                (0x444e_424f, 0),       // OBND
                (0x4c4c_5546, 0),       // FULL
                (0x314d_414e, 1),       // NAM1
                (0x324d_414e, 2),
                (0x334d_414e, 3),
                (0x344d_414e, 4),
                (0x354d_414e, 5),
                (0x364d_414e, 6),
                (0x374d_414e, 7),
                (0x4d41_4e46, 0x46), // FNAM
                (0x4d41_4e52, 0x52), // RNAM
                (0x4d41_4e48, 0x48), // HNAM
                (0x4d41_4e59, 0x59), // YNAM
                (0x4d41_4e57, 0x57), // WNAM
                (0x4d41_4e58, 0x58), // XNAM
                (0x4d41_4e4c, 0x4c), // LNAM
                (0x4d41_4e47, 0x47), // GNAM
                (0x4d41_4e5a, 0x5a), // ZNAM: nothing
            ],
            0usize,
        )));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].1);
            Ret::default()
        });
        e.register(FILE_CHUNK_SIZE, |_, _| ret(8));
        let added = Rc::new(RefCell::new(Vec::new()));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x7000_00e0,
                0x7000_0134,
                FILE_CHUNK_DATA_SIZED,
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_63d0, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        let words: Vec<u32> = [0x6c, 0x70, 0x74, 0x7c, 0x80, 0x84, 0x78]
            .iter()
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(e.mem.u32(this + 0x64), 0x46);
        assert_eq!(e.mem.u32(this + 0x60), 0x52);
        assert_eq!(
            *added.borrow(),
            vec![
                (this + 0x88, 0x48),
                (this + 0x90, 0x59),
                (this + 0x98, 0x57),
                (this + 0xa0, 0x58),
                (this + 0xa8, 0x4c),
                (this + 0xb0, 0x47)
            ]
        );
    }

    #[test]
    fn controller_defaults_and_list_clear() {
        let mut e = Engine::new();
        e.map(0x0102_6000, 0x1000);
        e.map(0x0103_4000, 0x1000);
        e.set_global(0x0102_6994, 1.5f32);
        e.set_global(0x0103_40a4, 2.5f32);
        e.set_global(0x0103_4294, 3.5f32);
        quiet(&mut e, &[LIST_CLEAR]);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x64, 9);
        e.call_log = Some(vec![]);
        e.call(0x0059_74f0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x70), 1.5);
        assert_eq!(e.mem.f32(this + 0x74), 2.5);
        assert_eq!(e.mem.f32(this + 0x7c), 3.5);
        assert_eq!(e.mem.u32(this + 0x80), 0x40);
        assert_eq!(e.mem.u32(this + 0x84), 0x150);
        let cleared: Vec<u32> = calls(&e, LIST_CLEAR).iter().map(|c| c[0] - this).collect();
        assert_eq!(cleared, CONTROLLER_LISTS.to_vec());
        assert_eq!(
            (
                e.mem.u32(this + 0x60),
                e.mem.u32(this + 0x64),
                e.mem.u32(this + 0x68)
            ),
            (0, 1, 0)
        );
        e.call_log = Some(vec![]);
        e.call(0x0059_75b0, &args![this]);
        assert_eq!(calls(&e, LIST_CLEAR).len(), 6);
    }

    #[test]
    fn tracks_and_media_set_destructor() {
        let mut e = Engine::new();
        e.map(0x0103_4000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0103_3000, 0x1000);
        e.set_global(0x0103_4424, 0.5f32);
        e.set_global(0x0101_6410, 0.25f32);
        e.set_global(0x0103_395c, 0.75f32);
        quiet(
            &mut e,
            &[
                0x0040_37b0,
                STRING_SET,
                0x007d_9ec0,
                0x0059_1300,
                0x0048_3630,
            ],
        );
        let track = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0059_77d0, &args![track]).u32(), track);
        assert_eq!(calls(&e, 0x0040_37b0), vec![vec![track]]);
        assert_eq!(calls(&e, STRING_SET), vec![vec![track, 0, 0]]);
        assert_eq!((e.mem.f32(track + 8), e.mem.f32(track + 0xc)), (0.5, 0.25));
        e.call(0x0059_7850, &args![track]);
        assert_eq!((e.mem.f32(track + 8), e.mem.f32(track + 0xc)), (0.75, 0.25));

        let set = e.mem.alloc(0xc4);
        e.call_log = Some(vec![]);
        e.call(0x0059_78b0, &args![set]);
        assert_eq!(e.mem.u32(set), 0x0103_42ec);
        assert_eq!(e.mem.u32(set + 0x18), 0x0103_42d8);
        let tracks: Vec<u32> = calls(&e, 0x007d_9ec0).iter().map(|c| c[0] - set).collect();
        assert_eq!(tracks, vec![0x98, 0x88, 0x78, 0x68, 0x58, 0x48]);
        assert_eq!(
            calls(&e, 0x0059_1300),
            vec![vec![set + 0x38], vec![set + 0x18]]
        );
        assert_eq!(calls(&e, 0x0048_3630), vec![vec![set]]);
    }

    #[test]
    fn getters_read_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 0x6c, 0xffff_ff30);
        e.mem.set_u8(this + 0x37, 0xab);
        assert_eq!(e.call(0x0059_8000, &args![this]).u32(), 3);
        e.mem.set_u32(this + 0x6c, 0x10);
        assert_eq!(e.call(0x0059_8000, &args![this]).u32(), 1);
        assert_eq!(e.call(0x0059_8020, &args![this]).u8(), 0xab);
    }

    #[test]
    fn fn_00597990_dispatches_by_kind() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        let this = e.mem.alloc(0xc4);
        e.register(0x0059_8550, |_, _| ret(1));
        e.register(0x0059_8060, |_, _| ret(1));
        quiet(&mut e, &[0x0059_8890]);
        // Kind 0 with an object: 00598550(flag, time, 4).
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7990, &args![this, 500u32, 0x101u32, 0x8000u32])
            .bool());
        assert_eq!(calls(&e, 0x0059_8550), vec![vec![this, 1, 500, 4]]);
        // Kind 2: 00598060 with the three words.
        e.mem.set_u32(this + 0x44, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7990, &args![this, 500u32, 0u32, 0x8000u32])
            .bool());
        assert_eq!(calls(&e, 0x0059_8060), vec![vec![this, 500, 0, 0x8000]]);
        // Kind 1 runs 00597a60 (a set flag answers false at once).
        e.mem.set_u32(this + 0x44, 1);
        e.register(MEDIA_IS_PLAYING, |_, _| ret(0));
        assert!(!e
            .call(0x0059_7990, &args![this, 500u32, 1u32, 0x8000u32])
            .bool());
        // Kind 3 with an object: false.
        e.mem.set_u32(this + 0x44, 3);
        assert!(!e
            .call(0x0059_7990, &args![this, 500u32, 0u32, 0x8000u32])
            .bool());
        // Kind 3 without one: true before the set time, else 00598890.
        e.set_global(MEDIA_SET_TIME, 600u32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_7990, &args![this, 500u32, 0u32, 0u32]).bool());
        assert!(calls(&e, 0x0059_8890).is_empty());
        assert!(!e.call(0x0059_7990, &args![this, 600u32, 0u32, 0u32]).bool());
        assert_eq!(calls(&e, 0x0059_8890), vec![vec![this, 600]]);
        // Kind 0 without one: 00598550; kind 2 without one: false.
        e.mem.set_u32(this + 0x44, 0);
        assert!(e.call(0x0059_7990, &args![this, 7u32, 0u32, 0u32]).bool());
        e.mem.set_u32(this + 0x44, 2);
        assert!(!e.call(0x0059_7990, &args![this, 7u32, 0u32, 0u32]).bool());
    }

    /// A music type and an object for `00597a60`: thresholds 10/20/30 for
    /// the first set (+0x54/+0x64/+0x74) and 40/50/60 for the other, all
    /// flag bits set, fade 2.0 s, delay 1.5 s; the object's level at +0x3C,
    /// its byte at +0x37 and state word at +0x6C.
    fn music(
        level: f32,
        first_set: bool,
        playing: bool,
        length: u32,
        position: u32,
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x0101_3000, 0x1000);
        e.map(0x011d_d000, 0x1000);
        e.set_global(THOUSAND, 1000.0f64);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(THOUSAND_FLOAT, 1000.0f32);
        let this = e.mem.alloc(0xc4);
        for (offset, value) in [
            (0x54u32, 10.0f32),
            (0x64, 20.0),
            (0x74, 30.0),
            (0x84, 40.0),
            (0x94, 50.0),
            (0xa4, 60.0),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        for track in [0x48u32, 0x58, 0x68, 0x78, 0x88, 0x98] {
            e.mem.set_f32(this + track + 8, track as f32);
        }
        e.mem.set_u8(this + 0xa8, 0x3f);
        e.mem.set_f32(this + 0xb4, 2.0);
        e.mem.set_f32(this + 0xac, 1.5);
        let object = e.mem.alloc(0x70);
        e.mem.set_f32(object + 0x3c, level);
        e.mem.set_u8(object + 0x37, u8::from(first_set));
        e.register_double(MEDIA_IS_PLAYING, move |_, _| ret(u32::from(playing)));
        e.register_double(MEDIA_LENGTH, move |_, _| ret(length));
        e.register_double(MEDIA_POSITION, move |_, _| ret(position));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
        e.register(0x0055_9450, |_, a| ret(a[0] + 1));
        quiet(&mut e, &[MEDIA_FADE_OUT, MEDIA_START, MEDIA_UPDATE]);
        (e, this, object)
    }

    #[test]
    fn fn_00597a60_starts_the_track_above_the_level() {
        // Level 0.15 * 100 = 15: the lowest threshold above it is 20.
        let (mut e, this, object) = music(0.15, true, false, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(e.mem.u8(this + 0x34), 2);
        assert_eq!(e.mem.u8(this + 0x33), 1);
        assert_eq!(e.mem.u32(this + 0x2c), 5000);
        assert_eq!(e.global::<u32>(NEXT_TRACK_TIME), 6500);
        assert_eq!(
            calls(&e, MEDIA_START),
            vec![vec![2, this + 0x59, 2000, 1, 1, 88.0f32.to_bits(), 0]]
        );
        assert_eq!(calls(&e, MEDIA_UPDATE).len(), 1);
        assert!(calls(&e, MEDIA_FADE_OUT).is_empty());

        // The other set: 15 is below 40.
        let (mut e, this, object) = music(0.15, false, false, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(e.mem.u8(this + 0x34), 1);
        assert_eq!(calls(&e, MEDIA_START)[0][5], 120.0f32.to_bits());
    }

    #[test]
    fn fn_00597a60_without_a_track_fades_out() {
        // Level 70 is above every threshold: no track.
        let (mut e, this, object) = music(0.7, true, true, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT), vec![vec![2, 2000]]);
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u8(this + 0x34), 0);
    }

    #[test]
    fn fn_00597a60_keeps_a_playing_unchanged_track() {
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u32(this + 0x2c), 0);

        // A changed track before the next start time: kept too.
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_START).is_empty());
        assert_eq!(e.mem.u8(this + 0x34), 0);

        // A set flag: false at once.
        let (mut e, this, object) = music(0.15, true, true, 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 5000u32, 1u32, object])
            .bool());
        assert!(calls(&e, MEDIA_LENGTH).is_empty());
    }

    #[test]
    fn fn_00597a60_restarts_a_finished_track() {
        // Started at 1000, 3000 long, 2 s fade: over once 2000 < time.
        let (mut e, this, object) = music(0.15, true, true, 3000, 4000);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.set_global(NEXT_TRACK_TIME, 9000u32);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT), vec![vec![2, 2000]]);
        // Restarted: no offset into the playing media.
        let started = calls(&e, MEDIA_START);
        assert_eq!(started.len(), 1);
        assert_eq!(started[0][6], 0);
        assert_eq!(e.mem.u32(this + 0x2c), 2500);

        // Not over yet (2000 is not below 2000): nothing.
        let (mut e, this, object) = music(0.15, true, true, 3000, 0);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u8(this + 0x33, 1);
        e.mem.set_u8(this + 0x34, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 2000u32, 0u32, object])
            .bool());
        assert!(calls(&e, MEDIA_FADE_OUT).is_empty());

        // Over, with an object state: false after the fade.
        let (mut e, this, object) = music(0.15, true, true, 3000, 0);
        e.mem.set_u32(this + 0x2c, 1000);
        e.mem.set_u32(object + 0x6c, 0x10);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0059_7a60, &args![this, 2500u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_FADE_OUT).len(), 1);
        assert!(calls(&e, MEDIA_START).is_empty());
    }

    #[test]
    fn fn_00597a60_starts_with_the_offset_into_the_media() {
        // Not playing, so a new start; the media's position gives the offset.
        let (mut e, this, object) = music(0.15, true, false, 0, 4200);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0059_7a60, &args![this, 5000u32, 0u32, object])
            .bool());
        assert_eq!(calls(&e, MEDIA_START)[0][6], 800);
    }
}
