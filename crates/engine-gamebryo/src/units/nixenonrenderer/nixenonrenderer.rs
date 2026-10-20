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

/// The pixel data constructor `00a7c190` (unnamed; `thiscall`: width, height, pixel format,
/// mipmap levels, faces) and the pixel format's set-up `00a622d0` (unnamed; `cdecl`).
const PIXEL_DATA_00A7C190: u32 = 0x00a7_c190;
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
                PIXEL_DATA_00A7C190,
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
        e.register(PIXEL_DATA_00A7C190, |e, a| {
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
