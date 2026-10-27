//! dsound-capture: what DirectSound 8 answers on this machine to the calls FalloutNV.exe 1.4.0.525
//! makes, as the platform's sound table (`engine_core::units::platform::dsound`), written to
//! stdout. 32-bit, as the game. The table describes the player's own sound device: it stays on
//! the player's machine (git-ignored), never in the repository. Windows only:
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dsound-capture/Cargo.toml > %LOCALAPPDATA%\nv-rs\dsound.tsv`
//! (or any file `NV_DSOUND_TABLE` names).
//!
//! What the game calls (see `crates/engine-core/src/units/platform/dsound.rs`):
//! - `BSWin32Audio::Init` `00aea230`: `DirectSoundCreate8(NULL)`, `SetCooperativeLevel(window,
//!   DSSCL_PRIORITY)`, `GetCaps`, the primary buffer (`DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D |
//!   DSBCAPS_CTRLVOLUME`), its `IDirectSound3DListener`, `SetPosition` and `SetDopplerFactor`.
//! - Sounds (`00aeb270`/`00aec530`, `00af0630`, `00af1430`): secondary buffers of the file's PCM
//!   format with `DSBCAPS_CTRLFREQUENCY | DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLFX` (the last left out
//!   under 151 ms) and either `DSBCAPS_CTRLPAN` or `DSBCAPS_CTRL3D | DSBCAPS_GETCURRENTPOSITION2 |
//!   DSBCAPS_MUTE3DATMAXDISTANCE`, `DS3DALG_HRTF_LIGHT` or none; `IDirectSoundBuffer8` and
//!   `IDirectSound3DBuffer`; `Lock`/`Unlock`; `SetFX` with the distortion or the I3DL2 reverb
//!   and `GetObjectInPath`; `Play`, `Stop`, the cursor, volume, pan and frequency.
//!
//! Lines (tab separated; words and results in hex):
//! - `DsCreate`, `DsCooperativeLevel`, `DsCaps` (the 0x60-byte `DSCAPS`), `DsPrimary`,
//!   `DsListener` (the `QueryInterface` result, then `GetAllParameters`' 0x40 bytes):
//!   `name<TAB>words<TAB>result<TAB>bytes`.
//! - `DsBuffer<TAB>flags alg tag channels rate bits bytes<TAB>result<TAB>DSBCAPS bytes<TAB>format
//!   bytes<TAB>frequency status play write<TAB>QI IDirectSoundBuffer8 result, QI
//!   IDirectSound3DBuffer result`.
//! - `DsFx<TAB>effect flags format bytes<TAB>SetFX result<TAB>its result code<TAB>
//!   GetObjectInPath result<TAB>GetAllParameters bytes`.
//! - `DsSet<TAB>method value<TAB>result<TAB>value read back`.
//! - `DsCursor<TAB>rate channels bits bytes looping frequency<TAB>samples` where the samples are
//!   `microseconds:play:write` at each change, from `Play`.
//! - `DsEnd`.
use std::ffi::c_void;
use std::io::Write;

#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn Sleep(ms: u32);
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassA(class: *const WndClassA) -> u16;
    #[allow(clippy::too_many_arguments)]
    fn CreateWindowExA(
        ex_style: u32,
        class: *const u8,
        title: *const u8,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: *mut c_void,
        menu: *mut c_void,
        instance: *mut c_void,
        param: *mut c_void,
    ) -> *mut c_void;
    fn DefWindowProcA(window: *mut c_void, message: u32, w: usize, l: isize) -> isize;
}

#[repr(C)]
struct WndClassA {
    style: u32,
    procedure: unsafe extern "system" fn(*mut c_void, u32, usize, isize) -> isize,
    class_extra: i32,
    window_extra: i32,
    instance: *mut c_void,
    icon: *mut c_void,
    cursor: *mut c_void,
    background: *mut c_void,
    menu_name: *const u8,
    class_name: *const u8,
}

/// A COM object: a pointer to its vtable.
#[repr(C)]
struct Obj {
    vtable: *const usize,
}

/// The function in slot `slot` (byte offset / 4) of `o`'s vtable, as `F`.
unsafe fn slot<F: Copy>(o: *mut Obj, offset: usize) -> F {
    let f = *(*o).vtable.add(offset / 4);
    std::mem::transmute_copy(&f)
}

type Fn0 = unsafe extern "system" fn(*mut Obj) -> u32;
type Fn1 = unsafe extern "system" fn(*mut Obj, usize) -> u32;
type Fn2 = unsafe extern "system" fn(*mut Obj, usize, usize) -> u32;
type Fn3 = unsafe extern "system" fn(*mut Obj, usize, usize, usize) -> u32;
type Fn4 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize) -> u32;
type Fn7 =
    unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize, usize, usize) -> u32;
type FnF1 = unsafe extern "system" fn(*mut Obj, f32, u32) -> u32;
type FnF3 = unsafe extern "system" fn(*mut Obj, f32, f32, f32, u32) -> u32;

const IID_BUFFER8: [u8; 16] = [
    0x49, 0xa4, 0x25, 0x68, 0x24, 0x75, 0x82, 0x4d, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e,
];
const IID_3DBUFFER: [u8; 16] = [
    0x86, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_LISTENER: [u8; 16] = [
    0x84, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
/// The GUIDs FalloutNV.exe passes (read from its `.rdata`: `01100ff4`, `01100fc4`, `01101004`,
/// `01100fd4`; the HRTF light algorithm from `00aeb270`'s stores).
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
const ALG_HRTF_LIGHT: [u32; 4] = [0xc241_3342, 0x11d2_1c1b, 0xc000_f594, 0xca8a_c24f];

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

unsafe fn bytes_of<T>(t: &T) -> Vec<u8> {
    std::slice::from_raw_parts(t as *const T as *const u8, std::mem::size_of::<T>()).to_vec()
}

unsafe extern "system" fn wndproc(w: *mut c_void, m: u32, wp: usize, lp: isize) -> isize {
    DefWindowProcA(w, m, wp, lp)
}

unsafe fn window() -> *mut c_void {
    let name = b"dsound-capture\0";
    let class = WndClassA {
        style: 0,
        procedure: wndproc,
        class_extra: 0,
        window_extra: 0,
        instance: std::ptr::null_mut(),
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    RegisterClassA(&class);
    CreateWindowExA(
        0,
        name.as_ptr(),
        name.as_ptr(),
        0,
        0,
        0,
        64,
        64,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

#[repr(C)]
struct WaveFormat {
    tag: u16,
    channels: u16,
    rate: u32,
    avg: u32,
    align: u16,
    bits: u16,
    extra: u16,
}

#[repr(C)]
struct BufferDesc {
    size: u32,
    flags: u32,
    bytes: u32,
    reserved: u32,
    format: *const WaveFormat,
    alg: [u32; 4],
}

fn format(channels: u16, rate: u32, bits: u16) -> WaveFormat {
    let align = channels * bits / 8;
    WaveFormat {
        tag: 1,
        channels,
        rate,
        avg: rate * align as u32,
        align,
        bits,
        extra: 0,
    }
}

/// A secondary buffer as the game asks for it; the result and the buffer (null on failure).
unsafe fn create(
    ds: *mut Obj,
    flags: u32,
    alg: bool,
    f: &WaveFormat,
    bytes: u32,
) -> (u32, *mut Obj) {
    let desc = BufferDesc {
        size: 0x24,
        flags,
        bytes,
        reserved: 0,
        format: f,
        alg: if alg { ALG_HRTF_LIGHT } else { [0; 4] },
    };
    let mut b: *mut Obj = std::ptr::null_mut();
    let hr = slot::<Fn3>(ds, 0xc)(ds, &desc as *const _ as usize, &mut b as *mut _ as usize, 0);
    (
        hr,
        if hr as i32 >= 0 {
            b
        } else {
            std::ptr::null_mut()
        },
    )
}

unsafe fn release(o: *mut Obj) {
    if !o.is_null() {
        slot::<Fn0>(o, 8)(o);
    }
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> (u32, *mut Obj) {
    let mut out: *mut Obj = std::ptr::null_mut();
    let hr = slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    (hr, out)
}

fn now_us(freq: i64, origin: i64) -> i64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    (c - origin) * 1_000_000 / freq
}

/// The flag sets the game uses: 2D (`CTRLPAN`) and 3D, each with and without `CTRLFX`.
const FLAG_SETS: [u32; 4] = [0x2e0, 0xe0, 0x302b0, 0x300b0];
const RATES: [u32; 7] = [8000, 11025, 16000, 22050, 32000, 44100, 48000];

unsafe fn buffers(o: &mut impl Write, ds: *mut Obj) {
    for flags in FLAG_SETS {
        let algs: &[bool] = if flags & 0x10 != 0 {
            &[false, true]
        } else {
            &[false]
        };
        for &alg in algs {
            for channels in [1u16, 2] {
                for rate in RATES {
                    for bits in [8u16, 16] {
                        let f = format(channels, rate, bits);
                        let a = f.align as u32;
                        let ms = |m: u32| (f.avg * m / 1000) / a * a;
                        let mut sizes = vec![0, 1, a, 2 * a, 3, 4, 100 * a, ms(1000), ms(250)];
                        // Around 150 ms, block by block.
                        let c = ms(150);
                        for d in 0..6u32 {
                            sizes.push(c + d * a);
                            sizes.push(c.saturating_sub((d + 1) * a));
                        }
                        sizes.sort_unstable();
                        sizes.dedup();
                        for bytes in sizes {
                            buffer(o, ds, flags, alg, &f, bytes);
                        }
                    }
                }
            }
        }
    }
}

unsafe fn buffer(
    o: &mut impl Write,
    ds: *mut Obj,
    flags: u32,
    alg: bool,
    f: &WaveFormat,
    bytes: u32,
) {
    let (hr, b) = create(ds, flags, alg, f, bytes);
    let words = format!(
        "{flags:x} {} {:x} {:x} {:x} {:x} {bytes:x}",
        u32::from(alg),
        f.tag,
        f.channels,
        f.rate,
        f.bits
    );
    if b.is_null() {
        writeln!(o, "DsBuffer\t{words}\t{hr:x}\t\t\t\t").unwrap();
        return;
    }
    let mut caps = [0u32; 5];
    caps[0] = 0x14;
    slot::<Fn1>(b, 0xc)(b, caps.as_mut_ptr() as usize);
    let mut fmt = [0u8; 0x12];
    let mut written = 0u32;
    slot::<Fn3>(b, 0x14)(
        b,
        fmt.as_mut_ptr() as usize,
        0x12,
        &mut written as *mut _ as usize,
    );
    let mut freq = 0u32;
    slot::<Fn1>(b, 0x20)(b, &mut freq as *mut _ as usize);
    let mut status = 0u32;
    slot::<Fn1>(b, 0x24)(b, &mut status as *mut _ as usize);
    let (mut play, mut write) = (0u32, 0u32);
    slot::<Fn2>(b, 0x10)(
        b,
        &mut play as *mut _ as usize,
        &mut write as *mut _ as usize,
    );
    let (q8, b8) = qi(b, &IID_BUFFER8);
    let (q3, b3) = qi(b, &IID_3DBUFFER);
    let caps_hex = hex(&bytes_of(&caps));
    writeln!(
        o,
        "DsBuffer\t{words}\t{hr:x}\t{caps_hex}\t{} {written:x}\t{freq:x} {status:x} {play:x} {write:x}\t{q8:x} {q3:x}",
        hex(&fmt)
    )
    .unwrap();
    release(b3);
    release(b8);
    release(b);
}

/// `DSEFFECTDESC` as the game fills it.
#[repr(C)]
struct EffectDesc {
    size: u32,
    flags: u32,
    class: [u8; 16],
    reserved1: u32,
    reserved2: u32,
}

unsafe fn effects(o: &mut impl Write, ds: *mut Obj) {
    for (name, class, iid, params) in [
        ("distortion", GUID_DISTORTION, IID_DISTORTION8, 0x14usize),
        ("i3dl2", GUID_I3DL2, IID_I3DL2_8, 0x30usize),
    ] {
        for flags in [0x2e0u32, 0x302b0, 0xe0] {
            for (channels, rate) in [(1u16, 22050u32), (2, 44100), (1, 44100), (1, 48000)] {
                let f = format(channels, rate, 16);
                let bytes = f.avg; // one second
                let (hr, b) = create(ds, flags, false, &f, bytes);
                if b.is_null() {
                    writeln!(o, "DsFx\t{name} {flags:x} {channels:x} {rate:x} {bytes:x}\tcreate {hr:x}\t\t\t").unwrap();
                    continue;
                }
                let (_, b8) = qi(b, &IID_BUFFER8);
                let desc = EffectDesc {
                    size: 0x20,
                    flags: 0,
                    class,
                    reserved1: 0,
                    reserved2: 0,
                };
                let mut code = 0xffff_ffffu32;
                let r = slot::<Fn3>(b8, 0x54)(
                    b8,
                    1,
                    &desc as *const _ as usize,
                    &mut code as *mut _ as usize,
                );
                let mut fx: *mut Obj = std::ptr::null_mut();
                let g = slot::<Fn4>(b8, 0x5c)(
                    b8,
                    class.as_ptr() as usize,
                    0,
                    iid.as_ptr() as usize,
                    &mut fx as *mut _ as usize,
                );
                let mut all = vec![0u8; params];
                if !fx.is_null() {
                    slot::<Fn1>(fx, 0x10)(fx, all.as_mut_ptr() as usize);
                }
                writeln!(
                    o,
                    "DsFx\t{name} {flags:x} {channels:x} {rate:x} {bytes:x}\t{r:x}\t{code:x}\t{g:x}\t{}",
                    hex(&all)
                )
                .unwrap();
                if !fx.is_null() {
                    fx_settings(o, name, fx);
                    release(fx);
                }
                release(b8);
                release(b);
            }
        }
    }
}

/// The effect settings the game makes: the distortion's `SetAllParameters` (gain from the INI,
/// edge 0, post-EQ 5000 Hz / 1135 Hz, pre-lowpass 5000 Hz) over a range of gains, and the
/// reverb's `SetPreset` over every preset number and a few past them.
unsafe fn fx_settings(o: &mut impl Write, name: &str, fx: *mut Obj) {
    if name == "distortion" {
        for gain in [-61.0f32, -60.0, -40.0, -18.0, -10.0, 0.0, 0.5] {
            let p: [f32; 5] = [gain, 0.0, 5000.0, 1135.0, 5000.0];
            let r = slot::<Fn1>(fx, 0xc)(fx, p.as_ptr() as usize);
            let mut back = [0u8; 0x14];
            slot::<Fn1>(fx, 0x10)(fx, back.as_mut_ptr() as usize);
            writeln!(
                o,
                "DsSet\tdistortion {:08x}\t{r:x}\t{}",
                gain.to_bits(),
                hex(&back)
            )
            .unwrap();
        }
    } else {
        for preset in 0..32u32 {
            let r = slot::<Fn1>(fx, 0x14)(fx, preset as usize);
            let mut back = [0u8; 0x30];
            slot::<Fn1>(fx, 0x10)(fx, back.as_mut_ptr() as usize);
            writeln!(o, "DsSet\ti3dl2preset {preset:x}\t{r:x}\t{}", hex(&back)).unwrap();
        }
    }
}

/// Volume, pan and frequency at the edges of their ranges, on a 2D and a 3D buffer.
unsafe fn settings(o: &mut impl Write, ds: *mut Obj) {
    for flags in [0x2e0u32, 0x302b0] {
        let f = format(1, 22050, 16);
        let (_, b) = create(ds, flags, false, &f, f.avg);
        for v in [-10001i32, -10000, -2500, -1, 0, 1] {
            let r = slot::<Fn1>(b, 0x3c)(b, v as usize);
            let mut back = 0i32;
            slot::<Fn1>(b, 0x18)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} volume {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        for v in [-10001i32, -10000, 0, 10000, 10001] {
            let r = slot::<Fn1>(b, 0x40)(b, v as usize);
            let mut back = 0i32;
            slot::<Fn1>(b, 0x1c)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} pan {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        for v in [0u32, 1, 99, 100, 11025, 22050, 100000, 200000, 200001] {
            let r = slot::<Fn1>(b, 0x44)(b, v as usize);
            let mut back = 0u32;
            slot::<Fn1>(b, 0x20)(b, &mut back as *mut _ as usize);
            writeln!(o, "DsSet\t{flags:x} frequency {v:x}\t{r:x}\t{back:x}").unwrap();
        }
        // Lock as the game does: (0, size, &p, &n, NULL, NULL, 0) and the whole buffer.
        let (mut p1, mut n1, mut p2, mut n2) = (0usize, 0u32, 0usize, 0u32);
        let r = slot::<Fn7>(b, 0x2c)(
            b,
            0,
            f.avg as usize,
            &mut p1 as *mut _ as usize,
            &mut n1 as *mut _ as usize,
            0,
            0,
            0,
        );
        writeln!(o, "DsSet\t{flags:x} lock 0 {:x} 0\t{r:x}\t{n1:x}", f.avg).unwrap();
        let u = slot::<Fn4>(b, 0x4c)(b, p1, n1 as usize, 0, 0);
        writeln!(o, "DsSet\t{flags:x} unlock\t{u:x}\t").unwrap();
        let r = slot::<Fn7>(b, 0x2c)(
            b,
            0,
            0,
            &mut p1 as *mut _ as usize,
            &mut n1 as *mut _ as usize,
            &mut p2 as *mut _ as usize,
            &mut n2 as *mut _ as usize,
            2,
        );
        writeln!(
            o,
            "DsSet\t{flags:x} lock 0 0 2\t{r:x}\t{n1:x} {} {n2:x}",
            u32::from(p2 != 0)
        )
        .unwrap();
        slot::<Fn4>(b, 0x4c)(b, p1, n1 as usize, p2, n2 as usize);
        if flags & 0x10 != 0 {
            let (q, b3) = qi(b, &IID_3DBUFFER);
            let mut all = [0u8; 0x40];
            all[0] = 0x40;
            let g = slot::<Fn1>(b3, 0xc)(b3, all.as_mut_ptr() as usize);
            writeln!(o, "DsSet\t3dbuffer defaults\t{q:x} {g:x}\t{}", hex(&all)).unwrap();
            for (name, offset, v) in [
                ("mindistance", 0x44usize, 1.0e9f32),
                ("maxdistance", 0x40, 1.0e9),
                ("mindistance", 0x44, 0.0),
                ("maxdistance", 0x40, -1.0),
            ] {
                let r = slot::<FnF1>(b3, offset)(b3, v, 1);
                writeln!(o, "DsSet\t3dbuffer {name} {:08x}\t{r:x}\t", v.to_bits()).unwrap();
            }
            let r = slot::<FnF3>(b3, 0x4c)(b3, -3.4e37, -3.4e37, -3.4e37, 1);
            writeln!(o, "DsSet\t3dbuffer position fdcccccc\t{r:x}\t").unwrap();
            release(b3);
        }
        release(b);
    }
}

/// The play and write cursors from `Play` on, at each change, for about 600 ms.
unsafe fn cursors(o: &mut impl Write, ds: *mut Obj) {
    let mut freq = 0i64;
    QueryPerformanceFrequency(&mut freq);
    for (channels, rate, bits, ms, looping, frequency) in [
        (1u16, 22050u32, 16u16, 1000u32, 1u32, 0u32),
        (2, 44100, 16, 1000, 1, 0),
        (1, 44100, 16, 1000, 1, 0),
        (2, 48000, 16, 1000, 1, 0),
        (1, 11025, 8, 1000, 1, 0),
        (1, 22050, 16, 300, 0, 0),
        (1, 22050, 16, 1000, 1, 11025),
        (1, 22050, 16, 1000, 1, 44100),
    ] {
        let f = format(channels, rate, bits);
        let bytes = f.avg * ms / 1000 / f.align as u32 * f.align as u32;
        let (_, b) = create(ds, 0x100e0 | 0x200, false, &f, bytes);
        if frequency != 0 {
            slot::<Fn1>(b, 0x44)(b, frequency as usize);
        }
        let mut origin = 0i64;
        let mut samples = Vec::new();
        let (mut last_p, mut last_w, mut last_s) = (u32::MAX, u32::MAX, u32::MAX);
        QueryPerformanceCounter(&mut origin);
        slot::<Fn3>(b, 0x30)(b, 0, 0, looping as usize);
        loop {
            let t = now_us(freq, origin);
            if t > 600_000 {
                break;
            }
            let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
            slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
            slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
            if (p, w, s) != (last_p, last_w, last_s) {
                samples.push(format!("{t}:{p:x}:{w:x}:{s:x}"));
                (last_p, last_w, last_s) = (p, w, s);
            }
        }
        slot::<Fn0>(b, 0x48)(b);
        let (mut p, mut w, mut s) = (0u32, 0u32, 0u32);
        slot::<Fn2>(b, 0x10)(b, &mut p as *mut _ as usize, &mut w as *mut _ as usize);
        slot::<Fn1>(b, 0x24)(b, &mut s as *mut _ as usize);
        writeln!(
            o,
            "DsCursor\t{rate:x} {channels:x} {bits:x} {bytes:x} {looping:x} {frequency:x}\t{}\tstopped {p:x}:{w:x}:{s:x}",
            samples.join(" ")
        )
        .unwrap();
        release(b);
        Sleep(50);
    }
}

fn main() {
    let mut o = std::io::stdout().lock();
    unsafe {
        let window = window();
        let mut ds: *mut Obj = std::ptr::null_mut();
        let hr = DirectSoundCreate8(std::ptr::null(), &mut ds, std::ptr::null_mut());
        writeln!(o, "DsCreate\t\t{hr:x}\t").unwrap();
        if ds.is_null() {
            writeln!(o, "DsEnd").unwrap();
            return;
        }
        let hr = slot::<Fn2>(ds, 0x18)(ds, window as usize, 2);
        writeln!(o, "DsCooperativeLevel\t2\t{hr:x}\t").unwrap();
        let mut caps = [0u32; 0x18];
        caps[0] = 0x60;
        let hr = slot::<Fn1>(ds, 0x10)(ds, caps.as_mut_ptr() as usize);
        writeln!(o, "DsCaps\t\t{hr:x}\t{}", hex(&bytes_of(&caps))).unwrap();
        let primary = BufferDesc {
            size: 0x24,
            flags: 0x91,
            bytes: 0,
            reserved: 0,
            format: std::ptr::null(),
            alg: [0; 4],
        };
        let mut pb: *mut Obj = std::ptr::null_mut();
        let hr = slot::<Fn3>(ds, 0xc)(
            ds,
            &primary as *const _ as usize,
            &mut pb as *mut _ as usize,
            0,
        );
        let mut pfmt = [0u8; 0x12];
        let mut written = 0u32;
        if !pb.is_null() {
            slot::<Fn3>(pb, 0x14)(
                pb,
                pfmt.as_mut_ptr() as usize,
                0x12,
                &mut written as *mut _ as usize,
            );
        }
        writeln!(o, "DsPrimary\t91\t{hr:x}\t{} {written:x}", hex(&pfmt)).unwrap();
        if !pb.is_null() {
            let (q, l) = qi(pb, &IID_LISTENER);
            let mut all = [0u8; 0x40];
            all[0] = 0x40;
            let g = if l.is_null() {
                0xffff_ffff
            } else {
                slot::<Fn1>(l, 0xc)(l, all.as_mut_ptr() as usize)
            };
            writeln!(o, "DsListener\t\t{q:x} {g:x}\t{}", hex(&all)).unwrap();
            if !l.is_null() {
                let r = slot::<FnF1>(l, 0x30)(l, 1.0, 0);
                writeln!(o, "DsSet\tlistener doppler 3f800000\t{r:x}\t").unwrap();
                release(l);
            }
        }
        buffers(&mut o, ds);
        effects(&mut o, ds);
        settings(&mut o, ds);
        cursors(&mut o, ds);
        release(pb);
        release(ds);
        writeln!(o, "DsEnd").unwrap();
    }
}
