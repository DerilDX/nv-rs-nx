//! `NiMain/run_00a9fe90` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor `00a8f190` (engine map unit `NiMain/ni2dbuffer.cpp`, `thiscall`, no
/// words).
const BASE_CONSTRUCT_00A8F190: u32 = 0x00a8_f190;
/// The vtable whose RTTI locator names `NiDepthStencilBuffer` (`010a2084`).
const NI_DEPTH_STENCIL_BUFFER_VTABLE: u32 = 0x010a_2084;
/// The value the format object's word at +4 must hold.
const FORMAT_KIND_15: u32 = 0xf;

// Translated from 00a9fe90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00a9fe90`): makes an `NiDepthStencilBuffer` (the vtable's
/// RTTI name). `cdecl`, five words: width, height, an unread word, a format object and a word
/// passed on. Answers 0 unless the format object's word at +4 is 0xf. Otherwise allocates
/// 0x14 bytes (`00aa13e0`), runs the base constructor `00a8f190` and sets the vtable
/// `010a2084` (a null allocation is used as it is), stores the width at +8 and the height at
/// +0xc, and calls the buffer's vtable slot +0x94 with (format object, last word). When that
/// answers false (`AL`) the buffer is deleted (slot 0 with 1) and 0 is answered; else the
/// buffer.
pub fn fn_00a9fe90(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_2: u32,
    format: Ptr,
    extra: u32,
) -> Ptr {
    if e.mem.u32(format.addr().wrapping_add(4)) != FORMAT_KIND_15 {
        return Ptr::NULL;
    }
    let block = e.call(NI_ALLOCATE, &args![0x14u32]).u32();
    let buffer = if block != 0 {
        e.call(BASE_CONSTRUCT_00A8F190, &args![block]);
        e.mem.set_u32(block, NI_DEPTH_STENCIL_BUFFER_VTABLE);
        block
    } else {
        0
    };
    let vtable = e.mem.u32(buffer);
    let slot = e.mem.u32(vtable.wrapping_add(0x94));
    e.mem.set_u32(buffer.wrapping_add(8), width);
    e.mem.set_u32(buffer.wrapping_add(0xc), height);
    if !e.call(slot, &args![buffer, format, extra]).bool() {
        e.vcall(buffer, 0, &args![1u32]);
        return Ptr::NULL;
    }
    Ptr::new(buffer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a9_fe90,
        fn_00a9fe90(u32, u32, u32, Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The buffer's vtable at `010a2084` with slot 0 (`0f00_3000`) and slot +0x94
    /// (`0f00_3094`, answering `accepts`).
    fn rig(accepts: bool) -> Engine {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x26];
        slots[0] = 0x0f00_3000;
        slots[0x25] = 0x0f00_3094;
        e.put_vtable(NI_DEPTH_STENCIL_BUFFER_VTABLE, &slots);
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT_00A8F190, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.register(0x0f00_3000, |_, _| Ret::default());
        if accepts {
            e.register(0x0f00_3094, |_, _| 1u32.into_ret());
        } else {
            // Only `AL` counts.
            e.register(0x0f00_3094, |_, _| 0x100u32.into_ret());
        }
        e
    }

    #[test]
    fn fn_00a9fe90_makes_the_buffer_when_its_set_up_succeeds() {
        let mut e = rig(true);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xf);
        e.call_log = Some(vec![]);
        let buffer = e
            .call(
                0x00a9_fe90,
                &args![640u32, 480u32, 0x77u32, format, 0x55u32],
            )
            .u32();
        assert_ne!(buffer, 0);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls(&e, BASE_CONSTRUCT_00A8F190), vec![vec![buffer]]);
        assert_eq!(e.mem.u32(buffer), NI_DEPTH_STENCIL_BUFFER_VTABLE);
        assert_eq!(e.mem.u32(buffer + 8), 640);
        assert_eq!(e.mem.u32(buffer + 0xc), 480);
        assert_eq!(calls(&e, 0x0f00_3094), vec![vec![buffer, format, 0x55]]);
        assert!(calls(&e, 0x0f00_3000).is_empty());
    }

    #[test]
    fn fn_00a9fe90_deletes_the_buffer_when_its_set_up_fails() {
        let mut e = rig(false);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xf);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00a9_fe90, &args![1u32, 2u32, 0u32, format, 0u32])
            .u32();
        assert_eq!(answer, 0);
        let buffer = calls(&e, 0x0f00_3094)[0][0];
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![buffer, 1]]);
    }

    #[test]
    fn fn_00a9fe90_refuses_another_format_kind() {
        let mut e = rig(true);
        let format = e.mem.alloc(8);
        e.mem.set_u32(format + 4, 0xe);
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00a9_fe90, &args![1u32, 2u32, 0u32, format, 0u32])
            .u32();
        assert_eq!(answer, 0);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
    }
}
