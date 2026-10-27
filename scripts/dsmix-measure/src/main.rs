//! dsmix-measure: what DirectSound 8 sends to the sound device for a known buffer and known
//! settings, captured with WASAPI loopback on the default output device, for the platform's
//! mixer (`engine_core::units::platform::mixer`). 32-bit, as the game. The test sounds are
//! made up (noise from a fixed generator, quiet: about -30 dBFS) and short; they are heard on
//! the speakers while it runs. Writes `<out-dir>/<case>.f32` (the device's samples as captured,
//! interleaved) and `<out-dir>/cases.tsv` (the case, its settings, the device format, frames
//! captured). Nothing else is written.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dsmix-measure/Cargo.toml -- <out-dir> [endpoint-id description]`
//!
//! With an endpoint (the GUID part of its id) and the DirectSound description of the same
//! device, it plays on and records that device instead of the default one: a device without
//! audio enhancements is needed for bit-exact captures (the default one here smeared the
//! signal). Modes, by environment variable: none (the base cases), `DSMIX_SWEEP` (volumes, pans,
//! 3D angles and distances), `DSMIX_DENSE` (every 7th volume), `DSMIX_3D` (a 3D position grid),
//! `DSMIX_MULTI` (several voices at once: the order of the sum).
use std::ffi::c_void;
use std::io::Write;

#[link(name = "dsound")]
extern "system" {
    fn DirectSoundCreate8(device: *const u8, out: *mut *mut Obj, outer: *mut c_void) -> u32;
}
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
    fn CoCreateInstance(clsid: *const [u8; 16], outer: *mut c_void, context: u32, iid: *const [u8; 16], out: *mut *mut Obj) -> u32;
}
#[link(name = "kernel32")]
extern "system" {
    fn Sleep(ms: u32);
}
#[link(name = "user32")]
extern "system" {
    fn GetDesktopWindow() -> *mut c_void;
}

#[repr(C)]
struct Obj {
    vtable: *const usize,
}

unsafe fn slot<F: Copy>(o: *mut Obj, offset: usize) -> F {
    let f = *(*o).vtable.add(offset / 4);
    std::mem::transmute_copy(&f)
}

type Fn0 = unsafe extern "system" fn(*mut Obj) -> u32;
type Fn1 = unsafe extern "system" fn(*mut Obj, usize) -> u32;
type Fn2 = unsafe extern "system" fn(*mut Obj, usize, usize) -> u32;
type Fn3 = unsafe extern "system" fn(*mut Obj, usize, usize, usize) -> u32;
type Fn4 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize) -> u32;
type Fn5 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize) -> u32;
type Fn7 = unsafe extern "system" fn(*mut Obj, usize, usize, usize, usize, usize, usize, usize) -> u32;
type FnInit = unsafe extern "system" fn(*mut Obj, u32, u32, i64, i64, *const u8, *const c_void) -> u32;
type FnF1 = unsafe extern "system" fn(*mut Obj, f32, u32) -> u32;
type FnF3 = unsafe extern "system" fn(*mut Obj, f32, f32, f32, u32) -> u32;

fn guid(s: &str) -> [u8; 16] {
    let h: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    let b = |i: usize| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap();
    [b(3), b(2), b(1), b(0), b(5), b(4), b(7), b(6), b(8), b(9), b(10), b(11), b(12), b(13), b(14), b(15)]
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
    let mut out: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    out
}

/// The test signal: noise from a fixed 32-bit generator, `frames` frames of `channels`
/// channels, `bits` bits, about -30 dBFS (16-bit: +-1000; 8-bit: 128 +- 4).
pub fn signal(frames: usize, channels: usize, bits: u16) -> Vec<u8> {
    signal_seeded(frames, channels, bits, 0x1234_5678)
}

/// [`signal`] from another seed (the voices played together in `DSMIX_MULTI`).
pub fn signal_seeded(frames: usize, channels: usize, bits: u16, seed: u32) -> Vec<u8> {
    let mut x = seed;
    let mut out = Vec::new();
    for _ in 0..frames * channels {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let v = (x % 2001) as i32 - 1000;
        if bits == 16 {
            out.extend_from_slice(&(v as i16).to_le_bytes());
        } else {
            out.push((128 + v / 250) as u8);
        }
    }
    out
}

struct Case {
    name: &'static str,
    channels: u16,
    rate: u32,
    bits: u16,
    flags: u32,
    volume: i32,
    pan: i32,
    frequency: u32,
    /// 3D: position, velocity, min distance, max distance; listener rolloff, Doppler.
    three_d: Option<([f32; 3], [f32; 3], f32, f32, f32, f32)>,
    distortion: Option<[f32; 5]>,
}

fn cases() -> Vec<Case> {
    let base = Case {
        name: "",
        channels: 1,
        rate: 48000,
        bits: 16,
        flags: 0xe0 | 0x8000,
        volume: 0,
        pan: 0,
        frequency: 0,
        three_d: None,
        distortion: None,
    };
    let mut v = Vec::new();
    let mut add = |name: &'static str, f: &dyn Fn(&mut Case)| {
        let mut c = Case { name, ..base };
        f(&mut c);
        v.push(c);
    };
    add("mono48", &|_| {});
    add("stereo48", &|c| c.channels = 2);
    add("mono48_8bit", &|c| c.bits = 8);
    for (n, vol) in [("vol_m100", -100), ("vol_m600", -600), ("vol_m2000", -2000), ("vol_m6000", -6000)] {
        add(n, &move |c| c.volume = vol);
    }
    for (n, pan) in [("pan_m10000", -10000), ("pan_m5000", -5000), ("pan_m1000", -1000), ("pan_p1000", 1000), ("pan_p5000", 5000), ("pan_p10000", 10000)] {
        add(n, &move |c| c.pan = pan);
    }
    add("stereo48_pan_p1000", &|c| {
        c.channels = 2;
        c.pan = 1000
    });
    for (n, rate) in [("mono22050", 22050), ("mono44100", 44100), ("mono11025", 11025), ("mono32000", 32000)] {
        add(n, &move |c| c.rate = rate);
    }
    for (n, f) in [("freq48_to_24000", 24000), ("freq48_to_47000", 47000), ("freq48_to_60000", 60000)] {
        add(n, &move |c| c.frequency = f);
    }
    for (n, pos, vel) in [
        ("3d_front_1", [0.0, 0.0, 1.0], [0.0; 3]),
        ("3d_right_1", [1.0, 0.0, 0.0], [0.0; 3]),
        ("3d_left_1", [-1.0, 0.0, 0.0], [0.0; 3]),
        ("3d_behind_1", [0.0, 0.0, -1.0], [0.0; 3]),
        ("3d_front_10", [0.0, 0.0, 10.0], [0.0; 3]),
        ("3d_right_3", [3.0, 0.0, 0.0], [0.0; 3]),
        ("3d_front_right_2", [1.4142135, 0.0, 1.4142135], [0.0; 3]),
        ("3d_above_1", [0.0, 1.0, 0.0], [0.0; 3]),
        ("3d_doppler", [0.0, 0.0, 5.0], [0.0, 0.0, -50.0]),
    ] {
        add(n, &move |c| {
            c.flags = 0x300b0 | 0x8000;
            c.three_d = Some((pos, vel, 1.0, 1.0e9, 1.0, 1.0));
        });
    }
    add("3d_front_10_min5", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 5.0, 1.0e9, 1.0, 1.0));
    });
    add("3d_front_10_rolloff2", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 1.0, 1.0e9, 2.0, 1.0));
    });
    add("3d_front_10_max4", &|c| {
        c.flags = 0x300b0 | 0x8000;
        c.three_d = Some(([0.0, 0.0, 10.0], [0.0; 3], 1.0, 4.0, 1.0, 1.0));
    });
    for (n, gain) in [("distortion_m18", -18.0f32), ("distortion_m40", -40.0), ("distortion_0", 0.0)] {
        add(n, &move |c| {
            c.flags = 0x2e0 | 0x8000;
            c.distortion = Some([gain, 0.0, 5000.0, 1135.0, 5000.0]);
        });
    }
    add("distortion_m18_edge50", &|c| {
        c.flags = 0x2e0 | 0x8000;
        c.distortion = Some([-18.0, 50.0, 2400.0, 2400.0, 8000.0]);
    });
    if std::env::var_os("DSMIX_SWEEP").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for vol in [-1, -5, -9, -10, -11, -49, -50, -51, -99, -101, -150, -199, -200, -555, -1000, -1234, -3000, -5000, -8000, -9000, -9999, -10000] {
            add(leak(format!("sweep_vol_{vol}")), &move |c| c.volume = vol);
        }
        for pan in [-1, -9, -10, -11, -50, -99, -100, -101, -250, -2000, -3000, -4000, -4500, -4900, -5001, -6000, -9999] {
            add(leak(format!("sweep_pan_{pan}")), &move |c| c.pan = pan);
        }
        add("sweep_vol_m300_pan_m700", &|c| {
            c.volume = -300;
            c.pan = -700
        });
        for deg in (0..=180).step_by(15) {
            let a = (deg as f32).to_radians();
            add(leak(format!("sweep_az_{deg}")), &move |c| {
                c.flags = 0x300b0 | 0x8000;
                c.three_d = Some(([a.sin(), 0.0, a.cos()], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
            });
        }
        for (d, min, roll) in [(0.5f32, 1.0f32, 1.0f32), (1.5, 1.0, 1.0), (2.0, 1.0, 1.0), (3.0, 1.0, 1.0), (7.0, 1.0, 1.0), (100.0, 1.0, 1.0), (2.0, 1.0, 0.5), (2.0, 1.0, 0.0), (20.0, 3.0, 1.0), (4.0, 1.0, 1.5)] {
            add(leak(format!("sweep_dist_{d}_{min}_{roll}")), &move |c| {
                c.flags = 0x300b0 | 0x8000;
                c.three_d = Some(([0.0, 0.0, d], [0.0; 3], min, 1.0e9, roll, 1.0));
            });
        }
        add("sweep_az_30_vol_m500", &|c| {
            c.flags = 0x300b0 | 0x8000;
            c.volume = -500;
            c.three_d = Some(([0.5, 0.0, 0.8660254], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
        });
    }
    if std::env::var_os("DSMIX_3D").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for xi in -8..=8 {
            for zi in -4..=4 {
                for yi in [0, 2] {
                    let (x, y, z) = (xi as f32 * 0.5, yi as f32 * 0.5, zi as f32);
                    if x == 0.0 && y == 0.0 && z == 0.0 {
                        continue;
                    }
                    add(leak(format!("sweep_dense_3d_{x}_{y}_{z}")), &move |c| {
                        c.flags = 0x300b0 | 0x8000;
                        c.three_d = Some(([x, y, z], [0.0; 3], 1.0, 1.0e9, 1.0, 1.0));
                    });
                }
            }
        }
    }
    if std::env::var_os("DSMIX_DENSE").is_some() {
        let leak = |s: String| -> &'static str { Box::leak(s.into_boxed_str()) };
        for vol in (1..=3000).step_by(7).map(|x: i32| -x) {
            add(leak(format!("sweep_dense_vol_{vol}")), &move |c| c.volume = vol);
        }
    }
    if std::env::var_os("DSMIX_SWEEP").is_some() {
        v.retain(|c| c.name.starts_with("sweep_"));
    }
    v
}

/// The default output device's loopback capture client: (audio client, capture client, the
/// mix format's bytes).
unsafe fn loopback(endpoint: Option<&str>) -> (*mut Obj, *mut Obj, Vec<u8>) {
    let mut en: *mut Obj = std::ptr::null_mut();
    let r = CoCreateInstance(
        &guid("bcde0395-e52f-467c-8e3d-c4579291692e"),
        std::ptr::null_mut(),
        0x17,
        &guid("a95664d2-9614-4f35-a746-de8db63617e6"),
        &mut en,
    );
    assert_eq!(r, 0, "MMDeviceEnumerator");
    let mut dev: *mut Obj = std::ptr::null_mut();
    match endpoint {
        Some(id) => {
            let wide: Vec<u16> = format!("{{0.0.0.00000000}}.{{{id}}}").encode_utf16().chain([0]).collect();
            let r = slot::<Fn2>(en, 0x14)(en, wide.as_ptr() as usize, &mut dev as *mut _ as usize);
            assert_eq!(r, 0, "no endpoint {id}");
        }
        None => {
            slot::<Fn3>(en, 0x10)(en, 0, 0, &mut dev as *mut _ as usize);
        }
    }
    let mut client: *mut Obj = std::ptr::null_mut();
    let iid_client = guid("1cb9ad4c-dbfa-4c32-b178-c2f568a703b2");
    slot::<Fn4>(dev, 0xc)(dev, iid_client.as_ptr() as usize, 0x17, 0, &mut client as *mut _ as usize);
    let mut fmt: *const u8 = std::ptr::null();
    slot::<Fn1>(client, 0x20)(client, &mut fmt as *mut _ as usize);
    let size = 18 + u16::from_le_bytes([*fmt.add(16), *fmt.add(17)]) as usize;
    let fmt_bytes = std::slice::from_raw_parts(fmt, size).to_vec();
    let r = slot::<FnInit>(client, 0xc)(client, 0, 0x2_0000, 20_000_000, 0, fmt, std::ptr::null());
    assert_eq!(r, 0, "IAudioClient::Initialize loopback");
    let mut cap: *mut Obj = std::ptr::null_mut();
    let iid_cap = guid("c8adbd64-e71e-48a0-a4de-185c395cd317");
    slot::<Fn2>(client, 0x38)(client, iid_cap.as_ptr() as usize, &mut cap as *mut _ as usize);
    (client, cap, fmt_bytes)
}

/// Drains the capture client into `out` (bytes); silent packets add zeros.
unsafe fn drain(cap: *mut Obj, frame_bytes: usize, out: &mut Vec<u8>) {
    loop {
        let mut n = 0u32;
        slot::<Fn1>(cap, 0x14)(cap, &mut n as *mut _ as usize);
        if n == 0 {
            return;
        }
        let (mut data, mut frames, mut flags) = (std::ptr::null_mut::<u8>(), 0u32, 0u32);
        slot::<Fn5>(cap, 0xc)(cap, &mut data as *mut _ as usize, &mut frames as *mut _ as usize, &mut flags as *mut _ as usize, 0, 0);
        let len = frames as usize * frame_bytes;
        if flags & 2 != 0 {
            out.extend(std::iter::repeat(0u8).take(len));
        } else {
            out.extend_from_slice(std::slice::from_raw_parts(data, len));
        }
        slot::<Fn1>(cap, 0x10)(cap, frames as usize);
    }
}

fn main() {
    let dir = std::path::PathBuf::from(std::env::args().nth(1).expect("the output folder"));
    std::fs::create_dir_all(&dir).unwrap();
    let mut tsv = std::fs::File::create(dir.join("cases.tsv")).unwrap();
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        let endpoint = std::env::args().nth(2);
        let (client, cap, fmt) = loopback(endpoint.as_deref());
        let channels = u16::from_le_bytes([fmt[2], fmt[3]]) as usize;
        let frame_bytes = u16::from_le_bytes([fmt[12], fmt[13]]) as usize;
        writeln!(tsv, "format\t{}", fmt.iter().map(|b| format!("{b:02x}")).collect::<String>()).unwrap();
        let mut ds: *mut Obj = std::ptr::null_mut();
        let device = match &endpoint {
            Some(id) => Some(direct_sound_guid(id)),
            None => None,
        };
        let r = DirectSoundCreate8(device.as_ref().map_or(std::ptr::null(), |g| g.as_ptr()), &mut ds, std::ptr::null_mut());
        writeln!(tsv, "device	{endpoint:?}	{device:x?}	{r:x}").unwrap();
        slot::<Fn2>(ds, 0x18)(ds, GetDesktopWindow() as usize, 2);
        // The primary buffer and its listener, as the game makes them.
        let desc: [u32; 9] = [0x24, 0x91, 0, 0, 0, 0, 0, 0, 0];
        let mut primary: *mut Obj = std::ptr::null_mut();
        slot::<Fn3>(ds, 0xc)(ds, desc.as_ptr() as usize, &mut primary as *mut _ as usize, 0);
        let listener = qi(primary, &guid("279afa84-4981-11ce-a521-0020af0be560"));
        slot::<Fn0>(client, 0x28)(client);
        let mut warm = Vec::new();
        Sleep(300);
        drain(cap, frame_bytes, &mut warm);
        if std::env::var_os("DSMIX_MULTI").is_some() {
            multi(ds, cap, frame_bytes, &dir, &mut tsv);
            slot::<Fn0>(client, 0x2c)(client);
            return;
        }
        for c in cases() {
            let dense = c.name.starts_with("sweep_dense");
            let frames = if dense { 2400 } else { (c.rate / 2) as usize }; // half a second
            let align = c.channels * c.bits / 8;
            let wf: [u8; 18] = {
                let mut b = [0u8; 18];
                b[0..2].copy_from_slice(&1u16.to_le_bytes());
                b[2..4].copy_from_slice(&c.channels.to_le_bytes());
                b[4..8].copy_from_slice(&c.rate.to_le_bytes());
                b[8..12].copy_from_slice(&(c.rate * u32::from(align)).to_le_bytes());
                b[12..14].copy_from_slice(&align.to_le_bytes());
                b[14..16].copy_from_slice(&c.bits.to_le_bytes());
                b
            };
            let bytes = frames as u32 * u32::from(align);
            let d: [u32; 9] = [0x24, c.flags, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
            let mut b: *mut Obj = std::ptr::null_mut();
            let r = slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            if b.is_null() {
                writeln!(tsv, "{}\tcreate {r:x}", c.name).unwrap();
                continue;
            }
            let data = signal(frames, c.channels as usize, c.bits);
            let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
            slot::<Fn7>(b, 0x2c)(b, 0, bytes as usize, &mut p as *mut _ as usize, &mut n as *mut _ as usize, 0, 0, 0);
            std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
            slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
            let mut results = Vec::new();
            results.push(slot::<Fn1>(b, 0x3c)(b, c.volume as usize));
            if c.flags & 0x40 != 0 {
                results.push(slot::<Fn1>(b, 0x40)(b, c.pan as usize));
            }
            if c.frequency != 0 {
                results.push(slot::<Fn1>(b, 0x44)(b, c.frequency as usize));
            }
            let mut three_d: *mut Obj = std::ptr::null_mut();
            if let Some((pos, vel, min, max, rolloff, doppler)) = c.three_d {
                three_d = qi(b, &guid("279afa86-4981-11ce-a521-0020af0be560"));
                results.push(slot::<FnF3>(three_d, 0x4c)(three_d, pos[0], pos[1], pos[2], 0));
                results.push(slot::<FnF3>(three_d, 0x50)(three_d, vel[0], vel[1], vel[2], 0));
                results.push(slot::<FnF1>(three_d, 0x44)(three_d, min, 0));
                results.push(slot::<FnF1>(three_d, 0x40)(three_d, max, 0));
                results.push(slot::<FnF1>(listener, 0x3c)(listener, rolloff, 0));
                results.push(slot::<FnF1>(listener, 0x30)(listener, doppler, 0));
            }
            let mut fx: *mut Obj = std::ptr::null_mut();
            if let Some(params) = c.distortion {
                let b8 = qi(b, &guid("6825a449-7524-4d82-920f-50e36ab3ab1e"));
                let mut ed = [0u32; 8];
                ed[0] = 0x20;
                let g = guid("ef114c90-cd1d-484e-96e5-09cfaf912a21");
                std::ptr::copy_nonoverlapping(g.as_ptr(), ed.as_mut_ptr().add(2) as *mut u8, 16);
                let mut code = 0u32;
                results.push(slot::<Fn3>(b8, 0x54)(b8, 1, ed.as_ptr() as usize, &mut code as *mut _ as usize));
                let iid = guid("8ecf4326-455f-4d8b-bda9-8d5d3e9e3e0b");
                results.push(slot::<Fn4>(b8, 0x5c)(b8, g.as_ptr() as usize, 0, iid.as_ptr() as usize, &mut fx as *mut _ as usize));
                if !fx.is_null() {
                    results.push(slot::<Fn1>(fx, 0xc)(fx, params.as_ptr() as usize));
                }
                slot::<Fn0>(b8, 8)(b8);
            }
            // Commit any deferred 3D settings, then capture: what is there, then the play.
            slot::<Fn0>(listener, 0x44)(listener);
            let mut before = Vec::new();
            drain(cap, frame_bytes, &mut before);
            let mut out = Vec::new();
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
            for _ in 0..if dense { 30 } else { 90 } {
                Sleep(10);
                drain(cap, frame_bytes, &mut out);
            }
            slot::<Fn0>(b, 0x48)(b);
            std::fs::write(dir.join(format!("{}.f32", c.name)), &out).unwrap();
            writeln!(
                tsv,
                "{}\t{} {} {} {:x} vol {} pan {} freq {} 3d {:?} fx {:?}\tresults {:x?}\tframes {} of {channels} channels",
                c.name,
                c.channels,
                c.rate,
                c.bits,
                c.flags,
                c.volume,
                c.pan,
                c.frequency,
                c.three_d,
                c.distortion,
                results,
                out.len() / frame_bytes
            )
            .unwrap();
            for o in [fx, three_d, b] {
                if !o.is_null() {
                    slot::<Fn0>(o, 8)(o);
                }
            }
            Sleep(100);
        }
        slot::<Fn0>(client, 0x2c)(client);
    }
}

/// `DSMIX_MULTI`: several mono 48 kHz 16-bit voices (seeds 1, 2, ...) with their own volume and
/// pan, played one right after the other, captured together: how the voices are added.
unsafe fn multi(ds: *mut Obj, cap: *mut Obj, frame_bytes: usize, dir: &std::path::Path, tsv: &mut std::fs::File) {
    let groups: [(&str, &[(i32, i32)]); 4] = [
        ("multi2", &[(0, 0), (-600, 0)]),
        ("multi3", &[(-100, 0), (-600, -1000), (-2000, 3000)]),
        ("multi4", &[(-50, 0), (-350, 0), (-1234, 0), (-777, 0)]),
        ("multi3_loud", &[(0, 0), (-1, 0), (-2, 0)]),
    ];
    for (name, voices) in groups {
        let frames = 24000usize;
        let wf: [u8; 18] = [1, 0, 1, 0, 0x80, 0xbb, 0, 0, 0, 0x77, 1, 0, 2, 0, 16, 0, 0, 0];
        let bytes = frames as u32 * 2;
        let mut made = Vec::new();
        for (i, &(volume, pan)) in voices.iter().enumerate() {
            let d: [u32; 9] = [0x24, 0xe0 | 0x8000, bytes, 0, wf.as_ptr() as u32, 0, 0, 0, 0];
            let mut b: *mut Obj = std::ptr::null_mut();
            slot::<Fn3>(ds, 0xc)(ds, d.as_ptr() as usize, &mut b as *mut _ as usize, 0);
            assert!(!b.is_null(), "{name}: CreateSoundBuffer");
            let mut data = signal_seeded(frames, 1, 16, i as u32 + 1);
            if name.ends_with("_loud") {
                // Near full scale, so that the sum's rounding shows.
                for s in data.chunks_mut(2) {
                    let v = i32::from(i16::from_le_bytes([s[0], s[1]])) * 30;
                    s.copy_from_slice(&(v as i16).to_le_bytes());
                }
            }
            let (mut p, mut n) = (std::ptr::null_mut::<u8>(), 0u32);
            slot::<Fn7>(b, 0x2c)(b, 0, bytes as usize, &mut p as *mut _ as usize, &mut n as *mut _ as usize, 0, 0, 0);
            std::ptr::copy_nonoverlapping(data.as_ptr(), p, n as usize);
            slot::<Fn4>(b, 0x4c)(b, p as usize, n as usize, 0, 0);
            slot::<Fn1>(b, 0x3c)(b, volume as usize);
            slot::<Fn1>(b, 0x40)(b, pan as usize);
            made.push(b);
        }
        let mut before = Vec::new();
        drain(cap, frame_bytes, &mut before);
        let mut out = Vec::new();
        for &b in &made {
            slot::<Fn3>(b, 0x30)(b, 0, 0, 0);
        }
        for _ in 0..70 {
            Sleep(10);
            drain(cap, frame_bytes, &mut out);
        }
        for &b in &made {
            slot::<Fn0>(b, 0x48)(b);
            slot::<Fn0>(b, 8)(b);
        }
        std::fs::write(dir.join(format!("{name}.f32")), &out).unwrap();
        writeln!(tsv, "{name}\tvoices {voices:?}\tframes {}", out.len() / frame_bytes).unwrap();
        Sleep(100);
    }
}

type EnumCallback = unsafe extern "system" fn(*const [u8; 16], *const u8, *const u8, *mut c_void) -> i32;
#[link(name = "dsound")]
extern "system" {
    fn DirectSoundEnumerateA(callback: EnumCallback, context: *mut c_void) -> u32;
}

/// The DirectSound device whose description contains the text in `context`.
unsafe extern "system" fn find_device(g: *const [u8; 16], description: *const u8, _module: *const u8, context: *mut c_void) -> i32 {
    let found = &mut *(context as *mut (String, Option<[u8; 16]>));
    let text = std::ffi::CStr::from_ptr(description as *const i8).to_string_lossy();
    if !g.is_null() && text.contains(found.0.as_str()) {
        found.1 = Some(*g);
        return 0;
    }
    1
}

/// The DirectSound device GUID for the description given as the program's third argument.
fn direct_sound_guid(_id: &str) -> [u8; 16] {
    let text = std::env::args().nth(3).expect("the device's description (third argument)");
    let mut found = (text, None);
    unsafe { DirectSoundEnumerateA(find_device, &mut found as *mut _ as *mut c_void) };
    found.1.unwrap_or_else(|| panic!("no DirectSound device {}", found.0))
}
