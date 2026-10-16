//! `BSHavok/bhkragdolltemplate.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cb9b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallAndSocketConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc9640`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ball_and_socket_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_9640;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cb9c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc66b0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_66b0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cba6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cba1d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_template_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cb_a1d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

// Translated from 00cbb130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cbaf90`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x1c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_ragdoll_template_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cb_af90;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00cb9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBallAndSocketConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_864c.
pub fn bhk_ball_and_socket_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_864c
}

// Translated from 00cb9c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8610.
pub fn bhk_ragdoll_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8610
}

// Translated from 00cba1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplateData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8400.
pub fn bhk_ragdoll_template_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8400
}

// Translated from 00cbaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollTemplate::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8408.
pub fn bhk_ragdoll_template_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8408
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb9b50, bhk_ball_and_socket_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cb9c00, bhk_ragdoll_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cba1c0, bhk_ragdoll_template_data_get_r_t_t_i() -> u32),
        entry!(0x00cbaf80, bhk_ragdoll_template_get_r_t_t_i() -> u32),
        entry!(0x00cb9b60, bhk_ball_and_socket_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cb9c10, bhk_ragdoll_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cba6d0, bhk_ragdoll_template_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cbb130, bhk_ragdoll_template_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_ball_and_socket_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_9640, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cb9b60, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cb9b60, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_9640), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_66b0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cb9c10, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cb9c10, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_66b0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_template_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cb_a1d0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cba6d0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cba6d0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00cb_a1d0), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ragdoll_template_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cb_af90, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cbb130, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cbb130, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x1c]]);
        assert_eq!(calls_to(&e, 0x00cb_af90), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_ball_and_socket_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_9b50, &args![]).u32(), 0x0126_864c);
    }

    #[test]
    fn bhk_ragdoll_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_9c00, &args![]).u32(), 0x0126_8610);
    }

    #[test]
    fn bhk_ragdoll_template_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_a1c0, &args![]).u32(), 0x0126_8400);
    }

    #[test]
    fn bhk_ragdoll_template_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_af80, &args![]).u32(), 0x0126_8408);
    }
}
