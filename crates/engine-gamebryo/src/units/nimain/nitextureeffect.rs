//! `NiMain/nitextureeffect.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTextureEffect::ms_RTTI` (the class's `NiRTTI` record).
const NI_TEXTURE_EFFECT_RTTI: u32 = 0x011f_5e94;

// Translated from 00a927d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_texture_effect_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TEXTURE_EFFECT_RTTI)
}

use crate::units::nimain::nimainsdm::set_literal_fixed_string;

/// The static fixed strings `00a927e0` sets: (global, address of the literal), in order.
const STATIC_STRINGS: [(u32, u32); 4] = [
    (0x011f_5e9c, 0x0109_fe10),
    (0x011f_5ea0, 0x0109_fe00),
    (0x011f_5ea4, 0x0109_fdf0),
    (0x011f_5ea8, 0x0109_fde8),
];

// Translated from 00a927e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no arguments):
/// sets four static fixed strings from literals (`STATIC_STRINGS`), each only when the
/// global does not hold the literal's address already.
pub fn ni_texture_effect_sdm_init(e: &mut Engine) {
    for (slot, literal) in STATIC_STRINGS {
        set_literal_fixed_string(e, slot, literal);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a927d0, ni_texture_effect_get_rtti() -> Ptr),
        entry!(0x00a9_27e0, ni_texture_effect_sdm_init()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_texture_effect_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a927d0, &args![]).u32(), NI_TEXTURE_EFFECT_RTTI);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_the_four_strings_unless_already_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        for (slot, _) in STATIC_STRINGS {
            e.mem.set_u32(slot, empty);
        }
        e.mem.set_u32(0x011f_5ea0, 0x0109_fe00);
        e.call_log = Some(vec![]);
        e.call(0x00a9_27e0, &args![]);
        assert_eq!(e.mem.u32(0x011f_5e9c), 0x010a_0e10);
        assert_eq!(e.mem.u32(0x011f_5ea0), 0x0109_fe00);
        assert_eq!(e.mem.u32(0x011f_5ea4), 0x010a_0df0);
        assert_eq!(e.mem.u32(0x011f_5ea8), 0x010a_0de8);
        assert_eq!(
            calls_to(&e, GET_FIXED_STRING),
            vec![vec![0x0109_fe10], vec![0x0109_fdf0], vec![0x0109_fde8]]
        );
    }
}
