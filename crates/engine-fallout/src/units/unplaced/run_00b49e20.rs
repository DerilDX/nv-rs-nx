//! `(unplaced)/run_00b49e20` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00b49e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running checksum, a
/// buffer and its length). A null buffer answers 1. Otherwise the low and
/// high halves of the checksum sum the bytes and the running sums, in blocks
/// of at most 0x15b0 bytes (16 at a time while at least 16 remain, then one
/// by one), each block ending with both halves taken modulo 0xfff1. Answers
/// `high << 16 | low`.
pub fn fn_00b49e20(e: &mut Engine, checksum: u32, buffer: u32, length: u32) -> u32 {
    let mut low = checksum & 0xffff;
    let mut high = (checksum >> 16) & 0xffff;
    if buffer == 0 {
        return 1;
    }
    let mut at = buffer;
    let mut length = length;
    while length != 0 {
        let mut block = if length < 0x15b0 { length } else { 0x15b0 };
        length -= block;
        while (block as i32) >= 0x10 {
            for i in 0..0x10 {
                low = low.wrapping_add(u32::from(e.mem.u8(at + i)));
                high = high.wrapping_add(low);
            }
            at = at.wrapping_add(0x10);
            block -= 0x10;
        }
        while block != 0 {
            low = low.wrapping_add(u32::from(e.mem.u8(at)));
            at = at.wrapping_add(1);
            high = high.wrapping_add(low);
            block -= 1;
        }
        low %= 0xfff1;
        high %= 0xfff1;
    }
    (high << 16) | low
}

/// The CRC table `00b4a070`'s byte-at-a-time branch reads (256 words in
/// `.rdata`; the exe's words are the reflected CRC-32 table, `00000000`,
/// `77073096`, `ee0e612c`, ...).
const CRC_TABLE: u32 = 0x010a_6550;
/// The two word-at-a-time CRC helpers `00b4a070` chooses between (`cdecl`,
/// the same three words): `00b4a270` (the branch the exe takes) and
/// `00b4a670`.
const CRC_TAKEN_HELPER: u32 = 0x00b4_a270;
const CRC_OTHER_HELPER: u32 = 0x00b4_a670;
/// The three further 256-word tables that follow `CRC_TABLE` in `.rdata`,
/// which `00b4a270`'s word steps index with the CRC's bytes 2, 1 and 0
/// (`CRC_TABLE` takes byte 3).
const CRC_TABLE_1: u32 = 0x010a_6950;
const CRC_TABLE_2: u32 = 0x010a_6d50;
const CRC_TABLE_3: u32 = 0x010a_7150;
/// The four 256-word tables after those, which `00b4a670` reads (its byte
/// steps the first; its word steps the four, with the CRC's bytes 0 to 3).
/// The words sampled from the exe are those of `CRC_TABLE` and its three
/// followers with their bytes reversed.
const CRC_SWAPPED_TABLE_0: u32 = 0x010a_7550;
const CRC_SWAPPED_TABLE_1: u32 = 0x010a_7950;
const CRC_SWAPPED_TABLE_2: u32 = 0x010a_7d50;
const CRC_SWAPPED_TABLE_3: u32 = 0x010a_8150;

/// The messages `00b49070` leaves in the stream's message word (`+0x18`):
/// "invalid distance too far back", "invalid distance code" and
/// "invalid literal/length code" (`.rdata`).
const MESSAGE_DISTANCE_TOO_FAR: u32 = 0x011a_d71c;
const MESSAGE_DISTANCE_CODE: u32 = 0x011a_d73c;
const MESSAGE_LITERAL_LENGTH_CODE: u32 = 0x011a_d754;

/// The 16-bit base and extra-bit tables `00b497d0` reads for its kinds 1 and
/// 2 (`.rdata`): kind 1 indexes the first two from symbol 257 (the exe
/// subtracts 0x202 bytes from each address), kind 2 the last two from 0.
const LENGTH_BASE: u32 = 0x010a_6450;
const LENGTH_EXTRA: u32 = 0x010a_6490;
const DISTANCE_BASE: u32 = 0x010a_64d0;
const DISTANCE_EXTRA: u32 = 0x010a_6510;

/// The address of the 16-bit element `index` of the array at `base`.
fn half_word_at(base: u32, index: u32) -> u32 {
    base.wrapping_add(index.wrapping_mul(2))
}

/// A decoding-table entry as the exe stores it: the operation byte, the bit
/// count byte and the 16-bit value.
fn table_entry(operation: u8, bits: u8, value: u16) -> u32 {
    u32::from(operation) | (u32::from(bits) << 8) | (u32::from(value) << 16)
}

/// Reads the next input byte into the bit buffer (the input pointer is
/// moved first, then the byte at it is added shifted by the bit count).
fn fast_pull_byte(e: &mut Engine, input: &mut u32, hold: &mut u32, bits: &mut u32) {
    *input = input.wrapping_add(1);
    let byte = u32::from(e.mem.u8(*input));
    *hold = byte.wrapping_shl(*bits).wrapping_add(*hold);
    *bits = bits.wrapping_add(8);
}

/// One byte copied in `00b49070`'s copy loops: both pointers move first,
/// then the byte at the source goes to the output.
fn fast_copy_byte(e: &mut Engine, from: &mut u32, out: &mut u32) {
    *from = from.wrapping_add(1);
    *out = out.wrapping_add(1);
    let byte = e.mem.u8(*from);
    e.mem.set_u8(*out, byte);
}

/// Copies `count` bytes (at least one: the exe's loop tests after copying).
fn fast_copy_run(e: &mut Engine, from: &mut u32, out: &mut u32, count: u32) {
    let mut count = count;
    loop {
        fast_copy_byte(e, from, out);
        count = count.wrapping_sub(1);
        if count == 0 {
            break;
        }
    }
}

// Translated from 00b49070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `zLib/inflate.c`; `cdecl`, two words:
/// the stream record and the output position the caller started at). The
/// body is a fast Huffman decoding loop over the stream's buffers, the shape
/// of zlib's `inflate_fast` (the identity is by the body). From the stream
/// it reads the input pointer and count (`+0x0`, `+0x4`) and the output
/// pointer and count (`+0xc`, `+0x10`); from the state (`+0x1c`) the window
/// size, fill, write position and buffer (`+0x20` .. `+0x2c`), the bit
/// buffer and its count (`+0x30`, `+0x34`), the literal/length and distance
/// tables (`+0x44`, `+0x48`) and their index bits (`+0x4c`, `+0x50`).
///
/// While at least 6 input bytes and 258 output bytes remain it tops the bit
/// buffer up two bytes at a time below 15 bits, looks the code up and drops
/// its bits; an operation of 0 is a literal byte; bit 0x10 is a length (base
/// plus the operation's low four bits of extra bits) followed by a distance
/// code from the distance table, then a copy from the output or, beyond what
/// was output, from the window (wrapping at its write position); a
/// distance beyond the window's fill sets the stream's message to "invalid
/// distance too far back" and the mode (`state +0x0`) to 0x1b. Without bit
/// 0x40 an operation names a second-level entry (its value plus the next
/// operation-many bits); with it a length code ends the block (bit 0x20:
/// mode 0xb) or is invalid (message, mode 0x1b), and a distance code is
/// invalid (message, mode 0x1b). On leaving, the whole unused bytes go
/// back to the input, the pointers and counts are written back and the bit
/// buffer keeps only its remaining bits.
pub fn fn_00b49070(e: &mut Engine, stream: u32, start: u32) {
    let state = e.mem.u32(stream + 0x1c);
    let mut input = e.mem.u32(stream).wrapping_sub(1);
    let last = input.wrapping_add(e.mem.u32(stream + 0x4)).wrapping_sub(5);
    let mut out = e.mem.u32(stream + 0xc).wrapping_sub(1);
    let begin = out.wrapping_sub(start.wrapping_sub(e.mem.u32(stream + 0x10)));
    let end = out
        .wrapping_add(e.mem.u32(stream + 0x10))
        .wrapping_sub(0x101);
    let window_size = e.mem.u32(state + 0x20);
    let window_have = e.mem.u32(state + 0x24);
    let window_write = e.mem.u32(state + 0x28);
    let window = e.mem.u32(state + 0x2c);
    let mut hold = e.mem.u32(state + 0x30);
    let mut bits = e.mem.u32(state + 0x34);
    let length_codes = e.mem.u32(state + 0x44);
    let distance_codes = e.mem.u32(state + 0x48);
    let length_mask = 1u32.wrapping_shl(e.mem.u32(state + 0x4c)).wrapping_sub(1);
    let distance_mask = 1u32.wrapping_shl(e.mem.u32(state + 0x50)).wrapping_sub(1);
    'decode: loop {
        if bits < 0xf {
            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
        }
        let mut code = e
            .mem
            .u32(length_codes.wrapping_add((hold & length_mask).wrapping_mul(4)));
        // The literal/length code (the exe's 00b49193).
        loop {
            let drop = (code >> 8) & 0xff;
            hold = hold.wrapping_shr(drop);
            bits = bits.wrapping_sub(drop);
            let operation = code & 0xff;
            if operation == 0 {
                out = out.wrapping_add(1);
                e.mem.set_u8(out, (code >> 16) as u8);
                break;
            }
            if operation & 0x10 != 0 {
                let mut length = code >> 16;
                let extra = operation & 0xf;
                if extra != 0 {
                    if bits < extra {
                        fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                    }
                    length = (1u32.wrapping_shl(extra).wrapping_sub(1) & hold).wrapping_add(length);
                    hold = hold.wrapping_shr(extra);
                    bits = bits.wrapping_sub(extra);
                }
                if bits < 0xf {
                    fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                    fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                }
                code = e
                    .mem
                    .u32(distance_codes.wrapping_add((hold & distance_mask).wrapping_mul(4)));
                // The distance code (the exe's 00b4929f).
                loop {
                    let drop = (code >> 8) & 0xff;
                    hold = hold.wrapping_shr(drop);
                    bits = bits.wrapping_sub(drop);
                    let operation = code & 0xff;
                    if operation & 0x10 != 0 {
                        let mut distance = code >> 16;
                        let extra = operation & 0xf;
                        if bits < extra {
                            fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                            if bits < extra {
                                fast_pull_byte(e, &mut input, &mut hold, &mut bits);
                            }
                        }
                        distance = (1u32.wrapping_shl(extra).wrapping_sub(1) & hold)
                            .wrapping_add(distance);
                        hold = hold.wrapping_shr(extra);
                        bits = bits.wrapping_sub(extra);
                        let mut copy = out.wrapping_sub(begin);
                        if distance > copy {
                            copy = distance.wrapping_sub(copy);
                            if copy > window_have {
                                e.mem.set_u32(stream + 0x18, MESSAGE_DISTANCE_TOO_FAR);
                                e.mem.set_u32(state, 0x1b);
                                break 'decode;
                            }
                            let mut from = window.wrapping_sub(1);
                            if window_write == 0 {
                                from = window_size.wrapping_sub(copy).wrapping_add(from);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = out.wrapping_sub(distance);
                                }
                            } else if window_write < copy {
                                from = window_size
                                    .wrapping_add(window_write)
                                    .wrapping_sub(copy)
                                    .wrapping_add(from);
                                copy = copy.wrapping_sub(window_write);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = window.wrapping_sub(1);
                                    if window_write < length {
                                        copy = window_write;
                                        length = length.wrapping_sub(copy);
                                        fast_copy_run(e, &mut from, &mut out, copy);
                                        from = out.wrapping_sub(distance);
                                    }
                                }
                            } else {
                                from = window_write.wrapping_sub(copy).wrapping_add(from);
                                if copy < length {
                                    length = length.wrapping_sub(copy);
                                    fast_copy_run(e, &mut from, &mut out, copy);
                                    from = out.wrapping_sub(distance);
                                }
                            }
                            while length > 2 {
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                length = length.wrapping_sub(3);
                            }
                            if length != 0 {
                                fast_copy_byte(e, &mut from, &mut out);
                                if length > 1 {
                                    fast_copy_byte(e, &mut from, &mut out);
                                }
                            }
                        } else {
                            // Copy from the output itself (three bytes at
                            // least: the exe's loop tests after copying).
                            let mut from = out.wrapping_sub(distance);
                            loop {
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                fast_copy_byte(e, &mut from, &mut out);
                                length = length.wrapping_sub(3);
                                if length <= 2 {
                                    break;
                                }
                            }
                            if length != 0 {
                                fast_copy_byte(e, &mut from, &mut out);
                                if length > 1 {
                                    fast_copy_byte(e, &mut from, &mut out);
                                }
                            }
                        }
                        break;
                    }
                    if operation & 0x40 == 0 {
                        let index = (1u32.wrapping_shl(operation).wrapping_sub(1) & hold)
                            .wrapping_add(code >> 16);
                        code = e
                            .mem
                            .u32(distance_codes.wrapping_add(index.wrapping_mul(4)));
                        continue;
                    }
                    e.mem.set_u32(stream + 0x18, MESSAGE_DISTANCE_CODE);
                    e.mem.set_u32(state, 0x1b);
                    break 'decode;
                }
                break;
            }
            if operation & 0x40 == 0 {
                let index =
                    (1u32.wrapping_shl(operation).wrapping_sub(1) & hold).wrapping_add(code >> 16);
                code = e.mem.u32(length_codes.wrapping_add(index.wrapping_mul(4)));
                continue;
            }
            if operation & 0x20 != 0 {
                e.mem.set_u32(state, 0xb);
            } else {
                e.mem.set_u32(stream + 0x18, MESSAGE_LITERAL_LENGTH_CODE);
                e.mem.set_u32(state, 0x1b);
            }
            break 'decode;
        }
        // The exe's 00b496f8: go on while input and output remain.
        if input >= last || out >= end {
            break;
        }
    }
    // The exe's 00b4970c: give back the unused whole bytes.
    let unused = bits >> 3;
    input = input.wrapping_sub(unused);
    bits = bits.wrapping_sub(unused << 3);
    hold &= 1u32.wrapping_shl(bits).wrapping_sub(1);
    e.mem.set_u32(stream, input.wrapping_add(1));
    e.mem.set_u32(stream + 0xc, out.wrapping_add(1));
    let input_left = if input < last {
        last.wrapping_sub(input).wrapping_add(5)
    } else {
        5u32.wrapping_sub(input.wrapping_sub(last))
    };
    e.mem.set_u32(stream + 0x4, input_left);
    let output_left = if out < end {
        end.wrapping_sub(out).wrapping_add(0x101)
    } else {
        0x101u32.wrapping_sub(out.wrapping_sub(end))
    };
    e.mem.set_u32(stream + 0x10, output_left);
    e.mem.set_u32(state + 0x30, hold);
    e.mem.set_u32(state + 0x34, bits);
}

// Translated from 00b497d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `zLib/inflate.c`; `cdecl`, six words: a
/// kind, an array of 16-bit code lengths, their count, the address of the
/// table pointer, the address of the root bit count and a 16-bit work
/// array). The body builds a canonical Huffman decoding table, the shape of
/// zlib's `inflate_table` (the identity is by the body). It counts the
/// lengths, clamps the root bits to the longest and shortest lengths used,
/// and answers -1 when no length is used, when the lengths over-subscribe,
/// or when they are incomplete unless the kind is not 0 and exactly one
/// code is used. It sorts the symbols by length into the work array and
/// fills the table: kind 0 entries carry the symbol itself (operation 0)
/// below 19, 19 as operation 0x60 and above it the work array's own words
/// as base and extra, kind 1 literals below 256 (operation 0), 256 as operation
/// 0x60 and the rest from the length tables, any other kind every symbol
/// from the distance tables. Codes longer than the root go to second-level
/// tables after the root table, linked from it (operation = the sub-table's
/// bits, bits = root, value = its offset in entries). Kind 1 answers 1 when
/// the entries used reach 0x506. An incomplete table is padded with
/// invalid entries (operation 0x40). On success the table pointer moves
/// past the entries used, the root bit count is written back and it
/// answers 0.
// Translation incomplete: 00b497d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00b497d0(
    e: &mut Engine,
    kind: u32,
    lens: u32,
    codes: u32,
    table: u32,
    bits: u32,
    work: u32,
) -> i32 {
    // The exe keeps its 16 length counts at EBP-0x24 and its 16 offsets at
    // EBP-0x78 of a 0x94-byte frame; a length index beyond 15 reaches the
    // rest of that memory, so both arrays live in a block laid out like it.
    e.with_stack(0x94, |e, frame| {
        let count = frame.addr() + 0x70;
        let offsets = frame.addr() + 0x1c;
        let mut index = 0u32;
        while index <= 0xf {
            e.mem.set_u16(half_word_at(count, index), 0);
            index += 1;
        }
        let mut symbol = 0u32;
        while symbol < codes {
            let length = u32::from(e.mem.u16(half_word_at(lens, symbol)));
            let counted = e.mem.u16(half_word_at(count, length)).wrapping_add(1);
            e.mem.set_u16(half_word_at(count, length), counted);
            symbol += 1;
        }
        let mut root = e.mem.u32(bits);
        let mut max = 0xfu32;
        while max >= 1 {
            if e.mem.u16(half_word_at(count, max)) != 0 {
                break;
            }
            max -= 1;
        }
        if root > max {
            root = max;
        }
        if max == 0 {
            return -1;
        }
        let mut min = 1u32;
        while min <= 0xf {
            if e.mem.u16(half_word_at(count, min)) != 0 {
                break;
            }
            min += 1;
        }
        if root < min {
            root = min;
        }
        let mut left = 1i32;
        let mut length = 1u32;
        while length <= 0xf {
            left <<= 1;
            left = left.wrapping_sub(i32::from(e.mem.u16(half_word_at(count, length))));
            if left < 0 {
                return -1;
            }
            length += 1;
        }
        if left > 0
            && (kind == 0 || codes.wrapping_sub(u32::from(e.mem.u16(half_word_at(count, 0)))) != 1)
        {
            return -1;
        }
        e.mem.set_u16(half_word_at(offsets, 1), 0);
        let mut length = 1u32;
        while length < 0xf {
            let next = e
                .mem
                .u16(half_word_at(offsets, length))
                .wrapping_add(e.mem.u16(half_word_at(count, length)));
            e.mem.set_u16(half_word_at(offsets, length + 1), next);
            length += 1;
        }
        let mut symbol = 0u32;
        while symbol < codes {
            let length = u32::from(e.mem.u16(half_word_at(lens, symbol)));
            if length != 0 {
                let slot = u32::from(e.mem.u16(half_word_at(offsets, length)));
                e.mem.set_u16(half_word_at(work, slot), symbol as u16);
                let moved = e.mem.u16(half_word_at(offsets, length)).wrapping_add(1);
                e.mem.set_u16(half_word_at(offsets, length), moved);
            }
            symbol += 1;
        }
        let (base, extra, end): (u32, u32, i32) = match kind {
            0 => (work, work, 0x13),
            1 => (LENGTH_BASE - 0x202, LENGTH_EXTRA - 0x202, 0x100),
            _ => (DISTANCE_BASE, DISTANCE_EXTRA, -1),
        };
        let mut huff = 0u32;
        let mut symbol = 0u32;
        let mut length = min;
        let mut next = e.mem.u32(table);
        let mut current = root;
        let mut drop = 0u32;
        let mut low = u32::MAX;
        let mut used = 1u32.wrapping_shl(root);
        let mask = used.wrapping_sub(1);
        if kind == 1 && used >= 0x506 {
            return 1;
        }
        // The exe's 00b49ac4: one entry (replicated) per symbol.
        loop {
            let entry_bits = length.wrapping_sub(drop) as u8;
            let sorted = e.mem.u16(half_word_at(work, symbol));
            let (operation, value) = if i32::from(sorted) < end {
                (0u8, sorted)
            } else if i32::from(sorted) > end {
                (
                    e.mem.u8(half_word_at(extra, u32::from(sorted))),
                    e.mem.u16(half_word_at(base, u32::from(sorted))),
                )
            } else {
                (0x60u8, 0u16)
            };
            let entry = table_entry(operation, entry_bits, value);
            let increment = 1u32.wrapping_shl(length.wrapping_sub(drop));
            let mut fill = 1u32.wrapping_shl(current);
            loop {
                fill = fill.wrapping_sub(increment);
                let at = huff.wrapping_shr(drop).wrapping_add(fill);
                e.mem.set_u32(next.wrapping_add(at.wrapping_mul(4)), entry);
                if fill == 0 {
                    break;
                }
            }
            let mut increment = 1u32.wrapping_shl(length.wrapping_sub(1));
            while huff & increment != 0 {
                increment >>= 1;
            }
            if increment != 0 {
                huff = (increment.wrapping_sub(1) & huff).wrapping_add(increment);
            } else {
                huff = 0;
            }
            symbol = symbol.wrapping_add(1);
            let left_of_length = e.mem.u16(half_word_at(count, length)).wrapping_sub(1);
            e.mem.set_u16(half_word_at(count, length), left_of_length);
            if left_of_length == 0 {
                if length == max {
                    break;
                }
                let sorted = u32::from(e.mem.u16(half_word_at(work, symbol)));
                length = u32::from(e.mem.u16(half_word_at(lens, sorted)));
            }
            if length > root && (huff & mask) != low {
                if drop == 0 {
                    drop = root;
                }
                next = next.wrapping_add(1u32.wrapping_shl(current).wrapping_mul(4));
                current = length.wrapping_sub(drop);
                let mut left = 1i32.wrapping_shl(current);
                while current.wrapping_add(drop) < max {
                    let counted = e.mem.u16(half_word_at(count, current.wrapping_add(drop)));
                    left = left.wrapping_sub(i32::from(counted));
                    if left <= 0 {
                        break;
                    }
                    current = current.wrapping_add(1);
                    left = left.wrapping_shl(1);
                }
                used = used.wrapping_add(1u32.wrapping_shl(current));
                if kind == 1 && used >= 0x506 {
                    return 1;
                }
                low = huff & mask;
                let root_table = e.mem.u32(table);
                e.mem
                    .set_u8(root_table.wrapping_add(low.wrapping_mul(4)), current as u8);
                let root_table = e.mem.u32(table);
                e.mem.set_u8(
                    root_table.wrapping_add(low.wrapping_mul(4)).wrapping_add(1),
                    root as u8,
                );
                let offset = (next.wrapping_sub(e.mem.u32(table)) as i32) >> 2;
                let root_table = e.mem.u32(table);
                e.mem.set_u16(
                    root_table.wrapping_add(low.wrapping_mul(4)).wrapping_add(2),
                    offset as u16,
                );
            }
        }
        // The exe's 00b49d2e: pad an incomplete code with invalid entries.
        let mut entry_bits = length.wrapping_sub(drop) as u8;
        while huff != 0 {
            if drop != 0 && (huff & mask) != low {
                // Back to the root table (the exe also sets its current
                // sub-table bits to the root here; nothing reads them after).
                drop = 0;
                length = root;
                next = e.mem.u32(table);
                entry_bits = length as u8;
            }
            let entry = table_entry(0x40, entry_bits, 0);
            let at = huff.wrapping_shr(drop);
            e.mem.set_u32(next.wrapping_add(at.wrapping_mul(4)), entry);
            let mut increment = 1u32.wrapping_shl(length.wrapping_sub(1));
            while huff & increment != 0 {
                increment >>= 1;
            }
            if increment != 0 {
                huff = (increment.wrapping_sub(1) & huff).wrapping_add(increment);
            } else {
                huff = 0;
            }
        }
        let moved = e.mem.u32(table).wrapping_add(used.wrapping_mul(4));
        e.mem.set_u32(table, moved);
        e.mem.set_u32(bits, root);
        0
    })
}

/// One step of `00b4a070`'s byte-at-a-time branch: the CRC shifted down a
/// byte, xor the table word the byte and the CRC's low byte select.
fn crc_table_step(e: &mut Engine, crc: u32, at: u32) -> u32 {
    let index = (u32::from(e.mem.u8(at)) ^ crc) & 0xff;
    (crc >> 8) ^ e.mem.u32(CRC_TABLE.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00b4a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length): a CRC-32 (the identity is by the table its other branch
/// reads, the exe's reflected CRC-32 table at `010a6550`). A null buffer
/// answers 0. The exe then tests the constant 1 and the low byte of a stack
/// word it set to 1 (a byte-order probe), both always true, so it answers
/// `00b4a270(crc, buffer, length)`; the probe's other answer calls
/// `00b4a670` with the same words, and the constant's other answer runs the
/// CRC here a byte at a time (complemented before and after, eight bytes per
/// round while at least eight remain, then one by one).
pub fn fn_00b4a070(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    if buffer == 0 {
        return 0;
    }
    // MOV EAX,1; TEST EAX,EAX: the exe tests a constant (always taken).
    let constant = 1u32;
    if constant != 0 {
        // The word 1 stored at EBP-0x4 and its first (low) byte read back.
        let probe = 1u32;
        if probe.to_le_bytes()[0] != 0 {
            return e.call(CRC_TAKEN_HELPER, &args![crc, buffer, length]).u32();
        }
        return e.call(CRC_OTHER_HELPER, &args![crc, buffer, length]).u32();
    }
    // Not reached (the constant above): the byte-at-a-time CRC.
    let mut crc = crc ^ 0xffff_ffff;
    let mut at = buffer;
    let mut length = length;
    while length >= 8 {
        for _ in 0..8 {
            crc = crc_table_step(e, crc, at);
            at = at.wrapping_add(1);
        }
        length -= 8;
    }
    while length != 0 {
        crc = crc_table_step(e, crc, at);
        at = at.wrapping_add(1);
        length -= 1;
    }
    crc ^ 0xffff_ffff
}

/// The word `index` of the 256-word table at `table`.
fn crc_word(e: &mut Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// One word step of `00b4a270` (after the word is xored in): the CRC's
/// bytes 0, 1, 2 and 3 index `CRC_TABLE_3`, `_2`, `_1` and `CRC_TABLE`, the
/// four words xored in the exe's order.
fn crc_word_step(e: &mut Engine, crc: u32) -> u32 {
    let mut next = crc_word(e, CRC_TABLE_3, crc & 0xff);
    next ^= crc_word(e, CRC_TABLE_2, (crc >> 8) & 0xff);
    next ^= crc_word(e, CRC_TABLE_1, (crc >> 0x10) & 0xff);
    next ^ crc_word(e, CRC_TABLE, crc >> 0x18)
}

// Translated from 00b4a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length; the branch `00b4a070` takes). The CRC is complemented,
/// then the bytes before the first 4-byte-aligned address go through one at
/// a time (`CRC_TABLE`, as `00b4a070`'s own byte branch), then words read
/// little-endian are xored in and stepped through the four tables, eight
/// per round while at least 32 bytes remain, then one per round while at
/// least four remain, then the last bytes one at a time; answers the CRC
/// complemented again.
pub fn fn_00b4a270(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    let mut crc = !crc;
    let mut buffer = buffer;
    let mut length = length;
    while length != 0 && buffer & 3 != 0 {
        crc = crc_table_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    let mut at = buffer;
    while length >= 0x20 {
        for _ in 0..8 {
            crc ^= e.mem.u32(at);
            at = at.wrapping_add(4);
            crc = crc_word_step(e, crc);
        }
        length -= 0x20;
    }
    while length >= 4 {
        crc ^= e.mem.u32(at);
        at = at.wrapping_add(4);
        crc = crc_word_step(e, crc);
        length -= 4;
    }
    let mut buffer = at;
    while length != 0 {
        crc = crc_table_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    !crc
}

/// The word with its four bytes reversed, as `00b4a670` builds it: the four
/// bytes shifted into place and added.
fn crc_bytes_reversed(value: u32) -> u32 {
    (value >> 0x18) + ((value >> 8) & 0xff00) + ((value & 0xff00) << 8) + ((value & 0xff) << 0x18)
}

/// One byte step of `00b4a670`: the CRC shifted up a byte, xor the word of
/// `CRC_SWAPPED_TABLE_0` the CRC's top byte xor the buffer's byte selects.
fn crc_swapped_byte_step(e: &mut Engine, crc: u32, at: u32) -> u32 {
    let index = (crc >> 0x18) ^ u32::from(e.mem.u8(at));
    (crc << 8) ^ crc_word(e, CRC_SWAPPED_TABLE_0, index)
}

/// One word step of `00b4a670` (after the word is xored in): the CRC's
/// bytes 0, 1, 2 and 3 index `CRC_SWAPPED_TABLE_0` to `_3`, the four words
/// xored in the exe's order.
fn crc_swapped_word_step(e: &mut Engine, crc: u32) -> u32 {
    let mut next = crc_word(e, CRC_SWAPPED_TABLE_0, crc & 0xff);
    next ^= crc_word(e, CRC_SWAPPED_TABLE_1, (crc >> 8) & 0xff);
    next ^= crc_word(e, CRC_SWAPPED_TABLE_2, (crc >> 0x10) & 0xff);
    next ^ crc_word(e, CRC_SWAPPED_TABLE_3, crc >> 0x18)
}

// Translated from 00b4a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words: a running CRC, a buffer
/// and its length; the branch `00b4a070` does not take). It works on the
/// CRC with its bytes reversed, complemented: the bytes before the first
/// 4-byte-aligned address one at a time (`CRC_SWAPPED_TABLE_0`), then words
/// (read little-endian, as every load on this CPU) xored in and stepped
/// through the four swapped tables, eight per round while at least 32 bytes
/// remain, then one per round while at least four remain, then the last
/// bytes one at a time; answers the result complemented with its bytes
/// reversed back. Its word pointer starts four bytes before the aligned
/// address and moves before each read, as the exe's does.
pub fn fn_00b4a670(e: &mut Engine, crc: u32, buffer: u32, length: u32) -> u32 {
    let mut crc = !crc_bytes_reversed(crc);
    let mut buffer = buffer;
    let mut length = length;
    while length != 0 && buffer & 3 != 0 {
        crc = crc_swapped_byte_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    let mut at = buffer.wrapping_sub(4);
    while length >= 0x20 {
        for _ in 0..8 {
            at = at.wrapping_add(4);
            crc ^= e.mem.u32(at);
            crc = crc_swapped_word_step(e, crc);
        }
        length -= 0x20;
    }
    while length >= 4 {
        at = at.wrapping_add(4);
        crc ^= e.mem.u32(at);
        crc = crc_swapped_word_step(e, crc);
        length -= 4;
    }
    let mut buffer = at.wrapping_add(4);
    while length != 0 {
        crc = crc_swapped_byte_step(e, crc, buffer);
        buffer = buffer.wrapping_add(1);
        length -= 1;
    }
    crc_bytes_reversed(!crc)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_9e20, fn_00b49e20(u32, u32, u32) -> u32),
        entry!(0x00b4_9070, fn_00b49070(u32, u32)),
        entry!(0x00b4_97d0, fn_00b497d0(u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00b4_a070, fn_00b4a070(u32, u32, u32) -> u32),
        entry!(0x00b4_a270, fn_00b4a270(u32, u32, u32) -> u32),
        entry!(0x00b4_a670, fn_00b4a670(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The same sums written out directly, for the check.
    fn expected(text: &[u8]) -> u32 {
        let (mut a, mut b) = (1u32, 0u32);
        for &c in text {
            a = (a + u32::from(c)) % 0xfff1;
            b = (b + a) % 0xfff1;
        }
        (b << 16) | a
    }

    #[test]
    fn fn_00b49e20_sums_short_and_long_buffers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00b4_9e20, &args![5u32, 0u32, 3u32]).u32(), 1);
        let text = b"Wikipedia";
        let at = e.mem.alloc(16);
        e.mem.write(at, text);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, at, 9u32]).u32(),
            0x11e6_0398
        );
        let long: Vec<u8> = (0..0x2000u32).map(|i| (i * 7 + 3) as u8).collect();
        let big = e.mem.alloc(0x2000);
        e.mem.write(big, &long);
        assert_eq!(
            e.call(0x00b4_9e20, &args![1u32, big, 0x2000u32]).u32(),
            expected(&long)
        );
    }

    /// Packs (value, bit count) fields into `len` bytes, least significant
    /// bit first.
    fn pack_bits(fields: &[(u32, u32)], len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        let mut at = 0usize;
        for &(value, count) in fields {
            for bit in 0..count {
                if (value >> bit) & 1 != 0 {
                    bytes[at / 8] |= 1 << (at % 8);
                }
                at += 1;
            }
        }
        bytes
    }

    /// Literal/length entries (two index bits): 0 a length of 3 plus two
    /// extra bits, 1 the block's end, 2 the literal 'A', 3 a link to 4 and 5
    /// (one more bit): the literal 'B' and an invalid code.
    const LENGTH_CODES: [u32; 6] = [
        0x0003_0212,
        0x0000_0260,
        0x0041_0200,
        0x0004_0201,
        0x0042_0100,
        0x0000_0140,
    ];
    /// Distance entries (two index bits): 0 a distance of 1 plus three extra
    /// bits, 1 an invalid code, 2 a link to 4 and 5 (one more bit): a
    /// distance of 2 and an invalid code.
    const DISTANCE_CODES: [u32; 6] = [
        0x0001_0213,
        0x0000_0240,
        0x0004_0201,
        0,
        0x0002_0110,
        0x0000_0140,
    ];

    /// What a run of 00b49070 left: the stream, the state, the input and the
    /// output buffers.
    struct FastRun {
        stream: u32,
        state: u32,
        input: u32,
        output: u32,
    }

    /// Runs 00b49070 over `input` (`available` bytes counted) with the
    /// given tables (two index bits each), the window "01234567" (eight
    /// bytes, `have` of them filled, write position `write`) and 300 bytes
    /// of output; the mode starts at 7.
    fn fast_run(
        e: &mut Engine,
        tables: (&[u32], &[u32]),
        input: &[u8],
        available: u32,
        have: u32,
        write: u32,
    ) -> FastRun {
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x60);
        let input_at = e.mem.alloc(64);
        e.mem.write(input_at, input);
        let output = e.mem.alloc(300);
        let window = e.mem.alloc(8);
        e.mem.write(window, b"01234567");
        let mut codes = [0u32; 2];
        for (slot, words) in [tables.0, tables.1].iter().enumerate() {
            let at = e.mem.alloc(4 * words.len() as u32);
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, *word);
            }
            codes[slot] = at;
        }
        e.mem.set_u32(stream, input_at);
        e.mem.set_u32(stream + 0x4, available);
        e.mem.set_u32(stream + 0xc, output);
        e.mem.set_u32(stream + 0x10, 300);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(state, 7);
        e.mem.set_u32(state + 0x20, 8);
        e.mem.set_u32(state + 0x24, have);
        e.mem.set_u32(state + 0x28, write);
        e.mem.set_u32(state + 0x2c, window);
        e.mem.set_u32(state + 0x44, codes[0]);
        e.mem.set_u32(state + 0x48, codes[1]);
        e.mem.set_u32(state + 0x4c, 2);
        e.mem.set_u32(state + 0x50, 2);
        e.call(0x00b4_9070, &args![stream, 300u32]);
        FastRun {
            stream,
            state,
            input: input_at,
            output,
        }
    }

    #[test]
    fn fn_00b49070_decodes_literals_copies_and_errors() {
        let mut e = Engine::new();
        let tables = (&LENGTH_CODES[..], &DISTANCE_CODES[..]);
        // 'A', 'B' through the link, a length of 5 at distance 2 (through
        // the distance link) from the output itself, the block's end.
        let bits = pack_bits(
            &[
                (2, 2),
                (3, 2),
                (0, 1),
                (0, 2),
                (2, 2),
                (2, 2),
                (0, 1),
                (1, 2),
            ],
            16,
        );
        let run = fast_run(&mut e, tables, &bits, 16, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 8), b"ABABABA\0");
        assert_eq!(e.mem.u32(run.state), 0xb);
        assert_eq!(e.mem.u32(run.stream), run.input + 2);
        assert_eq!(e.mem.u32(run.stream + 0x4), 14);
        assert_eq!(e.mem.u32(run.stream + 0xc), run.output + 7);
        assert_eq!(e.mem.u32(run.stream + 0x10), 293);
        assert_eq!(e.mem.u32(run.stream + 0x18), 0);
        assert_eq!(e.mem.u32(run.state + 0x30), 0);
        assert_eq!(e.mem.u32(run.state + 0x34), 2);

        // From the window: write position 0 (length 4, distance 2), behind
        // the write position (length 6, distance 5, wrapping to the
        // window's start), and before it (length 6, distance 2).
        let cases: [(u32, u32, u32, &[u8]); 3] = [
            (1, 1, 0, b"6767"),
            (3, 4, 3, b"670126"),
            (3, 1, 3, b"121212"),
        ];
        for (length_extra, distance_extra, write, expected) in cases {
            let bits = pack_bits(
                &[
                    (0, 2),
                    (length_extra, 2),
                    (0, 2),
                    (distance_extra, 3),
                    (1, 2),
                ],
                16,
            );
            let run = fast_run(&mut e, tables, &bits, 16, 8, write);
            assert_eq!(e.mem.bytes(run.output, expected.len() as u32), expected);
            assert_eq!(e.mem.u32(run.state), 0xb);
        }

        // Errors: a distance beyond the window's fill, an invalid distance
        // code, an invalid literal/length code (second level).
        let errors: [(&[(u32, u32)], u32, u32); 3] = [
            (
                &[(0, 2), (0, 2), (2, 2), (0, 1)],
                1,
                MESSAGE_DISTANCE_TOO_FAR,
            ),
            (&[(0, 2), (0, 2), (1, 2)], 8, MESSAGE_DISTANCE_CODE),
            (&[(3, 2), (1, 1)], 8, MESSAGE_LITERAL_LENGTH_CODE),
        ];
        for (fields, have, message) in errors {
            let bits = pack_bits(fields, 16);
            let run = fast_run(&mut e, tables, &bits, 16, have, 0);
            assert_eq!(e.mem.u32(run.stream + 0x18), message);
            assert_eq!(e.mem.u32(run.state), 0x1b);
            assert_eq!(e.mem.u8(run.output), 0);
        }

        // Extra bits that need one more byte (length) and two more bytes
        // (distance, after a code dropping 20 bits); the copy comes from the
        // window's last byte.
        let wide = (
            &[0x0003_021f, 0x0000_0260, 0, 0][..],
            &[0x0001_141f, 0, 0, 0][..],
        );
        let bits = pack_bits(&[(0, 2), (0, 15), (0, 20), (0, 15), (1, 2)], 16);
        let run = fast_run(&mut e, wide, &bits, 20, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 4), b"777\0");
        assert_eq!(e.mem.u32(run.state), 0xb);
        assert_eq!(e.mem.u32(run.stream), run.input + 7);
        assert_eq!(e.mem.u32(run.stream + 0x4), 13);
        assert_eq!(e.mem.u32(run.stream + 0x10), 297);
        assert_eq!(e.mem.u32(run.state + 0x34), 2);

        // The input runs out after one literal: the mode stays.
        let bits = pack_bits(&[(2, 2)], 16);
        let run = fast_run(&mut e, tables, &bits, 6, 8, 0);
        assert_eq!(e.mem.bytes(run.output, 2), b"A\0");
        assert_eq!(e.mem.u32(run.state), 7);
        assert_eq!(e.mem.u32(run.stream), run.input + 1);
        assert_eq!(e.mem.u32(run.stream + 0x4), 5);
        assert_eq!(e.mem.u32(run.state + 0x30), 0);
        assert_eq!(e.mem.u32(run.state + 0x34), 6);
    }

    /// Runs 00b497d0 of `kind` on `lengths` with root bits `root`; answers
    /// its result, the first `size` table words, how many entries the table
    /// pointer moved and the root bits written back.
    fn build_table(
        e: &mut Engine,
        kind: u32,
        lengths: &[u16],
        root: u32,
        size: u32,
    ) -> (i32, Vec<u32>, u32, u32) {
        let lens = e.mem.alloc(2 * lengths.len() as u32);
        for (i, length) in lengths.iter().enumerate() {
            e.mem.set_u16(lens + 2 * i as u32, *length);
        }
        let area = e.mem.alloc(4 * size + 0x40);
        let table = e.mem.alloc(4);
        e.mem.set_u32(table, area);
        let bits = e.mem.alloc(4);
        e.mem.set_u32(bits, root);
        let work = e.mem.alloc(0x240);
        let result = e
            .call(
                0x00b4_97d0,
                &args![kind, lens, lengths.len() as u32, table, bits, work],
            )
            .u32() as i32;
        let words = (0..size).map(|i| e.mem.u32(area + 4 * i)).collect();
        let moved = (e.mem.u32(table) - area) / 4;
        (result, words, moved, e.mem.u32(bits))
    }

    #[test]
    fn fn_00b497d0_builds_tables_and_refuses_bad_lengths() {
        let mut e = Engine::new();
        // Kind 0: lengths 2, 1, 3, 3 with three root bits.
        assert_eq!(
            build_table(&mut e, 0, &[2, 1, 3, 3], 3, 8),
            (
                0,
                vec![
                    0x0001_0100,
                    0x0000_0200,
                    0x0001_0100,
                    0x0002_0300,
                    0x0001_0100,
                    0x0000_0200,
                    0x0001_0100,
                    0x0003_0300,
                ],
                8,
                3
            )
        );
        // Kind 1: symbol 0 of length 1, 256 and 257 of length 2 with one
        // root bit: a second-level table for 256 (operation 0x60) and 257
        // (from the length tables, marked here).
        e.map(LENGTH_BASE, 0x100);
        e.mem.set_u16(LENGTH_BASE, 0x1234);
        e.mem.set_u8(LENGTH_EXTRA, 0x13);
        let mut lengths = vec![0u16; 258];
        lengths[0] = 1;
        lengths[256] = 2;
        lengths[257] = 2;
        assert_eq!(
            build_table(&mut e, 1, &lengths, 1, 4),
            (
                0,
                vec![0x0000_0100, 0x0002_0101, 0x0000_0160, 0x1234_0113],
                4,
                1
            )
        );
        // Kind 2: one code (symbol 1, length 1), root bits clamped from 2 to
        // 1, the other entry padded as invalid.
        e.map(DISTANCE_BASE, 0x80);
        e.mem.set_u16(DISTANCE_BASE + 2, 0x0abc);
        e.mem.set_u8(DISTANCE_EXTRA + 2, 0x11);
        assert_eq!(
            build_table(&mut e, 2, &[0, 1], 2, 2),
            (0, vec![0x0abc_0111, 0x0000_0140], 2, 1)
        );
        // No lengths, over-subscribed, incomplete for kind 0: -1, nothing
        // moved or written back.
        for lengths in [&[0u16, 0][..], &[1, 1, 1][..], &[1][..]] {
            let (result, _, moved, bits) = build_table(&mut e, 0, lengths, 9, 0);
            assert_eq!((result, moved, bits), (-1, 0, 9));
        }
        // Kind 1 with 11 root bits: 2048 entries reach the limit, answers 1.
        let (result, _, moved, bits) = build_table(&mut e, 1, &[11], 11, 0);
        assert_eq!((result, moved, bits), (1, 0, 11));
    }

    #[test]
    fn fn_00b4a070_answers_zero_or_the_taken_helper() {
        let mut e = Engine::new();
        e.register(CRC_TAKEN_HELPER, |_, a| (a[0] ^ a[1] ^ a[2]).into_ret());
        e.register(CRC_OTHER_HELPER, |_, _| 0xdead_beefu32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_a070, &args![5u32, 0u32, 3u32]).u32(), 0);
        assert_eq!(
            e.call(0x00b4_a070, &args![0x1111u32, 0x2000u32, 7u32])
                .u32(),
            0x1111 ^ 0x2000 ^ 7
        );
        let log = e.call_log.take().unwrap();
        let calls = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(at, _)| *at == addr)
                .map(|(_, a)| a.clone())
                .collect()
        };
        assert_eq!(calls(CRC_TAKEN_HELPER), vec![vec![0x1111, 0x2000, 7]]);
        assert!(calls(CRC_OTHER_HELPER).is_empty());
    }

    /// The reflected CRC-32 table (polynomial `edb88320`) and the three
    /// tables that step it a further byte each, built here (tests run without
    /// the exe).
    fn crc_tables() -> Vec<[u32; 256]> {
        let mut first = [0u32; 256];
        for (n, word) in first.iter_mut().enumerate() {
            let mut c = n as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 {
                    (c >> 1) ^ 0xedb8_8320
                } else {
                    c >> 1
                };
            }
            *word = c;
        }
        let mut tables = vec![first];
        for _ in 1..4 {
            let prior = tables[tables.len() - 1];
            let mut next = [0u32; 256];
            for (word, &before) in next.iter_mut().zip(prior.iter()) {
                *word = (before >> 8) ^ first[(before & 0xff) as usize];
            }
            tables.push(next);
        }
        tables
    }

    /// An engine with the eight tables where the exe keeps them: the four
    /// above, then the same four with each word's bytes reversed.
    fn crc_engine() -> Engine {
        let mut e = Engine::new();
        e.map(CRC_TABLE, 0x2000);
        let tables = crc_tables();
        for (k, table) in tables.iter().enumerate() {
            for (n, &word) in table.iter().enumerate() {
                let at = CRC_TABLE + (k as u32) * 0x400 + (n as u32) * 4;
                e.mem.set_u32(at, word);
                e.mem.set_u32(at + 0x1000, word.swap_bytes());
            }
        }
        e
    }

    /// CRC-32 a byte at a time, for the check.
    fn crc32(crc: u32, bytes: &[u8]) -> u32 {
        let first = crc_tables()[0];
        let mut c = !crc;
        for &b in bytes {
            c = (c >> 8) ^ first[((c ^ u32::from(b)) & 0xff) as usize];
        }
        !c
    }

    /// Writes `data` `offset` bytes past an aligned allocation; its address.
    fn place(e: &mut Engine, data: &[u8], offset: u32) -> u32 {
        let base = e.mem.alloc(data.len() as u32 + 8);
        assert_eq!(base & 3, 0);
        e.mem.write(base + offset, data);
        base + offset
    }

    #[test]
    fn fn_00b4a270_matches_crc32_through_every_loop() {
        let mut e = crc_engine();
        // Leaves the CRC as it is for an empty buffer (complemented twice).
        assert_eq!(
            e.call(0x00b4_a270, &args![0x1234_5678u32, 0u32, 0u32])
                .u32(),
            0x1234_5678
        );
        // The check value, from an unaligned start (bytes, words, bytes).
        let check = place(&mut e, b"123456789", 1);
        assert_eq!(
            e.call(0x00b4_a270, &args![0u32, check, 9u32]).u32(),
            0xcbf4_3926
        );
        // 78 bytes from +1: 3 alignment bytes, two 32-byte rounds, two words,
        // 3 bytes left; and from an aligned start; and chained in two calls.
        let data: Vec<u8> = (0..78u32).map(|i| (i * 37 + 11) as u8).collect();
        for offset in [0u32, 1, 2, 3] {
            let at = place(&mut e, &data, offset);
            assert_eq!(
                e.call(0x00b4_a270, &args![0u32, at, 78u32]).u32(),
                crc32(0, &data)
            );
            let head = e.call(0x00b4_a270, &args![0u32, at, 5u32]).u32();
            assert_eq!(
                e.call(0x00b4_a270, &args![head, at + 5, 73u32]).u32(),
                crc32(0, &data)
            );
        }
    }

    #[test]
    fn fn_00b4a670_steps_words_as_their_bytes_reversed() {
        let mut e = crc_engine();
        assert_eq!(
            e.call(0x00b4_a670, &args![0x1234_5678u32, 0u32, 0u32])
                .u32(),
            0x1234_5678
        );
        // Bytes only (three from +1): the plain CRC-32.
        let short = place(&mut e, b"abc", 1);
        assert_eq!(
            e.call(0x00b4_a670, &args![0u32, short, 3u32]).u32(),
            crc32(0, b"abc")
        );
        // With word steps, each aligned word's bytes count in reverse order
        // (the words are loaded little-endian, the tables step big-endian).
        let data: Vec<u8> = (0..78u32).map(|i| (i * 53 + 7) as u8).collect();
        for offset in [0u32, 1, 3] {
            let at = place(&mut e, &data, offset);
            let head = ((4 - offset) & 3) as usize;
            let words = (78 - head) / 4 * 4;
            let mut seen = data.clone();
            for group in seen[head..head + words].chunks_mut(4) {
                group.reverse();
            }
            assert_eq!(
                e.call(0x00b4_a670, &args![0x0bad_cafeu32, at, 78u32]).u32(),
                crc32(0x0bad_cafe, &seen)
            );
        }
    }
}
