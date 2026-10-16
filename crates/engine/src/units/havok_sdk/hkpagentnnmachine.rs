//! `Havok SDK/hkpagentnnmachine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions work on agent entries of 0x80 bytes (`hkpAgentNnEntry` and its agent data),
//! kept in 0x200-byte sectors of a track (`hkpAgentNnTrack`, Xbox PDB: `m_bytesUsedInLastSector`
//! at +0, then the `hkInplaceArray` of sectors at +4: data, size, capacity word, inplace
//! storage). An entry links two collidables (pointers at +0x10 and +0x14); each collidable lists
//! its entries in `m_collisionEntries` (data +0x50, size +0x54, capacity word +0x58; 8 bytes per
//! element: entry, partner collidable), and the entry's `m_agentIndexOnCollidable` (16-bit words
//! at +4 and +6) say where. The collision dispatcher has one 0x50-byte record per agent type
//! (the entry's byte at +1) with function pointers at +0x16b4 (destroy), +0x16c8 (update shape
//! collection filter) and +0x16d4 (warp time).
//!
//! SSE code computes lane by lane in `f32`; floats copied through `FLD`/`FSTP` go through
//! [`x87_quiet`] and [`x87_quiet_bits`].

#[allow(unused_imports)]
use crate::prelude::*;

/// Size of an agent entry and of a sector (four entries).
const ENTRY_SIZE: u32 = 0x80;
const SECTOR_SIZE: u32 = 0x200;
/// `hkMemoryRouter` lookup (`00c85750`, no arguments); its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reduce` (Xbox PDB, `00c909d0`, cdecl `(allocator, array, element size,
/// inplace storage, new capacity)`).
const ARRAY_REDUCE: u32 = 0x00c9_09d0;
/// Array insertion (`00cd1e20`, `this` = the array, `(allocator, index, source, count)`; the
/// callee cleans the stack).
const ARRAY_INSERT_AT: u32 = 0x00cd_1e20;
/// `hkQuaternion::setMul` (Xbox PDB, `00c66320`, `this` = result, `(a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The `float` the warp-time function stores when the stored time does not match.
const WARP_MISMATCH_TIME: u32 = 0x0101_2054;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Frees a block of `size` bytes through the router's allocator (vtable slot 8).
fn router_free(e: &mut Engine, block: u32, size: u32) {
    let allocator = router_allocator(e);
    e.vcall(allocator, 8, &args![block, size]);
}

/// The agent function the dispatcher keeps for the entry's agent type (byte at +1) at table
/// offset `table`.
fn dispatcher_function(e: &Engine, entry: u32, dispatcher: u32, table: u32) -> u32 {
    let record = u32::from(e.mem.u8(entry + 1)) * 0x50;
    e.mem
        .u32(dispatcher.wrapping_add(record).wrapping_add(table))
}

// Translated from 00cfdc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_WarpTimeInAgent` (Xbox PDB): for the entry's type (byte at +0), 2, 6, 10 and
/// 14 call the dispatcher's warp-time function (record +0x16d4; the dispatcher is the first word
/// of `input`) with the entry, its agent data at +0x20, `time_a`, `time_b` and `input`; types 4
/// and 12 first store `time_b` at +0x18 when `time_a` equals the stored time there (otherwise
/// the `float` at `01012054`) and call it with the agent data at +0x30. The other types do
/// nothing. Floats travel through `FLD`/`FSTP`, so signalling NaNs are quieted.
pub fn hk_agent_nn_machine_warp_time_in_agent(
    e: &mut Engine,
    entry: Ptr,
    time_a: f32,
    time_b: f32,
    input: Ptr,
) {
    let me = entry.addr();
    let agent_data = match e.mem.u8(me) {
        2 | 6 | 10 | 14 => me + 0x20,
        4 | 12 => {
            // An unordered (NaN) compare is not equal, as the `FUCOMP` flags say.
            let stored = e.mem.f32(me + 0x18);
            let new_time = if time_a == stored {
                x87_quiet(time_b)
            } else {
                x87_quiet(e.mem.f32(WARP_MISMATCH_TIME))
            };
            e.mem.set_f32(me + 0x18, new_time);
            me + 0x30
        }
        _ => return,
    };
    let dispatcher = e.mem.u32(input.addr());
    let function = dispatcher_function(e, me, dispatcher, 0x16d4);
    if function != 0 {
        e.call(
            function,
            &args![
                me,
                agent_data,
                x87_quiet_bits(time_a.to_bits()),
                x87_quiet_bits(time_b.to_bits()),
                input
            ],
        );
    }
}

// Translated from 00cfdd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_FindAgent` (Xbox PDB): the agent entry linking the two collidables, found
/// by searching the `m_collisionEntries` of the one with fewer entries (the first when equal)
/// for the other as partner; 0 when there is none.
pub fn hk_agent_nn_machine_find_agent(e: &mut Engine, first: Ptr, second: Ptr) -> Ptr {
    let (searched, wanted) = if e.mem.i32(first.addr() + 0x54) > e.mem.i32(second.addr() + 0x54) {
        (second.addr(), first.addr())
    } else {
        (first.addr(), second.addr())
    };
    let count = e.mem.i32(searched + 0x54);
    if count > 0 {
        let data = e.mem.u32(searched + 0x50);
        for index in 0..count as u32 {
            if e.mem.u32(data + index * 8 + 4) == wanted {
                return Ptr::new(e.mem.u32(data + index * 8));
            }
        }
    }
    Ptr::NULL
}

/// Copies the 0x80 bytes of an entry word by word.
fn copy_entry(e: &mut Engine, from: u32, to: u32) {
    for word in 0..ENTRY_SIZE / 4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Points the two collidables' `m_collisionEntries` elements for `entry` (at the indices the
/// entry stores at +4 and +6) at the entry's new address.
fn relink_entry(e: &mut Engine, entry: u32) {
    let first_index = u32::from(e.mem.u16(entry + 4));
    let second_index = u32::from(e.mem.u16(entry + 6));
    let first_collidable = e.mem.u32(entry + 0x10);
    let first_entries = e.mem.u32(first_collidable + 0x50);
    e.mem.set_u32(first_entries + first_index * 8, entry);
    let second_collidable = e.mem.u32(entry + 0x14);
    let second_entries = e.mem.u32(second_collidable + 0x50);
    e.mem.set_u32(second_entries + second_index * 8, entry);
}

/// The element count `hkArrayUtil` rounds up to: `((v >> 1) | v) + 1` with `v` the sizes's
/// `size + 3` smeared right by 16, 8, 4 and 2 bits (as a signed `int`, which can wrap
/// negative).
fn rounded_capacity(size: i32) -> i32 {
    let mut value = (size as u32).wrapping_add(3);
    value |= value >> 16;
    value |= value >> 8;
    value |= value >> 4;
    value |= value >> 2;
    ((value >> 1) | value).wrapping_add(1) as i32
}

/// The last sector of a track (`sectors[size - 1]`).
fn last_sector(e: &Engine, track: u32) -> u32 {
    let size = e.mem.u32(track + 8);
    let data = e.mem.u32(track + 4);
    e.mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4))
}

/// The tail of the track's last-entry removal that `00cfe020` and `00cff290` both contain: takes
/// one entry (0x80 bytes) off the last sector's used bytes; an emptied sector is freed and
/// removed from the sector array; then, unless the array's capacity word is negative (storage
/// not owned), the array shrinks to the rounded capacity (or back to the inplace storage when
/// the rounding is not positive).
fn drop_last_entry_slot(e: &mut Engine, track: u32) {
    let used = e.mem.u32(track).wrapping_sub(ENTRY_SIZE);
    e.mem.set_u32(track, used);
    let sectors = track + 4;
    if used == 0 {
        let last = last_sector(e, track);
        let size = e.mem.i32(sectors + 4);
        e.mem.set_i32(sectors + 4, size.wrapping_sub(1));
        router_free(e, last, SECTOR_SIZE);
        e.mem.set_u32(track, SECTOR_SIZE);
    }
    let capacity = e.mem.u32(sectors + 8);
    if (capacity as i32) < 0 {
        return;
    }
    let wanted = rounded_capacity(e.mem.i32(sectors + 4));
    if wanted < 1 {
        let allocator = router_allocator(e);
        e.call(
            ARRAY_REDUCE,
            &args![allocator, sectors, 4u32, sectors + 0xc, 1u32],
        );
    } else if wanted < (capacity & 0x3fff_ffff) as i32 {
        let allocator = router_allocator(e);
        e.call(
            ARRAY_REDUCE,
            &args![allocator, sectors, 4u32, 0u32, wanted as u32],
        );
    }
}

// Translated from 00cfe020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_InternalDeallocateEntry` (Xbox PDB): removes `entry` from the track by
/// moving the track's last entry into its place (all 0x80 bytes, then relinking the two
/// collidables' element pointers) when it is not the last itself, and releasing the freed slot
/// (`drop_last_entry_slot`).
pub fn hk_agent_nn_machine_internal_deallocate_entry(e: &mut Engine, track: Ptr, entry: Ptr) {
    let me = track.addr();
    let used = e.mem.u32(me);
    let last = last_sector(e, me)
        .wrapping_add(used)
        .wrapping_sub(ENTRY_SIZE);
    if entry.addr() != last {
        copy_entry(e, last, entry.addr());
        relink_entry(e, entry.addr());
    }
    drop_last_entry_slot(e, me);
}

// Translated from 00cfe820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_AllocateEntry` (Xbox PDB): the next free entry slot of the track. The
/// track arrives in EDI (the only argument): with room left in the last sector the slot is
/// there and the used bytes grow by 0x80; otherwise a new sector (0x200 bytes) is allocated
/// through the router's allocator, appended to the sector array (growing it when full), and its
/// first slot is returned.
pub fn hk_agent_nn_machine_allocate_entry(e: &mut Engine, track: Ptr) -> Ptr {
    let me = track.addr();
    let used = e.mem.u32(me);
    if used < SECTOR_SIZE {
        let sector = last_sector(e, me);
        e.mem.set_u32(me, used + ENTRY_SIZE);
        return Ptr::new(sector.wrapping_add(used));
    }
    let allocator = router_allocator(e);
    let sector = e.vcall(allocator, 4, &args![SECTOR_SIZE]).u32();
    let allocator = router_allocator(e);
    let sectors = me + 4;
    if e.mem.u32(sectors + 4) == e.mem.u32(sectors + 8) & 0x3fff_ffff {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, sectors, 4u32]);
    }
    let size = e.mem.u32(sectors + 4);
    let data = e.mem.u32(sectors);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), sector);
    e.mem.set_u32(sectors + 4, size.wrapping_add(1));
    e.mem.set_u32(me, ENTRY_SIZE);
    Ptr::new(sector)
}

// Translated from 00cfe1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentMachine_processTransformedShapes` (Xbox PDB): follows a chain of transform shapes
/// (shape type 15 at +0xc of the body's shape). `body` is a collision body (shape +0,
/// shape key +4, transform record +8, parent +0xc); `bodies` receives a body of 16 bytes per
/// level (the child shape at +0, 0 at +4, the transform buffer entry at +8, the parent body at
/// +0xc); `transforms` receives 0xb0 bytes per level. Per level it composes the record's
/// rotation (columns at +0, +0x10, +0x20) with the shape's own rotation (+0x30..+0x50) and
/// translation (+0x60), adds the record's translation (+0x30), copies +0x40 and +0x50,
/// multiplies the quaternions at +0x60 and +0x70 by the shape's at +0x20 (`00c66320`), writes
/// the offset `record +0x80 - shape +0x60` transformed by the transposed shape rotation at
/// +0x80, copies +0x90 and the `float` at +0xa0, and sets `*changed = 8`. Returns the last
/// body. The fourth stack word is not read. `bodies` must not be null.
pub fn hk_agent_machine_process_transformed_shapes(
    e: &mut Engine,
    body: Ptr,
    bodies: Ptr,
    transforms: Ptr,
    _unused_4: u32,
    changed: Ptr,
) -> Ptr {
    let mut body = body.addr();
    let mut bodies = bodies.addr();
    let mut transforms = transforms.addr();
    let mut shape = e.mem.u32(body);
    while e.mem.u32(shape + 0xc) == 0xf {
        let record = e.mem.u32(body + 8);
        let rotation = [
            load_vec(e, record),
            load_vec(e, record + 0x10),
            load_vec(e, record + 0x20),
        ];
        // The record's rotation applied to the shape's three columns and its translation.
        for column in 0..4u32 {
            let source = load_vec(e, shape + 0x30 + 0x10 * column);
            let product: Vec4 = std::array::from_fn(|lane| {
                (source[1] * rotation[1][lane] + source[0] * rotation[0][lane])
                    + source[2] * rotation[2][lane]
            });
            store_vec(e, transforms + 0x10 * column, product);
        }
        let record_translation = load_vec(e, record + 0x30);
        let moved = load_vec(e, transforms + 0x30);
        let translation: Vec4 = std::array::from_fn(|lane| record_translation[lane] + moved[lane]);
        store_vec(e, transforms + 0x30, translation);
        copy_vec(e, record + 0x40, transforms + 0x40);
        copy_vec(e, record + 0x50, transforms + 0x50);
        e.call(
            QUATERNION_SET_MUL,
            &args![transforms + 0x60, record + 0x60, shape + 0x20],
        );
        e.call(
            QUATERNION_SET_MUL,
            &args![transforms + 0x70, record + 0x70, shape + 0x20],
        );
        // The offset from the shape's translation, seen from inside the shape: the rows of
        // the shape's rotation are gathered with UNPCKLPS, UNPCKHPS, MOVLHPS, MOVHLPS and
        // SHUFPS (the fourth lanes take the odd values 'y' and 'w' of the last column).
        let offset_source = load_vec(e, record + 0x80);
        let shape_translation = load_vec(e, shape + 0x60);
        let offset: Vec4 =
            std::array::from_fn(|lane| offset_source[lane] - shape_translation[lane]);
        let first = load_vec(e, shape + 0x30);
        let second = load_vec(e, shape + 0x40);
        let third = load_vec(e, shape + 0x50);
        let x_rows = [first[0], second[0], third[0], third[1]];
        let y_rows = [first[1], second[1], third[1], third[3]];
        let z_rows = [first[2], second[2], third[2], third[3]];
        let local: Vec4 = std::array::from_fn(|lane| {
            (offset[1] * y_rows[lane] + offset[0] * x_rows[lane]) + offset[2] * z_rows[lane]
        });
        store_vec(e, transforms + 0x80, local);
        copy_vec(e, record + 0x90, transforms + 0x90);
        let extra = x87_quiet(e.mem.f32(record + 0xa0));
        e.mem.set_f32(transforms + 0xa0, extra);
        if bodies != 0 {
            e.mem.set_u32(bodies + 0xc, body);
            e.mem.set_u32(bodies + 8, transforms);
        }
        let child = e.mem.u32(shape + 0x14);
        transforms = transforms.wrapping_add(0xb0);
        e.mem.set_u32(bodies, child);
        e.mem.set_u32(bodies + 4, 0);
        body = bodies;
        shape = child;
        bodies = bodies.wrapping_add(0x10);
        e.mem.set_u8(changed.addr(), 8);
    }
    Ptr::new(body)
}

/// Gives back the scratch buffers of `hk_agent_nn_machine_update_shape_collection_filter`.
fn free_scratch(e: &mut Engine, blocks: Vec<u32>) {
    for block in blocks {
        e.mem.free(block);
    }
}

// Translated from 00cfeba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_UpdateShapeCollectionFilter` (Xbox PDB): for the entry's type (2, 6 → agent
/// data at +0x20; 4 → +0x30) calls the dispatcher's update function (record +0x16c8; the
/// dispatcher is the first word of `input`) with the entry, the agent data, the two collidables'
/// bodies (+0x10, +0x14), `input`, the word at entry +8 and `argument`. Types 10, 12 and 14
/// first resolve transform shapes on both bodies
/// (`hk_agent_machine_process_transformed_shapes`, into scratch buffers) and then continue as
/// type 2, 4 or 6 with the resolved bodies. Other types do nothing.
pub fn hk_agent_nn_machine_update_shape_collection_filter(
    e: &mut Engine,
    entry: Ptr,
    input: Ptr,
    argument: u32,
) {
    let me = entry.addr();
    let mut kind = u32::from(e.mem.u8(me));
    let mut first = e.mem.u32(me + 0x10);
    let mut second = e.mem.u32(me + 0x14);
    let mut scratch = Vec::new();
    let agent_data = loop {
        match kind {
            2 | 6 => break me + 0x20,
            4 => break me + 0x30,
            10 | 12 | 14 => {
                let first_bodies = e.mem.alloc(0x40);
                let first_transforms = e.mem.alloc(0x2c0);
                let second_bodies = e.mem.alloc(0x40);
                let second_transforms = e.mem.alloc(0x2c0);
                let changed = e.mem.alloc(8);
                scratch.extend([
                    first_bodies,
                    first_transforms,
                    second_bodies,
                    second_transforms,
                    changed,
                ]);
                first = hk_agent_machine_process_transformed_shapes(
                    e,
                    Ptr::new(first),
                    Ptr::new(first_bodies),
                    Ptr::new(first_transforms),
                    4,
                    Ptr::new(changed),
                )
                .addr();
                second = hk_agent_machine_process_transformed_shapes(
                    e,
                    Ptr::new(second),
                    Ptr::new(second_bodies),
                    Ptr::new(second_transforms),
                    4,
                    Ptr::new(changed),
                )
                .addr();
                kind &= 0xf7;
            }
            _ => {
                free_scratch(e, scratch);
                return;
            }
        }
    };
    let dispatcher = e.mem.u32(input.addr());
    let function = dispatcher_function(e, me, dispatcher, 0x16c8);
    if function != 0 {
        let partner_data = e.mem.u32(me + 8);
        e.call(
            function,
            &args![me, agent_data, first, second, input, partner_data, argument],
        );
    }
    free_scratch(e, scratch);
}

// Translated from 00cfea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_DestroyAgent` (Xbox PDB): for types 2, 6, 10 and 14 (agent data at +0x20)
/// and 4 and 12 (+0x30) calls the destroy function of the `dispatcher` (record +0x16b4) with the
/// entry, the agent data, the word at entry +8, `context` and `dispatcher`. Then, for each of
/// the entry's two collidables, removes its element from `m_collisionEntries` (the last element
/// moves into the freed index and its entry's stored index is updated, using the pointer at
/// +0x10 to tell which of the entry's two indices it is), shrinking the array to its rounded
/// capacity when that is smaller than the current one; finally releases the entry from the
/// track (`hk_agent_nn_machine_internal_deallocate_entry`).
pub fn hk_agent_nn_machine_destroy_agent(
    e: &mut Engine,
    track: Ptr,
    entry: Ptr,
    dispatcher: Ptr,
    context: u32,
) {
    let me = entry.addr();
    let agent_data = match e.mem.u8(me) {
        2 | 6 | 10 | 14 => Some(me + 0x20),
        4 | 12 => Some(me + 0x30),
        _ => None,
    };
    if let Some(agent_data) = agent_data {
        // The table entry is called without a null test, as the code does.
        let function = dispatcher_function(e, me, dispatcher.addr(), 0x16b4);
        let partner_data = e.mem.u32(me + 8);
        e.call(
            function,
            &args![me, agent_data, partner_data, context, dispatcher],
        );
    }
    for side in 0..2u32 {
        let collidable = e.mem.u32(me + 0x10 + 4 * side);
        let index = u32::from(e.mem.u16(me + 4 + 2 * side));
        let size = e.mem.i32(collidable + 0x54).wrapping_sub(1);
        e.mem.set_i32(collidable + 0x54, size);
        let array = collidable + 0x50;
        if size as u32 != index {
            let data = e.mem.u32(array);
            for word in 0..2 {
                let value = e.mem.u32(data + (size as u32) * 8 + 4 * word);
                e.mem.set_u32(data + index * 8 + 4 * word, value);
            }
        }
        if (index as i32) < size {
            let data = e.mem.u32(array);
            let moved = e.mem.u32(data + index * 8);
            let side_of_moved = u32::from(e.mem.u32(moved + 0x10) != collidable);
            e.mem.set_u16(moved + 4 + side_of_moved * 2, index as u16);
        }
        let allocator = router_allocator(e);
        let wanted = rounded_capacity(e.mem.i32(array + 4));
        if wanted < (e.mem.u32(array + 8) & 0x3fff_ffff) as i32 {
            e.call(
                ARRAY_REDUCE,
                &args![allocator, array, 8u32, 0u32, wanted as u32],
            );
        }
    }
    hk_agent_nn_machine_internal_deallocate_entry(e, track, entry);
}

// Translated from 00cfeca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_CopyAndRelinkAgentEntry` (Xbox PDB): allocates a slot in the track
/// (`hk_agent_nn_machine_allocate_entry`), copies the 0x80 bytes of `source` into it and
/// points the two collidables' element pointers at the copy. Leaves the new entry in EAX
/// (returned).
pub fn hk_agent_nn_machine_copy_and_relink_agent_entry(
    e: &mut Engine,
    track: Ptr,
    source: Ptr,
) -> Ptr {
    let entry = hk_agent_nn_machine_allocate_entry(e, track);
    copy_entry(e, source.addr(), entry.addr());
    relink_entry(e, entry.addr());
    entry
}

// Translated from 00cff290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAgentNnMachine_AppendTrack` (Xbox PDB): moves the entries of the track `source` onto the
/// end of `target`, last entry first (copy and relink into `target`, then release the slot in
/// `source`), while `target`'s last sector is not full (used bytes below 0x200) and `source`
/// still has sectors. What remains in `source` (whole sectors) is appended to `target`'s sector
/// array (`00cd1e20`), `target` takes `source`'s used bytes, and `source` is left empty with
/// 0x200 used.
pub fn hk_agent_nn_machine_append_track(e: &mut Engine, target: Ptr, source: Ptr) {
    let to = target.addr();
    let from = source.addr();
    while e.mem.u32(to) != SECTOR_SIZE && e.mem.u32(from + 8) != 0 {
        let used = e.mem.u32(from);
        let last = last_sector(e, from)
            .wrapping_add(used)
            .wrapping_sub(ENTRY_SIZE);
        hk_agent_nn_machine_copy_and_relink_agent_entry(e, target, Ptr::new(last));
        drop_last_entry_slot(e, from);
    }
    let remaining = e.mem.u32(from + 8);
    if remaining != 0 {
        let sectors = e.mem.u32(from + 4);
        let target_count = e.mem.u32(to + 8);
        let allocator = router_allocator(e);
        e.call(
            ARRAY_INSERT_AT,
            &args![to + 4, allocator, target_count, sectors, remaining],
        );
        let used = e.mem.u32(from);
        e.mem.set_u32(to, used);
        e.mem.set_u32(from + 8, 0);
        e.mem.set_u32(from, SECTOR_SIZE);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_dc90,
            hk_agent_nn_machine_warp_time_in_agent(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00cf_dd70, hk_agent_nn_machine_find_agent(Ptr, Ptr) -> Ptr),
        entry!(
            0x00cf_e020,
            hk_agent_nn_machine_internal_deallocate_entry(Ptr, Ptr)
        ),
        entry!(0x00cf_e820, hk_agent_nn_machine_allocate_entry(Ptr) -> Ptr),
        entry!(
            0x00cf_e1d0,
            hk_agent_machine_process_transformed_shapes(Ptr, Ptr, Ptr, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00cf_eba0,
            hk_agent_nn_machine_update_shape_collection_filter(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_ea30,
            hk_agent_nn_machine_destroy_agent(Ptr, Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_eca0,
            hk_agent_nn_machine_copy_and_relink_agent_entry(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cf_f290, hk_agent_nn_machine_append_track(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const AGENT_FUNCTION: u32 = 0x0900_0040;
    const ALLOCATE_SLOT: u32 = 0x0900_0001;
    const FREE_SLOT: u32 = 0x0900_0002;
    /// A capacity word with the "storage not owned" bit: the array is never shrunk.
    const NOT_OWNED: u32 = 0x8000_0001;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

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

    /// A memory router whose allocator hands out heap blocks (slot 4) and records the blocks
    /// it is given back (slot 8); returns the allocator's address.
    fn install_router(e: &mut Engine, log: &CallLog) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_SLOT, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        record(e, log, FREE_SLOT, 0);
        allocator
    }

    /// A dispatcher whose records for agent type index 1 hold `function` at all three tables.
    fn make_dispatcher(e: &mut Engine, function: u32) -> u32 {
        let dispatcher = e.mem.alloc(0x1800);
        for table in [0x16b4, 0x16c8, 0x16d4] {
            e.mem.set_u32(dispatcher + 0x50 + table, function);
        }
        dispatcher
    }

    fn make_entry(e: &mut Engine, kind: u8) -> u32 {
        let entry = e.mem.alloc(0x80);
        e.mem.set_u8(entry, kind);
        e.mem.set_u8(entry + 1, 1);
        entry
    }

    #[test]
    fn warp_time_calls_the_agent_function_per_type() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 2);
        let (time_a, time_b) = (1.5f32, 2.5f32);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![
                entry,
                entry + 0x20,
                time_a.to_bits(),
                time_b.to_bits(),
                input
            ]]
        );
        // A type without a warp function does nothing.
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 3);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert!(log.borrow().is_empty());
        // A signalling NaN is quieted on its way to the function.
        e.mem.set_u8(entry, 14);
        e.call(
            0x00cf_dc90,
            &args![entry, f32::from_bits(0x7f80_0001), time_b, input],
        );
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][2], 0x7fc0_0001);
        // Without a function in the table nothing is called.
        e.mem.set_u32(dispatcher + 0x50 + 0x16d4, 0);
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 2);
        e.call(0x00cf_dc90, &args![entry, time_a, time_b, input]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn warp_time_of_types_four_and_twelve_keeps_a_matching_time() {
        let mut e = engine();
        e.mem.set_f32(WARP_MISMATCH_TIME, -1.0);
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 4);
        e.mem.set_f32(entry + 0x18, 1.5);
        e.call(0x00cf_dc90, &args![entry, 1.5f32, 9.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), 9.0);
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][1], entry + 0x30);
        // A different stored time becomes the fallback constant.
        e.mem.set_u8(entry, 12);
        e.call(0x00cf_dc90, &args![entry, 4.0f32, 9.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        // A NaN never matches.
        e.mem.set_f32(entry + 0x18, f32::NAN);
        e.call(0x00cf_dc90, &args![entry, f32::NAN, 7.0f32, input]);
        assert_eq!(e.mem.f32(entry + 0x18), -1.0);
        assert_eq!(calls_to(&log, AGENT_FUNCTION).len(), 3);
    }

    /// A collidable with `count` collision entries, each (entry, partner) pair given.
    fn make_collidable(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let collidable = e.mem.alloc(0x60);
        let data = e.mem.alloc(8 * pairs.len().max(1) as u32);
        for (index, (entry, partner)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, *entry);
            e.mem.set_u32(data + 8 * index as u32 + 4, *partner);
        }
        e.mem.set_u32(collidable + 0x50, data);
        e.mem.set_i32(collidable + 0x54, pairs.len() as i32);
        collidable
    }

    #[test]
    fn find_agent_searches_the_shorter_list() {
        let mut e = engine();
        let a = make_collidable(&mut e, &[(0x1111, 0x9999), (0x2222, 0x8888)]);
        let b = make_collidable(&mut e, &[(0x3333, a)]);
        // b has fewer entries: its list is searched for a.
        assert_eq!(e.call(0x00cf_dd70, &args![a, b]).u32(), 0x3333);
        assert_eq!(e.call(0x00cf_dd70, &args![b, a]).u32(), 0x3333);
        // Equal lengths: the first one's list is searched for the second.
        let d = make_collidable(&mut e, &[(0x5555, 0x1234)]);
        let c = make_collidable(&mut e, &[(0x4444, d)]);
        assert_eq!(e.call(0x00cf_dd70, &args![c, d]).u32(), 0x4444);
        assert_eq!(e.call(0x00cf_dd70, &args![d, c]).u32(), 0);
        assert_eq!(e.call(0x00cf_dd70, &args![c, a]).u32(), 0);
        let empty = make_collidable(&mut e, &[]);
        assert_eq!(e.call(0x00cf_dd70, &args![empty, a]).u32(), 0);
    }

    /// A track with the given sectors (already allocated) and used bytes in the last one.
    fn make_track(e: &mut Engine, sectors: &[u32], used: u32, capacity_word: u32) -> u32 {
        let track = e.mem.alloc(0x20);
        let data = e.mem.alloc(4 * sectors.len().max(1) as u32);
        for (index, sector) in sectors.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *sector);
        }
        e.mem.set_u32(track, used);
        e.mem.set_u32(track + 4, data);
        e.mem.set_u32(track + 8, sectors.len() as u32);
        e.mem.set_u32(track + 0xc, capacity_word);
        track
    }

    /// Entry `index` of a sector linking collidables `a` and `b` at element `element`.
    fn init_entry(e: &mut Engine, entry: u32, a: u32, b: u32, element: u16, marker: u32) {
        e.mem.set_u8(entry, 2);
        e.mem.set_u8(entry + 1, 1);
        e.mem.set_u16(entry + 4, element);
        e.mem.set_u16(entry + 6, element);
        e.mem.set_u32(entry + 8, marker);
        e.mem.set_u32(entry + 0x10, a);
        e.mem.set_u32(entry + 0x14, b);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        e.mem.set_u32(a_data + 8 * u32::from(element), entry);
        e.mem.set_u32(a_data + 8 * u32::from(element) + 4, b);
        e.mem.set_u32(b_data + 8 * u32::from(element), entry);
        e.mem.set_u32(b_data + 8 * u32::from(element) + 4, a);
    }

    #[test]
    fn allocate_entry_uses_room_in_the_last_sector() {
        let mut e = engine();
        let sector = e.mem.alloc(0x200);
        let track = make_track(&mut e, &[sector], 0x100, 0x8001);
        assert_eq!(e.call(0x00cf_e820, &args![track]).u32(), sector + 0x100);
        assert_eq!(e.mem.u32(track), 0x180);
    }

    #[test]
    fn allocate_entry_adds_a_sector_when_the_last_is_full() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            reserve_log
                .borrow_mut()
                .push((ARRAY_RESERVE_MORE, a.to_vec()));
            // Room for another sector.
            let room = e.mem.alloc(16);
            let old = e.mem.u32(a[1]);
            e.mem.set_u32(room, e.mem.u32(old));
            e.mem.set_u32(a[1], room);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let sector = e.mem.alloc(0x200);
        // Size 1, capacity 1: full, so the array grows.
        let track = make_track(&mut e, &[sector], 0x200, 1);
        let entry = e.call(0x00cf_e820, &args![track]).u32();
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(track + 8), 2);
        let data = e.mem.u32(track + 4);
        assert_eq!(e.mem.u32(data), sector);
        assert_eq!(e.mem.u32(data + 4), entry);
        assert_ne!(entry, sector);
        // With room in the array it is not grown.
        e.mem.set_u32(track, 0x200);
        e.call(0x00cf_e820, &args![track]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(e.mem.u32(track + 8), 3);
    }

    #[test]
    fn deallocate_entry_moves_the_last_entry_into_the_gap() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        let sector = e.mem.alloc(0x200);
        // Both entries sit between the same two collidables, at elements 0 and 1.
        let a = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        init_entry(&mut e, sector + 0x80, a, b, 1, 0xbb);
        // A negative capacity word (storage not owned) keeps the array.
        let track = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(sector + 8), 0xbb);
        assert_eq!(e.mem.u16(sector + 4), 1);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        assert_eq!(e.mem.u32(a_data + 8), sector);
        assert_eq!(e.mem.u32(b_data + 8), sector);
        assert!(calls_to(&log, ARRAY_REDUCE).is_empty());
        // Removing the last entry only releases it; the emptied sector goes back.
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track), SECTOR_SIZE);
        assert_eq!(e.mem.u32(track + 8), 0);
        assert_eq!(calls_to(&log, FREE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, FREE_SLOT)[0][1..], [sector, 0x200]);
    }

    #[test]
    fn deallocate_entry_shrinks_an_owned_sector_array() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        // One sector, capacity 16: after the release the rounded capacity 4 + 3 + ... is below.
        let track = make_track(&mut e, &[sector], 0x80, 16);
        e.call(0x00cf_e020, &args![track, sector]);
        assert_eq!(e.mem.u32(track + 8), 0);
        let reduce = calls_to(&log, ARRAY_REDUCE);
        assert_eq!(reduce.len(), 1);
        assert_eq!(reduce[0], vec![allocator, track + 4, 4, 0, 4]);
    }

    #[test]
    fn rounded_capacity_follows_the_smearing_sequence() {
        assert_eq!(rounded_capacity(0), 4);
        assert_eq!(rounded_capacity(1), 8);
        assert_eq!(rounded_capacity(5), 16);
        assert!(rounded_capacity(0x7fff_fffd) < 1);
    }

    #[test]
    fn destroy_agent_unlinks_the_entry_from_both_collidables() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_REDUCE, 0);
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0)]);
        e.mem.set_u32(a + 0x58, 16);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        init_entry(&mut e, sector + 0x80, a, b, 1, 0xbb);
        let track = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        e.call(0x00cf_ea30, &args![track, sector, dispatcher, 0x55u32]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![sector, sector + 0x20, 0xaa, 0x55, dispatcher]]
        );
        // The collidables lost an element; the surviving entry now sits at element 0 of both
        // and was moved into the freed slot.
        assert_eq!(e.mem.i32(a + 0x54), 1);
        assert_eq!(e.mem.i32(b + 0x54), 1);
        assert_eq!(e.mem.u32(track), 0x80);
        assert_eq!(e.mem.u32(sector + 8), 0xbb);
        assert_eq!(e.mem.u16(sector + 4), 0);
        assert_eq!(e.mem.u16(sector + 6), 0);
        let a_data = e.mem.u32(a + 0x50);
        assert_eq!(e.mem.u32(a_data), sector);
        // Only `a` has a capacity larger than the rounded one (8 < 16).
        let reduce = calls_to(&log, ARRAY_REDUCE);
        assert_eq!(reduce, vec![vec![allocator, a + 0x50, 8, 0, 8]]);
    }

    #[test]
    fn destroy_agent_of_an_unknown_type_only_unlinks() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, sector, a, b, 0, 0xaa);
        e.mem.set_u8(sector, 7);
        let track = make_track(&mut e, &[sector], 0x80, NOT_OWNED);
        e.call(0x00cf_ea30, &args![track, sector, dispatcher, 0u32]);
        assert!(calls_to(&log, AGENT_FUNCTION).is_empty());
        assert_eq!(e.mem.i32(a + 0x54), 0);
        assert_eq!(e.mem.i32(b + 0x54), 0);
        assert_eq!(e.mem.u32(track), SECTOR_SIZE);
    }

    #[test]
    fn copy_and_relink_moves_an_entry_into_a_new_slot() {
        let mut e = engine();
        let sector = e.mem.alloc(0x200);
        let other = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0)]);
        init_entry(&mut e, other, a, b, 0, 0xcc);
        let track = make_track(&mut e, &[sector], 0x80, 0x8001);
        let copy = e.call(0x00cf_eca0, &args![track, other]).u32();
        assert_eq!(copy, sector + 0x80);
        assert_eq!(e.mem.u32(track), 0x100);
        assert_eq!(e.mem.u32(copy + 8), 0xcc);
        let a_data = e.mem.u32(a + 0x50);
        let b_data = e.mem.u32(b + 0x50);
        assert_eq!(e.mem.u32(a_data), copy);
        assert_eq!(e.mem.u32(b_data), copy);
    }

    #[test]
    fn append_track_moves_entries_until_the_target_is_full() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_INSERT_AT, 0);
        let target_sector = e.mem.alloc(0x200);
        let source_sector = e.mem.alloc(0x200);
        let a = make_collidable(&mut e, &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
        let b = make_collidable(&mut e, &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)]);
        init_entry(&mut e, target_sector, a, b, 0, 1);
        init_entry(&mut e, target_sector + 0x80, a, b, 1, 2);
        init_entry(&mut e, source_sector, a, b, 2, 3);
        init_entry(&mut e, source_sector + 0x80, a, b, 3, 4);
        init_entry(&mut e, source_sector + 0x100, a, b, 4, 5);
        let target = make_track(&mut e, &[target_sector], 0x100, NOT_OWNED);
        let source = make_track(&mut e, &[source_sector], 0x180, NOT_OWNED);
        e.call(0x00cf_f290, &args![target, source]);
        // Two entries moved (the last ones first); the target's sector is full.
        assert_eq!(e.mem.u32(target_sector + 0x100 + 8), 5);
        assert_eq!(e.mem.u32(target_sector + 0x180 + 8), 4);
        assert_eq!(e.mem.u32(target), 0x80);
        assert_eq!(e.mem.u32(source), SECTOR_SIZE);
        assert_eq!(e.mem.u32(source + 8), 0);
        let insert = calls_to(&log, ARRAY_INSERT_AT);
        assert_eq!(insert.len(), 1);
        assert_eq!(insert[0][0], target + 4);
        assert_eq!(insert[0][2..], [1, e.mem.u32(source + 4), 1]);
        let a_data = e.mem.u32(a + 0x50);
        assert_eq!(e.mem.u32(a_data + 8 * 4), target_sector + 0x100);
        assert_eq!(e.mem.u32(a_data + 8 * 3), target_sector + 0x180);
    }

    #[test]
    fn append_track_of_an_empty_source_changes_nothing() {
        let mut e = engine();
        let log = new_log();
        install_router(&mut e, &log);
        record(&mut e, &log, ARRAY_INSERT_AT, 0);
        let sector = e.mem.alloc(0x200);
        let target = make_track(&mut e, &[sector], 0x100, NOT_OWNED);
        let source = make_track(&mut e, &[], SECTOR_SIZE, NOT_OWNED);
        e.call(0x00cf_f290, &args![target, source]);
        assert_eq!(e.mem.u32(target), 0x100);
        assert!(calls_to(&log, ARRAY_INSERT_AT).is_empty());
    }

    struct TransformScene {
        body: u32,
        record: u32,
        shape: u32,
        child: u32,
    }

    /// A body whose shape is a transform shape (type 15) with a quarter turn about z and the
    /// translation (1, 2, 3), over a plain child shape; the body's record scales by (2, 3, 4)
    /// and translates by (10, 20, 30).
    fn transform_scene(e: &mut Engine) -> TransformScene {
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0xc, 5);
        let shape = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc, 0xf);
        e.mem.set_u32(shape + 0x14, child);
        store_vec(e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, shape + 0x40, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, shape + 0x50, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, shape + 0x60, [1.0, 2.0, 3.0, 0.0]);
        let record = e.mem.alloc(0xb0);
        store_vec(e, record, [2.0, 0.0, 0.0, 0.0]);
        store_vec(e, record + 0x10, [0.0, 3.0, 0.0, 0.0]);
        store_vec(e, record + 0x20, [0.0, 0.0, 4.0, 0.0]);
        store_vec(e, record + 0x30, [10.0, 20.0, 30.0, 0.0]);
        store_vec(e, record + 0x40, [7.0, 8.0, 9.0, 0.5]);
        store_vec(e, record + 0x50, [6.0, 5.0, 4.0, 0.25]);
        store_vec(e, record + 0x80, [5.0, 6.0, 7.0, 0.0]);
        store_vec(e, record + 0x90, [0.1, 0.2, 0.3, 0.4]);
        e.mem.set_f32(record + 0xa0, 0.75);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, record);
        TransformScene {
            body,
            record,
            shape,
            child,
        }
    }

    #[test]
    fn process_transformed_shapes_composes_one_level() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, QUATERNION_SET_MUL, 0);
        let scene = transform_scene(&mut e);
        let bodies = e.mem.alloc(0x40);
        let transforms = e.mem.alloc(0x2c0);
        let changed = e.mem.alloc(8);
        let last = e
            .call(
                0x00cf_e1d0,
                &args![scene.body, bodies, transforms, 4u32, changed],
            )
            .u32();
        // The chain ends at the one body written, which describes the plain child.
        assert_eq!(last, bodies);
        assert_eq!(e.mem.u32(bodies), scene.child);
        assert_eq!(e.mem.u32(bodies + 4), 0);
        assert_eq!(e.mem.u32(bodies + 8), transforms);
        assert_eq!(e.mem.u32(bodies + 0xc), scene.body);
        assert_eq!(e.mem.u8(changed), 8);
        // The record's scale applied to the shape's columns and translation.
        assert_eq!(load_vec(&e, transforms), [0.0, 3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x10), [-2.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x20), [0.0, 0.0, 4.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x30), [12.0, 26.0, 42.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x40), [7.0, 8.0, 9.0, 0.5]);
        assert_eq!(load_vec(&e, transforms + 0x50), [6.0, 5.0, 4.0, 0.25]);
        // (5, 6, 7) - (1, 2, 3) = (4, 4, 4) seen through the transposed rotation.
        assert_eq!(load_vec(&e, transforms + 0x80), [4.0, -4.0, 4.0, 0.0]);
        assert_eq!(load_vec(&e, transforms + 0x90), [0.1, 0.2, 0.3, 0.4]);
        assert_eq!(e.mem.f32(transforms + 0xa0), 0.75);
        let multiplies = calls_to(&log, QUATERNION_SET_MUL);
        assert_eq!(
            multiplies,
            vec![
                vec![transforms + 0x60, scene.record + 0x60, scene.shape + 0x20],
                vec![transforms + 0x70, scene.record + 0x70, scene.shape + 0x20],
            ]
        );
    }

    #[test]
    fn process_transformed_shapes_returns_a_plain_body_untouched() {
        let mut e = engine();
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0xc, 5);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, child);
        let changed = e.mem.alloc(8);
        let last = e
            .call(0x00cf_e1d0, &args![body, 0u32, 0u32, 4u32, changed])
            .u32();
        assert_eq!(last, body);
        assert_eq!(e.mem.u8(changed), 0);
    }

    #[test]
    fn update_shape_collection_filter_calls_the_dispatcher_function() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let entry = make_entry(&mut e, 4);
        e.mem.set_u32(entry + 8, 0x77);
        e.mem.set_u32(entry + 0x10, 0x1000);
        e.mem.set_u32(entry + 0x14, 0x2000);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert_eq!(
            calls_to(&log, AGENT_FUNCTION),
            vec![vec![entry, entry + 0x30, 0x1000, 0x2000, input, 0x77, 0x99]]
        );
        log.borrow_mut().clear();
        e.mem.set_u8(entry, 6);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert_eq!(calls_to(&log, AGENT_FUNCTION)[0][1], entry + 0x20);
        e.mem.set_u8(entry, 5);
        log.borrow_mut().clear();
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn update_shape_collection_filter_resolves_transform_shapes_first() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, AGENT_FUNCTION, 0);
        record(&mut e, &log, QUATERNION_SET_MUL, 0);
        let dispatcher = make_dispatcher(&mut e, AGENT_FUNCTION);
        let input = e.mem.alloc(8);
        e.mem.set_u32(input, dispatcher);
        let scene = transform_scene(&mut e);
        let plain_shape = e.mem.alloc(0x40);
        e.mem.set_u32(plain_shape + 0xc, 5);
        let plain = e.mem.alloc(0x10);
        e.mem.set_u32(plain, plain_shape);
        let entry = make_entry(&mut e, 12);
        e.mem.set_u32(entry + 0x10, scene.body);
        e.mem.set_u32(entry + 0x14, plain);
        e.call(0x00cf_eba0, &args![entry, input, 0x99u32]);
        let calls = calls_to(&log, AGENT_FUNCTION);
        assert_eq!(calls.len(), 1);
        // Type 12 continues as type 4 (agent data at +0x30), the first body is the resolved
        // one (a new body over the child shape), the second the untouched plain one.
        assert_eq!(calls[0][1], entry + 0x30);
        assert_ne!(calls[0][2], scene.body);
        assert_eq!(e.mem.u32(calls[0][2]), scene.child);
        assert_eq!(calls[0][3], plain);
        assert_eq!(calls_to(&log, QUATERNION_SET_MUL).len(), 2);
    }
}
