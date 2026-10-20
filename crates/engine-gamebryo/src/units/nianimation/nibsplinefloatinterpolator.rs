//! `NiAnimation/nibsplinefloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3ffc;

// Translated from 00a50b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI)
}

// Translated from 00a50b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x24 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_float_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x24u32]);
    }
    this
}

/// `NiBSplineInterpolator::LoadBinary` (`00a58080`, `thiscall (this,
/// stream)`).
const BASE_LOAD_BINARY: u32 = 0x00a5_8080;

/// One read through the stream reader (`stream + 0x24c`): its function at
/// +8, `cdecl (reader, destination, size, &4, 1)` (the component size 4 in
/// the caller's argument slot), the answer added to the reader's +4.
fn read_through_stream(e: &mut Engine, stream: u32, destination: u32, size: u32) {
    let reader = e.mem.u32(stream.wrapping_add(0x24c));
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let read = e.with_stack(4, |e, component| {
        e.mem.set_u32(component.addr(), 4);
        e.call(
            read_function,
            &args![reader, destination, size, component, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

// Translated from 00a50cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary`, then two 4-byte reads, into +0x1C (the value) and +0x20
/// (the control-point handle).
pub fn ni_b_spline_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    read_through_stream(e, stream.addr(), this.addr() + 0x1c, 4);
    read_through_stream(e, stream.addr(), this.addr() + 0x20, 4);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a50b70, ni_b_spline_float_interpolator_get_rtti() -> Ptr),
        entry!(0x00a50b80, ni_b_spline_float_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_0cd0,
            ni_b_spline_float_interpolator_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a50b70, &args![]).u32(),
            NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_float_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_7f80, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a50b80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a50b80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x24]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }

    #[test]
    fn b_spline_float_load_binary_reads_value_then_handle() {
        const READ: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            4u32.into_ret()
        });
        e.call_log = Some(vec![]);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 4, 0x10);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x24);
        e.call(0x00a5_0cd0, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 2);
        assert_eq!((reads[0].1[1], reads[0].1[2]), (this + 0x1c, 4));
        assert_eq!((reads[1].1[1], reads[1].1[2]), (this + 0x20, 4));
        assert_eq!(e.mem.u32(reader + 4), 0x18);
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_LOAD_BINARY && call.1 == vec![this, stream]));
    }
}
