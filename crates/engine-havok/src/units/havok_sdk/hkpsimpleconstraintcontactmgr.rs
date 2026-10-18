//! `Havok SDK/hkpsimpleconstraintcontactmgr.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cfdae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_da50`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cfdae0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_da50, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cfdb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_d720`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cfdb10(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_d720, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cfdad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8798.
pub fn hkp_simple_constraint_contact_mgr_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8798
}

/// `hkpWorldCallbackUtil::fireContactPointRemoved` (`00d01900`),
/// `hkpEntityCallbackUtil::fireContactPointRemovedInternal` (`00d001c0`).
const FIRE_CONTACT_POINT_REMOVED: u32 = 0x00d0_1900;
const FIRE_CONTACT_POINT_REMOVED_INTERNAL: u32 = 0x00d0_01c0;
/// `hkpWorldCallbackUtil::fireContactPointCallback` (`00d00ca0`),
/// `hkpEntityCallbackUtil::fireContactPointCallbackInternal` (`00cfff00`).
const FIRE_CONTACT_POINT_CALLBACK: u32 = 0x00d0_0ca0;
const FIRE_CONTACT_POINT_CALLBACK_INTERNAL: u32 = 0x00cf_ff00;
/// `hkLs_doSimpleCollisionResponse` (`00d0c480`).
const DO_SIMPLE_COLLISION_RESPONSE: u32 = 0x00d0_c480;
/// `hkpSimpleConstraintContactMgr::setContactPointProperties` (Xbox PDB, `00cfd800`).
const SET_CONTACT_POINT_PROPERTIES: u32 = 0x00cf_d800;
/// `hkpWorldCallbackUtil::fireContactPointAdded` (`00d01850`),
/// `hkpEntityCallbackUtil::fireContactPointAddedInternal` (`00d00110`).
const FIRE_CONTACT_POINT_ADDED: u32 = 0x00d0_1850;
const FIRE_CONTACT_POINT_ADDED_INTERNAL: u32 = 0x00d0_0110;
/// `hkpWorldCallbackUtil::fireContactProcess` (`00d019b0`),
/// `hkpEntityCallbackUtil::fireContactProcessInternal` (`00d00270`).
const FIRE_CONTACT_PROCESS: u32 = 0x00d0_19b0;
const FIRE_CONTACT_PROCESS_INTERNAL: u32 = 0x00d0_0270;
/// `hkStatisticsCollector::addNormalChunk` (`00d06fb0`) and `getAllocatedSize` (`00d06f80`).
const ADD_NORMAL_CHUNK: u32 = 0x00d0_6fb0;
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The `hkpSimpleContactConstraintData` constructor (`00d92f20`), the `hkpConstraintInstance`
/// constructor (`00cf3f40`), its `00cf3070` setter and its destructor (`00cf40b0`).
const CONTACT_CONSTRAINT_DATA_CONSTRUCT: u32 = 0x00d9_2f20;
const CONSTRAINT_INSTANCE_CONSTRUCT: u32 = 0x00cf_3f40;
const CONSTRAINT_INSTANCE_SET_PRIORITY: u32 = 0x00cf_3070;
const CONSTRAINT_INSTANCE_DESTRUCT: u32 = 0x00cf_40b0;
/// `hkpWorldOperationUtil::removeConstraintImmediately` (`00cb3dd0`).
const REMOVE_CONSTRAINT_IMMEDIATELY: u32 = 0x00cb_3dd0;
/// `atomSizeRoundUp` (`00d92440`) and the array growth helper `00c90940`.
const ATOM_SIZE_ROUND_UP: u32 = 0x00d9_2440;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;

/// The properties block of the contact point `id` of the manager `this` (id table at +0x20,
/// atom at +0x3c): zero when the id's slot holds `0xff`, else
/// `atom + 0x30 + (u16[atom + 6] << 5) + byte[atom + 10] * slot`.
fn contact_point_properties(e: &Engine, this: u32, id: u32) -> u32 {
    let slot = e.mem.u32(this + 0x20).wrapping_add(id);
    let index = e.mem.u8(slot);
    if index == 0xff {
        return 0;
    }
    let atom = e.mem.u32(this + 0x3c);
    let stride = u32::from(e.mem.u8(atom + 10));
    let scaled = stride.wrapping_mul(u32::from(index));
    let header = u32::from(e.mem.u16(atom + 6)) << 5;
    header
        .wrapping_add(0x30)
        .wrapping_add(scaled)
        .wrapping_add(atom)
}

// Translated from 00cfcc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::getConstraintInstance` (Xbox PDB): the constraint instance
/// embedded at `this + 0x44`.
pub fn hkp_simple_constraint_contact_mgr_get_constraint_instance(
    _e: &mut Engine,
    this: Ptr,
) -> Ptr {
    Ptr::new(this.addr() + 0x44)
}

// Translated from 00cfcc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::reserveContactPointsImpl` (Xbox PDB): answers `1` (refused)
/// when `u16[this + 0x10] + u16[atom + 4] + count` is greater (signed) than
/// `u16[atom + 0xc] - 2`, the atom being the pointer at +0x3c; otherwise adds the low byte of
/// `count` to the low byte of the reservation at +0x10 (stored as a 16-bit value) and answers `0`.
pub fn hkp_simple_constraint_contact_mgr_reserve_contact_points_impl(
    e: &mut Engine,
    this: Ptr,
    count: u32,
) -> u32 {
    let base = this.addr();
    let atom = e.mem.u32(base + 0x3c);
    let used = u32::from(e.mem.u16(atom + 4)).wrapping_add(count);
    let reserved = u32::from(e.mem.u16(base + 0x10));
    let total = reserved.wrapping_add(used);
    let limit = u32::from(e.mem.u16(atom + 0xc)).wrapping_sub(2);
    if (total as i32) > (limit as i32) {
        return 1;
    }
    let sum = e.mem.u8(base + 0x10).wrapping_add(count as u8);
    e.mem.set_u16(base + 0x10, u16::from(sum));
    0
}

// Translated from 00cfccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::toiCollisionResponseBeginCallback` (Xbox PDB): a jump to
/// slot `0x40` of the object embedded at `this + 0x14`, with that object as `this`.
pub fn hkp_simple_constraint_contact_mgr_toi_collision_response_begin_callback(
    e: &mut Engine,
    this: Ptr,
) {
    e.vcall(this.addr() + 0x14, 0x40, &args![]);
}

// Translated from 00cfccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::toiCollisionResponseEndCallback` (Xbox PDB): calls slot
/// `0x44` of the object embedded at `this + 0x14` (that object as `this`) with the six
/// stack words it received, the second one passed through the x87 stack as a `float`.
#[allow(clippy::too_many_arguments)]
pub fn hkp_simple_constraint_contact_mgr_toi_collision_response_end_callback(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    value: f32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
) {
    e.vcall(
        this.addr() + 0x14,
        0x44,
        &args![word_1, x87_quiet(value), word_3, word_4, word_5, word_6],
    );
}

// Translated from 00cfccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::removeToiImpl` (Xbox PDB): builds the contact-point-removed
/// event `{ u16 0xffff, word_3 (+4), body A (+8, the word at this + 0x58), body B (+0xc,
/// this + 0x5c), -, this (+0x14), word_2 (+0x18) }` and fires it at the world's callbacks
/// (`this + 0xc`) and, for each body whose callback count (`u16` at +0x204) is not zero, at
/// the body. The event's word at +0x10 is never written by the exe.
pub fn hkp_simple_constraint_contact_mgr_remove_toi_impl(
    e: &mut Engine,
    this: Ptr,
    word_2: u32,
    word_3: u32,
) {
    let base = this.addr();
    let body_a = e.mem.u32(base + 0x58);
    let body_b = e.mem.u32(base + 0x5c);
    e.with_stack(0x1c, |e, event| {
        let at = event.addr();
        e.mem.set_u16(at, 0xffff);
        e.mem.set_u32(at + 4, word_3);
        e.mem.set_u32(at + 8, body_a);
        e.mem.set_u32(at + 0xc, body_b);
        e.mem.set_u32(at + 0x14, base);
        e.mem.set_u32(at + 0x18, word_2);
        let world = e.mem.u32(base + 0xc);
        e.call(FIRE_CONTACT_POINT_REMOVED, &args![world, event]);
        if e.mem.u16(body_a + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_REMOVED_INTERNAL, &args![body_a, event]);
        }
        if e.mem.u16(body_b + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_REMOVED_INTERNAL, &args![body_b, event]);
        }
    });
}

// Translated from 00cfcd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::fireCallbacksForEarliestToi` (Xbox PDB). Register value
/// first: ECX (`this`) is never read; the object used is the first stack word `toi_event`.
/// Builds the contact-point callback event (type `2`, then `0` and `1` for the two bodies at
/// `toi_event + 0xc` and `+ 0x10`) from the event's fields and `word_2` and fires it at the
/// world's callbacks (the word at +8 of the first body) and at each body whose callback
/// count (`u16` at +0x204) is not zero. The event's byte at +0x1f and word at +0xc are never
/// written by the exe.
pub fn hkp_simple_constraint_contact_mgr_fire_callbacks_for_earliest_toi(
    e: &mut Engine,
    _this: Ptr,
    toi_event: Ptr,
    word_2: u32,
) {
    let toi = toi_event.addr();
    let body_a = e.mem.u32(toi + 0xc);
    let body_b = e.mem.u32(toi + 0x10);
    e.with_stack(0x34, |e, event| {
        let at = event.addr();
        e.mem.set_u32(at + 8, body_b);
        e.mem.set_u32(at + 0x14, toi + 0x50);
        e.mem.set_u32(at + 0x18, toi + 0x18);
        e.mem.set_u32(at + 0x20, toi + 4);
        e.mem.set_u32(at + 0x24, word_2);
        e.mem.set_u32(at, 2);
        e.mem.set_u32(at + 4, body_a);
        e.mem.set_u32(at + 0x10, 0);
        e.mem.set_u8(at + 0x1c, 0);
        e.mem.set_u8(at + 0x1d, 0);
        e.mem.set_u8(at + 0x1e, 0);
        e.mem.set_u32(at + 0x28, toi + 0x2c);
        e.mem.set_u32(at + 0x2c, 0);
        e.mem.set_u32(at + 0x30, 0);
        let world = e.mem.u32(body_a + 8);
        e.call(FIRE_CONTACT_POINT_CALLBACK, &args![world, event]);
        let body_a = e.mem.u32(toi + 0xc);
        e.mem.set_u32(at, 0);
        if e.mem.u16(body_a + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_CALLBACK_INTERNAL, &args![body_a, event]);
        }
        let body_b = e.mem.u32(toi + 0x10);
        e.mem.set_u32(at, 1);
        if e.mem.u16(body_b + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_CALLBACK_INTERNAL, &args![body_b, event]);
        }
    });
}

// Translated from 00cfce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::confirmToi` (Xbox PDB). Register value first: ECX (`this`)
/// is never read; `toi_event` is the first stack word. Unless the flag `8` of the byte at
/// `toi_event + 0x27` is set, calls `hkLs_doSimpleCollisionResponse(world, toi_event, value,
/// word_3)` (`value` through the x87 stack) when either body (the words at `toi_event + 0xc`
/// and `+ 0x10`) has a type byte (+0xe8) other than 4 and 5; the world is the word at +8 of
/// the first body.
pub fn hkp_simple_constraint_contact_mgr_confirm_toi(
    e: &mut Engine,
    _this: Ptr,
    toi_event: Ptr,
    value: f32,
    word_3: u32,
) {
    let toi = toi_event.addr();
    let flags = e.mem.u8(toi + 0x27);
    let body_a = e.mem.u32(toi + 0xc);
    let world = e.mem.u32(body_a + 8);
    if flags & 8 != 0 {
        return;
    }
    let type_a = e.mem.u8(body_a + 0xe8);
    let body_b = e.mem.u32(toi + 0x10);
    if type_a == 5 || type_a == 4 {
        let type_b = e.mem.u8(body_b + 0xe8);
        if type_b == 5 || type_b == 4 {
            return;
        }
    }
    e.call(
        DO_SIMPLE_COLLISION_RESPONSE,
        &args![world, toi_event, x87_quiet(value), word_3],
    );
}

// Translated from 00cfce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::addToiImpl` (Xbox PDB). Calls
/// `setContactPointProperties(word_1, word_2, 7, word_9)` (`00cfd800`), which answers body A;
/// body B is `body A ^ this[0x58] ^ this[0x5c]`. Builds the contact-point-added event
/// (`word_1`, `word_2`, 0, -, `word_6`, `word_7`, `word_9`, `*float_pointer` and `float_5`
/// through the x87 stack, `this`, 0, `word_3`, `word_4`) and fires it at the world's callbacks
/// (`this[3]`) and at each body whose callback count (`u16` at +0x204) is not zero. When the
/// event's result word (+0x20) is `1`, calls slot `0x20` of `this` with `(word_4[1], word_9)`
/// and answers the result word; otherwise writes the event's float (+0x1c) back to
/// `*float_pointer` and answers the result word. (The disassembly reads the result word at
/// +0x20 for the answer; the decompiler's stack variable at +0x1c is not it.)
#[allow(clippy::too_many_arguments)]
pub fn hkp_simple_constraint_contact_mgr_add_toi_impl(
    e: &mut Engine,
    this: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    float_5: f32,
    word_6: u32,
    word_7: u32,
    float_pointer: Ptr,
    word_9: u32,
) -> u32 {
    let base = this.addr();
    let body_a = e
        .call(
            SET_CONTACT_POINT_PROPERTIES,
            &args![this, word_1, word_2, 7u32, word_9],
        )
        .u32();
    let body_b = e.mem.u32(base + 0x5c) ^ e.mem.u32(base + 0x58) ^ body_a;
    e.with_stack(0x34, |e, event| {
        let at = event.addr();
        e.mem.set_u32(at, word_1);
        e.mem.set_u32(at + 4, word_2);
        e.mem.set_u32(at + 8, 0);
        e.mem.set_u32(at + 0x10, word_6);
        e.mem.set_u32(at + 0x14, word_7);
        let float_bits = x87_quiet_bits(e.mem.u32(float_pointer.addr()));
        e.mem.set_u32(at + 0x1c, float_bits);
        e.mem.set_u32(at + 0x28, word_3);
        e.mem.set_u32(at + 0x30, x87_quiet(float_5).to_bits());
        e.mem.set_u32(at + 0x20, 0);
        let world = e.mem.u32(base + 0xc);
        e.mem.set_u32(at + 0x18, word_9);
        e.mem.set_u32(at + 0x24, base);
        e.mem.set_u32(at + 0x2c, word_4);
        e.call(FIRE_CONTACT_POINT_ADDED, &args![world, event]);
        if e.mem.u16(body_a + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_ADDED_INTERNAL, &args![body_a, event]);
        }
        if e.mem.u16(body_b + 0x204) != 0 {
            e.call(FIRE_CONTACT_POINT_ADDED_INTERNAL, &args![body_b, event]);
        }
        let result = e.mem.u32(at + 0x20);
        if result == 1 {
            let target = e.mem.u32(word_4 + 4);
            e.vcall(base, 0x20, &args![target, word_9]);
            e.mem.u32(at + 0x20)
        } else {
            let back = x87_quiet_bits(e.mem.u32(at + 0x1c));
            e.mem.set_u32(float_pointer.addr(), back);
            result
        }
    })
}

// Translated from 00cfd3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::getContactPoint` (Xbox PDB): the contact point of `id`,
/// `atom + 0x30 + 0x20 * byte[ids + id]` (ids at +0x20, atom at +0x3c).
pub fn hkp_simple_constraint_contact_mgr_get_contact_point(
    e: &mut Engine,
    this: Ptr,
    id: u16,
) -> u32 {
    let ids = e.mem.u32(this.addr() + 0x20);
    let index = u32::from(e.mem.u8(ids.wrapping_add(u32::from(id))));
    let atom = e.mem.u32(this.addr() + 0x3c);
    (index << 5).wrapping_add(atom).wrapping_add(0x30)
}

// Translated from 00cfd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::getContactPointProperties` (Xbox PDB): zero when the id's
/// slot holds `0xff`, else `atom + 0x30 + (u16[atom + 6] << 5) + byte[atom + 10] * slot`.
pub fn hkp_simple_constraint_contact_mgr_get_contact_point_properties(
    e: &mut Engine,
    this: Ptr,
    id: u16,
) -> u32 {
    contact_point_properties(e, this.addr(), u32::from(id))
}

// Translated from 00cfd450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A base constructor of the contact manager's family (class name unconfirmed): sets the
/// `u16` at +6 to `1`, the vtable `0x010c_c8e4` and the word at +8 to `word`. Returns `this`.
pub fn fn_00cfd450(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), 0x010c_c8e4);
    e.mem.set_u32(this.addr() + 8, word);
    this
}

// Translated from 00cfd470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::processContactImpl` (Xbox PDB), `(collidable_a,
/// collidable_b, _unused_3, result)`. While the counter at +0x12 is zero: bodies are the
/// collidables plus their signed byte at +0x10; the new counter is the smaller `u16` at
/// +0xaa of the two; fires the contact-process event `{collidable_a, collidable_b, -, result,
/// properties pointers..., (+0x410) this}` at the world's callbacks (`this[3]`) and at each
/// body whose callback count (`u16` at +0x204) is not zero, then calls slot `0x14` of the
/// object at `result + 4` with `(this[0x0e], 8)`; otherwise the new counter is the old one
/// minus one. Then copies each 0x30-byte entry's 32 bytes of the result (from +0x10 up to the
/// pointer at `result`) to the atom's contact point of its id (non-temporal 16-byte stores).
pub fn hkp_simple_constraint_contact_mgr_process_contact_impl(
    e: &mut Engine,
    this: Ptr,
    collidable_a: Ptr,
    collidable_b: Ptr,
    _unused_3: u32,
    result: Ptr,
) {
    let base = this.addr();
    let counter = e.mem.u16(base + 0x12);
    let new_counter = if counter == 0 {
        let a = collidable_a.addr();
        let b = collidable_b.addr();
        let body_a = a.wrapping_add(e.mem.i8(a + 0x10) as u32);
        let body_b = b.wrapping_add(e.mem.i8(b + 0x10) as u32);
        let count_a = e.mem.u16(body_a + 0xaa);
        let count_b = e.mem.u16(body_b + 0xaa);
        let smaller = if count_a < count_b { count_a } else { count_b };
        let end = e.mem.u32(result.addr());
        e.with_stack(0x414, |e, event| {
            let at = event.addr();
            e.mem.set_u32(at, a);
            e.mem.set_u32(at + 4, b);
            e.mem.set_u32(at + 0xc, result.addr());
            e.mem.set_u32(at + 0x410, base);
            let mut cursor = result.addr() + 0x10;
            let mut out = at + 0x10;
            while cursor < end {
                let id = e.mem.u32(cursor + 0x20);
                let properties = contact_point_properties(e, base, id);
                e.mem.set_u32(out, properties);
                cursor = cursor.wrapping_add(0x30);
                out += 4;
            }
            let world = e.mem.u32(base + 0xc);
            e.call(FIRE_CONTACT_PROCESS, &args![world, event]);
            if e.mem.u16(body_a + 0x204) != 0 {
                e.call(FIRE_CONTACT_PROCESS_INTERNAL, &args![body_a, event]);
            }
            if e.mem.u16(body_b + 0x204) != 0 {
                e.call(FIRE_CONTACT_PROCESS_INTERNAL, &args![body_b, event]);
            }
        });
        let target = e.mem.u32(result.addr() + 4);
        let owner = e.mem.u32(base + 0x38);
        e.vcall(target, 0x14, &args![owner, 8u32]);
        smaller
    } else {
        counter.wrapping_sub(1)
    };
    e.mem.set_u16(base + 0x12, new_counter);
    let end = e.mem.u32(result.addr());
    let mut cursor = result.addr() + 0x10;
    while cursor < end {
        let id = e.mem.u32(cursor + 0x20);
        let table = e.mem.u32(base + 0x20);
        let slot = u32::from(e.mem.u8(id.wrapping_add(table)));
        let atom = e.mem.u32(base + 0x3c);
        let target = (slot << 5).wrapping_add(atom).wrapping_add(0x30);
        for offset in (0..0x20).step_by(4) {
            let word = e.mem.u32(cursor + offset);
            e.mem.set_u32(target + offset, word);
        }
        cursor = cursor.wrapping_add(0x30);
    }
}

// Translated from 00cfd5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::calcContentStatistics` (Xbox PDB), `(collector,
/// _unused_2)`: begins the object `"hkpSimpleConstraintContactMgr"` (collector slot `0x18`),
/// adds the chunk `"ContactAtom"` (atom at +0x3c, `u16` size at atom + 2), and, when the id
/// array's capacity word (+0x28) is not negative or the collector's byte at +0x10 is set,
/// adds `"ContactIds"` (slot `0x14`, kind `3`, ids at +0x20, count at +0x24, size from
/// `getAllocatedSize`); ends the object (slot `0x1c`).
pub fn hkp_simple_constraint_contact_mgr_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let base = this.addr();
    let target = collector.addr();
    e.vcall(target, 0x18, &args![0x010c_c90cu32]);
    let atom = e.mem.u32(base + 0x3c);
    let atom_size = u32::from(e.mem.u16(atom + 2));
    e.call(
        ADD_NORMAL_CHUNK,
        &args![collector, 0x010c_c900u32, atom, atom_size, 0u32],
    );
    let capacity = e.mem.u32(base + 0x28);
    if (capacity as i32) >= 0 || e.mem.u8(target + 0x10) != 0 {
        let ids = e.mem.u32(base + 0x20);
        let size = e
            .call(GET_ALLOCATED_SIZE, &args![ids, capacity & 0x3fff_ffff])
            .u32();
        let ids = e.mem.u32(base + 0x20);
        let count = e.mem.u32(base + 0x24);
        e.vcall(target, 0x14, &args![3u32, 0x010c_c8f4u32, ids, count, size]);
    }
    e.vcall(target, 0x1c, &args![]);
}

// Translated from 00cfd660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::getAllContactPointIds` (Xbox PDB): a jump to
/// `hkpDynamicsCpIdMgr::getAllUsedIds` (`00cfd9f0`) with the id manager embedded at
/// `this + 0x20` as `this` and the same output array.
pub fn hkp_simple_constraint_contact_mgr_get_all_contact_point_ids(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
) {
    hkp_dynamics_cp_id_mgr_get_all_used_ids(e, Ptr::new(this.addr() + 0x20), out);
}

// Translated from 00cfd670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::hkpSimpleConstraintContactMgr` (Xbox PDB, unnamed in the
/// map): `(world, body_a, body_b)`. Sets +6 to `1`, +8 to zero and the vtable `0x010c_c984`,
/// constructs the contact constraint data at +0x14 (`00d92f20`, with the instance at +0x44,
/// `body_a`, `body_b`) and the constraint instance at +0x44 (`00cf3f40`), stores the bodies
/// at +0x58 and +0x5c, clears the 16-bit fields at +0x10, +0x12, +0x18 and +0x48, sets
/// +0x50 to `this + 0x14`, +0x78 to `0xffffffff` and +0xc to the world, and passes the `u16`
/// priority looked up in the world's tables (indices from the bodies' signed bytes at +0x2a)
/// to `00cf3070`. Returns `this`.
pub fn fn_00cfd670(e: &mut Engine, this: Ptr, world: u32, body_a: u32, body_b: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, 0x010c_c984);
    e.call(
        CONTACT_CONSTRAINT_DATA_CONSTRUCT,
        &args![base + 0x14, base + 0x44, body_a, body_b],
    );
    e.call(CONSTRAINT_INSTANCE_CONSTRUCT, &args![base + 0x44, 1u32]);
    e.mem.set_u32(base + 0x58, body_a);
    e.mem.set_u16(base + 0x12, 0);
    e.mem.set_u16(base + 0x10, 0);
    e.mem.set_u16(base + 0x18, 0);
    e.mem.set_u16(base + 0x48, 0);
    e.mem.set_u32(base + 0x50, base + 0x14);
    e.mem.set_u32(base + 0x5c, body_b);
    e.mem.set_u32(base + 0x78, 0xffff_ffff);
    e.mem.set_u32(base + 0xc, world);
    let type_a = e.mem.i8(body_a + 0x2a) as u32;
    let type_b = e.mem.i8(body_b + 0x2a) as u32;
    let tables = e.mem.u32(world + 0x80);
    let row = type_a.wrapping_mul(5);
    let slot = type_b
        .wrapping_add(tables)
        .wrapping_add(row.wrapping_mul(2))
        .wrapping_add(0x1bb0);
    let index = (e.mem.i8(slot) as u32) << 6;
    let priority = u32::from(e.mem.u16(index.wrapping_add(tables).wrapping_add(0x1c5c)));
    e.call(
        CONSTRAINT_INSTANCE_SET_PRIORITY,
        &args![base + 0x44, priority],
    );
    this
}

// Translated from 00cfd720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::~hkpSimpleConstraintContactMgr` (Xbox PDB): sets the vtable
/// `0x010c_c984`; when the `u16` at atom + 4 (atom at +0x3c) is not zero, removes the
/// constraint instance at +0x44 from the world (`this[3]`) immediately (`00cb3dd0`, flag `1`);
/// clears the words at +0x58, +0x5c and +0x50, destroys the constraint instance (`00cf40b0`)
/// and the contact constraint data at +0x14 (`00cfda50`) and ends with the vtable
/// `0x0102_e388`.
pub fn hkp_simple_constraint_contact_mgr_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let atom = e.mem.u32(base + 0x3c);
    e.mem.set_u32(base, 0x010c_c984);
    if e.mem.u16(atom + 4) != 0 {
        let world = e.mem.u32(base + 0xc);
        e.call(
            REMOVE_CONSTRAINT_IMMEDIATELY,
            &args![world, base + 0x44, 1u32],
        );
    }
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u32(base + 0x50, 0);
    e.call(CONSTRAINT_INSTANCE_DESTRUCT, &args![base + 0x44]);
    hkp_simple_contact_constraint_data_destructor(e, Ptr::new(base + 0x14));
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00cfd780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleConstraintContactMgr::Factory::createContactMgr` (Xbox PDB), `(collidable_a,
/// collidable_b, _unused_3)`: allocates 0x80 bytes through the memory router's allocator
/// (slot `4`), stores `0x80` as the `u16` at +4 of the block, and constructs the contact
/// manager in it (`00cfd670`) with the world at `this + 8` and the bodies (the collidables
/// plus their signed byte at +0x10). Returns the block.
pub fn hkp_simple_constraint_contact_mgr_factory_create_contact_mgr(
    e: &mut Engine,
    this: Ptr,
    collidable_a: u32,
    collidable_b: u32,
    _unused_3: u32,
) -> Ptr {
    let body_a = collidable_a.wrapping_add(e.mem.i8(collidable_a + 0x10) as u32);
    let body_b = collidable_b.wrapping_add(e.mem.i8(collidable_b + 0x10) as u32);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x80u32]).u32();
    e.mem.set_u16(block + 4, 0x80);
    let world = e.mem.u32(this.addr() + 8);
    fn_00cfd670(e, Ptr::new(block), world, body_a, body_b)
}

// Translated from 00cfd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte-array fix-up on the id table (data +0, count +4; class unconfirmed, the id manager
/// of the contact manager): from the last entry down, every byte that is not `0xff` and is
/// greater (signed, as a number) than `threshold` is decremented.
pub fn fn_00cfd7d0(e: &mut Engine, this: Ptr, threshold: i32) {
    let base = this.addr();
    let mut index = e.mem.i32(base + 4).wrapping_sub(1);
    if index < 0 {
        return;
    }
    loop {
        let data = e.mem.u32(base);
        let slot = data.wrapping_add(index as u32);
        let value = e.mem.u8(slot);
        if value != 0xff && i32::from(value) > threshold {
            e.mem.set_u8(slot, value.wrapping_sub(1));
        }
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
    }
}

// Translated from 00cfd950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintAtomUtil::removeAtAndCopy` (Xbox PDB), cdecl `(atom, index)`:
/// decrements the `u16` count at atom + 4 and closes the gap at `index` by moving the later
/// contact points (0x20 bytes each, from atom + 0x30) down one place and, in step, the
/// per-point properties (a block of `byte[atom + 10]` bytes each, starting at
/// `atom + 0x30 + (u16[atom + 6] << 5) + stride * index`) down one block, in 4-byte words.
pub fn hkp_simple_contact_constraint_atom_util_remove_at_and_copy(
    e: &mut Engine,
    atom: Ptr,
    index: i32,
) {
    let base = atom.addr();
    let count = i32::from(e.mem.u16(base + 4));
    let stride = u32::from(e.mem.u8(base + 10));
    let scaled = stride.wrapping_mul(i32::from(index as i16) as u32);
    let header = u32::from(e.mem.u16(base + 6)) << 5;
    let last = count.wrapping_sub(1);
    e.mem.set_u16(base + 4, last as u16);
    let mut point = (index as u32)
        .wrapping_shl(5)
        .wrapping_add(base)
        .wrapping_add(0x30);
    let mut properties = header
        .wrapping_add(scaled.wrapping_add(base))
        .wrapping_add(0x30);
    if index >= last {
        return;
    }
    let words = stride >> 2;
    let mut remaining = last.wrapping_sub(index);
    loop {
        for offset in (0..0x20).step_by(4) {
            let word = e.mem.u32(point + 0x20 + offset);
            e.mem.set_u32(point + offset, word);
        }
        let next = properties.wrapping_add(stride);
        for word_index in 0..words {
            let word = e.mem.u32(next.wrapping_add(4 * word_index));
            e.mem.set_u32(properties.wrapping_add(4 * word_index), word);
        }
        point = point.wrapping_add(0x20);
        remaining = remaining.wrapping_sub(1);
        properties = next;
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00cfd9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpDynamicsCpIdMgr::getAllUsedIds` (Xbox PDB), `(out)`: for every index below the count
/// (+4, re-read each step) whose byte in the id table (data at +0) is not `0xff`, appends the
/// index as a `u16` to the output array (data +0, size +4, capacity word +8 masked with
/// `0x3fffffff`), growing it first through `00c90940(allocator, array, 2)` when full.
pub fn hkp_dynamics_cp_id_mgr_get_all_used_ids(e: &mut Engine, this: Ptr, out: Ptr) {
    let base = this.addr();
    let array = out.addr();
    let mut index: i32 = 0;
    if e.mem.i32(base + 4) <= 0 {
        return;
    }
    loop {
        let table = e.mem.u32(base);
        if e.mem.u8(table.wrapping_add(index as u32)) != 0xff {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            if e.mem.u32(array + 4) == capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE_MORE, &args![allocator, out, 2u32]);
            }
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            e.mem
                .set_u16(data.wrapping_add(size.wrapping_mul(2)), index as u16);
            e.mem.set_u32(array + 4, size.wrapping_add(1));
        }
        index += 1;
        if index >= e.mem.i32(base + 4) {
            break;
        }
    }
}

// Translated from 00cfda50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleContactConstraintData::~hkpSimpleContactConstraintData` (Xbox PDB): sets the
/// vtable `0x010c_c934`; when the atom at +0x28 exists, frees it through the allocator (slot
/// `8`) with `atomSizeRoundUp(u16[atom + 2])`; clears the size at +0x10 and, when the capacity
/// word at +0x14 is not negative, frees the array at +0xc (slot `0x10`, capacity masked with
/// `0x3fffffff`); resets +0xc and +0x14 (`0x80000000`) and ends with the vtable `0x0102_e388`.
pub fn hkp_simple_contact_constraint_data_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let atom = e.mem.u32(base + 0x28);
    e.mem.set_u32(base, 0x010c_c934);
    if atom != 0 {
        let size = e
            .call(ATOM_SIZE_ROUND_UP, &args![u32::from(e.mem.u16(atom + 2))])
            .u32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![atom, size]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x14);
    e.mem.set_u32(base + 0x10, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0xc);
        e.vcall(allocator, 0x10, &args![data, capacity & 0x3fff_ffff]);
    }
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x14, 0x8000_0000);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_cc60, hkp_simple_constraint_contact_mgr_get_constraint_instance(Ptr) -> Ptr),
        entry!(0x00cf_cc70, hkp_simple_constraint_contact_mgr_reserve_contact_points_impl(Ptr, u32) -> u32),
        entry!(
            0x00cf_ccb0,
            hkp_simple_constraint_contact_mgr_toi_collision_response_begin_callback(Ptr)
        ),
        entry!(
            0x00cf_ccc0,
            hkp_simple_constraint_contact_mgr_toi_collision_response_end_callback(
                Ptr, u32, f32, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00cf_ccf0,
            hkp_simple_constraint_contact_mgr_remove_toi_impl(Ptr, u32, u32)
        ),
        entry!(
            0x00cf_cd70,
            hkp_simple_constraint_contact_mgr_fire_callbacks_for_earliest_toi(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_ce30,
            hkp_simple_constraint_contact_mgr_confirm_toi(Ptr, Ptr, f32, u32)
        ),
        entry!(0x00cf_ce80, hkp_simple_constraint_contact_mgr_add_toi_impl(Ptr, u32, u32, u32, u32, f32, u32, u32, Ptr, u32) -> u32),
        entry!(0x00cf_d3f0, hkp_simple_constraint_contact_mgr_get_contact_point(Ptr, u16) -> u32),
        entry!(0x00cf_d410, hkp_simple_constraint_contact_mgr_get_contact_point_properties(Ptr, u16) -> u32),
        entry!(0x00cf_d450, fn_00cfd450(Ptr, u32) -> Ptr),
        entry!(
            0x00cf_d470,
            hkp_simple_constraint_contact_mgr_process_contact_impl(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00cf_d5e0,
            hkp_simple_constraint_contact_mgr_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_d660,
            hkp_simple_constraint_contact_mgr_get_all_contact_point_ids(Ptr, Ptr)
        ),
        entry!(0x00cf_d670, fn_00cfd670(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00cf_d720,
            hkp_simple_constraint_contact_mgr_destructor(Ptr)
        ),
        entry!(0x00cf_d780, hkp_simple_constraint_contact_mgr_factory_create_contact_mgr(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00cf_d7d0, fn_00cfd7d0(Ptr, i32)),
        entry!(
            0x00cf_d950,
            hkp_simple_contact_constraint_atom_util_remove_at_and_copy(Ptr, i32)
        ),
        entry!(
            0x00cf_d9f0,
            hkp_dynamics_cp_id_mgr_get_all_used_ids(Ptr, Ptr)
        ),
        entry!(
            0x00cf_da50,
            hkp_simple_contact_constraint_data_destructor(Ptr)
        ),
        entry!(0x00cfdad0, hkp_simple_constraint_contact_mgr_get_class_type() -> u32),
        entry!(0x00cf_dae0, deleting_destructor_00cfdae0(Ptr, u8) -> Ptr),
        entry!(0x00cf_db10, deleting_destructor_00cfdb10(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00cfdae0_frees_only_when_bit_zero_is_set() {
        check(0x00cf_dae0, 0x00cf_da50);
    }

    #[test]
    fn deleting_destructor_00cfdb10_frees_only_when_bit_zero_is_set() {
        check(0x00cf_db10, 0x00cf_d720);
    }

    #[test]
    fn hkp_simple_constraint_contact_mgr_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_dad0, &args![]).u32(), 0x0126_8798);
    }

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn new_log() -> EventLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs `[address, arguments...]`.
    fn log_calls(e: &mut Engine, address: u32, log: &EventLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    /// A double at `address` that logs `[address, arguments..., words of the structure
    /// the second argument points to (offsets in `offsets`)]`.
    fn log_event(e: &mut Engine, address: u32, offsets: &'static [u32], log: &EventLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            let mut entry = vec![address, a[0], a[1]];
            entry.extend(offsets.iter().map(|offset| e.mem.u32(a[1] + offset)));
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    const ALLOC_DOUBLE: u32 = 0x0900_0031;
    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0032;
    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0033;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0200;

    /// A memory router whose allocator logs `[4, size]` for slot 4 (and answers `block`),
    /// `[8, block, size]` for slot 8 and `[0x10, data, size]` for slot `0x10`.
    fn install_allocator(e: &mut Engine, log: &EventLog, block: u32) -> u32 {
        let mut slots = vec![0u32; 8];
        slots[1] = ALLOC_DOUBLE;
        slots[2] = FREE_BLOCK_DOUBLE;
        slots[4] = FREE_ARRAY_DOUBLE;
        e.put_vtable(ALLOCATOR_VTABLE, &slots);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let alloc_log = Rc::clone(log);
        e.register_double(ALLOC_DOUBLE, move |_, a| {
            alloc_log.borrow_mut().push(vec![4, a[1]]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let free_log = Rc::clone(log);
        e.register_double(FREE_BLOCK_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(vec![8, a[1], a[2]]);
            Ret::default()
        });
        let array_log = Rc::clone(log);
        e.register_double(FREE_ARRAY_DOUBLE, move |_, a| {
            array_log.borrow_mut().push(vec![0x10, a[1], a[2]]);
            Ret::default()
        });
        allocator
    }

    #[test]
    fn the_constraint_instance_is_embedded_at_0x44() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cf_cc60, &args![0x1000u32]).u32(), 0x1044);
    }

    #[test]
    fn reserving_contact_points_refuses_when_the_atom_would_overflow() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let atom = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x3c, atom);
        e.mem.set_u16(atom + 4, 3);
        e.mem.set_u16(atom + 0xc, 10);
        e.mem.set_u16(this + 0x10, 2);
        assert_eq!(e.call(0x00cf_cc70, &args![this, 3u32]).u32(), 0);
        assert_eq!(e.mem.u16(this + 0x10), 5);
        assert_eq!(e.call(0x00cf_cc70, &args![this, 1u32]).u32(), 1);
        assert_eq!(e.mem.u16(this + 0x10), 5);
        // Only the low byte of the reservation takes part in the sum.
        e.mem.set_u16(atom + 0xc, 0x1000);
        e.mem.set_u16(this + 0x10, 0x01ff);
        assert_eq!(e.call(0x00cf_cc70, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.mem.u16(this + 0x10), 1);
    }

    #[test]
    fn toi_response_callbacks_forward_to_the_embedded_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        let mut slots = vec![0u32; 0x20];
        slots[0x40 / 4] = 0x0900_0040;
        slots[0x44 / 4] = 0x0900_0044;
        e.put_vtable(0x0900_1000, &slots);
        e.mem.set_u32(this + 0x14, 0x0900_1000);
        let log = new_log();
        log_calls(&mut e, 0x0900_0040, &log);
        log_calls(&mut e, 0x0900_0044, &log);
        e.call(0x00cf_ccb0, &args![this]);
        e.call(
            0x00cf_ccc0,
            &args![this, 1u32, 1.5f32, 3u32, 4u32, 5u32, 6u32],
        );
        e.call(
            0x00cf_ccc0,
            &args![
                this,
                1u32,
                f32::from_bits(0x7f80_0001),
                3u32,
                4u32,
                5u32,
                6u32
            ],
        );
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x0900_0040, this + 0x14],
                vec![0x0900_0044, this + 0x14, 1, 1.5f32.to_bits(), 3, 4, 5, 6],
                vec![0x0900_0044, this + 0x14, 1, 0x7fc0_0001, 3, 4, 5, 6],
            ]
        );
    }

    /// Two bodies with the given callback counts at +0x204.
    fn two_bodies(e: &mut Engine, count_a: u16, count_b: u16) -> (u32, u32) {
        let body_a = e.mem.alloc(0x400);
        let body_b = e.mem.alloc(0x400);
        e.mem.set_u16(body_a + 0x204, count_a);
        e.mem.set_u16(body_b + 0x204, count_b);
        (body_a, body_b)
    }

    #[test]
    fn removing_a_toi_fires_the_removed_event() {
        let mut e = Engine::new();
        let (body_a, body_b) = two_bodies(&mut e, 0, 1);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0xc, 0x7000);
        e.mem.set_u32(this + 0x58, body_a);
        e.mem.set_u32(this + 0x5c, body_b);
        let log = new_log();
        const WORDS: &[u32] = &[0, 4, 8, 0xc, 0x14, 0x18];
        for address in [
            FIRE_CONTACT_POINT_REMOVED,
            FIRE_CONTACT_POINT_REMOVED_INTERNAL,
        ] {
            log_event(&mut e, address, WORDS, &log);
        }
        e.call(0x00cf_ccf0, &args![this, 0xaau32, 0xbbu32]);
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        let event = log[0][2];
        // The first word is 0xffff in its low half only; the rest of it is not written.
        assert_eq!(
            log[0],
            vec![
                FIRE_CONTACT_POINT_REMOVED,
                0x7000,
                event,
                0xffff,
                0xbb,
                body_a,
                body_b,
                this,
                0xaa
            ]
        );
        assert_eq!(log[1][..2], [FIRE_CONTACT_POINT_REMOVED_INTERNAL, body_b]);
        assert_eq!(log[1][2], event);
    }

    #[test]
    fn the_earliest_toi_fires_the_point_callback_at_the_world_and_both_bodies() {
        let mut e = Engine::new();
        let (body_a, body_b) = two_bodies(&mut e, 1, 1);
        e.mem.set_u32(body_a + 8, 0x7100);
        let toi = e.mem.alloc(0x100);
        e.mem.set_u32(toi + 0xc, body_a);
        e.mem.set_u32(toi + 0x10, body_b);
        let log = new_log();
        const WORDS: &[u32] = &[
            0, 4, 8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30,
        ];
        for address in [
            FIRE_CONTACT_POINT_CALLBACK,
            FIRE_CONTACT_POINT_CALLBACK_INTERNAL,
        ] {
            log_event(&mut e, address, WORDS, &log);
        }
        e.call(0x00cf_cd70, &args![0x1234u32, toi, 0xccu32]);
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        let event = log[0][2];
        assert_eq!(
            log[0],
            vec![
                FIRE_CONTACT_POINT_CALLBACK,
                0x7100,
                event,
                2,
                body_a,
                body_b,
                0,
                toi + 0x50,
                toi + 0x18,
                0,
                toi + 4,
                0xcc,
                toi + 0x2c,
                0,
                0
            ]
        );
        // The same event is reused with the type word 0 for body A and 1 for body B.
        assert_eq!(
            log[1][..4],
            [FIRE_CONTACT_POINT_CALLBACK_INTERNAL, body_a, event, 0]
        );
        assert_eq!(
            log[2][..4],
            [FIRE_CONTACT_POINT_CALLBACK_INTERNAL, body_b, event, 1]
        );
        // Without callback counts only the world is called.
        e.mem.set_u16(body_a + 0x204, 0);
        e.mem.set_u16(body_b + 0x204, 0);
        drop(log);
        let log = new_log();
        for address in [
            FIRE_CONTACT_POINT_CALLBACK,
            FIRE_CONTACT_POINT_CALLBACK_INTERNAL,
        ] {
            log_calls(&mut e, address, &log);
        }
        e.call(0x00cf_cd70, &args![0u32, toi, 0u32]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn confirming_a_toi_responds_unless_flagged_or_both_bodies_are_of_type_4_or_5() {
        let mut e = Engine::new();
        let (body_a, body_b) = two_bodies(&mut e, 0, 0);
        e.mem.set_u32(body_a + 8, 0x7200);
        let toi = e.mem.alloc(0x100);
        e.mem.set_u32(toi + 0xc, body_a);
        e.mem.set_u32(toi + 0x10, body_b);
        let log = new_log();
        log_calls(&mut e, DO_SIMPLE_COLLISION_RESPONSE, &log);
        let respond = |e: &mut Engine, flags: u8, type_a: u8, type_b: u8| -> usize {
            e.mem.set_u8(toi + 0x27, flags);
            e.mem.set_u8(body_a + 0xe8, type_a);
            e.mem.set_u8(body_b + 0xe8, type_b);
            log.borrow_mut().clear();
            e.call(0x00cf_ce30, &args![0x1234u32, toi, 0.5f32, 0x99u32]);
            log.borrow().len()
        };
        assert_eq!(respond(&mut e, 0, 3, 3), 1);
        assert_eq!(respond(&mut e, 0, 5, 3), 1);
        assert_eq!(respond(&mut e, 0, 4, 2), 1);
        assert_eq!(respond(&mut e, 0, 3, 4), 1);
        assert_eq!(respond(&mut e, 0, 4, 4), 0);
        assert_eq!(respond(&mut e, 0, 5, 4), 0);
        assert_eq!(respond(&mut e, 8, 3, 3), 0);
        assert_eq!(respond(&mut e, 0x77, 3, 3), 1);
        assert_eq!(
            *log.borrow(),
            vec![vec![
                DO_SIMPLE_COLLISION_RESPONSE,
                0x7200,
                toi,
                0.5f32.to_bits(),
                0x99
            ]]
        );
    }

    #[test]
    fn adding_a_toi_fires_the_added_event_and_follows_its_answer() {
        let mut e = Engine::new();
        let (body_a, body_b) = two_bodies(&mut e, 1, 0);
        let this = e.mem.alloc(0x80);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, 0x0900_0020);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0xc, 0x7300);
        e.mem.set_u32(this + 0x5c, 0x1234_0000);
        e.mem.set_u32(this + 0x58, body_b ^ 0x1234_0000 ^ body_a);
        let log = new_log();
        log_calls(&mut e, 0x0900_0020, &log);
        e.register_double(SET_CONTACT_POINT_PROPERTIES, move |_, _| Ret {
            eax: body_a,
            ..Ret::default()
        });
        log_calls(&mut e, FIRE_CONTACT_POINT_ADDED_INTERNAL, &log);
        let answer = Rc::new(RefCell::new(0u32));
        let answer_in_double = Rc::clone(&answer);
        let event_log = new_log();
        let event_log_in_double = Rc::clone(&event_log);
        e.register_double(FIRE_CONTACT_POINT_ADDED, move |e, a| {
            let words = (0..0x34)
                .step_by(4)
                .map(|offset| e.mem.u32(a[1] + offset))
                .collect();
            event_log_in_double.borrow_mut().push(words);
            e.mem.set_u32(a[1] + 0x20, *answer_in_double.borrow());
            e.mem.set_f32(a[1] + 0x1c, 7.5);
            Ret::default()
        });
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target + 4, 0x44);
        let float_pointer = e.mem.alloc(0x10);
        e.mem.set_u32(float_pointer, 0x7f80_0001);
        let call = |e: &mut Engine| {
            e.call(
                0x00cf_ce80,
                &args![
                    this,
                    0x11u32,
                    0x22u32,
                    0x33u32,
                    target,
                    2.5f32,
                    0x66u32,
                    0x77u32,
                    float_pointer,
                    0x99u32
                ],
            )
            .u32()
        };
        *answer.borrow_mut() = 1;
        assert_eq!(call(&mut e), 1);
        let events = event_log.borrow();
        assert_eq!(events[0][0], 0x11);
        assert_eq!(events[0][1], 0x22);
        assert_eq!(events[0][2], 0);
        assert_eq!(events[0][4], 0x66);
        assert_eq!(events[0][5], 0x77);
        assert_eq!(events[0][6], 0x99);
        assert_eq!(events[0][7], 0x7fc0_0001);
        assert_eq!(events[0][8], 0);
        assert_eq!(events[0][9], this);
        assert_eq!(events[0][10], 0x33);
        assert_eq!(events[0][11], target);
        assert_eq!(events[0][12], 2.5f32.to_bits());
        drop(events);
        let entries = log.borrow();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0][..2], [FIRE_CONTACT_POINT_ADDED_INTERNAL, body_a]);
        // Result 1: slot 0x20 of the manager gets (target[1], word 9); the float stays.
        assert_eq!(entries[1], vec![0x0900_0020, this, 0x44, 0x99]);
        assert_eq!(e.mem.u32(float_pointer), 0x7f80_0001);
        drop(entries);
        log.borrow_mut().clear();
        // Any other result: the event's float is written back through the pointer.
        *answer.borrow_mut() = 0;
        e.mem.set_u16(body_b + 0x204, 2);
        assert_eq!(call(&mut e), 0);
        assert_eq!(e.mem.f32(float_pointer), 7.5);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(
            log.borrow()[1][..2],
            [FIRE_CONTACT_POINT_ADDED_INTERNAL, body_b]
        );
    }

    /// A manager with an id table (id 5 holds slot 2, id 6 holds `0xff`) and an atom with
    /// the properties stride 0x10 and the header word 3.
    fn manager_with_atom(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x100);
        let ids = e.mem.alloc(0x20);
        e.mem.set_u8(ids + 5, 2);
        e.mem.set_u8(ids + 6, 0xff);
        let atom = e.mem.alloc(0x2100);
        e.mem.set_u16(atom + 6, 3);
        e.mem.set_u8(atom + 10, 0x10);
        e.mem.set_u32(this + 0x20, ids);
        e.mem.set_u32(this + 0x3c, atom);
        (this, atom)
    }

    #[test]
    fn contact_points_and_their_properties_come_from_the_id_table() {
        let mut e = Engine::new();
        let (this, atom) = manager_with_atom(&mut e);
        assert_eq!(
            e.call(0x00cf_d3f0, &args![this, 5u32]).u32(),
            atom + 0x30 + 0x40
        );
        assert_eq!(
            e.call(0x00cf_d410, &args![this, 5u32]).u32(),
            atom + 0x30 + 0x60 + 0x20
        );
        assert_eq!(e.call(0x00cf_d410, &args![this, 6u32]).u32(), 0);
        // The id is a 16-bit argument: the upper half of its stack word is ignored.
        assert_eq!(
            e.call(0x00cf_d410, &args![this, 0x0001_0005u32]).u32(),
            atom + 0x30 + 0x60 + 0x20
        );
    }

    #[test]
    fn the_small_constructor_sets_the_vtable_and_the_owner() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00cf_d450, &args![this, 0x4321u32]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_c8e4);
        assert_eq!(e.mem.u32(this + 8), 0x4321);
    }

    #[test]
    fn processing_contacts_fires_the_event_once_and_copies_the_points() {
        let mut e = Engine::new();
        let (this, atom) = manager_with_atom(&mut e);
        e.mem.set_u32(this + 0xc, 0x7400);
        e.mem.set_u32(this + 0x38, 0x5150);
        let ids = e.mem.u32(this + 0x20);
        e.mem.set_u8(ids, 1);
        e.mem.set_u8(ids + 1, 0xff);
        // The bodies are 0x20 above collidable A and 0x20 below collidable B.
        let block_a = e.mem.alloc(0x400);
        let block_b = e.mem.alloc(0x400);
        let collidable_a = block_a + 0x40;
        let collidable_b = block_b + 0x100;
        e.mem.set_u8(collidable_a + 0x10, 0x20);
        e.mem.set_u8(collidable_b + 0x10, 0xe0);
        let body_a = collidable_a + 0x20;
        let body_b = collidable_b - 0x20;
        e.mem.set_u16(body_a + 0xaa, 7);
        e.mem.set_u16(body_b + 0xaa, 5);
        e.mem.set_u16(body_a + 0x204, 1);
        // The result: two entries of 0x30 bytes (32 bytes of point, then the id).
        let result = e.mem.alloc(0x100);
        let object = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 8];
        slots[5] = 0x0900_0014;
        e.put_vtable(0x0900_2000, &slots);
        e.mem.set_u32(object, 0x0900_2000);
        e.mem.set_u32(result + 4, object);
        e.mem.set_u32(result, result + 0x70);
        for lane in 0..8 {
            e.mem.set_u32(result + 0x10 + 4 * lane, 0x100 + lane);
            e.mem.set_u32(result + 0x40 + 4 * lane, 0x200 + lane);
        }
        e.mem.set_u32(result + 0x30, 0);
        e.mem.set_u32(result + 0x60, 1);
        let log = new_log();
        log_event(
            &mut e,
            FIRE_CONTACT_PROCESS,
            &[0, 4, 0xc, 0x10, 0x14, 0x410],
            &log,
        );
        log_calls(&mut e, FIRE_CONTACT_PROCESS_INTERNAL, &log);
        log_calls(&mut e, 0x0900_0014, &log);
        e.call(
            0x00cf_d470,
            &args![this, collidable_a, collidable_b, 0u32, result],
        );
        {
            let log = log.borrow();
            assert_eq!(log.len(), 3);
            let event = log[0][2];
            assert_eq!(
                log[0],
                vec![
                    FIRE_CONTACT_PROCESS,
                    0x7400,
                    event,
                    collidable_a,
                    collidable_b,
                    result,
                    atom + 0xa0,
                    0,
                    this
                ]
            );
            assert_eq!(log[1], vec![FIRE_CONTACT_PROCESS_INTERNAL, body_a, event]);
            assert_eq!(log[2], vec![0x0900_0014, object, 0x5150, 8]);
        }
        assert_eq!(e.mem.u16(this + 0x12), 5);
        for lane in 0..8 {
            assert_eq!(e.mem.u32(atom + 0x50 + 4 * lane), 0x100 + lane);
            assert_eq!(e.mem.u32(atom + 0x30 + 0x1fe0 + 4 * lane), 0x200 + lane);
        }
        // A non-zero counter only counts down; the points are copied all the same.
        log.borrow_mut().clear();
        e.mem.set_u16(this + 0x12, 3);
        e.mem.set_u32(atom + 0x50, 0);
        e.call(
            0x00cf_d470,
            &args![this, collidable_a, collidable_b, 0u32, result],
        );
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u16(this + 0x12), 2);
        assert_eq!(e.mem.u32(atom + 0x50), 0x100);
    }

    #[test]
    fn the_statistics_name_the_atom_and_the_id_array() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        let atom = e.mem.alloc(0x40);
        e.mem.set_u16(atom + 2, 0x123);
        e.mem.set_u32(this + 0x3c, atom);
        e.mem.set_u32(this + 0x20, 0x6000);
        e.mem.set_u32(this + 0x24, 9);
        let collector = e.mem.alloc(0x40);
        let mut slots = vec![0u32; 8];
        slots[5] = 0x0900_0014;
        slots[6] = 0x0900_0018;
        slots[7] = 0x0900_001c;
        e.put_vtable(0x0900_2100, &slots);
        e.mem.set_u32(collector, 0x0900_2100);
        let log = new_log();
        for address in [0x0900_0014, 0x0900_0018, 0x0900_001c, ADD_NORMAL_CHUNK] {
            log_calls(&mut e, address, &log);
        }
        let sizes = new_log();
        let sizes_in_double = Rc::clone(&sizes);
        e.register_double(GET_ALLOCATED_SIZE, move |_, a| {
            sizes_in_double
                .borrow_mut()
                .push(vec![GET_ALLOCATED_SIZE, a[0], a[1]]);
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
        e.mem.set_u32(this + 0x28, 0x4000_0004);
        e.call(0x00cf_d5e0, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x0900_0018, collector, 0x010c_c90c],
                vec![ADD_NORMAL_CHUNK, collector, 0x010c_c900, atom, 0x123, 0],
                vec![0x0900_0014, collector, 3, 0x010c_c8f4, 0x6000, 9, 0x55],
                vec![0x0900_001c, collector],
            ]
        );
        assert_eq!(*sizes.borrow(), vec![vec![GET_ALLOCATED_SIZE, 0x6000, 4]]);
        // A negative capacity word skips the id array unless the collector asks for it.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x28, 0x8000_0004);
        e.call(0x00cf_d5e0, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 3);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00cf_d5e0, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 4);
        assert_eq!(sizes.borrow().len(), 2);
    }

    #[test]
    fn all_used_ids_are_appended_as_16_bit_numbers() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log, 0);
        let this = e.mem.alloc(0x100);
        let table = e.mem.alloc(0x10);
        for (index, value) in [0xffu8, 3, 0xff, 9].into_iter().enumerate() {
            e.mem.set_u8(table + index as u32, value);
        }
        e.mem.set_u32(this + 0x20, table);
        e.mem.set_u32(this + 0x24, 4);
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x2);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 8, 0x4000_0001);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            let grown = e.mem.alloc(0x20);
            let old = e.mem.u32(a[1]);
            let value = e.mem.u16(old);
            e.mem.set_u16(grown, value);
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 0x4000_0008);
            Ret::default()
        });
        e.call(0x00cf_d660, &args![this, out]);
        assert_eq!(e.mem.u32(out + 4), 2);
        let data = e.mem.u32(out);
        assert_eq!(e.mem.u16(data), 1);
        assert_eq!(e.mem.u16(data + 2), 3);
        // With no entries nothing is appended.
        e.mem.set_u32(this + 0x24, 0);
        e.call(0x00cf_d660, &args![this, out]);
        assert_eq!(e.mem.u32(out + 4), 2);
    }

    /// A world whose tables give priority `0x4455` to the pair of bodies' types (`-1` and
    /// `3`): returns the world and the two bodies.
    fn priority_world(e: &mut Engine) -> (u32, u32, u32) {
        let world = e.mem.alloc(0x100);
        let tables = e.mem.alloc(0x2100);
        e.mem.set_u32(world + 0x80, tables);
        let body_a = e.mem.alloc(0x80);
        let body_b = e.mem.alloc(0x80);
        e.mem.set_u8(body_a + 0x2a, 0xff);
        e.mem.set_u8(body_b + 0x2a, 3);
        // tables + 3 + (-1 * 5) * 2 + 0x1bb0 holds the index 2, whose entry is at 2 << 6.
        e.mem.set_u8(tables + 0x1bb0 - 7, 2);
        e.mem.set_u16(tables + 0x80 + 0x1c5c, 0x4455);
        (world, body_a, body_b)
    }

    const CONSTRUCTOR_CALLEES: [u32; 3] = [
        CONTACT_CONSTRAINT_DATA_CONSTRUCT,
        CONSTRAINT_INSTANCE_CONSTRUCT,
        CONSTRAINT_INSTANCE_SET_PRIORITY,
    ];

    #[test]
    fn the_contact_manager_constructor_sets_its_fields() {
        let mut e = Engine::new();
        let log = new_log();
        for address in CONSTRUCTOR_CALLEES {
            log_calls(&mut e, address, &log);
        }
        let (world, body_a, body_b) = priority_world(&mut e);
        let this = e.mem.alloc(0x100);
        for offset in [0x10, 0x12, 0x18, 0x48] {
            e.mem.set_u16(this + offset, 0xffff);
        }
        e.mem.set_u32(this + 8, 0xffff_ffff);
        let got = e
            .call(0x00cf_d670, &args![this, world, body_a, body_b])
            .u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![
                    CONTACT_CONSTRAINT_DATA_CONSTRUCT,
                    this + 0x14,
                    this + 0x44,
                    body_a,
                    body_b
                ],
                vec![CONSTRAINT_INSTANCE_CONSTRUCT, this + 0x44, 1],
                vec![CONSTRAINT_INSTANCE_SET_PRIORITY, this + 0x44, 0x4455],
            ]
        );
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_c984);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), world);
        assert_eq!(e.mem.u32(this + 0x50), this + 0x14);
        assert_eq!(e.mem.u32(this + 0x58), body_a);
        assert_eq!(e.mem.u32(this + 0x5c), body_b);
        assert_eq!(e.mem.u32(this + 0x78), 0xffff_ffff);
        for offset in [0x10, 0x12, 0x18, 0x48] {
            assert_eq!(e.mem.u16(this + offset), 0, "offset {offset:#x}");
        }
    }

    #[test]
    fn the_factory_allocates_and_constructs_a_contact_manager() {
        let mut e = Engine::new();
        let log = new_log();
        let block = e.mem.alloc(0x80);
        install_allocator(&mut e, &log, block);
        for address in CONSTRUCTOR_CALLEES {
            log_calls(&mut e, address, &log);
        }
        let (world, body_a, body_b) = priority_world(&mut e);
        let factory = e.mem.alloc(0x20);
        e.mem.set_u32(factory + 8, world);
        // The collidables sit 0x10 below A's body and 0x10 above B's.
        let collidable_a = body_a - 0x10;
        let collidable_b = body_b + 0x10;
        e.mem.set_u8(collidable_a + 0x10, 0x10);
        e.mem.set_u8(collidable_b + 0x10, 0xf0);
        let got = e
            .call(
                0x00cf_d780,
                &args![factory, collidable_a, collidable_b, 0u32],
            )
            .u32();
        assert_eq!(got, block);
        assert_eq!(e.mem.u16(block + 4), 0x80);
        let log = log.borrow();
        assert_eq!(log[0], vec![4, 0x80]);
        assert_eq!(
            log[1],
            vec![
                CONTACT_CONSTRAINT_DATA_CONSTRUCT,
                block + 0x14,
                block + 0x44,
                body_a,
                body_b
            ]
        );
        assert_eq!(e.mem.u32(block + 0xc), world);
    }

    #[test]
    fn the_contact_manager_destructor_removes_the_constraint_and_frees_its_blocks() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log, 0);
        for address in [REMOVE_CONSTRAINT_IMMEDIATELY, CONSTRAINT_INSTANCE_DESTRUCT] {
            log_calls(&mut e, address, &log);
        }
        e.register_double(ATOM_SIZE_ROUND_UP, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x100);
        let atom = e.mem.alloc(0x40);
        e.mem.set_u16(atom + 2, 0x30);
        e.mem.set_u16(atom + 4, 2);
        e.mem.set_u32(this + 0xc, 0x7500);
        e.mem.set_u32(this + 0x3c, atom);
        e.mem.set_u32(this + 0x20, 0x6000);
        e.mem.set_u32(this + 0x28, 0x4000_0006);
        e.mem.set_u32(this + 0x58, 1);
        e.mem.set_u32(this + 0x5c, 2);
        e.mem.set_u32(this + 0x50, 3);
        e.call(0x00cf_d720, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![REMOVE_CONSTRAINT_IMMEDIATELY, 0x7500, this + 0x44, 1],
                vec![CONSTRAINT_INSTANCE_DESTRUCT, this + 0x44],
                vec![8, atom, 0x77],
                vec![0x10, 0x6000, 6],
            ]
        );
        for offset in [0x50, 0x58, 0x5c, 0x24, 0x20] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x28), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // An atom without contact points is not removed from the world.
        log.borrow_mut().clear();
        e.mem.set_u16(atom + 4, 0);
        e.call(0x00cf_d720, &args![this]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(log.borrow()[0][0], CONSTRAINT_INSTANCE_DESTRUCT);
    }

    #[test]
    fn the_constraint_data_destructor_frees_the_atom_only_when_it_has_one() {
        let mut e = Engine::new();
        let log = new_log();
        install_allocator(&mut e, &log, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x6000);
        e.mem.set_u32(this + 0x10, 5);
        e.mem.set_u32(this + 0x14, 0x8000_0001);
        e.call(0x00cf_da50, &args![this]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn ids_above_the_threshold_are_decremented() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let table = e.mem.alloc(0x10);
        for (index, value) in [0u8, 1, 2, 3, 0xff, 2].into_iter().enumerate() {
            e.mem.set_u8(table + index as u32, value);
        }
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 4, 6);
        e.call(0x00cf_d7d0, &args![this, 1u32]);
        let got: Vec<u8> = (0..6).map(|index| e.mem.u8(table + index)).collect();
        assert_eq!(got, vec![0, 1, 1, 2, 0xff, 1]);
        // A negative threshold lowers every id that is not 0xff; an empty table is left alone.
        e.call(0x00cf_d7d0, &args![this, -1i32]);
        let got: Vec<u8> = (0..6).map(|index| e.mem.u8(table + index)).collect();
        assert_eq!(got, vec![0xff, 0, 0, 1, 0xff, 0]);
        e.mem.set_u32(this + 4, 0);
        e.call(0x00cf_d7d0, &args![this, -1i32]);
        assert_eq!(e.mem.u8(table), 0xff);
    }

    /// An atom of capacity three (header `u16` 3), `count` points `0x1000 * k + lane` and
    /// 6-byte property blocks starting `0xa0a0_a0a0 + k`.
    fn atom_with_points(e: &mut Engine, count: u16) -> u32 {
        let atom = e.mem.alloc(0x200);
        e.mem.set_u16(atom + 4, count);
        e.mem.set_u16(atom + 6, 3);
        e.mem.set_u8(atom + 10, 6);
        for point in 0..3 {
            for lane in 0..8 {
                e.mem
                    .set_u32(atom + 0x30 + 0x20 * point + 4 * lane, 0x1000 * point + lane);
            }
            e.mem.set_u32(atom + 0x90 + 6 * point, 0xa0a0_a0a0 + point);
        }
        atom
    }

    #[test]
    fn removing_a_contact_point_moves_the_later_ones_down() {
        let mut e = Engine::new();
        let atom = atom_with_points(&mut e, 3);
        e.call(0x00cf_d950, &args![atom, 0u32]);
        assert_eq!(e.mem.u16(atom + 4), 2);
        for lane in 0..8 {
            assert_eq!(e.mem.u32(atom + 0x30 + 4 * lane), 0x1000 + lane);
            assert_eq!(e.mem.u32(atom + 0x50 + 4 * lane), 0x2000 + lane);
            assert_eq!(e.mem.u32(atom + 0x70 + 4 * lane), 0x2000 + lane);
        }
        assert_eq!(e.mem.u32(atom + 0x90), 0xa0a0_a0a1);
        assert_eq!(e.mem.u32(atom + 0x96), 0xa0a0_a0a2);
        // The middle point: only the last one moves.
        let atom = atom_with_points(&mut e, 3);
        e.call(0x00cf_d950, &args![atom, 1u32]);
        assert_eq!(e.mem.u32(atom + 0x30), 0);
        assert_eq!(e.mem.u32(atom + 0x50), 0x2000);
        assert_eq!(e.mem.u32(atom + 0x90), 0xa0a0_a0a0);
        assert_eq!(e.mem.u32(atom + 0x96), 0xa0a0_a0a2);
        // The last point: only the count changes.
        let atom = atom_with_points(&mut e, 3);
        e.call(0x00cf_d950, &args![atom, 2u32]);
        assert_eq!(e.mem.u16(atom + 4), 2);
        assert_eq!(e.mem.u32(atom + 0x50), 0x1000);
        assert_eq!(e.mem.u32(atom + 0x70), 0x2000);
        assert_eq!(e.mem.u32(atom + 0x9c), 0xa0a0_a0a2);
    }
}
