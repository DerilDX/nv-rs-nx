//! `Havok SDK/hkpconstraintsolversetup.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

/// The memory router (`00c85750`, no arguments): the object whose `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array growing call (`00c90940`, cdecl: allocator, array, element size): makes room for
/// one more element in a Havok array.
const GROW_ARRAY: u32 = 0x00c9_0940;
/// `00d8aac0` (cdecl: the solver at `+4`, a range end, `+0xc`, `+0x40`): solves one range of
/// jacobian schemas; no name known.
const SOLVE_RANGE: u32 = 0x00d8_aac0;
/// `hkRigidMotionUtilApplyAccumulators` (`00d29bf0`, cdecl, six words).
const APPLY_ACCUMULATORS: u32 = 0x00d2_9bf0;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (`00d1a330`, cdecl: the manager, a pointer to the
/// entity pointers, their number).
const BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (`00d29830`, cdecl, five words; answers
/// the new accumulator end).
const APPLY_FORCES_AND_BUILD: u32 = 0x00d2_9830;
/// `hkSimpleContactConstraintDataBuildJacobian` (`00d72190`, cdecl: the atoms, the context, a
/// flag byte, a pointer to the schema stream pointer).
const BUILD_CONTACT_JACOBIAN: u32 = 0x00d7_2190;
/// `hkSolverBuildJacobianFromAtomsNotContact` (`00d6f460`, cdecl: the atoms, their size in
/// bytes (16 bits), the context, a pointer to the schema stream pointer).
const BUILD_ATOM_JACOBIAN: u32 = 0x00d6_f460;
/// `hkSimpleContactConstraintData_fireCallbacks` (`00d92900`, cdecl: the data, the context, a
/// word from the data, a flag).
const FIRE_CALLBACKS: u32 = 0x00d9_2900;

/// Copies the four words at `from` to `to` (the `MOVAPS` copies; the bits are not looked at).
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Zeroes the 16 bytes at `at` (`XORPS` and `MOVAPS`).
fn zero_vector(e: &mut Engine, at: u32) {
    for word in 0..4 {
        e.mem.set_u32(at + 4 * word, 0);
    }
}

/// Writes the empty accumulator the buffer starts with: type byte 1, the float at `+8` zero,
/// and the seven vectors from `+0x10` on zero (the first is `+0x30`, then `+0x10`, `+0x20`,
/// `+0x40`, `+0x50`, `+0x60`, `+0x70`; the other words keep their value).
fn write_empty_accumulator(e: &mut Engine, at: u32) {
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u8(at, 1);
    for offset in [0x30, 0x10, 0x20, 0x40, 0x50, 0x60, 0x70] {
        zero_vector(e, at + offset);
    }
}

// Translated from 00d88300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::shutdownSolver`: does nothing.
pub fn shutdown_solver(_e: &mut Engine) {}

// Translated from 00d88310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a byte at `out`: 1 when `count` accumulators (128 bytes each; one more when
/// `[buffer + 0x14] == [buffer + 0xc]`, the buffer still being empty) fit in the free room
/// `[buffer + 0x10] - [buffer + 0x14] - 0x10` (signed compare), else 0. Answers `out`. The class
/// is the solver setup; the name is not known.
pub fn buffer_has_room(e: &mut Engine, out: Ptr, buffer: Ptr, _unused_1: u32, count: u32) -> Ptr {
    let at = buffer.addr();
    let used = e.mem.u32(at + 0x14);
    let free = e.mem.u32(at + 0x10).wrapping_sub(used);
    let count = if used == e.mem.u32(at + 0xc) {
        count.wrapping_add(1)
    } else {
        count
    };
    let room = free.wrapping_add(0xffff_fff0) as i32;
    let needed = count.wrapping_shl(7) as i32;
    e.mem.set_u8(out.addr(), u8::from(needed <= room));
    out
}

// Translated from 00d88340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::subSolve`: with `flag == 0` the two range ends at `+0x2c` / `+0x3c`
/// restart from `+0x20` / `+0x30`; the floats from `[state + 0x4c]` up to `[state + 0x48]` are set
/// to 0.0 and `+0x4c` takes `+0x48`; each range whose start differs from its end (`+0x28`,
/// `+0x38`) is solved with `00d8aac0` (`[state + 4]`, the start, `[state + 0xc]`,
/// `[state + 0x40]`); the starts then take the ends.
pub fn sub_solve(e: &mut Engine, state: Ptr, flag: u32) {
    let at = state.addr();
    if flag == 0 {
        let first = e.mem.u32(at + 0x20);
        let second = e.mem.u32(at + 0x30);
        e.mem.set_u32(at + 0x2c, first);
        e.mem.set_u32(at + 0x3c, second);
    }
    let mut cursor = e.mem.u32(at + 0x4c);
    if cursor < e.mem.u32(at + 0x48) {
        loop {
            e.mem.set_u32(cursor, 0);
            cursor = cursor.wrapping_add(4);
            if cursor >= e.mem.u32(at + 0x48) {
                break;
            }
        }
    }
    let end = e.mem.u32(at + 0x48);
    e.mem.set_u32(at + 0x4c, end);
    for (start_field, end_field) in [(0x2c, 0x28), (0x3c, 0x38)] {
        let start = e.mem.u32(at + start_field);
        if start != e.mem.u32(at + end_field) {
            let solver = e.mem.u32(at + 4);
            let third = e.mem.u32(at + 0xc);
            let fourth = e.mem.u32(at + 0x40);
            e.call(SOLVE_RANGE, &args![solver, start, third, fourth]);
        }
    }
    let first = e.mem.u32(at + 0x28);
    let second = e.mem.u32(at + 0x38);
    e.mem.set_u32(at + 0x2c, first);
    e.mem.set_u32(at + 0x3c, second);
}

// Translated from 00d883c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::calcBufferOffsetsForSolve`: fills the offsets record `out` for a
/// buffer starting at `base`: `+0x14` and `+0x30` = `base`; `+0x3c` = `base + [state + 0x4c] * 0x80
/// + 0x90`; with `rows = 2 * ([state + 0x1c] / 0x90) + 8` (signed, truncated; the exe's
/// multiply-by-reciprocal), `+0x40` = `+0x3c` + `rows * 16 + [state + 0xc] + 0x10` and `+0x18` =
/// `((rows * 3 + [state + 0x14]) * 4 + 0x17 & !0xf) - base + [out + 0x40]`. With a non-zero
/// `base`, the empty accumulator is written at `base` and the byte 3 closes the accumulators
/// (at `base + ([state + 0x4c] + 1) * 0x80`). Answers `[out + 0x18]`.
pub fn calc_buffer_offsets_for_solve(
    e: &mut Engine,
    state: Ptr,
    base: u32,
    _unused_1: u32,
    out: Ptr,
) -> u32 {
    let at = state.addr();
    let record = out.addr();
    e.mem.set_u32(record + 0x14, base);
    let count = e.mem.u32(at + 0x4c);
    e.mem.set_u32(record + 0x30, base);
    let accumulators_end = base.wrapping_add(count.wrapping_shl(7)).wrapping_add(0x90);
    // IMUL by 0x38e38e39, then SAR 5 and the sign correction: a signed division by 0x90.
    let product = i64::from(e.mem.u32(at + 0x1c) as i32) * 0x38e3_8e39;
    let high = (product >> 32) as i32;
    let shifted = high >> 5;
    let quotient = shifted.wrapping_add(((shifted as u32) >> 31) as i32);
    let rows = quotient.wrapping_mul(2).wrapping_add(8) as u32;
    let vector_words = rows.wrapping_mul(3).wrapping_add(e.mem.u32(at + 0x14));
    let rows_bytes = rows.wrapping_shl(4).wrapping_add(e.mem.u32(at + 0xc));
    e.mem.set_u32(record + 0x3c, accumulators_end);
    let first = e.mem.u32(record + 0x14);
    let aligned = vector_words.wrapping_mul(4).wrapping_add(0x17) & 0xffff_fff0;
    let schemas = accumulators_end.wrapping_add(rows_bytes).wrapping_add(0x10);
    let lanes = aligned.wrapping_sub(first).wrapping_add(schemas);
    e.mem.set_u32(record + 0x40, schemas);
    e.mem.set_u32(record + 0x18, lanes);
    if first != 0 {
        let start = e.mem.u32(record + 0x30);
        write_empty_accumulator(e, start);
        e.mem
            .set_u8(start.wrapping_add(count.wrapping_add(1).wrapping_shl(7)), 3);
    }
    lanes
}

// Translated from 00d88470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks that the constraints in `array` (`count` pointers) fit the solver's buffers and writes
/// 1 or 0 at `out` (answers `out`). Each constraint's atom info (`[constraint + 0x30]`) adds its
/// `u16` at `+0x14` to one of two running sums (the first when `[constraint + 0x1c] < 5`, else
/// the second), starting from `[state + 0x28]` / `[state + 0x38]`, and its `u16` at `+0x18`
/// times 4 to the byte total, starting from `[state + 0x48]`. They fit when the total is at most
/// `[state + 0x44]` and the sums plus 4 are at most `[state + 0x24]` / `[state + 0x34]`
/// (unsigned). The exe works in pairs (separate byte totals for even and odd constraints,
/// summed at the end) and then a single trailing one. The class is the solver setup; the name is
/// not known.
pub fn constraints_fit_buffers(
    e: &mut Engine,
    out: Ptr,
    state: Ptr,
    array: Ptr,
    count: u32,
) -> Ptr {
    let at = state.addr();
    let mut sums = [e.mem.u32(at + 0x28), e.mem.u32(at + 0x38)];
    let start = array.addr();
    let end = start.wrapping_add(count.wrapping_mul(4));
    let bytes = end.wrapping_sub(start).wrapping_add(3) as i32;
    let adjust = if bytes < 0 { 3 } else { 0 };
    let pairs = bytes.wrapping_add(adjust) >> 2;
    let mut tail_total = e.mem.u32(at + 0x48);
    let mut even_total = 0u32;
    let mut odd_total = 0u32;
    let mut cursor = start;
    // The sums index and the two figures of a constraint: `[+0x1c] < 5` selects the first sum.
    let mut take = |e: &Engine, constraint: u32| -> u32 {
        let info = e.mem.u32(constraint + 0x30);
        let index = usize::from(e.mem.u8(constraint + 0x1c) >= 5);
        let uses = u32::from(e.mem.u16(info + 0x14));
        sums[index] = sums[index].wrapping_add(uses);
        u32::from(e.mem.u16(info + 0x18)).wrapping_mul(4)
    };
    if pairs >= 2 {
        let limit = end.wrapping_sub(4) as i32;
        loop {
            let first = e.mem.u32(cursor);
            even_total = even_total.wrapping_add(take(e, first));
            let second = e.mem.u32(cursor.wrapping_add(4));
            odd_total = odd_total.wrapping_add(take(e, second));
            cursor = cursor.wrapping_add(8);
            if (cursor as i32) >= limit {
                break;
            }
        }
    }
    if cursor < end {
        let last = e.mem.u32(cursor);
        tail_total = tail_total.wrapping_add(take(e, last));
    }
    let total = odd_total.wrapping_add(even_total).wrapping_add(tail_total);
    let fits = total <= e.mem.u32(at + 0x44)
        && sums[0].wrapping_add(4) <= e.mem.u32(at + 0x24)
        && sums[1].wrapping_add(4) <= e.mem.u32(at + 0x34);
    e.mem.set_u8(out.addr(), u8::from(fits));
    out
}

// Translated from 00d88580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::oneStepIntegrate`: for each of the `count` entities (signed count)
/// in `entities`: copies its accumulator (at `[entity + 0xa4] + buffer`: byte 0, word 4, the float
/// at +8 through the x87 stack, and the vectors from +0x10 to +0x70; vectors +0x40 and +0x50 of
/// the copy then take the accumulator's +0x10 and +0x20) to a local record, sets the byte at
/// `entity + 0xe9` to `0xff` and calls `hkRigidMotionUtilApplyAccumulators` (`step_info`,
/// `second_word`, the record, a pointer to the local holding `entity + 0xe0`, 1, 0), then
/// `entityBatchRecalcAabb` (`[[entity + 8] + 0x78]`, a pointer to a local holding the entity, 1),
/// and puts the old `entity + 0xe9` byte back through the local pointer (which the first call
/// may have changed). The local record's bytes that are not copied (1 to 3, and +0xc) are
/// uninitialised stack in the exe; here they are zero.
pub fn one_step_integrate(
    e: &mut Engine,
    step_info: u32,
    second_word: u32,
    buffer: u32,
    entities: Ptr,
    count: i32,
) {
    for index in 0..count {
        let entity = e.mem.u32(entities.addr() + 4 * index as u32);
        e.with_stack(0xc0, |e, frame| {
            let motion_slot = frame.addr();
            let entity_slot = frame.addr() + 4;
            let record = frame.addr() + 0x20;
            e.mem.set_u32(entity_slot, entity);
            e.mem.set_u32(motion_slot, entity.wrapping_add(0xe0));
            let source = e.mem.u32(entity + 0xa4).wrapping_add(buffer);
            let kind = e.mem.u8(source);
            e.mem.set_u8(record, kind);
            let word = e.mem.u32(source + 4);
            e.mem.set_u32(record + 4, word);
            let float = x87_quiet_bits(e.mem.u32(source + 8));
            e.mem.set_u32(record + 8, float);
            for vector in 1..8 {
                copy_vector(e, source + 0x10 * vector, record + 0x10 * vector);
            }
            copy_vector(e, source + 0x10, record + 0x40);
            copy_vector(e, source + 0x20, record + 0x50);
            let old = e.mem.u8(entity + 0xe9);
            e.mem.set_u8(entity + 0xe9, 0xff);
            e.call(
                APPLY_ACCUMULATORS,
                &args![step_info, second_word, record, motion_slot, 1u32, 0u32],
            );
            let entity_now = e.mem.u32(entity_slot);
            let owner = e.mem.u32(entity_now + 8);
            let manager = e.mem.u32(owner + 0x78);
            e.call(BATCH_RECALC_AABB, &args![manager, entity_slot, 1u32]);
            let motion = e.mem.u32(motion_slot);
            e.mem.set_u8(motion + 9, old);
        });
    }
}

// Translated from 00d88670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::internalAddAccumulators`: with `count != 0`: when the write cursor
/// `[state + 0x14]` is still at the buffer start `[state + 0xc]`, writes the empty accumulator
/// there, moves the cursor past it and zeroes the first two lane vectors at `[state + 0x18]`;
/// gives each entity (`array`, `count` pointers) the accumulator offset `i * 0x80 - start +
/// cursor` at `+0xa4`; calls `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (`[state]`,
/// the array, the count, `0xe0`, the old cursor), which answers the new cursor; moves the 32
/// bytes at `+0x40` of each accumulator to the lanes (from the first entity's lane on, 0x20 each)
/// and zeroes them; and writes the byte 3 at the new cursor.
pub fn internal_add_accumulators(e: &mut Engine, state: Ptr, array: Ptr, count: u32) {
    if count == 0 {
        return;
    }
    let at = state.addr();
    let cursor = e.mem.u32(at + 0x14);
    if cursor == e.mem.u32(at + 0xc) {
        write_empty_accumulator(e, cursor);
        e.mem.set_u32(at + 0x14, cursor.wrapping_add(0x80));
        let lanes = e.mem.u32(at + 0x18);
        zero_vector(e, lanes + 0x10);
        zero_vector(e, lanes);
    }
    let mut offset = 0u32;
    for index in 0..count {
        let entity = e.mem.u32(array.addr() + 4 * index);
        let value = offset
            .wrapping_sub(e.mem.u32(at + 0xc))
            .wrapping_add(e.mem.u32(at + 0x14));
        e.mem.set_u32(entity + 0xa4, value);
        offset = offset.wrapping_add(0x80);
    }
    let saved = e.mem.u32(at + 0x14);
    let first = e.mem.u32(array.addr());
    let mut lane = ((e.mem.u32(first + 0xa4) >> 7) << 5).wrapping_add(e.mem.u32(at + 0x18));
    let solver = e.mem.u32(at);
    let end = e
        .call(
            APPLY_FORCES_AND_BUILD,
            &args![solver, array, count, 0xe0u32, saved],
        )
        .u32();
    e.mem.set_u32(at + 0x14, end);
    for index in 0..count {
        let source = saved.wrapping_add(0x40).wrapping_add(0x80 * index);
        copy_vector(e, source, lane);
        copy_vector(e, source + 0x10, lane + 0x10);
        zero_vector(e, source);
        zero_vector(e, source + 0x10);
        lane = lane.wrapping_add(0x20);
    }
    let end = e.mem.u32(at + 0x14);
    e.mem.set_u8(end, 3);
}

// Translated from 00d88760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::internalAddJacobianSchemas`: for each constraint in `constraints`
/// (`count` pointers): fills the context (`[state + 8]`) with the two entities' accumulators
/// (`[entity + 0xa4] + [state + 0xc]`, at `+0x30` / `+0x34`; their lane vectors are copied to
/// `+0x40` / `+0x50` of each), the entities' `+0xf0` (`+0x38` / `+0x3c`), the constraint (`+0x60`),
/// the atom info's `+0x1c` word (`+0x64`, `+0x74`) and the entities' accumulator numbers
/// (`+0x6c` / `+0x70`); appends the 12-byte element (constraint, the schema stream pointer
/// `[state + 0x28]` or, for a constraint whose `+0x1c` byte is 5, `[state + 0x38]`, and the
/// smaller of the entities' floats at `+0x5c`, the second one when the compare is false or
/// unordered) to `list` (an array: data, size, capacity word with flags; grown with
/// `00c90940` when full); for a data object whose type (vtable slot `0x20`) is `0xb` calls
/// `fireCallbacks`; when bit 4 of the atom info's byte `+0x12` is set calls the data's vtable
/// slot `0x38` with the context; builds the jacobian (`BuildJacobian` for atoms of type
/// `0x16`, the other builder for the rest; both advance the stream pointer, which is stored
/// back); adds the atom info's `u16` at `+0x18` times 4 to `[state + 0x48]`; and clears the
/// copied lane vectors. Afterwards a zero ends each of the two streams.
///
/// The exe pushes a dword for the flag of `BuildJacobian` whose upper bytes are uninitialised
/// stack; the callee reads only its low byte, so 1 is passed.
pub fn internal_add_jacobian_schemas(
    e: &mut Engine,
    state: Ptr,
    constraints: Ptr,
    count: u32,
    list: Ptr,
) {
    let at = state.addr();
    let list_at = list.addr();
    let end = constraints.addr().wrapping_add(count.wrapping_mul(4));
    let context = e.mem.u32(at + 8);
    let mut cursor = constraints.addr();
    e.with_stack(0x10, |e, stream_slot| {
        let slot = stream_slot.addr();
        if cursor < end {
            loop {
                let base = e.mem.u32(at + 0xc);
                let constraint = e.mem.u32(cursor);
                let info = e.mem.u32(constraint + 0x30);
                let entity_a = e.mem.u32(info + 4);
                let counter_index = u32::from(e.mem.u8(constraint + 0x1c) == 5);
                let offset_a = e.mem.u32(entity_a + 0xa4);
                let accumulator_a = offset_a.wrapping_add(base);
                let entity_b = e.mem.u32(info + 8);
                let offset_b = e.mem.u32(entity_b + 0xa4);
                let accumulator_b = offset_b.wrapping_add(base);
                e.mem.set_u32(context + 0x30, accumulator_a);
                e.mem.set_u32(context + 0x34, accumulator_b);
                let lanes = e.mem.u32(at + 0x18);
                let lane_a = ((offset_a >> 7) << 5).wrapping_add(lanes);
                let lane_b = ((offset_b >> 7) << 5).wrapping_add(lanes);
                copy_vector(e, lane_a, accumulator_a + 0x40);
                copy_vector(e, lane_a + 0x10, accumulator_a + 0x50);
                copy_vector(e, lane_b, accumulator_b + 0x40);
                copy_vector(e, lane_b + 0x10, accumulator_b + 0x50);
                e.mem.set_u32(context + 0x60, constraint);
                e.mem.set_u32(context + 0x38, entity_a.wrapping_add(0xf0));
                e.mem.set_u32(context + 0x3c, entity_b.wrapping_add(0xf0));
                let word = e.mem.u32(info + 0x1c);
                e.mem.set_u32(context + 0x64, word);
                e.mem.set_u32(context + 0x6c, (offset_a >> 7) & 0xffff);
                e.mem.set_u32(context + 0x70, (offset_b >> 7) & 0xffff);
                e.mem.set_u32(context + 0x74, word);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let used = e.mem.u32(list_at + 4);
                if used == e.mem.u32(list_at + 8) & 0x3fff_ffff {
                    let allocator = e.mem.u32(router + 0x10);
                    e.call(GROW_ARRAY, &args![allocator, list, 0xcu32]);
                }
                let used = e.mem.u32(list_at + 4);
                let element = e
                    .mem
                    .u32(list_at)
                    .wrapping_add(used.wrapping_mul(3).wrapping_mul(4));
                e.mem.set_u32(list_at + 4, used.wrapping_add(1));
                let counter = at + 0x28 + (counter_index << 4);
                e.mem.set_u32(element, constraint);
                let stream = e.mem.u32(counter);
                e.mem.set_u32(element + 4, stream);
                let mass_a = f32::from_bits(x87_quiet_bits(e.mem.u32(entity_a + 0x5c)));
                let mass_b = f32::from_bits(x87_quiet_bits(e.mem.u32(entity_b + 0x5c)));
                let smaller = if mass_b > mass_a { mass_a } else { mass_b };
                e.mem.set_u32(element + 8, smaller.to_bits());
                e.mem.set_u32(slot, stream);
                let data = e.mem.u32(constraint + 0xc);
                let kind = e.vcall(data, 0x20, &args![]).u32();
                if kind == 0xb {
                    let data = e.mem.u32(constraint + 0xc);
                    let data_word = e.mem.u32(data + 0x28);
                    e.call(FIRE_CALLBACKS, &args![data, context, data_word, 1u32]);
                }
                let info = e.mem.u32(constraint + 0x30);
                if e.mem.u8(info + 0x12) & 4 != 0 {
                    let data = e.mem.u32(constraint + 0xc);
                    e.vcall(data, 0x38, &args![context]);
                }
                let info = e.mem.u32(constraint + 0x30);
                let size = u32::from(e.mem.u16(info + 0x10));
                let atoms = e.mem.u32(info + 0xc);
                if e.mem.u16(atoms) == 0x16 {
                    e.call(
                        BUILD_CONTACT_JACOBIAN,
                        &args![atoms, context, 1u32, stream_slot],
                    );
                } else {
                    e.call(
                        BUILD_ATOM_JACOBIAN,
                        &args![atoms, size, context, stream_slot],
                    );
                }
                let advanced = e.mem.u32(slot);
                e.mem.set_u32(counter, advanced);
                let info = e.mem.u32(constraint + 0x30);
                let words = u32::from(e.mem.u16(info + 0x18)).wrapping_mul(4);
                let total = e.mem.u32(at + 0x48).wrapping_add(words);
                e.mem.set_u32(at + 0x48, total);
                for accumulator in [accumulator_a, accumulator_b] {
                    zero_vector(e, accumulator + 0x40);
                    zero_vector(e, accumulator + 0x50);
                }
                cursor = cursor.wrapping_add(4);
                if cursor >= end {
                    break;
                }
            }
        }
    });
    for field in [0x28, 0x38] {
        let stream = e.mem.u32(at + field);
        e.mem.set_u32(stream, 0);
    }
}

// Translated from 00d889f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintSolverSetup::calcBufferSize`: runs `calcBufferOffsetsForSolve` for the base
/// address 0 on a local record (words `+0xc` / `+0x10` = 1, the array at `+0x5c` empty with
/// capacity word `0x80000000`) and answers its result, `[record + 0x18]`: the buffer size. A
/// local array whose capacity word is not negative would be freed through the allocator (vtable
/// slot `0x10`: data, bytes); the record never holds one.
pub fn calc_buffer_size(e: &mut Engine, state: Ptr) -> u32 {
    buffer_size_of(e, state)
}

// Translated from 00d88aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A second copy of `calcBufferSize`'s code (identical bytes, so the same function from another
/// translation unit); the name is not known.
pub fn fn_00d88aa0(e: &mut Engine, state: Ptr) -> u32 {
    buffer_size_of(e, state)
}

/// The body both copies of `calcBufferSize` share.
fn buffer_size_of(e: &mut Engine, state: Ptr) -> u32 {
    e.with_stack(0x80, |e, record| {
        let at = record.addr();
        e.mem.set_u32(at + 0xc, 1);
        e.mem.set_u32(at + 0x10, 1);
        e.mem.set_u32(at + 0x64, 0x8000_0000);
        let size = calc_buffer_offsets_for_solve(e, state, 0, 0, record);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(at + 0x64);
        if capacity as i32 >= 0 {
            let allocator = e.mem.u32(router + 0x10);
            let data = e.mem.u32(at + 0x5c);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        size
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_8300, shutdown_solver()),
        entry!(0x00d8_8310, buffer_has_room(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00d8_8340, sub_solve(Ptr, u32)),
        entry!(0x00d8_83c0, calc_buffer_offsets_for_solve(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00d8_8470, constraints_fit_buffers(Ptr, Ptr, Ptr, u32) -> Ptr),
        entry!(0x00d8_8580, one_step_integrate(u32, u32, u32, Ptr, i32)),
        entry!(0x00d8_8670, internal_add_accumulators(Ptr, Ptr, u32)),
        entry!(
            0x00d8_8760,
            internal_add_jacobian_schemas(Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d8_89f0, calc_buffer_size(Ptr) -> u32),
        entry!(0x00d8_8aa0, fn_00d88aa0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine that logs every call.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of the logged calls to `address`.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Sets `count` words from `at` to `base + n`.
    fn fill_words(e: &mut Engine, at: u32, count: u32, base: u32) {
        for word in 0..count {
            e.mem.set_u32(at + 4 * word, base + word);
        }
    }

    /// `count` words from `at`.
    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    #[test]
    fn shutdown_solver_does_nothing() {
        let mut e = engine();
        let r = e.call(0x00d8_8300, &args![]);
        assert_eq!((r.eax, r.edx), (0, 0));
    }

    #[test]
    fn buffer_has_room_compares_the_bytes_needed_with_the_free_room() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        // Free room 0x200 - 0x10 = 0x1f0, and the empty buffer counts one more accumulator.
        e.mem.set_u32(buffer + 0xc, 0x100);
        e.mem.set_u32(buffer + 0x10, 0x300);
        e.mem.set_u32(buffer + 0x14, 0x100);
        let run = |e: &mut Engine, count: u32| {
            e.mem.set_u8(out, 0xee);
            let r = e.call(0x00d8_8310, &args![out, buffer, 0xdeadu32, count]);
            assert_eq!(r.u32(), out);
            e.mem.u8(out)
        };
        assert_eq!(run(&mut e, 2), 1); // 3 * 0x80 = 0x180
        assert_eq!(run(&mut e, 3), 0); // 4 * 0x80 = 0x200
                                       // Not empty: free room 0x1e0 - 0x10 = 0x1d0, no extra accumulator.
        e.mem.set_u32(buffer + 0x14, 0x120);
        assert_eq!(run(&mut e, 3), 1);
        assert_eq!(run(&mut e, 4), 0);
        // Less than nothing free: the signed compare fails even for zero accumulators.
        e.mem.set_u32(buffer + 0x10, 0x100);
        assert_eq!(run(&mut e, 0), 0);
    }

    /// A solver state for `sub_solve` with the ranges `0xa0 -> 0xb0` and `0xc0 -> 0xd0`.
    fn sub_solve_state(e: &mut Engine) -> u32 {
        let state = e.mem.alloc(0x60);
        for (offset, value) in [
            (0x04, 0x111),
            (0x0c, 0x333),
            (0x20, 0xa0),
            (0x28, 0xb0),
            (0x30, 0xc0),
            (0x38, 0xd0),
            (0x40, 0x444),
        ] {
            e.mem.set_u32(state + offset, value);
        }
        state
    }

    #[test]
    fn sub_solve_restarts_the_ranges_clears_the_floats_and_solves_both() {
        let mut e = engine();
        let state = sub_solve_state(&mut e);
        e.register(SOLVE_RANGE, |_, _| Ret::default());
        let buffer = e.mem.alloc(0x20);
        fill_words(&mut e, buffer, 8, 0x7777_0000);
        e.mem.set_u32(state + 0x48, buffer + 0x10);
        e.mem.set_u32(state + 0x4c, buffer + 4);
        e.mem.set_u32(state + 0x2c, 1);
        e.mem.set_u32(state + 0x3c, 2);
        e.call(0x00d8_8340, &args![state, 0u32]);
        assert_eq!(
            words(&e, buffer, 8),
            [
                0x7777_0000,
                0,
                0,
                0,
                0x7777_0004,
                0x7777_0005,
                0x7777_0006,
                0x7777_0007
            ]
        );
        assert_eq!(e.mem.u32(state + 0x4c), buffer + 0x10);
        assert_eq!(
            calls_to(&e, SOLVE_RANGE),
            [
                vec![0x111, 0xa0, 0x333, 0x444],
                vec![0x111, 0xc0, 0x333, 0x444]
            ]
        );
        assert_eq!(e.mem.u32(state + 0x2c), 0xb0);
        assert_eq!(e.mem.u32(state + 0x3c), 0xd0);
    }

    #[test]
    fn sub_solve_with_a_flag_keeps_the_starts_and_skips_finished_ranges() {
        let mut e = engine();
        let state = sub_solve_state(&mut e);
        e.register(SOLVE_RANGE, |_, _| Ret::default());
        // The first range is already done (start == end); the second one starts at 0x77.
        e.mem.set_u32(state + 0x2c, 0xb0);
        e.mem.set_u32(state + 0x3c, 0x77);
        e.mem.set_u32(state + 0x48, 0x5000);
        e.mem.set_u32(state + 0x4c, 0x5000);
        e.call(0x00d8_8340, &args![state, 1u32]);
        assert_eq!(calls_to(&e, SOLVE_RANGE), [vec![0x111, 0x77, 0x333, 0x444]]);
        assert_eq!(e.mem.u32(state + 0x2c), 0xb0);
        assert_eq!(e.mem.u32(state + 0x3c), 0xd0);
        assert_eq!(e.mem.u32(state + 0x4c), 0x5000);
    }

    /// A state with 2 accumulators, `+0x1c` = `divided`, `+0x14` = 10 and `+0xc` = 0x20.
    fn offsets_state(e: &mut Engine, divided: i32, count: u32) -> u32 {
        let state = e.mem.alloc(0x60);
        e.mem.set_u32(state + 0x4c, count);
        e.mem.set_i32(state + 0x1c, divided);
        e.mem.set_u32(state + 0x14, 10);
        e.mem.set_u32(state + 0xc, 0x20);
        state
    }

    #[test]
    fn calc_buffer_offsets_for_solve_lays_out_the_buffer_and_writes_the_first_accumulator() {
        let mut e = engine();
        let state = offsets_state(&mut e, 5 * 0x90 + 0x8f, 2);
        let base = e.mem.alloc(0x200);
        fill_words(&mut e, base, 0x80, 0xdead_0000);
        let out = e.mem.alloc(0x50);
        let r = e.call(0x00d8_83c0, &args![state, base, 0u32, out]);
        // rows = 2 * 5 + 8 = 18; vectors (18 * 3 + 10) * 4 + 0x17 aligned down is 0x110.
        let schemas = base + 0x190 + 0x140 + 0x10;
        assert_eq!(r.u32(), 0x3f0);
        assert_eq!(e.mem.u32(out + 0x14), base);
        assert_eq!(e.mem.u32(out + 0x30), base);
        assert_eq!(e.mem.u32(out + 0x3c), base + 0x190);
        assert_eq!(e.mem.u32(out + 0x40), schemas);
        assert_eq!(e.mem.u32(out + 0x18), 0x3f0);
        assert_eq!(e.mem.u8(base), 1);
        assert_eq!(e.mem.u32(base + 4), 0xdead_0001);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.mem.u32(base + 0xc), 0xdead_0003);
        for offset in [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70] {
            assert_eq!(words(&e, base + offset, 4), [0; 4]);
        }
        // The closing byte sits after the two accumulators and the first one.
        assert_eq!(e.mem.u8(base + 0x180), 3);
        assert_eq!(e.mem.u32(base + 0x184), 0xdead_0000 + 0x61);
    }

    #[test]
    fn calc_buffer_offsets_for_solve_rounds_a_negative_division_toward_zero() {
        let mut e = engine();
        let state = offsets_state(&mut e, -0x91, 0);
        e.mem.set_u32(state + 0x14, 0);
        e.mem.set_u32(state + 0xc, 0);
        let out = e.mem.alloc(0x50);
        let r = e.call(0x00d8_83c0, &args![state, 0u32, 0u32, out]);
        // -0x91 / 0x90 = -1, so rows = 6; (18 * 4 + 0x17) & !0xf = 0x50; schemas at 0x90 + 0x60 + 0x10.
        assert_eq!(r.u32(), 0x50 + 0x100);
        assert_eq!(e.mem.u32(out + 0x3c), 0x90);
        assert_eq!(e.mem.u32(out + 0x40), 0x100);
        assert_eq!(e.mem.u32(out + 0x18), 0x150);
        // Base 0: nothing is written at address 0.
        assert_eq!(e.mem.u32(out + 0x30), 0);
    }

    /// A constraint with the type byte `kind` whose atom info has `uses` at `+0x14` and `size`
    /// at `+0x18`.
    fn sized_constraint(e: &mut Engine, kind: u8, uses: u16, size: u16) -> u32 {
        let constraint = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x40);
        e.mem.set_u8(constraint + 0x1c, kind);
        e.mem.set_u32(constraint + 0x30, info);
        e.mem.set_u16(info + 0x14, uses);
        e.mem.set_u16(info + 0x18, size);
        constraint
    }

    #[test]
    fn constraints_fit_buffers_adds_up_pairs_and_the_trailing_constraint() {
        let mut e = engine();
        let state = e.mem.alloc(0x60);
        e.mem.set_u32(state + 0x28, 1);
        e.mem.set_u32(state + 0x38, 2);
        e.mem.set_u32(state + 0x48, 4);
        // Sums: 1 + 3 + 1 = 5 and 2 + 7 = 9; bytes 4 + (5 + 2 + 1) * 4 = 36.
        let list = [
            sized_constraint(&mut e, 0, 3, 5),
            sized_constraint(&mut e, 5, 7, 2),
            sized_constraint(&mut e, 4, 1, 1),
        ];
        let array = e.mem.alloc(0x20);
        for (index, constraint) in list.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *constraint);
        }
        let out = e.mem.alloc(4);
        let run = |e: &mut Engine, count: u32| {
            e.mem.set_u8(out, 0xee);
            let r = e.call(0x00d8_8470, &args![out, state, array, count]);
            assert_eq!(r.u32(), out);
            e.mem.u8(out)
        };
        e.mem.set_u32(state + 0x44, 36);
        e.mem.set_u32(state + 0x24, 9);
        e.mem.set_u32(state + 0x34, 13);
        assert_eq!(run(&mut e, 3), 1);
        e.mem.set_u32(state + 0x44, 35);
        assert_eq!(run(&mut e, 3), 0);
        e.mem.set_u32(state + 0x44, 36);
        e.mem.set_u32(state + 0x24, 8);
        assert_eq!(run(&mut e, 3), 0);
        e.mem.set_u32(state + 0x24, 9);
        e.mem.set_u32(state + 0x34, 12);
        assert_eq!(run(&mut e, 3), 0);
        // Two constraints (one pair): sums 4 and 9, bytes 4 + 20 + 8 = 32.
        e.mem.set_u32(state + 0x44, 32);
        e.mem.set_u32(state + 0x24, 7);
        e.mem.set_u32(state + 0x34, 13);
        assert_eq!(run(&mut e, 2), 0);
        e.mem.set_u32(state + 0x24, 8);
        assert_eq!(run(&mut e, 2), 1);
        e.mem.set_u32(state + 0x44, 31);
        assert_eq!(run(&mut e, 2), 0);
        e.mem.set_u32(state + 0x44, 32);
        // Four constraints need a second round of the pair loop; one is only the trailing one.
        let more = e.mem.alloc(0x20);
        for (index, constraint) in [list[0], list[1], list[2], list[0]].iter().enumerate() {
            e.mem.set_u32(more + 4 * index as u32, *constraint);
        }
        // Sums: 1 + 3 + 1 + 3 = 8 and 9; bytes 4 + (5 + 1 + 2 + 5) * 4 = 56.
        e.mem.set_u32(state + 0x44, 56);
        e.mem.set_u32(state + 0x24, 12);
        e.mem.set_u32(state + 0x34, 13);
        let r = e.call(0x00d8_8470, &args![out, state, more, 4u32]);
        assert_eq!(r.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u32(state + 0x44, 55);
        e.call(0x00d8_8470, &args![out, state, more, 4u32]);
        assert_eq!(e.mem.u8(out), 0);
        // One constraint: the trailing path alone (sums 4 and 2 plus the 5 * 4 bytes).
        e.mem.set_u32(state + 0x44, 24);
        e.mem.set_u32(state + 0x24, 8);
        e.mem.set_u32(state + 0x34, 6);
        assert_eq!(run(&mut e, 1), 1);
        e.mem.set_u32(state + 0x44, 23);
        assert_eq!(run(&mut e, 1), 0);
        // No constraints: the base total against its limit.
        e.mem.set_u32(state + 0x44, 4);
        e.mem.set_u32(state + 0x24, 1);
        e.mem.set_u32(state + 0x34, 2);
        assert_eq!(run(&mut e, 0), 0);
        e.mem.set_u32(state + 0x24, 5);
        e.mem.set_u32(state + 0x34, 6);
        assert_eq!(run(&mut e, 0), 1);
    }

    /// An entity with its accumulator at `offset` and the byte `old` at `+0xe9`; its owner's
    /// `+0x78` is `manager`.
    fn integrate_entity(e: &mut Engine, offset: u32, old: u8, manager: u32) -> u32 {
        let entity = e.mem.alloc(0x100);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(owner + 0x78, manager);
        e.mem.set_u32(entity + 8, owner);
        e.mem.set_u32(entity + 0xa4, offset);
        e.mem.set_u8(entity + 0xe9, old);
        entity
    }

    /// What the `ApplyAccumulators` double saw: the arguments, the record's byte, word, float
    /// and vector words, the byte at the motion's `+9`, and the motion pointer.
    type IntegrateSeen = (Vec<u32>, u8, u32, u32, Vec<u32>, u8, u32);

    #[test]
    fn one_step_integrate_copies_the_accumulator_and_restores_the_flag_byte() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x200);
        fill_words(&mut e, buffer, 0x80, 0x1000);
        e.mem.set_u32(buffer + 0x80 + 8, 0x7f80_0001);
        let entities = [
            integrate_entity(&mut e, 0, 0x11, 0x7001),
            integrate_entity(&mut e, 0x80, 0x22, 0x7002),
        ];
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, entities[0]);
        e.mem.set_u32(array + 4, entities[1]);
        let seen = Rc::new(RefCell::new(Vec::<IntegrateSeen>::new()));
        let sink = Rc::clone(&seen);
        e.register_double(APPLY_ACCUMULATORS, move |e, a| {
            let motion = e.mem.u32(a[3]);
            sink.borrow_mut().push((
                a.to_vec(),
                e.mem.u8(a[2]),
                e.mem.u32(a[2] + 4),
                e.mem.u32(a[2] + 8),
                (4..32).map(|word| e.mem.u32(a[2] + 4 * word)).collect(),
                e.mem.u8(motion + 9),
                motion,
            ));
            Ret::default()
        });
        let aabb_entities = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&aabb_entities);
        e.register_double(BATCH_RECALC_AABB, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2]));
            Ret::default()
        });
        e.call(0x00d8_8580, &args![0x51u32, 0x52u32, buffer, array, 2i32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        for (index, (call, kind, word, float, vectors, flag_during, motion)) in
            seen.iter().enumerate()
        {
            let source = 0x1000 + 0x20 * index as u32;
            assert_eq!(&call[..2], [0x51, 0x52]);
            assert_eq!(&call[4..], [1, 0]);
            assert_eq!(*motion, entities[index] + 0xe0);
            assert_eq!(*kind, (source & 0xff) as u8);
            assert_eq!(*word, source + 1);
            let expected_float = if index == 1 { 0x7fc0_0001 } else { source + 2 };
            assert_eq!(*float, expected_float);
            assert_eq!(*flag_during, 0xff);
            // Vectors at +0x10 .. +0x70 of the record follow the accumulator's words, except
            // the two at +0x40 / +0x50 that repeat the first two.
            let mut expected: Vec<u32> = (4..32).map(|word| source + word).collect();
            for word in 0..8usize {
                expected[12 + word] = source + 4 + word as u32;
            }
            assert_eq!(vectors, &expected);
        }
        assert_eq!(
            *aabb_entities.borrow(),
            [(0x7001, entities[0], 1), (0x7002, entities[1], 1)]
        );
        assert_eq!(e.mem.u8(entities[0] + 0xe9), 0x11);
        assert_eq!(e.mem.u8(entities[1] + 0xe9), 0x22);
    }

    #[test]
    fn one_step_integrate_restores_through_the_motion_the_callee_left_and_skips_empty_counts() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x100);
        let entity = integrate_entity(&mut e, 0, 0x33, 0x7003);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, entity);
        let other = e.mem.alloc(0x20);
        e.register_double(APPLY_ACCUMULATORS, move |e, a| {
            e.mem.set_u32(a[3], other);
            Ret::default()
        });
        e.register(BATCH_RECALC_AABB, |_, _| Ret::default());
        e.call(0x00d8_8580, &args![0u32, 0u32, buffer, array, 0i32]);
        e.call(0x00d8_8580, &args![0u32, 0u32, buffer, array, -3i32]);
        assert!(calls_to(&e, APPLY_ACCUMULATORS).is_empty());
        e.call(0x00d8_8580, &args![0u32, 0u32, buffer, array, 1i32]);
        assert_eq!(e.mem.u8(other + 9), 0x33);
        assert_eq!(e.mem.u8(entity + 0xe9), 0xff);
    }

    /// A state for `internal_add_accumulators`: start `region`, cursor at `region + cursor`,
    /// lanes in a fresh block filled with ones; two entities.
    fn accumulator_rig(e: &mut Engine, cursor: u32) -> (u32, u32, u32, u32, [u32; 2], u32) {
        let region = e.mem.alloc(0x400);
        fill_words(e, region, 0x100, 0xaaaa_0000);
        let lanes = e.mem.alloc(0x100);
        fill_words(e, lanes, 0x40, 0xbbbb_0000);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(state, 0x500);
        e.mem.set_u32(state + 0xc, region);
        e.mem.set_u32(state + 0x14, region + cursor);
        e.mem.set_u32(state + 0x18, lanes);
        let entities = [e.mem.alloc(0xc0), e.mem.alloc(0xc0)];
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, entities[0]);
        e.mem.set_u32(array + 4, entities[1]);
        (state, region, lanes, array, entities, region + 0x300)
    }

    #[test]
    fn internal_add_accumulators_starts_the_buffer_and_moves_the_vectors_to_the_lanes() {
        let mut e = engine();
        let (state, region, lanes, array, entities, end) = accumulator_rig(&mut e, 0);
        // The builder leaves vectors at +0x40 / +0x50 of each new accumulator.
        e.register_double(APPLY_FORCES_AND_BUILD, move |e, a| {
            let saved = a[4];
            for index in 0..2u32 {
                for word in 0..8 {
                    e.mem.set_u32(
                        saved + 0x40 + 0x80 * index + 4 * word,
                        0xcc00 + 8 * index + word,
                    );
                }
            }
            Ret {
                eax: end,
                ..Ret::default()
            }
        });
        e.call(0x00d8_8670, &args![state, array, 2u32]);
        // The empty accumulator at the start and the first two lane vectors.
        assert_eq!(e.mem.u8(region), 1);
        assert_eq!(e.mem.u32(region + 8), 0);
        assert_eq!(words(&e, region + 0x10, 4), [0; 4]);
        assert_eq!(words(&e, lanes, 8), [0; 8]);
        // Offsets: i * 0x80 - start + cursor (cursor moved to start + 0x80).
        assert_eq!(e.mem.u32(entities[0] + 0xa4), 0x80);
        assert_eq!(e.mem.u32(entities[1] + 0xa4), 0x100);
        assert_eq!(
            calls_to(&e, APPLY_FORCES_AND_BUILD),
            [vec![0x500, array, 2, 0xe0, region + 0x80]]
        );
        // Lanes: the first entity's lane is at 0x20 (offset 0x80 >> 7 << 5), then 0x20 more.
        assert_eq!(
            words(&e, lanes + 0x20, 8),
            (0..8).map(|w| 0xcc00 + w).collect::<Vec<_>>()
        );
        assert_eq!(
            words(&e, lanes + 0x40, 8),
            (0..8).map(|w| 0xcc08 + w).collect::<Vec<_>>()
        );
        assert_eq!(words(&e, region + 0x80 + 0x40, 8), [0; 8]);
        assert_eq!(words(&e, region + 0x100 + 0x40, 8), [0; 8]);
        assert_eq!(e.mem.u32(state + 0x14), end);
        assert_eq!(e.mem.u8(end), 3);
    }

    #[test]
    fn internal_add_accumulators_continues_a_started_buffer_and_ignores_zero_counts() {
        let mut e = engine();
        let (state, region, lanes, array, entities, end) = accumulator_rig(&mut e, 0x100);
        e.register_double(APPLY_FORCES_AND_BUILD, move |_, _| Ret {
            eax: end,
            ..Ret::default()
        });
        e.call(0x00d8_8670, &args![state, array, 0u32]);
        assert!(calls_to(&e, APPLY_FORCES_AND_BUILD).is_empty());
        assert_eq!(e.mem.u32(state + 0x14), region + 0x100);
        e.call(0x00d8_8670, &args![state, array, 1u32]);
        // No new first accumulator: the first lane vectors keep their words.
        assert_eq!(e.mem.u32(region), 0xaaaa_0000);
        assert_eq!(e.mem.u32(lanes), 0xbbbb_0000);
        // Offset 0 - start + cursor = 0x100, lane at (0x100 >> 7) << 5 = 0x40.
        assert_eq!(e.mem.u32(entities[0] + 0xa4), 0x100);
        assert_eq!(
            calls_to(&e, APPLY_FORCES_AND_BUILD),
            [vec![0x500, array, 1, 0xe0, region + 0x100]]
        );
        assert_eq!(e.mem.u32(lanes + 0x40), 0xaaaa_0050);
        assert_eq!(e.mem.u32(region + 0x140), 0);
        assert_eq!(e.mem.u8(end), 3);
    }

    /// Everything `internal_add_jacobian_schemas` needs, with the handles the tests read.
    struct SchemaRig {
        state: u32,
        context: u32,
        constraint: u32,
        entities: [u32; 2],
        list: u32,
        streams: [u32; 2],
        accumulators: [u32; 2],
        data: u32,
        atoms: u32,
    }

    /// One constraint between two entities (accumulator offsets `0x80` / `0x100`, mass floats
    /// `mass_a` / `mass_b`) with the type byte `kind`, atom info flags `flags` and a data
    /// object of the virtual type `data_type`; the router answers an object with allocator
    /// word `0xa11c`.
    fn schema_rig(
        e: &mut Engine,
        kind: u8,
        flags: u8,
        data_type: u32,
        atoms_type: u16,
        masses: [u32; 2],
    ) -> SchemaRig {
        let base = e.mem.alloc(0x400);
        let lanes = e.mem.alloc(0x100);
        fill_words(e, lanes + 0x20, 8, 0xa000);
        fill_words(e, lanes + 0x40, 8, 0xb000);
        let state = e.mem.alloc(0x60);
        let context = e.mem.alloc(0x80);
        let streams = [e.mem.alloc(0x40), e.mem.alloc(0x40)];
        for stream in streams {
            fill_words(e, stream, 16, 0xffff_0000);
        }
        e.mem.set_u32(state + 8, context);
        e.mem.set_u32(state + 0xc, base);
        e.mem.set_u32(state + 0x18, lanes);
        e.mem.set_u32(state + 0x28, streams[0]);
        e.mem.set_u32(state + 0x38, streams[1]);
        e.mem.set_u32(state + 0x48, 100);
        let entities = [e.mem.alloc(0x100), e.mem.alloc(0x100)];
        e.mem.set_u32(entities[0] + 0xa4, 0x80);
        e.mem.set_u32(entities[1] + 0xa4, 0x100);
        e.mem.set_u32(entities[0] + 0x5c, masses[0]);
        e.mem.set_u32(entities[1] + 0x5c, masses[1]);
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x20, 0x0900_0020);
        e.mem.set_u32(vtable + 0x38, 0x0900_0038);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 0x28, 0x9999);
        e.register_double(0x0900_0020, move |_, _| Ret {
            eax: data_type,
            ..Ret::default()
        });
        e.register(0x0900_0038, |_, _| Ret::default());
        let atoms = e.mem.alloc(0x20);
        e.mem.set_u16(atoms, atoms_type);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 4, entities[0]);
        e.mem.set_u32(info + 8, entities[1]);
        e.mem.set_u32(info + 0xc, atoms);
        e.mem.set_u16(info + 0x10, 0x77);
        e.mem.set_u8(info + 0x12, flags);
        e.mem.set_u32(info + 0x1c, 0x1357);
        e.mem.set_u16(info + 0x18, 3);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u8(constraint + 0x1c, kind);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x30, info);
        let array = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, array);
        e.mem.set_u32(list + 8, 4);
        let router_object = e.mem.alloc(0x20);
        e.mem.set_u32(router_object + 0x10, 0xa11c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router_object,
            ..Ret::default()
        });
        SchemaRig {
            state,
            context,
            constraint,
            entities,
            list,
            streams,
            accumulators: [base + 0x80, base + 0x100],
            data,
            atoms,
        }
    }

    /// A builder double that logs the two accumulators' vectors at `+0x40` / `+0x50`, the
    /// stream pointer it was given, and advances it by `0x10`.
    type BuilderSeen = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, u32)>>>;

    fn register_builder(e: &mut Engine, address: u32, rig: &SchemaRig, seen: &BuilderSeen) {
        let accumulators = rig.accumulators;
        let sink = Rc::clone(seen);
        // The stream pointer is the last argument; the data word is the second one.
        e.register_double(address, move |e, a| {
            let slot = *a.last().unwrap();
            let stream = e.mem.u32(slot);
            sink.borrow_mut().push((
                words(e, accumulators[0] + 0x40, 8),
                words(e, accumulators[1] + 0x40, 8),
                stream,
            ));
            e.mem.set_u32(slot, stream + 0x10);
            Ret::default()
        });
    }

    #[test]
    fn internal_add_jacobian_schemas_fills_the_context_and_builds_an_atoms_jacobian() {
        let mut e = engine();
        // Kind 5 selects the second stream; the data is a simple contact with a callback.
        let rig = schema_rig(
            &mut e,
            5,
            4,
            0xb,
            0x30,
            [2.0f32.to_bits(), 1.0f32.to_bits()],
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        register_builder(&mut e, BUILD_ATOM_JACOBIAN, &rig, &seen);
        e.register(FIRE_CALLBACKS, |_, _| Ret::default());
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, rig.constraint);
        e.call(0x00d8_8760, &args![rig.state, array, 1u32, rig.list]);
        let context = rig.context;
        assert_eq!(e.mem.u32(context + 0x30), rig.accumulators[0]);
        assert_eq!(e.mem.u32(context + 0x34), rig.accumulators[1]);
        assert_eq!(e.mem.u32(context + 0x38), rig.entities[0] + 0xf0);
        assert_eq!(e.mem.u32(context + 0x3c), rig.entities[1] + 0xf0);
        assert_eq!(e.mem.u32(context + 0x60), rig.constraint);
        assert_eq!(e.mem.u32(context + 0x64), 0x1357);
        assert_eq!(e.mem.u32(context + 0x6c), 1);
        assert_eq!(e.mem.u32(context + 0x70), 2);
        assert_eq!(e.mem.u32(context + 0x74), 0x1357);
        // The builder saw the copied lane vectors and the second stream's start.
        let first_lane: Vec<u32> = (0..8).map(|w| 0xa000 + w).collect();
        let second_lane: Vec<u32> = (0..8).map(|w| 0xb000 + w).collect();
        assert_eq!(*seen.borrow(), [(first_lane, second_lane, rig.streams[1])]);
        assert_eq!(
            calls_to(&e, BUILD_ATOM_JACOBIAN)
                .iter()
                .map(|call| call[..3].to_vec())
                .collect::<Vec<_>>(),
            [vec![rig.atoms, 0x77, context]]
        );
        assert_eq!(
            calls_to(&e, FIRE_CALLBACKS),
            [vec![rig.data, context, 0x9999, 1]]
        );
        assert_eq!(calls_to(&e, 0x0900_0038), [vec![rig.data, context]]);
        assert!(calls_to(&e, GROW_ARRAY).is_empty());
        // The vectors are cleared again, the list holds the element, and the sums moved on.
        assert_eq!(words(&e, rig.accumulators[0] + 0x40, 8), [0; 8]);
        assert_eq!(words(&e, rig.accumulators[1] + 0x40, 8), [0; 8]);
        let data = e.mem.u32(rig.list);
        assert_eq!(e.mem.u32(rig.list + 4), 1);
        assert_eq!(
            words(&e, data, 3),
            [rig.constraint, rig.streams[1], 1.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(rig.state + 0x48), 112);
        assert_eq!(e.mem.u32(rig.state + 0x38), rig.streams[1] + 0x10);
        assert_eq!(e.mem.u32(rig.streams[1] + 0x10), 0);
        assert_eq!(e.mem.u32(rig.streams[0]), 0);
    }

    #[test]
    fn internal_add_jacobian_schemas_grows_a_full_list_and_builds_a_contact_jacobian() {
        let mut e = engine();
        // Kind 0 selects the first stream; atoms of type 0x16 are a contact; a NaN float.
        let rig = schema_rig(&mut e, 0, 0, 5, 0x16, [0x7f80_0001, 3.0f32.to_bits()]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        register_builder(&mut e, BUILD_CONTACT_JACOBIAN, &rig, &seen);
        // Capacity 2 with a flag bit set, and 2 used: full.
        let old = e.mem.alloc(0x20);
        e.mem.set_u32(rig.list, old);
        e.mem.set_u32(rig.list + 4, 2);
        e.mem.set_u32(rig.list + 8, 0x8000_0002);
        let grown = e.mem.alloc(0x60);
        let list = rig.list;
        e.register_double(GROW_ARRAY, move |e, _| {
            e.mem.set_u32(list, grown);
            e.mem.set_u32(list + 8, 0x8000_0004);
            Ret::default()
        });
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, rig.constraint);
        e.call(0x00d8_8760, &args![rig.state, array, 1u32, rig.list]);
        assert_eq!(calls_to(&e, GROW_ARRAY), [vec![0xa11c, rig.list, 0xc]]);
        assert_eq!(
            calls_to(&e, BUILD_CONTACT_JACOBIAN)
                .iter()
                .map(|call| call[..3].to_vec())
                .collect::<Vec<_>>(),
            [vec![rig.atoms, rig.context, 1]]
        );
        assert!(calls_to(&e, FIRE_CALLBACKS).is_empty());
        assert!(calls_to(&e, 0x0900_0038).is_empty());
        assert_eq!(seen.borrow()[0].2, rig.streams[0]);
        // The element went after the two old ones; the smaller float is the second (the
        // compare against a NaN is false).
        assert_eq!(e.mem.u32(rig.list + 4), 3);
        assert_eq!(
            words(&e, grown + 24, 3),
            [rig.constraint, rig.streams[0], 3.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(rig.state + 0x28), rig.streams[0] + 0x10);
        assert_eq!(e.mem.u32(rig.streams[0] + 0x10), 0);
    }

    #[test]
    fn internal_add_jacobian_schemas_quiets_a_signalling_float_it_picks() {
        let mut e = engine();
        let rig = schema_rig(&mut e, 0, 0, 5, 0x30, [1.0f32.to_bits(), 0x7f80_0001]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        register_builder(&mut e, BUILD_ATOM_JACOBIAN, &rig, &seen);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, rig.constraint);
        e.call(0x00d8_8760, &args![rig.state, array, 1u32, rig.list]);
        let data = e.mem.u32(rig.list);
        assert_eq!(e.mem.u32(data + 8), 0x7fc0_0001);
    }

    #[test]
    fn internal_add_jacobian_schemas_with_no_constraints_only_ends_the_streams() {
        let mut e = engine();
        let rig = schema_rig(&mut e, 0, 0, 5, 0x30, [0, 0]);
        let array = e.mem.alloc(4);
        e.call(0x00d8_8760, &args![rig.state, array, 0u32, rig.list]);
        assert_eq!(e.mem.u32(rig.streams[0]), 0);
        assert_eq!(e.mem.u32(rig.streams[1]), 0);
        assert_eq!(e.mem.u32(rig.streams[0] + 4), 0xffff_0001);
        assert!(calls_to(&e, MEMORY_ROUTER).is_empty());
        assert_eq!(e.mem.u32(rig.list + 4), 0);
    }

    #[test]
    fn calc_buffer_size_answers_the_lane_offset_for_base_zero_without_freeing() {
        let mut e = engine();
        let router_object = e.mem.alloc(0x20);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router_object,
            ..Ret::default()
        });
        let state = offsets_state(&mut e, 5 * 0x90, 2);
        // Base 0: rows 18, vectors (18 * 3 + 10) * 4 + 0x17 aligned = 0x110, schemas at
        // 0x190 + 0x140 + 0x10, so the size is 0x110 + 0x2e0.
        assert_eq!(e.call(0x00d8_89f0, &args![state]).u32(), 0x3f0);
        assert_eq!(e.call(0x00d8_8aa0, &args![state]).u32(), 0x3f0);
        assert_eq!(calls_to(&e, MEMORY_ROUTER).len(), 2);
    }
}
