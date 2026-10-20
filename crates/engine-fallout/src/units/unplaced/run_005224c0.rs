//! `(unplaced)/run_005224c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0046e8c0` (`tesdatahandler.cpp`; thiscall on `this`, a `bool` in AL).
const CHECK_0046E8C0: u32 = 0x0046_e8c0;
/// Reads the word at its `this` (`00559450`, thiscall).
const READ_POINTER: u32 = 0x0055_9450;
/// `004fb7a0` (`bgsexplosion.cpp`; thiscall on the object at +0x24c, one
/// word).
const CALLEE_004FB7A0: u32 = 0x004f_b7a0;
/// The fields read (offsets as the code reads them).
const OBJECT_24C: u32 = 0x24c;
const HOLDER_35C: u32 = 0x35c;
const BYTE_11F: u32 = 0x11f;

// Translated from 005224c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x22c of `this` (unnamed in the engine map).
pub fn fn_005224c0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x22c)
}

// Translated from 00522500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x234 of `this` (unnamed in the engine map).
pub fn fn_00522500(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x234)
}

// Translated from 00522520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x238 of `this` (unnamed in the engine map).
pub fn fn_00522520(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x238)
}

// Translated from 00522540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x23c of `this` (unnamed in the engine map).
pub fn fn_00522540(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x23c)
}

// Translated from 00522ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004fb7a0(object, value)` on the object at +0x24c when there is one,
/// else 0 (thiscall, `RET 4`; unnamed in the engine map).
pub fn fn_00522ba0(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let object = e.mem.u32(this.addr() + OBJECT_24C);
    if object == 0 {
        return 0;
    }
    e.call(CALLEE_004FB7A0, &args![object, value]).u32()
}

// Translated from 00522c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `0046e8c0(this)` answers true, the pointer held at +0x35c (read by
/// `00559450`); else 0 (unnamed in the engine map).
pub fn fn_00522c00(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(CHECK_0046E8C0, &args![this]).u32() & 0xff == 0 {
        return 0;
    }
    e.call(READ_POINTER, &args![this.addr() + HOLDER_35C]).u32()
}

// Translated from 00522c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `-1` unless `0046e8c0(this)` answers true; then the byte at +0x11f plus
/// 0x19, at most 0x1f (a signed comparison with 0x20; unnamed in the
/// engine map).
pub fn fn_00522c30(e: &mut Engine, this: Ptr) -> u32 {
    let mut value = u32::MAX;
    if e.call(CHECK_0046E8C0, &args![this]).u32() & 0xff != 0 {
        value = u32::from(e.mem.u8(this.addr() + BYTE_11F)) + 0x19;
        if value as i32 >= 0x20 {
            value = 0x1f;
        }
    }
    value
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0052_24c0, fn_005224c0(Ptr) -> u32),
        entry!(0x0052_2500, fn_00522500(Ptr) -> u32),
        entry!(0x0052_2520, fn_00522520(Ptr) -> u32),
        entry!(0x0052_2540, fn_00522540(Ptr) -> u32),
        entry!(0x0052_2ba0, fn_00522ba0(Ptr, u32) -> u32),
        entry!(0x0052_2c00, fn_00522c00(Ptr) -> u32),
        entry!(0x0052_2c30, fn_00522c30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_getters_read_their_offsets() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x240);
        for offset in [0x22cu32, 0x234, 0x238, 0x23c] {
            e.mem.set_u32(this + offset, 0x1000 + offset);
        }
        assert_eq!(e.call(0x0052_24c0, &args![this]).u32(), 0x122c);
        assert_eq!(e.call(0x0052_2500, &args![this]).u32(), 0x1234);
        assert_eq!(e.call(0x0052_2520, &args![this]).u32(), 0x1238);
        assert_eq!(e.call(0x0052_2540, &args![this]).u32(), 0x123c);
    }

    #[test]
    fn fn_00522ba0_asks_the_object_when_there_is_one() {
        let mut e = Engine::new();
        e.register(CALLEE_004FB7A0, |_, a| (a[0] ^ a[1]).into_ret());
        let this = e.mem.alloc(0x250);
        assert_eq!(e.call(0x0052_2ba0, &args![this, 5u32]).u32(), 0);
        e.mem.set_u32(this + OBJECT_24C, 0x0700_0000);
        assert_eq!(e.call(0x0052_2ba0, &args![this, 5u32]).u32(), 0x0700_0005);
    }

    #[test]
    fn fn_00522c00_reads_the_holder_only_when_the_check_passes() {
        let mut e = Engine::new();
        e.register(CHECK_0046E8C0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(READ_POINTER, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x360);
        e.mem.set_u32(this + HOLDER_35C, 0x0abc_0000);
        e.mem.set_u32(this, 0xffff_ff00);
        assert_eq!(e.call(0x0052_2c00, &args![this]).u32(), 0);
        e.mem.set_u32(this, 1);
        assert_eq!(e.call(0x0052_2c00, &args![this]).u32(), 0x0abc_0000);
    }

    #[test]
    fn fn_00522c30_adds_0x19_and_caps_at_0x1f() {
        let mut e = Engine::new();
        e.register(CHECK_0046E8C0, |e, a| e.mem.u32(a[0]).into_ret());
        let this = e.mem.alloc(0x120);
        assert_eq!(e.call(0x0052_2c30, &args![this]).u32(), u32::MAX);
        e.mem.set_u32(this, 1);
        for (byte, expected) in [(0u8, 0x19u32), (6, 0x1f), (7, 0x1f), (0xff, 0x1f)] {
            e.mem.set_u8(this + BYTE_11F, byte);
            assert_eq!(e.call(0x0052_2c30, &args![this]).u32(), expected);
        }
    }
}
