//! `BSHavok/bhkwheelconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cc5110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWheelConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc5090`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_wheel_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_5090;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkWheelConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_85ec.
pub fn bhk_wheel_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_85ec
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc5100, bhk_wheel_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc5110, bhk_wheel_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_wheel_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_5090, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cc5110, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc5110, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_5090), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_wheel_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_5100, &args![]).u32(), 0x0126_85ec);
    }
}
