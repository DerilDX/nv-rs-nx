//! `NiAnimation/niboolinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4228;

// Translated from 00a577b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_INTERP_CONTROLLER_RTTI)
}

/// `00a39010`: the jump target of `00a57c00`.
const JUMP_TARGET_00A57C00: u32 = 0x00a3_9010;

/// `00a39030`: the jump target of `00a57c10`.
const JUMP_TARGET_00A57C10: u32 = 0x00a3_9030;

// Translated from 00a57c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a39010` (engine map: `NiSingleInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiBoolInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_interp_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57C00, &args![this, stream]);
}

// Translated from 00a57c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::IsEqual` (Xbox PDB): a 5-byte jump to `00a39030` (engine map: `NiSingleInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiBoolInterpController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_bool_interp_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57C10, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a577b0, ni_bool_interp_controller_get_rtti() -> Ptr),
        entry!(0x00a5_7c00, ni_bool_interp_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a5_7c10, ni_bool_interp_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a3_8e00, fn_00a38e00(u32) -> u32),
        entry!(
            0x00a5_7710,
            ni_bool_interp_controller_create_pose_interpolator(u32, u32) -> u32
        ),
    ]
}

/// `00a2f8c0` (`thiscall`, no stack words): the `NiInterpController` base constructor, by
/// its use here.
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
/// The `NiSingleInterpController` vtable (RTTI).
const SINGLE_INTERP_CONTROLLER_VTABLE: u32 = 0x0109_6bd4;
/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiBoolInterpolator::NiBoolInterpolator` (engine map, `00a532d0`; `thiscall`, one word).
const BOOL_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_32d0;

// Translated from 00a38e00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed by its body (the engine map gives `NiBoolInterpController::CopyMembers`, folded
/// code; by the vtable it stores the `NiSingleInterpController` constructor; `thiscall`, no
/// stack words): the base constructor `00a2f8c0`, the `NiSingleInterpController` vtable,
/// no interpolator (+0x34). Answers `this`.
pub fn fn_00a38e00(e: &mut Engine, this: u32) -> u32 {
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this, SINGLE_INTERP_CONTROLLER_VTABLE);
    e.mem.set_u32(this + 0x34, 0);
    this
}

// Translated from 00a57710 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::CreatePoseInterpolator` (Xbox PDB; `thiscall`, one word it does
/// not read, `RET 4`): asks `this` for the target value (vtable slot +0xEC with the address
/// of a stack word, which starts out holding `this` from the `PUSH ECX`), then answers a
/// 0x18-byte `NiBoolInterpolator` built with that whole word, or null when `operator new`
/// fails.
pub fn ni_bool_interp_controller_create_pose_interpolator(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
) -> u32 {
    e.with_stack(4, |e, value| {
        let value = value.addr();
        e.mem.set_u32(value, this);
        e.vcall(this, 0xec, &args![value]);
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        if block == 0 {
            return 0;
        }
        let word = e.mem.u32(value);
        e.call(BOOL_INTERPOLATOR_CONSTRUCT, &args![block, word])
            .u32()
    })
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    #[test]
    fn fn_00a38e00_builds_the_single_interp_controller() {
        let mut e = Engine::new();
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x34, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a3_8e00, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), SINGLE_INTERP_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(INTERP_CONTROLLER_CONSTRUCT, vec![this])));
    }

    #[test]
    fn create_pose_interpolator_passes_the_target_value() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(table + 0xec, 0x0098_0000);
        e.mem.set_u32(this, table);
        // The slot writes only the low byte, as a bool.
        e.register(0x0098_0000, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| 0x6000u32.into_ret());
        e.register(BOOL_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_7710, &[this, 0]).u32(), 0x6000);
        let built: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == BOOL_INTERPOLATOR_CONSTRUCT)
            .collect();
        assert_eq!(
            built,
            vec![(
                BOOL_INTERPOLATOR_CONSTRUCT,
                vec![0x6000, (this & !0xff) | 1]
            )]
        );
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_7710, &[this, 0]).u32(), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a577b0, &args![]).u32(),
            NI_BOOL_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a57c00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7c00;
        const TARGET: u32 = 0x00a3_9010;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a57c10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7c10;
        const TARGET: u32 = 0x00a3_9030;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
