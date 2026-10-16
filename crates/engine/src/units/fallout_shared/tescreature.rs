//! `fallout shared/tescreature.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The float `fFootWeight` starts from (`01017718`, read with `FLD dword`).
const FOOT_WEIGHT_DEFAULT: u32 = 0x0101_7718;

layout! {
    /// `TESCreature` (Xbox PDB), 0x160 bytes on the PC (the PDB's 0x170 minus the
    /// 0x10 the PC `TESForm` loses). Only the fields used so far; names and
    /// types are the PDB's.
    pub struct TESCreature: 0x160 {
        /// `pSoundCreature` (Xbox PDB; shares its offset with `pCreatureSounds`).
        0x128 pSoundCreature: u32,
        /// `cHandReach` (Xbox PDB).
        0x130 cHandReach: u8,
        /// `fTurnSpeed` (Xbox PDB).
        0x134 fTurnSpeed: f32,
        /// `fBaseScale` (Xbox PDB).
        0x13c fBaseScale: f32,
        /// `pCombatStyle` (Xbox PDB).
        0x140 pCombatStyle: u32,
        /// `pBodyPartData` (Xbox PDB).
        0x144 pBodyPartData: u32,
        /// `eBloodImpactMaterial` (Xbox PDB).
        0x148 eBloodImpactMaterial: u32,
        /// `pImpactDataSet` (Xbox PDB).
        0x14c pImpactDataSet: u32,
        /// `bEmbeddedWeaponChecked` (Xbox PDB).
        0x150 bEmbeddedWeaponChecked: bool,
        /// `eSoundLevel` (Xbox PDB).
        0x154 eSoundLevel: u32,
        /// `pWeaponList` (Xbox PDB).
        0x158 pWeaponList: u32,
        /// `cCanUseHandToHand` (Xbox PDB).
        0x15c cCanUseHandToHand: u8,
    }
}

// Translated from 005f7230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7230 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESCreature::TESCreature` (Xbox PDB): the form constructor
/// `CreateFormOfType` (`00465110`) runs for form type 0x2b (0x160 bytes).
/// `ECX` is the object. It constructs the `TESActorBase` base (`005f75e0`)
/// and the two components at `+0x10c` and `+0x114`, stores the vtable
/// pointers of the object and its components, registers the form type
/// (`004f15a0`), runs `005f7a40`, seeds the AI form at `+0x90` (`0x32`,
/// `1`, `2`, `2` through `0047ef50`, `0047ee00`, `0047eed0`, `0047ef00`),
/// and, when `004516b0` on the object at `011c3f2c` answers false, sets the
/// faction rank of the actor base data at `+0x30` to the object at
/// `011cb550` and `0` (`0047d800`).
pub fn tescreature_tescreature(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x005f_75e0, &args![this]);
    e.call(0x0047_fdc0, &args![base + 0x10c]);
    e.call(0x0048_9a60, &args![base + 0x114]);
    for (offset, vtable) in [
        (0x00, 0x0104_8f5c),
        (0x30, 0x0104_8eec),
        (0x64, 0x0104_8ed8),
        (0x70, 0x0104_8ec4),
        (0x7c, 0x0104_8ea4),
        (0x90, 0x0104_8e84),
        (0xb0, 0x0104_8e6c),
        (0xb8, 0x0104_8e58),
        (0xc4, 0x0104_8e44),
        (0xd0, 0x0104_8e30),
        (0xdc, 0x0104_8e0c),
        (0xf4, 0x0104_8df8),
        (0x100, 0x0104_8dc8),
        (0x104, 0x0104_8db4),
        (0x10c, 0x0104_8d9c),
        (0x114, 0x0104_8d88),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x004f_15a0, &args![this, 0x2bu32]);
    e.call(0x005f_7a40, &args![this]);
    e.call(0x0047_ef50, &args![base + 0x90, 0x32u32, 0u32]);
    e.call(0x0047_ee00, &args![base + 0x90, 1u32, 0u32]);
    e.call(0x0047_eed0, &args![base + 0x90, 2u32, 0u32]);
    e.call(0x0047_ef00, &args![base + 0x90, 2u32, 0u32]);
    let checked = e.global::<u32>(0x011c_3f2c);
    if !e.call(0x0045_16b0, &args![checked]).bool() {
        let faction = e.global::<u32>(0x011c_b550);
        e.call(0x0047_d800, &args![base + 0x30, faction, 0u32]);
    }
    this
}

/// `009dace0` (named `CombatProcedureUseCombatItem::InitLoadGame` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_009DACE0: u32 = 0x009d_ace0;

// Translated from 005fb430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01048fcc): forwards `this` and one pointer to
/// `009dace0` (named `CombatProcedureUseCombatItem::InitLoadGame`; Xbox PDB
/// `?InitLoadGame@CombatProcedureUseCombatItem@@UAAXPAVBGSLoadFormBuffer@@@Z`, void), whose
/// body only calls the empty stub `004534f0` on the same two words. Returns nothing.
pub fn fn_005fb430(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_009DACE0, &args![this, buffer]);
}

// Translated from 005f75e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f75e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESActorBase::TESActorBase` (Xbox PDB): the constructor `TESCreature`'s (`005f7230`) and
/// `TESNPC`'s constructors run first; `ECX` is the object. It constructs the base
/// (`0050f6f0`) and the components at `+0x30`, `+0x64`, `+0x70`, `+0x7c`, `+0x90`, `+0xb0`,
/// `+0xb8`, `+0xc4`, `+0xd0`, `+0xdc`, `+0xf4`, `+0x100` (`005f78e0`) and `+0x104`, then
/// stores the vtable pointers. The exception-unwinding frame is not translated.
pub fn tesactorbase_tesactorbase(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0047_c700, &args![base + 0x30]);
    e.call(0x0048_1610, &args![base + 0x64]);
    e.call(0x0047_bd10, &args![base + 0x70]);
    e.call(0x0048_cfe0, &args![base + 0x7c]);
    e.call(0x0047_ec40, &args![base + 0x90]);
    e.call(0x0048_71d0, &args![base + 0xb0]);
    e.call(0x0047_ff70, &args![base + 0xb8]);
    e.call(0x0047_f9c0, &args![base + 0xc4]);
    e.call(0x0040_2d00, &args![base + 0xd0]);
    e.call(0x0048_8f50, &args![base + 0xdc]);
    e.call(0x0048_cc50, &args![base + 0xf4]);
    fn_005f78e0(e, Ptr::new(base + 0x100));
    e.call(0x0047_5350, &args![base + 0x104]);
    for (offset, vtable) in [
        (0x00, 0x0104_92ac),
        (0x30, 0x0104_923c),
        (0x64, 0x0104_9224),
        (0x70, 0x0104_9210),
        (0x7c, 0x0104_91f0),
        (0x90, 0x0104_91d0),
        (0xb0, 0x0104_91b8),
        (0xb8, 0x0104_91a4),
        (0xc4, 0x0104_9190),
        (0xd0, 0x0104_917c),
        (0xdc, 0x0104_9158),
        (0xf4, 0x0104_9144),
        (0x100, 0x0104_9114),
        (0x104, 0x0104_9100),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    this
}

// Translated from 005f78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the component at `+0x100` of `TESActorBase`: stores its base vtable
/// (`0x0104_9450`) and returns the object. The engine map gives it no name, so the class is
/// unconfirmed.
pub fn fn_005f78e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0104_9450);
    this
}

// Translated from 005f7b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit `0x200` of the actor base data's flags: calls
/// `TESActorBaseData::SetFlagBit` (`0047dd50`, `ECX` = `this`) with the bit `0x200`, the value
/// `clear == 0` and the second flag byte passed through. `ECX` is the actor base data
/// subobject. The engine map gives the function no name.
pub fn fn_005f7b50(e: &mut Engine, this: Ptr, clear: u8, notify: u8) {
    e.call(
        0x0047_dd50,
        &args![this, 0x200u32, u32::from(clear == 0), u32::from(notify)],
    );
}

// Translated from 005f7a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the step of the
/// `TESCreature` constructor (`005f7230`) that gives the creature fields their
/// start values. `ECX` is the object. Field names are the Xbox PDB's, at the PC
/// offset (the PDB offset minus 0x10, which every field set here agrees with):
/// the four bytes at `+0x12c` (`Data`, `CREATURE_DATA`: 0, 0x32, 0x32, 0x32),
/// `cHandReach` 0x20, `fTurnSpeed` 0.0, `fFootWeight` the float at `01017718`
/// (copied through the x87 stack, so a signalling NaN would be quieted),
/// `fBaseScale` 1.0, `eBloodImpactMaterial` 6, `eSoundLevel` 1,
/// `cCanUseHandToHand` 0xff, the other pointers and flags 0. Then it sets bit
/// `0x40` of the actor base data at `+0x30` (`TESActorBaseData::SetFlagBit`,
/// `0047dd50`, with 0x40, 1, 0), runs `005f7b50` on it with 0 and 0, and runs
/// `006ecd40` on the component at `+0xb0` with 0x32.
pub fn fn_005f7a40(e: &mut Engine, this: Ptr<TESCreature>) {
    let base = this.addr();
    e.mem.set_u8(base + 0x12c, 0);
    e.mem.set_u8(base + 0x12d, 0x32);
    e.mem.set_u8(base + 0x12e, 0x32);
    e.mem.set_u8(base + 0x12f, 0x32);
    e.set(this, TESCreature::cHandReach, 0x20);
    e.set(this, TESCreature::pImpactDataSet, 0);
    e.set(this, TESCreature::pWeaponList, 0);
    e.set(this, TESCreature::fTurnSpeed, 0.0);
    let foot_weight = x87_quiet_bits(e.global::<u32>(FOOT_WEIGHT_DEFAULT));
    e.mem.set_u32(base + 0x138, foot_weight);
    e.set(this, TESCreature::fBaseScale, 1.0);
    e.set(this, TESCreature::pSoundCreature, 0);
    e.set(this, TESCreature::pCombatStyle, 0);
    e.set(this, TESCreature::pBodyPartData, 0);
    e.set(this, TESCreature::bEmbeddedWeaponChecked, false);
    e.set(this, TESCreature::eSoundLevel, 1);
    e.set(this, TESCreature::eBloodImpactMaterial, 6);
    e.set(this, TESCreature::cCanUseHandToHand, 0xff);
    e.call(0x0047_dd50, &args![base + 0x30, 0x40u32, 1u32, 0u32]);
    fn_005f7b50(e, Ptr::new(base + 0x30), 0, 0);
    e.call(0x006e_cd40, &args![base + 0xb0, 0x32u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_7230, tescreature_tescreature(Ptr) -> Ptr),
        entry!(0x005f_b430, fn_005fb430(Ptr, Ptr)),
        entry!(0x005f_75e0, tesactorbase_tesactorbase(Ptr) -> Ptr),
        entry!(0x005f_78e0, fn_005f78e0(Ptr) -> Ptr),
        entry!(0x005f_7b50, fn_005f7b50(Ptr, u8, u8)),
        entry!(0x005f_7a40, fn_005f7a40(Ptr<TESCreature>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tescreature_tescreature_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[],
            calls: &[
                (0x005f_75e0, &[A::T(0x0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0x0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0x0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(0x1), A::V(0x0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(0x2), A::V(0x0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
                (0x0047_d800, &[A::T(0x30), A::V(0x1000001), A::V(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }

    #[test]
    fn tescreature_tescreature_skips_the_faction_rank_when_the_check_holds() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_7230,
            size: 0x160,
            globals: &[(0x011c_3f2c, 0x1000000), (0x011c_b550, 0x1000001)],
            returns: &[(0x0045_16b0, 1)],
            calls: &[
                (0x005f_75e0, &[A::T(0)]),
                (0x0047_fdc0, &[A::T(0x10c)]),
                (0x0048_9a60, &[A::T(0x114)]),
                (0x004f_15a0, &[A::T(0), A::V(0x2b)]),
                (0x005f_7a40, &[A::T(0)]),
                (0x0047_ef50, &[A::T(0x90), A::V(0x32), A::V(0)]),
                (0x0047_ee00, &[A::T(0x90), A::V(1), A::V(0)]),
                (0x0047_eed0, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0047_ef00, &[A::T(0x90), A::V(2), A::V(0)]),
                (0x0045_16b0, &[A::V(0x1000000)]),
            ],
            stores: &[
                (0x00, 4, 0x0104_8f5c),
                (0x30, 4, 0x0104_8eec),
                (0x64, 4, 0x0104_8ed8),
                (0x70, 4, 0x0104_8ec4),
                (0x7c, 4, 0x0104_8ea4),
                (0x90, 4, 0x0104_8e84),
                (0xb0, 4, 0x0104_8e6c),
                (0xb8, 4, 0x0104_8e58),
                (0xc4, 4, 0x0104_8e44),
                (0xd0, 4, 0x0104_8e30),
                (0xdc, 4, 0x0104_8e0c),
                (0xf4, 4, 0x0104_8df8),
                (0x100, 4, 0x0104_8dc8),
                (0x104, 4, 0x0104_8db4),
                (0x10c, 4, 0x0104_8d9c),
                (0x114, 4, 0x0104_8d88),
            ],
        });
    }

    #[test]
    fn forwarder_fn_005fb430_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_009DACE0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_b430, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_009DACE0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesactorbase_tesactorbase_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_75e0,
            size: 0x110,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0047_c700, &[A::T(0x30)]),
                (0x0048_1610, &[A::T(0x64)]),
                (0x0047_bd10, &[A::T(0x70)]),
                (0x0048_cfe0, &[A::T(0x7c)]),
                (0x0047_ec40, &[A::T(0x90)]),
                (0x0048_71d0, &[A::T(0xb0)]),
                (0x0047_ff70, &[A::T(0xb8)]),
                (0x0047_f9c0, &[A::T(0xc4)]),
                (0x0040_2d00, &[A::T(0xd0)]),
                (0x0048_8f50, &[A::T(0xdc)]),
                (0x0048_cc50, &[A::T(0xf4)]),
                (0x0047_5350, &[A::T(0x104)]),
            ],
            stores: &[
                (0x00, 4, 0x0104_92ac),
                (0x30, 4, 0x0104_923c),
                (0x64, 4, 0x0104_9224),
                (0x70, 4, 0x0104_9210),
                (0x7c, 4, 0x0104_91f0),
                (0x90, 4, 0x0104_91d0),
                (0xb0, 4, 0x0104_91b8),
                (0xb8, 4, 0x0104_91a4),
                (0xc4, 4, 0x0104_9190),
                (0xd0, 4, 0x0104_917c),
                (0xdc, 4, 0x0104_9158),
                (0xf4, 4, 0x0104_9144),
                // `005f78e0` stores 0x0104_9450 first; the constructor overwrites it.
                (0x100, 4, 0x0104_9114),
                (0x104, 4, 0x0104_9100),
            ],
        });
    }

    #[test]
    fn fn_005f78e0_stores_the_vtable() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_78e0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[],
            stores: &[(0x0, 4, 0x0104_9450)],
        });
    }

    #[test]
    fn fn_005f7b50_sets_the_flag_bit_from_the_negated_argument() {
        for (clear, notify, want) in [
            (0u32, 1u32, 1u32),
            (1, 0, 0),
            (0x100, 1, 1),
            (0x1_0001, 0, 0),
        ] {
            let mut e = Engine::new();
            e.register(0x0047_dd50, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x005f_7b50, &[0x5000, clear, notify]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1..],
                [(0x0047_dd50, vec![0x5000, 0x200, want, notify & 0xff])],
                "clear {clear:#x}"
            );
        }
    }
}

#[cfg(test)]
mod start_value_tests {
    use super::*;

    fn run(float_bits: u32) -> (Engine, u32, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(FOOT_WEIGHT_DEFAULT, float_bits);
        for address in [0x0047_dd50, 0x006e_cd40] {
            e.register(address, |_, _| Ret::default());
        }
        let creature = e.mem.alloc(0x160);
        e.mem.write(creature, &[0xcd; 0x160]);
        e.call_log = Some(vec![]);
        e.call(0x005f_7a40, &args![creature]);
        let log = e.call_log.take().unwrap();
        (e, creature, log.into_iter().skip(1).collect())
    }

    #[test]
    fn fn_005f7a40_sets_the_start_values_and_calls_the_components() {
        let (e, c, log) = run(0x4040_0000);
        let bytes = |offset: u32| e.mem.u8(c + offset);
        let word = |offset: u32| e.mem.u32(c + offset);
        assert_eq!(
            [bytes(0x12c), bytes(0x12d), bytes(0x12e), bytes(0x12f)],
            [0, 0x32, 0x32, 0x32]
        );
        assert_eq!(bytes(0x130), 0x20);
        assert_eq!(bytes(0x150), 0);
        assert_eq!(bytes(0x15c), 0xff);
        assert_eq!(word(0x134), 0);
        assert_eq!(word(0x138), 0x4040_0000);
        assert_eq!(word(0x13c), 0x3f80_0000);
        for offset in [0x128, 0x140, 0x144, 0x14c, 0x158] {
            assert_eq!(word(offset), 0, "{offset:#x}");
        }
        assert_eq!(word(0x148), 6);
        assert_eq!(word(0x154), 1);
        // Bytes the function never names keep their value.
        assert_eq!(bytes(0x131), 0xcd);
        assert_eq!(bytes(0x15d), 0xcd);
        assert_eq!(log[0], (0x0047_dd50, vec![c + 0x30, 0x40, 1, 0]));
        assert_eq!(log[1], (0x0047_dd50, vec![c + 0x30, 0x200, 1, 0]));
        assert_eq!(log[2], (0x006e_cd40, vec![c + 0xb0, 0x32]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn fn_005f7a40_quiets_a_signalling_nan_foot_weight() {
        let (e, c, _) = run(0x7f80_0001);
        assert_eq!(e.mem.u32(c + 0x138), 0x7fc0_0001);
    }
}
