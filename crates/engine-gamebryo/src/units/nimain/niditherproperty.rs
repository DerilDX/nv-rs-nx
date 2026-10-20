//! `NiMain/niditherproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDitherProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_DITHER_PROPERTY_RTTI: u32 = 0x011f_5f48;

// Translated from 00a9a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dither_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DITHER_PROPERTY_RTTI)
}

/// `NiProperty::LoadBinary` (`00a5d7f0`, `RET 4`).
const NI_PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// `NiStream::m_pkIstr` (Xbox PDB) +0x24C: the `NiBinaryStream*` read from.
const STREAM_INPUT: u32 = 0x24c;
/// `NiStream::m_usNiPropertyFlags` (Xbox PDB) +0x288.
const STREAM_PROPERTY_FLAGS: u32 = 0x288;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (Xbox PDB) +4.
const BINARY_STREAM_POSITION: u32 = 0x4;
/// `NiBinaryStream::m_pfnRead` (Xbox PDB) +8: a `cdecl` function pointer
/// `(stream, buffer, size, &component size, component count)` returning the
/// number of bytes read.
const BINARY_STREAM_READ: u32 = 0x8;
/// The first file version whose dither property stores its own flags.
const OWN_FLAGS_VERSION: u32 = 0x0a00_0102;
/// `NiDitherProperty::m_uFlags` (Xbox PDB) +0x18.
const DITHER_FLAGS: u32 = 0x18;

// Translated from 00a95190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`, then
/// the flags at +0x18: for files before version `0x0A000102` bit 0 of the
/// stream's `m_usNiPropertyFlags` (+0x288); otherwise 2 bytes read into them
/// through the input stream's `m_pfnRead` (`(istr, this + 0x18, 2,
/// &component size = 2, 1)`; the component size is the stream argument's
/// stack word, overwritten with 2), the stream's position advanced by the
/// bytes read.
pub fn ni_dither_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_PROPERTY_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) < OWN_FLAGS_VERSION {
        let flags = e.mem.u16(stream.addr().wrapping_add(STREAM_PROPERTY_FLAGS)) & 1;
        e.mem.set_u16(this.addr().wrapping_add(DITHER_FLAGS), flags);
        return;
    }
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.with_stack(4, |e, component_size| {
        e.mem.set_u32(component_size.addr(), 2);
        let got = e
            .call(
                read,
                &args![
                    input,
                    this.addr().wrapping_add(DITHER_FLAGS),
                    2u32,
                    component_size,
                    1u32
                ],
            )
            .u32();
        let position = input.wrapping_add(BINARY_STREAM_POSITION);
        let advanced = e.mem.u32(position).wrapping_add(got);
        e.mem.set_u32(position, advanced);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9a5e0, ni_dither_property_get_rtti() -> Ptr),
        entry!(0x00a95190, ni_dither_property_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dither_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9a5e0, &args![]).u32(), NI_DITHER_PROPERTY_RTTI);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    /// A stand-in `m_pfnRead` (at a made-up address): writes `0x1234` into
    /// the buffer and answers 2 bytes read.
    const FAKE_READ: u32 = 0x7100_0000;

    #[test]
    fn ni_dither_property_load_binary_takes_old_flags_or_reads_its_own() {
        for version in [0x0a00_0101u32, 0x0a00_0102] {
            let mut e = Engine::new();
            e.register(NI_PROPERTY_LOAD_BINARY, |_, _| Ret::default());
            e.register(FAKE_READ, |e, a| {
                assert_eq!(&a[2..], [2, a[3], 1]);
                assert_eq!(e.mem.u32(a[3]), 2);
                e.mem.set_u16(a[1], 0x1234);
                Ret {
                    eax: 2,
                    ..Ret::default()
                }
            });
            let this: Ptr = Ptr::new(e.mem.alloc(0x1c));
            let stream = e.mem.alloc(0x5c4);
            let input = e.mem.alloc(0x10);
            e.mem.set_u32(input + 4, 7);
            e.mem.set_u32(input + 8, FAKE_READ);
            e.mem.set_u32(stream + 0xd8, version);
            e.mem.set_u32(stream + 0x24c, input);
            e.mem.set_u16(stream + 0x288, 0xfff3);
            e.call_log = Some(vec![]);
            e.call(0x00a95190, &args![this, stream]);
            let log = e.call_log.take().unwrap();
            let parent = log.iter().find(|c| c.0 == NI_PROPERTY_LOAD_BINARY).unwrap();
            assert_eq!(parent.1, [this.addr(), stream]);
            if version < 0x0a00_0102 {
                assert_eq!(e.mem.u16(this.addr() + 0x18), 1);
                assert!(log.iter().all(|c| c.0 != FAKE_READ));
                assert_eq!(e.mem.u32(input + 4), 7);
            } else {
                assert_eq!(e.mem.u16(this.addr() + 0x18), 0x1234);
                let read = log.iter().find(|c| c.0 == FAKE_READ).unwrap();
                assert_eq!(&read.1[..2], [input, this.addr() + 0x18]);
                assert_eq!(e.mem.u32(input + 4), 9);
            }
        }
    }
}
