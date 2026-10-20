//! `(unplaced)/run_005f7460` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks `this` through its vtable slot +8 with `value` and answers the
/// integer result as the exe's `FILD` leaves it in ST0 (exact, so
/// answered as a `double`; the engine map has no name).
pub fn fn_005f7480(e: &mut Engine, this: Ptr, value: u32) -> f64 {
    let result = e.vcall(this.addr(), 8, &args![value]).i32();
    f64::from(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_7480, fn_005f7480(Ptr, u32) -> f64)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_8_result_is_loaded_as_an_integer() {
        const TARGET: u32 = 0x0300_0000;
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, TARGET);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.register(TARGET, |_, a| (a[1].wrapping_neg()).into_ret());
        assert_eq!(e.call(0x005f_7480, &args![this, 7u32]).f64(), -7.0);
        assert_eq!(
            e.call(0x005f_7480, &args![this, 0x7fff_ffffu32]).f64(),
            -2_147_483_647.0
        );
    }
}
