//! `fallout shared/aiformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFollowerDist@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0104F3E8: u32 = 0x0104_f3e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006442e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFollowerDist@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006442e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0104F3E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `00403e20` (`thiscall` on a setting): the address of its value (+4), or of
/// a zeroed scratch `float` without a setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00406ce0` (`cdecl`, one `float`, result in ST0; no Xbox PDB name).
const FLOAT_STEP_00406CE0: u32 = 0x0040_6ce0;
/// `004e4450` (`cdecl`, one `float`, result in ST0; no Xbox PDB name).
const FLOAT_STEP_004E4450: u32 = 0x004e_4450;
/// `0047f060` (`cdecl`, the actor value; returns a `bool` in AL; no Xbox
/// PDB name).
const ACCEPTS_ACTOR_VALUE_0047F060: u32 = 0x0047_f060;
/// `00643bf0` (`cdecl`: actor value, `int*`, `float*`; unit
/// `(unplaced)/run_00642860`): a word from the table at `0119b144` and the value of the
/// setting from the table at `0119b17c`, by actor value.
const ENTRY_00643BF0: u32 = 0x0064_3bf0;
/// `ActorValueOwner::GetClampedActorFloatValue` (`0066ef50`, engine map;
/// `thiscall`, one word; result in ST0).
const GET_CLAMPED_ACTOR_FLOAT_VALUE: u32 = 0x0066_ef50;
/// `ActorValue::ClampActorValue` (`0066f190`, engine map; `cdecl`: actor
/// value, `float`; result in ST0).
const CLAMP_ACTOR_VALUE: u32 = 0x0066_f190;
/// `Actor::GetClass` (`00884350`, engine map; `thiscall`, no stack words).
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
/// `005a5f40` (`thiscall` on the class, one word: the actor value; returns
/// a `bool` in AL; no Xbox PDB name).
const CLASS_TEST_005A5F40: u32 = 0x005a_5f40;

/// The address of a setting's value (`00403e20`).
fn setting(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// `FLD dword [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

// Translated from 00643c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::CalculateDerivedSkillValue` (Xbox PDB, matched by the engine
/// map; `cdecl`, result in ST0): `base + 00406ce0(first * s[11cdb30]) +
/// 004e4450(second * s[11cdd84])`, each sum stored as a `float` (`s[..]` is
/// the value of the setting at that address).
pub fn ai_formulas_calculate_derived_skill_value(
    e: &mut Engine,
    base: f32,
    first: f32,
    second: f32,
) -> f32 {
    let mut total = x87_quiet_bits(base.to_bits());
    let scale = setting(e, 0x011c_db30);
    let term = x87_store32(x87_mul(
        x87_load32(first.to_bits()),
        x87_m32(e.mem.u32(scale)),
    ));
    let step = e
        .call(FLOAT_STEP_00406CE0, &args![x87_quiet_bits(term)])
        .f64();
    total = x87_store32(x87_add(step, x87_m32(total)));
    let scale = setting(e, 0x011c_dd84);
    let term = x87_store32(x87_mul(
        x87_load32(second.to_bits()),
        x87_m32(e.mem.u32(scale)),
    ));
    let step = e
        .call(FLOAT_STEP_004E4450, &args![x87_quiet_bits(term)])
        .f64();
    total = x87_store32(x87_add(step, x87_m32(total)));
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00643c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::AVDeriveSkill` (Xbox PDB, matched by the engine map;
/// `cdecl`, result in ST0): 0 unless `0047f060` accepts the actor value.
/// Otherwise the actor value's entry (`00643bf0`: a word, initially -1, and
/// a base `float`); when the word is not -1 it is read as an actor value and
/// the result is [`ai_formulas_calculate_derived_skill_value`] of the base,
/// that value and actor value 0xB, read clamped (`0066ef50`) or, with
/// `use_virtual` set, as the owner's vtable +0x04 plus +0x18 values clamped
/// by `ActorValue::ClampActorValue`; then the setting at `011ccf5c` is added
/// when the owner's vtable +0x24 object answers its +0xF0 and +0x100 slots
/// and its class (`Actor::GetClass`, engine map) answers `005a5f40(index)`. Slot
/// meanings are not confirmed; they are described by offset.
pub fn ai_formulas_av_derive_skill(e: &mut Engine, owner: u32, index: u32, use_virtual: u8) -> f32 {
    e.with_stack(8, |e, locals| {
        let entry_word = locals.addr();
        let result = locals.addr() + 4;
        e.mem.set_u32(result, x87_store32(0.0));
        if !e.call(ACCEPTS_ACTOR_VALUE_0047F060, &args![index]).bool() {
            return f32::from_bits(e.mem.u32(result));
        }
        e.mem.set_u32(entry_word, 0xffff_ffff);
        e.call(ENTRY_00643BF0, &args![index, entry_word, result]);
        let other_index = e.mem.u32(entry_word);
        if other_index != 0xffff_ffff {
            let (other_value, skill) = if use_virtual == 0 {
                let skill = e
                    .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 0xbu32])
                    .f64();
                let skill = x87_store32(skill);
                let other_value = e
                    .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, other_index])
                    .f64();
                (x87_store32(other_value), skill)
            } else {
                let skill = clamped_sum(e, owner, 0xb);
                let other_value = clamped_sum(e, owner, other_index);
                (other_value, skill)
            };
            let base = f32::from_bits(x87_quiet_bits(e.mem.u32(result)));
            let value = ai_formulas_calculate_derived_skill_value(
                e,
                base,
                f32::from_bits(other_value),
                f32::from_bits(skill),
            );
            e.mem.set_u32(result, x87_store32(f64::from(value)));
        }
        let object = e.vcall(owner, 0x24, &args![]).u32();
        if object != 0
            && e.vcall(object, 0xf0, &args![]).bool()
            && e.vcall(object, 0x100, &args![]).bool()
            && e.call(ACTOR_GET_CLASS, &args![object]).u32() != 0
        {
            let class = e.call(ACTOR_GET_CLASS, &args![object]).u32();
            if e.call(CLASS_TEST_005A5F40, &args![class, index]).bool() {
                let bonus = setting(e, 0x011c_cf5c);
                let sum = x87_add(x87_load32(e.mem.u32(result)), x87_m32(e.mem.u32(bonus)));
                e.mem.set_u32(result, x87_store32(sum));
            }
        }
        f32::from_bits(x87_quiet_bits(e.mem.u32(result)))
    })
}

/// The owner's vtable +0x04 and +0x18 values of `index` added and clamped
/// (`ActorValue::ClampActorValue`), as the `float` word the exe leaves in
/// the argument slot.
fn clamped_sum(e: &mut Engine, owner: u32, index: u32) -> u32 {
    let first = e.vcall(owner, 0x4, &args![index]).f64();
    let second = e.vcall(owner, 0x18, &args![index]).f64();
    let sum = x87_store32(x87_add(second, first));
    let clamped = e
        .call(CLAMP_ACTOR_VALUE, &args![index, x87_quiet_bits(sum)])
        .f64();
    x87_store32(clamped)
}

/// `(clamped value 7 + s[add]) * s[scale]` as a `float` result.
fn scaled_value_7(e: &mut Engine, owner: u32, add: u32, scale: u32) -> u32 {
    let value = e
        .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 7u32])
        .f64();
    let add = setting(e, add);
    let sum = x87_add(load(e, add), value);
    let scale = setting(e, scale);
    x87_store32(x87_mul(load(e, scale), sum))
}

// Translated from 00643e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; result in
/// ST0; no Xbox PDB name): for actor value 0x13 `(clamped value 7 +
/// s[11cdf78]) * s[11cdf44]`, for 0x14 `(clamped value 7 + s[11cde50]) *
/// s[11cdce8]`, else 0.
pub fn fn_00643e40(e: &mut Engine, owner: u32, index: u32) -> f32 {
    let result = match index {
        0x13 => scaled_value_7(e, owner, 0x011c_df78, 0x011c_df44),
        0x14 => scaled_value_7(e, owner, 0x011c_de50, 0x011c_dce8),
        _ => x87_store32(0.0),
    };
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00643ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; result in
/// ST0; no Xbox PDB name): for actor value 0x38 `s[11cd3a8] + owner vtable
/// +0x0C(0x2D) * s[11cdaac]`, else 0.
pub fn fn_00643ed0(e: &mut Engine, owner: u32, index: u32) -> f32 {
    let mut result = x87_store32(0.0);
    if index == 0x38 {
        let scale = setting(e, 0x011c_daac);
        let value = e.vcall(owner, 0xc, &args![0x2du32]).f64();
        let term = x87_mul(value, x87_m32(e.mem.u32(scale)));
        let add = setting(e, 0x011c_d3a8);
        result = x87_store32(x87_add(load(e, add), term));
    }
    f32::from_bits(x87_quiet_bits(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_42e0, fn_006442e0(Ptr)),
        entry!(
            0x0064_3c20,
            ai_formulas_calculate_derived_skill_value(f32, f32, f32) -> f32
        ),
        entry!(0x0064_3c90, ai_formulas_av_derive_skill(u32, u32, u8) -> f32),
        entry!(0x0064_3e40, fn_00643e40(u32, u32) -> f32),
        entry!(0x0064_3ed0, fn_00643ed0(u32, u32) -> f32),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006442e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0064_42e0, ARRAY_VTABLE_0104F3E8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod derivation_tests {
    use super::*;

    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;
    const SLOT_C: u32 = 0x0001_0030;
    const SLOT_D: u32 = 0x0001_0040;
    const SLOT_E: u32 = 0x0001_0050;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_c000, 0x3000);
        e.register(SETTING_VALUE, |_, a| word(a[0] + 4));
        e
    }

    fn set(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn arg_f32(a: &[u32], i: usize) -> f32 {
        f32::from_bits(a[i])
    }

    fn set_skill_steps(e: &mut Engine) {
        set(e, 0x011c_db30, 2.0);
        set(e, 0x011c_dd84, 0.5);
        e.register(FLOAT_STEP_00406CE0, |_, a| {
            st0(f64::from(arg_f32(a, 0)) + 1.0)
        });
        e.register(FLOAT_STEP_004E4450, |_, a| {
            st0(f64::from(arg_f32(a, 0)) * 2.0)
        });
    }

    #[test]
    fn calculate_derived_skill_value_adds_both_steps() {
        let mut e = engine();
        set_skill_steps(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x0064_3c20, &args![1.0f32, 3.0f32, 4.0f32]).f32();
        // 1 + (3 * 2 + 1) + (4 * 0.5 * 2).
        assert_eq!(r, 12.0);
        let log = e.call_log.take().unwrap();
        let steps: Vec<_> = log
            .iter()
            .filter(|c| c.0 == FLOAT_STEP_00406CE0 || c.0 == FLOAT_STEP_004E4450)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(steps, vec![vec![6.0f32.to_bits()], vec![2.0f32.to_bits()]]);
    }

    /// An owner whose vtable +0x04 answers the index, +0x18 answers 1 and
    /// +0x24 the object `SLOT_C` returns; the object's +0xF0 and +0x100 are
    /// `SLOT_D` and `SLOT_E`.
    fn owner(e: &mut Engine) -> (u32, u32) {
        let owner = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x10];
        slots[1] = SLOT_A;
        slots[0x18 / 4] = SLOT_B;
        slots[0x24 / 4] = SLOT_C;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(owner, 0x0002_0000);
        let mut slots = vec![0u32; 0x48];
        slots[0xf0 / 4] = SLOT_D;
        slots[0x100 / 4] = SLOT_E;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(object, 0x0002_1000);
        e.register(SLOT_A, |_, a| st0(f64::from(a[1])));
        e.register(SLOT_B, |_, _| st0(1.0));
        e.register(SLOT_C, |_, _| word(0));
        e.register(SLOT_D, |_, _| word(1));
        e.register(SLOT_E, |_, _| word(1));
        (owner, object)
    }

    #[test]
    fn av_derive_skill_takes_each_branch() {
        let mut e = engine();
        set_skill_steps(&mut e);
        let (owner, object) = owner(&mut e);
        e.register(ACCEPTS_ACTOR_VALUE_0047F060, |_, _| word(0));
        assert_eq!(e.call(0x0064_3c90, &args![owner, 0x20u32, 0u32]).f32(), 0.0);

        e.register(ACCEPTS_ACTOR_VALUE_0047F060, |_, a| {
            assert_eq!(a, &[0x20]);
            word(1)
        });
        e.register(ENTRY_00643BF0, |e, a| {
            assert_eq!(a[0], 0x20);
            e.mem.set_u32(a[1], 5);
            e.mem.set_f32(a[2], 10.0);
            Ret::default()
        });
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            st0(if a[1] == 0xb { 2.0 } else { 3.0 })
        });
        // 10 + (3 * 2 + 1) + (2 * 0.5 * 2).
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 0u32]).f32(),
            19.0
        );

        // Through the vtable: 0xB gives 12, 5 gives 6; the class bonus.
        e.register(CLAMP_ACTOR_VALUE, |_, a| st0(f64::from(arg_f32(a, 1))));
        e.register_double(SLOT_C, move |_, _| word(object));
        e.register(ACTOR_GET_CLASS, |_, _| word(0x9000));
        e.register(CLASS_TEST_005A5F40, |_, a| {
            assert_eq!(a, &[0x9000, 0x20]);
            word(1)
        });
        set(&mut e, 0x011c_cf5c, 0.5);
        e.call_log = Some(vec![]);
        // 10 + (6 * 2 + 1) + (12 * 0.5 * 2) + 0.5.
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 1u32]).f32(),
            35.5
        );
        let clamps: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == CLAMP_ACTOR_VALUE)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            clamps,
            vec![vec![0xb, 12.0f32.to_bits()], vec![5, 6.0f32.to_bits()]]
        );

        // A -1 entry word keeps the base value; a class without the skill
        // adds nothing.
        e.register(ENTRY_00643BF0, |e, a| {
            e.mem.set_f32(a[2], 10.0);
            Ret::default()
        });
        e.register(CLASS_TEST_005A5F40, |_, _| word(0));
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 1u32]).f32(),
            10.0
        );
    }

    #[test]
    fn fn_00643e40_scales_actor_value_7_for_0x13_and_0x14() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a, &[0x4000, 7]);
            st0(4.0)
        });
        set(&mut e, 0x011c_df78, 1.0);
        set(&mut e, 0x011c_df44, 2.0);
        set(&mut e, 0x011c_de50, 2.0);
        set(&mut e, 0x011c_dce8, 0.5);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x13u32]).f32(), 10.0);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x14u32]).f32(), 3.0);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x15u32]).f32(), 0.0);
    }

    #[test]
    fn fn_00643ed0_scales_the_owner_value_for_0x38() {
        let mut e = engine();
        let owner = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 4];
        slots[3] = SLOT_A;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(owner, 0x0002_0000);
        e.register(SLOT_A, |_, a| {
            assert_eq!(a[1], 0x2d);
            st0(4.0)
        });
        set(&mut e, 0x011c_daac, 0.5);
        set(&mut e, 0x011c_d3a8, 1.0);
        assert_eq!(e.call(0x0064_3ed0, &args![owner, 0x38u32]).f32(), 3.0);
        assert_eq!(e.call(0x0064_3ed0, &args![owner, 0x37u32]).f32(), 0.0);
    }
}
