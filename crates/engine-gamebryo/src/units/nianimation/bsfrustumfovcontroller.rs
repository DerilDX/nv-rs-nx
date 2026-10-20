//! `NiAnimation/bsfrustumfovcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSFrustumFOVController::ms_RTTI` (the class's `NiRTTI` record).
const BS_FRUSTUM_FOV_CONTROLLER_RTTI: u32 = 0x011f_3ec8;

// Translated from 00a42e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_frustum_fov_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_FRUSTUM_FOV_CONTROLLER_RTTI)
}

/// `00a57c00`: the jump target of `00a44ce0`.
const JUMP_TARGET_00A44CE0: u32 = 0x00a5_7c00;

/// `00a57c10`: the jump target of `00a44cf0`.
const JUMP_TARGET_00A44CF0: u32 = 0x00a5_7c10;

// Translated from 00a44ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57c00` (engine map: `NiBoolInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSFrustumFOVController@@UAAXAAVNiStream@@@Z`.
pub fn bs_frustum_fov_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A44CE0, &args![this, stream]);
}

// Translated from 00a44cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::IsEqual` (Xbox PDB): a 5-byte jump to `00a57c10` (engine map: `NiBoolInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSFrustumFOVController@@UAA_NPAVNiObject@@@Z`.
pub fn bs_frustum_fov_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A44CF0, &args![this, other]).bool()
}

/// `NiSingleInterpController::m_spInterpolator` (Xbox PDB; PC offset).
const INTERPOLATOR: u32 = 0x34;
/// `NiStream::m_uiNifFileVersion` +0xd8 (Xbox PDB; PC offset).
const STREAM_VERSION: u32 = 0xd8;
/// The first file version whose controllers link their interpolator (older
/// ones link float data).
const VERSION_WITH_INTERPOLATOR: u32 = 0x0a01_0068;
/// The size `00a42d00` allocates for the interpolator it makes,
/// and the controller size the deleting destructor frees.
const FLOAT_INTERPOLATOR_SIZE: u32 = 0x18;
const CONTROLLER_SIZE: u32 = 0x38;

/// `NiPSysModifierCtlr::LinkObject` (`00a57630`; the engine map's name for
/// the base link).
const BASE_LINK_OBJECT: u32 = 0x00a5_7630;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a3c340` (unnamed in the engine map; `thiscall`, one word): the
/// `NiFloatInterpolator` constructor taking a float data pointer.
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c340;
/// `00a6d290` (unit `nitimecontroller.cpp`): the destructor body the
/// deleting destructor runs.
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
/// The interpolator's virtual slot `0xbc` (no arguments) that the link runs
/// on an interpolator it made.
const SLOT_INTERPOLATOR_COLLAPSE: u32 = 0xbc;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and
/// `NiRefObject::DeleteThis` (virtual slot 1).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

// Translated from 00a42d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsfrustumfovcontroller.cpp`; a
/// `LinkObject`): the base link (`00a57630`), then, in files older than
/// version `0x0a010068`, the linked float data (`GetObjectFromLinkID`)
/// wrapped in a new 0x18-byte `NiFloatInterpolator` (`00a3c340`; null when
/// the allocation fails) that becomes the interpolator (counted; the old
/// one released), and its slot `0xbc` runs.
pub fn fn_00a42d00(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_VERSION)) >= VERSION_WITH_INTERPOLATOR {
        return;
    }
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    let block = e.call(NI_ALLOCATE, &args![FLOAT_INTERPOLATOR_SIZE]).u32();
    let interpolator = if block == 0 {
        0
    } else {
        e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![block, data])
            .u32()
    };
    let slot = this.addr().wrapping_add(INTERPOLATOR);
    let old = e.mem.u32(slot);
    if old != interpolator {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, interpolator);
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator.wrapping_add(4)]);
        }
    }
    let interpolator = e.mem.u32(slot);
    e.vcall(interpolator, SLOT_INTERPOLATOR_COLLAPSE, &args![]);
}

// Translated from 00a56a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFrustumFOVController::_scalar_deleting_destructor_` (Xbox PDB name
/// from the engine map; the body would fit any 0x38-byte single-interpolator
/// controller): releases the interpolator, runs the destructor body
/// `00a6d290`, then the sized delete of 0x38 bytes when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_frustum_fov_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let interpolator = e.mem.u32(this.addr().wrapping_add(INTERPOLATOR));
    if interpolator != 0 {
        release(e, interpolator);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, CONTROLLER_SIZE]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42e80, bs_frustum_fov_controller_get_rtti() -> Ptr),
        entry!(0x00a4_4ce0, bs_frustum_fov_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_4cf0, bs_frustum_fov_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_2d00, fn_00a42d00(Ptr, Ptr)),
        entry!(0x00a5_6a90, bs_frustum_fov_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_frustum_fov_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a42e80, &args![]).u32(),
            BS_FRUSTUM_FOV_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a44ce0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_4ce0;
        const TARGET: u32 = 0x00a5_7c00;
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
    fn thunk_00a44cf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_4cf0;
        const TARGET: u32 = 0x00a5_7c10;
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

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const COLLAPSE: u32 = 0x00ee_0300;
    const DELETE_THIS: u32 = 0x00ee_0400;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    fn counting(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                Ret {
                    eax: count,
                    ..Ret::default()
                }
            });
        }
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        e.mem.set_u32(vtable + SLOT_INTERPOLATOR_COLLAPSE, COLLAPSE);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn fn_00a42d00_wraps_old_float_data_in_an_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e, &log);
        let made = object(&mut e, 0);
        logged(&mut e, &log, BASE_LINK_OBJECT, 0);
        logged(&mut e, &log, GET_OBJECT_FROM_LINK_ID, 0x7700);
        logged(&mut e, &log, NI_ALLOCATE, made);
        logged(&mut e, &log, FLOAT_INTERPOLATOR_CONSTRUCT, made);
        logged(&mut e, &log, COLLAPSE, 0);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0400_0002);
        let this = e.mem.alloc(0x38);
        e.call(0x00a4_2d00, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_LINK_OBJECT, vec![this, stream]),
                (GET_OBJECT_FROM_LINK_ID, vec![stream]),
                (NI_ALLOCATE, vec![0x18]),
                (FLOAT_INTERPOLATOR_CONSTRUCT, vec![made, 0x7700]),
                (INTERLOCKED_INCREMENT, vec![made + 4]),
                (COLLAPSE, vec![made]),
            ]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATOR), made);
        log.borrow_mut().clear();
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        e.call(0x00a4_2d00, &args![this, stream]);
        assert_eq!(*log.borrow(), [(BASE_LINK_OBJECT, vec![this, stream])]);
    }

    #[test]
    fn scalar_deleting_destructor_releases_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        counting(&mut e, &log);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        logged(&mut e, &log, DELETE_THIS, 0);
        let interpolator = object(&mut e, 1);
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + INTERPOLATOR, interpolator);
        assert_eq!(e.call(0x00a5_6a90, &args![this, 1u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            [
                (INTERLOCKED_DECREMENT, vec![interpolator + 4]),
                (DELETE_THIS, vec![interpolator]),
                (TIME_CONTROLLER_DESTRUCT, vec![this]),
                (NI_SIZED_DELETE, vec![this, 0x38]),
            ]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + INTERPOLATOR, 0);
        e.call(0x00a5_6a90, &args![this, 2u32]);
        assert_eq!(*log.borrow(), [(TIME_CONTROLLER_DESTRUCT, vec![this])]);
    }
}
