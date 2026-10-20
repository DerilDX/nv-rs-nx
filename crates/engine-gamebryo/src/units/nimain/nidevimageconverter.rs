//! `NiMain/nidevimageconverter.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

// Translated from 00a7b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a7aff0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0xa00 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_dev_image_converter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a7_aff0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xa00u32]);
    }
    this
}

/// `NiImageConverter`'s constructor (`00a76650`, engine map: `niimageconverter.cpp`;
/// `thiscall`, no stack words).
const IMAGE_CONVERTER_CONSTRUCT: u32 = 0x00a7_6650;
/// `NiDevImageConverter`'s virtual table (`0109d9c4`; its RTTI names `NiDevImageConverter`).
const NI_DEV_IMAGE_CONVERTER_VTABLE: u32 = 0x0109_d9c4;
/// `00a8bd20` (unnamed; `thiscall`, no stack words), run on the member at +0x780.
const MEMBER_780_CONSTRUCT: u32 = 0x00a8_bd20;
const MEMBER_780: u32 = 0x780;
/// The reader list (`NiTPointerList` at +0x990: head, tail, count).
const READERS: u32 = 0x990;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// Bytes of each reader the constructor makes.
const READER_SIZE: u32 = 0x180;
/// The three reader constructors, in the exe's order (`00a8e410` and `00a8d750` unnamed,
/// `00a8c190` in `niddsreader.cpp`; `thiscall`, no stack words).
const READER_CONSTRUCTORS: [u32; 3] = [0x00a8_e410, 0x00a8_d750, 0x00a8_c190];
/// `0043a010` (engine map: `NiTPointerListBase<..>::AddHead`; its body takes a cleared list
/// item from the item pool and answers it), `thiscall` on the list's count word.
const LIST_ITEM_NEW: u32 = 0x0043_a010;

// Translated from 00a7b110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::NiDevImageConverter` (Xbox PDB), `thiscall`, no stack words: the
/// `NiImageConverter` constructor, the class's virtual table, the member at +0x780
/// (`00a8bd20`), an empty reader list at +0x990, then three readers of 0x180 bytes (from the
/// memory manager; a failed allocation puts a null reader) made by `00a8e410`, `00a8d750` and
/// `00a8c190` and each put at the head of the list (`0043a010` gives the item). Answers `this`.
pub fn ni_dev_image_converter_ni_dev_image_converter(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(IMAGE_CONVERTER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_DEV_IMAGE_CONVERTER_VTABLE);
    e.call(
        MEMBER_780_CONSTRUCT,
        &args![this.addr().wrapping_add(MEMBER_780)],
    );
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr().wrapping_add(READERS));
    e.set(list, NiTPointerList::m_uiCount, 0);
    e.set(list, NiTPointerList::m_pkHead, 0);
    e.set(list, NiTPointerList::m_pkTail, 0);
    for constructor in READER_CONSTRUCTORS {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, READER_SIZE])
            .u32();
        let reader = if block != 0 {
            e.call(constructor, &args![block]).u32()
        } else {
            0
        };
        let count_word = list.addr().wrapping_add(8);
        let item: Ptr<NiTListItem> = e.call(LIST_ITEM_NEW, &args![count_word]).ptr();
        e.set(item, NiTListItem::m_element, reader);
        e.set(item, NiTListItem::m_pkPrev, 0);
        let head = e.get(list, NiTPointerList::m_pkHead);
        e.set(item, NiTListItem::m_pkNext, head);
        let head = e.get(list, NiTPointerList::m_pkHead);
        if head != 0 {
            e.set(
                Ptr::<NiTListItem>::new(head),
                NiTListItem::m_pkPrev,
                item.addr(),
            );
        } else {
            e.set(list, NiTPointerList::m_pkTail, item.addr());
        }
        let count = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(list, NiTPointerList::m_uiCount, count);
        e.set(list, NiTPointerList::m_pkHead, item.addr());
    }
    this
}

/// `NiDevImageConverter::PixelBits` (Xbox PDB name of the 0x18-byte record
/// `00a779b0` fills): the four component masks (+0..+0xc), the four left
/// shifts (+0x10..+0x13) and the four right shifts `8 - bits` (+0x14..+0x17).
const PIXEL_BITS_SIZE: u32 = 0x18;

/// `00a62190` (`thiscall` on an `NiPixelFormat`, `RET 4`: a component index):
/// the component's mask; `00a62230` (same shape): its shift byte.
const FORMAT_COMPONENT_MASK: u32 = 0x00a6_2190;
const FORMAT_COMPONENT_SHIFT: u32 = 0x00a6_2230;

/// The bits byte of the first of the format's four component records
/// (`format+0x14`, 0xc bytes each: type word, .., bits byte at `+8`) whose
/// type is `component`; 0 when none is.
fn component_bits(e: &Engine, format: u32, component: u32) -> u8 {
    for index in 0..4u32 {
        if e.mem.u32(format + 0x14 + index * 0xc) == component {
            return e.mem.u8(format + 0x1c + index * 0xc);
        }
    }
    0
}

/// `NiPixelFormat` (Xbox PDB) as `00a77980` reads it: `m_akComponents` from +0x14, four
/// entries of 0xc bytes, each the component kind word at +0 and its bit count byte at +8.
const PIXEL_FORMAT_COMPONENTS: u32 = 0x14;
const PIXEL_FORMAT_COMPONENT_SIZE: u32 = 0xc;
const PIXEL_FORMAT_COMPONENT_BITS: u32 = 0x8;

// Translated from 00a77980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nidevimageconverter.cpp` range), `thiscall` on an
/// `NiPixelFormat` with one word (a component kind; the shape of `NiPixelFormat::GetBits`,
/// unconfirmed): the bit count byte of the first of the four components whose kind is the word,
/// 0 when none is (only `AL` is set; the rest of `EAX` is not part of the answer).
pub fn fn_00a77980(e: &mut Engine, this: Ptr, kind: u32) -> u8 {
    let format = this.addr();
    for index in 0..4u32 {
        let entry = format
            .wrapping_add(PIXEL_FORMAT_COMPONENTS)
            .wrapping_add(index * PIXEL_FORMAT_COMPONENT_SIZE);
        if e.mem.u32(entry) == kind {
            return e.mem.u8(entry.wrapping_add(PIXEL_FORMAT_COMPONENT_BITS));
        }
    }
    0
}

// Translated from 00a779b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::PixelBits::PixelBits` (engine map name), `thiscall`,
/// `RET 4` (the pixel format): for each component 0..3 in turn its mask
/// (`00a62190`) at `+4c`, its shift byte (`00a62230`) at `+0x10+c` and
/// `8 - bits` of its component record at `+0x14+c`. Answers `this`.
pub fn ni_dev_image_converter_pixel_bits(e: &mut Engine, this: Ptr, format: Ptr) -> Ptr {
    let t = this.addr();
    for component in 0..4u32 {
        let mask = e
            .call(FORMAT_COMPONENT_MASK, &args![format, component])
            .u32();
        e.mem.set_u32(t + 4 * component, mask);
        let shift = e
            .call(FORMAT_COMPONENT_SHIFT, &args![format, component])
            .u32() as u8;
        e.mem.set_u8(t + 0x10 + component, shift);
        let bits = component_bits(e, format.addr(), component);
        e.mem.set_u8(t + 0x14 + component, 8u8.wrapping_sub(bits));
    }
    this
}

/// One channel of a pixel: the byte moved right by the record's `8 - bits`
/// byte, left by its shift byte, and masked (the x86 shift counts use their
/// low five bits).
fn channel(e: &Engine, bits: u32, index: u32, value: u32) -> u32 {
    let right = u32::from(e.mem.u8(bits + 0x14 + index));
    let left = u32::from(e.mem.u8(bits + 0x10 + index));
    value.wrapping_shr(right).wrapping_shl(left) & e.mem.u32(bits + 4 * index)
}

// Translated from 00a76f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: one of `ConvertPixelDataFormat`'s row
/// converters (`cdecl`, eight words: width, height, the level's size, the
/// destination, the palette, the destination's `PixelBits`, the source and
/// the source's `PixelBits`; the size, the palette and the source's bits are
/// not read). For `height` rows of `width` pixels: three source bytes become
/// one 32-bit word (each byte placed through the destination bits' shifts and
/// masks of components 0, 1 and 2, and 0xff for component 3).
#[allow(clippy::too_many_arguments)] // the converters' eight stack words
pub fn fn_00a76f80(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut out = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from + 1));
            let third = u32::from(e.mem.u8(from + 2));
            from = from.wrapping_add(3);
            let word = channel(e, bits, 0, first)
                | channel(e, bits, 1, second)
                | channel(e, bits, 2, third)
                | channel(e, bits, 3, 0xff);
            e.mem.set_u32(out, word);
            out = out.wrapping_add(4);
        }
    }
}

// Translated from 00a771d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its
/// bytes): one of `ConvertPixelDataFormat`'s row converters, the same eight
/// `cdecl` words as `00a76f80`. For `height` rows of `width` pixels: four
/// source bytes become one 32-bit word, each byte placed through the destination
/// bits' shifts and masks of components 0..3.
#[allow(clippy::too_many_arguments)] // the converters' eight stack words
pub fn fn_00a771d0(
    e: &mut Engine,
    width: u32,
    height: u32,
    _unused_size: u32,
    destination: u32,
    _unused_palette: u32,
    bits: u32,
    source: u32,
    _unused_source_bits: u32,
) {
    let mut out = destination;
    let mut from = source;
    for _ in 0..height {
        for _ in 0..width {
            let first = u32::from(e.mem.u8(from));
            let second = u32::from(e.mem.u8(from + 1));
            let third = u32::from(e.mem.u8(from + 2));
            let fourth = u32::from(e.mem.u8(from + 3));
            from = from.wrapping_add(4);
            let word = channel(e, bits, 0, first)
                | channel(e, bits, 2, third)
                | channel(e, bits, 1, second)
                | channel(e, bits, 3, fourth);
            e.mem.set_u32(out, word);
            out = out.wrapping_add(4);
        }
    }
}

/// `NiPixelFormat::operator==` (`00a76540`, `thiscall`, `RET 4`: the other
/// format) and the other comparison `00a765c0` (engine map
/// `NiPixelFormat::operator__ov2`, same shape).
const FORMAT_EQUALS: u32 = 0x00a7_6540;
const FORMAT_COMPARE_00A765C0: u32 = 0x00a7_65c0;
/// The static pixel formats the converter compares with (`NiPixelFormat`
/// globals of 0x44 bytes in `.data`).
const FORMAT_011AA180: u32 = 0x011a_a180;
const FORMAT_011AA1C8: u32 = 0x011a_a1c8;
const FORMAT_011AA210: u32 = 0x011a_a210;
const FORMAT_011AA258: u32 = 0x011a_a258;
const FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const FORMAT_011AA2E8: u32 = 0x011a_a2e8;
const FORMAT_011AA330: u32 = 0x011a_a330;
const FORMAT_011AA3C0: u32 = 0x011a_a3c0;
const FORMAT_011AA408: u32 = 0x011a_a408;
const FORMAT_011AA450: u32 = 0x011a_a450;

/// `format == other` through `00a76540`.
fn format_is(e: &mut Engine, format: u32, other: u32) -> bool {
    e.call(FORMAT_EQUALS, &args![format, other]).bool()
}

// Translated from 00a78010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::IsAnAlphaSupportedMipmapGenerationFormat` (engine
/// map name; a virtual method, `thiscall`, `RET 4`: the format): whether the
/// format equals `011aa1c8`, `011aa258` or `011aa2a0` (tested in that order).
pub fn ni_dev_image_converter_is_an_alpha_supported_mipmap_generation_format(
    e: &mut Engine,
    _this: Ptr,
    format: u32,
) -> bool {
    format_is(e, format, FORMAT_011AA1C8)
        || format_is(e, format, FORMAT_011AA258)
        || format_is(e, format, FORMAT_011AA2A0)
}

// Translated from 00a78050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::IsANonAlphaSupportedMipmapGenerationFormat` (engine
/// map name; a virtual method, `thiscall`, `RET 4`: the format): whether the
/// format equals `011aa180`, `011aa210` or `011aa2e8` (in that order).
pub fn ni_dev_image_converter_is_a_non_alpha_supported_mipmap_generation_format(
    e: &mut Engine,
    _this: Ptr,
    format: u32,
) -> bool {
    format_is(e, format, FORMAT_011AA180)
        || format_is(e, format, FORMAT_011AA210)
        || format_is(e, format, FORMAT_011AA2E8)
}

/// `00a77960` (`thiscall` on a format, no stack words): whether its format
/// word (+4) is 4..6.
const FORMAT_IS_COMPRESSED_00A77960: u32 = 0x00a7_7960;

// Translated from 00a77e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::CanConvertPixelData` (engine map name; virtual,
/// `thiscall`, `RET 8`: the source and the destination formats). The
/// destination's `PixelBits` are made first. Equal formats: true. Otherwise
/// both need bit 0 of their first byte, the destination's format word (+4)
/// must not be 4..6, and a 24-bit destination (+1) needs the masks 0xff00 for
/// component 1 and 0xff or 0xff0000 for component 0. Then by the source:
/// `011aa180`/`011aa1c8` (a destination of format 0 or 1 needs 24 or 32 bits;
/// any other must compare 0 with `00a765c0` against `011aa180`, else against
/// `011aa1c8`), `011aa210`/`011aa258` (format 0 or 1: 16, 24 or 32 bits; other:
/// the same two-step test against `011aa210` and `011aa258`),
/// `011aa2e8`/`011aa2a0`/a compressed source (`00a77960`) (format 0, 1, 8 or
/// 9: 16, 24 or 32 bits), `011aa408`/`011aa450` (format 8 or 9: 16, 24 or 32
/// bits), `011aa330`/`011aa3c0` (16 bits); anything else is false.
pub fn ni_dev_image_converter_can_convert_pixel_data(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
    destination: u32,
) -> bool {
    let kind = e.mem.u32(destination + 4);
    let depth = u32::from(e.mem.u8(destination + 1));
    let (mask_0, mask_1) = e.with_stack(PIXEL_BITS_SIZE, |e, bits| {
        ni_dev_image_converter_pixel_bits(e, bits, Ptr::new(destination));
        (e.mem.u32(bits.addr()), e.mem.u32(bits.addr() + 4))
    });
    if format_is(e, source, destination) {
        return true;
    }
    if e.mem.u8(source) & 1 == 0 || e.mem.u8(destination) & 1 == 0 {
        return false;
    }
    let format_word = e.mem.u32(destination + 4) as i32;
    if (4..=6).contains(&format_word) {
        return false;
    }
    if depth == 0x18 && (mask_1 != 0xff00 || (mask_0 != 0xff && mask_0 != 0x00ff_0000)) {
        return false;
    }
    let any_depth = |depth: u32| depth == 0x10 || depth == 0x18 || depth == 0x20;
    let palette_kind = kind == 8 || kind == 9;
    let second_test = |e: &mut Engine, first: u32, second: u32| {
        e.call(FORMAT_COMPARE_00A765C0, &args![destination, first])
            .u32() as u8
            == 0
            || e.call(FORMAT_COMPARE_00A765C0, &args![destination, second])
                .u32() as u8
                == 0
    };
    if format_is(e, source, FORMAT_011AA180) || format_is(e, source, FORMAT_011AA1C8) {
        if kind == 0 || kind == 1 {
            return depth == 0x18 || depth == 0x20;
        }
        return second_test(e, FORMAT_011AA180, FORMAT_011AA1C8);
    }
    if format_is(e, source, FORMAT_011AA210) || format_is(e, source, FORMAT_011AA258) {
        if kind == 0 || kind == 1 {
            return any_depth(depth);
        }
        return second_test(e, FORMAT_011AA210, FORMAT_011AA258);
    }
    if format_is(e, source, FORMAT_011AA2E8)
        || format_is(e, source, FORMAT_011AA2A0)
        || e.call(FORMAT_IS_COMPRESSED_00A77960, &args![source]).u32() as u8 != 0
    {
        if kind == 0 || kind == 1 {
            return any_depth(depth);
        }
        return palette_kind && any_depth(depth);
    }
    if format_is(e, source, FORMAT_011AA408) || format_is(e, source, FORMAT_011AA450) {
        return palette_kind && any_depth(depth);
    }
    if format_is(e, source, FORMAT_011AA330) || format_is(e, source, FORMAT_011AA3C0) {
        return depth == 0x10;
    }
    false
}

/// CRT `_strcpy_s` (`00ec65a6`, `cdecl`), `NiPath::Standardize` (`00aa2a00`,
/// `cdecl`), `NiFilename::NiFilename` (`00aa27d0`, `thiscall`, the path) and
/// `NiFile::GetFile` (`00aa1a20`, `cdecl`: path, mode, buffer size).
const STRCPY_S: u32 = 0x00ec_65a6;
const PATH_STANDARDIZE: u32 = 0x00aa_2a00;
const NI_FILENAME_CONSTRUCT: u32 = 0x00aa_27d0;
const NI_FILE_GET_FILE: u32 = 0x00aa_1a20;

// Translated from 00a77d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a77d10 - the /GS stack cookie (011c16bc, __security_check_cookie 00ec408c) is not translated
/// `NiDevImageConverter::ReadImageFile` (engine map name; virtual,
/// `thiscall`, `RET 0xc`: the file name, a word passed to the reader, and an
/// open file or 0). The name is copied into a 0x104-byte buffer, standardized
/// and split (`NiFilename`). Each reader of the list at `+0x990` whose slot
/// `+4` accepts the extension gets the file (the given one, or a new one from
/// `NiFile::GetFile(path, 0, 0x8000)`; none answers 0): a file whose slot `+4`
/// answers false is deleted (slot 0 with 1, even a given one) and 0 answered;
/// otherwise the reader's slot `+8` reads it with the word. A result deletes
/// the file (even a given one) and is answered; no result deletes a file made
/// here and goes on with the next reader. After the last reader a given file
/// is deleted and 0 answered.
pub fn ni_dev_image_converter_read_image_file(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    word: u32,
    given: u32,
) -> u32 {
    let mut item = e.mem.u32(this.addr() + READERS);
    e.with_stack(0x50c, |e, frame| {
        // The frame: the word (+0), the split name (+4, 0x403 bytes), the
        // path (+0x408, 0x104 bytes).
        let base = frame.addr();
        let filename = base + 4;
        let path = base + 0x408;
        e.mem.set_u32(base, word);
        e.call(STRCPY_S, &args![path, 0x104u32, name]);
        e.call(PATH_STANDARDIZE, &args![path]);
        e.call(NI_FILENAME_CONSTRUCT, &args![filename, path]);
        while item != 0 {
            let reader = e.mem.u32(item + 8);
            item = e.mem.u32(item);
            if !e.vcall(reader, 4, &args![filename + 0x103]).bool() {
                continue;
            }
            let file = if given != 0 {
                given
            } else {
                e.call(NI_FILE_GET_FILE, &args![path, 0u32, 0x8000u32])
                    .u32()
            };
            if file == 0 {
                return 0;
            }
            if !e.vcall(file, 4, &args![]).bool() {
                e.vcall(file, 0, &args![1u32]);
                return 0;
            }
            let word = e.mem.u32(base);
            let data = e.vcall(reader, 8, &args![file, word]).u32();
            if data != 0 {
                e.vcall(file, 0, &args![1u32]);
                return data;
            }
            if given == 0 {
                e.vcall(file, 0, &args![1u32]);
            }
        }
        if given != 0 {
            e.vcall(given, 0, &args![1u32]);
        }
        0
    })
}

/// `NiTextureCodec::ExpandCompressedData` (`00a8be70`, `thiscall` on the codec
/// at `+0x780`, `RET 4`: the pixel data), the `NiPointer` assignment
/// `00a5f860` (`thiscall` on the pointer, `RET 4`: the object), the
/// `NiPointer` release `00b66f20` (`thiscall` on the pointer), the
/// power-of-two test `00a77920` (`cdecl`, a word; a byte answer), the Gamebryo
/// allocation `00aa13e0` (`cdecl`: the size) and
/// `NiPixelData::NiPixelData_ov2` (`00a7c190`, `thiscall`, `RET 0x14`: width,
/// height, format address, mipmap levels, faces).
const CODEC_EXPAND_COMPRESSED_DATA: u32 = 0x00a8_be70;
const NI_POINTER_ASSIGN: u32 = 0x00a5_f860;
const NI_POINTER_RELEASE: u32 = 0x00b6_6f20;
const IS_POWER_OF_TWO_00A77920: u32 = 0x00a7_7920;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// `InterlockedDecrement` (import slot).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiPixelData` words the converter reads: the pixels (+0x50), the widths
/// (+0x54), the heights (+0x58), the level offsets (+0x5c), the level count
/// (+0x60) and the face count (+0x6c).
const DATA_PIXELS: u32 = 0x50;
const DATA_WIDTHS: u32 = 0x54;
const DATA_HEIGHTS: u32 = 0x58;
const DATA_OFFSETS: u32 = 0x5c;
const DATA_LEVELS: u32 = 0x60;
const DATA_FACES: u32 = 0x6c;

/// The inline `NiPointer` release of `object` (when not null).
fn release_object(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a7a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::ConvertPixelData` (engine map name; virtual,
/// `thiscall`, `RET 0x10`: the source data, the destination format, a
/// destination data or 0, and the mipmap byte). Both need bit 0 of their
/// format's first byte (source `+8`). A compressed source (format word `+0xc`
/// 4..6) of another format than the destination is expanded first (the codec
/// at `+0x780`). With the mipmap byte, a one-level source of power-of-two
/// width (and height, `00a77920`) and a destination format 0, 1, 8 or 9 gets
/// its levels (`GenerateMipmapLevels` with the destination data). A given
/// destination of the destination format (and more than one level when
/// mipmapping) that is the source or takes it (slot `+0x30` with -1) is
/// answered after the two pointers are released (`00b66f20`). Otherwise a
/// palette destination format (8 or 9) gets a palettized copy (slot `+0x34`);
/// a new `NiPixelData` of the source's size, the destination format, the
/// source's levels and faces is filled through slot `+0x30` (-1) and
/// answered, or 0 when that fails (the new data is not freed); the
/// intermediate pointers are released either way.
pub fn ni_dev_image_converter_convert_pixel_data(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    format: u32,
    destination: u32,
    mipmap: u32,
) -> u32 {
    let t = this.addr();
    if e.mem.u8(source + 8) & 1 == 0 || e.mem.u8(format) & 1 == 0 {
        return 0;
    }
    e.with_stack(12, |e, frame| {
        // The frame: the expanded data (+0), the mipmapped data (+4), the
        // palettized data (+8); each an `NiPointer`.
        let expanded = frame.addr();
        let mipmapped = expanded + 4;
        let palettized = expanded + 8;
        let mut current = source;
        let source_kind = e.mem.u32(source + 0xc) as i32;
        if (4..=6).contains(&source_kind) && e.mem.u32(format + 4) != e.mem.u32(source + 0xc) {
            let data = e
                .call(CODEC_EXPAND_COMPRESSED_DATA, &args![t + MEMBER_780, source])
                .u32();
            e.call(NI_POINTER_ASSIGN, &args![expanded, data]);
            current = e.mem.u32(expanded);
        }
        if mipmap as u8 != 0 && e.mem.u32(current + DATA_LEVELS) <= 1 {
            let width = e.mem.u32(e.mem.u32(current + DATA_WIDTHS));
            if width != 0 && width & width.wrapping_sub(1) == 0 {
                let height = e.mem.u32(e.mem.u32(current + DATA_HEIGHTS));
                let square = e.call(IS_POWER_OF_TWO_00A77920, &args![height]).u32() as u8 != 0;
                let kind = e.mem.u32(format + 4);
                if square && (kind == 0 || kind == 1 || kind == 8 || kind == 9) {
                    let data = ni_dev_image_converter_generate_mipmap_levels(
                        e,
                        this,
                        current,
                        destination,
                    );
                    e.call(NI_POINTER_ASSIGN, &args![mipmapped, data]);
                    let made = e.mem.u32(mipmapped);
                    if made != 0 {
                        current = made;
                    }
                }
            }
        }
        if destination != 0
            && format_is(e, destination + 8, format)
            && (mipmap as u8 == 0 || e.mem.u32(destination + DATA_LEVELS) > 1)
            && (destination == current
                || e.vcall(t, 0x30, &args![destination, current, 0xffff_ffffu32])
                    .bool())
        {
            e.call(NI_POINTER_RELEASE, &args![mipmapped]);
            e.call(NI_POINTER_RELEASE, &args![expanded]);
            return destination;
        }
        let mut palette_data = 0;
        let kind = e.mem.u32(format + 4);
        if kind == 8 || kind == 9 {
            let data = e.vcall(t, 0x34, &args![current, format]).u32();
            e.call(NI_POINTER_ASSIGN, &args![palettized, data]);
            palette_data = e.mem.u32(palettized);
            if palette_data != 0 {
                current = palette_data;
            }
        }
        let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
        let made = if block != 0 {
            let width = e.mem.u32(e.mem.u32(current + DATA_WIDTHS));
            let height = e.mem.u32(e.mem.u32(current + DATA_HEIGHTS));
            let levels = e.mem.u32(current + DATA_LEVELS);
            let faces = e.mem.u32(current + DATA_FACES);
            e.call(
                NI_PIXEL_DATA_CONSTRUCT,
                &args![block, width, height, format, levels, faces],
            )
            .u32()
        } else {
            0
        };
        let filled = e
            .vcall(t, 0x30, &args![made, current, 0xffff_ffffu32])
            .bool();
        release_object(e, palette_data);
        let mipmapped_data = e.mem.u32(mipmapped);
        release_object(e, mipmapped_data);
        let expanded_data = e.mem.u32(expanded);
        release_object(e, expanded_data);
        if filled {
            made
        } else {
            0
        }
    })
}

/// `NiPixelFormat::NiPixelFormat` (`00a7cca0`, `thiscall`, no stack words).
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;

/// One box-filtered level: `size` bytes per pixel (3 or 4) from the level
/// above at `from` into `to`, `width` x `height` pixels, the level above
/// `above_width` x `above_height`. A level above one pixel wide (or else one
/// pixel high) averages pixel pairs; otherwise each pixel averages a 2x2
/// block of two rows.
#[allow(clippy::too_many_arguments)] // the loop's registers
fn filter_level(
    e: &mut Engine,
    size: u32,
    from: u32,
    to: u32,
    width: u32,
    height: u32,
    above_width: u32,
    above_height: u32,
) {
    let mut from = from;
    let mut to = to;
    let row = above_width.wrapping_mul(size);
    let pair = |e: &mut Engine, from: u32, to: u32| {
        for byte in 0..size {
            let sum = u32::from(e.mem.u8(from + byte)) + u32::from(e.mem.u8(from + size + byte));
            e.mem.set_u8(to + byte, (sum >> 1) as u8);
        }
    };
    if above_width == 1 {
        for _ in 0..height {
            pair(e, from, to);
            to = to.wrapping_add(size);
            from = from.wrapping_add(2 * size);
        }
        return;
    }
    if above_height == 1 {
        for _ in 0..width {
            pair(e, from, to);
            to = to.wrapping_add(size);
            from = from.wrapping_add(2 * size);
        }
        return;
    }
    for _ in 0..height {
        if width != 0 {
            let mut next = from.wrapping_add(row);
            for _ in 0..width {
                for byte in 0..size {
                    let sum = u32::from(e.mem.u8(from + byte))
                        + u32::from(e.mem.u8(from + size + byte))
                        + u32::from(e.mem.u8(next + byte))
                        + u32::from(e.mem.u8(next + size + byte));
                    e.mem.set_u8(to + byte, (sum >> 2) as u8);
                }
                to = to.wrapping_add(size);
                from = from.wrapping_add(2 * size);
                next = next.wrapping_add(2 * size);
            }
        }
        from = from.wrapping_add(row);
    }
}

// Translated from 00a78090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::GenerateMipmapLevels` (engine map name; virtual,
/// `thiscall`, `RET 8`: the source data and a destination data or 0). The
/// working format is `011aa2e8` when the source format passes slot `+0x24`
/// (non-alpha mipmap formats), else `011aa2a0` when it passes slot `+0x20`,
/// else 0 is answered (a constructed `NiPixelFormat` local is overwritten by a
/// copy of 0x11 words). The destination is used when it has that format, more
/// than one level and the source's face count; otherwise a new `NiPixelData`
/// of the source's size with that format, all levels (0) and the source's
/// faces. A destination other than the source is filled from it (slot
/// `+0x30` with face 0; false answers 0). Then for each face every level from
/// 1 is box-filtered from the one above it: 3 bytes per pixel for `011aa2e8`,
/// 4 for `011aa2a0`. Answers the destination.
pub fn ni_dev_image_converter_generate_mipmap_levels(
    e: &mut Engine,
    this: Ptr,
    source: u32,
    destination: u32,
) -> u32 {
    let t = this.addr();
    e.with_stack(0x44, |e, working| {
        let working = working.addr();
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![working]);
        let template = if e.vcall(t, 0x24, &args![source + 8]).bool() {
            FORMAT_011AA2E8
        } else if e.vcall(t, 0x20, &args![source + 8]).bool() {
            FORMAT_011AA2A0
        } else {
            return 0;
        };
        for word in 0..0x11u32 {
            let value = e.mem.u32(template + 4 * word);
            e.mem.set_u32(working + 4 * word, value);
        }
        let reuse = destination != 0
            && format_is(e, destination + 8, working)
            && e.mem.u32(destination + DATA_LEVELS) > 1
            && e.mem.u32(destination + DATA_FACES) == e.mem.u32(source + DATA_FACES);
        let target = if reuse {
            destination
        } else {
            let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
            if block != 0 {
                let faces = e.mem.u32(source + DATA_FACES);
                let height = e.mem.u32(e.mem.u32(source + DATA_HEIGHTS));
                let width = e.mem.u32(e.mem.u32(source + DATA_WIDTHS));
                e.call(
                    NI_PIXEL_DATA_CONSTRUCT,
                    &args![block, width, height, working, 0u32, faces],
                )
                .u32()
            } else {
                0
            }
        };
        let levels = e.mem.u32(target + DATA_LEVELS);
        if source != target && !e.vcall(t, 0x30, &args![target, source, 0u32]).bool() {
            return 0;
        }
        let mut face = 0;
        while face < e.mem.u32(source + DATA_FACES) {
            let size = if format_is(e, working, FORMAT_011AA2E8) {
                3
            } else if format_is(e, working, FORMAT_011AA2A0) {
                4
            } else {
                0
            };
            if size != 0 {
                let mut level = 1;
                while level < levels {
                    let offsets = e.mem.u32(target + DATA_OFFSETS);
                    let all = e.mem.u32(target + DATA_LEVELS);
                    let face_offset = e.mem.u32(offsets + 4 * all).wrapping_mul(face);
                    let pixels = e.mem.u32(target + DATA_PIXELS);
                    let from = e
                        .mem
                        .u32(offsets + 4 * (level - 1))
                        .wrapping_add(pixels)
                        .wrapping_add(face_offset);
                    let to = pixels
                        .wrapping_add(face_offset)
                        .wrapping_add(e.mem.u32(offsets + 4 * level));
                    let widths = e.mem.u32(target + DATA_WIDTHS);
                    let width = e.mem.u32(widths + 4 * level);
                    let above_width = e.mem.u32(widths + 4 * (level - 1));
                    let heights = e.mem.u32(target + DATA_HEIGHTS);
                    let height = e.mem.u32(heights + 4 * level);
                    let above_height = if above_width == 1 {
                        0
                    } else {
                        e.mem.u32(heights + 4 * (level - 1))
                    };
                    filter_level(e, size, from, to, width, height, above_width, above_height);
                    level += 1;
                }
            }
            face += 1;
        }
        target
    })
}

/// The shared list-node pool's lock (`011c5f80`, count at `011c5f84`, taken
/// with `0040fbf0`) and free list (`011c5f58`).
const NODE_POOL_LOCK: u32 = 0x011c_5f80;
const NODE_POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const NODE_POOL_FREE: u32 = 0x011c_5f58;
const LOCK_TAKE: u32 = 0x0040_fbf0;
/// `NiTPointerListBase<..>::RemoveAll` (`00b71300`), the member at `+0x780`'s
/// destructor (`00a29680`) and `NiImageConverter`'s destructor body
/// (`00a76680`), all `thiscall` with no stack words.
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
const MEMBER_780_DESTRUCT: u32 = 0x00a2_9680;
const IMAGE_CONVERTER_DESTRUCT: u32 = 0x00a7_6680;

// Translated from 00a7aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiDevImageConverter`'s destructor body
/// (`thiscall`, no stack words). Stores the class's vtable; while the reader
/// list (`+0x990`) has items: the head is unlinked, its reader kept and
/// cleared, the item goes back to the shared pool (under the pool lock), the
/// count goes down and the reader (when not null) is deleted (slot 0 with 1).
/// Then `RemoveAll` on the list, the member at `+0x780`'s destructor and
/// `NiImageConverter`'s destructor body.
pub fn fn_00a7aff0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, NI_DEV_IMAGE_CONVERTER_VTABLE);
    let list: Ptr<NiTPointerList> = Ptr::new(t + READERS);
    while e.get(list, NiTPointerList::m_uiCount) != 0 {
        let item = e.get(list, NiTPointerList::m_pkHead);
        let next = e.mem.u32(item);
        e.set(list, NiTPointerList::m_pkHead, next);
        if next != 0 {
            e.mem.set_u32(next + 4, 0);
        } else {
            e.set(list, NiTPointerList::m_pkTail, 0);
        }
        let reader = e.mem.u32(item + 8);
        e.mem.set_u32(item + 8, 0);
        e.call(LOCK_TAKE, &args![NODE_POOL_LOCK, 0u32]);
        e.mem.set_u32(item + 4, 0);
        let free = e.global::<u32>(NODE_POOL_FREE);
        e.mem.set_u32(item, free);
        let count = e.global::<u32>(NODE_POOL_LOCK_COUNT).wrapping_sub(1);
        e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
        e.set_global::<u32>(NODE_POOL_FREE, item);
        if count == 0 {
            e.set_global::<u32>(NODE_POOL_LOCK, 0);
        }
        let remaining = e.get(list, NiTPointerList::m_uiCount).wrapping_sub(1);
        e.set(list, NiTPointerList::m_uiCount, remaining);
        if reader != 0 {
            e.vcall(reader, 0, &args![1u32]);
        }
    }
    e.call(POINTER_LIST_REMOVE_ALL, &args![list]);
    e.call(MEMBER_780_DESTRUCT, &args![t + MEMBER_780]);
    e.call(IMAGE_CONVERTER_DESTRUCT, &args![this]);
}

/// `NiPalette`'s copy constructor (`00a8b080`, `thiscall`, `RET 4`: the
/// other palette), its assignment (`00a8b190`, `thiscall`, `RET 4`), its
/// constructor from entries (`00a8b110`, `thiscall`, `RET 0xc`: 0, the entry
/// count, the entries) and the pixel data's palette setter (`00a76a60`,
/// `thiscall`, `RET 4`).
const PALETTE_COPY: u32 = 0x00a8_b080;
const PALETTE_ASSIGN: u32 = 0x00a8_b190;
const PALETTE_FROM_ENTRIES: u32 = 0x00a8_b110;
const DATA_SET_PALETTE: u32 = 0x00a7_6a60;
/// `NiPixelData`'s palette (`+0x4c`).
const DATA_PALETTE: u32 = 0x4c;
/// The CRT `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;

/// The levels `first..=last` of every face copied straight from the source
/// (each level's size from the source's offsets) with `memcpy`.
fn copy_levels(e: &mut Engine, destination: u32, source: u32, faces: u32, first: u32, last: u32) {
    let mut face = 0;
    while face < faces {
        let mut level = first;
        while level <= last {
            let offsets = e.mem.u32(source + DATA_OFFSETS);
            let size = e
                .mem
                .u32(offsets + 4 * (level + 1))
                .wrapping_sub(e.mem.u32(offsets + 4 * level));
            let all = e.mem.u32(source + DATA_LEVELS);
            let from = e
                .mem
                .u32(offsets + 4 * all)
                .wrapping_mul(face)
                .wrapping_add(e.mem.u32(offsets + 4 * level))
                .wrapping_add(e.mem.u32(source + DATA_PIXELS));
            let offsets = e.mem.u32(destination + DATA_OFFSETS);
            let all = e.mem.u32(destination + DATA_LEVELS);
            let to = e
                .mem
                .u32(offsets + 4 * all)
                .wrapping_mul(face)
                .wrapping_add(e.mem.u32(offsets + 4 * level))
                .wrapping_add(e.mem.u32(destination + DATA_PIXELS));
            e.call(MEMCPY_00EC44D0, &args![to, from, size]);
            level += 1;
        }
        face += 1;
    }
}

/// The destination gets a copy of the source's palette: assigned into its
/// own when it has one, else a new copy (0x24 bytes; a failed allocation sets
/// a null palette).
fn copy_palette(e: &mut Engine, destination: u32, source_palette: u32) {
    let own = e.mem.u32(destination + DATA_PALETTE);
    if own != 0 {
        e.call(PALETTE_ASSIGN, &args![own, source_palette]);
        return;
    }
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    let palette = if block != 0 {
        e.call(PALETTE_COPY, &args![block, source_palette]).u32()
    } else {
        0
    };
    e.call(DATA_SET_PALETTE, &args![destination, palette]);
}

/// A new palette of `count` entries made from the source palette's entries
/// (`+0x14`) for the destination (a failed allocation sets a null palette).
fn palette_from_entries(e: &mut Engine, destination: u32, source_palette: u32, count: u32) {
    let block = e.call(NI_ALLOCATE, &args![0x24u32]).u32();
    if block == 0 {
        e.call(DATA_SET_PALETTE, &args![destination, 0u32]);
        return;
    }
    let entries = e.mem.u32(source_palette + 0x14);
    let palette = e
        .call(PALETTE_FROM_ENTRIES, &args![block, 0u32, count, entries])
        .u32();
    e.call(DATA_SET_PALETTE, &args![destination, palette]);
}

/// The second palette step of the palettized copies: the source must have a
/// palette (else false); it is copied, and when the destination's palette
/// has a word at `+0xc` the fourth byte of its first entry becomes 0xff.
fn copy_palette_opaque(e: &mut Engine, destination: u32, source: u32) -> bool {
    let source_palette = e.mem.u32(source + DATA_PALETTE);
    if source_palette == 0 {
        return false;
    }
    copy_palette(e, destination, source_palette);
    let palette = e.mem.u32(destination + DATA_PALETTE);
    if e.mem.u32(palette + 0xc) != 0 {
        let entries = e.mem.u32(palette + 0x14);
        e.mem.set_u8(entries + 3, 0xff);
    }
    true
}

/// What `ConvertPixelDataFormat` does after its format tests.
enum Conversion {
    /// Refused: false.
    Refuse,
    /// Copied straight, then `then`.
    Copy(AfterCopy),
    /// Through the row converter at this address.
    Convert(u32),
}

/// The palette step after a straight palettized copy.
enum AfterCopy {
    /// `copy_palette_opaque`.
    Opaque,
    /// A new palette of this many entries (the source must have a palette).
    Entries(u32),
}

/// The converter for a source matched to a pair of static formats: by
/// destination bits per pixel among `(depth, converter)`.
fn by_depth(depth: u32, table: &[(u32, u32)]) -> Conversion {
    table
        .iter()
        .find(|(d, _)| *d == depth)
        .map_or(Conversion::Refuse, |(_, c)| Conversion::Convert(*c))
}

// Translated from 00a7a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDevImageConverter::ConvertPixelDataFormat` (engine map name; virtual,
/// `thiscall`, `RET 0xc`: the destination data, the source data and a level,
/// -1 for all). The level-0 sizes and the face counts must match, and the
/// level(s) exist in both (all: the destination's levels, which the source
/// must have). Equal formats copy the levels straight (`memcpy`) and the
/// palette (assigned, or a new copy). Otherwise (both `PixelBits` made; a
/// compressed destination, 4..6, refuses; a 24-bit destination needs the
/// green mask 0xff00 and a red one of 0xff or 0xff0000) the source format
/// picks the way: the palettized `011aa180`/`011aa1c8`/`011aa210`/`011aa258`
/// (a palette required) convert to a format-0/1 destination by its bits per
/// pixel, or copy straight to their partner format (then the palette:
/// opaque copy, or a new 16- or 256-entry one); `011aa2e8`, `011aa2a0` (format
/// 0/1), `011aa408`, `011aa450` (format 8/9) convert by bits per pixel;
/// `011aa330`/`011aa3c0` convert into each other; any other format-0/1
/// source converts into `011aa2a0` (16 and 32 bits by whether the source has
/// an alpha mask) or `011aa2e8`. The row converter runs for each face and
/// level with the width, height and size (of the destination), both level
/// addresses, the source palette and both `PixelBits`. Answers whether it
/// converted.
pub fn ni_dev_image_converter_convert_pixel_data_format(
    e: &mut Engine,
    _this: Ptr,
    destination: u32,
    source: u32,
    level: u32,
) -> bool {
    let first_width = e.mem.u32(e.mem.u32(source + DATA_WIDTHS));
    if first_width != e.mem.u32(e.mem.u32(destination + DATA_WIDTHS)) {
        return false;
    }
    if e.mem.u32(e.mem.u32(source + DATA_HEIGHTS))
        != e.mem.u32(e.mem.u32(destination + DATA_HEIGHTS))
    {
        return false;
    }
    let faces = e.mem.u32(source + DATA_FACES);
    if faces != e.mem.u32(destination + DATA_FACES) {
        return false;
    }
    let (first, last) = if level == u32::MAX {
        let levels = e.mem.u32(destination + DATA_LEVELS);
        if e.mem.u32(source + DATA_LEVELS) < levels {
            return false;
        }
        (0, levels.wrapping_sub(1))
    } else {
        if level >= e.mem.u32(source + DATA_LEVELS) || level >= e.mem.u32(destination + DATA_LEVELS)
        {
            return false;
        }
        (level, level)
    };
    if format_is(e, source + 8, destination + 8) {
        copy_levels(e, destination, source, faces, first, last);
        let source_palette = e.mem.u32(source + DATA_PALETTE);
        if source_palette != 0 {
            copy_palette(e, destination, source_palette);
        }
        return true;
    }
    e.with_stack(0x38, |e, frame| {
        // The frame: the source's `PixelBits` (+8) and the destination's
        // (+0x20).
        let source_bits = frame.addr() + 8;
        let destination_bits = frame.addr() + 0x20;
        let source_kind = e.mem.u32(source + 0xc);
        let depth = u32::from(e.mem.u8(destination + 9));
        let kind = e.mem.u32(destination + 0xc);
        let source_depth = u32::from(e.mem.u8(source + 9));
        ni_dev_image_converter_pixel_bits(e, Ptr::new(destination_bits), Ptr::new(destination + 8));
        ni_dev_image_converter_pixel_bits(e, Ptr::new(source_bits), Ptr::new(source + 8));
        if (4..=6).contains(&kind) {
            return false;
        }
        if depth == 0x18 {
            if e.mem.u32(destination_bits + 4) != 0xff00 {
                return false;
            }
            let red = e.mem.u32(destination_bits);
            if red != 0xff && red != 0x00ff_0000 {
                return false;
            }
        }
        let plain_kind = kind == 0 || kind == 1;
        let palette_kind = kind == 8 || kind == 9;
        let has_palette = |e: &Engine| e.mem.u32(source + DATA_PALETTE) != 0;
        let conversion = if format_is(e, source + 8, FORMAT_011AA180) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(depth, &[(0x18, 0x00a7_8f70), (0x20, 0x00a7_90a0)])
            } else if format_is(e, destination + 8, FORMAT_011AA1C8) {
                Conversion::Copy(AfterCopy::Opaque)
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA1C8) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(depth, &[(0x18, 0x00a7_8f70), (0x20, 0x00a7_9310)])
            } else if format_is(e, destination + 8, FORMAT_011AA180) {
                Conversion::Copy(AfterCopy::Entries(0x10))
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA210) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_95d0),
                        (0x18, 0x00a7_9e20),
                        (0x20, 0x00a7_98c0),
                    ],
                )
            } else if format_is(e, destination + 8, FORMAT_011AA258) {
                Conversion::Copy(AfterCopy::Opaque)
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA258) {
            if !has_palette(e) {
                Conversion::Refuse
            } else if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_9ae0),
                        (0x18, 0x00a7_9e20),
                        (0x20, 0x00a7_9ef0),
                    ],
                )
            } else if format_is(e, destination + 8, FORMAT_011AA210) {
                Conversion::Copy(AfterCopy::Entries(0x100))
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA2E8) {
            if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_6ea0),
                        (0x18, 0x00a7_a160),
                        (0x20, 0x00a7_6f80),
                    ],
                )
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA2A0) {
            if plain_kind {
                by_depth(
                    depth,
                    &[
                        (0x10, 0x00a7_7040),
                        (0x18, 0x00a7_7110),
                        (0x20, 0x00a7_71d0),
                    ],
                )
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA408) {
            if palette_kind {
                by_depth(depth, &[(0x10, 0x00a7_7880), (0x20, 0x00a7_7800)])
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA450) {
            if palette_kind {
                by_depth(depth, &[(0x10, 0x00a7_7740), (0x20, 0x00a7_76b0)])
            } else {
                Conversion::Refuse
            }
        } else if format_is(e, source + 8, FORMAT_011AA330)
            && format_is(e, destination + 8, FORMAT_011AA3C0)
        {
            Conversion::Convert(0x00a7_6d70)
        } else if format_is(e, source + 8, FORMAT_011AA3C0)
            && format_is(e, destination + 8, FORMAT_011AA330)
        {
            Conversion::Convert(0x00a7_6e90)
        } else if source_kind != 0 && source_kind != 1 {
            Conversion::Refuse
        } else if format_is(e, destination + 8, FORMAT_011AA2A0) {
            let alpha = e.mem.u32(source_bits + 0xc) != 0;
            match source_depth {
                0x10 => Conversion::Convert(if alpha { 0x00a7_7330 } else { 0x00a7_73d0 }),
                0x18 => Conversion::Convert(0x00a7_7460),
                0x20 => Conversion::Convert(if alpha { 0x00a7_7590 } else { 0x00a7_7630 }),
                _ => Conversion::Refuse,
            }
        } else if format_is(e, destination + 8, FORMAT_011AA2E8) {
            by_depth(
                source_depth,
                &[
                    (0x10, 0x00a7_72b0),
                    (0x18, 0x00a7_a1f0),
                    (0x20, 0x00a7_7510),
                ],
            )
        } else {
            Conversion::Refuse
        };
        match conversion {
            Conversion::Refuse => false,
            Conversion::Copy(after) => {
                copy_levels(e, destination, source, faces, first, last);
                match after {
                    AfterCopy::Opaque => copy_palette_opaque(e, destination, source),
                    AfterCopy::Entries(count) => {
                        let source_palette = e.mem.u32(source + DATA_PALETTE);
                        if source_palette == 0 {
                            return false;
                        }
                        palette_from_entries(e, destination, source_palette, count);
                        true
                    }
                }
            }
            Conversion::Convert(converter) => {
                let mut face = 0;
                while face < faces {
                    let mut level = first;
                    while level <= last {
                        let offsets = e.mem.u32(source + DATA_OFFSETS);
                        let all = e.mem.u32(source + DATA_LEVELS);
                        let from = e
                            .mem
                            .u32(offsets + 4 * all)
                            .wrapping_mul(face)
                            .wrapping_add(e.mem.u32(offsets + 4 * level))
                            .wrapping_add(e.mem.u32(source + DATA_PIXELS));
                        let palette = e.mem.u32(source + DATA_PALETTE);
                        let offsets = e.mem.u32(destination + DATA_OFFSETS);
                        let all = e.mem.u32(destination + DATA_LEVELS);
                        let to = e
                            .mem
                            .u32(offsets + 4 * all)
                            .wrapping_mul(face)
                            .wrapping_add(e.mem.u32(offsets + 4 * level))
                            .wrapping_add(e.mem.u32(destination + DATA_PIXELS));
                        let size = e
                            .mem
                            .u32(offsets + 4 * (level + 1))
                            .wrapping_sub(e.mem.u32(offsets + 4 * level));
                        let height = e.mem.u32(e.mem.u32(destination + DATA_HEIGHTS) + 4 * level);
                        let width = e.mem.u32(e.mem.u32(destination + DATA_WIDTHS) + 4 * level);
                        e.call(
                            converter,
                            &args![
                                width,
                                height,
                                size,
                                to,
                                palette,
                                destination_bits,
                                from,
                                source_bits
                            ],
                        );
                        level += 1;
                    }
                    face += 1;
                }
                true
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7b260, ni_dev_image_converter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a7_b110,
            ni_dev_image_converter_ni_dev_image_converter(Ptr) -> Ptr
        ),
        entry!(0x00a7_7980, fn_00a77980(Ptr, u32) -> u8),
        entry!(0x00a7_79b0, ni_dev_image_converter_pixel_bits(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a7_6f80,
            fn_00a76f80(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00a7_71d0,
            fn_00a771d0(u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00a7_8010, ni_dev_image_converter_is_an_alpha_supported_mipmap_generation_format(Ptr, u32) -> bool),
        entry!(0x00a7_8050, ni_dev_image_converter_is_a_non_alpha_supported_mipmap_generation_format(Ptr, u32) -> bool),
        entry!(0x00a7_7e40, ni_dev_image_converter_can_convert_pixel_data(Ptr, u32, u32) -> bool),
        entry!(0x00a7_7d10, ni_dev_image_converter_read_image_file(Ptr, u32, u32, u32) -> u32),
        entry!(0x00a7_a280, ni_dev_image_converter_convert_pixel_data(Ptr, u32, u32, u32, u32) -> u32),
        entry!(0x00a7_8090, ni_dev_image_converter_generate_mipmap_levels(Ptr, u32, u32) -> u32),
        entry!(0x00a7_aff0, fn_00a7aff0(Ptr)),
        entry!(0x00a7_a4f0, ni_dev_image_converter_convert_pixel_data_format(Ptr, u32, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dev_image_converter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a7_aff0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a7b260, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a7b260, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xa00]]);
        assert_eq!(calls_to(&e, 0x00a7_aff0), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b000s_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn rig(failing_allocation: u32) -> Engine {
        let mut e = Engine::new();
        e.register(IMAGE_CONVERTER_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(MEMBER_780_CONSTRUCT, |_, a| a[0].into_ret());
        let mut allocations = 0;
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            allocations += 1;
            if allocations == failing_allocation {
                0u32.into_ret()
            } else {
                e.mem.alloc(a[1]).into_ret()
            }
        });
        for constructor in READER_CONSTRUCTORS {
            e.register(constructor, |_, a| a[0].into_ret());
        }
        e.register(LIST_ITEM_NEW, |e, _| {
            let item = e.mem.alloc(0x0c);
            e.mem.set_u32(item, 0xdead);
            item.into_ret()
        });
        e
    }

    #[test]
    fn ni_dev_image_converter_puts_its_three_readers_at_the_head() {
        let mut e = rig(0);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this + READERS, 0x1111);
        assert_eq!(e.call(0x00a7_b110, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DEV_IMAGE_CONVERTER_VTABLE);
        assert_eq!(calls(&e, MEMBER_780_CONSTRUCT), vec![vec![this + 0x780]]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, READER_SIZE]; 3]
        );
        assert_eq!(calls(&e, LIST_ITEM_NEW), vec![vec![this + READERS + 8]; 3]);
        let readers: Vec<u32> = READER_CONSTRUCTORS
            .iter()
            .map(|c| calls(&e, *c)[0][0])
            .collect();
        // Head to tail: the last reader made first.
        let head = e.mem.u32(this + READERS);
        let tail = e.mem.u32(this + READERS + 4);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
        let mut item = head;
        let mut walked = vec![];
        let mut previous = 0;
        while item != 0 {
            assert_eq!(e.mem.u32(item + 4), previous);
            walked.push(e.mem.u32(item + 8));
            previous = item;
            item = e.mem.u32(item);
        }
        assert_eq!(previous, tail);
        assert_eq!(walked, vec![readers[2], readers[1], readers[0]]);
    }

    #[test]
    fn ni_dev_image_converter_keeps_a_null_reader_when_allocation_fails() {
        let mut e = rig(2);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xa00);
        e.call(0x00a7_b110, &args![this]);
        assert!(calls(&e, READER_CONSTRUCTORS[1]).is_empty());
        let head = e.mem.u32(this + READERS);
        let middle = e.mem.u32(head);
        assert_eq!(e.mem.u32(middle + 8), 0);
        assert_eq!(e.mem.u32(this + READERS + 8), 3);
    }
}

#[cfg(test)]
mod b008_tests {
    use super::*;

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()[1..].to_vec()
    }

    /// An object whose vtable slots `0..0x40` are recorders `base + slot`.
    fn object(e: &mut Engine, base: u32) -> u32 {
        let table = e.mem.alloc(0x40);
        for slot in (0..0x40).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        let o = e.mem.alloc(0x80);
        e.mem.set_u32(o, table);
        o
    }

    /// A pixel format whose component records are `(type, bits)`.
    fn format(e: &mut Engine, components: [(u32, u8); 4]) -> u32 {
        let f = e.mem.alloc(0x44);
        for (i, (kind, bits)) in components.iter().enumerate() {
            e.mem.set_u32(f + 0x14 + 0xc * i as u32, *kind);
            e.mem.set_u8(f + 0x1c + 0xc * i as u32, *bits);
        }
        f
    }

    #[test]
    fn fn_00a77980_finds_the_component_bits() {
        let mut e = Engine::new();
        let f = format(&mut e, [(3, 1), (1, 6), (1, 7), (0, 5)]);
        assert_eq!(e.call(0x00a7_7980, &args![f, 1u32]).u32() & 0xff, 6);
        assert_eq!(e.call(0x00a7_7980, &args![f, 0u32]).u32() & 0xff, 5);
        assert_eq!(e.call(0x00a7_7980, &args![f, 2u32]).u32() & 0xff, 0);
    }

    #[test]
    fn ni_dev_image_converter_pixel_bits_fills_the_record() {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, a| (0x100 + a[1]).into_ret());
        e.register(FORMAT_COMPONENT_SHIFT, |_, a| (0x320 + a[1]).into_ret());
        let f = format(&mut e, [(0, 5), (1, 6), (2, 5), (7, 1)]);
        let bits = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00a7_79b0, &args![bits, f]).u32(), bits);
        for c in 0..4 {
            assert_eq!(e.mem.u32(bits + 4 * c), 0x100 + c);
            assert_eq!(e.mem.u8(bits + 0x10 + c), 0x20 + c as u8);
        }
        assert_eq!(
            (0..4)
                .map(|c| e.mem.u8(bits + 0x14 + c))
                .collect::<Vec<_>>(),
            vec![3, 2, 3, 8]
        );
    }

    /// `PixelBits` for 8-bit channels placed at bytes 2, 1, 0, 3 (BGRA words).
    fn bgra_bits(e: &mut Engine) -> u32 {
        let bits = e.mem.alloc(0x18);
        for (c, (mask, left)) in [
            (0x00ff_0000u32, 16u8),
            (0xff00, 8),
            (0xff, 0),
            (0xff00_0000, 24),
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_u32(bits + 4 * c as u32, *mask);
            e.mem.set_u8(bits + 0x10 + c as u32, *left);
            e.mem.set_u8(bits + 0x14 + c as u32, 0);
        }
        bits
    }

    #[test]
    fn fn_00a76f80_packs_three_bytes_with_opaque_alpha() {
        let mut e = Engine::new();
        let bits = bgra_bits(&mut e);
        let source = e.mem.alloc(12);
        e.mem
            .write(source, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        let out = e.mem.alloc(16);
        e.call(
            0x00a7_6f80,
            &args![2u32, 2u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0xff01_0203);
        assert_eq!(e.mem.u32(out + 12), 0xff0a_0b0c);
    }

    #[test]
    fn fn_00a771d0_packs_four_bytes() {
        let mut e = Engine::new();
        let bits = bgra_bits(&mut e);
        e.mem.set_u8(bits + 0x14, 4);
        let source = e.mem.alloc(8);
        e.mem.write(source, &[0xf0, 2, 3, 4, 5, 6, 7, 8]);
        let out = e.mem.alloc(8);
        e.call(
            0x00a7_71d0,
            &args![1u32, 2u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0x040f_0203);
        assert_eq!(e.mem.u32(out + 4), 0x0800_0607);
        // No rows: nothing written.
        e.mem.set_u32(out, 0x55);
        e.call(
            0x00a7_71d0,
            &args![1u32, 0u32, 0u32, out, 0u32, bits, source, 0u32],
        );
        assert_eq!(e.mem.u32(out), 0x55);
    }

    #[test]
    fn mipmap_format_tests_compare_three_formats() {
        let mut e = Engine::new();
        e.register(FORMAT_EQUALS, |_, a| {
            u32::from(a[1] == FORMAT_011AA2A0 || a[1] == FORMAT_011AA2E8).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_8010, &args![0u32, 0x10u32]).bool());
        let compared: Vec<u32> = calls(&mut e).into_iter().map(|c| c.1[1]).collect();
        assert_eq!(
            compared,
            vec![FORMAT_011AA1C8, FORMAT_011AA258, FORMAT_011AA2A0]
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_8050, &args![0u32, 0x10u32]).bool());
        let compared: Vec<u32> = calls(&mut e).into_iter().map(|c| c.1[1]).collect();
        assert_eq!(
            compared,
            vec![FORMAT_011AA180, FORMAT_011AA210, FORMAT_011AA2E8]
        );
        e.register(FORMAT_EQUALS, |_, _| Ret::default());
        assert!(!e.call(0x00a7_8010, &args![0u32, 0x10u32]).bool());
        assert!(!e.call(0x00a7_8050, &args![0u32, 0x10u32]).bool());
    }

    /// `CanConvertPixelData` with a source equal to `source_as` (one of the
    /// static formats, or 0 for none), a destination of `kind` and `depth`.
    fn can_convert(source_as: u32, kind: u32, depth: u8, compare: u32) -> bool {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, a| {
            [0xffu32, 0xff00, 0xff_0000, 0][a[1] as usize].into_ret()
        });
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        e.register(FORMAT_IS_COMPRESSED_00A77960, |_, _| Ret::default());
        e.register_double(FORMAT_COMPARE_00A765C0, move |_, _| compare.into_ret());
        let source = e.mem.alloc(0x44);
        e.mem.set_u8(source, 1);
        let destination = e.mem.alloc(0x44);
        e.mem.set_u8(destination, 1);
        e.mem.set_u8(destination + 1, depth);
        e.mem.set_u32(destination + 4, kind);
        e.register_double(FORMAT_EQUALS, move |_, a| {
            u32::from(a[0] == source && a[1] == source_as).into_ret()
        });
        e.call(0x00a7_7e40, &args![0u32, source, destination])
            .bool()
    }

    #[test]
    fn ni_dev_image_converter_can_convert_pixel_data_by_source() {
        assert!(can_convert(FORMAT_011AA180, 0, 0x20, 1));
        assert!(!can_convert(FORMAT_011AA180, 1, 0x10, 1));
        assert!(can_convert(FORMAT_011AA1C8, 2, 0x10, 0));
        assert!(!can_convert(FORMAT_011AA1C8, 2, 0x10, 1));
        assert!(can_convert(FORMAT_011AA258, 0, 0x10, 1));
        assert!(can_convert(FORMAT_011AA2A0, 8, 0x18, 1));
        assert!(!can_convert(FORMAT_011AA2A0, 3, 0x18, 1));
        assert!(can_convert(FORMAT_011AA450, 9, 0x20, 1));
        assert!(!can_convert(FORMAT_011AA450, 0, 0x20, 1));
        assert!(can_convert(FORMAT_011AA3C0, 3, 0x10, 1));
        assert!(!can_convert(FORMAT_011AA3C0, 3, 0x20, 1));
        assert!(!can_convert(0, 0, 0x20, 1));
        // A compressed destination is refused.
        assert!(!can_convert(FORMAT_011AA180, 5, 0x20, 1));
    }

    #[test]
    fn ni_dev_image_converter_can_convert_pixel_data_equal_and_flags() {
        let mut e = Engine::new();
        e.register(FORMAT_COMPONENT_MASK, |_, _| Ret::default());
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        e.register(FORMAT_EQUALS, |_, _| 1u32.into_ret());
        let f = e.mem.alloc(0x44);
        assert!(e.call(0x00a7_7e40, &args![0u32, f, f]).bool());
        e.register(FORMAT_EQUALS, |_, _| Ret::default());
        let g = e.mem.alloc(0x44);
        assert!(!e.call(0x00a7_7e40, &args![0u32, f, g]).bool());
        // A 24-bit destination needs the 0xff00 green mask.
        e.mem.set_u8(f, 1);
        e.mem.set_u8(g, 1);
        e.mem.set_u8(g + 1, 0x18);
        assert!(!e.call(0x00a7_7e40, &args![0u32, f, g]).bool());
    }

    const READ_SLOTS: u32 = 0x0131_0000;
    const FILE_SLOTS: u32 = 0x0132_0000;

    fn reader_engine(accepts: u32, data: u32, file_ok: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        for f in [STRCPY_S, PATH_STANDARDIZE, NI_FILENAME_CONSTRUCT] {
            e.register(f, |_, _| Ret::default());
        }
        let reader = object(&mut e, READ_SLOTS);
        e.register_double(READ_SLOTS + 4, move |_, _| accepts.into_ret());
        e.register_double(READ_SLOTS + 8, move |_, _| data.into_ret());
        let file = object(&mut e, FILE_SLOTS);
        e.register_double(FILE_SLOTS + 4, move |_, _| file_ok.into_ret());
        e.register_double(NI_FILE_GET_FILE, move |_, _| file.into_ret());
        let item = e.mem.alloc(12);
        e.mem.set_u32(item + 8, reader);
        let this = e.mem.alloc(0xa00);
        e.mem.set_u32(this + READERS, item);
        e.call_log = Some(vec![]);
        (e, this, file)
    }

    #[test]
    fn ni_dev_image_converter_read_image_file_reads_with_a_new_file() {
        let (mut e, this, file) = reader_engine(1, 0x7000, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0x7000
        );
        let log = calls(&mut e);
        let read: Vec<_> = log.iter().filter(|c| c.0 == READ_SLOTS + 8).collect();
        assert_eq!(read[0].1[1..].to_vec(), vec![file, 9]);
        assert!(log.contains(&(FILE_SLOTS, vec![file, 1])));
        assert_eq!(log[0].1[1..].to_vec(), vec![0x104, 0x500]);
    }

    #[test]
    fn ni_dev_image_converter_read_image_file_failures() {
        // No result: the new file is deleted and 0 answered.
        let (mut e, this, file) = reader_engine(1, 0, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&mut e).contains(&(FILE_SLOTS, vec![file, 1])));
        // A file that is not ready is deleted.
        let (mut e, this, file) = reader_engine(1, 0x7000, 0);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, 0u32])
                .u32(),
            0
        );
        assert!(calls(&mut e).contains(&(FILE_SLOTS, vec![file, 1])));
        // No reader accepts: a given file is deleted at the end.
        let (mut e, this, file) = reader_engine(0, 0x7000, 1);
        assert_eq!(
            e.call(0x00a7_7d10, &args![this, 0x500u32, 9u32, file])
                .u32(),
            0
        );
        let log = calls(&mut e);
        assert!(log.contains(&(FILE_SLOTS, vec![file, 1])));
        assert!(log.iter().all(|c| c.0 != NI_FILE_GET_FILE));
    }

    const CONVERTER_SLOTS: u32 = 0x0133_0000;

    /// Pixel data with one level of `width` x `height`, the format word at
    /// +0xc and bit 0 of its format byte set.
    fn pixel_data(e: &mut Engine, width: u32, height: u32, kind: u32, levels: u32) -> u32 {
        let d = e.mem.alloc(0x74);
        e.mem.set_u8(d + 8, 1);
        e.mem.set_u32(d + 0xc, kind);
        let w = e.mem.alloc(4);
        e.mem.set_u32(w, width);
        let h = e.mem.alloc(4);
        e.mem.set_u32(h, height);
        e.mem.set_u32(d + DATA_WIDTHS, w);
        e.mem.set_u32(d + DATA_HEIGHTS, h);
        e.mem.set_u32(d + DATA_LEVELS, levels);
        e.mem.set_u32(d + DATA_FACES, 1);
        d
    }

    #[test]
    fn ni_dev_image_converter_convert_pixel_data_makes_new_data() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        let block = e.mem.alloc(0x74);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        let source = pixel_data(&mut e, 4, 4, 0, 1);
        let format = e.mem.alloc(0x44);
        e.mem.set_u8(format, 1);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
            .u32();
        assert_eq!(made, block);
        let log = calls(&mut e);
        assert!(log.contains(&(NI_PIXEL_DATA_CONSTRUCT, vec![block, 4, 4, format, 1, 1])));
        assert!(log.contains(&(
            CONVERTER_SLOTS + 0x30,
            vec![this, block, source, 0xffff_ffff]
        )));
        // The fill failing answers 0.
        e.register(CONVERTER_SLOTS + 0x30, |_, _| Ret::default());
        let made = e
            .call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
            .u32();
        assert_eq!(made, 0);
        // A format without bit 0 answers 0 at once.
        e.mem.set_u8(format, 0);
        assert_eq!(
            e.call(0x00a7_a280, &args![this, source, format, 0u32, 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn ni_dev_image_converter_convert_pixel_data_reuses_the_destination() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        e.register(FORMAT_EQUALS, |_, _| 1u32.into_ret());
        e.register(NI_POINTER_RELEASE, |_, _| Ret::default());
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(CODEC_EXPAND_COMPRESSED_DATA, |_, _| Ret::default());
        let source = pixel_data(&mut e, 4, 4, 5, 1);
        let destination = pixel_data(&mut e, 4, 4, 0, 1);
        let format = e.mem.alloc(0x44);
        e.mem.set_u8(format, 1);
        e.mem.set_u32(format + 4, 5);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x00a7_a280, &args![this, source, format, destination, 0u32])
            .u32();
        assert_eq!(made, destination);
        let log = calls(&mut e);
        assert!(log.iter().all(|c| c.0 != CODEC_EXPAND_COMPRESSED_DATA));
        assert_eq!(log.iter().filter(|c| c.0 == NI_POINTER_RELEASE).count(), 2);
        // A compressed source of another format is expanded first.
        e.mem.set_u32(format + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_a280, &args![this, source, format, destination, 0u32]);
        assert!(calls(&mut e).contains(&(
            CODEC_EXPAND_COMPRESSED_DATA,
            vec![this + MEMBER_780, source]
        )));
    }

    #[test]
    fn ni_dev_image_converter_generate_mipmap_levels_box_filters_32_bit() {
        let mut e = Engine::new();
        let this = object(&mut e, CONVERTER_SLOTS);
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, _| Ret::default());
        e.register(CONVERTER_SLOTS + 0x24, |_, _| Ret::default());
        e.register(CONVERTER_SLOTS + 0x20, |_, _| 1u32.into_ret());
        e.map(FORMAT_011AA2A0, 0x44);
        e.mem.set_u32(FORMAT_011AA2A0, 0x2a0);
        e.map(FORMAT_011AA2E8, 0x44);
        // The working copy is compared by its first word.
        e.register(FORMAT_EQUALS, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0x2a0 && a[1] == FORMAT_011AA2A0).into_ret()
        });
        // A 2x2 source and a 3-level destination (2x2, 1x1 ... ) built by hand.
        let target = e.mem.alloc(0x74);
        let pixels = e.mem.alloc(0x20);
        e.mem.write(
            pixels,
            &[
                10, 20, 30, 40, 12, 22, 32, 42, 14, 24, 34, 44, 16, 26, 36, 46,
            ],
        );
        let offsets = e.mem.alloc(16);
        for (i, v) in [0u32, 16, 20, 20].iter().enumerate() {
            e.mem.set_u32(offsets + 4 * i as u32, *v);
        }
        let widths = e.mem.alloc(8);
        e.mem.set_u32(widths, 2);
        e.mem.set_u32(widths + 4, 1);
        let heights = e.mem.alloc(8);
        e.mem.set_u32(heights, 2);
        e.mem.set_u32(heights + 4, 1);
        e.mem.set_u32(target + DATA_PIXELS, pixels);
        e.mem.set_u32(target + DATA_OFFSETS, offsets);
        e.mem.set_u32(target + DATA_WIDTHS, widths);
        e.mem.set_u32(target + DATA_HEIGHTS, heights);
        e.mem.set_u32(target + DATA_LEVELS, 2);
        e.mem.set_u32(target + DATA_FACES, 1);
        e.register_double(NI_ALLOCATE, move |_, _| target.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CONVERTER_SLOTS + 0x30, |_, _| 1u32.into_ret());
        let source = e.mem.alloc(0x74);
        e.mem.set_u32(source + DATA_FACES, 1);
        e.mem.set_u32(source + DATA_WIDTHS, widths);
        e.mem.set_u32(source + DATA_HEIGHTS, heights);
        let made = e.call(0x00a7_8090, &args![this, source, 0u32]).u32();
        assert_eq!(made, target);
        let level: Vec<u8> = (0..4).map(|i| e.mem.u8(pixels + 16 + i)).collect();
        assert_eq!(level, vec![13, 23, 33, 43]);
        // Neither mipmap format: 0.
        e.register(CONVERTER_SLOTS + 0x20, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_8090, &args![this, source, 0u32]).u32(), 0);
    }

    #[test]
    fn filter_level_pairs_rows_and_columns() {
        let mut e = Engine::new();
        let from = e.mem.alloc(12);
        e.mem.write(from, &[0, 10, 20, 2, 12, 22]);
        let to = e.mem.alloc(4);
        // A level above one pixel wide: pairs of pixels.
        filter_level(&mut e, 3, from, to, 1, 1, 1, 2);
        assert_eq!(
            (0..3).map(|i| e.mem.u8(to + i)).collect::<Vec<_>>(),
            vec![1, 11, 21]
        );
    }

    #[test]
    fn fn_00a7aff0_takes_the_readers_down() {
        let mut e = Engine::new();
        e.map(NODE_POOL_FREE, 4);
        e.map(NODE_POOL_LOCK, 8);
        e.register(LOCK_TAKE, |e, _| {
            let count = e.global::<u32>(NODE_POOL_LOCK_COUNT) + 1;
            e.set_global::<u32>(NODE_POOL_LOCK_COUNT, count);
            Ret::default()
        });
        for f in [
            POINTER_LIST_REMOVE_ALL,
            MEMBER_780_DESTRUCT,
            IMAGE_CONVERTER_DESTRUCT,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let reader = object(&mut e, READ_SLOTS);
        let this = e.mem.alloc(0xa00);
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        e.mem.set_u32(a, b);
        e.mem.set_u32(a + 8, reader);
        e.mem.set_u32(b + 4, a);
        e.mem.set_u32(this + READERS, a);
        e.mem.set_u32(this + READERS + 4, b);
        e.mem.set_u32(this + READERS + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a7_aff0, &args![this]);
        let log = calls(&mut e);
        assert_eq!(e.mem.u32(this), NI_DEV_IMAGE_CONVERTER_VTABLE);
        assert_eq!(e.mem.u32(this + READERS + 8), 0);
        assert_eq!(e.global::<u32>(NODE_POOL_FREE), b);
        assert!(log.contains(&(READ_SLOTS, vec![reader, 1])));
        let tail: Vec<u32> = log.iter().rev().take(3).map(|c| c.0).collect();
        assert_eq!(
            tail,
            vec![
                IMAGE_CONVERTER_DESTRUCT,
                MEMBER_780_DESTRUCT,
                POINTER_LIST_REMOVE_ALL
            ]
        );
    }
}

#[cfg(test)]
mod b008_format_tests {
    use super::*;

    /// One-face pixel data of `levels` levels (4x4, 2x2, ...) with 4 bytes
    /// per pixel.
    fn data(e: &mut Engine, levels: u32, depth: u8, kind: u32) -> u32 {
        let d = e.mem.alloc(0x74);
        e.mem.set_u8(d + 9, depth);
        e.mem.set_u32(d + 0xc, kind);
        let widths = e.mem.alloc(4 * levels);
        let heights = e.mem.alloc(4 * levels);
        let offsets = e.mem.alloc(4 * levels + 4);
        let mut offset = 0;
        for l in 0..levels {
            let side = 4 >> l;
            e.mem.set_u32(widths + 4 * l, side);
            e.mem.set_u32(heights + 4 * l, side);
            e.mem.set_u32(offsets + 4 * l, offset);
            offset += side * side * 4;
        }
        e.mem.set_u32(offsets + 4 * levels, offset);
        let pixels = e.mem.alloc(offset);
        e.mem.set_u32(d + DATA_WIDTHS, widths);
        e.mem.set_u32(d + DATA_HEIGHTS, heights);
        e.mem.set_u32(d + DATA_OFFSETS, offsets);
        e.mem.set_u32(d + DATA_PIXELS, pixels);
        e.mem.set_u32(d + DATA_LEVELS, levels);
        e.mem.set_u32(d + DATA_FACES, 1);
        d
    }

    const CONVERTER: u32 = 0x00a7_90a0;

    fn engine(equal: fn(&Engine, u32, u32) -> bool) -> Engine {
        let mut e = Engine::new();
        e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
        e.register(FORMAT_COMPONENT_MASK, |_, a| {
            [0xffu32, 0xff00, 0xff_0000, 0][a[1] as usize].into_ret()
        });
        e.register(FORMAT_COMPONENT_SHIFT, |_, _| Ret::default());
        for f in [PALETTE_ASSIGN, DATA_SET_PALETTE, CONVERTER] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(PALETTE_COPY, |_, a| a[0].into_ret());
        e.register(PALETTE_FROM_ENTRIES, |_, a| a[0].into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        // Formats compare equal by the rule given.
        e.register_double(FORMAT_EQUALS, move |e, a| {
            u32::from(equal(e, a[0], a[1])).into_ret()
        });
        e
    }

    #[test]
    fn same_formats_copy_every_level_and_the_palette() {
        let mut e = engine(|e, a, b| e.mem.u32(a + 4) == e.mem.u32(b + 4));
        let source = data(&mut e, 3, 32, 0);
        let destination = data(&mut e, 3, 32, 0);
        e.mem.set_u32(source + DATA_PALETTE, 0x7000);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a7_a4f0,
                &args![0u32, destination, source, 0xffff_ffffu32]
            )
            .bool());
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMCPY_00EC44D0)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(copies, vec![64, 16, 4]);
        assert!(log.iter().any(|c| c.0 == PALETTE_COPY));
        // A missing level refuses.
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 3u32])
            .bool());
        // Different sizes refuse.
        let small = data(&mut e, 1, 32, 0);
        e.mem.set_u32(e.mem.u32(small + DATA_WIDTHS), 2);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, small, source, 0u32])
            .bool());
    }

    #[test]
    fn a_palettized_source_converts_by_depth() {
        // The source format's word at +0x3c (data +0x44) is 0x180 to match 011aa180.
        fn rule(e: &Engine, a: u32, b: u32) -> bool {
            b == FORMAT_011AA180 && e.mem.u32(a + 0x3c) == 0x180
        }
        let mut e = engine(rule);
        let source = data(&mut e, 2, 8, 8);
        e.mem.set_u32(source + 0x44, 0x180);
        e.mem.set_u32(source + DATA_PALETTE, 0x7000);
        let destination = data(&mut e, 2, 32, 0);
        e.mem.set_u32(destination + 0x44, 0x999);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 1u32])
            .bool());
        let calls: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == CONVERTER)
            .map(|c| c.1)
            .collect();
        assert_eq!(calls.len(), 1);
        let pixels = e.mem.u32(destination + DATA_PIXELS);
        assert_eq!(calls[0][..5].to_vec(), vec![2, 2, 16, pixels + 64, 0x7000]);
        // Without a palette: refused.
        e.mem.set_u32(source + DATA_PALETTE, 0);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 1u32])
            .bool());
    }

    #[test]
    fn unknown_formats_and_compressed_destinations_refuse() {
        let mut e = engine(|_, _, _| false);
        let source = data(&mut e, 1, 32, 3);
        let destination = data(&mut e, 1, 32, 0);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
        e.mem.set_u32(destination + 0xc, 5);
        assert!(!e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
    }

    #[test]
    fn palettized_copies_set_up_the_palette() {
        fn rule(e: &Engine, a: u32, b: u32) -> bool {
            (b == FORMAT_011AA258 && e.mem.u32(a + 0x3c) == 0x258)
                || (b == FORMAT_011AA210 && e.mem.u32(a + 0x3c) == 0x210)
        }
        let mut e = engine(rule);
        let source = data(&mut e, 1, 8, 8);
        e.mem.set_u32(source + 0x44, 0x258);
        let palette = e.mem.alloc(0x24);
        e.mem.set_u32(palette + 0x14, 0x7700);
        e.mem.set_u32(source + DATA_PALETTE, palette);
        let destination = data(&mut e, 1, 8, 8);
        e.mem.set_u32(destination + 0x44, 0x210);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a7_a4f0, &args![0u32, destination, source, 0u32])
            .bool());
        let log = e.call_log.take().unwrap();
        let made: Vec<_> = log.iter().filter(|c| c.0 == PALETTE_FROM_ENTRIES).collect();
        assert_eq!(made[0].1[1..].to_vec(), vec![0, 0x100, 0x7700]);
        assert!(log.iter().any(|c| c.0 == MEMCPY_00EC44D0));
    }
}

#[cfg(test)]
mod b000y_tests {
    use super::*;

    #[test]
    fn fn_00a77980_answers_the_first_matching_components_bits() {
        let mut e = Engine::new();
        let format = e.mem.alloc(0x44);
        for (index, (kind, bits)) in [(0x11u32, 5u8), (0x12, 6), (0x12, 9), (0x13, 0)]
            .iter()
            .enumerate()
        {
            let entry = format + 0x14 + 0xc * index as u32;
            e.mem.set_u32(entry, *kind);
            e.mem.set_u8(entry + 8, *bits);
        }
        let bits = |e: &mut Engine, kind: u32| e.call(0x00a7_7980, &args![format, kind]).u8();
        assert_eq!(bits(&mut e, 0x11), 5);
        assert_eq!(bits(&mut e, 0x12), 6);
        assert_eq!(bits(&mut e, 0x13), 0);
        assert_eq!(bits(&mut e, 0x14), 0);
    }
}
