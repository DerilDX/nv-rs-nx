//! `NiCollision/run_00e99300` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nicollision::nicollisiondata::float_operand;

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_0126FA60: u32 = 0x0126_fa60;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00E98FF0: u32 = 0x00e9_8ff0;
const EXIT_HANDLER_ARGUMENT_00E99260: u32 = 0x00e9_9260;

// Translated from 00e99300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `0126fa60`; when it was 0, calls
/// `00aa2080(00e98ff0, 00e99260)`. Returns `this`, which it does not otherwise read.
pub fn fn_00e99300(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_0126FA60);
    e.set_global(USE_COUNT_0126FA60, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00E98FF0, EXIT_HANDLER_ARGUMENT_00E99260],
        );
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// Byte `0126fa5c`: set to 1 when `fn_00e98ff0` has started.
const OBJECTS_BUILT: u32 = 0x0126_fa5c;
/// The six function addresses `fn_00e98ff0` stores first: (global, function).
const FUNCTION_GLOBALS: [(u32, u32); 6] = [
    (0x0126_fa80, 0x00ea_7490),
    (0x0126_fa84, 0x00ea_04b0),
    (0x0126_fa90, 0x00e9_e780),
    (0x0126_fa88, 0x00ea_cb00),
    (0x0126_fa7c, 0x00ea_c170),
    (0x0126_fa8c, 0x00e9_efe0),
];
/// Words of the first 0x90-byte object (offset, value).
const FIRST_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_b9d0),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_7530),
    (0x1c, 0x00ea_0ad0),
    (0x20, 0x00ea_2030),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f620),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f760),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_e980),
    (0x64, 0x00e9_e980),
    (0x68, 0x00e9_e980),
    (0x6c, 0x0000_0000),
    (0x70, 0x00e9_e9e0),
    (0x74, 0x00e9_e980),
    (0x78, 0x00e9_cd10),
    (0x7c, 0x00e9_cde0),
    (0x80, 0x00e9_cf50),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_d070),
];
/// Words of the second 0x90-byte object (offset, value).
const SECOND_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_bb20),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_3950),
    (0x1c, 0x00ea_7d40),
    (0x20, 0x00ea_3c50),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f400),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f510),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_eae0),
    (0x64, 0x00e9_eae0),
    (0x68, 0x00e9_eae0),
    (0x6c, 0x00e9_eae0),
    (0x70, 0x00e9_eb60),
    (0x74, 0x00e9_eae0),
    (0x78, 0x00e9_d2c0),
    (0x7c, 0x00e9_d540),
    (0x80, 0x00e9_da30),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_dfa0),
];
/// Words of the first 0x18-byte object (offset, value).
const THIRD_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c240),
    (0x4, 0x00ea_2190),
    (0x8, 0x00e9_f8c0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ea70),
    (0x14, 0x00e9_d150),
];
/// Words of the second 0x18-byte object (offset, value).
const FOURTH_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c3f0),
    (0x4, 0x00ea_3d60),
    (0x8, 0x00e9_fdc0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ec10),
    (0x14, 0x00e9_e2c0),
];
/// The three function addresses `fn_00e98ff0` stores last: (global, function).
const LAST_GLOBALS: [(u32, u32); 3] = [
    (0x0126_faa4, 0x00e9_c7d0),
    (0x0126_faa8, 0x00e9_c800),
    (0x0126_faac, 0x00e9_c850),
];

/// Allocates `size` bytes (`00aa1070`) and stores the (offset, value) words in them.
fn make_object(e: &mut Engine, size: u32, words: &[(u32, u32)]) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    for (offset, value) in words {
        e.mem.set_u32(block.wrapping_add(*offset), *value);
    }
    block
}

// Translated from 00e98ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00e99300` registers; no
/// arguments, NiCollision): once only (byte `0126fa5c`), registers the class name at
/// `0109b0e8` with the creation function `00ead090` (`00a64900`), stores six function
/// addresses at `0126fa7c` to `0126fa90`, builds two 0x90-byte and two 0x18-byte objects of
/// function addresses (`00aa1070`; no check for a failed allocation, as in the exe), keeps
/// them in `0126fa94`, `0126fa98`, `0126fa9c` and `0126faa0` and stores three more
/// function addresses at `0126faa4` to `0126faac`.
pub fn fn_00e98ff0(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) != 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 1);
    e.call(REGISTER_LOADER, &args![0x0109_b0e8u32, 0x00ea_d090u32]);
    for (address, function) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, function);
    }
    let first = make_object(e, 0x90, &FIRST_OBJECT);
    let second = make_object(e, 0x90, &SECOND_OBJECT);
    let third = make_object(e, 0x18, &THIRD_OBJECT);
    let fourth = make_object(e, 0x18, &FOURTH_OBJECT);
    e.mem.set_u32(0x0126_fa98, second);
    e.mem.set_u32(0x0126_fa94, first);
    e.mem.set_u32(0x0126_fa9c, third);
    e.mem.set_u32(0x0126_faa0, fourth);
    for (address, function) in LAST_GLOBALS {
        e.mem.set_u32(address, function);
    }
}

/// `00a63520` (`cdecl`, one word, unit `NiMain/nistream.cpp`, no Xbox name): called by
/// `fn_00e99260` with the class name `fn_00e98ff0` registered.
const RELEASE_LOADER: u32 = 0x00a6_3520;
/// `00aa10f0` (`cdecl`, one word, no Xbox name): called on each of the four blocks
/// `fn_00e98ff0` built (by its use here the free paired with `00aa1070`; unconfirmed).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// The four object globals `fn_00e98ff0` fills, in the order `fn_00e99260` frees them.
const OBJECT_GLOBALS: [u32; 4] = [0x0126_fa94, 0x0126_fa98, 0x0126_fa9c, 0x0126_faa0];

// Translated from 00e99260 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the exit handler `fn_00e99300` registers, no arguments,
/// `cdecl`): undoes `fn_00e98ff0` once (byte `0126fa5c` set): clears the byte, calls
/// `00a63520` with the class name `0109b0e8`, clears the six function globals, hands the
/// four objects (`0126fa94`, `0126fa98`, `0126fa9c`, `0126faa0`) to `00aa10f0` one by one
/// and clears those four and the three last globals (`0126faa4` to `0126faac`).
pub fn fn_00e99260(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) == 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 0);
    e.call(RELEASE_LOADER, &args![0x0109_b0e8u32]);
    let first = e.mem.u32(OBJECT_GLOBALS[0]);
    for (address, _) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    e.call(FREE_BLOCK, &args![first]);
    for address in &OBJECT_GLOBALS[1..] {
        let block = e.mem.u32(*address);
        e.call(FREE_BLOCK, &args![block]);
    }
    for address in OBJECT_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    for (address, _) in LAST_GLOBALS {
        e.mem.set_u32(address, 0);
    }
}

/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00e98aa0` (`thiscall`, one word, no Xbox name): run on the element `fn_00e99330` hands out.
const ELEMENT_SET: u32 = 0x00e9_8aa0;
/// `00a90180` (`thiscall`, one word: the new capacity; unit `NiMain/nioldscreenspace.cpp` in
/// the engine map, no Xbox name): grows the pool's pointer array (by its use here).
const ARRAY_GROW: u32 = 0x00a9_0180;
/// `00ba3aa0` (`thiscall`, (index, address of the pointer), no Xbox name): stores a pointer
/// into the pool's array (by its use here).
const ARRAY_STORE: u32 = 0x00ba_3aa0;

// Translated from 00e99330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): hands out the next element of
/// a pool of 0x44-byte objects. The pool keeps its pointer array at +0x4, the array's
/// capacity (`u16`) at +0x8, its length (`u16`) at +0xA, the growth step (`u16`) at +0xE
/// and the count handed out at +0x10. While the count is below the length, the element at
/// that index gets `value` through `00e98aa0`. Otherwise a new 0x44-byte block comes from
/// the memory manager (when it is not null: words +0x0 and +0x4 zeroed, the floats at
/// +0x34 to +0x40 set to 0.0, then `00e98aa0` with `value`); when the length has reached
/// the capacity the array grows to step + length (`00a90180`), and the new pointer (0 when
/// the allocation failed) is stored at index length (`00ba3aa0`). Either way the count goes
/// up by one and the element is answered.
pub fn fn_00e99330(e: &mut Engine, this: u32, value: u32) -> u32 {
    let length = u32::from(e.mem.u16(this + 0xa));
    let handed_out = e.mem.u32(this + 0x10);
    if handed_out < length {
        let array = e.mem.u32(this + 0x4);
        let element = e.mem.u32(array.wrapping_add(handed_out.wrapping_mul(4)));
        e.call(ELEMENT_SET, &args![element, value]);
        let count = e.mem.u32(this + 0x10);
        e.mem.set_u32(this + 0x10, count.wrapping_add(1));
        return element;
    }
    let element = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
        .u32();
    if element != 0 {
        e.mem.set_u32(element, 0);
        e.mem.set_u32(element + 0x4, 0);
        for offset in [0x34, 0x38, 0x3c, 0x40] {
            e.mem.set_f32(element + offset, 0.0);
        }
        e.call(ELEMENT_SET, &args![element, value]);
    }
    let capacity = u32::from(e.mem.u16(this + 0x8));
    let length = u32::from(e.mem.u16(this + 0xa));
    if length >= capacity {
        let step = u32::from(e.mem.u16(this + 0xe));
        e.call(ARRAY_GROW, &args![this, step + length]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(ARRAY_STORE, &args![this, length, slot]);
    });
    let count = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0x10, count.wrapping_add(1));
    element
}

/// The bound test of a pick ray (`00eada70`, cdecl: the object, the ray origin and direction).
const RAY_HITS_BOUND: u32 = 0x00ea_da70;
/// The pick of a tri-based geometry (`00e9b4f0`, cdecl, the same four words).
const PICK_GEOMETRY: u32 = 0x00e9_b4f0;
/// `0043b300` (cdecl: an `NiRTTI` record, an object; no Xbox name): whether the object's class
/// is that one or derives from it (by its use here).
const IS_KIND_OF: u32 = 0x0043_b300;
/// The two flags (bytes) that let the pick take objects of the classes whose `NiRTTI` records
/// are at `011f4a48` and `011f5f64` by their world translation.
const PICK_CLASS_FLAG_0126FA65: u32 = 0x0126_fa65;
const PICK_CLASS_FLAG_0126FA64: u32 = 0x0126_fa64;
const PICK_CLASS_RTTI_011F4A48: u32 = 0x011f_4a48;
const PICK_CLASS_RTTI_011F5F64: u32 = 0x011f_5f64;
/// The `NiRTTI` record at `011f5eb4`, the class `fn_00e9c6f0` hands to `fn_00e9c680` (a node
/// with an active child index at +0xb0, `NiSwitchNode::m_iIndex` by the Xbox PDB's layout).
const SWITCH_NODE_RTTI_011F5EB4: u32 = 0x011f_5eb4;
/// `00439ef0` (thiscall on a point: the out point, the other point): the difference into the
/// out point; answers it.
const POINT_MINUS_POINT: u32 = 0x0043_9ef0;
/// `00439e90` (thiscall on a point: the out point, the other point): the sum; answers it.
const POINT_PLUS_POINT: u32 = 0x0043_9e90;
/// `NiPoint3::Dot` (`004b6190`, thiscall on one point, the other point), in ST0.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// `00457990` (thiscall on a point): its length, in ST0.
const POINT_LENGTH: u32 = 0x0045_7990;
/// `0045bb20` (thiscall on a point: the out point, a float): the scaled point; answers it.
const POINT_TIMES_SCALAR: u32 = 0x0045_bb20;
/// The `float` 0.0 at `01011d78` the ray's direction dot product is compared with.
const ZERO_01011D78: u32 = 0x0101_1d78;

/// Whether the pick is a "find first" that already has a result: pick type (+0) 1, sort type
/// (+4) 1 and a valid result count (+0x28) other than 0.
fn pick_is_done(e: &Engine, pick: u32) -> bool {
    e.mem.u32(pick) == 1 && e.mem.u32(pick + 4) == 1 && e.mem.u32(pick + 0x28) != 0
}

// Translated from 00e9c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the ray origin and direction, the `NiPick`, the object):
/// the recursive pick of one scene graph object. False for a null object, or an app-culled one
/// (flags +0x30 bit 0) when the pick observes the cull flag (+0x11). With `node` the object's
/// `IsNode` (virtual slot +0xc): false when the node's flags have bit 21; false when the ray
/// misses the object's bound (`00eada70`) unless the node's flags have bit 13. Then a node is
/// picked through its children (`fn_00e9c6f0`), a tri-based geometry (`IsTriBasedGeom`, slot
/// +0x1c) through `00e9b4f0`. Otherwise, for an object of one of two classes (each enabled by
/// its flag byte, `0126fa65` with the `NiRTTI` at `011f4a48` first, then `0126fa64` with
/// `011f5f64`; `0043b300`): when the vector from the origin to the object's world translation
/// (+0x8c, `00439ef0`) is not behind the ray (`NiPoint3::Dot`, compared with the float at
/// `01011d78`, unordered counts as in front), a record is added (`fn_00e99330` on the results at
/// pick +0x18) with that vector's length (`00457990`) as its distance; for the second class,
/// with an intersect type (+8) of 1, its point (+8) is the origin plus the direction times the
/// length (`0045bb20`, `00439e90`). Those cases answer false.
pub fn fn_00e9c490(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, object: Ptr) -> bool {
    if object.is_null() {
        return false;
    }
    let p = pick.addr();
    let o = object.addr();
    if e.mem.u8(p + 0x11) != 0 && e.mem.u8(o + 0x30) & 1 != 0 {
        return false;
    }
    let node = e.vcall(o, 0xc, &args![]).u32();
    if node != 0 && (e.mem.u32(node + 0x30) >> 21) & 1 != 0 {
        return false;
    }
    let hit = e
        .call(RAY_HITS_BOUND, &args![object, origin, direction])
        .bool();
    if !hit && !(node != 0 && (e.mem.u32(node + 0x30) >> 13) & 1 != 0) {
        return false;
    }
    if node != 0 {
        return fn_00e9c6f0(e, origin, direction, pick, object);
    }
    if e.vcall(o, 0x1c, &args![]).u32() != 0 {
        return e
            .call(PICK_GEOMETRY, &args![origin, direction, pick, object])
            .bool();
    }
    let first_class = e.mem.u8(PICK_CLASS_FLAG_0126FA65) != 0
        && e.call(IS_KIND_OF, &args![PICK_CLASS_RTTI_011F4A48, object])
            .bool();
    let second_class = !first_class
        && e.mem.u8(PICK_CLASS_FLAG_0126FA64) != 0
        && e.call(IS_KIND_OF, &args![PICK_CLASS_RTTI_011F5F64, object])
            .bool();
    if !first_class && !second_class {
        return false;
    }
    e.with_stack(0x18, |e, locals| {
        let offset = locals.addr();
        let sum = offset + 0xc;
        e.call(
            POINT_MINUS_POINT,
            &args![o.wrapping_add(0x8c), offset, origin],
        );
        let along = e.call(NI_POINT3_DOT, &args![direction, offset]).f64();
        if along < float_operand(e, ZERO_01011D78) {
            return false;
        }
        if first_class {
            let length = x87_store32(e.call(POINT_LENGTH, &args![offset]).f64());
            let record = fn_00e99330(e, p + 0x18, o);
            e.mem
                .set_u32(record.wrapping_add(0x14), x87_quiet_bits(length));
            return false;
        }
        let record = fn_00e99330(e, p + 0x18, o);
        let length = x87_quiet_bits(x87_store32(e.call(POINT_LENGTH, &args![offset]).f64()));
        e.mem.set_u32(record.wrapping_add(0x14), length);
        if e.mem.u32(p + 8) == 1 {
            let scaled = e
                .call(POINT_TIMES_SCALAR, &args![direction, offset, length])
                .u32();
            let point = e.call(POINT_PLUS_POINT, &args![origin, sum, scaled]).u32();
            for word in [0u32, 4, 8] {
                let value = e.mem.u32(point.wrapping_add(word));
                e.mem.set_u32(record.wrapping_add(8 + word), value);
            }
        }
        false
    })
}

// Translated from 00e9c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the pick traversal's four words): the pick of a switch
/// node (the active index at +0xb0, `NiSwitchNode::m_iIndex` by the Xbox PDB's layout). False for
/// a negative index; true when the pick is a finished "find first" (`pick_is_done`); otherwise
/// the active child (the children's `m_pBase` at +0xa0, `m_usSize` at +0xa6; an index past the
/// size or a null child answers false) is picked (`fn_00e9c490`) and its answer returned.
pub fn fn_00e9c680(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, node: Ptr) -> bool {
    let n = node.addr();
    let index = e.mem.u32(n + 0xb0) as i32;
    if index < 0 {
        return false;
    }
    if pick_is_done(e, pick.addr()) {
        return true;
    }
    if index >= i32::from(e.mem.u16(n + 0xa6)) {
        return false;
    }
    let children = e.mem.u32(n + 0xa0);
    let child = e
        .mem
        .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
    if child == 0 {
        return false;
    }
    fn_00e9c490(e, origin, direction, pick, Ptr::new(child))
}

// Translated from 00e9c6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the pick traversal's four words): the pick of a node. A
/// pick of intersect type (+8) 0 and pick type (+0) 1 just adds the node as a record
/// (`fn_00e99330`) and answers true. A node whose class chain (`GetRTTI`, slot +8, then each
/// record's parent at +4) reaches the record at `011f5eb4` is picked as a switch node
/// (`fn_00e9c680`). Otherwise every non-null child (`m_pBase` +0xa0, `m_usSize` +0xa6, read
/// again each step) is picked in order, answering true as soon as the pick is a finished "find
/// first" and otherwise whether any child answered true. A null node is read through (the exe
/// faults there).
pub fn fn_00e9c6f0(e: &mut Engine, origin: Ptr, direction: Ptr, pick: Ptr, node: Ptr) -> bool {
    let p = pick.addr();
    if e.mem.u32(p + 8) == 0 && e.mem.u32(p) == 1 {
        fn_00e99330(e, p + 0x18, node.addr());
        return true;
    }
    let n = node.addr();
    if n != 0 {
        let mut rtti = e.vcall(n, 8, &args![]).u32();
        while rtti != 0 {
            if rtti == SWITCH_NODE_RTTI_011F5EB4 {
                return fn_00e9c680(e, origin, direction, pick, node);
            }
            rtti = e.mem.u32(rtti.wrapping_add(4));
        }
    }
    let mut found = false;
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(n.wrapping_add(0xa6))) {
        if pick_is_done(e, p) {
            return true;
        }
        let children = e.mem.u32(n.wrapping_add(0xa0));
        let child = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
        if child != 0 && fn_00e9c490(e, origin, direction, pick, Ptr::new(child)) {
            found = true;
        }
        index += 1;
    }
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_c490, fn_00e9c490(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_c680, fn_00e9c680(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_c6f0, fn_00e9c6f0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e9_9300, fn_00e99300(Ptr) -> Ptr),
        entry!(0x00e9_8ff0, fn_00e98ff0()),
        entry!(0x00e9_9260, fn_00e99260()),
        entry!(0x00e9_9330, fn_00e99330(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e99260_releases_once_and_clears_the_globals() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(RELEASE_LOADER, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // Not built: nothing.
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        e.mem.set_u8(OBJECTS_BUILT, 1);
        for (i, address) in (0x5000u32..).zip(OBJECT_GLOBALS) {
            e.mem.set_u32(address, i);
        }
        for (address, f) in FUNCTION_GLOBALS.iter().chain(LAST_GLOBALS.iter()) {
            e.mem.set_u32(*address, *f);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 0);
        assert_eq!(calls_to(&e, RELEASE_LOADER), vec![vec![0x0109_b0e8]]);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![vec![0x5000], vec![0x5001], vec![0x5002], vec![0x5003]]
        );
        for address in (0x0126_fa7c..=0x0126_faac).step_by(4) {
            assert_eq!(e.mem.u32(address), 0, "{address:08x}");
        }
    }

    #[test]
    fn fn_00e99330_reuses_then_allocates_and_grows() {
        let mut e = Engine::new();
        let pool = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let reused = e.mem.alloc(0x44);
        e.mem.set_u32(pool + 0x4, array);
        e.mem.set_u32(array, reused);
        e.mem.set_u16(pool + 0x8, 2);
        e.mem.set_u16(pool + 0xa, 1);
        e.mem.set_u16(pool + 0xe, 3);
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(ARRAY_GROW, |_, _| Ret::default());
        let fresh = e.mem.alloc(0x44);
        e.mem.set_u32(fresh, 0xffff_ffff);
        e.mem.set_u32(fresh + 0x40, 0xffff_ffff);
        e.register_double(MEMORY_ALLOCATE, move |_, _| fresh.into_ret());
        e.register_double(ARRAY_STORE, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), fresh);
            Ret::default()
        });

        // Count 0 < length 1: the element at index 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 9]).u32(), reused);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![reused, 9]]);
        assert_eq!(e.mem.u32(pool + 0x10), 1);

        // Count 1 == length 1 < capacity 2: a new block, stored at index 1, no growth.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), fresh);
        assert_eq!(e.mem.u32(fresh), 0);
        assert_eq!(e.mem.u32(fresh + 0x40), 0);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![fresh, 7]]);
        assert!(calls_to(&e, ARRAY_GROW).is_empty());
        let stores = calls_to(&e, ARRAY_STORE);
        assert_eq!(stores.len(), 1);
        assert_eq!(&stores[0][..2], &[pool, 1]);
        assert_eq!(e.mem.u32(pool + 0x10), 2);

        // Length 2 == capacity 2: grows to 3 + 2 first; a failed allocation stores 0.
        e.mem.set_u16(pool + 0xa, 2);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_STORE, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), 0);
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![pool, 5]]);
        assert!(calls_to(&e, ELEMENT_SET).is_empty());
        assert_eq!(e.mem.u32(pool + 0x10), 3);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9300, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00e9_8ff0, 0x00e9_9260]]
        );
        e.call(0x00e9_9300, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_four_objects_once() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        let loaders: Vec<_> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == REGISTER_LOADER)
            .collect();
        assert_eq!(
            loaders,
            vec![(REGISTER_LOADER, vec![0x0109_b0e8, 0x00ea_d090])]
        );
        let first = e.mem.u32(0x0126_fa94);
        let second = e.mem.u32(0x0126_fa98);
        let third = e.mem.u32(0x0126_fa9c);
        let fourth = e.mem.u32(0x0126_faa0);
        assert_eq!(e.mem.u32(first), 0x00ea_b9d0);
        assert_eq!(e.mem.u32(first + 0x60), 0x00e9_e980);
        assert_eq!(e.mem.u32(first + 0x8c), 0x00e9_d070);
        assert_eq!(e.mem.u32(second), 0x00ea_bb20);
        assert_eq!(e.mem.u32(second + 0x60), 0x00e9_eae0);
        assert_eq!(e.mem.u32(second + 0x70), 0x00e9_eb60);
        assert_eq!(e.mem.u32(third), 0x00ea_c240);
        assert_eq!(e.mem.u32(third + 0x14), 0x00e9_d150);
        assert_eq!(e.mem.u32(fourth), 0x00ea_c3f0);
        assert_eq!(e.mem.u32(fourth + 0x14), 0x00e9_e2c0);
        assert_eq!(e.mem.u32(0x0126_fa80), 0x00ea_7490);
        assert_eq!(e.mem.u32(0x0126_faac), 0x00e9_c850);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}

#[cfg(test)]
mod n034_pick_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// Test functions for the virtual slots: answer 0, answer the object itself, answer 1.
    const ANSWER_ZERO: u32 = 0x00ab_1000;
    const ANSWER_SELF: u32 = 0x00ab_1001;
    const ANSWER_ONE: u32 = 0x00ab_1002;
    const ANSWER_SWITCH_RTTI: u32 = 0x00ab_1003;

    fn doubles(e: &mut Engine) {
        e.register(ANSWER_ZERO, |_, _| Ret::default());
        e.register(ANSWER_SELF, |_, a| a[0].into_ret());
        e.register(ANSWER_ONE, |_, _| 1u32.into_ret());
        e.register(ANSWER_SWITCH_RTTI, |_, _| {
            SWITCH_NODE_RTTI_011F5EB4.into_ret()
        });
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(RAY_HITS_BOUND, |_, _| 1u32.into_ret());
        e.map(0x011f_5000, 0x1000);
        e.map(0x0126_f000, 0x1000);
    }

    /// An object whose `GetRTTI`, `IsNode` and `IsTriBasedGeom` are the given functions.
    fn object(e: &mut Engine, rtti: u32, is_node: u32, is_geometry: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 8, rtti);
        e.mem.set_u32(vtable + 0xc, is_node);
        e.mem.set_u32(vtable + 0x1c, is_geometry);
        let o = e.mem.alloc(0xc0);
        e.mem.set_u32(o, vtable);
        o
    }

    /// A pick (type, sort, intersect) with a results pool of four ready elements at +0x18.
    fn pick(e: &mut Engine, kind: u32, sort: u32, intersect: u32) -> (u32, Vec<u32>) {
        let p = e.mem.alloc(0x40);
        e.mem.set_u32(p, kind);
        e.mem.set_u32(p + 4, sort);
        e.mem.set_u32(p + 8, intersect);
        let array = e.mem.alloc(0x10);
        let mut records = vec![];
        for i in 0..4 {
            let r = e.mem.alloc(0x44);
            e.mem.set_u32(array + 4 * i, r);
            records.push(r);
        }
        e.mem.set_u32(p + 0x18 + 4, array);
        e.mem.set_u16(p + 0x18 + 0xa, 4);
        (p, records)
    }

    fn with_children(e: &mut Engine, node: u32, children: &[u32]) {
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *c);
        }
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
    }

    #[test]
    fn traversal_rejects_null_culled_and_missed_objects() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (p, _) = pick(&mut e, 0, 0, 1);
        assert!(!fn_00e9c490(
            &mut e,
            Ptr::new(0x10),
            Ptr::new(0x20),
            Ptr::new(p),
            Ptr::NULL
        ));
        let geometry = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        e.mem.set_u32(geometry + 0x30, 1);
        e.mem.set_u8(p + 0x11, 1);
        e.register(PICK_GEOMETRY, |_, _| 1u32.into_ret());
        let r = e.call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry]);
        assert!(!r.bool());
        e.mem.set_u8(p + 0x11, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry])
            .bool());
        assert_eq!(
            calls_to(&e, PICK_GEOMETRY),
            vec![vec![0x10, 0x20, p, geometry]]
        );
        assert_eq!(
            calls_to(&e, RAY_HITS_BOUND),
            vec![vec![geometry, 0x10, 0x20]]
        );
        // A missed bound stops the pick.
        e.register(RAY_HITS_BOUND, |_, _| Ret::default());
        assert!(!e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, geometry])
            .bool());
        // A node with flag bit 21 is skipped before the bound test; bit 13 picks it anyway.
        let node = object(&mut e, ANSWER_ZERO, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[geometry]);
        e.mem.set_u32(node + 0x30, 1 << 21);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, node])
            .bool());
        assert!(calls_to(&e, RAY_HITS_BOUND).is_empty());
        e.mem.set_u32(node + 0x30, 1 << 13);
        e.mem.set_u32(geometry + 0x30, 0);
        e.register_double(RAY_HITS_BOUND, move |_, a| (a[0] == geometry).into_ret());
        assert!(e
            .call(0x00e9_c490, &args![0x10u32, 0x20u32, p, node])
            .bool());
    }

    #[test]
    fn nodes_pick_each_child_and_stop_when_done() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(PICK_GEOMETRY, |e, a| {
            // The pick finds a result on its first geometry.
            e.mem.set_u32(a[2] + 0x28, 1);
            1u32.into_ret()
        });
        let first = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let second = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let node = object(&mut e, ANSWER_ZERO, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[first, 0, second]);
        // Find all: every child.
        let (p, _) = pick(&mut e, 0, 0, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, PICK_GEOMETRY).len(), 2);
        // Find first, no sort: stops after the first result.
        let (p, _) = pick(&mut e, 1, 1, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, PICK_GEOMETRY).len(), 1);
        // Bound-only find first: the node itself becomes a record.
        let (p, r) = pick(&mut e, 1, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]).bool());
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![r[0], node]]);
        assert!(calls_to(&e, PICK_GEOMETRY).is_empty());
    }

    #[test]
    fn switch_nodes_pick_only_the_active_child() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(PICK_GEOMETRY, |_, a| (a[3] & 0xff).into_ret());
        let first = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let second = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ONE);
        let node = object(&mut e, ANSWER_SWITCH_RTTI, ANSWER_SELF, ANSWER_ZERO);
        with_children(&mut e, node, &[first, second]);
        let (p, _) = pick(&mut e, 0, 0, 1);
        e.mem.set_u32(node + 0xb0, 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_c6f0, &args![1u32, 2u32, p, node]);
        assert_eq!(calls_to(&e, PICK_GEOMETRY), vec![vec![1, 2, p, second]]);
        for index in [-1i32, 2] {
            e.mem.set_u32(node + 0xb0, index as u32);
            e.call_log = Some(vec![]);
            assert!(!e.call(0x00e9_c680, &args![1u32, 2u32, p, node]).bool());
            assert!(calls_to(&e, PICK_GEOMETRY).is_empty());
        }
        // A finished find first answers true before the index is checked against the size.
        let (p, _) = pick(&mut e, 1, 1, 1);
        e.mem.set_u32(p + 0x28, 1);
        assert!(e.call(0x00e9_c680, &args![1u32, 2u32, p, node]).bool());
    }

    #[test]
    fn class_picks_add_a_record_in_front_of_the_ray() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(IS_KIND_OF, |_, a| {
            (a[0] == PICK_CLASS_RTTI_011F5F64).into_ret()
        });
        e.register(POINT_MINUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(NI_POINT3_DOT, |e, a| Ret {
            st0: (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i) * e.mem.f32(a[1] + 4 * i)))
                .sum(),
            ..Ret::default()
        });
        e.register(POINT_LENGTH, |e, a| Ret {
            st0: (0..3)
                .map(|i| f64::from(e.mem.f32(a[0] + 4 * i).powi(2)))
                .sum::<f64>()
                .sqrt(),
            ..Ret::default()
        });
        e.register(POINT_TIMES_SCALAR, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * s;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_PLUS_POINT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.map(0x0101_1000, 0x1000);
        let leaf = object(&mut e, ANSWER_ZERO, ANSWER_ZERO, ANSWER_ZERO);
        e.mem.set_f32(leaf + 0x8c + 8, 4.0);
        let origin = e.mem.alloc(12);
        let up = e.mem.alloc(12);
        e.mem.set_f32(up + 8, 1.0);
        let down = e.mem.alloc(12);
        e.mem.set_f32(down + 8, -1.0);
        // No class flag: nothing.
        let (p, r) = pick(&mut e, 0, 0, 1);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 0);
        // The second class: a record 4 along the ray, with its point.
        e.mem.set_u8(PICK_CLASS_FLAG_0126FA64, 1);
        e.mem.set_u8(PICK_CLASS_FLAG_0126FA65, 1);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 1);
        assert_eq!(e.mem.f32(r[0] + 0x14), 4.0);
        assert_eq!(e.mem.f32(r[0] + 0x10), 4.0);
        // Behind the ray: no record.
        assert!(!e.call(0x00e9_c490, &args![origin, down, p, leaf]).bool());
        assert_eq!(e.mem.u32(p + 0x18 + 0x10), 1);
        // The first class: the distance only.
        e.register(IS_KIND_OF, |_, a| {
            (a[0] == PICK_CLASS_RTTI_011F4A48).into_ret()
        });
        e.mem.set_u32(r[1] + 0x10, 0x1234);
        assert!(!e.call(0x00e9_c490, &args![origin, up, p, leaf]).bool());
        assert_eq!(e.mem.f32(r[1] + 0x14), 4.0);
        assert_eq!(e.mem.u32(r[1] + 0x10), 0x1234);
    }
}
