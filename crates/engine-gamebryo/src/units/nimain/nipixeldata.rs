//! `NiMain/nipixeldata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPixelData::ms_RTTI` (the class's `NiRTTI` record).
const NI_PIXEL_DATA_RTTI: u32 = 0x011f_4a90;

// Translated from 00a7c020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_pixel_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PIXEL_DATA_RTTI)
}

// Translated from 00a7c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7c390`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x74 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_pixel_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_c390;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x74u32]);
    }
    this
}

layout! {
    /// `NiPixelData` (Xbox PDB), 0x74 bytes (the sized delete frees 0x74); the PC code uses
    /// the same offsets.
    pub struct NiPixelData: 0x74 {
        /// `m_kPixelFormat` (Xbox PDB): an `NiPixelFormat` of 0x44 bytes.
        0x08 m_kPixelFormat: u32,
        /// `m_spPalette` (Xbox PDB).
        0x4c m_spPalette: u32,
        /// `m_pucPixels` (Xbox PDB).
        0x50 m_pucPixels: u32,
        /// `m_puiWidth` (Xbox PDB).
        0x54 m_puiWidth: u32,
        /// `m_puiHeight` (Xbox PDB).
        0x58 m_puiHeight: u32,
        /// `m_puiOffsetInBytes` (Xbox PDB).
        0x5c m_puiOffsetInBytes: u32,
        /// `m_uiMipmapLevels` (Xbox PDB).
        0x60 m_uiMipmapLevels: u32,
        /// `m_uiPixelStride` (Xbox PDB).
        0x64 m_uiPixelStride: u32,
        /// `m_uiRevID` (Xbox PDB).
        0x68 m_uiRevID: u32,
        /// `m_uiFaces` (Xbox PDB).
        0x6c m_uiFaces: u32,
        /// `bNoConvert` (Xbox PDB).
        0x70 bNoConvert: u8,
    }
}

/// `NiPixelData`'s virtual table.
const NI_PIXEL_DATA_VTABLE: u32 = 0x0109_dbb4;
/// `NiPixelFormat` (Xbox PDB): its 0x11 words, `m_ucBitsPerPixel` (+0x01) and `m_eFormat`
/// (+0x04; `FORMAT_DXT1` .. `FORMAT_DXT5` are 4..6).
const PIXEL_FORMAT_WORDS: u32 = 0x11;
const FORMAT_BITS_PER_PIXEL: u32 = 0x01;
const FORMAT_KIND: u32 = 0x04;
const FORMAT_DXT1: i32 = 4;
const FORMAT_DXT5: i32 = 6;
/// The constructor's stack frame (0xc4 bytes): the bytes per pixel (+0x00), the offsets of the
/// levels (+0x04, one more than the levels), their widths (+0x44) and heights (+0x84), room
/// for 16 levels each.
const FRAME_SIZE: u32 = 0xc4;
const FRAME_OFFSETS: u32 = 0x04;
const FRAME_WIDTHS: u32 = 0x44;
const FRAME_HEIGHTS: u32 = 0x84;

// Callees outside this unit.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `__thiscall`).
const PIXEL_FORMAT_CONSTRUCTOR: u32 = 0x00a7_cca0;
/// `NiPixelData::AllocateData` (`00a8f010`, `__thiscall`: levels, faces, bytes per face).
const ALLOCATE_DATA: u32 = 0x00a8_f010;
/// `00a8f060` (`__thiscall`, `nipixeldata_xenon.cpp`): frees the pixel buffers.
const FREE_DATA: u32 = 0x00a8_f060;
/// `_memcpy` (`00ec44d0`, cdecl, three words).
const MEMCPY: u32 = 0x00ec_44d0;
/// `InterlockedDecrement` import slot; `DeleteThis` (virtual +0x4).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const DELETE_THIS_SLOT: u32 = 0x4;

// Translated from 00a7be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two words: width, height; the shape of
/// `NiPixelData::ComputeMipmapLevels`): 0 when either side is 0, else the larger of the two
/// sides' bit lengths (the number of halvings to 1, plus one).
pub fn fn_00a7be30(_e: &mut Engine, width: u32, height: u32) -> u32 {
    if width == 0 || height == 0 {
        return 0;
    }
    let width_levels = 32 - width.leading_zeros();
    let height_levels = 32 - height.leading_zeros();
    if height_levels < width_levels {
        width_levels
    } else {
        height_levels
    }
}

// Translated from 00a7c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPixelData::NiPixelData` (Xbox PDB `NiPixelData_ov2`; `__thiscall`, five words: width,
/// height, format, mipmap levels, faces). The `NiObject` constructor, the class's virtual
/// table, the pixel format reset (`NiPixelFormat::NiPixelFormat`) and then copied from
/// `format`, no palette, `bNoConvert` cleared. Zero levels become the full chain
/// ([`fn_00a7be30`]). Each level's side is the previous one halved (0 read as 1); its size is
/// `stride * width * height` for an uncompressed format (`stride` = bits per pixel / 8),
/// `width * height / 2` when that stride is 0, and for a DXT format (stride 0) the 4x4 blocks
/// times 8 (DXT1) or 16 bytes. The offsets accumulate on the stack with the sides;
/// `AllocateData(levels, faces, total)` makes the buffers and the sides and offsets are copied
/// in (`_memcpy`). The revision becomes 1. Answers `this`.
pub fn ni_pixel_data_ni_pixel_data(
    e: &mut Engine,
    this: Ptr<NiPixelData>,
    width: u32,
    height: u32,
    format: Ptr,
    levels: u32,
    faces: u32,
) -> Ptr<NiPixelData> {
    let base = this.addr();
    let format = format.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_PIXEL_DATA_VTABLE);
    e.call(PIXEL_FORMAT_CONSTRUCTOR, &args![base + 0x8]);
    e.set(this, NiPixelData::m_spPalette, 0);
    e.set(this, NiPixelData::bNoConvert, 0);
    for i in 0..PIXEL_FORMAT_WORDS {
        let word = e.mem.u32(format + 4 * i);
        e.mem.set_u32(base + 0x8 + 4 * i, word);
    }
    let levels = if levels == 0 {
        fn_00a7be30(e, width, height)
    } else {
        levels
    };
    e.set(this, NiPixelData::m_uiMipmapLevels, levels);
    e.set(this, NiPixelData::m_uiFaces, faces);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let offsets = frame + FRAME_OFFSETS;
        let widths = frame + FRAME_WIDTHS;
        let heights = frame + FRAME_HEIGHTS;
        let slot = |table: u32, i: u32| table.wrapping_add(i.wrapping_mul(4));
        let kind = e.mem.i32(format + FORMAT_KIND);
        e.mem.set_u32(offsets, 0);
        let (mut w, mut h) = (width, height);
        if (FORMAT_DXT1..=FORMAT_DXT5).contains(&kind) {
            e.set(this, NiPixelData::m_uiPixelStride, 0);
            let block = if e.mem.i32(format + FORMAT_KIND) != FORMAT_DXT1 {
                16
            } else {
                8
            };
            let mut i = 0;
            while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
                let side_w = if w != 0 { w } else { 1 };
                e.mem.set_u32(slot(widths, i), side_w);
                let side_h = if h != 0 { h } else { 1 };
                e.mem.set_u32(slot(heights, i), side_h);
                let blocks =
                    ((side_h.wrapping_add(3) & !3).wrapping_mul(side_w.wrapping_add(3) & !3)) >> 4;
                let next = blocks
                    .wrapping_mul(block)
                    .wrapping_add(e.mem.u32(slot(offsets, i)));
                e.mem.set_u32(slot(offsets, i + 1), next);
                i += 1;
                w >>= 1;
                h >>= 1;
            }
        } else {
            let stride = u32::from(e.mem.u8(format + FORMAT_BITS_PER_PIXEL)) >> 3;
            e.mem.set_u32(frame, stride);
            e.set(this, NiPixelData::m_uiPixelStride, stride);
            let mut i = 0;
            while i < e.get(this, NiPixelData::m_uiMipmapLevels) {
                let side_w = if w != 0 { w } else { 1 };
                e.mem.set_u32(slot(widths, i), side_w);
                let side_h = if h != 0 { h } else { 1 };
                e.mem.set_u32(slot(heights, i), side_h);
                let size = if stride != 0 {
                    e.mem.u32(frame).wrapping_mul(side_h).wrapping_mul(side_w)
                } else {
                    side_h.wrapping_mul(side_w) >> 1
                };
                let next = size.wrapping_add(e.mem.u32(slot(offsets, i)));
                e.mem.set_u32(slot(offsets, i + 1), next);
                i += 1;
                w >>= 1;
                h >>= 1;
            }
        }
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let total = e.mem.u32(slot(offsets, levels));
        e.call(ALLOCATE_DATA, &args![this, levels, faces, total]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let bytes = levels.wrapping_mul(4);
        let to = e.get(this, NiPixelData::m_puiWidth);
        e.call(MEMCPY, &args![to, widths, bytes]);
        let to = e.get(this, NiPixelData::m_puiHeight);
        e.call(MEMCPY, &args![to, heights, bytes]);
        let levels = e.get(this, NiPixelData::m_uiMipmapLevels);
        let to = e.get(this, NiPixelData::m_puiOffsetInBytes);
        e.call(
            MEMCPY,
            &args![to, offsets, levels.wrapping_mul(4).wrapping_add(4)],
        );
    });
    e.set(this, NiPixelData::m_uiRevID, 1);
    this
}

/// The palette's reference given up (deleted at the last one).
fn release_palette(e: &mut Engine, palette: u32) {
    let count = e.call(INTERLOCKED_DECREMENT, &args![palette + 4]).u32();
    if count == 0 {
        e.vcall(palette, DELETE_THIS_SLOT, &args![]);
    }
}

// Translated from 00a7c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPixelData` destructor body (unnamed in the map): the class's virtual table, the
/// palette released and cleared, the buffers freed (`00a8f060`), then the palette pointer
/// released again if anything put one back (without clearing it, as the inlined `NiPointer`
/// destructor does), and the `NiObject` destructor (a tail jump).
pub fn fn_00a7c390(e: &mut Engine, this: Ptr<NiPixelData>) {
    e.mem.set_u32(this.addr(), NI_PIXEL_DATA_VTABLE);
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        release_palette(e, palette);
        e.set(this, NiPixelData::m_spPalette, 0);
    }
    e.call(FREE_DATA, &args![this]);
    let palette = e.get(this, NiPixelData::m_spPalette);
    if palette != 0 {
        release_palette(e, palette);
    }
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7be30, fn_00a7be30(u32, u32) -> u32),
        entry!(
            0x00a7c190,
            ni_pixel_data_ni_pixel_data(
                Ptr<NiPixelData>,
                u32,
                u32,
                Ptr,
                u32,
                u32,
            ) -> Ptr<NiPixelData>
        ),
        entry!(0x00a7c390, fn_00a7c390(Ptr<NiPixelData>)),
        entry!(0x00a7c020, ni_pixel_data_get_rtti() -> Ptr),
        entry!(0x00a7c450, ni_pixel_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_pixel_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7c020, &args![]).u32(), NI_PIXEL_DATA_RTTI);
    }

    #[test]
    fn ni_pixel_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_c390, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7c450, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7c450, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x74]]);
        assert_eq!(calls_to(&e, 0x00a7_c390), vec![vec![this]; 2]);
    }
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
    fn fn_00a7be30_counts_levels() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a7be30, &args![0u32, 8u32]).u32(), 0);
        assert_eq!(e.call(0x00a7be30, &args![8u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00a7be30, &args![1u32, 1u32]).u32(), 1);
        assert_eq!(e.call(0x00a7be30, &args![256u32, 16u32]).u32(), 9);
        assert_eq!(e.call(0x00a7be30, &args![3u32, 1000u32]).u32(), 10);
    }

    /// Doubles for the constructor's callees: `AllocateData` makes three tables of 16 words
    /// and the copies are real.
    fn constructor_doubles(e: &mut Engine) {
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PIXEL_FORMAT_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(ALLOCATE_DATA, |e, a| {
            let tables = e.mem.alloc(0xc0);
            e.mem.set_u32(a[0] + 0x54, tables);
            e.mem.set_u32(a[0] + 0x58, tables + 0x40);
            e.mem.set_u32(a[0] + 0x5c, tables + 0x80);
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
    }

    fn table(e: &Engine, data: u32, at: u32, count: u32) -> Vec<u32> {
        let table = e.mem.u32(data + at);
        (0..count).map(|i| e.mem.u32(table + 4 * i)).collect()
    }

    fn format(e: &mut Engine, bits: u8, kind: i32) -> u32 {
        let f = e.mem.alloc(0x44);
        e.mem.set_u8(f + 1, bits);
        e.mem.set_i32(f + 4, kind);
        e.mem.set_u32(f + 0x40, 0xabc);
        f
    }

    #[test]
    fn constructor_lays_out_uncompressed_levels() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let data = e.mem.alloc(0x74);
        let f = format(&mut e, 32, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a7c190, &args![data, 8u32, 2u32, f, 0u32, 1u32])
                .u32(),
            data
        );
        assert_eq!(e.mem.u32(data), NI_PIXEL_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x48), 0xabc);
        assert_eq!(e.mem.u32(data + 0x60), 4);
        assert_eq!(e.mem.u32(data + 0x64), 4);
        assert_eq!(e.mem.u32(data + 0x68), 1);
        assert_eq!(e.mem.u32(data + 0x6c), 1);
        assert_eq!(table(&e, data, 0x54, 4), vec![8, 4, 2, 1]);
        assert_eq!(table(&e, data, 0x58, 4), vec![2, 1, 1, 1]);
        assert_eq!(table(&e, data, 0x5c, 5), vec![0, 64, 80, 88, 92]);
        assert_eq!(calls_to(&e, ALLOCATE_DATA), vec![vec![data, 4, 1, 92]]);
        // Four bits per pixel (stride 0): half a byte each.
        let f = format(&mut e, 4, 2);
        e.call(0x00a7c190, &args![data, 4u32, 4u32, f, 2u32, 6u32]);
        assert_eq!(e.mem.u32(data + 0x64), 0);
        assert_eq!(table(&e, data, 0x5c, 3), vec![0, 8, 10]);
        assert_eq!(e.mem.u32(data + 0x6c), 6);
    }

    #[test]
    fn constructor_lays_out_compressed_levels() {
        let mut e = Engine::new();
        constructor_doubles(&mut e);
        let data = e.mem.alloc(0x74);
        let dxt1 = format(&mut e, 4, FORMAT_DXT1);
        e.call(0x00a7c190, &args![data, 8u32, 8u32, dxt1, 0u32, 1u32]);
        assert_eq!(e.mem.u32(data + 0x64), 0);
        // 2x2 blocks, then 1 block, 1 block, 1 block: 8 bytes each.
        assert_eq!(table(&e, data, 0x5c, 5), vec![0, 32, 40, 48, 56]);
        let dxt5 = format(&mut e, 8, FORMAT_DXT5);
        e.call(0x00a7c190, &args![data, 8u32, 4u32, dxt5, 2u32, 1u32]);
        assert_eq!(table(&e, data, 0x5c, 3), vec![0, 32, 48]);
        assert_eq!(table(&e, data, 0x58, 2), vec![4, 2]);
    }

    const DELETE_DOUBLE: u32 = 0x0070_6000;

    #[test]
    fn destructor_releases_the_palette_and_frees_the_buffers() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(FREE_DATA, |_, _| Ret::default());
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            Ret {
                eax: count,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0x74);
        let palette = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(palette, vtable);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_DOUBLE);
        e.mem.set_u32(palette + 4, 1);
        e.mem.set_u32(data + 0x4c, palette);
        e.call_log = Some(vec![]);
        e.call(0x00a7c390, &args![data]);
        assert_eq!(e.mem.u32(data), NI_PIXEL_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x4c), 0);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![palette]]);
        assert_eq!(calls_to(&e, FREE_DATA), vec![vec![data]]);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![data]]);
    }
}
