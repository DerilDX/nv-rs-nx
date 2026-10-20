//! `NiMain/nifrustum.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFrustum` (Xbox PDB): `m_fLeft` (+0x00), `m_fRight` (+0x04), `m_fTop` (+0x08),
/// `m_fBottom` (+0x0c), `m_fNear` (+0x10), `m_fFar` (+0x14), `m_bOrtho` (+0x18).
const FRUSTUM_ORTHO: u32 = 0x18;

// Translated from 00a71b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFrustum::NiFrustum` (Xbox PDB; `__thiscall`, one word: `bOrtho`): stores only the
/// orthographic flag (the low byte of the word) and answers `this`; the planes are left as
/// they are.
pub fn ni_frustum_ni_frustum(e: &mut Engine, this: Ptr, ortho: u32) -> Ptr {
    e.mem.set_u8(this.addr() + FRUSTUM_ORTHO, ortho as u8);
    this
}

// Translated from 00a71b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiFrustum` constructor from its planes (unnamed in the map; `__thiscall`, seven words:
/// left, right, top, bottom, near, far as floats, then `bOrtho`): each float is copied through
/// the x87 stack (`FLD`/`FSTP`, so a signalling NaN arrives quiet) and the flag's low byte is
/// stored. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a71b80(
    e: &mut Engine,
    this: Ptr,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    near: f32,
    far: f32,
    ortho: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, x87_quiet(left).to_bits());
    e.mem.set_u8(base + FRUSTUM_ORTHO, ortho as u8);
    e.mem.set_u32(base + 0x04, x87_quiet(right).to_bits());
    e.mem.set_u32(base + 0x08, x87_quiet(top).to_bits());
    e.mem.set_u32(base + 0x0c, x87_quiet(bottom).to_bits());
    e.mem.set_u32(base + 0x10, x87_quiet(near).to_bits());
    e.mem.set_u32(base + 0x14, x87_quiet(far).to_bits());
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a71b70, ni_frustum_ni_frustum(Ptr, u32) -> Ptr),
        entry!(0x00a71b80, fn_00a71b80(Ptr, f32, f32, f32, f32, f32, f32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    #[test]
    fn ni_frustum_ni_frustum_sets_only_the_flag() {
        let mut e = Engine::new();
        let f = e.mem.alloc(0x1c);
        e.mem.set_u32(f, 0x1234);
        assert_eq!(e.call(0x00a71b70, &args![f, 0x0101u32]).u32(), f);
        assert_eq!(e.mem.u8(f + 0x18), 1);
        assert_eq!(e.mem.u32(f), 0x1234);
    }

    #[test]
    fn fn_00a71b80_copies_the_planes_through_the_x87_stack() {
        let mut e = Engine::new();
        let f = e.mem.alloc(0x1c);
        e.mem.set_u8(f + 0x19, 0x77);
        let signalling = 0x7f80_0001u32;
        let back = e.call(
            0x00a71b80,
            &args![
                f,
                (-1.0f32).to_bits(),
                1.0f32.to_bits(),
                0.5f32.to_bits(),
                (-0.5f32).to_bits(),
                signalling,
                1000.0f32.to_bits(),
                0x0100u32
            ],
        );
        assert_eq!(back.u32(), f);
        assert_eq!(e.mem.u32(f), (-1.0f32).to_bits());
        assert_eq!(e.mem.u32(f + 0x0c), (-0.5f32).to_bits());
        assert_eq!(e.mem.u32(f + 0x10), 0x7fc0_0001);
        assert_eq!(e.mem.u32(f + 0x14), 1000.0f32.to_bits());
        assert_eq!(e.mem.u8(f + 0x18), 0);
        assert_eq!(e.mem.u8(f + 0x19), 0x77);
    }
}
