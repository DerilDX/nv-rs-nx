//! `fallout shared/bgsterminal.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00500940 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns receiver +0x18 without reading memory. The source-unit mapping
/// is by address range; no Xbox PDB name is available for this accessor.
/// Script callers use it to obtain their embedded header.
pub fn fn_00500940(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x18))
}

/// The string constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// The thread-local word `004fffe0` stores (`+0x2b8` of the TLS block).
const TLS_WORD: u32 = 0x2b8;

// Translated from 004fffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at `+0x2b8` of the calling thread's TLS block
/// (`*(FS:[0x2c])[_tls_index] + 0x2b8`). cdecl, one word. The quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_004fffe0(e: &mut Engine, value: u32) {
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_WORD, value);
}

// Translated from 00500910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an object of the `SCRIPT_REFERENCED_OBJECT` layout (Xbox PDB):
/// the editor ID string at `+0` (`004037b0`), then `pForm` (`+8`) and
/// `uiVariableID` (`+0xc`) zeroed. Returns `this`. The engine map's name
/// (`BSSimpleList<SCRIPT_REFERENCED_OBJECT_P>::AddTail`) belongs to folded
/// code; the body is a constructor. The stage item loader `0060f6a0` calls it.
pub fn fn_00500910(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0);
    this
}

// Translated from 005010c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005010f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005010c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_10f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004fffe0, fn_004fffe0(u32)),
        entry!(0x00500910, fn_00500910(Ptr) -> Ptr),
        entry!(0x00500940, fn_00500940(Ptr) -> Ptr),
        entry!(0x0050_10c0, fn_005010c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_word_takes_the_value() {
        let mut e = Engine::new();
        e.call(0x004f_ffe0, &[0x1234_5678]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_WORD), 0x1234_5678);
        e.call(0x004f_ffe0, &[0]);
        assert_eq!(e.mem.u32(tls + TLS_WORD), 0);
    }

    #[test]
    fn referenced_object_constructor_builds_the_string_and_zeroes() {
        let mut e = Engine::new();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        for offset in [8, 0xc] {
            e.mem.set_u32(object + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_0910, &[object]).u32(), object);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(STRING_CONSTRUCT, vec![object])]);
    }

    #[test]
    fn header_address_matches_native_without_dereferencing_receiver() {
        // Five native cases on unpacked 1.4.0.525, SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f:
        // EAX is receiver+24 modulo 2^32, RET 0 and no faults.
        let mut e = Engine::new();
        for receiver in [0u32, 1, 0x1000_0000, u32::MAX - 15, u32::MAX] {
            assert_eq!(
                e.call(0x00500940, &[receiver]).u32(),
                receiver.wrapping_add(24)
            );
        }
    }
    #[test]
    fn at_005010c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_10f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_10c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_10c0)
                .collect();
            let mut expected = vec![(0x0050_10f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
