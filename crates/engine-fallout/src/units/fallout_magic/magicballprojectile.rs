//! `fallout/magic/magicballprojectile.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions here are on the player's movement path: they reach the Havok character
//! proxy of a `bhkCharacterController` (`0080fb00` is what `MoveAsProjectile` runs first).

#[allow(unused_imports)]
use crate::prelude::*;

/// The accessor `004ae750` forwards to for a non-null object (`00620b80`, `this` only; it
/// is a thin entry for `0044ddc0`, listed under `cameracaster.cpp` in the engine map).
const PROXY_OF_OBJECT: u32 = 0x0062_0b80;
/// `hkpCharacterProxy` linear velocity setter (`00cac960`, `this` = the proxy, one argument:
/// the address of a four-float vector).
const PROXY_SET_LINEAR_VELOCITY: u32 = 0x00ca_c960;

/// `004a3e00` (unnamed in the engine map): `cdecl`, two stack words (destination, source). Reads
/// three floats from the source, scales each by `004a3e90` and writes them to the destination's
/// first three slots (through `00560d30`), then writes `0.0` to the fourth.
const VECTOR_SCALE_COPY: u32 = 0x004a_3e00;
/// `00c6e050` (unnamed in the engine map, `bhkcharactercontroller.obj`): `thiscall`, `RET 8`.
const CHARACTER_CONTROLLER_CALL: u32 = 0x00c6_e050;
/// `00629670` (unnamed in the engine map): `thiscall`, `RET 8`; with a non-zero flag it ORs the
/// mask into the dword at `+4`, otherwise it hands the mask to `006296b0`.
const FLAG_SET_OR_CLEAR: u32 = 0x0062_9670;
/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const READ_FIRST_WORD: u32 = 0x0055_9450;
/// `bhkWorldObject::GetAVObject` (Xbox PDB, `00c85c80`, `RET 4`).
const WORLD_OBJECT_GET_AV_OBJECT: u32 = 0x00c8_5c80;

// Translated from 00810510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: hands `this` (as the destination) and the
/// pointer it was given (the source) to `004a3e00`, the scaled vector copy.
pub fn fn_00810510(e: &mut Engine, this: Ptr, source: Ptr) {
    e.call(VECTOR_SCALE_COPY, &args![this, source]);
}

// Translated from 00810550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: copies the float argument to `+0x8c` of `this`
/// with `FLD`/`FSTP` (a signalling NaN is stored quiet).
pub fn fn_00810550(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_u32(
        this.addr().wrapping_add(0x8c),
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 00810610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: calls `00629670` on the object at `this + 0x410`
/// with the mask `0x100000` and the flag byte it was given (zero-extended).
pub fn fn_00810610(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(
        FLAG_SET_OR_CLEAR,
        &args![this.addr().wrapping_add(0x410), 0x0010_0000u32, flag],
    );
}

// Translated from 00810640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`: calls `00c6e050` on `this` with `1000` and the
/// word it was given.
pub fn fn_00810640(e: &mut Engine, this: Ptr, value: u32) {
    e.call(CHARACTER_CONTROLLER_CALL, &args![this, 1000u32, value]);
}

// Translated from 00810660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: the object at `*(this + 0x594)` (read through
/// `00559450`), or null when that is null; otherwise what `bhkWorldObject::GetAVObject`
/// (`00c85c80`) answers for it with a zero argument. The decompiler hangs the zero on the wrong
/// call; the disassembly pushes it for `GetAVObject`.
pub fn fn_00810660(e: &mut Engine, this: Ptr) -> u32 {
    let slot = this.addr().wrapping_add(0x594);
    if e.call(READ_FIRST_WORD, &args![slot]).u32() == 0 {
        return 0;
    }
    let object = e.call(READ_FIRST_WORD, &args![slot]).u32();
    e.call(WORLD_OBJECT_GET_AV_OBJECT, &args![object, 0u32])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0081_0510, fn_00810510(Ptr, Ptr)),
        entry!(0x0081_0550, fn_00810550(Ptr, f32)),
        entry!(0x0081_0610, fn_00810610(Ptr, u8)),
        entry!(0x0081_0640, fn_00810640(Ptr, u32)),
        entry!(0x0081_0660, fn_00810660(Ptr) -> u32),
        entry!(0x004ae750, bhk_character_proxy_operator_p(Ptr) -> u32),
        entry!(0x0080fb00, fn_0080fb00(Ptr, Ptr)),
    ]
}

// Translated from 004ae750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::operatorP` (name from the engine map): null when `this` is null,
/// otherwise what `00620b80` answers for it.
pub fn bhk_character_proxy_operator_p(e: &mut Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        e.call(PROXY_OF_OBJECT, &args![this]).u32()
    }
}

// Translated from 0080fb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Looks up the proxy of `this` (`004ae750`) and, when there is
/// one, runs the proxy's linear velocity setter (`00cac960`) with `vector`.
pub fn fn_0080fb00(e: &mut Engine, this: Ptr, vector: Ptr) {
    let proxy = bhk_character_proxy_operator_p(e, this);
    if proxy != 0 {
        e.call(PROXY_SET_LINEAR_VELOCITY, &args![proxy, vector]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.register(PROXY_OF_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        e.register(PROXY_SET_LINEAR_VELOCITY, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn proxy_operator_is_null_for_null_and_forwards_otherwise() {
        let mut e = rig();
        assert_eq!(e.call(0x004a_e750, &args![0u32]).u32(), 0);
        assert!(calls(&e, PROXY_OF_OBJECT).is_empty());
        assert_eq!(e.call(0x004a_e750, &args![0x4000u32]).u32(), 0x4100);
        assert_eq!(calls(&e, PROXY_OF_OBJECT), vec![vec![0x4000]]);
    }

    #[test]
    fn set_velocity_runs_only_when_a_proxy_exists() {
        let mut e = rig();
        e.call(0x0080_fb00, &args![0u32, 0x9000u32]);
        assert!(calls(&e, PROXY_SET_LINEAR_VELOCITY).is_empty());
        e.call(0x0080_fb00, &args![0x4000u32, 0x9000u32]);
        assert_eq!(
            calls(&e, PROXY_SET_LINEAR_VELOCITY),
            vec![vec![0x4100, 0x9000]]
        );
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00810510_passes_this_as_destination_and_the_argument_as_source() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x004a_3e00, |_, _| Ret::default());
        e.call(0x0081_0510, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls(&e, 0x004a_3e00), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn at_00810550_stores_the_float_quiet_at_offset_0x8c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.call(0x0081_0550, &args![this, 2.5f32]);
        assert_eq!(e.mem.u32(this + 0x8c), 2.5f32.to_bits());
        // A signalling NaN comes out quiet (FLD/FSTP).
        e.call(0x0081_0550, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x8c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x88), 0);
        assert_eq!(e.mem.u32(this + 0x90), 0);
    }

    #[test]
    fn at_00810610_flags_the_object_at_offset_0x410_with_the_mask() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x0062_9670, |_, _| Ret::default());
        e.call(0x0081_0610, &args![0x4000u32, 1u8]);
        e.call(0x0081_0610, &args![0x4000u32, 0u8]);
        assert_eq!(
            calls(&e, 0x0062_9670),
            vec![vec![0x4410, 0x0010_0000, 1], vec![0x4410, 0x0010_0000, 0]]
        );
    }

    #[test]
    fn at_00810640_calls_the_controller_with_1000_and_the_word() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c6_e050, |_, _| Ret::default());
        e.call(0x0081_0640, &args![0x4000u32, 0xfeed_beefu32]);
        assert_eq!(
            calls(&e, 0x00c6_e050),
            vec![vec![0x4000, 1000, 0xfeed_beef]]
        );
    }

    #[test]
    fn at_00810660_answers_null_without_an_object_and_the_av_object_with_one() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x600);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x00c8_5c80, |_, a| (a[0] + 0x10).into_ret());
        assert_eq!(e.call(0x0081_0660, &args![this]).u32(), 0);
        assert!(calls(&e, 0x00c8_5c80).is_empty());
        e.mem.set_u32(this + 0x594, 0x7000);
        assert_eq!(e.call(0x0081_0660, &args![this]).u32(), 0x7010);
        assert_eq!(calls(&e, 0x00c8_5c80), vec![vec![0x7000, 0]]);
        assert_eq!(calls(&e, 0x0055_9450).last(), Some(&vec![this + 0x594]));
    }
}
