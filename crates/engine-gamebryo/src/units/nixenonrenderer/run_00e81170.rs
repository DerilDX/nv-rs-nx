//! `NiXenonRenderer/run_00e81170` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a8f440` (unnamed; `thiscall`, no stack words) and
/// `NiD3DShaderInterface`'s vtable (`010ef39c`, RTTI `.?AVNiD3DShaderInterface@@`).
const BASE_CONSTRUCT_00A8F440: u32 = 0x00a8_f440;
const SHADER_INTERFACE_VTABLE_010EF39C: u32 = 0x010e_f39c;

// Translated from 00e818a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e81170`): the `NiD3DShaderInterface` constructor (the
/// vtable it stores has that class's RTTI), `thiscall`, no stack words: the base constructor
/// `00a8f440`, the words +0x14 to +0x1c and the byte +0x20 cleared, the vtable `010ef39c`.
/// Answers `this`.
pub fn fn_00e818a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00A8F440, &args![base]);
    for offset in [0x14u32, 0x18, 0x1c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x20, 0);
    e.mem.set_u32(base, SHADER_INTERFACE_VTABLE_010EF39C);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e8_18a0, fn_00e818a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00e818a0_builds_the_shader_interface() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_00A8F440, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.mem.write(this, &[0xcc; 0x24]);
        assert_eq!(e.call(0x00e8_18a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_INTERFACE_VTABLE_010EF39C);
        assert_eq!(e.mem.bytes(this + 0x14, 0xd), vec![0; 0xd]);
        assert_eq!(e.mem.u8(this + 0x21), 0xcc);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCT_00A8F440, vec![this]));
    }
}
