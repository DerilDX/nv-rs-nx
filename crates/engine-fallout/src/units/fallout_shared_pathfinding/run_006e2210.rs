//! `fallout shared/pathfinding/run_006e2210` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006e22d0` calls on the member (`004ff7e0`).
const CALLEE_004FF7E0: u32 = 0x004f_f7e0;

// Translated from 006e22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `004ff7e0` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_006e22d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_004FF7E0, &args![this.byte_add(0x4)]);
}

/// `PathingLocation::PathingLocation_ov2` (engine map), `thiscall`, one
/// word.
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cc40;
/// `00865e50`: saves bytes into the save buffer (`thiscall`: data, size,
/// 0).
const SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (engine map), `thiscall`, two words.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `TESObjectREFR::Activate` (engine map), `thiscall`, four words.
const REFR_ACTIVATE: u32 = 0x0057_3170;

// Translated from 006e2280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: constructor: the flags word at `+0` cleared, the pathing
/// location at `+4` from `location` (`PathingLocation_ov2`), the point at
/// `+0x2c` copied from `point`, the reference at `+0x38` cleared. Returns
/// `this`.
pub fn fn_006e2280(e: &mut Engine, this: Ptr, location: u32, point: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.call(PATHING_LOCATION_CONSTRUCT, &args![base + 4, location]);
    for i in 0..3 {
        let word = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(base + 0x2c + 4 * i, word);
    }
    e.mem.set_u32(base + 0x38, 0);
    this
}

// Translated from 006e22f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: when [`fn_006e2330`] holds, activates the reference at
/// `+0x38` for `activator` (`TESObjectREFR::Activate(activator, 0, 0, 1)`)
/// and returns its result; false otherwise.
pub fn fn_006e22f0(e: &mut Engine, this: Ptr, activator: u32) -> bool {
    if !fn_006e2330(e, this) {
        return false;
    }
    let reference = e.mem.u32(this.addr() + 0x38);
    e.call(
        REFR_ACTIVATE,
        &args![reference, activator, 0u32, 0u32, 1u32],
    )
    .bool()
}

// Translated from 006e2330 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: whether the low two bits of the word at `+0` are both set.
pub fn fn_006e2330(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr()) & 3 == 3
}

// Translated from 006e2350 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: saves the object into `buffer`: the flags word (4 bytes),
/// the point at `+0x2c` (12 bytes), the reference at `+0x38`'s form id,
/// then the pathing location at `+4` through its vtable slot 0.
pub fn fn_006e2350(e: &mut Engine, this: Ptr, buffer: u32) {
    let base = this.addr();
    e.call(SAVE_BYTES, &args![buffer, base, 4u32, 0u32]);
    e.call(SAVE_BYTES, &args![buffer, base + 0x2c, 0xcu32, 0u32]);
    let reference = e.mem.u32(base + 0x38);
    e.call(SAVE_FORM_ID, &args![buffer, reference, 0u32]);
    e.vcall(base + 4, 0, &args![buffer]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_22d0, fn_006e22d0(Ptr)),
        entry!(0x006e_2280, fn_006e2280(Ptr, u32, Ptr) -> Ptr),
        entry!(0x006e_22f0, fn_006e22f0(Ptr, u32) -> bool),
        entry!(0x006e_2330, fn_006e2330(Ptr) -> bool),
        entry!(0x006e_2350, fn_006e2350(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006e22d0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_004FF7E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_22d0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_004FF7E0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }
}

#[cfg(test)]
mod b040_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn constructor_fills_location_and_point() {
        let mut e = Engine::new();
        e.register(PATHING_LOCATION_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.write(this, &[0xff; 0x3c]);
        let point = e.mem.alloc(12);
        e.mem.set_u32(point, 1);
        e.mem.set_u32(point + 8, 3);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006e_2280, &args![this, 0x5500u32, point]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 1);
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(
            calls_to(&e, PATHING_LOCATION_CONSTRUCT),
            vec![vec![this + 4, 0x5500]]
        );
    }

    #[test]
    fn activates_only_with_both_bits() {
        let mut e = Engine::new();
        e.register(REFR_ACTIVATE, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x38, 0x6600);
        e.mem.set_u32(this, 2);
        assert!(!e.call(0x006e_2330, &args![this]).bool());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x006e_22f0, &args![this, 0x14u32]).bool());
        assert!(calls_to(&e, REFR_ACTIVATE).is_empty());
        e.mem.set_u32(this, 7);
        assert!(e.call(0x006e_2330, &args![this]).bool());
        assert!(e.call(0x006e_22f0, &args![this, 0x14u32]).bool());
        assert_eq!(
            calls_to(&e, REFR_ACTIVATE),
            vec![vec![0x6600, 0x14, 0, 0, 1]]
        );
    }

    #[test]
    fn save_writes_flags_point_reference_and_location() {
        let mut e = Engine::new();
        e.register(SAVE_BYTES, |_, _| Ret::default());
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(0x7b00_0000, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, 0x7b00_0000);
        e.mem.set_u32(this + 4, table);
        e.mem.set_u32(this + 0x38, 0x6600);
        e.call_log = Some(vec![]);
        e.call(0x006e_2350, &args![this, 0x9900u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (SAVE_BYTES, vec![0x9900, this, 4, 0]),
                (SAVE_BYTES, vec![0x9900, this + 0x2c, 0xc, 0]),
                (SAVE_FORM_ID, vec![0x9900, 0x6600, 0]),
                (0x7b00_0000, vec![this + 4, 0x9900]),
            ]
        );
    }
}
