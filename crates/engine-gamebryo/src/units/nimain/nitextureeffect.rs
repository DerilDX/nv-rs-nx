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

/// `00a5b690` (`cdecl`, one word): the fixed-string handle for a C string (null gives the
/// null handle).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_STRING_HANDLE: u32 = 0x0109_b220;

// Translated from 00a928b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitextureeffect.cpp`, no arguments; one of the `NiMain`
/// shutdown steps of `00a8a280`): for each of the four static fixed strings that
/// `NiTextureEffect::_SDMInit` sets (`011f5e9c`, `011f5ea0`, `011f5ea4`, `011f5ea8`, in that
/// order) that is not null, stores the handle `00a5b690(0)` gives and then releases the old
/// handle (`InterlockedDecrement` of its count 8 bytes before it) unless it is the empty
/// string's. The decompiler shows the empty-string comparison as a comparison with 0; the
/// disassembly compares with the global `0109b220`.
pub fn fn_00a928b0(e: &mut Engine) {
    for (slot, _) in STATIC_STRINGS {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(FIXED_STRING_FROM_TEXT, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            if old != e.global::<u32>(EMPTY_STRING_HANDLE) {
                e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a927d0, ni_texture_effect_get_rtti() -> Ptr),
        entry!(0x00a9_27e0, ni_texture_effect_sdm_init()),
        entry!(0x00a9_28b0, fn_00a928b0()),
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

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a928b0_clears_the_set_strings_and_releases_the_counted_ones() {
        let mut e = Engine::new();
        e.map(EMPTY_STRING_HANDLE & !0xfff, 0x1000);
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(EMPTY_STRING_HANDLE, 0x4444);
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x7770u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.mem.set_u32(0x011f_5e9c, 0x1008);
        e.mem.set_u32(0x011f_5ea0, 0);
        e.mem.set_u32(0x011f_5ea4, 0x4444);
        e.mem.set_u32(0x011f_5ea8, 0x2008);
        e.call_log = Some(vec![]);
        e.call(0x00a9_28b0, &args![]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a9_28b0)
            .collect();
        assert_eq!(
            log,
            vec![
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (INTERLOCKED_DECREMENT, vec![0x1000]),
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (INTERLOCKED_DECREMENT, vec![0x2000]),
            ]
        );
        assert_eq!(e.mem.u32(0x011f_5e9c), 0x7770);
        assert_eq!(e.mem.u32(0x011f_5ea0), 0);
        assert_eq!(e.mem.u32(0x011f_5ea4), 0x7770);
        assert_eq!(e.mem.u32(0x011f_5ea8), 0x7770);
    }
}
