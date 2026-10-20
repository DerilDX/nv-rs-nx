//! `fallout shared/region system/run_004f6de0` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00eca190`: a CRT math routine taking a `double` on the stack and
/// returning in `ST0` (Ghidra's library match names it `cos`; unconfirmed).
const DOUBLE_MATH_00ECA190: u32 = 0x00ec_a190;

// Translated from 004f6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 5` (`ECX` only, no stack words).
pub fn fn_004f6de0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 5)
}

// Translated from 004f6e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `float` wrapper of the `double` routine `00eca190` (cdecl, one word):
/// widens `value` (`FLD dword`, `FSTP qword`), calls it, and rounds the
/// result to a `float` (`FSTP dword`, then `FLD dword` into `ST0`). Ghidra's
/// library matcher calls this `logf` and the callee `cos`; neither is
/// confirmed.
pub fn fn_004f6e40(e: &mut Engine, value: f32) -> f32 {
    let widened = x87_load32(value.to_bits());
    let result = e.call(DOUBLE_MATH_00ECA190, &args![widened]).f64();
    x87_quiet(f32::from_bits(x87_store32(result)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_6de0, fn_004f6de0(Ptr) -> u8),
        entry!(0x004f_6e40, fn_004f6e40(f32) -> f32),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_004f6de0_reads_the_byte_at_5() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.mem.set_u8(object + 5, 0x9c);
        assert_eq!(e.call(0x004f_6de0, &args![object]).u32() & 0xff, 0x9c);
    }

    #[test]
    fn fn_004f6e40_passes_the_widened_value_and_rounds_the_result() {
        let mut e = Engine::new();
        e.register(DOUBLE_MATH_00ECA190, |_, a| {
            let value = f64::take(a, &mut 0);
            (value / 3.0).into_ret()
        });
        e.call_log = Some(vec![]);
        let back = e.call(0x004f_6e40, &args![1.0f32]).f32();
        assert_eq!(back, (1.0f64 / 3.0) as f32);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == DOUBLE_MATH_00ECA190).collect();
        assert_eq!(inner[0].1, args![1.0f64]);
        // A signalling NaN argument reaches the routine quiet.
        e.call_log = Some(vec![]);
        e.call(0x004f_6e40, &[0x7f80_0001]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == DOUBLE_MATH_00ECA190).collect();
        let bits = u64::from(inner[0].1[0]) | u64::from(inner[0].1[1]) << 32;
        assert_eq!(bits, 0x7ff8_0000_2000_0000);
    }
}
