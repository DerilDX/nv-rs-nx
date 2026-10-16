//! `Havok SDK/hkpentity.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9d160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d1_dff0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d160(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d1_dff0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c9d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_d3b0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d460(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_d3b0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00c9d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_cf60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9d490(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_cf60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpEntity` fields the translations of this file read, by their offsets (Xbox PDB names;
/// the PC offsets are the ones the code uses).
const ENTITY_WORLD: u32 = 0x08;
/// `m_collidable.m_shape` (the collidable starts at +0x10).
const ENTITY_SHAPE: u32 = 0x10;
/// `m_collidable.m_boundingVolumeData` (+0x30) and its child data pointer (+0x54).
const ENTITY_BOUNDING_DATA: u32 = 0x30;
const ENTITY_BOUNDING_CHILD_DATA: u32 = 0x54;
/// `m_constraintsSlave`: data pointer and size.
const ENTITY_SLAVES_DATA: u32 = 0xb4;
const ENTITY_SLAVES_SIZE: u32 = 0xb8;
/// `m_simulationIsland`.
const ENTITY_ISLAND: u32 = 0xcc;
/// `m_motion` (+0xe0) and the motion type byte (+0xe8).
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_MOTION_TYPE: u32 = 0xe8;
/// `m_extendedListeners`.
const ENTITY_EXTENDED_LISTENERS: u32 = 0x214;
/// The `float` (`0x7f7fffee`) `deactivate` stores in the two motion counters.
const DEACTIVATION_COUNTER_LIMIT: u32 = 0x0104_da48;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB, cdecl `(collision input, entities,
/// count)`).
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpWorldOperationUtil::splitSimulationIsland_ov2` (Xbox PDB, cdecl `(world, island)`).
const SPLIT_SIMULATION_ISLAND: u32 = 0x00cb_6e70;
/// `hkpWorldOperationUtil::markIslandInactive` (Xbox PDB, cdecl `(world, island)`).
const MARK_ISLAND_INACTIVE: u32 = 0x00cb_5420;
/// Island activation of the world operation util (`hkpworldoperationutil.cpp`, cdecl `(world,
/// island)`; no name in the engine map).
const ACTIVATE_ISLAND: u32 = 0x00cb_5500;
/// A member function of the motion (`this` = `entity + 0xe0`, three byte-sized arguments read
/// from the world at +0x305, +0x306 and +0x307; the callee cleans the stack).
const MOTION_SET_DEACTIVATION_FLAGS: u32 = 0x00c9_0ed0;
/// `hkpCollidable::BoundingVolumeData::deallocate` and `::allocate` (Xbox PDB; `this` = the
/// entity + 0x30; `allocate` takes the size).
const BOUNDING_DATA_DEALLOCATE: u32 = 0x00d0_1a80;
const BOUNDING_DATA_ALLOCATE: u32 = 0x00d0_1ae0;
/// The small-array growth helper (cdecl `(array, element size)`; no name in the engine map).
const SMALL_ARRAY_GROW: u32 = 0x00d0_6cb0;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The extended listeners block of the entity (`+0x214`), made on first use: 0x10 bytes from
/// the allocator (slot 4), holding two empty small arrays (capacity word `0x8000`). A failed
/// allocation stores a null pointer, which the code then dereferences.
fn extended_listeners(e: &mut Engine, entity: u32) -> u32 {
    if e.mem.u32(entity + ENTITY_EXTENDED_LISTENERS) == 0 {
        let allocator = router_allocator(e);
        let block = e.vcall(allocator, 4, &args![0x10u32]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u16(block + 4, 0);
            e.mem.set_u16(block + 6, 0x8000);
            e.mem.set_u32(block + 8, 0);
            e.mem.set_u16(block + 0xc, 0);
            e.mem.set_u16(block + 0xe, 0x8000);
        }
        e.mem.set_u32(entity + ENTITY_EXTENDED_LISTENERS, block);
    }
    e.mem.u32(entity + ENTITY_EXTENDED_LISTENERS)
}

// Translated from 00c9bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::updateCachedAabb` (Xbox PDB): with a world and a shape, recalculates the AABB of
/// this one entity through the world's collision input (`world + 0x78`).
pub fn hkp_entity_update_cached_aabb(e: &mut Engine, this: Ptr) {
    let world = e.mem.u32(this.addr() + ENTITY_WORLD);
    if world != 0 && e.mem.u32(this.addr() + ENTITY_SHAPE) != 0 {
        let input = e.mem.u32(world + 0x78);
        e.with_stack(4, |e, list| {
            e.mem.set_u32(list.addr(), this.addr());
            e.call(ENTITY_BATCH_RECALC_AABB, &args![input, list, 1u32]);
        });
    }
}

// Translated from 00c9bff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::isActive` (Xbox PDB): writes the island's activation state (bits 2 and 3 of the
/// island byte at +0x26) to the byte at `result`, 0 for an entity without an island.
pub fn hkp_entity_is_active(e: &mut Engine, this: Ptr, result: Ptr) {
    let island = e.mem.u32(this.addr() + ENTITY_ISLAND);
    let state = if island == 0 {
        0
    } else {
        (e.mem.u8(island + 0x26) >> 2) & 3
    };
    e.mem.set_u8(result.addr(), state);
}

// Translated from 00c9c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::deactivate` (Xbox PDB): for an entity whose island is active, first (when the
/// island byte at +0x25 has any of its low four bits set) splits the island with the world's
/// word at +0xb4 cleared meanwhile, then sets both motion counters (+0x1dc, +0x1ec) of every
/// entity of the island (list at island +0x48, count at +0x4c) to the large `float` at
/// `0104da48`, and marks the island inactive.
pub fn hkp_entity_deactivate(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if island == 0 || (e.mem.u8(island + 0x26) >> 2) & 3 == 0 {
        return;
    }
    if e.mem.u8(island + 0x25) & 0xf != 0 {
        let world = e.mem.u32(me + ENTITY_WORLD);
        let saved = e.mem.u32(world + 0xb4);
        e.mem.set_u32(world + 0xb4, 0);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let flags = e.mem.u8(island + 0x25);
        e.mem.set_u8(island + 0x25, flags & 0xf3);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let flags = e.mem.u8(island + 0x25);
        e.mem.set_u8(island + 0x25, (flags & 0xfd) | 1);
        let island = e.mem.u32(me + ENTITY_ISLAND);
        let world = e.mem.u32(me + ENTITY_WORLD);
        e.call(SPLIT_SIMULATION_ISLAND, &args![world, island]);
        let world = e.mem.u32(me + ENTITY_WORLD);
        e.mem.set_u32(world + 0xb4, saved);
    }
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if e.mem.i32(island + 0x4c) > 0 {
        let limit = x87_quiet(e.mem.f32(DEACTIVATION_COUNTER_LIMIT));
        let mut slot = e.mem.u32(island + 0x48);
        let mut index = 0;
        loop {
            let entity = e.mem.u32(slot);
            e.mem.set_f32(entity + 0x1dc, limit);
            index += 1;
            e.mem.set_f32(entity + 0x1ec, limit);
            slot += 4;
            if index >= e.mem.i32(island + 0x4c) {
                break;
            }
        }
    }
    let world = e.mem.u32(me + ENTITY_WORLD);
    e.call(MARK_ISLAND_INACTIVE, &args![world, island]);
}

/// `cmpLessConstraint` of `hkpentity.cpp`'s anonymous namespace: constraint `a` sorts before
/// `b` when its priority word (+0x34, unsigned) is lower, or equal and the `m_uid` (+0xd4) of
/// its other entity (the xor of the two entity pointers at +0x14 and +0x18 with `owner`) is
/// lower.
fn constraint_less(e: &mut Engine, a: u32, b: u32, owner: u32) -> bool {
    let key_a = e.mem.u32(a + 0x34);
    let key_b = e.mem.u32(b + 0x34);
    if key_a != key_b {
        return key_a < key_b;
    }
    let other_a = e.mem.u32(a + 0x18) ^ e.mem.u32(a + 0x14) ^ owner;
    let other_b = e.mem.u32(b + 0x18) ^ e.mem.u32(b + 0x14) ^ owner;
    e.mem.u32(other_a + 0xd4) < e.mem.u32(other_b + 0xd4)
}

fn array_slot(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(4))
}

// Translated from 00c9d250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpConstraintInstance, cmpLessConstraint>` (Xbox PDB):
/// sorts the constraint pointers `array[low..=high]` with `constraint_less` (middle pivot,
/// Hoare partition, recursion on the lower part, loop on the upper); `owner` is the entity
/// whose slave constraints these are.
pub fn hk_algorithm_quick_sort_recursive_cmp_less_constraint(
    e: &mut Engine,
    array: u32,
    low: i32,
    high: i32,
    owner: u32,
) {
    let mut low = low;
    loop {
        let pivot = e.mem.u32(array_slot(array, (low + high) >> 1));
        let mut left = low;
        let mut right = high;
        loop {
            loop {
                let item = e.mem.u32(array_slot(array, left));
                if !constraint_less(e, item, pivot, owner) {
                    break;
                }
                left += 1;
            }
            loop {
                let item = e.mem.u32(array_slot(array, right));
                if !constraint_less(e, pivot, item, owner) {
                    break;
                }
                right -= 1;
            }
            if right < left {
                break;
            }
            if right != left {
                let from_right = e.mem.u32(array_slot(array, right));
                let from_left = e.mem.u32(array_slot(array, left));
                e.mem.set_u32(array_slot(array, right), from_left);
                e.mem.set_u32(array_slot(array, left), from_right);
            }
            left += 1;
            right -= 1;
            if left > right {
                break;
            }
        }
        if low < right {
            hk_algorithm_quick_sort_recursive_cmp_less_constraint(e, array, low, right, owner);
        }
        low = left;
        if low >= high {
            return;
        }
    }
}

// Translated from 00c9c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::sortConstraintsSlavesDeterministically` (Xbox PDB): for a motion of type 5 sorts
/// the slave constraint list (at +0xb4, count +0xb8) when it has more than one entry, then
/// writes each constraint's new position into the 16-bit field at +0x22 of its entry in the
/// other entity's master list (data at +0xac, count 16-bit at +0xb0, 0x30 bytes per entry, the
/// constraint pointer first).
pub fn hkp_entity_sort_constraints_slaves_deterministically(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.mem.u8(me + ENTITY_MOTION_TYPE) != 5 {
        return;
    }
    let count = e.mem.i32(me + ENTITY_SLAVES_SIZE);
    if count > 1 {
        let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
        hk_algorithm_quick_sort_recursive_cmp_less_constraint(e, data, 0, count - 1, me);
    }
    let mut position = 0i32;
    while position < e.mem.i32(me + ENTITY_SLAVES_SIZE) {
        let data = e.mem.u32(me + ENTITY_SLAVES_DATA);
        let constraint = e.mem.u32(array_slot(data, position));
        let other = e.mem.u32(constraint + 0x18) ^ e.mem.u32(constraint + 0x14) ^ me;
        let entries = u32::from(e.mem.u16(other + 0xb0));
        let masters = e.mem.u32(other + 0xac);
        for entry in 0..entries {
            if e.mem.u32(masters + entry * 0x30) == constraint {
                e.mem
                    .set_u16(masters + entry * 0x30 + 0x22, position as u16);
                break;
            }
        }
        position += 1;
    }
}

// Translated from 00c9c040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::setCachedShapeData` (Xbox PDB): releases the entity's bounding-volume data when
/// it has child data (+0x54), then, for a `world` and `shape` (both required), looks up the
/// shape type (`shape + 0xc`) in the world's dispatcher table (`world + 0x80`, entries at
/// +0x110). A negative low byte takes the shape's slot `0x10` result; otherwise bit 8 must be
/// set (and then the entity must not be of motion type 4 or 5). Allocates bounding-volume data
/// of the size that result's slot `4` answers, sets `+0x30 = 1`, `+0x40 = 0` and, for motion
/// type 5, recalculates the AABB.
pub fn hkp_entity_set_cached_shape_data(e: &mut Engine, this: Ptr, world: Ptr, shape: Ptr) {
    let me = this.addr();
    if e.mem.u32(me + ENTITY_BOUNDING_CHILD_DATA) != 0 {
        e.call(BOUNDING_DATA_DEALLOCATE, &args![me + ENTITY_BOUNDING_DATA]);
    }
    if world.is_null() || shape.is_null() {
        return;
    }
    let shape_type = e.mem.u32(shape.addr() + 0xc);
    let dispatcher = e.mem.u32(world.addr() + 0x80);
    let flags = e.mem.u32(dispatcher + 0x110 + shape_type * 4);
    let second_form = (flags as u8) & 0x80 == 0;
    if second_form && flags & 0x100 == 0 {
        return;
    }
    let inner = e.vcall(shape.addr(), 0x10, &args![]).u32();
    let motion_type = e.mem.u8(me + ENTITY_MOTION_TYPE);
    if (motion_type == 5 || motion_type == 4) && second_form {
        return;
    }
    let size = e.vcall(inner, 4, &args![]).u32();
    e.call(
        BOUNDING_DATA_ALLOCATE,
        &args![me + ENTITY_BOUNDING_DATA, size],
    );
    e.mem.set_u32(me + ENTITY_BOUNDING_DATA, 1);
    e.mem.set_u32(me + 0x40, 0);
    if e.mem.u8(me + ENTITY_MOTION_TYPE) == 5 {
        hkp_entity_update_cached_aabb(e, this);
    }
}

// Translated from 00c9c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEntity::activate` (Xbox PDB): for an entity without an active island, whose motion type
/// is not 5 and which has a world, clears the two words at +0xea and +0xec, activates the
/// island through the world operation util and passes the world's three flag bytes
/// (+0x305, +0x306, +0x307) to the motion (`this + 0xe0`).
pub fn hkp_entity_activate(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let island = e.mem.u32(me + ENTITY_ISLAND);
    if island != 0 && (e.mem.u8(island + 0x26) >> 2) & 3 != 0 {
        return;
    }
    if e.mem.u8(me + ENTITY_MOTION_TYPE) == 5 {
        return;
    }
    let world = e.mem.u32(me + ENTITY_WORLD);
    if world == 0 {
        return;
    }
    e.mem.set_u16(me + 0xea, 0);
    e.mem.set_u16(me + 0xec, 0);
    e.call(ACTIVATE_ISLAND, &args![world, island]);
    let world = e.mem.u32(me + ENTITY_WORLD);
    let first = u32::from(e.mem.u8(world + 0x305));
    let second = u32::from(e.mem.u8(world + 0x306));
    let third = u32::from(e.mem.u8(world + 0x307));
    e.call(
        MOTION_SET_DEACTIVATION_FLAGS,
        &args![me + ENTITY_MOTION, first, second, third],
    );
}

// Translated from 00c9c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): adds `listener` to the second small array of the entity's
/// extended listeners block (`+0x214`, array at +8 of the block; made on first use). Reuses the
/// first null entry; otherwise grows the array (`00d06cb0`) when it is full and appends.
pub fn fn_00c9c5b0(e: &mut Engine, this: Ptr, listener: u32) {
    let block = extended_listeners(e, this.addr());
    let array = block + 8;
    let size = u32::from(e.mem.u16(block + 0xc));
    if size > 0 {
        let data = e.mem.u32(array);
        let mut index = 0u32;
        loop {
            if e.mem.u32(data + index * 4) == 0 {
                e.mem.set_u32(data + index * 4, listener);
                return;
            }
            index += 1;
            if index >= u32::from(e.mem.u16(array + 4)) {
                break;
            }
        }
    }
    let capacity = e.mem.u16(array + 6) & 0x3fff;
    if size as u16 == capacity {
        e.call(SMALL_ARRAY_GROW, &args![array, 4u32]);
    }
    let count = u32::from(e.mem.u16(array + 4));
    let data = e.mem.u32(array);
    e.mem.set_u32(data + count * 4, listener);
    e.mem.set_u16(array + 4, (count as u16).wrapping_add(1));
}

// Translated from 00c9c670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown): in the second small array of the entity's extended listeners
/// block (made on first use) sets the first entry equal to `listener` to null; when there is
/// none (or the array is empty) writes the null to the element before the data, as the code
/// does.
pub fn fn_00c9c670(e: &mut Engine, this: Ptr, listener: u32) {
    let block = extended_listeners(e, this.addr());
    let array = block + 8;
    let count = u32::from(e.mem.u16(block + 0xc));
    let data = e.mem.u32(array);
    for index in 0..count {
        if e.mem.u32(data + index * 4) == listener {
            e.mem.set_u32(data + index * 4, 0);
            return;
        }
    }
    e.mem.set_u32(data.wrapping_sub(4), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_d160, deleting_destructor_00c9d160(Ptr, u8) -> Ptr),
        entry!(0x00c9_d460, deleting_destructor_00c9d460(Ptr, u8) -> Ptr),
        entry!(0x00c9_d490, deleting_destructor_00c9d490(Ptr, u8) -> Ptr),
        entry!(0x00c9_bfa0, hkp_entity_update_cached_aabb(Ptr)),
        entry!(0x00c9_bff0, hkp_entity_is_active(Ptr, Ptr)),
        entry!(0x00c9_c240, hkp_entity_deactivate(Ptr)),
        entry!(
            0x00c9_d250,
            hk_algorithm_quick_sort_recursive_cmp_less_constraint(u32, i32, i32, u32)
        ),
        entry!(
            0x00c9_c860,
            hkp_entity_sort_constraints_slaves_deterministically(Ptr)
        ),
        entry!(0x00c9_c040, hkp_entity_set_cached_shape_data(Ptr, Ptr, Ptr)),
        entry!(0x00c9_c1d0, hkp_entity_activate(Ptr)),
        entry!(0x00c9_c5b0, fn_00c9c5b0(Ptr, u32)),
        entry!(0x00c9_c670, fn_00c9c670(Ptr, u32)),
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
    fn deleting_destructor_00c9d160_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d160, 0x00d1_dff0);
    }

    #[test]
    fn deleting_destructor_00c9d460_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d460, 0x00c9_d3b0);
    }

    #[test]
    fn deleting_destructor_00c9d490_frees_only_when_bit_zero_is_set() {
        check(0x00c9_d490, 0x00c9_cf60);
    }

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// Registers a double at `address` that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &CallLog, address: u32, result: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn update_cached_aabb_needs_a_world_and_a_shape() {
        let mut e = engine();
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(ENTITY_BATCH_RECALC_AABB, move |e, a| {
            let first = e.mem.u32(a[1]);
            seen.borrow_mut()
                .push((ENTITY_BATCH_RECALC_AABB, vec![a[0], first, a[2]]));
            Ret::default()
        });
        let entity = e.mem.alloc(0x220);
        e.call(0x00c9_bfa0, &args![entity]);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x78, 0x1234);
        e.mem.set_u32(entity + 8, world);
        e.call(0x00c9_bfa0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(entity + 0x10, 0x5555);
        e.call(0x00c9_bfa0, &args![entity]);
        assert_eq!(
            calls_to(&log, ENTITY_BATCH_RECALC_AABB),
            vec![vec![0x1234, entity, 1]]
        );
    }

    #[test]
    fn is_active_reads_the_island_state_bits() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0x77);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 0);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0b1011_0100);
        e.mem.set_u32(entity + 0xcc, island);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u8(island + 0x26, 0b1111_1100);
        e.call(0x00c9_bff0, &args![entity, out]);
        assert_eq!(e.mem.u8(out), 3);
    }

    struct DeactivateScene {
        entity: u32,
        world: u32,
        island: u32,
        members: [u32; 2],
    }

    fn deactivate_scene(e: &mut Engine, island_flags: u8) -> DeactivateScene {
        e.mem.set_u32(0x0104_da48, 0x7f7f_ffee);
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        let island = e.mem.alloc(0x80);
        let members = [e.mem.alloc(0x220), e.mem.alloc(0x220)];
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, members[0]);
        e.mem.set_u32(list + 4, members[1]);
        e.mem.set_u32(island + 0x48, list);
        e.mem.set_u32(island + 0x4c, 2);
        e.mem.set_u8(island + 0x25, island_flags);
        e.mem.set_u8(island + 0x26, 0b0000_0100);
        e.mem.set_u32(entity + 8, world);
        e.mem.set_u32(entity + 0xcc, island);
        e.mem.set_u32(world + 0xb4, 0xabcd);
        DeactivateScene {
            entity,
            world,
            island,
            members,
        }
    }

    #[test]
    fn deactivate_splits_the_island_and_marks_it_inactive() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0x02);
        let log = new_log();
        let seen = Rc::clone(&log);
        e.register_double(SPLIT_SIMULATION_ISLAND, move |e, a| {
            // The world's word is cleared and the island's flags rewritten at the call.
            seen.borrow_mut().push((
                SPLIT_SIMULATION_ISLAND,
                vec![
                    a[0],
                    a[1],
                    e.mem.u32(a[0] + 0xb4),
                    u32::from(e.mem.u8(a[1] + 0x25)),
                ],
            ));
            Ret::default()
        });
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert_eq!(
            calls_to(&log, SPLIT_SIMULATION_ISLAND),
            vec![vec![scene.world, scene.island, 0, 0x01]]
        );
        assert_eq!(e.mem.u32(scene.world + 0xb4), 0xabcd);
        for member in scene.members {
            assert_eq!(e.mem.u32(member + 0x1dc), 0x7f7f_ffee);
            assert_eq!(e.mem.u32(member + 0x1ec), 0x7f7f_ffee);
        }
        assert_eq!(
            calls_to(&log, MARK_ISLAND_INACTIVE),
            vec![vec![scene.world, scene.island]]
        );
        let order: Vec<u32> = log.borrow().iter().map(|(at, _)| *at).collect();
        assert_eq!(order, vec![SPLIT_SIMULATION_ISLAND, MARK_ISLAND_INACTIVE]);
    }

    #[test]
    fn deactivate_without_split_flags_only_resets_counters() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0xf0);
        let log = new_log();
        record(&mut e, &log, SPLIT_SIMULATION_ISLAND, 0);
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert!(calls_to(&log, SPLIT_SIMULATION_ISLAND).is_empty());
        assert_eq!(calls_to(&log, MARK_ISLAND_INACTIVE).len(), 1);
        assert_eq!(e.mem.u32(scene.members[1] + 0x1ec), 0x7f7f_ffee);
    }

    #[test]
    fn deactivate_ignores_an_entity_without_an_active_island() {
        let mut e = engine();
        let scene = deactivate_scene(&mut e, 0x02);
        let log = new_log();
        record(&mut e, &log, MARK_ISLAND_INACTIVE, 0);
        e.mem.set_u8(scene.island + 0x26, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        e.mem.set_u32(scene.entity + 0xcc, 0);
        e.call(0x00c9_c240, &args![scene.entity]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(scene.members[0] + 0x1dc), 0);
    }

    /// A constraint with priority `key` whose other entity (reached through the xor of the
    /// two entity words with `owner`) has the uid `uid`.
    fn make_constraint(e: &mut Engine, owner: u32, key: u32, uid: u32) -> (u32, u32) {
        let other = e.mem.alloc(0x220);
        e.mem.set_u32(other + 0xd4, uid);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0x34, key);
        e.mem.set_u32(constraint + 0x14, owner);
        e.mem.set_u32(constraint + 0x18, other);
        (constraint, other)
    }

    #[test]
    fn quick_sort_orders_by_priority_then_uid() {
        let mut e = engine();
        let owner = 0x1000;
        let specs = [(3, 50), (1, 40), (2, 10), (1, 5), (3, 7), (2, 99), (0, 60)];
        let mut items = Vec::new();
        for (key, uid) in specs {
            items.push((make_constraint(&mut e, owner, key, uid).0, (key, uid)));
        }
        let array = e.mem.alloc(4 * items.len() as u32);
        for (index, (constraint, _)) in items.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *constraint);
        }
        e.call(
            0x00c9_d250,
            &args![array, 0u32, items.len() as u32 - 1, owner],
        );
        let mut expected = items.clone();
        expected.sort_by_key(|(_, order)| *order);
        for (index, (constraint, _)) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(array + 4 * index as u32), *constraint);
        }
    }

    #[test]
    fn quick_sort_leaves_a_single_element_range_alone() {
        let mut e = engine();
        let array = e.mem.alloc(8);
        let item = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(array, item);
        e.mem.set_u32(array + 4, 0x2222);
        e.call(0x00c9_d250, &args![array, 0u32, 0u32, owner]);
        assert_eq!(e.mem.u32(array), item);
        assert_eq!(e.mem.u32(array + 4), 0x2222);
    }

    #[test]
    fn sort_slaves_sorts_and_numbers_the_masters_entries() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let specs = [(2, 30), (1, 20), (1, 10)];
        let mut made = Vec::new();
        for (key, uid) in specs {
            let (constraint, other) = make_constraint(&mut e, entity, key, uid);
            // The other entity's master list: a decoy first, then the constraint.
            let masters = e.mem.alloc(0x60);
            e.mem.set_u32(masters, 0xdead);
            e.mem.set_u32(masters + 0x30, constraint);
            e.mem.set_u32(other + 0xac, masters);
            e.mem.set_u16(other + 0xb0, 2);
            made.push((constraint, masters, (key, uid)));
        }
        let slaves = e.mem.alloc(12);
        for (index, (constraint, _, _)) in made.iter().enumerate() {
            e.mem.set_u32(slaves + 4 * index as u32, *constraint);
        }
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_i32(entity + 0xb8, 3);
        // Motion type 4: untouched.
        e.mem.set_u8(entity + 0xe8, 4);
        e.call(0x00c9_c860, &args![entity]);
        assert_eq!(e.mem.u32(slaves), made[0].0);
        e.mem.set_u8(entity + 0xe8, 5);
        e.call(0x00c9_c860, &args![entity]);
        let mut expected = made.clone();
        expected.sort_by_key(|(_, _, order)| *order);
        for (position, (constraint, masters, _)) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(slaves + 4 * position as u32), *constraint);
            assert_eq!(e.mem.u16(masters + 0x30 + 0x22), position as u16);
            assert_eq!(e.mem.u16(masters + 0x22), 0);
        }
    }

    const SHAPE_VTABLE_SLOT_DOUBLE: u32 = 0x0900_0020;
    const INNER_SIZE_SLOT_DOUBLE: u32 = 0x0900_0021;

    struct ShapeScene {
        entity: u32,
        world: u32,
        shape: u32,
        inner: u32,
    }

    fn shape_scene(e: &mut Engine, flags: u32) -> ShapeScene {
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        let dispatcher = e.mem.alloc(0x200);
        e.mem.set_u32(world + 0x80, dispatcher);
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, 5);
        e.mem.set_u32(dispatcher + 0x110 + 5 * 4, flags);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, SHAPE_VTABLE_SLOT_DOUBLE);
        e.mem.set_u32(shape, vtable);
        let inner = e.mem.alloc(0x40);
        let inner_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(inner_vtable + 4, INNER_SIZE_SLOT_DOUBLE);
        e.mem.set_u32(inner, inner_vtable);
        e.register_double(SHAPE_VTABLE_SLOT_DOUBLE, move |_, _| Ret {
            eax: inner,
            ..Ret::default()
        });
        e.register_double(INNER_SIZE_SLOT_DOUBLE, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        ShapeScene {
            entity,
            world,
            shape,
            inner,
        }
    }

    #[test]
    fn set_cached_shape_data_releases_old_data_first() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_DEALLOCATE, 0);
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        e.mem.set_u32(scene.entity + 0x54, 0x4444);
        e.call(0x00c9_c040, &args![scene.entity, 0u32, scene.shape]);
        assert_eq!(
            calls_to(&log, BOUNDING_DATA_DEALLOCATE),
            vec![vec![scene.entity + 0x30]]
        );
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.call(0x00c9_c040, &args![scene.entity, scene.world, 0u32]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
    }

    #[test]
    fn set_cached_shape_data_allocates_for_a_flagged_shape() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        record(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.mem.set_u32(scene.entity + 0x40, 9);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(
            calls_to(&log, BOUNDING_DATA_ALLOCATE),
            vec![vec![scene.entity + 0x30, 0x77]]
        );
        assert_eq!(e.mem.u32(scene.entity + 0x30), 1);
        assert_eq!(e.mem.u32(scene.entity + 0x40), 0);
        assert!(calls_to(&log, ENTITY_BATCH_RECALC_AABB).is_empty());
        assert_ne!(scene.inner, 0);
    }

    #[test]
    fn set_cached_shape_data_second_form_needs_bit_eight_and_a_dynamic_motion() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x100);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        e.mem.set_u8(scene.entity + 0xe8, 5);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.mem.set_u8(scene.entity + 0xe8, 4);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert!(calls_to(&log, BOUNDING_DATA_ALLOCATE).is_empty());
        e.mem.set_u8(scene.entity + 0xe8, 1);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(calls_to(&log, BOUNDING_DATA_ALLOCATE).len(), 1);
        let none = shape_scene(&mut e, 0x01);
        e.call(0x00c9_c040, &args![none.entity, none.world, none.shape]);
        assert_eq!(calls_to(&log, BOUNDING_DATA_ALLOCATE).len(), 1);
    }

    #[test]
    fn set_cached_shape_data_recalculates_the_aabb_for_motion_type_five() {
        let mut e = engine();
        let scene = shape_scene(&mut e, 0x80);
        let log = new_log();
        record(&mut e, &log, BOUNDING_DATA_ALLOCATE, 0);
        record(&mut e, &log, ENTITY_BATCH_RECALC_AABB, 0);
        e.mem.set_u8(scene.entity + 0xe8, 5);
        e.mem.set_u32(scene.entity + 8, scene.world);
        e.mem.set_u32(scene.entity + 0x10, scene.shape);
        e.call(0x00c9_c040, &args![scene.entity, scene.world, scene.shape]);
        assert_eq!(calls_to(&log, ENTITY_BATCH_RECALC_AABB).len(), 1);
    }

    #[test]
    fn activate_runs_for_an_inactive_non_fixed_entity_with_a_world() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u8(world + 0x307, 3);
        let log = new_log();
        record(&mut e, &log, ACTIVATE_ISLAND, 0);
        record(&mut e, &log, MOTION_SET_DEACTIVATION_FLAGS, 0);
        // No world: nothing happens.
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(entity + 8, world);
        e.mem.set_u16(entity + 0xea, 5);
        e.mem.set_u16(entity + 0xec, 6);
        e.call(0x00c9_c1d0, &args![entity]);
        assert_eq!(e.mem.u16(entity + 0xea), 0);
        assert_eq!(e.mem.u16(entity + 0xec), 0);
        assert_eq!(calls_to(&log, ACTIVATE_ISLAND), vec![vec![world, 0]]);
        assert_eq!(
            calls_to(&log, MOTION_SET_DEACTIVATION_FLAGS),
            vec![vec![entity + 0xe0, 1, 2, 3]]
        );
    }

    #[test]
    fn activate_skips_active_islands_and_motion_type_five() {
        let mut e = engine();
        let entity = e.mem.alloc(0x220);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(entity + 8, world);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0b0000_1000);
        e.mem.set_u32(entity + 0xcc, island);
        let log = new_log();
        record(&mut e, &log, ACTIVATE_ISLAND, 0);
        record(&mut e, &log, MOTION_SET_DEACTIVATION_FLAGS, 0);
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        // An inactive island is passed on.
        e.mem.set_u8(island + 0x26, 0);
        e.mem.set_u8(entity + 0xe8, 5);
        e.call(0x00c9_c1d0, &args![entity]);
        assert!(log.borrow().is_empty());
        e.mem.set_u8(entity + 0xe8, 2);
        e.call(0x00c9_c1d0, &args![entity]);
        assert_eq!(calls_to(&log, ACTIVATE_ISLAND), vec![vec![world, island]]);
    }

    const ALLOCATOR_SLOT_DOUBLE: u32 = 0x0900_0030;

    /// A memory router whose allocator hands out heap blocks, and a double for the small
    /// array growth helper that gives the array room for four entries.
    fn listener_scene(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATOR_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATOR_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(SMALL_ARRAY_GROW, |e, a| {
            let room = e.mem.alloc(16);
            e.mem.set_u32(a[0], room);
            e.mem.set_u16(a[0] + 6, 4);
            Ret::default()
        });
        e.mem.alloc(0x220)
    }

    #[test]
    fn listener_add_makes_the_block_and_reuses_null_entries() {
        let mut e = engine();
        let entity = listener_scene(&mut e);
        e.call(0x00c9_c5b0, &args![entity, 7u32]);
        let block = e.mem.u32(entity + 0x214);
        assert_ne!(block, 0);
        assert_eq!(e.mem.u16(block + 0xc), 1);
        assert_eq!(e.mem.u16(block + 0xe), 4);
        let data = e.mem.u32(block + 8);
        assert_eq!(e.mem.u32(data), 7);
        e.call(0x00c9_c5b0, &args![entity, 8u32]);
        assert_eq!(e.mem.u32(data + 4), 8);
        assert_eq!(e.mem.u16(block + 0xc), 2);
        // Remove the first, then a new listener takes its place instead of growing the array.
        e.call(0x00c9_c670, &args![entity, 7u32]);
        assert_eq!(e.mem.u32(data), 0);
        e.call(0x00c9_c5b0, &args![entity, 9u32]);
        assert_eq!(e.mem.u32(data), 9);
        assert_eq!(e.mem.u16(block + 0xc), 2);
        // A listener in the first array (+0) is not touched.
        assert_eq!(e.mem.u16(block + 4), 0);
    }

    #[test]
    fn listener_remove_writes_before_the_data_when_absent() {
        let mut e = engine();
        let entity = listener_scene(&mut e);
        let block = e.mem.alloc(0x10);
        let region = e.mem.alloc(0x10);
        e.mem.set_u32(region, 0xaaaa);
        e.mem.set_u32(region + 4, 5);
        e.mem.set_u32(block + 8, region + 4);
        e.mem.set_u16(block + 0xc, 1);
        e.mem.set_u32(entity + 0x214, block);
        e.call(0x00c9_c670, &args![entity, 6u32]);
        assert_eq!(e.mem.u32(region), 0);
        assert_eq!(e.mem.u32(region + 4), 5);
    }
}
