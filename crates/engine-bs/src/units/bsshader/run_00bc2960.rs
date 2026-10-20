//! `BSShader/run_00bc2960` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The vtable this constructor sets (`010ba608`; no RTTI locator before it).
const VTABLE_010BA608: u32 = 0x010b_a608;
/// The `float`s copied to `+0xb4` (`01016410`) and `+0xb0` (`010ba600`).
const VALUE_FOR_B4_01016410: u32 = 0x0101_6410;
const VALUE_FOR_B0_010BA600: u32 = 0x010b_a600;

// Translated from 00bc2980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`BSShader`); a constructor of an `NiNode` subclass whose class
/// is not confirmed (its vtable `010ba608` has no RTTI locator). `thiscall`, no stack words;
/// answers `this`. Constructs the `NiNode` (child array size 0), copies the `float`s at
/// `01016410` to `+0xb4` and `010ba600` to `+0xb0` (`FLD`/`FSTP`: a signalling NaN comes out
/// quiet), sets the vtable and clears the bytes at `+0xad` and `+0xac`.
pub fn fn_00bc2980(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    let first = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B4_01016410));
    e.mem.set_u32(base.wrapping_add(0xb4), first);
    e.mem.set_u32(base, VTABLE_010BA608);
    let second = x87_quiet_bits(e.global::<u32>(VALUE_FOR_B0_010BA600));
    e.mem.set_u8(base.wrapping_add(0xad), 0);
    e.mem.set_u32(base.wrapping_add(0xb0), second);
    e.mem.set_u8(base.wrapping_add(0xac), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00bc_2980, fn_00bc2980(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc2980_constructs_the_node_and_its_fields() {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x010b_a000, 0x1000);
        e.set_global(VALUE_FOR_B4_01016410, 100.0f32);
        e.set_global(VALUE_FOR_B0_010BA600, 0x7f80_0003u32);
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xb8);
        e.mem.set_u8(this + 0xac, 0xff);
        e.mem.set_u8(this + 0xad, 0xff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00bc_2980, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let node: Vec<_> = log.iter().filter(|c| c.0 == NI_NODE_CONSTRUCT).collect();
        assert_eq!(node[0].1, vec![this, 0]);
        assert_eq!(e.mem.u32(this), VTABLE_010BA608);
        assert_eq!(e.mem.f32(this + 0xb4), 100.0);
        assert_eq!(e.mem.u32(this + 0xb0), 0x7fc0_0003);
        assert_eq!(e.mem.u8(this + 0xac), 0);
        assert_eq!(e.mem.u8(this + 0xad), 0);
    }
}
