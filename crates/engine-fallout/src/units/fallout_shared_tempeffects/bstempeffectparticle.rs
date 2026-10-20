//! `fallout shared/tempeffects/bstempeffectparticle.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00688f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffectParticle` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00689720` and frees 0x74 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_particle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0068_9720, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x74u32]);
    }
    this
}

// Translated from 00688f80 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6b84 without reading the receiver. The map names this body `BSTempEffectParticle::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00688f80(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6b84
}

// Translated from 0068a810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bstempeffectparticle.cpp`). `__fastcall` getter: the dword at
/// `+0x4c` of `this`.
pub fn fn_0068a810(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x4c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_a810, fn_0068a810(Ptr) -> u32),
        entry!(0x0068_8f80, fn_00688f80(Ptr) -> u32),
        entry!(0x0068_8f90, bs_temp_effect_particle_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_temp_effect_particle_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_8f90, 0x0068_9720, 0x74);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00688f80_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_8f80, &args![0x1000u32]).u32(), 0x011d_6b84);
        assert_eq!(e.call(0x0068_8f80, &args![0u32]).u32(), 0x011d_6b84);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0068a810_answers_the_dword_at_offset_0x4c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x48, 1);
        e.mem.set_u32(this + 0x4c, 0x0bad_f00d);
        e.mem.set_u32(this + 0x50, 2);
        assert_eq!(e.call(0x0068_a810, &args![this]).u32(), 0x0bad_f00d);
    }
}
