//! `Havok SDK/hkpvehicledefaultengine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultEngine`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultEngine` (names unconfirmed).
    pub struct HkpVehicleDefaultEngine: 0x30 {
        /// `m_minRPM`.
        0x08 m_min_rpm: f32,
        /// `m_optRPM`.
        0x0c m_opt_rpm: f32,
        /// `m_maxRPM`.
        0x10 m_max_rpm: f32,
        /// `m_maxTorque`.
        0x14 m_max_torque: f32,
        /// `m_torqueFactorAtMinRPM`.
        0x18 m_torque_factor_at_min_rpm: f32,
        /// `m_torqueFactorAtMaxRPM`.
        0x1c m_torque_factor_at_max_rpm: f32,
        /// `m_resistanceFactorAtMinRPM`.
        0x20 m_resistance_factor_at_min_rpm: f32,
        /// `m_resistanceFactorAtOptRPM`.
        0x24 m_resistance_factor_at_opt_rpm: f32,
        /// `m_resistanceFactorAtMaxRPM`.
        0x28 m_resistance_factor_at_max_rpm: f32,
        /// `m_clutchSlipRPM`.
        0x2c m_clutch_slip_rpm: f32,
    }
}

/// The double `0.5` at `01011588`.
const HALF: f64 = 0.5;

// Translated from 00e440a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultEngine::calcEngineInfo` (Xbox PDB), `(this, _, _, throttle, rpm, output)`;
/// the first two stack words are not read. `throttle` and `rpm` point to floats, `output`
/// receives the torque at `+0` and the corrected rpm at `+4`. Every `float` store rounds
/// (the intermediate values are kept in double precision, which is what the x87 does here);
/// `T` = the throttle, `r` = the rpm, `min`, `opt`, `max`, `maxTorque`, `fMin`, `fMax`,
/// `rMin`, `rOpt`, `rMax` and `slip` the fields of `this`:
/// 1. `output[1] = r`. When `min > r`: `g = float(slip * T)`, `h = float(min * 0.5)`, and
///    `output[1] = float(g + min)` if `h > r`, else `float(min + (g * (r - h)) / (min - h))`;
/// 2. `D = float(output[1] - opt)`. For `D` not below zero (also for a NaN): when `max <=
///    output[1]` (or a NaN) the rpm is limited to `max` and the torque is `float(T * 0.0 -
///    float(rMax * maxTorque))`; otherwise, with `k = float(1 / (max - opt))`, `u = float((rMax -
///    rOpt) * k)`, `V = float((u * D + rOpt) * maxTorque)`, `W = float(k * ((fMax - 1) * k))` and
///    `Z = float(((D * D) * W + 1) * maxTorque)` the torque is `float(T * Z - V)`;
/// 3. for `D` below zero: `k = float(1 / (min - opt))`, `u = float((rMin - rOpt) * k)`, `P4 =
///    float((u * D + rOpt) * maxTorque)`, `W = float(k * ((fMin - 1) * k))`, `P1 = float(((D * D)
///    * W + 1) * maxTorque)`, the rpm is read again and with `r2` it, the torque is `float(T *
///    P1 - float((r2 / min) * P4))` when `min > r2` and `float(T * P1 - P4)` otherwise (also for
///    a NaN).
pub fn calc_engine_info(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultEngine>,
    _unused_1: u32,
    _unused_2: u32,
    throttle: Ptr,
    rpm: Ptr,
    output: Ptr,
) {
    let out = output.addr();
    let r = x87_quiet(e.mem.f32(rpm.addr()));
    e.mem.set_f32(out + 4, r);
    let t = x87_quiet(e.mem.f32(throttle.addr()));
    let min = e.get(this, HkpVehicleDefaultEngine::m_min_rpm);
    let opt = e.get(this, HkpVehicleDefaultEngine::m_opt_rpm);
    let max = e.get(this, HkpVehicleDefaultEngine::m_max_rpm);
    let max_torque = f64::from(e.get(this, HkpVehicleDefaultEngine::m_max_torque));
    let wide = |value: f32| f64::from(value);
    if min > r {
        let g = (wide(e.get(this, HkpVehicleDefaultEngine::m_clutch_slip_rpm)) * wide(t)) as f32;
        let h = (wide(min) * HALF) as f32;
        if h > r {
            e.mem.set_f32(out + 4, (wide(g) + wide(min)) as f32);
        } else {
            let lift = (wide(r) - wide(h)) * wide(g);
            let corrected = lift / (wide(min) - wide(h));
            e.mem.set_f32(out + 4, (corrected + wide(min)) as f32);
        }
    }
    let rpm_out = x87_quiet(e.mem.f32(out + 4));
    let d = (wide(rpm_out) - wide(opt)) as f32;
    let r_opt = e.get(
        this,
        HkpVehicleDefaultEngine::m_resistance_factor_at_opt_rpm,
    );
    if !(d < 0.0) {
        let rpm_out = x87_quiet(e.mem.f32(out + 4));
        let r_max = e.get(
            this,
            HkpVehicleDefaultEngine::m_resistance_factor_at_max_rpm,
        );
        if !(max > rpm_out) {
            e.mem.set_f32(out + 4, x87_quiet(max));
            let drag = (wide(r_max) * max_torque) as f32;
            let torque = (wide(t) * 0.0) - wide(drag);
            e.mem.set_f32(out, torque as f32);
            return;
        }
        let k = (1.0 / (wide(max) - wide(opt))) as f32;
        let u = ((wide(r_max) - wide(r_opt)) * wide(k)) as f32;
        let v = ((wide(u) * wide(d) + wide(r_opt)) * max_torque) as f32;
        let f_max = e.get(this, HkpVehicleDefaultEngine::m_torque_factor_at_max_rpm);
        let w = (wide(k) * ((wide(f_max) - 1.0) * wide(k))) as f32;
        let z = (((wide(d) * wide(d)) * wide(w) + 1.0) * max_torque) as f32;
        e.mem.set_f32(out, (wide(t) * wide(z) - wide(v)) as f32);
        return;
    }
    let k = (1.0 / (wide(min) - wide(opt))) as f32;
    let r_min = e.get(
        this,
        HkpVehicleDefaultEngine::m_resistance_factor_at_min_rpm,
    );
    let u = ((wide(r_min) - wide(r_opt)) * wide(k)) as f32;
    let p4 = ((wide(u) * wide(d) + wide(r_opt)) * max_torque) as f32;
    let f_min = e.get(this, HkpVehicleDefaultEngine::m_torque_factor_at_min_rpm);
    let w = (wide(k) * ((wide(f_min) - 1.0) * wide(k))) as f32;
    let p1 = (((wide(d) * wide(d)) * wide(w) + 1.0) * max_torque) as f32;
    let r2 = x87_quiet(e.mem.f32(rpm.addr()));
    let torque = if min > r2 {
        let y = ((wide(r2) / wide(min)) * wide(p4)) as f32;
        wide(t) * wide(p1) - wide(y)
    } else {
        wide(t) * wide(p1) - wide(p4)
    };
    e.mem.set_f32(out, torque as f32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e4_40a0,
        calc_engine_info(Ptr<HkpVehicleDefaultEngine>, u32, u32, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;

    /// An engine with min 8, opt 16, max 32, max torque 2, torque factors 3 (min) and 0.5 (max),
    /// resistance factors 2.5 (min), 0.5 (opt) and 1.5 (max) and clutch slip 0.5. Runs
    /// `calcEngineInfo` with the throttle and rpm and returns `(torque, output rpm)`.
    fn run(throttle: f32, rpm: f32) -> (f32, f32) {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        for (offset, value) in [
            (0x08, 8.0f32),
            (0x0c, 16.0),
            (0x10, 32.0),
            (0x14, 2.0),
            (0x18, 3.0),
            (0x1c, 0.5),
            (0x20, 2.5),
            (0x24, 0.5),
            (0x28, 1.5),
            (0x2c, 0.5),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        let throttle_at = e.mem.alloc(4);
        e.mem.set_f32(throttle_at, throttle);
        let rpm_at = e.mem.alloc(4);
        e.mem.set_f32(rpm_at, rpm);
        let output = e.mem.alloc(8);
        e.call(
            0x00e4_40a0,
            &args![this, 1u32, 2u32, throttle_at, rpm_at, output],
        );
        (e.mem.f32(output), e.mem.f32(output + 4))
    }

    #[test]
    fn above_the_maximum_the_rpm_is_limited_and_the_engine_drags() {
        // The torque is T * 0 - 1.5 * 2 and the rpm becomes the maximum.
        assert_eq!(run(1.0, 40.0), (-3.0, 32.0));
        // Exactly the maximum counts as above.
        assert_eq!(run(1.0, 32.0), (-3.0, 32.0));
    }

    #[test]
    fn between_the_optimum_and_the_maximum_the_curve_interpolates() {
        // D = 8, k = 1/16, V = ((1/16 * 8) + 0.5) * 2 = 2, W = -1/512, Z = 1.75.
        assert_eq!(run(2.0, 24.0), (3.5 - 2.0, 24.0));
    }

    #[test]
    fn below_the_optimum_the_other_curve_is_used() {
        // D = -4, k = -1/8, P4 = 3, W = 1/32, P1 = 3; the rpm is not below the minimum.
        assert_eq!(run(2.0, 12.0), (6.0 - 3.0, 12.0));
    }

    #[test]
    fn below_the_minimum_the_rpm_is_raised_by_the_throttle() {
        // g = 0.5 * 2 = 1, h = 4. At h the raised rpm is the minimum; D = -8, P4 = 5, P1 = 6
        // and the rpm 4 is below the minimum: y = (4 / 8) * 5 = 2.5.
        assert_eq!(run(2.0, 4.0), (12.0 - 2.5, 8.0));
        // Between h and the minimum: 8 + (1 * (6 - 4)) / (8 - 4) = 8.5.
        assert_eq!(run(2.0, 6.0).1, 8.5);
        // Below h: g + min = 9.
        assert_eq!(run(2.0, 2.0).1, 9.0);
    }

    #[test]
    fn a_nan_rpm_counts_as_above_the_maximum() {
        // Every comparison with a NaN is unordered, which the exe treats as "not below".
        assert_eq!(run(1.0, f32::NAN), (-3.0, 32.0));
    }
}
