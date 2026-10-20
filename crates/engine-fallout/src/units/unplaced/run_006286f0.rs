//! `(unplaced)/run_006286f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004a3c90` (thiscall, source): copies four words (a vector).
const VECTOR_COPY: u32 = 0x004a_3c90;
/// `00458b20`: the address of the vector at `01267e30`.
const VECTOR_AT_01267E30: u32 = 0x0045_8b20;
/// `004ae750` (thiscall): null for a null `this`, otherwise `00620b80`'s
/// result (engine map name `bhkCharacterProxy::operator*`, unconfirmed).
const OBJECT_PROXY: u32 = 0x004a_e750;
/// `00a29680`: an empty function (a lone `RET`).
const EMPTY_FUNCTION: u32 = 0x00a2_9680;
/// `00984230` (thiscall): `this + 0xb0`.
const BOUNDS_AT_B0: u32 = 0x0098_4230;
/// `006281a0` (thiscall, source): copies the two vectors of a box (min at
/// +0, max at +0x10, `004a3f10` each).
const BOX_COPY: u32 = 0x0062_81a0;
/// `00458a60` (thiscall: out mask, other): per-lane `this <= other` mask.
const LANES_LESS_EQUAL: u32 = 0x0045_8a60;
/// `00458990` (thiscall, lane bits): whether every lane in the bits is set
/// in the mask (`MOVMSKPS`).
const MASK_ALL_SET: u32 = 0x0045_8990;
/// `00622570` (thiscall, byte): stores the byte at `this` and returns
/// `this`.
const STORE_BYTE: u32 = 0x0062_2570;
/// `00437c10` (thiscall): the box constructor (two vector constructors).
const BOX_CONSTRUCT: u32 = 0x0043_7c10;
/// `006815c0` (thiscall): the vector constructor (returns `this`).
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
/// `004589c0` (thiscall, float): sets all four lanes to the float.
const VECTOR_SPLAT: u32 = 0x0045_89c0;
/// `00628640` (thiscall: a, b): lane-wise maximum of two vectors.
const VECTOR_MAX: u32 = 0x0062_8640;
/// `006285e0` (thiscall: a, b): lane-wise minimum of two vectors.
const VECTOR_MIN: u32 = 0x0062_85e0;
/// `004a3f10` (thiscall, source): copies a vector.
const VECTOR_SET: u32 = 0x004a_3f10;
/// `004543c0` (thiscall): the cell's object (`0041b9a0` or `00451010`).
const CELL_OBJECT: u32 = 0x0045_43c0;
/// `00541c80` (thiscall): null for a null `this`, otherwise `00541cb0`.
const CELL_OBJECT_DATA: u32 = 0x0054_1c80;
/// The float `004589c0` sets the maximum to before the loop (`0104e148`).
const LOWEST_FLOAT: u32 = 0x0104_e148;
/// The float `004589c0` sets the minimum to before the loop (`0104da48`).
const HIGHEST_FLOAT: u32 = 0x0104_da48;
/// `00726070` (thiscall): the word at +4 (an array's count).
const ARRAY_COUNT: u32 = 0x0072_6070;
/// `00629ab0` (thiscall, index): the address of element `index` (data at
/// +0, four bytes each).
const ARRAY_ELEMENT: u32 = 0x0062_9ab0;
/// `00460140` (thiscall): `this + 0x10`.
const PLUS_10: u32 = 0x0046_0140;
/// `00559450` (thiscall): the word at +0.
const WORD_AT_0: u32 = 0x0055_9450;
/// `006286d0` (thiscall): the word at +0x58 (engine map name
/// `LowProcess::GetNumberOfItemsActivated`, a folded body).
const WORD_AT_58: u32 = 0x0062_86d0;
/// `007d6bb0` (thiscall): `this + 0x14`.
const PLUS_14: u32 = 0x007d_6bb0;
/// `005609f0` (thiscall): whether the byte at `this` is non-zero.
const BYTE_SET: u32 = 0x0056_09f0;
/// `0043b540` (thiscall): a word of the object (passed to `008c71b0`).
const OBJECT_TYPE_WORD: u32 = 0x0043_b540;
/// `008c71b0` (thiscall, word): stores the word at +0.
const STORE_WORD: u32 = 0x008c_71b0;
/// `0043b4d0` (thiscall): reads the stored word's type.
const WORD_TYPE: u32 = 0x0043_b4d0;
/// `0062b4e0` `TESHavokUtilities::FindCollidableRef` (Xbox PDB name in the
/// engine map; cdecl, one word).
const FIND_COLLIDABLE_REF: u32 = 0x0062_b4e0;
/// `006295e0` `LoadedAreaBound::WarpActor` (Xbox PDB name in the engine
/// map; thiscall, reference).
const WARP_ACTOR: u32 = 0x0062_95e0;
/// `009611e0` (thiscall): the element's actor word.
const ELEMENT_ACTOR: u32 = 0x0096_11e0;
/// `00624700` (thiscall): the element's inner array.
const ELEMENT_INNER_ARRAY: u32 = 0x0062_4700;
/// `004b59f0` (cdecl, one word): the reference the object belongs to.
const OBJECT_REFERENCE: u32 = 0x004b_59f0;
/// `00517670` (thiscall): a kind number of the reference (4, 5 and 7 are
/// skipped).
const REFERENCE_KIND: u32 = 0x0051_7670;
/// `006299a0` (thiscall): the local array constructor.
const LIST_CONSTRUCT: u32 = 0x0062_99a0;
/// `006299c0` (thiscall, element address): appends the word at the address.
const LIST_APPEND: u32 = 0x0062_99c0;
/// `00629f10` (thiscall: element address, start, end): the index of the
/// element, or -1.
const LIST_FIND: u32 = 0x0062_9f10;
/// `00629b40` (thiscall): the local array destructor.
const LIST_DESTRUCT: u32 = 0x0062_9b40;
/// `004b5a80` (cdecl, one word).
const REFERENCE_KEY: u32 = 0x004b_5a80;
/// `00628f90` (thiscall: key, out array).
const COLLECT_FOR_KEY: u32 = 0x0062_8f90;

/// A box on the stack: min at +0, max at +0x10 (0x20 bytes).
const BOX_SIZE: u32 = 0x20;
/// A local array (data, count, ...), 0xc bytes.
const LIST_SIZE: u32 = 0xc;

// Translated from 00628da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the word at +0x20, assigns null to the
/// six `NiPointer`s at +0x08..+0x1C (`0066b0d0`) and tears the member at
/// +0x24 down (`00438af0`).
pub fn fn_00628da0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 0x20, 0);
    for index in 0..6u32 {
        e.call(0x0066_b0d0, &args![t + index * 4 + 8, 0u32]);
    }
    e.call(0x0043_8af0, &args![t + 0x24]);
}

// Translated from 00628950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a box from two vectors: copies `min` to +0 and
/// `max` to +0x10 (`004a3c90`). Returns `this`.
pub fn fn_00628950(e: &mut Engine, this: Ptr, min: Ptr, max: Ptr) -> Ptr {
    e.call(VECTOR_COPY, &args![this, min]);
    e.call(VECTOR_COPY, &args![this.byte_add(0x10), max]);
    this
}

// Translated from 006288a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this + 0x110`.
pub fn fn_006288a0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0x110)
}

// Translated from 006288c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes the object's box to `out`: when
/// `004ae750` gives a proxy, its box at +0xB0 (between two calls of the
/// empty `00a29680`); otherwise a box made of the vector at `01267e30`
/// twice.
// Translation incomplete: 006288c0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_006288c0(e: &mut Engine, this: Ptr, out: Ptr) {
    let proxy = e.call(OBJECT_PROXY, &args![this]).u32();
    if proxy != 0 {
        e.call(EMPTY_FUNCTION, &args![this]);
        let bounds = e.call(BOUNDS_AT_B0, &args![proxy]).u32();
        e.call(BOX_COPY, &args![out, bounds]);
        e.call(EMPTY_FUNCTION, &args![this]);
    } else {
        e.with_stack(BOX_SIZE, |e, local| {
            let first = e.call(VECTOR_AT_01267E30, &[]).u32();
            let second = e.call(VECTOR_AT_01267E30, &[]).u32();
            let made = fn_00628950(e, local, Ptr::new(second), Ptr::new(first));
            e.call(BOX_COPY, &args![out, made]);
        });
    }
}

// Translated from 00628cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `this = a & b`, four words (`ANDPS`).
// Translation incomplete: 00628cc0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628cc0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let mut lanes = [0u32; 4];
    for (lane, value) in lanes.iter_mut().enumerate() {
        let offset = lane as u32 * 4;
        *value =
            e.mem.u32(a.addr().wrapping_add(offset)) & e.mem.u32(b.addr().wrapping_add(offset));
    }
    for (lane, value) in lanes.iter().enumerate() {
        e.mem
            .set_u32(this.addr().wrapping_add(lane as u32 * 4), *value);
    }
}

// Translated from 00628c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the boxes `this` and `other` overlap
/// on x, y and z (`this.min <= other.max` and `other.min <= this.max` in
/// lanes 0-2, `00458990(7)`); returns that callee's `EAX`.
// Translation incomplete: 00628c50 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628c50(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.with_stack(0x30, |e, locals| {
        let low = locals;
        let high = locals.byte_add(0x10);
        let both = locals.byte_add(0x20);
        e.call(LANES_LESS_EQUAL, &args![this, low, other.byte_add(0x10)]);
        e.call(LANES_LESS_EQUAL, &args![other, high, this.byte_add(0x10)]);
        fn_00628cc0(e, both, low, high);
        e.call(MASK_ALL_SET, &args![both, 7u32]).u32()
    })
}

// Translated from 00628d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the box `this` contains the box
/// `other` on x, y and z (`this.min <= other.min` and `other.max <=
/// this.max`, `00458990(7)`), stored as a byte at `out` (`00622570`).
/// Returns `out`.
// Translation incomplete: 00628d10 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628d10(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    let inside = e.with_stack(0x30, |e, locals| {
        let low = locals;
        let high = locals.byte_add(0x10);
        let both = locals.byte_add(0x20);
        e.call(LANES_LESS_EQUAL, &args![this, low, other]);
        e.call(
            LANES_LESS_EQUAL,
            &args![other.byte_add(0x10), high, this.byte_add(0x10)],
        );
        fn_00628cc0(e, both, low, high);
        e.call(MASK_ALL_SET, &args![both, 7u32]).u32() != 0
    });
    e.call(STORE_BYTE, &args![out, inside as u32]);
    out
}

/// The test `00628df0` and `00628980` make for an object's box (`object +
/// 0x14`, filled through the actor word's object's vtable +0x3C): true when
/// `bounds` contains it (`00628d10`) and overlaps it (`00628c50`).
fn object_inside(e: &mut Engine, bounds: Ptr, object: u32, owner: u32, object_box: Ptr) -> bool {
    let target = e.call(WORD_AT_58, &args![owner]).u32();
    let box_source = e.call(PLUS_14, &args![object]).u32();
    e.vcall(target, 0x3c, &args![box_source, object_box]);
    let inside = e.with_stack(4, |e, flag| {
        let result = fn_00628d10(e, bounds, flag, object_box);
        e.call(BYTE_SET, &args![result]).bool()
    });
    inside && fn_00628c50(e, bounds, object_box) != 0
}

// Translated from 00628df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for each object of the array `objects` (its
/// element +0x10) whose word at +0 is set and whose box is not inside
/// `bounds` (see `object_inside`): when its type word (`0043b540` stored
/// through `008c71b0`, read by `0043b4d0`) is 0x1E and it has a collidable
/// reference (`TESHavokUtilities::FindCollidableRef`) for which vtable
/// +0x100 answers true, warps that reference (`LoadedAreaBound::WarpActor`).
// Translation incomplete: 00628df0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00628df0(e: &mut Engine, this: Ptr, bounds: Ptr, objects: Ptr, owner: u32) {
    e.with_stack(BOX_SIZE + 4, |e, locals| {
        let object_box = locals;
        let type_word = locals.byte_add(BOX_SIZE);
        e.call(BOX_CONSTRUCT, &args![object_box]);
        let mut index = 0i32;
        while index < e.call(ARRAY_COUNT, &args![objects]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![objects, index]).u32();
            let item = e.mem.u32(element);
            let object = e.call(PLUS_10, &args![item]).u32();
            if e.call(WORD_AT_0, &args![object]).u32() != 0
                && !object_inside(e, bounds, object, owner, object_box)
            {
                let word = e.call(OBJECT_TYPE_WORD, &args![object]).u32();
                e.call(STORE_WORD, &args![type_word, word]);
                if e.call(WORD_TYPE, &args![type_word]).u32() == 0x1e {
                    let reference = e.call(FIND_COLLIDABLE_REF, &args![object]).u32();
                    if reference != 0 && e.vcall(reference, 0x100, &[]).bool() {
                        e.call(WARP_ACTOR, &args![this, reference]);
                    }
                }
            }
            index += 1;
        }
    });
}

// Translated from 00628980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: for each element of `groups` (its actor word
/// `009611e0` and inner array `00624700`), gathers the references
/// (`004b59f0`) of the inner array's objects (element +0x10, word at +0 set)
/// whose box is not inside `bounds` and whose kind (`00517670`) is not 4, 5
/// or 7, into a local array; then, for each gathered reference not yet in a
/// second local array (`00629f10` gives -1), collects the references for its
/// key (`004b5a80`, `00628f90`) into a third array and appends them all to
/// the second. The three arrays are destroyed with `00629b40`.
// Translation incomplete: 00628980 - SEH exception unwinding and the /GS stack-cookie check (00ec408c) are not translated.
pub fn fn_00628980(e: &mut Engine, this: Ptr, bounds: Ptr, groups: Ptr) {
    e.with_stack(BOX_SIZE + 4 + 4 + LIST_SIZE * 3, |e, locals| {
        let object_box = locals;
        let reference_slot = locals.byte_add(BOX_SIZE).addr();
        let gathered = locals.byte_add(BOX_SIZE + 8);
        let done = locals.byte_add(BOX_SIZE + 8 + LIST_SIZE);
        let for_key = locals.byte_add(BOX_SIZE + 8 + 2 * LIST_SIZE);
        e.call(BOX_CONSTRUCT, &args![object_box]);
        e.call(LIST_CONSTRUCT, &args![gathered]);
        let mut group = 0i32;
        while group < e.call(ARRAY_COUNT, &args![groups]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![groups, group]).u32();
            let item = e.mem.u32(element);
            let actor = e.call(ELEMENT_ACTOR, &args![item]).u32();
            let element = e.call(ARRAY_ELEMENT, &args![groups, group]).u32();
            let item = e.mem.u32(element);
            let inner = e.call(ELEMENT_INNER_ARRAY, &args![item]).u32();
            let mut index = 0i32;
            while index < e.call(ARRAY_COUNT, &args![inner]).i32() {
                let element = e.call(ARRAY_ELEMENT, &args![inner, index]).u32();
                let item = e.mem.u32(element);
                let object = e.call(PLUS_10, &args![item]).u32();
                if e.call(WORD_AT_0, &args![object]).u32() != 0
                    && !object_inside(e, bounds, object, actor, object_box)
                {
                    let reference = e.call(OBJECT_REFERENCE, &args![object]).u32();
                    e.mem.set_u32(reference_slot, reference);
                    if reference != 0
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 4
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 5
                        && e.call(REFERENCE_KIND, &args![reference]).u32() != 7
                    {
                        e.call(LIST_APPEND, &args![gathered, reference_slot]);
                    }
                }
                index += 1;
            }
            group += 1;
        }
        e.call(LIST_CONSTRUCT, &args![done]);
        let mut index = 0i32;
        while index < e.call(ARRAY_COUNT, &args![gathered]).i32() {
            let element = e.call(ARRAY_ELEMENT, &args![gathered, index]).u32();
            let found = e
                .call(LIST_FIND, &args![done, element, 0u32, 0xffff_ffffu32])
                .i32();
            if found == -1 {
                let element = e.call(ARRAY_ELEMENT, &args![gathered, index]).u32();
                let reference = e.mem.u32(element);
                let key = e.call(REFERENCE_KEY, &args![reference]).u32();
                e.call(LIST_CONSTRUCT, &args![for_key]);
                e.call(COLLECT_FOR_KEY, &args![this, key, for_key]);
                let mut added = 0i32;
                while added < e.call(ARRAY_COUNT, &args![for_key]).i32() {
                    let element = e.call(ARRAY_ELEMENT, &args![for_key, added]).u32();
                    e.call(LIST_APPEND, &args![done, element]);
                    added += 1;
                }
                e.call(LIST_DESTRUCT, &args![for_key]);
            }
            index += 1;
        }
        e.call(LIST_DESTRUCT, &args![done]);
        e.call(LIST_DESTRUCT, &args![gathered]);
    });
}

// Translated from 006286f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: computes the box around the six objects at
/// +0x08..+0x1C (each one's box from `006288c0`, starting from max = the
/// float at `0104e148` and min = the float at `0104da48` in every lane);
/// then, when the cell at +0x20 has an object (`004543c0`, `00541c80`),
/// checks its objects at +0x110 against the box (`00628df0`) and its two
/// arrays at +0x28 and +0x34 (`00628980`).
// Translation incomplete: 006286f0 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_006286f0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    // Locals: the box (+0), max (+0x20), min (+0x30), an object's box (+0x40).
    e.with_stack(0x60, |e, locals| {
        let bounds = locals;
        let max = locals.byte_add(0x20);
        let min = locals.byte_add(0x30);
        let object_box = locals.byte_add(0x40);
        e.call(BOX_CONSTRUCT, &args![bounds]);
        e.call(VECTOR_CONSTRUCT, &args![min]);
        e.call(VECTOR_CONSTRUCT, &args![max]);
        let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
        e.call(VECTOR_SPLAT, &args![max, lowest]);
        let highest = x87_quiet_bits(e.global::<u32>(HIGHEST_FLOAT));
        e.call(VECTOR_SPLAT, &args![min, highest]);
        for index in 0..6u32 {
            e.call(BOX_CONSTRUCT, &args![object_box]);
            let object = e.call(WORD_AT_0, &args![t + index * 4 + 8]).u32();
            fn_006288c0(e, Ptr::new(object), object_box);
            e.call(VECTOR_MAX, &args![max, max, object_box.byte_add(0x10)]);
            e.call(VECTOR_MIN, &args![min, min, object_box]);
        }
        e.call(VECTOR_SET, &args![bounds, min]);
        e.call(VECTOR_SET, &args![bounds.byte_add(0x10), max]);
        let cell = e.mem.u32(t + 0x20);
        let cell_object = e.call(CELL_OBJECT, &args![cell]).u32();
        let data = if cell_object != 0 {
            e.call(CELL_OBJECT_DATA, &args![cell_object]).u32()
        } else {
            0
        };
        if data != 0 {
            let first_array = data + 0x28;
            let second_array = data + 0x34;
            let objects = fn_006288a0(e, Ptr::new(data));
            fn_00628df0(e, this, bounds, objects, data);
            fn_00628980(e, this, bounds, Ptr::new(first_array));
            fn_00628980(e, this, bounds, Ptr::new(second_array));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_8da0, fn_00628da0(Ptr)),
        entry!(0x0062_8950, fn_00628950(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0062_88a0, fn_006288a0(Ptr) -> Ptr),
        entry!(0x0062_88c0, fn_006288c0(Ptr, Ptr)),
        entry!(0x0062_8cc0, fn_00628cc0(Ptr, Ptr, Ptr)),
        entry!(0x0062_8c50, fn_00628c50(Ptr, Ptr) -> u32),
        entry!(0x0062_8d10, fn_00628d10(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0062_8df0, fn_00628df0(Ptr, Ptr, Ptr, u32)),
        entry!(0x0062_8980, fn_00628980(Ptr, Ptr, Ptr)),
        entry!(0x0062_86f0, fn_006286f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lanes(e: &Engine, p: u32) -> [f32; 4] {
        [
            e.mem.f32(p),
            e.mem.f32(p + 4),
            e.mem.f32(p + 8),
            e.mem.f32(p + 12),
        ]
    }

    fn set_lanes(e: &mut Engine, p: u32, v: [f32; 4]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
    }

    /// Doubles with the vector semantics the callees have.
    fn vector_doubles(e: &mut Engine) {
        e.register(LANES_LESS_EQUAL, |e, a| {
            let (x, y) = (lanes(e, a[0]), lanes(e, a[2]));
            for i in 0..4 {
                let m = if x[i] <= y[i] { u32::MAX } else { 0 };
                e.mem.set_u32(a[1] + 4 * i as u32, m);
            }
            Ret::default()
        });
        e.register(MASK_ALL_SET, |e, a| {
            let mut mask = 0;
            for i in 0..4 {
                mask |= (e.mem.u32(a[0] + 4 * i) >> 31) << i;
            }
            Ret {
                eax: (mask & a[1] == a[1]) as u32,
                ..Ret::default()
            }
        });
        e.register(STORE_BYTE, |e, a| {
            e.mem.set_u8(a[0], a[1] as u8);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(BYTE_SET, |e, a| Ret {
            eax: (e.mem.u8(a[0]) != 0) as u32,
            ..Ret::default()
        });
        e.register(BOX_COPY, |e, a| {
            let words = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        e.register(VECTOR_SET, |e, a| {
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        e.register(VECTOR_COPY, |e, a| {
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(VECTOR_SPLAT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1]);
            }
            Ret::default()
        });
        e.register(VECTOR_MAX, |e, a| {
            let (x, y) = (lanes(e, a[1]), lanes(e, a[2]));
            set_lanes(e, a[0], [0, 1, 2, 3].map(|i| x[i].max(y[i])));
            Ret::default()
        });
        e.register(VECTOR_MIN, |e, a| {
            let (x, y) = (lanes(e, a[1]), lanes(e, a[2]));
            set_lanes(e, a[0], [0, 1, 2, 3].map(|i| x[i].min(y[i])));
            Ret::default()
        });
        for addr in [BOX_CONSTRUCT, VECTOR_CONSTRUCT, EMPTY_FUNCTION] {
            e.register(addr, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
    }

    fn new_box(e: &mut Engine, min: [f32; 4], max: [f32; 4]) -> u32 {
        let b = e.mem.alloc(0x20);
        set_lanes(e, b, min);
        set_lanes(e, b + 0x10, max);
        b
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00628da0_clears_the_pointers_and_tears_down() {
        let mut e = Engine::new();
        e.register(0x0066_b0d0, |_, _| Ret::default());
        e.register(0x0043_8af0, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x20, 9);
        e.call_log = Some(vec![]);
        e.call(0x0062_8da0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let assigned: Vec<u32> = calls(&e, 0x0066_b0d0).iter().map(|a| a[0]).collect();
        assert_eq!(
            assigned,
            (0..6).map(|i| this + 8 + 4 * i).collect::<Vec<_>>()
        );
        assert_eq!(calls(&e, 0x0043_8af0), vec![vec![this + 0x24]]);
    }

    #[test]
    fn fn_00628950_builds_a_box_from_two_vectors() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let min = new_box(&mut e, [1.0, 2.0, 3.0, 4.0], [0.0; 4]);
        let max = new_box(&mut e, [5.0, 6.0, 7.0, 8.0], [0.0; 4]);
        assert_eq!(e.call(0x0062_8950, &args![this, min, max]).u32(), this);
        assert_eq!(lanes(&e, this), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(lanes(&e, this + 0x10), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn fn_006288a0_returns_this_plus_110() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0062_88a0, &args![0x1000u32]).u32(), 0x1110);
    }

    #[test]
    fn fn_006288c0_takes_the_proxy_box_or_the_fixed_vector() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        e.register(OBJECT_PROXY, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(BOUNDS_AT_B0, |_, a| Ret {
            eax: a[0] + 0xb0,
            ..Ret::default()
        });
        e.map(0x0126_7e30, 0x10);
        set_lanes(&mut e, 0x0126_7e30, [9.0, 8.0, 7.0, 6.0]);
        e.register(VECTOR_AT_01267E30, |_, _| Ret {
            eax: 0x0126_7e30,
            ..Ret::default()
        });
        let proxy = e.mem.alloc(0xd0);
        set_lanes(&mut e, proxy + 0xb0, [1.0; 4]);
        set_lanes(&mut e, proxy + 0xc0, [2.0; 4]);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, proxy);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x0062_88c0, &args![this, out]);
        assert_eq!(lanes(&e, out), [1.0; 4]);
        assert_eq!(lanes(&e, out + 0x10), [2.0; 4]);
        assert_eq!(calls(&e, EMPTY_FUNCTION).len(), 2);
        e.mem.set_u32(this, 0);
        e.call(0x0062_88c0, &args![this, out]);
        assert_eq!(lanes(&e, out), [9.0, 8.0, 7.0, 6.0]);
        assert_eq!(lanes(&e, out + 0x10), [9.0, 8.0, 7.0, 6.0]);
    }

    #[test]
    fn fn_00628cc0_ands_four_words() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(a + 4 * i, 0xff00_ff00 | i);
            e.mem.set_u32(b + 4 * i, 0x0ff0_0ff0 | (i << 4));
        }
        e.call(0x0062_8cc0, &args![this, a, b]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + 4 * i), 0x0f00_0f00 & 0xffff_ff00);
        }
    }

    #[test]
    fn fn_00628c50_tests_overlap_on_three_axes() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let near = new_box(&mut e, [5.0, 5.0, 5.0, 50.0], [15.0, 15.0, 15.0, 60.0]);
        let far = new_box(&mut e, [5.0, 11.0, 5.0, 0.0], [15.0, 15.0, 15.0, 0.0]);
        assert_eq!(e.call(0x0062_8c50, &args![this, near]).u32(), 1);
        assert_eq!(e.call(0x0062_8c50, &args![this, far]).u32(), 0);
    }

    #[test]
    fn fn_00628d10_tests_containment_into_a_byte() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let this = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let inside = new_box(&mut e, [1.0, 1.0, 1.0, -5.0], [9.0, 9.0, 9.0, 50.0]);
        let across = new_box(&mut e, [1.0; 4], [9.0, 11.0, 9.0, 9.0]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x0062_8d10, &args![this, out, inside]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x0062_8d10, &args![this, out, across]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// An array (data at +0, count at +4) of `items`.
    fn array_of(e: &mut Engine, items: &[u32]) -> u32 {
        let data = e.mem.alloc(4 * items.len() as u32 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        let array = e.mem.alloc(LIST_SIZE);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, items.len() as u32);
        array
    }

    /// Doubles for the object walks: an item's object is `item + 0x10`;
    /// the object's box (+0x14) is copied by the target's vtable +0x3C; the
    /// object word at +0 says it is live; +0x40 holds its type word, +0x44
    /// its collidable reference, +0x48 its reference, +0x4C the kind.
    fn walk_doubles(e: &mut Engine) {
        vector_doubles(e);
        e.register(ARRAY_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(ARRAY_ELEMENT, |e, a| Ret {
            eax: e.mem.u32(a[0]) + 4 * a[1],
            ..Ret::default()
        });
        e.register(PLUS_10, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        e.register(WORD_AT_0, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(WORD_AT_58, |_, _| Ret {
            eax: 0x0091_0000,
            ..Ret::default()
        });
        e.register(PLUS_14, |_, a| Ret {
            eax: a[0] + 0x14,
            ..Ret::default()
        });
        e.map(0x0091_0000, 4);
        e.put_vtable(0x0092_0000, &[0; 0x50]);
        e.mem.set_u32(0x0091_0000, 0x0092_0000);
        e.mem.set_u32(0x0092_0000 + 0x3c, 0x7000_003c);
        e.mem.set_u32(0x0092_0000 + 0x100, 0x7000_0100);
        e.register(0x7000_003c, |e, a| {
            let words = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[2], &words);
            Ret::default()
        });
        e.register(0x7000_0100, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(OBJECT_TYPE_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(WORD_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(FIND_COLLIDABLE_REF, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x44),
            ..Ret::default()
        });
        e.register(WARP_ACTOR, |_, _| Ret::default());
        e.register(OBJECT_REFERENCE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x48),
            ..Ret::default()
        });
        e.register(REFERENCE_KIND, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
    }

    /// An item whose object (+0x10) is live, with its box at object +0x14.
    fn item(e: &mut Engine, min: f32, max: f32, type_word: u32) -> u32 {
        let item = e.mem.alloc(0x70);
        let object = item + 0x10;
        e.mem.set_u32(object, 1);
        set_lanes(e, object + 0x14, [min; 4]);
        set_lanes(e, object + 0x24, [max; 4]);
        e.mem.set_u32(object + 0x40, type_word);
        let reference = e.mem.alloc(4);
        e.mem.set_u32(reference, 0x0092_0000);
        e.mem.set_u32(object + 0x44, reference);
        item
    }

    #[test]
    fn fn_00628df0_warps_collidable_references_outside_the_bounds() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        let bounds = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let inside = item(&mut e, 1.0, 2.0, 0x1e);
        let outside = item(&mut e, 20.0, 30.0, 0x1e);
        let other_type = item(&mut e, 20.0, 30.0, 0x1d);
        let dead = item(&mut e, 20.0, 30.0, 0x1e);
        e.mem.set_u32(dead + 0x10, 0);
        let objects = array_of(&mut e, &[inside, outside, other_type, dead]);
        e.call_log = Some(vec![]);
        e.call(
            0x0062_8df0,
            &args![0x5000u32, bounds, objects, 0x0091_0000u32],
        );
        let warped = calls(&e, WARP_ACTOR);
        assert_eq!(warped, vec![vec![0x5000, e.mem.u32(outside + 0x54)]]);
    }

    #[test]
    fn fn_00628980_gathers_new_references_and_collects_for_each() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        let gathered = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = gathered.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            let word = e.mem.u32(a[1]);
            log.borrow_mut().push((a[0], word));
            if e.mem.u32(a[0]) == 0 {
                let data = e.mem.alloc(0x40);
                e.mem.set_u32(a[0], data);
            }
            let count = e.mem.u32(a[0] + 4);
            let data = e.mem.u32(a[0]);
            e.mem.set_u32(data + 4 * count, word);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        // Local arrays start empty; the done list never holds the element.
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(LIST_FIND, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(REFERENCE_KEY, |_, a| Ret {
            eax: a[0] ^ 0xff,
            ..Ret::default()
        });
        e.register(COLLECT_FOR_KEY, |_, _| Ret::default());
        e.register(ELEMENT_ACTOR, |_, _| Ret {
            eax: 0x0091_0000,
            ..Ret::default()
        });
        e.register(ELEMENT_INNER_ARRAY, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let bounds = new_box(&mut e, [0.0; 4], [10.0; 4]);
        let mut objects = vec![];
        for kind in [3u32, 4, 5, 7] {
            let it = item(&mut e, 20.0, 30.0, 0);
            let reference = e.mem.alloc(4);
            e.mem.set_u32(reference, kind);
            e.mem.set_u32(it + 0x58, reference);
            objects.push(it);
        }
        let inside = item(&mut e, 1.0, 2.0, 0);
        objects.push(inside);
        let inner = array_of(&mut e, &objects);
        let group = e.mem.alloc(8);
        e.mem.set_u32(group + 4, inner);
        let groups = array_of(&mut e, &[group]);
        e.call_log = Some(vec![]);
        e.call(0x0062_8980, &args![0x5000u32, bounds, groups]);
        let appended = gathered.borrow().clone();
        assert_eq!(appended.len(), 1, "only the kind-3 reference is gathered");
        let first_reference = e.mem.u32(objects[0] + 0x58);
        assert_eq!(appended[0].1, first_reference);
        assert_eq!(
            calls(&e, COLLECT_FOR_KEY)[0][..2].to_vec(),
            vec![0x5000, first_reference ^ 0xff]
        );
        assert_eq!(calls(&e, LIST_DESTRUCT).len(), 3);
    }

    #[test]
    fn fn_006286f0_bounds_the_six_objects_and_checks_the_cell() {
        let mut e = Engine::new();
        walk_doubles(&mut e);
        e.map(LOWEST_FLOAT, 4);
        e.map(HIGHEST_FLOAT, 4);
        e.set_global(LOWEST_FLOAT, 0xff7f_ffeeu32);
        e.set_global(HIGHEST_FLOAT, 0x7f7f_ffeeu32);
        e.register(OBJECT_PROXY, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(BOUNDS_AT_B0, |_, a| Ret {
            eax: a[0] + 0xb0,
            ..Ret::default()
        });
        e.register(CELL_OBJECT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CELL_OBJECT_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x30);
        for i in 0..6u32 {
            let proxy = e.mem.alloc(0xd0);
            let v = i as f32;
            set_lanes(&mut e, proxy + 0xb0, [v, -v, v, 0.0]);
            set_lanes(&mut e, proxy + 0xc0, [v + 1.0, 1.0, 2.0 * v, 0.0]);
            let holder = e.mem.alloc(4);
            e.mem.set_u32(holder, proxy);
            e.mem.set_u32(this + 8 + 4 * i, holder);
        }
        let cell = e.mem.alloc(4);
        e.mem.set_u32(this + 0x20, cell);
        e.call_log = Some(vec![]);
        e.call(0x0062_86f0, &args![this]);
        assert!(calls(&e, CELL_OBJECT_DATA).is_empty(), "no cell object");

        let copies = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = copies.clone();
        e.register_double(VECTOR_SET, move |e, a| {
            record.borrow_mut().push(lanes(e, a[1]));
            let words = e.mem.bytes(a[1], 0x10);
            e.mem.write(a[0], &words);
            Ret::default()
        });
        let cell_object = e.mem.alloc(4);
        e.mem.set_u32(cell, cell_object);
        let data = e.mem.alloc(0x120);
        e.mem.set_u32(cell_object, data);
        // Empty arrays at +0x110, +0x28 and +0x34.
        e.register(LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_86f0, &args![this]);
        let counted: Vec<u32> = calls(&e, ARRAY_COUNT).iter().map(|a| a[0]).collect();
        assert_eq!(counted[0], data + 0x110);
        assert!(counted.contains(&(data + 0x28)));
        assert!(counted.contains(&(data + 0x34)));
        let set = calls(&e, VECTOR_SET);
        assert_eq!(set[1][0], set[0][0] + 0x10);
        let copied = copies.borrow().clone();
        assert_eq!(copied[0], [0.0, -5.0, 0.0, 0.0]);
        assert_eq!(copied[1], [6.0, 1.0, 10.0, 0.0]);
    }
}
