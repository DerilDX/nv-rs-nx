//! `NiMain/nitransform.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3::ZERO` (the three floats `MakeIdentity` copies into the translation).
const NI_POINT3_ZERO: u32 = 0x011f_426c;
/// `NiTransform` (Xbox PDB): `m_Rotate` (a 3x3 matrix, +0x00), `m_Translate` (+0x24),
/// `m_fScale` (+0x30).
const TRANSFORM_TRANSLATE: u32 = 0x24;
const TRANSFORM_SCALE: u32 = 0x30;
/// `NiMatrix3::LoadBinary` (`00a59a30`) and `NiPoint3::LoadBinary` (`00a59be0`), both
/// `__thiscall`, one word: the stream.
const MATRIX_LOAD_BINARY: u32 = 0x00a5_9a30;
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiStream`'s input stream (+0x24c), an `NiBinaryStream` whose read function
/// (`m_pfnRead`, +0x8: cdecl, stream, buffer, bytes, component sizes, component count) the
/// code calls, and whose `m_uiAbsoluteCurrentPos` (+0x4) grows by what it read.
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_POSITION: u32 = 0x4;

// Translated from 00a86bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransform::MakeIdentity` (Xbox PDB): the identity rotation (1.0 on the diagonal, 0.0
/// elsewhere), the translation copied from `NiPoint3::ZERO` and a scale of 1.0.
pub fn ni_transform_make_identity(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let one = 1.0f32.to_bits();
    let zero = 0.0f32.to_bits();
    e.mem.set_u32(base, one);
    for offset in [0x04, 0x08, 0x0c, 0x14, 0x18, 0x1c] {
        e.mem.set_u32(base + offset, zero);
    }
    e.mem.set_u32(base + 0x10, one);
    e.mem.set_u32(base + 0x20, one);
    for i in 0..3 {
        let word = e.mem.u32(NI_POINT3_ZERO + 4 * i);
        e.mem.set_u32(base + TRANSFORM_TRANSLATE + 4 * i, word);
    }
    e.mem.set_u32(base + TRANSFORM_SCALE, one);
}

// Translated from 00a86c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransform::LoadBinary` (Xbox PDB; `__thiscall`, one word: the `NiStream`): the rotation
/// (`NiMatrix3::LoadBinary`), the translation (`NiPoint3::LoadBinary`), then the scale read
/// straight through the input stream's read function (4 bytes, one component of 4 bytes; the
/// component size lives in the stream argument's own stack word), its position moving on by
/// what was read.
pub fn ni_transform_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(MATRIX_LOAD_BINARY, &args![this, stream]);
    e.call(
        POINT_LOAD_BINARY,
        &args![base + TRANSFORM_TRANSLATE, stream],
    );
    let input = e.mem.u32(stream.addr() + STREAM_INPUT);
    let read = e.mem.u32(input + BINARY_STREAM_READ);
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        let bytes = e
            .call(
                read,
                &args![input, base + TRANSFORM_SCALE, 4u32, sizes, 1u32],
            )
            .u32();
        let position = e
            .mem
            .u32(input + BINARY_STREAM_POSITION)
            .wrapping_add(bytes);
        e.mem.set_u32(input + BINARY_STREAM_POSITION, position);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a86bf0, ni_transform_make_identity(Ptr)),
        entry!(0x00a86c80, ni_transform_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn ni_transform_make_identity_writes_the_identity() {
        let mut e = Engine::new();
        e.map(NI_POINT3_ZERO, 12);
        e.mem.set_u32(NI_POINT3_ZERO + 4, 0x8000_0000);
        let t = e.mem.alloc(0x34);
        for off in (0..0x34).step_by(4) {
            e.mem.set_u32(t + off, 0x7fc0_0001);
        }
        e.call(0x00a86bf0, &args![t]);
        let words: Vec<u32> = (0..13).map(|i| e.mem.u32(t + 4 * i)).collect();
        let one = 0x3f80_0000;
        assert_eq!(
            words,
            vec![one, 0, 0, 0, one, 0, 0, 0, one, 0, 0x8000_0000, 0, one]
        );
    }

    const READ_DOUBLE: u32 = 0x0070_1000;

    #[test]
    fn ni_transform_load_binary_reads_the_three_parts() {
        let mut e = Engine::new();
        e.register(MATRIX_LOAD_BINARY, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_DOUBLE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], 0x4000_0000);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let stream = e.mem.alloc(0x250);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ_DOUBLE);
        e.mem.set_u32(input + BINARY_STREAM_POSITION, 0x20);
        let t = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a86c80, &args![t, stream]);
        assert_eq!(calls_to(&e, MATRIX_LOAD_BINARY), vec![vec![t, stream]]);
        assert_eq!(
            calls_to(&e, POINT_LOAD_BINARY),
            vec![vec![t + 0x24, stream]]
        );
        assert_eq!(calls_to(&e, READ_DOUBLE)[0][..2], [input, t + 0x30]);
        assert_eq!(e.mem.u32(t + 0x30), 0x4000_0000);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 0x24);
    }
}
