//! `fallout shared/tesaiform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0047ede0` calls on the member (`0067e400`).
const CALLEE_0067E400: u32 = 0x0067_e400;

// Translated from 0047ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESAIForm::ClearDataComponent` (Xbox PDB name in the engine map). Thin wrapper: calls `0067e400` with `ECX` = `this + 0x18` and passes on nothing (the callee returns no value).
pub fn fn_0047ede0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0067E400, &args![this.byte_add(0x18)]);
}

/// The four byte setters of the AI data (`0047ee00`, `0047eed0`, `0047ef00`, `0047ef50`):
/// store `value` at `offset` and, when `notify` is not zero, run `0047f790` with 8 on the same
/// object.
fn set_ai_byte(e: &mut Engine, this: Ptr, offset: u32, value: u8, notify: u8) {
    e.mem.set_u8(this.addr() + offset, value);
    if notify != 0 {
        e.call(0x0047_f790, &args![this, 8u32]);
    }
}

// Translated from 0047ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x4` of the AI data (the part of `TESAIForm` at `+0x90` of an actor base)
/// and, when `notify` is not zero, runs `0047f790` with 8. The engine map gives it no name.
pub fn fn_0047ee00(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 4, value, notify);
}

// Translated from 0047eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x5` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047eed0(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 5, value, notify);
}

// Translated from 0047ef00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x6` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047ef00(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 6, value, notify);
}

// Translated from 0047ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x7` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047ef50(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 7, value, notify);
}

// Translated from 0047f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` is in `0x20..=0x2d` (signed compare). The engine map gives it no name.
pub fn fn_0047f060(_e: &mut Engine, value: i32) -> bool {
    (0x20..0x2e).contains(&value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_ede0, fn_0047ede0(Ptr)),
        entry!(0x0047_ee00, fn_0047ee00(Ptr, u8, u8)),
        entry!(0x0047_eed0, fn_0047eed0(Ptr, u8, u8)),
        entry!(0x0047_ef00, fn_0047ef00(Ptr, u8, u8)),
        entry!(0x0047_ef50, fn_0047ef50(Ptr, u8, u8)),
        entry!(0x0047_f060, fn_0047f060(i32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0047ede0_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0067E400, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_ede0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0067E400).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    /// Calls the setter at `addr` and returns the object's bytes and the logged calls.
    #[allow(clippy::type_complexity)]
    fn run(addr: u32, value: u32, notify: u32) -> (Vec<u8>, Vec<(u32, Vec<u32>)>, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        e.register(0x0047_f790, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(addr, &[this, value, notify]);
        let log = e.call_log.take().unwrap();
        (e.mem.bytes(this, 0x10), log[1..].to_vec(), this)
    }

    #[test]
    fn the_four_setters_store_their_byte_and_notify_only_when_asked() {
        for (addr, offset) in [
            (0x0047_ee00, 4),
            (0x0047_eed0, 5),
            (0x0047_ef00, 6),
            (0x0047_ef50, 7),
        ] {
            let (bytes, calls, _) = run(addr, 0x1_0032, 0);
            let mut want = vec![0xcd_u8; 0x10];
            want[offset] = 0x32;
            assert_eq!(bytes, want, "{addr:08x}");
            assert!(calls.is_empty());

            let (bytes, calls, this) = run(addr, 7, 0x100_0001);
            want[offset] = 7;
            assert_eq!(bytes, want, "{addr:08x}");
            assert_eq!(calls, [(0x0047_f790, vec![this, 8])]);

            // Only the low byte of the flag word counts.
            let (_, calls, _) = run(addr, 7, 0x100);
            assert!(calls.is_empty());
        }
    }

    #[test]
    fn fn_0047f060_accepts_0x20_to_0x2d() {
        let mut e = Engine::new();
        for (value, want) in [
            (0x1f, false),
            (0x20, true),
            (0x2d, true),
            (0x2e, false),
            (-1, false),
            (i32::MIN, false),
        ] {
            assert_eq!(e.call(0x0047_f060, &[value as u32]).bool(), want, "{value}");
        }
    }
}
