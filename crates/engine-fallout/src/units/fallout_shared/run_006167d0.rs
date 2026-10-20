//! `fallout shared/run_006167d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here work on a `TESReputation` form (fields read at their PC
//! offsets, which are 0x10 lower than the Xbox PDB's: `TESForm` is 0x18 bytes
//! on PC): `REPUTATION_DATA::fMaximumValue` at +0x40, `fPositiveValue` at
//! +0x44 and `fNegativeValue` at +0x48.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Setting` value accessor (`00403e20`, `this` = the setting): the
/// address of its value (+4), or of a zeroed static when `this` is null.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `fReputationThresholdOne` (the name its static initializer `00f53580`
/// gives it).
const THRESHOLD_ONE: u32 = 0x011c_ba7c;
/// `fReputationThresholdTwo` (static initializer `00f535b0`).
const THRESHOLD_TWO: u32 = 0x011c_bc70;
/// `fReputationThresholdThree` (static initializer `00f535e0`).
const THRESHOLD_THREE: u32 = 0x011c_bd68;

/// The float value of a `Setting`, read through its accessor (`FLD dword`).
fn setting_float(e: &mut Engine, setting: u32) -> f64 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_load32(e.mem.u32(value))
}

// Translated from 00616950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reputation level (0 to 3) of one side of a `TESReputation`:
/// `positive` 1 uses the positive fraction (`00616a20`), anything else the
/// negative one (`00616a40`). Level 1 when the fraction is at least
/// `fReputationThresholdOne` and below `...Two`, 2 when at least `...Two` and
/// below `...Three`, 3 when at least `...Three`, otherwise 0 (a NaN on
/// either side of a comparison fails it).
pub fn fn_00616950(e: &mut Engine, this: Ptr, positive: i32) -> u8 {
    let fraction = if positive == 1 {
        fn_00616a20(e, this)
    } else {
        fn_00616a40(e, this)
    };
    // FSTP float, then FLD float for each comparison.
    let fraction = x87_load32(fraction.to_bits());
    if setting_float(e, THRESHOLD_ONE) <= fraction && setting_float(e, THRESHOLD_TWO) > fraction {
        return 1;
    }
    if setting_float(e, THRESHOLD_TWO) <= fraction && setting_float(e, THRESHOLD_THREE) > fraction {
        return 2;
    }
    if setting_float(e, THRESHOLD_THREE) <= fraction {
        return 3;
    }
    0
}

// Translated from 00616a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fPositiveValue / fMaximumValue` of a `TESReputation`, stored as a float.
pub fn fn_00616a20(e: &mut Engine, this: Ptr) -> f32 {
    // TESReputation::fPositiveValue (Xbox PDB +0x54) +0x44, fMaximumValue +0x40.
    let positive = x87_load32(e.mem.u32(this.addr() + 0x44));
    let maximum = x87_m32(e.mem.u32(this.addr() + 0x40));
    f32::from_bits(x87_quiet_bits(x87_store32(x87_div(positive, maximum))))
}

// Translated from 00616a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fNegativeValue / fMaximumValue` of a `TESReputation`, stored as a float.
pub fn fn_00616a40(e: &mut Engine, this: Ptr) -> f32 {
    // TESReputation::fNegativeValue (Xbox PDB +0x58) +0x48, fMaximumValue +0x40.
    let negative = x87_load32(e.mem.u32(this.addr() + 0x48));
    let maximum = x87_m32(e.mem.u32(this.addr() + 0x40));
    f32::from_bits(x87_quiet_bits(x87_store32(x87_div(negative, maximum))))
}

// Translated from 00616a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reputation title index (0 to 6) from the positive level (`00616950`
/// with 1) and the negative level (with 0), for one of three tables picked
/// by `table` (0, 1 or 2; any other value gives 0 unless both levels are 0).
/// Both levels 0 give 1; a combination a table does not list gives 0.
pub fn fn_00616a90(e: &mut Engine, this: Ptr, table: i8) -> u8 {
    let positive = fn_00616950(e, this, 1) as i8;
    let negative = fn_00616950(e, this, 0) as i8;
    if positive == 0 && negative == 0 {
        return 1;
    }
    match table {
        0 => {
            if positive == 1 && negative == 1 {
                return 3;
            }
            if positive == 3 {
                if negative == 2 {
                    return 2;
                }
                if negative == 3 {
                    return 5;
                }
            }
            if positive == 2 {
                if negative == 3 {
                    return 2;
                }
                if negative == 2 {
                    return 4;
                }
            }
        }
        1 => {
            if positive == 1 && negative == 0 {
                return 4;
            }
            if positive == 2 {
                if negative == 0 {
                    return 5;
                }
                if negative == 1 {
                    return 2;
                }
            }
            if positive == 3 {
                if negative == 0 {
                    return 6;
                }
                if negative == 1 {
                    return 3;
                }
            }
        }
        2 => {
            if negative == 1 && positive == 0 {
                return 4;
            }
            if negative == 2 {
                if positive == 0 {
                    return 5;
                }
                if positive == 1 {
                    return 2;
                }
            }
            if negative == 3 {
                if positive == 0 {
                    return 6;
                }
                if positive == 1 {
                    return 3;
                }
            }
        }
        _ => {}
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00616950, fn_00616950(Ptr, i32) -> u8),
        entry!(0x00616a20, fn_00616a20(Ptr) -> f32),
        entry!(0x00616a40, fn_00616a40(Ptr) -> f32),
        entry!(0x00616a90, fn_00616a90(Ptr, i8) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A reputation with the given maximum, positive and negative values,
    /// and the three thresholds 0.25, 0.5 and 0.75 behind a double of the
    /// setting accessor.
    fn reputation(e: &mut Engine, maximum: f32, positive: f32, negative: f32) -> Ptr {
        e.register(SETTING_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.map(0x011c_b000, 0x1000);
        e.set_global(THRESHOLD_ONE + 4, 0.25f32);
        e.set_global(THRESHOLD_TWO + 4, 0.5f32);
        e.set_global(THRESHOLD_THREE + 4, 0.75f32);
        let form = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_f32(form.addr() + 0x40, maximum);
        e.mem.set_f32(form.addr() + 0x44, positive);
        e.mem.set_f32(form.addr() + 0x48, negative);
        form
    }

    #[test]
    fn fractions_divide_by_the_maximum() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 40.0, 10.0, 30.0);
        assert_eq!(e.call(0x0061_6a20, &args![form]).f32(), 0.25);
        assert_eq!(e.call(0x0061_6a40, &args![form]).f32(), 0.75);
    }

    #[test]
    fn levels_follow_the_thresholds() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 100.0, 10.0, 25.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0061_6950, &args![form, 0u32]).u8(), 1);
        e.mem.set_f32(form.addr() + 0x44, 50.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 2);
        e.mem.set_f32(form.addr() + 0x44, 90.0);
        assert_eq!(e.call(0x0061_6950, &args![form, 1u32]).u8(), 3);
        // 0 / 0 is NaN: every comparison fails.
        let empty = reputation(&mut e, 0.0, 0.0, 0.0);
        assert_eq!(e.call(0x0061_6950, &args![empty, 1u32]).u8(), 0);
    }

    #[test]
    fn titles_come_from_the_two_levels() {
        let mut e = Engine::new();
        let form = reputation(&mut e, 100.0, 0.0, 0.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 1);
        // Positive level 3, negative level 2.
        e.mem.set_f32(form.addr() + 0x44, 80.0);
        e.mem.set_f32(form.addr() + 0x48, 60.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 2);
        assert_eq!(e.call(0x0061_6a90, &args![form, 1u32]).u8(), 0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 7u32]).u8(), 0);
        // Positive 2, negative 0.
        e.mem.set_f32(form.addr() + 0x44, 60.0);
        e.mem.set_f32(form.addr() + 0x48, 0.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 1u32]).u8(), 5);
        // Positive 0, negative 3.
        e.mem.set_f32(form.addr() + 0x44, 0.0);
        e.mem.set_f32(form.addr() + 0x48, 80.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 2u32]).u8(), 6);
        // Positive 1, negative 1.
        e.mem.set_f32(form.addr() + 0x44, 30.0);
        e.mem.set_f32(form.addr() + 0x48, 30.0);
        assert_eq!(e.call(0x0061_6a90, &args![form, 0u32]).u8(), 3);
    }
}
