//! `NiMain/nipixelformat.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPixelFormat` (Xbox PDB), 0x44 bytes. `m_akComponents` (+0x14) is four
    /// `NiComponentSpec`s of 0xC bytes: `m_eComponent` (+0), `m_eRepresentation` (+4),
    /// `m_ucBitsPerComponent` (+8), `m_bSigned` (+9).
    pub struct NiPixelFormat: 0x44 {
        /// `m_uFlags` (Xbox PDB).
        0x00 m_uFlags: u8,
        /// `m_ucBitsPerPixel` (Xbox PDB).
        0x01 m_ucBitsPerPixel: u8,
        /// `m_eFormat` (Xbox PDB).
        0x04 m_eFormat: u32,
        /// `m_eTiling` (Xbox PDB).
        0x08 m_eTiling: u32,
        /// `m_uiRendererHint` (Xbox PDB).
        0x0C m_uiRendererHint: u32,
        /// `m_uiExtraData` (Xbox PDB).
        0x10 m_uiExtraData: u32,
    }
}

/// Offset of `m_akComponents` (Xbox PDB) and the size of one `NiComponentSpec`.
const COMPONENTS: u32 = 0x14;
const COMPONENT_SIZE: u32 = 0xc;

// Translated from 00a7cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelFormat::NiPixelFormat` (engine map; `thiscall`, no stack word): the default
/// format. Flags 1, 8 bits per pixel, format 2, tiling 0, renderer hint `0xFFFFFFFF`, extra
/// data 0; the first component spec (3, 0x10, 8 bits, signed byte 1), the other three
/// (5, 0x13, 0 bits, signed byte 1). The numbers are stored as the exe has them; their enum
/// names are not confirmed here. The stores follow the exe's order. Returns `this`.
pub fn ni_pixel_format_ni_pixel_format(
    e: &mut Engine,
    this: Ptr<NiPixelFormat>,
) -> Ptr<NiPixelFormat> {
    e.set(this, NiPixelFormat::m_ucBitsPerPixel, 8);
    e.set(this, NiPixelFormat::m_eFormat, 2);
    e.set(this, NiPixelFormat::m_uiRendererHint, 0xffff_ffff);
    e.set(this, NiPixelFormat::m_uiExtraData, 0);
    e.set(this, NiPixelFormat::m_uFlags, 1);
    e.set(this, NiPixelFormat::m_eTiling, 0);
    let first = this.addr() + COMPONENTS;
    e.mem.set_u32(first, 0x10);
    e.mem.set_u32(first + 4, 3);
    e.mem.set_u8(first + 8, 8);
    e.mem.set_u8(first + 9, 1);
    for index in 1..4u32 {
        let spec = first + COMPONENT_SIZE * index;
        e.mem.set_u32(spec, 0x13);
        e.mem.set_u32(spec + 4, 5);
        e.mem.set_u8(spec + 8, 0);
        e.mem.set_u8(spec + 9, 1);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a7_cca0,
        ni_pixel_format_ni_pixel_format(Ptr<NiPixelFormat>) -> Ptr<NiPixelFormat>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_writes_the_default_format() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        for offset in (0..0x44).step_by(4) {
            e.mem.set_u32(this + offset, 0xaaaa_aaaa);
        }
        let r = e.call(0x00a7_cca0, &[this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(e.mem.u8(this + 1), 8);
        // The two padding bytes after m_ucBitsPerPixel are left as they were.
        assert_eq!(e.mem.u8(this + 2), 0xaa);
        let words: Vec<u32> = [4u32, 8, 0xc, 0x10, 0x14, 0x18]
            .iter()
            .map(|&o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![2, 0, 0xffff_ffff, 0, 0x10, 3]);
        assert_eq!([e.mem.u8(this + 0x1c), e.mem.u8(this + 0x1d)], [8, 1]);
        for spec in [0x20u32, 0x2c, 0x38] {
            assert_eq!(
                [e.mem.u32(this + spec), e.mem.u32(this + spec + 4)],
                [0x13, 5]
            );
            assert_eq!(
                [e.mem.u8(this + spec + 8), e.mem.u8(this + spec + 9)],
                [0, 1]
            );
            assert_eq!(e.mem.u8(this + spec + 0xa), 0xaa);
        }
    }
}
