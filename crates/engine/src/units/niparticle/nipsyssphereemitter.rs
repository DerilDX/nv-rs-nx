//! `NiParticle/nipsyssphereemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c20020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_sphere_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

// Translated from 00c1fe10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSphereEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2510.
pub fn ni_p_sys_sphere_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2510
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1fe10, ni_p_sys_sphere_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c20020, ni_p_sys_sphere_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_sphere_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00c20020, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c20020, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_sphere_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1_fe10, &args![]).u32(), 0x0120_2510);
    }
}
