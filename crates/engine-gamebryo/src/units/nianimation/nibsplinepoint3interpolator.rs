//! `NiAnimation/nibsplinepoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplinePoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3ff4;

// Translated from 00a506b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI)
}

/// `NiBSplineInterpolator::LoadBinary` (`00a58080`) and `CopyMembers`
/// (`00a57fd0`, `thiscall (this, dest, cloning process)`; the engine map
/// has no name); `NiColor::LoadBinary` (`00a59be0`, `thiscall (this,
/// stream)`: three floats).
const BASE_LOAD_BINARY: u32 = 0x00a5_8080;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00a50680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::CopyMembers` (body; the engine map has no
/// name): the base class's `CopyMembers`, then the point value (+0x1C, 3
/// words) and the control-point handle (+0x28) copied word for word.
pub fn fn_00a50680(e: &mut Engine, this: Ptr, dest: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    for offset in [0x1c, 0x20, 0x24, 0x28] {
        let value = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(dest.addr().wrapping_add(offset), value);
    }
}

// Translated from 00a50910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary`, the point value loaded as three floats (`NiColor`'s
/// loader on +0x1C), then the control-point handle read (4 bytes into
/// +0x28 through the stream reader's function at its +8, `cdecl (reader,
/// this + 0x28, 4, &4, 1)`, the answer added to the reader's +4).
pub fn ni_b_spline_point3_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x1c, stream]);
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let destination = this.addr() + 0x28;
    let read = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read_function, &args![reader, destination, 4u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a506b0, ni_b_spline_point3_interpolator_get_rtti() -> Ptr),
        entry!(0x00a5_0680, fn_00a50680(Ptr, Ptr, Ptr)),
        entry!(
            0x00a5_0910,
            ni_b_spline_point3_interpolator_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a506b0, &args![]).u32(),
            NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn b_spline_point3_copy_members_copies_value_and_handle() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        let dest = e.mem.alloc(0x2c);
        for (i, offset) in [0x1c, 0x20, 0x24, 0x28].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x7f80_0001 + i as u32);
        }
        e.call(0x00a5_0680, &args![this, dest, 0x7000u32]);
        for (i, offset) in [0x1c, 0x20, 0x24, 0x28].iter().enumerate() {
            assert_eq!(e.mem.u32(dest + offset), 0x7f80_0001 + i as u32);
        }
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_COPY_MEMBERS && call.1 == vec![this, dest, 0x7000]));
    }

    #[test]
    fn b_spline_point3_load_binary_loads_the_point_then_the_handle() {
        const READ: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |_, _| 4u32.into_ret());
        e.call_log = Some(vec![]);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x2c);
        e.call(0x00a5_0910, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|call| call.0).collect();
        assert_eq!(order, vec![BASE_LOAD_BINARY, COLOR_LOAD_BINARY, READ]);
        assert_eq!(log[2].1, vec![this + 0x1c, stream]);
        assert_eq!((log[3].1[1], log[3].1[2]), (this + 0x28, 4));
        assert_eq!(e.mem.u32(reader + 4), 4);
    }
}
