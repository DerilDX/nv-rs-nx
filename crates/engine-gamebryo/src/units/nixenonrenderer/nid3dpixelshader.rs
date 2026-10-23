//! `NiXenonRenderer/nid3dpixelshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiD3DPixelShader`'s vtable (`010ef7d4`, RTTI `.?AVNiD3DPixelShader@@`).
const VTABLE: u32 = 0x010e_f7d4;
/// The shader factory's pixel-shader name removal `00e937c0` (`cdecl`, one word: the shader).
const FACTORY_REMOVE_PIXEL_SHADER: u32 = 0x00e9_37c0;
/// `NiXenonRenderer::PurgePixelShader` (engine map, `00e699c0`, `thiscall` on the renderer:
/// the shader).
const PURGE_PIXEL_SHADER: u32 = 0x00e6_99c0;
/// The base destructor body `00e89840` (unit `nid3dshaderprogram.cpp`, unnamed in the engine
/// map; `thiscall`, no stack words), reached by a jump.
const SHADER_PROGRAM_DESTRUCT: u32 = 0x00e8_9840;

// Translated from 00e89970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `NiD3DPixelShader`'s destructor
/// body. Stores the class's vtable, removes the shader from the factory (`00e937c0`); when
/// the shader handle at +0x2c is set, the renderer at +0x24 purges it
/// (`NiXenonRenderer::PurgePixelShader`); clears +0x2c and ends in the base destructor body
/// `00e89840` (a jump).
pub fn fn_00e89970(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(FACTORY_REMOVE_PIXEL_SHADER, &args![this]);
    if e.mem.u32(base + 0x2c) != 0 {
        let renderer = e.mem.u32(base + 0x24);
        e.call(PURGE_PIXEL_SHADER, &args![renderer, this]);
    }
    e.mem.set_u32(base + 0x2c, 0);
    e.call(SHADER_PROGRAM_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_9970, fn_00e89970(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e89970_purges_a_held_shader_then_runs_the_base() {
        for (handle, purges) in [(0x0e00_7000u32, true), (0, false)] {
            let mut e = Engine::new();
            for callee in [PURGE_PIXEL_SHADER, SHADER_PROGRAM_DESTRUCT] {
                e.register(callee, |_, _| Ret::default());
            }
            // The factory removal sees the vtable stored already.
            e.register(FACTORY_REMOVE_PIXEL_SHADER, |e, a| {
                assert_eq!(e.mem.u32(a[0]), VTABLE);
                Ret::default()
            });
            let this = e.mem.alloc(0x30);
            e.mem.set_u32(this + 0x24, 0x0e00_6000);
            e.mem.set_u32(this + 0x2c, handle);
            e.call_log = Some(vec![]);
            e.call(0x00e8_9970, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(FACTORY_REMOVE_PIXEL_SHADER, vec![this])];
            if purges {
                expected.push((PURGE_PIXEL_SHADER, vec![0x0e00_6000, this]));
            }
            expected.push((SHADER_PROGRAM_DESTRUCT, vec![this]));
            assert_eq!(log[1..].to_vec(), expected);
            assert_eq!(e.mem.u32(this + 0x2c), 0);
        }
    }
}
