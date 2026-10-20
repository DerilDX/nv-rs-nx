//! `NiXenonRenderer/run_00e75a70` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiXenonRenderer::CreateDirect3D9` (engine map, `00e69460`; `cdecl`, one word): -1 when
/// Direct3D cannot be created.
const CREATE_DIRECT3D9: u32 = 0x00e6_9460;
/// `_strncpy_s` (`00ec8d5f`, `cdecl`: destination, size, source, count), the error text buffer
/// it fills (`011f4648`, 0x100 bytes) and the message copied into it
/// (`"Creation failed: Could not initialize DirectX9"`, `010ee6c8`).
const STRNCPY_S: u32 = 0x00ec_8d5f;
const ERROR_MESSAGE_011F4648: u32 = 0x011f_4648;
const CREATION_FAILED_010EE6C8: u32 = 0x010e_e6c8;
/// `00a29680` (unnamed; `cdecl`, a message) and the message passed to it
/// (`"NiDX9Renderer::Create> Invalid device ID specified - FAILING\n"`, `010ee688`).
const REPORT_00A29680: u32 = 0x00a2_9680;
const INVALID_DEVICE_010EE688: u32 = 0x010e_e688;
/// The allocation `00aa13e0` (`cdecl`, size), the renderer's size and its constructor
/// (`00e754a0`, unnamed; `thiscall`, no stack words).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const RENDERER_SIZE: u32 = 0xb80;
const RENDERER_CONSTRUCT: u32 = 0x00e7_54a0;
/// The renderer's initialisation (`00e72e60`, `nixenonrenderer.cpp`; `thiscall` with fourteen
/// words): a nonzero byte on success.
const RENDERER_INITIALIZE: u32 = 0x00e7_2e60;
/// `00a63520` (`nistream.cpp`; `cdecl`, a class name), `NiStream::RegisterLoader` (engine map,
/// `00a64900`; `cdecl`: class name, loader) and what they are given: the class name
/// `"NiPersistentSrcTextureRendererData"` (`0109f0a0`) and the loader at `00e916d0`.
const STREAM_00A63520: u32 = 0x00a6_3520;
const NI_STREAM_REGISTER_LOADER: u32 = 0x00a6_4900;
const PERSISTENT_SRC_TEXTURE_RENDERER_DATA: u32 = 0x0109_f0a0;
const PERSISTENT_SRC_TEXTURE_LOADER: u32 = 0x00e9_16d0;

// Translated from 00e76210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (its own message calls it `NiDX9Renderer::Create`), `cdecl` with
/// 15 words: the first 14 go to the renderer's initialisation, the 15th to
/// `NiXenonRenderer::CreateDirect3D9`. When that answers -1 the error text gets "Creation
/// failed: Could not initialize DirectX9" (`_strncpy_s`, at most 0xff characters), `00a29680`
/// reports the invalid device ID, and null is answered. Otherwise a 0xb80-byte renderer is
/// made (`00e754a0`; null when the allocation fails, and `00e72e60` still runs on it) and
/// initialised with the 14 words; a failed initialisation deletes a made renderer (its
/// virtual slot 0 with 1) and answers null. Success registers
/// `NiPersistentSrcTextureRendererData` (`00a63520`, then `NiStream::RegisterLoader` with
/// `00e916d0`) and answers the renderer.
#[allow(clippy::too_many_arguments)]
pub fn fn_00e76210(
    e: &mut Engine,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
    seventh: u32,
    eighth: u32,
    ninth: u32,
    tenth: u32,
    eleventh: u32,
    twelfth: u32,
    thirteenth: u32,
    fourteenth: u32,
    device: u32,
) -> u32 {
    if e.call(CREATE_DIRECT3D9, &args![device]).i32() == -1 {
        e.call(
            STRNCPY_S,
            &args![
                ERROR_MESSAGE_011F4648,
                0x100u32,
                CREATION_FAILED_010EE6C8,
                0xffu32
            ],
        );
        e.call(REPORT_00A29680, &args![INVALID_DEVICE_010EE688]);
        return 0;
    }
    let block = e.call(NEW_OBJECT, &args![RENDERER_SIZE]).u32();
    let renderer = if block != 0 {
        e.call(RENDERER_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let initialized = e
        .call(
            RENDERER_INITIALIZE,
            &args![
                renderer, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth,
                tenth, eleventh, twelfth, thirteenth, fourteenth
            ],
        )
        .u8();
    if initialized == 0 {
        if renderer != 0 {
            e.vcall(renderer, 0, &args![1u32]);
        }
        return 0;
    }
    e.call(
        STREAM_00A63520,
        &args![PERSISTENT_SRC_TEXTURE_RENDERER_DATA],
    );
    e.call(
        NI_STREAM_REGISTER_LOADER,
        &args![
            PERSISTENT_SRC_TEXTURE_RENDERER_DATA,
            PERSISTENT_SRC_TEXTURE_LOADER
        ],
    );
    renderer
}

/// The registry entry of [`fn_00e76210`]: its 15 words.
fn fn_00e76210_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let w = |i: usize| words[i];
    fn_00e76210(
        e,
        w(0),
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
    )
    .into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00e7_6210, fn_00e76210_entry as AbiFn)]
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    const WORDS: [u32; 15] = [11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
    const RENDERER_VTABLE: u32 = 0x6000;
    const RENDERER_DELETE: u32 = 0x6100;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(direct3d: u32, allocation: bool, initialized: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(CREATE_DIRECT3D9, move |_, _| direct3d.into_ret());
        e.register(STRNCPY_S, |_, _| Ret::default());
        e.register(REPORT_00A29680, |_, _| Ret::default());
        e.register_double(NEW_OBJECT, move |e, a| {
            if allocation {
                e.mem.alloc(a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(RENDERER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], RENDERER_VTABLE);
            a[0].into_ret()
        });
        e.put_vtable(RENDERER_VTABLE, &[RENDERER_DELETE]);
        e.register(RENDERER_DELETE, |_, _| Ret::default());
        e.register_double(RENDERER_INITIALIZE, move |_, _| initialized.into_ret());
        e.register(STREAM_00A63520, |_, _| Ret::default());
        e.register(NI_STREAM_REGISTER_LOADER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e76210_fails_without_direct3d() {
        let mut e = rig(u32::MAX, true, 1);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        assert_eq!(calls(&e, CREATE_DIRECT3D9), vec![vec![25]]);
        assert_eq!(
            calls(&e, STRNCPY_S),
            vec![vec![
                ERROR_MESSAGE_011F4648,
                0x100,
                CREATION_FAILED_010EE6C8,
                0xff
            ]]
        );
        assert_eq!(
            calls(&e, REPORT_00A29680),
            vec![vec![INVALID_DEVICE_010EE688]]
        );
        assert!(calls(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn fn_00e76210_initialises_and_registers_the_loader() {
        let mut e = rig(0, true, 0x101);
        let renderer = e.call(0x00e7_6210, &WORDS).u32();
        assert_ne!(renderer, 0);
        assert_eq!(calls(&e, NEW_OBJECT), vec![vec![RENDERER_SIZE]]);
        let mut expected = vec![renderer];
        expected.extend_from_slice(&WORDS[..14]);
        assert_eq!(calls(&e, RENDERER_INITIALIZE), vec![expected]);
        assert_eq!(
            calls(&e, STREAM_00A63520),
            vec![vec![PERSISTENT_SRC_TEXTURE_RENDERER_DATA]]
        );
        assert_eq!(
            calls(&e, NI_STREAM_REGISTER_LOADER),
            vec![vec![
                PERSISTENT_SRC_TEXTURE_RENDERER_DATA,
                PERSISTENT_SRC_TEXTURE_LOADER
            ]]
        );
        assert!(calls(&e, RENDERER_DELETE).is_empty());
    }

    #[test]
    fn fn_00e76210_deletes_a_renderer_that_fails_to_initialise() {
        let mut e = rig(0, true, 0x100);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        let renderer = calls(&e, RENDERER_CONSTRUCT)[0][0];
        assert_eq!(calls(&e, RENDERER_DELETE), vec![vec![renderer, 1]]);
        assert!(calls(&e, STREAM_00A63520).is_empty());
        // No renderer: the initialisation still runs on null; nothing is deleted.
        let mut e = rig(0, false, 0);
        assert_eq!(e.call(0x00e7_6210, &WORDS).u32(), 0);
        assert_eq!(calls(&e, RENDERER_INITIALIZE)[0][0], 0);
        assert!(calls(&e, RENDERER_DELETE).is_empty());
    }
}
