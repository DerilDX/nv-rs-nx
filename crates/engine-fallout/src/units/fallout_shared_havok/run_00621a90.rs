//! `fallout shared/havok/run_00621a90` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00621a90`: thiscall on the same object; its result is the object the
/// next call runs on.
const FIRST_CALL: u32 = 0x0062_1a90;
/// `0044ddc0`: thiscall on that result; its result is the argument of
/// `00621480`.
const SECOND_CALL: u32 = 0x0044_ddc0;
/// `00621480`: cdecl, one word; the result is left in EAX.
const FINAL_CALL: u32 = 0x0062_1480;

// Translated from 00621ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall`: `00621480(0044ddc0(00621a90(this)))`, with the
/// callees run on the previous result as their `this`. Returns what
/// `00621480` returns.
pub fn fn_00621ad0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.call(FIRST_CALL, &args![this]).u32();
    let second = e.call(SECOND_CALL, &args![first]).u32();
    e.call(FINAL_CALL, &args![second]).u32()
}

/// `00620b80` (the engine map: `cameracaster.cpp` range): `thiscall`; a thin entry for
/// `0044ddc0`; its result is null or the object `00cac980` reads.
const PROXY_OF_OBJECT_00620B80: u32 = 0x0062_0b80;
/// `00cac980` (unnamed in the engine map, `hkpcharacterproxy.cpp`): `thiscall`; the dword at
/// `this + 0x30`.
const PROXY_FIELD_AT_30: u32 = 0x00ca_c980;

// Translated from 00621a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: null when `00620b80(this)` answers null; otherwise
/// what `00cac980` answers for it.
pub fn fn_00621a90(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(PROXY_OF_OBJECT_00620B80, &args![this]).u32();
    if proxy == 0 {
        0
    } else {
        e.call(PROXY_FIELD_AT_30, &args![proxy]).u32()
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_1a90, fn_00621a90(Ptr) -> u32),
        entry!(0x00621ad0, fn_00621ad0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chains_the_three_calls_and_returns_the_last_result() {
        let mut e = Engine::new();
        e.register(FIRST_CALL, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(SECOND_CALL, |_, a| Ret {
            eax: a[0] * 10,
            ..Ret::default()
        });
        e.register(FINAL_CALL, |_, a| Ret {
            eax: a[0] + 5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0062_1ad0, &args![Ptr::<()>::new(2)]).u32(), 35);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00621a90_answers_the_proxy_field_or_null() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(PROXY_OF_OBJECT_00620B80, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PROXY_FIELD_AT_30, |_, a| (a[0] + 0x30).into_ret());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0062_1a90, &args![this]).u32(), 0);
        let hits = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == PROXY_FIELD_AT_30)
            .count();
        assert_eq!(hits, 0);
        e.mem.set_u32(this, 0x7000);
        assert_eq!(e.call(0x0062_1a90, &args![this]).u32(), 0x7030);
    }
}
