//! `fallout shared/tempeffects/bstempeffect.cpp` (Xbox PDB source unit), subsystem `fallout shared/tempeffects`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSTempEffect` (Xbox PDB), 0x18 bytes on both builds (the scalar deleting
    /// destructor frees 0x18): an `NiObject` and the effect's timing.
    pub struct BSTempEffect: 0x18 {
        /// `fLifetime` (Xbox PDB).
        0x08 fLifetime: u32,
        /// `pCell` (Xbox PDB): `TESObjectCELL*`.
        0x0C pCell: u32,
        /// `fAge` (Xbox PDB).
        0x10 fAge: u32,
        /// `bInitialized` (Xbox PDB).
        0x14 bInitialized: u8,
    }
}

/// `BSTempEffect`'s vtable (`01069754`), stored by the destructor.
const BS_TEMP_EFFECT_VTABLE: u32 = 0x0106_9754;
/// `NiObject`'s destructor body (`00a5d3d0`, `thiscall`).
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;

// Translated from 00682860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSTempEffect` destructor body (unnamed in the engine map; the scalar deleting
/// destructor `00682830` and the derived destructors call it; `thiscall`): stores the
/// class's vtable, clears `pCell`, `fLifetime`, `fAge` (0.0) and `bInitialized`, then
/// runs `NiObject`'s destructor (`00a5d3d0`).
pub fn bs_temp_effect_destructor(e: &mut Engine, this: Ptr<BSTempEffect>) {
    e.mem.set_u32(this.addr(), BS_TEMP_EFFECT_VTABLE);
    e.set(this, BSTempEffect::pCell, 0);
    e.set(this, BSTempEffect::fLifetime, 0);
    e.set(this, BSTempEffect::fAge, 0);
    e.set(this, BSTempEffect::bInitialized, 0);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 006828a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed time): adds the
/// time to `fAge` and answers whether the effect is still alive: true unless
/// `fLifetime < fAge` (`FCOMPP`, `TEST AH,5`, `JP`: equal ages and NaN answer true).
pub fn bs_temp_effect_update(e: &mut Engine, this: Ptr<BSTempEffect>, seconds: u32) -> bool {
    let age = x87_load32(e.get(this, BSTempEffect::fAge));
    let age = x87_store32(x87_add(age, x87_m32(seconds)));
    e.set(this, BSTempEffect::fAge, age);
    let age = x87_load32(e.get(this, BSTempEffect::fAge));
    let lifetime = x87_load32(e.get(this, BSTempEffect::fLifetime));
    !(lifetime < age)
}

// Translated from 00682830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTempEffect` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00682860` and frees 0x18 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_temp_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0068_2860, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00682800 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 011d6a44 without reading the receiver. The map names this body `BSTempEffect::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00682800(_e: &mut Engine, _this: Ptr) -> u32 {
    0x011d_6a44
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_2800, fn_00682800(Ptr) -> u32),
        entry!(0x0068_2830, bs_temp_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0068_2860, bs_temp_effect_destructor(Ptr<BSTempEffect>)),
        entry!(
            0x0068_28a0,
            bs_temp_effect_update(Ptr<BSTempEffect>, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod temp_effect_tests {
    use super::*;

    #[test]
    fn destructor_clears_the_timing_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xee; 0x18]);
        e.call_log = Some(vec![]);
        e.call(0x0068_2860, &[this]);
        assert_eq!(e.mem.u32(this), BS_TEMP_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_eeee);
        for offset in [8u32, 0xc, 0x10] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x15), 0xee);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_OBJECT_DESTRUCTOR, vec![this]));
    }

    #[test]
    fn update_ages_and_answers_alive_until_past_the_lifetime() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_f32(this + 0x10, 0.25);
        assert!(e.call(0x0068_28a0, &[this, 0.5f32.to_bits()]).bool());
        assert_eq!(e.mem.f32(this + 0x10), 0.75);
        // Reaching the lifetime exactly is still alive.
        assert!(e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
        assert!(!e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
        assert_eq!(e.mem.f32(this + 0x10), 1.25);
        // A NaN lifetime answers alive.
        e.mem.set_f32(this + 8, f32::NAN);
        assert!(e.call(0x0068_28a0, &[this, 0.25f32.to_bits()]).bool());
    }
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
    fn bs_temp_effect_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0068_2830, 0x0068_2860, 0x18);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00682800_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0068_2800, &args![0x1000u32]).u32(), 0x011d_6a44);
        assert_eq!(e.call(0x0068_2800, &args![0u32]).u32(), 0x011d_6a44);
    }
}
