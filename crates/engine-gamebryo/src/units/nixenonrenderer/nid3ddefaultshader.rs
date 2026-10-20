//! `NiXenonRenderer/nid3ddefaultshader.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00e818a0` (unnamed; `thiscall`, no stack words), the fixed string
/// maker `00a5b690` (unnamed; `cdecl`, a C string; answers the handle), the empty fixed string
/// (`0109b220`), `InterlockedDecrement` (import slot), the shader's name
/// (`"Default Shader From Gamebryo 2.1"` at `010eee94`, read in Ghidra) and
/// `NiD3DDefaultShader`'s vtable (`010eecac`, RTTI `.?AVNiD3DDefaultShader@@`).
const BASE_CONSTRUCT_00E818A0: u32 = 0x00e8_18a0;
const FIXED_STRING_MAKE_00A5B690: u32 = 0x00a5_b690;
const FIXED_STRING_EMPTY_0109B220: u32 = 0x0109_b220;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const DEFAULT_NAME_010EEE94: u32 = 0x010e_ee94;
const DEFAULT_SHADER_VTABLE_010EECAC: u32 = 0x010e_ecac;

// Translated from 00e7c6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DDefaultShader::NiD3DDefaultShader` (engine map), `thiscall`, no stack words. Runs the
/// base constructor `00e818a0`, clears the bytes +0x24 and +0x25 and the words +0x28 to +0x38,
/// stores the vtable `010eecac`, clears +0x50 to +0x58, and names the shader: when the name
/// handle at +8 is not the address `010eee94` itself, +8 gets a new fixed string of that text
/// (`00a5b690`) and the old handle, unless it is the empty fixed string, is released
/// (`InterlockedDecrement` on the handle - 8). Sets the byte +0x20 and answers `this`.
pub fn ni_d3d_default_shader_ni_d3d_default_shader(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00E818A0, &args![base]);
    e.mem.set_u8(base + 0x24, 0);
    e.mem.set_u8(base + 0x25, 0);
    for offset in [0x28u32, 0x2c, 0x30, 0x34, 0x38] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base, DEFAULT_SHADER_VTABLE_010EECAC);
    for offset in [0x50u32, 0x54, 0x58] {
        e.mem.set_u32(base + offset, 0);
    }
    let old = e.mem.u32(base + 8);
    if old != DEFAULT_NAME_010EEE94 {
        let name = e
            .call(FIXED_STRING_MAKE_00A5B690, &args![DEFAULT_NAME_010EEE94])
            .u32();
        e.mem.set_u32(base + 8, name);
        if old != e.global::<u32>(FIXED_STRING_EMPTY_0109B220) {
            e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
        }
    }
    e.mem.set_u8(base + 0x20, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_c6b0, ni_d3d_default_shader_ni_d3d_default_shader(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.set_global(FIXED_STRING_EMPTY_0109B220, 0x0e00_0008u32);
        e.register(BASE_CONSTRUCT_00E818A0, |_, a| a[0].into_ret());
        e.register(FIXED_STRING_MAKE_00A5B690, |_, _| 0x0e10_0008u32.into_ret());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn default_shader_constructor_names_the_shader() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.mem.write(this, &[0xcc; 0x5c]);
        e.mem.set_u32(this + 8, 0x0e20_0008);
        assert_eq!(e.call(0x00e7_c6b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), DEFAULT_SHADER_VTABLE_010EECAC);
        assert_eq!(e.mem.u32(this + 8), 0x0e10_0008);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.bytes(this + 0x24, 2), vec![0; 2]);
        assert_eq!(e.mem.bytes(this + 0x28, 0x14), vec![0; 0x14]);
        assert_eq!(e.mem.bytes(this + 0x50, 0xc), vec![0; 0xc]);
        assert_eq!(
            calls(&e, FIXED_STRING_MAKE_00A5B690),
            vec![vec![DEFAULT_NAME_010EEE94]]
        );
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0e20_0000]]
        );
    }

    #[test]
    fn default_shader_constructor_keeps_the_empty_string_and_the_name_itself() {
        let mut e = engine();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 8, 0x0e00_0008);
        e.call(0x00e7_c6b0, &args![this]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        e.mem.set_u32(this + 8, DEFAULT_NAME_010EEE94);
        e.call(0x00e7_c6b0, &args![this]);
        assert_eq!(e.mem.u32(this + 8), DEFAULT_NAME_010EEE94);
        assert_eq!(calls(&e, FIXED_STRING_MAKE_00A5B690).len(), 1);
    }
}
