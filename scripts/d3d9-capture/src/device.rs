//! `d3d9-capture device`: how the retail Direct3D 9 runtime answers the device calls the game
//! makes, measured on a windowed HAL device with hardware vertex processing on a hidden window
//! (never shown). These answers are the runtime's general behaviour (they do not describe the
//! player's hardware); the platform's layer is written from them. Each line: the call, the case,
//! the HRESULT (hex), and what was read back afterwards where the case says so.
//!
//! `cargo run --release --manifest-path scripts/d3d9-capture/Cargo.toml -- device` (64-bit, as
//! the rest of the tool; the game runs the 32-bit runtime).
//!
//! Modes (a call that faults the runtime is run alone, so the line before it says it was
//! tried): `device` (the draws from memory and the scene checks), `device <case>` (one risky
//! draw: `null-data`, `type-0`, `type-7`, `type-9`, `count-0`, `null-declaration`), `device
//! outside` and `device outside-present` (what makes a draw from memory outside a scene fail),
//! `device fixed` (materials, lights, software vertex processing on a hardware and a mixed
//! device), `device fixed null-material|null-light|get-light`, and `device fixed
//! "texture:<case>"` (one UpdateTexture or StretchRect case, named as in `textures`).
use std::ffi::c_void;
use std::io::Write;

/// `IDirect3DDevice9` slots used here (d3d9.h order).
const RELEASE: usize = 2;
const BEGIN_SCENE: usize = 41;
const END_SCENE: usize = 42;
const CREATE_VERTEX_BUFFER: usize = 26;
const CREATE_INDEX_BUFFER: usize = 27;
const DRAW_INDEXED_PRIMITIVE: usize = 82;
const SET_INDICES: usize = 104;
const DRAW_PRIMITIVE: usize = 81;
const DRAW_PRIMITIVE_UP: usize = 83;
const SET_FVF: usize = 89;
const SET_VERTEX_DECLARATION: usize = 87;
const SET_STREAM_SOURCE: usize = 100;
const GET_STREAM_SOURCE: usize = 101;
const SET_RENDER_STATE: usize = 57;
const SET_TEXTURE: usize = 65;
const PRESENT: usize = 17;

type P = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct PresentParameters {
    width: u32,
    height: u32,
    format: u32,
    count: u32,
    multisample: u32,
    quality: u32,
    swap_effect: u32,
    window: P,
    windowed: i32,
    auto_depth: i32,
    depth_format: u32,
    flags: u32,
    refresh: u32,
    interval: u32,
}

unsafe fn slot(object: P, index: usize) -> usize {
    *(*(object as *const *const usize)).add(index)
}

/// Calls slot `index` of `object` with `args` (all words; at most 8).
unsafe fn call(object: P, index: usize, args: &[usize]) -> u32 {
    let f = slot(object, index);
    match args.len() {
        0 => std::mem::transmute::<usize, unsafe extern "system" fn(P) -> u32>(f)(object),
        1 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize) -> u32>(f)(object, args[0]),
        2 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize) -> u32>(f)(object, args[0], args[1]),
        3 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2]),
        4 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3]),
        5 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4]),
        6 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5]),
        7 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5], args[6]),
        8 => std::mem::transmute::<usize, unsafe extern "system" fn(P, usize, usize, usize, usize, usize, usize, usize, usize) -> u32>(f)(object, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]),
        n => panic!("{n} arguments"),
    }
}

/// The stream 0 binding read back: buffer, offset, stride.
unsafe fn stream0(device: P) -> (usize, u32, u32) {
    let (mut buffer, mut offset, mut stride) = (0usize, 0xaaaa_aaaau32, 0xaaaa_aaaau32);
    call(device, GET_STREAM_SOURCE, &[0, &mut buffer as *mut usize as usize, &mut offset as *mut u32 as usize, &mut stride as *mut u32 as usize]);
    if buffer != 0 {
        // GetStreamSource adds a reference.
        call(buffer as P, RELEASE, &[]);
    }
    (buffer, offset, stride)
}

pub unsafe fn measure(o: &mut impl Write) {
    let instance = crate::GetModuleHandleA(std::ptr::null());
    let name = b"nv-rs d3d9-capture device\0";
    let class = crate::WndClassA {
        style: 3,
        procedure: crate::DefWindowProcA,
        class_extra: 0,
        window_extra: 0,
        instance,
        icon: std::ptr::null_mut(),
        cursor: std::ptr::null_mut(),
        background: std::ptr::null_mut(),
        menu_name: std::ptr::null(),
        class_name: name.as_ptr(),
    };
    crate::RegisterClassA(&class);
    // WS_OVERLAPPED without WS_VISIBLE: the window is never shown.
    let window = crate::CreateWindowExA(0, name.as_ptr(), name.as_ptr(), 0, 0, 0, 320, 240,
        std::ptr::null_mut(), std::ptr::null_mut(), instance, std::ptr::null_mut());
    assert!(!window.is_null());
    let d3d = crate::Direct3DCreate9(32) as P;
    assert!(!d3d.is_null());
    // D3DPRESENT_PARAMETERS: 320x240, D3DFMT_UNKNOWN back buffer, one, discard, windowed,
    // automatic D24S8 depth.
    let mut pp = PresentParameters {
        width: 320,
        height: 240,
        format: 0,
        count: 1,
        multisample: 0,
        quality: 0,
        swap_effect: 1,
        window,
        windowed: 1,
        auto_depth: 1,
        depth_format: 75,
        flags: 0,
        refresh: 0,
        interval: 0,
    };
    let mut device: P = std::ptr::null_mut();
    // IDirect3D9::CreateDevice (slot 16): adapter 0, HAL, window, hardware vertex processing.
    let hr = call(d3d, 16, &[0, 1, window as usize, 0x40, &mut pp as *mut PresentParameters as usize, &mut device as *mut P as usize]);
    writeln!(o, "CreateDevice\thal windowed hardware\t{hr:x}").unwrap(); o.flush().unwrap();
    assert!(!device.is_null(), "no device");

    // The vertices the movie player draws: XYZRHW | TEX1 (FVF 0x104), 0x18 bytes each.
    let vertices = [0f32; 6 * 6];
    let data = vertices.as_ptr() as usize;
    let up = |device: P, kind: usize, count: usize, data: usize, stride: usize| call(device, DRAW_PRIMITIVE_UP, &[kind, count, data, stride]);

    // `device outside`: which condition made the first draws from memory fail: each step on
    // the fresh device, outside any scene, reported in turn.
    if std::env::args().nth(2).as_deref() == Some("outside") {
        call(device, SET_FVF, &[0x104]);
        let mut buffer: P = std::ptr::null_mut();
        call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let hr = up(device, 5, 2, data, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP\tfresh, fvf, a vertex buffer on stream 0, never in a scene\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
        call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfresh, fvf, no stream 0, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, END_SCENE, &[]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after a draw in a scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        call(device, BEGIN_SCENE, &[]);
        call(device, END_SCENE, &[]);
        call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "DrawPrimitiveUP\tfvf, after an empty scene and a Present\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        // A draw from a vertex buffer on a fresh device's history is the other check: a second
        // device below.
        o.flush().unwrap();
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("outside-present") {
        call(device, SET_FVF, &[0x104]);
        writeln!(o, "Present\tfresh\t{:x}", call(device, PRESENT, &[0, 0, 0, 0])).unwrap(); o.flush().unwrap();
        writeln!(o, "DrawPrimitiveUP\tfvf, no stream 0, after a Present, never in a scene\t{:x}", up(device, 5, 2, data, 0x18)).unwrap(); o.flush().unwrap();
        o.flush().unwrap();
        return;
    }
    if std::env::args().nth(2).as_deref() == Some("fixed") {
        fixed_function(o, device);
        if let Some(case) = std::env::args().nth(3).filter(|a| a.starts_with("texture:")) {
            textures(o, device, &case[8..]);
            return;
        }
        call(device, RELEASE, &[]);
        // The same calls on a device with mixed vertex processing (0x80).
        let mut mixed: P = std::ptr::null_mut();
        let mut pp2 = PresentParameters { ..pp };
        let hr = call(d3d, 16, &[0, 1, window as usize, 0x80, &mut pp2 as *mut PresentParameters as usize, &mut mixed as *mut P as usize]);
        writeln!(o, "CreateDevice\thal windowed mixed\t{hr:x}").unwrap(); o.flush().unwrap();
        if !mixed.is_null() {
            software_vertex_processing(o, mixed, "mixed");
            call(mixed, RELEASE, &[]);
        }
        call(d3d, RELEASE, &[]);
        return;
    }
    // Without an FVF or a declaration.
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\tno fvf, outside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_FVF, &[0x104]);
    let hr = up(device, 5, 2, data, 0x18);
    writeln!(o, "DrawPrimitiveUP\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    for (case, kind, count, d, stride) in [
        ("strip 2", 5usize, 2usize, data, 0x18usize),
        ("list 6 (movie quads)", 2, 6, data, 0x18),
        ("type 1 points", 1, 2, data, 0x18),
        ("type 3 lines", 3, 1, data, 0x18),
        ("stride 0", 5, 2, data, 0),
        ("stride 4", 5, 2, data, 4),
    ] {
        let hr = up(device, kind, count, d, stride);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {case}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    // Stream 0 after a draw from memory: with a vertex buffer bound before.
    let mut buffer: P = std::ptr::null_mut();
    let hr = call(device, CREATE_VERTEX_BUFFER, &[0x18 * 6, 8, 0x104, 0, &mut buffer as *mut P as usize, 0]);
    writeln!(o, "CreateVertexBuffer\t0x90 bytes, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let before = stream0(device);
    writeln!(o, "GetStreamSource\tafter SetStreamSource\tbuffer {} offset {:x} stride {:x}", before.0 != 0, before.1, before.2).unwrap(); o.flush().unwrap();
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();

    // The cases that may fault inside the runtime, one per run (`device <case>`): the line
    // before the call says it was tried; EndScene and Present follow, each reported.
    let risky = std::env::args().nth(2).unwrap_or_default();
    if !risky.is_empty() {
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\ttrying").unwrap(); o.flush().unwrap();
        call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
        let (kind, count, d) = match risky.as_str() {
            "null-data" => (5, 2, 0),
            "type-0" => (0, 2, data),
            "type-7" => (7, 2, data),
            "type-9" => (9, 2, data),
            "count-0" => (5, 0, data),
            _ => {
                call(device, SET_VERTEX_DECLARATION, &[0]);
                (5, 2, data)
            }
        };
        let hr = up(device, kind, count, d, 0x18);
        let after = stream0(device);
        writeln!(o, "DrawPrimitiveUP\tin a scene, {risky}\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
        let hr = call(device, END_SCENE, &[]);
        writeln!(o, "EndScene\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        let hr = call(device, PRESENT, &[0, 0, 0, 0]);
        writeln!(o, "Present\tafter {risky}\t{hr:x}").unwrap(); o.flush().unwrap();
        return;
    }
    writeln!(o, "EndScene\tafter the draws\t{:x}", call(device, END_SCENE, &[])).unwrap(); o.flush().unwrap();
    // A draw from a vertex buffer outside a scene.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0, 0x18]);
    let hr = call(device, DRAW_PRIMITIVE, &[5, 0, 2]);
    writeln!(o, "DrawPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // An indexed draw outside a scene (16-bit indices, write-only, default pool).
    let mut indices: P = std::ptr::null_mut();
    let hr = call(device, CREATE_INDEX_BUFFER, &[12, 8, 101, 0, &mut indices as *mut P as usize, 0]);
    writeln!(o, "CreateIndexBuffer\t12 bytes, 16-bit, write-only, default pool\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[indices as usize]);
    let hr = call(device, DRAW_INDEXED_PRIMITIVE, &[4, 0, 0, 4, 0, 2]);
    writeln!(o, "DrawIndexedPrimitive\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_INDICES, &[0]);
    call(indices, RELEASE, &[]);
    // Stream 0 after a draw from memory that fails: outside a scene, and with stride 0.
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0x18);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\toutside a scene, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    call(device, BEGIN_SCENE, &[]);
    call(device, SET_STREAM_SOURCE, &[0, buffer as usize, 0x18, 0x18]);
    let hr = up(device, 5, 2, data, 0);
    let after = stream0(device);
    writeln!(o, "DrawPrimitiveUP\tin a scene, stride 0, a vertex buffer on stream 0\t{hr:x}\tafter: buffer {} offset {:x} stride {:x}", after.0 != 0, after.1, after.2).unwrap(); o.flush().unwrap();
    let hr = call(device, BEGIN_SCENE, &[]);
    writeln!(o, "BeginScene\tinside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, END_SCENE, &[]);
    let hr = call(device, END_SCENE, &[]);
    writeln!(o, "EndScene\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    // State calls outside a scene.
    let hr = call(device, SET_RENDER_STATE, &[7, 0]);
    writeln!(o, "SetRenderState\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, SET_TEXTURE, &[0, 0]);
    writeln!(o, "SetTexture\toutside a scene, null\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, PRESENT, &[0, 0, 0, 0]);
    writeln!(o, "Present\toutside a scene\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, SET_STREAM_SOURCE, &[0, 0, 0, 0]);
    call(buffer, RELEASE, &[]);
    let refs = call(device, RELEASE, &[]);
    writeln!(o, "Release\tdevice\t{refs:x}").unwrap(); o.flush().unwrap();
    call(d3d, RELEASE, &[]);
    crate::DestroyWindow(window);
    crate::UnregisterClassA(name.as_ptr(), instance);
}

const SET_MATERIAL: usize = 49;
const SET_LIGHT: usize = 51;
const GET_LIGHT: usize = 52;
const LIGHT_ENABLE: usize = 53;
const GET_LIGHT_ENABLE: usize = 54;
const SET_SOFTWARE_VERTEX_PROCESSING: usize = 77;
const GET_SOFTWARE_VERTEX_PROCESSING: usize = 78;
const CREATE_TEXTURE: usize = 23;
const UPDATE_TEXTURE: usize = 31;
const STRETCH_RECT: usize = 34;
const CREATE_RENDER_TARGET: usize = 28;
const CREATE_OFFSCREEN_PLAIN_SURFACE: usize = 36;
const TEXTURE_GET_SURFACE_LEVEL: usize = 18;

fn words(w: &[u32]) -> String {
    w.iter().map(|x| format!("{x:08x}")).collect::<Vec<_>>().join(" ")
}

/// A `D3DLIGHT9` (26 words): type, diffuse, specular, ambient (4 floats each), position,
/// direction (3 each), range, falloff, attenuation 0..2, theta, phi.
fn light(kind: u32) -> [u32; 26] {
    let f = |v: f32| v.to_bits();
    let mut l = [0u32; 26];
    l[0] = kind;
    for w in l.iter_mut().take(13).skip(1) {
        *w = f(1.0);
    }
    l[18] = f(1.0); // direction (0, 0, 1)
    l[19] = f(100.0); // range
    l[20] = f(1.0); // falloff
    l[21] = f(1.0); // attenuation 0
    l[24] = f(0.5); // theta
    l[25] = f(1.0); // phi
    l
}

unsafe fn fixed_function(o: &mut impl Write, device: P) {
    let mut material = [0u32; 17];
    for w in material.iter_mut().take(16) {
        *w = 1f32.to_bits();
    }
    let null = std::env::args().nth(3).unwrap_or_default();
    if null == "null-material" || null == "null-light" {
        writeln!(o, "{null}\ttrying").unwrap();
        o.flush().unwrap();
        let hr = if null == "null-material" { call(device, SET_MATERIAL, &[0]) } else { call(device, SET_LIGHT, &[0, 0]) };
        writeln!(o, "{null}\t{hr:x}").unwrap(); o.flush().unwrap();
        return;
    }
    for (case, ptr) in [("a material", material.as_mut_ptr() as usize)] {
        writeln!(o, "SetMaterial\t{case}\t{:x}", call(device, SET_MATERIAL, &[ptr])).unwrap(); o.flush().unwrap();
    }
    let f = |v: f32| v.to_bits();
    let mut cases: Vec<(&str, u32, [u32; 26])> = vec![
        ("point", 0, light(1)),
        ("spot", 1, light(2)),
        ("directional", 2, light(3)),
        ("type 0", 3, light(0)),
        ("type 4", 3, light(4)),
        ("index 9", 9, light(1)),
    ];
    let edit = |kind: u32, at: usize, value: u32| {
        let mut l = light(kind);
        l[at] = value;
        l
    };
    cases.push(("point, attenuation all 0", 4, edit(1, 21, 0)));
    cases.push(("point, range -1", 4, edit(1, 19, f(-1.0))));
    cases.push(("point, range 0", 4, edit(1, 19, 0)));
    cases.push(("point, range 1e30", 4, edit(1, 19, f(1e30))));
    cases.push(("directional, direction 0", 4, edit(3, 18, 0)));
    cases.push(("spot, theta above phi", 4, edit(2, 24, f(2.0))));
    cases.push(("spot, phi above pi", 4, edit(2, 25, f(4.0))));
    cases.push(("point, attenuation 0 negative", 4, edit(1, 21, f(-1.0))));
    cases.push(("point, NaN diffuse", 4, edit(1, 1, 0x7fc0_0000)));
    for (case, index, l) in &cases {
        let hr = call(device, SET_LIGHT, &[*index as usize, l.as_ptr() as usize]);
        writeln!(o, "SetLight\t{case} at {index}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    let mut back = [0xaaaa_aaaau32; 26];
    if std::env::args().nth(3).as_deref() == Some("get-light") {
        writeln!(o, "GetLight\tat 0 (a point light set)\t{:x}\t{}", call(device, GET_LIGHT, &[0, back.as_mut_ptr() as usize]), words(&back)).unwrap(); o.flush().unwrap();
        writeln!(o, "GetLight\tat 7 never set\t{:x}", call(device, GET_LIGHT, &[7, back.as_mut_ptr() as usize])).unwrap(); o.flush().unwrap();
        return;
    }
    for (case, index, enable) in [
        ("set light 0, TRUE", 0usize, 1usize),
        ("set light 0, FALSE", 0, 0),
        ("set light 0, 2", 0, 2),
        ("never set 7, TRUE", 7, 1),
        ("never set 8, FALSE", 8, 0),
        ("index 9 set, TRUE", 9, 1),
    ] {
        let hr = call(device, LIGHT_ENABLE, &[index, enable]);
        let mut on = 0xaaaa_aaaau32;
        let hr2 = call(device, GET_LIGHT_ENABLE, &[index, &mut on as *mut u32 as usize]);
        writeln!(o, "LightEnable\t{case}\t{hr:x}\tGetLightEnable {hr2:x} {on:x}").unwrap(); o.flush().unwrap();
    }
    let hr = call(device, GET_LIGHT, &[7, back.as_mut_ptr() as usize]);
    writeln!(o, "GetLight\tat 7 after LightEnable\t{hr:x}\t{}", words(&back)).unwrap(); o.flush().unwrap();
    for index in 10..20usize {
        let hr = call(device, LIGHT_ENABLE, &[index, 1]);
        writeln!(o, "LightEnable\tTRUE, light {index}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    software_vertex_processing(o, device, "hardware");
}

unsafe fn software_vertex_processing(o: &mut impl Write, device: P, kind: &str) {
    for value in [1usize, 0, 2] {
        let hr = call(device, SET_SOFTWARE_VERTEX_PROCESSING, &[value]);
        let now = call(device, GET_SOFTWARE_VERTEX_PROCESSING, &[]);
        writeln!(o, "SetSoftwareVertexProcessing\t{kind} device, {value}\t{hr:x}\tGet {now:x}").unwrap(); o.flush().unwrap();
    }
}

unsafe fn textures(o: &mut impl Write, device: P, only: &str) {
    let texture = |w: usize, h: usize, levels: usize, usage: usize, format: usize, pool: usize| {
        let mut t: P = std::ptr::null_mut();
        call(device, CREATE_TEXTURE, &[w, h, levels, usage, format, pool, &mut t as *mut P as usize, 0]);
        t
    };
    // UpdateTexture: system memory to default pool, A8R8G8B8 (21), 64x64.
    let sys = texture(64, 64, 0, 0, 21, 2);
    let sys1 = texture(64, 64, 1, 0, 21, 2);
    let sys_small = texture(32, 32, 0, 0, 21, 2);
    let sys_x8 = texture(64, 64, 0, 0, 22, 2);
    let def = texture(64, 64, 0, 0, 21, 0);
    let def1 = texture(64, 64, 1, 0, 21, 0);
    let dynamic = texture(64, 64, 1, 0x200, 21, 0);
    let managed = texture(64, 64, 0, 0, 21, 1);
    for (case, src, dst) in [
        ("sysmem full chain to default full chain", sys, def),
        ("sysmem 1 level to default 1 level", sys1, def1),
        ("sysmem 1 level to dynamic default 1 level", sys1, dynamic),
        ("sysmem full chain to default 1 level", sys, def1),
        ("sysmem 1 level to default full chain", sys1, def),
        ("sysmem 32x32 to default 64x64", sys_small, def),
        ("sysmem X8R8G8B8 to default A8R8G8B8", sys_x8, def),
        ("default to default", def, def1),
        ("sysmem to managed", sys, managed),
        ("sysmem to sysmem", sys, sys1),
        ("null source", std::ptr::null_mut(), def),
    ] {
        if case != only {
            continue;
        }
        writeln!(o, "UpdateTexture\t{case}\ttrying").unwrap(); o.flush().unwrap();
        writeln!(o, "UpdateTexture\t{case}\t{:x}", call(device, UPDATE_TEXTURE, &[src as usize, dst as usize])).unwrap(); o.flush().unwrap();
    }
    let level = |t: P| {
        let mut s: P = std::ptr::null_mut();
        call(t, TEXTURE_GET_SURFACE_LEVEL, &[0, &mut s as *mut P as usize]);
        s
    };
    let target = |w: usize, h: usize, format: usize| {
        let mut s: P = std::ptr::null_mut();
        call(device, CREATE_RENDER_TARGET, &[w, h, format, 0, 0, 0, &mut s as *mut P as usize, 0]);
        s
    };
    let plain = |pool: usize| {
        let mut s: P = std::ptr::null_mut();
        call(device, CREATE_OFFSCREEN_PLAIN_SURFACE, &[64, 64, 21, pool, &mut s as *mut P as usize, 0]);
        s
    };
    let rt_a = target(64, 64, 21);
    let rt_b = target(64, 64, 21);
    let rt_big = target(128, 128, 21);
    let rt_x8 = target(64, 64, 22);
    let rt_tex_level = level(texture(64, 64, 1, 1, 21, 0));
    let def_level = level(def1);
    let sys_level = level(sys1);
    let plain_default = plain(0);
    let plain_sys = plain(2);
    let rect = [0i32, 0, 32, 32];
    let r = rect.as_ptr() as usize;
    for (case, src, src_rect, dst, dst_rect, filter) in [
        ("render target to render target", rt_a, 0, rt_b, 0, 0usize),
        ("render target to render target, linear", rt_a, 0, rt_b, 0, 2),
        ("render target to render target, point", rt_a, 0, rt_b, 0, 1),
        ("render target to a bigger render target", rt_a, 0, rt_big, 0, 0),
        ("render target to render target, rects", rt_a, r, rt_b, r, 0),
        ("render target to X8R8G8B8 render target", rt_a, 0, rt_x8, 0, 0),
        ("render target to render-target texture level", rt_a, 0, rt_tex_level, 0, 0),
        ("render-target texture level to render target", rt_tex_level, 0, rt_a, 0, 0),
        ("default plain surface to render target", plain_default, 0, rt_a, 0, 0),
        ("render target to default plain surface", rt_a, 0, plain_default, 0, 0),
        ("default texture level to render target", def_level, 0, rt_a, 0, 0),
        ("render target to default texture level", rt_a, 0, def_level, 0, 0),
        ("sysmem plain surface to render target", plain_sys, 0, rt_a, 0, 0),
        ("sysmem texture level to render target", sys_level, 0, rt_a, 0, 0),
        ("render target to itself", rt_a, 0, rt_a, 0, 0),
        ("render target to null", rt_a, 0, std::ptr::null_mut(), 0, 0),
    ] {
        if case != only {
            continue;
        }
        writeln!(o, "StretchRect\t{case}\ttrying").unwrap(); o.flush().unwrap();
        let hr = call(device, STRETCH_RECT, &[src as usize, src_rect, dst as usize, dst_rect, filter]);
        writeln!(o, "StretchRect\t{case}\t{hr:x}").unwrap(); o.flush().unwrap();
    }
    if only != "in a scene" {
        return;
    }
    call(device, BEGIN_SCENE, &[]);
    let hr = call(device, STRETCH_RECT, &[rt_a as usize, 0, rt_b as usize, 0, 0]);
    writeln!(o, "StretchRect\tin a scene, render target to render target\t{hr:x}").unwrap(); o.flush().unwrap();
    let hr = call(device, UPDATE_TEXTURE, &[sys1 as usize, def1 as usize]);
    writeln!(o, "UpdateTexture\tin a scene, sysmem 1 level to default 1 level\t{hr:x}").unwrap(); o.flush().unwrap();
    call(device, END_SCENE, &[]);
}
