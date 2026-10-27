//! The platform's DirectSound 8 layer: `DirectSoundCreate8` (DSOUND import by ordinal 11, slot
//! `00fdf034`, behind the thunk `00ee81e0`), `IDirectSound8`, the primary buffer and its
//! `IDirectSound3DListener`, secondary buffers (`IDirectSoundBuffer`/`IDirectSoundBuffer8`), their
//! `IDirectSound3DBuffer` and the distortion effect object, plus `CoInitialize` and
//! `CoCreateInstance`. Every call the game makes is answered as DirectSound answers it.
//!
//! Sound reaches the player's device only when the host asks for it ([`start_output`]): each
//! secondary buffer is then also a [`super::mixer::Voice`] (its samples copied at every
//! `Unlock`, its volume, pan and looping kept with the buffer's), mixed newest first by the
//! measured exact path on the output thread ([`super::sound_out`]) at the device's own pace. The
//! cursors the game reads stay on the platform clock (above), so what is heard and what the game
//! reads are two clocks that agree only as far as the platform clock follows real time. Where
//! the exact path ends, the buffer stops the run at `Play` (and `SetFrequency` while playing)
//! with "not provided" until the maintainer decides: a buffer whose frequency is not the
//! device's rate (Windows resamples it; not yet pinned down), a 3D buffer (Windows' 3D gains not
//! yet pinned down), a buffer with the distortion effect (not analysed), and a device that does
//! not mix two channels.
//!
//! The answers that depend on the player's sound device and Windows come from the sound table,
//! captured on the player's machine by `scripts/dsound-capture` (git-ignored: `NV_DSOUND_TABLE`,
//! default `%LOCALAPPDATA%\nv-rs\dsound.tsv`): `DirectSoundCreate8`'s result, the cooperative
//! level's, `GetCaps`, the primary buffer, the listener's defaults, and the secondary buffers,
//! effects and settings the game asks for. Where an answer follows a rule (which buffer sizes
//! and formats are refused, the ranges of volume, pan, frequency and the distortion's
//! parameters), the layer answers by the rule and checks the rule against every captured line
//! when it loads the table: a table that disagrees stops the run. A secondary buffer of a kind
//! the capture didn't ask (flags, 3D algorithm, format) is not provided. The state of buffers,
//! the listener and the effects is this layer's own.
//!
//! Measured on this machine (2026-10-23, Windows 11, `dsound-capture`, 3,500 lines):
//! - `CreateSoundBuffer` for the game's four flag sets, 7 rates, mono and stereo, 8 and 16 bit
//!   and sizes around each limit (3,360 buffers): under 4 bytes `DSERR_INVALIDPARAM`; with
//!   `DSBCAPS_CTRLFX` and fewer than `floor(rate * 150 / 1000)` frames `DSERR_BUFFERTOOSMALL`;
//!   then 3D and not mono `DSERR_INVALIDPARAM`; 3D with `DS3DALG_HRTF_LIGHT` succeeds with
//!   `DS_NO_VIRTUALIZATION`; else `DS_OK`. A new buffer reports its flags plus
//!   `DSBCAPS_LOCSOFTWARE`, its size, its format (0x12 bytes), its rate, status 0, cursors 0;
//!   `IDirectSoundBuffer8` is there and `IDirectSound3DBuffer` only with `DSBCAPS_CTRL3D`.
//! - Volume -10000..0, pan -10000..10000, frequency 0 (the format's rate) or 100..200000; out of
//!   range `DSERR_INVALIDPARAM` (checked before the control: a 3D buffer's in-range pan is
//!   `DSERR_CONTROLUNAVAIL`). `Lock(0, size)` and `Lock(.., DSBLOCK_ENTIREBUFFER)` give one
//!   region of the whole buffer.
//! - `SetFX` with the distortion: `DS_OK`, result `DSFXR_LOCSOFTWARE`; the I3DL2 reverb:
//!   `REGDB_E_CLASSNOTREG`, result `DSFXR_UNKNOWN`, and its `GetObjectInPath`
//!   `DSERR_OBJECTNOTFOUND`; without `DSBCAPS_CTRLFX` both `DSERR_CONTROLUNAVAIL`. The
//!   distortion's gain must be -60..0.
//! - The cursors (seven buffers sampled at every change for 600 ms after `Play`; the first
//!   `Play` of the run took longer than that): DirectSound moves them in steps of the device's
//!   10 ms period. After `n` periods the write cursor is `floor(n * frequency / 100) + 1`
//!   frames on and the play cursor `floor(3 * rate / 100)` frames behind it (three periods at
//!   the format's rate, whatever the frequency). With the first period placed where the
//!   capture saw it, this gives every sample of the 44100 and 48000 Hz stereo and the 11025 Hz
//!   8-bit buffers exactly; the 16-bit mono buffers and the ones with another frequency are
//!   one frame off at some samples (Windows' resampler). The first period begins 1.7 to 7.7 ms
//!   after `Play`, varying from run to run; here the periods are the platform clock's 10 ms
//!   steps (`Engine::clock_now_us`), so runs repeat. A non-looping buffer stops when the write
//!   cursor reaches its end (both cursors then read 0, status 0); `Stop` leaves both cursors at
//!   the write cursor (at the end, for a buffer that stopped by itself).
//!
//! As in the DirectInput layer (`super::dinput`), the objects live in engine memory with a vtable
//! pointer at `+0`, each provided slot a platform function (code addresses from [`CODE_BASE`]);
//! a slot not provided is left unregistered, so a call to it stops the run as an open function
//! and is recorded with its address.
//!
//! What the game calls: `BSXenonAudio::Init` (`00aea230`), the sounds of `BSWin32GameSound`
//! (`00aeb270`, `00aec530`, `00af0630`, `00af1430` and the playing functions around them) and the
//! listener (`00aea8b0`); `CoCreateInstance` only from the DirectShow music thread (`00830b30`)
//! and the filter registration (`009fba40`, `009fbab0`), which this layer does not provide.

use super::mixer::Voice;
use crate::{Engine, Ptr};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// Where the platform's DirectSound functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e00_0400;
pub const SOUND_CODE: u32 = CODE_BASE;
pub const BUFFER_CODE: u32 = CODE_BASE + 0x100;
pub const LISTENER_CODE: u32 = CODE_BASE + 0x200;
pub const BUFFER3D_CODE: u32 = CODE_BASE + 0x300;
pub const EFFECT_CODE: u32 = CODE_BASE + 0x380;

/// `IDirectSound8`'s slots (dsound.h order).
pub const SOUND_SLOTS: [&str; 12] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "CreateSoundBuffer",
    "GetCaps",
    "DuplicateSoundBuffer",
    "SetCooperativeLevel",
    "Compact",
    "GetSpeakerConfig",
    "SetSpeakerConfig",
    "Initialize",
    "VerifyCertification",
];
/// `IDirectSoundBuffer8`'s slots (the primary buffer is an `IDirectSoundBuffer`, the first 21).
pub const BUFFER_SLOTS: [&str; 24] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetCaps",
    "GetCurrentPosition",
    "GetFormat",
    "GetVolume",
    "GetPan",
    "GetFrequency",
    "GetStatus",
    "Initialize",
    "Lock",
    "Play",
    "SetCurrentPosition",
    "SetFormat",
    "SetVolume",
    "SetPan",
    "SetFrequency",
    "Stop",
    "Unlock",
    "Restore",
    "SetFX",
    "AcquireResources",
    "GetObjectInPath",
];
/// `IDirectSound3DListener`'s slots.
pub const LISTENER_SLOTS: [&str; 18] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetAllParameters",
    "GetDistanceFactor",
    "GetDopplerFactor",
    "GetOrientation",
    "GetPosition",
    "GetRolloffFactor",
    "GetVelocity",
    "SetAllParameters",
    "SetDistanceFactor",
    "SetDopplerFactor",
    "SetOrientation",
    "SetPosition",
    "SetRolloffFactor",
    "SetVelocity",
    "CommitDeferredSettings",
];
/// `IDirectSound3DBuffer`'s slots.
pub const BUFFER3D_SLOTS: [&str; 21] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetAllParameters",
    "GetConeAngles",
    "GetConeOrientation",
    "GetConeOutsideVolume",
    "GetMaxDistance",
    "GetMinDistance",
    "GetMode",
    "GetPosition",
    "GetVelocity",
    "SetAllParameters",
    "SetConeAngles",
    "SetConeOrientation",
    "SetConeOutsideVolume",
    "SetMaxDistance",
    "SetMinDistance",
    "SetMode",
    "SetPosition",
    "SetVelocity",
];
/// `IDirectSoundFXDistortion8`'s slots.
pub const DISTORTION_SLOTS: [&str; 5] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "SetAllParameters",
    "GetAllParameters",
];

/// Result codes (dsound.h / winerror.h).
pub const DS_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const DSERR_INVALIDPARAM: u32 = 0x8007_0057;
pub const DSERR_UNSUPPORTED: u32 = 0x8000_4001;
pub const DSERR_ALREADYINITIALIZED: u32 = 0x8878_0082;
pub const DSERR_CONTROLUNAVAIL: u32 = 0x8878_001e;
pub const DSERR_BUFFERTOOSMALL: u32 = 0x8878_00b4;
pub const DS_NO_VIRTUALIZATION: u32 = 0x0878_000a;

/// Interface and class ids.
const IID_LISTENER: [u8; 16] = [
    0x84, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_BUFFER3D: [u8; 16] = [
    0x86, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_BUFFER8: [u8; 16] = [
    0x49, 0xa4, 0x25, 0x68, 0x24, 0x75, 0x82, 0x4d, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e,
];
/// `GUID_DSFX_STANDARD_DISTORTION` and `IID_IDirectSoundFXDistortion8` (the values at the exe's
/// `01100ff4` and `01100fc4`), and the I3DL2 reverb's (`01101004`, `01100fd4`).
const GUID_DISTORTION: [u8; 16] = [
    0x90, 0x4c, 0x11, 0xef, 0x1d, 0xcd, 0x4e, 0x48, 0x96, 0xe5, 0x09, 0xcf, 0xaf, 0x91, 0x2a, 0x21,
];
const IID_DISTORTION8: [u8; 16] = [
    0x26, 0x43, 0xcf, 0x8e, 0x5f, 0x45, 0x8b, 0x4d, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b,
];
const GUID_I3DL2: [u8; 16] = [
    0x71, 0x5e, 0x98, 0xef, 0xc7, 0xd5, 0xd4, 0x42, 0xba, 0x4d, 0x2d, 0x07, 0x3e, 0x2e, 0x96, 0xf4,
];
const IID_I3DL2_8: [u8; 16] = [
    0x6a, 0x6a, 0x16, 0x4b, 0x66, 0x0d, 0xf3, 0x43, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4,
];
/// `DS3DALG_HRTF_LIGHT` as the game stores it (`00aeb270`).
const ALG_HRTF_LIGHT: [u32; 4] = [0xc241_3342, 0x11d2_1c1b, 0xc000_f594, 0xca8a_c24f];

/// `DSSCL_*` cooperative levels.
const DSSCL_NORMAL: u32 = 1;
const DSSCL_WRITEPRIMARY: u32 = 4;
/// `DSBCAPS_*` flags.
const DSBCAPS_PRIMARYBUFFER: u32 = 0x1;
const DSBCAPS_LOCSOFTWARE: u32 = 0x8;
const DSBCAPS_CTRL3D: u32 = 0x10;
const DSBCAPS_CTRLFREQUENCY: u32 = 0x20;
const DSBCAPS_CTRLPAN: u32 = 0x40;
const DSBCAPS_CTRLVOLUME: u32 = 0x80;
const DSBCAPS_CTRLFX: u32 = 0x200;
/// `DSBPLAY_LOOPING`, `DSBLOCK_ENTIREBUFFER`, `DSBSTATUS_PLAYING`, `DSBSTATUS_LOOPING`.
const DSBPLAY_LOOPING: u32 = 1;
const DSBLOCK_ENTIREBUFFER: u32 = 2;
const DSBSTATUS_PLAYING: u32 = 1;
const DSBSTATUS_LOOPING: u32 = 4;
/// `DSCAPS` (0x60 bytes), `DSBCAPS` (0x14), `DSBUFFERDESC` (0x24, DirectX 7 and later),
/// `WAVEFORMATEX` (0x12), `DSEFFECTDESC` (0x20), `DS3DLISTENER` (0x40).
const DSCAPS_SIZE: u32 = 0x60;
const DSBCAPS_SIZE: u32 = 0x14;
const BUFFER_DESC_SIZE: u32 = 0x24;
const FORMAT_SIZE: u32 = 0x12;
const EFFECT_DESC_SIZE: u32 = 0x20;
const LISTENER_SIZE: u32 = 0x40;
/// The device's period (measured: the cursors move every 10 ms) and the smallest buffer.
const PERIOD_US: u64 = 10_000;
const MIN_BUFFER_BYTES: u32 = 4;

/// How to make the sound table, for the stop when it is missing.
pub const CAPTURE_HINT: &str = "run `cargo run --release --target i686-pc-windows-msvc \
     --manifest-path scripts/dsound-capture/Cargo.toml > <file>` on this machine and point \
     NV_DSOUND_TABLE at the file (default: %LOCALAPPDATA%/nv-rs/dsound.tsv)";

/// What a secondary buffer request is: flags, whether the HRTF algorithm is asked, and the
/// format's tag, channels, rate and bits.
type Kind = (u32, bool, u16, u16, u32, u16);

/// The captured sound table (`scripts/dsound-capture`), checked against the layer's rules.
#[derive(Debug, Clone, Default)]
pub struct Table {
    create: u32,
    cooperative: BTreeMap<u32, u32>,
    caps: (u32, Vec<u8>),
    primary: u32,
    listener: (u32, Vec<u8>),
    /// The kinds of secondary buffer the capture made.
    kinds: BTreeSet<Kind>,
    /// `SetFX` / `GetObjectInPath` by (effect, buffer has `DSBCAPS_CTRLFX`): `SetFX`'s result,
    /// its result code, `GetObjectInPath`'s result.
    effects: BTreeMap<(String, bool), (u32, u32, u32)>,
}

fn hex(text: &str) -> u32 {
    u32::from_str_radix(text, 16)
        .unwrap_or_else(|_| panic!("a hex word in the sound table: {text}"))
}

fn words(text: &str) -> Vec<u32> {
    text.split(' ').filter(|w| !w.is_empty()).map(hex).collect()
}

fn bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex bytes"))
        .collect()
}

fn disagree(line: &str, rule: impl std::fmt::Display) -> ! {
    panic!("the sound table disagrees with the layer ({rule}): {line}; re-capture it or report it")
}

/// The answer the layer gives a secondary `CreateSoundBuffer` (see the module documentation).
fn create_result(kind: Kind, bytes: u32) -> u32 {
    let (flags, hrtf, _, channels, rate, bits) = kind;
    let align = u32::from(channels) * u32::from(bits) / 8;
    if bytes < MIN_BUFFER_BYTES {
        DSERR_INVALIDPARAM
    } else if flags & DSBCAPS_CTRLFX != 0 && u64::from(bytes / align) < u64::from(rate) * 150 / 1000
    {
        DSERR_BUFFERTOOSMALL
    } else if flags & DSBCAPS_CTRL3D != 0 && channels != 1 {
        DSERR_INVALIDPARAM
    } else if flags & DSBCAPS_CTRL3D != 0 && hrtf {
        DS_NO_VIRTUALIZATION
    } else {
        DS_OK
    }
}

fn volume_ok(v: i32) -> bool {
    (-10000..=0).contains(&v)
}

fn pan_ok(v: i32) -> bool {
    (-10000..=10000).contains(&v)
}

fn frequency_ok(v: u32) -> bool {
    v == 0 || (100..=200_000).contains(&v)
}

/// `DSFXDistortion`'s ranges (dsound.h: gain -60..0, edge 0..100, the frequencies 100..8000).
fn distortion_ok(p: [f32; 5]) -> bool {
    (-60.0..=0.0).contains(&p[0])
        && (0.0..=100.0).contains(&p[1])
        && p[2..].iter().all(|f| (100.0..=8000.0).contains(f))
}

impl Table {
    /// Parses and checks a capture.
    pub fn parse(text: &str) -> Table {
        let mut t = Table::default();
        let mut ended = false;
        for line in text.lines().filter(|l| !l.is_empty()) {
            let f: Vec<&str> = line.split('\t').collect();
            let field = |i: usize| *f.get(i).unwrap_or(&"");
            match f[0] {
                "DsCreate" => t.create = hex(field(2)),
                "DsCooperativeLevel" => {
                    t.cooperative.insert(hex(field(1)), hex(field(2)));
                }
                "DsCaps" => t.caps = (hex(field(2)), bytes(field(3))),
                "DsPrimary" => t.primary = hex(field(2)),
                "DsListener" => {
                    let r = words(field(2));
                    t.listener = (r[0], bytes(field(3)));
                }
                "DsBuffer" => t.check_buffer(line, &f),
                "DsFx" => t.check_effect(line, &f),
                "DsSet" => check_setting(line, &f),
                "DsCursor" => {}
                "DsEnd" => ended = true,
                other => panic!("the sound table has a line this layer does not know: {other}"),
            }
        }
        assert!(
            ended,
            "the sound table is cut short (no DsEnd): {CAPTURE_HINT}"
        );
        t
    }

    fn check_buffer(&mut self, line: &str, f: &[&str]) {
        let w = words(f[1]);
        let kind: Kind = (w[0], w[1] != 0, w[2] as u16, w[3] as u16, w[4], w[5] as u16);
        let size = w[6];
        let result = hex(f[2]);
        let want = create_result(kind, size);
        if result != want {
            disagree(line, format!("CreateSoundBuffer answers {want:x}"));
        }
        self.kinds.insert(kind);
        if (result as i32) < 0 {
            return;
        }
        // The new buffer as the layer makes it.
        let caps = bytes(f[3]);
        let read = |b: &[u8], o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let (flags, _, tag, channels, rate, bits) = kind;
        let align = u32::from(channels) * u32::from(bits) / 8;
        let format = format_bytes(tag, channels, rate, bits);
        let state = words(f[5]);
        let queries = words(f[6]);
        let three_d = if flags & DSBCAPS_CTRL3D != 0 {
            DS_OK
        } else {
            E_NOINTERFACE
        };
        let ok = read(&caps, 0) == DSBCAPS_SIZE
            && read(&caps, 4) == flags | DSBCAPS_LOCSOFTWARE
            && read(&caps, 8) == size
            && read(&caps, 12) == 0
            && read(&caps, 16) == 0
            && f[4] == format!("{} 12", hex_string(&format))
            && state == [rate, 0, 0, 0]
            && queries == [DS_OK, three_d];
        if !ok || align == 0 {
            disagree(line, "a new buffer's caps, format, state and interfaces");
        }
    }

    fn check_effect(&mut self, line: &str, f: &[&str]) {
        let w: Vec<&str> = f[1].split(' ').collect();
        if f[2].starts_with("create") {
            return;
        }
        let flags = hex(w[1]);
        let key = (w[0].to_string(), flags & DSBCAPS_CTRLFX != 0);
        let value = (hex(f[2]), hex(f[3]), hex(f[4]));
        if let Some(old) = self.effects.insert(key, value) {
            if old != value {
                disagree(line, "the same effect answers the same for every format");
            }
        }
    }

    /// The captured answer to `SetFX`/`GetObjectInPath` for this effect.
    fn effect(&self, name: &str, fx: bool) -> (u32, u32, u32) {
        *self
            .effects
            .get(&(name.to_string(), fx))
            .unwrap_or_else(|| {
                panic!("SetFX({name}) was not captured in the sound table: not provided")
            })
    }
}

fn hex_string(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn format_bytes(tag: u16, channels: u16, rate: u32, bits: u16) -> Vec<u8> {
    let align = channels * bits / 8;
    let mut b = Vec::new();
    b.extend_from_slice(&tag.to_le_bytes());
    b.extend_from_slice(&channels.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&(rate * u32::from(align)).to_le_bytes());
    b.extend_from_slice(&align.to_le_bytes());
    b.extend_from_slice(&bits.to_le_bytes());
    b.extend_from_slice(&0u16.to_le_bytes());
    b
}

/// Checks a `DsSet` line against the layer's rules.
fn check_setting(line: &str, f: &[&str]) {
    let w: Vec<&str> = f[1].split(' ').collect();
    // The 3D buffer's defaults line has two results (the query's and `GetAllParameters`').
    let result = hex(f[2].split(' ').next().unwrap_or_default());
    let expect = |want: u32| {
        if result != want {
            disagree(line, format!("answers {want:x}"));
        }
    };
    let ok = |good: bool| if good { DS_OK } else { DSERR_INVALIDPARAM };
    match w.as_slice() {
        ["distortion", gain] => {
            let g = f32::from_bits(hex(gain));
            expect(ok(distortion_ok([g, 0.0, 5000.0, 1135.0, 5000.0])));
        }
        [_, "volume", v] => expect(ok(volume_ok(hex(v) as i32))),
        [flags, "pan", v] => {
            let has_pan = hex(flags) & DSBCAPS_CTRLPAN != 0;
            let v = hex(v) as i32;
            expect(match (pan_ok(v), has_pan) {
                (false, _) => DSERR_INVALIDPARAM,
                (true, false) => DSERR_CONTROLUNAVAIL,
                (true, true) => DS_OK,
            });
        }
        [_, "frequency", v] => expect(ok(frequency_ok(hex(v)))),
        ["3dbuffer", "mindistance" | "maxdistance", v] => {
            expect(ok(f32::from_bits(hex(v)) > 0.0));
        }
        [_, "lock", ..]
        | [_, "unlock"]
        | ["3dbuffer", ..]
        | ["listener", ..]
        | ["i3dl2preset", _] => {}
        _ => panic!("the sound table has a setting this layer does not know: {line}"),
    }
}

/// Gives the layer the player's sound table (`scripts/dsound-capture`'s output).
pub fn load_table(e: &mut Engine, text: &str) {
    e.kernel.sound.table = Some(Table::parse(text));
}

/// The loaded table; loaded from `NV_DSOUND_TABLE` (default `%LOCALAPPDATA%\nv-rs\dsound.tsv`)
/// the first time it is needed. Without one the run stops and says how to capture it.
fn table(e: &mut Engine) -> &Table {
    if e.kernel.sound.table.is_none() {
        let path = std::env::var_os("NV_DSOUND_TABLE")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("LOCALAPPDATA")
                    .map(|d| std::path::PathBuf::from(d).join("nv-rs").join("dsound.tsv"))
            });
        let text = path
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_else(|| panic!("the sound table ({path:?}) is not there: {CAPTURE_HINT}"));
        load_table(e, &text);
    }
    e.kernel.sound.table.as_ref().expect("loaded")
}

/// The DirectSound 8 `DS3DLISTENER` state.
#[derive(Debug, Clone, Copy)]
struct Listener {
    position: [f32; 3],
    velocity: [f32; 3],
    front: [f32; 3],
    top: [f32; 3],
    distance_factor: f32,
    rolloff: f32,
    doppler: f32,
}

/// Whether a buffer plays.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Play {
    /// Stopped with both cursors at this byte.
    Stopped(u32),
    /// Playing since the clock read `start` (microseconds), from frame `base`.
    Playing {
        looping: bool,
        start: u64,
        base: u64,
    },
    /// A non-looping buffer that played to its end.
    Ended,
}

#[derive(Debug, Clone)]
struct Buffer {
    references: u32,
    flags: u32,
    /// The listener this primary buffer gave out (0: none yet).
    listener: u32,
    /// Secondary buffers: the size, the format, the sample memory (engine memory), the 3D
    /// interface (0: not asked yet).
    bytes: u32,
    format: Vec<u8>,
    data: u32,
    three_d: u32,
    volume: i32,
    pan: i32,
    frequency: u32,
    play: Play,
    locks: u32,
    effect: Option<&'static str>,
    min_distance: f32,
    max_distance: f32,
    position: [f32; 3],
    velocity: [f32; 3],
}

impl Buffer {
    fn align(&self) -> u32 {
        u32::from(u16::from_le_bytes([self.format[12], self.format[13]]))
    }

    fn rate(&self) -> u32 {
        u32::from_le_bytes([
            self.format[4],
            self.format[5],
            self.format[6],
            self.format[7],
        ])
    }

    fn frequency(&self) -> u32 {
        if self.frequency == 0 {
            self.rate()
        } else {
            self.frequency
        }
    }

    fn frames(&self) -> u64 {
        u64::from(self.bytes / self.align())
    }
}

/// The DirectSound objects (`Kernel::sound`).
#[derive(Debug, Default)]
pub struct Sound {
    sound_vtable: u32,
    buffer_vtable: u32,
    listener_vtable: u32,
    buffer3d_vtable: u32,
    distortion_vtable: u32,
    devices: BTreeMap<u32, (u32, u32)>,
    buffers: BTreeMap<u32, Buffer>,
    listeners: BTreeMap<u32, (u32, Listener)>,
    /// The 3D interfaces: the buffer each belongs to (they share its reference count).
    buffers3d: BTreeMap<u32, u32>,
    /// The distortion objects: reference count and `DSFXDistortion`.
    distortions: BTreeMap<u32, (u32, [f32; 5])>,
    /// `CoInitialize` calls on the main thread (the first answers `S_OK`, later ones `S_FALSE`).
    com_initialized: u32,
    /// The sound table ([`load_table`]).
    table: Option<Table>,
    /// The DirectShow filter graphs ([`super::dshow`]).
    pub(crate) show: super::dshow::Show,
    /// The open Ogg Vorbis files ([`super::vorbisfile`]).
    pub(crate) vorbis: super::vorbisfile::Files,
    /// The voices heard on the device, once the host started the output ([`start_output`]).
    mix: Option<Mix>,
}

/// The output thread and the voices it plays (the buffers' engine addresses, newest `Play`
/// first).
struct Mix {
    voices: Arc<Mutex<Voices>>,
    rate: u32,
    /// The thread (none in the tests, which render by hand).
    _output: Option<super::sound_out::Output>,
}

impl std::fmt::Debug for Mix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mix {{ rate: {} }}", self.rate)
    }
}

#[derive(Default)]
struct Voices {
    /// Every secondary buffer's voice, by address.
    all: BTreeMap<u32, Voice>,
    /// The playing ones, newest `Play` first.
    playing: Vec<u32>,
}

impl super::sound_out::Source for Voices {
    fn render(&mut self, _rate: u32, channels: u16, out: &mut [f32]) {
        assert!(
            channels == 2,
            "a sound device mixing {channels} channels is not provided"
        );
        let Voices { all, playing } = self;
        let mut voices: Vec<&mut Voice> = Vec::with_capacity(playing.len());
        // `playing` holds distinct addresses; take each voice once, in its order.
        let mut by: BTreeMap<u32, &mut Voice> = all.iter_mut().map(|(k, v)| (*k, v)).collect();
        for at in playing.iter() {
            if let Some(v) = by.remove(at) {
                voices.push(v);
            }
        }
        super::mixer::render(&mut voices, out);
        playing.retain(|at| all.get(at).is_some_and(|v| v.position.is_some()));
    }
}

/// Starts playing the game's sound on the player's default device; false when there is no
/// device (or not on Windows). Buffers made before keep playing silently until their next
/// `Play`.
pub fn start_output(e: &mut Engine) -> bool {
    if e.kernel.sound.mix.is_some() {
        return true;
    }
    let voices = Arc::new(Mutex::new(Voices::default()));
    {
        let mut v = voices.lock().expect("the voices");
        for (&at, b) in &e.kernel.sound.buffers {
            if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
                v.all.insert(at, voice_of(e, b));
            }
        }
    }
    let source: Arc<Mutex<dyn super::sound_out::Source>> = voices.clone();
    let Some(output) = super::sound_out::start(source) else {
        return false;
    };
    e.kernel.sound.mix = Some(Mix {
        voices,
        rate: output.rate,
        _output: Some(output),
    });
    true
}

/// A buffer's voice: its samples as they are in engine memory now, its settings, not playing.
fn voice_of(e: &Engine, b: &Buffer) -> Voice {
    Voice {
        data: e.mem.bytes(b.data, b.bytes),
        channels: u16::from_le_bytes([b.format[2], b.format[3]]),
        bits: u16::from_le_bytes([b.format[14], b.format[15]]),
        volume: b.volume,
        pan: b.pan,
        looping: false,
        position: None,
    }
}

/// Runs `f` on the voice of the buffer at `at`, when the output is on.
fn with_voice(e: &mut Engine, at: u32, f: impl FnOnce(&mut Voices, u32)) {
    if let Some(m) = &e.kernel.sound.mix {
        let mut v = m.voices.lock().expect("the voices");
        f(&mut v, at);
    }
}

/// For a platform layer that plays sound of its own through DirectSound (Bink's DirectSound
/// sound system, `super::bink`): starts `pcm` (`channels`, `bits`, `rate`) as a voice under
/// `key` (an engine address of the caller's, distinct from every buffer's), heard when the
/// output is on, with volume, pan and frequency at their defaults. A rate other than the
/// device's is not provided (the mixer's exact path, `super::mixer`).
pub fn play_stream(e: &mut Engine, key: u32, pcm: Vec<u8>, channels: u16, bits: u16, rate: u32) {
    let Some(m) = &e.kernel.sound.mix else {
        return;
    };
    assert!(
        rate == m.rate,
        "a stream heard at {rate} Hz on a {} Hz device is not provided (Windows' resampler is \
         not pinned down; the maintainer decides)",
        m.rate
    );
    let mut v = m.voices.lock().expect("the voices");
    v.all.insert(
        key,
        Voice {
            data: pcm,
            channels,
            bits,
            volume: 0,
            pan: 0,
            looping: false,
            position: Some(0),
        },
    );
    v.playing.retain(|&p| p != key);
    v.playing.insert(0, key);
}

/// Holds (`paused`) or goes on with a stream from [`play_stream`] where it is.
pub fn pause_stream(e: &mut Engine, key: u32, paused: bool) {
    with_voice(e, key, |m, at| {
        m.playing.retain(|&p| p != at);
        if !paused && m.all.get(&at).is_some_and(|v| v.position.is_some()) {
            m.playing.insert(0, at);
        }
    });
}

/// Ends a stream from [`play_stream`].
pub fn stop_stream(e: &mut Engine, key: u32) {
    with_voice(e, key, |m, at| {
        m.all.remove(&at);
        m.playing.retain(|&p| p != at);
    });
}

type Slots<const N: usize> = [(u32, crate::AbiFn); N];

fn register_code(e: &mut Engine) {
    let sound: Slots<6> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, sound_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, sound_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, create_sound_buffer(Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (4, crate::entry!(0, get_caps(Ptr, Ptr) -> u32).1),
        (
            6,
            crate::entry!(0, set_cooperative_level(Ptr, u32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in sound {
        e.register(SOUND_CODE + 4 * slot, f);
    }
    let buffer: Slots<20> = [
        (
            0,
            crate::entry!(0, buffer_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, buffer_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer_release(Ptr) -> u32).1),
        (3, crate::entry!(0, buffer_get_caps(Ptr, Ptr) -> u32).1),
        (
            4,
            crate::entry!(0, get_current_position(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (5, crate::entry!(0, get_format(Ptr, Ptr, u32, Ptr) -> u32).1),
        (6, crate::entry!(0, get_volume(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, get_pan(Ptr, Ptr) -> u32).1),
        (8, crate::entry!(0, get_frequency(Ptr, Ptr) -> u32).1),
        (9, crate::entry!(0, get_status(Ptr, Ptr) -> u32).1),
        (
            11,
            crate::entry!(0, lock(Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (12, crate::entry!(0, play(Ptr, u32, u32, u32) -> u32).1),
        (
            13,
            crate::entry!(0, set_current_position(Ptr, u32) -> u32).1,
        ),
        (15, crate::entry!(0, set_volume(Ptr, i32) -> u32).1),
        (16, crate::entry!(0, set_pan(Ptr, i32) -> u32).1),
        (17, crate::entry!(0, set_frequency(Ptr, u32) -> u32).1),
        (18, crate::entry!(0, stop(Ptr) -> u32).1),
        (
            19,
            crate::entry!(0, unlock(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (21, crate::entry!(0, set_fx(Ptr, u32, Ptr, Ptr) -> u32).1),
        (
            23,
            crate::entry!(0, get_object_in_path(Ptr, Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
    ];
    for (slot, f) in buffer {
        e.register(BUFFER_CODE + 4 * slot, f);
    }
    let listener: Slots<13> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, listener_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, listener_release(Ptr) -> u32).1),
        (3, crate::entry!(0, listener_get_all_parameters(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, listener_get_position(Ptr, Ptr) -> u32).1),
        (10, crate::entry!(0, listener_set_all_parameters(Ptr, Ptr, u32) -> u32).1),
        (11, crate::entry!(0, listener_set_distance_factor(Ptr, f32, u32) -> u32).1),
        (12, crate::entry!(0, listener_set_doppler_factor(Ptr, f32, u32) -> u32).1),
        (
            13,
            crate::entry!(0, listener_set_orientation(Ptr, f32, f32, f32, f32, f32, f32, u32) -> u32).1,
        ),
        (14, crate::entry!(0, listener_set_position(Ptr, f32, f32, f32, u32) -> u32).1),
        (15, crate::entry!(0, listener_set_rolloff_factor(Ptr, f32, u32) -> u32).1),
        (16, crate::entry!(0, listener_set_velocity(Ptr, f32, f32, f32, u32) -> u32).1),
        (17, crate::entry!(0, listener_commit(Ptr) -> u32).1),
    ];
    for (slot, f) in listener {
        e.register(LISTENER_CODE + 4 * slot, f);
    }
    let three_d: Slots<7> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, buffer3d_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer3d_release(Ptr) -> u32).1),
        (
            16,
            crate::entry!(0, set_max_distance(Ptr, f32, u32) -> u32).1,
        ),
        (
            17,
            crate::entry!(0, set_min_distance(Ptr, f32, u32) -> u32).1,
        ),
        (
            19,
            crate::entry!(0, buffer3d_set_position(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
        (
            20,
            crate::entry!(0, buffer3d_set_velocity(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in three_d {
        e.register(BUFFER3D_CODE + 4 * slot, f);
    }
    let distortion: Slots<4> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, distortion_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, distortion_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, distortion_set_all_parameters(Ptr, Ptr) -> u32).1,
        ),
    ];
    for (slot, f) in distortion {
        e.register(EFFECT_CODE + 4 * slot, f);
    }
}

fn vtable(e: &mut Engine, code: u32, slots: u32) -> u32 {
    let table = e.mem.alloc(4 * slots);
    for i in 0..slots {
        e.mem.set_u32(table + 4 * i, code + 4 * i);
    }
    table
}

fn object(e: &mut Engine, vtable: u32) -> u32 {
    let object = e.mem.alloc(8);
    e.mem.set_u32(object, vtable);
    object
}

// Platform: replaces OLE32.DLL CoInitialize (import)
/// `CoInitialize(NULL)`: COM on the calling thread; `S_OK` the first time, `S_FALSE` after.
pub fn co_initialize(e: &mut Engine, reserved: u32) -> u32 {
    if reserved != 0 {
        return DSERR_INVALIDPARAM;
    }
    e.kernel.sound.com_initialized += 1;
    if e.kernel.sound.com_initialized == 1 {
        DS_OK
    } else {
        S_FALSE
    }
}

// Platform: replaces OLE32.DLL CoCreateInstance (import)
/// `CoCreateInstance(clsid, outer, context, iid, out)`: the game asks only for DirectShow's
/// filter graph (the music thread `00830b30`), which [`super::dshow`] provides, and the filter
/// mapper (the filter registration `009fba40`/`009fbab0`), which stops the run as any other class.
pub fn co_create_instance(
    e: &mut Engine,
    clsid: Ptr,
    outer: u32,
    context: u32,
    iid: Ptr,
    out: Ptr,
) -> u32 {
    super::dshow::co_create_instance(e, clsid, outer, context, iid, out)
}

// Platform: replaces DSOUND.DLL DirectSoundCreate8 (import by ordinal 11)
/// `DirectSoundCreate8(device, out, outer)`: the default device (`device` NULL) as a new
/// `IDirectSound8` through `out` with the captured result; a failure the capture recorded is
/// answered as captured with `out` NULL. A chosen device or an outer object is not provided.
pub fn direct_sound_create8(e: &mut Engine, device: u32, out: Ptr, outer: u32) -> u32 {
    if device != 0 || outer != 0 {
        panic!("DirectSoundCreate8 with a device GUID or an outer object is not provided");
    }
    let result = table(e).create;
    if result != DS_OK {
        e.mem.set_u32(out.addr(), 0);
        return result;
    }
    if e.kernel.sound.sound_vtable == 0 {
        register_code(e);
        e.kernel.sound.sound_vtable = vtable(e, SOUND_CODE, SOUND_SLOTS.len() as u32);
        e.kernel.sound.buffer_vtable = vtable(e, BUFFER_CODE, BUFFER_SLOTS.len() as u32);
        e.kernel.sound.listener_vtable = vtable(e, LISTENER_CODE, LISTENER_SLOTS.len() as u32);
        e.kernel.sound.buffer3d_vtable = vtable(e, BUFFER3D_CODE, BUFFER3D_SLOTS.len() as u32);
        e.kernel.sound.distortion_vtable = vtable(e, EFFECT_CODE, DISTORTION_SLOTS.len() as u32);
    }
    let table = e.kernel.sound.sound_vtable;
    let device = object(e, table);
    e.kernel.sound.devices.insert(device, (1, 0));
    e.mem.set_u32(out.addr(), device);
    DS_OK
}

fn no_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn device(e: &mut Engine, this: Ptr) -> &mut (u32, u32) {
    e.kernel
        .sound
        .devices
        .get_mut(&this.addr())
        .expect("an IDirectSound8 of this layer")
}

fn sound_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.0 += 1;
    d.0
}

fn sound_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.0 -= 1;
    let left = d.0;
    if left == 0 {
        e.kernel.sound.devices.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `SetCooperativeLevel(window, level)`: the captured answer for that level (the game asks
/// `DSSCL_PRIORITY`); a level outside `DSSCL_NORMAL..DSSCL_WRITEPRIMARY` is `DSERR_INVALIDPARAM`;
/// a level the capture didn't ask is not provided.
fn set_cooperative_level(e: &mut Engine, this: Ptr, _window: u32, level: u32) -> u32 {
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&level) {
        return DSERR_INVALIDPARAM;
    }
    let result = *table(e)
        .cooperative
        .get(&level)
        .unwrap_or_else(|| panic!("SetCooperativeLevel({level}) was not captured: not provided"));
    if result == DS_OK {
        device(e, this).1 = level;
    }
    result
}

/// `GetCaps(caps)`: `caps->dwSize` must be 0x60; the captured `DSCAPS`.
fn get_caps(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    let _ = device(e, this);
    let at = caps.addr();
    if e.mem.u32(at) != DSCAPS_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let (result, bytes) = table(e).caps.clone();
    if result == DS_OK {
        e.mem.write(at, &bytes);
    }
    result
}

fn new_buffer(flags: u32) -> Buffer {
    Buffer {
        references: 1,
        flags,
        listener: 0,
        bytes: 0,
        format: Vec::new(),
        data: 0,
        three_d: 0,
        volume: 0,
        pan: 0,
        frequency: 0,
        play: Play::Stopped(0),
        locks: 0,
        effect: None,
        // DS3D_DEFAULTMINDISTANCE and DS3D_DEFAULTMAXDISTANCE (the capture's defaults).
        min_distance: 1.0,
        max_distance: 1.0e9,
        position: [0.0; 3],
        velocity: [0.0; 3],
    }
}

/// `CreateSoundBuffer(desc, out, outer)`: the primary buffer (flags `DSBCAPS_PRIMARYBUFFER`
/// with `DSBCAPS_CTRL3D` and `DSBCAPS_CTRLVOLUME` at most, size and format 0) with the captured
/// result, or a secondary buffer of a kind the capture made, answered by the measured rule
/// (module documentation). `out` is written only on success.
fn create_sound_buffer(e: &mut Engine, this: Ptr, desc: Ptr, out: Ptr, outer: u32) -> u32 {
    let _ = device(e, this);
    if outer != 0 {
        return DSERR_INVALIDPARAM;
    }
    let d = desc.addr();
    assert!(
        e.mem.u32(d) == BUFFER_DESC_SIZE,
        "CreateSoundBuffer with a description of {:#x} bytes is not provided",
        e.mem.u32(d)
    );
    let flags = e.mem.u32(d + 4);
    let size = e.mem.u32(d + 8);
    let format = e.mem.u32(d + 0x10);
    let mut buffer = if flags & DSBCAPS_PRIMARYBUFFER != 0 {
        assert!(
            flags & !(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D | DSBCAPS_CTRLVOLUME) == 0
                && flags == 0x91,
            "IDirectSound8::CreateSoundBuffer of a primary buffer with flags {flags:#x} is not provided"
        );
        if size != 0 || format != 0 {
            return DSERR_INVALIDPARAM;
        }
        let result = table(e).primary;
        if result != DS_OK {
            return result;
        }
        new_buffer(flags)
    } else {
        assert!(
            format != 0,
            "CreateSoundBuffer without a format is not provided"
        );
        let f = e.mem.bytes(format, FORMAT_SIZE);
        let w16 = |o: usize| u16::from_le_bytes([f[o], f[o + 1]]);
        let w32 = |o: usize| u32::from_le_bytes([f[o], f[o + 1], f[o + 2], f[o + 3]]);
        let (tag, channels, rate, bits) = (w16(0), w16(2), w32(4), w16(14));
        let alg: Vec<u32> = (0..4).map(|i| e.mem.u32(d + 0x14 + 4 * i)).collect();
        let hrtf = alg == ALG_HRTF_LIGHT;
        let kind: Kind = (flags, hrtf, tag, channels, rate, bits);
        let align = u32::from(channels) * u32::from(bits) / 8;
        assert!(
            (hrtf || alg == [0; 4])
                && table(e).kinds.contains(&kind)
                && f == format_bytes(tag, channels, rate, bits)
                && (size < MIN_BUFFER_BYTES || size % align == 0),
            "CreateSoundBuffer of a buffer the capture didn't make (flags {flags:#x}, algorithm \
             {alg:x?}, format {}, {size} bytes) is not provided",
            hex_string(&f)
        );
        let result = create_result(kind, size);
        if (result as i32) < 0 {
            return result;
        }
        let mut b = new_buffer(flags);
        b.bytes = size;
        b.format = f;
        b.data = e.mem.alloc(size);
        if result == DS_NO_VIRTUALIZATION {
            let at = insert_buffer(e, b);
            e.mem.set_u32(out.addr(), at);
            return result;
        }
        b
    };
    buffer.references = 1;
    let at = insert_buffer(e, buffer);
    e.mem.set_u32(out.addr(), at);
    DS_OK
}

fn insert_buffer(e: &mut Engine, b: Buffer) -> u32 {
    let table = e.kernel.sound.buffer_vtable;
    let at = object(e, table);
    if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
        let v = voice_of(e, &b);
        with_voice(e, at, |m, at| {
            m.all.insert(at, v);
        });
    }
    e.kernel.sound.buffers.insert(at, b);
    at
}

fn buffer(e: &mut Engine, this: Ptr) -> &mut Buffer {
    e.kernel
        .sound
        .buffers
        .get_mut(&this.addr())
        .expect("a buffer of this layer")
}

fn secondary(e: &mut Engine, this: Ptr, call: &str) -> Buffer {
    let b = buffer(e, this).clone();
    assert!(
        b.flags & DSBCAPS_PRIMARYBUFFER == 0,
        "IDirectSoundBuffer::{call} on the primary buffer is not provided"
    );
    b
}

/// `QueryInterface(iid, out)` on a buffer: the primary buffer made with `DSBCAPS_CTRL3D` gives
/// its `IDirectSound3DListener` (one per buffer, a new reference each time); a secondary buffer
/// gives itself as `IDirectSoundBuffer8` and, with `DSBCAPS_CTRL3D`, its `IDirectSound3DBuffer`
/// (sharing its reference count); any other interface is `E_NOINTERFACE`.
fn buffer_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    let id = e.mem.bytes(iid.addr(), 16);
    let b = buffer(e, this).clone();
    if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
        if id == IID_BUFFER8 {
            buffer(e, this).references += 1;
            e.mem.set_u32(out.addr(), this.addr());
            return DS_OK;
        }
        if id == IID_BUFFER3D && b.flags & DSBCAPS_CTRL3D != 0 {
            let three_d = if b.three_d == 0 {
                let table = e.kernel.sound.buffer3d_vtable;
                let at = object(e, table);
                e.kernel.sound.buffers3d.insert(at, this.addr());
                buffer(e, this).three_d = at;
                at
            } else {
                b.three_d
            };
            buffer(e, this).references += 1;
            e.mem.set_u32(out.addr(), three_d);
            return DS_OK;
        }
        return no_interface(e, this, iid, out);
    }
    let is_3d_primary = b.flags & DSBCAPS_CTRL3D != 0;
    if id != IID_LISTENER || !is_3d_primary {
        return no_interface(e, this, iid, out);
    }
    let listener = if b.listener != 0 {
        let l = e
            .kernel
            .sound
            .listeners
            .get_mut(&b.listener)
            .expect("its listener");
        l.0 += 1;
        b.listener
    } else {
        let defaults = listener_defaults(e);
        let table = e.kernel.sound.listener_vtable;
        let listener = object(e, table);
        e.kernel.sound.listeners.insert(listener, (1, defaults));
        buffer(e, this).listener = listener;
        listener
    };
    e.mem.set_u32(out.addr(), listener);
    DS_OK
}

/// The listener's state when the primary buffer gives it out: the captured `DS3DLISTENER`.
fn listener_defaults(e: &mut Engine) -> Listener {
    let (result, b) = table(e).listener.clone();
    assert!(
        result == DS_OK && b.len() == LISTENER_SIZE as usize,
        "the sound table has no listener: {CAPTURE_HINT}"
    );
    let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    let v = |o: usize| [f(o), f(o + 4), f(o + 8)];
    Listener {
        position: v(4),
        velocity: v(0x10),
        front: v(0x1c),
        top: v(0x28),
        distance_factor: f(0x34),
        rolloff: f(0x38),
        doppler: f(0x3c),
    }
}

fn buffer_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let b = buffer(e, this);
    b.references += 1;
    b.references
}

/// Drops one reference of the buffer at `at`; frees it (its memory, its 3D interface) at 0.
fn release_buffer(e: &mut Engine, at: u32) -> u32 {
    let b = e
        .kernel
        .sound
        .buffers
        .get_mut(&at)
        .expect("a buffer of this layer");
    b.references -= 1;
    let left = b.references;
    if left == 0 {
        let b = e.kernel.sound.buffers.remove(&at).expect("the buffer");
        with_voice(e, at, |m, at| {
            m.all.remove(&at);
            m.playing.retain(|&p| p != at);
        });
        if b.data != 0 {
            e.mem.free(b.data);
        }
        if b.three_d != 0 {
            e.kernel.sound.buffers3d.remove(&b.three_d);
            e.mem.free(b.three_d);
        }
        e.mem.free(at);
    }
    left
}

fn buffer_release(e: &mut Engine, this: Ptr) -> u32 {
    release_buffer(e, this.addr())
}

/// `GetCaps(caps)`: `caps->dwSize` must be 0x14; the flags (with `DSBCAPS_LOCSOFTWARE`) and
/// size, transfer rate and processor overhead 0 (measured).
fn buffer_get_caps(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    let b = secondary(e, this, "GetCaps");
    let at = caps.addr();
    if e.mem.u32(at) != DSBCAPS_SIZE {
        return DSERR_INVALIDPARAM;
    }
    e.mem.set_u32(at + 4, b.flags | DSBCAPS_LOCSOFTWARE);
    e.mem.set_u32(at + 8, b.bytes);
    e.mem.set_u32(at + 12, 0);
    e.mem.set_u32(at + 16, 0);
    DS_OK
}

/// `GetFormat(format, size, written)`: the 0x12-byte `WAVEFORMATEX` (when `format` is given and
/// `size` holds it) and its size through `written`. A smaller `size` is not provided.
fn get_format(e: &mut Engine, this: Ptr, format: Ptr, size: u32, written: Ptr) -> u32 {
    let b = secondary(e, this, "GetFormat");
    if !format.is_null() {
        assert!(
            size >= FORMAT_SIZE,
            "GetFormat into {size} bytes is not provided"
        );
        e.mem.write(format.addr(), &b.format);
    }
    if !written.is_null() {
        e.mem.set_u32(written.addr(), FORMAT_SIZE);
    }
    DS_OK
}

fn get_volume(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetVolume").volume;
    e.mem.set_i32(out.addr(), v);
    DS_OK
}

fn get_pan(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetPan").pan;
    e.mem.set_i32(out.addr(), v);
    DS_OK
}

/// `GetFrequency(out)`: the frequency set, or the format's rate.
fn get_frequency(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetFrequency").frequency();
    e.mem.set_u32(out.addr(), v);
    DS_OK
}

/// `SetVolume(v)`: -10000..0, else `DSERR_INVALIDPARAM`. Without `DSBCAPS_CTRLVOLUME` not
/// provided (only the pan's order was measured).
fn set_volume(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let b = secondary(e, this, "SetVolume");
    if !volume_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    assert!(
        b.flags & DSBCAPS_CTRLVOLUME != 0,
        "SetVolume without DSBCAPS_CTRLVOLUME is not provided"
    );
    buffer(e, this).volume = v;
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.volume = v;
        }
    });
    DS_OK
}

/// `SetPan(v)`: -10000..10000, else `DSERR_INVALIDPARAM`; then without `DSBCAPS_CTRLPAN`
/// `DSERR_CONTROLUNAVAIL` (measured order).
fn set_pan(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let b = secondary(e, this, "SetPan");
    if !pan_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    if b.flags & DSBCAPS_CTRLPAN == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    buffer(e, this).pan = v;
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.pan = v;
        }
    });
    DS_OK
}

/// `SetFrequency(v)`: 0 (the format's rate) or 100..200000, else `DSERR_INVALIDPARAM`. A
/// playing buffer goes on from where it is at the new speed. Without `DSBCAPS_CTRLFREQUENCY`
/// not provided.
fn set_frequency(e: &mut Engine, this: Ptr, v: u32) -> u32 {
    let b = secondary(e, this, "SetFrequency");
    if !frequency_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    assert!(
        b.flags & DSBCAPS_CTRLFREQUENCY != 0,
        "SetFrequency without DSBCAPS_CTRLFREQUENCY is not provided"
    );
    let now = e.clock_now_us();
    let b = buffer(e, this);
    if let Play::Playing {
        looping,
        start,
        base,
    } = b.play
    {
        let n = periods(start, now);
        let consumed = base + frames(n, b.frequency());
        b.play = Play::Playing {
            looping,
            start: now,
            base: consumed,
        };
    }
    b.frequency = v;
    if matches!(b.play, Play::Playing { .. }) {
        heard_exactly(e, this, "SetFrequency");
    }
    DS_OK
}

/// Where the mixer's exact path ends (module documentation): with the output on, a buffer
/// that would be heard at another rate than the device's, in 3D or through the distortion
/// stops the run.
fn heard_exactly(e: &mut Engine, this: Ptr, call: &str) {
    let Some(rate) = e.kernel.sound.mix.as_ref().map(|m| m.rate) else {
        return;
    };
    let b = buffer(e, this);
    let f = b.frequency();
    assert!(
        f == rate,
        "{call}: a buffer heard at {f} Hz on a {rate} Hz device is not provided (Windows' \
         resampler is not pinned down; the maintainer decides)"
    );
    assert!(
        b.flags & DSBCAPS_CTRL3D == 0,
        "{call}: a 3D buffer heard on the device is not provided (Windows' 3D gains are not \
         pinned down; the maintainer decides)"
    );
    assert!(
        b.effect.is_none(),
        "{call}: a buffer heard through the {:?} effect is not provided (not analysed)",
        b.effect
    );
}

/// Device periods from `start` to `now` (the platform clock's 10 ms steps).
fn periods(start: u64, now: u64) -> u64 {
    now / PERIOD_US - start / PERIOD_US
}

/// Frames a buffer moves on in `n` periods at `frequency`.
fn frames(n: u64, frequency: u32) -> u64 {
    n * u64::from(frequency) / 100
}

/// Brings a playing non-looping buffer that reached its end to `Ended`; answers the play and
/// write cursors in bytes.
fn cursors(e: &mut Engine, this: Ptr) -> (u32, u32) {
    let now = e.clock_now_us();
    let b = buffer(e, this);
    match b.play {
        Play::Stopped(at) => (at, at),
        Play::Ended => (0, 0),
        Play::Playing {
            looping,
            start,
            base,
        } => {
            let (n, f, len, align) = (periods(start, now), b.frequency(), b.frames(), b.align());
            let consumed = base + frames(n, f);
            if !looping && consumed >= len {
                b.play = Play::Ended;
                return (0, 0);
            }
            let write = consumed + u64::from(n >= 1);
            // The play cursor trails the write cursor by three periods at the format's rate.
            let lag = 3 * u64::from(b.rate()) / 100;
            let play = if write - base > lag {
                write - lag
            } else {
                base
            };
            let wrap = |x: u64| if looping { x % len } else { x.min(len) };
            ((wrap(play) as u32) * align, (wrap(write) as u32) * align)
        }
    }
}

/// `GetCurrentPosition(play, write)`: the cursors (module documentation); either pointer may be
/// NULL.
fn get_current_position(e: &mut Engine, this: Ptr, play: Ptr, write: Ptr) -> u32 {
    let _ = secondary(e, this, "GetCurrentPosition");
    let (p, w) = cursors(e, this);
    if !play.is_null() {
        e.mem.set_u32(play.addr(), p);
    }
    if !write.is_null() {
        e.mem.set_u32(write.addr(), w);
    }
    DS_OK
}

/// `GetStatus(out)`: `DSBSTATUS_PLAYING`, with `DSBSTATUS_LOOPING` when looping; 0 when stopped
/// or played to the end (measured).
fn get_status(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let _ = secondary(e, this, "GetStatus");
    let _ = cursors(e, this);
    let status = match buffer(e, this).play {
        Play::Playing { looping, .. } => {
            DSBSTATUS_PLAYING | if looping { DSBSTATUS_LOOPING } else { 0 }
        }
        _ => 0,
    };
    e.mem.set_u32(out.addr(), status);
    DS_OK
}

/// `Lock(offset, bytes, p1, n1, p2, n2, flags)`: with `DSBLOCK_ENTIREBUFFER` (offset 0) or 0,
/// the region from `offset`, wrapping into a second region at the start. `bytes` 0 or larger
/// than the buffer without `DSBLOCK_ENTIREBUFFER` is `DSERR_INVALIDPARAM`; the write cursor flag
/// and a second region the caller has no pointer for are not provided.
#[allow(clippy::too_many_arguments)] // The interface's receiver and its seven words.
fn lock(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    bytes: u32,
    p1: Ptr,
    n1: Ptr,
    p2: Ptr,
    n2: Ptr,
    flags: u32,
) -> u32 {
    let b = secondary(e, this, "Lock");
    let (offset, bytes) = match flags {
        DSBLOCK_ENTIREBUFFER => {
            assert!(
                offset == 0,
                "Lock of the whole buffer from {offset} is not provided"
            );
            (0, b.bytes)
        }
        0 => {
            if bytes == 0 || bytes > b.bytes || offset >= b.bytes {
                return DSERR_INVALIDPARAM;
            }
            (offset, bytes)
        }
        _ => panic!("IDirectSoundBuffer::Lock flags {flags:#x} are not provided"),
    };
    let first = bytes.min(b.bytes - offset);
    let second = bytes - first;
    assert!(
        second == 0 || !p2.is_null(),
        "Lock wrapping round without a second pointer is not provided"
    );
    e.mem.set_u32(p1.addr(), b.data + offset);
    e.mem.set_u32(n1.addr(), first);
    if !p2.is_null() {
        e.mem
            .set_u32(p2.addr(), if second == 0 { 0 } else { b.data });
        e.mem.set_u32(n2.addr(), second);
    }
    buffer(e, this).locks += 1;
    DS_OK
}

/// `Unlock(p1, n1, p2, n2)`: ends a `Lock`; an unlock without a lock is not provided.
fn unlock(e: &mut Engine, this: Ptr, _p1: u32, _n1: u32, _p2: u32, _n2: u32) -> u32 {
    let b = buffer(e, this);
    assert!(
        b.locks > 0,
        "IDirectSoundBuffer::Unlock without a Lock is not provided"
    );
    b.locks -= 1;
    let (data, bytes) = (b.data, b.bytes);
    if e.kernel.sound.mix.is_some() {
        let samples = e.mem.bytes(data, bytes);
        with_voice(e, this.addr(), |m, at| {
            if let Some(voice) = m.all.get_mut(&at) {
                voice.data = samples;
            }
        });
    }
    DS_OK
}

/// `Play(0, 0, flags)` with flags 0 or `DSBPLAY_LOOPING`: plays from where the buffer stopped
/// (on the platform clock); a playing buffer only takes the looping flag. Playing a buffer that
/// played to its end, or from its end, is not provided.
fn play(e: &mut Engine, this: Ptr, reserved: u32, priority: u32, flags: u32) -> u32 {
    let b = secondary(e, this, "Play");
    assert!(
        reserved == 0 && priority == 0 && flags & !DSBPLAY_LOOPING == 0,
        "IDirectSoundBuffer::Play({reserved:#x}, {priority:#x}, {flags:#x}) is not provided"
    );
    let looping = flags & DSBPLAY_LOOPING != 0;
    let _ = cursors(e, this);
    let now = e.clock_now_us();
    let play = match buffer(e, this).play {
        Play::Stopped(at) => {
            assert!(
                at < b.bytes,
                "Play from the end of the buffer is not provided"
            );
            Play::Playing {
                looping,
                start: now,
                base: u64::from(at / b.align()),
            }
        }
        Play::Playing { start, base, .. } => Play::Playing {
            looping,
            start,
            base,
        },
        Play::Ended => panic!("Play on a buffer that played to its end is not provided"),
    };
    let was_playing = matches!(buffer(e, this).play, Play::Playing { .. });
    buffer(e, this).play = play;
    heard_exactly(e, this, "Play");
    let Play::Playing { base, .. } = play else {
        unreachable!()
    };
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.looping = looping;
            if !was_playing {
                voice.position = Some(base as usize);
                m.playing.retain(|&p| p != at);
                m.playing.insert(0, at);
            }
        }
    });
    DS_OK
}

/// `Stop()`: both cursors stay at the write cursor (at the end for a buffer that played to its
/// end), measured.
fn stop(e: &mut Engine, this: Ptr) -> u32 {
    let b = secondary(e, this, "Stop");
    let ended = buffer(e, this).play == Play::Ended;
    let (_, write) = cursors(e, this);
    let ended = ended || buffer(e, this).play == Play::Ended;
    buffer(e, this).play = match b.play {
        Play::Stopped(at) => Play::Stopped(at),
        _ if ended => Play::Stopped(b.bytes),
        _ => Play::Stopped(write),
    };
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.position = None;
        }
        m.playing.retain(|&p| p != at);
    });
    DS_OK
}

/// `SetCurrentPosition(at)` on a stopped buffer: both cursors there; past the end
/// `DSERR_INVALIDPARAM`. On a playing buffer, or one that played to its end, not provided.
fn set_current_position(e: &mut Engine, this: Ptr, at: u32) -> u32 {
    let b = secondary(e, this, "SetCurrentPosition");
    let _ = cursors(e, this);
    if at > b.bytes {
        return DSERR_INVALIDPARAM;
    }
    let state = buffer(e, this).play;
    assert!(
        matches!(state, Play::Stopped(_)),
        "SetCurrentPosition on a buffer that is not stopped is not provided"
    );
    buffer(e, this).play = Play::Stopped(at - at % b.align());
    DS_OK
}

fn effect_name(e: &Engine, class: u32) -> Option<&'static str> {
    let id = e.mem.bytes(class, 16);
    if id == GUID_DISTORTION {
        Some("distortion")
    } else if id == GUID_I3DL2 {
        Some("i3dl2")
    } else {
        None
    }
}

/// `SetFX(count, descriptions, results)` with one effect, the distortion or the I3DL2 reverb,
/// on a stopped, unlocked buffer: the captured answer (and result code when `results` is
/// given). Other effects, counts or descriptions are not provided.
fn set_fx(e: &mut Engine, this: Ptr, count: u32, descriptions: Ptr, results: Ptr) -> u32 {
    let b = secondary(e, this, "SetFX");
    let d = descriptions.addr();
    let name = effect_name(e, d + 8);
    assert!(
        count == 1
            && e.mem.u32(d) == EFFECT_DESC_SIZE
            && e.mem.u32(d + 4) == 0
            && e.mem.u32(d + 0x18) == 0
            && e.mem.u32(d + 0x1c) == 0
            && name.is_some(),
        "IDirectSoundBuffer8::SetFX with these effects is not provided"
    );
    let name = name.expect("checked");
    assert!(
        b.locks == 0 && matches!(b.play, Play::Stopped(_)),
        "SetFX on a playing or locked buffer is not provided"
    );
    let fx = b.flags & DSBCAPS_CTRLFX != 0;
    let (result, code, _) = table(e).effect(name, fx);
    if !results.is_null() && fx {
        e.mem.set_u32(results.addr(), code);
    }
    if result == DS_OK {
        buffer(e, this).effect = Some(name);
    }
    result
}

/// `GetObjectInPath(class, 0, iid, out)` for the effect `SetFX` set: the captured answer, and for
/// the distortion a new `IDirectSoundFXDistortion8` (its parameters `DSFXDistortion` as the
/// capture's defaults are not kept: the game sets them all at once).
fn get_object_in_path(
    e: &mut Engine,
    this: Ptr,
    class: Ptr,
    index: u32,
    iid: Ptr,
    out: Ptr,
) -> u32 {
    let b = secondary(e, this, "GetObjectInPath");
    let name = effect_name(e, class.addr());
    let interface = e.mem.bytes(iid.addr(), 16);
    let fits = match name {
        Some("distortion") => interface == IID_DISTORTION8,
        Some("i3dl2") => interface == IID_I3DL2_8,
        _ => false,
    };
    assert!(
        index == 0 && fits,
        "GetObjectInPath for this object is not provided"
    );
    let name = name.expect("checked");
    let fx = b.flags & DSBCAPS_CTRLFX != 0;
    let (_, _, result) = table(e).effect(name, fx);
    if result != DS_OK {
        return result;
    }
    assert!(
        b.effect == Some(name) && name == "distortion",
        "GetObjectInPath({name}) without SetFX is not provided"
    );
    let table = e.kernel.sound.distortion_vtable;
    let at = object(e, table);
    e.kernel.sound.distortions.insert(at, (1, [0.0; 5]));
    e.mem.set_u32(out.addr(), at);
    DS_OK
}

fn distortion(e: &mut Engine, this: Ptr) -> &mut (u32, [f32; 5]) {
    e.kernel
        .sound
        .distortions
        .get_mut(&this.addr())
        .expect("a distortion of this layer")
}

fn distortion_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = distortion(e, this);
    d.0 += 1;
    d.0
}

fn distortion_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = distortion(e, this);
    d.0 -= 1;
    let left = d.0;
    if left == 0 {
        e.kernel.sound.distortions.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `SetAllParameters(params)`: `DSFXDistortion` within its ranges (module documentation), else
/// `DSERR_INVALIDPARAM`. A NaN is not provided.
fn distortion_set_all_parameters(e: &mut Engine, this: Ptr, params: Ptr) -> u32 {
    let p: [f32; 5] = std::array::from_fn(|i| e.mem.f32(params.addr() + 4 * i as u32));
    assert!(
        p.iter().all(|f| !f.is_nan()),
        "a NaN distortion parameter is not provided"
    );
    if !distortion_ok(p) {
        return DSERR_INVALIDPARAM;
    }
    distortion(e, this).1 = p;
    DS_OK
}

/// The buffer a 3D interface belongs to.
fn owner(e: &Engine, this: Ptr) -> u32 {
    *e.kernel
        .sound
        .buffers3d
        .get(&this.addr())
        .expect("an IDirectSound3DBuffer of this layer")
}

fn buffer3d_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let at = owner(e, this);
    buffer_add_ref(e, Ptr::new(at))
}

fn buffer3d_release(e: &mut Engine, this: Ptr) -> u32 {
    let at = owner(e, this);
    release_buffer(e, at)
}

fn apply_ok(apply: u32) {
    assert!(
        apply <= 1,
        "a 3D setting with apply {apply:#x} is not provided"
    );
}

/// `SetMinDistance(v, apply)`: above 0, else `DSERR_INVALIDPARAM` (measured: 0 refused). NaN is
/// not provided.
fn set_min_distance(e: &mut Engine, this: Ptr, v: f32, apply: u32) -> u32 {
    apply_ok(apply);
    assert!(!v.is_nan(), "a NaN distance is not provided");
    if v <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    let at = owner(e, this);
    buffer(e, Ptr::new(at)).min_distance = v;
    DS_OK
}

/// `SetMaxDistance(v, apply)`: above 0, else `DSERR_INVALIDPARAM` (measured: -1 refused).
fn set_max_distance(e: &mut Engine, this: Ptr, v: f32, apply: u32) -> u32 {
    apply_ok(apply);
    assert!(!v.is_nan(), "a NaN distance is not provided");
    if v <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    let at = owner(e, this);
    buffer(e, Ptr::new(at)).max_distance = v;
    DS_OK
}

fn buffer3d_set_position(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    apply_ok(apply);
    assert!(
        ![x, y, z].iter().any(|f| f.is_nan()),
        "a NaN position is not provided"
    );
    let at = owner(e, this);
    buffer(e, Ptr::new(at)).position = [x, y, z];
    DS_OK
}

fn buffer3d_set_velocity(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    apply_ok(apply);
    assert!(
        ![x, y, z].iter().any(|f| f.is_nan()),
        "a NaN velocity is not provided"
    );
    let at = owner(e, this);
    buffer(e, Ptr::new(at)).velocity = [x, y, z];
    DS_OK
}

fn listener(e: &mut Engine, this: Ptr) -> &mut (u32, Listener) {
    e.kernel
        .sound
        .listeners
        .get_mut(&this.addr())
        .expect("an IDirectSound3DListener of this layer")
}

fn listener_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 += 1;
    l.0
}

fn listener_release(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 -= 1;
    let left = l.0;
    if left == 0 {
        e.kernel.sound.listeners.remove(&this.addr());
        for b in e.kernel.sound.buffers.values_mut() {
            if b.listener == this.addr() {
                b.listener = 0;
            }
        }
        e.mem.free(this.addr());
    }
    left
}

fn put3(e: &mut Engine, at: u32, v: [f32; 3]) {
    for (i, f) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, f);
    }
}

fn get3(e: &Engine, at: u32) -> [f32; 3] {
    [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
}

/// `GetAllParameters(params)`: `params->dwSize` must be 0x40; the listener's state.
fn listener_get_all_parameters(e: &mut Engine, this: Ptr, params: Ptr) -> u32 {
    let p = params.addr();
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let l = listener(e, this).1;
    put3(e, p + 4, l.position);
    put3(e, p + 0x10, l.velocity);
    put3(e, p + 0x1c, l.front);
    put3(e, p + 0x28, l.top);
    e.mem.set_f32(p + 0x34, l.distance_factor);
    e.mem.set_f32(p + 0x38, l.rolloff);
    e.mem.set_f32(p + 0x3c, l.doppler);
    DS_OK
}

/// `GetPosition(out)`.
fn listener_get_position(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let position = listener(e, this).1.position;
    put3(e, out.addr(), position);
    DS_OK
}

/// `SetAllParameters(params, apply)`: `params->dwSize` must be 0x40. Every change takes effect
/// at once here (`DS3D_IMMEDIATE` or `DS3D_DEFERRED`: nothing is mixed, so a deferred setting is
/// the same state as an applied one).
fn listener_set_all_parameters(e: &mut Engine, this: Ptr, params: Ptr, _apply: u32) -> u32 {
    let p = params.addr();
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let new = Listener {
        position: get3(e, p + 4),
        velocity: get3(e, p + 0x10),
        front: get3(e, p + 0x1c),
        top: get3(e, p + 0x28),
        distance_factor: e.mem.f32(p + 0x34),
        rolloff: e.mem.f32(p + 0x38),
        doppler: e.mem.f32(p + 0x3c),
    };
    listener(e, this).1 = new;
    DS_OK
}

fn listener_set_distance_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.distance_factor = value;
    DS_OK
}

fn listener_set_doppler_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.doppler = value;
    DS_OK
}

fn listener_set_rolloff_factor(e: &mut Engine, this: Ptr, value: f32, _apply: u32) -> u32 {
    listener(e, this).1.rolloff = value;
    DS_OK
}

#[allow(clippy::too_many_arguments)] // The interface's receiver, six floats and the apply word.
fn listener_set_orientation(
    e: &mut Engine,
    this: Ptr,
    fx: f32,
    fy: f32,
    fz: f32,
    tx: f32,
    ty: f32,
    tz: f32,
    _apply: u32,
) -> u32 {
    let l = &mut listener(e, this).1;
    l.front = [fx, fy, fz];
    l.top = [tx, ty, tz];
    DS_OK
}

fn listener_set_position(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, _apply: u32) -> u32 {
    listener(e, this).1.position = [x, y, z];
    DS_OK
}

fn listener_set_velocity(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, _apply: u32) -> u32 {
    listener(e, this).1.velocity = [x, y, z];
    DS_OK
}

/// `CommitDeferredSettings()`: settings already apply (see `SetAllParameters`); `DS_OK`.
fn listener_commit(e: &mut Engine, this: Ptr) -> u32 {
    let _ = listener(e, this);
    DS_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A made-up capture (not a real machine's) in `dsound-capture`'s form: the start-up
    /// answers, a few buffers of the kinds the tests make, the effects and settings.
    fn table_text() -> String {
        let mut t = String::new();
        t += "DsCreate\t\t0\t\nDsCooperativeLevel\t2\t0\t\n";
        let mut caps = vec![0u8; 0x60];
        caps[0] = 0x60;
        caps[4..8].copy_from_slice(&0xf1fu32.to_le_bytes());
        t += &format!("DsCaps\t\t0\t{}\n", hex_string(&caps));
        t += "DsPrimary\t91\t0\t010002002256000044ac0000020008000000 12\n";
        let mut listener = vec![0u8; 0x40];
        listener[0] = 0x40;
        for o in [0x24usize, 0x2c, 0x34, 0x38, 0x3c] {
            listener[o..o + 4].copy_from_slice(&1.0f32.to_le_bytes());
        }
        t += &format!("DsListener\t\t0 0\t{}\n", hex_string(&listener));
        for (flags, alg, channels, size) in [
            (0x2e0u32, 0u32, 1u16, 3u32),
            (0x2e0, 0, 1, 6612),
            (0x2e0, 0, 1, 44100),
            (0xe0, 0, 1, 200),
            (0x302b0, 0, 1, 44100),
            (0x302b0, 0, 2, 88200),
            (0x302b0, 1, 1, 44100),
        ] {
            let kind: Kind = (flags, alg != 0, 1, channels, 22050, 16);
            let result = create_result(kind, size);
            let words = format!("{flags:x} {alg} 1 {channels:x} 5622 10 {size:x}");
            if (result as i32) < 0 {
                t += &format!("DsBuffer\t{words}\t{result:x}\t\t\t\t\n");
            } else {
                let mut caps = Vec::new();
                for w in [0x14, flags | 8, size, 0, 0] {
                    caps.extend_from_slice(&u32::to_le_bytes(w));
                }
                let three_d = if flags & 0x10 != 0 { 0 } else { E_NOINTERFACE };
                t += &format!(
                    "DsBuffer\t{words}\t{result:x}\t{}\t{} 12\t5622 0 0 0\t0 {three_d:x}\n",
                    hex_string(&caps),
                    hex_string(&format_bytes(1, channels, 22050, 16))
                );
            }
        }
        t += "DsFx\tdistortion 2e0 1 5622 ac44\t0\t2\t0\t00\n";
        t += "DsFx\tdistortion e0 1 5622 ac44\t8878001e\tffffffff\t8878001e\t00\n";
        t += "DsFx\ti3dl2 2e0 1 5622 ac44\t80040154\t5\t88781161\t00\n";
        t += "DsFx\ti3dl2 e0 1 5622 ac44\t8878001e\tffffffff\t8878001e\t00\n";
        t += "DsSet\tdistortion c2740000\t80070057\t00\nDsSet\tdistortion 00000000\t0\t00\n";
        t += "DsSet\t302b0 pan 0\t8878001e\t0\nDsSet\t2e0 pan 2711\t80070057\t2710\n";
        t += "DsEnd\n";
        t
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        load_table(&mut e, &table_text());
        e.mem.map(0x1000, 0x2000);
        e
    }

    /// `BSXenonAudio::Init`'s calls in its order; the device; answers 0x1000.. scratch.
    fn start_up(e: &mut Engine) -> u32 {
        assert_eq!(direct_sound_create8(e, 0, Ptr::new(0x1000), 0), DS_OK);
        e.mem.u32(0x1000)
    }

    /// A secondary buffer of 22050 Hz 16-bit PCM through `CreateSoundBuffer`.
    fn create(
        e: &mut Engine,
        ds: u32,
        flags: u32,
        hrtf: bool,
        channels: u16,
        size: u32,
    ) -> (u32, u32) {
        let format = 0x1600;
        e.mem.write(format, &format_bytes(1, channels, 22050, 16));
        let desc = 0x1700;
        for (i, w) in [0x24, flags, size, 0, format].into_iter().enumerate() {
            e.mem.set_u32(desc + 4 * i as u32, w);
        }
        let alg = if hrtf { ALG_HRTF_LIGHT } else { [0; 4] };
        for (i, w) in alg.into_iter().enumerate() {
            e.mem.set_u32(desc + 0x14 + 4 * i as u32, w);
        }
        e.mem.set_u32(0x1004, 0);
        let r = e.vcall(ds, 0xc, &[desc, 0x1004, 0]).u32();
        (r, e.mem.u32(0x1004))
    }

    #[test]
    fn the_audio_start_up_gets_a_device_a_primary_buffer_and_a_listener() {
        let mut e = engine();
        assert_eq!(co_initialize(&mut e, 0), DS_OK);
        assert_eq!(co_initialize(&mut e, 0), S_FALSE);
        let ds = start_up(&mut e);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 2]).u32(), DS_OK);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 9]).u32(), DSERR_INVALIDPARAM);
        e.mem.set_u32(0x1100, DSCAPS_SIZE);
        assert_eq!(e.vcall(ds, 0x10, &[0x1100]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1104), 0xf1f, "the captured flags");
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        assert_eq!(e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]).u32(), DS_OK);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x1008]).u32(), DS_OK);
        let listener = e.mem.u32(0x1008);
        assert_ne!(listener, 0);
        assert_eq!(
            e.vcall(
                listener,
                0x38,
                &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 1]
            )
            .u32(),
            DS_OK
        );
        assert_eq!(e.vcall(listener, 0x1c, &[0x1400]).u32(), DS_OK);
        assert_eq!([0, 4, 8].map(|o| e.mem.f32(0x1400 + o)), [1.0, 2.0, 3.0]);
        e.mem.set_u32(0x1500, LISTENER_SIZE);
        assert_eq!(e.vcall(listener, 0xc, &[0x1500]).u32(), DS_OK);
        assert_eq!(e.mem.f32(0x1500 + 0x34), 1.0, "distance factor");
        assert_eq!(e.mem.f32(0x1500 + 0x28 + 4), 1.0, "top is +y");
        e.mem.write(0x1300, &[9u8; 16]);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x100c]).u32(), E_NOINTERFACE);
        assert_eq!(e.vcall(listener, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(primary, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(ds, 8, &[]).u32(), 0);
    }

    #[test]
    fn buffers_are_refused_and_made_as_measured() {
        let mut e = engine();
        let ds = start_up(&mut e);
        assert_eq!(create(&mut e, ds, 0x2e0, false, 1, 3).0, DSERR_INVALIDPARAM);
        assert_eq!(
            create(&mut e, ds, 0x2e0, false, 1, 6612).0,
            DSERR_BUFFERTOOSMALL
        );
        assert_eq!(
            create(&mut e, ds, 0x302b0, false, 2, 88200).0,
            DSERR_INVALIDPARAM
        );
        let (r, hrtf) = create(&mut e, ds, 0x302b0, true, 1, 44100);
        assert_eq!(r, DS_NO_VIRTUALIZATION);
        assert_ne!(hrtf, 0);
        let (r, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(r, DS_OK);
        e.mem.set_u32(0x1800, 0x14);
        assert_eq!(e.vcall(b, 0xc, &[0x1800]).u32(), DS_OK);
        assert_eq!([4, 8].map(|o| e.mem.u32(0x1800 + o)), [0x2e8, 44100]);
        assert_eq!(e.vcall(b, 0x14, &[0x1900, 0x12, 0x1920]).u32(), DS_OK);
        assert_eq!(e.mem.bytes(0x1900, 0x12), format_bytes(1, 1, 22050, 16));
        assert_eq!(e.mem.u32(0x1920), 0x12);
        assert_eq!(e.vcall(b, 0x20, &[0x1924]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1924), 22050);
        // IDirectSoundBuffer8 yes, IDirectSound3DBuffer no (2D).
        e.mem.write(0x1300, &IID_BUFFER8);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x1928]).u32(), DS_OK);
        e.mem.write(0x1300, &IID_BUFFER3D);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x192c]).u32(), E_NOINTERFACE);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 0);
    }

    #[test]
    #[should_panic(expected = "the capture didn't make")]
    fn a_buffer_the_capture_did_not_make_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        create(&mut e, ds, 0x2a0, false, 1, 44100);
    }

    #[test]
    fn a_3d_buffer_shares_its_reference_count_and_keeps_its_settings() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        e.mem.write(0x1300, &IID_BUFFER3D);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x1928]).u32(), DS_OK);
        let three_d = e.mem.u32(0x1928);
        assert_eq!(
            e.vcall(three_d, 0x44, &[1.0e9f32.to_bits(), 1]).u32(),
            DS_OK
        );
        assert_eq!(e.vcall(three_d, 0x44, &[0, 1]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(
            e.vcall(three_d, 0x40, &[(-1.0f32).to_bits(), 1]).u32(),
            DSERR_INVALIDPARAM
        );
        let p = (-3.4e37f32).to_bits();
        assert_eq!(e.vcall(three_d, 0x4c, &[p, p, p, 1]).u32(), DS_OK);
        // Pan: in range but no DSBCAPS_CTRLPAN.
        assert_eq!(e.vcall(b, 0x40, &[0]).u32(), DSERR_CONTROLUNAVAIL);
        assert_eq!(e.vcall(b, 0x40, &[10001]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x3c, &[(-10000i32) as u32]).u32(), DS_OK);
        assert_eq!(e.vcall(three_d, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 0);
    }

    #[test]
    fn lock_gives_the_buffer_s_memory() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(
            e.vcall(b, 0x2c, &[0, 44100, 0x1a00, 0x1a04, 0, 0, 0]).u32(),
            DS_OK
        );
        let p = e.mem.u32(0x1a00);
        assert_eq!(e.mem.u32(0x1a04), 44100);
        e.mem.write(p, &[7; 16]);
        assert_eq!(e.vcall(b, 0x4c, &[p, 44100, 0, 0]).u32(), DS_OK);
        assert_eq!(
            e.vcall(b, 0x2c, &[0, 0, 0x1a00, 0x1a04, 0x1a08, 0x1a0c, 2])
                .u32(),
            DS_OK
        );
        assert_eq!(
            [e.mem.u32(0x1a00), e.mem.u32(0x1a04), e.mem.u32(0x1a08)],
            [p, 44100, 0]
        );
        assert_eq!(e.mem.bytes(p, 2), [7, 7]);
        // Wrapping round.
        assert_eq!(
            e.vcall(b, 0x2c, &[44000, 300, 0x1a10, 0x1a14, 0x1a18, 0x1a1c, 0])
                .u32(),
            DS_OK
        );
        assert_eq!(
            [e.mem.u32(0x1a14), e.mem.u32(0x1a18), e.mem.u32(0x1a1c)],
            [100, p, 200]
        );
    }

    #[test]
    fn volume_pan_and_frequency_keep_their_ranges() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(
            e.vcall(b, 0x3c, &[(-10001i32) as u32]).u32(),
            DSERR_INVALIDPARAM
        );
        assert_eq!(e.vcall(b, 0x3c, &[1]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x3c, &[(-2500i32) as u32]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x18, &[0x1a00]).u32(), DS_OK);
        assert_eq!(e.mem.i32(0x1a00), -2500);
        assert_eq!(e.vcall(b, 0x40, &[10000]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x44, &[99]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x44, &[200_001]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x44, &[11025]).u32(), DS_OK);
        e.vcall(b, 0x20, &[0x1a04]);
        assert_eq!(e.mem.u32(0x1a04), 11025);
        assert_eq!(e.vcall(b, 0x44, &[0]).u32(), DS_OK);
        e.vcall(b, 0x20, &[0x1a04]);
        assert_eq!(e.mem.u32(0x1a04), 22050, "0 is the format's rate");
    }

    /// The game's `SetFX` (`00aec530`): one `DSEFFECTDESC` of 0x20 bytes.
    fn effect_desc(e: &mut Engine, class: &[u8; 16]) -> u32 {
        let d = 0x1b00;
        e.mem.set_u32(d, 0x20);
        e.mem.set_u32(d + 4, 0);
        e.mem.write(d + 8, class);
        e.mem.set_u32(d + 0x18, 0);
        e.mem.set_u32(d + 0x1c, 0);
        d
    }

    #[test]
    fn the_distortion_works_and_the_reverb_is_missing_as_captured() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        assert_eq!(e.vcall(b, 0x54, &[1, d, 0x1c00]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1c00), 2, "DSFXR_LOCSOFTWARE");
        e.mem.write(0x1c10, &GUID_DISTORTION);
        e.mem.write(0x1c20, &IID_DISTORTION8);
        assert_eq!(e.vcall(b, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(), DS_OK);
        let fx = e.mem.u32(0x1c30);
        for (i, v) in [-18.0f32, 0.0, 5000.0, 1135.0, 5000.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(0x1c40 + 4 * i as u32, v);
        }
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DS_OK);
        e.mem.set_f32(0x1c40, -61.0);
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(fx, 8, &[]).u32(), 0);
        // The reverb, on a second buffer.
        let (_, b2) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let d = effect_desc(&mut e, &GUID_I3DL2);
        assert_eq!(e.vcall(b2, 0x54, &[1, d, 0]).u32(), 0x8004_0154);
        e.mem.write(0x1c10, &GUID_I3DL2);
        e.mem.write(0x1c20, &IID_I3DL2_8);
        assert_eq!(
            e.vcall(b2, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(),
            0x8878_1161
        );
        // Without DSBCAPS_CTRLFX.
        let (_, b3) = create(&mut e, ds, 0xe0, false, 1, 200);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        e.mem.set_u32(0x1c00, 0xffff_ffff);
        assert_eq!(
            e.vcall(b3, 0x54, &[1, d, 0x1c00]).u32(),
            DSERR_CONTROLUNAVAIL
        );
        assert_eq!(e.mem.u32(0x1c00), 0xffff_ffff);
    }

    fn position(e: &mut Engine, b: u32) -> (u32, u32, u32) {
        e.vcall(b, 0x10, &[0x1d00, 0x1d04]);
        e.vcall(b, 0x24, &[0x1d08]);
        (e.mem.u32(0x1d00), e.mem.u32(0x1d04), e.mem.u32(0x1d08))
    }

    #[test]
    fn the_cursors_move_by_10_ms_periods_of_the_platform_clock() {
        let mut e = engine();
        let ds = start_up(&mut e);
        // 22050 Hz mono 16-bit, one second, looping, started at 5 ms.
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 5;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 1]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0, 0, 5));
        e.clock_ms = 10; // the first period
        assert_eq!(position(&mut e, b), (0, 2 * 221, 5));
        e.clock_ms = 30; // the third: the play cursor's first frame
        assert_eq!(position(&mut e, b), (2, 2 * 662, 5));
        e.clock_ms = 40;
        assert_eq!(position(&mut e, b), (2 * 222, 2 * 883, 5));
        // Past the end of the buffer: round again.
        e.clock_ms = 1010; // 101 periods: 22270 frames
        assert_eq!(position(&mut e, b).1, 2 * (22270 + 1 - 22050));
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(
            position(&mut e, b),
            (2 * 221, 2 * 221, 0),
            "both at the write cursor"
        );
    }

    #[test]
    fn a_buffer_played_once_stops_at_its_end() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 990;
        assert_eq!(position(&mut e, b).2, 1);
        e.clock_ms = 1000; // 100 periods: 22050 frames, the whole buffer
        assert_eq!(position(&mut e, b), (0, 0, 0));
        e.vcall(b, 0x48, &[]);
        assert_eq!(
            position(&mut e, b),
            (44100, 44100, 0),
            "at the end after Stop"
        );
        assert_eq!(e.vcall(b, 0x34, &[0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0, 0, 0));
    }

    #[test]
    fn a_new_frequency_goes_on_from_where_the_buffer_is() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 1]);
        e.clock_ms = 20; // 441 frames
        e.vcall(b, 0x44, &[44100]);
        e.clock_ms = 30; // one period at 44100: 441 more
        assert_eq!(position(&mut e, b).1, 2 * (441 + 441 + 1));
    }

    /// The output on, at the test format's rate, without a device: the tests render by hand.
    fn hear(e: &mut Engine) -> Arc<Mutex<Voices>> {
        let voices = Arc::new(Mutex::new(Voices::default()));
        e.kernel.sound.mix = Some(Mix {
            voices: voices.clone(),
            rate: 22050,
            _output: None,
        });
        voices
    }

    /// Fills the buffer `b` (`bytes` long) with 16-bit `samples` through `Lock`/`Unlock`.
    fn fill(e: &mut Engine, b: u32, bytes: u32, samples: &[i16]) {
        let (p1, n1) = (0x1800, 0x1804);
        e.vcall(b, 0x2c, &[0, bytes, p1, n1, 0, 0, DSBLOCK_ENTIREBUFFER]);
        let at = e.mem.u32(p1);
        let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        e.mem.write(at, &data);
        let n = e.mem.u32(n1);
        e.vcall(b, 0x4c, &[at, n, 0, 0]);
    }

    #[test]
    fn heard_buffers_are_mixed_newest_first_with_their_settings() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let size = 44100;
        let (_, a) = create(&mut e, ds, 0x2e0, false, 1, size);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, size);
        let mut sa = vec![0i16; (size / 2) as usize];
        let mut sb = sa.clone();
        sa[..3].copy_from_slice(&[1000, -2000, 3000]);
        sb[..3].copy_from_slice(&[7, 11, -13]);
        fill(&mut e, a, size, &sa);
        fill(&mut e, b, size, &sb);
        e.vcall(a, 0x3c, &[(-600i32) as u32]);
        e.vcall(b, 0x40, &[1000]);
        e.vcall(a, 0x30, &[0, 0, 0]);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        assert_eq!(voices.lock().unwrap().playing, vec![b, a]);
        let mut out = [0f32; 6];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        let x = |v: i16| f32::from(v) / 32768.0;
        let (ga, gb) = (
            super::super::mixer::gain(-600),
            super::super::mixer::gain(-1000),
        );
        for i in 0..3 {
            // b first (newest), its left side attenuated by the pan; then a at -6 dB.
            assert_eq!(out[2 * i], x(sb[i]) * gb + x(sa[i]) * ga);
            assert_eq!(out[2 * i + 1], x(sb[i]) + x(sa[i]) * ga);
        }
        e.vcall(a, 0x48, &[]);
        assert_eq!(voices.lock().unwrap().playing, vec![b]);
        e.vcall(b, 0x8, &[]);
        let v = voices.lock().unwrap();
        assert!(v.playing.is_empty() && !v.all.contains_key(&b) && v.all.contains_key(&a));
    }

    #[test]
    #[should_panic(expected = "a buffer heard at 11025 Hz on a 22050 Hz device is not provided")]
    fn a_heard_buffer_at_another_rate_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let _ = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.vcall(b, 0x44, &[11025]);
    }

    #[test]
    fn a_stream_plays_pauses_and_stops() {
        let mut e = engine();
        let voices = hear(&mut e);
        let pcm: Vec<u8> = [1000i16, -1000, 2000, -2000, 3000, -3000]
            .iter()
            .flat_map(|s| s.to_le_bytes())
            .collect();
        play_stream(&mut e, 0x9000, pcm, 2, 16, 22050);
        let mut out = [0f32; 2];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        assert_eq!(out, [1000.0 / 32768.0, -1000.0 / 32768.0]);
        pause_stream(&mut e, 0x9000, true);
        assert!(voices.lock().unwrap().playing.is_empty());
        pause_stream(&mut e, 0x9000, false);
        let mut out = [0f32; 2];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        assert_eq!(out, [2000.0 / 32768.0, -2000.0 / 32768.0]);
        stop_stream(&mut e, 0x9000);
        assert!(voices.lock().unwrap().all.is_empty());
    }

    #[test]
    #[should_panic(expected = "a stream heard at 48000 Hz on a 22050 Hz device is not provided")]
    fn a_stream_at_another_rate_stops() {
        let mut e = engine();
        let _ = hear(&mut e);
        play_stream(&mut e, 0x9000, vec![0; 4], 2, 16, 48000);
    }

    #[test]
    #[should_panic(expected = "a 3D buffer heard on the device is not provided")]
    fn a_heard_3d_buffer_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let _ = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "the sound table disagrees")]
    fn a_table_that_disagrees_stops() {
        let text = table_text().replace(
            "DsBuffer\t2e0 0 1 1 5622 10 3\t80070057",
            "DsBuffer\t2e0 0 1 1 5622 10 3\t0",
        );
        Table::parse(&text);
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn co_create_instance_stops() {
        let mut e = engine();
        co_create_instance(
            &mut e,
            Ptr::new(0x1000),
            0,
            1,
            Ptr::new(0x1010),
            Ptr::new(0x1020),
        );
    }

    #[test]
    fn a_slot_not_provided_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(ds, 0x1c, &[]); // Compact
        }));
        assert!(stopped.is_err());
    }
}
