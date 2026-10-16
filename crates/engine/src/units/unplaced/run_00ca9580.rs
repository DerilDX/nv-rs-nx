//! `(unplaced)/run_00ca9580` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Two floats the constructor copies from `.rdata` (`FLD`/`FSTP`, so a
/// signalling NaN comes out quiet): stored at +0x20 and +0x1C.
const FLOAT_AT_0X20: u32 = 0x0101_7868;
const FLOAT_AT_0X1C: u32 = 0x010c_8190;
/// The vtable the constructor installs.
const VTABLE: u32 = 0x010c_8198;
/// `__thiscall` call the constructor ends with, on the object stored at +0xC
/// (`00c90510`, `hkreferencedobject.cpp`, no arguments).
const REFERENCED_OBJECT_ADD_REFERENCE: u32 = 0x00c9_0510;

// Translated from 00ca9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Constructor (`__thiscall`, two arguments): sets the word at
/// +6 to 1, +0xC to `owner`, the vtable, +0x10 and +0x14 to `value`, +8 to 0,
/// the byte at +0x18 to 1, the float at +0x20 from `01017868`, the float at
/// +0x1C from `010c8190` and the float at +0x24 to 1.0; then calls `00c90510`
/// on the object at +0xC (`owner`). Returns `this`.
pub fn fn_00ca9580(e: &mut Engine, this: Ptr, owner: u32, value: u32) -> Ptr {
    let base = this.addr();
    let first = x87_quiet_bits(e.global::<u32>(FLOAT_AT_0X20));
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 0xc, owner);
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base + 0x10, value);
    e.mem.set_u32(base + 0x14, value);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0x18, 1);
    e.mem.set_u32(base + 0x20, first);
    let second = x87_quiet_bits(e.global::<u32>(FLOAT_AT_0X1C));
    let referenced = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 0x1c, second);
    e.mem.set_u32(base + 0x24, 1.0f32.to_bits());
    e.call(REFERENCED_OBJECT_ADD_REFERENCE, &args![referenced]);
    this
}

// Translated from 00ca9600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall` getter: the word at `this + 0x10`.
pub fn fn_00ca9600(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x10)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca9580, fn_00ca9580(Ptr, u32, u32) -> Ptr),
        entry!(0x00ca9600, fn_00ca9600(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ca9580_fills_the_fields_and_calls_the_owner() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x010c_8000, 0x1000);
        // A signalling NaN comes out quiet; the other value is kept.
        e.set_global(FLOAT_AT_0X20, 0x7f80_0001u32);
        e.set_global(FLOAT_AT_0X1C, 624.0f32.to_bits());
        e.register(REFERENCED_OBJECT_ADD_REFERENCE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x28);
        for offset in 0..10 {
            e.mem.set_u32(this + 4 * offset, 0xffff_ffff);
        }
        let result = e.call(
            0x00ca_9580,
            &args![Ptr::<()>::new(this), 0x6000u32, 0x7000u32],
        );
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x6000);
        assert_eq!(e.mem.u32(this + 0x10), 0x7000);
        assert_eq!(e.mem.u32(this + 0x14), 0x7000);
        assert_eq!(e.mem.u8(this + 0x18), 1);
        assert_eq!(e.mem.u8(this + 0x19), 0xff);
        assert_eq!(e.mem.u32(this + 0x1c), 624.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x24), 1.0f32.to_bits());
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(REFERENCED_OBJECT_ADD_REFERENCE, vec![0x6000])));
    }

    #[test]
    fn fn_00ca9600_reads_the_word_at_0x10() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0xabcd_1234);
        assert_eq!(
            e.call(0x00ca_9600, &args![Ptr::<()>::new(this)]).u32(),
            0xabcd_1234
        );
    }
}
