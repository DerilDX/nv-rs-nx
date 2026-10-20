//! `NiAnimation/nisingleinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSingleInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_SINGLE_INTERP_CONTROLLER_RTTI: u32 = 0x011f_3714;

// Translated from 00a38e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_single_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SINGLE_INTERP_CONTROLLER_RTTI)
}

/// `00c9a6b0`: the jump target of `00a35850`.
const JUMP_TARGET_00A35850: u32 = 0x00c9_a6b0;

// Translated from 00a35850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::LinkObject` (Xbox PDB): a 5-byte jump to `00c9a6b0` (engine map: `NiInterpController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiSingleInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_single_interp_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35850, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a38e20, ni_single_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a3_5850,
            ni_single_interp_controller_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a3_8e30,
            ni_single_interp_controller_get_interpolator(u32, u32) -> u32
        ),
        entry!(
            0x00a3_8e40,
            ni_single_interp_controller_set_interpolator(u32, u32, u32)
        ),
        entry!(
            0x00a3_8f50,
            ni_single_interp_controller_process_clone(u32, u32)
        ),
        entry!(
            0x00a3_8f80,
            ni_single_interp_controller_load_binary(u32, u32)
        ),
        entry!(
            0x00a3_9080,
            ni_single_interp_controller_get_interpolator_index_fx(u32, u32) -> u16
        ),
    ]
}

/// `NiSingleInterpController::m_spInterpolator` (Xbox PDB +0x34, the same on PC).
const INTERPOLATOR: u32 = 0x34;
/// The imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiTimeController::ProcessClone` (`00a6d0c0`), `NiInterpController::LoadBinary`
/// (`00a2f910`; both `thiscall`, one word) and `NiStream::ResolveLinkID` (`00a64620`;
/// `thiscall` on the stream).
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const INTERP_CONTROLLER_LOAD_BINARY: u32 = 0x00a2_f910;
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The 16-bit "invalid index" answer (`.rdata`, 0xFFFF in the exe).
const INVALID_INDEX: u32 = 0x0109_6320;

/// The inlined `NiPointer` assignment of `object` to `m_spInterpolator`: when it differs,
/// the old one released (`InterlockedDecrement`; `DeleteThis`, slot +4, at zero) and the new
/// one counted up.
fn assign_interpolator(e: &mut Engine, this: u32, object: u32) {
    let old = e.mem.u32(this + INTERPOLATOR);
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 0x4, &[]);
    }
    e.mem.set_u32(this + INTERPOLATOR, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00a38e30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetInterpolator` (Xbox PDB; `thiscall`, the index it does not
/// read, `RET 4`): `m_spInterpolator`.
pub fn ni_single_interp_controller_get_interpolator(
    e: &mut Engine,
    this: u32,
    _unused_1: u32,
) -> u32 {
    e.mem.u32(this + INTERPOLATOR)
}

// Translated from 00a38e40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::SetInterpolator` (Xbox PDB; `thiscall`, (interpolator, index
/// it does not read), `RET 8`): assigns `m_spInterpolator`.
pub fn ni_single_interp_controller_set_interpolator(
    e: &mut Engine,
    this: u32,
    interpolator: u32,
    _unused_2: u32,
) {
    assign_interpolator(e, this, interpolator);
}

// Translated from 00a38f50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::ProcessClone` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiTimeController::ProcessClone`, then the interpolator (when there is one) gets the
/// process too (vtable slot +0x68).
pub fn ni_single_interp_controller_process_clone(e: &mut Engine, this: u32, process: u32) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, process]);
    let interpolator = e.mem.u32(this + INTERPOLATOR);
    if interpolator != 0 {
        e.vcall(interpolator, 0x68, &args![process]);
    }
}

// Translated from 00a38f80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::LoadBinary` (Xbox PDB; `thiscall`, one word, `RET 4`):
/// `NiInterpController::LoadBinary`, then for a file of version 10.1.0.104 or later the
/// interpolator link (`NiStream::ResolveLinkID`) assigned to `m_spInterpolator`.
pub fn ni_single_interp_controller_load_binary(e: &mut Engine, this: u32, stream: u32) {
    e.call(INTERP_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream + 0xd8) < 0x0a01_0068 {
        return;
    }
    let linked = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_interpolator(e, this, linked);
}

// Translated from 00a39080 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiSingleInterpController::GetInterpolatorIndexFx` (Xbox PDB; `thiscall`, the address of
/// a fixed string, `RET 4`): 0 when the target's name (+0x8 of +0x2C) is that string, else
/// the 16-bit value at `01096320`.
pub fn ni_single_interp_controller_get_interpolator_index_fx(
    e: &mut Engine,
    this: u32,
    name: u32,
) -> u16 {
    let target = e.mem.u32(this + 0x2c);
    if e.mem.u32(target + 0x8) == e.mem.u32(name) {
        return 0;
    }
    e.mem.u16(INVALID_INDEX)
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(table + 0x68, 0x0099_0068);
        e.mem.set_u32(object, table);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
        e.register(0x0099_0068, |_, _| Ret::default());
    }

    #[test]
    fn get_and_set_interpolator_count_references() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x38);
        let first = counted(&mut e, 0);
        let second = counted(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8e40, &[this, first, 9]);
        assert_eq!(e.call(0x00a3_8e30, &[this, 0]).u32(), first);
        assert_eq!(e.mem.u32(first + 4), 1);
        // The same one again: nothing changes.
        e.call(0x00a3_8e40, &[this, first, 9]);
        assert_eq!(e.mem.u32(first + 4), 1);
        e.call(0x00a3_8e40, &[this, second, 9]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
        e.call(0x00a3_8e40, &[this, 0, 9]);
        assert_eq!(e.call(0x00a3_8e30, &[this, 0]).u32(), 0);
    }

    #[test]
    fn process_clone_passes_the_process_to_the_interpolator() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8f50, &[this, 0x77]);
        assert!(calls_to(&e, 0x0099_0068).is_empty());
        let interpolator = counted(&mut e, 1);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        e.call(0x00a3_8f50, &[this, 0x77]);
        assert_eq!(calls_to(&e, 0x0099_0068), vec![vec![interpolator, 0x77]]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn load_binary_resolves_the_link_for_new_files() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(INTERP_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let linked = counted(&mut e, 0);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| linked.into_ret());
        let this = e.mem.alloc(0x38);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 0xd8, 0x0a01_0067);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8f80, &[this, stream]);
        assert!(calls_to(&e, STREAM_RESOLVE_LINK_ID).is_empty());
        e.mem.set_u32(stream + 0xd8, 0x0a01_0068);
        e.call(0x00a3_8f80, &[this, stream]);
        assert_eq!(e.mem.u32(this + INTERPOLATOR), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert_eq!(calls_to(&e, INTERP_CONTROLLER_LOAD_BINARY).len(), 2);
    }

    #[test]
    fn interpolator_index_matches_the_target_name() {
        let mut e = Engine::new();
        e.map(INVALID_INDEX & !0xfff, 0x1000);
        e.mem.set_u16(INVALID_INDEX, 0xffff);
        let this = e.mem.alloc(0x38);
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target + 8, 0x4444);
        e.mem.set_u32(this + 0x2c, target);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x4444);
        assert_eq!(e.call(0x00a3_9080, &[this, name]).u16(), 0);
        e.mem.set_u32(name, 0x5555);
        assert_eq!(e.call(0x00a3_9080, &[this, name]).u16(), 0xffff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_single_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a38e20, &args![]).u32(),
            NI_SINGLE_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a35850_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5850;
        const TARGET: u32 = 0x00c9_a6b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}
