//! `NiXenonRenderer/run_00e93f50` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The globals `00e93f50` keeps: the renderer (`0126fa2c`) and its device (`0126fa28`,
/// counted with the device's `AddRef`/`Release`, slots 4 and 8).
const RENDERER_0126FA2C: u32 = 0x0126_fa2c;
const DEVICE_0126FA28: u32 = 0x0126_fa28;

// Translated from 00e93f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e93f50`), `cdecl`, the renderer: keeps it at
/// `0126fa2c`. With a renderer, the device held at `0126fa28` is released (slot 8) and
/// replaced by the renderer's device (+0x288; counted, slot 4, when not null); without one
/// the held device is released and the word cleared.
pub fn fn_00e93f50(e: &mut Engine, renderer: u32) {
    e.mem.set_u32(RENDERER_0126FA2C, renderer);
    let held = e.mem.u32(DEVICE_0126FA28);
    if renderer != 0 {
        let device = e.mem.u32(renderer.wrapping_add(0x288));
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(DEVICE_0126FA28, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
    } else {
        if held != 0 {
            e.vcall(held, 8, &args![]);
        }
        e.mem.set_u32(DEVICE_0126FA28, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_3f50, fn_00e93f50(u32))]
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e93f50_swaps_the_held_device() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.put_vtable(0x6000, &[0, 0x6104, 0x6108]);
        e.register(0x6104, |_, _| Ret::default());
        e.register(0x6108, |_, _| Ret::default());
        let device = e.mem.alloc(4);
        e.mem.set_u32(device, 0x6000);
        let renderer = e.mem.alloc(0x300);
        e.mem.set_u32(renderer + 0x288, device);
        e.call_log = Some(vec![]);
        e.call(0x00e9_3f50, &args![renderer]);
        assert_eq!(e.mem.u32(RENDERER_0126FA2C), renderer);
        assert_eq!(e.mem.u32(DEVICE_0126FA28), device);
        e.call(0x00e9_3f50, &args![0u32]);
        assert_eq!(e.mem.u32(DEVICE_0126FA28), 0);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(0x6104, vec![device])));
        assert!(log.contains(&(0x6108, vec![device])));
    }
}
