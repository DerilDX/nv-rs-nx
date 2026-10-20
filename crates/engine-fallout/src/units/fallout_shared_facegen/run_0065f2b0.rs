//! `fallout shared/facegen/run_0065f2b0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0065f580` calls on the member (`0065f7c0`).
const CALLEE_0065F7C0: u32 = 0x0065_f7c0;

/// The function `fn_0065f6e0` calls on the member (`0065f7f0`).
const CALLEE_0065F7F0: u32 = 0x0065_f7f0;

// Translated from 0065f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7c0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f580(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7C0, &args![this.byte_add(0x8)]).u32()
}

// Translated from 0065f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7f0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f6e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7F0, &args![this.byte_add(0x8)]).u32()
}

// The functions below are `std::vector` template code for 6-byte elements and the copy
// algorithms it uses (unnamed in the map; the element type is not identified), plus the
// copy constructors of two vectors and of the 0x28-byte record holding three of them. The
// vectors keep their allocator at `+8` and `_Myfirst`, `_Mylast` and `_Myend` at `+0xc`,
// `+0x10` and `+0x14`.

/// The vector's allocator, `_Myfirst`, `_Mylast` and `_Myend`.
const VECTOR_ALLOCATOR: u32 = 0x08;
const VECTOR_FIRST: u32 = 0x0c;
const VECTOR_LAST: u32 = 0x10;
const VECTOR_END: u32 = 0x14;
/// `0065e750` (registered with the caller's `ECX` as its first word, then two stack words it
/// does not read): gives the iterator tag byte.
const ITERATOR_TAG_0065E750: u32 = 0x0065_e750;
/// `0043b1d0` (cdecl, two words; reads only the first): the word an iterator points at.
const UNWRAP_0043B1D0: u32 = 0x0043_b1d0;
/// `operator new` (`00401000`, cdecl, one word), `std::bad_alloc`'s constructor (`0044d6c0`,
/// thiscall, one word), `_CxxThrowException` (`00ec7894`, two words; it does not return in the
/// game) and `std::bad_alloc`'s throw information (`01140ef8`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const BAD_ALLOC_CTOR_0044D6C0: u32 = 0x0044_d6c0;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const BAD_ALLOC_THROW_INFO_01140EF8: u32 = 0x0114_0ef8;

// Translated from 0065f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Destroy(first, last)` for the 6-byte vector (thiscall, two words, `RET
/// 8`): [`fn_0065f960`] with the range and the allocator at `+8`.
pub fn fn_0065f2b0(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_0065f960(e, first, last, this.addr() + VECTOR_ALLOCATOR);
}

// Translated from 0065f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Buy(count)` for 6-byte elements (thiscall, one word, `RET 4`; the result
/// in `AL`): clears `_Myfirst`, `_Mylast` and `_Myend`; 0 for a count of 0. When `max_size()`
/// ([`fn_0065f580`]) is below the count it calls the length error `0065f640` (no arguments; it
/// does not return in the game, the code after it returns 1); otherwise it allocates `count`
/// elements (`0065f6c0`, thiscall on the allocator) into `_Myfirst` and `_Mylast` and sets
/// `_Myend` to `_Myfirst + count * 6`. Returns 1.
pub fn fn_0065f380(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
    if count == 0 {
        return false;
    }
    if fn_0065f580(e, this) < count {
        e.call(0x0065_f640, &args![]);
    } else {
        let block = e
            .call(0x0065_f6c0, &args![base + VECTOR_ALLOCATOR, count])
            .u32();
        e.mem.set_u32(base + VECTOR_FIRST, block);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem.set_u32(base + VECTOR_LAST, first);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem
            .set_u32(base + VECTOR_END, count.wrapping_mul(6).wrapping_add(first));
    }
    true
}

// Translated from 0065f400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the second 6-byte vector's `_Destroy(first, last)` (thiscall, two words, `RET 8`):
/// [`fn_0065f9a0`] with the range and the allocator at `+8`.
pub fn fn_0065f400(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_0065f9a0(e, first, last, this.addr() + VECTOR_ALLOCATOR);
}

// Translated from 0065f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Tidy()` for 6-byte elements (thiscall): when `_Myfirst` is set,
/// destroys `[_Myfirst, _Mylast)` ([`fn_0065f400`]) and deallocates the buffer
/// (`0064df10(allocator, _Myfirst, (_Myend - _Myfirst) / 6)`, signed division); then clears the
/// three pointers.
pub fn fn_0065f430(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + VECTOR_FIRST) != 0 {
        let first = e.mem.u32(base + VECTOR_FIRST);
        let last = e.mem.u32(base + VECTOR_LAST);
        fn_0065f400(e, this, first, last);
        let bytes = e
            .mem
            .u32(base + VECTOR_END)
            .wrapping_sub(e.mem.u32(base + VECTOR_FIRST));
        let count = (bytes as i32).wrapping_div(6) as u32;
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.call(0x0064_df10, &args![base + VECTOR_ALLOCATOR, first, count]);
    }
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
}

// Translated from 0065f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::end()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065e000` (thiscall on the result) with `_Mylast` and the vector.
/// Returns the iterator.
pub fn fn_0065f550(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + VECTOR_LAST);
    e.call(0x0065_e000, &args![out, last, this]);
    out
}

// Translated from 0065f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the byte allocator's `allocate(count)` (thiscall, one word, `RET 4`; `this` is not
/// read): [`fn_0065fa30`] with the count and 0, its result returned.
pub fn fn_0065f7a0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_0065fa30(e, count, 0)
}

// Translated from 0065f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the byte allocator's `max_size()` (thiscall): the local `0xffffffff`, or 1 when
/// that is 0 (the compiler kept the test; it always returns `0xffffffff`).
pub fn fn_0065f7f0(_e: &mut Engine, _this: Ptr) -> u32 {
    let count = std::hint::black_box(0xffff_ffffu32);
    if count > 0 {
        count
    } else {
        1
    }
}

/// `_Destroy_range(first, last, allocator)` for the 6-byte vectors (cdecl, three words): the
/// iterator tag from `0065e750` (`ECX` the address of the `first` argument word), then
/// `destroy` (cdecl) with the range, the allocator and the tag byte.
fn destroy_range(
    e: &mut Engine,
    first: u32,
    last: u32,
    allocator: u32,
    destroy: &dyn Fn(&mut Engine, u32, u32, u32, u32),
) {
    e.with_stack(8, |e, arguments| {
        let (first_at, last_at) = (arguments.addr(), arguments.addr() + 4);
        e.mem.set_u32(first_at, first);
        e.mem.set_u32(last_at, last);
        let tag = e
            .call(ITERATOR_TAG_0065E750, &args![first_at, first_at, last_at])
            .u32() as u8;
        let last = e.mem.u32(last_at);
        let first = e.mem.u32(first_at);
        destroy(e, first, last, allocator, tag as u32);
    });
}

// Translated from 0065f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Destroy_range` for the first 6-byte vector (cdecl, three words; see
/// [`destroy_range`]): its range destructor is the empty stub `0040fbe0` (four words).
pub fn fn_0065f960(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(
        e,
        first,
        last,
        allocator,
        &|e, first, last, allocator, tag| {
            e.call(0x0040_fbe0, &args![first, last, allocator, tag]);
        },
    );
}

// Translated from 0065f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Destroy_range` for the second 6-byte vector (cdecl, three words; see
/// [`destroy_range`]): its range destructor is [`fn_0065ff00`].
pub fn fn_0065f9a0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(
        e,
        first,
        last,
        allocator,
        &|e, first, last, allocator, tag| {
            fn_0065ff00(e, first, last, allocator, tag);
        },
    );
}

/// `std::_Allocate(count, T*)` for one element size: a count of 0 stays 0; when `0xffffffff /
/// count` is below the element size it constructs a `std::bad_alloc` (12 bytes on the stack,
/// message 0) and throws it (which does not return in the game; the code after it is the
/// allocation). Returns `operator new(bytes)`, `bytes` being what `size` makes of the count.
fn allocate_elements(e: &mut Engine, count: u32, element: u32, size: fn(u32) -> u32) -> u32 {
    if count != 0 && 0xffff_ffff / count < element {
        e.with_stack(0xc, |e, exception| {
            e.call(BAD_ALLOC_CTOR_0044D6C0, &args![exception, 0u32]);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![exception, BAD_ALLOC_THROW_INFO_01140EF8],
            );
        });
    }
    e.call(OPERATOR_NEW, &args![size(count)]).u32()
}

// Translated from 0065f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` for 6-byte elements (cdecl, two words; the second is not read):
/// see [`allocate_elements`]; the size is `count * 6` (`IMUL`).
pub fn fn_0065f9e0(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 6, |count| count.wrapping_mul(6))
}

// Translated from 0065fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` for bytes (cdecl, two words; the second is not read): see
/// [`allocate_elements`] with element size 1 (the throw is reached by no count); the size is
/// the count.
pub fn fn_0065fa30(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 1, |count| count)
}

// Translated from 0065fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the copy constructor of a 0x58-byte record (thiscall, one word: the source; `RET
/// 4`): copies the float at `+0` (`FLD`/`FSTP`) and the words at `+4`, `+8` and `+0xc`, then
/// copy-constructs the three 0x18-byte vectors at `+0x10` with the `eh vector copy constructor
/// iterator` (`00ecaebb`, stdcall, six words: destination, source, 0x18, 3, the copy
/// constructor [`fn_0065fc10`] and the destructor `0065f060`). Returns `this`.
pub fn fn_0065fab0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    let from = source.addr();
    let value = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(base, value);
    for offset in [4u32, 8, 0xc] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(base + offset, word);
    }
    e.call(
        0x00ec_aebb,
        &args![
            base + 0x10,
            from + 0x10,
            0x18u32,
            3u32,
            0x0065_fc10u32,
            0x0065_f060u32
        ],
    );
    this
}

/// The vector copy constructor shape the two copy constructors below share: the allocator is copied into a
/// stack word (`0065de30` on the word, which `PUSH ECX` set to `this`) and passed by value to
/// the base constructor `0065d3a0`; the size of `other` (`size`, thiscall) is bought (`buy`,
/// thiscall, result in `AL`), and when that succeeds `copy` (thiscall, five words: `begin()`
/// and `end()` of `other` as two-word iterators from [`fn_0065fd10`] and [`fn_0065fd40`], then
/// `_Myfirst`) fills the buffer and its result becomes `_Mylast`. Returns `this`.
fn vector_copy(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    size: u32,
    buy: &dyn Fn(&mut Engine, Ptr, u32) -> bool,
    copy: u32,
) -> Ptr {
    let base = this.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        e.call(0x0065_de30, &args![slot, other.addr() + VECTOR_ALLOCATOR]);
        let word = e.mem.u32(slot.addr());
        e.call(0x0065_d3a0, &args![base, word]);
    });
    let count = e.call(size, &args![other]).u32();
    if buy(e, this, count) {
        e.with_stack(0x10, |e, iterators| {
            // `begin()` at +0 (EBP-0x24), `end()` at +8 (EBP-0x1c).
            let end = fn_0065fd40(e, other, Ptr::new(iterators.addr() + 8)).addr();
            let begin = fn_0065fd10(e, other, iterators).addr();
            let first = e.mem.u32(base + VECTOR_FIRST);
            let (begin_base, begin_pointer) = (e.mem.u32(begin), e.mem.u32(begin + 4));
            let (end_base, end_pointer) = (e.mem.u32(end), e.mem.u32(end + 4));
            let result = e
                .call(
                    copy,
                    &args![
                        base,
                        begin_base,
                        begin_pointer,
                        end_base,
                        end_pointer,
                        first
                    ],
                )
                .u32();
            e.mem.set_u32(base + VECTOR_LAST, result);
        });
    }
    this
}

// Translated from 0065fb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065fb10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (`_Tidy` and rethrow) are not translated
/// Unnamed: the first 6-byte vector's copy constructor (thiscall, one word, `RET 4`): see
/// [`vector_copy`] with `size` `00657bc0`, `buy` [`fn_0065f380`] and `copy` `0065ff30`.
pub fn fn_0065fb10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_copy(e, this, other, 0x0065_7bc0, &fn_0065f380, 0x0065_ff30)
}

// Translated from 0065fc10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065fc10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (`_Tidy` and rethrow) are not translated
/// Unnamed: the copy constructor of the vector in the 0x18-byte slots of [`fn_0065fab0`]
/// (thiscall, one word, `RET 4`): see [`vector_copy`] with `size` `0065cc50`, `buy` `00657ee0`
/// (called by address) and `copy` `0065ff60`.
pub fn fn_0065fc10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_copy(
        e,
        this,
        other,
        0x0065_cc50,
        &|e, this, count| e.call(0x0065_7ee0, &args![this, count]).bool(),
        0x0065_ff60,
    )
}

// Translated from 0065fd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::begin()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065da80` (thiscall on the result) with `_Myfirst` and the vector.
pub fn fn_0065fd10(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let first = e.mem.u32(this.addr() + VECTOR_FIRST);
    e.call(0x0065_da80, &args![out, first, this]);
    out
}

// Translated from 0065fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::end()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065da80` (thiscall on the result) with `_Mylast` and the vector.
pub fn fn_0065fd40(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + VECTOR_LAST);
    e.call(0x0065_da80, &args![out, last, this]);
    out
}

// Translated from 0065fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl; its callers push three words): returns its first argument.
pub fn fn_0065fd70(_e: &mut Engine, first: u32, _unused_2: u32, _unused_3: u32) -> u32 {
    first
}

// Translated from 0065fe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: unwraps a checked iterator (cdecl, one word: the iterator's address): `0043b1d0`
/// with the address and the byte at `EBP-1` (the top byte of the caller's `ECX`, which the
/// `PUSH ECX` of the frame left there); `0043b1d0` reads only its first word, so the byte is
/// passed as 0 here. Returns `0043b1d0`'s result.
pub fn fn_0065fe40(e: &mut Engine, iterator: u32) -> u32 {
    e.call(UNWRAP_0043B1D0, &args![iterator, 0u32]).u32()
}

/// The shape of the checked uninitialized copy and copy wrappers (cdecl, four words: first,
/// last, destination, allocator; a frame of four local bytes below `EBP`). The byte at `EBP-1`
/// is set to 0; the iterator tag comes from `0065e750` with `ECX` the address of the
/// destination word (pushed: the addresses of the first and destination words, or with
/// `one_push` only the destination's, the next stack word being the frame's local) and is
/// kept at `EBP-2`; last and first are unwrapped ([`fn_0065fe40`]); then `next` runs with the
/// unwrapped first, last, the destination, the allocator, the tag and the zero byte.
fn checked_forward(
    e: &mut Engine,
    words: [u32; 4],
    one_push: bool,
    next: fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    e.with_stack(0x1c, |e, frame| {
        // EBP is frame + 4: the local at frame + 0..4, the arguments from EBP + 8.
        let ebp = frame.addr() + 4;
        let (first_at, last_at, destination_at, allocator_at) =
            (ebp + 8, ebp + 0xc, ebp + 0x10, ebp + 0x14);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(first_at + 4 * i as u32, *word);
        }
        e.mem.set_u8(ebp - 1, 0);
        let tag = if one_push {
            let local = e.mem.u32(ebp - 4);
            e.call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, destination_at, local],
            )
        } else {
            e.call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, first_at, destination_at],
            )
        }
        .u32() as u8;
        e.mem.set_u8(ebp - 2, tag);
        let zero = e.mem.u8(ebp - 1) as u32;
        let tag = e.mem.u8(ebp - 2) as u32;
        let allocator = e.mem.u32(allocator_at);
        let destination = e.mem.u32(destination_at);
        let last = fn_0065fe40(e, last_at);
        let first = fn_0065fe40(e, first_at);
        next(e, first, last, destination, allocator, tag, zero)
    })
}

// Translated from 0065fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for the first 6-byte vector (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660070`].
pub fn fn_0065fde0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660070)
}

// Translated from 0065fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the plain-data copy wrapper for the second 6-byte vector (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660270`].
pub fn fn_0065fea0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660270)
}

// Translated from 0065ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the range destructor of the second 6-byte vector (cdecl; its caller pushes four
/// words, it reads three): `0065e9a0` (thiscall on the allocator, one word) on each element
/// of `[first, last)`.
pub fn fn_0065ff00(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    let mut element = first;
    while element != last {
        e.call(0x0065_e9a0, &args![allocator, element]);
        element = element.wrapping_add(6);
    }
}

// Translated from 0065ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked uninitialized copy wrapper (cdecl, four words): see [`checked_forward`]
/// (one push), then [`fn_00660210`].
pub fn fn_0065ff90(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], true, fn_00660210)
}

// Translated from 0065ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked uninitialized copy wrapper (cdecl, four words): see [`checked_forward`]
/// (one push), then [`fn_00660230`].
pub fn fn_0065ffe0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], true, fn_00660230)
}

/// The element loop of `_Uninitialized_copy` for one element size (cdecl, six words; the tag
/// words are not read): `construct` (thiscall on the allocator, two words: the destination and
/// the source element) for each element, both pointers advancing by the element size. Returns
/// the destination's end.
fn construct_each(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    element: u32,
    construct: u32,
) -> u32 {
    let (mut source, mut target) = (first, destination);
    while source != last {
        e.call(construct, &args![allocator, target, source]);
        target = target.wrapping_add(element);
        source = source.wrapping_add(element);
    }
    target
}

// Translated from 00660070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00660070 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 6-byte elements (cdecl, six words): see
/// [`construct_each`] with `006601f0`.
pub fn fn_00660070(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 6, 0x0066_01f0)
}

// Translated from 00660210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, six words; the last two are not read): [`fn_00660340`] with the first four,
/// its result returned.
pub fn fn_00660210(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_00660340(e, first, last, destination, allocator)
}

// Translated from 00660230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, six words; the last two are not read): [`fn_006603a0`] with the first four,
/// its result returned.
pub fn fn_00660230(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_006603a0(e, first, last, destination, allocator)
}

// Translated from 00660270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the plain-data copy (cdecl, six words; it reads the first three): `memmove_s`
/// (`00ec7ce3`, cdecl: destination, size, source, count) of the `last - first` bytes when that
/// is not 0. Returns `destination + (last - first)`.
pub fn fn_00660270(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let bytes = last.wrapping_sub(first);
    let end = destination.wrapping_add(bytes);
    if bytes > 0 {
        e.call(0x00ec_7ce3, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 00660340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for 0x1c-byte elements (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660510`].
pub fn fn_00660340(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660510)
}

// Translated from 006603a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for 0x58-byte elements (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_006605d0`].
pub fn fn_006603a0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_006605d0)
}

// Translated from 00660510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00660510 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 0x1c-byte elements (cdecl, six words): see
/// [`construct_each`] with `00660300`.
pub fn fn_00660510(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 0x1c, 0x0066_0300)
}

// Translated from 006605d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006605d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 0x58-byte elements (cdecl, six words): see
/// [`construct_each`] with `00660320`.
pub fn fn_006605d0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 0x58, 0x0066_0320)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_f580, fn_0065f580(Ptr) -> u32),
        entry!(0x0065_f6e0, fn_0065f6e0(Ptr) -> u32),
        entry!(0x0065_f2b0, fn_0065f2b0(Ptr, u32, u32)),
        entry!(0x0065_f380, fn_0065f380(Ptr, u32) -> bool),
        entry!(0x0065_f400, fn_0065f400(Ptr, u32, u32)),
        entry!(0x0065_f430, fn_0065f430(Ptr)),
        entry!(0x0065_f550, fn_0065f550(Ptr, Ptr) -> Ptr),
        entry!(0x0065_f7a0, fn_0065f7a0(Ptr, u32) -> u32),
        entry!(0x0065_f7f0, fn_0065f7f0(Ptr) -> u32),
        entry!(0x0065_f960, fn_0065f960(u32, u32, u32)),
        entry!(0x0065_f9a0, fn_0065f9a0(u32, u32, u32)),
        entry!(0x0065_f9e0, fn_0065f9e0(u32, u32) -> u32),
        entry!(0x0065_fa30, fn_0065fa30(u32, u32) -> u32),
        entry!(0x0065_fab0, fn_0065fab0(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fb10, fn_0065fb10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fc10, fn_0065fc10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd10, fn_0065fd10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd40, fn_0065fd40(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd70, fn_0065fd70(u32, u32, u32) -> u32),
        entry!(0x0065_fde0, fn_0065fde0(u32, u32, u32, u32) -> u32),
        entry!(0x0065_fe40, fn_0065fe40(u32) -> u32),
        entry!(0x0065_fea0, fn_0065fea0(u32, u32, u32, u32) -> u32),
        entry!(0x0065_ff00, fn_0065ff00(u32, u32, u32, u32)),
        entry!(0x0065_ff90, fn_0065ff90(u32, u32, u32, u32) -> u32),
        entry!(0x0065_ffe0, fn_0065ffe0(u32, u32, u32, u32) -> u32),
        entry!(0x0066_0070, fn_00660070(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0210, fn_00660210(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0230, fn_00660230(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0270, fn_00660270(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0340, fn_00660340(u32, u32, u32, u32) -> u32),
        entry!(0x0066_03a0, fn_006603a0(u32, u32, u32, u32) -> u32),
        entry!(0x0066_0510, fn_00660510(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_05d0, fn_006605d0(u32, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0065f580_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f580, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }

    #[test]
    fn l68d_0065f6e0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7F0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f6e0, &args![0x4200u32]).u32(), 0x175d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x8]);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Doubles for the iterator helpers: the tag is 0x5a, unwrapping reads the word.
    fn iterator_doubles(e: &mut Engine) {
        e.register(ITERATOR_TAG_0065E750, |_, _| 0x1_5au32.into_ret());
        e.register(UNWRAP_0043B1D0, |e, a| e.mem.u32(a[0]).into_ret());
    }

    #[test]
    fn b046_0065f2b0_and_0065f960_destroy_through_the_empty_stub() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0040_fbe0, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0065_f2b0, &args![0x4000u32, 0x100u32, 0x112u32]);
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tag.len(), 1);
        assert_eq!(tag[0][0], tag[0][1]);
        assert_eq!(tag[0][2], tag[0][1] + 4);
        assert_eq!(
            calls_to(&e, 0x0040_fbe0),
            vec![vec![0x100, 0x112, 0x4008, 0x5a]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_f960, &args![1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, 0x0040_fbe0), vec![vec![1, 2, 3, 0x5a]]);
    }

    #[test]
    fn b046_0065f380_buys_six_byte_elements() {
        let mut e = Engine::new();
        e.register(0x0065_f7c0, |_, _| 10u32.into_ret());
        e.register(0x0065_f640, quiet);
        e.register(0x0065_f6c0, |_, _| 0x9000u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcc; 0x18]);
        assert!(!e.call(0x0065_f380, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_f380, &args![this, 11u32]).bool());
        assert_eq!(calls_to(&e, 0x0065_f640).len(), 1);
        assert!(calls_to(&e, 0x0065_f6c0).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_f380, &args![this, 10u32]).bool());
        assert_eq!(calls_to(&e, 0x0065_f6c0), vec![vec![this + 8, 10]]);
        assert_eq!(e.mem.u32(this + 0xc), 0x9000);
        assert_eq!(e.mem.u32(this + 0x10), 0x9000);
        assert_eq!(e.mem.u32(this + 0x14), 0x9000 + 60);
    }

    #[test]
    fn b046_0065f400_0065f9a0_and_0065ff00_destroy_each_element() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0065_e9a0, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0065_f400, &args![0x4000u32, 0x100u32, 0x10cu32]);
        assert_eq!(
            calls_to(&e, 0x0065_e9a0),
            vec![vec![0x4008, 0x100], vec![0x4008, 0x106]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_f9a0, &args![0x100u32, 0x100u32, 0x50u32]);
        assert!(calls_to(&e, 0x0065_e9a0).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0065_ff00, &args![0x200u32, 0x206u32, 0x50u32, 0u32]);
        assert_eq!(calls_to(&e, 0x0065_e9a0), vec![vec![0x50, 0x200]]);
    }

    #[test]
    fn b046_0065f430_tidies_the_buffer() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0065_e9a0, quiet);
        e.register(0x0064_df10, quiet);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0065_f430, &args![this]);
        assert!(calls_to(&e, 0x0064_df10).is_empty());
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1006);
        e.mem.set_u32(this + 0x14, 0x1012);
        e.call_log = Some(vec![]);
        e.call(0x0065_f430, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_e9a0), vec![vec![this + 8, 0x1000]]);
        assert_eq!(calls_to(&e, 0x0064_df10), vec![vec![this + 8, 0x1000, 3]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn b046_0065f550_0065fd10_and_0065fd40_make_iterators() {
        let mut e = Engine::new();
        e.register(0x0065_e000, quiet);
        e.register(0x0065_da80, quiet);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x100);
        e.mem.set_u32(this + 0x10, 0x200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f550, &args![this, 0x50u32]).u32(), 0x50);
        assert_eq!(e.call(0x0065_fd10, &args![this, 0x60u32]).u32(), 0x60);
        assert_eq!(e.call(0x0065_fd40, &args![this, 0x70u32]).u32(), 0x70);
        assert_eq!(calls_to(&e, 0x0065_e000), vec![vec![0x50, 0x200, this]]);
        assert_eq!(
            calls_to(&e, 0x0065_da80),
            vec![vec![0x60, 0x100, this], vec![0x70, 0x200, this]]
        );
    }

    #[test]
    fn b046_0065f7a0_0065fa30_and_0065f9e0_allocate() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (a[0] + 1).into_ret());
        e.register(BAD_ALLOC_CTOR_0044D6C0, quiet);
        e.register(CXX_THROW_EXCEPTION, quiet);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f7a0, &args![0u32, 7u32]).u32(), 8);
        assert_eq!(e.call(0x0065_fa30, &args![0u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x0065_f9e0, &args![3u32, 0u32]).u32(), 19);
        assert_eq!(e.call(0x0065_f9e0, &args![0u32, 0u32]).u32(), 1);
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        e.call_log = Some(vec![]);
        // 0xffffffff / 0x30000000 is 5, below 6: the throw, then the wrapped size.
        assert_eq!(
            e.call(0x0065_f9e0, &args![0x3000_0000u32, 0u32]).u32(),
            0x2000_0001
        );
        let thrown = calls_to(&e, CXX_THROW_EXCEPTION);
        assert_eq!(thrown.len(), 1);
        assert_eq!(thrown[0][1], BAD_ALLOC_THROW_INFO_01140EF8);
        assert_eq!(
            calls_to(&e, BAD_ALLOC_CTOR_0044D6C0),
            vec![vec![thrown[0][0], 0]]
        );
    }

    #[test]
    fn b046_0065f7f0_answers_all_ones() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0065_f7f0, &args![0u32]).u32(), 0xffff_ffff);
    }

    #[test]
    fn b046_0065fab0_copies_the_head_and_the_three_vectors() {
        let mut e = Engine::new();
        e.register(0x00ec_aebb, quiet);
        let source = e.mem.alloc(0x58);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 4, 4);
        e.mem.set_u32(source + 8, 8);
        e.mem.set_u32(source + 0xc, 0xc);
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fab0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 8), 8);
        assert_eq!(e.mem.u32(this + 0xc), 0xc);
        assert_eq!(
            calls_to(&e, 0x00ec_aebb),
            vec![vec![
                this + 0x10,
                source + 0x10,
                0x18,
                3,
                0x0065_fc10,
                0x0065_f060
            ]]
        );
    }

    fn copy_world(e: &mut Engine) {
        e.register(0x0065_de30, quiet);
        e.register(0x0065_d3a0, quiet);
        e.register(0x0065_7bc0, |_, _| 2u32.into_ret());
        e.register(0x0065_cc50, |_, _| 2u32.into_ret());
        e.register(0x0065_f7c0, |_, _| 10u32.into_ret());
        e.register(0x0065_f6c0, |_, _| 0x9000u32.into_ret());
        e.register(0x0065_da80, |e, a| {
            e.mem.set_u32(a[0], 0xb0b0);
            e.mem.set_u32(a[0] + 4, a[1]);
            a[0].into_ret()
        });
        e.register(0x0065_ff30, |_, _| 0x900cu32.into_ret());
        e.register(0x0065_ff60, |_, _| 0x900cu32.into_ret());
    }

    #[test]
    fn b046_0065fb10_copies_a_six_byte_vector() {
        let mut e = Engine::new();
        copy_world(&mut e);
        let other = e.mem.alloc(0x18);
        e.mem.set_u32(other + 0xc, 0x100);
        e.mem.set_u32(other + 0x10, 0x10c);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fb10, &args![this, other]).u32(), this);
        let slot = calls_to(&e, 0x0065_de30);
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0][1], other + 8);
        assert_eq!(calls_to(&e, 0x0065_d3a0), vec![vec![this, this]]);
        assert_eq!(calls_to(&e, 0x0065_7bc0), vec![vec![other]]);
        assert_eq!(
            calls_to(&e, 0x0065_ff30),
            vec![vec![this, 0xb0b0, 0x100, 0xb0b0, 0x10c, 0x9000]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x900c);
        assert_eq!(e.mem.u32(this + 0x14), 0x900c);
    }

    #[test]
    fn b046_0065fc10_copies_only_when_bought() {
        let mut e = Engine::new();
        copy_world(&mut e);
        e.register(0x0065_7ee0, |_, _| 0x100u32.into_ret());
        let other = e.mem.alloc(0x18);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fc10, &args![this, other]).u32(), this);
        assert_eq!(calls_to(&e, 0x0065_7ee0), vec![vec![this, 2]]);
        assert!(calls_to(&e, 0x0065_ff60).is_empty());
        e.register(0x0065_7ee0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0065_fc10, &args![this, other]);
        assert_eq!(calls_to(&e, 0x0065_ff60).len(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x900c);
    }

    #[test]
    fn b046_0065fd70_and_0065fe40_pass_through() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        assert_eq!(e.call(0x0065_fd70, &args![5u32, 6u32, 7u32]).u32(), 5);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fe40, &args![slot]).u32(), 0x4444);
        assert_eq!(calls_to(&e, UNWRAP_0043B1D0), vec![vec![slot, 0]]);
    }

    #[test]
    fn b046_0065fde0_and_00660070_construct_each_six_byte_element() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_01f0, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_fde0, &args![0x100u32, 0x10cu32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x80c);
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tag.len(), 1);
        assert_eq!(tag[0][0], tag[0][2]);
        assert_eq!(tag[0][1] + 8, tag[0][0]);
        assert_eq!(
            calls_to(&e, 0x0066_01f0),
            vec![vec![0x50, 0x800, 0x100], vec![0x50, 0x806, 0x106]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x200u32, 0x206u32, 0x900u32, 0x60u32, 0x5au32, 0u32];
        assert_eq!(e.call(0x0066_0070, &words).u32(), 0x906);
        assert_eq!(calls_to(&e, 0x0066_01f0), vec![vec![0x60, 0x900, 0x200]]);
    }

    #[test]
    fn b046_0065fea0_and_00660270_move_the_bytes() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x00ec_7ce3, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_fea0, &args![0x100u32, 0x10cu32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x80c);
        assert_eq!(
            calls_to(&e, 0x00ec_7ce3),
            vec![vec![0x800, 0xc, 0x100, 0xc]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x100u32, 0x100u32, 0x800u32, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x0066_0270, &words).u32(), 0x800);
        assert!(calls_to(&e, 0x00ec_7ce3).is_empty());
    }

    #[test]
    fn b046_0065ff90_00660210_00660340_and_00660510_reach_the_0x1c_loop() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_0300, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_ff90, &args![0x100u32, 0x138u32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x838);
        let tags = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tags.len(), 2);
        // The one-push form: the destination's address twice, then the frame's local (0).
        assert_eq!(tags[0][0], tags[0][1]);
        assert_eq!(tags[0][2], 0);
        assert_eq!(
            calls_to(&e, 0x0066_0300),
            vec![vec![0x50, 0x800, 0x100], vec![0x50, 0x81c, 0x11c]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x100u32, 0x11cu32, 0x800u32, 0x50u32, 1u32, 2u32];
        assert_eq!(e.call(0x0066_0210, &words).u32(), 0x81c);
        assert_eq!(
            e.call(0x0066_0340, &args![0x100u32, 0x100u32, 0x800u32, 0x50u32])
                .u32(),
            0x800
        );
        assert_eq!(e.call(0x0066_0510, &words).u32(), 0x81c);
    }

    #[test]
    fn b046_0065ffe0_00660230_006603a0_and_006605d0_reach_the_0x58_loop() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_0320, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_ffe0, &args![0x100u32, 0x158u32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x858);
        assert_eq!(calls_to(&e, 0x0066_0320), vec![vec![0x50, 0x800, 0x100]]);
        let words = args![0x100u32, 0x1b0u32, 0x800u32, 0x50u32, 1u32, 2u32];
        assert_eq!(e.call(0x0066_0230, &words).u32(), 0x8b0);
        assert_eq!(
            e.call(0x0066_03a0, &args![0x100u32, 0x158u32, 0x800u32, 0x50u32])
                .u32(),
            0x858
        );
        assert_eq!(e.call(0x0066_05d0, &words).u32(), 0x8b0);
    }
}
