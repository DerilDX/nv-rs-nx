//! `NiMain/nicullingprocess.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCullingProcess::ms_RTTI` (the class's `NiRTTI` record).
const NI_CULLING_PROCESS_RTTI: u32 = 0x011f_4990;

// Translated from 00a693f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_culling_process_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CULLING_PROCESS_RTTI)
}

/// `NiCullingProcess`'s vtable (`0109c28c`, RTTI `.?AVNiCullingProcess@@`).
const NI_CULLING_PROCESS_VTABLE: u32 = 0x0109_c28c;
/// The frustum constructor `00a71b80` (`NiMain/nifrustum.cpp` in the engine map; `thiscall`,
/// `RET 0x1c`: six floats and a word).
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b80;
/// `NiPlane::NiPlane` (engine map; `thiscall`, no stack words).
const PLANE_CONSTRUCT: u32 = 0x00a6_9940;

// Translated from 00a69400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCullingProcess::NiCullingProcess` (unnamed in the engine map; the vtable it sets is
/// `NiCullingProcess`'s by its RTTI). `thiscall`, one word (`RET 4`): the visible set. Sets
/// the vtable, clears `m_bUseVirtualAppend` (+4), constructs the frustum `m_kFrustum` (+0x10)
/// with six 0.0 floats and 0 (`00a71b80`), constructs the six planes of `m_kPlanes` (+0x2c,
/// 0x10 bytes each, `NiPlane::NiPlane`), sets `m_uiActivePlanes` (+0x8c) to 0x3f, stores the
/// visible set at +8 (`m_pkVisibleSet`) and clears +0xc (`m_pkCamera`). Field names from the
/// Xbox PDB, at the PC offsets this body uses. Answers `this`.
pub fn ni_culling_process_ni_culling_process(e: &mut Engine, this: Ptr, visible_set: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, NI_CULLING_PROCESS_VTABLE);
    e.mem.set_u8(at + 4, 0);
    // FLDZ, then FST float into the six float words: +0.0 each, pushed as its bit pattern.
    let zero = 0.0f32.to_bits();
    e.call(
        FRUSTUM_CONSTRUCT,
        &args![at + 0x10, zero, zero, zero, zero, zero, zero, 0u32],
    );
    let planes = at + 0x2c;
    let mut plane = planes;
    // EBP counts 5 down to 0 (`SUB 1; JNS`): six planes.
    let mut remaining: i32 = 5;
    loop {
        e.call(PLANE_CONSTRUCT, &args![plane]);
        plane += 0x10;
        remaining -= 1;
        if remaining < 0 {
            break;
        }
    }
    e.mem.set_u32(planes + 0x60, 0x3f);
    e.mem.set_u32(at + 8, visible_set);
    e.mem.set_u32(at + 0xc, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a693f0, ni_culling_process_get_rtti() -> Ptr),
        entry!(0x00a6_9400, ni_culling_process_ni_culling_process(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_culling_process_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a693f0, &args![]).u32(), NI_CULLING_PROCESS_RTTI);
    }
}

#[cfg(test)]
mod b000o_tests {
    use super::*;

    #[test]
    fn ni_culling_process_constructor_builds_frustum_and_planes() {
        let mut e = Engine::new();
        e.register(FRUSTUM_CONSTRUCT, |e, a| {
            // Runs after the vtable and the flag are set.
            assert_eq!(e.mem.u32(a[0] - 0x10), NI_CULLING_PROCESS_VTABLE);
            assert_eq!(e.mem.u8(a[0] - 0xc), 0);
            a[0].into_ret()
        });
        e.register(PLANE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcd; 0x90]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a6_9400, &args![this, 0x5150u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        let frustum: Vec<_> = log.iter().filter(|c| c.0 == FRUSTUM_CONSTRUCT).collect();
        assert_eq!(frustum.len(), 1);
        assert_eq!(frustum[0].1, vec![this + 0x10, 0, 0, 0, 0, 0, 0, 0]);
        let planes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == PLANE_CONSTRUCT)
            .map(|c| c.1[0])
            .collect();
        let expected: Vec<u32> = (0..6).map(|i| this + 0x2c + 0x10 * i).collect();
        assert_eq!(planes, expected);
        assert_eq!(e.mem.u32(this), NI_CULLING_PROCESS_VTABLE);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 0xcd);
        assert_eq!(e.mem.u32(this + 8), 0x5150);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x8c), 0x3f);
    }
}
