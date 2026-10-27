//! The platform's DirectShow filter graph, for the game's music: `CoCreateInstance` of
//! `CLSID_FilterGraph` and the interfaces `FalloutAudioMedia::ThreadProc` (`00830b30`) uses on it:
//! `IFilterGraph`/`IGraphBuilder` (`AddFilter`, `Render`), `IMediaControl` (`Run`, `Pause`,
//! `Stop`), `IMediaPosition` (`get_Duration`, `get_CurrentPosition`, `put_CurrentPosition`),
//! `IBasicAudio` (`put_Volume`) and `IMediaEvent` (`GetEvent`, `FreeEventParams`). The game adds
//! its own source filter (`CBSAReader`, over the `.mp3` in the archives) and renders its output
//! pin; Windows then builds the rest of the graph (measured: "MPEG-I Stream Splitter", "MP3
//! Decoder DMO", "Default DirectSound Device"). Here `Render` connects the game's pin to this
//! layer's own input pin through the game's own pin code, reads the whole stream through the
//! pin's `IAsyncReader` (`Length`, `SyncRead`) and decodes it with `crates/mp3`. Nothing is heard
//! yet (the mixer and the device output come next): the graph keeps time on the platform clock
//! (`Engine::clock_now_us`).
//!
//! Measured on this machine (2026-10-24, `scripts/dshow-measure` on the player's `Data\Music`,
//! read only, the graph muted; `build43-scratch/measure/dshow-*-2026-10-24.tsv`):
//! - `get_Duration` of all 199 tracks is `floor(size * 8 * 10^7 / bitrate)` 100-ns units, the
//!   size the stream's and the bitrate its first audio frame's, bit for bit as a `double`.
//! - `AddFilter(f, NULL)` names the filter `"0001"` (the graph numbers the filters it names).
//! - `Run` from stopped answers `S_FALSE`; the first one ever queues `EC_CLOCK_CHANGED` at once,
//!   and every one from stopped an `EC_PAUSED` that comes later (the next 100 ms round);
//!   `Run` while paused or running answers `S_OK` and queues nothing. Every `Pause` (running or
//!   paused) answers `S_OK` and queues `EC_PAUSED` at once. `Stop` answers `S_OK` and keeps the
//!   position.
//! - The position moves at the stream's rate while running (from 36 to 58 ms after `Run` on the
//!   device: here at once) and stays while paused or stopped; at the end it is the duration
//!   and `EC_COMPLETE` (code 1, parameters 0) is queued. `put_CurrentPosition` below 0 is
//!   `E_INVALIDARG`; past the end it is the duration.
//! - `GetEvent` with nothing queued answers `E_ABORT`. `put_Volume` takes -10000..0, else
//!   `E_INVALIDARG`.
//! - The decoded samples (the MP3 Decoder DMO's output, grabbed in the graph): see
//!   [`windows_samples`]: this layer's samples are Windows' for 99.9748% of all 199 tracks' samples.
//!
//! What this layer does differently, as nothing of it is visible to the game's own code but the
//! calls into its filter: Windows' splitter reads the stream as it plays (requests of its own);
//! here the whole stream is read once, at `Render`. The filter's own state methods (`Pause`,
//! `Run`, `Stop`, `SetSyncSource`) are not called: no stream runs through it here.

use crate::{Engine, Ptr};
use std::collections::VecDeque;

/// Where the platform's DirectShow functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e00_0800;
const BUILDER_CODE: u32 = CODE_BASE;
const CONTROL_CODE: u32 = CODE_BASE + 0x80;
const POSITION_CODE: u32 = CODE_BASE + 0x100;
const AUDIO_CODE: u32 = CODE_BASE + 0x180;
const EVENT_CODE: u32 = CODE_BASE + 0x200;
const PIN_CODE: u32 = CODE_BASE + 0x280;

/// The interfaces of a graph object, by their offset in it.
const BUILDER: u32 = 0;
const CONTROL: u32 = 4;
const POSITION: u32 = 8;
const AUDIO: u32 = 0xc;
const EVENT: u32 = 0x10;
const GRAPH_BYTES: u32 = 0x14;

/// Result codes (winerror.h, vfwmsgs.h).
pub const S_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const E_NOTIMPL: u32 = 0x8000_4001;
pub const E_ABORT: u32 = 0x8000_4004;
pub const E_INVALIDARG: u32 = 0x8007_0057;
pub const VFW_E_TYPE_NOT_ACCEPTED: u32 = 0x8004_022a;
/// Event codes (evcode.h).
pub const EC_COMPLETE: u32 = 1;
pub const EC_CLOCK_CHANGED: u32 = 0x0d;
pub const EC_PAUSED: u32 = 0x0e;

fn guid(text: &str) -> [u8; 16] {
    let h: Vec<u8> = text
        .bytes()
        .filter(u8::is_ascii_hexdigit)
        .map(|c| (c as char).to_digit(16).unwrap_or(0) as u8)
        .collect();
    let b = |i: usize| h[2 * i] << 4 | h[2 * i + 1];
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

/// The ids the game passes (from its `.rdata`) and those of the pin protocol.
fn clsid_filter_graph() -> [u8; 16] {
    guid("e436ebb3-524f-11ce-9f53-0020af0ba770")
}
fn iid(name: &str) -> [u8; 16] {
    guid(match name {
        "IUnknown" => "00000000-0000-0000-c000-000000000046",
        "IFilterGraph" => "56a8689f-0ad4-11ce-b03a-0020af0ba770",
        "IGraphBuilder" => "56a868a9-0ad4-11ce-b03a-0020af0ba770",
        "IMediaControl" => "56a868b1-0ad4-11ce-b03a-0020af0ba770",
        "IMediaPosition" => "56a868b2-0ad4-11ce-b03a-0020af0ba770",
        "IBasicAudio" => "56a868b3-0ad4-11ce-b03a-0020af0ba770",
        "IMediaEvent" => "56a868b6-0ad4-11ce-b03a-0020af0ba770",
        "IMediaEventEx" => "56a868c0-0ad4-11ce-b03a-0020af0ba770",
        "IPin" => "56a86891-0ad4-11ce-b03a-0020af0ba770",
        "IAsyncReader" => "56a868aa-0ad4-11ce-b03a-0020af0ba770",
        "MEDIATYPE_Stream" => "e436eb83-524f-11ce-9f53-0020af0ba770",
        "MEDIASUBTYPE_MPEG1Audio" => "e436eb87-524f-11ce-9f53-0020af0ba770",
        _ => unreachable!("an id this layer names"),
    })
}

/// Where a graph is.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Run {
    Stopped,
    Paused,
    /// Running since the clock read `start` (microseconds).
    Running {
        start: u64,
    },
}

/// A filter graph.
#[derive(Debug)]
struct Graph {
    references: u32,
    /// The game's filter (its `IBaseFilter`) and this layer's input pin.
    filter: u32,
    pin: u32,
    /// The stream's length and its duration (100 ns).
    stream_bytes: u64,
    duration: i64,
    /// The decoded samples (interleaved 16-bit) and their format, for the output to come.
    samples: Vec<i16>,
    sample_rate: u32,
    channels: u16,
    run: Run,
    /// The stream position (100 ns) where it stopped or paused, or where the run began.
    base: i64,
    ran: bool,
    completed: bool,
    /// Queued events: code, two parameters, and the clock time (microseconds) after which it
    /// can be taken.
    events: VecDeque<(u32, u32, u32, u64)>,
    volume: i32,
}

/// This layer's input pin.
#[derive(Debug)]
struct Pin {
    references: u32,
    connected: u32,
    reader: u32,
}

/// The DirectShow objects (in `Kernel::sound`).
#[derive(Debug, Default)]
pub struct Show {
    vtables: [u32; 6],
    graphs: std::collections::BTreeMap<u32, Graph>,
    pins: std::collections::BTreeMap<u32, Pin>,
    /// The filters `AddFilter` named so far (the next name's number).
    named: u32,
}

type Slots<const N: usize> = [(u32, crate::AbiFn); N];

fn register_code(e: &mut Engine) {
    let builder: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (3, crate::entry!(0, add_filter(Ptr, Ptr, Ptr) -> u32).1),
        (12, crate::entry!(0, render(Ptr, Ptr) -> u32).1),
    ];
    let control: Slots<6> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, control_run(Ptr) -> u32).1),
        (8, crate::entry!(0, control_pause(Ptr) -> u32).1),
        (9, crate::entry!(0, control_stop(Ptr) -> u32).1),
    ];
    let position: Slots<6> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, get_duration(Ptr, Ptr) -> u32).1),
        (8, crate::entry!(0, put_current_position(Ptr, f64) -> u32).1),
        (9, crate::entry!(0, get_current_position(Ptr, Ptr) -> u32).1),
    ];
    let audio: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (7, crate::entry!(0, put_volume(Ptr, i32) -> u32).1),
        (8, crate::entry!(0, get_volume(Ptr, Ptr) -> u32).1),
    ];
    let event: Slots<5> = [
        (0, crate::entry!(0, query_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, release(Ptr) -> u32).1),
        (
            8,
            crate::entry!(0, get_event(Ptr, Ptr, Ptr, Ptr, i32) -> u32).1,
        ),
        (
            12,
            crate::entry!(0, free_event_params(Ptr, u32, u32, u32) -> u32).1,
        ),
    ];
    let pin: Slots<7> = [
        (
            0,
            crate::entry!(0, pin_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, pin_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, pin_release(Ptr) -> u32).1),
        (
            4,
            crate::entry!(0, receive_connection(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (5, crate::entry!(0, pin_disconnect(Ptr) -> u32).1),
        (9, crate::entry!(0, query_direction(Ptr, Ptr) -> u32).1),
        (12, crate::entry!(0, enum_media_types(Ptr, Ptr) -> u32).1),
    ];
    let sets: [(&[(u32, crate::AbiFn)], u32); 6] = [
        (&builder, BUILDER_CODE),
        (&control, CONTROL_CODE),
        (&position, POSITION_CODE),
        (&audio, AUDIO_CODE),
        (&event, EVENT_CODE),
        (&pin, PIN_CODE),
    ];
    for (i, (slots, code)) in sets.into_iter().enumerate() {
        for &(slot, f) in slots {
            e.register(code + 4 * slot, f);
        }
        // Each vtable is 32 slots; a slot not provided stops the run as an open function.
        let table = e.mem.alloc(4 * 32);
        for s in 0..32 {
            e.mem.set_u32(table + 4 * s, code + 4 * s);
        }
        e.kernel.sound.show.vtables[i] = table;
    }
}

/// `CoCreateInstance(clsid, outer, context, iid, out)`: `CLSID_FilterGraph` in process
/// (`CLSCTX_INPROC_SERVER`), no outer object, asked for one of the graph's interfaces: a new
/// graph. Every other class is not provided (the DirectShow filter mapper the game's filter
/// registration would ask for, `009fba40`/`009fbab0`, is not reached by the game's run).
pub fn co_create_instance(
    e: &mut Engine,
    clsid: Ptr,
    outer: u32,
    context: u32,
    interface: Ptr,
    out: Ptr,
) -> u32 {
    let class = e.mem.bytes(clsid.addr(), 16);
    let wanted = e.mem.bytes(interface.addr(), 16);
    assert!(
        class == clsid_filter_graph() && outer == 0 && context == 1,
        "CoCreateInstance({}, outer {outer:#x}, context {context:#x}) is not provided",
        hex(&class)
    );
    if e.kernel.sound.show.vtables[0] == 0 {
        register_code(e);
    }
    let at = e.mem.alloc(GRAPH_BYTES);
    for i in 0..5 {
        let table = e.kernel.sound.show.vtables[i as usize];
        e.mem.set_u32(at + 4 * i, table);
    }
    e.kernel.sound.show.graphs.insert(
        at,
        Graph {
            references: 0,
            filter: 0,
            pin: 0,
            stream_bytes: 0,
            duration: 0,
            samples: Vec::new(),
            sample_rate: 0,
            channels: 0,
            run: Run::Stopped,
            base: 0,
            ran: false,
            completed: false,
            events: VecDeque::new(),
            volume: 0,
        },
    );
    let r = query(e, at, &wanted, out);
    assert!(
        r == S_OK,
        "CoCreateInstance(CLSID_FilterGraph) for {} is not provided",
        hex(&wanted)
    );
    r
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The graph an interface pointer belongs to, from the vtable it holds.
fn graph_of(e: &Engine, this: Ptr) -> u32 {
    let vtable = e.mem.u32(this.addr());
    let tables = e.kernel.sound.show.vtables;
    let offset = [BUILDER, CONTROL, POSITION, AUDIO, EVENT]
        .into_iter()
        .zip(tables)
        .find(|&(_, t)| t == vtable)
        .map(|(o, _)| o)
        .expect("an interface of this layer's filter graph");
    this.addr() - offset
}

fn graph(e: &mut Engine, at: u32) -> &mut Graph {
    e.kernel
        .sound
        .show
        .graphs
        .get_mut(&at)
        .expect("a filter graph of this layer")
}

/// Hands out the graph's interface for `wanted` with a reference.
fn query(e: &mut Engine, at: u32, wanted: &[u8], out: Ptr) -> u32 {
    let offset = if wanted == iid("IUnknown")
        || wanted == iid("IFilterGraph")
        || wanted == iid("IGraphBuilder")
    {
        BUILDER
    } else if wanted == iid("IMediaControl") {
        CONTROL
    } else if wanted == iid("IMediaPosition") {
        POSITION
    } else if wanted == iid("IBasicAudio") {
        AUDIO
    } else if wanted == iid("IMediaEvent") || wanted == iid("IMediaEventEx") {
        EVENT
    } else {
        e.mem.set_u32(out.addr(), 0);
        return E_NOINTERFACE;
    };
    graph(e, at).references += 1;
    e.mem.set_u32(out.addr(), at + offset);
    S_OK
}

fn query_interface(e: &mut Engine, this: Ptr, interface: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let wanted = e.mem.bytes(interface.addr(), 16);
    query(e, at, &wanted, out)
}

fn add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let g = graph(e, at);
    g.references += 1;
    g.references
}

/// `Release()`: at the last reference the graph lets go of the game's filter as DirectShow
/// does: the pins disconnected (the game's pin's `Disconnect`), `JoinFilterGraph(NULL, NULL)`
/// and the reference `AddFilter` took released.
fn release(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let g = graph(e, at);
    g.references -= 1;
    let left = g.references;
    if left == 0 {
        let g = e.kernel.sound.show.graphs.remove(&at).expect("the graph");
        if g.pin != 0 {
            let connected = e
                .kernel
                .sound
                .show
                .pins
                .get(&g.pin)
                .map_or(0, |p| p.connected);
            if connected != 0 {
                e.vcall(connected, 0x14, &[]);
            }
            pin_disconnect(e, Ptr::new(g.pin));
            pin_release(e, Ptr::new(g.pin));
        }
        if g.filter != 0 {
            e.vcall(g.filter, 0x34, &[0, 0]);
            e.vcall(g.filter, 8, &[]);
        }
        e.mem.free(at);
    }
    left
}

/// `AddFilter(filter, name)`: keeps the filter with a reference and tells it its graph
/// (`IBaseFilter::JoinFilterGraph(graph, name)`); a NULL name becomes the graph's own
/// (`"0001"`, measured). A second filter is not provided.
fn add_filter(e: &mut Engine, this: Ptr, filter: Ptr, name: Ptr) -> u32 {
    let at = graph_of(e, this);
    assert!(
        graph(e, at).filter == 0 && !filter.is_null(),
        "IFilterGraph::AddFilter of a second filter is not provided"
    );
    e.vcall(filter.addr(), 4, &[]);
    graph(e, at).filter = filter.addr();
    let named = if name.is_null() {
        e.kernel.sound.show.named += 1;
        let text: Vec<u16> = format!("{:04}", e.kernel.sound.show.named)
            .encode_utf16()
            .chain([0])
            .collect();
        let block = e.mem.alloc(2 * text.len() as u32);
        for (i, c) in text.iter().enumerate() {
            e.mem.set_u16(block + 2 * i as u32, *c);
        }
        block
    } else {
        name.addr()
    };
    let r = e.vcall(filter.addr(), 0x34, &[at + BUILDER, named]).u32();
    if name.is_null() {
        e.mem.free(named);
    }
    r
}

/// `Render(pin)`: the game's output pin connected to this layer's input pin (the pin's own
/// `Connect`, which agrees the stream's media type and hands this pin its `IAsyncReader`); the
/// whole stream read through that reader (`Length`, `SyncRead`) and decoded. A stream that is
/// not MP3 is not provided.
fn render(e: &mut Engine, this: Ptr, out_pin: Ptr) -> u32 {
    let at = graph_of(e, this);
    let pin = e.mem.alloc(4);
    let table = e.kernel.sound.show.vtables[5];
    e.mem.set_u32(pin, table);
    e.kernel.sound.show.pins.insert(
        pin,
        Pin {
            references: 1,
            connected: 0,
            reader: 0,
        },
    );
    graph(e, at).pin = pin;
    let r = e.vcall(out_pin.addr(), 0xc, &[pin, 0]).u32();
    if (r as i32) < 0 {
        return r;
    }
    let reader = e.kernel.sound.show.pins[&pin].reader;
    assert!(
        reader != 0,
        "IGraphBuilder::Render: the pin gave no IAsyncReader; not provided"
    );
    let lengths = e.mem.alloc(16);
    let r = e.vcall(reader, 0x20, &[lengths, lengths + 8]).u32();
    assert!(
        r == S_OK,
        "IAsyncReader::Length answered {r:#x}: not provided"
    );
    let total = e.mem.u64(lengths);
    e.mem.free(lengths);
    assert!(
        total > 0 && total < 1 << 30,
        "a stream of {total} bytes is not provided"
    );
    let buffer = e.mem.alloc(total as u32);
    let r = e.vcall(reader, 0x1c, &[0, 0, total as u32, buffer]).u32();
    assert!(
        r == S_OK,
        "IAsyncReader::SyncRead answered {r:#x}: not provided"
    );
    let data = e.mem.bytes(buffer, total as u32);
    e.mem.free(buffer);
    let mut decoder = ::mp3::Decoder::new(&data[..]).unwrap_or_else(|err| {
        panic!("IGraphBuilder::Render of a stream mp3 cannot read ({err}): not provided")
    });
    let bitrate = u64::from(decoder.info().bitrate);
    assert!(bitrate != 0, "a free-format MP3 is not provided");
    let (rate, channels) = (decoder.sample_rate(), decoder.channels());
    let samples = windows_samples(&mut decoder);
    let g = graph(e, at);
    g.stream_bytes = total;
    g.duration = (total * 8 * 10_000_000 / bitrate) as i64;
    g.samples = samples;
    g.sample_rate = rate;
    g.channels = channels;
    S_OK
}

/// The samples Windows' graph gives for a stream (measured against the MP3 Decoder DMO's
/// output in the player's graph on all 199 tracks: same length, 99.9748% of the samples the same
/// and the rest one step apart, the decoders' own rounding): every frame decoded, the Xing/Info
/// frame too (silence: its side information is empty), the first 528 frames of output dropped
/// (the decoder's delay), each sample scaled by 32768 and cut toward zero.
fn windows_samples<B: AsRef<[u8]>>(decoder: &mut ::mp3::Decoder<B>) -> Vec<i16> {
    decoder.set_gapless(false);
    let channels = usize::from(decoder.channels());
    let mut all: Vec<f32> = Vec::new();
    if decoder.info().xing.is_some() {
        all.resize(decoder.info().samples_per_frame * channels, 0.0);
    }
    while let Some(frame) = decoder.next_frame() {
        all.extend_from_slice(&frame.samples);
    }
    let skip = (528 * channels).min(all.len());
    all[skip..]
        .iter()
        .map(|&x| ((x * 32768.0).trunc() as i32).clamp(-32768, 32767) as i16)
        .collect()
}

/// The stream position now (100 ns), queueing `EC_COMPLETE` when a run reached the end.
fn position_now(e: &mut Engine, at: u32) -> i64 {
    let now = e.clock_now_us();
    let g = graph(e, at);
    match g.run {
        Run::Running { start } => {
            let p = g.base + (now.saturating_sub(start) * 10) as i64;
            if p >= g.duration {
                if !g.completed {
                    g.completed = true;
                    g.events.push_back((EC_COMPLETE, 0, 0, 0));
                }
                g.duration
            } else {
                p
            }
        }
        _ => g.base,
    }
}

/// `Run()`: see the module documentation.
fn control_run(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let now = e.clock_now_us();
    let g = graph(e, at);
    match g.run {
        Run::Running { .. } => S_OK,
        Run::Paused => {
            g.run = Run::Running { start: now };
            S_OK
        }
        Run::Stopped => {
            if !g.ran {
                g.ran = true;
                g.events.push_back((EC_CLOCK_CHANGED, 0, 0, 0));
            }
            // The pause the graph goes through completes later (measured: the next round).
            g.events.push_back((EC_PAUSED, 0, 0, now + 1));
            g.run = Run::Running { start: now };
            S_FALSE
        }
    }
}

/// `Pause()`: see the module documentation. Pausing a stopped graph is not provided.
fn control_pause(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    let g = graph(e, at);
    assert!(
        g.run != Run::Stopped,
        "IMediaControl::Pause on a stopped graph is not provided"
    );
    g.base = p;
    g.run = Run::Paused;
    g.events.push_back((EC_PAUSED, 0, 0, 0));
    S_OK
}

/// `Stop()`: the position stays where it was (measured).
fn control_stop(e: &mut Engine, this: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    let g = graph(e, at);
    g.base = p;
    g.run = Run::Stopped;
    S_OK
}

/// `get_Duration(out)`: the duration in seconds (`REFTIME`).
fn get_duration(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let d = graph(e, at).duration;
    e.mem.set_f64(out.addr(), d as f64 / 1e7);
    S_OK
}

/// `get_CurrentPosition(out)`: the position in seconds.
fn get_current_position(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let p = position_now(e, at);
    e.mem.set_f64(out.addr(), p as f64 / 1e7);
    S_OK
}

/// `put_CurrentPosition(seconds)`: below 0 `E_INVALIDARG`, past the end the end; a running
/// graph goes on from there.
fn put_current_position(e: &mut Engine, this: Ptr, seconds: f64) -> u32 {
    let at = graph_of(e, this);
    assert!(
        !seconds.is_nan(),
        "put_CurrentPosition(NaN) is not provided"
    );
    if seconds < 0.0 {
        return E_INVALIDARG;
    }
    let now = e.clock_now_us();
    let g = graph(e, at);
    let units = ((seconds * 1e7) as i64).min(g.duration);
    g.base = units;
    g.completed = false;
    if let Run::Running { .. } = g.run {
        g.run = Run::Running { start: now };
    }
    S_OK
}

/// `put_Volume(v)`: -10000..0 (hundredths of a decibel), else `E_INVALIDARG`.
fn put_volume(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let at = graph_of(e, this);
    if !(-10000..=0).contains(&v) {
        return E_INVALIDARG;
    }
    graph(e, at).volume = v;
    S_OK
}

fn get_volume(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let at = graph_of(e, this);
    let v = graph(e, at).volume;
    e.mem.set_i32(out.addr(), v);
    S_OK
}

/// `GetEvent(code, p1, p2, 0)`: the oldest event that can be taken, else `E_ABORT`. A timeout
/// other than 0 is not provided.
fn get_event(e: &mut Engine, this: Ptr, code: Ptr, p1: Ptr, p2: Ptr, timeout: i32) -> u32 {
    assert!(
        timeout == 0,
        "IMediaEvent::GetEvent with a timeout of {timeout} ms is not provided"
    );
    let at = graph_of(e, this);
    let _ = position_now(e, at);
    let now = e.clock_now_us();
    let g = graph(e, at);
    match g.events.front() {
        Some(&(c, a, b, after)) if now >= after => {
            g.events.pop_front();
            e.mem.set_u32(code.addr(), c);
            e.mem.set_u32(p1.addr(), a);
            e.mem.set_u32(p2.addr(), b);
            S_OK
        }
        _ => E_ABORT,
    }
}

/// `FreeEventParams(code, p1, p2)`: this layer's events carry nothing to free.
fn free_event_params(_e: &mut Engine, _this: Ptr, _code: u32, _p1: u32, _p2: u32) -> u32 {
    S_OK
}

fn pin(e: &mut Engine, this: Ptr) -> &mut Pin {
    e.kernel
        .sound
        .show
        .pins
        .get_mut(&this.addr())
        .expect("this layer's input pin")
}

fn pin_query_interface(e: &mut Engine, this: Ptr, interface: Ptr, out: Ptr) -> u32 {
    let wanted = e.mem.bytes(interface.addr(), 16);
    if wanted == iid("IUnknown") || wanted == iid("IPin") {
        pin(e, this).references += 1;
        e.mem.set_u32(out.addr(), this.addr());
        return S_OK;
    }
    e.mem.set_u32(out.addr(), 0);
    E_NOINTERFACE
}

fn pin_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let p = pin(e, this);
    p.references += 1;
    p.references
}

fn pin_release(e: &mut Engine, this: Ptr) -> u32 {
    let p = pin(e, this);
    p.references -= 1;
    let left = p.references;
    if left == 0 {
        e.kernel.sound.show.pins.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `ReceiveConnection(connector, type)`: the stream type the game's pin offers
/// (`MEDIATYPE_Stream`, `MEDIASUBTYPE_MPEG1Audio`) is taken, as the splitter takes it: the
/// connector kept with a reference and asked for its `IAsyncReader` (which the game's pin
/// requires before it completes the connection). Another type is `VFW_E_TYPE_NOT_ACCEPTED`.
fn receive_connection(e: &mut Engine, this: Ptr, connector: Ptr, media_type: Ptr) -> u32 {
    let t = media_type.addr();
    if e.mem.bytes(t, 16) != iid("MEDIATYPE_Stream")
        || e.mem.bytes(t + 0x10, 16) != iid("MEDIASUBTYPE_MPEG1Audio")
    {
        return VFW_E_TYPE_NOT_ACCEPTED;
    }
    e.vcall(connector.addr(), 4, &[]);
    let id = e.mem.alloc(16 + 4);
    e.mem.write(id, &iid("IAsyncReader"));
    let r = e.vcall(connector.addr(), 0, &[id, id + 16]).u32();
    let reader = e.mem.u32(id + 16);
    e.mem.free(id);
    assert!(
        r == S_OK,
        "the game's pin has no IAsyncReader ({r:#x}): not provided"
    );
    let p = pin(e, this);
    p.connected = connector.addr();
    p.reader = reader;
    S_OK
}

/// `Disconnect()`: lets go of the connector and its reader.
fn pin_disconnect(e: &mut Engine, this: Ptr) -> u32 {
    let (connected, reader) = {
        let p = pin(e, this);
        (
            std::mem::take(&mut p.connected),
            std::mem::take(&mut p.reader),
        )
    };
    if reader != 0 {
        e.vcall(reader, 8, &[]);
    }
    if connected != 0 {
        e.vcall(connected, 8, &[]);
        S_OK
    } else {
        S_FALSE
    }
}

/// `QueryDirection(out)`: `PINDIR_INPUT` (0).
fn query_direction(e: &mut Engine, _this: Ptr, out: Ptr) -> u32 {
    e.mem.set_u32(out.addr(), 0);
    S_OK
}

/// `EnumMediaTypes(out)`: this pin proposes no types (the connecting pin's own are tried).
fn enum_media_types(_e: &mut Engine, _this: Ptr, _out: Ptr) -> u32 {
    E_NOTIMPL
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stand-in for the game's filter and pin (the real ones are game code in the engine
    /// crates): `JoinFilterGraph` recorded; the pin's `Connect` calls `ReceiveConnection` with
    /// the stream type, as the game's `CBasePin::Connect` ends up doing; `IAsyncReader` over an
    /// MP3 made of silent frames.
    struct Fake {
        filter: u32,
        pin: u32,
        reader: u32,
    }

    /// One MPEG-1 Layer III frame header, 128 kbit/s, 44100 Hz, mono, then zeros (a silent
    /// frame: no side information, no data).
    fn mp3(frames: usize) -> Vec<u8> {
        let mut v = Vec::new();
        for _ in 0..frames {
            let mut f = vec![0u8; 417];
            f[..4].copy_from_slice(&[0xff, 0xfb, 0x90, 0xc0]);
            v.extend_from_slice(&f);
        }
        v
    }

    fn fake(e: &mut Engine, data: Vec<u8>) -> Fake {
        let base = 0x7f10_0000;
        let vt = |e: &mut Engine, code: u32| {
            let t = e.mem.alloc(4 * 16);
            for s in 0..16 {
                e.mem.set_u32(t + 4 * s, code + 4 * s);
            }
            t
        };
        let filter = e.mem.alloc(4);
        let fv = vt(e, base);
        e.mem.set_u32(filter, fv);
        e.register(base + 4, |_, _| 2u32.into_ret_());
        e.register(base + 8, |_, _| 1u32.into_ret_());
        e.register(base + 0x34, |e, a| {
            e.mem.set_u32(0x7f20_0000, a[1]);
            e.mem.set_u32(0x7f20_0004, a[2]);
            0u32.into_ret_()
        });
        let pin = e.mem.alloc(4);
        let pv = vt(e, base + 0x100);
        e.mem.set_u32(pin, pv);
        let reader = e.mem.alloc(4);
        let rv = vt(e, base + 0x200);
        e.mem.set_u32(reader, rv);
        e.mem.map(0x7f20_0000, 0x1000);
        e.mem.set_u32(0x7f20_0010, reader);
        e.mem.set_u32(0x7f20_0014, pin);
        e.register(base + 0x100 + 4, |_, _| 2u32.into_ret_());
        e.register(base + 0x100 + 8, |_, _| 1u32.into_ret_());
        e.register(base + 0x100, |e, a| {
            // QueryInterface(IID_IAsyncReader)
            let reader = e.mem.u32(0x7f20_0010);
            e.mem.set_u32(a[2], reader);
            0u32.into_ret_()
        });
        e.register(base + 0x100 + 0xc, |e, a| {
            // Connect(receiver, NULL): QueryDirection then ReceiveConnection with the type.
            let (receiver, own) = (a[1], e.mem.u32(0x7f20_0014));
            let t = e.mem.alloc(0x48);
            e.mem.write(t, &iid("MEDIATYPE_Stream"));
            e.mem.write(t + 0x10, &iid("MEDIASUBTYPE_MPEG1Audio"));
            let d = e.mem.alloc(4);
            e.vcall(receiver, 0x24, &[d]);
            assert_eq!(e.mem.u32(d), 0);
            e.vcall(receiver, 0x10, &[own, t])
        });
        e.register(base + 0x100 + 0x14, |_, _| 0u32.into_ret_());
        e.register(base + 0x200 + 8, |_, _| 1u32.into_ret_());
        let size = data.len() as u64;
        e.register_double(base + 0x200 + 0x20, move |e, a| {
            e.mem.set_u64(a[1], size);
            e.mem.set_u64(a[2], size);
            0u32.into_ret_()
        });
        e.register_double(base + 0x200 + 0x1c, move |e, a| {
            assert_eq!((a[1], a[2], a[3] as usize), (0, 0, data.len()));
            e.mem.write(a[4], &data);
            0u32.into_ret_()
        });
        Fake {
            filter,
            pin,
            reader,
        }
    }

    trait IntoRet {
        fn into_ret_(self) -> crate::Ret;
    }
    impl IntoRet for u32 {
        fn into_ret_(self) -> crate::Ret {
            crate::abi::RetVal::into_ret(self)
        }
    }

    /// The music thread's start (`00830b30`): the graph, the filter added, its pin rendered, the
    /// interfaces asked for. Answers the interfaces (graph, control, position, audio, events).
    fn start(e: &mut Engine, f: &Fake) -> [u32; 5] {
        let ids = e.mem.alloc(16 * 6 + 4 * 6);
        let names = [
            "IFilterGraph",
            "IGraphBuilder",
            "IBasicAudio",
            "IMediaControl",
            "IMediaPosition",
            "IMediaEvent",
        ];
        for (i, n) in names.iter().enumerate() {
            e.mem.write(ids + 16 * i as u32, &iid(n));
        }
        let outs = ids + 16 * 6;
        let clsid = e.mem.alloc(16);
        e.mem.write(clsid, &clsid_filter_graph());
        assert_eq!(
            co_create_instance(e, Ptr::new(clsid), 0, 1, Ptr::new(ids), Ptr::new(outs)),
            S_OK
        );
        let fg = e.mem.u32(outs);
        assert_eq!(e.vcall(fg, 0xc, &[f.filter, 0]).u32(), S_OK);
        assert_eq!(e.mem.u32(0x7f20_0000), fg, "JoinFilterGraph got the graph");
        assert_eq!(e.mem.u16(e.mem.u32(0x7f20_0004)), b'0' as u16);
        assert_eq!(e.vcall(fg, 0, &[ids + 16, outs + 4]).u32(), S_OK);
        let gb = e.mem.u32(outs + 4);
        assert_eq!(e.vcall(gb, 0x30, &[f.pin]).u32(), S_OK);
        let mut got = [fg, 0, 0, 0, 0];
        for (k, i) in [(3usize, 3u32), (2, 2), (4, 4), (5, 5)] {
            assert_eq!(e.vcall(gb, 0, &[ids + 16 * i, outs + 4 * i]).u32(), S_OK);
            got[k - 1] = e.mem.u32(outs + 4 * i);
        }
        let _ = f.reader;
        [fg, got[2], got[3], got[1], got[4]]
    }

    fn position(e: &mut Engine, p: u32) -> f64 {
        let out = e.mem.alloc(8);
        e.vcall(p, 0x24, &[out]);
        e.mem.f64(out)
    }

    fn events(e: &mut Engine, ev: u32) -> Vec<u32> {
        let out = e.mem.alloc(12);
        let mut got = Vec::new();
        while e.vcall(ev, 0x20, &[out, out + 4, out + 8, 0]).u32() == S_OK {
            got.push(e.mem.u32(out));
            e.vcall(ev, 0x30, &[e.mem.u32(out), 0, 0]);
        }
        got
    }

    #[test]
    fn the_music_thread_s_calls_are_answered_as_measured() {
        let mut e = Engine::new();
        let data = mp3(100);
        let size = data.len() as u64;
        let f = fake(&mut e, data);
        let [fg, control, pos, audio, ev] = start(&mut e, &f);
        // The samples: 100 frames of 1152 less the decoder's 528, all silent.
        let samples = &e.kernel.sound.show.graphs[&fg].samples;
        assert_eq!(samples.len(), 100 * 1152 - 528);
        assert!(samples.iter().all(|&s| s == 0));
        // Duration: size * 8 * 10^7 / 128000 in 100 ns.
        let out = e.mem.alloc(8);
        e.vcall(pos, 0x1c, &[out]);
        assert_eq!(
            e.mem.f64(out),
            (size * 8 * 10_000_000 / 128_000) as f64 / 1e7
        );
        assert_eq!(
            e.vcall(audio, 0x1c, &[(-10001i32) as u32]).u32(),
            E_INVALIDARG
        );
        assert_eq!(e.vcall(audio, 0x1c, &[(-2000i32) as u32]).u32(), S_OK);
        // The first Run: S_FALSE, EC_CLOCK_CHANGED at once, EC_PAUSED a moment later.
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_FALSE);
        assert_eq!(events(&mut e, ev), [EC_CLOCK_CHANGED]);
        e.clock_ms += 100;
        assert_eq!(events(&mut e, ev), [EC_PAUSED]);
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_OK);
        assert!((position(&mut e, pos) - 0.1).abs() < 1e-9);
        // Pause: EC_PAUSED each time; the position stays.
        assert_eq!(e.vcall(control, 0x20, &[]).u32(), S_OK);
        e.clock_ms += 500;
        assert_eq!(e.vcall(control, 0x20, &[]).u32(), S_OK);
        assert_eq!(events(&mut e, ev), [EC_PAUSED, EC_PAUSED]);
        assert!((position(&mut e, pos) - 0.1).abs() < 1e-9);
        // Run on to the end: EC_COMPLETE, the position the duration.
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_OK);
        e.clock_ms += 10_000;
        assert_eq!(events(&mut e, ev), [EC_COMPLETE]);
        // The loop restart: Stop, put_CurrentPosition(0), Run.
        assert_eq!(e.vcall(control, 0x24, &[]).u32(), S_OK);
        let zero = 0.0f64.to_bits();
        assert_eq!(
            e.vcall(pos, 0x20, &[zero as u32, (zero >> 32) as u32])
                .u32(),
            S_OK
        );
        assert_eq!(position(&mut e, pos), 0.0);
        let minus = (-1.0f64).to_bits();
        assert_eq!(
            e.vcall(pos, 0x20, &[minus as u32, (minus >> 32) as u32])
                .u32(),
            E_INVALIDARG
        );
        assert_eq!(e.vcall(control, 0x1c, &[]).u32(), S_FALSE);
        assert!(
            events(&mut e, ev).is_empty(),
            "EC_PAUSED comes later; no second EC_CLOCK_CHANGED"
        );
        // Released as the game does: everything, then the filter is let go.
        for i in [ev, pos, control, audio] {
            e.vcall(i, 8, &[]);
        }
        e.vcall(fg, 8, &[]);
        assert_eq!(e.vcall(fg, 8, &[]).u32(), 0);
        assert_eq!(e.mem.u32(0x7f20_0000), 0, "JoinFilterGraph(NULL)");
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn another_class_stops() {
        let mut e = Engine::new();
        let ids = e.mem.alloc(32);
        co_create_instance(
            &mut e,
            Ptr::new(ids),
            0,
            1,
            Ptr::new(ids + 16),
            Ptr::new(ids + 16),
        );
    }
}
