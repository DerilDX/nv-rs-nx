//! `NiXenonRenderer/nid3dshaderconstantmap.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e81d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderconstantmap.cpp`): the constant map's renderer
/// set-up, `thiscall` with one stack word (the renderer), stored in `m_pkD3DRenderer` (+0x3c).
/// With a renderer: the old device in `m_pkD3DDevice` (+0x38) is released (its vtable slot +8,
/// `stdcall`), the renderer's device (+0x288) is kept there and referenced (slot +4) when not
/// null, and `m_pkD3DRenderState` (+0x40) takes the renderer's +0x8b8. Without one: the old
/// device is released, and the device and render state are cleared (Xbox PDB names).
pub fn fn_00e81d00(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    let renderer = renderer.addr();
    e.mem.set_u32(base + 0x3c, renderer);
    if renderer != 0 {
        let device = e.mem.u32(renderer + 0x288);
        let old = e.mem.u32(base + 0x38);
        if old != 0 {
            e.vcall(old, 8, &args![]);
        }
        e.mem.set_u32(base + 0x38, device);
        if device != 0 {
            e.vcall(device, 4, &args![]);
        }
        let renderer = e.mem.u32(base + 0x3c);
        let state = e.mem.u32(renderer.wrapping_add(0x8b8));
        e.mem.set_u32(base + 0x40, state);
        return;
    }
    let old = e.mem.u32(base + 0x38);
    if old != 0 {
        e.vcall(old, 8, &args![]);
    }
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_u32(base + 0x40, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_1d00, fn_00e81d00(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE_VTABLE: u32 = 0x0e00_0000;
    const ADD_REFERENCE: u32 = 0x0e01_0004;
    const RELEASE: u32 = 0x0e01_0008;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00e81d00_swaps_the_device_and_takes_the_render_state() {
        let mut e = Engine::new();
        e.put_vtable(DEVICE_VTABLE, &[0, ADD_REFERENCE, RELEASE]);
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(RELEASE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let old = e.mem.alloc(4);
        let device = e.mem.alloc(4);
        e.mem.set_u32(old, DEVICE_VTABLE);
        e.mem.set_u32(device, DEVICE_VTABLE);
        let renderer = e.mem.alloc(0x8bc);
        e.mem.set_u32(renderer + 0x288, device);
        e.mem.set_u32(renderer + 0x8b8, 0x0e02_0000);
        let map = e.mem.alloc(0x44);
        e.mem.set_u32(map + 0x38, old);
        e.call(0x00e8_1d00, &args![map, renderer]);
        assert_eq!(e.mem.u32(map + 0x3c), renderer);
        assert_eq!(e.mem.u32(map + 0x38), device);
        assert_eq!(e.mem.u32(map + 0x40), 0x0e02_0000);
        assert_eq!(calls(&e, RELEASE), vec![vec![old]]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![device]]);
        // No renderer: the device released and both words cleared.
        e.call(0x00e8_1d00, &args![map, 0u32]);
        assert_eq!(calls(&e, RELEASE), vec![vec![old], vec![device]]);
        assert_eq!(e.mem.u32(map + 0x38), 0);
        assert_eq!(e.mem.u32(map + 0x40), 0);
        assert_eq!(e.mem.u32(map + 0x3c), 0);
    }
}
