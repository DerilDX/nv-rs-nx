//! `FaceGen/trifileio.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The FaceGen SDK's `.tri` reader, compiled with the debug `std::string`
//! of the SDK's runtime (size at +0x14, capacity at +0x18, the text inline
//! at +4 below 16 characters).

#[allow(unused_imports)]
use crate::prelude::*;

/// `std::string::erase` (`0044a690`: `this`; position, count) and the
/// append of `count` copies of a character (`00aadeb0`: `this`; count,
/// character).
const STRING_ERASE: u32 = 0x0044_a690;
const STRING_APPEND_CHARS: u32 = 0x00aa_deb0;
/// `std::string::npos` (the exe's data).
const NPOS: u32 = 0x0101_73f0;

// Translated from 00ab46e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A method that does nothing (`this` only).
pub fn fn_00ab46e0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00aba370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::resize` (`this`; size, character): grows by appending
/// `size - length` copies of `fill`, or erases from `size` to the end.
pub fn fn_00aba370(e: &mut Engine, this: Ptr, size: u32, fill: u8) {
    let length = e.mem.u32(this.addr().wrapping_add(0x14));
    if size <= length {
        let npos = e.global::<u32>(NPOS);
        e.call(STRING_ERASE, &args![this, size, npos]);
    } else {
        let length = e.mem.u32(this.addr().wrapping_add(0x14));
        e.call(
            STRING_APPEND_CHARS,
            &args![this, size.wrapping_sub(length), u32::from(fill)],
        );
    }
}

/// `_memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// `std::string` construction from text (`0044a4c0`: `this`; text), the
/// empty construction / release (`0044a730`: `this`; two words), assign
/// (`0044a580`: `this`; other, position, count) and copy construction
/// (`0044a500`: `this`; other).
const STRING_FROM_TEXT: u32 = 0x0044_a4c0;
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_COPY: u32 = 0x0044_a500;
/// `"FRTRI003"`, the file's magic, and `""`.
const TEXT_MAGIC: u32 = 0x010a_2d54;
const TEXT_EMPTY: u32 = 0x0101_1584;
/// `BSFaceGenBinaryFile` (Xbox PDB names): constructor (`00ac5490`:
/// `this`; magic string by value), `openForRead` (`00ac5540`: `this`; name
/// string by value, 0), `readHeader` (`00ac5e00`: `this`; buffer, 1,
/// size), `read` (`00ac5cf0`: `this`; buffer, size, count), the close and
/// destructor bodies of the failure paths (`00ac5c00`, `00aaaa80`, after
/// the vtable `010a2c38` is put back) and the destructor of the later
/// paths (`00aaab30`).
const FILE_CONSTRUCT: u32 = 0x00ac_5490;
const FILE_OPEN_FOR_READ: u32 = 0x00ac_5540;
const FILE_READ_HEADER: u32 = 0x00ac_5e00;
const FILE_READ: u32 = 0x00ac_5cf0;
const FILE_CLOSE: u32 = 0x00ac_5c00;
const FILE_DESTRUCT_BASE: u32 = 0x00aa_aa80;
const FILE_DESTRUCT: u32 = 0x00aa_ab30;
const FILE_VTABLE: u32 = 0x010a_2c38;
/// The mesh's vector resizes: `NiPoint3`s (`00aba7b0`: vector; count,
/// three words), 16-byte rows (`00abaaa0`: vector; count, four words),
/// 32-byte named entries (`00ab9eb0`: vector; count), 0x2C-byte entries
/// (`00abb3c0`: vector; count, eleven words, from `00aba220`), 0x34-byte
/// entries (`00abb700`: vector; count, thirteen words, from `00aba270`),
/// 0x38-byte entries (`00abbae0`: vector; count, fourteen words, from
/// `00aba2f0`), two-float rows (`00abad80`: vector; count, two words) and
/// words (`00aba5d0`: vector; count, value); the clears of +0x60 and
/// +0x78 (`00ab9cd0`, `00ab9dc0`).
const RESIZE_POINTS: u32 = 0x00ab_a7b0;
const RESIZE_ROWS: u32 = 0x00ab_aaa0;
const RESIZE_NAMED: u32 = 0x00ab_9eb0;
const RESIZE_2C: u32 = 0x00ab_b3c0;
const RESIZE_34: u32 = 0x00ab_b700;
const RESIZE_38: u32 = 0x00ab_bae0;
const RESIZE_PAIRS: u32 = 0x00ab_ad80;
const RESIZE_WORDS: u32 = 0x00ab_a5d0;
const MAKE_2C: u32 = 0x00ab_a220;
const MAKE_34: u32 = 0x00ab_a270;
const MAKE_38: u32 = 0x00ab_a2f0;
const CLEAR_60: u32 = 0x00ab_9cd0;
const CLEAR_78: u32 = 0x00ab_9dc0;
/// A word vector's element address (`006578b0`: vector; index).
const WORD_AT: u32 = 0x0065_78b0;
/// The name buffer, a `std::vector<char>`: constructor (`00aba0d0`:
/// `this`; size) and its two destructor steps (`00abc0a0`, `00abc120`).
const BUFFER_CONSTRUCT: u32 = 0x00ab_a0d0;
const BUFFER_RELEASE: u32 = 0x00ab_c0a0;
const BUFFER_DESTRUCT: u32 = 0x00ab_c120;
/// The CRT's invalid-index report (`00ec7c56`).
const INVALID_INDEX: u32 = 0x00ec_7c56;
/// `futSetLastError` (Xbox PDB name of `00ac6430`, cdecl: code, string
/// by value).
const SET_LAST_ERROR: u32 = 0x00ac_6430;

/// The by-value words of the object of `size` bytes at `at`.
fn words_of(e: &Engine, at: Ptr, size: u32) -> Vec<u32> {
    (0..size / 4)
        .map(|i| e.mem.u32(at.addr() + i * 4))
        .collect()
}

/// `(last - first) / size` of a vector (first at +0xC, last at +0x10),
/// signed division (`CDQ; IDIV`).
fn count_div(e: &Engine, vector: u32, size: u32) -> u32 {
    let first = e.mem.u32(vector.wrapping_add(0xc));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    ((last.wrapping_sub(first) as i32).wrapping_div(size as i32)) as u32
}

/// `(last - first) >> shift` of a vector (`SAR`).
fn count_sar(e: &Engine, vector: u32, shift: u32) -> u32 {
    let first = e.mem.u32(vector.wrapping_add(0xc));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    ((last.wrapping_sub(first) as i32) >> shift) as u32
}

/// The element `index` of a vector, after the exe's bounds check
/// (`index < count`, reported to the CRT otherwise).
fn element(e: &mut Engine, vector: u32, index: u32, count: u32, size: u32) -> u32 {
    if index >= count {
        e.call(INVALID_INDEX, &[]);
    }
    index
        .wrapping_mul(size)
        .wrapping_add(e.mem.u32(vector.wrapping_add(0xc)))
}

/// The first element of a vector, after the exe's non-empty check.
fn first_element(e: &mut Engine, vector: u32, count: u32) -> u32 {
    if count == 0 {
        e.call(INVALID_INDEX, &[]);
    }
    e.mem.u32(vector.wrapping_add(0xc))
}

fn read(e: &mut Engine, file: Ptr, buffer: u32, size: u32, count: u32) -> bool {
    e.call(FILE_READ, &args![file, buffer, size, count]).bool()
}

/// The early failure path: the file's vtable put back, closed and
/// destroyed; false.
fn give_up(e: &mut Engine, file: Ptr) -> bool {
    e.mem.set_u32(file.addr(), FILE_VTABLE);
    e.call(FILE_CLOSE, &args![file]);
    e.call(FILE_DESTRUCT_BASE, &args![file]);
    false
}

fn release_buffer(e: &mut Engine, buffer: Ptr) {
    e.call(BUFFER_RELEASE, &args![buffer]);
    e.call(BUFFER_DESTRUCT, &args![buffer]);
}

/// Where a name goes: the string of entry `index` of the vector at
/// `mesh + vector`, `string` bytes into the entry, the entry's size and
/// how the exe computes the count.
#[derive(Clone, Copy)]
struct NameSlot {
    vector: u32,
    size: u32,
    shift: Option<u32>,
    string: u32,
}

impl NameSlot {
    fn address(self, e: &mut Engine, mesh: u32, index: u32) -> u32 {
        let vector = mesh.wrapping_add(self.vector);
        let count = match self.shift {
            Some(shift) => count_sar(e, vector, shift),
            None => count_div(e, vector, self.size),
        };
        element(e, vector, index, count, self.size).wrapping_add(self.string)
    }
}

/// Reads a name of `length` bytes into the slot's string: through the
/// buffer `buffer`, resized to `length - 1`; `copy_each` copies every
/// character (the morph loops), otherwise only the one at `length - 1`
/// after an empty counting loop (the loops of named entries and modifiers,
/// as compiled). A length of 1 assigns `""` through `temp`. Returns false
/// after the failure path when the read fails.
#[allow(clippy::too_many_arguments)]
fn read_name(
    e: &mut Engine,
    file: Ptr,
    mesh: u32,
    slot: NameSlot,
    index: u32,
    length: u32,
    buffer: Ptr,
    temp: Ptr,
    copy_each: bool,
) -> bool {
    e.call(BUFFER_CONSTRUCT, &args![buffer, length]);
    let buffer_first = |e: &Engine| e.mem.u32(buffer.addr() + 0xc);
    let buffer_size = |e: &Engine| {
        e.mem
            .u32(buffer.addr() + 0x10)
            .wrapping_sub(e.mem.u32(buffer.addr() + 0xc))
    };
    if buffer_size(e) == 0 {
        e.call(INVALID_INDEX, &[]);
    }
    let data = buffer_first(e);
    if !read(e, file, data, 1, length) {
        release_buffer(e, buffer);
        return give_up(e, file);
    }
    if length > 1 {
        let string = slot.address(e, mesh, index);
        fn_00aba370(e, Ptr::new(string), length - 1, 0);
        let copy = |e: &mut Engine, k: u32| {
            if k >= buffer_size(e) {
                e.call(INVALID_INDEX, &[]);
            }
            let source = buffer_first(e).wrapping_add(k);
            let string = slot.address(e, mesh, index);
            if k > e.mem.u32(string.wrapping_add(0x14)) {
                e.call(INVALID_INDEX, &[]);
            }
            let text = if e.mem.u32(string.wrapping_add(0x18)) >= 0x10 {
                e.mem.u32(string.wrapping_add(4))
            } else {
                string.wrapping_add(4)
            };
            let c = e.mem.u8(source);
            e.mem.set_u8(text.wrapping_add(k), c);
        };
        let mut k = 0u32;
        if copy_each {
            while k < length - 1 {
                copy(e, k);
                k += 1;
            }
        } else {
            while k < length - 1 {
                k += 1;
            }
            copy(e, k);
        }
    } else {
        let empty = e.call(STRING_FROM_TEXT, &args![temp, TEXT_EMPTY]).u32();
        let string = slot.address(e, mesh, index);
        let npos = e.global::<u32>(NPOS);
        e.call(STRING_ASSIGN, &args![string, empty, 0u32, npos]);
        e.call(STRING_TIDY, &args![temp, 1u32, 0u32]);
    }
    release_buffer(e, buffer);
    true
}

// Translated from 00ab5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fffReadTriFile` (Xbox PDB, cdecl: file name `std::string *`, mesh):
/// reads a FaceGen `.tri` file (`"FRTRI003"`) into the mesh: the header
/// (0x38 bytes: vertex, triangle, quad, named-entry, modifier, extra-UV
/// and flag counts...), then the vertices (+0x0, also room for the
/// modifier vertices), triangles (+0x18), quads (+0x30), the named
/// entries (+0x90: a word and a name each), the modifier entries (+0xA8:
/// a word, a point and a name), the UV data when header flag 1 is set
/// (+0x48, +0x60, +0x78), the differential morphs (+0xC0: name, scale and
/// `short` offsets scaled into points) and the statistical morphs (+0xD8:
/// name, first vertex index, vertex indices). Every failed read closes
/// the file and returns false; a vertex count that does not add up sets
/// the last error (`futSetLastError(1, name)`) and returns false.
// Translation incomplete: 00ab5ec0 - SEH exception frame (FS:[0]) and the /GS stack cookie check (00ec408c) are not modelled.
pub fn fff_read_tri_file(e: &mut Engine, name: Ptr, mesh: u32) -> bool {
    e.with_stack(0x4d0, |e, frame| {
        let ebp = frame.addr() + 0x4d0;
        let local = |offset: u32| Ptr::<()>::new(ebp - offset);
        let header = local(0x48);
        let file = local(0x94);
        let h = |e: &Engine, offset: u32| e.mem.u32(header.addr() + offset);
        e.call(MEMSET, &args![header, 0u32, 0x38u32]);
        let magic = e.with_stack(0x1c, |e, text| {
            e.call(STRING_FROM_TEXT, &args![text, TEXT_MAGIC]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(magic);
        e.call(FILE_CONSTRUCT, &words);
        let path = e.with_stack(0x1c, |e, text| {
            e.call(STRING_TIDY, &args![text, 0u32, 0u32]);
            let npos = e.global::<u32>(NPOS);
            e.call(STRING_ASSIGN, &args![text, name, 0u32, npos]);
            words_of(e, text, 0x1c)
        });
        let mut words = vec![file.addr()];
        words.extend(path);
        words.push(0);
        if !e.call(FILE_OPEN_FOR_READ, &words).bool() {
            return give_up(e, file);
        }
        if !e
            .call(FILE_READ_HEADER, &args![file, header, 1u32, 0x38u32])
            .bool()
        {
            return give_up(e, file);
        }
        fn_00ab46e0(e, header);
        let vertices = h(e, 0).wrapping_add(h(e, 0x24));
        e.call(
            RESIZE_POINTS,
            &args![
                mesh,
                vertices,
                0.0f32.to_bits(),
                0.0f32.to_bits(),
                0.0f32.to_bits()
            ],
        );
        e.call(
            RESIZE_POINTS,
            &args![mesh + 0x18, h(e, 4), 0u32, 0u32, 0u32],
        );
        e.call(
            RESIZE_ROWS,
            &args![mesh + 0x30, h(e, 8), 0u32, 0u32, 0u32, 0u32],
        );
        e.call(RESIZE_NAMED, &args![mesh + 0x90, h(e, 0xc)]);
        for (resize, make, size, count, vector) in [
            (RESIZE_2C, MAKE_2C, 0x2cu32, 0x10u32, 0xa8u32),
            (RESIZE_34, MAKE_34, 0x34, 0x1c, 0xc0),
            (RESIZE_38, MAKE_38, 0x38, 0x20, 0xd8),
        ] {
            let n = h(e, count);
            let value = e.with_stack(size, |e, value| {
                e.call(make, &args![value]);
                words_of(e, value, size)
            });
            let mut words = vec![mesh + vector, n];
            words.extend(value);
            e.call(resize, &words);
        }
        if h(e, 0).wrapping_add(h(e, 0x24)) != 0 {
            let count = count_div(e, mesh, 0xc);
            let first = first_element(e, mesh, count);
            let n = h(e, 0).wrapping_add(h(e, 0x24));
            if !read(e, file, first, 0xc, n) {
                return give_up(e, file);
            }
        }
        if h(e, 4) != 0 {
            let vector = mesh + 0x18;
            let count = count_div(e, vector, 0xc);
            let first = first_element(e, vector, count);
            if !read(e, file, first, 0xc, h(e, 4)) {
                return give_up(e, file);
            }
        }
        if h(e, 8) != 0 {
            let vector = mesh + 0x30;
            let count = count_sar(e, vector, 4);
            let first = first_element(e, vector, count);
            if !read(e, file, first, 0x10, h(e, 8)) {
                return give_up(e, file);
            }
        }
        // The named entries (+0x90, 32 bytes: a word, then the name).
        let named = NameSlot {
            vector: 0x90,
            size: 0x20,
            shift: Some(5),
            string: 4,
        };
        let mut i = 0u32;
        while i < h(e, 0xc) {
            let entry = named.address(e, mesh, i).wrapping_sub(4);
            if !read(e, file, entry, 4, 1) {
                return give_up(e, file);
            }
            let length = local(0xa0);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    named,
                    i,
                    length,
                    local(0xb8),
                    local(0x158),
                    false,
                )
            {
                return false;
            }
            i += 1;
        }
        // The modifier entries (+0xA8, 0x2C bytes: a word, a point, the name).
        let modifiers = NameSlot {
            vector: 0xa8,
            size: 0x2c,
            shift: None,
            string: 0x10,
        };
        let mut i = 0u32;
        while i < h(e, 0x10) {
            let entry = modifiers.address(e, mesh, i).wrapping_sub(0x10);
            if !read(e, file, entry, 4, 1) {
                return give_up(e, file);
            }
            let entry = modifiers.address(e, mesh, i).wrapping_sub(0x10);
            if !read(e, file, entry.wrapping_add(4), 0xc, 1) {
                return give_up(e, file);
            }
            let length = local(0xc0);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    modifiers,
                    i,
                    length,
                    local(0xd8),
                    local(0x178),
                    false,
                )
            {
                return false;
            }
            i += 1;
        }
        if h(e, 0x18) & 1 != 0 {
            let zero = 0.0f32.to_bits();
            if h(e, 0x14) == 0 {
                e.call(RESIZE_PAIRS, &args![mesh + 0x48, h(e, 0), zero, zero]);
                if h(e, 0) != 0 {
                    let vector = mesh + 0x48;
                    let count = count_sar(e, vector, 3);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 8, h(e, 0)) {
                        return give_up(e, file);
                    }
                }
                e.call(CLEAR_60, &args![mesh + 0x60]);
                e.call(CLEAR_78, &args![mesh + 0x78]);
            } else {
                e.call(RESIZE_PAIRS, &args![mesh + 0x48, h(e, 0x14), zero, zero]);
                if h(e, 0x14) != 0 {
                    let vector = mesh + 0x48;
                    let count = count_sar(e, vector, 3);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 8, h(e, 0x14)) {
                        return give_up(e, file);
                    }
                }
                e.call(
                    RESIZE_POINTS,
                    &args![mesh + 0x60, h(e, 4), 0u32, 0u32, 0u32],
                );
                if h(e, 4) != 0 {
                    let vector = mesh + 0x60;
                    let count = count_div(e, vector, 0xc);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 0xc, h(e, 4)) {
                        return give_up(e, file);
                    }
                }
                e.call(
                    RESIZE_ROWS,
                    &args![mesh + 0x78, h(e, 8), 0u32, 0u32, 0u32, 0u32],
                );
                if h(e, 8) != 0 {
                    let vector = mesh + 0x78;
                    let count = count_sar(e, vector, 4);
                    let first = first_element(e, vector, count);
                    if !read(e, file, first, 0x10, h(e, 8)) {
                        return give_up(e, file);
                    }
                }
            }
        }
        // The differential morphs (+0xC0, 0x34 bytes: name, then points at +0x1C).
        let differential = NameSlot {
            vector: 0xc0,
            size: 0x34,
            shift: None,
            string: 0,
        };
        let mut i = 0u32;
        while i < h(e, 0x1c) {
            let length = local(0xe4);
            if !read(e, file, length.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(length.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    differential,
                    i,
                    length,
                    local(0xfc),
                    local(0x19c),
                    true,
                )
            {
                return false;
            }
            let scale = local(0xe0);
            if !read(e, file, scale.addr(), 4, 1) {
                return give_up(e, file);
            }
            let morph = differential.address(e, mesh, i);
            let zero = 0.0f32.to_bits();
            e.call(
                RESIZE_POINTS,
                &args![morph + 0x1c, h(e, 0), zero, zero, zero],
            );
            let offsets = local(0x10c);
            let mut j = 0u32;
            while j < h(e, 0) {
                if !read(e, file, offsets.addr(), 2, 3) {
                    return give_up(e, file);
                }
                let scale_bits = e.mem.u32(scale.addr());
                let scaled = |e: &Engine, at: u32| {
                    let value = e.mem.i16(offsets.addr() + at) as i32;
                    x87_store32(x87_mul(x87_m32(scale_bits), f64::from(value)))
                };
                let z = scaled(e, 4);
                let y = scaled(e, 2);
                let x = scaled(e, 0);
                let (x, y, z) = (x87_quiet_bits(x), x87_quiet_bits(y), x87_quiet_bits(z));
                let morph = differential.address(e, mesh, i);
                let points = morph.wrapping_add(0x1c);
                let count = count_div(e, points, 0xc);
                let point = element(e, points, j, count, 0xc);
                e.mem.set_u32(point, x);
                e.mem.set_u32(point + 4, y);
                e.mem.set_u32(point + 8, z);
                j += 1;
            }
            i += 1;
        }
        // The statistical morphs (+0xD8, 0x38 bytes: name, first vertex at
        // +0x1C, vertex indices at +0x20).
        let statistical = NameSlot {
            vector: 0xd8,
            size: 0x38,
            shift: None,
            string: 0,
        };
        let mut offset = h(e, 0);
        let mut i = 0u32;
        while i < h(e, 0x20) {
            let count_slot = local(0x110);
            if !read(e, file, count_slot.addr(), 4, 1) {
                return give_up(e, file);
            }
            let length = e.mem.u32(count_slot.addr());
            if length != 0
                && !read_name(
                    e,
                    file,
                    mesh,
                    statistical,
                    i,
                    length,
                    local(0x128),
                    local(0x1cc),
                    true,
                )
            {
                return false;
            }
            let morph = statistical.address(e, mesh, i);
            e.mem.set_u32(morph.wrapping_add(0x1c), offset);
            if !read(e, file, count_slot.addr(), 4, 1) {
                return give_up(e, file);
            }
            let morph = statistical.address(e, mesh, i);
            let n = e.mem.u32(count_slot.addr());
            e.call(RESIZE_WORDS, &args![morph + 0x20, n, 0u32]);
            if e.mem.u32(count_slot.addr()) != 0 {
                let morph = statistical.address(e, mesh, i);
                let indices = e
                    .call(WORD_AT, &args![morph.wrapping_add(0x20), 0u32])
                    .u32();
                let n = e.mem.u32(count_slot.addr());
                if !read(e, file, indices, 4, n) {
                    e.call(FILE_DESTRUCT, &args![file]);
                    return false;
                }
            }
            offset = offset.wrapping_add(e.mem.u32(count_slot.addr()));
            i += 1;
        }
        if offset != h(e, 0).wrapping_add(h(e, 0x24)) {
            let copy = e.with_stack(0x1c, |e, text| {
                e.call(STRING_COPY, &args![text, name]);
                words_of(e, text, 0x1c)
            });
            let mut words = vec![1u32];
            words.extend(copy);
            e.call(SET_LAST_ERROR, &words);
            e.call(FILE_DESTRUCT, &args![file]);
            return false;
        }
        e.call(FILE_DESTRUCT, &args![file]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_46e0, fn_00ab46e0(Ptr)),
        entry!(0x00ab_a370, fn_00aba370(Ptr, u32, u8)),
        entry!(0x00ab_5ec0, fff_read_tri_file(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
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
    fn nothing_happens() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00ab_46e0, &args![0x1234u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn resize_grows_or_erases() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(NPOS, 0xffff_ffffu32);
        e.register(STRING_ERASE, |_, _| Ret::default());
        e.register(STRING_APPEND_CHARS, |_, _| Ret::default());
        let s = e.mem.alloc(0x1c);
        e.mem.set_u32(s + 0x14, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ab_a370, &args![s, 8u32, 0x120u32]);
        e.call(0x00ab_a370, &args![s, 5u32, 0x20u32]);
        e.call(0x00ab_a370, &args![s, 2u32, 0x20u32]);
        assert_eq!(calls_to(&e, STRING_APPEND_CHARS), vec![vec![s, 3, 0x20]]);
        assert_eq!(
            calls_to(&e, STRING_ERASE),
            vec![vec![s, 5, 0xffff_ffff], vec![s, 2, 0xffff_ffff]]
        );
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Makes `vector` (first +0xC, last +0x10) hold `count` zeroed elements
    /// of `size` bytes.
    fn grow(e: &mut Engine, vector: u32, count: u32, size: u32) {
        let first = e.mem.alloc(count * size + 4);
        e.mem.set_u32(vector + 0xc, first);
        e.mem.set_u32(vector + 0x10, first + count * size);
    }

    /// Doubles for the file (reading `stream` in order), the vectors and
    /// strings; the header is `header` (0x38 bytes).
    fn tri_engine(header: [u32; 14], stream: Vec<u8>, opens: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(NPOS, 0xffff_ffffu32);
        let stream = Rc::new(RefCell::new((stream, 0usize)));
        e.register_double(FILE_READ, move |e, a| {
            let (bytes, at) = &mut *stream.borrow_mut();
            let n = (a[2] * a[3]) as usize;
            let chunk = bytes[*at..*at + n].to_vec();
            *at += n;
            e.mem.write(a[1], &chunk);
            ret(1)
        });
        e.register_double(FILE_READ_HEADER, move |e, a| {
            for (i, word) in header.iter().enumerate() {
                e.mem.set_u32(a[1] + i as u32 * 4, *word);
            }
            ret(1)
        });
        e.register_double(FILE_OPEN_FOR_READ, move |_, _| ret(u32::from(opens)));
        for f in [
            MEMSET,
            STRING_FROM_TEXT,
            STRING_TIDY,
            STRING_ASSIGN,
            STRING_COPY,
            FILE_CONSTRUCT,
            FILE_CLOSE,
            FILE_DESTRUCT_BASE,
            FILE_DESTRUCT,
            MAKE_2C,
            MAKE_34,
            MAKE_38,
            CLEAR_60,
            CLEAR_78,
            BUFFER_RELEASE,
            BUFFER_DESTRUCT,
            INVALID_INDEX,
            SET_LAST_ERROR,
            STRING_ERASE,
            RESIZE_NAMED,
            RESIZE_2C,
            RESIZE_ROWS,
            RESIZE_PAIRS,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(RESIZE_POINTS, |e, a| {
            grow(e, a[0], a[1], 0xc);
            Ret::default()
        });
        e.register(RESIZE_34, |e, a| {
            grow(e, a[0], a[1], 0x34);
            Ret::default()
        });
        e.register(RESIZE_38, |e, a| {
            grow(e, a[0], a[1], 0x38);
            Ret::default()
        });
        e.register(RESIZE_WORDS, |e, a| {
            grow(e, a[0], a[1], 4);
            Ret::default()
        });
        e.register(WORD_AT, |e, a| ret(e.mem.u32(a[0] + 0xc) + a[1] * 4));
        e.register(BUFFER_CONSTRUCT, |e, a| {
            grow(e, a[0], a[1], 1);
            Ret::default()
        });
        // The string append sets the size (the text stays zeroed).
        e.register(STRING_APPEND_CHARS, |e, a| {
            let size = e.mem.u32(a[0] + 0x14) + a[1];
            e.mem.set_u32(a[0] + 0x14, size);
            Ret::default()
        });
        e
    }

    fn mesh(e: &mut Engine) -> u32 {
        e.mem.alloc(0xf0)
    }

    #[test]
    fn tri_file_that_does_not_open() {
        let mut e = tri_engine([0; 14], vec![], false);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ab_5ec0, &args![0x500u32, mesh]).bool());
        assert_eq!(calls_to(&e, FILE_CLOSE).len(), 1);
        assert_eq!(calls_to(&e, FILE_DESTRUCT_BASE).len(), 1);
        assert!(calls_to(&e, FILE_READ_HEADER).is_empty());
        let open = calls_to(&e, FILE_OPEN_FOR_READ);
        assert_eq!(open[0].len(), 9);
        assert_eq!(open[0][8], 0);
    }

    #[test]
    fn tri_file_with_vertices_and_a_differential_morph() {
        let mut header = [0u32; 14];
        header[0] = 2;
        header[7] = 1;
        let mut stream = vec![];
        for v in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0] {
            stream.extend(v.to_le_bytes());
        }
        stream.extend(3u32.to_le_bytes());
        stream.extend(b"ab\0");
        stream.extend(0.5f32.to_le_bytes());
        for s in [2i16, 4, 6, -2, 0, 8] {
            stream.extend(s.to_le_bytes());
        }
        let mut e = tri_engine(header, stream, true);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let vertices = e.mem.u32(mesh + 0xc);
        let read: Vec<f32> = (0..6).map(|i| e.mem.f32(vertices + i * 4)).collect();
        assert_eq!(read, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let morph = e.mem.u32(mesh + 0xc0 + 0xc);
        assert_eq!(e.mem.u32(morph + 0x14), 2);
        assert_eq!(e.mem.bytes(morph + 4, 2), b"ab".to_vec());
        let points = e.mem.u32(morph + 0x1c + 0xc);
        let scaled: Vec<f32> = (0..6).map(|i| e.mem.f32(points + i * 4)).collect();
        assert_eq!(scaled, vec![1.0, 2.0, 3.0, -1.0, 0.0, 4.0]);
        assert_eq!(calls_to(&e, FILE_DESTRUCT).len(), 1);
        assert!(calls_to(&e, SET_LAST_ERROR).is_empty());
        assert!(calls_to(&e, INVALID_INDEX).is_empty());
    }

    #[test]
    fn tri_file_whose_counts_do_not_add_up() {
        let mut header = [0u32; 14];
        header[8] = 1;
        let mut stream = vec![];
        stream.extend(0u32.to_le_bytes());
        stream.extend(1u32.to_le_bytes());
        stream.extend(7u32.to_le_bytes());
        let mut e = tri_engine(header, stream, true);
        let mesh = mesh(&mut e);
        e.call_log = Some(vec![]);
        assert!(!fff_read_tri_file(&mut e, Ptr::new(0x500), mesh));
        let morph = e.mem.u32(mesh + 0xd8 + 0xc);
        assert_eq!(e.mem.u32(morph + 0x1c), 0);
        let indices = e.mem.u32(morph + 0x20 + 0xc);
        assert_eq!(e.mem.u32(indices), 7);
        let error = calls_to(&e, SET_LAST_ERROR);
        assert_eq!(error.len(), 1);
        assert_eq!(error[0][0], 1);
        assert_eq!(error[0].len(), 8);
        assert_eq!(calls_to(&e, FILE_DESTRUCT).len(), 1);
    }
}
