//! `NiMain/niddsreader.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The virtual tables the constructor stores (RTTI `NiImageReader`, then `NiDDSReader`).
const NI_IMAGE_READER_VTABLE: u32 = 0x0109_f3d0;
const NI_DDS_READER_VTABLE: u32 = 0x0109_f48c;
/// `InitializeCriticalSection` (import slot) and `NiPixelFormat::NiPixelFormat` (engine map,
/// `00a7cca0`; `thiscall`, no stack words).
const IMPORT_INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
const NI_PIXEL_FORMAT_CONSTRUCT: u32 = 0x00a7_cca0;
/// The pixel format copied into the reader (0x11 words at `011aa498`) and where it goes.
const PIXEL_FORMAT_SOURCE_011AA498: u32 = 0x011a_a498;
const PIXEL_FORMAT: u32 = 0x110;
const PIXEL_FORMAT_WORDS: u32 = 0x11;

// Translated from 00a8c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`niddsreader.cpp`): the `NiDDSReader` constructor (the RTTI of
/// the virtual table it stores), `thiscall`, no stack words. The `NiImageReader` virtual
/// table, its critical section at +0x80 (`InitializeCriticalSection`), the `NiDDSReader`
/// virtual table, the pixel format at +0x110 (`NiPixelFormat::NiPixelFormat`), +0x100 and
/// +0x104 cleared, then the 0x11 words at `011aa498` copied over the pixel format (`REP
/// MOVSD`). Answers `this`.
pub fn fn_00a8c190(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_IMAGE_READER_VTABLE);
    e.call(
        IMPORT_INITIALIZE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.mem.set_u32(base, NI_DDS_READER_VTABLE);
    let format = base.wrapping_add(PIXEL_FORMAT);
    e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
    e.mem.set_u32(base.wrapping_add(0x100), 0);
    e.mem.set_u32(base.wrapping_add(0x104), 0);
    for word in 0..PIXEL_FORMAT_WORDS {
        let value = e.mem.u32(PIXEL_FORMAT_SOURCE_011AA498 + 4 * word);
        e.mem.set_u32(format.wrapping_add(4 * word), value);
    }
    this
}

/// The `EnterCriticalSection` and `LeaveCriticalSection` import slots
/// (`kernel32`, stdcall, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// Copies the 0x11 words of a pixel format (`REP MOVSD`).
fn copy_format(e: &mut Engine, to: u32, from: u32) {
    for word in 0..0x11u32 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// `NiStricmp` (`00aa1360`, `cdecl`: two texts) and `".dds"` in `.rdata`
/// (`0101aa24`).
const NI_STRICMP: u32 = 0x00aa_1360;
const DDS_EXTENSION: u32 = 0x0101_aa24;

// Translated from 00a8c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDDSReader::CanReadFile` (engine map name; virtual, `thiscall`, `RET 4`:
/// the extension): whether `NiStricmp(extension, ".dds")` is 0.
pub fn ni_dds_reader_can_read_file(e: &mut Engine, _this: Ptr, extension: u32) -> bool {
    e.call(NI_STRICMP, &args![extension, DDS_EXTENSION]).u32() == 0
}

/// The read helper `00a575b0` (`cdecl`: the stream and the destination of
/// one 4-byte item).
const STREAM_READ_WORD: u32 = 0x00a5_75b0;
/// The seek origin word the reader passes with its skips (`010a2484`).
const SEEK_ORIGIN: u32 = 0x010a_2484;
/// `00a8c100` (`thiscall`, `RET 0x14`: red, green, blue, alpha masks and the
/// bit count): whether the masks make a format the reader takes.
const MASKS_SUPPORTED_00A8C100: u32 = 0x00a8_c100;
/// `NiPixelFormat::CreateFromRGBAMasks` (`00a7cd20`, `cdecl`: the result's
/// address, the bit count and the four masks; answers the result).
const FORMAT_FROM_MASKS: u32 = 0x00a7_cd20;
/// `00a8c120` (`thiscall`, `RET 0x48`: a format's address and a format of
/// 0x11 words by value): answers the address of the format to use.
const CHOOSE_FORMAT_00A8C120: u32 = 0x00a8_c120;
/// The `"DDS "` magic word.
const DDS_MAGIC: u32 = 0x2053_4444;

/// The stream's binary read (its function at `+8`, `cdecl`: the stream, the
/// destination, the item size, the address of the count word set to `size`
/// first, and 1), its result added to the stream's `+4`.
fn read_raw(e: &mut Engine, stream: u32, destination: u32, size: u32, count_word: u32) {
    let read = e.mem.u32(stream + 8);
    e.mem.set_u32(count_word, size);
    let done = e
        .call(read, &args![stream, destination, size, count_word, 1u32])
        .u32();
    let total = e.mem.u32(stream + 4).wrapping_add(done);
    e.mem.set_u32(stream + 4, total);
}

// Translated from 00a8c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDDSReader::ReadHeader` (engine map name; virtual, `thiscall`, `RET 0x18`:
/// the stream, the width, height, pixel format (0x11 words), mipmapped byte
/// and face count to fill). Under the reader's critical section (`+0x80`):
/// the magic must be `"DDS "` (a byte-swapped one turns the stream's swapping
/// on, its slot `+0x10` with 1), the header size 0x7c and the flags must have
/// 0x1007; the height (`+0x100`) and width (`+0x104`) are read, then the pitch
/// and the depth (a set depth with the depth flag 0x800000 refuses), the
/// mipmap count, 0x2c bytes skipped (slot `+0x14` with the origin word
/// `010a2484`), the pixel format's size, flags and FourCC. A FourCC format
/// (flag 4) picks a static format (`DXT1`, `DXT3`, `DXT5`, or the six codes
/// 0x6f..0x74) into `+0x110` and the answer, skipping the masks (0x14 bytes);
/// an RGB format (flag 0x40) reads the bit count and the red, green, blue and
/// alpha masks (alpha 0 without flag 1), which must pass `00a8c100`, and the
/// answer gets `CreateFromRGBAMasks` (also kept at `+0x110`) refined by
/// `00a8c120`. Then the caps: one of 8 or 0x401000 is needed; caps 2 refuse a
/// volume (0x200000), a cube map (0x200) needs all six faces (0x400..0x8000)
/// and gives 6 faces (else 1); 0xc bytes are skipped. The width and height go
/// out; a complex mipmapped file (caps 8 and 0x400000) with a mipmap count
/// other than 1 sets the mipmapped byte and keeps the count at `+0x108`, else
/// the byte is cleared and `+0x108` is 1. Answers whether the header was
/// taken (the section is left either way).
#[allow(clippy::too_many_arguments)] // the reader's six stack words
pub fn ni_dds_reader_read_header(
    e: &mut Engine,
    this: Ptr,
    stream: u32,
    width: u32,
    height: u32,
    format: u32,
    mipmapped: u32,
    faces: u32,
) -> bool {
    let t = this.addr();
    let section = t + 0x80;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let taken = e.with_stack(0x80, |e, frame| {
        // The frame (offsets of the exe's own locals): the swapped magic and
        // later the alpha mask (+0), the pitch, depth and format size (+8),
        // the magic (+0xc), the flags (+0x10), the format flags (+0x14), the
        // green mask (+0x18), the count word and later the red mask (+0x1c),
        // caps (+0x20), the header size (+0x24), the FourCC (+0x28), the bit
        // count (+0x2c), the blue mask (+0x30), caps 2 (+0x34), the mipmap
        // count (+0x38), a pixel format (+0x3c).
        let b = frame.addr();
        let count = b + 0x1c;
        read_raw(e, stream, b + 0xc, 4, count);
        let magic = e.mem.u32(b + 0xc);
        if magic != DDS_MAGIC {
            e.mem.set_u32(b, magic.swap_bytes());
            if e.mem.u32(b) != DDS_MAGIC {
                return false;
            }
            e.vcall(stream, 0x10, &args![1u32]);
        }
        read_raw(e, stream, b + 0x24, 4, count);
        if e.mem.u32(b + 0x24) != 0x7c {
            return false;
        }
        read_raw(e, stream, b + 0x10, 4, count);
        if e.mem.u32(b + 0x10) & 0x1007 != 0x1007 {
            return false;
        }
        read_raw(e, stream, t + 0x100, 4, count);
        read_raw(e, stream, t + 0x104, 4, count);
        read_raw(e, stream, b + 8, 4, count);
        read_raw(e, stream, b + 8, 4, count);
        if e.mem.u32(b + 0x10) & 0x0080_0000 != 0 && e.mem.u32(b + 8) != 0 {
            return false;
        }
        read_raw(e, stream, b + 0x38, 4, count);
        let origin = e.global::<u32>(SEEK_ORIGIN);
        e.vcall(stream, 0x14, &args![0x2cu32, origin]);
        read_raw(e, stream, b + 8, 4, count);
        read_raw(e, stream, b + 0x14, 4, count);
        read_raw(e, stream, b + 0x28, 4, count);
        let pixel_flags = e.mem.u8(b + 0x14);
        e.mem.set_u8(t + 0x10c, 0);
        if pixel_flags & 4 != 0 {
            let chosen = match e.mem.u32(b + 0x28) {
                0x3154_5844 => 0x011a_a498,
                0x3354_5844 => 0x011a_a4e0,
                0x3554_5844 => 0x011a_a528,
                0x6f => 0x011a_a6d8,
                0x70 => 0x011a_a648,
                0x71 => 0x011a_a5b8,
                0x72 => 0x011a_a600,
                0x73 => 0x011a_a690,
                0x74 => 0x011a_a570,
                _ => return false,
            };
            copy_format(e, t + 0x110, chosen);
            copy_format(e, format, t + 0x110);
            let origin = e.global::<u32>(SEEK_ORIGIN);
            e.vcall(stream, 0x14, &args![0x14u32, origin]);
        } else if pixel_flags & 0x40 != 0 {
            for at in [0x2cu32, 0x1c, 0x18, 0x30, 0] {
                e.call(STREAM_READ_WORD, &args![stream, b + at]);
            }
            let alpha = if e.mem.u8(b + 0x14) & 1 == 0 {
                e.mem.set_u32(b, 0);
                0
            } else {
                e.mem.u32(b)
            };
            let bits = e.mem.u32(b + 0x2c);
            let blue = e.mem.u32(b + 0x30);
            let green = e.mem.u32(b + 0x18);
            let red = e.mem.u32(b + 0x1c);
            if !e
                .call(
                    MASKS_SUPPORTED_00A8C100,
                    &args![this, red, green, blue, alpha, bits],
                )
                .bool()
            {
                return false;
            }
            let made = e
                .call(
                    FORMAT_FROM_MASKS,
                    &args![b + 0x3c, bits, red, green, blue, alpha],
                )
                .u32();
            copy_format(e, format, made);
            copy_format(e, t + 0x110, made);
            let mut words = vec![t, b + 0x3c];
            for word in 0..0x11u32 {
                words.push(e.mem.u32(format + 4 * word));
            }
            let chosen = e.call(CHOOSE_FORMAT_00A8C120, &words).u32();
            copy_format(e, format, chosen);
        } else {
            return false;
        }
        e.call(STREAM_READ_WORD, &args![stream, b + 0x20]);
        let caps = e.mem.u32(b + 0x20);
        if caps & 8 == 0 && caps & 0x0040_1000 == 0 {
            return false;
        }
        e.call(STREAM_READ_WORD, &args![stream, b + 0x34]);
        let caps_2 = e.mem.u32(b + 0x34);
        if caps_2 & 0x0020_0000 != 0 {
            return false;
        }
        let cube = caps_2 & 0x200;
        if cube != 0 {
            for face in [0x400u32, 0x800, 0x1000, 0x2000, 0x4000, 0x8000] {
                if caps_2 & face == 0 {
                    return false;
                }
            }
        }
        e.mem.set_u32(faces, if cube != 0 { 6 } else { 1 });
        let origin = e.global::<u32>(SEEK_ORIGIN);
        e.vcall(stream, 0x14, &args![0xcu32, origin]);
        let w = e.mem.u32(t + 0x104);
        e.mem.set_u32(width, w);
        let h = e.mem.u32(t + 0x100);
        e.mem.set_u32(height, h);
        let caps = e.mem.u32(b + 0x20);
        let mipmaps = e.mem.u32(b + 0x38);
        if caps & 8 != 0 && caps & 0x0040_0000 != 0 && mipmaps != 1 {
            e.mem.set_u8(mipmapped, 1);
            e.mem.set_u32(t + 0x108, mipmaps);
        } else {
            e.mem.set_u8(mipmapped, 0);
            e.mem.set_u32(t + 0x108, 1);
        }
        true
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    taken
}

/// `NiPixelFormat::operator==` (`00a76540`, `thiscall`, `RET 4`), the Gamebryo
/// allocation (`00aa13e0`, `cdecl`) and `NiPixelData::NiPixelData_ov2`
/// (`00a7c190`, `thiscall`, `RET 0x14`).
const FORMAT_EQUALS: u32 = 0x00a7_6540;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
/// The level converters by the file's bits per pixel (`cdecl`, 39 words: the
/// stream, the data, the file's format and the wanted format by value, the
/// level, the face and the byte at `+0x10c`): `NiDDSReader::Read16Bit`
/// (`00a8cc40`), `00a8c6e0` (24 bits) and `00a8c7a0` (32 bits).
const READ_16_BIT: u32 = 0x00a8_cc40;
const READ_24_BIT_00A8C6E0: u32 = 0x00a8_c6e0;
const READ_32_BIT_00A8C7A0: u32 = 0x00a8_c7a0;

/// The face index the reader's jump tables give for a counter that starts
/// at -1: 1..5 for 0..4, else 0.
fn face_of(counter: u32) -> u32 {
    match counter {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        _ => 0,
    }
}

// Translated from 00a8d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDDSReader::ReadFile` (engine map name; virtual, `thiscall`, `RET 8`: the
/// stream and an existing pixel data or 0). Under the critical section
/// (`+0x80`): the header (slot `+0xc`) into a local format, width, height,
/// mipmapped byte and face count; refused answers 0. The given data is used
/// when its size, format, level count (`+0x108`) and faces match; otherwise a
/// new `NiPixelData`. When the format equals the file's (`+0x110`) every
/// level of every face is read straight into place (count word 1);
/// otherwise each level goes through the converter for the file's bits per
/// pixel (`+0x111`: 16, 24 or 32; others are skipped) with both formats by
/// value. Answers the data.
pub fn ni_dds_reader_read_file(e: &mut Engine, this: Ptr, stream: u32, given: u32) -> u32 {
    let t = this.addr();
    e.with_stack(0x70, |e, frame| {
        // The frame: the mipmapped byte (+3), the faces (+8), the level count
        // (+0xc), the height (+0x18), the width (+0x1c), the format (+0x20),
        // the count word (+0x6c).
        let b = frame.addr();
        let format = b + 0x20;
        e.call(NI_PIXEL_FORMAT_CONSTRUCT, &args![format]);
        let section = t + 0x80;
        e.call(ENTER_CRITICAL_SECTION, &args![section]);
        if !e
            .vcall(
                t,
                0xc,
                &args![stream, b + 0x1c, b + 0x18, format, b + 3, b + 8],
            )
            .bool()
        {
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            return 0;
        }
        let faces = e.mem.u32(b + 8);
        let reuse = given != 0
            && e.mem.u32(e.mem.u32(given + 0x54)) == e.mem.u32(t + 0x104)
            && e.mem.u32(e.mem.u32(given + 0x58)) == e.mem.u32(t + 0x100)
            && e.call(FORMAT_EQUALS, &args![given + 8, format]).bool()
            && e.mem.u32(given + 0x60) == e.mem.u32(t + 0x108)
            && e.mem.u32(given + 0x6c) == faces;
        let data = if reuse {
            given
        } else {
            let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
            if block != 0 {
                let levels = e.mem.u32(t + 0x108);
                let h = e.mem.u32(t + 0x100);
                let w = e.mem.u32(t + 0x104);
                e.call(
                    NI_PIXEL_DATA_CONSTRUCT,
                    &args![block, w, h, format, levels, faces],
                )
                .u32()
            } else {
                0
            }
        };
        let levels = e.mem.u32(data + 0x60);
        e.mem.set_u32(b + 0xc, levels);
        let count_word = b + 0x6c;
        if e.call(FORMAT_EQUALS, &args![format, t + 0x110]).bool() {
            let mut counter = u32::MAX;
            if e.mem.u32(b + 8) != 0 {
                loop {
                    let face = face_of(counter);
                    let mut level = 0;
                    while level < e.mem.u32(b + 0xc) {
                        let offsets = e.mem.u32(data + 0x5c);
                        let all = e.mem.u32(data + 0x60);
                        let face_offset = e.mem.u32(offsets + 4 * all).wrapping_mul(face);
                        let start = e.mem.u32(offsets + 4 * level);
                        let size = e.mem.u32(offsets + 4 * (level + 1)).wrapping_sub(start);
                        let at = face_offset
                            .wrapping_add(start)
                            .wrapping_add(e.mem.u32(data + 0x50));
                        let read = e.mem.u32(stream + 8);
                        e.mem.set_u32(count_word, 1);
                        let done = e
                            .call(read, &args![stream, at, size, count_word, 1u32])
                            .u32();
                        let total = e.mem.u32(stream + 4).wrapping_add(done);
                        e.mem.set_u32(stream + 4, total);
                        level += 1;
                    }
                    counter = counter.wrapping_add(1);
                    if counter.wrapping_add(1) >= e.mem.u32(b + 8) {
                        break;
                    }
                }
            }
        } else if e.mem.u32(b + 8) != 0 {
            let mut counter = u32::MAX;
            loop {
                let face = face_of(counter);
                let mut level = 0;
                if levels != 0 {
                    while level < e.mem.u32(b + 0xc) {
                        let converter = match e.mem.u8(t + 0x111) {
                            0x10 => Some(READ_16_BIT),
                            0x18 => Some(READ_24_BIT_00A8C6E0),
                            0x20 => Some(READ_32_BIT_00A8C7A0),
                            _ => None,
                        };
                        if let Some(converter) = converter {
                            let mut words = vec![stream, data];
                            for word in 0..0x11u32 {
                                words.push(e.mem.u32(t + 0x110 + 4 * word));
                            }
                            for word in 0..0x11u32 {
                                words.push(e.mem.u32(format + 4 * word));
                            }
                            let flag = u32::from(e.mem.u8(t + 0x10c));
                            words.extend([level, face, flag]);
                            e.call(converter, &words);
                        }
                        level += 1;
                    }
                }
                counter = counter.wrapping_add(1);
                if counter.wrapping_add(1) >= e.mem.u32(b + 8) {
                    break;
                }
            }
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        data
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_c190, fn_00a8c190(Ptr) -> Ptr),
        entry!(0x00a8_c0e0, ni_dds_reader_can_read_file(Ptr, u32) -> bool),
        entry!(0x00a8_c1e0, ni_dds_reader_read_header(Ptr, u32, u32, u32, u32, u32, u32) -> bool),
        entry!(0x00a8_d0d0, ni_dds_reader_read_file(Ptr, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a8c190_builds_the_reader() {
        let mut e = Engine::new();
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |_, _| Ret::default());
        e.map(PIXEL_FORMAT_SOURCE_011AA498, 0x44);
        for w in 0..0x11 {
            e.mem
                .set_u32(PIXEL_FORMAT_SOURCE_011AA498 + 4 * w, 0x100 + w);
        }
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x100, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_c190, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_DDS_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        assert_eq!(e.mem.u32(this + 0x110), 0x100);
        assert_eq!(e.mem.u32(this + 0x150), 0x110);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])
        );
        assert_eq!(log[2], (NI_PIXEL_FORMAT_CONSTRUCT, vec![this + 0x110]));
    }

    #[test]
    fn ni_dds_reader_can_read_file_compares_the_extension() {
        let mut e = Engine::new();
        e.register(NI_STRICMP, |_, a| u32::from(a[0] != 0x10).into_ret());
        assert!(e.call(0x00a8_c0e0, &args![0u32, 0x10u32]).bool());
        assert!(!e.call(0x00a8_c0e0, &args![0u32, 0x11u32]).bool());
    }

    const READ: u32 = 0x0134_0000;
    const STREAM_SLOTS: u32 = 0x0134_1000;

    /// A stream whose binary read hands out `words` in turn (each read writes
    /// one word, or its low bytes for a smaller size).
    fn stream(e: &mut Engine, words: Vec<u32>) -> u32 {
        let mut words = words.into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let v = words.next().expect("no more header words");
            e.mem.set_u32(a[1], v);
            a[2].into_ret()
        });
        let table = e.mem.alloc(0x20);
        for slot in (0..0x20).step_by(4) {
            e.mem.set_u32(table + slot, STREAM_SLOTS + slot);
            e.register(STREAM_SLOTS + slot, |_, _| Ret::default());
        }
        let s = e.mem.alloc(0x250);
        e.mem.set_u32(s, table);
        e.mem.set_u32(s + 8, READ);
        e.register(STREAM_READ_WORD, |e, a| {
            let read = e.mem.u32(a[0] + 8);
            e.mem.set_u32(0x0134_2000, 4);
            e.call(read, &[a[0], a[1], 4, 0x0134_2000, 1]);
            Ret::default()
        });
        e.map(0x0134_2000, 4);
        s
    }

    fn header_engine() -> Engine {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.map(SEEK_ORIGIN, 4);
        e.set_global::<u32>(SEEK_ORIGIN, 1);
        e.map(0x011a_a498, 0x44);
        e.mem.set_u32(0x011a_a498, 0xd1);
        e
    }

    /// The header words up to the format's FourCC: magic, size, flags,
    /// height, width, pitch, depth, mipmaps, then (after the skip) the
    /// format's size, flags and FourCC.
    fn header(flags: u32, mipmaps: u32, pixel_flags: u32, four_cc: u32) -> Vec<u32> {
        vec![
            DDS_MAGIC,
            0x7c,
            flags,
            64,
            32,
            0,
            0,
            mipmaps,
            32,
            pixel_flags,
            four_cc,
        ]
    }

    #[test]
    fn ni_dds_reader_read_header_takes_a_dxt1_mipmapped_file() {
        let mut e = header_engine();
        let mut words = header(0x1007, 6, 4, 0x3154_5844);
        words.extend([0x0040_1008, 0]);
        let s = stream(&mut e, words);
        let this = e.mem.alloc(0x180);
        let out = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        let taken = e
            .call(
                0x00a8_c1e0,
                &args![this, s, out, out + 4, out + 8, out + 0x50, out + 0x54],
            )
            .bool();
        assert!(taken);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (32, 64));
        assert_eq!(e.mem.u32(out + 8), 0xd1);
        assert_eq!(e.mem.u32(this + 0x110), 0xd1);
        assert_eq!(e.mem.u8(out + 0x50), 1);
        assert_eq!(e.mem.u32(out + 0x54), 1);
        assert_eq!(e.mem.u32(this + 0x108), 6);
        let log = e.call_log.take().unwrap();
        let skips: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == STREAM_SLOTS + 0x14)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(skips, vec![0x2c, 0x14, 0xc]);
        assert_eq!(
            log.last().unwrap(),
            &(LEAVE_CRITICAL_SECTION, vec![this + 0x80])
        );
    }

    #[test]
    fn ni_dds_reader_read_header_refuses_bad_headers() {
        let cases: Vec<Vec<u32>> = vec![
            vec![0x1234_5678],
            vec![DDS_MAGIC, 0x7b],
            vec![DDS_MAGIC, 0x7c, 0x1006],
            header(0x1007, 1, 4, 0x3254_5844),
            header(0x1007, 1, 0, 0),
        ];
        for words in cases {
            let mut e = header_engine();
            let s = stream(&mut e, words);
            let this = e.mem.alloc(0x180);
            let out = e.mem.alloc(0x60);
            let taken = e
                .call(
                    0x00a8_c1e0,
                    &args![this, s, out, out + 4, out + 8, out + 0x50, out + 0x54],
                )
                .bool();
            assert!(!taken);
        }
    }

    #[test]
    fn ni_dds_reader_read_header_cube_maps_and_swapped_magic() {
        let mut e = header_engine();
        let mut words = header(0x1007, 1, 4, 0x3154_5844);
        words[0] = DDS_MAGIC.swap_bytes();
        words.extend([8, 0xfe00]);
        let s = stream(&mut e, words);
        let this = e.mem.alloc(0x180);
        let out = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a8_c1e0,
                &args![this, s, out, out + 4, out + 8, out + 0x50, out + 0x54]
            )
            .bool());
        assert_eq!(e.mem.u32(out + 0x54), 6);
        assert_eq!(e.mem.u8(out + 0x50), 0);
        assert_eq!(e.mem.u32(this + 0x108), 1);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(STREAM_SLOTS + 0x10, vec![s, 1])));
        // A cube map missing a face is refused.
        let mut e = header_engine();
        let mut words = header(0x1007, 1, 4, 0x3154_5844);
        words.extend([8, 0x7e00]);
        let s = stream(&mut e, words);
        let this = e.mem.alloc(0x180);
        let out = e.mem.alloc(0x60);
        assert!(!e
            .call(
                0x00a8_c1e0,
                &args![this, s, out, out + 4, out + 8, out + 0x50, out + 0x54]
            )
            .bool());
    }

    #[test]
    fn ni_dds_reader_read_header_rgb_masks() {
        let mut e = header_engine();
        e.register(MASKS_SUPPORTED_00A8C100, |_, _| 1u32.into_ret());
        e.register(FORMAT_FROM_MASKS, |e, a| {
            e.mem.set_u32(a[0], 0xf0);
            a[0].into_ret()
        });
        let chosen = e.mem.alloc(0x44);
        e.mem.set_u32(chosen, 0xc0);
        e.register_double(CHOOSE_FORMAT_00A8C120, move |_, a| {
            assert_eq!(a.len(), 2 + 0x11);
            assert_eq!(a[2], 0xf0);
            chosen.into_ret()
        });
        let mut words = header(0x1007, 1, 0x40, 0);
        words.extend([32, 0xff0000, 0xff00, 0xff, 0xff00_0000, 0x1000, 0]);
        let s = stream(&mut e, words);
        let this = e.mem.alloc(0x180);
        let out = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x00a8_c1e0,
                &args![this, s, out, out + 4, out + 8, out + 0x50, out + 0x54]
            )
            .bool());
        assert_eq!(e.mem.u32(out + 8), 0xc0);
        assert_eq!(e.mem.u32(this + 0x110), 0xf0);
        let log = e.call_log.take().unwrap();
        let masks: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MASKS_SUPPORTED_00A8C100)
            .collect();
        // No alpha flag: the alpha mask is 0.
        assert_eq!(
            masks[0].1[1..].to_vec(),
            vec![0xff0000, 0xff00, 0xff, 0, 32]
        );
    }

    #[test]
    fn face_of_follows_the_jump_tables() {
        assert_eq!(
            [u32::MAX, 0, 1, 2, 3, 4, 5].map(face_of),
            [0, 1, 2, 3, 4, 5, 0]
        );
    }

    const READER_SLOTS: u32 = 0x0135_0000;

    /// A reader whose header slot (+0xc) answers `ok` and fills one face of
    /// `levels` levels of a 4x4 image.
    fn reader(e: &mut Engine, ok: u32, faces: u32) -> u32 {
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, READER_SLOTS);
        e.register_double(READER_SLOTS, move |e, a| {
            e.mem.set_u32(a[2], 4);
            e.mem.set_u32(a[3], 4);
            e.mem.set_u32(a[6], faces);
            ok.into_ret()
        });
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 0x100, 4);
        e.mem.set_u32(this + 0x104, 4);
        e.mem.set_u32(this + 0x108, 2);
        this
    }

    fn data(e: &mut Engine) -> u32 {
        let d = e.mem.alloc(0x74);
        let offsets = e.mem.alloc(12);
        for (i, v) in [0u32, 16, 20].iter().enumerate() {
            e.mem.set_u32(offsets + 4 * i as u32, *v);
        }
        e.mem.set_u32(d + 0x5c, offsets);
        e.mem.set_u32(d + 0x60, 2);
        e.mem.set_u32(d + 0x50, 0x9000);
        d
    }

    #[test]
    fn ni_dds_reader_read_file_reads_levels_straight() {
        let mut e = Engine::new();
        for f in [
            NI_PIXEL_FORMAT_CONSTRUCT,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(FORMAT_EQUALS, |_, _| 1u32.into_ret());
        let d = data(&mut e);
        e.register_double(NI_ALLOCATE, move |_, _| d.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(READ, |_, a| a[2].into_ret());
        let this = reader(&mut e, 1, 2);
        let s = e.mem.alloc(0x10);
        e.mem.set_u32(s + 8, READ);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_d0d0, &args![this, s, 0u32]).u32(), d);
        let reads: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == READ)
            .map(|c| vec![c.1[1], c.1[2]])
            .collect();
        assert_eq!(
            reads,
            vec![
                vec![0x9000, 16],
                vec![0x9010, 4],
                vec![0x9014, 16],
                vec![0x9024, 4]
            ]
        );
        assert_eq!(e.mem.u32(s + 4), 40);
    }

    #[test]
    fn ni_dds_reader_read_file_converts_and_refuses() {
        let mut e = Engine::new();
        for f in [
            NI_PIXEL_FORMAT_CONSTRUCT,
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            READ_32_BIT_00A8C7A0,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(FORMAT_EQUALS, |_, _| Ret::default());
        let d = data(&mut e);
        e.register_double(NI_ALLOCATE, move |_, _| d.into_ret());
        e.register(NI_PIXEL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let this = reader(&mut e, 1, 1);
        e.mem.set_u8(this + 0x111, 0x20);
        e.mem.set_u8(this + 0x10c, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_d0d0, &args![this, 0x500u32, 0u32]).u32(), d);
        let converts: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == READ_32_BIT_00A8C7A0)
            .map(|c| c.1)
            .collect();
        assert_eq!(converts.len(), 2);
        assert_eq!(converts[1].len(), 39);
        assert_eq!(converts[1][..2].to_vec(), vec![0x500, d]);
        assert_eq!(converts[1][36..].to_vec(), vec![1, 0, 1]);
        let this = reader(&mut e, 0, 1);
        assert_eq!(e.call(0x00a8_d0d0, &args![this, 0x500u32, 0u32]).u32(), 0);
    }
}

#[cfg(test)]
mod b000t_tests {
    use super::*;

    #[test]
    fn fn_00a8c190_builds_the_dds_reader() {
        let mut e = Engine::new();
        e.map(PIXEL_FORMAT_SOURCE_011AA498, 4 * PIXEL_FORMAT_WORDS);
        for word in 0..PIXEL_FORMAT_WORDS {
            e.mem
                .set_u32(PIXEL_FORMAT_SOURCE_011AA498 + 4 * word, 0x100 + word);
        }
        e.register(IMPORT_INITIALIZE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(NI_PIXEL_FORMAT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x160);
        e.mem.set_u32(this + 0x100, 7);
        assert_eq!(e.call(0x00a8_c190, &args![this]).u32(), this);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(IMPORT_INITIALIZE_CRITICAL_SECTION, vec![this + 0x80])));
        assert!(log.contains(&(NI_PIXEL_FORMAT_CONSTRUCT, vec![this + PIXEL_FORMAT])));
        assert_eq!(e.mem.u32(this), NI_DDS_READER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x100), 0);
        for word in 0..PIXEL_FORMAT_WORDS {
            assert_eq!(e.mem.u32(this + PIXEL_FORMAT + 4 * word), 0x100 + word);
        }
    }
}
