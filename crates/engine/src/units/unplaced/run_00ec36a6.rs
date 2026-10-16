//! `(unplaced)/run_00ec36a6` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec3bda (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the two bytes of the low half (unnamed in the map; a runtime
/// library function, `cdecl`, one word; the half-word swap `00407a90`
/// calls it). The whole of `EAX` is the result: the low half is the swapped
/// half (`(value << 8) + (value >> 8)` in 16 bits), the high half is bits 8
/// to 23 of the argument, which callers ignore.
pub fn fn_00ec3bda(_e: &mut Engine, value: u32) -> u32 {
    let shifted = value << 8;
    let low = (shifted as u16).wrapping_add((value as u16) >> 8);
    (shifted & 0xffff_0000) | low as u32
}

/// `___report_gsfailure` (`00ed159c`): the stack-cookie failure handler; it
/// does not return.
const REPORT_GS_FAILURE: u32 = 0x00ed_159c;
/// The security cookie (`.data`).
const SECURITY_COOKIE_011C16BC: u32 = 0x011c_16bc;

// Translated from 00ec408c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__security_check_cookie` (CRT, `__fastcall`): returns when `cookie`
/// equals the security cookie at `011c16bc`, otherwise jumps to
/// `___report_gsfailure` (`00ed159c`), which never returns.
pub fn security_check_cookie(e: &mut Engine, cookie: u32) {
    if cookie == e.global::<u32>(SECURITY_COOKIE_011C16BC) {
        return;
    }
    e.call(REPORT_GS_FAILURE, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec408c, security_check_cookie(u32)),
        entry!(0x00ec3bda, fn_00ec3bda(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_low_half_and_keeps_the_shifted_high_half() {
        let mut e = Engine::new();
        let mut swap = |value: u32| e.call(0x00ec_3bda, &args![value]).u32();
        assert_eq!(swap(0x0000_1234), 0x0012_3412);
        assert_eq!(swap(0x0000_ff01), 0x00ff_01ff);
        assert_eq!(swap(0xabcd_1234), 0xcd12_3412);
        assert_eq!(swap(0), 0);
        assert_eq!(swap(0x0000_00ff) as u16, 0xff00);
    }

    #[test]
    fn security_check_cookie_returns_on_a_match_and_reports_otherwise() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.set_global(SECURITY_COOKIE_011C16BC, 0xdead_beefu32);
        e.register_double(REPORT_GS_FAILURE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beefu32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_408c, &args![0xdead_beeeu32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == REPORT_GS_FAILURE)
                .count(),
            1
        );
    }
}
