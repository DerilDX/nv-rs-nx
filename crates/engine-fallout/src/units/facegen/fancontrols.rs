//! `FaceGen/fancontrols.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! FaceGen's control groups (`FanGenCtlsGrpC`) and the pieces of the C++ library and of
//! `FR2MatrixVTC<float>` (a `std::vector<float>` of 0x18 bytes with its row and column counts
//! at +0x18 and +0x1C) this unit instantiates. A linear control is 0x3C bytes: a matrix and
//! its label string at +0x20.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    string_chars, STRING_ASSIGN, STRING_LENGTH, STRING_NPOS, STRING_TIDY, VECTOR_FIRST, VECTOR_LAST,
};
use crate::units::facegen::binaryfile::{MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT};
use crate::units::facegen::run_00aba3c0::{
    advance_by_address, erase_by_address, grown_capacity, max_or_one, resize_with_value,
    vector_capacity, vector_size, with_value_slot, ElementSize, ResizeShape, WordsArg,
    THROW_LENGTH_0065F640, VECTOR_ALLOCATOR, VECTOR_END,
};

/// The matrix's row and column counts.
const MATRIX_ROWS: u32 = 0x18;
const MATRIX_COLUMNS: u32 = 0x1c;
/// The linear control's size and its label.
const CONTROL_SIZE: u32 = 0x3c;
const CONTROL_LABEL: u32 = 0x20;

/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and the source file names
/// (`"...\FaceGen\FanControls.cpp"` at `010a2c5c`, `"...\facegen\matrixVT.hpp"` at
/// `01065830`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const FAN_CONTROLS_SOURCE: u32 = 0x010a_2c5c;
const MATRIX_SOURCE: u32 = 0x0106_5830;
/// The matrix iterators: begin (`0044a170`, `thiscall`: out iterator), end (`0065f550`,
/// `thiscall`: out iterator; answers its address), made from a pointer and a container
/// (`0065da80`, `thiscall` on the iterator: pointer, container), compared (`0065dfd0`,
/// `thiscall`: the other; answers a byte, not 0 when they differ), dereferenced (`0064dbd0` and
/// `0064dda0`, `thiscall`; answer the element's address), post-incremented (`006590c0`,
/// `thiscall`: out copy and 0) and pre-incremented (`00659190`, `thiscall`).
const MATRIX_BEGIN: u32 = 0x0044_a170;
const MATRIX_END: u32 = 0x0065_f550;
const MAKE_ITERATOR: u32 = 0x0065_da80;
const ITERATORS_DIFFER: u32 = 0x0065_dfd0;
const DEREFERENCE: u32 = 0x0064_dbd0;
const DEREFERENCE_CHECKED: u32 = 0x0064_dda0;
const POST_INCREMENT: u32 = 0x0065_90c0;
const PRE_INCREMENT: u32 = 0x0065_9190;
/// The matrix's vector constructor (`00449fd0`), its resize with a value (`0060b410`:
/// count, float), the matrix copy constructor (`00658f30`: source), the matrix destructor
/// (`00449680`) and the matrix times a float (`00658260`, `FR2MatrixVTC<float>::operator*`:
/// result, float), all `thiscall`.
const VECTOR_CONSTRUCT: u32 = 0x0044_9fd0;
const VECTOR_RESIZE_WITH: u32 = 0x0060_b410;
const MATRIX_COPY: u32 = 0x0065_8f30;
const MATRIX_DESTRUCT: u32 = 0x0044_9680;
const MATRIX_TIMES_FLOAT: u32 = 0x0065_8260;
/// `0044a0a0` (`thiscall`, no stack words): what the control constructor runs on its matrix
/// after the counts.
const CONTROL_MATRIX_INIT: u32 = 0x0044_a0a0;
/// The product of a group's matrix with a vector (`00658360`, `thiscall`: result, the other),
/// the element pointer of a float vector (`0060b370`, `thiscall`: out word, index) and the
/// float vector's tidy (`0065f240`, `thiscall`).
const MATRIX_PRODUCT: u32 = 0x0065_8360;
const FLOAT_VECTOR_POINTER: u32 = 0x0060_b370;
const FLOAT_VECTOR_TIDY: u32 = 0x0065_f240;
/// The string's grow (`0044a8d0`, `thiscall`: new size, 0; answers a byte), the length throw
/// (`00ec30b0`, no arguments; does not return in the game), `memset` (`00ec61c0`, `cdecl`),
/// the string copy constructor (`0044a500`, `thiscall`: source).
const STRING_GROW: u32 = 0x0044_a8d0;
const STRING_THROW_LENGTH: u32 = 0x00ec_30b0;
const MEMSET: u32 = 0x00ec_61c0;
const STRING_COPY: u32 = 0x0044_a500;
/// The linear control's destructor (`006593e0`) and scalar deleting destructor (`00659380`:
/// flags), both `thiscall` (unit `fallout shared/facegen/run_006592c0`).
const CONTROL_DESTRUCT: u32 = 0x0065_93e0;
const CONTROL_DELETING_DESTRUCT: u32 = 0x0065_9380;
/// The linear control vector's helpers outside this unit: the allocation (`00658c10`, `cdecl`:
/// count, hint), the iterator advance (`00aaecb0`), the erase (`00aae200`), the range
/// assignment (`00aaf6f0`, `cdecl`: first, last, value) and the backward move (`00aaf860`,
/// `cdecl`, six words: first, last, end and three tags).
const CONTROL_ALLOCATE: u32 = 0x0065_8c10;
const CONTROL_ADVANCE: u32 = 0x00aa_ecb0;
const CONTROL_ERASE: u32 = 0x00aa_e200;
const CONTROL_ASSIGN: u32 = 0x00aa_f6f0;
const CONTROL_MOVE_BACKWARD: u32 = 0x00aa_f860;
/// The uninitialized copy of controls (`00aafa80`, `cdecl`, six words), unit `run_00aafa80`.
const CONTROL_UNINITIALIZED_COPY: u32 = 0x00aa_fa80;

// Translated from 00658650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator*=(float)` (the map's `operator__ov3`; `thiscall`, the
/// float): walks the matrix from its begin iterator (`0044a170`) to its end (`0065f550`,
/// `0065dfd0`) and multiplies each element (`0064dbd0`) by `scale` (`FLD`, `FMUL`, `FSTP`),
/// advancing with the post-increment (`006590c0`). Returns `this`.
pub fn fn_00658650(e: &mut Engine, this: Ptr, scale: f32) -> Ptr {
    e.with_stack(0x18, |e, locals| {
        let iterator = locals.addr();
        let end = locals.addr() + 8;
        let copy = locals.addr() + 0x10;
        e.call(MATRIX_BEGIN, &args![this, iterator]);
        loop {
            let end = e.call(MATRIX_END, &args![this, end]).u32();
            if e.call(ITERATORS_DIFFER, &args![iterator, end]).u8() == 0 {
                break;
            }
            let element = e.call(DEREFERENCE, &args![iterator]).u32();
            let product = x87_mul(x87_load32(e.mem.u32(element)), x87_m32(scale.to_bits()));
            e.mem.set_u32(element, x87_store32(product));
            e.call(POST_INCREMENT, &args![iterator, copy, 0u32]);
        }
    });
    this
}

// Translated from 00aabdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aabdb0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: one control's value (`thiscall`, four words: the group (below 5),
/// two indices (below 2) and a table of 0x40-byte vectors; result in `ST0`). Each bound that
/// is not met asserts (`fr2Assert(FanControls.cpp, 0xc6 / 0xc7 / 0xc8)`). An unloaded group
/// set (byte +0 is 0) gives 0.0. Otherwise the control at `+0x324 + group * 0xA0 + i * 0x50 +
/// j * 0x28` is multiplied with `table + j * 0x40` (`00658360`), the product's first float is
/// taken and the product freed (`0065f240` and the memory manager's free of its proxy), and the
/// control's offset (+0x24) is added.
pub fn fn_00aabdb0(
    e: &mut Engine,
    this: Ptr,
    group: i32,
    first: i32,
    second: i32,
    table: u32,
) -> f32 {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc6u32]);
    }
    if first >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc7u32]);
    }
    if second >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc8u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return 0.0;
    }
    let control = this
        .addr()
        .wrapping_add((group as u32).wrapping_mul(0xa0))
        .wrapping_add(0x324)
        .wrapping_add((first as u32).wrapping_mul(0x50))
        .wrapping_add((second as u32).wrapping_mul(0x28));
    let value = e.with_stack(0x28, |e, locals| {
        let product = locals.addr();
        let out = locals.addr() + 0x20;
        let other = table.wrapping_add((second as u32).wrapping_shl(6));
        let result = e
            .call(MATRIX_PRODUCT, &args![control, product, other])
            .u32();
        let pointer = e
            .call(FLOAT_VECTOR_POINTER, &args![result, out, 0u32])
            .u32();
        let floats = e.mem.u32(pointer);
        let value = x87_quiet_bits(e.mem.u32(floats));
        e.call(FLOAT_VECTOR_TIDY, &args![product]);
        let proxy = e.mem.u32(product);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
        value
    });
    let offset = e.mem.u32(control.wrapping_add(0x24));
    let sum = x87_add(x87_load32(value), x87_m32(offset));
    f32::from_bits(x87_store32(sum))
}

// Translated from 00aadd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the result and a 2x2 float matrix): the inverse. The
/// determinant `m0 * m3 - m1 * m2` and its reciprocal are stored as floats; the result is
/// `(m3, -m1, -m2, m0)` times the reciprocal (`FCHS` before the multiply). Returns `result`.
pub fn fn_00aadd20(e: &mut Engine, result: u32, matrix: u32) -> u32 {
    let m: Vec<u32> = (0..4).map(|i| e.mem.u32(matrix + 4 * i)).collect();
    let a = x87_mul(x87_load32(m[0]), x87_m32(m[3]));
    let b = x87_mul(x87_load32(m[1]), x87_m32(m[2]));
    let determinant = x87_store32(x87_sub(a, b));
    let reciprocal = x87_store32(x87_div(1.0, x87_load32(determinant)));
    let scale = |value: f64| x87_store32(x87_mul(value, x87_m32(reciprocal)));
    let r0 = scale(x87_load32(m[3]));
    let r1 = scale(-x87_load32(m[1]));
    let r2 = scale(-x87_load32(m[2]));
    let r3 = scale(x87_load32(m[0]));
    for (i, word) in [r0, r1, r2, r3].iter().enumerate() {
        e.mem.set_u32(result + 4 * i as u32, *word);
    }
    result
}

// Translated from 00aadda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator/(float)` (the map's `operator__ov4`; `thiscall`: the result
/// and the divisor): the matrix times `1 / divisor` (stored as a float) through `00658260`.
/// Returns `result`.
pub fn fn_00aadda0(e: &mut Engine, this: Ptr, result: u32, divisor: f32) -> u32 {
    let reciprocal = x87_store32(x87_div(1.0, x87_load32(divisor.to_bits())));
    let reciprocal = x87_store32(x87_load32(reciprocal));
    e.call(MATRIX_TIMES_FLOAT, &args![this, result, reciprocal]);
    result
}

// Translated from 00aadde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the linear control vector's `resize(count)` (`thiscall`): builds a
/// default control (`00aade20`) in the argument area and passes it by value (fifteen words) to
/// the resize with a value (`00aadf50`).
pub fn fn_00aadde0(e: &mut Engine, this: Ptr, count: u32) {
    e.with_stack(CONTROL_SIZE, |e, value| {
        fn_00aade20(e, value);
        let mut words = [0u32; 15];
        for (i, word) in words.iter_mut().enumerate() {
            *word = e.mem.u32(value.addr() + 4 * i as u32);
        }
        fn_00aadf50(e, this, count, words);
    });
}

// Translated from 00aade20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aade20 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control's default constructor: the matrix's vector
/// (`00449fd0`), zero rows and columns, `0044a0a0` on the matrix, and an empty label. Returns
/// `this`.
pub fn fn_00aade20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + MATRIX_ROWS, 0);
    e.mem.set_u32(this.addr() + MATRIX_COLUMNS, 0);
    e.call(CONTROL_MATRIX_INIT, &args![this]);
    e.call(STRING_TIDY, &args![this.addr() + CONTROL_LABEL, 0u32, 0u32]);
    this
}

// Translated from 00aadeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::append(count, character)` (unnamed in the engine map; `thiscall`, two words):
/// a count that reaches `npos - length` calls the length throw (`00ec30b0`, which does not
/// return in the game). For a count above 0 the string grows (`0044a8d0(length + count, 0)`);
/// when it can, the characters are set (`00aae180`), the length updated and a terminating 0
/// written. Returns `this`.
pub fn fn_00aadeb0(e: &mut Engine, this: Ptr, count: u32, character: u8) -> Ptr {
    let string = this.addr();
    let npos: u32 = e.global(STRING_NPOS);
    if npos.wrapping_sub(e.mem.u32(string + STRING_LENGTH)) <= count {
        e.call(STRING_THROW_LENGTH, &[]);
    }
    if count == 0 {
        return this;
    }
    let new_length = e.mem.u32(string + STRING_LENGTH).wrapping_add(count);
    if e.call(STRING_GROW, &args![this, new_length, 0u32]).u8() == 0 {
        return this;
    }
    let length = e.mem.u32(string + STRING_LENGTH);
    fn_00aae180(e, this, length, count, character);
    e.mem.set_u32(string + STRING_LENGTH, new_length);
    let chars = string_chars(e, string);
    e.mem.set_u8(chars.wrapping_add(new_length), 0);
    this
}

// Translated from 00aadf50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aadf50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control vector's `resize(count, value)` (`thiscall`,
/// sixteen words: the count and the control by value): grows with `00aae3a0`, or shrinks with
/// the advance `00aaecb0` and the erase `00aae200`; then the value is destroyed (`006593e0`).
pub fn fn_00aadf50(e: &mut Engine, this: Ptr, count: u32, value: [u32; 15]) {
    let shape = ResizeShape {
        size: ElementSize::Divide(CONTROL_SIZE),
        insert: |e, this, proxy, at, count, value| fn_00aae3a0(e, this, proxy, at, count, value),
        advance: |e, iterator, count| advance_by_address(CONTROL_ADVANCE, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(CONTROL_ERASE, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &value, |e, slot| {
        resize_with_value(e, this, count, slot, &shape);
        e.call(CONTROL_DESTRUCT, &args![slot]);
    });
}

fn fn_00aadf50_entry(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<15>) {
    fn_00aadf50(e, this, count, value.0)
}

// Translated from 00aae180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::_Chassign(offset, count, character)` (unnamed in the engine map; `thiscall`,
/// three words): one character is stored directly, more through `memset` (the character
/// sign-extended).
pub fn fn_00aae180(e: &mut Engine, this: Ptr, offset: u32, count: u32, character: u8) {
    let chars = string_chars(e, this.addr());
    if count == 1 {
        e.mem.set_u8(chars.wrapping_add(offset), character);
        return;
    }
    let fill = character as i8 as i32 as u32;
    e.call(MEMSET, &args![chars.wrapping_add(offset), fill, count]);
}

// Translated from 00aae3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aae3a0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: the linear control vector's `_Insert_n(where, count, value)`
/// (`thiscall`, four words). Nothing for 0; past the maximum (0x4444444) `0065f640` throws.
/// Past the capacity, a block of half as much again (at least enough; `00658c10`) gets the
/// controls before `where` (`00aafa30`), `count` copies (`00aaf8a0`) and the rest (`00aaf7e0`);
/// the old controls are destroyed (`00659380` with flag 0) and their block freed. Otherwise
/// the value is copied first: when fewer than `count` controls follow `where`, they move up
/// (`00aafa30`), the gap past the old end gets copies (`00aaf8a0`) and the rest is assigned
/// (`00aaf6f0`); else the last `count` controls are copied past the end (`00aaf7e0`), the others
/// move up (`00aaf860`) and `count` controls from `where` are assigned. The copy is destroyed
/// after.
pub fn fn_00aae3a0(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Divide(CONTROL_SIZE);
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    let maximum = max_or_one(0x0444_4444);
    if maximum.wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH_0065F640, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        let needed = vector_size(e, vector, size).wrapping_add(count);
        let new_capacity = grown_capacity(capacity, 0x0444_4444, needed);
        let block = e.call(CONTROL_ALLOCATE, &args![new_capacity, 0u32]).u32();
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let mut cursor = fn_00aafa30(e, first, at, block, allocator);
        fn_00aaf8a0(e, cursor, count, value, allocator, 0, 0);
        cursor = count.wrapping_mul(CONTROL_SIZE).wrapping_add(cursor);
        let last = e.mem.u32(vector + VECTOR_LAST);
        fn_00aaf7e0(e, at, last, cursor, allocator);
        let new_size = vector_size(e, vector, size).wrapping_add(count);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        if first != 0 {
            let last = e.mem.u32(vector + VECTOR_LAST);
            let mut element = first;
            while element != last {
                e.call(CONTROL_DELETING_DESTRUCT, &args![element, 0u32]);
                element = element.wrapping_add(CONTROL_SIZE);
            }
            let first = e.mem.u32(vector + VECTOR_FIRST);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
        }
        e.mem.set_u32(
            vector + VECTOR_END,
            new_capacity.wrapping_mul(CONTROL_SIZE).wrapping_add(block),
        );
        e.mem.set_u32(
            vector + VECTOR_LAST,
            new_size.wrapping_mul(CONTROL_SIZE).wrapping_add(block),
        );
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    e.with_stack(CONTROL_SIZE, |e, temporary| {
        let copy = temporary.addr();
        if after < count {
            fn_00aaec10(e, temporary, Ptr::new(value));
            let last = e.mem.u32(vector + VECTOR_LAST);
            fn_00aafa30(
                e,
                at,
                last,
                count.wrapping_mul(CONTROL_SIZE).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            fn_00aaf8a0(e, last, missing, copy, allocator, 0, 0);
            let last = count
                .wrapping_mul(CONTROL_SIZE)
                .wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            let end = last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            e.call(CONTROL_ASSIGN, &args![at, end, copy]);
            e.call(CONTROL_DESTRUCT, &args![copy]);
        } else {
            e.call(MATRIX_COPY, &args![copy, value]);
            e.call(
                STRING_COPY,
                &args![copy + CONTROL_LABEL, value + CONTROL_LABEL],
            );
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            let new_last = fn_00aaf7e0(e, tail, old_last, last, allocator);
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            let tail = old_last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            e.call(
                CONTROL_MOVE_BACKWARD,
                &args![at, tail, old_last, 0u32, 0u32, 0u32],
            );
            let end = count.wrapping_mul(CONTROL_SIZE).wrapping_add(at);
            e.call(CONTROL_ASSIGN, &args![at, end, copy]);
            e.call(STRING_TIDY, &args![copy + CONTROL_LABEL, 1u32, 0u32]);
            e.call(MATRIX_DESTRUCT, &args![copy]);
        }
    });
}

// Translated from 00aaec10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaec10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control's copy constructor (`thiscall`, the source):
/// the matrix's copy constructor (`00658f30`), then the label made empty and assigned the
/// source's. Returns `this`.
pub fn fn_00aaec10(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(MATRIX_COPY, &args![this, source]);
    let label = this.addr() + CONTROL_LABEL;
    e.call(STRING_TIDY, &args![label, 0u32, 0u32]);
    let npos: u32 = e.global(STRING_NPOS);
    e.call(
        STRING_ASSIGN,
        &args![label, source.addr() + CONTROL_LABEL, 0u32, npos],
    );
    this
}

// Translated from 00aaed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaed70 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FR2MatrixVTC<float>::operator-` (the map's `operator__ov5`; `thiscall`: the result and the
/// other matrix). Matrices of different sizes assert (`fr2Assert(matrixVT.hpp, 0x1ca)`). A
/// temporary matrix of this one's size is filled with zeros (`00449fd0`, `0060b410`); iterators
/// over it, this and the other (`0065da80`) walk to this one's end (`0065dfd0`), storing
/// `this - other` per element (`0064dda0`; `FLD`, `FSUB`, `FSTP`) and advancing this one's, the
/// other's and the temporary's (`00659190`). The result is copy-constructed from the temporary
/// (`00658f30`), which is then destroyed (`00449680`). Returns `result`.
pub fn fn_00aaed70(e: &mut Engine, this: Ptr, result: u32, other: Ptr) -> u32 {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(a + MATRIX_ROWS) != e.mem.u32(b + MATRIX_ROWS)
        || e.mem.u32(a + MATRIX_COLUMNS) != e.mem.u32(b + MATRIX_COLUMNS)
    {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x1cau32]);
    }
    let columns = e.mem.u32(a + MATRIX_COLUMNS);
    let rows = e.mem.u32(a + MATRIX_ROWS);
    e.with_stack(0x40, |e, locals| {
        let temporary = locals.addr();
        let out = locals.addr() + 0x20;
        let at_out = out;
        let at_a = out + 8;
        let at_b = out + 0x10;
        let at_end = out + 0x18;
        e.call(VECTOR_CONSTRUCT, &args![temporary]);
        e.mem.set_u32(temporary + MATRIX_ROWS, rows);
        e.mem.set_u32(temporary + MATRIX_COLUMNS, columns);
        let count = e
            .mem
            .u32(temporary + MATRIX_COLUMNS)
            .wrapping_mul(e.mem.u32(temporary + MATRIX_ROWS));
        e.call(
            VECTOR_RESIZE_WITH,
            &args![temporary, count, x87_store32(0.0)],
        );
        let first = e.mem.u32(temporary + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_out, first, temporary]);
        let first = e.mem.u32(a + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_a, first, a]);
        let first = e.mem.u32(b + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_b, first, b]);
        loop {
            let last = e.mem.u32(a + VECTOR_LAST);
            e.call(MAKE_ITERATOR, &args![at_end, last, a]);
            if e.call(ITERATORS_DIFFER, &args![at_a, at_end]).u8() == 0 {
                break;
            }
            let destination = e.call(DEREFERENCE_CHECKED, &args![at_out]).u32();
            let left = e.call(DEREFERENCE_CHECKED, &args![at_a]).u32();
            let right = e.call(DEREFERENCE_CHECKED, &args![at_b]).u32();
            let difference = x87_sub(x87_load32(e.mem.u32(left)), x87_m32(e.mem.u32(right)));
            e.mem.set_u32(destination, x87_store32(difference));
            e.call(PRE_INCREMENT, &args![at_a]);
            e.call(PRE_INCREMENT, &args![at_b]);
            e.call(PRE_INCREMENT, &args![at_out]);
        }
        e.call(MATRIX_COPY, &args![result, temporary]);
        e.call(MATRIX_DESTRUCT, &args![temporary]);
    });
    result
}

// Translated from 00aaf7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: first, last, destination, allocator): the
/// controls' uninitialized copy through `00aafa80` (with two tag words). Returns the
/// destination's end.
pub fn fn_00aaf7e0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    e.call(
        CONTROL_UNINITIALIZED_COPY,
        &args![first, last, destination, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00aaf8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaf8a0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: destination, count, the value's address, the
/// allocator and two tags): copy-constructs (`00aaec10`) `count` controls from the value,
/// skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00aaf8a0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00aaec10(e, Ptr::new(destination), Ptr::new(value));
        }
        count -= 1;
        destination = destination.wrapping_add(CONTROL_SIZE);
    }
}

// Translated from 00aafa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the same uninitialized copy of controls as
/// `00aaf7e0` (`00aafa80`). Returns the destination's end.
pub fn fn_00aafa30(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    e.call(
        CONTROL_UNINITIALIZED_COPY,
        &args![first, last, destination, allocator, 0u32, 0u32],
    )
    .u32()
}

/// The matrix helpers `readFromFile` calls: a matrix's norm (`006588a0`, `thiscall`; result in
/// `ST0`), its transpose (`006587a0`, `thiscall`: the result; answers it), the assignment
/// (`006581e0`, `FR2MatrixVTC<float>::operator=`, `thiscall`: the source), the CRT's
/// `sqrt(double)` (`00ec6054`, `cdecl`, result in `ST0`), and the free of a temporary's proxy
/// (`0065d6e0`, `thiscall`).
const MATRIX_NORM: u32 = 0x0065_88a0;
const MATRIX_TRANSPOSE: u32 = 0x0065_87a0;
const MATRIX_ASSIGN: u32 = 0x0065_81e0;
const SQRT: u32 = 0x00ec_6054;
const VECTOR_FREE_PROXY: u32 = 0x0065_d6e0;
/// The smallest norm `readFromFile` accepts without asserting (`double` 0.001 at `0101e788`).
const NORM_THRESHOLD: u32 = 0x0101_e788;
/// The binary file's `read` (vtable slot +0x10: buffer, element size, count).
const FILE_READ: u32 = 0x10;

/// `FanGenCtlsGrpC` (Xbox PDB) on PC, 0xFEC bytes: `initialized` (+0), `distrib` (+0x04, five
/// race distributions of 0xA0 bytes), `attr` (+0x324, five groups of 2x2 linear controls of
/// 0x28 bytes: a matrix, its inverse norm +0x20 and its offset +0x24), `race` (+0x644, 5x5 race
/// offset controls of 0x48 bytes: two matrices, the inverse norm sum +0x40 and an offset +0x44),
/// `basis` (+0xD4C, five groups of two pairs of matrices) and `invAttCov` (+0xFCC, five groups
/// of two 2x2 float matrices).
const ATTRIBUTES: u32 = 0x324;
const RACE_OFFSETS: u32 = 0x644;
const BASIS: u32 = 0xd4c;
const INVERSE_COVARIANCE: u32 = 0xfcc;

/// `file.read(buffer, size, count)`.
fn file_read(e: &mut Engine, file: Ptr, buffer: u32, size: u32, count: u32) -> bool {
    e.vcall(file.addr(), FILE_READ, &args![buffer, size, count])
        .u8()
        != 0
}

/// Sets a matrix's counts and resizes its floats to `count` zeros (`0060b410`).
fn shape_matrix(e: &mut Engine, matrix: u32, rows: u32, columns: u32, count: u32) {
    e.mem.set_u32(matrix + MATRIX_ROWS, rows);
    e.mem.set_u32(matrix + MATRIX_COLUMNS, columns);
    e.call(VECTOR_RESIZE_WITH, &args![matrix, count, x87_store32(0.0)]);
}

/// The address of a matrix's floats (`0060b370` with index 0, its out word at `out`).
fn matrix_floats(e: &mut Engine, matrix: u32, out: u32) -> u32 {
    let pointer = e
        .call(FLOAT_VECTOR_POINTER, &args![matrix, out, 0u32])
        .u32();
    e.mem.u32(pointer)
}

/// Frees a temporary matrix: its floats (`0065f240`) and its proxy (the memory manager).
fn free_temporary(e: &mut Engine, matrix: u32) {
    e.call(FLOAT_VECTOR_TIDY, &args![matrix]);
    let proxy = e.mem.u32(matrix);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

/// A matrix's norm through `sqrt`: the norm (`006588a0`) stored as a float, passed as a
/// `double` to `sqrt`, and the result stored as a float; it asserts (`fr2Assert(FanControls.cpp,
/// line)`) unless the result is above 0.001.
fn checked_norm(e: &mut Engine, matrix: u32, line: u32) -> u32 {
    let squared = x87_store32(e.call(MATRIX_NORM, &args![matrix]).f64());
    let root = e.call(SQRT, &args![x87_load32(squared)]).f64();
    let norm = x87_quiet_bits(x87_store32(root));
    let threshold: f64 = e.global(NORM_THRESHOLD);
    if !(x87_load32(norm) > threshold) {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, line]);
    }
    norm
}

// Translated from 00aaaf10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaaf10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FanGenCtlsGrpC::readFromFile` (Xbox PDB; `thiscall`: the binary file and the 2x2 count
/// table `n`, whose first column `n[k][0]` gives the sizes). Any failed read answers false.
/// 1. For each group and each 2x2 linear control: a 1 x `n[j][0]` matrix of zeros, its floats
///    and its offset (+0x24) read, and the inverse of its norm stored (+0x20).
/// 2. For each pair of different groups: two 1 x `n[k][0]` matrices read, their norms summed,
///    the offset (+0x44) read and the inverse of the sum stored (+0x40).
/// 3. For each group's distribution: two `n[k][0]` x 1 matrices, a square matrix of the total
///    size (`n[0][0] + n[1][0]`) and two `n[k][0]` square matrices, all read.
/// 4. For each group and control `i`: the basis' first matrix becomes control `[0][i]` over its
///    norm (`sqrt`, asserting above 0.001; `00aadda0`, `006581e0`); the second becomes control
///    `[1][i]` minus the first times the first float of `[1][i] * transpose(first)`
///    (`006587a0`, `00658360`, `00658260`, `00aaed70`), then is divided by its own norm (line
///    0x98; `00658650` with the inverse). Every temporary is freed.
/// 5. A constant-true check (`fr2Assert` at line 0xa1 never runs). For each group and control
///    `i`, the 2x2 matrix of first floats of `[a][i] * transpose([b][i])` is summed into
///    `invAttCov` and replaced by its inverse (`00aadd20`).
///
/// Then `initialized` is set and the answer is true.
pub fn fan_gen_ctls_grp_c_read_from_file(e: &mut Engine, this: Ptr, file: Ptr, table: u32) -> bool {
    let base = this.addr();
    let size_at = |e: &Engine, k: u32| e.mem.u32(table + k * 8);
    e.with_stack(0x100, |e, locals| {
        let out = locals.addr();
        let first_temporary = locals.addr() + 0x20;
        let second_temporary = locals.addr() + 0x40;
        let third_temporary = locals.addr() + 0x60;
        let fourth_temporary = locals.addr() + 0x80;
        let fifth_temporary = locals.addr() + 0xa0;
        let inverse = locals.addr() + 0xc0;
        let attribute = |group: u32, row: u32, column: u32| {
            base + ATTRIBUTES + group * 0xa0 + row * 0x50 + column * 0x28
        };
        for group in 0..5u32 {
            for row in 0..2u32 {
                for column in 0..2u32 {
                    let size = size_at(e, column);
                    let control = attribute(group, row, column);
                    shape_matrix(e, control, 1, size, size);
                    let floats = matrix_floats(e, control, out);
                    let count = size_at(e, column);
                    if !file_read(e, file, floats, 4, count) {
                        return false;
                    }
                    if !file_read(e, file, control + 0x24, 4, 1) {
                        return false;
                    }
                    let norm = e.call(MATRIX_NORM, &args![control]).f64();
                    e.mem
                        .set_u32(control + 0x20, x87_store32(x87_div(1.0, norm)));
                }
            }
        }
        for group in 0..5u32 {
            for other in 0..5u32 {
                if other == group {
                    continue;
                }
                let offsets = base + RACE_OFFSETS + group * 0x168 + other * 0x48;
                let mut sum = x87_store32(0.0);
                for k in 0..2u32 {
                    let size = size_at(e, k);
                    let control = offsets + k * 0x20;
                    shape_matrix(e, control, 1, size, size);
                    let floats = matrix_floats(e, control, out);
                    let count = size_at(e, k);
                    if !file_read(e, file, floats, 4, count) {
                        return false;
                    }
                    let norm = e.call(MATRIX_NORM, &args![control]).f64();
                    sum = x87_store32(x87_add(norm, x87_m32(sum)));
                }
                if !file_read(e, file, offsets + 0x44, 4, 1) {
                    return false;
                }
                let reciprocal = x87_div(1.0, x87_load32(sum));
                e.mem.set_u32(offsets + 0x40, x87_store32(reciprocal));
            }
        }
        let total = size_at(e, 0).wrapping_add(size_at(e, 1));
        for group in 0..5u32 {
            let distribution = base + 4 + group * 0xa0;
            for k in 0..2u32 {
                let size = size_at(e, k);
                let mean = distribution + k * 0x20;
                shape_matrix(e, mean, size, 1, size);
                let floats = matrix_floats(e, mean, out);
                if !file_read(e, file, floats, 4, size) {
                    return false;
                }
            }
            let covariance = base + 0x44 + group * 0xa0;
            shape_matrix(e, covariance, total, total, total.wrapping_mul(total));
            let floats = matrix_floats(e, covariance, out);
            if !file_read(e, file, floats, 4, total.wrapping_mul(total)) {
                return false;
            }
            for k in 0..2u32 {
                let size = size_at(e, k);
                let block = distribution + 0x60 + k * 0x20;
                shape_matrix(e, block, size, size, size.wrapping_mul(size));
                let floats = matrix_floats(e, block, out);
                if !file_read(e, file, floats, 4, size.wrapping_mul(size)) {
                    return false;
                }
            }
        }
        for group in 0..5u32 {
            for control_index in 0..2u32 {
                let control = base + ATTRIBUTES + group * 0xa0 + control_index * 0x28;
                let basis = base + BASIS + group * 0x80 + control_index * 0x40;
                let norm = checked_norm(e, control, 0x90);
                let divided =
                    fn_00aadda0(e, Ptr::new(control), first_temporary, f32::from_bits(norm));
                e.call(MATRIX_ASSIGN, &args![basis, divided]);
                e.call(FLOAT_VECTOR_TIDY, &args![first_temporary]);
                e.call(VECTOR_FREE_PROXY, &args![first_temporary]);
                let transposed = e
                    .call(MATRIX_TRANSPOSE, &args![basis, second_temporary])
                    .u32();
                let product = e
                    .call(
                        MATRIX_PRODUCT,
                        &args![control + 0x50, third_temporary, transposed],
                    )
                    .u32();
                let floats = matrix_floats(e, product, out);
                let projection = x87_quiet_bits(e.mem.u32(floats));
                free_temporary(e, third_temporary);
                free_temporary(e, second_temporary);
                let scaled = e
                    .call(
                        MATRIX_TIMES_FLOAT,
                        &args![basis, fourth_temporary, x87_quiet_bits(projection)],
                    )
                    .u32();
                let difference = fn_00aaed70(
                    e,
                    Ptr::new(control + 0x50),
                    fifth_temporary,
                    Ptr::new(scaled),
                );
                e.call(MATRIX_ASSIGN, &args![basis + 0x20, difference]);
                free_temporary(e, fifth_temporary);
                free_temporary(e, fourth_temporary);
                let norm = checked_norm(e, basis + 0x20, 0x98);
                let reciprocal = x87_store32(x87_div(1.0, x87_load32(norm)));
                fn_00658650(
                    e,
                    Ptr::new(basis + 0x20),
                    f32::from_bits(x87_quiet_bits(reciprocal)),
                );
            }
        }
        // The code tests a constant 1 here; its assertion (line 0xa1) never runs.
        if std::hint::black_box(1u32) == 0 {
            e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xa1u32]);
        }
        for group in 0..5u32 {
            for control_index in 0..2u32 {
                let covariance = base + INVERSE_COVARIANCE + group * 0x20 + control_index * 0x10;
                e.call(MEMSET, &args![covariance, 0u32, 0x10u32]);
                for a in 0..2u32 {
                    for b in 0..2u32 {
                        let cell = covariance + a * 8 + b * 4;
                        let transposed = e
                            .call(
                                MATRIX_TRANSPOSE,
                                &args![attribute(group, b, control_index), first_temporary],
                            )
                            .u32();
                        let product = e
                            .call(
                                MATRIX_PRODUCT,
                                &args![
                                    attribute(group, a, control_index),
                                    second_temporary,
                                    transposed
                                ],
                            )
                            .u32();
                        let floats = matrix_floats(e, product, out);
                        let sum = x87_add(x87_load32(e.mem.u32(floats)), x87_m32(e.mem.u32(cell)));
                        e.mem.set_u32(cell, x87_store32(sum));
                        free_temporary(e, second_temporary);
                        free_temporary(e, first_temporary);
                    }
                }
                let result = fn_00aadd20(e, inverse, covariance);
                for i in 0..4u32 {
                    let word = e.mem.u32(result + 4 * i);
                    e.mem.set_u32(covariance + 4 * i, word);
                }
            }
        }
        e.mem.set_u8(base, 1);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_8650, fn_00658650(Ptr, f32) -> Ptr),
        entry!(
            0x00aa_af10,
            fan_gen_ctls_grp_c_read_from_file(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00aa_bdb0, fn_00aabdb0(Ptr, i32, i32, i32, u32) -> f32),
        entry!(0x00aa_dd20, fn_00aadd20(u32, u32) -> u32),
        entry!(0x00aa_dda0, fn_00aadda0(Ptr, u32, f32) -> u32),
        entry!(0x00aa_dde0, fn_00aadde0(Ptr, u32)),
        entry!(0x00aa_de20, fn_00aade20(Ptr) -> Ptr),
        entry!(0x00aa_deb0, fn_00aadeb0(Ptr, u32, u8) -> Ptr),
        entry!(0x00aa_df50, fn_00aadf50_entry(Ptr, u32, WordsArg<15>)),
        entry!(0x00aa_e180, fn_00aae180(Ptr, u32, u32, u8)),
        entry!(0x00aa_e3a0, fn_00aae3a0(Ptr, u32, u32, u32, u32)),
        entry!(0x00aa_ec10, fn_00aaec10(Ptr, Ptr) -> Ptr),
        entry!(0x00aa_ed70, fn_00aaed70(Ptr, u32, Ptr) -> u32),
        entry!(0x00aa_f7e0, fn_00aaf7e0(u32, u32, u32, u32) -> u32),
        entry!(0x00aa_f8a0, fn_00aaf8a0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00aa_fa30, fn_00aafa30(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Doubles for the float vectors: a resize allocates `count` floats (the block at +0x0C),
    /// the element pointer gives that block, the strings and the allocator are quiet.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.set_global(NORM_THRESHOLD, 0.001f64);
        e.register(VECTOR_RESIZE_WITH, |e, a| {
            let block = e.mem.alloc((a[1] * 4).max(4));
            for i in 0..a[1] {
                e.mem.set_u32(block + 4 * i, a[2]);
            }
            e.mem.set_u32(a[0] + 0xc, block);
            e.mem.set_u32(a[0] + 0x10, block + 4 * a[1]);
            Ret::default()
        });
        e.register(FLOAT_VECTOR_POINTER, |e, a| {
            let block = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], block + 4 * a[2]);
            a[1].into_ret()
        });
        e.register(MEMSET, |e, a| {
            let fill = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &fill);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                FR2_ASSERT,
                STRING_TIDY,
                STRING_ASSIGN,
                STRING_COPY,
                MEMORY_MANAGER_FREE,
                FLOAT_VECTOR_TIDY,
                VECTOR_FREE_PROXY,
                VECTOR_CONSTRUCT,
                MATRIX_COPY,
                MATRIX_DESTRUCT,
                MATRIX_ASSIGN,
                MATRIX_TIMES_FLOAT,
                CONTROL_MATRIX_INIT,
                CONTROL_DESTRUCT,
                CONTROL_DELETING_DESTRUCT,
                CONTROL_ASSIGN,
                CONTROL_MOVE_BACKWARD,
                MAKE_ITERATOR,
                PRE_INCREMENT,
                POST_INCREMENT,
            ],
        );
        e
    }

    #[test]
    fn scale_multiplies_each_element() {
        let mut e = engine();
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 1.5);
        e.mem.set_f32(floats + 4, -2.0);
        // The iterator walks the two floats: begin stores the pointer, the end compare stops
        // after two, the dereference reads it and the post-increment advances it.
        e.register_double(MATRIX_BEGIN, move |e, a| {
            e.mem.set_u32(a[1] + 4, floats);
            Ret::default()
        });
        e.register(MATRIX_END, |_, a| a[1].into_ret());
        e.register_double(ITERATORS_DIFFER, move |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != floats + 8).into_ret()
        });
        e.register(DEREFERENCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(POST_INCREMENT, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 4);
            Ret::default()
        });
        let matrix = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0065_8650, &args![matrix, 3.0f32]).u32(), matrix);
        assert_eq!(e.mem.f32(floats), 4.5);
        assert_eq!(e.mem.f32(floats + 4), -6.0);
    }

    #[test]
    fn control_value_adds_the_offset_and_asserts_bounds() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        e.register(MATRIX_PRODUCT, |e, a| {
            let block = e.mem.alloc(4);
            e.mem.set_f32(block, 2.5);
            e.mem.set_u32(a[1] + 0xc, block);
            a[1].into_ret()
        });
        // Not loaded: 0.0, but the bounds are still checked.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_bdb0, &args![this, 5i32, 2i32, 1i32, 0u32])
                .f32(),
            0.0
        );
        let log = e.call_log.take().unwrap();
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines, vec![0xc6, 0xc7]);
        e.mem.set_u8(this, 1);
        let control = this + 0x324 + 2 * 0xa0 + 0x50 + 0x28;
        e.mem.set_f32(control + 0x24, 0.25);
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_bdb0, &args![this, 2i32, 1i32, 1i32, table])
                .f32(),
            2.75
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MATRIX_PRODUCT)[0][0], control);
        assert_eq!(calls_to(&log, MATRIX_PRODUCT)[0][2], table + 0x40);
    }

    #[test]
    fn inverse_of_a_2x2_matrix() {
        let mut e = engine();
        let matrix = e.mem.alloc(16);
        for (i, v) in [4.0f32, 7.0, 2.0, 6.0].iter().enumerate() {
            e.mem.set_f32(matrix + 4 * i as u32, *v);
        }
        let result = e.mem.alloc(16);
        assert_eq!(e.call(0x00aa_dd20, &args![result, matrix]).u32(), result);
        let r: Vec<f32> = (0..4).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(r, vec![0.6, -0.7, -0.2, 0.4]);
    }

    #[test]
    fn divide_multiplies_by_the_reciprocal() {
        let mut e = engine();
        let matrix = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_dda0, &args![matrix, 0x500u32, 4.0f32]).u32(),
            0x500
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MATRIX_TIMES_FLOAT),
            vec![vec![matrix, 0x500, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn default_control_and_its_copy() {
        let mut e = engine();
        let control = e.mem.alloc(CONTROL_SIZE);
        e.mem.set_u32(control + MATRIX_ROWS, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_de20, &args![control]).u32(), control);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(control + MATRIX_ROWS), 0);
        assert_eq!(calls_to(&log, CONTROL_MATRIX_INIT), vec![vec![control]]);
        assert_eq!(
            calls_to(&log, STRING_TIDY),
            vec![vec![control + 0x20, 0, 0]]
        );
        let copy = e.mem.alloc(CONTROL_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ec10, &args![copy, control]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MATRIX_COPY), vec![vec![copy, control]]);
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![copy + 0x20, control + 0x20, 0, u32::MAX]]
        );
    }

    #[test]
    fn string_append_grows_and_fills() {
        let mut e = engine();
        let string = e.mem.alloc(0x1c);
        e.mem.write(string + 4, b"ab\0");
        e.mem.set_u32(string + STRING_LENGTH, 2);
        e.mem.set_u32(string + 0x18, 15);
        e.register(STRING_GROW, |_, _| 1u32.into_ret());
        quiet(&mut e, &[STRING_THROW_LENGTH]);
        assert_eq!(
            e.call(0x00aa_deb0, &args![string, 3u32, b'x']).u32(),
            string
        );
        // Three characters through memset (the double above zero-fills): length 5, then 0.
        assert_eq!(e.mem.u32(string + STRING_LENGTH), 5);
        assert_eq!(e.mem.bytes(string + 4, 6), b"abxxx\0".to_vec());
        // One character is stored directly.
        e.call_log = Some(vec![]);
        e.call(0x00aa_deb0, &args![string, 1u32, b'y']);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMSET).is_empty());
        assert_eq!(e.mem.bytes(string + 4, 7), b"abxxxy\0".to_vec());
        // Nothing for 0; a failed grow leaves the string.
        e.call(0x00aa_deb0, &args![string, 0u32, b'z']);
        e.register(STRING_GROW, |_, _| Ret::default());
        e.call(0x00aa_deb0, &args![string, 2u32, b'z']);
        assert_eq!(e.mem.u32(string + STRING_LENGTH), 6);
        // A count reaching npos - length throws.
        e.call_log = Some(vec![]);
        e.call(0x00aa_deb0, &args![string, u32::MAX - 6, b'z']);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_THROW_LENGTH).len(), 1);
        // The character assignment on its own: a sign-extended memset fill.
        e.call_log = Some(vec![]);
        e.call(0x00aa_e180, &args![string, 0u32, 2u32, 0x80u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![string + 4, 0xffff_ff80, 2]]
        );
    }

    /// Doubles for the control vector's helpers outside the unit: the allocation and the
    /// uninitialized copy (`00aafa80`, copying each control's words).
    fn control_engine() -> Engine {
        let mut e = engine();
        e.register(CONTROL_ALLOCATE, |e, a| {
            e.mem.alloc((a[0] * CONTROL_SIZE).max(4)).into_ret()
        });
        e.register(CONTROL_UNINITIALIZED_COPY, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            if a[2] != 0 && !bytes.is_empty() {
                e.mem.write(a[2], &bytes);
            }
            (a[2] + (a[1] - a[0])).into_ret()
        });
        e.register(MATRIX_COPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e
    }

    fn control_vector(e: &mut Engine, tags: &[u32], capacity: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let block = e.mem.alloc((capacity * CONTROL_SIZE).max(4));
        for (i, tag) in tags.iter().enumerate() {
            e.mem.set_u32(block + CONTROL_SIZE * i as u32, *tag);
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(
            vector + VECTOR_LAST,
            block + CONTROL_SIZE * tags.len() as u32,
        );
        e.mem
            .set_u32(vector + VECTOR_END, block + CONTROL_SIZE * capacity);
        Ptr::new(vector)
    }

    fn tags_of(e: &Engine, vector: Ptr) -> Vec<u32> {
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        (first..last)
            .step_by(CONTROL_SIZE as usize)
            .map(|a| e.mem.u32(a))
            .collect()
    }

    #[test]
    fn control_resize_and_insert() {
        let mut e = control_engine();
        // Resize an empty vector to two default controls (00aadde0 -> 00aadf50 -> 00aae3a0).
        let vector = control_vector(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_dde0, &args![vector, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_ALLOCATE), vec![vec![2, 0]]);
        assert_eq!(tags_of(&e, vector).len(), 2);
        // The value is destroyed after the resize.
        assert_eq!(calls_to(&log, CONTROL_DESTRUCT).len(), 1);

        // Reallocating insert: the old controls are destroyed with flag 0.
        let vector = control_vector(&mut e, &[1, 2], 2);
        let value = e.mem.alloc(CONTROL_SIZE);
        e.mem.set_u32(value, 9);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_e3a0,
            &args![vector, 0u32, first + CONTROL_SIZE, 1u32, value],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(tags_of(&e, vector), vec![1, 9, 2]);
        assert_eq!(
            calls_to(&log, CONTROL_DELETING_DESTRUCT),
            vec![vec![first, 0], vec![first + CONTROL_SIZE, 0]]
        );

        // With room: the copy (by its two constructors) and the backward move.
        let vector = control_vector(&mut e, &[1, 2, 3], 8);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x00aa_e3a0, &args![vector, 0u32, first, 1u32, value]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_COPY).len(), 1);
        assert_eq!(calls_to(&log, CONTROL_MOVE_BACKWARD).len(), 1);
        assert_eq!(calls_to(&log, MATRIX_DESTRUCT).len(), 1);
        assert_eq!(tags_of(&e, vector).len(), 4);
        // With room at the end: the tail is filled with copies.
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        e.call_log = Some(vec![]);
        e.call(0x00aa_e3a0, &args![vector, 0u32, last, 2u32, value]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_DESTRUCT).len(), 1);
        assert_eq!(tags_of(&e, vector)[4..], [9, 9]);
        // Shrinking resizes erase through 00aae200 after advancing.
        quiet(&mut e, &[CONTROL_ADVANCE, CONTROL_ERASE]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_dde0, &args![vector, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_ERASE).len(), 1);
        assert_eq!(calls_to(&log, CONTROL_ADVANCE)[0][1], 1);
    }

    #[test]
    fn control_copies_and_fills() {
        let mut e = control_engine();
        let block = e.mem.alloc(CONTROL_SIZE * 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_f8a0, &args![block, 2u32, block, 0u32, 0u32, 0u32]);
        e.call(0x00aa_f8a0, &args![0u32, 1u32, block, 0u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MATRIX_COPY),
            vec![vec![block, block], vec![block + CONTROL_SIZE, block]]
        );
        let target = e.mem.alloc(CONTROL_SIZE * 2);
        for addr in [0x00aa_f7e0u32, 0x00aa_fa30] {
            e.call_log = Some(vec![]);
            let end = e
                .call(addr, &args![block, block + CONTROL_SIZE, target, 0u32])
                .u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(end, target + CONTROL_SIZE);
            assert_eq!(
                calls_to(&log, CONTROL_UNINITIALIZED_COPY),
                vec![vec![block, block + CONTROL_SIZE, target, 0, 0, 0]]
            );
        }
    }

    #[test]
    fn matrix_difference_walks_both_matrices() {
        let mut e = engine();
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        let left = e.mem.alloc(8);
        let right = e.mem.alloc(8);
        e.mem.set_f32(left, 5.0);
        e.mem.set_f32(left + 4, 1.0);
        e.mem.set_f32(right, 2.0);
        e.mem.set_f32(right + 4, 4.0);
        for (m, block) in [(a, left), (b, right)] {
            e.mem.set_u32(m + MATRIX_ROWS, 1);
            e.mem.set_u32(m + MATRIX_COLUMNS, 2);
            e.mem.set_u32(m + VECTOR_FIRST, block);
            e.mem.set_u32(m + VECTOR_LAST, block + 8);
        }
        // Iterators hold the pointer at +4; the end is this matrix's last pointer.
        e.register(MAKE_ITERATOR, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(ITERATORS_DIFFER, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(DEREFERENCE_CHECKED, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PRE_INCREMENT, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 4);
            Ret::default()
        });
        let result = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ed70, &args![a, result, b]).u32(), result);
        let log = e.call_log.take().unwrap();
        let copy = calls_to(&log, MATRIX_COPY);
        assert_eq!(copy.len(), 1);
        let temporary = copy[0][1];
        let floats = e.mem.u32(temporary + 0xc);
        assert_eq!(e.mem.f32(floats), 3.0);
        assert_eq!(e.mem.f32(floats + 4), -3.0);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // Different sizes assert.
        e.mem.set_u32(b + MATRIX_ROWS, 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_ed70, &args![a, result, b]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FR2_ASSERT), vec![vec![MATRIX_SOURCE, 0x1ca]]);
    }

    /// An engine for `readFromFile`: every read succeeds (zero bytes) until `fail_at` reads,
    /// norms are 4.0, the transpose records its source and the product gives 2.0 for a
    /// control times its own transpose, else 0.0.
    fn reader(fail_at: usize) -> (Engine, Ptr, Ptr, u32) {
        let mut e = engine();
        let mut reads = 0usize;
        e.register_double(0x0040_8000, move |e, a| {
            reads += 1;
            if reads == fail_at {
                return Ret::default();
            }
            let zeros = vec![0u8; (a[2] * a[3]) as usize];
            e.mem.write(a[1], &zeros);
            1u32.into_ret()
        });
        e.put_vtable(0x0090_b000, &[0, 0, 0, 0, 0x0040_8000]);
        let file = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(file.addr(), 0x0090_b000);
        e.register(MATRIX_NORM, |_, _| st0(4.0));
        e.register(SQRT, |_, a| st0(f64::take(a, &mut 0).sqrt()));
        e.register(MATRIX_TRANSPOSE, |e, a| {
            e.mem.set_u32(a[1] + 0x18, a[0]);
            a[1].into_ret()
        });
        e.register(MATRIX_PRODUCT, |e, a| {
            let source = e.mem.u32(a[2] + 0x18);
            let block = e.mem.alloc(4);
            e.mem.set_f32(block, if source == a[0] { 2.0 } else { 0.0 });
            e.mem.set_u32(a[1] + 0xc, block);
            a[1].into_ret()
        });
        e.register(ITERATORS_DIFFER, |_, _| Ret::default());
        e.register(MATRIX_BEGIN, |_, _| Ret::default());
        e.register(MATRIX_END, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_FLOAT, |e, a| {
            let rows = e.mem.u32(a[0] + MATRIX_ROWS);
            let columns = e.mem.u32(a[0] + MATRIX_COLUMNS);
            e.mem.set_u32(a[1] + MATRIX_ROWS, rows);
            e.mem.set_u32(a[1] + MATRIX_COLUMNS, columns);
            a[1].into_ret()
        });
        e.register(MATRIX_ASSIGN, |e, a| {
            let rows = e.mem.u32(a[1] + MATRIX_ROWS);
            let columns = e.mem.u32(a[1] + MATRIX_COLUMNS);
            e.mem.set_u32(a[0] + MATRIX_ROWS, rows);
            e.mem.set_u32(a[0] + MATRIX_COLUMNS, columns);
            a[0].into_ret()
        });
        let this = Ptr::new(e.mem.alloc(0x1000));
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 1);
        e.mem.set_u32(table + 8, 2);
        (e, this, file, table)
    }

    #[test]
    fn read_from_file_reads_every_part() {
        let (mut e, this, file, table) = reader(0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_af10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let base = this.addr();
        assert_eq!(e.mem.u8(base), 1);
        // Linear controls: 1 x n[j][0] matrices, inverse norm 1/4.
        assert_eq!(e.mem.u32(base + ATTRIBUTES + 0x28 + MATRIX_COLUMNS), 2);
        assert_eq!(e.mem.f32(base + ATTRIBUTES + 0x20), 0.25);
        // Race offsets: the inverse of the two norms' sum, 1/8.
        assert_eq!(e.mem.f32(base + RACE_OFFSETS + 0x48 + 0x40), 0.125);
        // Distributions: a 3 x 3 covariance (1 + 2).
        assert_eq!(e.mem.u32(base + 0x44 + MATRIX_ROWS), 3);
        // The inverse covariance of diag(2, 2).
        let inverse: Vec<f32> = (0..4)
            .map(|i| e.mem.f32(base + INVERSE_COVARIANCE + 4 * i))
            .collect();
        assert_eq!(inverse, vec![0.5, 0.0, 0.0, 0.5]);
        // No norm below 0.001, so no assertion.
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // Each basis' second matrix is scaled by its inverse norm (sqrt(4) = 2).
        assert_eq!(calls_to(&log, SQRT).len(), 20);
    }

    #[test]
    fn read_from_file_stops_at_a_failed_read_and_asserts_small_norms() {
        for fail_at in [1usize, 2, 41, 43, 101, 103] {
            let (mut e, this, file, table) = reader(fail_at);
            assert!(
                !e.call(0x00aa_af10, &args![this, file, table]).bool(),
                "{fail_at}"
            );
            assert_eq!(e.mem.u8(this.addr()), 0);
        }
        let (mut e, this, file, table) = reader(0);
        e.register(MATRIX_NORM, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_af10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines.len(), 20);
        assert_eq!(&lines[..2], &[0x90, 0x98]);
    }

    #[test]
    fn control_resize_with_value_takes_fifteen_words() {
        let mut e = control_engine();
        let vector = control_vector(&mut e, &[], 0);
        let mut call = vec![vector.addr(), 1];
        call.extend(7..22u32);
        e.call_log = Some(vec![]);
        e.call(0x00aa_df50, &call);
        let log = e.call_log.take().unwrap();
        assert_eq!(tags_of(&e, vector), vec![7]);
        let destroyed = calls_to(&log, CONTROL_DESTRUCT);
        assert_eq!(destroyed.len(), 1);
        assert_eq!(e.mem.u32(destroyed[0][0]), 7);
    }
}
