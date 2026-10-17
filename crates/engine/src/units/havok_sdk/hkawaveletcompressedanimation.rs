//! `Havok SDK/hkawaveletcompressedanimation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaWaveletCompressedAnimation` (Xbox PDB name) keeps its data in blocks. The fields the
//! functions here read are described by offset (unconfirmed): the number of transform tracks
//! (`+0x10`) and of float tracks (`+0x14`), the block size in frames (`+0x2c`), two bytes
//! (`+0x30`, `+0x31`) that make up the word handed to the decompressor, the number of values in
//! a block (`+0x34`), offsets in the data of the minimums (`+0x38`), the ranges (`+0x3c`) and
//! the bit widths (`+0x40`), of the track words (`+0x44`), of the static values (`+0x48`) with
//! the counts of the transform and the float part (`+0x4c`, `+0x50`), of the block table
//! (`+0x54`) and of the quantised data (`+0x5c`), and the data pointer (`+0x64`).
//!
//! The x87 code computes in `f64` and rounds when a `float` is stored. The callees outside this
//! unit (`_decompress`, `hkaTrackAnalysis::calcDynamicStats` and `recompose`) are called by
//! address.
//!
//! Only `00e30200` of this unit is translated; the others are listed in the commit message.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;
use crate::units::havok_sdk::hkpbvagent::profile_start_record;
use crate::units::havok_sdk::hkpsphereshape::profile_record;

/// `_decompress` (`00e300b0`, usercall: `EAX` = destination, cdecl stack arguments).
const DECOMPRESS: u32 = 0x00e3_00b0;
/// `hkaTrackAnalysis::calcDynamicStats` (`00e4e610`) and `recompose` (`00e4f640`).
const CALC_DYNAMIC_STATS: u32 = 0x00e4_e610;
const RECOMPOSE: u32 = 0x00e4_f640;
/// The profiling labels of `samplePartialWithDataChunks`: `"LtSampleWaveChunk"`,
/// `"StStaticW"`, `"StDecompressWChunk"`, `"StRecomposeWChunk"` and the end label.
const LABEL_SAMPLE_CHUNK: u32 = 0x010e_b344;
const LABEL_STATIC: u32 = 0x010e_b338;
const LABEL_DECOMPRESS_CHUNK: u32 = 0x010e_b324;
const LABEL_RECOMPOSE_CHUNK: u32 = 0x010e_b310;
const LABEL_END: u32 = 0x0107_6f38;

/// Reserves `bytes` (rounded up to 128) from the memory router's bump pointer and returns the
/// start of the block.
fn reserve(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let start = e.mem.u32(router);
    e.mem.set_u32(
        router,
        start.wrapping_add(bytes.wrapping_add(0x7f) & 0xffff_ff80),
    );
    start
}

/// Sets the memory router's bump pointer back to `start`.
fn release(e: &mut Engine, start: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
}

/// The address of the float of row `row` at column `column` in a table of rows `block_size`
/// floats long.
fn cell(table: u32, row: u32, block_size: u32, column: u32) -> u32 {
    table.wrapping_add(
        row.wrapping_mul(block_size)
            .wrapping_add(column)
            .wrapping_mul(4),
    )
}

/// Decompresses one block of `count` rows into `table` (`_decompress`): the word, the three
/// offsets and the stream are read from the animation `anim` and the data base `data`.
fn decompress_block(e: &mut Engine, table: u32, anim: u32, data: u32, count: u32, stream: u32) {
    let block_size = e.mem.u32(anim + 0x2c);
    let word = (u32::from(e.mem.u8(anim + 0x30)) << 8) | u32::from(e.mem.u8(anim + 0x31));
    let minimums = e.mem.u32(anim + 0x38).wrapping_add(data);
    let ranges = e.mem.u32(anim + 0x3c).wrapping_add(data);
    let widths = e.mem.u32(anim + 0x40).wrapping_add(data);
    e.call(
        DECOMPRESS,
        &args![table, count, block_size, word, minimums, ranges, widths, stream],
    );
}

// Translated from 00e30200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaWaveletCompressedAnimation::samplePartialWithDataChunks` (Xbox PDB name; cdecl, plain
/// `RET`): samples the tracks from a block of data chunks as `getDataChunks` (`00e2fac0`)
/// laid them out. `chunks` holds the animation (`+0`, plus the byte at `+8`), the data base
/// (`+0xc`, plus the byte at `+0x14`) and the streams of the first (`+0x18`, byte `+0x20`) and
/// the second (`+0x24`, byte `+0x2c`) block; `chunk_count` is 3 or 4.
///
/// Between profile records the function counts the values to decode (the animation's own
/// count, or the dynamic count `calcDynamicStats` (`00e4e610`) finds when the requested track
/// counts differ), reserves two buffers from the memory router, decompresses the first block
/// and, for `frame % blockSize`, blends two neighbouring columns with weights `1 - t` and `t`
/// (three chunks). With four chunks the first block is scaled by `1 - t`, the second block is
/// decompressed and its first column, scaled by `t`, is added. The result goes through
/// `recompose` (`00e4f640`) and the router is reset.
pub fn sample_partial_with_data_chunks(
    e: &mut Engine,
    frame: u32,
    time: f32,
    transform_count: u32,
    transforms: u32,
    float_count: u32,
    floats: u32,
    chunks: u32,
    chunk_count: u32,
) {
    let anim = e
        .mem
        .u32(chunks)
        .wrapping_add(u32::from(e.mem.u8(chunks + 8)));
    profile_start_record(e, LABEL_SAMPLE_CHUNK, LABEL_STATIC);
    let data = e
        .mem
        .u32(chunks + 0xc)
        .wrapping_add(u32::from(e.mem.u8(chunks + 0x14)));
    let mut count = e.mem.u32(anim + 0x34);
    let static_values = e.mem.u32(anim + 0x48).wrapping_add(data);
    let anim_tracks = e.mem.u32(anim + 0x10);
    let words = e.mem.u32(anim + 0x44).wrapping_add(data);
    if transform_count != anim_tracks || float_count != e.mem.u32(anim + 0x14) {
        let float_words = words.wrapping_add(anim_tracks.wrapping_mul(2));
        let stats = e.mem.alloc(0x38);
        let sum = if float_count == 0 {
            e.call(
                CALC_DYNAMIC_STATS,
                &args![words, float_words, transform_count, 0u32, stats],
            );
            e.mem
                .u32(stats + 0x20)
                .wrapping_add(e.mem.u32(stats + 0x24))
        } else {
            e.call(
                CALC_DYNAMIC_STATS,
                &args![words, float_words, anim_tracks, float_count, stats],
            );
            e.mem
                .u32(stats + 0x24)
                .wrapping_add(e.mem.u32(stats + 0x28))
                .wrapping_add(e.mem.u32(stats + 0x20))
        };
        count = sum.wrapping_add(e.mem.u32(stats + 0x1c));
        e.mem.free(stats);
    }
    profile_record(e, LABEL_DECOMPRESS_CHUNK);
    let block_size = e.mem.u32(anim + 0x2c);
    let rest = (1.0 - f64::from(time)) as f32;
    let table = reserve(e, block_size.wrapping_mul(count).wrapping_mul(4));
    let result = reserve(e, count.wrapping_mul(4));
    let first_stream = u32::from(e.mem.u8(chunks + 0x20)).wrapping_add(e.mem.u32(chunks + 0x18));
    decompress_block(e, table, anim, data, count, first_stream);
    let block_size = e.mem.u32(anim + 0x2c);
    if block_size == 0 {
        panic!("integer divide fault at 0x00e30404");
    }
    let column = frame % block_size;
    if chunk_count == 3 {
        for row in 0..count {
            let low = f64::from(e.mem.f32(cell(table, row, block_size, column)));
            let high = f64::from(e.mem.f32(cell(table, row, block_size, column) + 4));
            let value = (high * f64::from(time) + low * f64::from(rest)) as f32;
            e.mem
                .set_f32(result.wrapping_add(row.wrapping_mul(4)), value);
        }
    } else {
        for row in 0..count {
            let low = f64::from(e.mem.f32(cell(table, row, block_size, column)));
            let value = (low * f64::from(rest)) as f32;
            e.mem
                .set_f32(result.wrapping_add(row.wrapping_mul(4)), value);
        }
        let second_stream =
            u32::from(e.mem.u8(chunks + 0x2c)).wrapping_add(e.mem.u32(chunks + 0x24));
        decompress_block(e, table, anim, data, count, second_stream);
        let block_size = e.mem.u32(anim + 0x2c);
        for row in 0..count {
            let high = f64::from(e.mem.f32(cell(table, row, block_size, 0)));
            let at = result.wrapping_add(row.wrapping_mul(4));
            let value = (high * f64::from(time) + f64::from(e.mem.f32(at))) as f32;
            e.mem.set_f32(at, value);
        }
    }
    profile_record(e, LABEL_RECOMPOSE_CHUNK);
    let analysis = e.mem.alloc(0x20);
    e.mem.set_u32(analysis, transform_count);
    e.mem.set_u32(analysis + 4, float_count);
    e.mem.set_u32(analysis + 8, words);
    let tracks = e.mem.u32(anim + 0x10);
    e.mem
        .set_u32(analysis + 0xc, words.wrapping_add(tracks.wrapping_mul(2)));
    e.mem.set_u32(analysis + 0x10, static_values);
    let static_count = e.mem.u32(anim + 0x4c);
    e.mem.set_u32(
        analysis + 0x14,
        static_values.wrapping_add(static_count.wrapping_mul(4)),
    );
    e.mem.set_u32(analysis + 0x18, result);
    let float_static = e.mem.u32(anim + 0x50);
    e.mem.set_u32(
        analysis + 0x1c,
        result.wrapping_add(float_static.wrapping_mul(4)),
    );
    e.call(RECOMPOSE, &args![analysis, transforms, floats]);
    e.mem.free(analysis);
    release(e, result);
    release(e, table);
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_0200,
        sample_partial_with_data_chunks(u32, f32, u32, u32, u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A router double whose bump pointer starts at a fresh block; returns the address of the
    /// pointer.
    fn install_router(e: &mut Engine) -> u32 {
        let router = e.mem.alloc(0x20);
        let base = e.mem.alloc(0x400);
        e.mem.set_u32(router, base);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        router
    }

    /// A profiling stream with room for 4 sixteen-byte records.
    fn install_stream(e: &mut Engine) -> u32 {
        let records = e.mem.alloc(0x40);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x40);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    /// An animation with block size 4, 2 transform tracks and 1 float track, data base `0x6000`
    /// (plus a unit offset of 3 in the chunk struct) and the given dynamic value count.
    fn setup(e: &mut Engine) -> (u32, u32) {
        let anim = e.mem.alloc(0x80);
        for (offset, value) in [
            (0x10, 2),
            (0x14, 1),
            (0x2c, 4),
            (0x34, 3),
            (0x38, 0x100),
            (0x3c, 0x200),
            (0x40, 0x300),
            (0x44, 0x400),
            (0x48, 0x500),
            (0x4c, 5),
            (0x50, 6),
        ] {
            e.mem.set_u32(anim + offset, value);
        }
        e.mem.set_u8(anim + 0x30, 0x12);
        e.mem.set_u8(anim + 0x31, 0x34);
        let chunks = e.mem.alloc(0x40);
        e.mem.set_u32(chunks, anim - 2);
        e.mem.set_u8(chunks + 8, 2);
        e.mem.set_u32(chunks + 0xc, 0x6000 - 3);
        e.mem.set_u8(chunks + 0x14, 3);
        e.mem.set_u32(chunks + 0x18, 0x7000 - 5);
        e.mem.set_u8(chunks + 0x20, 5);
        e.mem.set_u32(chunks + 0x24, 0x8000 - 7);
        e.mem.set_u8(chunks + 0x2c, 7);
        (anim, chunks)
    }

    /// Doubles for the callees: `_decompress` fills the table with `row * 10 + column + pass`
    /// where `pass` counts its calls, and `recompose` and the stats function only log.
    fn install_callees(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let passes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&passes);
        e.register_double(DECOMPRESS, move |e, a| {
            let pass = log.borrow().len() as f32;
            log.borrow_mut().push(a.to_vec());
            let (table, count, block_size) = (a[0], a[1], a[2]);
            for row in 0..count {
                for column in 0..block_size {
                    e.mem.set_f32(
                        table + 4 * (row * block_size + column),
                        (row * 10 + column) as f32 + 100.0 * pass,
                    );
                }
            }
            Ret::default()
        });
        e.register_double(CALC_DYNAMIC_STATS, |e, a| {
            for (offset, value) in [(0x1c, 1u32), (0x20, 2), (0x24, 3), (0x28, 4)] {
                e.mem.set_u32(a[4] + offset, value);
            }
            Ret::default()
        });
        e.register_double(RECOMPOSE, |_, _| Ret::default());
        passes
    }

    /// The words of the analysis block and the 6 result floats `recompose` was given.
    type Recomposed = Rc<RefCell<Vec<(Vec<u32>, Vec<f32>)>>>;

    fn record_recompose(e: &mut Engine) -> Recomposed {
        let seen: Recomposed = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(RECOMPOSE, move |e, a| {
            let words = (0..8).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            let result = e.mem.u32(a[0] + 0x18);
            let floats = (0..6).map(|i| e.mem.f32(result + 4 * i)).collect();
            sink.borrow_mut().push((words, floats));
            Ret::default()
        });
        seen
    }

    /// The labels of the records in a stream whose first record is 16 bytes long.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x40;
        let cursor = e.mem.u32(stream + 4);
        let mut labels = vec![e.mem.u32(first), e.mem.u32(first + 0xc)];
        let mut at = first + 0x10;
        while at < cursor {
            labels.push(e.mem.u32(at));
            at += 12;
        }
        labels
    }

    #[test]
    fn three_chunks_blend_two_neighbouring_columns() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        let stream = install_stream(&mut e);
        let passes = install_callees(&mut e);
        let seen = record_recompose(&mut e);
        let (_, chunks) = setup(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // Matching track counts keep the animation's value count (3). Column 6 % 4 = 2: row r
        // blends r * 10 + 2 and r * 10 + 3 with t = 0.25.
        e.call(
            0x00e3_0200,
            &args![6u32, 0.25f32, 2u32, transforms, 1u32, floats, chunks, 3u32],
        );
        assert!(calls(&e, CALC_DYNAMIC_STATS).is_empty());
        assert_eq!(
            *passes.borrow(),
            vec![vec![base, 3, 4, 0x1234, 0x6100, 0x6200, 0x6300, 0x7000]]
        );
        assert_eq!(e.mem.u32(router), base);
        let (words, result) = seen.borrow()[0].clone();
        assert_eq!(
            words,
            vec![
                2,
                1,
                0x6400,
                0x6404,
                0x6500,
                0x6514,
                base + 0x80,
                base + 0x98
            ]
        );
        assert_eq!(result[..3], [2.25, 12.25, 22.25]);
        assert_eq!(calls(&e, RECOMPOSE)[0][1..], [transforms, floats]);
        assert_eq!(
            labels(&e, stream),
            vec![
                LABEL_SAMPLE_CHUNK,
                LABEL_STATIC,
                LABEL_DECOMPRESS_CHUNK,
                LABEL_RECOMPOSE_CHUNK,
                LABEL_END
            ]
        );
    }

    #[test]
    fn four_chunks_add_the_second_block_and_other_counts_use_the_stats() {
        let mut e = engine();
        let router = install_router(&mut e);
        let base = e.mem.u32(router);
        install_stream(&mut e);
        let passes = install_callees(&mut e);
        let seen = record_recompose(&mut e);
        let (_, chunks) = setup(&mut e);
        let transforms = e.mem.alloc(0x20);
        let floats = e.mem.alloc(0x20);
        // Three transform tracks and no float tracks differ from the animation's 2 and 1: the
        // stats double gives 1 + 2 + 3 = 6 values. Column 5 % 4 = 1.
        e.call(
            0x00e3_0200,
            &args![5u32, 0.25f32, 3u32, transforms, 0u32, floats, chunks, 4u32],
        );
        assert_eq!(calls(&e, CALC_DYNAMIC_STATS)[0][1..4], [0x6400 + 4, 3, 0]);
        assert_eq!(passes.borrow().len(), 2);
        assert_eq!(passes.borrow()[0][1], 6);
        assert_eq!(passes.borrow()[0][7], 0x7000);
        assert_eq!(passes.borrow()[1][7], 0x8000);
        assert_eq!(e.mem.u32(router), base);
        let (_, result) = seen.borrow()[0].clone();
        for (row, got) in result.iter().enumerate() {
            let row = row as f32;
            let want = (row * 10.0 + 1.0) * 0.75 + (row * 10.0 + 100.0) * 0.25;
            assert_eq!(*got, want);
        }
    }

    #[test]
    #[should_panic(expected = "integer divide fault at 0x00e30404")]
    fn a_zero_block_size_faults() {
        let mut e = engine();
        install_router(&mut e);
        install_stream(&mut e);
        install_callees(&mut e);
        let (anim, chunks) = setup(&mut e);
        e.mem.set_u32(anim + 0x2c, 0);
        e.call(
            0x00e3_0200,
            &args![6u32, 0.25f32, 2u32, 0u32, 1u32, 0u32, chunks, 3u32],
        );
    }
}
