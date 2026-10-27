//! dshow-measure: what DirectShow's filter graph answers for the game's music, for the
//! platform's filter graph (`engine_core::units::platform::dshow`). 32-bit, as the game.
//! Reads the MP3 files given (the player's `Data\Music`, read only); writes nothing but stdout.
//! The graph's sound is muted (`put_Volume(-10000)`) before it runs.
//!
//! `cargo run --release --target i686-pc-windows-msvc --manifest-path scripts/dshow-measure/Cargo.toml -- <mp3 files..> > dshow.tsv`
//!
//! Lines: `file<TAB>path`; `render<TAB>HRESULT`; `filters<TAB>names, render order`;
//! `duration<TAB>hr<TAB>the double's bits`; with `--timing` also `run<TAB>samples` (milliseconds
//! since `Run`:`get_CurrentPosition` bits, at each change, for 1.5 s), `volume<TAB>value hr..`,
//! `seek<TAB>..` and `event<TAB>ms code p1 p2` for the events after a seek to 0.5 s before the end.
use std::ffi::c_void;
use std::io::Write;

#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut c_void, flags: u32) -> u32;
    fn CoCreateInstance(
        clsid: *const [u8; 16],
        outer: *mut c_void,
        context: u32,
        iid: *const [u8; 16],
        out: *mut *mut Obj,
    ) -> u32;
}
#[link(name = "kernel32")]
extern "system" {
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn Sleep(ms: u32);
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
type FnD = unsafe extern "system" fn(*mut Obj, f64) -> u32;

fn guid(s: &str) -> [u8; 16] {
    let h: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    let b = |i: usize| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap();
    [
        b(3),
        b(2),
        b(1),
        b(0),
        b(5),
        b(4),
        b(7),
        b(6),
        b(8),
        b(9),
        b(10),
        b(11),
        b(12),
        b(13),
        b(14),
        b(15),
    ]
}

fn now_ms(freq: i64, origin: i64) -> f64 {
    let mut c = 0i64;
    unsafe { QueryPerformanceCounter(&mut c) };
    (c - origin) as f64 * 1000.0 / freq as f64
}

unsafe fn qi(o: *mut Obj, iid: &[u8; 16]) -> *mut Obj {
    let mut out: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(o, 0)(o, iid.as_ptr() as usize, &mut out as *mut _ as usize);
    out
}

unsafe fn filters(graph: *mut Obj) -> String {
    let mut en: *mut Obj = std::ptr::null_mut();
    slot::<Fn1>(graph, 0x14)(graph, &mut en as *mut _ as usize);
    let mut names = Vec::new();
    loop {
        let mut f: *mut Obj = std::ptr::null_mut();
        let mut got = 0u32;
        if slot::<Fn3>(en, 0xc)(
            en,
            1,
            &mut f as *mut _ as usize,
            &mut got as *mut _ as usize,
        ) != 0
            || got == 0
        {
            break;
        }
        let mut info = [0u16; 130];
        slot::<Fn1>(f, 0x30)(f, info.as_mut_ptr() as usize);
        let end = info[..128].iter().position(|&c| c == 0).unwrap_or(128);
        names.push(String::from_utf16_lossy(&info[..end]));
        let g = (info.as_ptr().add(128) as *const *mut Obj).read_unaligned();
        if !g.is_null() {
            slot::<Fn0>(g, 8)(g);
        }
        slot::<Fn0>(f, 8)(f);
    }
    slot::<Fn0>(en, 8)(en);
    names.join(", ")
}

/// The game's rounds (`00830b30`): every 100 ms `get_CurrentPosition`, `get_Duration`,
/// `put_Volume(-10000)`, `Run`, then `GetEvent(.., 0)` until it fails, each event freed; on
/// events 1..3 (a looping file) `Stop`, `put_CurrentPosition(0.0)`, `Run`. Round 5 pauses,
/// round 8 runs again, round 10 seeks to 1 s before the end. 40 rounds.
unsafe fn game_rounds(
    o: &mut impl Write,
    freq: i64,
    control: *mut Obj,
    position: *mut Obj,
    audio: *mut Obj,
    event: *mut Obj,
    d: f64,
) {
    let mut origin = 0i64;
    QueryPerformanceCounter(&mut origin);
    for round in 0..40 {
        Sleep(100);
        let t = now_ms(freq, origin);
        let (mut p, mut dur) = (0f64, 0f64);
        let rp = slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
        let rd = slot::<Fn1>(position, 0x1c)(position, &mut dur as *mut _ as usize);
        let rv = slot::<Fn1>(audio, 0x1c)(audio, (-10000i32) as usize);
        let (action, rc) = match round {
            5..=7 => ("pause", slot::<Fn0>(control, 0x20)(control)),
            25..=27 => ("stop", slot::<Fn0>(control, 0x24)(control)),
            10 => {
                let r = slot::<FnD>(position, 0x20)(position, d - 1.0);
                ("seek", r | slot::<Fn0>(control, 0x1c)(control) << 16)
            }
            _ => ("run", slot::<Fn0>(control, 0x1c)(control)),
        };
        let mut events = Vec::new();
        loop {
            let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
            let r = slot::<Fn4>(event, 0x20)(
                event,
                &mut code as *mut _ as usize,
                &mut p1 as *mut _ as usize,
                &mut p2 as *mut _ as usize,
                0,
            );
            if r as i32 != 0 {
                if events.is_empty() {
                    events.push(format!("none {r:x}"));
                }
                break;
            }
            slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
            events.push(format!("{code:x}:{p1:x}:{p2:x}"));
            if (1..=3).contains(&code) {
                let s = slot::<Fn0>(control, 0x24)(control);
                let z = slot::<FnD>(position, 0x20)(position, 0.0);
                let r = slot::<Fn0>(control, 0x1c)(control);
                events.push(format!("restart {s:x} {z:x} {r:x}"));
            }
        }
        writeln!(
            o,
            "round\t{round}\t{t:.1}\t{rp:x} {p}\t{rd:x} {dur}\t{rv:x}\t{action} {rc:x}\t{}",
            events.join(" ")
        )
        .unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let timing = args.iter().any(|a| a == "--timing");
    let game = args.iter().any(|a| a == "--game");
    let grab = args
        .iter()
        .position(|a| a == "--grab")
        .map(|i| args[i + 1].clone());
    let mut o = std::io::stdout().lock();
    let clsid_graph = guid("e436ebb3-524f-11ce-9f53-0020af0ba770");
    let iid_builder = guid("56a868a9-0ad4-11ce-b03a-0020af0ba770");
    let iid_control = guid("56a868b1-0ad4-11ce-b03a-0020af0ba770");
    let iid_position = guid("56a868b2-0ad4-11ce-b03a-0020af0ba770");
    let iid_audio = guid("56a868b3-0ad4-11ce-b03a-0020af0ba770");
    let iid_event = guid("56a868b6-0ad4-11ce-b03a-0020af0ba770");
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), 0);
        name_probe(&mut o);
        let mut freq = 0i64;
        QueryPerformanceFrequency(&mut freq);
        let skip: Vec<&String> = grab.iter().collect();
        for path in args
            .iter()
            .filter(|a| !a.starts_with("--") && !skip.contains(a))
        {
            writeln!(o, "file\t{path}").unwrap();
            let mut gb: *mut Obj = std::ptr::null_mut();
            let hr = CoCreateInstance(&clsid_graph, std::ptr::null_mut(), 1, &iid_builder, &mut gb);
            if hr != 0 {
                writeln!(o, "create\t{hr:x}").unwrap();
                continue;
            }
            let wide: Vec<u16> = path.encode_utf16().chain([0]).collect();
            let hr = slot::<Fn2>(gb, 0x34)(gb, wide.as_ptr() as usize, 0);
            writeln!(o, "render\t{hr:x}").unwrap();
            writeln!(o, "filters\t{}", filters(gb)).unwrap();
            let control = qi(gb, &iid_control);
            let position = qi(gb, &iid_position);
            let audio = qi(gb, &iid_audio);
            let event = qi(gb, &iid_event);
            let mut d = 0f64;
            let hr = slot::<Fn1>(position, 0x1c)(position, &mut d as *mut _ as usize);
            writeln!(o, "duration\t{hr:x}\t{:016x}\t{d}", d.to_bits()).unwrap();
            if timing && !audio.is_null() {
                for v in [-10001i32, 1, 0, -10000] {
                    let r = slot::<Fn1>(audio, 0x1c)(audio, v as usize);
                    let mut back = 0i32;
                    slot::<Fn1>(audio, 0x20)(audio, &mut back as *mut _ as usize);
                    writeln!(o, "volume\t{v}\t{r:x}\t{back}").unwrap();
                }
                let mut origin = 0i64;
                QueryPerformanceCounter(&mut origin);
                let r = slot::<Fn0>(control, 0x1c)(control);
                let mut samples = Vec::new();
                let mut last = u64::MAX;
                loop {
                    let t = now_ms(freq, origin);
                    if t > 1500.0 {
                        break;
                    }
                    let mut p = 0f64;
                    slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                    if p.to_bits() != last {
                        samples.push(format!("{t:.3}:{p}"));
                        last = p.to_bits();
                    }
                }
                writeln!(o, "run\t{r:x}\t{}", samples.join(" ")).unwrap();
                // Pause, read, run again.
                let r = slot::<Fn0>(control, 0x20)(control);
                let mut p = 0f64;
                Sleep(200);
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                Sleep(200);
                let mut p2 = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p2 as *mut _ as usize);
                writeln!(o, "pause\t{r:x}\t{p} {p2}").unwrap();
                let target = d - 0.5;
                let r = slot::<FnD>(position, 0x20)(position, target);
                let mut p = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                writeln!(o, "seek\t{target}\t{r:x}\t{p}").unwrap();
                QueryPerformanceCounter(&mut origin);
                let r = slot::<Fn0>(control, 0x1c)(control);
                writeln!(o, "rerun\t{r:x}").unwrap();
                loop {
                    let t = now_ms(freq, origin);
                    if t > 3000.0 {
                        break;
                    }
                    let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
                    let r = slot::<Fn4>(event, 0x20)(
                        event,
                        &mut code as *mut _ as usize,
                        &mut p1 as *mut _ as usize,
                        &mut p2 as *mut _ as usize,
                        10,
                    );
                    if r == 0 {
                        let mut p = 0f64;
                        slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                        writeln!(o, "event\t{t:.3}\t{code:x} {p1:x} {p2:x}\tposition {p}").unwrap();
                        slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
                    }
                }
                let mut p = 0f64;
                slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                writeln!(o, "end\tposition {p}").unwrap();
                // A seek past the end and to a negative time.
                for t in [d + 1.0, -1.0] {
                    let r = slot::<FnD>(position, 0x20)(position, t);
                    let mut p = 0f64;
                    slot::<Fn1>(position, 0x24)(position, &mut p as *mut _ as usize);
                    writeln!(o, "seek\t{t}\t{r:x}\t{p}").unwrap();
                }
                let r = slot::<Fn0>(control, 0x24)(control);
                writeln!(o, "stop\t{r:x}").unwrap();
            }
            if let Some(dir) = &grab {
                grab_pcm(&mut o, gb, dir, path);
            }
            if game && !audio.is_null() {
                game_rounds(&mut o, freq, control, position, audio, event, d);
            }
            for x in [event, audio, position, control, gb] {
                if !x.is_null() {
                    slot::<Fn0>(x, 8)(x);
                }
            }
        }
    }
}

/// The name the graph gives a filter added with a NULL name: the File Source (Async) filter
/// added with `AddFilter(f, NULL)`, then `QueryFilterInfo`.
unsafe fn name_probe(o: &mut impl Write) {
    let clsid_graph = guid("e436ebb3-524f-11ce-9f53-0020af0ba770");
    let iid_graph = guid("56a8689f-0ad4-11ce-b03a-0020af0ba770");
    let clsid_async = guid("e436ebb5-524f-11ce-9f53-0020af0ba770");
    let iid_filter = guid("56a86895-0ad4-11ce-b03a-0020af0ba770");
    let mut fg: *mut Obj = std::ptr::null_mut();
    let mut f: *mut Obj = std::ptr::null_mut();
    let a = CoCreateInstance(&clsid_graph, std::ptr::null_mut(), 1, &iid_graph, &mut fg);
    let b = CoCreateInstance(&clsid_async, std::ptr::null_mut(), 1, &iid_filter, &mut f);
    for _ in 0..2 {
        let r = slot::<Fn2>(fg, 0xc)(fg, f as usize, 0);
        let mut info = [0u16; 130];
        slot::<Fn1>(f, 0x30)(f, info.as_mut_ptr() as usize);
        let end = info[..128].iter().position(|&c| c == 0).unwrap_or(128);
        writeln!(
            o,
            "addfilter\t{a:x} {b:x}\t{r:x}\t{}",
            String::from_utf16_lossy(&info[..end])
        )
        .unwrap();
        let g = (info.as_ptr().add(128) as *const *mut Obj).read_unaligned();
        if !g.is_null() {
            slot::<Fn0>(g, 8)(g);
        }
    }
    slot::<Fn0>(f, 8)(f);
    slot::<Fn0>(fg, 8)(fg);
}

/// The bytes the decoder delivered, gathered by the sample grabber's callback.
static mut GRABBED: Vec<u8> = Vec::new();

unsafe extern "system" fn cb_query(this: *mut Obj, _iid: *const u8, out: *mut *mut Obj) -> u32 {
    *out = this;
    0
}
unsafe extern "system" fn cb_ref(_this: *mut Obj) -> u32 {
    1
}
unsafe extern "system" fn cb_sample(_this: *mut Obj, _time: f64, _sample: *mut Obj) -> u32 {
    0
}
#[allow(static_mut_refs)]
unsafe extern "system" fn cb_buffer(
    _this: *mut Obj,
    _time: f64,
    buffer: *const u8,
    len: i32,
) -> u32 {
    GRABBED.extend_from_slice(std::slice::from_raw_parts(buffer, len as usize));
    0
}

#[repr(C)]
struct CallbackVtable {
    query: unsafe extern "system" fn(*mut Obj, *const u8, *mut *mut Obj) -> u32,
    add_ref: unsafe extern "system" fn(*mut Obj) -> u32,
    release: unsafe extern "system" fn(*mut Obj) -> u32,
    sample: unsafe extern "system" fn(*mut Obj, f64, *mut Obj) -> u32,
    buffer: unsafe extern "system" fn(*mut Obj, f64, *const u8, i32) -> u32,
}
static CALLBACK: CallbackVtable = CallbackVtable {
    query: cb_query,
    add_ref: cb_ref,
    release: cb_ref,
    sample: cb_sample,
    buffer: cb_buffer,
};

/// The first pin of `f` in direction `dir` (0 input, 1 output).
unsafe fn pin_of(f: *mut Obj, dir: u32) -> *mut Obj {
    let mut en: *mut Obj = std::ptr::null_mut();
    slot::<Fn1>(f, 0x28)(f, &mut en as *mut _ as usize);
    loop {
        let mut p: *mut Obj = std::ptr::null_mut();
        let mut got = 0u32;
        if slot::<Fn3>(en, 0xc)(
            en,
            1,
            &mut p as *mut _ as usize,
            &mut got as *mut _ as usize,
        ) != 0
            || got == 0
        {
            slot::<Fn0>(en, 8)(en);
            return std::ptr::null_mut();
        }
        let mut d = 9u32;
        slot::<Fn1>(p, 0x24)(p, &mut d as *mut _ as usize);
        if d == dir {
            slot::<Fn0>(en, 8)(en);
            return p;
        }
        slot::<Fn0>(p, 8)(p);
    }
}

/// The filter of the graph whose name starts with `name`.
unsafe fn find_filter(graph: *mut Obj, name: &str) -> *mut Obj {
    let wide: Vec<u16> = name.encode_utf16().chain([0]).collect();
    let mut f: *mut Obj = std::ptr::null_mut();
    slot::<Fn2>(graph, 0x18)(graph, wide.as_ptr() as usize, &mut f as *mut _ as usize);
    f
}

/// The rendered graph's decoder output through a sample grabber into a null renderer, run
/// without a clock; the bytes written to `dir`/<file name>.pcm and their count reported.
#[allow(static_mut_refs)]
unsafe fn grab_pcm(o: &mut impl Write, gb: *mut Obj, dir: &str, path: &str) {
    GRABBED.clear();
    let renderer = find_filter(gb, "Default DirectSound Device");
    let decoder = find_filter(gb, "MP3 Decoder DMO");
    let r1 = slot::<Fn1>(gb, 0x10)(gb, renderer as usize);
    let out = pin_of(decoder, 1);
    let iid_filter = guid("56a86895-0ad4-11ce-b03a-0020af0ba770");
    let mut grabber: *mut Obj = std::ptr::null_mut();
    let mut null: *mut Obj = std::ptr::null_mut();
    let r2 = CoCreateInstance(
        &guid("c1f400a0-3f08-11d3-9f0b-006008039e37"),
        std::ptr::null_mut(),
        1,
        &iid_filter,
        &mut grabber,
    );
    let r3 = CoCreateInstance(
        &guid("c1f400a4-3f08-11d3-9f0b-006008039e37"),
        std::ptr::null_mut(),
        1,
        &iid_filter,
        &mut null,
    );
    let name: Vec<u16> = "g".encode_utf16().chain([0]).collect();
    slot::<Fn2>(gb, 0xc)(gb, grabber as usize, name.as_ptr() as usize);
    let name: Vec<u16> = "n".encode_utf16().chain([0]).collect();
    slot::<Fn2>(gb, 0xc)(gb, null as usize, name.as_ptr() as usize);
    let sg = qi(grabber, &guid("6b652fff-11fe-4fce-92ad-0266b5d7c78f"));
    slot::<Fn1>(sg, 0xc)(sg, 0);
    slot::<Fn1>(sg, 0x18)(sg, 0);
    let cb = &CALLBACK as *const CallbackVtable;
    let cb_obj = Box::into_raw(Box::new(Obj {
        vtable: cb as *const usize,
    }));
    let r4 = slot::<Fn2>(sg, 0x24)(sg, cb_obj as usize, 1);
    let g_in = pin_of(grabber, 0);
    let g_out = pin_of(grabber, 1);
    let n_in = pin_of(null, 0);
    let r5 = slot::<Fn2>(gb, 0x2c)(gb, out as usize, g_in as usize);
    let r6 = slot::<Fn2>(gb, 0x2c)(gb, g_out as usize, n_in as usize);
    let mf = qi(gb, &guid("56a86899-0ad4-11ce-b03a-0020af0ba770"));
    let r7 = slot::<Fn1>(mf, 0x20)(mf, 0);
    let control = qi(gb, &guid("56a868b1-0ad4-11ce-b03a-0020af0ba770"));
    let event = qi(gb, &guid("56a868b6-0ad4-11ce-b03a-0020af0ba770"));
    slot::<Fn0>(control, 0x1c)(control);
    loop {
        let (mut code, mut p1, mut p2) = (0i32, 0usize, 0usize);
        let r = slot::<Fn4>(event, 0x20)(
            event,
            &mut code as *mut _ as usize,
            &mut p1 as *mut _ as usize,
            &mut p2 as *mut _ as usize,
            30_000,
        );
        if r != 0 {
            break;
        }
        slot::<Fn3>(event, 0x30)(event, code as usize, p1, p2);
        if code == 1 {
            break;
        }
    }
    slot::<Fn0>(control, 0x24)(control);
    // The format the grabber was connected with.
    let mut mt = [0u8; 0x48];
    slot::<Fn1>(sg, 0x14)(sg, mt.as_mut_ptr() as usize);
    let fmt_ptr = u32::from_le_bytes([mt[0x44], mt[0x45], mt[0x46], mt[0x47]]) as *const u8;
    let fmt = if fmt_ptr.is_null() {
        Vec::new()
    } else {
        std::slice::from_raw_parts(fmt_ptr, 18).to_vec()
    };
    let file = std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    std::fs::write(
        std::path::Path::new(dir).join(format!("{file}.pcm")),
        &GRABBED,
    )
    .unwrap();
    writeln!(
        o,
        "grab\t{r1:x} {r2:x} {r3:x} {r4:x} {r5:x} {r6:x} {r7:x}\t{}\tformat {}",
        GRABBED.len(),
        fmt.iter().map(|b| format!("{b:02x}")).collect::<String>()
    )
    .unwrap();
    for x in [
        event, control, mf, sg, g_in, g_out, n_in, out, null, grabber, decoder, renderer,
    ] {
        if !x.is_null() {
            slot::<Fn0>(x, 8)(x);
        }
    }
}
