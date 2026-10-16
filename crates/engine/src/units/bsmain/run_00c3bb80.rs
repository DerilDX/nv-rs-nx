//! `BSMain/run_00c3bb80` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00aa4c60` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_00AA4C60: u32 = 0x00aa_4c60;
/// The vtable stored at `+0` (RTTI class `MessageHandler`).
const VTABLE_00C3BCF0: u32 = 0x010c_14a8;
/// The global that receives `this` (`.data`).
const INSTANCE_01202D74: u32 = 0x0120_2d74;

// Translated from 00c3bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00aa4c60` on `this`, stores the vtable `010c14a8` (RTTI
/// class `MessageHandler`) at `+0`, the low byte of its stack word at `+4`, and `this` in the
/// global at `01202d74`. Returns `this`.
pub fn fn_00c3bcf0(e: &mut Engine, this: Ptr, flag: u8) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00AA4C60, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_00C3BCF0);
    e.mem.set_u8(this.addr() + 4, flag);
    e.set_global(INSTANCE_01202D74, this.addr());
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_bcf0, fn_00c3bcf0(Ptr, u8) -> Ptr)]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vtable_the_byte_and_the_instance() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.register(BASE_CONSTRUCTOR_00AA4C60, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xaaaa_aaaa);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_bcf0, &args![this, 0x7fu32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00AA4C60),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_00C3BCF0);
        assert_eq!(e.mem.u32(this + 4), 0xaaaa_aa7f);
        assert_eq!(e.global::<u32>(INSTANCE_01202D74), this);
    }
}
