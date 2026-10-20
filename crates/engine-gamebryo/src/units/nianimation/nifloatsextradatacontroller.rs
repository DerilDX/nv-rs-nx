//! `NiAnimation/nifloatsextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatsExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_3fcc;

// Translated from 00a4ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI)
}

/// The base class's `CopyMembers` (`00a38ef0`, `thiscall (this, dest,
/// cloning process)`; the engine map's name for that address,
/// `NiPSysModifierCtlr::CopyMembers`, is a folded body's).
const BASE_COPY_MEMBERS: u32 = 0x00a3_8ef0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots, one word:
/// the address).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The pooled empty `NiFixedString` handle (`0109b220`), never counted.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// The controller's extra-data name (`NiFixedString`) at +0x38.
const EXTRA_DATA_NAME: u32 = 0x38;

// Translated from 00c312f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataController::CopyMembers` (Xbox PDB): the base class's
/// `CopyMembers`, then the extra-data name at +0x38 assigned to `dest`
/// (when it differs: the new handle counted unless empty, the old one
/// released unless empty).
pub fn ni_floats_extra_data_controller_copy_members(
    e: &mut Engine,
    this: Ptr,
    dest: Ptr,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let name = e.mem.u32(this.addr() + EXTRA_DATA_NAME);
    let slot = dest.addr().wrapping_add(EXTRA_DATA_NAME);
    if e.mem.u32(slot) == name {
        return;
    }
    let empty = e.global::<u32>(FIXED_STRING_EMPTY);
    if name != empty {
        e.call(INTERLOCKED_INCREMENT, &args![name.wrapping_sub(8)]);
    }
    let old = e.mem.u32(slot);
    if old != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(slot, name);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4ef60, ni_floats_extra_data_controller_get_rtti() -> Ptr),
        entry!(
            0x00c3_12f0,
            ni_floats_extra_data_controller_copy_members(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4ef60, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_CONTROLLER_RTTI
        );
    }

    #[test]
    fn floats_extra_data_copy_members_assigns_the_counted_name() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_EMPTY, 4);
        e.set_global(FIXED_STRING_EMPTY, 0x0500_0008u32);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        let name = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(name - 8, 1);
        let old = e.mem.alloc(0x10) + 8;
        e.mem.set_u32(old - 8, 2);
        let this = e.mem.alloc(0x40);
        let dest = e.mem.alloc(0x40);
        e.mem.set_u32(this + EXTRA_DATA_NAME, name);
        e.mem.set_u32(dest + EXTRA_DATA_NAME, old);
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + EXTRA_DATA_NAME), name);
        assert_eq!(e.mem.u32(name - 8), 2);
        assert_eq!(e.mem.u32(old - 8), 1);
        // The same handle again: no counting.
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(name - 8), 2);
        // The empty string replaces it without being counted.
        e.mem.set_u32(this + EXTRA_DATA_NAME, 0x0500_0008);
        e.call(0x00c3_12f0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + EXTRA_DATA_NAME), 0x0500_0008);
        assert_eq!(e.mem.u32(name - 8), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|call| call.0 == BASE_COPY_MEMBERS)
                .count(),
            3
        );
    }
}
