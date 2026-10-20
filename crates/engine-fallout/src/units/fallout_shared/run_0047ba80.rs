//! `fallout shared/run_0047ba80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0x80 of the byte at +0x100 is clear.
pub fn fn_0047bcf0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x100) & 0x80 == 0
}

// Translated from 0047bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm`'s `InitializeDataComponent` (vtable `0101aef0`, RTTI
/// `BGSTouchSpellForm`): the word at +4 becomes 0 and the 16-bit value at
/// +8 0xFF.
pub fn fn_0047bd40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 8, 0xff);
}

// Translated from 0047bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTouchSpellForm`'s constructor: the `BaseFormComponent` base
/// (`00402d30`), its vtable `0101aef0` and `InitializeDataComponent`
/// (`0047bd40`). Returns `this`.
pub fn fn_0047bd10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_aef0);
    fn_0047bd40(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_bcf0, fn_0047bcf0(Ptr) -> bool),
        entry!(0x0047_bd40, fn_0047bd40(Ptr)),
        entry!(0x0047_bd10, fn_0047bd10(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    #[test]
    fn b020_0047bcf0_tests_bit_0x80() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x104);
        e.mem.set_u8(p + 0x100, 0x7f);
        assert!(e.call(0x0047_bcf0, &args![p]).bool());
        e.mem.set_u8(p + 0x100, 0x80);
        assert!(!e.call(0x0047_bcf0, &args![p]).bool());
    }

    #[test]
    fn b020_0047bd10_constructs_and_0047bd40_initializes() {
        let mut e = Engine::new();
        e.register(0x0040_2d30, |_, _| Ret::default());
        let p = e.mem.alloc(0xc);
        e.mem.set_u32(p + 4, 5);
        e.mem.set_u32(p + 8, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_bd10, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), 0x0101_aef0);
        assert_eq!(e.mem.u32(p + 4), 0);
        assert_eq!(e.mem.u32(p + 8), 0xffff_00ff);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x0040_2d30, vec![p]));
        e.mem.set_u16(p + 8, 0);
        e.call(0x0047_bd40, &args![p]);
        assert_eq!(e.mem.u16(p + 8), 0xff);
    }
}
