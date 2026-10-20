//! `BSSystem/run_00b006b0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00afeac0` (`thiscall`, `RET 4`): opens the file named by its argument for the object and
/// returns the word stored at +0.
const OPEN_FOR: u32 = 0x00af_eac0;
/// `00aff090` (`thiscall`, no stack word): closes the handle at +0x10.
const CLOSE_FOR: u32 = 0x00af_f090;

// Translated from 00b00900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`): a constructor. Clears the word at +0,
/// stores `second` at +4, `third` at +8 and the low byte of `fourth` at +0xc, then stores at
/// +0 what `00afeac0` returns for `first`. Returns `this`. (What `00afeac0` does with `first`
/// is not confirmed here.)
pub fn fn_00b00900(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, second);
    e.mem.set_u32(base + 8, third);
    e.mem.set_u8(base + 0xc, fourth as u8);
    let result = e.call(OPEN_FOR, &args![this, first]).u32();
    e.mem.set_u32(base, result);
    this
}

// Translated from 00b00950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `00aff090` on `this`.
pub fn fn_00b00950(e: &mut Engine, this: Ptr) {
    e.call(CLOSE_FOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0900, fn_00b00900(Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00b0_0950, fn_00b00950(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_the_words_then_the_open_result() {
        let mut e = Engine::new();
        e.register(OPEN_FOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0);
            0x99u32.into_ret()
        });
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, 0x55);
        e.call_log = Some(vec![]);
        let made = e
            .call(
                0x00b0_0900,
                &args![this, 0x10u32, 0x20u32, 0x30u32, 0x1234_5601u32],
            )
            .u32();
        assert_eq!(made, this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(OPEN_FOR, vec![this, 0x10])]);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)],
            [0x99, 0x20, 0x30]
        );
        assert_eq!(e.mem.u8(this + 0xc), 1);
    }

    #[test]
    fn fn_00b00950_closes_through_00aff090() {
        let mut e = Engine::new();
        e.register(CLOSE_FOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00b0_0950, &args![0x7000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(CLOSE_FOR, vec![0x7000])]);
    }
}
