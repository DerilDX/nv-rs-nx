//! `fallout shared/magic/magicformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The value of a game setting (`00403e20`, `ECX` = the setting): the
/// address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Game settings (names from their static initializers):
/// `fMagicAreaBaseCostMult`, `fMagicDurMagBaseCostMult`,
/// `fMagicCostScale`, `fMagicRangeTargetCostMult`.
const AREA_BASE_COST_MULT: u32 = 0x011d_195c;
const DUR_MAG_BASE_COST_MULT: u32 = 0x011d_1998;
const COST_SCALE: u32 = 0x011d_198c;
const RANGE_TARGET_COST_MULT: u32 = 0x011d_1950;
/// The two-float function applied to the magnitude term (`00404010`,
/// cdecl: value, 1.0), result in `ST0`.
const MAGNITUDE_TERM: u32 = 0x0040_4010;
/// The two-float function applied to the duration (`004dd130`, cdecl:
/// duration, `fMagicCostScale`), result in `ST0`.
const DURATION_TERM: u32 = 0x004d_d130;

fn setting(e: &mut Engine, setting: u32) -> u32 {
    let p = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(p)
}

// Translated from 006490b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFormulas::GetMagicEffectCost` (Xbox PDB, cdecl): `range factor *
/// (00404010(magnitude * fMagicAreaBaseCostMult, 1.0) * (base cost *
/// fMagicDurMagBaseCostMult * area) * duration factor)`, where an area of
/// 0 counts as 1, the duration factor is `004dd130(duration,
/// fMagicCostScale)` (1.0 for 0 duration) and the range factor is
/// `fMagicRangeTargetCostMult` with `target` set (else 1.0). Each step is
/// the exe's x87 operation and `float` store; the result is in `ST0`.
pub fn magic_formulas_get_magic_effect_cost(
    e: &mut Engine,
    base_cost: f32,
    magnitude: i32,
    area: i32,
    duration: i32,
    target: u8,
) -> f64 {
    let one = 1.0f32.to_bits();
    let mult = setting(e, AREA_BASE_COST_MULT);
    let scaled = x87_store32(x87_mul(f64::from(magnitude), x87_m32(mult)));
    let magnitude_term = x87_store32(e.call(MAGNITUDE_TERM, &args![scaled, one]).f64());
    let area_factor = if area == 0 {
        one
    } else {
        x87_store32(f64::from(area))
    };
    let mult = setting(e, DUR_MAG_BASE_COST_MULT);
    let base = x87_store32(x87_mul(
        x87_mul(x87_m32(base_cost.to_bits()), x87_m32(mult)),
        x87_m32(area_factor),
    ));
    let duration_factor = if duration == 0 {
        one
    } else {
        let scale = x87_quiet_bits(setting(e, COST_SCALE));
        let duration = x87_store32(f64::from(duration));
        x87_store32(e.call(DURATION_TERM, &args![duration, scale]).f64())
    };
    let duration_factor = x87_quiet_bits(duration_factor);
    let cost = x87_store32(x87_mul(
        x87_mul(x87_m32(magnitude_term), x87_m32(base)),
        x87_m32(duration_factor),
    ));
    let range_factor = if target != 0 {
        x87_quiet_bits(setting(e, RANGE_TARGET_COST_MULT))
    } else {
        one
    };
    let result = x87_store32(x87_mul(x87_m32(range_factor), x87_m32(cost)));
    x87_load32(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0064_90b0,
        magic_formulas_get_magic_effect_cost(f32, i32, i32, i32, u8) -> f64
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Settings with their values at `setting + 4` (as `00403e20` gives
    /// them), and doubles that multiply their two floats.
    fn cost_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_1000, 0x1000);
        e.set_global(AREA_BASE_COST_MULT + 4, 0.5f32);
        e.set_global(DUR_MAG_BASE_COST_MULT + 4, 2.0f32);
        e.set_global(COST_SCALE + 4, 3.0f32);
        e.set_global(RANGE_TARGET_COST_MULT + 4, 1.5f32);
        e.register(SETTING_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        for f in [MAGNITUDE_TERM, DURATION_TERM] {
            e.register(f, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0])) * f64::from(f32::from_bits(a[1])),
                ..Ret::default()
            });
        }
        e
    }

    #[test]
    fn cost_multiplies_the_terms() {
        let mut e = cost_engine();
        e.call_log = Some(vec![]);
        // (10 * 0.5) * 1.0 = 5; 4 * 2 * 3 = 24; 2 * 3 = 6; 5 * 24 * 6 = 720;
        // times 1.5 for a target.
        let words = args![4.0f32, 10i32, 3i32, 2i32, 1u32];
        let cost = e.call(0x0064_90b0, &words).f64();
        assert_eq!(cost, 1080.0);
        assert_eq!(
            calls_to(&e, MAGNITUDE_TERM),
            vec![vec![5.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, DURATION_TERM),
            vec![vec![2.0f32.to_bits(), 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn zero_area_and_duration_count_as_one() {
        let mut e = cost_engine();
        e.call_log = Some(vec![]);
        let cost = magic_formulas_get_magic_effect_cost(&mut e, 4.0, 10, 0, 0, 0);
        // 5 * (4 * 2 * 1) * 1 * 1 = 40.
        assert_eq!(cost, 40.0);
        assert!(calls_to(&e, DURATION_TERM).is_empty());
    }
}
