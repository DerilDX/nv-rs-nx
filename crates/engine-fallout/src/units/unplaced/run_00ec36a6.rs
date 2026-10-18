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

/// The call at the end (`00ad0470`: the stream base at `this + offset` plus two stack words,
/// `RET 8`). The engine map puts it in `FaceGen/algs.obj`; its identity is unconfirmed.
const STREAM_BASE_INIT_00AD0470: u32 = 0x00ad_0470;
/// Vbtable stored at `+0` and at `+4` when the most-derived flag is set; the objects they
/// point at have RTTI names `basic_ostream` (`010a318c`) and `basic_ios` (`010a2fa4`).
const VBTABLE_010A3190: u32 = 0x010a_3190;
const VBTABLE_010A2FA4: u32 = 0x010a_2fa4;
/// The vftable stored in the virtual base subobject (RTTI class
/// `basic_ostream<char,std::char_traits<char>>`).
const VFTABLE_010A318C: u32 = 0x010a_318c;

// Translated from 00ec3a75 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec3a75 - the compiler's exception frame (`__EH_prolog3` at
// `00ece255`, `__EH_epilog3` at `00ece2be`: FS:[0] chain) is not translated.
/// Unnamed in the engine map: the shape of a `std::basic_ostream` constructor with a virtual
/// base. When `most_derived` is non-zero it stores the vbtables `010a3190` at `+0` and
/// `010a2fa4` at `+4`; it then reads the virtual-base offset from `[[this] + 4]`, stores the
/// vftable `010a318c` at `this + offset` and calls `00ad0470(this + offset, first, second)`.
/// Returns `this`.
pub fn fn_00ec3a75(e: &mut Engine, this: Ptr, first: u32, second: u32, most_derived: u32) -> Ptr {
    let base = this.addr();
    if most_derived != 0 {
        e.mem.set_u32(base, VBTABLE_010A3190);
        e.mem.set_u32(base + 4, VBTABLE_010A2FA4);
    }
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.mem.set_u32(base.wrapping_add(offset), VFTABLE_010A318C);
    let offset = e.mem.u32(e.mem.u32(base) + 4);
    e.call(
        STREAM_BASE_INIT_00AD0470,
        &args![base.wrapping_add(offset), first, second],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec408c, security_check_cookie(u32)),
        entry!(0x00ec3bda, fn_00ec3bda(u32) -> u32),
        entry!(0x00ec_3a75, fn_00ec3a75(Ptr, u32, u32, u32) -> Ptr),
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

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_the_vbtables_and_initialises_the_virtual_base() {
        let mut e = Engine::new();
        e.put_vtable(VBTABLE_010A3190, &[0, 0x10]);
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_3a75, &args![this, 0x11u32, 0x22u32, 1u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this), VBTABLE_010A3190);
        assert_eq!(e.mem.u32(this + 4), VBTABLE_010A2FA4);
        assert_eq!(e.mem.u32(this + 0x10), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x10, 0x11, 0x22]]
        );
    }

    #[test]
    fn keeps_the_existing_vbtable_when_not_most_derived() {
        let mut e = Engine::new();
        e.register(STREAM_BASE_INIT_00AD0470, |_, _| Ret::default());
        let vbtable = e.mem.alloc(8);
        e.mem.set_u32(vbtable + 4, 0x18);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, vbtable);
        e.mem.set_u32(this + 4, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00ec_3a75, &args![this, 1u32, 2u32, 0u32]);
        assert_eq!(e.mem.u32(this), vbtable);
        assert_eq!(e.mem.u32(this + 4), 0x77);
        assert_eq!(e.mem.u32(this + 0x18), VFTABLE_010A318C);
        assert_eq!(
            calls_to(&mut e, STREAM_BASE_INIT_00AD0470),
            vec![vec![this + 0x18, 1, 2]]
        );
    }
}
