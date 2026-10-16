//! `Havok SDK/hkpcollectioncollectionagent3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The "agent3" of two shape collections works on the agent stream of an `hkAgent1nMachine`
//! (a pair of collections whose children are paired by bounding boxes). The unit registers it
//! with the collision dispatcher as an entry of the agent3 table: a 0x34-byte block of
//! function words (`+0` to `+0x2C`) and flag bytes (`+0x30` `isPredictive`, `+0x31`
//! `ignoreSymmetric`, `+0x32` `reusePreviousEntry`), handed to `00cfbd60`.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmenu::font::fistp;
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{load_matrix, transform_point};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, maxps, minps, store_vec, zip_with, Vec4};

/// `hkpCollisionDispatcher` agent3 registration (`00cfbd60`: `this` = the dispatcher, `RET 0xC`;
/// the entry, type a, type b).
const REGISTER_AGENT3: u32 = 0x00cf_bd60;
/// The agent stream visitor with the statistics (`hkAgent1nMachine`, cdecl: array, a word,
/// collector).
const STREAM_CALC_STATISTICS: u32 = 0x00d9_7250;
/// The agent stream visitor `00da4900` (cdecl: the stream, a 7-word block).
const STREAM_VISIT: u32 = 0x00da_4900;
/// The label `"hkpCollectionCollectionAgent3"` (`010ceca8`) the statistics are announced under.
const LABEL_STATISTICS: u32 = 0x010c_eca8;
/// A byte of the global agent3 state the registration sets (`01268ad5`).
const AGENT3_READY: u32 = 0x0126_8ad5;
/// The dispatcher byte the registration sets (`+0x1E22`).
const DISPATCHER_AGENT3: u32 = 0x1e22;

// Translated from 00d34d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollectionCollectionAgent3::calcStatistics` (name from the engine map, unconfirmed;
/// cdecl): tells the collector (slot `0x18`) the label `"hkpCollectionCollectionAgent3"` (`010ceca8`), lets `00d97250` add the
/// stream's statistics for the second and third words and the collector, and calls the
/// collector's slot `0x1C` (a tail jump). The first word is not used.
pub fn hkp_collection_collection_agent3_calc_statistics(
    e: &mut Engine,
    _unused_1: u32,
    stream: u32,
    word: u32,
    collector: Ptr,
) {
    e.vcall(collector.addr(), 0x18, &args![LABEL_STATISTICS]);
    e.call(STREAM_CALC_STATISTICS, &args![stream, word, collector]);
    e.vcall(collector.addr(), 0x1c, &args![]);
}

// Translated from 00d34db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry word `+0x18` of the collection pair's agent3 table entry (the map gives it no name;
/// cdecl): builds a 7-word block on the stack from the third to the seventh words, with the
/// answers of slot `0x10` of the objects the third and fourth words point to in between,
/// and hands it with the second word to `00da4900`. The first word is not used.
pub fn fn_00d34db0(
    e: &mut Engine,
    _unused_1: u32,
    stream: u32,
    first: Ptr,
    second: Ptr,
    fifth: u32,
    sixth: u32,
    seventh: u32,
) {
    let first_object = e.mem.u32(first.addr());
    let second_object = e.mem.u32(second.addr());
    let first_answer = e.vcall(first_object, 0x10, &args![]).u32();
    let second_answer = e.vcall(second_object, 0x10, &args![]).u32();
    e.with_stack(0x1c, |e, block| {
        let block = block.addr();
        for (index, word) in [
            first.addr(),
            second.addr(),
            first_answer,
            second_answer,
            fifth,
            sixth,
            seventh,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(block + 4 * index as u32, word);
        }
        e.call(STREAM_VISIT, &args![stream, block]);
    });
}

/// `hkAabbUtil::initOffsetAabbInput` (cdecl: the motion state, the output block); the output
/// is the block the swept boxes below are read from.
const INIT_OFFSET_AABB_INPUT: u32 = 0x00cb_ec90;
/// Copies one 32-byte integer box (cdecl: destination, source).
const COPY_AABB_INT: u32 = 0x00cf_8690;
/// The float `0.5` (`01016248`), the four-lane `1.0` (`010ec7f0`), the four-lane upper bound
/// `2147352576.0` (`011b6d70`) and the four-lane `0.0` (`01267e30`).
const HALF: u32 = 0x0101_6248;
const ONES: u32 = 0x010e_c7f0;
const BOX_UPPER: u32 = 0x011b_6d70;
const ZEROS: u32 = 0x0126_7e30;

/// The frame of the exe's functions that compute integer boxes: 0x434 bytes, with the box
/// block at `+0xA0` (the output of `00cbec90`: the motion pointer `+0`, up to the swept steps
/// of 0x40 bytes from `+0x10`, their count `+0x110`, the first step's transform `+0x120` and
/// the inverse of the motion `+0x160`) and the scratch the child shape container is given at
/// `+0x240`.
const FRAME_SIZE: u32 = 0x434;
const OFFSET_INPUT: u32 = 0xa0;
const SCRATCH: u32 = 0x240;

/// Walks the parent chain (`+0xC`) of `body` to its root and hands the root's motion (`+8`) to
/// `hkAabbUtil::initOffsetAabbInput`, which fills the block at `offset_input`.
fn init_offset_input(e: &mut Engine, body: u32, offset_input: u32) {
    let mut root = body;
    loop {
        let parent = e.mem.u32(root + 0xc);
        if parent == 0 {
            break;
        }
        root = parent;
    }
    let motion = e.mem.u32(root + 8);
    e.call(INIT_OFFSET_AABB_INPUT, &args![motion, offset_input]);
}

/// Widens the box (`min` at `aabb`, `max` at `aabb + 0x10`) by what the motion sweeps over
/// (the block at `offset_input`, see `FRAME_SIZE`): the box's centre `c` (`(1 - 0.5) * max + 0.5
/// * min`) is carried to the first step through the inverse transform and the first
/// transform; with a step count above 0, `(p - origin) * step.w + origin` (`origin` is the
/// four floats at the motion's `+0x80`, `p` the centre in the inverse transform's space) is
/// carried through every step's transform; the new box is the lane-wise `MINPS`/`MAXPS`
/// of all of these points, widened by `min(r, angle * r + h)` per lane (`h` the half
/// extents, `r` their length through `SQRTSS`, `angle` the motion's float at `+0x9C`).
fn widen_box(e: &mut Engine, offset_input: u32, aabb: u32) {
    let motion = e.mem.u32(offset_input);
    let first = load_matrix(e, offset_input + 0x120);
    let inverse = load_matrix(e, offset_input + 0x160);
    let min = load_vec(e, aabb);
    let max = load_vec(e, aabb + 0x10);
    let half = [e.mem.f32(HALF); 4];
    let ones = load_vec(e, ONES);
    let extent = zip_with(zip_with(max, min, |a, b| a - b), half, |a, b| a * b);
    let upper_part = zip_with(zip_with(ones, half, |a, b| a - b), max, |a, b| a * b);
    let lower_part = zip_with(half, min, |a, b| a * b);
    let centre = zip_with(upper_part, lower_part, |a, b| a + b);
    let inverse_centre = transform_point(&inverse, centre);
    let first_centre = transform_point(&first, inverse_centre);
    let mut upper = zip_with(centre, first_centre, maxps);
    let mut lower = zip_with(centre, first_centre, minps);
    let steps = e.mem.u32(offset_input + 0x110) as i32;
    if steps > 0 {
        let origin = load_vec(e, motion + 0x80);
        let delta = zip_with(inverse_centre, origin, |a, b| a - b);
        for step in 0..steps as u32 {
            let rows = load_matrix(e, offset_input + 0x10 + 0x40 * step);
            let scaled = zip_with([rows[3][3]; 4], delta, |a, b| a * b);
            let point = zip_with(scaled, origin, |a, b| a + b);
            let moved = transform_point(&rows, point);
            upper = zip_with(upper, moved, maxps);
            lower = zip_with(lower, moved, minps);
        }
    }
    let squares = zip_with(extent, extent, |a, b| a * b);
    let sum_xy = squares[1] + squares[0];
    let length = (squares[2] + sum_xy).sqrt();
    let angle = e.mem.f32(motion + 0x9c);
    let reach = zip_with([angle * length; 4], extent, |a, b| a + b);
    let radius = zip_with([length; 4], reach, minps);
    store_vec(e, aabb, zip_with(lower, radius, |a, b| a - b));
    store_vec(e, aabb + 0x10, zip_with(upper, radius, |a, b| a + b));
}

/// The four words of a box corner: `sum * scale` per lane, `MINPS` against `2147352576.0`,
/// `MAXPS` against 0, then `FISTP` (the default rounding, nearest even).
fn int_lanes(sum: Vec4, scale: Vec4, upper: Vec4, zeros: Vec4) -> [u32; 4] {
    let scaled = zip_with(sum, scale, |a, b| a * b);
    let capped = zip_with(scaled, upper, minps);
    zip_with(capped, zeros, maxps).map(|lane| fistp(f64::from(lane)) as u32)
}

/// The 32-byte integer box (`AabbInt`) at `dst`, made from the box (`min` at `aabb`, `max` at
/// `aabb + 0x10`) with the offsets and scale of the input (offsets `+0x20` for the minimum,
/// `+0x30` for the maximum, scale `+0x40`): lane-wise `(offset + min) * scale` and `(max +
/// offset) * scale`, converted as in `int_lanes`. The minimum's words go to `dst + 0` to
/// `+8`, `key` to `+0xC`, the maximum's four words to `dst + 0x10` to `+0x1C`.
fn write_int_box(e: &mut Engine, input: u32, aabb: u32, dst: u32, key: u32) {
    let scale = load_vec(e, input + 0x40);
    let upper = load_vec(e, BOX_UPPER);
    let zeros = load_vec(e, ZEROS);
    let min = load_vec(e, aabb);
    let max = load_vec(e, aabb + 0x10);
    let offset_min = load_vec(e, input + 0x20);
    let offset_max = load_vec(e, input + 0x30);
    let low = int_lanes(zip_with(offset_min, min, |a, b| a + b), scale, upper, zeros);
    let high = int_lanes(zip_with(max, offset_max, |a, b| a + b), scale, upper, zeros);
    for lane in 0..3 {
        e.mem.set_u32(dst + 4 * lane, low[lane as usize]);
    }
    for lane in 0..4 {
        e.mem.set_u32(dst + 0x10 + 4 * lane, high[lane as usize]);
    }
    e.mem.set_u32(dst + 0xc, key);
}

/// The 32-byte integer box at `dst` made from a compact box at `src` (words at `+0` to `+8`
/// and `+0x10` to `+0x18`, extra bytes at `+0xC` to `+0xE` and `+0x1C` to `+0x1E`, a shift at
/// `+0xF`): the minimum is each word minus its byte shifted left by the shift (`dst + 0` to
/// `+8`), the maximum each word plus its byte shifted (`dst + 0x10` to `+0x18`). The key
/// (`dst + 0xC`) and the last word (`dst + 0x1C`) are not written.
fn decode_compact_box(e: &mut Engine, src: u32, dst: u32) {
    let shift = u32::from(e.mem.u8(src + 0xf));
    let extra = |e: &Engine, byte_at: u32| u32::from(e.mem.u8(src + byte_at)).wrapping_shl(shift);
    for lane in 0..3u32 {
        let low = e.mem.u32(src + 4 * lane).wrapping_sub(extra(e, 0xc + lane));
        e.mem.set_u32(dst + 4 * lane, low);
        let high = e
            .mem
            .u32(src + 0x10 + 4 * lane)
            .wrapping_add(extra(e, 0x1c + lane));
        e.mem.set_u32(dst + 0x10 + 4 * lane, high);
    }
}

// Translated from 00d34e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the integer boxes of `count` children of a shape collection (the map gives the
/// function no name; cdecl). `agent` holds the input at `+0xC` (tolerance `+8`, offsets `+0x20`
/// and `+0x30`, scale `+0x40`, and through `+0x60` a block whose word `+0x10` says whether the
/// motion sweeps), `container` is the object that gives the child shape of a key (slot
/// `0x14`: the key and a scratch block) and `body` the collidable whose motion (`+8`) the
/// children are asked for their box with (slot `0x1C` of the child: the motion, the
/// tolerance as an `FLD`/`FSTP` copy, the box). For each key of `keys` the box is widened by
/// the sweep (`widen_box`, with the block `00cbec90` makes from the root of `body`'s parent
/// chain) when the sweep word is set, and written to `out` as a 32-byte element (see
/// `write_int_box`; `out + 0xC` is the key). Assumes the default x87 rounding mode.
pub fn fn_00d34e10(
    e: &mut Engine,
    agent: Ptr,
    container: Ptr,
    body: Ptr,
    keys: Ptr,
    count: i32,
    out: Ptr,
) {
    let sweep_source = e.mem.u32(e.mem.u32(agent.addr() + 0xc) + 0x60);
    let sweeps = e.mem.u32(sweep_source + 0x10) != 0;
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let aabb = frame + 0x30;
        if sweeps {
            init_offset_input(e, body.addr(), frame + OFFSET_INPUT);
        }
        let mut index = 0;
        while index < count {
            let key = e.mem.u32(keys.addr() + 4 * index as u32);
            let shape = e
                .vcall(container.addr(), 0x14, &args![key, frame + SCRATCH])
                .u32();
            let input = e.mem.u32(agent.addr() + 0xc);
            let tolerance = x87_quiet_bits(e.mem.u32(input + 8));
            let transform = e.mem.u32(body.addr() + 8);
            e.vcall(shape, 0x1c, &args![transform, tolerance, aabb]);
            if sweeps {
                widen_box(e, frame + OFFSET_INPUT, aabb);
            }
            let input = e.mem.u32(agent.addr() + 0xc);
            write_int_box(e, input, aabb, out.addr() + 0x20 * index as u32, key);
            index += 1;
        }
    });
}

// Translated from 00d351c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCollectionBvTreeAgent3_extractCachedAabbsOrRecalculate` (name from the engine map; cdecl):
/// fills `out` with the 32-byte integer boxes of the children of the collection `iter`
/// walks and answers how many there are. `body` is the collidable (shape container
/// `+0x44`, parent `+0xC`, motion `+8`, a compact box block at `+0x20` with a counter at
/// `+0x40` and a key array at `+0x48`) and `input` the process input (tolerance `+8`, offsets
/// `+0x20`/`+0x30`, scale `+0x40` and, through `+0x60`, the sweep word at `+0x10` as for
/// `00d34e10`).
///
/// A body without a parent first tries its cache. With the cache pointer set and the block's
/// first word not above the one at `+0x10` it decodes one element per cached box (the
/// counter `+0x40`): with the sweep word set from the compact form (`decode_compact_box`),
/// else by `00cf8690`, the key from the key array. An empty counter answers 0. Without that
/// cache but with the first word not above the one at `+0x10` and `flag` set the one box in
/// the body's compact block is decoded (or copied) as element 0 with key 0 and 1 is the
/// answer. Otherwise the boxes are made again: with the sweep word set the block of
/// `00cbec90` is made from the root of the parent chain; `iter` gives the first key (slot
/// 8; -1 ends), the child shape (slot `0x14`: the key and a scratch block) and the next key
/// (slot `0xC`); the child's box (slot `0x1C`: the body's motion, the tolerance copy, the
/// box) is widened (`widen_box`) when sweeping, and written as in `00d34e10`. With the sweep
/// word set and more than one element, the elements are sorted by `00d36360` (the unsigned
/// word at `+0`), except for the cache without it and the single box. Assumes the default x87
/// rounding mode.
pub fn hk_collection_bv_tree_agent3_extract_cached_aabbs_or_recalculate(
    e: &mut Engine,
    body: Ptr,
    flag: u32,
    iter: Ptr,
    input: Ptr,
    out: Ptr,
) -> u32 {
    let body_at = body.addr();
    let sweep_source = e.mem.u32(input.addr() + 0x60);
    let sweeps = e.mem.u32(sweep_source + 0x10) != 0;
    let out_at = out.addr();
    if e.mem.u32(body_at + 0xc) == 0 {
        let cache = e.mem.u32(body_at + 0x44);
        let block = body_at + 0x20;
        if cache != 0 && e.mem.u32(block) <= e.mem.u32(block + 0x10) {
            let boxes = i32::from(e.mem.u16(block + 0x20));
            if boxes == 0 {
                return 0;
            }
            let keys = e.mem.u32(block + 0x28);
            let mut index = 0;
            let mut count = 0;
            while index < boxes {
                let src = cache + 0x20 * index as u32;
                let dst = out_at + 0x20 * index as u32;
                if sweeps {
                    decode_compact_box(e, src, dst);
                } else {
                    e.call(COPY_AABB_INT, &args![dst, src]);
                }
                let key = e.mem.u32(keys + 4 * index as u32);
                e.mem.set_u32(dst + 0xc, key);
                count += 1;
                index += 1;
            }
            return if sweeps {
                sort_boxes(e, out_at, count)
            } else {
                count
            };
        }
        if e.mem.u32(block) <= e.mem.u32(block + 0x10) && flag != 0 {
            if sweeps {
                decode_compact_box(e, block, out_at);
            } else {
                e.call(COPY_AABB_INT, &args![out_at, block]);
            }
            e.mem.set_u32(out_at + 0xc, 0);
            return 1;
        }
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let aabb = frame + 0x40;
        if sweeps {
            init_offset_input(e, body_at, frame + OFFSET_INPUT);
        }
        let mut count = 0u32;
        let mut key = e.vcall(iter.addr(), 8, &args![]).u32();
        while key != 0xffff_ffff {
            let shape = e
                .vcall(iter.addr(), 0x14, &args![key, frame + SCRATCH])
                .u32();
            let tolerance = x87_quiet_bits(e.mem.u32(input.addr() + 8));
            let transform = e.mem.u32(body_at + 8);
            e.vcall(shape, 0x1c, &args![transform, tolerance, aabb]);
            if sweeps {
                widen_box(e, frame + OFFSET_INPUT, aabb);
            }
            write_int_box(e, input.addr(), aabb, out_at + 0x20 * count, key);
            count += 1;
            key = e.vcall(iter.addr(), 0xc, &args![key]).u32();
        }
        sort_boxes(e, out_at, count)
    })
}

/// The end of `00d351c0`: with more than one element in `out`, `00d36360` sorts them (the
/// comparison object is a word of which the exe passes only the low byte, 0; the sort does
/// not read it). Answers `count`.
fn sort_boxes(e: &mut Engine, out: u32, count: u32) -> u32 {
    if count as i32 > 1 {
        hk_algorithm_quick_sort_recursive_hk1_axis_sweep_aabb_int_hk_algorithm_less_hk1_axis_sweep_aabb_int(
            e,
            Ptr::new(out),
            0,
            count as i32 - 1,
            0,
        );
    }
    count
}

// Translated from 00d36270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent3 table entry (the map gives it no name; cdecl): function words `+0`
/// `00da6fa0`, `+4` `00da6dc0`, `+8`, `+0xC`, `+0x10`, `+0x14` and `+0x28` zero, `+0x18`
/// `00d34db0`, `+0x1C` `00d34d30`, `+0x20` `00d34d60`, `+0x24` `00d34d80`, `+0x2C` `00d35770`,
/// and the flag byte `+0x30` 1. The bytes after it (`+0x31` to `+0x33`) are not written.
pub fn fn_00d36270(e: &mut Engine, entry: Ptr) {
    let entry = entry.addr();
    for (offset, word) in [
        (0, 0x00da_6fa0),
        (0x2c, 0x00d3_5770),
        (0x28, 0),
        (8, 0),
        (0xc, 0),
        (0x10, 0),
        (0x14, 0),
        (0x18, 0x00d3_4db0),
        (0x1c, 0x00d3_4d30),
        (0x20, 0x00d3_4d60),
        (0x24, 0x00d3_4d80),
        (4, 0x00da_6dc0),
    ] {
        e.mem.set_u32(entry + offset, word);
    }
    e.mem.set_u8(entry + 0x30, 1);
}

// Translated from 00d362c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the collection-pair agent3 with a dispatcher (the map gives it no name; cdecl).
/// The entry is built in a zeroed block on the stack (`00d36270` fills it; the flag bytes
/// `+0x31`, `+0x32` are set to 1 after the first registration) and handed to `00cfbd60` for
/// the shape type pairs (8, 8), (7, 8), (8, 7) and (7, 7). Then the dispatcher's byte
/// `+0x1E22` and the global byte `01268ad5` are set to 1.
pub fn fn_00d362c0(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x40, |e, entry| {
        let entry = entry.addr();
        fn_00d36270(e, Ptr::new(entry));
        e.call(REGISTER_AGENT3, &args![dispatcher, entry, 8u32, 8u32]);
        e.mem.set_u8(entry + 0x31, 1);
        e.mem.set_u8(entry + 0x32, 1);
        for (type_a, type_b) in [(7u32, 8u32), (8, 7), (7, 7)] {
            e.call(REGISTER_AGENT3, &args![dispatcher, entry, type_a, type_b]);
        }
    });
    e.mem.set_u8(dispatcher.addr() + DISPATCHER_AGENT3, 1);
    e.mem.set_u8(AGENT3_READY, 1);
}

// Translated from 00d36360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hk1AxisSweep::AabbInt, hkAlgorithm::less<hk1AxisSweep::AabbInt>>`
/// (name from the engine map; the body is a quicksort of 32-byte elements): sorts the
/// elements `low` to `high` (inclusive, signed indexes) of `array` by the unsigned word at
/// their `+0`. The pivot is the key of the middle element `(low + high) >> 1`; the two
/// indexes run towards each other, exchanging out-of-place elements (32 bytes); then the
/// lower part is sorted by recursion and the upper part by looping. `compare` (the empty
/// comparison object, one word on the stack) is only handed on to the recursion.
pub fn hk_algorithm_quick_sort_recursive_hk1_axis_sweep_aabb_int_hk_algorithm_less_hk1_axis_sweep_aabb_int(
    e: &mut Engine,
    array: Ptr,
    low: i32,
    high: i32,
    compare: u32,
) {
    let element = |index: i32| array.addr().wrapping_add((index as u32) << 5);
    let mut low = low;
    loop {
        let middle = low.wrapping_add(high) >> 1;
        let key = e.mem.u32(element(middle));
        let (mut i, mut j) = (low, high);
        loop {
            while e.mem.u32(element(i)) < key {
                i = i.wrapping_add(1);
            }
            while key < e.mem.u32(element(j)) {
                j = j.wrapping_sub(1);
            }
            if j < i {
                break;
            }
            if j != i {
                let at_i = e.mem.bytes(element(i), 0x20);
                let at_j = e.mem.bytes(element(j), 0x20);
                e.mem.write(element(i), &at_j);
                e.mem.write(element(j), &at_i);
            }
            j = j.wrapping_sub(1);
            i = i.wrapping_add(1);
            if i > j {
                break;
            }
        }
        if low < j {
            hk_algorithm_quick_sort_recursive_hk1_axis_sweep_aabb_int_hk_algorithm_less_hk1_axis_sweep_aabb_int(
                e, array, low, j, compare,
            );
        }
        if i >= high {
            return;
        }
        low = i;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d3_4d30,
            hkp_collection_collection_agent3_calc_statistics(u32, u32, u32, Ptr)
        ),
        entry!(0x00d3_4db0, fn_00d34db0(u32, u32, Ptr, Ptr, u32, u32, u32)),
        entry!(0x00d3_4e10, fn_00d34e10(Ptr, Ptr, Ptr, Ptr, i32, Ptr)),
        entry!(
            0x00d3_51c0,
            hk_collection_bv_tree_agent3_extract_cached_aabbs_or_recalculate(
                Ptr, u32, Ptr, Ptr, Ptr
            ) -> u32
        ),
        entry!(0x00d3_6270, fn_00d36270(Ptr)),
        entry!(0x00d3_62c0, fn_00d362c0(Ptr)),
        entry!(
            0x00d3_6360,
            hk_algorithm_quick_sort_recursive_hk1_axis_sweep_aabb_int_hk_algorithm_less_hk1_axis_sweep_aabb_int(Ptr, i32, i32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const COLLECTOR: u32 = 0x0900_0000;
    const FIRST: u32 = 0x0910_0000;
    const SECOND: u32 = 0x0920_0000;

    fn words(e: &Engine, at: u32, n: u32) -> Vec<u32> {
        (0..n).map(|word| e.mem.u32(at + 4 * word)).collect()
    }

    #[test]
    fn calc_statistics_brackets_the_stream_statistics_00d34d30() {
        let mut e = engine();
        let log = new_log();
        let collector = logging_object(&mut e, &log, COLLECTOR);
        log_double(&mut e, &log, STREAM_CALC_STATISTICS, 0);
        e.call(0x00d3_4d30, &[0xdead, 0x11, 0x22, collector]);
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![COLLECTOR + 0x18, STREAM_CALC_STATISTICS, COLLECTOR + 0x1c]
        );
        assert_eq!(
            calls(&log, COLLECTOR + 0x18),
            vec![vec![collector, LABEL_STATISTICS]]
        );
        assert_eq!(
            calls(&log, STREAM_CALC_STATISTICS),
            vec![vec![0x11, 0x22, collector]]
        );
        assert_eq!(calls(&log, COLLECTOR + 0x1c), vec![vec![collector]]);
    }

    #[test]
    fn the_visit_block_holds_the_words_and_the_two_answers_00d34db0() {
        let mut e = engine();
        let log = new_log();
        let first = logging_object(&mut e, &log, FIRST);
        let second = logging_object(&mut e, &log, SECOND);
        log_double(&mut e, &log, FIRST + 0x10, 0xa1);
        log_double(&mut e, &log, SECOND + 0x10, 0xb2);
        let holders = e.mem.alloc(8);
        e.mem.set_u32(holders, first);
        e.mem.set_u32(holders + 4, second);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(STREAM_VISIT, move |e, a| {
            let mut noted = a.to_vec();
            noted.extend(words(e, a[1], 7));
            copy.borrow_mut().push(noted);
            Ret::default()
        });
        e.call(
            0x00d3_4db0,
            &[0xdead, 0x1000, holders, holders + 4, 0x55, 0x66, 0x77],
        );
        assert_eq!(
            *seen.borrow(),
            vec![vec![
                0x1000,
                seen.borrow()[0][1],
                holders,
                holders + 4,
                0xa1,
                0xb2,
                0x55,
                0x66,
                0x77
            ]]
        );
        assert_eq!(calls(&log, FIRST + 0x10), vec![vec![first]]);
        assert_eq!(calls(&log, SECOND + 0x10), vec![vec![second]]);
    }

    #[test]
    fn the_entry_filler_sets_the_functions_and_the_first_flag_00d36270() {
        let mut e = engine();
        let entry = e.mem.alloc(0x40);
        e.mem.write(entry, &[0xaa; 0x40]);
        e.call(0x00d3_6270, &[entry]);
        assert_eq!(
            words(&e, entry, 12),
            vec![
                0x00da_6fa0,
                0x00da_6dc0,
                0,
                0,
                0,
                0,
                0x00d3_4db0,
                0x00d3_4d30,
                0x00d3_4d60,
                0x00d3_4d80,
                0,
                0x00d3_5770
            ]
        );
        assert_eq!(e.mem.u8(entry + 0x30), 1);
        assert_eq!(e.mem.bytes(entry + 0x31, 3), vec![0xaa; 3]);
        assert_eq!(e.mem.u32(entry + 0x34), 0xaaaa_aaaa);
    }

    #[test]
    fn registration_covers_the_four_type_pairs_00d362c0() {
        let mut e = engine();
        let dispatcher = e.mem.alloc(0x2000);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT3, move |e, a| {
            let mut noted = a.to_vec();
            noted.extend(words(e, a[1], 12));
            noted.extend(e.mem.bytes(a[1] + 0x30, 4).into_iter().map(u32::from));
            copy.borrow_mut().push(noted);
            Ret::default()
        });
        e.call(0x00d3_62c0, &[dispatcher]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 4);
        let pairs: Vec<[u32; 2]> = seen.iter().map(|call| [call[2], call[3]]).collect();
        assert_eq!(pairs, vec![[8, 8], [7, 8], [8, 7], [7, 7]]);
        let flags: Vec<[u32; 4]> = seen
            .iter()
            .map(|call| [call[16], call[17], call[18], call[19]])
            .collect();
        // The entry is the same block; the flag bytes +0x31 and +0x32 are set after the first.
        assert_eq!(flags[0], [1, 0, 0, 0]);
        assert_eq!(flags[1..], [[1, 1, 1, 0]; 3]);
        for call in seen.iter() {
            assert_eq!(call[0], dispatcher);
            assert_eq!(call[4..16], seen[0][4..16]);
        }
        assert_eq!(seen[0][4..8], [0x00da_6fa0, 0x00da_6dc0, 0, 0]);
        assert_eq!(e.mem.u8(dispatcher + 0x1e22), 1);
        assert_eq!(e.mem.u8(AGENT3_READY), 1);
    }

    /// An array of 32-byte elements: the key `keys[k]` at `+0`, the rest a pattern of `k`.
    fn make_array(e: &mut Engine, keys: &[u32]) -> u32 {
        let array = e.mem.alloc(32 * keys.len() as u32);
        for (k, key) in keys.iter().enumerate() {
            let at = array + 32 * k as u32;
            e.mem.set_u32(at, *key);
            for word in 1..8 {
                e.mem.set_u32(at + 4 * word, 100 * k as u32 + word);
            }
        }
        array
    }

    fn elements(e: &Engine, array: u32, count: usize) -> Vec<Vec<u32>> {
        (0..count)
            .map(|k| words(e, array + 32 * k as u32, 8))
            .collect()
    }

    /// Sorts the elements `low` to `high` of an array with the given keys and checks that
    /// they ended up in the order of the keys, as a permutation of the same elements, and the
    /// others did not move.
    fn sort_case(keys: &[u32], low: i32, high: i32) {
        let mut e = engine();
        let array = make_array(&mut e, keys);
        let before = elements(&e, array, keys.len());
        e.call(0x00d3_6360, &[array, low as u32, high as u32, 0]);
        let after = elements(&e, array, keys.len());
        let (from, to) = (low as usize, high as usize);
        for k in 0..keys.len() {
            if k < from || k > to {
                assert_eq!(after[k], before[k], "element {k} must not move");
            }
        }
        let mut moved: Vec<Vec<u32>> = after[from..=to].to_vec();
        let mut original: Vec<Vec<u32>> = before[from..=to].to_vec();
        for pair in moved.windows(2) {
            assert!(
                pair[0][0] <= pair[1][0],
                "{:?} before {:?}",
                pair[0],
                pair[1]
            );
        }
        moved.sort();
        original.sort();
        assert_eq!(moved, original);
    }

    #[test]
    fn sorts_by_the_unsigned_key_and_keeps_the_whole_elements_00d36360() {
        let keys = [
            5,
            3,
            0xffff_fff0,
            3,
            9,
            0x8000_0000,
            0,
            5,
            5,
            1,
            7,
            0xffff_fff0,
            2,
            0x7fff_ffff,
        ];
        sort_case(&keys, 0, keys.len() as i32 - 1);
    }

    #[test]
    fn sorts_only_the_given_range_00d36360() {
        let keys = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 11, 10];
        sort_case(&keys, 2, 8);
        sort_case(&keys, 4, 5);
        sort_case(&keys, 6, 6);
    }

    #[test]
    fn sorted_reversed_and_equal_keys_work_too_00d36360() {
        let up: Vec<u32> = (0..20).collect();
        let down: Vec<u32> = (0..20).rev().collect();
        sort_case(&up, 0, 19);
        sort_case(&down, 0, 19);
        sort_case(&[4; 17], 0, 16);
    }

    #[test]
    fn sorts_a_longer_pseudo_random_array_00d36360() {
        let mut state = 12345u32;
        let keys: Vec<u32> = (0..200)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                state >> 20
            })
            .collect();
        sort_case(&keys, 0, 199);
    }

    const CONTAINER: u32 = 0x0950_0000;
    const SHAPE: u32 = 0x0960_0000;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;

    /// Puts the constants the box code loads into memory.
    fn constants(e: &mut Engine) {
        e.mem.set_f32(HALF, 0.5);
        store_vec(e, ONES, [1.0; 4]);
        store_vec(e, BOX_UPPER, [f32::from_bits(0x4eff_fc00); 4]);
        store_vec(e, ZEROS, [0.0; 4]);
    }

    /// The boxes (minimum, maximum) a child shape's `getAabb` answers with, in call order.
    type Boxes = Rc<RefCell<Vec<[Vec4; 2]>>>;

    /// A scene of box making: an input (tolerance `tolerance_bits`, offsets (0.5, 1.5, 2.5,
    /// 3.5) and (10, 20, 30, 40), scale (2, 4, 8, 16), and behind `+0x60` a block with the
    /// sweep word `sweep`), a shape whose `getAabb` answers `answers` in order and a
    /// container (`container` object, slot `0x14` gives the shape).
    struct Scene {
        e: Engine,
        log: Log,
        input: u32,
        shape: u32,
        container: u32,
    }

    fn scene(tolerance_bits: u32, sweep: u32, answers: Vec<[Vec4; 2]>) -> Scene {
        let mut e = engine();
        constants(&mut e);
        let log = new_log();
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input + 8, tolerance_bits);
        store_vec(&mut e, input + 0x20, [0.5, 1.5, 2.5, 3.5]);
        store_vec(&mut e, input + 0x30, [10.0, 20.0, 30.0, 40.0]);
        store_vec(&mut e, input + 0x40, [2.0, 4.0, 8.0, 16.0]);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 0x10, sweep);
        e.mem.set_u32(input + 0x60, block);
        let shape = logging_object(&mut e, &log, SHAPE);
        let container = logging_object(&mut e, &log, CONTAINER);
        log_double(&mut e, &log, CONTAINER + 0x14, shape);
        let queue: Boxes = Rc::new(RefCell::new(answers));
        let notes = Rc::clone(&log);
        e.register_double(SHAPE + 0x1c, move |e, a| {
            notes.borrow_mut().push((SHAPE + 0x1c, a.to_vec()));
            let [min, max] = queue.borrow_mut().remove(0);
            store_vec(e, a[3], min);
            store_vec(e, a[3] + 0x10, max);
            Ret::default()
        });
        Scene {
            e,
            log,
            input,
            shape,
            container,
        }
    }

    fn int_words(e: &Engine, at: u32) -> Vec<u32> {
        words(e, at, 8)
    }

    /// An agent whose input (`+0xC`) is the scene's.
    fn agent_of(s: &mut Scene) -> u32 {
        let agent = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(agent + 0xc, s.input);
        agent
    }

    #[test]
    fn writes_scaled_rounded_and_clamped_boxes_00d34e10() {
        let answers = vec![
            [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]],
            [[-100.0, f32::NAN, 1e30, 0.0], [0.1, -5.0, 1e30, f32::NAN]],
            // Ties go to the even word.
            [[0.75, -0.625, -2.4375, 0.0], [-9.25, -19.375, 0.0, 0.0]],
        ];
        let mut s = scene(SIGNALLING_NAN, 0, answers);
        let agent = agent_of(&mut s);
        let body = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(body + 8, 0x7777);
        let keys = s.e.mem.alloc(0x10);
        for (n, key) in [7u32, 9, 11].into_iter().enumerate() {
            s.e.mem.set_u32(keys + 4 * n as u32, key);
        }
        let out = s.e.mem.alloc(0x60);
        s.e.mem.write(out, &[0xaa; 0x60]);
        s.e.call(0x00d3_4e10, &[agent, s.container, body, keys, 3, out]);
        assert_eq!(int_words(&s.e, out), vec![3, 14, 44, 7, 30, 104, 296, 768]);
        let upper = 0x7ffe_0000;
        assert_eq!(
            int_words(&s.e, out + 0x20),
            vec![0, upper, upper, 9, 20, 60, upper, upper]
        );
        assert_eq!(
            int_words(&s.e, out + 0x40),
            vec![2, 4, 0, 11, 2, 2, 240, 640]
        );
        // The container is asked for each key with a scratch block, the child for its box
        // with the body's transform and the quieted tolerance.
        let asked = calls(&s.log, CONTAINER + 0x14);
        let keys_asked: Vec<u32> = asked.iter().map(|call| call[1]).collect();
        assert_eq!(keys_asked, vec![7, 9, 11]);
        assert_ne!(asked[0][2], 0);
        let boxes = calls(&s.log, SHAPE + 0x1c);
        assert_eq!(boxes.len(), 3);
        assert_eq!(boxes[0][..3], [s.shape, 0x7777, 0x7fe0_0001]);
    }

    /// Registers the double of `00cbec90` that notes its arguments only.
    fn note_init(s: &mut Scene) {
        let log = Rc::clone(&s.log);
        log_double(&mut s.e, &log, INIT_OFFSET_AABB_INPUT, 0);
    }

    #[test]
    fn a_count_of_zero_makes_no_boxes_but_a_sweep_still_asks_for_the_block_00d34e10() {
        let mut s = scene(0, 1, Vec::new());
        note_init(&mut s);
        let agent = agent_of(&mut s);
        let body = s.e.mem.alloc(0x20);
        let out = s.e.mem.alloc(0x20);
        s.e.call(0x00d3_4e10, &[agent, s.container, body, 0x3000, 0, out]);
        assert_eq!(calls(&s.log, INIT_OFFSET_AABB_INPUT).len(), 1);
        assert!(calls(&s.log, SHAPE + 0x1c).is_empty());
        assert_eq!(int_words(&s.e, out), vec![0; 8]);
    }

    /// A body `body` whose parent chain ends in a root with a motion block (origin (0.5, 0, 0,
    /// 0), `angle`), and the double of `00cbec90`: it notes `(motion, block)` and fills the
    /// block with identity inverse and first transforms (first translation (1, 0, 0, 0)), the
    /// step count and identity steps with the translations `translations`. Returns the body
    /// and the motion.
    fn sweep_rig(s: &mut Scene, angle: f32, translations: &[Vec4]) -> (u32, u32) {
        let motion = s.e.mem.alloc(0xb0);
        store_vec(&mut s.e, motion + 0x80, [0.5, 0.0, 0.0, 0.0]);
        s.e.mem.set_f32(motion + 0x9c, angle);
        let root = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(root + 8, motion);
        let middle = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(middle + 0xc, root);
        let body = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(body + 8, 0x7777);
        s.e.mem.set_u32(body + 0xc, middle);
        let steps = translations.to_vec();
        let notes = Rc::clone(&s.log);
        s.e.register_double(INIT_OFFSET_AABB_INPUT, move |e, a| {
            notes
                .borrow_mut()
                .push((INIT_OFFSET_AABB_INPUT, a.to_vec()));
            e.mem.set_u32(a[1], a[0]);
            let identity = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ];
            for (n, translation) in steps.iter().enumerate() {
                let at = a[1] + 0x10 + 0x40 * n as u32;
                for (row, value) in identity.iter().enumerate() {
                    store_vec(e, at + 0x10 * row as u32, *value);
                }
                store_vec(e, at + 0x30, *translation);
            }
            e.mem.set_u32(a[1] + 0x110, steps.len() as u32);
            for (base, translation) in [(0x120, [1.0, 0.0, 0.0, 0.0]), (0x160, [0.0; 4])] {
                for (row, value) in identity.iter().enumerate() {
                    store_vec(e, a[1] + base + 0x10 * row as u32, *value);
                }
                store_vec(e, a[1] + base + 0x30, translation);
            }
            Ret::default()
        });
        (body, motion)
    }

    /// A scene with one box, minimum (-1, -2, -3, 0) and maximum (3, 2, 1, 0), whose input has
    /// the sweep word set, the minimum's offset 10, no maximum offset and the scale 1.
    fn sweep_scene(angle: f32, translations: &[Vec4]) -> (Scene, u32, u32) {
        let answers = vec![[[-1.0, -2.0, -3.0, 0.0], [3.0, 2.0, 1.0, 0.0]]];
        let mut s = scene(0, 1, answers);
        store_vec(&mut s.e, s.input + 0x20, [10.0; 4]);
        store_vec(&mut s.e, s.input + 0x30, [0.0; 4]);
        store_vec(&mut s.e, s.input + 0x40, [1.0; 4]);
        let (body, motion) = sweep_rig(&mut s, angle, translations);
        (s, body, motion)
    }

    /// Runs `00d34e10` on the sweep scene for one key (0x31); returns the words made.
    fn sweep_boxes(angle: f32, translations: &[Vec4]) -> (Vec<u32>, Vec<Vec<u32>>, u32) {
        let (mut s, body, motion) = sweep_scene(angle, translations);
        let agent = agent_of(&mut s);
        let keys = s.e.mem.alloc(4);
        s.e.mem.set_u32(keys, 0x31);
        let out = s.e.mem.alloc(0x20);
        s.e.call(0x00d3_4e10, &[agent, s.container, body, keys, 1, out]);
        (
            int_words(&s.e, out),
            calls(&s.log, INIT_OFFSET_AABB_INPUT),
            motion,
        )
    }

    #[test]
    fn sweeping_boxes_cover_every_step_00d34e10() {
        let steps = [[10.0, 0.0, 0.0, 2.0], [-4.0, 3.0, 0.0, 0.0]];
        let (made, init, motion) = sweep_boxes(0.0, &steps);
        assert_eq!(made, vec![4, 8, 6, 0x31, 14, 5, 2, 2]);
        // The block is made from the root of the parent chain.
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][0], motion);
    }

    #[test]
    fn sweeping_boxes_without_steps_grow_by_the_angle_00d34e10() {
        let (made, _, _) = sweep_boxes(0.0, &[]);
        assert_eq!(made, vec![9, 8, 7, 0x31, 4, 2, 1, 0]);
        let (made, _, _) = sweep_boxes(0.5, &[]);
        assert_eq!(made, vec![8, 7, 6, 0x31, 5, 3, 2, 2]);
    }

    /// The compact box at `at`: words `low` at `+0` to `+8` and `high` at `+0x10` to `+0x18`,
    /// the extra bytes `extra_low` at `+0xC` to `+0xE` and `extra_high` at `+0x1C` to `+0x1E`
    /// and the shift at `+0xF`.
    fn compact_box(
        e: &mut Engine,
        at: u32,
        words_low_high: ([u32; 3], [u32; 3]),
        extra: ([u8; 3], [u8; 3]),
        shift: u8,
    ) {
        for lane in 0..3usize {
            e.mem.set_u32(at + 4 * lane as u32, words_low_high.0[lane]);
            e.mem
                .set_u32(at + 0x10 + 4 * lane as u32, words_low_high.1[lane]);
            e.mem.set_u8(at + 0xc + lane as u32, extra.0[lane]);
            e.mem.set_u8(at + 0x1c + lane as u32, extra.1[lane]);
        }
        e.mem.set_u8(at + 0xf, shift);
    }

    /// A body without a parent whose cache holds `boxes` 32-byte entries (every word of entry
    /// `k` is `100 + k`), with the keys `0x10`, `0x20`, ... and the block words `1` and `2`.
    fn cached_body(e: &mut Engine, boxes: u32) -> u32 {
        let body = e.mem.alloc(0x60);
        let cache = e.mem.alloc(0x20 * boxes.max(1));
        let keys = e.mem.alloc(4 * boxes.max(1));
        for k in 0..boxes {
            for word in 0..8 {
                e.mem.set_u32(cache + 0x20 * k + 4 * word, 100 + k);
            }
            e.mem.set_u32(keys + 4 * k, 0x10 * (k + 1));
        }
        e.mem.set_u32(body + 0x44, cache);
        e.mem.set_u32(body + 0x20, 1);
        e.mem.set_u32(body + 0x30, 2);
        e.mem.set_u16(body + 0x40, boxes as u16);
        e.mem.set_u32(body + 0x48, keys);
        body
    }

    /// Registers the copy of one 32-byte box (`00cf8690`) as a double.
    fn copy_double(e: &mut Engine, log: &Log) {
        let notes = Rc::clone(log);
        e.register_double(COPY_AABB_INT, move |e, a| {
            notes.borrow_mut().push((COPY_AABB_INT, a.to_vec()));
            let bytes = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
    }

    /// Runs `00d351c0` for `body` with `flag` and the scene's input; returns the answer and
    /// the output block (0x80 bytes, prefilled with `0xaa`).
    fn extract(s: &mut Scene, body: u32, flag: u32, iter: u32) -> (u32, u32) {
        let out = s.e.mem.alloc(0x80);
        s.e.mem.write(out, &[0xaa; 0x80]);
        let count =
            s.e.call(0x00d3_51c0, &[body, flag, iter, s.input, out])
                .u32();
        (count, out)
    }

    #[test]
    fn the_cache_is_copied_with_its_keys_when_nothing_sweeps_00d351c0() {
        let mut s = scene(0, 0, Vec::new());
        let log = Rc::clone(&s.log);
        copy_double(&mut s.e, &log);
        let body = cached_body(&mut s.e, 3);
        let (count, out) = extract(&mut s, body, 0, 0x4000);
        assert_eq!(count, 3);
        for k in 0..3 {
            let word = 100 + k;
            let expected = [word, word, word, 0x10 * (k + 1), word, word, word, word];
            assert_eq!(int_words(&s.e, out + 0x20 * k), expected.to_vec());
        }
        assert_eq!(calls(&s.log, COPY_AABB_INT).len(), 3);
    }

    #[test]
    fn an_empty_cache_makes_no_boxes_00d351c0() {
        let mut s = scene(0, 0, Vec::new());
        let body = cached_body(&mut s.e, 0);
        let (count, out) = extract(&mut s, body, 0, 0x4000);
        assert_eq!(count, 0);
        assert_eq!(s.e.mem.u32(out), 0xaaaa_aaaa);
    }

    #[test]
    fn a_sweep_decodes_the_compact_cache_and_sorts_by_the_first_word_00d351c0() {
        let mut s = scene(0, 1, Vec::new());
        let body = cached_body(&mut s.e, 3);
        let cache = s.e.mem.u32(body + 0x44);
        for (k, first) in [300u32, 100, 200].into_iter().enumerate() {
            compact_box(
                &mut s.e,
                cache + 0x20 * k as u32,
                ([first, 50, 60], [400, 500, 600]),
                ([1, 2, 3], [4, 5, 6]),
                0,
            );
        }
        let (count, out) = extract(&mut s, body, 0, 0x4000);
        assert_eq!(count, 3);
        // Sorted by the decoded first word; the keys and the untouched last word travel along.
        let expected = [
            [99, 48, 57, 0x20, 404, 505, 606, 0xaaaa_aaaa],
            [199, 48, 57, 0x30, 404, 505, 606, 0xaaaa_aaaa],
            [299, 48, 57, 0x10, 404, 505, 606, 0xaaaa_aaaa],
        ];
        for (k, want) in expected.into_iter().enumerate() {
            assert_eq!(int_words(&s.e, out + 0x20 * k as u32), want.to_vec());
        }
    }

    #[test]
    fn the_shift_scales_the_extra_bytes_and_wraps_at_32_00d351c0() {
        for shift in [3u8, 35] {
            let mut s = scene(0, 1, Vec::new());
            let body = cached_body(&mut s.e, 1);
            let cache = s.e.mem.u32(body + 0x44);
            compact_box(
                &mut s.e,
                cache,
                ([50, 60, 70], [150, 160, 170]),
                ([1, 2, 3], [4, 5, 6]),
                shift,
            );
            let (count, out) = extract(&mut s, body, 0, 0x4000);
            assert_eq!(count, 1);
            assert_eq!(int_words(&s.e, out)[..7], [42, 44, 46, 0x10, 182, 200, 218]);
        }
    }

    /// Sets the single box of a body without a cache: a compact box (or, for `copy`, a
    /// pattern) in the body's block at `+0x20`.
    fn single_body(e: &mut Engine, first: u32, second: u32) -> u32 {
        let body = e.mem.alloc(0x60);
        for word in 0..8 {
            e.mem.set_u32(body + 0x20 + 4 * word, 7 + word);
        }
        e.mem.set_u32(body + 0x20, first);
        e.mem.set_u32(body + 0x30, second);
        body
    }

    #[test]
    fn a_single_box_without_a_cache_is_one_element_with_key_0_00d351c0() {
        // Copied when nothing sweeps.
        let mut s = scene(0, 0, Vec::new());
        let log = Rc::clone(&s.log);
        copy_double(&mut s.e, &log);
        let body = single_body(&mut s.e, 1, 9);
        let (count, out) = extract(&mut s, body, 1, 0x4000);
        assert_eq!(count, 1);
        assert_eq!(int_words(&s.e, out), vec![1, 8, 9, 0, 9, 12, 13, 14]);
        // Decoded when sweeping.
        let mut s = scene(0, 1, Vec::new());
        let body = single_body(&mut s.e, 1, 9);
        compact_box(
            &mut s.e,
            body + 0x20,
            ([1, 60, 70], [9, 160, 170]),
            ([1, 2, 3], [4, 5, 6]),
            0,
        );
        let (count, out) = extract(&mut s, body, 1, 0x4000);
        assert_eq!(count, 1);
        assert_eq!(int_words(&s.e, out)[..7], [0, 58, 67, 0, 13, 165, 176]);
    }

    /// The scene of a recalculation: the container is also the key iterator (slot 8 gives
    /// the first key, slot `0xC` the one after its argument, from `keys`) and the shape answers
    /// the boxes `answers`.
    fn recalc_scene(sweep: u32, keys: &[u32], answers: Vec<[Vec4; 2]>) -> Scene {
        let mut s = scene(0, sweep, answers);
        let log = Rc::clone(&s.log);
        let first = keys[0];
        log_double(&mut s.e, &log, CONTAINER + 8, first);
        let order = keys.to_vec();
        let notes = Rc::clone(&log);
        s.e.register_double(CONTAINER + 0xc, move |_, a| {
            notes.borrow_mut().push((CONTAINER + 0xc, a.to_vec()));
            let at = order.iter().position(|key| *key == a[1]).unwrap();
            Ret {
                eax: order.get(at + 1).copied().unwrap_or(0xffff_ffff),
                ..Ret::default()
            }
        });
        s
    }

    #[test]
    fn without_a_cache_the_boxes_are_made_again_and_sorted_00d351c0() {
        // First words 30, 10, 20 for the keys 5, 6, 7 ((0.5 + x) * 2).
        let answers = vec![
            [[14.5, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]],
            [[4.5, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]],
            [[9.5, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]],
        ];
        let mut s = recalc_scene(0, &[5, 6, 7], answers);
        // A body with a parent takes the recalculation straight away.
        let body = s.e.mem.alloc(0x60);
        s.e.mem.set_u32(body + 0xc, 0x1234);
        s.e.mem.set_u32(body + 8, 0x7777);
        s.e.mem.set_u32(s.input + 8, SIGNALLING_NAN);
        let container = s.container;
        let (count, out) = extract(&mut s, body, 1, container);
        assert_eq!(count, 3);
        let firsts: Vec<u32> = (0..3).map(|k| s.e.mem.u32(out + 0x20 * k)).collect();
        let keys: Vec<u32> = (0..3).map(|k| s.e.mem.u32(out + 0x20 * k + 0xc)).collect();
        assert_eq!(firsts, vec![10, 20, 30]);
        assert_eq!(keys, vec![6, 7, 5]);
        let boxes = calls(&s.log, SHAPE + 0x1c);
        assert_eq!(boxes[0][..3], [s.shape, 0x7777, 0x7fe0_0001]);
        assert_eq!(calls(&s.log, CONTAINER + 0x14).len(), 3);
    }

    #[test]
    fn a_first_key_of_minus_one_ends_the_recalculation_at_once_00d351c0() {
        let mut s = recalc_scene(0, &[0xffff_ffff], Vec::new());
        let body = s.e.mem.alloc(0x60);
        s.e.mem.set_u32(body + 0xc, 0x1234);
        let container = s.container;
        let (count, _) = extract(&mut s, body, 0, container);
        assert_eq!(count, 0);
        assert!(calls(&s.log, SHAPE + 0x1c).is_empty());
    }

    #[test]
    fn a_sweeping_recalculation_widens_the_boxes_before_the_sort_00d351c0() {
        // Two boxes of the sweep scene, the second 5 further along x.
        let answers = vec![
            [[4.0, -2.0, -3.0, 0.0], [8.0, 2.0, 1.0, 0.0]],
            [[-1.0, -2.0, -3.0, 0.0], [3.0, 2.0, 1.0, 0.0]],
        ];
        let mut s = recalc_scene(1, &[5, 6], answers);
        store_vec(&mut s.e, s.input + 0x20, [10.0; 4]);
        store_vec(&mut s.e, s.input + 0x30, [0.0; 4]);
        store_vec(&mut s.e, s.input + 0x40, [1.0; 4]);
        let (body, motion) = sweep_rig(&mut s, 0.0, &[]);
        let container = s.container;
        let (count, out) = extract(&mut s, body, 0, container);
        assert_eq!(count, 2);
        assert_eq!(int_words(&s.e, out), vec![9, 8, 7, 6, 4, 2, 1, 0]);
        assert_eq!(int_words(&s.e, out + 0x20), vec![14, 8, 7, 5, 9, 2, 1, 0]);
        let init = calls(&s.log, INIT_OFFSET_AABB_INPUT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][0], motion);
    }

    #[test]
    fn a_body_with_a_cache_that_is_not_ordered_is_recalculated_00d351c0() {
        // The block's first word is above the one at `+0x10`: no cache.
        let mut s = recalc_scene(0, &[5], vec![[[1.0; 4], [2.0; 4]]]);
        let body = cached_body(&mut s.e, 2);
        s.e.mem.set_u32(body + 0x20, 9);
        s.e.mem.set_u32(body + 0x30, 2);
        let container = s.container;
        let (count, out) = extract(&mut s, body, 1, container);
        assert_eq!(count, 1);
        assert_eq!(s.e.mem.u32(out + 0xc), 5);
        // Without the flag the single box of the body is not used either.
        let mut s = recalc_scene(0, &[5], vec![[[1.0; 4], [2.0; 4]]]);
        let body = single_body(&mut s.e, 1, 9);
        let container = s.container;
        let (count, out) = extract(&mut s, body, 0, container);
        assert_eq!(count, 1);
        assert_eq!(s.e.mem.u32(out + 0xc), 5);
    }
}
