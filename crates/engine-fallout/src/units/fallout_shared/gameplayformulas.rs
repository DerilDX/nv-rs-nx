//! `fallout shared/gameplayformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043d4d0` (`thiscall` on an integer `Setting`, no stack words): answers the address of
/// the setting's value. `00403e20` (`thiscall` on a float `Setting`): the same for a float.
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings, by the names their static initializers give them.
const I_XP_BASE: u32 = 0x011d_03e8;
const I_XP_BUMP_BASE: u32 = 0x011d_0ae4;
const F_ACTOR_SWIM_BREATH_BASE: u32 = 0x011d_0868;
const F_ACTOR_SWIM_BREATH_MULT: u32 = 0x011d_0e28;
const F_LOCK_SKILL_BASE: u32 = 0x011d_0550;
/// The two doubles `fn_00648c10` scales and offsets by (0.5 and 10.0 in the exe).
const SCALE_DOUBLE: u32 = 0x0101_1588;
const OFFSET_DOUBLE: u32 = 0x0102_0758;
/// `00406cc0` (`cdecl`, one float, answers `ST0`; no engine-map name).
const FLOAT_FUNCTION: u32 = 0x0040_6cc0;
/// `_ftol2` (`00ec62c0`, the value in `ST0`).
const FTOL2: u32 = 0x00ec_62c0;
/// `004a8f20` and `00647b70` (`cdecl`, two words each; no engine-map names).
const COMBINE_FIRST: u32 = 0x004a_8f20;
const COMBINE_SECOND: u32 = 0x0064_7b70;

// Translated from 00648b50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetRequiredExperiencePoints` (Xbox PDB; `cdecl`, the level): 0 for a
/// level of 1 or less; otherwise `iXPBase` plus, for each further level below `level`, a
/// step that starts at `iXPBase` and grows by `iXPBumpBase` each time (wrapping 32-bit
/// arithmetic, as the exe's `ADD`).
pub fn game_play_formulas_get_required_experience_points(e: &mut Engine, level: i32) -> i32 {
    if level <= 1 {
        return 0;
    }
    let base = e.call(INT_SETTING_VALUE, &args![I_XP_BASE]).u32();
    let mut total = e.mem.i32(base);
    let mut step = total;
    let bump = e.call(INT_SETTING_VALUE, &args![I_XP_BUMP_BASE]).u32();
    let bump = e.mem.i32(bump);
    let mut i = 1i32;
    while i < level.wrapping_sub(1) {
        step = step.wrapping_add(bump);
        total = total.wrapping_add(step);
        i += 1;
    }
    total
}

// Translated from 00648c10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): `_ftol2(00406cc0(first * 0.5 + 10.0))`
/// (the doubles at `01011588` and `01020758`, the sum rounded to a float), plus one when
/// `second` is even and `first` odd and positive (the C remainders `% 2`).
pub fn fn_00648c10(e: &mut Engine, first: i32, second: i32) -> i32 {
    let scaled = x87_mul(f64::from(first), e.mem.f64(SCALE_DOUBLE));
    let sum = x87_store32(x87_add(scaled, e.mem.f64(OFFSET_DOUBLE)));
    let value = e.call(FLOAT_FUNCTION, &args![x87_quiet_bits(sum)]).f64();
    let mut result = e.call(FTOL2, &args![value]).i32();
    if second % 2 == 0 && first % 2 == 1 {
        result = result.wrapping_add(1);
    }
    result
}

// Translated from 00648a10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::SwimBreathTime` (Xbox PDB; `cdecl`, one word; answers `ST0`):
/// `fActorSwimBreathMult * value + fActorSwimBreathBase`, rounded to a float.
pub fn game_play_formulas_swim_breath_time(e: &mut Engine, value: i32) -> f32 {
    let base = e
        .call(FLOAT_SETTING_VALUE, &args![F_ACTOR_SWIM_BREATH_BASE])
        .u32();
    let mult = e
        .call(FLOAT_SETTING_VALUE, &args![F_ACTOR_SWIM_BREATH_MULT])
        .u32();
    let product = x87_mul(x87_load32(e.mem.u32(mult)), f64::from(value));
    let sum = x87_add(product, x87_m32(e.mem.u32(base)));
    f32::from_bits(x87_quiet_bits(x87_store32(sum)))
}

// Translated from 00648d30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetForceLockChance` (Xbox PDB; `cdecl`, two words):
/// `00647b70(004a8f20(first - second + _ftol2(<the float setting at 011d0550>), 100), 0)`
/// (the `PUSH 0` and `PUSH 0x64` at the start are those calls' second words).
pub fn game_play_formulas_get_force_lock_chance(e: &mut Engine, first: i32, second: i32) -> u32 {
    let difference = first.wrapping_sub(second);
    let setting = e.call(FLOAT_SETTING_VALUE, &args![F_LOCK_SKILL_BASE]).u32();
    let base = e.call(FTOL2, &args![x87_load32(e.mem.u32(setting))]).i32();
    let value = difference.wrapping_add(base);
    let limited = e.call(COMBINE_FIRST, &args![value, 0x64u32]).u32();
    e.call(COMBINE_SECOND, &args![limited, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_8b50,
            game_play_formulas_get_required_experience_points(i32) -> i32
        ),
        entry!(0x0064_8c10, fn_00648c10(i32, i32) -> i32),
        entry!(0x0064_8a10, game_play_formulas_swim_breath_time(i32) -> f32),
        entry!(
            0x0064_8d30,
            game_play_formulas_get_force_lock_chance(i32, i32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Settings whose value sits at +4 (the double answers `this + 4`).
    fn settings(e: &mut Engine) {
        e.map(0x011d_0000, 0x1000);
        e.register(INT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
    }

    #[test]
    fn required_experience_grows_by_the_bump() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_u32(I_XP_BASE + 4, 200);
        e.mem.set_u32(I_XP_BUMP_BASE + 4, 150);
        let at = |e: &mut Engine, level: i32| e.call(0x0064_8b50, &[level as u32]).i32();
        assert_eq!(at(&mut e, 1), 0);
        assert_eq!(at(&mut e, -5), 0);
        assert_eq!(at(&mut e, 2), 200);
        assert_eq!(at(&mut e, 3), 200 + 350);
        assert_eq!(at(&mut e, 4), 200 + 350 + 500);
    }

    #[test]
    fn fn_00648c10_rounds_and_adds_one_for_the_parities() {
        let mut e = Engine::new();
        e.map(SCALE_DOUBLE & !0xfff, 0x1000);
        e.map(OFFSET_DOUBLE & !0xfff, 0x1000);
        e.mem.set_f64(SCALE_DOUBLE, 0.5);
        e.mem.set_f64(OFFSET_DOUBLE, 10.0);
        e.register(FLOAT_FUNCTION, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        let at = |e: &mut Engine, a: i32, b: i32| e.call(0x0064_8c10, &[a as u32, b as u32]).i32();
        e.call_log = Some(vec![]);
        // 6 * 0.5 + 10 = 13.
        assert_eq!(at(&mut e, 6, 2), 13);
        assert_eq!(calls_to(&e, FLOAT_FUNCTION), vec![vec![13.0f32.to_bits()]]);
        // 5: 12.5 truncated to 12, odd first and even second: 13.
        assert_eq!(at(&mut e, 5, 4), 13);
        assert_eq!(at(&mut e, 5, 3), 12);
        // -5 % 2 is -1: no extra point.
        assert_eq!(at(&mut e, -5, 0), 7);
    }

    #[test]
    fn swim_breath_time_is_mult_times_value_plus_base() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_f32(F_ACTOR_SWIM_BREATH_BASE + 4, 10.0);
        e.mem.set_f32(F_ACTOR_SWIM_BREATH_MULT + 4, 1.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_8a10, &[4]).f64(), 16.0);
        assert_eq!(
            calls_to(&e, FLOAT_SETTING_VALUE),
            vec![
                vec![F_ACTOR_SWIM_BREATH_BASE],
                vec![F_ACTOR_SWIM_BREATH_MULT]
            ]
        );
    }

    #[test]
    fn force_lock_chance_combines_the_difference_and_the_base() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_f32(F_LOCK_SKILL_BASE + 4, 20.75);
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(COMBINE_FIRST, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        e.register(COMBINE_SECOND, |_, a| {
            (a[0] as i32).max(a[1] as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_8d30, &[50, 30]).u32(), 40);
        assert_eq!(calls_to(&e, COMBINE_FIRST), vec![vec![40, 100]]);
        assert_eq!(calls_to(&e, COMBINE_SECOND), vec![vec![40, 0]]);
    }
}
