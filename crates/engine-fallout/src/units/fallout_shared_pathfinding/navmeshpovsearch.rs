//! `fallout shared/pathfinding/navmeshpovsearch.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3B0: u32 = 0x0106_b3b0;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3D4: u32 = 0x0106_b3d4;
/// `0049faf0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_0049FAF0: u32 = 0x0049_faf0;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3E8: u32 = 0x0106_b3e8;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B3FC: u32 = 0x0106_b3fc;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B410: u32 = 0x0106_b410;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B424: u32 = 0x0106_b424;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B44C: u32 = 0x0106_b44c;
/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0106B460: u32 = 0x0106_b460;

// Translated from 006a14e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a14e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3B0);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a1700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@PAUNavMeshPOV@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a1700(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3D4);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchNode@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1890(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a18e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VNavMeshPOVSearchLink@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a18e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B3FC);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@V?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `0049faf0(this, 1)`.
pub fn fn_006a19e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B410);
    e.call(ARRAY_RELEASE_0049FAF0, &args![this, 1u32]);
}

// Translated from 006a1a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@URemoveIndex@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a1a30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B424);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAV?$AStarNode@VNavMeshPOVSearchNode@@VNavMeshPOVSearchLink@@@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2580(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B44C);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

// Translated from 006a2900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFastNavMeshVertHandle@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006a2900(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0106B460);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `006240d0` (unnamed in the engine map): the constructor `006a0480` runs on `this`.
const BASE_CTOR_006240D0: u32 = 0x0062_40d0;
/// `006815c0` (unnamed in the engine map): returns its `this` (here run as
/// the constructor of a member with nothing to construct).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `ScrapHeap::Allocate` (`00aa54a0`, engine map; `thiscall`, `RET 8`: size, alignment).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
/// The alignment word `006a1f90` passes to the scrap heap.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `00559450` (unnamed in the engine map): the word at +0 of its `this`.
const FIRST_OF_00559450: u32 = 0x0055_9450;
/// `00558dd0` (unnamed in the engine map; `thiscall`, `RET 4`).
const STEP_00558DD0: u32 = 0x0055_8dd0;
/// `00aa5410` (unnamed in the engine map; `thiscall`, `RET 4`): the set-up
/// `006a1f70` runs with 0xD000.
const SCRAP_INIT_00AA5410: u32 = 0x00aa_5410;
/// `0042f5d0` (unnamed in the engine map; cdecl, a pointer and a byte count).
const BYTES_0042F5D0: u32 = 0x0042_f5d0;
/// `004b5570` (unnamed in the engine map; cdecl, two floats): the angle of
/// a 2D vector, called with (y, x); ST0.
const ANGLE_004B5570: u32 = 0x004b_5570;
/// `00401050` (unnamed in the engine map; `RET 0x10`: first, element size,
/// count, constructor): runs a constructor over an array.
const CONSTRUCT_ARRAY_00401050: u32 = 0x0040_1050;
/// `00588c50`, the element constructor `006a0870` passes to `00401050`.
const ELEMENT_CTOR_00588C50: u32 = 0x0058_8c50;
/// `004e4470` (unnamed in the engine map; cdecl, a float): ST0 (the first
/// coordinate of the direction `006a04e0` builds).
const DIRECTION_X_004E4470: u32 = 0x004e_4470;
/// `004e44b0` (unnamed in the engine map; cdecl, a float): ST0 (the second
/// coordinate).
const DIRECTION_Y_004E44B0: u32 = 0x004e_44b0;
/// `006985f0` (unnamed in the engine map; `thiscall`, `RET 4`): copies the
/// 2D point it is given into `this`.
const POINT_SET_006985F0: u32 = 0x0069_85f0;
/// `00698620` (unnamed in the engine map; `thiscall`, `RET 8`: out, other):
/// the 2D difference `this - other` into `out`.
const POINT_MINUS_00698620: u32 = 0x0069_8620;
/// `00698680` (unnamed in the engine map): the quadrant of a 2D vector (an int).
const POINT_QUADRANT_00698680: u32 = 0x0069_8680;
/// `006c4350` (unnamed in the engine map; `thiscall`, `RET 4`): the base
/// part `006a0c10` and `006a0e50` construct first.
const BASE_SET_006C4350: u32 = 0x006c_4350;
/// `0.0` (double).
const ZERO: u32 = 0x0101_2060;
/// `2 pi` as the float closest to it, stored as a double.
const TWO_PI: u32 = 0x0101_ff48;
/// The `BSSimpleArray` slot (byte offset) that allocates a buffer for a count.
const SLOT_ALLOCATE: u32 = 0x04;
/// The `BSSimpleArray` slot (byte offset) that reallocates a buffer (old buffer, count).
const SLOT_REALLOCATE: u32 = 0x0c;
/// `00438b90` (unnamed in the engine map): whether the array is full.
const ARRAY_FULL_00438B90: u32 = 0x0043_8b90;
/// `009a3910` (unnamed in the engine map): the array's grown capacity.
const ARRAY_GROWN_CAPACITY_009A3910: u32 = 0x009a_3910;
/// `006a8500` (unnamed in the engine map): frees the array's buffer.
const ARRAY_FREE_BUFFER_006A8500: u32 = 0x006a_8500;
/// `00692480` (unnamed in the engine map; `thiscall`, `RET 0xC`: new, old,
/// count): moves elements to a new buffer.
const MOVE_ELEMENTS_00692480: u32 = 0x0069_2480;
/// `memmove` (`00ec7230`, cdecl).
const MEMMOVE: u32 = 0x00ec_7230;
/// `006e6da0` (unnamed in the engine map; cdecl, size and place): placement
/// `new` (returns the place).
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
/// `006b1d80` (unnamed in the engine map): the 0x40-byte element constructor.
const ELEMENT_CTOR_006B1D80: u32 = 0x006b_1d80;
/// `006e8f80` (unnamed in the engine map; `thiscall`, `RET 4`): the address
/// of an array's element.
const ARRAY_AT_006E8F80: u32 = 0x006e_8f80;
/// `006e2280` (unnamed in the engine map; `thiscall`, `RET 8`): the
/// 0x3C-byte element constructor with two words.
const NODE_CTOR_006E2280: u32 = 0x006e_2280;

// Translated from 006a1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the address of element `index` of a
/// `BSSimpleArray` of 12-byte elements (buffer at +4).
pub fn fn_006a1440(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0xc)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 006a1490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the address of element `index` of a
/// `BSSimpleArray` of 0x24-byte elements (buffer at +4).
pub fn fn_006a1490(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x24)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 006a1400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): forwards to `006a1490`.
pub fn fn_006a1400(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    fn_006a1490(e, this, index)
}

// Translated from 006a1b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x54.
pub fn fn_006a1b60(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x54)
}

// Translated from 006a0480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a constructor: `006240d0` on `this`, then
/// `006815c0` on +0xC. Returns `this`.
pub fn fn_006a0480(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CTOR_006240D0, &args![this]);
    e.call(IDENTITY_006815C0, &args![this.addr() + 0xc]);
    this
}

// Translated from 006a1f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two words): `ScrapHeap::Allocate` on
/// `heap` of `size` bytes with the alignment word at `010a2720`.
pub fn fn_006a1f90(e: &mut Engine, size: u32, heap: Ptr) -> u32 {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    e.call(SCRAP_HEAP_ALLOCATE, &args![heap, size, alignment])
        .u32()
}

// Translated from 0069ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at +0x68 (ST0, `FLD`).
pub fn fn_0069ef80(e: &mut Engine, this: Ptr) -> f64 {
    x87_load32(e.mem.u32(this.addr() + 0x68))
}

// Translated from 0069e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: what `00559450` returns for +0x94.
pub fn fn_0069e390(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FIRST_OF_00559450, &args![this.addr() + 0x94]).u32()
}

// Translated from 006a1420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00558dd0(this, 0)`, its result returned.
pub fn fn_006a1420(e: &mut Engine, this: Ptr) -> u32 {
    e.call(STEP_00558DD0, &args![this, 0u32]).u32()
}

// Translated from 006a1f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00aa5410(this, 0xD000)`. Returns `this`.
pub fn fn_006a1f70(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SCRAP_INIT_00AA5410, &args![this, 0xd000u32]);
    this
}

// Translated from 0069f900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): `a + b` (wrapping), `this` unread.
pub fn fn_0069f900(_e: &mut Engine, _this: Ptr, a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

// Translated from 006a28b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`; `this` unread): `0042f5d0(first,
/// count * 8)`, its result returned.
pub fn fn_006a28b0(e: &mut Engine, _this: Ptr, first: u32, count: u32) -> u32 {
    e.call(BYTES_0042F5D0, &args![first, count << 3]).u32()
}

// Translated from 006a0840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the angle of the 2D vector at `this`:
/// `004b5570(y, x)` with both floats copied through the FPU (ST0).
pub fn fn_006a0840(e: &mut Engine, this: Ptr) -> f64 {
    let x = x87_quiet_bits(e.mem.u32(this.addr()));
    let y = x87_quiet_bits(e.mem.u32(this.addr() + 4));
    e.call(ANGLE_004B5570, &args![y, x]).f64()
}

// Translated from 006a0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs two 0x14-byte elements at +4
/// (`00401050` with the constructor `00588c50`). Returns `this`.
pub fn fn_006a0870(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        CONSTRUCT_ARRAY_00401050,
        &args![this.addr() + 4, 0x14u32, 2u32, ELEMENT_CTOR_00588C50],
    );
    this
}

// Translated from 006a04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): copies the three words at `point`
/// to `this` and the float `value` (through the FPU) to +0xC. Returns `this`.
pub fn fn_006a04b0(e: &mut Engine, this: Ptr, point: Ptr, value: f32) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 4 * i, word);
    }
    e.mem
        .set_u32(this.addr() + 0xc, x87_quiet_bits(value.to_bits()));
    this
}

// Translated from 006a04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the 2D direction of `angle`: +0 from
/// `004e4470(angle)`, +4 from `004e44b0(angle)` (each ST0 stored as a
/// float). Returns `this`.
pub fn fn_006a04e0(e: &mut Engine, this: Ptr, angle: f32) -> Ptr {
    let pushed = x87_quiet_bits(angle.to_bits());
    let x = e.call(DIRECTION_X_004E4470, &args![pushed]).f64();
    e.mem.set_u32(this.addr(), x87_store32(x));
    let pushed = x87_quiet_bits(angle.to_bits());
    let y = e.call(DIRECTION_Y_004E44B0, &args![pushed]).f64();
    e.mem.set_u32(this.addr() + 4, x87_store32(y));
    this
}

// Translated from 006a05e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`): sets the point at +0 from
/// `first`, the point at +8 from `second` (`006985f0`) and the word at
/// +0x10 to `flag`. Returns `this`.
pub fn fn_006a05e0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, flag: u32) -> Ptr {
    e.call(POINT_SET_006985F0, &args![this, first]);
    e.call(POINT_SET_006985F0, &args![this.addr() + 8, second]);
    e.mem.set_u32(this.addr() + 0x10, flag);
    this
}

// Translated from 006a0620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the copy constructor of the
/// `006a05e0` object: both points (`006985f0`) and the word at +0x10.
/// Returns `this`.
pub fn fn_006a0620(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(POINT_SET_006985F0, &args![this, source]);
    e.call(
        POINT_SET_006985F0,
        &args![this.addr() + 8, source.addr() + 8],
    );
    let flag = e.mem.u32(source.addr() + 0x10);
    e.mem.set_u32(this.addr() + 0x10, flag);
    this
}

// Translated from 006a06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): orders two 2D vectors by angle.
/// With the quadrants (`00698680`, signed) of `this` and `other`: a lower
/// quadrant is true, a higher one false; in the same quadrant, for
/// quadrants 0 and 3 true when `other`'s second coordinate is not below
/// this one's (false when unordered), else true when it is not above it
/// (false when unordered).
pub fn fn_006a06f0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mine = e.call(POINT_QUADRANT_00698680, &args![this]).i32();
    let theirs = e.call(POINT_QUADRANT_00698680, &args![other]).i32();
    if mine < theirs {
        return true;
    }
    if mine > theirs {
        return false;
    }
    let this_y = x87_load32(e.mem.u32(this.addr() + 4));
    let other_y = x87_load32(e.mem.u32(other.addr() + 4));
    if mine == 0 || mine == 3 {
        other_y >= this_y
    } else {
        other_y <= this_y
    }
}

// Translated from 006a0660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): with the segment's two points (+0
/// and +8; their roles swap when the word at +0x10 is non-zero), compares
/// `point - start` against `end - start` by angle (`006a06f0`); the
/// differences are `00698620` into two locals.
pub fn fn_006a0660(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let swapped = e.mem.u32(this.addr() + 0x10) != 0;
    let start = if swapped {
        this.addr() + 8
    } else {
        this.addr()
    };
    let end = if swapped {
        this.addr()
    } else {
        this.addr() + 8
    };
    // The edge difference (EBP-0x10) and the point difference (EBP-8).
    e.with_stack(0x10, |e, locals| {
        let edge = locals.addr();
        let offset = locals.addr() + 8;
        e.call(POINT_MINUS_00698620, &args![end, edge, start]);
        e.call(POINT_MINUS_00698620, &args![point, offset, start]);
        fn_006a06f0(e, Ptr::new(offset), Ptr::new(edge))
    })
}

// Translated from 006a07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the angle of the segment's
/// direction (`(end - start)` by `00698620` with +8 and +0, then
/// `006a0840`), stored as a float `a`, scaled: with the word at +0x10 clear,
/// `(a + 2pi) * scale` for a negative `a`, else `a * scale`; with it set,
/// `-a * scale` for a negative `a`, else `(2pi - a) * scale`. Each result
/// is stored as a float and returned (ST0).
pub fn fn_006a07a0(e: &mut Engine, this: Ptr, scale: f32) -> f64 {
    let scale = x87_m32(scale.to_bits());
    let angle_bits = e.with_stack(8, |e, local| {
        e.call(POINT_MINUS_00698620, &args![this.addr() + 8, local, this]);
        let angle = fn_006a0840(e, local);
        x87_store32(angle)
    });
    let angle = x87_load32(angle_bits);
    let negative = angle < e.global::<f64>(ZERO);
    let two_pi = e.global::<f64>(TWO_PI);
    let result = if e.mem.u32(this.addr() + 0x10) == 0 {
        if negative {
            x87_mul(x87_add(angle, two_pi), scale)
        } else {
            x87_mul(angle, scale)
        }
    } else if negative {
        x87_mul(-angle, scale)
    } else {
        x87_mul(x87_sub(two_pi, angle), scale)
    };
    x87_load32(x87_store32(result))
}

// Translated from 006a0c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): `006c4350(this, base)`, then the
/// three words at `point` copied to +0xC. Returns `this`.
pub fn fn_006a0c10(e: &mut Engine, this: Ptr, base: u32, point: Ptr) -> Ptr {
    e.call(BASE_SET_006C4350, &args![this, base]);
    for i in 0..3 {
        let word = e.mem.u32(point.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

// Translated from 006a0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): the copy constructor of the
/// `006a0c10` object: `006c4350(this, source)`, then the three words at
/// `source + 0xC`. Returns `this`.
pub fn fn_006a0e50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(BASE_SET_006C4350, &args![this, source]);
    for i in 0..3 {
        let word = e.mem.u32(source.addr() + 0xc + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    this
}

// Translated from 006a0e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the 2D rotation (complex product)
/// of `this` by `other` into `out`: `(x*ox - y*oy, y*ox + x*oy)`, each
/// coordinate computed on the FPU and stored as a float in a local (made
/// by `006815c0`), then copied by `006985f0`. Returns `out`.
pub fn fn_006a0e90(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    e.with_stack(8, |e, local| {
        e.call(IDENTITY_006815C0, &args![local]);
        let x = e.mem.u32(this.addr());
        let y = e.mem.u32(this.addr() + 4);
        let ox = e.mem.u32(other.addr());
        let oy = e.mem.u32(other.addr() + 4);
        let first = x87_mul(x87_load32(x), x87_m32(ox));
        let second = x87_mul(x87_load32(y), x87_m32(oy));
        e.mem
            .set_u32(local.addr(), x87_store32(x87_sub(first, second)));
        let first = x87_mul(x87_load32(y), x87_m32(ox));
        let second = x87_mul(x87_load32(x), x87_m32(oy));
        e.mem
            .set_u32(local.addr() + 4, x87_store32(x87_add(first, second)));
        e.call(POINT_SET_006985F0, &args![out, local]);
    });
    out
}

// Translated from 006a1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`; `this` unread): unlinks `node` from
/// its list. When its +0x58 is set, the word at that address gets the
/// node's +0x5C; when +0x5C is set, that node's +0x58 gets this node's
/// +0x58; then both are cleared.
pub fn fn_006a1fb0(e: &mut Engine, _this: Ptr, node: Ptr) {
    let base = node.addr();
    let back = e.mem.u32(base + 0x58);
    if back != 0 {
        let next = e.mem.u32(base + 0x5c);
        e.mem.set_u32(back, next);
    }
    let next = e.mem.u32(base + 0x5c);
    if next != 0 {
        let back = e.mem.u32(base + 0x58);
        e.mem.set_u32(next + 0x58, back);
    }
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0);
}

// Translated from 006a2630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006a2630 - its FS:[0] exception frame (the unwind state of each placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`): constructs `count` 0x40-byte
/// elements from `first`: placement `new` (`006e6da0(0x40, place)`) and,
/// when that gives a place, `006b1d80` on it.
pub fn fn_006a2630(e: &mut Engine, _this: Ptr, first: u32, count: u32) {
    let mut index: u32 = 0;
    while index < count {
        let place = (index << 6).wrapping_add(first);
        let element = e.call(PLACEMENT_NEW_006E6DA0, &args![0x40u32, place]).u32();
        if element != 0 {
            e.call(ELEMENT_CTOR_006B1D80, &args![element]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006a2130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the initialization of a
/// `BSSimpleArray` of 0x40-byte elements: clears buffer, size and
/// capacity; the reserve is raised to `size` when smaller (unsigned); a
/// non-zero reserve allocates through vtable slot +4 and sets the
/// capacity; a non-zero size constructs the elements (`006a2630`) and sets
/// the size.
pub fn fn_006a2130(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let mut reserve = reserve;
    if size > reserve {
        reserve = size;
    }
    if reserve > 0 {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![reserve]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, reserve);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + 4);
        fn_006a2630(e, this, buffer, size);
        e.mem.set_u32(base + 8, size);
    }
}

/// The shared body of `006a21c0` and `006a2770` (a `BSSimpleArray`'s
/// buffer change to `capacity`, keeping `count` elements, `mover` moving
/// them to a new buffer).
fn array_reserve(
    e: &mut Engine,
    this: Ptr,
    capacity: u32,
    count: u32,
    mover: fn(&mut Engine, Ptr, u32, u32, u32),
) {
    let base = this.addr();
    if e.mem.u32(base + 4) != 0 {
        if count == e.mem.u32(base + 0xc) {
            let old = e.mem.u32(base + 4);
            let buffer = e.vcall(base, SLOT_REALLOCATE, &args![old, capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
        } else {
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
            let old = e.mem.u32(base + 4);
            mover(e, this, buffer, old, count);
            e.call(ARRAY_FREE_BUFFER_006A8500, &args![this]);
            e.mem.set_u32(base + 4, buffer);
        }
    } else {
        let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    }
}

fn move_by_address(e: &mut Engine, this: Ptr, new: u32, old: u32, count: u32) {
    e.call(MOVE_ELEMENTS_00692480, &args![this, new, old, count]);
}

// Translated from 006a2270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0xC`; `this` unread): moves `count`
/// 0x3C-byte elements from `source` to `destination` one `memmove` each:
/// forwards when the destination is below the source, backwards (a signed
/// index from `count - 1` down to 0) when above; nothing when equal or for
/// a zero count.
pub fn fn_006a2270(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index: u32 = 0;
        while index < count {
            let to = index.wrapping_mul(0x3c).wrapping_add(destination);
            let from = index.wrapping_mul(0x3c).wrapping_add(source);
            e.call(MEMMOVE, &args![to, from, 0x3cu32]);
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let to = (index as u32).wrapping_mul(0x3c).wrapping_add(destination);
            let from = (index as u32).wrapping_mul(0x3c).wrapping_add(source);
            e.call(MEMMOVE, &args![to, from, 0x3cu32]);
            index = index.wrapping_sub(1);
        }
    }
}

// Translated from 006a21c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): changes the buffer of a
/// `BSSimpleArray` of 0x3C-byte elements to `capacity`. Without a buffer it
/// allocates one (vtable slot +4) and sets the capacity. With one: when
/// `count` equals the capacity it reallocates in place (slot +0xC with the
/// old buffer); otherwise it allocates a new buffer, moves the elements
/// (`006a2270`), frees the old buffer (`006a8500`) and keeps the new one
/// (the capacity is left to the caller).
pub fn fn_006a21c0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    array_reserve(e, this, capacity, count, fn_006a2270);
}

// Translated from 006a2770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): as `006a21c0`, with the elements
/// moved by `00692480`.
pub fn fn_006a2770(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    array_reserve(e, this, capacity, count, move_by_address);
}

// Translated from 006a1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reserves one more element of a `BSSimpleArray`
/// of 0x3C-byte elements. When full (`00438b90`): with no capacity, a
/// buffer of 4 (vtable slot +4); otherwise the grown capacity (`009a3910`)
/// through `006a21c0` with the current size. Then the size grows by one;
/// returns the new element's index.
pub fn fn_006a1920(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.call(ARRAY_FULL_00438B90, &args![this]).u8() != 0 {
        if e.mem.u32(base + 0xc) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(base, SLOT_ALLOCATE, &args![capacity]).u32();
            e.mem.set_u32(base + 4, buffer);
            e.mem.set_u32(base + 0xc, capacity);
        } else {
            let capacity = e.call(ARRAY_GROWN_CAPACITY_009A3910, &args![this]).u32();
            let count = e.mem.u32(base + 8);
            fn_006a21c0(e, this, capacity, count);
            e.mem.set_u32(base + 0xc, capacity);
        }
    }
    let size = e.mem.u32(base + 8).wrapping_add(1);
    e.mem.set_u32(base + 8, size);
    e.mem.u32(base + 8).wrapping_sub(1)
}

// Translated from 0069f920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0069f920 - its FS:[0] exception frame (the unwind state of the placement-new construction) is not translated
/// Unnamed in the engine map (`RET 8`): appends a 0x3C-byte node to the
/// array at +0x20: reserves an index (`006a1920`), takes its element
/// (`006e8f80`), placement-`new`s it (`006e6da0(0x3C, place)`) and, when that
/// gives a place, constructs it with `first` and `second` (`006e2280`).
/// Returns the index.
pub fn fn_0069f920(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let array = Ptr::new(this.addr() + 0x20);
    let index = fn_006a1920(e, array);
    let place = e.call(ARRAY_AT_006E8F80, &args![array, index]).u32();
    let node = e.call(PLACEMENT_NEW_006E6DA0, &args![0x3cu32, place]).u32();
    if node != 0 {
        e.call(NODE_CTOR_006E2280, &args![node, first, second]);
    }
    index
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006a_1440, fn_006a1440(Ptr, u32) -> u32),
        entry!(0x006a_1490, fn_006a1490(Ptr, u32) -> u32),
        entry!(0x006a_1400, fn_006a1400(Ptr, u32) -> u32),
        entry!(0x006a_1b60, fn_006a1b60(Ptr) -> u32),
        entry!(0x006a_0480, fn_006a0480(Ptr) -> Ptr),
        entry!(0x006a_1f90, fn_006a1f90(u32, Ptr) -> u32),
        entry!(0x0069_ef80, fn_0069ef80(Ptr) -> f64),
        entry!(0x0069_e390, fn_0069e390(Ptr) -> u32),
        entry!(0x006a_1420, fn_006a1420(Ptr) -> u32),
        entry!(0x006a_1f70, fn_006a1f70(Ptr) -> Ptr),
        entry!(0x0069_f900, fn_0069f900(Ptr, u32, u32) -> u32),
        entry!(0x006a_28b0, fn_006a28b0(Ptr, u32, u32) -> u32),
        entry!(0x006a_0840, fn_006a0840(Ptr) -> f64),
        entry!(0x006a_0870, fn_006a0870(Ptr) -> Ptr),
        entry!(0x006a_04b0, fn_006a04b0(Ptr, Ptr, f32) -> Ptr),
        entry!(0x006a_04e0, fn_006a04e0(Ptr, f32) -> Ptr),
        entry!(0x006a_05e0, fn_006a05e0(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x006a_0620, fn_006a0620(Ptr, Ptr) -> Ptr),
        entry!(0x006a_06f0, fn_006a06f0(Ptr, Ptr) -> bool),
        entry!(0x006a_0660, fn_006a0660(Ptr, Ptr) -> bool),
        entry!(0x006a_07a0, fn_006a07a0(Ptr, f32) -> f64),
        entry!(0x006a_0c10, fn_006a0c10(Ptr, u32, Ptr) -> Ptr),
        entry!(0x006a_0e50, fn_006a0e50(Ptr, Ptr) -> Ptr),
        entry!(0x006a_0e90, fn_006a0e90(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006a_1fb0, fn_006a1fb0(Ptr, Ptr)),
        entry!(0x006a_2630, fn_006a2630(Ptr, u32, u32)),
        entry!(0x006a_2130, fn_006a2130(Ptr, u32, u32)),
        entry!(0x006a_2270, fn_006a2270(Ptr, u32, u32, u32)),
        entry!(0x006a_21c0, fn_006a21c0(Ptr, u32, u32)),
        entry!(0x006a_2770, fn_006a2770(Ptr, u32, u32)),
        entry!(0x006a_1920, fn_006a1920(Ptr) -> u32),
        entry!(0x0069_f920, fn_0069f920(Ptr, u32, u32) -> u32),
        entry!(0x006a_14e0, fn_006a14e0(Ptr)),
        entry!(0x006a_1700, fn_006a1700(Ptr)),
        entry!(0x006a_1890, fn_006a1890(Ptr)),
        entry!(0x006a_18e0, fn_006a18e0(Ptr)),
        entry!(0x006a_19e0, fn_006a19e0(Ptr)),
        entry!(0x006a_1a30, fn_006a1a30(Ptr)),
        entry!(0x006a_2580, fn_006a2580(Ptr)),
        entry!(0x006a_2900, fn_006a2900(Ptr)),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_006a14e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_14e0, ARRAY_VTABLE_0106B3B0, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a1700_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1700, ARRAY_VTABLE_0106B3D4, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1890_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1890, ARRAY_VTABLE_0106B3E8, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a18e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_18e0, ARRAY_VTABLE_0106B3FC, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a19e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_19e0, ARRAY_VTABLE_0106B410, ARRAY_RELEASE_0049FAF0);
    }

    #[test]
    fn fn_006a1a30_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_1a30, ARRAY_VTABLE_0106B424, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2580_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2580, ARRAY_VTABLE_0106B44C, ARRAY_RELEASE_008454F0);
    }

    #[test]
    fn fn_006a2900_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x006a_2900, ARRAY_VTABLE_0106B460, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod pov_search_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    /// The 2D helpers: copy, difference and a stand-in quadrant (0 for a
    /// non-negative first coordinate, else 1).
    fn points(e: &mut Engine) {
        e.register(POINT_SET_006985F0, |e, a| {
            let (x, y) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], x);
            e.mem.set_u32(a[0] + 4, y);
            Ret::default()
        });
        e.register(POINT_MINUS_00698620, |e, a| {
            let x = e.mem.f32(a[0]) - e.mem.f32(a[2]);
            let y = e.mem.f32(a[0] + 4) - e.mem.f32(a[2] + 4);
            e.mem.set_f32(a[1], x);
            e.mem.set_f32(a[1] + 4, y);
            Ret::default()
        });
        e.register(POINT_QUADRANT_00698680, |e, a| {
            let quadrant = u32::from(e.mem.f32(a[0]) < 0.0);
            quadrant.into_ret()
        });
    }

    #[test]
    fn element_addresses_and_small_getters() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x1000);
        assert_eq!(
            e.call(0x006a_1440, &args![array, 3u32]).u32(),
            0x1000 + 0x24
        );
        assert_eq!(
            e.call(0x006a_1490, &args![array, 3u32]).u32(),
            0x1000 + 0x6c
        );
        assert_eq!(
            e.call(0x006a_1400, &args![array, 2u32]).u32(),
            0x1000 + 0x48
        );
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0x54, 0x5454);
        assert_eq!(e.call(0x006a_1b60, &args![object]).u32(), 0x5454);
        e.mem.set_u32(object + 0x68, 0xff80_0001);
        let loaded = e.call(0x0069_ef80, &args![object]).f64();
        assert!(loaded.is_nan());
        e.mem.set_f32(object + 0x68, 1.5);
        assert_eq!(e.call(0x0069_ef80, &args![object]).f64(), 1.5);
        assert_eq!(
            e.call(0x0069_f900, &args![0u32, 0xffff_fff0u32, 0x20u32])
                .u32(),
            0x10
        );
    }

    #[test]
    fn forwarding_wrappers_pass_their_words() {
        let mut e = Engine::new();
        e.register(BASE_CTOR_006240D0, |_, _| Ret::default());
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        e.register(SCRAP_HEAP_ALLOCATE, |_, _| 0x7777u32.into_ret());
        e.register(FIRST_OF_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STEP_00558DD0, |_, _| 0x5858u32.into_ret());
        e.register(SCRAP_INIT_00AA5410, |_, _| Ret::default());
        e.register(BYTES_0042F5D0, |_, a| (a[0] + a[1]).into_ret());
        e.register(CONSTRUCT_ARRAY_00401050, |_, _| Ret::default());
        e.map(0x010a_2000, 0x1000);
        e.set_global::<u32>(SCRAP_ALIGNMENT, 4);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 0x94, 0x9494);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006a_0480, &args![this]).u32(), this);
        assert_eq!(
            e.call(0x006a_1f90, &args![0x30u32, 0x4000u32]).u32(),
            0x7777
        );
        assert_eq!(e.call(0x0069_e390, &args![this]).u32(), 0x9494);
        assert_eq!(e.call(0x006a_1420, &args![this]).u32(), 0x5858);
        assert_eq!(e.call(0x006a_1f70, &args![this]).u32(), this);
        assert_eq!(
            e.call(0x006a_28b0, &args![0u32, 0x100u32, 3u32]).u32(),
            0x118
        );
        assert_eq!(e.call(0x006a_0870, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BASE_CTOR_006240D0), vec![vec![this]]);
        assert_eq!(calls_to(&log, IDENTITY_006815C0), vec![vec![this + 0xc]]);
        assert_eq!(
            calls_to(&log, SCRAP_HEAP_ALLOCATE),
            vec![vec![0x4000, 0x30, 4]]
        );
        assert_eq!(calls_to(&log, STEP_00558DD0), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&log, SCRAP_INIT_00AA5410),
            vec![vec![this, 0xd000]]
        );
        assert_eq!(
            calls_to(&log, CONSTRUCT_ARRAY_00401050),
            vec![vec![this + 4, 0x14, 2, ELEMENT_CTOR_00588C50]]
        );
    }

    #[test]
    fn at_006a0840_asks_the_angle_with_y_first() {
        let mut e = Engine::new();
        e.register(ANGLE_004B5570, |_, a| {
            f32::from_bits(a[0]).atan2(f32::from_bits(a[1])).into_ret()
        });
        let p = point(&mut e, 0.0, 1.0);
        let angle = e.call(0x006a_0840, &args![p]).f64();
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-6);
        e.mem.set_u32(p, 0xff80_0001);
        e.call_log = Some(vec![]);
        e.call(0x006a_0840, &args![p]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ANGLE_004B5570),
            vec![vec![1.0f32.to_bits(), 0xffc0_0001]]
        );
    }

    #[test]
    fn point_record_constructors() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(BASE_SET_006C4350, |_, _| Ret::default());
        let source = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(source + 4 * i, 0x100 + i);
        }
        let this = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x006a_04b0, &args![this, source, 0xff80_0002u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 8), 0x102);
        assert_eq!(e.mem.u32(this + 0xc), 0xffc0_0002);

        let this = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x006a_05e0, &args![this, source, source + 8, 7u32])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 4), 0x101);
        assert_eq!(e.mem.u32(this + 0xc), 0x103);
        assert_eq!(e.mem.u32(this + 0x10), 7);
        let copy = e.mem.alloc(0x18);
        assert_eq!(e.call(0x006a_0620, &args![copy, this]).u32(), copy);
        for offset in [0u32, 4, 8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(copy + offset), e.mem.u32(this + offset));
        }

        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006a_0c10, &args![this, 0x55u32, source]).u32(),
            this
        );
        assert_eq!(e.call(0x006a_0e50, &args![copy, this]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x14), 0x102);
        assert_eq!(e.mem.u32(copy + 0x10), 0x101);
        assert_eq!(
            calls_to(&log, BASE_SET_006C4350),
            vec![vec![this, 0x55], vec![copy, this]]
        );
    }

    #[test]
    fn at_006a04e0_stores_both_direction_coordinates() {
        let mut e = Engine::new();
        e.register(DIRECTION_X_004E4470, |_, a| {
            f32::from_bits(a[0]).cos().into_ret()
        });
        e.register(DIRECTION_Y_004E44B0, |_, a| {
            f32::from_bits(a[0]).sin().into_ret()
        });
        let this = e.mem.alloc(8);
        assert_eq!(
            e.call(0x006a_04e0, &args![this, 0.0f32.to_bits()]).u32(),
            this
        );
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(e.mem.f32(this + 4), 0.0);
    }

    #[test]
    fn at_006a06f0_orders_by_quadrant_then_coordinate() {
        let mut e = Engine::new();
        points(&mut e);
        let call = |e: &mut Engine, a: (f32, f32), b: (f32, f32)| {
            let p = point(e, a.0, a.1);
            let q = point(e, b.0, b.1);
            e.call(0x006a_06f0, &args![p, q]).bool()
        };
        assert!(call(&mut e, (1.0, 1.0), (-1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (1.0, 1.0)));
        // Quadrant 0: true when the other's y is not below.
        assert!(call(&mut e, (1.0, 1.0), (1.0, 2.0)));
        assert!(call(&mut e, (1.0, 1.0), (1.0, 1.0)));
        assert!(!call(&mut e, (1.0, 2.0), (1.0, 1.0)));
        // Quadrant 1: true when the other's y is not above.
        assert!(call(&mut e, (-1.0, 2.0), (-1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (-1.0, 2.0)));
        // Unordered: false in both kinds of quadrant.
        assert!(!call(&mut e, (1.0, f32::NAN), (1.0, 1.0)));
        assert!(!call(&mut e, (-1.0, 1.0), (-1.0, f32::NAN)));
    }

    #[test]
    fn at_006a0660_compares_point_and_edge_directions() {
        let mut e = Engine::new();
        points(&mut e);
        let segment = e.mem.alloc(0x14);
        e.mem.set_f32(segment + 8, 1.0);
        e.mem.set_f32(segment + 0xc, 1.0);
        // Edge (0,0)->(1,1); the point (1,2) is at a larger angle.
        let p = point(&mut e, 1.0, 2.0);
        assert!(!e.call(0x006a_0660, &args![segment, p]).bool());
        let q = point(&mut e, 2.0, 1.0);
        assert!(e.call(0x006a_0660, &args![segment, q]).bool());
        // Swapped: the edge runs (1,1)->(0,0).
        e.mem.set_u32(segment + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x006a_0660, &args![segment, p]);
        let log = e.call_log.take().unwrap();
        let minus = calls_to(&log, POINT_MINUS_00698620);
        assert_eq!(minus[0][0], segment);
        assert_eq!(minus[0][2], segment + 8);
        assert_eq!(minus[1][0], p);
    }

    #[test]
    fn at_006a07a0_scales_the_angle_by_side() {
        let mut e = Engine::new();
        points(&mut e);
        e.map(0x0101_2000, 0x1_0000);
        e.set_global::<f64>(ZERO, 0.0);
        e.set_global::<f64>(TWO_PI, f64::from(std::f32::consts::TAU));
        e.register(ANGLE_004B5570, |_, a| {
            f64::from(f32::from_bits(a[0]))
                .atan2(f64::from(f32::from_bits(a[1])))
                .into_ret()
        });
        let segment = e.mem.alloc(0x14);
        let run = |e: &mut Engine, x: f32, y: f32, flag: u32| {
            e.mem.set_f32(segment + 8, x);
            e.mem.set_f32(segment + 0xc, y);
            e.mem.set_u32(segment + 0x10, flag);
            e.call(0x006a_07a0, &args![segment, 2.0f32.to_bits()]).f64()
        };
        let tau = f64::from(std::f32::consts::TAU);
        let quarter = f64::from(std::f32::consts::FRAC_PI_2);
        assert_eq!(run(&mut e, 0.0, 1.0, 0), quarter * 2.0);
        let below = f64::from((-std::f32::consts::FRAC_PI_2 as f64 + tau) as f32 * 2.0);
        assert_eq!(run(&mut e, 0.0, -1.0, 0), below);
        assert_eq!(run(&mut e, 0.0, -1.0, 1), quarter * 2.0);
        let above = f64::from(((tau - quarter) as f32) * 2.0);
        assert_eq!(run(&mut e, 0.0, 1.0, 1), above);
    }

    #[test]
    fn at_006a0e90_rotates_into_out() {
        let mut e = Engine::new();
        points(&mut e);
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        let a = point(&mut e, 1.0, 2.0);
        let b = point(&mut e, 3.0, 4.0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x006a_0e90, &args![a, out, b]).u32(), out);
        assert_eq!(e.mem.f32(out), 1.0 * 3.0 - 2.0 * 4.0);
        assert_eq!(e.mem.f32(out + 4), 2.0 * 3.0 + 1.0 * 4.0);
    }

    #[test]
    fn at_006a1fb0_unlinks_a_node() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0x60);
        let next = e.mem.alloc(0x60);
        let back = e.mem.alloc(4);
        e.mem.set_u32(node + 0x58, back);
        e.mem.set_u32(node + 0x5c, next);
        e.mem.set_u32(next + 0x58, node + 0x5c);
        e.call(0x006a_1fb0, &args![0u32, node]);
        assert_eq!(e.mem.u32(back), next);
        assert_eq!(e.mem.u32(next + 0x58), back);
        assert_eq!(e.mem.u32(node + 0x58), 0);
        assert_eq!(e.mem.u32(node + 0x5c), 0);
        // A lone node only clears its links.
        e.call(0x006a_1fb0, &args![0u32, node]);
        assert_eq!(e.mem.u32(node + 0x58), 0);
    }

    const ARRAY_SLOTS: u32 = 0x0f05_0000;
    const ALLOCATE_SLOT: u32 = 0x0f05_1000;
    const REALLOCATE_SLOT: u32 = 0x0f05_2000;

    fn array(e: &mut Engine, buffer: u32, size: u32, capacity: u32) -> u32 {
        e.put_vtable(ARRAY_SLOTS, &[0, ALLOCATE_SLOT, 0, REALLOCATE_SLOT]);
        e.register(ALLOCATE_SLOT, |_, a| (0x1000 + a[1]).into_ret());
        e.register(REALLOCATE_SLOT, |_, a| (0x2000 + a[2]).into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, ARRAY_SLOTS);
        e.mem.set_u32(this + 4, buffer);
        e.mem.set_u32(this + 8, size);
        e.mem.set_u32(this + 0xc, capacity);
        this
    }

    #[test]
    fn at_006a2630_constructs_each_placed_element() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| {
            let place = if a[1] == 0x5040 { 0 } else { a[1] };
            place.into_ret()
        });
        e.register(ELEMENT_CTOR_006B1D80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006a_2630, &args![0u32, 0x5000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW_006E6DA0),
            vec![vec![0x40, 0x5000], vec![0x40, 0x5040], vec![0x40, 0x5080]]
        );
        assert_eq!(
            calls_to(&log, ELEMENT_CTOR_006B1D80),
            vec![vec![0x5000], vec![0x5080]]
        );
    }

    #[test]
    fn at_006a2130_reserves_and_constructs() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        let this = array(&mut e, 0xdead, 0xdead, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x006a_2130, &args![this, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT), vec![vec![this, 2]]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x1002, 2, 2)
        );
        assert_eq!(calls_to(&log, PLACEMENT_NEW_006E6DA0).len(), 2);
        e.call(0x006a_2130, &args![this, 0u32, 0u32]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn at_006a2270_moves_in_the_safe_direction() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        let moves = |e: &mut Engine, to: u32, from: u32, count: u32| {
            e.call_log = Some(vec![]);
            e.call(0x006a_2270, &args![0u32, to, from, count]);
            let log = e.call_log.take().unwrap();
            calls_to(&log, MEMMOVE)
        };
        assert_eq!(
            moves(&mut e, 0x1000, 0x2000, 2),
            vec![vec![0x1000, 0x2000, 0x3c], vec![0x103c, 0x203c, 0x3c]]
        );
        assert_eq!(
            moves(&mut e, 0x2000, 0x1000, 2),
            vec![vec![0x203c, 0x103c, 0x3c], vec![0x2000, 0x1000, 0x3c]]
        );
        assert!(moves(&mut e, 0x1000, 0x1000, 2).is_empty());
        assert!(moves(&mut e, 0x1000, 0x2000, 0).is_empty());
        // A count above i32::MAX starts the backward index negative.
        assert!(moves(&mut e, 0x2000, 0x1000, 0x8000_0001).is_empty());
    }

    #[test]
    fn at_006a21c0_and_006a2770_change_the_buffer() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_FREE_BUFFER_006A8500, |_, _| Ret::default());
        e.register(MOVE_ELEMENTS_00692480, |_, _| Ret::default());
        // No buffer: allocate and set the capacity.
        let this = array(&mut e, 0, 0, 0);
        e.call(0x006a_21c0, &args![this, 8u32, 0u32]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x1008, 8));
        // Count equals the capacity: reallocate in place.
        let this = array(&mut e, 0x5000, 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_21c0, &args![this, 8u32, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, REALLOCATE_SLOT), vec![vec![this, 0x5000, 8]]);
        assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x2008, 4));
        // Otherwise: new buffer, move, free the old one.
        let this = array(&mut e, 0x5000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_21c0, &args![this, 8u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MEMMOVE).len(), 2);
        assert_eq!(calls_to(&log, MEMMOVE)[0], vec![0x1008, 0x5000, 0x3c]);
        assert_eq!(calls_to(&log, ARRAY_FREE_BUFFER_006A8500), vec![vec![this]]);
        assert_eq!(e.mem.u32(this + 4), 0x1008);
        let this = array(&mut e, 0x5000, 2, 4);
        e.call_log = Some(vec![]);
        e.call(0x006a_2770, &args![this, 8u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MOVE_ELEMENTS_00692480),
            vec![vec![this, 0x1008, 0x5000, 2]]
        );
    }

    #[test]
    fn at_006a1920_grows_when_full_and_returns_the_index() {
        let mut e = Engine::new();
        e.register(ARRAY_FULL_00438B90, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) >= e.mem.u32(a[0] + 0xc)).into_ret()
        });
        e.register(ARRAY_GROWN_CAPACITY_009A3910, |e, a| {
            (e.mem.u32(a[0] + 0xc) * 2).into_ret()
        });
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(ARRAY_FREE_BUFFER_006A8500, |_, _| Ret::default());
        let this = array(&mut e, 0, 0, 0);
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 0);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0x1004, 1, 4)
        );
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 1);
        e.mem.set_u32(this + 8, 4);
        assert_eq!(e.call(0x006a_1920, &args![this]).u32(), 4);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        // Full at its capacity: reallocated in place through slot +0xC.
        assert_eq!(e.mem.u32(this + 4), 0x2008);
    }

    #[test]
    fn at_0069f920_appends_and_constructs_a_node() {
        let mut e = Engine::new();
        e.register(ARRAY_FULL_00438B90, |_, _| Ret::default());
        e.register(ARRAY_AT_006E8F80, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x3c).into_ret()
        });
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(NODE_CTOR_006E2280, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x8000);
        e.mem.set_u32(this + 0x28, 2);
        e.mem.set_u32(this + 0x2c, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_f920, &args![this, 0x11u32, 0x22u32]).u32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ARRAY_AT_006E8F80),
            vec![vec![this + 0x20, 2]]
        );
        assert_eq!(
            calls_to(&log, PLACEMENT_NEW_006E6DA0),
            vec![vec![0x3c, 0x8078]]
        );
        assert_eq!(
            calls_to(&log, NODE_CTOR_006E2280),
            vec![vec![0x8078, 0x11, 0x22]]
        );
        // No place: no construction.
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0069_f920, &args![this, 0x11u32, 0x22u32]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, NODE_CTOR_006E2280).is_empty());
    }
}
