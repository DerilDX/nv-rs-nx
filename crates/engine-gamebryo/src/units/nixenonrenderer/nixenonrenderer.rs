//! `NiXenonRenderer/nixenonrenderer.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_a9a0, ni_xenon_renderer_get_d3d_format_string(i32) -> u32),
        entry!(0x00e6_9e50, fn_00e69e50(u32) -> u32),
        entry!(0x00e6_c550, fn_00e6c550(Ptr) -> u8),
        entry!(0x00e6_9460, ni_xenon_renderer_create_direct3d9(u8) -> i32),
        (0x00e7_2e60, fn_00e72e60_entry as AbiFn),
        entry!(0x00e6_9510, fn_00e69510(u32, Ptr, Ptr) -> u8),
        entry!(0x00e6_9c90, fn_00e69c90(u32) -> u32),
        entry!(0x00e6_9d00, fn_00e69d00(u32) -> u32),
        entry!(0x00e6_e6e0, fn_00e6e6e0(Ptr, u32)),
        entry!(0x00e7_2cf0, fn_00e72cf0(Ptr, u32, u32)),
        entry!(0x00e6_b990, fn_00e6b990(Ptr)),
        (0x00e6_bf50, fn_00e6bf50_entry as AbiFn),
        entry!(0x00e6_c240, fn_00e6c240(Ptr, u32) -> u8),
        entry!(0x00e6_ec90, fn_00e6ec90(Ptr) -> u8),
        entry!(0x00e7_45a0, fn_00e745a0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e6_eea0,
            fn_00e6eea0(Ptr<NiDX9Renderer>, u32, u32) -> bool
        ),
        entry!(0x00e6_f310, fn_00e6f310(Ptr<NiDX9Renderer>, u32, u32)),
        entry!(0x00e7_4840, fn_00e74840(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_3f60,
            ni_xenon_renderer_precache_geometry(Ptr<NiDX9Renderer>, u32, u32, u32, u32) -> bool
        ),
        entry!(0x00a7_65c0, ni_pixel_format_operator_not_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_50f0, ni_vertex_color_property_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00b7_99a0, ni_d3d_utility_get_d3d_from_ni(u32, Ptr) -> u32),
        entry!(0x00e6_9410, fn_00e69410() -> bool),
        entry!(0x00e6_95e0, fn_00e695e0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_95f0, fn_00e695f0(Ptr<NiDX9Renderer>) -> u8),
        entry!(0x00e6_9600, fn_00e69600(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9620, fn_00e69620(Ptr<NiDX9Renderer>) -> u32),
        entry!(
            0x00e6_9640,
            fn_00e69640(Ptr<NiDX9Renderer>, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_9660, fn_00e69660(Ptr<NiDX9Renderer>) -> bool),
        entry!(0x00e6_96a0, fn_00e696a0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9790, fn_00e69790(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e6_9960, fn_00e69960(Ptr<NiDX9Renderer>, u32) -> u32),
        entry!(
            0x00e6_9970,
            ni_xenon_renderer_purge_vertex_shader(Ptr<NiDX9Renderer>, u32) -> u32
        ),
        entry!(
            0x00e6_99c0,
            ni_xenon_renderer_purge_pixel_shader(Ptr<NiDX9Renderer>, u32) -> u32
        ),
        entry!(
            0x00e6_9a20,
            fn_00e69a20(Ptr<NiDX9Renderer>, u32, u32, u32) -> bool
        ),
        entry!(0x00e6_9a60, fn_00e69a60() -> u32),
        entry!(0x00e6_9b00, fn_00e69b00(Ptr, u32) -> u32),
        entry!(0x00e6_9b40, fn_00e69b40(Ptr, u32) -> u32),
        entry!(0x00e6_d740, fn_00e6d740(Ptr, Ptr)),
        entry!(0x00e6_dc40, fn_00e6dc40(Ptr<NiDX9Renderer>, Ptr) -> bool),
        entry!(
            0x00e6_dc80,
            fn_00e6dc80(Ptr<NiDX9Renderer>, Ptr, u32) -> bool
        ),
        entry!(0x00e6_dcc0, fn_00e6dcc0(Ptr<NiDX9Renderer>, Ptr) -> bool),
        entry!(0x00e6_df80, fn_00e6df80(Ptr<NiDX9Renderer>, Ptr) -> u32),
        entry!(
            0x00e6_ee00,
            fn_00e6ee00(Ptr<NiDX9Renderer>, u32, u32, u32) -> u32
        ),
        entry!(0x00e6_fa90, fn_00e6fa90(Ptr, Ptr, Ptr, u32)),
        entry!(0x00e7_1cc0, fn_00e71cc0(Ptr<NiDX9Renderer>, u32)),
        entry!(0x00e7_1cf0, fn_00e71cf0(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_2c20,
            fn_00e72c20(Ptr<NiDX9Renderer>, Ptr, u32) -> bool
        ),
        entry!(0x00e7_2e50, fn_00e72e50(Ptr) -> u32),
        entry!(0x00e7_5310, fn_00e75310(Ptr<NiDX9Renderer>, u32)),
        entry!(0x00e7_5340, fn_00e75340(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_5380,
            ni_xenon_renderer_purge_all_d3d_shaders(Ptr<NiDX9Renderer>)
        ),
        entry!(0x00e7_57d0, fn_00e757d0(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_5810, fn_00e75810(Ptr<NiDX9Renderer>) -> u32),
        entry!(0x00e7_5840, fn_00e75840(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_58f0, fn_00e758f0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(0x00e7_59c0, fn_00e759c0(Ptr<NiDX9Renderer>, Ptr)),
        entry!(
            0x00e7_62f0,
            fn_00e762f0(Ptr<NiDX9Renderer>, u32) -> Ptr<NiDX9Renderer>
        ),
        entry!(0x00e7_bc90, fn_00e7bc90(Ptr, u32, u32) -> Ptr),
        entry!(
            0x00e6_da60,
            fn_00e6da60(Ptr<NiDX9Renderer>, Ptr, Ptr, u32, u32) -> bool
        ),
        entry!(0x00e7_1e40, fn_00e71e40(Ptr<NiDX9Renderer>, u32)),
        entry!(
            0x00e7_2980,
            fn_00e72980(Ptr<NiDX9Renderer>, u32, Ptr, u32, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x00e7_2aa0,
            fn_00e72aa0(Ptr<NiDX9Renderer>, u32, u32, u32, u32, u32, u32) -> bool
        ),
    ]
}

// ---- Direct3D creation and the renderer's initialisation (batch b000t) ----

/// The Direct3D globals: the `IDirect3D9` object (`0126f0d8`), the byte set when it is an
/// `IDirect3D9Ex` (`0126f0d0`), and the two creation functions `00e69410` looks up
/// (`Direct3DCreate9` at `0126f0dc`, `Direct3DCreate9Ex` at `0126f0e0`; both `stdcall`).
const DIRECT3D: u32 = 0x0126_f0d8;
const DIRECT3D_IS_EX: u32 = 0x0126_f0d0;
const DIRECT3D_CREATE9: u32 = 0x0126_f0dc;
const DIRECT3D_CREATE9_EX: u32 = 0x0126_f0e0;
/// `00e69410` (unnamed, `nixenonrenderer.cpp`; no arguments): a nonzero byte when the
/// Direct3D library is usable.
const LOAD_DIRECT3D: u32 = 0x00e6_9410;
/// `00a29680` (unnamed; `cdecl`, a message) and the messages `CreateDirect3D9` passes it.
const REPORT_00A29680: u32 = 0x00a2_9680;
const CREATE_EX_FAILED_010ED598: u32 = 0x010e_d598;
const CREATE_EX_SUCCEEDED_010ED570: u32 = 0x010e_d570;
const CREATE_FAILED_010ED550: u32 = 0x010e_d550;
const CREATE_SUCCEEDED_010ED52C: u32 = 0x010e_d52c;
/// The SDK version both creation functions are given.
const D3D_SDK_VERSION: u32 = 0x20;

// Translated from 00e69460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::CreateDirect3D9` (engine map), `cdecl`, one word (its low byte: try the
/// `Ex` interface). Answers 0, or -1 on failure; with the object already made (`0126f0d8`) it
/// answers 0 at once. When the library is not usable (`00e69410`) it reports "Create D3D9
/// instance...FAILED" and answers -1. With `Direct3DCreate9Ex` found and the byte set it calls
/// it (version 0x20, an out word): a null object reports "Create D3D9Ex
/// instance...FAILED" and makes the answer -1 (the plain creation is still tried), otherwise
/// the `Ex` byte and the object are set and "...SUCCESSFUL" reported. Without an object then,
/// `Direct3DCreate9(0x20)` makes it: null reports "Create D3D9 instance...FAILED" and answers
/// -1; otherwise it reports "...SUCCESSFUL" and answers the earlier result.
pub fn ni_xenon_renderer_create_direct3d9(e: &mut Engine, try_ex: u8) -> i32 {
    let mut result = 0i32;
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    if e.call(LOAD_DIRECT3D, &args![]).u8() == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    let create_ex = e.mem.u32(DIRECT3D_CREATE9_EX);
    if create_ex != 0 && try_ex != 0 {
        // The out word is the slot of the pushed ECX.
        let object = e.with_stack(4, |e, out| {
            e.call(create_ex, &args![D3D_SDK_VERSION, out]);
            e.mem.u32(out.addr())
        });
        if object == 0 {
            e.call(REPORT_00A29680, &args![CREATE_EX_FAILED_010ED598]);
            result = -1;
        } else {
            e.mem.set_u8(DIRECT3D_IS_EX, 1);
            e.mem.set_u32(DIRECT3D, object);
            e.call(REPORT_00A29680, &args![CREATE_EX_SUCCEEDED_010ED570]);
        }
    }
    if e.mem.u32(DIRECT3D) != 0 {
        return result;
    }
    let create = e.mem.u32(DIRECT3D_CREATE9);
    let object = e.call(create, &args![D3D_SDK_VERSION]).u32();
    e.mem.set_u32(DIRECT3D, object);
    if object == 0 {
        e.call(REPORT_00A29680, &args![CREATE_FAILED_010ED550]);
        return -1;
    }
    e.call(REPORT_00A29680, &args![CREATE_SUCCEEDED_010ED52C]);
    result
}

/// `IDirect3D9::CreateDevice` (vtable slot 0x40) and `IDirect3D9Ex::CreateDeviceEx` (0x50),
/// named by the interfaces' slot order; the arguments match.
const CREATE_DEVICE: u32 = 0x40;
const CREATE_DEVICE_EX: u32 = 0x50;
/// The adapter list (`0126f0e8`: element pointer +8, 16-bit count +0xe).
const ADAPTER_LIST: u32 = 0x0126_f0e8;
/// Globals the initialisation clears first (a `float` and a byte).
const ZEROED_VALUE_0126EFD4: u32 = 0x0126_efd4;
const ZEROED_FLAG_0126EFD1: u32 = 0x0126_efd1;
/// The error text (`011f4648`, 0x100 bytes, filled by `_strncpy_s` `00ec8d5f` or by
/// `00e69ad0`, `cdecl` with a message).
const ERROR_MESSAGE_011F4648: u32 = 0x011f_4648;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const SET_ERROR_00E69AD0: u32 = 0x00e6_9ad0;
/// The callees of the initialisation (`nixenonrenderer.cpp` unless noted; unnamed in the
/// engine map unless noted).
const RESET_00E69A60: u32 = 0x00e6_9a60;
const DEVICE_SETTINGS_00E69510: u32 = 0x00e6_9510;
const PRESENT_PARAMETERS_00E6BF50: u32 = 0x00e6_bf50;
const AFTER_DEVICE_00E6C240: u32 = 0x00e6_c240;
const BACK_BUFFER_00E7D6B0: u32 = 0x00e7_d6b0;
const DEPTH_STENCIL_00E7D850: u32 = 0x00e7_d850;
const RENDER_TARGET_GROUP_00EE8710: u32 = 0x00ee_8710;
const NI_POINTER_CONSTRUCT_00B6AED0: u32 = 0x00b6_aed0;
const WINDOW_TARGET_00E72CF0: u32 = 0x00e7_2cf0;
const FORMAT_00E69D00: u32 = 0x00e6_9d00;
const FORMAT_00E69C90: u32 = 0x00e6_9c90;
const TEXTURE_DEFAULTS_00E6EC90: u32 = 0x00e6_ec90;
const SHADER_LIBRARY_00E91590: u32 = 0x00e9_1590;
const INITIALIZE_SHADER_SYSTEM: u32 = 0x00e8_1940;
const NEW_OBJECT: u32 = 0x00aa_13e0;
const CONSTRUCT_00E8D0E0: u32 = 0x00e8_d0e0;
const CONSTRUCT_00E8FB20: u32 = 0x00e8_fb20;
const CONSTRUCT_00E90A80: u32 = 0x00e9_0a80;
const GEOMETRY_GROUP_MANAGER_00E88EB0: u32 = 0x00e8_8eb0;
const SET_FLAG_00E88C20: u32 = 0x00e8_8c20;
const CONSTRUCT_00E8BAE0: u32 = 0x00e8_bae0;
const NI_FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
const NI_SHADER_FACTORY_GET_INSTANCE: u32 = 0x00a9_fff0;
const CONSTRUCT_00E909E0: u32 = 0x00e9_09e0;
const SET_SHADER_00E6E6E0: u32 = 0x00e6_e6e0;
/// The `float` constants of the default frustum (`01012054`, `0101e2bc`), the `float` 2^32
/// (`01099378`) and the `double` the height-to-width ratio is compared with (`0101de30`).
const FRUSTUM_SIDE_01012054: u32 = 0x0101_2054;
const FRUSTUM_NEAR_0101E2BC: u32 = 0x0101_e2bc;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const RATIO_LIMIT_0101DE30: u32 = 0x0101_de30;
/// The four words passed to the renderer's virtual slot 0x18c with the frustum and viewport.
const CAMERA_WORDS_SLOT_18C: [u32; 4] = [0x011f_426c, 0x011a_946c, 0x011a_9478, 0x011a_9484];
/// The messages and error texts of the initialisation.
const INVALID_DEVICE_TYPE_010EE20C: u32 = 0x010e_e20c;
const INVALID_DEVICE_ID_010EE1C8: u32 = 0x010e_e1c8;
const INVALID_ADAPTER_010EE1A0: u32 = 0x010e_e1a0;
const INVALID_ADAPTER_REPORT_010EE160: u32 = 0x010e_e160;
const ATTEMPTING_010EE118: u32 = 0x010e_e118;
const DEVICE_NOT_VALID_010EE0E8: u32 = 0x010e_e0e8;
const NO_DEVICE_OF_TYPE_010EE0AC: u32 = 0x010e_e0ac;
const FEWER_BACK_BUFFERS_010EE048: u32 = 0x010e_e048;
const NEXT_LOWER_TYPE_010EDFE0: u32 = 0x010e_dfe0;
const NO_HARDWARE_DEVICE_010EDFAC: u32 = 0x010e_dfac;
const NO_DEVICE_OF_ANY_TYPE_010EDF60: u32 = 0x010e_df60;
const NO_TEXTURE_DEFAULTS_010EDF10: u32 = 0x010e_df10;
const NO_REFERENCE_DEVICE_010EDEDC: u32 = 0x010e_dedc;
const NO_REFERENCE_DEVICE_REPORT_010EDE88: u32 = 0x010e_de88;
const NO_HARDWARE_OF_ANY_TYPE_010EDE30: u32 = 0x010e_de30;

/// The renderer's fields the initialisation uses: the device (+0x288), the device type
/// (+0x5c8) and behaviour flags (+0x5cc) `00e69510` fills, the adapter (+0x5c4), the adapter
/// and device descriptions found (+0x5d8, +0x5dc), and the 14 words kept from +0xa98.
const DEVICE: u32 = 0x288;
const DEVICE_TYPE: u32 = 0x5c8;
const BEHAVIOR_FLAGS: u32 = 0x5cc;
const ADAPTER: u32 = 0x5c4;
const WINDOW: u32 = 0x3bc;
const FOCUS_WINDOW: u32 = 0x3c0;
/// The initialisation's frame (`SUB ESP,0x80` and four pushed registers): the vertex-buffer
/// and depth-stencil out words (+0x10, +0x14), the `CreateDeviceEx` out words (+0x18,
/// +0x1c), the full-screen display mode (+0x24, 0x18 bytes; later the viewport), the frustum
/// (+0x3c) and the present parameters (+0x58, 0x38 bytes).
const INIT_FRAME: u32 = 0x90;
const FRAME_BLOCK_OUT: u32 = 0x10;
const FRAME_DEPTH_OUT: u32 = 0x14;
const FRAME_EX_OUT: u32 = 0x18;
const FRAME_EX_RETRY_OUT: u32 = 0x1c;
const FRAME_MODE: u32 = 0x24;
const FRAME_FRUSTUM: u32 = 0x3c;
const FRAME_PRESENT: u32 = 0x58;

/// The adapter description for the renderer's adapter (+0x5c4): the list's element when the
/// index is below its 16-bit count, else null.
fn adapter_description(e: &Engine, this: u32) -> u32 {
    let list = e.mem.u32(ADAPTER_LIST);
    let index = e.mem.u32(this + ADAPTER);
    if index < u32::from(e.mem.u16(list.wrapping_add(0xe))) {
        let elements = e.mem.u32(list.wrapping_add(8));
        e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

/// The adapter description's device description for the renderer's device type: +0x460 for
/// type 1, else +0x464.
fn device_description(e: &Engine, this: u32, adapter: u32) -> u32 {
    if e.mem.u32(this + DEVICE_TYPE) == 1 {
        e.mem.u32(adapter.wrapping_add(0x460))
    } else {
        e.mem.u32(adapter.wrapping_add(0x464))
    }
}

/// `CreateDevice` or, on an `Ex` object, `CreateDeviceEx` (with the full-screen display mode
/// only when the present parameters are not windowed, +0x20, and the device through `out`);
/// answers the `HRESULT`.
fn create_device(e: &mut Engine, this: u32, frame: u32, out: u32) -> i32 {
    let direct3d = e.mem.u32(DIRECT3D);
    let present = frame + FRAME_PRESENT;
    let adapter = e.mem.u32(this + ADAPTER);
    let device_type = e.mem.u32(this + DEVICE_TYPE);
    let focus = e.mem.u32(this + FOCUS_WINDOW);
    let behavior = e.mem.u32(this + BEHAVIOR_FLAGS);
    if e.mem.u8(DIRECT3D_IS_EX) != 0 {
        let mode = if e.mem.u32(present + 0x20) != 0 {
            0
        } else {
            frame + FRAME_MODE
        };
        let result = e
            .vcall(
                direct3d,
                CREATE_DEVICE_EX,
                &args![adapter, device_type, focus, behavior, present, mode, out],
            )
            .i32();
        let device = e.mem.u32(out);
        e.mem.set_u32(this + DEVICE, device);
        result
    } else {
        e.vcall(
            direct3d,
            CREATE_DEVICE,
            &args![
                adapter,
                device_type,
                focus,
                behavior,
                present,
                this + DEVICE
            ],
        )
        .i32()
    }
}

/// A word made unsigned by `FILD` and adding the `float` 2^32 when negative.
fn unsigned_on_x87(e: &Engine, word: u32) -> f64 {
    let value = f64::from(word as i32);
    if (word as i32) < 0 {
        x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        value
    }
}

/// What ends a pass of the device loop.
enum DeviceAttempt {
    Created,
    NextType,
}

// Translated from 00e72e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`; its messages call it
/// `NiDX9Renderer::Initialize`), `thiscall` with 14 words; answers a byte, 1 on success. The
/// parameter names follow what the code does with them: the width and height go to the
/// full-screen display mode, `flags` bits 0x40 and 0x20 add behaviour flags 2 and 4, the window
/// (+0x3bc) goes to the present parameters' set-up and the window target, the focus window
/// (+0x3c0) and adapter (+0x5c4) to `CreateDevice`, the device kind to `00e69510`, and the
/// last word is the display mode's refresh rate; the others are only kept and passed on.
///
/// Keeps the 14 words at +0xa98, clears the `float` at `0126efd4` and the byte `0126efd1`,
/// runs `00e69a60`, and lets `00e69510` turn the device kind into the device type and
/// behaviour flags (failing: "Invalid 3D device type"). The adapter's description (none:
/// "Invalid Adapter") and its device description for the type (none, or without caps: "Invalid
/// 3D device type") are kept. Then, with the present parameters cleared, for each device kind
/// from the requested one: the device description must still be valid ("Requested device not
/// valid"); `00e6bf50` fills the present parameters; the device is created (`CreateDevice`, or
/// `CreateDeviceEx` with a full-screen display mode of size 0x18: width, height, refresh rate,
/// the back buffer format, scan-line ordering 1), +0xad0 is set when height / width (as
/// unsigned values on the x87) is below 0.75 or unordered; a failure clears +0x6fc and, when
/// the call changed the back buffer count, is retried once ("trying fewer backbuffers"). A
/// failed kind moves to the next one: after the reference kind (4) "Could not create requested
/// reference device"; reaching kind 4 "Could not create hardware device of any type"; a kind
/// `00e69510` refuses "Invalid 3D device type". With a device: none at +0x288 fails ("Could not
/// create hardware device"); `00e6c240` must succeed; +0x5d4 and +0x5d5 get the behaviour
/// flags' inverted bit 6 and bit 7; the back buffer and (with automatic depth-stencil) the
/// depth surface are fetched; the render target group (`00ee8710`) gets them (slots 0xa8,
/// 0xac), the window's target is set (`00e72cf0` with an `NiPointer` to the group), the format
/// of the group's first buffer is kept at +0x5d0; the texture defaults (`00e6ec90`, failing:
/// "Could not initialize texture defaults") and the shader and geometry objects are made
/// (+0x8b8 to +0x8c8, `NiD3DShaderInterface::InitializeShaderSystem`); the default frustum
/// (-1, 1, 1, -1, the near `float` at `0101e2bc`, 1) and viewport (0, 1, 1, 0) go to the
/// renderer's slot 0x18c; then `NiShaderFactory::GetInstance`, a 0x40-byte object (`00e909e0`;
/// its slot 0x4c runs even when the allocation failed) handed to `00e6e6e0`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e72e60(
    e: &mut Engine,
    this: Ptr,
    width: u32,
    height: u32,
    flags: u32,
    window: u32,
    focus_window: u32,
    adapter: u32,
    device_kind: u32,
    eighth: u32,
    ninth: u32,
    tenth: u32,
    eleventh: u32,
    twelfth: u32,
    thirteenth: u32,
    refresh_rate: u32,
) -> u8 {
    let this = this.addr();
    let words = [
        width,
        height,
        flags,
        window,
        focus_window,
        adapter,
        device_kind,
        eighth,
        ninth,
        tenth,
        eleventh,
        twelfth,
        thirteenth,
        refresh_rate,
    ];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(this + 0xa98 + 4 * i as u32, *word);
    }
    // FLDZ / FSTP float.
    e.mem.set_u32(ZEROED_VALUE_0126EFD4, x87_store32(0.0));
    e.mem.set_u8(ZEROED_FLAG_0126EFD1, 0);
    e.call(RESET_00E69A60, &args![]);
    e.mem.set_u32(this + ADAPTER, adapter);
    e.mem.set_u32(this + WINDOW, window);
    e.mem.set_u32(this + FOCUS_WINDOW, focus_window);
    let device_type_at = this + DEVICE_TYPE;
    let behavior_at = this + BEHAVIOR_FLAGS;
    let report = |e: &mut Engine, message: u32| {
        e.call(REPORT_00A29680, &args![message]);
    };
    let copy_error = |e: &mut Engine, message: u32| {
        e.call(
            STRNCPY_S,
            &args![ERROR_MESSAGE_011F4648, 0x100u32, message, 0xffu32],
        );
    };
    let invalid_device = |e: &mut Engine| {
        copy_error(e, INVALID_DEVICE_TYPE_010EE20C);
        report(e, INVALID_DEVICE_ID_010EE1C8);
        0u8
    };
    if e.call(
        DEVICE_SETTINGS_00E69510,
        &args![device_kind, device_type_at, behavior_at],
    )
    .u8()
        == 0
    {
        return invalid_device(e);
    }
    if flags as u8 & 0x40 != 0 {
        let value = e.mem.u32(behavior_at) | 2;
        e.mem.set_u32(behavior_at, value);
    }
    if flags as u8 & 0x20 != 0 {
        let value = e.mem.u32(behavior_at) | 4;
        e.mem.set_u32(behavior_at, value);
    }
    let adapter_desc = adapter_description(e, this);
    e.mem.set_u32(this + 0x5d8, adapter_desc);
    if adapter_desc == 0 {
        copy_error(e, INVALID_ADAPTER_010EE1A0);
        report(e, INVALID_ADAPTER_REPORT_010EE160);
        return 0;
    }
    let device_desc = device_description(e, this, adapter_desc);
    let kept = if e.mem.u32(device_desc.wrapping_add(4)) != 0 {
        device_desc
    } else {
        0
    };
    e.mem.set_u32(this + 0x5dc, kept);
    if kept == 0 {
        return invalid_device(e);
    }

    e.with_stack(INIT_FRAME, |e, frame| {
        let frame = frame.addr();
        let present = frame + FRAME_PRESENT;
        e.call(MEMSET_ADDR, &args![present, 0u32, 0x38u32]);
        let mut kind = device_kind;
        loop {
            report(e, ATTEMPTING_010EE118);
            let adapter_desc = adapter_description(e, this);
            let device_desc = device_description(e, this, adapter_desc);
            let mut attempt = DeviceAttempt::NextType;
            if e.mem.u32(device_desc.wrapping_add(4)) == 0 || device_desc == 0 {
                copy_error(e, DEVICE_NOT_VALID_010EE0E8);
                report(e, NO_DEVICE_OF_TYPE_010EE0AC);
            } else {
                let filled = e
                    .call(
                        PRESENT_PARAMETERS_00E6BF50,
                        &args![
                            this,
                            e.mem.u32(this + WINDOW),
                            width,
                            height,
                            flags,
                            twelfth,
                            eighth,
                            ninth,
                            thirteenth,
                            eleventh,
                            refresh_rate,
                            tenth,
                            present
                        ],
                    )
                    .u8();
                if filled != 0 {
                    let back_buffers = e.mem.u32(present + 0xc);
                    let mode = frame + FRAME_MODE;
                    for i in 0..6 {
                        e.mem.set_u32(mode + 4 * i, 0);
                    }
                    let back_buffer_format = e.mem.u32(present + 8);
                    e.mem.set_u32(mode + 0xc, refresh_rate);
                    e.mem.set_u32(mode, 0x18);
                    e.mem.set_u32(mode + 8, height);
                    e.mem.set_u32(mode + 4, width);
                    e.mem.set_u32(mode + 0x14, 1);
                    e.mem.set_u32(mode + 0x10, back_buffer_format);
                    let result = create_device(e, this, frame, frame + FRAME_EX_OUT);
                    let tall = unsigned_on_x87(e, e.mem.u32(present + 4));
                    let wide = unsigned_on_x87(e, e.mem.u32(present));
                    let ratio = x87_div(tall, wide);
                    let limit = e.mem.f64(RATIO_LIMIT_0101DE30);
                    // FCOMP / FNSTSW / TEST AH,1: C0 is set below the limit or unordered.
                    let below = !(ratio >= limit);
                    e.mem.set_u8(this + 0xad0, u8::from(below));
                    if result >= 0 {
                        attempt = DeviceAttempt::Created;
                    } else {
                        e.mem.set_u8(this + 0x6fc, 0);
                        if back_buffers != e.mem.u32(present + 0xc) {
                            report(e, FEWER_BACK_BUFFERS_010EE048);
                            let retry = create_device(e, this, frame, frame + FRAME_EX_RETRY_OUT);
                            if retry >= 0 {
                                attempt = DeviceAttempt::Created;
                            }
                        }
                    }
                }
            }
            if let DeviceAttempt::Created = attempt {
                break;
            }
            if kind == 4 {
                e.call(SET_ERROR_00E69AD0, &args![NO_REFERENCE_DEVICE_010EDEDC]);
                report(e, NO_REFERENCE_DEVICE_REPORT_010EDE88);
                return 0;
            }
            kind = kind.wrapping_add(1);
            if kind == 4 {
                report(e, NO_HARDWARE_OF_ANY_TYPE_010EDE30);
                return 0;
            }
            report(e, NEXT_LOWER_TYPE_010EDFE0);
            let accepted = e
                .call(
                    DEVICE_SETTINGS_00E69510,
                    &args![kind, device_type_at, behavior_at],
                )
                .u8();
            if accepted == 0 {
                e.call(SET_ERROR_00E69AD0, &args![INVALID_DEVICE_TYPE_010EE20C]);
                report(e, INVALID_DEVICE_ID_010EE1C8);
                return 0;
            }
        }

        if e.mem.u32(this + DEVICE) == 0 {
            e.call(SET_ERROR_00E69AD0, &args![NO_HARDWARE_DEVICE_010EDFAC]);
            report(e, NO_DEVICE_OF_ANY_TYPE_010EDF60);
            return 0;
        }
        if e.call(AFTER_DEVICE_00E6C240, &args![this, present]).u8() == 0 {
            return 0;
        }
        let behavior = e.mem.u32(behavior_at);
        e.mem.set_u8(this + 0x5d4, !((behavior >> 6) as u8) & 1);
        e.mem.set_u8(this + 0x5d5, ((behavior >> 7) as u8) & 1);
        let block_out = frame + FRAME_BLOCK_OUT;
        let depth_out = frame + FRAME_DEPTH_OUT;
        e.mem.set_u32(block_out, 0);
        e.mem.set_u32(depth_out, 0);
        let device = e.mem.u32(this + DEVICE);
        let blocks = e
            .call(BACK_BUFFER_00E7D6B0, &args![device, present, block_out])
            .u32();
        if e.mem.u32(present + 0x24) != 0 {
            let device = e.mem.u32(this + DEVICE);
            e.call(DEPTH_STENCIL_00E7D850, &args![device, depth_out]);
        }
        let group = e
            .call(RENDER_TARGET_GROUP_00EE8710, &args![1u32, this])
            .u32();
        e.mem.set_u32(this + 0x884, group);
        let back_buffer = e.mem.u32(block_out);
        e.vcall(group, 0xa8, &args![back_buffer, 0u32]);
        let group = e.mem.u32(this + 0x884);
        let depth = e.mem.u32(depth_out);
        e.vcall(group, 0xac, &args![depth]);
        // An `NiPointer` to the group made in the outgoing argument slot, passed by value.
        let pointer = e.with_stack(4, |e, slot| {
            let group = e.mem.u32(this + 0x884);
            e.call(NI_POINTER_CONSTRUCT_00B6AED0, &args![slot, group]);
            e.mem.u32(slot.addr())
        });
        e.call(
            WINDOW_TARGET_00E72CF0,
            &args![this + 0x890, window, pointer],
        );
        let group = e.mem.u32(this + 0x884);
        let buffer = e.vcall(group, 0x9c, &args![0u32]).u32();
        let raw_format = e.mem.u32(buffer.wrapping_add(0xc));
        let converted = e.call(FORMAT_00E69D00, &args![raw_format]).u32();
        let format = e.call(FORMAT_00E69C90, &args![converted]).u32();
        e.mem.set_u32(this + 0x5d0, format);
        if e.call(TEXTURE_DEFAULTS_00E6EC90, &args![this]).u8() == 0 {
            report(e, NO_TEXTURE_DEFAULTS_010EDF10);
            return 0;
        }
        let library = e
            .call(SHADER_LIBRARY_00E91590, &args![this, this + 0x28c, 1u32])
            .u32();
        e.mem.set_u32(this + 0x8b8, library);
        e.call(INITIALIZE_SHADER_SYSTEM, &args![this]);
        let made = |e: &mut Engine, size: u32, construct: u32, words: &[u32]| {
            let block = e.call(NEW_OBJECT, &args![size]).u32();
            if block == 0 {
                return 0;
            }
            let mut all = vec![block];
            all.extend_from_slice(words);
            e.call(construct, &all).u32()
        };
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x100, CONSTRUCT_00E8D0E0, &[device]);
        e.mem.set_u32(this + 0x8bc, object);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x4c, CONSTRUCT_00E8FB20, &[device]);
        e.mem.set_u32(this + 0x8c0, object);
        let object = made(e, 0x10, CONSTRUCT_00E90A80, &[this]);
        e.mem.set_u32(this + 0x8c4, object);
        let device = e.mem.u32(this + DEVICE);
        let first = e.mem.u32(this + 0x8bc);
        let manager = e
            .call(GEOMETRY_GROUP_MANAGER_00E88EB0, &args![device, first])
            .u32();
        e.mem.set_u32(this + 0x8ac, manager);
        let renderer_flags = e.vcall(this, 0xa0, &args![]).u32();
        let manager = e.mem.u32(this + 0x8ac);
        e.call(
            SET_FLAG_00E88C20,
            &args![manager, u32::from((renderer_flags >> 1) as u8 & 1)],
        );
        let manager = e.mem.u32(this + 0x8ac);
        let group_1 = e.vcall(manager, 4, &args![1u32]).u32();
        e.mem.set_u32(this + 0x8b0, group_1);
        let manager = e.mem.u32(this + 0x8ac);
        let group_2 = e.vcall(manager, 4, &args![2u32]).u32();
        e.mem.set_u32(this + 0x8b4, group_2);
        let library = e.mem.u32(this + 0x8b8);
        let device = e.mem.u32(this + DEVICE);
        let object = made(e, 0x23c, CONSTRUCT_00E8BAE0, &[library, device]);
        e.mem.set_u32(this + 0x8c8, object);
        if e.mem.u32(blocks.wrapping_add(0x30)) != 0 {
            let library = e.mem.u32(this + 0x8b8);
            e.vcall(library, 0x34, &args![1u32]);
        }
        let group = e.mem.u32(this + 0x884);
        e.mem.set_u32(this + 0x888, group);
        let frustum = frame + FRAME_FRUSTUM;
        e.call(NI_FRUSTUM_CONSTRUCT, &args![frustum, 0u32]);
        let side = x87_quiet_bits(e.mem.u32(FRUSTUM_SIDE_01012054));
        let near = x87_quiet_bits(e.mem.u32(FRUSTUM_NEAR_0101E2BC));
        let one = x87_store32(1.0);
        let zero = x87_store32(0.0);
        for (offset, value) in [
            (0xc, side),
            (0, side),
            (8, one),
            (4, one),
            (0x10, near),
            (0x14, one),
        ] {
            e.mem.set_u32(frustum + offset, value);
        }
        let viewport = frame + FRAME_MODE;
        for (offset, value) in [(0, zero), (0xc, zero), (4, one), (8, one)] {
            e.mem.set_u32(viewport + offset, value);
        }
        let [a, b, c, d] = CAMERA_WORDS_SLOT_18C;
        e.vcall(this, 0x18c, &args![a, b, c, d, frustum, viewport]);
        e.mem.set_u32(this + 0x888, 0);
        e.mem.set_u32(this + 0x88c, 0);
        e.call(NI_SHADER_FACTORY_GET_INSTANCE, &args![]);
        let block = e.call(NEW_OBJECT, &args![0x40u32]).u32();
        let shader = if block != 0 {
            e.call(CONSTRUCT_00E909E0, &args![block]).u32()
        } else {
            0
        };
        e.vcall(shader, 0x4c, &args![]);
        e.call(SET_SHADER_00E6E6E0, &args![this, shader]);
        1
    })
}

/// `memset` (`00ec61c0`).
const MEMSET_ADDR: u32 = 0x00ec_61c0;

/// The registry entry of [`fn_00e72e60`]: `this` and its 14 words.
fn fn_00e72e60_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    u32::from(fn_00e72e60(
        e,
        Ptr::new(w(0)),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        w(12),
        w(13),
        w(14),
    ))
    .into_ret()
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    const CREATE9: u32 = 0x7200_0000;
    const CREATE9_EX: u32 = 0x7200_0010;
    const OBJECT: u32 = 0x7300_0000;
    const OBJECT_EX: u32 = 0x7300_0100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn direct3d_rig(usable: bool, ex: u32, ex_object: u32, object: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(DIRECT3D_CREATE9, CREATE9);
        e.mem.set_u32(DIRECT3D_CREATE9_EX, ex);
        e.register_double(LOAD_DIRECT3D, move |_, _| u32::from(usable).into_ret());
        e.register(REPORT_00A29680, |_, _| Ret::default());
        e.register_double(CREATE9_EX, move |e, a| {
            e.mem.set_u32(a[1], ex_object);
            Ret::default()
        });
        e.register_double(CREATE9, move |_, _| object.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn create_direct3d9_prefers_ex_when_asked() {
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT_EX);
        assert_eq!(e.mem.u8(DIRECT3D_IS_EX), 1);
        assert!(calls(&e, CREATE9).is_empty());
        assert_eq!(calls(&e, CREATE9_EX)[0][0], D3D_SDK_VERSION);
        // Made already: nothing more.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), 0);
        assert!(calls(&e, LOAD_DIRECT3D).is_empty());
    }

    #[test]
    fn create_direct3d9_falls_back_and_keeps_the_ex_failure() {
        let mut e = direct3d_rig(true, CREATE9_EX, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_EX_FAILED_010ED598],
                vec![CREATE_SUCCEEDED_010ED52C]
            ]
        );
        // Without the byte (or the Ex function) only the plain creation runs.
        let mut e = direct3d_rig(true, CREATE9_EX, OBJECT_EX, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![0x100u32]).i32(), 0);
        assert_eq!(e.mem.u32(DIRECT3D), OBJECT);
        assert!(calls(&e, CREATE9_EX).is_empty());
    }

    #[test]
    fn create_direct3d9_failures() {
        let mut e = direct3d_rig(false, 0, 0, OBJECT);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![CREATE_FAILED_010ED550]]
        );
        let mut e = direct3d_rig(true, 0, 0, 0);
        assert_eq!(e.call(0x00e6_9460, &args![1u32]).i32(), -1);
        assert_eq!(calls(&e, CREATE9), vec![vec![D3D_SDK_VERSION]]);
    }

    // ---- 00e72e60 ----

    const D3D_VTABLE: u32 = 0x7400_0000;
    const D3D_SLOTS: u32 = 0x7500_0000;
    const DEVICE_OBJECT: u32 = 0x7600_0000;
    const RENDERER_VTABLE: u32 = 0x7410_0000;
    const RENDERER_SLOTS: u32 = 0x7510_0000;
    const GROUP_VTABLE: u32 = 0x7420_0000;
    const GROUP_SLOTS: u32 = 0x7520_0000;
    const ADAPTER_INDEX: u32 = 0;
    const WORDS: [u32; 14] = [
        640,
        480,
        0x60,
        0x11,
        0x22,
        ADAPTER_INDEX,
        1,
        8,
        9,
        10,
        11,
        12,
        13,
        60,
    ];

    /// The Direct3D and renderer doubles. `create_results` are the `CreateDevice` results in
    /// order (the last repeats); `00e6bf50` sets the back buffer count to `back_buffers`.
    struct Setup {
        ex: bool,
        create_results: Vec<i32>,
        adapter_ok: bool,
        caps: u32,
    }

    fn vtable(e: &mut Engine, at: u32, slots: u32, count: u32) {
        let targets: Vec<u32> = (0..count).map(|i| slots + 4 * i).collect();
        e.put_vtable(at, &targets);
    }

    fn init_rig(setup: Setup) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_e000, 0x2000);
        e.map(0x0101_0000, 0x10000);
        e.map(0x0109_9000, 0x1000);
        e.mem.set_u32(FRUSTUM_SIDE_01012054, (-1.0f32).to_bits());
        e.mem.set_u32(FRUSTUM_NEAR_0101E2BC, 0.1f32.to_bits());
        e.mem.set_f64(RATIO_LIMIT_0101DE30, 0.75);
        e.mem
            .set_u32(TWO_TO_32_01099378, 4_294_967_296.0f32.to_bits());
        // The Direct3D object and its slots.
        vtable(&mut e, D3D_VTABLE, D3D_SLOTS, 0x20);
        let direct3d = e.mem.alloc(4);
        e.mem.set_u32(direct3d, D3D_VTABLE);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.mem.set_u8(DIRECT3D_IS_EX, u8::from(setup.ex));
        let mut results = setup.create_results.clone().into_iter();
        let mut last = 0;
        let slot = if setup.ex {
            CREATE_DEVICE_EX
        } else {
            CREATE_DEVICE
        };
        e.register_double(D3D_SLOTS + slot, move |e, a| {
            let result = results.next().unwrap_or(last);
            last = result;
            // Present parameters: a changed back buffer count on the first failure.
            if result < 0 {
                e.mem.set_u32(a[5] + 0xc, 1);
            }
            let out = if setup.ex { a[7] } else { a[6] };
            e.mem
                .set_u32(out, if result >= 0 { DEVICE_OBJECT } else { 0 });
            (result as u32).into_ret()
        });
        // The adapter list: one adapter with a device description for type 1.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(ADAPTER_LIST, list);
        let elements = e.mem.alloc(4);
        e.mem.set_u32(list + 8, elements);
        e.mem.set_u16(list + 0xe, 1);
        let adapter = e.mem.alloc(0x468);
        if setup.adapter_ok {
            e.mem.set_u32(elements, adapter);
        }
        let device_desc = e.mem.alloc(0x144);
        e.mem.set_u32(device_desc + 4, setup.caps);
        e.mem.set_u32(adapter + 0x460, device_desc);
        e.mem.set_u32(adapter + 0x464, device_desc);
        // Device kind -> type 1 for kinds below 3, behaviour flags 0x40.
        e.register(DEVICE_SETTINGS_00E69510, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[2], 0x40);
            u32::from(a[0] < 3).into_ret()
        });
        e.register(PRESENT_PARAMETERS_00E6BF50, |e, a| {
            let present = a[12];
            e.mem.set_u32(present, 640);
            e.mem.set_u32(present + 4, 480);
            e.mem.set_u32(present + 8, 0x16);
            e.mem.set_u32(present + 0xc, 2);
            e.mem.set_u32(present + 0x24, 1);
            1u32.into_ret()
        });
        let blocks = e.mem.alloc(0x40);
        e.mem.set_u32(blocks + 0x30, 1);
        e.register_double(BACK_BUFFER_00E7D6B0, move |e, a| {
            e.mem.set_u32(a[2], 0x5151);
            blocks.into_ret()
        });
        e.register(DEPTH_STENCIL_00E7D850, |e, a| {
            e.mem.set_u32(a[1], 0x5252);
            Ret::default()
        });
        // The render target group.
        vtable(&mut e, GROUP_VTABLE, GROUP_SLOTS, 0x30);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group, GROUP_VTABLE);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(buffer + 0xc, 0x77);
        e.register_double(RENDER_TARGET_GROUP_00EE8710, move |_, _| group.into_ret());
        e.register_double(GROUP_SLOTS + 0x9c, move |_, _| buffer.into_ret());
        e.register(GROUP_SLOTS + 0x4c, |_, _| Ret::default());
        e.register(GROUP_SLOTS + 4, |_, a| (0x900 + a[1]).into_ret());
        for slot in [0xa8, 0xac, 0x34] {
            e.register(GROUP_SLOTS + slot, |_, _| Ret::default());
        }
        e.register(NI_POINTER_CONSTRUCT_00B6AED0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // Every constructor answers an object with the group's table.
        for construct in [
            CONSTRUCT_00E8D0E0,
            CONSTRUCT_00E8FB20,
            CONSTRUCT_00E90A80,
            CONSTRUCT_00E8BAE0,
            CONSTRUCT_00E909E0,
        ] {
            e.register(construct, |e, a| {
                e.mem.set_u32(a[0], GROUP_VTABLE);
                a[0].into_ret()
            });
        }
        e.register_double(GEOMETRY_GROUP_MANAGER_00E88EB0, move |_, _| {
            group.into_ret()
        });
        e.register_double(SHADER_LIBRARY_00E91590, move |_, _| group.into_ret());
        for no_op in [
            RESET_00E69A60,
            STRNCPY_S,
            REPORT_00A29680,
            SET_ERROR_00E69AD0,
            WINDOW_TARGET_00E72CF0,
            INITIALIZE_SHADER_SYSTEM,
            SET_FLAG_00E88C20,
            NI_FRUSTUM_CONSTRUCT,
            NI_SHADER_FACTORY_GET_INSTANCE,
            SET_SHADER_00E6E6E0,
            MEMSET_ADDR,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(FORMAT_00E69D00, |_, a| (a[0] + 1).into_ret());
        e.register(FORMAT_00E69C90, |_, a| (a[0] + 1).into_ret());
        for ok in [AFTER_DEVICE_00E6C240, TEXTURE_DEFAULTS_00E6EC90] {
            e.register(ok, |_, _| 1u32.into_ret());
        }
        // The renderer and its slots 0xa0 and 0x18c.
        vtable(&mut e, RENDERER_VTABLE, RENDERER_SLOTS, 0x70);
        e.register(RENDERER_SLOTS + 0xa0, |_, _| 2u32.into_ret());
        e.register(RENDERER_SLOTS + 0x18c, |e, a| {
            // Frustum and viewport as the renderer receives them.
            let frustum: Vec<f32> = (0..6).map(|i| e.mem.f32(a[5] + 4 * i)).collect();
            let viewport: Vec<f32> = (0..4).map(|i| e.mem.f32(a[6] + 4 * i)).collect();
            assert_eq!(frustum, vec![-1.0, 1.0, 1.0, -1.0, 0.1, 1.0]);
            assert_eq!(viewport, vec![0.0, 1.0, 1.0, 0.0]);
            Ret::default()
        });
        let this = e.mem.alloc(0xb80);
        e.mem.set_u32(this, RENDERER_VTABLE);
        e.call_log = Some(vec![]);
        (e, this)
    }

    fn initialize(e: &mut Engine, this: u32) -> u8 {
        let mut words = vec![this];
        words.extend_from_slice(&WORDS);
        e.call(0x00e7_2e60, &words).u8()
    }

    #[test]
    fn initialize_creates_the_device_and_the_renderer_objects() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        assert_eq!(e.mem.u32(this + 0xa98), 640);
        assert_eq!(e.mem.u32(this + 0xacc), 60);
        // Behaviour flags: 0x40 from 00e69510, 2 for flags & 0x40, 4 for flags & 0x20.
        assert_eq!(e.mem.u32(this + BEHAVIOR_FLAGS), 0x46);
        assert_eq!(e.mem.u8(this + 0x5d4), 0);
        assert_eq!(e.mem.u8(this + 0x5d5), 0);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE);
        assert_eq!(create.len(), 1);
        assert_eq!(&create[0][1..5], &[ADAPTER_INDEX, 1, 0x22, 0x46]);
        assert_eq!(create[0][6], this + DEVICE);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        // 480 / 640 = 0.75 is not below 0.75.
        assert_eq!(e.mem.u8(this + 0xad0), 0);
        let group = e.mem.u32(this + 0x884);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xa8), vec![vec![group, 0x5151, 0]]);
        assert_eq!(calls(&e, GROUP_SLOTS + 0xac), vec![vec![group, 0x5252]]);
        assert_eq!(
            calls(&e, WINDOW_TARGET_00E72CF0),
            vec![vec![this + 0x890, 0x11, group]]
        );
        assert_eq!(e.mem.u32(this + 0x5d0), 0x79);
        assert_eq!(e.mem.u32(this + 0x8b0), 0x901);
        assert_eq!(e.mem.u32(this + 0x8b4), 0x902);
        assert_eq!(calls(&e, SET_FLAG_00E88C20)[0][1], 1);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x34).len(), 1);
        assert_eq!(calls(&e, RENDERER_SLOTS + 0x18c).len(), 1);
        assert_eq!(e.mem.u32(this + 0x888), 0);
        let shader = calls(&e, SET_SHADER_00E6E6E0)[0][1];
        assert_ne!(shader, 0);
        assert_eq!(calls(&e, GROUP_SLOTS + 0x4c), vec![vec![shader]]);
    }

    #[test]
    fn initialize_retries_with_fewer_back_buffers_through_create_device_ex() {
        let (mut e, this) = init_rig(Setup {
            ex: true,
            create_results: vec![-1, 0],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 1);
        let create = calls(&e, D3D_SLOTS + CREATE_DEVICE_EX);
        assert_eq!(create.len(), 2);
        // Windowed is 0 in the present parameters: the display mode is passed.
        assert_ne!(create[0][6], 0);
        assert_ne!(create[1][7], create[0][7]);
        assert_eq!(e.mem.u32(this + DEVICE), DEVICE_OBJECT);
        assert!(calls(&e, REPORT_00A29680).contains(&vec![FEWER_BACK_BUFFERS_010EE048]));
    }

    #[test]
    fn initialize_walks_the_device_kinds_down_and_fails() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![-1],
            adapter_ok: true,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        // Kinds 1 and 2 (each with its retry: the failure changed the back buffer count),
        // then kind 3 refused.
        assert_eq!(calls(&e, D3D_SLOTS + CREATE_DEVICE).len(), 4);
        let kinds: Vec<u32> = calls(&e, DEVICE_SETTINGS_00E69510)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(kinds, vec![1, 2, 3]);
        assert_eq!(
            calls(&e, SET_ERROR_00E69AD0),
            vec![vec![INVALID_DEVICE_TYPE_010EE20C]]
        );
    }

    #[test]
    fn initialize_refuses_a_missing_adapter_or_device() {
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: false,
            caps: 1,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![INVALID_ADAPTER_REPORT_010EE160]]
        );
        let (mut e, this) = init_rig(Setup {
            ex: false,
            create_results: vec![0],
            adapter_ok: true,
            caps: 0,
        });
        assert_eq!(initialize(&mut e, this), 0);
        assert_eq!(e.mem.u32(this + 0x5dc), 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], INVALID_DEVICE_TYPE_010EE20C);
        assert!(calls(&e, D3D_SLOTS + CREATE_DEVICE).is_empty());
    }
}

// ---- The renderer's set-up helpers (batch b000v) ----

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_UP: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DOWN: u32 = 0x00fd_f1c4;

/// Releases a reference-counted object as the code inlines it: its count (+4) down through
/// `InterlockedDecrement` and, at zero, its virtual slot 4 (no stack words).
fn release_object(e: &mut Engine, object: u32) {
    if e.call(IMPORT_INTERLOCKED_DOWN, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00e69510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`: a device kind, the device type
/// word and the behaviour flags word. Clears bits 4 to 7 of the flags, then for kinds 0 to 3
/// sets device type 1 (hardware) and for kinds 4 to 6 device type 2 with the flag 0x50, 0x40,
/// 0x80, 0x20, 0x20, 0x40, 0x80 (the jump table at `00e695a4`, read in Ghidra); answers 1, or
/// 0 for any other kind.
pub fn fn_00e69510(e: &mut Engine, kind: u32, device_type: Ptr, behavior: Ptr) -> u8 {
    let flags = e.mem.u32(behavior.addr()) & 0xffff_ff0f;
    e.mem.set_u32(behavior.addr(), flags);
    let (device, flag) = match kind {
        0 => (1u32, 0x50u32),
        1 => (1, 0x40),
        2 => (1, 0x80),
        3 => (1, 0x20),
        4 => (2, 0x20),
        5 => (2, 0x40),
        6 => (2, 0x80),
        _ => return 0,
    };
    e.mem.set_u32(device_type.addr(), device);
    let flags = e.mem.u32(behavior.addr()) | flag;
    e.mem.set_u32(behavior.addr(), flags);
    1
}

// Translated from 00e69c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, an index: the format number for
/// indexes 1 to 11 (the jump table at `00e69cc8`, read in Ghidra): 1 gives 0x14, 2 and 3 0x16,
/// 4 0x17, 5 and 6 0x18, 7 and 11 0x1e, 8 0x1b; anything else 0.
pub fn fn_00e69c90(_e: &mut Engine, index: u32) -> u32 {
    match index {
        1 => 0x14,
        2 | 3 => 0x16,
        4 => 0x17,
        5 | 6 => 0x18,
        7 | 11 => 0x1e,
        8 => 0x1b,
        _ => 0,
    }
}

// Translated from 00e69d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, a format number: its index (the
/// byte map at `00e69dd8` and jump table at `00e69d88`, read in Ghidra): 0x14 to 0x1e give 1 to
/// 0xb, 0x6f 0xc, 0x70 0xd, 0x24 0xe, 0x71 to 0x74 0xf to 0x12; anything else (and anything
/// above 0x74) 0.
pub fn fn_00e69d00(_e: &mut Engine, format: u32) -> u32 {
    match format {
        0x14..=0x1e => format - 0x13,
        0x6f => 0xc,
        0x70 => 0xd,
        0x24 => 0xe,
        0x71..=0x74 => format - 0x62,
        _ => 0,
    }
}

// Translated from 00e6e6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall` with a shader: a non-null
/// shader other than the one held at +0x14 replaces it (`NiPointer` assignment: the old one
/// released, the new one counted); a null shader changes nothing.
pub fn fn_00e6e6e0(e: &mut Engine, this: Ptr, shader: u32) {
    if shader == 0 {
        return;
    }
    let slot = this.addr().wrapping_add(0x14);
    let held = e.mem.u32(slot);
    if held == shader {
        return;
    }
    if held != 0 {
        release_object(e, held);
    }
    e.mem.set_u32(slot, shader);
    e.call(IMPORT_INTERLOCKED_UP, &args![shader.wrapping_add(4)]);
}

// Translated from 00e72cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the window-target map's set
/// (`NiTPointerMap<HWND, NiPointer<NiRenderTargetGroup>>` by the RTTI of the renderer's map at
/// +0x890), `thiscall`: the key and an `NiPointer` passed by value (its word). The bucket is
/// the map's virtual slot 4 of the key; an item whose key equals it (slot 8) has its value
/// replaced (`NiPointer` assignment); otherwise a new item (slot 0x14) is filled through slot
/// 0xc (given the key and a counted copy of the pointer), put at the bucket's head and
/// counted (+0xc). The by-value pointer is released at the end.
pub fn fn_00e72cf0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, 4, &args![key]).u32();
    let buckets = e.mem.u32(map.wrapping_add(8));
    let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(map, 8, &args![key, item_key]).u8() != 0 {
            let held = e.mem.u32(item.wrapping_add(8));
            if held != value {
                if held != 0 {
                    release_object(e, held);
                }
                e.mem.set_u32(item.wrapping_add(8), value);
                if value != 0 {
                    e.call(IMPORT_INTERLOCKED_UP, &args![value.wrapping_add(4)]);
                }
            }
            if value != 0 {
                release_object(e, value);
            }
            return;
        }
        item = e.mem.u32(item);
    }
    let new_item = e.vcall(map, 0x14, &args![]).u32();
    if value != 0 {
        e.call(IMPORT_INTERLOCKED_UP, &args![value.wrapping_add(4)]);
    }
    e.vcall(map, 0xc, &args![new_item, key, value]);
    let buckets = e.mem.u32(map.wrapping_add(8));
    let head = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(new_item, head);
    let buckets = e.mem.u32(map.wrapping_add(8));
    e.mem
        .set_u32(buckets.wrapping_add(bucket.wrapping_mul(4)), new_item);
    let count = e.mem.u32(map.wrapping_add(0xc)).wrapping_add(1);
    e.mem.set_u32(map.wrapping_add(0xc), count);
    if value != 0 {
        release_object(e, value);
    }
}

/// The globals `00e6b990` copies twice (six words from `011a946c`), the `float`s it loads
/// (`0101e2bc`, `01016410`), the byte table it fills (`0126efec` to `0126f04d`) and the block
/// it clears (`0126efd8`, 0x76 bytes).
const DEFAULT_WORDS_011A946C: u32 = 0x011a_946c;
const FLOAT_SOURCE_01016410: u32 = 0x0101_6410;
const BIT_TABLE_0126EFEC: u32 = 0x0126_efec;
const CLEARED_BLOCK_0126EFD8: u32 = 0x0126_efd8;

/// The byte stores `00e6b990` makes into the table at `0126efec` (offset, value), in the
/// exe's order.
const BIT_TABLE_STORES: [(u32, u8); 46] = [
    (0x00, 0x18),
    (0x01, 0x20),
    (0x02, 0x20),
    (0x03, 0x10),
    (0x04, 0x10),
    (0x05, 0x10),
    (0x06, 0x10),
    (0x07, 0x08),
    (0x08, 0x08),
    (0x09, 0x10),
    (0x0a, 0x10),
    (0x0b, 0x20),
    (0x0c, 0x20),
    (0x0d, 0x20),
    (0x0e, 0x20),
    (0x0f, 0x20),
    (0x10, 0x40),
    (0x14, 0x10),
    (0x15, 0x08),
    (0x1e, 0x08),
    (0x1f, 0x10),
    (0x20, 0x08),
    (0x28, 0x10),
    (0x29, 0x10),
    (0x2a, 0x20),
    (0x2b, 0x20),
    (0x2c, 0x20),
    (0x2f, 0x20),
    (0x32, 0x10),
    (0x33, 0x20),
    (0x35, 0x10),
    (0x37, 0x20),
    (0x39, 0x20),
    (0x3b, 0x20),
    (0x3c, 0x10),
    (0x3e, 0x20),
    (0x3f, 0x20),
    (0x3d, 0x10),
    (0x51, 0x10),
    (0x52, 0x20),
    (0x5a, 0x40),
    (0x5b, 0x10),
    (0x5c, 0x20),
    (0x5d, 0x40),
    (0x5e, 0x20),
    (0x5f, 0x40),
];
/// The last two stores of the table (`0126f04c`, `0126f04d`).
const BIT_TABLE_LAST: [(u32, u8); 2] = [(0x60, 0x80), (0x61, 0x10)];

// Translated from 00e6b990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the renderer's field set-up the
/// constructor ends with, `thiscall`, no stack words. Clears the device (+0x288) and the
/// 0x130 caps bytes (+0x28c, `memset`), sets many fields to 0, 1 or their defaults (+0x5e4
/// 1.0, +0x5e0 0xff808080, +0x5c8 1, the six words at `011a946c` twice from +0x634, the
/// `float`s at `0101e2bc` and `01016410` at +0x684 and +0x688, an identity matrix of 0x40
/// bytes at +0x68c, the four 0x58-byte blocks from +0x700 cleared with the words +0x860 to
/// +0x86c and the objects at +0x870 to +0x87c released, 0.0 and 1.0 at +0x94c to +0x97c,
/// +0x880 0x16), clears the 0x76 bytes at `0126efd8` (`memset`) and fills the byte table at
/// `0126efec`.
pub fn fn_00e6b990(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let set = |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(base + offset, value);
    let set_byte = |e: &mut Engine, offset: u32, value: u8| e.mem.set_u8(base + offset, value);
    let one = x87_store32(1.0);
    let zero = x87_store32(0.0);
    set(e, 0x288, 0);
    e.call(MEMSET_ADDR, &args![base + 0x28c, 0u32, 0x130u32]);
    set(e, 0x5e4, one);
    set(e, 0x3bc, 0);
    set_byte(e, 0x3c4, 0);
    for offset in [0x5c4u32, 0x3c0, 0x5cc, 0x5d0] {
        set(e, offset, 0);
    }
    set_byte(e, 0x5d4, 0);
    set_byte(e, 0x5d5, 0);
    for offset in [0x6e4u32, 0x6e8, 0x6ec, 0x6f0] {
        set(e, offset, 0);
    }
    set_byte(e, 0x6f4, 0);
    set_byte(e, 0x6f5, 0);
    set(e, 0x8a0, 0);
    set_byte(e, 0x8a4, 0);
    set_byte(e, 0x8a5, 0);
    set(e, 0x6f8, 0);
    set_byte(e, 0x6fc, 0);
    set(e, 0x5e0, 0xff80_8080);
    set(e, 0x5e8, 0);
    set(e, 0x5ec, 0);
    set_byte(e, 0x5f0, 0);
    set(e, 0x5c8, 1);
    for copy in [0x634u32, 0x64c] {
        for word in 0..6 {
            let value = e.mem.u32(DEFAULT_WORDS_011A946C + 4 * word);
            set(e, copy + 4 * word, value);
        }
    }
    let near = x87_quiet_bits(e.mem.u32(FRUSTUM_NEAR_0101E2BC));
    set(e, 0x684, near);
    let second = x87_quiet_bits(e.mem.u32(FLOAT_SOURCE_01016410));
    set(e, 0x688, second);
    e.call(MEMSET_ADDR, &args![base + 0x68c, 0u32, 0x40u32]);
    for offset in [0x6c8u32, 0x6b4, 0x6a0, 0x68c] {
        set(e, offset, one);
    }
    set(e, 0x620, 0);
    set(e, 0x624, 0);
    set(e, 0x6d4, 1);
    set(e, 0x6dc, zero);
    set(e, 0x6d8, 1);
    set(e, 0x6cc, 0);
    set(e, 0x6e0, one);
    set(e, 0x6d0, 0);
    for block in 0..4u32 {
        let start = 0x700 + 0x58 * block;
        for word in 0..0x16 {
            set(e, start + 4 * word, 0);
        }
        let slot = 0x870 + 4 * block;
        set(e, slot - 0x10, 0);
        let held = e.mem.u32(base + slot);
        if held != 0 {
            release_object(e, held);
            set(e, slot, 0);
        }
    }
    for offset in [0x94cu32, 0x95c, 0x96c] {
        set(e, offset, zero);
    }
    set(e, 0x880, 0x16);
    set(e, 0x97c, one);
    for offset in [
        0x888u32, 0x88c, 0x8bc, 0x8c0, 0x8c4, 0x8b8, 0x8c8, 0x8ac, 0x8b0, 0x8b4, 0xa80, 0xa84,
        0xa88, 0xa90,
    ] {
        set(e, offset, 0);
    }
    e.mem.set_u16(base + 0xa8c, 0);
    for offset in [0xa94u32, 0x280, 0x284] {
        set(e, offset, 0);
    }
    e.call(MEMSET_ADDR, &args![CLEARED_BLOCK_0126EFD8, 0u32, 0x76u32]);
    for (offset, value) in BIT_TABLE_STORES.iter().chain(BIT_TABLE_LAST.iter()) {
        e.mem.set_u8(BIT_TABLE_0126EFEC + offset, *value);
    }
}

/// What the present parameters' set-up calls: the frame-buffer format chooser (`00e7aed0`,
/// unnamed; `thiscall` on the device description: windowed byte, bit depth), the format for an
/// index (`004dcbe0`, `renderer.cpp`; `cdecl`), the depth/stencil chooser (`00e7b0f0`, unnamed;
/// `thiscall` on the device description: format, back buffer format, bit depth, stencil
/// flag), the multisample type of a mode word (`00b6c200`, unnamed; `cdecl`), the presentation
/// interval of a word (`00e69e50`, unnamed; `cdecl`), the full-screen mode matcher (`00e7b190`,
/// unnamed; `thiscall` on the adapter description: format, width, height, the refresh word)
/// and `IDirect3D9::CheckDeviceMultiSampleType` (slot 0x2c by the interface's order).
const FRAMEBUFFER_FORMAT_00E7AED0: u32 = 0x00e7_aed0;
const FORMAT_FOR_INDEX_004DCBE0: u32 = 0x004d_cbe0;
const DEPTH_FORMAT_00E7B0F0: u32 = 0x00e7_b0f0;
const MULTISAMPLE_00B6C200: u32 = 0x00b6_c200;
const PRESENT_INTERVAL_00E69E50: u32 = 0x00e6_9e50;
const MATCH_MODE_00E7B190: u32 = 0x00e7_b190;
const CHECK_DEVICE_MULTI_SAMPLE_TYPE: u32 = 0x2c;
const NO_FRAMEBUFFER_FORMAT_010ED800: u32 = 0x010e_d800;
const NO_FRAMEBUFFER_FORMAT_REPORT_010ED7B8: u32 = 0x010e_d7b8;
const NO_DEPTH_FORMAT_010ED778: u32 = 0x010e_d778;
const NO_DEPTH_FORMAT_REPORT_010ED730: u32 = 0x010e_d730;
const NO_ANTIALIASING_010ED6D8: u32 = 0x010e_d6d8;
const NO_FULLSCREEN_MODE_010ED69C: u32 = 0x010e_d69c;
const NO_FULLSCREEN_MODE_REPORT_010ED650: u32 = 0x010e_d650;
const CLOSE_REFRESH_RATE_010ED5F0: u32 = 0x010e_d5f0;

// Translated from 00e6bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`): the present parameters' set-up,
/// `thiscall` with 12 words (the window, width, height, flags, a mode word, the requested
/// frame-buffer format, the depth/stencil format, the back buffer count, the swap word, the
/// refresh rate, the interval word, the present parameters); answers a byte.
///
/// Flags bit 2 clear means windowed, bit 4 set a 16-bit depth (else 32). Without a requested
/// format `00e7aed0` chooses one ("No matching framebuffer format available" when none). The
/// format of that index (`004dcbe0`) is the back buffer format; without a depth/stencil format
/// `00e7b0f0` chooses one (with flags bit 3; "No matching depth/stencil format available").
/// The 0x38 bytes are cleared and filled: width, height, format, back buffer count, the
/// multisample type of the mode word (`00b6c200`; its quality the mode word's low 31 bits for
/// type 1, else 0), the swap effect (for mode words 0 and 1: 1, 2 or 3 from the swap word 1,
/// 2, 3, else 1), the window, windowed, automatic depth-stencil (flags bit 0 clear), the
/// depth/stencil format, the lockable-back-buffer flag (mode word 1), the refresh rate and the
/// interval (`00e69e50`). `CheckDeviceMultiSampleType` failing reports "Could not support
/// requested antialiasing mode" and clears the type; otherwise the quality is capped to the
/// levels - 1. The count is kept within 1 to 3. Windowed clears the refresh rate; full screen
/// matches the mode (`00e7b190`; failing: "Could not match desired fullscreen mode") and
/// reports a changed refresh rate. Windowed intervals other than 0, 1 and 0x80000000 become
/// 0x80000000; full-screen intervals the device does not support (its caps word +0x18 of the
/// device description) become 1. Answers 1.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e6bf50(
    e: &mut Engine,
    this: Ptr,
    window: u32,
    width: u32,
    height: u32,
    flags: u32,
    mode: u32,
    framebuffer: u32,
    depth_stencil: u32,
    back_buffers: u32,
    swap: u32,
    refresh_rate: u32,
    interval: u32,
    present: Ptr,
) -> u8 {
    let this = this.addr();
    let windowed = !((flags >> 2) as u8) & 1;
    let device_desc = e.mem.u32(this + 0x5dc);
    let bit_depth = if flags as u8 & 0x10 != 0 {
        0x10u32
    } else {
        0x20
    };
    let mut framebuffer = framebuffer;
    if framebuffer == 0 {
        framebuffer = e
            .call(
                FRAMEBUFFER_FORMAT_00E7AED0,
                &args![device_desc, u32::from(windowed), bit_depth],
            )
            .u32();
        if framebuffer == 0 {
            e.call(
                STRNCPY_S,
                &args![
                    ERROR_MESSAGE_011F4648,
                    0x100u32,
                    NO_FRAMEBUFFER_FORMAT_010ED800,
                    0xffu32
                ],
            );
            e.call(
                REPORT_00A29680,
                &args![NO_FRAMEBUFFER_FORMAT_REPORT_010ED7B8],
            );
            return 0;
        }
    }
    let format = e.call(FORMAT_FOR_INDEX_004DCBE0, &args![framebuffer]).u32();
    let mut depth = depth_stencil;
    if depth == 0 {
        let display_format = fn_00e69c90(e, framebuffer);
        let device_desc = e.mem.u32(this + 0x5dc);
        depth = e
            .call(
                DEPTH_FORMAT_00E7B0F0,
                &args![device_desc, display_format, format, bit_depth, flags & 8],
            )
            .u32();
        if depth == 0 {
            e.call(
                STRNCPY_S,
                &args![
                    ERROR_MESSAGE_011F4648,
                    0x100u32,
                    NO_DEPTH_FORMAT_010ED778,
                    0xffu32
                ],
            );
            e.call(REPORT_00A29680, &args![NO_DEPTH_FORMAT_REPORT_010ED730]);
            return 0;
        }
    }
    let p = present.addr();
    e.call(MEMSET_ADDR, &args![p, 0u32, 0x38u32]);
    e.mem.set_u32(p + 8, format);
    e.mem.set_u32(p, width);
    e.mem.set_u32(p + 4, height);
    let multisample = e.call(MULTISAMPLE_00B6C200, &args![mode]).u32();
    e.mem.set_u32(p + 0x10, multisample);
    let quality = if multisample == 1 {
        mode & 0x7fff_ffff
    } else {
        0
    };
    e.mem.set_u32(p + 0x14, quality);
    e.mem.set_u32(p + 0x28, depth);
    e.mem.set_u32(p + 0xc, back_buffers);
    e.mem.set_u32(p + 0x24, !flags & 1);
    e.mem.set_u32(p + 0x20, u32::from(windowed));
    let swap_effect = if mode == 0 || mode == 1 {
        match swap {
            2 => 2,
            3 => 3,
            _ => 1,
        }
    } else {
        1
    };
    e.mem.set_u32(p + 0x18, swap_effect);
    e.mem.set_u32(p + 0x30, refresh_rate);
    e.mem.set_u32(p + 0x1c, window);
    e.mem.set_u32(p + 0x2c, u32::from(mode == 1));
    let presentation = e.call(PRESENT_INTERVAL_00E69E50, &args![interval]).u32();
    e.mem.set_u32(p + 0x34, presentation);
    let checked = e.with_stack(4, |e, levels| {
        let direct3d = e.mem.u32(DIRECT3D);
        let adapter = e.mem.u32(this + ADAPTER);
        let device_type = e.mem.u32(this + DEVICE_TYPE);
        let result = e
            .vcall(
                direct3d,
                CHECK_DEVICE_MULTI_SAMPLE_TYPE,
                &args![
                    adapter,
                    device_type,
                    format,
                    u32::from(windowed),
                    multisample,
                    levels
                ],
            )
            .i32();
        (result, e.mem.u32(levels.addr()))
    });
    if checked.0 < 0 {
        e.call(REPORT_00A29680, &args![NO_ANTIALIASING_010ED6D8]);
        e.mem.set_u32(p + 0x10, 0);
    } else if checked.1 <= e.mem.u32(p + 0x14) {
        e.mem.set_u32(p + 0x14, checked.1.wrapping_sub(1));
    }
    let count = e.mem.u32(p + 0xc);
    if count == 0 {
        e.mem.set_u32(p + 0xc, 1);
    } else if count > 3 {
        e.mem.set_u32(p + 0xc, 3);
    }
    let requested_rate = e.mem.u32(p + 0x30);
    if e.mem.u32(p + 0x20) != 0 {
        e.mem.set_u32(p + 0x30, 0);
    } else {
        let format_word = e.mem.u32(p + 8);
        let index = fn_00e69d00(e, format_word);
        let display_format = fn_00e69c90(e, index);
        let adapter_desc = e.mem.u32(this + 0x5d8);
        let width = e.mem.u32(p);
        let height = e.mem.u32(p + 4);
        let matched = e
            .call(
                MATCH_MODE_00E7B190,
                &args![adapter_desc, display_format, width, height, p + 0x30],
            )
            .u32();
        if matched == 0 {
            e.call(SET_ERROR_00E69AD0, &args![NO_FULLSCREEN_MODE_010ED69C]);
            e.call(REPORT_00A29680, &args![NO_FULLSCREEN_MODE_REPORT_010ED650]);
            return 0;
        }
        if requested_rate != e.mem.u32(p + 0x30) {
            e.call(REPORT_00A29680, &args![CLOSE_REFRESH_RATE_010ED5F0]);
        }
    }
    let interval_word = e.mem.u32(p + 0x34);
    if e.mem.u32(p + 0x20) != 0 {
        if interval_word != 0 && interval_word != 0x8000_0000 && interval_word != 1 {
            e.mem.set_u32(p + 0x34, 0x8000_0000);
        }
    } else {
        let supported = e.mem.u32(device_desc.wrapping_add(0x18));
        if interval_word & supported == 0 {
            e.mem.set_u32(p + 0x34, 1);
        }
    }
    1
}

/// The caps fields, renderer fields and globals `00e6c240` fills, and its set-up calls
/// (`00e7f830`, `00e7f840`, `00e7f850`, unnamed; `cdecl`, one word).
const CAPS_SET_UP_00E7F830: u32 = 0x00e7_f830;
const CAPS_SET_UP_00E7F840: u32 = 0x00e7_f840;
const CAPS_SET_UP_00E7F850: u32 = 0x00e7_f850;
const NO_DEVICE_CAPS_010ED924: u32 = 0x010e_d924;
const NO_DEVICE_CAPS_REPORT_010ED8E0: u32 = 0x010e_d8e0;
const NO_MULTI_TEXTURE_010ED898: u32 = 0x010e_d898;
const NO_MULTI_TEXTURE_REPORT_010ED840: u32 = 0x010e_d840;
/// `IDirect3DDevice9::GetDeviceCaps` (slot 0x1c by the interface's order).
const GET_DEVICE_CAPS_SLOT: u32 = 0x1c;

// Translated from 00e6c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall` with one word it does not
/// read; answers a byte. The device's caps go to +0x28c (`GetDeviceCaps`; failing: "Could not
/// query device caps"). Fewer than two texture blend stages or simultaneous textures (+0x320,
/// +0x324) fail ("Device does not support single-pass multi-texture"). Otherwise the renderer
/// takes its limits and flags from the caps (+0x5ec from caps bits 1 and 8, +0x6e4 the
/// stream count or 4, +0x6e8 to +0x6f5, +0x8a0 to +0x8a5, the vertex and pixel shader
/// version words at +0x918 to +0x93c), passes the stage counts to `00e7f830`, `00e7f840` and
/// `00e7f850` (0x10 for pixel shader 2.0 and above), and sets the render-state defaults at
/// `0126f050` to `0126f0e4` (their values the exe's constants and caps bits 10 and 26).
pub fn fn_00e6c240(e: &mut Engine, this: Ptr, _unused_1: u32) -> u8 {
    let this = this.addr();
    let device = e.mem.u32(this + DEVICE);
    let caps = this + 0x28c;
    if e.vcall(device, GET_DEVICE_CAPS_SLOT, &args![caps]).i32() < 0 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                NO_DEVICE_CAPS_010ED924,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![NO_DEVICE_CAPS_REPORT_010ED8E0]);
        return 0;
    }
    let field = |e: &Engine, offset: u32| e.mem.u32(this + offset);
    let stages = field(e, 0x320);
    if stages < 2 || field(e, 0x324) < 2 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                NO_MULTI_TEXTURE_010ED898,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![NO_MULTI_TEXTURE_REPORT_010ED840]);
        return 0;
    }
    let set = |e: &mut Engine, offset: u32, value: u32| e.mem.set_u32(this + offset, value);
    let set_byte = |e: &mut Engine, offset: u32, value: u8| e.mem.set_u8(this + offset, value);
    let texture_caps = field(e, 0x2c8);
    set(e, 0x5ec, 0);
    if texture_caps & 2 == 0 {
        set(e, 0x5ec, 8);
    } else if texture_caps & 0x100 != 0 {
        set(e, 0x5ec, 4);
    }
    let streams = field(e, 0x334);
    set(e, 0x6e4, streams);
    if e.mem.u8(this + 0x5d4) != 0 && e.mem.u8(this + 0x5d5) == 0 {
        set(e, 0x6e4, 4);
    }
    if field(e, 0x6e4) >= 4 {
        let value = field(e, 0x5ec) | 2;
        set(e, 0x5ec, value);
    }
    let raster = field(e, 0x2ac);
    let vertex_version = field(e, 0x348);
    let pixel_limit = field(e, 0x350);
    set_byte(e, 0x6f4, ((texture_caps >> 16) as u8) & 1);
    let primitive = field(e, 0x298);
    set_byte(e, 0x6f5, ((primitive >> 29) as u8) & 1);
    let max_vertex = field(e, 0x37c);
    set(e, 0x8a0, max_vertex);
    set_byte(e, 0x8a4, ((raster >> 18) as u8) & 1);
    set_byte(e, 0x8a5, ((raster >> 19) as u8) & 1);
    set(e, 0x6e8, vertex_version);
    let pixel_version = field(e, 0x358);
    set(e, 0x6ec, pixel_version);
    set(e, 0x6f0, pixel_limit);
    set(e, 0x918, pixel_version);
    set(e, 0x91c, 0xffff_0000);
    set(e, 0x920, pixel_version);
    set(e, 0x924, pixel_limit);
    set(e, 0x928, 0xfffe_0000);
    set(e, 0x92c, pixel_limit);
    for offset in [0x930u32, 0x934, 0x938] {
        set(e, offset, 0xfffe_0000);
    }
    set(e, 0x93c, 2);
    e.call(CAPS_SET_UP_00E7F830, &args![stages]);
    let textures = field(e, 0x324);
    e.call(CAPS_SET_UP_00E7F840, &args![textures]);
    let samplers = if field(e, 0x6ec) >= 0xffff_0200 {
        0x10
    } else {
        field(e, 0x320)
    };
    e.call(CAPS_SET_UP_00E7F850, &args![samplers]);
    let vertex_caps = field(e, 0x2cc);
    let bit_10 = ((vertex_caps >> 10) as u8) & 1;
    e.mem.set_u8(0x0126_f0d1, bit_10);
    let bit_26 = ((field(e, 0x2cc) >> 26) as u8) & 1;
    for global in [0x0126_f058u32, 0x0126_f064, 0x0126_f0ac] {
        e.mem.set_u32(global, 0);
    }
    e.mem.set_u8(0x0126_f0d2, bit_26);
    for global in [
        0x0126_f0c4u32,
        0x0126_f0b8,
        0x0126_f0b0,
        0x0126_f0b4,
        0x0126_f0a4,
    ] {
        e.mem.set_u32(global, 3);
    }
    e.mem.set_u32(0x0126_f09c, 2 + u32::from(bit_26 != 0));
    for global in [
        0x0126_f0c8u32,
        0x0126_f0cc,
        0x0126_f0c0,
        0x0126_f0bc,
        0x0126_f054,
        0x0126_f050,
        0x0126_f078,
        0x0126_f074,
        0x0126_f07c,
        0x0126_f084,
        0x0126_f080,
        0x0126_f094,
    ] {
        e.mem.set_u32(global, 1);
    }
    for global in [
        0x0126_f060u32,
        0x0126_f05c,
        0x0126_f06c,
        0x0126_f068,
        0x0126_f070,
        0x0126_f088,
        0x0126_f090,
        0x0126_f08c,
        0x0126_f0a8,
        0x0126_f0a0,
    ] {
        e.mem.set_u32(global, 2);
    }
    e.mem.set_u32(0x0126_f098, 2 + u32::from(bit_10 != 0));
    let filter = if field(e, 0x2c8) & 0x400 != 0 {
        0x101
    } else {
        0
    };
    e.mem.set_u32(0x0126_f0e4, filter + 2);
    1
}

/// What `00e6ec90` uses: the sampler set-up `00e6dfc0` (unnamed; `thiscall`: block, a byte, a
/// count), `00e6c550` (unnamed; `thiscall`, no stack words), the pixel data constructor
/// `00a7c190` (unnamed; `thiscall`: width, height, pixel format, mipmap levels, faces), the
/// fixed-string maker `00a5b690`, the source texture maker `00a5ff10` (unnamed; `cdecl`: pixel
/// data, name, format preferences), the empty fixed string (`0109b220`) and the texture's
/// name `"Clipper Image"` (`010edbb8`).
const SAMPLER_SET_UP_00E6DFC0: u32 = 0x00e6_dfc0;
const SET_UP_00E6C550: u32 = 0x00e6_c550;
const PIXEL_DATA_CONSTRUCT_00A7C190: u32 = 0x00a7_c190;
const FIXED_STRING_MAKE_00A5B690: u32 = 0x00a5_b690;
const SOURCE_TEXTURE_00A5FF10: u32 = 0x00a5_ff10;
const EMPTY_FIXED_STRING_0109B220: u32 = 0x0109_b220;
const CLIPPER_IMAGE_010EDBB8: u32 = 0x010e_dbb8;

// Translated from 00e6ec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`; the initialisation's "texture
/// defaults"), `thiscall`, no stack words; answers 1. Sets up the four sampler blocks (+0x700
/// with 0 and 3, +0x758 with 1 and 3, +0x7b0 with 0 and 5, +0x808 with 1 and 5, `00e6dfc0`),
/// runs `00e6c550`, makes a 2 by 1 pixel data (0x74 bytes, `00a7c190` with the pixel format at
/// +0x860; counted) and writes its pixels by the format's bit count (byte +1): 16 bits: bytes
/// 3 and 2 0xff, 0 and 1 0; 32 bits: bytes 3 and 2 0, 7 to 4 0xff, 0 and 1 0; others nothing.
/// The texture "Clipper Image" made from it (`00a5ff10`, preferences 6, 0, 0) is held at
/// +0x8fc (`NiPointer` assignment); the name and the pixel data are released.
pub fn fn_00e6ec90(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    for (offset, first, count) in [
        (0x700u32, 0u32, 3u32),
        (0x758, 1, 3),
        (0x7b0, 0, 5),
        (0x808, 1, 5),
    ] {
        e.call(
            SAMPLER_SET_UP_00E6DFC0,
            &args![base, base + offset, first, count],
        );
    }
    e.call(SET_UP_00E6C550, &args![base]);
    let format = e.mem.u32(base + 0x860);
    let block = e.call(NEW_OBJECT, &args![0x74u32]).u32();
    let pixels = if block != 0 {
        let made = e
            .call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![block, 2u32, 1u32, format, 1u32, 1u32],
            )
            .u32();
        if made != 0 {
            e.call(IMPORT_INTERLOCKED_UP, &args![made + 4]);
        }
        made
    } else {
        0
    };
    let data = e.mem.u32(pixels.wrapping_add(0x5c));
    let first = e
        .mem
        .u32(data)
        .wrapping_add(e.mem.u32(pixels.wrapping_add(0x50)));
    let bits = e.mem.u8(format.wrapping_add(1));
    if bits == 0x10 {
        e.mem.set_u8(first + 3, 0xff);
        e.mem.set_u8(first + 2, 0xff);
        e.mem.set_u8(first, 0);
        e.mem.set_u8(first + 1, 0);
    } else if bits == 0x20 {
        e.mem.set_u8(first + 3, 0);
        e.mem.set_u8(first + 2, 0);
        for offset in [7u32, 6, 5, 4] {
            e.mem.set_u8(first + offset, 0xff);
        }
        e.mem.set_u8(first, 0);
        e.mem.set_u8(first + 1, 0);
    }
    // The format preferences (three words) and the name's stack word.
    let texture = e.with_stack(0x10, |e, frame| {
        let preferences = frame.addr();
        let name = frame.addr() + 0xc;
        e.mem.set_u32(preferences, 6);
        e.mem.set_u32(preferences + 8, 0);
        e.mem.set_u32(preferences + 4, 0);
        let handle = e
            .call(FIXED_STRING_MAKE_00A5B690, &args![CLIPPER_IMAGE_010EDBB8])
            .u32();
        e.mem.set_u32(name, handle);
        let texture = e
            .call(SOURCE_TEXTURE_00A5FF10, &args![pixels, handle, preferences])
            .u32();
        (texture, e.mem.u32(name))
    });
    let (texture, handle) = texture;
    let held = e.mem.u32(base + 0x8fc);
    if held != texture {
        if held != 0 {
            release_object(e, held);
        }
        e.mem.set_u32(base + 0x8fc, texture);
        if texture != 0 {
            e.call(IMPORT_INTERLOCKED_UP, &args![texture + 4]);
        }
    }
    if handle != e.mem.u32(EMPTY_FIXED_STRING_0109B220) {
        e.call(IMPORT_INTERLOCKED_DOWN, &args![handle.wrapping_sub(8)]);
    }
    release_object(e, pixels);
    1
}

/// The registry entry of [`fn_00e6bf50`]: `this` and its 12 words.
fn fn_00e6bf50_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    u32::from(fn_00e6bf50(
        e,
        Ptr::new(w(0)),
        w(1),
        w(2),
        w(3),
        w(4),
        w(5),
        w(6),
        w(7),
        w(8),
        w(9),
        w(10),
        w(11),
        Ptr::new(w(12)),
    ))
    .into_ret()
}

#[cfg(test)]
mod b000v_tests {
    use super::*;

    const OBJECT_VTABLE: u32 = 0x6000;
    const SLOTS: u32 = 0x6100_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with a virtual table whose slot at byte offset `n` is `SLOTS + n`, the
    /// interlocked imports doubled (the decrement answering `count_after_release`).
    fn engine(count_after_release: u32) -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x80).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(OBJECT_VTABLE, &slots);
        e.register(IMPORT_INTERLOCKED_UP, |_, _| Ret::default());
        e.register_double(IMPORT_INTERLOCKED_DOWN, move |_, _| {
            count_after_release.into_ret()
        });
        e.register(SLOTS + 4, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, OBJECT_VTABLE);
        object
    }

    #[test]
    fn fn_00e69510_maps_device_kinds() {
        let mut e = Engine::new();
        let device_type = e.mem.alloc(4);
        let behavior = e.mem.alloc(4);
        let expected = [
            (1, 0x50),
            (1, 0x40),
            (1, 0x80),
            (1, 0x20),
            (2, 0x20),
            (2, 0x40),
            (2, 0x80),
        ];
        for (kind, (device, flag)) in expected.iter().enumerate() {
            e.mem.set_u32(behavior, 0xff);
            let ok = e
                .call(0x00e6_9510, &args![kind as u32, device_type, behavior])
                .u8();
            assert_eq!(ok, 1);
            assert_eq!(e.mem.u32(device_type), *device);
            assert_eq!(e.mem.u32(behavior), 0x0f | flag);
        }
        e.mem.set_u32(behavior, 0xff);
        assert_eq!(
            e.call(0x00e6_9510, &args![7u32, device_type, behavior])
                .u8(),
            0
        );
        assert_eq!(e.mem.u32(behavior), 0x0f);
    }

    #[test]
    fn format_tables() {
        let mut e = Engine::new();
        let mut index = |format: u32| e.call(0x00e6_9d00, &args![format]).u32();
        assert_eq!(index(0x14), 1);
        assert_eq!(index(0x1e), 0xb);
        assert_eq!(index(0x24), 0xe);
        assert_eq!(index(0x6f), 0xc);
        assert_eq!(index(0x70), 0xd);
        assert_eq!(index(0x71), 0xf);
        assert_eq!(index(0x74), 0x12);
        assert_eq!(index(0x75), 0);
        assert_eq!(index(0x23), 0);
        let mut e = Engine::new();
        let mut format = |i: u32| e.call(0x00e6_9c90, &args![i]).u32();
        let values: Vec<u32> = (0..13).map(&mut format).collect();
        assert_eq!(
            values,
            vec![0, 0x14, 0x16, 0x16, 0x17, 0x18, 0x18, 0x1e, 0x1b, 0, 0, 0x1e, 0]
        );
    }

    #[test]
    fn fn_00e6e6e0_replaces_the_held_shader() {
        let mut e = engine(0);
        let this = e.mem.alloc(0x20);
        let old = object(&mut e);
        let new = object(&mut e);
        e.mem.set_u32(this + 0x14, old);
        e.call(0x00e6_e6e0, &args![this, new]);
        assert_eq!(e.mem.u32(this + 0x14), new);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![old + 4]]);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![old]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_UP), vec![vec![new + 4]]);
        e.call_log = Some(vec![]);
        e.call(0x00e6_e6e0, &args![this, new]);
        e.call(0x00e6_e6e0, &args![this, 0u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 2);
    }

    #[test]
    fn fn_00e72cf0_sets_new_and_existing_keys() {
        let mut e = engine(1);
        let map = object(&mut e);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.register(SLOTS + 4, |_, a| (a[1] & 1).into_ret());
        e.register(SLOTS + 8, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(SLOTS + 0x14, |e, _| e.mem.alloc(0xc).into_ret());
        e.register(SLOTS + 0xc, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        let value = object(&mut e);
        e.call(0x00e7_2cf0, &args![map, 0x11u32, value]);
        let item = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(item + 4), 0x11);
        assert_eq!(e.mem.u32(item + 8), value);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        // The copy counted, the by-value pointer released.
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_UP), vec![vec![value + 4]]);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DOWN), vec![vec![value + 4]]);
        // The same key again with another value replaces it.
        let other = object(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00e7_2cf0, &args![map, 0x11u32, other]);
        assert_eq!(e.mem.u32(item + 8), other);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DOWN),
            vec![vec![value + 4], vec![other + 4]]
        );
    }

    #[test]
    fn fn_00e6b990_sets_the_renderer_defaults() {
        let mut e = engine(0);
        e.map(0x0126_e000, 0x2000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x0101_6000, 0x9000);
        e.mem.set_u32(FRUSTUM_NEAR_0101E2BC, 0.1f32.to_bits());
        e.mem.set_u32(FLOAT_SOURCE_01016410, 2.5f32.to_bits());
        for word in 0..6 {
            e.mem
                .set_u32(DEFAULT_WORDS_011A946C + 4 * word, 0x100 + word);
        }
        e.register(MEMSET_ADDR, |_, _| Ret::default());
        let this = e.mem.alloc(0xb80);
        let held = object(&mut e);
        e.mem.set_u32(this + 0x874, held);
        e.call(0x00e6_b990, &args![this]);
        assert_eq!(e.mem.f32(this + 0x5e4), 1.0);
        assert_eq!(e.mem.u32(this + 0x5e0), 0xff80_8080);
        assert_eq!(e.mem.u32(this + 0x5c8), 1);
        assert_eq!(e.mem.u32(this + 0x638), 0x101);
        assert_eq!(e.mem.u32(this + 0x660), 0x105);
        assert_eq!(e.mem.f32(this + 0x684), 0.1);
        assert_eq!(e.mem.f32(this + 0x688), 2.5);
        assert_eq!(e.mem.f32(this + 0x6b4), 1.0);
        assert_eq!(e.mem.f32(this + 0x6e0), 1.0);
        assert_eq!(e.mem.f32(this + 0x97c), 1.0);
        assert_eq!(e.mem.u32(this + 0x880), 0x16);
        assert_eq!(e.mem.u32(this + 0x874), 0);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![held]]);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC), 0x18);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC + 0x60), 0x80);
        assert_eq!(e.mem.u8(BIT_TABLE_0126EFEC + 0x61), 0x10);
        assert!(calls(&e, MEMSET_ADDR).contains(&vec![CLEARED_BLOCK_0126EFD8, 0, 0x76]));
    }

    /// A renderer for the present parameters' set-up: Direct3D at `0126f0d8` (slot 0x2c
    /// answering `levels` quality levels), a device description whose interval caps are 1,
    /// and the callees doubled.
    fn present_rig(levels: u32, mode_found: bool) -> (Engine, u32, u32) {
        let mut e = engine(1);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        let direct3d = object(&mut e);
        e.mem.set_u32(DIRECT3D, direct3d);
        e.register_double(SLOTS + CHECK_DEVICE_MULTI_SAMPLE_TYPE, move |e, a| {
            e.mem.set_u32(a[6], levels);
            (if levels == 0 { 0x8876_086cu32 } else { 0 }).into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let device_desc = e.mem.alloc(0x144);
        e.mem.set_u32(device_desc + 0x18, 1);
        e.mem.set_u32(this + 0x5dc, device_desc);
        e.mem.set_u32(this + ADAPTER, 0);
        e.mem.set_u32(this + DEVICE_TYPE, 1);
        e.register(FRAMEBUFFER_FORMAT_00E7AED0, |_, _| 3u32.into_ret());
        e.register(FORMAT_FOR_INDEX_004DCBE0, |_, _| 0x16u32.into_ret());
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0x4bu32.into_ret());
        e.register(MULTISAMPLE_00B6C200, |_, a| a[0].into_ret());
        e.register(PRESENT_INTERVAL_00E69E50, |_, a| a[0].into_ret());
        e.register_double(MATCH_MODE_00E7B190, move |e, a| {
            e.mem.set_u32(a[4], 75);
            u32::from(mode_found).into_ret()
        });
        for no_op in [MEMSET_ADDR, REPORT_00A29680, STRNCPY_S, SET_ERROR_00E69AD0] {
            e.register(no_op, |_, _| Ret::default());
        }
        (e, this, device_desc)
    }

    fn set_up(e: &mut Engine, this: u32, flags: u32, mode: u32, interval: u32) -> (u8, Vec<u32>) {
        let present = e.mem.alloc(0x38);
        let ok = e
            .call(
                0x00e6_bf50,
                &args![
                    this, 0x99u32, 640u32, 480u32, flags, mode, 0u32, 0u32, 5u32, 2u32, 60u32,
                    interval, present
                ],
            )
            .u8();
        (ok, (0..14).map(|i| e.mem.u32(present + 4 * i)).collect())
    }

    #[test]
    fn fn_00e6bf50_fills_windowed_present_parameters() {
        let (mut e, this, _) = present_rig(4, true);
        let (ok, p) = set_up(&mut e, this, 0, 1, 7);
        assert_eq!(ok, 1);
        // width, height, format, count (capped 3), type 1, quality min(1, 4 - 1), swap 2,
        // window, windowed, auto depth, depth, lockable, refresh 0, interval 0x80000000.
        assert_eq!(
            p,
            vec![
                640,
                480,
                0x16,
                3,
                1,
                1,
                2,
                0x99,
                1,
                1,
                0x4b,
                1,
                0,
                0x8000_0000
            ]
        );
        assert_eq!(calls(&e, FRAMEBUFFER_FORMAT_00E7AED0)[0][1..], [1, 0x20]);
        assert!(calls(&e, MATCH_MODE_00E7B190).is_empty());
    }

    #[test]
    fn fn_00e6bf50_matches_the_full_screen_mode() {
        let (mut e, this, _) = present_rig(0, true);
        // Flags 4 (full screen) | 0x10 (16-bit) | 1 (no auto depth); mode word 2.
        let (ok, p) = set_up(&mut e, this, 0x15, 2, 4);
        assert_eq!(ok, 1);
        // Type 2 (refused, cleared), quality 0, swap 1, refresh matched to 75, interval
        // unsupported (caps 1) -> 1.
        assert_eq!(
            p,
            vec![640, 480, 0x16, 3, 0, 0, 1, 0x99, 0, 0, 0x4b, 0, 75, 1]
        );
        assert!(calls(&e, REPORT_00A29680).contains(&vec![NO_ANTIALIASING_010ED6D8]));
        assert!(calls(&e, REPORT_00A29680).contains(&vec![CLOSE_REFRESH_RATE_010ED5F0]));
        assert_eq!(calls(&e, FRAMEBUFFER_FORMAT_00E7AED0)[0][1..], [0, 0x10]);
        let (mut e, this, _) = present_rig(4, false);
        assert_eq!(set_up(&mut e, this, 4, 0, 1).0, 0);
        assert_eq!(
            calls(&e, SET_ERROR_00E69AD0),
            vec![vec![NO_FULLSCREEN_MODE_010ED69C]]
        );
    }

    #[test]
    fn fn_00e6bf50_refuses_without_formats() {
        let (mut e, this, _) = present_rig(4, true);
        e.register(FRAMEBUFFER_FORMAT_00E7AED0, |_, _| 0u32.into_ret());
        assert_eq!(set_up(&mut e, this, 0, 0, 0).0, 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], NO_FRAMEBUFFER_FORMAT_010ED800);
        let (mut e, this, _) = present_rig(4, true);
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0u32.into_ret());
        assert_eq!(set_up(&mut e, this, 8, 0, 0).0, 0);
        assert_eq!(calls(&e, STRNCPY_S)[0][2], NO_DEPTH_FORMAT_010ED778);
        assert_eq!(
            calls(&e, DEPTH_FORMAT_00E7B0F0)[0][1..],
            [0x16, 0x16, 0x20, 8]
        );
    }

    #[test]
    fn fn_00e6c240_takes_the_device_caps() {
        let mut e = engine(1);
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        let device = object(&mut e);
        e.register(SLOTS + GET_DEVICE_CAPS_SLOT, |e, a| {
            let caps = a[1] - 0x28c;
            e.mem.set_u32(caps + 0x320, 8);
            e.mem.set_u32(caps + 0x324, 8);
            e.mem.set_u32(caps + 0x2c8, 0x1_0102);
            e.mem.set_u32(caps + 0x2cc, 0x400_0400);
            e.mem.set_u32(caps + 0x334, 16);
            e.mem.set_u32(caps + 0x358, 0xffff_0300);
            0u32.into_ret()
        });
        for no_op in [
            CAPS_SET_UP_00E7F830,
            CAPS_SET_UP_00E7F840,
            CAPS_SET_UP_00E7F850,
            STRNCPY_S,
            REPORT_00A29680,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb80);
        e.mem.set_u32(this + DEVICE, device);
        assert_eq!(e.call(0x00e6_c240, &args![this, 0u32]).u8(), 1);
        // Caps bit 1 set and 0x100 set: 4, then | 2 for 16 streams.
        assert_eq!(e.mem.u32(this + 0x5ec), 6);
        assert_eq!(e.mem.u32(this + 0x6e4), 16);
        assert_eq!(e.mem.u8(this + 0x6f4), 1);
        assert_eq!(e.mem.u32(this + 0x6ec), 0xffff_0300);
        assert_eq!(e.mem.u32(this + 0x93c), 2);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850), vec![vec![0x10]]);
        assert_eq!(e.mem.u8(0x0126_f0d1), 1);
        assert_eq!(e.mem.u8(0x0126_f0d2), 1);
        assert_eq!(e.mem.u32(0x0126_f09c), 3);
        assert_eq!(e.mem.u32(0x0126_f098), 3);
        assert_eq!(e.mem.u32(0x0126_f0e4), 2);
        assert_eq!(e.mem.u32(0x0126_f0c4), 3);
        assert_eq!(e.mem.u32(0x0126_f050), 1);
        assert_eq!(e.mem.u32(0x0126_f060), 2);
        // One texture stage: refused.
        e.register(SLOTS + GET_DEVICE_CAPS_SLOT, |e, a| {
            e.mem.set_u32(a[1] - 0x28c + 0x320, 1);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00e6_c240, &args![this, 0u32]).u8(), 0);
        assert!(calls(&e, STRNCPY_S).contains(&vec![
            ERROR_MESSAGE_011F4648,
            0x100,
            NO_MULTI_TEXTURE_010ED898,
            0xff
        ]));
    }

    #[test]
    fn fn_00e6ec90_makes_the_clipper_texture() {
        let mut e = engine(1);
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        for no_op in [SAMPLER_SET_UP_00E6DFC0, SET_UP_00E6C550] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let pixels_buffer = e.mem.alloc(0x10);
        e.register_double(PIXEL_DATA_CONSTRUCT_00A7C190, move |e, a| {
            e.mem.set_u32(a[0], OBJECT_VTABLE);
            let table = e.mem.alloc(4);
            e.mem.set_u32(table, pixels_buffer);
            e.mem.set_u32(a[0] + 0x5c, table);
            e.mem.set_u32(a[0] + 0x50, 0);
            a[0].into_ret()
        });
        e.register(FIXED_STRING_MAKE_00A5B690, |_, _| 0x7008u32.into_ret());
        let texture = e.mem.alloc(8);
        e.register_double(SOURCE_TEXTURE_00A5FF10, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 6);
            texture.into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let format = e.mem.alloc(4);
        e.mem.set_u8(format + 1, 0x20);
        e.mem.set_u32(this + 0x860, format);
        assert_eq!(e.call(0x00e6_ec90, &args![this]).u8(), 1);
        assert_eq!(
            e.mem.bytes(pixels_buffer, 8),
            vec![0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff]
        );
        assert_eq!(e.mem.u32(this + 0x8fc), texture);
        let samplers = calls(&e, SAMPLER_SET_UP_00E6DFC0);
        assert_eq!(samplers[3], vec![this, this + 0x808, 1, 5]);
        let pixel_data = calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190);
        assert_eq!(&pixel_data[0][1..], &[2, 1, format, 1, 1]);
        assert_eq!(
            calls(&e, SOURCE_TEXTURE_00A5FF10)[0][..2],
            [pixel_data[0][0], 0x7008]
        );
        // The name, then the pixel data, released.
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DOWN),
            vec![vec![0x7000], vec![pixel_data[0][0] + 4]]
        );
    }
}

// ---- Format names, intervals and the clipper data (batch b000w) ----

/// The "unknown" format string (`01015890`).
const UNKNOWN_FORMAT_01015890: u32 = 0x0101_5890;

// Translated from 00e7a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::GetD3DFormatString` (engine map), `cdecl`, a Direct3D format number: the
/// address of its `"D3DFMT_..."` name. `MET1` and the four-character codes above it (`DXT1`
/// to `DXT5`, `UYVY`, `YUY2`, `RGBG`, `GRGB`) by compare; 0x14 to 0x75 through the switch's
/// byte map at `00e7ac44` and jump table at `00e7ab7c` (read in Ghidra); everything else the
/// string at `01015890`.
pub fn ni_xenon_renderer_get_d3d_format_string(_e: &mut Engine, format: i32) -> u32 {
    if format > 0x3154_454d {
        return match format as u32 {
            0x3154_5844 => 0x010e_e908,
            0x3254_5844 => 0x010e_e914,
            0x3354_5844 => 0x010e_e8f0,
            0x3454_5844 => 0x010e_e8e4,
            0x3554_5844 => 0x010e_e8c4,
            0x3259_5559 => 0x010e_e8fc,
            0x4247_5247 => 0x010e_e8d0,
            0x4742_4752 => 0x010e_e8b0,
            0x5956_5955 => 0x010e_e8a4,
            _ => UNKNOWN_FORMAT_01015890,
        };
    }
    if format == 0x3154_454d {
        return 0x010e_e920;
    }
    match format as u32 {
        0x14 => 0x010e_ec38,
        0x15 => 0x010e_ec28,
        0x16 => 0x010e_ec18,
        0x17 => 0x010e_ec08,
        0x18 => 0x010e_ebf8,
        0x19 => 0x010e_ebe8,
        0x1a => 0x010e_ebd8,
        0x1b => 0x010e_ebc8,
        0x1c => 0x010e_ebbc,
        0x1d => 0x010e_ebac,
        0x1e => 0x010e_eb9c,
        0x1f => 0x010e_eb88,
        0x20 => 0x010e_eb78,
        0x21 => 0x010e_eb68,
        0x22 => 0x010e_eb58,
        0x23 => 0x010e_eb44,
        0x24 => 0x010e_eb30,
        0x28 => 0x010e_eb24,
        0x29 => 0x010e_eb18,
        0x32 => 0x010e_eb0c,
        0x33 => 0x010e_eb00,
        0x34 => 0x010e_eaf4,
        0x3c => 0x010e_eae8,
        0x3d => 0x010e_ead8,
        0x3e => 0x010e_eac8,
        0x3f => 0x010e_eab8,
        0x40 => 0x010e_eaa8,
        0x43 => 0x010e_ea94,
        0x46 => 0x010e_ea80,
        0x47 => 0x010e_ea74,
        0x49 => 0x010e_ea64,
        0x4b => 0x010e_ea54,
        0x4d => 0x010e_ea44,
        0x4f => 0x010e_ea34,
        0x50 => 0x010e_ea28,
        0x51 => 0x010e_e9f4,
        0x52 => 0x010e_ea10,
        0x53 => 0x010e_ea00,
        0x64 => 0x010e_e9e0,
        0x65 => 0x010e_e9d0,
        0x66 => 0x010e_e9c0,
        0x6e => 0x010e_e9ac,
        0x6f => 0x010e_e9a0,
        0x70 => 0x010e_e990,
        0x71 => 0x010e_e978,
        0x72 => 0x010e_e96c,
        0x73 => 0x010e_e95c,
        0x74 => 0x010e_e944,
        0x75 => 0x010e_e934,
        _ => UNKNOWN_FORMAT_01015890,
    }
}

// Translated from 00e69e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `cdecl`, an interval number: the
/// presentation interval flag (the jump table at `00e69e84`, read in Ghidra): 0 gives
/// 0x80000000 (immediate), 1 to 4 give 1, 2, 4 and 8; anything else 0.
pub fn fn_00e69e50(_e: &mut Engine, interval: u32) -> u32 {
    match interval {
        0 => 0x8000_0000,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        _ => 0,
    }
}

/// The pixel format's set-up `00a622d0` (unnamed; `cdecl`); the pixel data constructor is
/// `PIXEL_DATA_CONSTRUCT_00A7C190` above.
const PIXEL_FORMAT_SET_UP_00A622D0: u32 = 0x00a6_22d0;

// Translated from 00e6c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nixenonrenderer.cpp`), `thiscall`, no stack words; answers a
/// byte. For each of the four sampler blocks (0x16 words each from +0x700): the first
/// non-null pixel format among its entries 1, 0, 2, 4, 3, 5 is kept at +0x860 + 4 * block;
/// with one, a 4 by 4 pixel data (0x74 bytes, `00a7c190`) is held at +0x870 + 4 * block
/// (`NiPointer` assignment) and its pixels filled with 0xff (`memset` of its size, +0x64, times
/// 16). Then without a pixel format at +0x860 it answers 0; otherwise `00a622d0` runs on it,
/// +0x880 gets its +0x10 and it answers 1.
pub fn fn_00e6c550(e: &mut Engine, this: Ptr) -> u8 {
    let base = this.addr();
    for block in 0..4u32 {
        let first = 0x16 * block;
        let holder = base + 0x870 + 4 * block;
        let format_at = holder - 0x10;
        e.mem.set_u32(format_at, 0);
        for entry in [1u32, 0, 2, 4, 3, 5] {
            let at = base + 0x700 + 4 * (entry + first);
            if e.mem.u32(at) != 0 {
                let format = e.mem.u32(at);
                e.mem.set_u32(format_at, format);
                break;
            }
        }
        let format = e.mem.u32(format_at);
        if format == 0 {
            continue;
        }
        let allocated = e.call(NEW_OBJECT, &args![0x74u32]).u32();
        let pixels = if allocated != 0 {
            e.call(
                PIXEL_DATA_CONSTRUCT_00A7C190,
                &args![allocated, 4u32, 4u32, format, 1u32, 1u32],
            )
            .u32()
        } else {
            0
        };
        let held = e.mem.u32(holder);
        if held != pixels {
            if held != 0 {
                release_object(e, held);
            }
            e.mem.set_u32(holder, pixels);
            if pixels != 0 {
                e.call(IMPORT_INTERLOCKED_UP, &args![pixels + 4]);
            }
        }
        let data = e.mem.u32(holder);
        let size = e.mem.u32(data.wrapping_add(0x64)) << 4;
        let table = e.mem.u32(data.wrapping_add(0x5c));
        let start = e
            .mem
            .u32(table)
            .wrapping_add(e.mem.u32(data.wrapping_add(0x50)));
        e.call(MEMSET_ADDR, &args![start, 0xffu32, size]);
    }
    let format = e.mem.u32(base + 0x860);
    if format == 0 {
        return 0;
    }
    e.call(PIXEL_FORMAT_SET_UP_00A622D0, &args![format]);
    let format = e.mem.u32(base + 0x860);
    let value = e.mem.u32(format.wrapping_add(0x10));
    e.mem.set_u32(base + 0x880, value);
    1
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn get_d3d_format_string_names_formats() {
        let mut e = Engine::new();
        let mut name = |format: u32| e.call(0x00e7_a9a0, &args![format]).u32();
        assert_eq!(name(0x14), 0x010e_ec38);
        assert_eq!(name(0x16), 0x010e_ec18);
        assert_eq!(name(0x4b), 0x010e_ea54);
        assert_eq!(name(0x75), 0x010e_e934);
        assert_eq!(name(0x25), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x76), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x13), UNKNOWN_FORMAT_01015890);
        assert_eq!(name(0x3154_454d), 0x010e_e920);
        assert_eq!(name(0x3154_5844), 0x010e_e908);
        assert_eq!(name(0x5956_5955), 0x010e_e8a4);
        assert_eq!(name(0x5956_5956), UNKNOWN_FORMAT_01015890);
    }

    #[test]
    fn fn_00e69e50_maps_intervals() {
        let mut e = Engine::new();
        let values: Vec<u32> = (0..6)
            .map(|i| e.call(0x00e6_9e50, &args![i as u32]).u32())
            .collect();
        assert_eq!(values, vec![0x8000_0000, 1, 2, 4, 8, 0]);
    }

    #[test]
    fn fn_00e6c550_fills_the_clipper_pixels() {
        let mut e = Engine::new();
        e.register(0x00fd_f1c0, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET_ADDR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, |_, _| Ret::default());
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            let table = e.mem.alloc(4);
            e.mem.set_u32(table, 0x9000);
            e.mem.set_u32(a[0] + 0x5c, table);
            e.mem.set_u32(a[0] + 0x50, 0x10);
            e.mem.set_u32(a[0] + 0x64, 3);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0xb80);
        let format = e.mem.alloc(0x20);
        e.mem.set_u32(format + 0x10, 0x55);
        // Block 0: entries 1 empty, 0 set; block 2: entry 5 set; blocks 1 and 3 empty.
        e.mem.set_u32(this + 0x700, format);
        e.mem.set_u32(this + 0x700 + 4 * (0x2c + 5), 0x777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_c550, &args![this]).u8(), 1);
        assert_eq!(e.mem.u32(this + 0x860), format);
        assert_eq!(e.mem.u32(this + 0x864), 0);
        assert_eq!(e.mem.u32(this + 0x868), 0x777);
        assert_ne!(e.mem.u32(this + 0x870), 0);
        assert_eq!(e.mem.u32(this + 0x874), 0);
        assert_eq!(e.mem.u32(this + 0x880), 0x55);
        let fills: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == MEMSET_ADDR)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(fills, vec![vec![0x9010, 0xff, 0x30]; 2]);
        // No pixel format in block 0: 0.
        let empty = e.mem.alloc(0xb80);
        assert_eq!(e.call(0x00e6_c550, &args![empty]).u8(), 0);
    }
}

// ---- Batch b048 (a07) ----

layout! {
    /// `NiDX9Renderer` (PC only; RTTI `.?AVNiDX9Renderer@@`), 0xB80 bytes (its deleting
    /// destructor frees 0xB80). Fields by their use in this unit.
    pub struct NiDX9Renderer: 0xB80 {
        /// A `CRITICAL_SECTION` guarding the textures' renderer data (+0x180).
        0x180 texture_lock: u32,
        /// The vertex declaration `00e696a0` makes (position, normal, colour, one UV set).
        0x280 declaration_pncu: u32,
        /// The vertex declaration `00e69790` makes (the same with a second UV float).
        0x284 declaration_pncuu: u32,
        /// `IDirect3DDevice9*`.
        0x288 device: u32,
        /// The background colour as a `D3DCOLOR` (`0xAARRGGBB`; bytes B, G, R, A).
        0x5E0 background_color: u32,
        /// The word `00e757d0` answers.
        0x5EC word_5ec: u32,
        /// The word `00e695e0` answers.
        0x8A0 word_8a0: u32,
        /// The byte `00e695f0` answers.
        0x8A4 byte_8a4: u8,
        /// The object `00e69600` answers and `00e69620` calls.
        0x884 object_884: u32,
        /// The current render target group (`00e75810` answers it; the clear and the
        /// viewport read its size).
        0x888 target_group: u32,
        /// The object `00e69960`, `00e69970` and `00e699c0` call.
        0x8B8 object_8b8: u32,
        /// The object `00e69660` calls `00e8bf80` on.
        0x8BC object_8bc: u32,
        /// The object `00e69640` calls (vtable `010f086c`).
        0x8C4 object_8c4: u32,
        /// The object `00e71cc0` calls `00e8bd40` on.
        0x8C8 object_8c8: u32,
        /// The shaders `PurgeAllD3DShaders` releases: an `NiTPointerList` (head, tail, count).
        0x90C shader_list_head: u32,
        0x910 shader_list_tail: u32,
        0x914 shader_list_count: u32,
    }
}

/// The renderer's `+0x6FC` byte: set, `00e69660` does not end the scene.
const SKIP_END_SCENE: u32 = 0x6fc;
/// The map of textures' renderer data at +0x8CC (`NiTPointerMap`; `SetAt` `00b57ea0`,
/// `RemoveAt` `00e91fd0`).
const TEXTURE_DATA_MAP: u32 = 0x8cc;

/// `IDirect3DDevice9::EndScene` (slot 42) and `CreateVertexDeclaration` (slot 86).
const DEVICE_END_SCENE: u32 = 0xa8;
const DEVICE_CREATE_VERTEX_DECLARATION: u32 = 0x158;
/// `IUnknown::Release` (slot 2).
const RELEASE: u32 = 0x8;

/// `00a29680` (`cdecl`, a format and its arguments): the debug print.
/// `InterlockedIncrement` / `InterlockedDecrement` (KERNEL32 import slots).
/// `EnterCriticalSection` / `LeaveCriticalSection` (KERNEL32 import slots).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `LoadLibraryA` / `GetProcAddress` (KERNEL32 import slots).
const LOAD_LIBRARY_A: u32 = 0x00fd_f0b0;
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// The allocator (`NEW_OBJECT` is above): `00aa1460` (`cdecl`, block and size), `00aa1110`
/// (`cdecl`, block: free).
const SIZED_DELETE: u32 = 0x00aa_1460;
const FREE: u32 = 0x00aa_1110;
/// `NiTPointerMap::SetAt` (`00b57ea0`, `thiscall`: key, value) and `GetAt` (`00a53c30`,
/// `thiscall`: key, &value; true when found), and `RemoveAt` (`00e91fd0`, `thiscall`: key).
const MAP_SET_AT: u32 = 0x00b5_7ea0;
const MAP_GET_AT: u32 = 0x00a5_3c30;
const MAP_REMOVE_AT: u32 = 0x00e9_1fd0;

/// The `NiTPointerList` node pool's free list and its spin lock: the lock (`011c5f80`, taken
/// with `0040fbf0`, `thiscall`, one word), its count (`011c5f84`) and the free-list head
/// (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE: u32 = 0x011c_5f58;
const NODE_POOL_LOCK_TAKE: u32 = 0x0040_fbf0;

// Translated from 00a765c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::operator!=` (Xbox PDB, `operator__ov2`): `thiscall`, one word (the other
/// format). Different when the words at +4 and +8, the bytes at +0 and +1 differ, when the
/// words at +0xC differ and neither is -1, or when any of the four 0xC-byte channels from
/// +0x14 differs in its two words or its two bytes.
pub fn ni_pixel_format_operator_not_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(b.wrapping_add(4)) != e.mem.u32(a.wrapping_add(4))
        || e.mem.u8(b) != e.mem.u8(a)
        || e.mem.u32(b.wrapping_add(8)) != e.mem.u32(a.wrapping_add(8))
        || e.mem.u8(b.wrapping_add(1)) != e.mem.u8(a.wrapping_add(1))
    {
        return true;
    }
    let theirs = e.mem.u32(b.wrapping_add(0xc));
    let ours = e.mem.u32(a.wrapping_add(0xc));
    if theirs != ours && ours != 0xffff_ffff && theirs != 0xffff_ffff {
        return true;
    }
    for i in 0..4u32 {
        let channel = 0x14 + 0xc * i;
        let (x, y) = (b.wrapping_add(channel), a.wrapping_add(channel));
        if e.mem.u32(x) != e.mem.u32(y)
            || e.mem.u32(x.wrapping_add(4)) != e.mem.u32(y.wrapping_add(4))
            || e.mem.u8(x.wrapping_add(8)) != e.mem.u8(y.wrapping_add(8))
            || e.mem.u8(x.wrapping_add(9)) != e.mem.u8(y.wrapping_add(9))
        {
            return true;
        }
    }
    false
}

/// `NiProperty`'s destructor as `NiVertexColorProperty` runs it (`00a5c240`).
const PROPERTY_DESTRUCT: u32 = 0x00a5_c240;

// Translated from 00a950f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVertexColorProperty`'s scalar deleting destructor (Xbox PDB): `thiscall`, one word
/// (flags). The destructor (`00a5c240`), then with bit 0 the block goes back (`00aa1460`,
/// 0x1C bytes); answers the object.
pub fn ni_vertex_color_property_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(PROPERTY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

/// `00b71a40` (`cdecl`, four words): the conversion `GetD3DFromNi` forwards to.
const D3D_FROM_NI_00B71A40: u32 = 0x00b7_1a40;

// Translated from 00b799a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DUtility::GetD3DFromNi` (Xbox PDB): `cdecl`, two words (a destination and a source).
/// Calls `00b71a40` with the destination, the source, the source + 0x24 and the source's
/// float at +0x30, and answers what it answers.
pub fn ni_d3d_utility_get_d3d_from_ni(e: &mut Engine, destination: u32, source: Ptr) -> u32 {
    let s = source.addr();
    let value = x87_quiet_bits(e.mem.u32(s.wrapping_add(0x30)));
    e.call(
        D3D_FROM_NI_00B71A40,
        &args![destination, source, s.wrapping_add(0x24), value],
    )
    .u32()
}

/// `D3D9.DLL`'s module and its two entry points (`Direct3DCreate9Ex`, `Direct3DCreate9`).
const D3D9_MODULE: u32 = 0x0126_f0d4;
/// `"D3D9.DLL"`, `"Direct3DCreate9Ex"`, `"Direct3DCreate9"`.
const D3D9_DLL_TEXT: u32 = 0x010e_d520;
const DIRECT3D_CREATE_9_EX_TEXT: u32 = 0x010e_d50c;
const DIRECT3D_CREATE_9_TEXT: u32 = 0x010e_d4fc;

// Translated from 00e69410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether `Direct3DCreate9` is at hand (no arguments). Once
/// found it answers true at once; else loads `D3D9.DLL` and looks up `Direct3DCreate9Ex` and
/// `Direct3DCreate9` (stored at `0126f0e0` / `0126f0dc`), true when the latter was found.
pub fn fn_00e69410(e: &mut Engine) -> bool {
    if e.global::<u32>(DIRECT3D_CREATE9) != 0 {
        return true;
    }
    let module = e.call(LOAD_LIBRARY_A, &args![D3D9_DLL_TEXT]).u32();
    e.set_global(D3D9_MODULE, module);
    if module != 0 {
        let create_ex = e
            .call(GET_PROC_ADDRESS, &args![module, DIRECT3D_CREATE_9_EX_TEXT])
            .u32();
        e.set_global(DIRECT3D_CREATE9_EX, create_ex);
        let module = e.global::<u32>(D3D9_MODULE);
        let create = e
            .call(GET_PROC_ADDRESS, &args![module, DIRECT3D_CREATE_9_TEXT])
            .u32();
        e.set_global(DIRECT3D_CREATE9, create);
        if create != 0 {
            return true;
        }
    }
    false
}

// Translated from 00e695e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xD8): the word at +0x8A0.
pub fn fn_00e695e0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::word_8a0)
}

// Translated from 00e695f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xDC): the byte at +0x8A4.
pub fn fn_00e695f0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u8 {
    e.get(this, NiDX9Renderer::byte_8a4)
}

// Translated from 00e69600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xC8): the object at +0x884.
pub fn fn_00e69600(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::object_884)
}

// Translated from 00e69620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xD4): the object at +0x884's virtual +0xB0
/// with 0, answered as it is.
pub fn fn_00e69620(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_884);
    e.vcall(object, 0xb0, &args![0u32]).u32()
}

// Translated from 00e69640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xE0): a jump to the object at +0x8C4's
/// virtual +0x10 (`00e90c70` for its vtable `010f086c`, three stack words) with the same
/// three words.
pub fn fn_00e69640(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8c4);
    e.vcall(object, 0x10, &args![first, second, third]).u32()
}

/// `00e8bf80` (`thiscall`, no stack words), called on the object at +0x8BC.
const OBJECT_8BC_CALL_00E8BF80: u32 = 0x00e8_bf80;

// Translated from 00e69660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x180, the slot `Do_EndFrame` has on Xbox):
/// unless the byte at +0x6FC is set, ends the scene (`IDirect3DDevice9::EndScene`; a
/// failure answers false) and calls `00e8bf80` on the object at +0x8BC; answers true.
pub fn fn_00e69660(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> bool {
    if e.mem.u8(this.addr().wrapping_add(SKIP_END_SCENE)) == 0 {
        let device = e.get(this, NiDX9Renderer::device);
        if (e.vcall(device, DEVICE_END_SCENE, &args![]).u32() as i32) < 0 {
            return false;
        }
        let object = e.get(this, NiDX9Renderer::object_8bc);
        e.call(OBJECT_8BC_CALL_00E8BF80, &args![object]);
    }
    true
}

/// One `D3DVERTEXELEMENT9` (stream, offset, type, method, usage, usage index).
type VertexElement = (u16, u16, u8, u8, u8, u8);

/// Writes the elements as the stack array the code builds and creates the declaration into
/// `slot` (`IDirect3DDevice9::CreateVertexDeclaration`); a failure clears the slot. Answers
/// the slot's declaration.
fn create_declaration(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    slot: u32,
    elements: &[VertexElement],
) -> u32 {
    let slot = this.addr().wrapping_add(slot);
    if e.mem.u32(slot) != 0 {
        return e.mem.u32(slot);
    }
    let device = e.get(this, NiDX9Renderer::device);
    let failed = e.with_stack(8 * elements.len() as u32, |e, array| {
        for (i, (stream, offset, kind, method, usage, index)) in elements.iter().enumerate() {
            let at = array.addr() + 8 * i as u32;
            e.mem.set_u16(at, *stream);
            e.mem.set_u16(at + 2, *offset);
            e.mem.set_u8(at + 4, *kind);
            e.mem.set_u8(at + 5, *method);
            e.mem.set_u8(at + 6, *usage);
            e.mem.set_u8(at + 7, *index);
        }
        (e.vcall(
            device,
            DEVICE_CREATE_VERTEX_DECLARATION,
            &args![array, slot],
        )
        .u32() as i32)
            < 0
    });
    if failed {
        e.mem.set_u32(slot, 0);
        return 0;
    }
    e.mem.u32(slot)
}

// Translated from 00e696a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the renderer's position / normal / colour / texture vertex
/// declaration, made on first use (`thiscall`, no stack words). The elements: `FLOAT3`
/// position at 0, `FLOAT3` normal at 12, `D3DCOLOR` colour at 24, `FLOAT2` texture
/// coordinate 0 at 28, then `D3DDECL_END`. A failed creation clears the slot and answers 0.
pub fn fn_00e696a0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    create_declaration(
        e,
        this,
        0x280,
        &[
            (0, 0, 2, 0, 0, 0),
            (0, 12, 2, 0, 3, 0),
            (0, 24, 4, 0, 10, 0),
            (0, 28, 1, 0, 5, 0),
            (0xff, 0, 0x11, 0, 0, 0),
        ],
    )
}

// Translated from 00e69790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: like `00e696a0` with a sixth element, a `FLOAT1` texture
/// coordinate 1 at 36 (slot +0x284).
pub fn fn_00e69790(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    create_declaration(
        e,
        this,
        0x284,
        &[
            (0, 0, 2, 0, 0, 0),
            (0, 12, 2, 0, 3, 0),
            (0, 24, 4, 0, 10, 0),
            (0, 28, 1, 0, 5, 0),
            (0, 36, 0, 0, 5, 1),
            (0xff, 0, 0x11, 0, 0, 0),
        ],
    )
}

// Translated from 00e69960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF4): a jump to the object at +0x8B8's
/// virtual +0x54 (one stack word) with the same word.
pub fn fn_00e69960(e: &mut Engine, this: Ptr<NiDX9Renderer>, value: u32) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, 0x54, &args![value]).u32()
}

/// The purge both `PurgeVertexShader` and `PurgePixelShader` run: the object at +0x8B8's
/// virtual `own` with (0, 0); the shader's virtual `get` (a Direct3D object, released when
/// set); the shader's virtual `clear` with 0, whose answer is the result.
fn purge_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
    own: u32,
    get: u32,
    clear: u32,
) -> u32 {
    let object = e.get(this, NiDX9Renderer::object_8b8);
    e.vcall(object, own, &args![0u32, 0u32]);
    let d3d = e.vcall(shader, get, &args![]).u32();
    if d3d != 0 {
        e.vcall(d3d, RELEASE, &args![]);
    }
    e.vcall(shader, clear, &args![0u32]).u32()
}

// Translated from 00e69970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgeVertexShader` (the engine map's name): `thiscall`, one word (the
/// shader). The object at +0x8B8's virtual +0x8C with (0, 0), the shader's Direct3D vertex
/// shader (virtual +0x84) released, and the shader's virtual +0x88 with 0.
pub fn ni_xenon_renderer_purge_vertex_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
) -> u32 {
    purge_shader(e, this, shader, 0x8c, 0x84, 0x88)
}

// Translated from 00e699c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgePixelShader` (the engine map's name): as `PurgeVertexShader` with
/// the virtuals +0x7C (on both objects) and +0x80.
pub fn ni_xenon_renderer_purge_pixel_shader(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    shader: u32,
) -> u32 {
    purge_shader(e, this, shader, 0x7c, 0x7c, 0x80)
}

/// `00e7d9d0` (`cdecl`, four words: the device, &first, second, third).
const DEVICE_CALL_00E7D9D0: u32 = 0x00e7_d9d0;

// Translated from 00e69a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x154): `thiscall`, three words. False for a
/// first word of 0; else `00e7d9d0` with the device, the address of the first word (its
/// stack slot) and the other two, true when that answers nonzero.
pub fn fn_00e69a20(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> bool {
    if first == 0 {
        return false;
    }
    let device = e.get(this, NiDX9Renderer::device);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), first);
        e.call(DEVICE_CALL_00E7D9D0, &args![device, slot, second, third])
            .u32()
            != 0
    })
}

/// The adapter description `00e69a60` makes once (`0126f0e8`) and the argument it passes
/// (`0126f0f8`).
const ADAPTER_INFO_ARGUMENT: u32 = 0x0126_f0f8;

// Translated from 00e69a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the adapter description (no arguments), made on first use:
/// with `Direct3DCreate9` at hand a temporary `IDirect3D9` builds a 0x14-byte object
/// (`00e7bc90` with it and `0126f0f8`; 0 when the allocation failed) and is released.
pub fn fn_00e69a60(e: &mut Engine) -> u32 {
    let info = e.global::<u32>(ADAPTER_LIST);
    if info != 0 {
        return info;
    }
    if fn_00e69410(e) {
        let create = e.global::<u32>(DIRECT3D_CREATE9);
        let direct3d = e.call(create, &args![D3D_SDK_VERSION]).u32();
        if direct3d != 0 {
            let block = e.call(NEW_OBJECT, &args![0x14u32]).u32();
            if block != 0 {
                let info = fn_00e7bc90(e, Ptr::new(block), direct3d, ADAPTER_INFO_ARGUMENT);
                e.set_global(ADAPTER_LIST, info.addr());
                e.vcall(direct3d, RELEASE, &args![]);
                return e.global::<u32>(ADAPTER_LIST);
            }
            e.set_global(ADAPTER_LIST, 0u32);
            e.vcall(direct3d, RELEASE, &args![]);
        }
    }
    e.global::<u32>(ADAPTER_LIST)
}

// Translated from 00e69b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (an RTTI record). Walks the chain the
/// object's virtual +0xC answers (each link's next at +4) and answers the object when the
/// record is on it, else 0.
pub fn fn_00e69b00(e: &mut Engine, this: Ptr, rtti: u32) -> u32 {
    let mut link = e.vcall(this.addr(), 0xc, &args![]).u32();
    while link != 0 {
        if link == rtti {
            return this.addr();
        }
        link = e.mem.u32(link.wrapping_add(4));
    }
    0
}

// Translated from 00e69b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (an index). The element at that index of
/// the array at +0x28 when it is below the count at +0x20 (unsigned), else 0.
pub fn fn_00e69b40(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index < e.mem.u32(this.addr().wrapping_add(0x20)) {
        let array = e.mem.u32(this.addr().wrapping_add(0x28));
        e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
    } else {
        0
    }
}

// Translated from 00e6d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x104): `thiscall`, one word (an object; the
/// renderer is not read). Frees the object's block at +0x28 (`00aa1110`) and clears +0x28
/// and +0x24.
pub fn fn_00e6d740(e: &mut Engine, _this: Ptr, object: Ptr) {
    let o = object.addr();
    let block = e.mem.u32(o.wrapping_add(0x28));
    e.call(FREE, &args![block]);
    e.mem.set_u32(o.wrapping_add(0x28), 0);
    e.mem.set_u32(o.wrapping_add(0x24), 0);
}

/// `00e68ef0` (`cdecl`: texture, renderer) and `00e8abf0` (`cdecl`: texture, renderer): the
/// renderer data makers `00e6dc40` and `00e6dcc0` call.
const MAKE_TEXTURE_DATA_00E68EF0: u32 = 0x00e6_8ef0;
const MAKE_TEXTURE_DATA_00E8ABF0: u32 = 0x00e8_abf0;
/// `00e8ff60` (`cdecl`: texture, renderer, a word).
const MAKE_TEXTURE_DATA_00E8FF60: u32 = 0x00e8_ff60;

// Translated from 00e6dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x13C): `thiscall`, one word (a texture).
/// Under the lock at +0x180, makes the texture's renderer data (`00e68ef0`) unless it has
/// some at +0x24; true when it has data.
pub fn fn_00e6dc40(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> bool {
    let lock = this.addr().wrapping_add(0x180);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    let mut data = e.mem.u32(texture.addr().wrapping_add(0x24));
    if data == 0 {
        data = e
            .call(MAKE_TEXTURE_DATA_00E68EF0, &args![texture, this])
            .u32();
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    data != 0
}

// Translated from 00e6dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x140): `thiscall`, two words (a texture and
/// a word). Without renderer data at +0x24 makes it (`00e8ff60` with the texture, the
/// renderer and the word; none answers false) and records it in the map at +0x8CC under the
/// texture. True otherwise.
pub fn fn_00e6dc80(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr, value: u32) -> bool {
    if e.mem.u32(texture.addr().wrapping_add(0x24)) == 0 {
        let data = e
            .call(MAKE_TEXTURE_DATA_00E8FF60, &args![texture, this, value])
            .u32();
        if data == 0 {
            return false;
        }
        let map = this.addr().wrapping_add(TEXTURE_DATA_MAP);
        e.call(MAP_SET_AT, &args![map, texture, data]);
    }
    true
}

// Translated from 00e6dcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x144): `thiscall`, one word (a texture).
/// Takes the lock at +0x180; with renderer data at +0x24 answers true at once (the lock is
/// not given back on that path); else makes it (`00e8abf0`), gives the lock back and
/// answers whether it was made.
pub fn fn_00e6dcc0(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> bool {
    let lock = this.addr().wrapping_add(0x180);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    if e.mem.u32(texture.addr().wrapping_add(0x24)) != 0 {
        return true;
    }
    let data = e
        .call(MAKE_TEXTURE_DATA_00E8ABF0, &args![texture, this])
        .u32();
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
    data != 0
}

// Translated from 00e6df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x15C): `thiscall`, one word (a texture).
/// Clears the texture's renderer data (+0x24) and jumps to the map at +0x8CC's `RemoveAt`
/// (`00e91fd0`) with the texture.
pub fn fn_00e6df80(e: &mut Engine, this: Ptr<NiDX9Renderer>, texture: Ptr) -> u32 {
    e.mem.set_u32(texture.addr().wrapping_add(0x24), 0);
    let map = this.addr().wrapping_add(TEXTURE_DATA_MAP);
    e.call(MAP_REMOVE_AT, &args![map, texture]).u32()
}

/// `NiD3DShaderInterface::SetD3DRenderer` (`00e818d0`, `thiscall`, one word).
const SHADER_SET_D3D_RENDERER: u32 = 0x00e8_18d0;
/// `00e7db70` (`thiscall` on the shader list, one word: the address of the element):
/// `AddTail`.
const SHADER_LIST_ADD: u32 = 0x00e7_db70;
/// `00e7d5b0` (`thiscall` on the shader list): removes the tail node.
const SHADER_LIST_REMOVE_TAIL: u32 = 0x00e7_d5b0;

// Translated from 00e75310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, one word (a shader). Clears the shader's renderer
/// (`SetD3DRenderer(0)`) and adds it to the list at +0x90C (`00e7db70` with the address of
/// the stack word).
pub fn fn_00e75310(e: &mut Engine, this: Ptr<NiDX9Renderer>, shader: u32) {
    e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
    let list = this.addr().wrapping_add(0x90c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), shader);
        e.call(SHADER_LIST_ADD, &args![list, slot]);
    });
}

// Translated from 00e75340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: as `00e75310`, with the shader's virtual +0xCC called first.
pub fn fn_00e75340(e: &mut Engine, this: Ptr<NiDX9Renderer>, shader: u32) {
    e.vcall(shader, 0xcc, &args![]);
    e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
    let list = this.addr().wrapping_add(0x90c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), shader);
        e.call(SHADER_LIST_ADD, &args![list, slot]);
    });
}

/// The list node's return to the pool, as the code inlines it: the element cleared, the pool
/// lock taken (`0040fbf0` with 0), the node's previous cleared and the node pushed on the
/// free list, the lock count dropped (the lock cleared at 0).
fn free_list_node(e: &mut Engine, node: u32) {
    e.mem.set_u32(node.wrapping_add(8), 0);
    e.call(NODE_POOL_LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
    e.mem.set_u32(node.wrapping_add(4), 0);
    let free = e.global::<u32>(NODE_POOL_FREE);
    e.mem.set_u32(node, free);
    let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
    e.set_global(NODE_POOL_LOCK_COUNT, count);
    e.set_global(NODE_POOL_FREE, node);
    if count == 0 {
        e.set_global(NODE_POOL_LOCK, 0u32);
    }
}

// Translated from 00e75380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PurgeAllD3DShaders` (the engine map's name): `thiscall`, no stack
/// words. While the shader list at +0x90C is not empty: takes the head's shader, calls its
/// virtual +0xCC and `SetD3DRenderer(0)`, then removes the first node holding it (head:
/// unlinked and pooled; tail: `00e7d5b0`; middle: unlinked and pooled), the count dropping
/// for a pooled node.
pub fn ni_xenon_renderer_purge_all_d3d_shaders(e: &mut Engine, this: Ptr<NiDX9Renderer>) {
    let t = this.addr();
    let list = t.wrapping_add(0x90c);
    while e.get(this, NiDX9Renderer::shader_list_count) != 0 {
        let head = e.mem.u32(list);
        let shader = e.mem.u32(head.wrapping_add(8));
        e.vcall(shader, 0xcc, &args![]);
        e.call(SHADER_SET_D3D_RENDERER, &args![shader, 0u32]);
        let head = e.mem.u32(list);
        let mut node = head;
        let mut found = 0;
        while node != 0 {
            let next = e.mem.u32(node);
            if e.mem.u32(node.wrapping_add(8)) == shader {
                found = node;
                break;
            }
            node = next;
        }
        if found == 0 {
            continue;
        }
        if found == head {
            let next = e.mem.u32(head);
            e.mem.set_u32(list, next);
            if next != 0 {
                e.mem.set_u32(next.wrapping_add(4), 0);
            } else {
                e.mem.set_u32(list.wrapping_add(4), 0);
            }
            free_list_node(e, head);
        } else if found == e.mem.u32(list.wrapping_add(4)) {
            e.call(SHADER_LIST_REMOVE_TAIL, &args![list]);
            continue;
        } else {
            let previous = e.mem.u32(found.wrapping_add(4));
            let next = e.mem.u32(found);
            if previous != 0 {
                e.mem.set_u32(previous, next);
            }
            if next != 0 {
                e.mem.set_u32(next.wrapping_add(4), previous);
            }
            free_list_node(e, found);
        }
        let count = e.mem.u32(list.wrapping_add(8)).wrapping_sub(1);
        e.mem.set_u32(list.wrapping_add(8), count);
    }
}

// Translated from 00e757d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xA0): the word at +0x5EC.
pub fn fn_00e757d0(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::word_5ec)
}

// Translated from 00e75810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xCC): the word at +0x888.
pub fn fn_00e75810(e: &mut Engine, this: Ptr<NiDX9Renderer>) -> u32 {
    e.get(this, NiDX9Renderer::target_group)
}

/// `255.0` (`double`) and `1/255` (`double`, a float's value).
const SCALE_TO_BYTE: u32 = 0x0101_e568;
const SCALE_FROM_BYTE: u32 = 0x0102_31e8;

/// `FISTP` under the code's chop control word, keeping the low byte (`MOVZX` of it).
fn channel_byte(value: f64) -> u32 {
    (crate::x87::fistp_chop(value) as u32) & 0xff
}

// Translated from 00e75840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xB0, a `SetBackgroundColor`): `thiscall`,
/// one word (an `NiColor`). Each channel times 255, truncated, its low byte packed into
/// `0xFFRRGGBB` at +0x5E0.
pub fn fn_00e75840(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_TO_BYTE));
    let red = x87_mul(x87_load32(e.mem.u32(c)), scale);
    let mut packed = (channel_byte(red) | 0xffff_ff00) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(4))), scale);
    packed = (packed | channel_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(c.wrapping_add(8))));
    packed |= channel_byte(blue);
    e.set(this, NiDX9Renderer::background_color, packed);
}

// Translated from 00e758f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xAC, a `SetBackgroundColor` for `NiColorA`):
/// `thiscall`, one word. Alpha, red, green and blue times 255, truncated, packed into
/// `0xAARRGGBB` at +0x5E0.
pub fn fn_00e758f0(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_TO_BYTE));
    let alpha = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(0xc))), scale);
    let mut packed = channel_byte(alpha) << 8;
    let red = x87_mul(x87_load32(e.mem.u32(c)), scale);
    packed = (packed | channel_byte(red)) << 8;
    let green = x87_mul(x87_load32(e.mem.u32(c.wrapping_add(4))), scale);
    packed = (packed | channel_byte(green)) << 8;
    let blue = x87_mul(scale, x87_m32(e.mem.u32(c.wrapping_add(8))));
    packed |= channel_byte(blue);
    e.set(this, NiDX9Renderer::background_color, packed);
}

// Translated from 00e759c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xB4, a `GetBackgroundColor`): `thiscall`,
/// one word (an `NiColorA` to fill). Each byte of the colour at +0x5E0 times 1/255.
pub fn fn_00e759c0(e: &mut Engine, this: Ptr<NiDX9Renderer>, color: Ptr) {
    let t = this.addr();
    let c = color.addr();
    let scale = x87_load64(e.mem.u64(SCALE_FROM_BYTE));
    let alpha = f64::from(e.mem.u8(t + 0x5e3));
    e.mem
        .set_u32(c.wrapping_add(0xc), x87_store32(x87_mul(alpha, scale)));
    let red = f64::from(e.mem.u8(t + 0x5e2));
    e.mem.set_u32(c, x87_store32(x87_mul(red, scale)));
    let green = f64::from(e.mem.u8(t + 0x5e1));
    e.mem
        .set_u32(c.wrapping_add(4), x87_store32(x87_mul(green, scale)));
    let blue = f64::from(e.mem.u8(t + 0x5e0));
    e.mem
        .set_u32(c.wrapping_add(8), x87_store32(x87_mul(scale, blue)));
}

/// `NiDX9Renderer`'s destructor (`00e75a70`).
const RENDERER_DESTRUCT: u32 = 0x00e7_5a70;

// Translated from 00e762f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDX9Renderer`'s scalar deleting destructor (vtable slot 0): `thiscall`, one word
/// (flags). The destructor (`00e75a70`), then with bit 0 the block goes back (`00aa1460`,
/// 0xB80 bytes); answers the renderer.
pub fn fn_00e762f0(e: &mut Engine, this: Ptr<NiDX9Renderer>, flags: u32) -> Ptr<NiDX9Renderer> {
    e.call(RENDERER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xb80u32]);
    }
    this
}

/// The adapter description's vtable (`010eeca4`) and its fill (`00e7bbf0`, `thiscall`, two
/// words, in `run_00e7b200`).
const ADAPTER_INFO_VTABLE: u32 = 0x010e_eca4;
const ADAPTER_INFO_FILL: u32 = 0x00e7_bbf0;

// Translated from 00e7bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the adapter description's constructor (`thiscall`, two words:
/// the `IDirect3D9` and a word passed on). An empty array (vtable `010eeca4`, the words at
/// +0 and +8 and the counts at +0xC, +0xE, +0x10 cleared, growth 1 at +0x12), then the fill
/// (`00e7bbf0`) with both words; answers the object.
pub fn fn_00e7bc90(e: &mut Engine, this: Ptr, direct3d: u32, value: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 0x12, 1);
    e.mem.set_u16(t + 0xe, 0);
    e.mem.set_u16(t + 0xc, 0);
    e.mem.set_u16(t + 0x10, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 4, ADAPTER_INFO_VTABLE);
    e.mem.set_u32(t, 0);
    e.call(ADAPTER_INFO_FILL, &args![this, direct3d, value]);
    this
}

/// `0043a010` (`thiscall`, no stack words).
const MEMBER_CALL_0043A010: u32 = 0x0043_a010;

// Translated from 00e72e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a jump to `0043a010` on the object's member at +0xC.
pub fn fn_00e72e50(e: &mut Engine, this: Ptr) -> u32 {
    e.call(MEMBER_CALL_0043A010, &args![this.addr().wrapping_add(0xc)])
        .u32()
}

/// `00653290` (`thiscall`, one word) and its argument here (`011f4a28`); `00e8bd40`
/// (`thiscall` on the object at +0x8C8, one word).
const OBJECT_LOOKUP_00653290: u32 = 0x0065_3290;
const OBJECT_LOOKUP_ARGUMENT: u32 = 0x011f_4a28;
const OBJECT_8C8_CALL_00E8BD40: u32 = 0x00e8_bd40;

// Translated from 00e71cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF8): `thiscall`, one word (an object).
/// For a set object, what `00653290` answers for `011f4a28` on it, when set, goes to
/// `00e8bd40` on the object at +0x8C8.
pub fn fn_00e71cc0(e: &mut Engine, this: Ptr<NiDX9Renderer>, object: u32) {
    if object == 0 {
        return;
    }
    let found = e
        .call(
            OBJECT_LOOKUP_00653290,
            &args![object, OBJECT_LOOKUP_ARGUMENT],
        )
        .u32();
    if found != 0 {
        let target = e.get(this, NiDX9Renderer::object_8c8);
        e.call(OBJECT_8C8_CALL_00E8BD40, &args![target, found]);
    }
}

/// The RTTI record `00e6ee00` checks for (`0126f5d8`).
const SHADER_RTTI_0126F5D8: u32 = 0x0126_f5d8;
/// `00e7bd20` (`cdecl`, one word), `00e7b0f0` (`thiscall` on the object at +0x5DC, four
/// words: a key) and `00e7c500` (`cdecl`, one word: makes the value).
const KEY_PART_00E7BD20: u32 = 0x00e7_bd20;
const MAKE_VALUE_00E7C500: u32 = 0x00e7_c500;

// Translated from 00e6ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x94): `thiscall`, three words. Asks the
/// object at +0x884's virtual +0xC8 (with 0) for an object and keeps it when it derives from
/// the record at `0126f5d8` (`00e69b00`); builds a key with `00e7b0f0` on the object at
/// +0x5DC from that object's word +0x28, `00e7bd20` of the first word and the other two;
/// answers the map at +0xB14's value for the key when it has a nonzero one, else makes one
/// (`00e7c500`), stores it in the map and answers it.
pub fn fn_00e6ee00(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    let t = this.addr();
    let source = e.get(this, NiDX9Renderer::object_884);
    let object = e.vcall(source, 0xc8, &args![0u32]).u32();
    let derived = if object == 0 {
        0
    } else {
        fn_00e69b00(e, Ptr::new(object), SHADER_RTTI_0126F5D8)
    };
    let part = e.call(KEY_PART_00E7BD20, &args![first]).u32();
    let maker = e.mem.u32(t.wrapping_add(0x5dc));
    let word = e.mem.u32(derived.wrapping_add(0x28));
    let key = e
        .call(
            DEPTH_FORMAT_00E7B0F0,
            &args![maker, word, part, second, third],
        )
        .u32();
    let map = t.wrapping_add(0xb14);
    // The first argument's stack slot holds the map's answer.
    let found = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    if found.0 && found.1 != 0 {
        return found.1;
    }
    let value = e.call(MAKE_VALUE_00E7C500, &args![key]).u32();
    e.call(MAP_SET_AT, &args![map, key, value]);
    value
}

// Translated from 00e72c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, two words (a geometry and an object or 0). False
/// when the geometry's data (+0xB8) has a word at +0x34; true when the chain +0xBC, +0xC,
/// +0xC ends in an object with a word at +0x28 (false when the chain breaks), or when the
/// renderer's virtual +0xA0 answers bit 2. Then with the object (or the geometry's +0xC0):
/// true when the word at +0x6F0 is above `0xFFFE0000` and its virtual +0xD4 answers true,
/// or when the byte at +0x5D5 is set and its virtual +0xBC answers an object whose virtual
/// +0xA8 answers true; else false.
pub fn fn_00e72c20(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr, object: u32) -> bool {
    let t = this.addr();
    let g = geometry.addr();
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.mem.u32(data.wrapping_add(0x34)) != 0 {
        return false;
    }
    let first = e.mem.u32(g.wrapping_add(0xbc));
    if first == 0 {
        return false;
    }
    let second = e.mem.u32(first.wrapping_add(0xc));
    if second == 0 {
        return false;
    }
    let third = e.mem.u32(second.wrapping_add(0xc));
    if e.mem.u32(third.wrapping_add(0x28)) != 0 {
        return true;
    }
    if e.vcall(t, 0xa0, &args![]).u32() & 2 != 0 {
        return true;
    }
    let mut target = object;
    if target == 0 {
        target = e.mem.u32(g.wrapping_add(0xc0));
    }
    if e.mem.u32(t.wrapping_add(0x6f0)) > 0xfffe_0000
        && target != 0
        && e.vcall(target, 0xd4, &args![]).u8() != 0
    {
        return true;
    }
    if e.mem.u8(t.wrapping_add(0x5d5)) != 0
        && target != 0
        && e.vcall(target, 0xbc, &args![]).u32() != 0
    {
        let inner = e.vcall(target, 0xbc, &args![]).u32();
        if e.vcall(inner, 0xa8, &args![]).u8() != 0 {
            return true;
        }
    }
    false
}

/// The texture creators `00e6fa90` calls: `00a670c0` (`thiscall`, seven words), `00a84ca0`
/// (`thiscall` on the second word's +8), `00a7b6c0` (`thiscall`, two words), `00a760c0`
/// (`thiscall`, three words).
const TEXTURE_SET_00A670C0: u32 = 0x00a6_70c0;
const OBJECT_CALL_00A84CA0: u32 = 0x00a8_4ca0;
const TEXTURE_CALL_00A7B6C0: u32 = 0x00a7_b6c0;
const TEXTURE_CALL_00A760C0: u32 = 0x00a7_60c0;
/// `0043b300` (`cdecl`: an RTTI record, an object): whether the object is of that class;
/// the two records `00e6fa90` asks about.
const IS_KIND_OF: u32 = 0x0043_b300;
const TEXTURE_RTTI_011F4A88: u32 = 0x011f_4a88;
const TEXTURE_RTTI_011F4A70: u32 = 0x011f_4a70;

// Translated from 00e6fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall`, three words (a texture, an object, a flag); the
/// renderer is not read. Only for a texture whose word +0xE has `0x4000` in bits 12..14:
/// `00a670c0` with the word +8, then (by the byte +0x38's bits 0..3) the words +0x20, +0x24,
/// +0x28, the word +0x2C and bit 0 of the word +0xC, and the top four bits of the word +0xC.
/// With the flag set and bit 5 of the byte clear, `00a84ca0` on the object's +8; with it
/// clear and bit 4 clear, `00a7b6c0` (word +0x40, 0) or `00a760c0` (word +0x44, word +0x48,
/// 0) for the texture's class (`011f4a88` / `011f4a70`).
pub fn fn_00e6fa90(e: &mut Engine, _this: Ptr, texture: Ptr, object: Ptr, flag: u32) {
    let x = texture.addr();
    if x == 0 || e.mem.u16(x.wrapping_add(0xe)) & 0x7000 != 0x4000 {
        return;
    }
    let width = u32::from(e.mem.u16(x.wrapping_add(8)));
    let bits = u32::from(e.mem.u8(x.wrapping_add(0x38)));
    let top = u32::from(e.mem.u16(x.wrapping_add(0xc))) & 0xf000;
    if bits == 0 {
        e.call(
            TEXTURE_SET_00A670C0,
            &args![texture, width, 0u32, 0u32, 0u32, 0u32, 0u32, top],
        );
    } else {
        let first = if bits & 1 != 0 {
            e.mem.u32(x.wrapping_add(0x20))
        } else {
            0
        };
        let second = if bits & 2 != 0 {
            e.mem.u32(x.wrapping_add(0x24))
        } else {
            0
        };
        let third = if bits & 4 != 0 {
            e.mem.u32(x.wrapping_add(0x28))
        } else {
            0
        };
        let (bit, fourth) = if bits & 8 != 0 {
            (
                u32::from(e.mem.u16(x.wrapping_add(0xc))) & 1,
                e.mem.u32(x.wrapping_add(0x2c)),
            )
        } else {
            (0, 0)
        };
        let top = u32::from(e.mem.u16(x.wrapping_add(0xc))) & 0xf000;
        e.call(
            TEXTURE_SET_00A670C0,
            &args![texture, width, first, second, third, fourth, bit, top],
        );
    }
    if flag != 0 {
        if bits & 0x20 == 0 {
            let target = e.mem.u32(object.addr().wrapping_add(8));
            e.call(OBJECT_CALL_00A84CA0, &args![target]);
        }
        return;
    }
    if bits & 0x10 != 0 {
        return;
    }
    if e.call(IS_KIND_OF, &args![TEXTURE_RTTI_011F4A88, texture])
        .u8()
        != 0
    {
        let value = u32::from(e.mem.u16(x.wrapping_add(0x40)));
        e.call(TEXTURE_CALL_00A7B6C0, &args![texture, value, 0u32]);
        return;
    }
    if e.call(IS_KIND_OF, &args![TEXTURE_RTTI_011F4A70, texture])
        .u8()
        != 0
    {
        let value = u32::from(e.mem.u16(x.wrapping_add(0x44)));
        let other = e.mem.u32(x.wrapping_add(0x48));
        e.call(TEXTURE_CALL_00A760C0, &args![texture, value, other, 0u32]);
    }
}

// Translated from 00e71cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0xF0, the slot after `PrecacheGeometry`):
/// `thiscall`, one word (a geometry). For a geometry with data at +0x34 whose array (count
/// +0x20, elements +0x28) is not empty and starts with a set element: under the locks at
/// +0x80 and +0x100, for each element looks up its key (+4) in the map at +0x610 and takes
/// every record (records chained at +0x20) owned by this geometry out of the chain (a first
/// record's successor replaces it in the map, or the key goes), freeing each (0x24 bytes).
/// Then, for any geometry with data, the object at +0x8AC's virtual +0x1C with (geometry,
/// 0).
pub fn fn_00e71cf0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: u32) {
    let t = this.addr();
    if geometry == 0 {
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(0x34));
    if data == 0 {
        return;
    }
    if e.mem.u32(data.wrapping_add(0x20)) != 0 && e.mem.u32(e.mem.u32(data.wrapping_add(0x28))) != 0
    {
        let (first_lock, second_lock) = (t.wrapping_add(0x80), t.wrapping_add(0x100));
        e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
        e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
        let mut index = 0u32;
        if e.mem.u32(data.wrapping_add(0x20)) != 0 {
            loop {
                // The first pass checks the index against the count (always below it); the
                // loop's own test keeps the later ones below it too.
                let array = e.mem.u32(data.wrapping_add(0x28));
                let element = e.mem.u32(array.wrapping_add(index * 4));
                let key = e.mem.u32(element.wrapping_add(4));
                unlink_records(e, t, key, 0, geometry);
                index += 1;
                if index >= e.mem.u32(data.wrapping_add(0x20)) {
                    break;
                }
            }
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
        e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
    }
    let object = e.mem.u32(t.wrapping_add(0x8ac));
    e.vcall(object, 0x1c, &args![geometry, 0u32]);
}

/// `00e8c120` (`thiscall` on the object at +0x8BC, nine words), `00e8bfa0` (`thiscall` on it,
/// two words), and the geometry data's `00e8ef10`, `00e8f010`, `00e8f040` (`thiscall`, one
/// word each) and `00e8eeb0` (`thiscall`: whether every stream it lists has its buffer).
const GROUP_PREPARE_00E8C120: u32 = 0x00e8_c120;
const GROUP_STREAM_00E8BFA0: u32 = 0x00e8_bfa0;
const DATA_SET_COUNT_00E8EF10: u32 = 0x00e8_ef10;
const DATA_SET_FIRST_00E8F010: u32 = 0x00e8_f010;
const DATA_SET_SECOND_00E8F040: u32 = 0x00e8_f040;
const DATA_IS_PACKED_00E8EEB0: u32 = 0x00e8_eeb0;
/// The cap on bit 0 of the object's word +0xC that `00e6da60` packs (`011bf558`).
const STREAM_LIMIT_011BF558: u32 = 0x011b_f558;

// Translated from 00e6da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: prepares a geometry data's streams (`thiscall`, four words:
/// the data, the object, a partition or 0, a source or 0). Asks the source's virtual +0xB0
/// for a stream count and an object; failing that (false for a partition whose word +0x20
/// is above 4) the object at +0x8BC's `00e8c120` fills a count of 1, two words and four more
/// locals. The count goes to the data (`00e8ef10`); with a first word it goes to the data
/// (`00e8f010`) and the second into its first stream slot (when it has slots), else the
/// source's object goes to the data (`00e8f040`) and each slot gets the source's virtual
/// +0xA0 of its index. The data's word +4 packs bit 0 of the object's word +0xC (at most the
/// word at `011bf558`) into the top byte with `0x400000` (object word +0x28 set), `0x800000`
/// (word +0x24 set) and `0x200000` (+0x24 set and the word +0xC's top four bits set); with a
/// source and the byte +0x5D4 the data's byte +0x14 takes the source's virtual +0xA8. Then
/// each stream is set up (`00e8bfa0`); on a failure the ones done are undone from the
/// failing index down to 1 (the data's +8 object, virtual +0x1C) and the answer is false.
///
/// The four locals `00e8c120` fills start out as the exe leaves them (not set); here 0.
pub fn fn_00e6da60(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    data: Ptr,
    object: Ptr,
    partition: u32,
    source: u32,
) -> bool {
    let t = this.addr();
    let d = data.addr();
    let o = object.addr();
    let prepared = e.with_stack(0x24, |e, locals| {
        // +0: the count, +4 and +8: the two words, +0xC: the source's object, +0x10..+0x1C:
        // the four other locals, +0x20: the source's stack slot.
        let l = locals.addr();
        let count = l;
        let first = l + 4;
        let second = l + 8;
        let source_object = l + 0xc;
        let mut answered = false;
        if source != 0 {
            answered = e.vcall(source, 0xb0, &args![source_object, count]).u8() != 0;
        }
        if !answered {
            if partition != 0 && e.mem.u16(partition.wrapping_add(0x20)) > 4 {
                return None;
            }
            e.mem.set_u32(l + 0x20, source);
            e.mem.set_u32(count, 1);
            let group = e.get(this, NiDX9Renderer::object_8bc);
            e.call(
                GROUP_PREPARE_00E8C120,
                &args![
                    group,
                    object,
                    partition,
                    first,
                    second,
                    l + 0x1c,
                    l + 0x18,
                    l + 0x14,
                    l + 0x10,
                    l + 0x20
                ],
            );
        }
        Some((
            e.mem.u32(count),
            e.mem.u32(first),
            e.mem.u32(second),
            e.mem.u32(source_object),
        ))
    });
    let Some((count, first, second, source_object)) = prepared else {
        return false;
    };
    e.call(DATA_SET_COUNT_00E8EF10, &args![data, count]);
    if first != 0 {
        e.call(DATA_SET_FIRST_00E8F010, &args![data, first]);
        if e.mem.u32(d.wrapping_add(0x20)) > 0 {
            let slots = e.mem.u32(d.wrapping_add(0x24));
            e.mem.set_u32(slots, second);
        }
    } else {
        e.call(DATA_SET_SECOND_00E8F040, &args![data, source_object]);
        let mut index = 0u32;
        while index < count {
            let stream = e.vcall(source, 0xa0, &args![index]).u32();
            if index < e.mem.u32(d.wrapping_add(0x20)) {
                let slots = e.mem.u32(d.wrapping_add(0x24));
                e.mem.set_u32(slots.wrapping_add(index * 4), stream);
            }
            index += 1;
        }
    }
    let has_28 = e.mem.u32(o.wrapping_add(0x28)) != 0;
    let has_24 = e.mem.u32(o.wrapping_add(0x24)) != 0;
    let top_bits = has_24 && e.mem.u16(o.wrapping_add(0xc)) & 0xf000 != 0;
    let mut low = u32::from(e.mem.u16(o.wrapping_add(0xc))) & 1;
    let limit = e.global::<u32>(STREAM_LIMIT_011BF558);
    if low > limit {
        low = limit;
    }
    let mut flags = 0u32;
    if has_28 {
        flags = 0x40_0000;
    }
    if has_24 {
        flags |= 0x80_0000;
    }
    if top_bits {
        flags |= 0x20_0000;
    }
    e.mem.set_u32(d.wrapping_add(4), (low << 24) | flags);
    if source != 0 && e.mem.u8(t.wrapping_add(0x5d4)) != 0 {
        let value = e.vcall(source, 0xa8, &args![]).u8();
        e.mem.set_u8(d.wrapping_add(0x14), value);
    }
    let mut index = 0u32;
    while index < count {
        let group = e.get(this, NiDX9Renderer::object_8bc);
        if e.call(GROUP_STREAM_00E8BFA0, &args![group, data, index])
            .u8()
            == 0
        {
            while index != 0 {
                let owner = e.mem.u32(d.wrapping_add(8));
                e.vcall(owner, 0x1c, &args![data, index]);
                index -= 1;
            }
            return false;
        }
        index += 1;
    }
    true
}

/// A geometry record (0x24 bytes) that `00e72980` and `00e72aa0` chain per key in the map at
/// +0x610 (the previous head at +0x20).
const RECORD_SIZE: u32 = 0x24;

/// Records one stream key of `data` in the map at +0x610: a new record (filled by `fill`)
/// whose +0x20 links the key's previous record (when the map had one), stored under the key.
fn chain_record(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    key: u32,
    fill: impl FnOnce(&mut Engine, u32),
) {
    let record = e.call(NEW_OBJECT, &args![RECORD_SIZE]).u32();
    fill(e, record);
    let map = this.addr().wrapping_add(0x610);
    let (hit, previous) = e.with_stack(4, |e, slot| {
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    e.mem
        .set_u32(record.wrapping_add(0x20), if hit { previous } else { 0 });
    e.call(MAP_SET_AT, &args![map, key, record]);
}

/// The key of stream `index` of `data` (+0x28's element, its word +4; the element is 0 past
/// the count at +0x20).
fn stream_key(e: &Engine, data: u32, index: u32) -> u32 {
    let element = if index < e.mem.u32(data.wrapping_add(0x20)) {
        e.mem
            .u32(e.mem.u32(data.wrapping_add(0x28)).wrapping_add(index * 4))
    } else {
        0
    };
    e.mem.u32(element.wrapping_add(4))
}

// Translated from 00e72980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: precaches one geometry's buffers (`thiscall`, eight words; the
/// first is not read). True at once when the object's data (+0x34) is packed (`00e8eeb0`).
/// Else the data takes the object's virtual +0x90 (+0x18), its word +8 (+0x1C), the third
/// and fourth words as 16-bit values (+0x40, +0x44), the fifth and sixth (+0x50, +0x4C) and
/// the seventh as a 16-bit value (+0x48); `00e6da60` prepares it (with no partition and the
/// eighth word as source; false ends here) and each of its streams is recorded under its key
/// (object, source, the data and the stream index).
pub fn fn_00e72980(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    _unused_1: u32,
    object: Ptr,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    source: u32,
) -> bool {
    let o = object.addr();
    let data = e.mem.u32(o.wrapping_add(0x34));
    if e.call(DATA_IS_PACKED_00E8EEB0, &args![data]).u8() != 0 {
        return true;
    }
    let vertices = u32::from(e.vcall(o, 0x90, &args![]).u16());
    let word_8 = u32::from(e.mem.u16(o.wrapping_add(8)));
    e.mem.set_u32(data.wrapping_add(0x1c), word_8);
    e.mem.set_u32(data.wrapping_add(0x18), vertices);
    e.mem.set_u32(data.wrapping_add(0x50), fifth);
    e.mem.set_u32(data.wrapping_add(0x44), fourth & 0xffff);
    e.mem.set_u32(data.wrapping_add(0x40), third & 0xffff);
    e.mem.set_u32(data.wrapping_add(0x4c), sixth);
    e.mem.set_u32(data.wrapping_add(0x48), seventh & 0xffff);
    let prepared = fn_00e6da60(e, this, Ptr::new(data), object, 0, source);
    if !prepared {
        return false;
    }
    let count = e.mem.u32(data.wrapping_add(0x20));
    let mut index = 0u32;
    while index < count {
        let key = stream_key(e, data, index);
        chain_record(e, this, key, |e, record| {
            e.mem.set_u32(record, o);
            e.mem.set_u32(record.wrapping_add(0xc), source);
            for offset in [4, 8, 0x10, 0x14] {
                e.mem.set_u32(record.wrapping_add(offset), 0);
            }
            e.mem.set_u32(record.wrapping_add(0x18), data);
            e.mem.set_u32(record.wrapping_add(0x1c), index);
        });
        index += 1;
    }
    prepared
}

// Translated from 00e72aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: precaches a skinned geometry's partitions (`thiscall`, six
/// words; the first is not read). For each 0x2C-byte partition of the third word's +0xC
/// object (count +8, array +0xC) whose data (+0x28) is not packed: the data takes the
/// partition's 16-bit words +0x1C (+0x18 and +0x1C), +0x22 (+0x48; 0 becomes 1) and +0x1E
/// (+0x40, +0x44) and its words +0x14 (+0x50) and +0x18 (+0x4C); `00e6da60` prepares it with
/// the second word, the partition and the fourth word (a failure makes the answer false and
/// moves on), and each stream is recorded under its key (second, third, partition, fourth,
/// fifth and sixth words, the data, the stream index). Answers true unless a preparation
/// failed.
pub fn fn_00e72aa0(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    _unused_1: u32,
    object: u32,
    skin: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> bool {
    let instance = e.mem.u32(skin.wrapping_add(0xc));
    let partitions = e.mem.u32(instance.wrapping_add(0xc));
    let mut remaining = e.mem.u32(instance.wrapping_add(8));
    let mut result = true;
    if remaining == 0 {
        return result;
    }
    let mut at = partitions.wrapping_add(0x1c);
    loop {
        let data = e.mem.u32(at.wrapping_add(0xc));
        if e.call(DATA_IS_PACKED_00E8EEB0, &args![data]).u8() == 0 {
            let vertices = u32::from(e.mem.u16(at));
            let strips = e.mem.u16(at.wrapping_add(6));
            let bones = u32::from(e.mem.u16(at.wrapping_add(2)));
            let word_14 = e.mem.u32(at.wrapping_sub(8));
            let word_18 = e.mem.u32(at.wrapping_sub(4));
            e.mem.set_u32(data.wrapping_add(0x18), vertices);
            e.mem.set_u32(data.wrapping_add(0x1c), vertices);
            let strips = if strips == 0 { 1 } else { u32::from(strips) };
            e.mem.set_u32(data.wrapping_add(0x48), strips);
            e.mem.set_u32(data.wrapping_add(0x40), bones);
            e.mem.set_u32(data.wrapping_add(0x44), bones);
            e.mem.set_u32(data.wrapping_add(0x50), word_14);
            e.mem.set_u32(data.wrapping_add(0x4c), word_18);
            let partition = at.wrapping_sub(0x1c);
            if fn_00e6da60(e, this, Ptr::new(data), Ptr::new(object), partition, fourth) {
                let count = e.mem.u32(data.wrapping_add(0x20));
                let mut index = 0u32;
                while index < count {
                    let key = stream_key(e, data, index);
                    chain_record(e, this, key, |e, record| {
                        e.mem.set_u32(record.wrapping_add(8), partition);
                        e.mem.set_u32(record, object);
                        e.mem.set_u32(record.wrapping_add(0xc), fourth);
                        e.mem.set_u32(record.wrapping_add(0x10), fifth);
                        e.mem.set_u32(record.wrapping_add(4), skin);
                        e.mem.set_u32(record.wrapping_add(0x14), sixth);
                        e.mem.set_u32(record.wrapping_add(0x20), 0);
                        e.mem.set_u32(record.wrapping_add(0x18), data);
                        e.mem.set_u32(record.wrapping_add(0x1c), index);
                    });
                    index += 1;
                }
            } else {
                result = false;
            }
        }
        at = at.wrapping_add(0x2c);
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
    result
}

/// `NiD3DGeometryGroupManager::RemoveSkinPartitionFromGroup` (`00e88d00`, `thiscall` on the
/// object at +0x8AC, one word).
const REMOVE_SKIN_PARTITION_FROM_GROUP: u32 = 0x00e8_8d00;

/// Takes out of the map at +0x610 every record under `key` whose word at `owner_offset` is
/// `owner`, as `00e71cf0` and `00e71e40` inline it: a removed first record's successor takes
/// its place in the map (or the key goes); the record is freed (0x24 bytes).
fn unlink_records(e: &mut Engine, this: u32, key: u32, owner_offset: u32, owner: u32) {
    let map = this.wrapping_add(0x610);
    let (hit, head) = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let hit = e.call(MAP_GET_AT, &args![map, key, slot]).u8() != 0;
        (hit, e.mem.u32(slot.addr()))
    });
    if !hit {
        return;
    }
    let mut record = head;
    let mut previous = 0u32;
    while record != 0 {
        if e.mem.u32(record.wrapping_add(owner_offset)) == owner {
            let next;
            if previous != 0 {
                let after = e.mem.u32(record.wrapping_add(0x20));
                e.mem.set_u32(previous.wrapping_add(0x20), after);
                next = e.mem.u32(record.wrapping_add(0x20));
            } else if e.mem.u32(record.wrapping_add(0x20)) != 0 {
                let after = e.mem.u32(record.wrapping_add(0x20));
                e.call(MAP_SET_AT, &args![map, key, after]);
                next = e.mem.u32(record.wrapping_add(0x20));
            } else {
                e.call(MAP_REMOVE_AT, &args![map, key]);
                next = 0;
            }
            e.mem.set_u32(record.wrapping_add(0x20), 0);
            e.call(SIZED_DELETE, &args![record, RECORD_SIZE]);
            record = next;
        } else {
            previous = record;
            record = e.mem.u32(record.wrapping_add(0x20));
        }
    }
}

// Translated from 00e71e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x100): purges a skin instance's partitions
/// (`thiscall`, one word). Only for an instance whose partition object (+0xC) has data on its
/// first partition (+0x28): for each of the count (+8) 0x2C-byte partitions whose data has
/// streams starting with a set one, under the locks at +0x80 and +0x100 the records of that
/// partition (record +8) leave the map at +0x610 under each stream's key. Then the group
/// manager at +0x8AC removes the instance (`RemoveSkinPartitionFromGroup`).
pub fn fn_00e71e40(e: &mut Engine, this: Ptr<NiDX9Renderer>, instance: u32) {
    let t = this.addr();
    if instance == 0 {
        return;
    }
    let partitions = e.mem.u32(instance.wrapping_add(0xc));
    if partitions == 0 || e.mem.u32(partitions.wrapping_add(0x28)) == 0 {
        return;
    }
    let mut remaining = e.mem.u32(instance.wrapping_add(8));
    if remaining != 0 {
        let mut partition = partitions;
        loop {
            let data = e.mem.u32(partition.wrapping_add(0x28));
            if e.mem.u32(data.wrapping_add(0x20)) != 0
                && e.mem.u32(e.mem.u32(data.wrapping_add(0x28))) != 0
            {
                let (first_lock, second_lock) = (t.wrapping_add(0x80), t.wrapping_add(0x100));
                e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
                e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
                let mut index = 0u32;
                if e.mem.u32(data.wrapping_add(0x20)) != 0 {
                    loop {
                        let array = e.mem.u32(data.wrapping_add(0x28));
                        let element = e.mem.u32(array.wrapping_add(index * 4));
                        let key = e.mem.u32(element.wrapping_add(4));
                        unlink_records(e, t, key, 8, partition);
                        index += 1;
                        if index >= e.mem.u32(data.wrapping_add(0x20)) {
                            break;
                        }
                    }
                }
                e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
                e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
            }
            partition = partition.wrapping_add(0x2c);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    e.call(REMOVE_SKIN_PARTITION_FROM_GROUP, &args![manager, instance]);
}

/// `00e6dfc0` (`thiscall`, three words: a format table, a word, a count): fills a table of
/// formats.
/// `00a5b690` (`cdecl`, one word: a C string): an `NiFixedString` handle for the string.
/// `NiSourceTexture::Create_ov2` (`00a5ff10`, `cdecl`: pixel data, name, &format prefs).
/// `"Clipper Image"`, the clipper texture's name.
/// The `NiFixedString` handle whose count is not kept (`0109b220`).
/// The clipper texture (`NiPointer` at +0x8FC).
#[cfg(test)]
const CLIPPER_TEXTURE: u32 = 0x8fc;

/// `NiGeometry`'s RTTI record (`011f5b70`), the two classes `PrecacheGeometry` packs as
/// triangle lists (`011f4a40`) or strips (`011f4a20`), and their data's `GetTriList`
/// (`00a7b850`) and `GetStripLists` (`00a75f10`; `thiscall`).
const GEOMETRY_RTTI: u32 = 0x011f_5b70;
const TRI_SHAPE_RTTI_011F4A40: u32 = 0x011f_4a40;
const TRI_STRIPS_RTTI_011F4A20: u32 = 0x011f_4a20;
const TRI_SHAPE_DATA_GET_TRI_LIST: u32 = 0x00a7_b850;
const TRI_STRIPS_DATA_GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// `NiD3DGeometryGroupManager::AddObjectToGroup_ov2` (`00e88dc0`, `thiscall` on the manager
/// at +0x8AC, six words).
const ADD_OBJECT_TO_GROUP: u32 = 0x00e8_8dc0;
/// `"PrecacheGeometry> No shader found for object %s!\n"`.
const NO_SHADER_TEXT: u32 = 0x010e_e404;

// Translated from 00e73f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiXenonRenderer::PrecacheGeometry` (the engine map's name; renderer vtable +0xEC):
/// `thiscall`, four words (the geometry, two words for the group, a source or 0). False for
/// no geometry or one that is not an `NiGeometry` (its RTTI chain); false with a message for
/// one without a shader (+0xC0). Else, under the locks at +0x80 and +0x100: whether it is
/// skinned (`00e72c20`), the source (or the shader's virtual +0xBC), the group manager at
/// +0x8AC adds it (`AddObjectToGroup`: the group at +0x8B0, the data +0xB8, the skin +0xBC,
/// the skinned flag, the two words); a skinned geometry packs its partitions (`00e72aa0`),
/// a triangle shape or strips its data (`00e72980` with the data's virtual +0xA8, its word
/// +0x40, its triangle list or strip lists, and 0/1 or the strips' words +0x48/+0x44). The
/// answer is the packing's (false for other classes).
///
/// The skinned flag is a byte local passed as a whole stack word; its upper bytes are not set
/// in the exe (here 0).
pub fn ni_xenon_renderer_precache_geometry(
    e: &mut Engine,
    this: Ptr<NiDX9Renderer>,
    geometry: u32,
    first: u32,
    second: u32,
    source: u32,
) -> bool {
    let t = this.addr();
    if geometry == 0 {
        return false;
    }
    let mut rtti = e.vcall(geometry, 8, &args![]).u32();
    loop {
        if rtti == 0 {
            return false;
        }
        if rtti == GEOMETRY_RTTI {
            break;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    let skin = e.mem.u32(geometry.wrapping_add(0xbc));
    let shader = e.mem.u32(geometry.wrapping_add(0xc0));
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    if shader == 0 {
        let name = e.mem.u32(geometry.wrapping_add(8));
        e.call(REPORT_00A29680, &args![NO_SHADER_TEXT, name]);
        return false;
    }
    let skinned = fn_00e72c20(e, this, Ptr::new(geometry), 0);
    let first_lock = t.wrapping_add(0x80);
    e.call(ENTER_CRITICAL_SECTION, &args![first_lock]);
    let second_lock = t.wrapping_add(0x100);
    e.call(ENTER_CRITICAL_SECTION, &args![second_lock]);
    let mut source = source;
    if source == 0 {
        source = e.vcall(shader, 0xbc, &args![]).u32();
    }
    let manager = e.mem.u32(t.wrapping_add(0x8ac));
    let group = e.mem.u32(t.wrapping_add(0x8b0));
    e.call(
        ADD_OBJECT_TO_GROUP,
        &args![
            manager,
            group,
            data,
            skin,
            u32::from(skinned),
            first,
            second
        ],
    );
    let mut result = false;
    if skinned {
        result = fn_00e72aa0(e, this, geometry, data, skin, source, first, second);
    } else if e
        .call(IS_KIND_OF, &args![TRI_SHAPE_RTTI_011F4A40, geometry])
        .u8()
        != 0
    {
        let count = u32::from(e.vcall(data, 0xa8, &args![]).u16());
        let word_40 = u32::from(e.mem.u16(data.wrapping_add(0x40)));
        let list = e.call(TRI_SHAPE_DATA_GET_TRI_LIST, &args![data]).u32();
        result = fn_00e72980(
            e,
            this,
            geometry,
            Ptr::new(data),
            count,
            word_40,
            list,
            0,
            1,
            source,
        );
    } else if e
        .call(IS_KIND_OF, &args![TRI_STRIPS_RTTI_011F4A20, geometry])
        .u8()
        != 0
    {
        let count = u32::from(e.vcall(data, 0xa8, &args![]).u16());
        let word_40 = u32::from(e.mem.u16(data.wrapping_add(0x40)));
        let lists = e.call(TRI_STRIPS_DATA_GET_STRIP_LISTS, &args![data]).u32();
        let word_48 = e.mem.u32(data.wrapping_add(0x48));
        let word_44 = u32::from(e.mem.u16(data.wrapping_add(0x44)));
        result = fn_00e72980(
            e,
            this,
            geometry,
            Ptr::new(data),
            count,
            word_40,
            lists,
            word_48,
            word_44,
            source,
        );
    }
    e.call(LEAVE_CRITICAL_SECTION, &args![second_lock]);
    e.call(LEAVE_CRITICAL_SECTION, &args![first_lock]);
    result
}

/// `_ftol2_sse` (`00ec62c0`): truncates ST0 (a leading `f64` argument) to an integer in EAX.
const FTOL: u32 = 0x00ec_62c0;
/// `4294967296.0` (`float`), added to a negative `FILD` of an unsigned value.
/// The record `00e6f310` checks the render target's depth buffer against (`0126f5ac`) and its
/// two queries (`00e7cdf0`: has a depth buffer with a stencil; `00e7ce10`: has a depth
/// buffer; `thiscall`).
const DEPTH_BUFFER_RTTI_0126F5AC: u32 = 0x0126_f5ac;
const DEPTH_HAS_STENCIL_00E7CDF0: u32 = 0x00e7_cdf0;
const DEPTH_HAS_DEPTH_00E7CE10: u32 = 0x00e7_ce10;
/// `IDirect3DDevice9::Clear` (slot 43) and `SetViewport` (slot 47).
const DEVICE_CLEAR: u32 = 0xac;
const DEVICE_SET_VIEWPORT: u32 = 0xbc;

/// `FILD` of an unsigned word as the code does it (`FILD`; plus 2^32 when negative), stored
/// as a float.
fn unsigned_to_float(e: &Engine, value: u32) -> u32 {
    let mut x = f64::from(value as i32);
    if (value as i32) < 0 {
        x = x87_add(x, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_store32(x)
}

// Translated from 00e6f310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x188, `Do_ClearBuffer`'s slot on Xbox):
/// `thiscall`, two words (a rectangle of four floats or 0, the buffers to clear: bit 0 the
/// target, bit 1 the stencil, bit 2 the depth). Nothing without buffers, with the byte at
/// +0x6FC set or without a render target group (+0x888). The rectangle (left, top, right,
/// bottom as fractions) becomes a `D3DRECT` of the target's size (virtuals +0x8C, +0x90 with
/// 0): `left·w`, `(1 − top)·h`, `right·w`, `(1 − bottom)·h`, truncated; without one the whole
/// target. The depth and stencil bits count only when the group's depth buffer (virtual
/// +0xCC, of the class `0126f5ac`) has them. The viewport (+0x6CC; width and height at
/// +0x6D4, +0x6D8 from the target) is set and the device clears with the colour (+0x5E0),
/// depth (+0x5E4) and stencil (+0x5E8).
pub fn fn_00e6f310(e: &mut Engine, this: Ptr<NiDX9Renderer>, rect: u32, buffers: u32) {
    let t = this.addr();
    if buffers == 0 || e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    if group == 0 {
        return;
    }
    let width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let group = e.get(this, NiDX9Renderer::target_group);
    let height = e.vcall(group, 0x90, &args![0u32]).u32();
    let corners: [u32; 4] = if rect != 0 {
        let w = x87_load32(unsigned_to_float(e, width));
        let h = x87_load32(unsigned_to_float(e, height));
        let left = x87_mul(x87_load32(e.mem.u32(rect)), w);
        let x1 = e.call(FTOL, &args![left]).u32();
        let top = x87_mul(x87_sub(1.0, x87_load32(e.mem.u32(rect + 8))), h);
        let y1 = e.call(FTOL, &args![top]).u32();
        let right = x87_mul(w, x87_load32(e.mem.u32(rect + 4)));
        let x2 = e.call(FTOL, &args![right]).u32();
        let bottom = x87_mul(x87_sub(1.0, x87_load32(e.mem.u32(rect + 0xc))), h);
        let y2 = e.call(FTOL, &args![bottom]).u32();
        [x1, y1, x2, y2]
    } else {
        [0, 0, width, height]
    };
    let mut flags = buffers & 1;
    let group = e.get(this, NiDX9Renderer::target_group);
    let depth = e.vcall(group, 0xcc, &args![]).u32();
    if depth != 0 {
        let buffer = fn_00e69b00(e, Ptr::new(depth), DEPTH_BUFFER_RTTI_0126F5AC);
        if buffer != 0 {
            if buffers & 4 != 0 && e.call(DEPTH_HAS_STENCIL_00E7CDF0, &args![buffer]).u8() != 0 {
                flags |= 2;
            }
            if buffers & 2 != 0 && e.call(DEPTH_HAS_DEPTH_00E7CE10, &args![buffer]).u8() != 0 {
                flags |= 4;
            }
        }
    }
    let group = e.get(this, NiDX9Renderer::target_group);
    let viewport_width = e.vcall(group, 0x8c, &args![0u32]).u32();
    e.mem.set_u32(t + 0x6d4, viewport_width);
    let group = e.get(this, NiDX9Renderer::target_group);
    let viewport_height = e.vcall(group, 0x90, &args![0u32]).u32();
    e.mem.set_u32(t + 0x6d8, viewport_height);
    let device = e.get(this, NiDX9Renderer::device);
    e.vcall(device, DEVICE_SET_VIEWPORT, &args![t + 0x6cc]);
    let z = x87_quiet_bits(e.mem.u32(t + 0x5e4));
    let device = e.get(this, NiDX9Renderer::device);
    let stencil = e.mem.u32(t + 0x5e8);
    let color = e.get(this, NiDX9Renderer::background_color);
    e.with_stack(0x10, |e, area| {
        for (i, v) in corners.iter().enumerate() {
            e.mem.set_u32(area.addr() + 4 * i as u32, *v);
        }
        e.vcall(
            device,
            DEVICE_CLEAR,
            &args![1u32, area, flags, color, z, stencil],
        );
    });
}

/// `IDirect3DDevice9::DrawPrimitive` (slot 81) and `DrawIndexedPrimitive` (slot 82).
const DEVICE_DRAW_PRIMITIVE: u32 = 0x144;
const DEVICE_DRAW_INDEXED_PRIMITIVE: u32 = 0x148;
/// The bound `00e74840` copies when the geometry has none (`011f4288`).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// The renderer's copy of the drawn object's bound (centre at +0x664, radius at +0x670).
const DRAWN_BOUND: u32 = 0x664;

// Translated from 00e74840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1B8): draws a packed geometry (`thiscall`,
/// one word). Nothing with the byte at +0x6FC set or for data (+0xB8) without vertices
/// (virtual +0x90). The bound (+0x20, or the one at `011f4288`) is copied to +0x664; the
/// shader's virtual +0x90 is called. A skinned geometry (`00e72c20`) draws each partition's
/// strips (or its triangle list) with `DrawIndexedPrimitive` (type 5 when the first
/// partition has strips, else 4; the partition's base vertex +0x38 of its data, its vertex
/// count, running start indices); else the renderer data (+0x34 of the geometry data) draws
/// indexed (with an index buffer at +0x34) or not (`DrawPrimitive`). Then the data's word
/// +0xE keeps only its top four bits.
pub fn fn_00e74840(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    let bound = match e.mem.u32(g.wrapping_add(0x20)) {
        0 => DEFAULT_BOUND,
        bound => bound,
    };
    let center_y = e.mem.u32(bound.wrapping_add(4));
    let center_z = e.mem.u32(bound.wrapping_add(8));
    let center_x = e.mem.u32(bound);
    let radius = e.mem.u32(bound.wrapping_add(0xc));
    let skinned = fn_00e72c20(e, this, geometry, 0);
    let render_data = e
        .mem
        .u32(e.mem.u32(g.wrapping_add(0xb8)).wrapping_add(0x34));
    let shader = e.mem.u32(g.wrapping_add(0xc0));
    e.vcall(shader, 0x90, &args![]);
    e.mem.set_u32(t + DRAWN_BOUND + 0xc, x87_quiet_bits(radius));
    e.mem.set_u32(t + DRAWN_BOUND, center_x);
    e.mem.set_u32(t + DRAWN_BOUND + 4, center_y);
    e.mem.set_u32(t + DRAWN_BOUND + 8, center_z);
    if skinned {
        let instance = e.mem.u32(skin.wrapping_add(0xc));
        let mut remaining = e.mem.u32(instance.wrapping_add(8));
        let partitions = e.mem.u32(instance.wrapping_add(0xc));
        let kind = u32::from(e.mem.u16(partitions.wrapping_add(0x22)) != 0) + 4;
        if remaining != 0 {
            let mut at = partitions.wrapping_add(0x1c);
            loop {
                let part_data = e.mem.u32(at.wrapping_add(0xc));
                let mut strip = 0u32;
                let mut start = 0u32;
                while strip < e.mem.u32(part_data.wrapping_add(0x48)) {
                    let lengths = e.mem.u32(part_data.wrapping_add(0x4c));
                    let count = if lengths != 0 {
                        u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
                    } else {
                        e.mem.u32(part_data.wrapping_add(0x40))
                    };
                    let vertices = u32::from(e.mem.u16(at));
                    let device = e.get(this, NiDX9Renderer::device);
                    let base = e.mem.u32(part_data.wrapping_add(0x38));
                    e.vcall(
                        device,
                        DEVICE_DRAW_INDEXED_PRIMITIVE,
                        &args![kind, base, 0u32, vertices, start, count],
                    );
                    strip += 1;
                    start = start.wrapping_add(count).wrapping_add(2);
                }
                at = at.wrapping_add(0x2c);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
    } else if e.mem.u32(render_data.wrapping_add(0x34)) != 0 {
        let mut strip = 0u32;
        let mut start = 0u32;
        while strip < e.mem.u32(render_data.wrapping_add(0x48)) {
            let lengths = e.mem.u32(render_data.wrapping_add(0x4c));
            let count = if lengths != 0 {
                u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
            } else {
                e.mem.u32(render_data.wrapping_add(0x40))
            };
            let vertices = e.mem.u32(render_data.wrapping_add(0x18));
            let device = e.get(this, NiDX9Renderer::device);
            let base = e.mem.u32(render_data.wrapping_add(0x38));
            let kind = e.mem.u32(render_data.wrapping_add(0x3c));
            e.vcall(
                device,
                DEVICE_DRAW_INDEXED_PRIMITIVE,
                &args![kind, base, 0u32, vertices, start, count],
            );
            strip += 1;
            start = start.wrapping_add(count).wrapping_add(2);
        }
    } else {
        let count = e.mem.u32(render_data.wrapping_add(0x40));
        let first = e.mem.u32(render_data.wrapping_add(0x38));
        let device = e.get(this, NiDX9Renderer::device);
        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
        e.vcall(device, DEVICE_DRAW_PRIMITIVE, &args![kind, first, count]);
    }
    let flags = e.mem.u16(data.wrapping_add(0xe));
    e.mem.set_u16(data.wrapping_add(0xe), flags & 0xf000);
}

/// The caps `GetDeviceCaps` fills (+0x28C), as the tests name them.
#[cfg(test)]
const DEVICE_CAPS: u32 = 0x28c;

/// `00e7cc50` (`cdecl`: the device, a render-target index): clears that render target, and
/// `00e7cca0` (`cdecl`: the device): clears the depth/stencil buffer (true on success).
const CLEAR_RENDER_TARGET_00E7CC50: u32 = 0x00e7_cc50;
const CLEAR_DEPTH_STENCIL_00E7CCA0: u32 = 0x00e7_cca0;
/// The classes `00e6eea0` checks: a render-target buffer (`0126f59c`), a depth/stencil
/// buffer (`0126f5ac`) and the default one's (`0126f5d8`).
const RENDER_BUFFER_RTTI_0126F59C: u32 = 0x0126_f59c;
/// `"NiDX9Renderer::BeginUsingRenderTargetGroup> Failed - %s"` and its three reasons (an
/// output target, the depth/stencil buffer, the depth/stencil buffer set to none).
const BEGIN_GROUP_FAILED_TEXT: u32 = 0x010e_dc58;
const OUTPUT_TARGET_FAILED_TEXT: u32 = 0x010e_dc18;
const DEPTH_STENCIL_FAILED_TEXT: u32 = 0x010e_dc90;
const DEPTH_STENCIL_NULL_FAILED_TEXT: u32 = 0x010e_dbc8;

/// Whether `object`'s RTTI chain (virtual +0xC, links at +4) has `rtti`: the object, or 0.
fn kind_of(e: &mut Engine, object: u32, rtti: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    fn_00e69b00(e, Ptr::new(object), rtti)
}

/// The failure path of `00e6eea0`: back to the default group (+0x884) through the
/// renderer's own virtual +0x194, then the message.
fn begin_group_failed(e: &mut Engine, this: Ptr<NiDX9Renderer>, clear: u32, reason: u32) -> bool {
    let default = e.get(this, NiDX9Renderer::object_884);
    e.vcall(this.addr(), 0x194, &args![default, clear]);
    e.call(REPORT_00A29680, &args![BEGIN_GROUP_FAILED_TEXT, reason]);
    false
}

// Translated from 00e6eea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x194, `Do_BeginUsingRenderTargetGroup`'s
/// slot on Xbox): `thiscall`, two words (the group, the buffers to clear). True at once with
/// the byte +0x6FC set. Clears render targets 1..=3 (`00e7cc50`), sets each of the group's
/// buffers (virtual +0xA4 count, +0xC8 buffer) of the render-buffer class as target (its
/// virtual +0x5C with the device and the index) and the depth/stencil buffer (+0xCC; its
/// virtual +0x60, or `00e7cca0` without one); a failure goes back to the default group and
/// answers false with its message. Then stores the group at +0x888 (and at +0x88C when its
/// first buffer is of the default class) and clears it (the renderer's virtual +0x188 with
/// no rectangle). Answers true.
pub fn fn_00e6eea0(e: &mut Engine, this: Ptr<NiDX9Renderer>, group: u32, clear: u32) -> bool {
    let t = this.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return true;
    }
    for target in 1..=3u32 {
        let device = e.get(this, NiDX9Renderer::device);
        e.call(CLEAR_RENDER_TARGET_00E7CC50, &args![device, target]);
    }
    let mut index = 0u32;
    if e.vcall(group, 0xa4, &args![]).u32() != 0 {
        loop {
            let buffer = e.vcall(group, 0xc8, &args![index]).u32();
            let target = kind_of(e, buffer, RENDER_BUFFER_RTTI_0126F59C);
            if target != 0 {
                let device = e.get(this, NiDX9Renderer::device);
                if e.vcall(target, 0x5c, &args![device, index]).u8() == 0 {
                    return begin_group_failed(e, this, clear, OUTPUT_TARGET_FAILED_TEXT);
                }
            }
            index += 1;
            if index >= e.vcall(group, 0xa4, &args![]).u32() {
                break;
            }
        }
    }
    let depth = e.vcall(group, 0xcc, &args![]).u32();
    let depth_buffer = kind_of(e, depth, DEPTH_BUFFER_RTTI_0126F5AC);
    if depth_buffer != 0 {
        let device = e.get(this, NiDX9Renderer::device);
        if e.vcall(depth_buffer, 0x60, &args![device]).u8() == 0 {
            return begin_group_failed(e, this, clear, DEPTH_STENCIL_FAILED_TEXT);
        }
    } else {
        let device = e.get(this, NiDX9Renderer::device);
        if e.call(CLEAR_DEPTH_STENCIL_00E7CCA0, &args![device]).u8() == 0 {
            return begin_group_failed(e, this, clear, DEPTH_STENCIL_NULL_FAILED_TEXT);
        }
    }
    let first = e.vcall(group, 0xc8, &args![0u32]).u32();
    let default = kind_of(e, first, SHADER_RTTI_0126F5D8);
    e.set(this, NiDX9Renderer::target_group, group);
    if default != 0 {
        e.mem.set_u32(t + 0x88c, group);
    }
    e.vcall(t, 0x188, &args![0u32, clear]);
    true
}

/// `float` 1.0 bits (the code stores `FLD1`).
#[cfg(test)]
const ONE: u32 = 0x3f80_0000;

// Translated from 00e745a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (renderer vtable +0x1B4): draws a packed geometry with its
/// partial-draw records (`thiscall`, one word). As `00e74840`, with two differences: a
/// skinned partition without strip lengths draws the count at +0xC4 of the geometry's
/// virtual +0x2C object when there is one; an unskinned indexed strip with a draw object
/// (the geometry's virtual +0x28) draws its active records instead (0x14 bytes each from
/// +0xC4, active when byte +0x10 is set: start +0, count +0xC; one record with the start 0
/// and the count of the object's data word +0x40 when its byte +0xCD is set).
pub fn fn_00e745a0(e: &mut Engine, this: Ptr<NiDX9Renderer>, geometry: Ptr) {
    let t = this.addr();
    let g = geometry.addr();
    if e.mem.u8(t + SKIP_END_SCENE) != 0 {
        return;
    }
    let data = e.mem.u32(g.wrapping_add(0xb8));
    if e.vcall(data, 0x90, &args![]).u16() == 0 {
        return;
    }
    let skin = e.mem.u32(g.wrapping_add(0xbc));
    let skinned = fn_00e72c20(e, this, geometry, 0);
    let shader = e.mem.u32(g.wrapping_add(0xc0));
    let render_data = e
        .mem
        .u32(e.mem.u32(g.wrapping_add(0xb8)).wrapping_add(0x34));
    e.vcall(shader, 0x90, &args![]);
    if skinned {
        let instance = e.mem.u32(skin.wrapping_add(0xc));
        let mut remaining = e.mem.u32(instance.wrapping_add(8));
        let partitions = e.mem.u32(instance.wrapping_add(0xc));
        let kind = u32::from(e.mem.u16(partitions.wrapping_add(0x22)) != 0) + 4;
        if remaining != 0 {
            let mut at = partitions.wrapping_add(0x1c);
            loop {
                let part_data = e.mem.u32(at.wrapping_add(0xc));
                let mut strip = 0u32;
                let mut start = 0u32;
                while strip < e.mem.u32(part_data.wrapping_add(0x48)) {
                    let lengths = e.mem.u32(part_data.wrapping_add(0x4c));
                    let count = if lengths != 0 {
                        u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
                    } else {
                        let mut count = e.mem.u32(part_data.wrapping_add(0x40));
                        if e.vcall(g, 0x2c, &args![]).u32() != 0 {
                            let partial = e.vcall(g, 0x2c, &args![]).u32();
                            count = e.mem.u32(partial.wrapping_add(0xc4));
                        }
                        count
                    };
                    let vertices = u32::from(e.mem.u16(at));
                    let device = e.get(this, NiDX9Renderer::device);
                    let base = e.mem.u32(part_data.wrapping_add(0x38));
                    e.vcall(
                        device,
                        DEVICE_DRAW_INDEXED_PRIMITIVE,
                        &args![kind, base, 0u32, vertices, start, count],
                    );
                    strip += 1;
                    start = start.wrapping_add(count).wrapping_add(2);
                }
                at = at.wrapping_add(0x2c);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
    } else if e.mem.u32(render_data.wrapping_add(0x34)) != 0 {
        let mut start = 0u32;
        let mut strip = 0u32;
        while strip < e.mem.u32(render_data.wrapping_add(0x48)) {
            let lengths = e.mem.u32(render_data.wrapping_add(0x4c));
            let count = if lengths != 0 {
                u32::from(e.mem.u16(lengths.wrapping_add(strip * 2))).wrapping_sub(2)
            } else {
                e.mem.u32(render_data.wrapping_add(0x40))
            };
            if e.vcall(g, 0x28, &args![]).u32() == 0 {
                let vertices = e.mem.u32(render_data.wrapping_add(0x18));
                let device = e.get(this, NiDX9Renderer::device);
                let base = e.mem.u32(render_data.wrapping_add(0x38));
                let kind = e.mem.u32(render_data.wrapping_add(0x3c));
                e.vcall(
                    device,
                    DEVICE_DRAW_INDEXED_PRIMITIVE,
                    &args![kind, base, 0u32, vertices, start, count],
                );
            } else {
                let draw = e.vcall(g, 0x28, &args![]).u32();
                let whole = e.mem.u8(draw.wrapping_add(0xcd)) != 0;
                let records = if whole {
                    1
                } else {
                    e.mem.u32(draw.wrapping_add(0xc8))
                };
                let mut offset = 0u32;
                let mut left = records;
                while left != 0 {
                    let array = e.mem.u32(draw.wrapping_add(0xc4));
                    let record = array.wrapping_add(offset);
                    if e.mem.u8(record.wrapping_add(0x10)) != 0 {
                        let whole = e.mem.u8(draw.wrapping_add(0xcd)) != 0;
                        let part_count = if whole {
                            let draw_data = e.mem.u32(draw.wrapping_add(0xb8));
                            u32::from(e.mem.u16(draw_data.wrapping_add(0x40)))
                        } else {
                            e.mem.u32(record.wrapping_add(0xc))
                        };
                        let part_start = if whole { 0 } else { e.mem.u32(record) };
                        let device = e.get(this, NiDX9Renderer::device);
                        let vertices = e.mem.u32(render_data.wrapping_add(0x18));
                        let base = e.mem.u32(render_data.wrapping_add(0x38));
                        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
                        e.vcall(
                            device,
                            DEVICE_DRAW_INDEXED_PRIMITIVE,
                            &args![kind, base, 0u32, vertices, part_start, part_count],
                        );
                    }
                    offset = offset.wrapping_add(0x14);
                    left -= 1;
                }
            }
            start = start.wrapping_add(count).wrapping_add(2);
            strip += 1;
        }
    } else {
        let count = e.mem.u32(render_data.wrapping_add(0x40));
        let first = e.mem.u32(render_data.wrapping_add(0x38));
        let device = e.get(this, NiDX9Renderer::device);
        let kind = e.mem.u32(render_data.wrapping_add(0x3c));
        e.vcall(device, DEVICE_DRAW_PRIMITIVE, &args![kind, first, count]);
    }
    let flags = e.mem.u16(data.wrapping_add(0xe));
    e.mem.set_u16(data.wrapping_add(0xe), flags & 0xf000);
}

#[cfg(test)]
mod b048_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A vtable at `table` whose slot `n` calls the double at `base + 4 * n`.
    fn vtable(e: &mut Engine, table: u32, base: u32) {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
    }

    /// An object whose vtable slot at byte `slot` is the double `base + slot`.
    fn object(e: &mut Engine, table: u32, base: u32) -> u32 {
        vtable(e, table, base);
        let o = e.mem.alloc(0x40);
        e.mem.set_u32(o, table);
        o
    }

    fn renderer(e: &mut Engine) -> Ptr<NiDX9Renderer> {
        e.new_object::<NiDX9Renderer>()
    }

    #[test]
    fn pixel_formats_compare_their_fields_and_channels() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x44);
        let b = e.mem.alloc(0x44);
        let ne = |e: &mut Engine| e.call(0x00a7_65c0, &args![a, b]).bool();
        assert!(!ne(&mut e));
        e.mem.set_u32(b + 0xc, 0xffff_ffff);
        e.mem.set_u32(a + 0xc, 5);
        assert!(!ne(&mut e));
        e.mem.set_u32(b + 0xc, 6);
        assert!(ne(&mut e));
        e.mem.set_u32(b + 0xc, 5);
        e.mem.set_u8(b + 0x1d + 0xc * 3, 1);
        assert!(ne(&mut e));
        e.mem.set_u8(b + 0x1d + 0xc * 3, 0);
        e.mem.set_u8(b + 1, 2);
        assert!(ne(&mut e));
    }

    #[test]
    fn small_destructors_and_forwarders() {
        let mut e = Engine::new();
        e.register(PROPERTY_DESTRUCT, nop);
        e.register(SIZED_DELETE, nop);
        e.register(RENDERER_DESTRUCT, nop);
        e.register(D3D_FROM_NI_00B71A40, |_, _| 7u32.into_ret());
        e.register(MEMBER_CALL_0043A010, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_50f0, &args![0x5000u32, 1u32]).u32(), 0x5000);
        e.call(0x00a9_50f0, &args![0x5000u32, 0u32]);
        assert_eq!(e.call(0x00e7_62f0, &args![0x6000u32, 1u32]).u32(), 0x6000);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![vec![0x5000, 0x1c], vec![0x6000, 0xb80]]
        );
        assert_eq!(calls(&e, PROPERTY_DESTRUCT).len(), 2);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x30, 1.5f32.to_bits());
        assert_eq!(e.call(0x00b7_99a0, &args![0x77u32, source]).u32(), 7);
        assert_eq!(
            calls(&e, D3D_FROM_NI_00B71A40),
            vec![vec![0x77, source, source + 0x24, 1.5f32.to_bits()]]
        );
        assert_eq!(e.call(0x00e7_2e50, &args![0x100u32]).u32(), 0x10c);
    }

    fn d3d9_rig(e: &mut Engine) {
        e.map(D3D9_MODULE, 0x30);
        e.map(DIRECT3D_IS_EX, 4);
        e.register(LOAD_LIBRARY_A, |_, _| 0x7700u32.into_ret());
        e.register(GET_PROC_ADDRESS, |_, a| {
            if a[1] == DIRECT3D_CREATE_9_TEXT {
                0x6200_0000u32
            } else {
                0x6200_0004
            }
            .into_ret()
        });
        e.register(REPORT_00A29680, nop);
    }

    #[test]
    fn direct3d_entry_points_load_once() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9410, &args![]).bool());
        assert_eq!(e.global::<u32>(DIRECT3D_CREATE9), 0x6200_0000);
        assert_eq!(e.global::<u32>(DIRECT3D_CREATE9_EX), 0x6200_0004);
        assert!(e.call(0x00e6_9410, &args![]).bool());
        assert_eq!(calls(&e, LOAD_LIBRARY_A), vec![vec![D3D9_DLL_TEXT]]);
        e.set_global(DIRECT3D_CREATE9, 0u32);
        e.register(LOAD_LIBRARY_A, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_9410, &args![]).bool());
    }

    #[test]
    fn create_direct3d9_tries_the_ex_entry_first() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        // Ex fails (leaves the out word 0), the plain creation works: the answer stays -1.
        e.register(0x6200_0004, nop);
        e.register(0x6200_0000, |_, _| 0xd3d0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), -1);
        assert_eq!(e.global::<u32>(DIRECT3D), 0xd3d0);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![
                vec![CREATE_EX_FAILED_010ED598],
                vec![CREATE_SUCCEEDED_010ED52C]
            ]
        );
        // Made already: 0 at once.
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), 0);
        // Ex works.
        e.set_global(DIRECT3D, 0u32);
        e.register(0x6200_0004, |e, a| {
            e.mem.set_u32(a[1], 0xe0e0);
            Ret::default()
        });
        assert_eq!(e.call(0x00e6_9460, &args![true]).i32(), 0);
        assert_eq!(e.global::<u32>(DIRECT3D), 0xe0e0);
        assert_eq!(e.global::<u8>(DIRECT3D_IS_EX), 1);
        // Neither: -1 with the failure message.
        e.set_global(DIRECT3D, 0u32);
        e.register(0x6200_0000, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00e6_9460, &args![false]).i32(), -1);
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![CREATE_FAILED_010ED550]
        );
    }

    #[test]
    fn mode_table_sets_the_count_and_flags() {
        let mut e = Engine::new();
        let count = e.mem.alloc(4);
        let flags = e.mem.alloc(4);
        let cases = [
            (0u32, 1u32, 0x50u32),
            (1, 1, 0x40),
            (2, 1, 0x80),
            (3, 1, 0x20),
            (4, 2, 0x20),
            (5, 2, 0x40),
            (6, 2, 0x80),
        ];
        for (mode, value, set) in cases {
            e.mem.set_u32(flags, 0xffff_ffff);
            assert!(e.call(0x00e6_9510, &args![mode, count, flags]).bool());
            assert_eq!(e.mem.u32(count), value);
            assert_eq!(e.mem.u32(flags), 0xffff_ff0f | set);
        }
        e.mem.set_u32(flags, 0xff);
        assert!(!e.call(0x00e6_9510, &args![7u32, count, flags]).bool());
        assert_eq!(e.mem.u32(flags), 0x0f);
    }

    #[test]
    fn getters_and_object_forwarders() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::word_8a0, 1);
        e.set(r, NiDX9Renderer::byte_8a4, 2);
        e.set(r, NiDX9Renderer::word_5ec, 3);
        e.set(r, NiDX9Renderer::target_group, 4);
        let o884 = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_884, o884);
        let o8c4 = object(&mut e, 0x6400, 0x6140_0000);
        e.set(r, NiDX9Renderer::object_8c4, o8c4);
        let o8b8 = object(&mut e, 0x6800, 0x6180_0000);
        e.set(r, NiDX9Renderer::object_8b8, o8b8);
        e.register(0x6100_00b0, |_, a| (a[1] + 9).into_ret());
        e.register(0x6140_0010, |_, a| (a[1] + a[2] + a[3]).into_ret());
        e.register(0x6180_0054, |_, a| a[1].into_ret());
        assert_eq!(e.call(0x00e6_95e0, &args![r]).u32(), 1);
        assert_eq!(e.call(0x00e6_95f0, &args![r]).u32(), 2);
        assert_eq!(e.call(0x00e7_57d0, &args![r]).u32(), 3);
        assert_eq!(e.call(0x00e7_5810, &args![r]).u32(), 4);
        assert_eq!(e.call(0x00e6_9600, &args![r]).u32(), o884);
        assert_eq!(e.call(0x00e6_9620, &args![r]).u32(), 9);
        assert_eq!(e.call(0x00e6_9640, &args![r, 1u32, 2u32, 3u32]).u32(), 6);
        assert_eq!(e.call(0x00e6_9960, &args![r, 5u32]).u32(), 5);
    }

    #[test]
    fn end_frame_ends_the_scene_unless_skipped() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let device = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::device, device);
        e.set(r, NiDX9Renderer::object_8bc, 0x8bc0);
        e.register(0x6100_0000 + DEVICE_END_SCENE, |_, _| 0u32.into_ret());
        e.register(OBJECT_8BC_CALL_00E8BF80, nop);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_9660, &args![r]).bool());
        assert_eq!(calls(&e, OBJECT_8BC_CALL_00E8BF80), vec![vec![0x8bc0]]);
        e.register(0x6100_0000 + DEVICE_END_SCENE, |_, _| {
            0x8000_4005u32.into_ret()
        });
        assert!(!e.call(0x00e6_9660, &args![r]).bool());
        e.mem.set_u8(r.addr() + SKIP_END_SCENE, 1);
        assert!(e.call(0x00e6_9660, &args![r]).bool());
        assert_eq!(calls(&e, 0x6100_0000 + DEVICE_END_SCENE).len(), 2);
        assert_eq!(calls(&e, OBJECT_8BC_CALL_00E8BF80).len(), 1);
    }

    #[test]
    fn vertex_declarations_are_made_once_from_the_stack_elements() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let device = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::device, device);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        let base = r.addr();
        e.register_double(
            0x6100_0000 + DEVICE_CREATE_VERTEX_DECLARATION,
            move |e, a| {
                let len = if a[2] == base + 0x284 { 0x30 } else { 0x28 };
                log.borrow_mut().push(e.mem.bytes(a[1], len));
                e.mem.set_u32(a[2], 0xdec1);
                Ret::default()
            },
        );
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0xdec1);
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0xdec1);
        assert_eq!(e.call(0x00e6_9790, &args![r]).u32(), 0xdec1);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(
            &seen[0][..0x28],
            &[
                0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 12, 0, 2, 0, 3, 0, 0, 0, 24, 0, 4, 0, 10, 0, 0, 0,
                28, 0, 1, 0, 5, 0, 0xff, 0, 0, 0, 0x11, 0, 0, 0
            ]
        );
        assert_eq!(
            &seen[1][0x20..0x30],
            &[0, 0, 36, 0, 0, 0, 5, 1, 0xff, 0, 0, 0, 0x11, 0, 0, 0]
        );
        // A failed creation clears the slot.
        e.mem.set_u32(r.addr() + 0x280, 0);
        e.register(0x6100_0000 + DEVICE_CREATE_VERTEX_DECLARATION, |e, a| {
            e.mem.set_u32(a[2], 0x55);
            0x8000_0000u32.into_ret()
        });
        assert_eq!(e.call(0x00e6_96a0, &args![r]).u32(), 0);
        assert_eq!(e.mem.u32(r.addr() + 0x280), 0);
    }

    #[test]
    fn purges_release_the_shaders_direct3d_objects() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let cache = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_8b8, cache);
        let shader = object(&mut e, 0x6400, 0x6140_0000);
        let d3d = object(&mut e, 0x6800, 0x6180_0000);
        for a in [
            0x6100_008c,
            0x6100_007c,
            0x6180_0008,
            0x6140_0088,
            0x6140_0080,
        ] {
            e.register(a, |_, _| 0x33u32.into_ret());
        }
        e.register_double(0x6140_0084, move |_, _| d3d.into_ret());
        e.register(0x6140_007c, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e6_9970, &args![r, shader]).u32(), 0x33);
        assert_eq!(calls(&e, 0x6100_008c), vec![vec![cache, 0, 0]]);
        assert_eq!(calls(&e, 0x6180_0008), vec![vec![d3d]]);
        assert_eq!(calls(&e, 0x6140_0088), vec![vec![shader, 0]]);
        e.call(0x00e6_99c0, &args![r, shader]);
        assert_eq!(calls(&e, 0x6100_007c), vec![vec![cache, 0, 0]]);
        assert_eq!(calls(&e, 0x6180_0008).len(), 1);
        assert_eq!(calls(&e, 0x6140_0080), vec![vec![shader, 0]]);
    }

    #[test]
    fn fn_00e69a20_passes_the_address_of_its_first_word() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::device, 0xd3d);
        e.register(DEVICE_CALL_00E7D9D0, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e6_9a20, &args![r, 0u32, 2u32, 3u32]).bool());
        assert!(e.call(0x00e6_9a20, &args![r, 9u32, 2u32, 3u32]).bool());
        let c = calls(&e, DEVICE_CALL_00E7D9D0);
        assert_eq!(c.len(), 1);
        assert_eq!((c[0][0], c[0][2], c[0][3]), (0xd3d, 2, 3));
    }

    #[test]
    fn adapter_info_is_made_once_with_a_temporary_direct3d() {
        let mut e = Engine::new();
        d3d9_rig(&mut e);
        e.map(ADAPTER_LIST, 4);
        let d3d = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0008, nop);
        e.register_double(0x6200_0000, move |_, _| d3d.into_ret());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ADAPTER_INFO_FILL, nop);
        e.call_log = Some(vec![]);
        let info = e.call(0x00e6_9a60, &args![]).u32();
        assert_ne!(info, 0);
        assert_eq!(e.mem.u32(info + 4), ADAPTER_INFO_VTABLE);
        assert_eq!(e.mem.u16(info + 0x12), 1);
        assert_eq!(
            calls(&e, ADAPTER_INFO_FILL),
            vec![vec![info, d3d, ADAPTER_INFO_ARGUMENT]]
        );
        assert_eq!(calls(&e, 0x6100_0008), vec![vec![d3d]]);
        assert_eq!(e.call(0x00e6_9a60, &args![]).u32(), info);
        assert_eq!(calls(&e, NEW_OBJECT).len(), 1);
    }

    #[test]
    fn rtti_chain_and_indexed_element() {
        let mut e = Engine::new();
        let o = object(&mut e, 0x6000, 0x6100_0000);
        let link2 = e.mem.alloc(8);
        let link1 = e.mem.alloc(8);
        e.mem.set_u32(link1 + 4, link2);
        e.register_double(0x6100_000c, move |_, _| link1.into_ret());
        assert_eq!(e.call(0x00e6_9b00, &args![o, link2]).u32(), o);
        assert_eq!(e.call(0x00e6_9b00, &args![o, 0x1234u32]).u32(), 0);
        let a = e.mem.alloc(0x30);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements + 4, 0x99);
        e.mem.set_u32(a + 0x28, elements);
        e.mem.set_u32(a + 0x20, 2);
        assert_eq!(e.call(0x00e6_9b40, &args![a, 1u32]).u32(), 0x99);
        assert_eq!(e.call(0x00e6_9b40, &args![a, 2u32]).u32(), 0);
    }

    #[test]
    fn format_tables() {
        let mut e = Engine::new();
        let c90: Vec<u32> = (0..13)
            .map(|i| e.call(0x00e6_9c90, &args![i as u32]).u32())
            .collect();
        assert_eq!(
            c90,
            vec![0, 0x14, 0x16, 0x16, 0x17, 0x18, 0x18, 0x1e, 0x1b, 0, 0, 0x1e, 0]
        );
        let d00 = |e: &mut Engine, f: u32| e.call(0x00e6_9d00, &args![f]).u32();
        assert_eq!(d00(&mut e, 20), 1);
        assert_eq!(d00(&mut e, 30), 11);
        assert_eq!(d00(&mut e, 31), 0);
        assert_eq!(d00(&mut e, 36), 14);
        assert_eq!(d00(&mut e, 111), 12);
        assert_eq!(d00(&mut e, 112), 13);
        assert_eq!(d00(&mut e, 113), 15);
        assert_eq!(d00(&mut e, 116), 18);
        assert_eq!(d00(&mut e, 117), 0);
        let e50: Vec<u32> = (0..6)
            .map(|i| e.call(0x00e6_9e50, &args![i as u32]).u32())
            .collect();
        assert_eq!(e50, vec![0x8000_0000, 1, 2, 4, 8, 0]);
        let s = |e: &mut Engine, f: u32| e.call(0x00e7_a9a0, &args![f]).u32();
        assert_eq!(s(&mut e, 21), 0x010e_ec28);
        assert_eq!(s(&mut e, 81), 0x010e_e9f4);
        assert_eq!(s(&mut e, 0x3154_454d), 0x010e_e920);
        assert_eq!(s(&mut e, 0x3154_5844), 0x010e_e908);
        assert_eq!(s(&mut e, 0x5956_5955), 0x010e_e8a4);
        assert_eq!(s(&mut e, 37), UNKNOWN_FORMAT_01015890);
        assert_eq!(s(&mut e, 0xffff_ffff), UNKNOWN_FORMAT_01015890);
    }

    #[test]
    fn texture_renderer_data() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let texture = e.mem.alloc(0x30);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            FREE,
        ] {
            e.register(a, nop);
        }
        e.register(MAP_REMOVE_AT, |_, _| 1u32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E68EF0, |_, _| 0xda7au32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E8ABF0, |_, _| 0u32.into_ret());
        e.register(MAKE_TEXTURE_DATA_00E8FF60, |_, a| a[2].into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_dc40, &args![r, texture]).bool());
        assert!(!e.call(0x00e6_dcc0, &args![r, texture]).bool());
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
        assert!(!e.call(0x00e6_dc80, &args![r, texture, 0u32]).bool());
        assert!(e.call(0x00e6_dc80, &args![r, texture, 5u32]).bool());
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![r.addr() + 0x8cc, texture, 5]]
        );
        e.mem.set_u32(texture + 0x24, 0x1);
        assert!(e.call(0x00e6_dcc0, &args![r, texture]).bool());
        // That path leaves the lock taken.
        assert_eq!(calls(&e, LEAVE_CRITICAL_SECTION).len(), 2);
        assert!(e.call(0x00e6_dc80, &args![r, texture, 5u32]).bool());
        assert_eq!(e.call(0x00e6_df80, &args![r, texture]).u32(), 1);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
        assert_eq!(
            calls(&e, MAP_REMOVE_AT),
            vec![vec![r.addr() + 0x8cc, texture]]
        );
        e.mem.set_u32(texture + 0x28, 0xb10c);
        e.mem.set_u32(texture + 0x24, 0xb10c);
        e.call(0x00e6_d740, &args![r, texture]);
        assert_eq!(calls(&e, FREE), vec![vec![0xb10c]]);
        assert_eq!(e.mem.u32(texture + 0x28), 0);
        assert_eq!(e.mem.u32(texture + 0x24), 0);
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn pointer_member_assignment() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        let this = e.mem.alloc(0x20);
        let first = object(&mut e, 0x6000, 0x6100_0000);
        let second = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0004, nop);
        e.call_log = Some(vec![]);
        e.call(0x00e6_e6e0, &args![this, first]);
        e.call(0x00e6_e6e0, &args![this, first]);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.call(0x00e6_e6e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0x14), first);
        e.call(0x00e6_e6e0, &args![this, second]);
        assert_eq!(e.mem.u32(this + 0x14), second);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![first]]);
    }

    #[test]
    fn shaders_join_and_leave_the_list() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let shader = object(&mut e, 0x6000, 0x6100_0000);
        e.register(SHADER_SET_D3D_RENDERER, nop);
        e.register(0x6100_00cc, nop);
        e.register(SHADER_LIST_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e7_5310, &args![r, shader]);
        e.call(0x00e7_5340, &args![r, shader]);
        assert_eq!(
            calls(&e, SHADER_SET_D3D_RENDERER),
            vec![vec![shader, 0], vec![shader, 0]]
        );
        assert_eq!(calls(&e, 0x6100_00cc), vec![vec![shader]]);
        let adds = calls(&e, SHADER_LIST_ADD);
        assert_eq!(adds.len(), 2);
        assert!(adds.iter().all(|a| a[0] == r.addr() + 0x90c));
    }

    #[test]
    fn purge_all_shaders_empties_the_list_into_the_pool() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.map(NODE_POOL_FREE, 0x30);
        e.register(SHADER_SET_D3D_RENDERER, nop);
        e.register(NODE_POOL_LOCK_TAKE, |e, _| {
            let c = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global(NODE_POOL_LOCK_COUNT, c);
            Ret::default()
        });
        let first = object(&mut e, 0x6000, 0x6100_0000);
        let second = object(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_00cc, nop);
        let n2 = e.mem.alloc(12);
        let n1 = e.mem.alloc(12);
        e.mem.set_u32(n1, n2);
        e.mem.set_u32(n1 + 8, first);
        e.mem.set_u32(n2 + 4, n1);
        e.mem.set_u32(n2 + 8, second);
        let list = r.addr() + 0x90c;
        e.mem.set_u32(list, n1);
        e.mem.set_u32(list + 4, n2);
        e.mem.set_u32(list + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e7_5380, &args![r]);
        assert_eq!(e.mem.u32(list), 0);
        assert_eq!(e.mem.u32(list + 4), 0);
        assert_eq!(e.mem.u32(list + 8), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE), n2);
        assert_eq!(e.mem.u32(n2), n1);
        assert_eq!(e.global::<u32>(NODE_POOL_LOCK_COUNT), 0);
        assert_eq!(
            calls(&e, SHADER_SET_D3D_RENDERER),
            vec![vec![first, 0], vec![second, 0]]
        );
    }

    #[test]
    fn background_colours_pack_and_unpack() {
        let mut e = Engine::new();
        e.map(SCALE_TO_BYTE, 8);
        e.map(SCALE_FROM_BYTE, 8);
        e.mem.set_u64(SCALE_TO_BYTE, 255.0f64.to_bits());
        e.mem
            .set_u64(SCALE_FROM_BYTE, f64::from(1.0f32 / 255.0).to_bits());
        let r = renderer(&mut e);
        let color = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 0.5, 0.25, 0.75].iter().enumerate() {
            e.mem.set_u32(color + 4 * i as u32, v.to_bits());
        }
        e.call(0x00e7_5840, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xffff_7f3f);
        e.call(0x00e7_58f0, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xbfff_7f3f);
        let out = e.mem.alloc(0x10);
        e.call(0x00e7_59c0, &args![r, out]);
        let k = f64::from(1.0f32 / 255.0);
        assert_eq!(e.mem.u32(out + 0xc), ((191.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out), ((255.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out + 4), ((127.0 * k) as f32).to_bits());
        assert_eq!(e.mem.u32(out + 8), ((63.0 * k) as f32).to_bits());
        // A channel out of range keeps the low byte of the integer indefinite (0).
        e.mem.set_u32(color, f32::NAN.to_bits());
        e.call(0x00e7_5840, &args![r, color]);
        assert_eq!(e.get(r, NiDX9Renderer::background_color), 0xff00_7f3f);
    }

    #[test]
    fn object_lookup_forwards_to_the_object_at_0x8c8() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        e.set(r, NiDX9Renderer::object_8c8, 0x8c80);
        e.register(OBJECT_LOOKUP_00653290, |_, a| {
            if a[0] == 1 { 0 } else { 0x77 }.into_ret()
        });
        e.register(OBJECT_8C8_CALL_00E8BD40, nop);
        e.call_log = Some(vec![]);
        e.call(0x00e7_1cc0, &args![r, 0u32]);
        e.call(0x00e7_1cc0, &args![r, 1u32]);
        e.call(0x00e7_1cc0, &args![r, 2u32]);
        assert_eq!(
            calls(&e, OBJECT_LOOKUP_00653290),
            vec![
                vec![1, OBJECT_LOOKUP_ARGUMENT],
                vec![2, OBJECT_LOOKUP_ARGUMENT]
            ]
        );
        assert_eq!(
            calls(&e, OBJECT_8C8_CALL_00E8BD40),
            vec![vec![0x8c80, 0x77]]
        );
    }

    #[test]
    fn keyed_values_are_made_once() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let source = object(&mut e, 0x6000, 0x6100_0000);
        e.set(r, NiDX9Renderer::object_884, source);
        let shader = object(&mut e, 0x6400, 0x6140_0000);
        e.mem.set_u32(shader + 0x28, 0x28);
        let rtti = e.mem.alloc(8);
        e.register_double(0x6100_00c8, move |_, _| shader.into_ret());
        e.register_double(0x6140_000c, move |_, _| rtti.into_ret());
        e.mem.set_u32(rtti + 4, SHADER_RTTI_0126F5D8);
        e.mem.set_u32(r.addr() + 0x5dc, 0x5dc0);
        e.register(KEY_PART_00E7BD20, |_, a| (a[0] + 1).into_ret());
        e.register(DEPTH_FORMAT_00E7B0F0, |_, _| 0x4e40u32.into_ret());
        let stored = Rc::new(RefCell::new(0u32));
        let get = stored.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], *get.borrow());
            u32::from(*get.borrow() != 0).into_ret()
        });
        let set = stored.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            *set.borrow_mut() = a[2];
            Ret::default()
        });
        e.register(MAKE_VALUE_00E7C500, |_, _| 0x7a1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e6_ee00, &args![r, 1u32, 2u32, 3u32]).u32(),
            0x7a1
        );
        assert_eq!(
            e.call(0x00e6_ee00, &args![r, 1u32, 2u32, 3u32]).u32(),
            0x7a1
        );
        assert_eq!(calls(&e, MAKE_VALUE_00E7C500), vec![vec![0x4e40]]);
        assert_eq!(
            calls(&e, DEPTH_FORMAT_00E7B0F0)[0],
            vec![0x5dc0, 0x28, 2, 2, 3]
        );
        assert_eq!(
            calls(&e, MAP_SET_AT),
            vec![vec![r.addr() + 0xb14, 0x4e40, 0x7a1]]
        );
    }

    #[test]
    fn geometry_check_branches() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        vtable(&mut e, 0x6000, 0x6100_0000);
        e.mem.set_u32(r.addr(), 0x6000);
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        let geometry = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(geometry + 0xb8, data);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        let third = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xbc, first);
        e.mem.set_u32(first + 0xc, second);
        e.mem.set_u32(second + 0xc, third);
        let check = |e: &mut Engine, o: u32| e.call(0x00e7_2c20, &args![r, geometry, o]).bool();
        assert!(!check(&mut e, 0));
        e.mem.set_u32(third + 0x28, 1);
        assert!(check(&mut e, 0));
        e.mem.set_u32(third + 0x28, 0);
        e.register(0x6100_00a0, |_, _| 2u32.into_ret());
        assert!(check(&mut e, 0));
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        let target = object(&mut e, 0x6400, 0x6140_0000);
        e.register(0x6140_00d4, |_, _| 1u32.into_ret());
        assert!(!check(&mut e, target));
        e.mem.set_u32(r.addr() + 0x6f0, 0xfffe_0001);
        assert!(check(&mut e, target));
        e.mem.set_u32(r.addr() + 0x6f0, 0);
        e.mem.set_u8(r.addr() + 0x5d5, 1);
        let inner = object(&mut e, 0x6800, 0x6180_0000);
        e.register_double(0x6140_00bc, move |_, _| inner.into_ret());
        e.register(0x6180_00a8, |_, _| 1u32.into_ret());
        // The geometry's own object at +0xC0 stands in for a 0 argument.
        e.mem.set_u32(geometry + 0xc0, target);
        assert!(check(&mut e, 0));
        e.mem.set_u32(data + 0x34, 1);
        assert!(!check(&mut e, target));
    }

    #[test]
    fn default_textures_pick_formats_and_fill_their_pixels() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        ref_counting(&mut e);
        // Group 0: index 0 and 2 set (index 0 wins over 2; 1 is empty). Group 2: index 5.
        let (f15, f16, f17) = (e.mem.alloc(0x20), e.mem.alloc(0x20), e.mem.alloc(0x20));
        e.mem.set_u32(f15 + 0x10, 0x115);
        e.mem.set_u32(t + 0x700, f15);
        e.mem.set_u32(t + 0x708, f16);
        e.mem.set_u32(t + 0x700 + 4 * (5 + 0x2c), f17);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        vtable(&mut e, 0x6000, 0x6100_0000);
        e.register(0x6100_0004, nop);
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            e.mem.set_u32(a[0], 0x6000);
            let pixels = e.mem.alloc(0x40);
            let buffers = e.mem.alloc(4);
            e.mem.set_u32(buffers, pixels);
            e.mem.set_u32(a[0] + 0x5c, buffers);
            e.mem.set_u32(a[0] + 0x50, 8);
            e.mem.set_u32(a[0] + 0x64, 2);
            a[0].into_ret()
        });
        e.register(MEMSET_ADDR, nop);
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, nop);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_c550, &args![r]).bool());
        assert_eq!(e.mem.u32(t + 0x860), f15);
        assert_eq!(e.mem.u32(t + 0x864), 0);
        assert_eq!(e.mem.u32(t + 0x868), f17);
        assert_eq!(e.mem.u32(t + 0x880), 0x115);
        let constructs = calls(&e, PIXEL_DATA_CONSTRUCT_00A7C190);
        assert_eq!(constructs.len(), 2);
        assert_eq!(constructs[0][1..], [4, 4, f15, 1, 1]);
        let memsets = calls(&e, MEMSET_ADDR);
        assert_eq!((memsets[0][1], memsets[0][2]), (0xff, 0x20));
        assert_eq!(calls(&e, PIXEL_FORMAT_SET_UP_00A622D0), vec![vec![f15]]);
        // Nothing in the first group: false.
        e.mem.set_u32(t + 0x700, 0);
        e.mem.set_u32(t + 0x708, 0);
        assert!(!e.call(0x00e6_c550, &args![r]).bool());
    }

    #[test]
    fn texture_creation_by_flags_and_class() {
        let mut e = Engine::new();
        for a in [
            TEXTURE_SET_00A670C0,
            OBJECT_CALL_00A84CA0,
            TEXTURE_CALL_00A7B6C0,
            TEXTURE_CALL_00A760C0,
        ] {
            e.register(a, nop);
        }
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TEXTURE_RTTI_011F4A70).into_ret()
        });
        let texture = e.mem.alloc(0x50);
        e.mem.set_u16(texture + 0xe, 0x4000);
        e.mem.set_u16(texture + 8, 0x20);
        e.mem.set_u16(texture + 0xc, 0x3001);
        e.mem.set_u32(texture + 0x20, 0xa);
        e.mem.set_u32(texture + 0x2c, 0xd);
        e.mem.set_u16(texture + 0x44, 0x44);
        e.mem.set_u32(texture + 0x48, 0x48);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0x0b8);
        e.call_log = Some(vec![]);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(
            calls(&e, TEXTURE_SET_00A670C0),
            vec![vec![texture, 0x20, 0, 0, 0, 0, 0, 0x3000]]
        );
        assert_eq!(
            calls(&e, TEXTURE_CALL_00A760C0),
            vec![vec![texture, 0x44, 0x48, 0]]
        );
        e.mem.set_u8(texture + 0x38, 0x9);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 1u32]);
        assert_eq!(
            calls(&e, TEXTURE_SET_00A670C0)[1],
            vec![texture, 0x20, 0xa, 0, 0, 0xd, 1, 0x3000]
        );
        assert_eq!(calls(&e, OBJECT_CALL_00A84CA0), vec![vec![0x0b8]]);
        e.mem.set_u8(texture + 0x38, 0x30);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 1u32]);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(calls(&e, OBJECT_CALL_00A84CA0).len(), 1);
        assert_eq!(calls(&e, TEXTURE_CALL_00A760C0).len(), 1);
        // Not a 0x4000 texture: nothing.
        e.mem.set_u16(texture + 0xe, 0x3000);
        e.call(0x00e6_fa90, &args![0u32, texture, object, 0u32]);
        assert_eq!(calls(&e, TEXTURE_SET_00A670C0).len(), 4);
    }

    #[test]
    fn geometry_purge_unlinks_its_records() {
        let mut e = Engine::new();
        let r = renderer(&mut e);
        let t = r.addr();
        let tracker = object(&mut e, 0x6000, 0x6100_0000);
        e.mem.set_u32(t + 0x8ac, tracker);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            MAP_REMOVE_AT,
            SIZED_DELETE,
            0x6100_001c,
        ] {
            e.register(a, nop);
        }
        let geometry = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0x34, data);
        let element = e.mem.alloc(8);
        e.mem.set_u32(element + 4, 0x4e1);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, element);
        e.mem.set_u32(data + 0x20, 1);
        e.mem.set_u32(data + 0x28, array);
        // Records: ours, someone else's, ours.
        let r3 = e.mem.alloc(0x24);
        let r2 = e.mem.alloc(0x24);
        let r1 = e.mem.alloc(0x24);
        e.mem.set_u32(r1, geometry);
        e.mem.set_u32(r1 + 0x20, r2);
        e.mem.set_u32(r2, other);
        e.mem.set_u32(r2 + 0x20, r3);
        e.mem.set_u32(r3, geometry);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], r1);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, MAP_SET_AT), vec![vec![t + 0x610, 0x4e1, r2]]);
        assert_eq!(e.mem.u32(r2 + 0x20), 0);
        assert_eq!(
            calls(&e, SIZED_DELETE),
            vec![vec![r1, 0x24], vec![r3, 0x24]]
        );
        assert_eq!(calls(&e, 0x6100_001c), vec![vec![tracker, geometry, 0]]);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![t + 0x100], vec![t + 0x80]]
        );
        // A lone record of ours: the key goes.
        e.mem.set_u32(r1 + 0x20, 0);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![t + 0x610, 0x4e1]]);
        // No data: only the tracker call.
        e.mem.set_u32(data + 0x20, 0);
        e.call(0x00e7_1cf0, &args![r, geometry]);
        assert_eq!(calls(&e, ENTER_CRITICAL_SECTION).len(), 4);
        assert_eq!(calls(&e, 0x6100_001c).len(), 3);
    }
}

#[cfg(test)]
mod b048_buffer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    /// A renderer, a geometry data with `streams` stream slots, and doubles for the data's
    /// and the group's calls (the group's stream set-up fails at index `fail_at`).
    fn rig(e: &mut Engine, streams: u32, fail_at: u32) -> (Ptr<NiDX9Renderer>, u32) {
        let r = e.new_object::<NiDX9Renderer>();
        e.set(r, NiDX9Renderer::object_8bc, 0x8bc0);
        let data = e.mem.alloc(0x60);
        let slots = e.mem.alloc(4 * streams.max(1));
        e.mem.set_u32(data + 0x20, streams);
        e.mem.set_u32(data + 0x24, slots);
        let owner = object(e, 0x6800, 0x6180_0000, 0x10);
        e.mem.set_u32(data + 8, owner);
        e.register(0x6180_001c, nop);
        for a in [
            DATA_SET_COUNT_00E8EF10,
            DATA_SET_FIRST_00E8F010,
            DATA_SET_SECOND_00E8F040,
        ] {
            e.register(a, nop);
        }
        e.register(GROUP_PREPARE_00E8C120, |e, a| {
            e.mem.set_u32(a[3], 0xf1);
            e.mem.set_u32(a[4], 0x5ec);
            Ret::default()
        });
        e.register_double(GROUP_STREAM_00E8BFA0, move |_, a| {
            u32::from(a[2] != fail_at).into_ret()
        });
        e.map(STREAM_LIMIT_011BF558, 4);
        e.set_global(STREAM_LIMIT_011BF558, 1u32);
        (r, data)
    }

    #[test]
    fn stream_preparation_through_the_group() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 2, 99);
        let object = e.mem.alloc(0x30);
        e.mem.set_u16(object + 0xc, 0x1001);
        e.mem.set_u32(object + 0x24, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e6_da60, &args![r, data, object, 0u32, 0u32])
            .bool());
        let prepare = calls(&e, GROUP_PREPARE_00E8C120);
        assert_eq!(prepare.len(), 1);
        assert_eq!(&prepare[0][..3], &[0x8bc0, object, 0]);
        assert_eq!(calls(&e, DATA_SET_COUNT_00E8EF10), vec![vec![data, 1]]);
        assert_eq!(calls(&e, DATA_SET_FIRST_00E8F010), vec![vec![data, 0xf1]]);
        assert_eq!(e.mem.u32(e.mem.u32(data + 0x24)), 0x5ec);
        assert_eq!(e.mem.u32(data + 4), 0x0100_0000 | 0x80_0000 | 0x20_0000);
        assert_eq!(
            calls(&e, GROUP_STREAM_00E8BFA0),
            vec![vec![0x8bc0, data, 0]]
        );
        // A partition with more than four bones is refused before the group is asked.
        let partition = e.mem.alloc(0x30);
        e.mem.set_u16(partition + 0x20, 5);
        assert!(!e
            .call(0x00e6_da60, &args![r, data, object, partition, 0u32])
            .bool());
        assert_eq!(calls(&e, GROUP_PREPARE_00E8C120).len(), 1);
    }

    #[test]
    fn stream_preparation_through_a_source_and_its_undo() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 3, 2);
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 0x28, 1);
        let source = object_with_streams(&mut e);
        e.mem.set_u8(r.addr() + 0x5d4, 1);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e6_da60, &args![r, data, object, 0u32, source])
            .bool());
        assert!(calls(&e, GROUP_PREPARE_00E8C120).is_empty());
        assert_eq!(calls(&e, DATA_SET_SECOND_00E8F040), vec![vec![data, 0x50b]]);
        let slots = e.mem.u32(data + 0x24);
        assert_eq!(
            (e.mem.u32(slots), e.mem.u32(slots + 4), e.mem.u32(slots + 8)),
            (0x100, 0x101, 0x102)
        );
        assert_eq!(e.mem.u32(data + 4), 0x40_0000);
        assert_eq!(e.mem.u8(data + 0x14), 1);
        // Stream 2 failed: streams 2 and 1 are undone.
        let owner = e.mem.u32(data + 8);
        assert_eq!(
            calls(&e, 0x6180_001c),
            vec![vec![owner, data, 2], vec![owner, data, 1]]
        );
    }

    /// A source whose virtual +0xB0 answers three streams and the object 0x50B, +0xA0 the
    /// stream 0x100 + index and +0xA8 true.
    fn object_with_streams(e: &mut Engine) -> u32 {
        let source = object(e, 0x6400, 0x6140_0000, 0x10);
        e.register(0x6140_00b0, |e, a| {
            e.mem.set_u32(a[1], 0x50b);
            e.mem.set_u32(a[2], 3);
            1u32.into_ret()
        });
        e.register(0x6140_00a0, |_, a| (0x100 + a[1]).into_ret());
        e.register(0x6140_00a8, |_, _| 1u32.into_ret());
        source
    }

    fn map_rig(e: &mut Engine) -> Rc<RefCell<Vec<(u32, u32)>>> {
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        let map = Rc::new(RefCell::new(Vec::<(u32, u32)>::new()));
        let get = map.clone();
        e.register_double(MAP_GET_AT, move |e, a| {
            match get.borrow().iter().find(|(k, _)| *k == a[1]) {
                Some((_, v)) => {
                    e.mem.set_u32(a[2], *v);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let set = map.clone();
        e.register_double(MAP_SET_AT, move |_, a| {
            let mut m = set.borrow_mut();
            m.retain(|(k, _)| *k != a[1]);
            m.push((a[1], a[2]));
            Ret::default()
        });
        map
    }

    /// Stream elements with keys `0x4e0 + index`.
    fn keyed_streams(e: &mut Engine, data: u32, count: u32) {
        let array = e.mem.alloc(4 * count);
        for i in 0..count {
            let element = e.mem.alloc(8);
            e.mem.set_u32(element + 4, 0x4e0 + i);
            e.mem.set_u32(array + 4 * i, element);
        }
        e.mem.set_u32(data + 0x28, array);
    }

    #[test]
    fn geometry_precache_records_each_stream() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 2, 99);
        keyed_streams(&mut e, data, 2);
        let map = map_rig(&mut e);
        let object = object(&mut e, 0x6000, 0x6100_0000, 0x40);
        e.mem.set_u32(object + 0x34, data);
        e.mem.set_u16(object + 8, 0x77);
        e.register(0x6100_0090, |_, _| 0x1_0042u32.into_ret());
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 0u32.into_ret());
        map.borrow_mut().push((0x4e1, 0xabc));
        e.call_log = Some(vec![]);
        let words = args![
            r,
            0u32,
            object,
            0x1_0003u32,
            0x2_0004u32,
            5u32,
            6u32,
            0x3_0007u32,
            0u32
        ];
        assert!(e.call(0x00e7_2980, &words).bool());
        assert_eq!(e.mem.u32(data + 0x18), 0x42);
        assert_eq!(e.mem.u32(data + 0x1c), 0x77);
        assert_eq!(
            (
                e.mem.u32(data + 0x40),
                e.mem.u32(data + 0x44),
                e.mem.u32(data + 0x48)
            ),
            (3, 4, 7)
        );
        assert_eq!((e.mem.u32(data + 0x50), e.mem.u32(data + 0x4c)), (5, 6));
        let m = map.borrow();
        let first = m.iter().find(|(k, _)| *k == 0x4e0).unwrap().1;
        let second = m.iter().find(|(k, _)| *k == 0x4e1).unwrap().1;
        assert_eq!(e.mem.u32(first), object);
        assert_eq!(e.mem.u32(first + 0x18), data);
        assert_eq!(e.mem.u32(first + 0x20), 0);
        assert_eq!(e.mem.u32(second + 0x1c), 1);
        assert_eq!(e.mem.u32(second + 0x20), 0xabc);
        drop(m);
        // Packed already: true at once.
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e7_2980, &words).bool());
        assert_eq!(calls(&e, 0x6100_0090).len(), 1);
    }

    #[test]
    fn skin_precache_records_each_partition_stream() {
        let mut e = Engine::new();
        let (r, data) = rig(&mut e, 1, 99);
        keyed_streams(&mut e, data, 1);
        let map = map_rig(&mut e);
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 0u32.into_ret());
        let partitions = e.mem.alloc(0x2c * 2);
        e.mem.set_u32(partitions + 0x28, data);
        e.mem.set_u16(partitions + 0x1c, 9);
        e.mem.set_u16(partitions + 0x1e, 4);
        e.mem.set_u32(partitions + 0x14, 0x14);
        e.mem.set_u32(partitions + 0x18, 0x18);
        // The second partition's data is packed already.
        let packed = e.mem.alloc(0x30);
        e.mem.set_u32(partitions + 0x2c + 0x28, packed);
        e.register_double(DATA_IS_PACKED_00E8EEB0, move |_, a| {
            u32::from(a[0] == packed).into_ret()
        });
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 8, 2);
        e.mem.set_u32(instance + 0xc, partitions);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 0xc, instance);
        let object = e.mem.alloc(0x30);
        assert!(e
            .call(
                0x00e7_2aa0,
                &args![r, 0u32, object, skin, 0u32, 0x55u32, 0x66u32]
            )
            .bool());
        assert_eq!((e.mem.u32(data + 0x18), e.mem.u32(data + 0x1c)), (9, 9));
        assert_eq!(e.mem.u32(data + 0x48), 1);
        assert_eq!((e.mem.u32(data + 0x40), e.mem.u32(data + 0x44)), (4, 4));
        assert_eq!(
            (e.mem.u32(data + 0x50), e.mem.u32(data + 0x4c)),
            (0x14, 0x18)
        );
        let record = map.borrow()[0].1;
        assert_eq!(e.mem.u32(record + 8), partitions);
        assert_eq!(e.mem.u32(record + 4), skin);
        assert_eq!(
            (e.mem.u32(record + 0x10), e.mem.u32(record + 0x14)),
            (0x55, 0x66)
        );
        // A failed preparation makes the answer false.
        e.mem.set_u16(partitions + 0x20, 5);
        assert!(!e
            .call(
                0x00e7_2aa0,
                &args![r, 0u32, object, skin, 0u32, 0x55u32, 0x66u32]
            )
            .bool());
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn pointer_map_set_at_adds_and_replaces() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        let map = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.register(0x6100_0004, |_, a| (a[1] & 1).into_ret());
        e.register(0x6100_0008, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x6100_0014, |e, _| e.mem.alloc(12).into_ret());
        e.register(0x6100_000c, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        let first = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        let second = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        e.register(0x6140_0004, nop);
        // The caller hands each value over with one reference of its own (the by-value copy).
        e.mem.set_u32(first + 4, 1);
        e.call(0x00e7_2cf0, &args![map, 3u32, first]);
        let item = e.mem.u32(buckets + 4);
        assert_eq!(e.mem.u32(item + 8), first);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.mem.set_u32(second + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e7_2cf0, &args![map, 3u32, second]);
        assert_eq!(e.mem.u32(item + 8), second);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(calls(&e, 0x6140_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
    }

    #[test]
    fn skin_purge_unlinks_each_partitions_records() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t + 0x8ac, 0x8ac0);
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            MAP_SET_AT,
            MAP_REMOVE_AT,
            SIZED_DELETE,
            REMOVE_SKIN_PARTITION_FROM_GROUP,
        ] {
            e.register(a, nop);
        }
        let data = e.mem.alloc(0x30);
        keyed_streams(&mut e, data, 1);
        e.mem.set_u32(data + 0x20, 1);
        let partitions = e.mem.alloc(0x2c);
        e.mem.set_u32(partitions + 0x28, data);
        let instance = e.mem.alloc(0x10);
        e.mem.set_u32(instance + 8, 1);
        e.mem.set_u32(instance + 0xc, partitions);
        let record = e.mem.alloc(0x24);
        e.mem.set_u32(record + 8, partitions);
        e.register_double(MAP_GET_AT, move |e, a| {
            e.mem.set_u32(a[2], record);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_1e40, &args![r, instance]);
        assert_eq!(calls(&e, MAP_REMOVE_AT), vec![vec![t + 0x610, 0x4e0]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![record, 0x24]]);
        assert_eq!(
            calls(&e, REMOVE_SKIN_PARTITION_FROM_GROUP),
            vec![vec![0x8ac0, instance]]
        );
        // Without data on the first partition nothing happens at all.
        e.mem.set_u32(partitions + 0x28, 0);
        e.call(0x00e7_1e40, &args![r, instance]);
        assert_eq!(calls(&e, REMOVE_SKIN_PARTITION_FROM_GROUP).len(), 1);
    }
}

#[cfg(test)]
mod b048_setup_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    fn ref_counting(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_UP, |e, a| {
            let v = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DOWN, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn clipper_texture_from_a_two_texel_image() {
        let mut e = Engine::new();
        ref_counting(&mut e);
        e.map(EMPTY_FIXED_STRING_0109B220, 4);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let format = e.mem.alloc(0x20);
        e.mem.set_u8(format + 1, 0x20);
        e.mem.set_u32(t + 0x700, format);
        let targets: Vec<u32> = (0..8).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.register(0x6100_0004, nop);
        e.register(SAMPLER_SET_UP_00E6DFC0, nop);
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PIXEL_DATA_CONSTRUCT_00A7C190, |e, a| {
            let texels = e.mem.alloc(0x10);
            e.mem.set_u32(texels, 0x1111_1111);
            e.mem.set_u32(texels + 4, 0x2222_2222);
            let buffers = e.mem.alloc(4);
            e.mem.set_u32(buffers, texels);
            e.mem.set_u32(a[0], 0x6000);
            e.mem.set_u32(a[0] + 0x5c, buffers);
            e.mem.set_u32(a[0] + 0x64, 1);
            a[0].into_ret()
        });
        e.register(MEMSET_ADDR, nop);
        e.register(PIXEL_FORMAT_SET_UP_00A622D0, nop);
        let handle_block = e.mem.alloc(0x10);
        let handle = handle_block + 8;
        e.register_double(FIXED_STRING_MAKE_00A5B690, move |_, _| handle.into_ret());
        let texture = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.register_double(SOURCE_TEXTURE_00A5FF10, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), 6);
            texture.into_ret()
        });
        e.mem.set_u32(handle_block, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_ec90, &args![r]).bool());
        let fills = calls(&e, SAMPLER_SET_UP_00E6DFC0);
        assert_eq!(
            fills,
            vec![
                vec![t, t + 0x700, 0, 3],
                vec![t, t + 0x758, 1, 3],
                vec![t, t + 0x7b0, 0, 5],
                vec![t, t + 0x808, 1, 5]
            ]
        );
        let created = calls(&e, SOURCE_TEXTURE_00A5FF10);
        let pixels = created[0][0];
        assert_eq!(created[0][1], handle);
        let texels = e.mem.u32(e.mem.u32(pixels + 0x5c));
        assert_eq!(e.mem.u32(texels), 0);
        assert_eq!(e.mem.u32(texels + 4), 0xffff_ffff);
        assert_eq!(e.mem.u32(t + CLIPPER_TEXTURE), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        // The name's count and the pixel data's reference went back down.
        assert_eq!(e.mem.u32(handle_block), 0);
        assert_eq!(e.mem.u32(pixels + 4), 0);
        assert_eq!(calls(&e, 0x6100_0004), vec![vec![pixels]]);
    }

    #[test]
    fn precache_geometry_by_class() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t, 0x6000);
        e.mem.set_u32(t + 0x8ac, 0x8ac0);
        e.mem.set_u32(t + 0x8b0, 0x8b00);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.register(0x6100_00a0, |_, _| 0u32.into_ret());
        for a in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            ADD_OBJECT_TO_GROUP,
            REPORT_00A29680,
        ] {
            e.register(a, nop);
        }
        e.map(GEOMETRY_RTTI, 8);
        let geometry = object(&mut e, 0x6400, 0x6140_0000, 0xd0);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, GEOMETRY_RTTI);
        e.register_double(0x6140_0008, move |_, _| rtti.into_ret());
        // No shader: false with the message.
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(calls(&e, REPORT_00A29680).len(), 1);
        let shader = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        e.register(0x6180_00bc, |_, _| 0x5ce0u32.into_ret());
        e.mem.set_u32(geometry + 0xc0, shader);
        // A data object with renderer data (not skinned; the packing finds it packed).
        let data = object(&mut e, 0x6c00, 0x61c0_0000, 0x60);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(geometry + 0xbc, 0x5c1);
        e.register(DATA_IS_PACKED_00E8EEB0, |_, _| 1u32.into_ret());
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TRI_SHAPE_RTTI_011F4A40).into_ret()
        });
        e.register(0x61c0_00a8, |_, _| 3u32.into_ret());
        e.register(TRI_SHAPE_DATA_GET_TRI_LIST, |_, _| 0x7215u32.into_ret());
        e.register(TRI_STRIPS_DATA_GET_STRIP_LISTS, |_, _| 0x5791u32.into_ret());
        assert!(e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(
            calls(&e, ADD_OBJECT_TO_GROUP)[0],
            vec![0x8ac0, 0x8b00, data, 0x5c1, 0, 1, 2]
        );
        assert_eq!(calls(&e, TRI_SHAPE_DATA_GET_TRI_LIST), vec![vec![data]]);
        assert_eq!(
            calls(&e, LEAVE_CRITICAL_SECTION),
            vec![vec![t + 0x100], vec![t + 0x80]]
        );
        // Strips go through their strip lists; another class packs nothing.
        e.register(IS_KIND_OF, |_, a| {
            u32::from(a[0] == TRI_STRIPS_RTTI_011F4A20).into_ret()
        });
        assert!(e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 9u32])
            .bool());
        assert_eq!(calls(&e, TRI_STRIPS_DATA_GET_STRIP_LISTS), vec![vec![data]]);
        e.register(IS_KIND_OF, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 9u32])
            .bool());
        // Not a geometry: false at once.
        e.register(0x6140_0008, |_, _| 0u32.into_ret());
        assert!(!e
            .call(0x00e7_3f60, &args![r, geometry, 1u32, 2u32, 0u32])
            .bool());
        assert_eq!(calls(&e, ADD_OBJECT_TO_GROUP).len(), 3);
    }
}

#[cfg(test)]
mod b048_draw_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    fn ftol(_: &mut Engine, a: &[u32]) -> Ret {
        let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
        (v.trunc() as i32).into_ret()
    }

    #[test]
    fn clear_converts_the_rectangle_and_checks_the_depth_buffer() {
        let mut e = Engine::new();
        e.map(TWO_TO_32_01099378, 4);
        e.set_global(TWO_TO_32_01099378, 4294967296.0f32.to_bits());
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let group = object(&mut e, 0x6400, 0x6140_0000, 0x10);
        e.set(r, NiDX9Renderer::target_group, group);
        e.register(0x6140_008c, |_, _| 640u32.into_ret());
        e.register(0x6140_0090, |_, _| 480u32.into_ret());
        let depth = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti + 4, DEPTH_BUFFER_RTTI_0126F5AC);
        e.register_double(0x6180_000c, move |_, _| rtti.into_ret());
        e.register_double(0x6140_00cc, move |_, _| depth.into_ret());
        e.register(DEPTH_HAS_STENCIL_00E7CDF0, |_, _| 1u32.into_ret());
        e.register(DEPTH_HAS_DEPTH_00E7CE10, |_, _| 0u32.into_ret());
        e.register(FTOL, ftol);
        e.register(0x6100_0000 + DEVICE_SET_VIEWPORT, nop);
        e.register(0x6100_0000 + DEVICE_CLEAR, |e, a| {
            let r: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            (r[0] | r[1] << 10 | r[2] << 20).into_ret()
        });
        e.set(r, NiDX9Renderer::background_color, 0xff20_4060);
        e.mem.set_u32(t + 0x5e4, 1.0f32.to_bits());
        e.mem.set_u32(t + 0x5e8, 7);
        let rect = e.mem.alloc(0x10);
        for (i, v) in [0.25f32, 0.5, 0.75, 1.0].iter().enumerate() {
            e.mem.set_u32(rect + 4 * i as u32, v.to_bits());
        }
        e.call_log = Some(vec![]);
        e.call(0x00e6_f310, &args![r, rect, 7u32]);
        let clears = calls(&e, 0x6100_0000 + DEVICE_CLEAR);
        assert_eq!(
            clears,
            vec![vec![
                device,
                1,
                clears[0][2],
                1 | 2,
                0xff20_4060,
                1.0f32.to_bits(),
                7
            ]]
        );
        assert_eq!(e.mem.u32(t + 0x6d4), 640);
        assert_eq!(e.mem.u32(t + 0x6d8), 480);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_SET_VIEWPORT),
            vec![vec![device, t + 0x6cc]]
        );
        // left 0.25·640, top (1 − 0.75)·480, right 0.5·640, bottom 0.
        let corners: Vec<u32> = calls(&e, FTOL)
            .iter()
            .map(|a| f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as u32)
            .collect();
        assert_eq!(corners, vec![160, 120, 320, 0]);
        // Nothing to clear, or skipping: no call.
        e.call(0x00e6_f310, &args![r, rect, 0u32]);
        e.mem.set_u8(t + SKIP_END_SCENE, 1);
        e.call(0x00e6_f310, &args![r, rect, 1u32]);
        assert_eq!(calls(&e, 0x6100_0000 + DEVICE_CLEAR).len(), 1);
        // Without a rectangle the whole target, without a depth buffer only the target bit.
        e.mem.set_u8(t + SKIP_END_SCENE, 0);
        e.register(0x6140_00cc, |_, _| 0u32.into_ret());
        e.call(0x00e6_f310, &args![r, 0u32, 7u32]);
        let clears = calls(&e, 0x6100_0000 + DEVICE_CLEAR);
        assert_eq!(clears[1][3], 1);
        assert_eq!(clears.len(), 2);
        assert_eq!(calls(&e, FTOL).len(), 4);
    }

    fn draw_rig(e: &mut Engine) -> (Ptr<NiDX9Renderer>, u32, u32, u32) {
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6140_0000 + 4 * n).collect();
        e.put_vtable(0x6400, &targets);
        e.mem.set_u32(r.addr(), 0x6400);
        e.register(0x6140_00a0, |_, _| 0u32.into_ret());
        e.register(0x6100_0000 + DEVICE_DRAW_PRIMITIVE, nop);
        e.register(0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE, nop);
        let geometry = e.mem.alloc(0xd0);
        let data = object(e, 0x6800, 0x6180_0000, 0x60);
        e.register(0x6180_0090, |_, _| 4u32.into_ret());
        e.mem.set_u32(geometry + 0xb8, data);
        let shader = object(e, 0x6c00, 0x61c0_0000, 0x10);
        e.register(0x61c0_0090, nop);
        e.mem.set_u32(geometry + 0xc0, shader);
        let bound = e.mem.alloc(0x10);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_u32(bound + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u32(geometry + 0x20, bound);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        (r, geometry, data, render_data)
    }

    #[test]
    fn draw_without_an_index_buffer_draws_primitives() {
        let mut e = Engine::new();
        let (r, geometry, data, render_data) = draw_rig(&mut e);
        e.mem.set_u32(render_data + 0x3c, 4);
        e.mem.set_u32(render_data + 0x38, 10);
        e.mem.set_u32(render_data + 0x40, 20);
        e.mem.set_u16(data + 0xe, 0x3fff);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4840, &args![r, geometry]);
        let device = e.get(r, NiDX9Renderer::device);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_PRIMITIVE),
            vec![vec![device, 4, 10, 20]]
        );
        assert_eq!(e.mem.u16(data + 0xe), 0x3000);
        assert_eq!(e.mem.u32(r.addr() + DRAWN_BOUND + 4), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(r.addr() + DRAWN_BOUND + 0xc), 4.0f32.to_bits());
    }

    #[test]
    fn draw_with_an_index_buffer_draws_each_strip() {
        let mut e = Engine::new();
        let (r, geometry, _, render_data) = draw_rig(&mut e);
        e.mem.set_u32(render_data + 0x34, 1);
        e.mem.set_u32(render_data + 0x3c, 5);
        e.mem.set_u32(render_data + 0x38, 10);
        e.mem.set_u32(render_data + 0x18, 30);
        e.mem.set_u32(render_data + 0x48, 2);
        let lengths = e.mem.alloc(4);
        e.mem.set_u16(lengths, 6);
        e.mem.set_u16(lengths + 2, 5);
        e.mem.set_u32(render_data + 0x4c, lengths);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4840, &args![r, geometry]);
        let device = e.get(r, NiDX9Renderer::device);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE),
            vec![
                vec![device, 5, 10, 0, 30, 0, 4],
                vec![device, 5, 10, 0, 30, 6, 3]
            ]
        );
        // Without vertices nothing is drawn.
        e.register(0x6180_0090, |_, _| 0x1_0000u32.into_ret());
        e.call(0x00e7_4840, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE).len(),
            2
        );
    }
}

#[cfg(test)]
mod b048_caps_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    #[test]
    fn device_caps_set_the_flags_and_defaults() {
        let mut e = Engine::new();
        e.map(0x0126_f050, 0x98);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        for a in [
            STRNCPY_S,
            REPORT_00A29680,
            CAPS_SET_UP_00E7F830,
            CAPS_SET_UP_00E7F840,
            CAPS_SET_UP_00E7F850,
        ] {
            e.register(a, nop);
        }
        e.register(0x6100_0000 + GET_DEVICE_CAPS_SLOT, |_, _| {
            0x8000_0000u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![
                ERROR_MESSAGE_011F4648,
                0x100,
                NO_DEVICE_CAPS_010ED924,
                0xff
            ]]
        );
        e.register(0x6100_0000 + GET_DEVICE_CAPS_SLOT, |_, _| 0u32.into_ret());
        e.mem.set_u32(t + 0x320, 1);
        e.mem.set_u32(t + 0x324, 4);
        assert!(!e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(
            calls(&e, REPORT_00A29680).last().unwrap(),
            &vec![NO_MULTI_TEXTURE_REPORT_010ED840]
        );
        e.mem.set_u32(t + 0x320, 8);
        e.mem.set_u32(t + 0x2c8, 0x0001_0502);
        e.mem.set_u32(t + 0x2cc, 0x0400_0400);
        e.mem.set_u32(t + 0x298, 0x2000_0000);
        e.mem.set_u32(t + 0x2ac, 0x000c_0000);
        e.mem.set_u32(t + 0x334, 4);
        e.mem.set_u32(t + 0x348, 0xfffe_0300);
        e.mem.set_u32(t + 0x350, 0xffff_0300);
        e.mem.set_u32(t + 0x358, 0xffff_0300);
        e.mem.set_u32(t + 0x37c, 0x37c);
        assert!(e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(e.mem.u32(t + 0x5ec), 4 | 2);
        assert_eq!(e.mem.u32(t + 0x6e4), 4);
        assert_eq!(
            (
                e.mem.u8(t + 0x6f4),
                e.mem.u8(t + 0x6f5),
                e.mem.u8(t + 0x8a4),
                e.mem.u8(t + 0x8a5)
            ),
            (1, 1, 1, 1)
        );
        assert_eq!(e.mem.u32(t + 0x8a0), 0x37c);
        assert_eq!(e.mem.u32(t + 0x6ec), 0xffff_0300);
        assert_eq!(e.mem.u32(t + 0x93c), 2);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F830), vec![vec![8]]);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F840), vec![vec![4]]);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850), vec![vec![0x10]]);
        assert_eq!(e.global::<u8>(0x0126_f0d1), 1);
        assert_eq!(e.global::<u8>(0x0126_f0d2), 1);
        assert_eq!(e.global::<u32>(0x0126_f09c), 3);
        assert_eq!(e.global::<u32>(0x0126_f098), 3);
        assert_eq!(e.global::<u32>(0x0126_f0b0), 3);
        assert_eq!(e.global::<u32>(0x0126_f088), 2);
        assert_eq!(e.global::<u32>(0x0126_f0e4), 0x103);
        // Without texture caps bit 1: 8, and a lower pixel version passes the stage count.
        e.mem.set_u32(t + 0x2c8, 0);
        e.mem.set_u32(t + 0x334, 2);
        e.mem.set_u32(t + 0x358, 0xffff_0100);
        assert!(e.call(0x00e6_c240, &args![r, 0u32]).bool());
        assert_eq!(e.mem.u32(t + 0x5ec), 8);
        assert_eq!(calls(&e, CAPS_SET_UP_00E7F850)[1], vec![8]);
        assert_eq!(e.global::<u32>(0x0126_f0e4), 2);
    }

    /// An object whose class chain (virtual +0xC) holds `rtti`.
    fn of_kind(e: &mut Engine, table: u32, base: u32, rtti: u32) -> u32 {
        let o = object(e, table, base, 0x10);
        let link = e.mem.alloc(8);
        e.mem.set_u32(link + 4, rtti);
        e.register_double(base + 0xc, move |_, _| link.into_ret());
        o
    }

    #[test]
    fn begin_using_a_group_sets_its_buffers() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        let targets: Vec<u32> = (0..0x80).map(|n| 0x6100_0000 + 4 * n).collect();
        e.put_vtable(0x6000, &targets);
        e.mem.set_u32(t, 0x6000);
        e.set(r, NiDX9Renderer::device, 0xde01);
        e.set(r, NiDX9Renderer::object_884, 0x884d);
        e.map(0x0126_f590, 0x60);
        for a in [
            CLEAR_RENDER_TARGET_00E7CC50,
            REPORT_00A29680,
            0x6100_0188,
            0x6100_0194,
        ] {
            e.register(a, nop);
        }
        e.register(CLEAR_DEPTH_STENCIL_00E7CCA0, |_, _| 1u32.into_ret());
        let buffer = of_kind(&mut e, 0x6400, 0x6140_0000, RENDER_BUFFER_RTTI_0126F59C);
        e.register(0x6140_005c, |_, _| 1u32.into_ret());
        let group = object(&mut e, 0x6800, 0x6180_0000, 0x10);
        e.register(0x6180_00a4, |_, _| 1u32.into_ret());
        e.register_double(0x6180_00c8, move |_, _| buffer.into_ret());
        e.register(0x6180_00cc, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(
            calls(&e, CLEAR_RENDER_TARGET_00E7CC50),
            vec![vec![0xde01, 1], vec![0xde01, 2], vec![0xde01, 3]]
        );
        assert_eq!(calls(&e, 0x6140_005c), vec![vec![buffer, 0xde01, 0]]);
        assert_eq!(calls(&e, CLEAR_DEPTH_STENCIL_00E7CCA0), vec![vec![0xde01]]);
        assert_eq!(e.get(r, NiDX9Renderer::target_group), group);
        assert_eq!(e.mem.u32(t + 0x88c), 0);
        assert_eq!(calls(&e, 0x6100_0188), vec![vec![t, 0, 7]]);
        // A failing target falls back to the default group.
        e.register(0x6140_005c, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(calls(&e, 0x6100_0194), vec![vec![t, 0x884d, 7]]);
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![BEGIN_GROUP_FAILED_TEXT, OUTPUT_TARGET_FAILED_TEXT]]
        );
        // A depth buffer that fails.
        e.register(0x6140_005c, |_, _| 1u32.into_ret());
        let depth = of_kind(&mut e, 0x6c00, 0x61c0_0000, DEPTH_BUFFER_RTTI_0126F5AC);
        e.register(0x61c0_0060, |_, _| 0u32.into_ret());
        e.register_double(0x6180_00cc, move |_, _| depth.into_ret());
        assert!(!e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(
            calls(&e, REPORT_00A29680)[1],
            vec![BEGIN_GROUP_FAILED_TEXT, DEPTH_STENCIL_FAILED_TEXT]
        );
        // Skipping: true at once.
        e.mem.set_u8(t + SKIP_END_SCENE, 1);
        assert!(e.call(0x00e6_eea0, &args![r, group, 7u32]).bool());
        assert_eq!(calls(&e, CLEAR_RENDER_TARGET_00E7CC50).len(), 9);
    }
}

#[cfg(test)]
mod b048_partial_draw_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn object(e: &mut Engine, table: u32, base: u32, size: u32) -> u32 {
        let targets: Vec<u32> = (0..0x80).map(|n| base + 4 * n).collect();
        e.put_vtable(table, &targets);
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, table);
        o
    }

    #[test]
    fn partial_draw_uses_the_active_records() {
        let mut e = Engine::new();
        let r = e.new_object::<NiDX9Renderer>();
        let device = object(&mut e, 0x6000, 0x6100_0000, 0x10);
        e.set(r, NiDX9Renderer::device, device);
        let targets: Vec<u32> = (0..0x40).map(|n| 0x6140_0000 + 4 * n).collect();
        e.put_vtable(0x6400, &targets);
        e.mem.set_u32(r.addr(), 0x6400);
        e.register(0x6140_00a0, |_, _| 0u32.into_ret());
        e.register(0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE, nop);
        let geometry = object(&mut e, 0x6800, 0x6180_0000, 0xd0);
        let data = object(&mut e, 0x6c00, 0x61c0_0000, 0x60);
        e.register(0x61c0_0090, |_, _| 4u32.into_ret());
        e.mem.set_u32(geometry + 0xb8, data);
        let shader = object(&mut e, 0x7000, 0x6200_0000, 0x10);
        e.register(0x6200_0090, nop);
        e.mem.set_u32(geometry + 0xc0, shader);
        let render_data = e.mem.alloc(0x60);
        e.mem.set_u32(data + 0x34, render_data);
        e.mem.set_u32(render_data + 0x34, 1);
        e.mem.set_u32(render_data + 0x48, 1);
        e.mem.set_u32(render_data + 0x40, 9);
        e.mem.set_u32(render_data + 0x3c, 4);
        // Two records, only the second active.
        let draw = e.mem.alloc(0xd0);
        let records = e.mem.alloc(0x28);
        e.mem.set_u32(records + 0x14, 3);
        e.mem.set_u32(records + 0x14 + 0xc, 5);
        e.mem.set_u8(records + 0x14 + 0x10, 1);
        e.mem.set_u32(draw + 0xc4, records);
        e.mem.set_u32(draw + 0xc8, 2);
        e.register_double(0x6180_0028, move |_, _| draw.into_ret());
        e.mem.set_u16(data + 0xe, 0xffff);
        e.call_log = Some(vec![]);
        e.call(0x00e7_45a0, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE),
            vec![vec![device, 4, 0, 0, 0, 3, 5]]
        );
        assert_eq!(e.mem.u16(data + 0xe), 0xf000);
        // Without a draw object the whole strip is drawn.
        e.register(0x6180_0028, |_, _| 0u32.into_ret());
        e.call(0x00e7_45a0, &args![r, geometry]);
        assert_eq!(
            calls(&e, 0x6100_0000 + DEVICE_DRAW_INDEXED_PRIMITIVE)[1],
            vec![device, 4, 0, 0, 0, 0, 9]
        );
    }

    #[test]
    fn member_set_up_clears_and_fills_the_tables() {
        let mut e = Engine::new();
        e.map(CLEARED_BLOCK_0126EFD8, 0x80);
        e.map(DEFAULT_WORDS_011A946C, 0x18);
        e.map(FRUSTUM_NEAR_0101E2BC, 4);
        e.map(FLOAT_SOURCE_01016410, 4);
        e.set_global(FRUSTUM_NEAR_0101E2BC, 2.0f32.to_bits());
        e.set_global(DEFAULT_WORDS_011A946C + 4, 0x44u32);
        e.register(MEMSET_ADDR, nop);
        let r = e.new_object::<NiDX9Renderer>();
        let t = r.addr();
        e.mem.set_u32(t + 0x700, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00e6_b990, &args![r]);
        assert_eq!(e.mem.u32(t + 0x5e4), ONE);
        assert_eq!(e.mem.u32(t + 0x5e0), 0xff80_8080);
        assert_eq!(e.mem.u32(t + 0x638), 0x44);
        assert_eq!(e.mem.u32(t + 0x650), 0x44);
        assert_eq!(e.mem.u32(t + 0x684), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(t + 0x6b4), ONE);
        assert_eq!(e.mem.u32(t + 0x6e0), ONE);
        assert_eq!(e.mem.u32(t + 0x700), 0);
        assert_eq!(e.mem.u32(t + 0x880), 0x16);
        assert_eq!(e.mem.u32(t + 0x97c), ONE);
        assert_eq!(e.global::<u8>(CLEARED_BLOCK_0126EFD8 + 20), 0x18);
        assert_eq!(e.global::<u8>(CLEARED_BLOCK_0126EFD8 + 116), 0x80);
        assert_eq!(
            calls(&e, MEMSET_ADDR),
            vec![
                vec![t + DEVICE_CAPS, 0, 0x130],
                vec![t + 0x68c, 0, 0x40],
                vec![CLEARED_BLOCK_0126EFD8, 0, 0x76]
            ]
        );
    }
}
