//! `(unplaced)/run_00b45c00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `malloc` (`00ecd1c7`), `free` (`00ecd291`) and `memcpy` (`00ec44d0`),
/// all `cdecl`.
const MALLOC: u32 = 0x00ec_d1c7;
const FREE: u32 = 0x00ec_d291;
const MEMCPY: u32 = 0x00ec_44d0;

// Translated from 00b49020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default allocator `00b43e70` installs
/// (`cdecl`, three words: an opaque word, a count and a size). With a
/// non-zero opaque word the count goes through `size - size + count`; then
/// `malloc(count * size)` is answered.
pub fn fn_00b49020(e: &mut Engine, opaque: u32, count: u32, size: u32) -> u32 {
    let mut count = count;
    if opaque != 0 {
        count = size.wrapping_sub(size).wrapping_add(count);
    }
    e.call(MALLOC, &args![count.wrapping_mul(size)]).u32()
}

// Translated from 00b49050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default free (`cdecl`, two words: an
/// opaque word, not read, and the block): `free(block)`.
pub fn fn_00b49050(e: &mut Engine, _unused_opaque: u32, block: u32) {
    e.call(FREE, &args![block]);
}

// Translated from 00b45db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the stream record): takes
/// the stream down. Without a stream, a state (`+0x1c`) or a free function
/// (`+0x24`) answers -2. Frees the state's window (`+0x2c`) when there is
/// one, then the state when it was made by the set-up (`+0x34`), both
/// through the free function with the opaque word (`+0x28`); clears `+0x1c`
/// and answers 0.
pub fn fn_00b45db0(e: &mut Engine, stream: u32) -> i32 {
    if stream == 0 || e.mem.u32(stream + 0x1c) == 0 || e.mem.u32(stream + 0x24) == 0 {
        return -2;
    }
    let state = e.mem.u32(stream + 0x1c);
    let window = e.mem.u32(state + 0x2c);
    if window != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        e.call(free, &args![opaque, window]);
    }
    if e.mem.u32(stream + 0x34) != 0 {
        let free = e.mem.u32(stream + 0x24);
        let opaque = e.mem.u32(stream + 0x28);
        let state = e.mem.u32(stream + 0x1c);
        e.call(free, &args![opaque, state]);
    }
    e.mem.set_u32(stream + 0x1c, 0);
    0
}

// Translated from 00b45c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words: the stream record and the
/// output position): keeps the last output in the state's window. The window
/// (`+0x2c`, `1 << bits` bytes from the allocator with the opaque word) is
/// made on first use (none answers 1); its size (`+0x20`) set with the write
/// position (`+0x28`) and fill (`+0x24`) cleared. The new output is the
/// position minus the stream's `+0x10`: at least a window's worth copies the
/// window's size from the output's end, write position 0, fill the size;
/// less is copied at the write position up to the window's end and the rest
/// wraps to its start (then the write position is the rest and the fill the
/// size), or the write position moves on (wrapping at the size) and the fill
/// grows while below the size. Copies go through `memcpy`; answers 0.
pub fn fn_00b45c00(e: &mut Engine, stream: u32, position: u32) -> i32 {
    let state = e.mem.u32(stream + 0x1c);
    if e.mem.u32(state + 0x2c) == 0 {
        let allocate = e.mem.u32(stream + 0x20);
        let opaque = e.mem.u32(stream + 0x28);
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        let window = e.call(allocate, &args![opaque, size, 1u32]).u32();
        e.mem.set_u32(state + 0x2c, window);
        if e.mem.u32(state + 0x2c) == 0 {
            return 1;
        }
    }
    if e.mem.u32(state + 0x20) == 0 {
        let size = 1u32.wrapping_shl(e.mem.u32(state + 0x1c));
        e.mem.set_u32(state + 0x20, size);
        e.mem.set_u32(state + 0x28, 0);
        e.mem.set_u32(state + 0x24, 0);
    }
    let copy = position.wrapping_sub(e.mem.u32(stream + 0x10));
    let size = e.mem.u32(state + 0x20);
    let window = e.mem.u32(state + 0x2c);
    let end = e.mem.u32(stream + 0xc);
    if copy >= size {
        e.call(MEMCPY, &args![window, end.wrapping_sub(size), size]);
        e.mem.set_u32(state + 0x28, 0);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
        return 0;
    }
    let write = e.mem.u32(state + 0x28);
    let mut part = size.wrapping_sub(write);
    if part > copy {
        part = copy;
    }
    e.call(
        MEMCPY,
        &args![window.wrapping_add(write), end.wrapping_sub(copy), part],
    );
    let rest = copy.wrapping_sub(part);
    if rest != 0 {
        let window = e.mem.u32(state + 0x2c);
        let end = e.mem.u32(stream + 0xc);
        e.call(MEMCPY, &args![window, end.wrapping_sub(rest), rest]);
        e.mem.set_u32(state + 0x28, rest);
        let size = e.mem.u32(state + 0x20);
        e.mem.set_u32(state + 0x24, size);
    } else {
        let write = e.mem.u32(state + 0x28).wrapping_add(part);
        e.mem.set_u32(state + 0x28, write);
        if e.mem.u32(state + 0x28) == e.mem.u32(state + 0x20) {
            e.mem.set_u32(state + 0x28, 0);
        }
        if e.mem.u32(state + 0x24) < e.mem.u32(state + 0x20) {
            let fill = e.mem.u32(state + 0x24).wrapping_add(part);
            e.mem.set_u32(state + 0x24, fill);
        }
    }
    0
}

/// The fixed code tables `fn_00b45bd0` installs (`.rdata`): 512 length
/// codes, then 32 distance codes. Each code is a word: kind bits (byte 0),
/// bit count (byte 1) and value (high half).
const FIXED_LENGTH_CODES: u32 = 0x010a_5a98;
const FIXED_DISTANCE_CODES: u32 = 0x010a_6298;

// Translated from 00b45bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word: the decompressor state):
/// points the state's length codes (`+0x44`) at the fixed table `010a5a98`
/// with 9 index bits (`+0x4c`), and its distance codes (`+0x48`) at
/// `010a6298` with 5 (`+0x50`).
pub fn fn_00b45bd0(e: &mut Engine, state: u32) {
    e.mem.set_u32(state + 0x44, FIXED_LENGTH_CODES);
    e.mem.set_u32(state + 0x4c, 9);
    e.mem.set_u32(state + 0x48, FIXED_DISTANCE_CODES);
    e.mem.set_u32(state + 0x50, 5);
}

/// `00b4a070` (`cdecl`, three words: a running value, a buffer and its
/// length; answers the new value): the checksum `inflate` keeps over the
/// bytes of a header of the `1f 8b` kind (not translated here).
const HEADER_CHECKSUM: u32 = 0x00b4_a070;
/// `00b49e20` (`run_00b49e20.rs`, `cdecl`, same three words): the sum
/// checksum of the stream data; a null buffer answers its start value 1.
const DATA_CHECKSUM: u32 = 0x00b4_9e20;
/// `00b497d0` (`cdecl`, six words: the table kind 0/1/2, the code lengths,
/// their count, the address of the next free code slot, the address of the
/// index bit count, a work area; answers 0 when the table was built).
const TABLE_BUILDER: u32 = 0x00b4_97d0;
/// `00b49070` (`cdecl`, two words: the stream record and the output left at
/// the start of the call): the decoder `inflate` hands a block's codes to
/// while at least 6 input bytes and 258 output bytes are left.
const FAST_DECODER: u32 = 0x00b4_9070;

/// The order the code-length code lengths of a coded block arrive in (19
/// `u16` symbol numbers, `.rdata`).
const CODE_LENGTH_ORDER: u32 = 0x010a_6318;

/// The messages `inflate` leaves in the stream's message word (`+0x18`).
const MSG_INCORRECT_HEADER_CHECK: u32 = 0x011a_d4b4;
const MSG_UNKNOWN_METHOD_IN_HEADER: u32 = 0x011a_d4cc;
const MSG_INVALID_WINDOW_SIZE: u32 = 0x011a_d4e8;
const MSG_UNKNOWN_METHOD_IN_FLAGS: u32 = 0x011a_d4fc;
const MSG_UNKNOWN_HEADER_FLAGS: u32 = 0x011a_d518;
const MSG_HEADER_CRC_MISMATCH: u32 = 0x011a_d534;
const MSG_INVALID_BLOCK_TYPE: u32 = 0x011a_d548;
const MSG_INVALID_STORED_LENGTHS: u32 = 0x011a_d55c;
const MSG_TOO_MANY_SYMBOLS: u32 = 0x011a_d57c;
const MSG_INVALID_CODE_LENGTHS: u32 = 0x011a_d5a0;
const MSG_REPEAT_WITHOUT_LENGTH: u32 = 0x011a_d5bc;
const MSG_REPEAT_TOO_LONG: u32 = 0x011a_d5d8;
const MSG_INVALID_LENGTH_SET: u32 = 0x011a_d5f4;
const MSG_INVALID_DISTANCE_SET: u32 = 0x011a_d610;
const MSG_INVALID_LENGTH_CODE: u32 = 0x011a_d628;
const MSG_INVALID_DISTANCE_CODE: u32 = 0x011a_d644;
const MSG_DISTANCE_TOO_FAR: u32 = 0x011a_d65c;
const MSG_INCORRECT_LENGTH: u32 = 0x011a_d67c;

/// The stream record's words `inflate` uses.
const STREAM_INPUT: u32 = 0x00;
const STREAM_INPUT_LEFT: u32 = 0x04;
const STREAM_TOTAL_IN: u32 = 0x08;
const STREAM_OUTPUT: u32 = 0x0c;
const STREAM_OUTPUT_LEFT: u32 = 0x10;
const STREAM_TOTAL_OUT: u32 = 0x14;
const STREAM_MESSAGE: u32 = 0x18;
const STREAM_STATE: u32 = 0x1c;
/// Summary `inflate` leaves on return: bits held, 0x40 on the last block,
/// 0x80 between blocks.
const STREAM_BLOCK_SUMMARY: u32 = 0x2c;
const STREAM_CHECKSUM: u32 = 0x30;

/// The decompressor state's words `inflate` uses (the window words
/// `+0x1c`..`+0x2c` are the ones `fn_00b45c00` keeps).
const STATE_MODE: u32 = 0x00;
const STATE_LAST_BLOCK: u32 = 0x04;
/// Header kinds accepted: bit 0 the `78 xx` kind, bit 1 the `1f 8b` kind.
const STATE_WRAP: u32 = 0x08;
const STATE_HAVE_DICTIONARY: u32 = 0x0c;
const STATE_HEADER_FLAGS: u32 = 0x10;
const STATE_CHECKSUM: u32 = 0x14;
const STATE_TOTAL: u32 = 0x18;
const STATE_WINDOW_BITS: u32 = 0x1c;
const STATE_WINDOW_SIZE: u32 = 0x20;
const STATE_WINDOW_FILL: u32 = 0x24;
const STATE_WINDOW_WRITE: u32 = 0x28;
const STATE_WINDOW: u32 = 0x2c;
const STATE_HOLD: u32 = 0x30;
const STATE_BITS: u32 = 0x34;
const STATE_LENGTH: u32 = 0x38;
const STATE_DISTANCE: u32 = 0x3c;
const STATE_EXTRA: u32 = 0x40;
const STATE_LENGTH_CODES: u32 = 0x44;
const STATE_DISTANCE_CODES: u32 = 0x48;
const STATE_LENGTH_INDEX_BITS: u32 = 0x4c;
const STATE_DISTANCE_INDEX_BITS: u32 = 0x50;
const STATE_CODE_COUNT: u32 = 0x54;
const STATE_LENGTH_COUNT: u32 = 0x58;
const STATE_DISTANCE_COUNT: u32 = 0x5c;
const STATE_HAVE: u32 = 0x60;
const STATE_NEXT_CODE: u32 = 0x64;
/// 320 `u16` code lengths.
const STATE_LENGTHS: u32 = 0x68;
const STATE_WORK: u32 = 0x2e8;
const STATE_CODES: u32 = 0x528;

/// The modes `inflate` names more than once.
const MODE_BLOCK_START: u32 = 0x0b;
const MODE_BLOCK_HEADER: u32 = 0x0c;
const MODE_CODES: u32 = 0x12;
const MODE_BAD: u32 = 0x1b;
const MODE_NO_MEMORY: u32 = 0x1c;

/// `inflate`'s working copies of the input position and count and of the
/// bit buffer (the stream's `+0x00`/`+0x04`, the state's `+0x30`/`+0x34`).
struct Input {
    next: u32,
    have: u32,
    hold: u32,
    bits: u32,
}

impl Input {
    /// Takes one input byte into the bit buffer above the bits held; false
    /// (nothing taken) when no input is left.
    fn pull(&mut self, e: &Engine) -> bool {
        if self.have == 0 {
            return false;
        }
        self.have -= 1;
        let byte = u32::from(e.mem.u8(self.next));
        self.hold = byte.wrapping_shl(self.bits).wrapping_add(self.hold);
        self.next = self.next.wrapping_add(1);
        self.bits = self.bits.wrapping_add(8);
        true
    }

    /// Pulls bytes while fewer than `count` bits are held; false when the
    /// input ran out first.
    fn need(&mut self, e: &Engine, count: u32) -> bool {
        while self.bits < count {
            if !self.pull(e) {
                return false;
            }
        }
        true
    }

    /// Drops the low `count` bits.
    fn drop_bits(&mut self, count: u32) {
        self.hold = self.hold.wrapping_shr(count);
        self.bits = self.bits.wrapping_sub(count);
    }

    /// Empties the bit buffer.
    fn clear(&mut self) {
        self.hold = 0;
        self.bits = 0;
    }

    /// The low `count` bits held.
    fn low(&self, count: u32) -> u32 {
        1u32.wrapping_shl(count).wrapping_sub(1) & self.hold
    }
}

/// A code word's kind bits, bit count and value.
fn code_kind(code: u32) -> u32 {
    code & 0xff
}
fn code_bits(code: u32) -> u32 {
    (code >> 8) & 0xff
}
fn code_value(code: u32) -> u32 {
    code >> 16
}

/// The code word at `index` of the table at `table`.
fn code_at(e: &Engine, table: u32, index: u32) -> u32 {
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

/// Writes the working copies back: output position and count to the
/// stream, input position and count to the stream, the bit buffer to the
/// state.
fn inflate_store(e: &mut Engine, stream: u32, state: u32, put: u32, left: u32, input: &Input) {
    e.mem.set_u32(stream + STREAM_OUTPUT, put);
    e.mem.set_u32(stream + STREAM_OUTPUT_LEFT, left);
    e.mem.set_u32(stream + STREAM_INPUT, input.next);
    e.mem.set_u32(stream + STREAM_INPUT_LEFT, input.have);
    e.mem.set_u32(state + STATE_HOLD, input.hold);
    e.mem.set_u32(state + STATE_BITS, input.bits);
}

/// Leaves `message` in the stream and puts the state in the failed mode.
fn inflate_fail(e: &mut Engine, stream: u32, state: u32, message: u32) {
    e.mem.set_u32(stream + STREAM_MESSAGE, message);
    e.mem.set_u32(state + STATE_MODE, MODE_BAD);
}

/// Runs the header checksum over the low `count` bytes of `hold`, stored
/// low byte first in a stack buffer as `inflate` stores them.
fn inflate_checksum_held(e: &mut Engine, state: u32, hold: u32, count: u32) {
    let sum = e.with_stack(4, |e, buffer| {
        for i in 0..count {
            e.mem.set_u8(buffer.addr() + i, (hold >> (8 * i)) as u8);
        }
        let running = e.mem.u32(state + STATE_CHECKSUM);
        e.call(HEADER_CHECKSUM, &args![running, buffer.addr(), count])
            .u32()
    });
    e.mem.set_u32(state + STATE_CHECKSUM, sum);
}

/// Decodes one code of the table in the state word `table` (index bit count
/// in `index_bits`) the way `inflate`'s length and distance modes do: the
/// first-level code once enough bits are held, then, when its kind bits
/// have no high nibble (and, with `nonzero_kind`, are not zero), the code of
/// the sub-table it points to, dropping the first code's bits. The answered
/// code's own bits are not dropped. `None` when the input ran out.
fn inflate_decode(
    e: &Engine,
    state: u32,
    table: u32,
    index_bits: u32,
    input: &mut Input,
    nonzero_kind: bool,
) -> Option<u32> {
    let first = loop {
        let index = input.low(e.mem.u32(state + index_bits));
        let code = code_at(e, e.mem.u32(state + table), index);
        if code_bits(code) <= input.bits {
            break code;
        }
        if !input.pull(e) {
            return None;
        }
    };
    if (nonzero_kind && code_kind(first) == 0) || code_kind(first) & 0xf0 != 0 {
        return Some(first);
    }
    let code = loop {
        let index = code_value(first).wrapping_add(
            input
                .low(code_bits(first) + code_kind(first))
                .wrapping_shr(code_bits(first)),
        );
        let code = code_at(e, e.mem.u32(state + table), index);
        if code_bits(first) + code_bits(code) <= input.bits {
            break code;
        }
        if !input.pull(e) {
            return None;
        }
    };
    input.drop_bits(code_bits(first));
    Some(code)
}

// Translated from 00b44000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `inflate` (engine map; unit `zLib/inflate.c`; `cdecl`, two words: the
/// stream record and the flush kind). Answers -2 without a stream, a state
/// (`+0x1c`) or an output (`+0x0c`), or with input left but no input
/// pointer. Otherwise runs the state's mode machine over the stream's input
/// and output, keeping working copies of the input, output and bit buffer:
/// header checks (`78 xx` and `1f 8b` kinds, with their flags, extra field,
/// name, comment and header checksum), the dictionary id (answers 2 when no
/// dictionary is set), block headers, stored blocks (copied with `memcpy`),
/// coded blocks (code-length, length and distance tables through
/// `00b497d0`, the fixed ones through `fn_00b45bd0`), literal and match
/// output (through `00b49070` while at least 6 input and 258 output bytes
/// are left), and the trailer. A bad stream leaves a message (`+0x18`) and
/// answers -3; the end answers 1. When the input or output runs out, or
/// after the end, the copies are written back, the window is updated
/// (`fn_00b45c00`, -4 and the no-memory mode when it fails), the totals
/// grow and the block summary (`+0x2c`) is set; no progress (or flush kind
/// 4) with nothing else to report answers -5.
///
/// As the exe has it: the trailer's checksum words are read but never
/// compared, and the data checksum is cleared (state `+0x14`, stream
/// `+0x30`) wherever output was counted; the name and comment fields go
/// through the header checksum only under flag 0x2000, which the flags mode
/// refuses; and a bad repeat in the code lengths still goes on to build
/// the tables, whose success puts the codes mode over the failed one.
pub fn inflate(e: &mut Engine, stream: u32, flush: i32) -> i32 {
    if stream == 0
        || e.mem.u32(stream + STREAM_STATE) == 0
        || e.mem.u32(stream + STREAM_OUTPUT) == 0
        || (e.mem.u32(stream + STREAM_INPUT) == 0 && e.mem.u32(stream + STREAM_INPUT_LEFT) != 0)
    {
        return -2;
    }
    let state = e.mem.u32(stream + STREAM_STATE);
    if e.mem.u32(state + STATE_MODE) == MODE_BLOCK_START {
        e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_HEADER);
    }
    let mut put = e.mem.u32(stream + STREAM_OUTPUT);
    let mut left = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
    let mut input = Input {
        next: e.mem.u32(stream + STREAM_INPUT),
        have: e.mem.u32(stream + STREAM_INPUT_LEFT),
        hold: e.mem.u32(state + STATE_HOLD),
        bits: e.mem.u32(state + STATE_BITS),
    };
    let input_start = input.have;
    let mut output_start = left;
    let mut ret = 0i32;
    'run: loop {
        let mode = e.mem.u32(state + STATE_MODE);
        match mode {
            // Header.
            0x00 => {
                if e.mem.u32(state + STATE_WRAP) == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_HEADER);
                    continue 'run;
                }
                if !input.need(e, 0x10) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_WRAP) & 2 != 0 && input.hold == 0x8b1f {
                    let start = e.call(HEADER_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                    e.mem.set_u32(state + STATE_CHECKSUM, start);
                    inflate_checksum_held(e, state, input.hold, 2);
                    input.clear();
                    e.mem.set_u32(state + STATE_MODE, 1);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HEADER_FLAGS, 0);
                if e.mem.u32(state + STATE_WRAP) & 1 == 0
                    || ((input.hold & 0xff) << 8).wrapping_add(input.hold >> 8) % 0x1f != 0
                {
                    inflate_fail(e, stream, state, MSG_INCORRECT_HEADER_CHECK);
                    continue 'run;
                }
                if input.hold & 0xf != 8 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_METHOD_IN_HEADER);
                    continue 'run;
                }
                input.drop_bits(4);
                if (input.hold & 0xf) + 8 > e.mem.u32(state + STATE_WINDOW_BITS) {
                    inflate_fail(e, stream, state, MSG_INVALID_WINDOW_SIZE);
                    continue 'run;
                }
                let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                let next = if input.hold & 0x200 != 0 {
                    9
                } else {
                    MODE_BLOCK_START
                };
                e.mem.set_u32(state + STATE_MODE, next);
                input.clear();
            }
            // Header flags (method byte and flag byte).
            0x01 => {
                if !input.need(e, 0x10) {
                    break 'run;
                }
                e.mem.set_u32(state + STATE_HEADER_FLAGS, input.hold);
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0xff != 8 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_METHOD_IN_FLAGS);
                    continue 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0xe000 != 0 {
                    inflate_fail(e, stream, state, MSG_UNKNOWN_HEADER_FLAGS);
                    continue 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 2);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 2);
            }
            // Header time word.
            0x02 => {
                if !input.need(e, 0x20) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 4);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 3);
            }
            // Header extra-flags and system bytes.
            0x03 => {
                if !input.need(e, 0x10) {
                    break 'run;
                }
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    inflate_checksum_held(e, state, input.hold, 2);
                }
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 4);
            }
            // Extra field length.
            0x04 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x400 != 0 {
                    if !input.need(e, 0x10) {
                        break 'run;
                    }
                    e.mem.set_u32(state + STATE_LENGTH, input.hold);
                    if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                        inflate_checksum_held(e, state, input.hold, 2);
                    }
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 5);
            }
            // Extra field.
            0x05 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x400 != 0 {
                    let mut copy = e.mem.u32(state + STATE_LENGTH);
                    if copy > input.have {
                        copy = input.have;
                    }
                    if copy != 0 {
                        if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                            let running = e.mem.u32(state + STATE_CHECKSUM);
                            let sum = e
                                .call(HEADER_CHECKSUM, &args![running, input.next, copy])
                                .u32();
                            e.mem.set_u32(state + STATE_CHECKSUM, sum);
                        }
                        input.have = input.have.wrapping_sub(copy);
                        input.next = input.next.wrapping_add(copy);
                        let length = e.mem.u32(state + STATE_LENGTH).wrapping_sub(copy);
                        e.mem.set_u32(state + STATE_LENGTH, length);
                    }
                    if e.mem.u32(state + STATE_LENGTH) != 0 {
                        break 'run;
                    }
                }
                e.mem.set_u32(state + STATE_MODE, 6);
            }
            // Name (0x800) and comment (0x1000): bytes up to a zero.
            0x06 | 0x07 => {
                let flag = if mode == 0x06 { 0x800 } else { 0x1000 };
                if e.mem.u32(state + STATE_HEADER_FLAGS) & flag != 0 {
                    if input.have == 0 {
                        break 'run;
                    }
                    let mut copy = 0u32;
                    let last = loop {
                        let byte = u32::from(e.mem.u8(input.next.wrapping_add(copy)));
                        copy = copy.wrapping_add(1);
                        if byte == 0 || copy >= input.have {
                            break byte;
                        }
                    };
                    if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x2000 != 0 {
                        let running = e.mem.u32(state + STATE_CHECKSUM);
                        let sum = e
                            .call(HEADER_CHECKSUM, &args![running, input.next, copy])
                            .u32();
                        e.mem.set_u32(state + STATE_CHECKSUM, sum);
                    }
                    input.have = input.have.wrapping_sub(copy);
                    input.next = input.next.wrapping_add(copy);
                    if last != 0 {
                        break 'run;
                    }
                }
                e.mem.set_u32(state + STATE_MODE, mode + 1);
            }
            // Header checksum.
            0x08 => {
                if e.mem.u32(state + STATE_HEADER_FLAGS) & 0x200 != 0 {
                    if !input.need(e, 0x10) {
                        break 'run;
                    }
                    if input.hold != e.mem.u32(state + STATE_CHECKSUM) & 0xffff {
                        inflate_fail(e, stream, state, MSG_HEADER_CRC_MISMATCH);
                        continue 'run;
                    }
                    input.clear();
                }
                let start = e.call(HEADER_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
            }
            // Dictionary id (high byte first).
            0x09 => {
                if !input.need(e, 0x20) {
                    break 'run;
                }
                let hold = input.hold;
                let id = ((hold >> 24) & 0xff)
                    .wrapping_add((hold >> 8) & 0xff00)
                    .wrapping_add((hold & 0xff00) << 8)
                    .wrapping_add((hold & 0xff) << 24);
                e.mem.set_u32(state + STATE_CHECKSUM, id);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 0x0a);
            }
            // Dictionary wanted.
            0x0a => {
                if e.mem.u32(state + STATE_HAVE_DICTIONARY) == 0 {
                    inflate_store(e, stream, state, put, left, &input);
                    return 2;
                }
                let start = e.call(DATA_CHECKSUM, &args![0u32, 0u32, 0u32]).u32();
                e.mem.set_u32(state + STATE_CHECKSUM, start);
                let check = e.mem.u32(state + STATE_CHECKSUM);
                e.mem.set_u32(stream + STREAM_CHECKSUM, check);
                e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
            }
            // Block start (stops here for flush kind 5) and block header.
            0x0b | 0x0c => {
                if mode == MODE_BLOCK_START && flush == 5 {
                    break 'run;
                }
                if e.mem.u32(state + STATE_LAST_BLOCK) != 0 {
                    input.drop_bits(input.bits & 7);
                    e.mem.set_u32(state + STATE_MODE, 0x18);
                    continue 'run;
                }
                if !input.need(e, 3) {
                    break 'run;
                }
                e.mem.set_u32(state + STATE_LAST_BLOCK, input.hold & 1);
                input.drop_bits(1);
                // The exe's switch on the two type bits; its default (above
                // 3) cannot be reached and skips to the same drop.
                match input.hold & 3 {
                    0 => e.mem.set_u32(state + STATE_MODE, 0x0d),
                    1 => {
                        fn_00b45bd0(e, state);
                        e.mem.set_u32(state + STATE_MODE, MODE_CODES);
                    }
                    2 => e.mem.set_u32(state + STATE_MODE, 0x0f),
                    _ => inflate_fail(e, stream, state, MSG_INVALID_BLOCK_TYPE),
                }
                input.drop_bits(2);
            }
            // Stored block lengths.
            0x0d => {
                input.drop_bits(input.bits & 7);
                if !input.need(e, 0x20) {
                    break 'run;
                }
                if input.hold & 0xffff != (input.hold >> 16) ^ 0xffff {
                    inflate_fail(e, stream, state, MSG_INVALID_STORED_LENGTHS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_LENGTH, input.hold & 0xffff);
                input.clear();
                e.mem.set_u32(state + STATE_MODE, 0x0e);
            }
            // Stored block copy.
            0x0e => {
                let mut copy = e.mem.u32(state + STATE_LENGTH);
                if copy == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
                    continue 'run;
                }
                if copy > input.have {
                    copy = input.have;
                }
                if copy > left {
                    copy = left;
                }
                if copy == 0 {
                    break 'run;
                }
                e.call(MEMCPY, &args![put, input.next, copy]);
                input.have = input.have.wrapping_sub(copy);
                input.next = input.next.wrapping_add(copy);
                left = left.wrapping_sub(copy);
                put = put.wrapping_add(copy);
                let length = e.mem.u32(state + STATE_LENGTH).wrapping_sub(copy);
                e.mem.set_u32(state + STATE_LENGTH, length);
            }
            // Coded block counts.
            0x0f => {
                if !input.need(e, 0x0e) {
                    break 'run;
                }
                e.mem
                    .set_u32(state + STATE_LENGTH_COUNT, (input.hold & 0x1f) + 0x101);
                input.drop_bits(5);
                e.mem
                    .set_u32(state + STATE_DISTANCE_COUNT, (input.hold & 0x1f) + 1);
                input.drop_bits(5);
                e.mem
                    .set_u32(state + STATE_CODE_COUNT, (input.hold & 0xf) + 4);
                input.drop_bits(4);
                if e.mem.u32(state + STATE_LENGTH_COUNT) > 0x11e
                    || e.mem.u32(state + STATE_DISTANCE_COUNT) > 0x1e
                {
                    inflate_fail(e, stream, state, MSG_TOO_MANY_SYMBOLS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HAVE, 0);
                e.mem.set_u32(state + STATE_MODE, 0x10);
            }
            // Code-length code lengths, then their table.
            0x10 => {
                while e.mem.u32(state + STATE_HAVE) < e.mem.u32(state + STATE_CODE_COUNT) {
                    if !input.need(e, 3) {
                        break 'run;
                    }
                    let have = e.mem.u32(state + STATE_HAVE);
                    let symbol = u32::from(
                        e.mem
                            .u16(CODE_LENGTH_ORDER.wrapping_add(have.wrapping_mul(2))),
                    );
                    e.mem.set_u16(
                        state
                            .wrapping_add(symbol.wrapping_mul(2))
                            .wrapping_add(STATE_LENGTHS),
                        (input.hold & 7) as u16,
                    );
                    e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                    input.drop_bits(3);
                }
                while e.mem.u32(state + STATE_HAVE) < 0x13 {
                    let have = e.mem.u32(state + STATE_HAVE);
                    let symbol = u32::from(
                        e.mem
                            .u16(CODE_LENGTH_ORDER.wrapping_add(have.wrapping_mul(2))),
                    );
                    e.mem.set_u16(
                        state
                            .wrapping_add(symbol.wrapping_mul(2))
                            .wrapping_add(STATE_LENGTHS),
                        0,
                    );
                    e.mem.set_u32(state + STATE_HAVE, have + 1);
                }
                e.mem.set_u32(state + STATE_NEXT_CODE, state + STATE_CODES);
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_LENGTH_CODES, next);
                e.mem.set_u32(state + STATE_LENGTH_INDEX_BITS, 7);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            0u32,
                            state + STATE_LENGTHS,
                            0x13u32,
                            state + STATE_NEXT_CODE,
                            state + STATE_LENGTH_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_CODE_LENGTHS);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_HAVE, 0);
                e.mem.set_u32(state + STATE_MODE, 0x11);
            }
            // Code lengths of the length and distance codes, then their
            // tables.
            0x11 => {
                'lengths: while e.mem.u32(state + STATE_HAVE)
                    < e.mem
                        .u32(state + STATE_LENGTH_COUNT)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE_COUNT))
                {
                    let code = loop {
                        let index = input.low(e.mem.u32(state + STATE_LENGTH_INDEX_BITS));
                        let code = code_at(e, e.mem.u32(state + STATE_LENGTH_CODES), index);
                        if code_bits(code) <= input.bits {
                            break code;
                        }
                        if !input.pull(e) {
                            break 'run;
                        }
                    };
                    let value = code_value(code);
                    if value < 0x10 {
                        if !input.need(e, code_bits(code)) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let have = e.mem.u32(state + STATE_HAVE);
                        e.mem.set_u16(
                            state
                                .wrapping_add(have.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            value as u16,
                        );
                        e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                        continue 'lengths;
                    }
                    let (length, copy) = if value == 0x10 {
                        if !input.need(e, code_bits(code) + 2) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let have = e.mem.u32(state + STATE_HAVE);
                        if have == 0 {
                            inflate_fail(e, stream, state, MSG_REPEAT_WITHOUT_LENGTH);
                            break 'lengths;
                        }
                        let previous = u32::from(
                            e.mem
                                .u16(state.wrapping_add(have.wrapping_mul(2)).wrapping_add(0x66)),
                        );
                        let copy = (input.hold & 3) + 3;
                        input.drop_bits(2);
                        (previous, copy)
                    } else if value == 0x11 {
                        if !input.need(e, code_bits(code) + 3) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let copy = (input.hold & 7) + 3;
                        input.drop_bits(3);
                        (0, copy)
                    } else {
                        if !input.need(e, code_bits(code) + 7) {
                            break 'run;
                        }
                        input.drop_bits(code_bits(code));
                        let copy = (input.hold & 0x7f) + 0xb;
                        input.drop_bits(7);
                        (0, copy)
                    };
                    let total = e
                        .mem
                        .u32(state + STATE_LENGTH_COUNT)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE_COUNT));
                    if e.mem.u32(state + STATE_HAVE).wrapping_add(copy) > total {
                        inflate_fail(e, stream, state, MSG_REPEAT_TOO_LONG);
                        break 'lengths;
                    }
                    for _ in 0..copy {
                        let have = e.mem.u32(state + STATE_HAVE);
                        e.mem.set_u16(
                            state
                                .wrapping_add(have.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            length as u16,
                        );
                        e.mem.set_u32(state + STATE_HAVE, have.wrapping_add(1));
                    }
                }
                // The exe goes on here after a bad repeat too (see above).
                e.mem.set_u32(state + STATE_NEXT_CODE, state + STATE_CODES);
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_LENGTH_CODES, next);
                e.mem.set_u32(state + STATE_LENGTH_INDEX_BITS, 9);
                let length_count = e.mem.u32(state + STATE_LENGTH_COUNT);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            1u32,
                            state + STATE_LENGTHS,
                            length_count,
                            state + STATE_NEXT_CODE,
                            state + STATE_LENGTH_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_LENGTH_SET);
                    continue 'run;
                }
                let next = e.mem.u32(state + STATE_NEXT_CODE);
                e.mem.set_u32(state + STATE_DISTANCE_CODES, next);
                e.mem.set_u32(state + STATE_DISTANCE_INDEX_BITS, 6);
                let distance_count = e.mem.u32(state + STATE_DISTANCE_COUNT);
                let length_count = e.mem.u32(state + STATE_LENGTH_COUNT);
                ret = e
                    .call(
                        TABLE_BUILDER,
                        &args![
                            2u32,
                            state
                                .wrapping_add(length_count.wrapping_mul(2))
                                .wrapping_add(STATE_LENGTHS),
                            distance_count,
                            state + STATE_NEXT_CODE,
                            state + STATE_DISTANCE_INDEX_BITS,
                            state + STATE_WORK
                        ],
                    )
                    .i32();
                if ret != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_DISTANCE_SET);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_MODE, MODE_CODES);
            }
            // Length (or literal) code.
            0x12 => {
                if input.have >= 6 && left >= 0x102 {
                    inflate_store(e, stream, state, put, left, &input);
                    e.call(FAST_DECODER, &args![stream, output_start]);
                    put = e.mem.u32(stream + STREAM_OUTPUT);
                    left = e.mem.u32(stream + STREAM_OUTPUT_LEFT);
                    input.next = e.mem.u32(stream + STREAM_INPUT);
                    input.have = e.mem.u32(stream + STREAM_INPUT_LEFT);
                    input.hold = e.mem.u32(state + STATE_HOLD);
                    input.bits = e.mem.u32(state + STATE_BITS);
                    continue 'run;
                }
                let Some(code) = inflate_decode(
                    e,
                    state,
                    STATE_LENGTH_CODES,
                    STATE_LENGTH_INDEX_BITS,
                    &mut input,
                    true,
                ) else {
                    break 'run;
                };
                input.drop_bits(code_bits(code));
                e.mem.set_u32(state + STATE_LENGTH, code_value(code));
                let kind = code_kind(code);
                if kind == 0 {
                    e.mem.set_u32(state + STATE_MODE, 0x17);
                } else if kind & 0x20 != 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_BLOCK_START);
                } else if kind & 0x40 != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_LENGTH_CODE);
                } else {
                    e.mem.set_u32(state + STATE_EXTRA, kind & 0xf);
                    e.mem.set_u32(state + STATE_MODE, 0x13);
                }
            }
            // Length extra bits.
            0x13 => {
                let extra = e.mem.u32(state + STATE_EXTRA);
                if extra != 0 {
                    if !input.need(e, extra) {
                        break 'run;
                    }
                    let length = input
                        .low(extra)
                        .wrapping_add(e.mem.u32(state + STATE_LENGTH));
                    e.mem.set_u32(state + STATE_LENGTH, length);
                    input.drop_bits(extra);
                }
                e.mem.set_u32(state + STATE_MODE, 0x14);
            }
            // Distance code.
            0x14 => {
                let Some(code) = inflate_decode(
                    e,
                    state,
                    STATE_DISTANCE_CODES,
                    STATE_DISTANCE_INDEX_BITS,
                    &mut input,
                    false,
                ) else {
                    break 'run;
                };
                input.drop_bits(code_bits(code));
                if code_kind(code) & 0x40 != 0 {
                    inflate_fail(e, stream, state, MSG_INVALID_DISTANCE_CODE);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_DISTANCE, code_value(code));
                e.mem.set_u32(state + STATE_EXTRA, code_kind(code) & 0xf);
                e.mem.set_u32(state + STATE_MODE, 0x15);
            }
            // Distance extra bits and the distance check.
            0x15 => {
                let extra = e.mem.u32(state + STATE_EXTRA);
                if extra != 0 {
                    if !input.need(e, extra) {
                        break 'run;
                    }
                    let distance = input
                        .low(extra)
                        .wrapping_add(e.mem.u32(state + STATE_DISTANCE));
                    e.mem.set_u32(state + STATE_DISTANCE, distance);
                    input.drop_bits(extra);
                }
                let reach = e
                    .mem
                    .u32(state + STATE_WINDOW_FILL)
                    .wrapping_add(output_start)
                    .wrapping_sub(left);
                if e.mem.u32(state + STATE_DISTANCE) > reach {
                    inflate_fail(e, stream, state, MSG_DISTANCE_TOO_FAR);
                    continue 'run;
                }
                e.mem.set_u32(state + STATE_MODE, 0x16);
            }
            // Match copy (from the window or the output), byte by byte.
            0x16 => {
                if left == 0 {
                    break 'run;
                }
                let made = output_start.wrapping_sub(left);
                let distance = e.mem.u32(state + STATE_DISTANCE);
                let length = e.mem.u32(state + STATE_LENGTH);
                let (mut from, mut copy) = if distance > made {
                    let mut copy = distance.wrapping_sub(made);
                    let write = e.mem.u32(state + STATE_WINDOW_WRITE);
                    let window = e.mem.u32(state + STATE_WINDOW);
                    let from = if copy > write {
                        copy = copy.wrapping_sub(write);
                        e.mem
                            .u32(state + STATE_WINDOW_SIZE)
                            .wrapping_sub(copy)
                            .wrapping_add(window)
                    } else {
                        write.wrapping_sub(copy).wrapping_add(window)
                    };
                    if copy > length {
                        copy = length;
                    }
                    (from, copy)
                } else {
                    (put.wrapping_sub(distance), length)
                };
                if copy > left {
                    copy = left;
                }
                left = left.wrapping_sub(copy);
                e.mem
                    .set_u32(state + STATE_LENGTH, length.wrapping_sub(copy));
                loop {
                    let byte = e.mem.u8(from);
                    e.mem.set_u8(put, byte);
                    put = put.wrapping_add(1);
                    from = from.wrapping_add(1);
                    copy = copy.wrapping_sub(1);
                    if copy == 0 {
                        break;
                    }
                }
                if e.mem.u32(state + STATE_LENGTH) == 0 {
                    e.mem.set_u32(state + STATE_MODE, MODE_CODES);
                }
            }
            // Literal.
            0x17 => {
                if left == 0 {
                    break 'run;
                }
                let literal = e.mem.u8(state + STATE_LENGTH);
                e.mem.set_u8(put, literal);
                put = put.wrapping_add(1);
                left -= 1;
                e.mem.set_u32(state + STATE_MODE, MODE_CODES);
            }
            // Trailer checksum word (read, not compared).
            0x18 => {
                if e.mem.u32(state + STATE_WRAP) != 0 {
                    if !input.need(e, 0x20) {
                        break 'run;
                    }
                    output_start = output_start.wrapping_sub(left);
                    let total_out = e
                        .mem
                        .u32(stream + STREAM_TOTAL_OUT)
                        .wrapping_add(output_start);
                    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total_out);
                    let total = e.mem.u32(state + STATE_TOTAL).wrapping_add(output_start);
                    e.mem.set_u32(state + STATE_TOTAL, total);
                    if output_start != 0 {
                        e.mem.set_u32(state + STATE_CHECKSUM, 0);
                        e.mem.set_u32(stream + STREAM_CHECKSUM, 0);
                    }
                    output_start = left;
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 0x19);
            }
            // Trailer length word (with header flags only).
            0x19 => {
                if e.mem.u32(state + STATE_WRAP) != 0 && e.mem.u32(state + STATE_HEADER_FLAGS) != 0
                {
                    if !input.need(e, 0x20) {
                        break 'run;
                    }
                    if input.hold != e.mem.u32(state + STATE_TOTAL) {
                        inflate_fail(e, stream, state, MSG_INCORRECT_LENGTH);
                        continue 'run;
                    }
                    input.clear();
                }
                e.mem.set_u32(state + STATE_MODE, 0x1a);
            }
            // Done.
            0x1a => {
                ret = 1;
                break 'run;
            }
            // Failed.
            0x1b => {
                ret = -3;
                break 'run;
            }
            MODE_NO_MEMORY => return -4,
            _ => return -2,
        }
    }
    inflate_store(e, stream, state, put, left, &input);
    if (e.mem.u32(state + STATE_WINDOW_SIZE) != 0
        || ((e.mem.u32(state + STATE_MODE) as i32) < 0x18
            && output_start != e.mem.u32(stream + STREAM_OUTPUT_LEFT)))
        && fn_00b45c00(e, stream, output_start) != 0
    {
        e.mem.set_u32(state + STATE_MODE, MODE_NO_MEMORY);
        return -4;
    }
    let input_used = input_start.wrapping_sub(e.mem.u32(stream + STREAM_INPUT_LEFT));
    let output_made = output_start.wrapping_sub(e.mem.u32(stream + STREAM_OUTPUT_LEFT));
    let total_in = e.mem.u32(stream + STREAM_TOTAL_IN).wrapping_add(input_used);
    e.mem.set_u32(stream + STREAM_TOTAL_IN, total_in);
    let total_out = e
        .mem
        .u32(stream + STREAM_TOTAL_OUT)
        .wrapping_add(output_made);
    e.mem.set_u32(stream + STREAM_TOTAL_OUT, total_out);
    let total = e.mem.u32(state + STATE_TOTAL).wrapping_add(output_made);
    e.mem.set_u32(state + STATE_TOTAL, total);
    if e.mem.u32(state + STATE_WRAP) != 0 && output_made != 0 {
        e.mem.set_u32(state + STATE_CHECKSUM, 0);
        e.mem.set_u32(stream + STREAM_CHECKSUM, 0);
    }
    let last = if e.mem.u32(state + STATE_LAST_BLOCK) != 0 {
        0x40
    } else {
        0
    };
    let between = if e.mem.u32(state + STATE_MODE) == MODE_BLOCK_START {
        0x80
    } else {
        0
    };
    let summary = e
        .mem
        .u32(state + STATE_BITS)
        .wrapping_add(last)
        .wrapping_add(between);
    e.mem.set_u32(stream + STREAM_BLOCK_SUMMARY, summary);
    if ((input_used == 0 && output_made == 0) || flush == 4) && ret == 0 {
        ret = -5;
    }
    ret
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b4_9020, fn_00b49020(u32, u32, u32) -> u32),
        entry!(0x00b4_9050, fn_00b49050(u32, u32)),
        entry!(0x00b4_5db0, fn_00b45db0(u32) -> i32),
        entry!(0x00b4_5c00, fn_00b45c00(u32, u32) -> i32),
        entry!(0x00b4_5bd0, fn_00b45bd0(u32)),
        entry!(0x00b4_4000, inflate(u32, i32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_allocator_and_free() {
        let mut e = Engine::new();
        e.register(MALLOC, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FREE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_9020, &args![0u32, 3u32, 4u32]).u32(), 0x100c);
        assert_eq!(e.call(0x00b4_9020, &args![1u32, 3u32, 4u32]).u32(), 0x100c);
        e.call_log = Some(vec![]);
        e.call(0x00b4_9050, &args![9u32, 0x500u32]);
        assert_eq!(e.call_log.take().unwrap()[1], (FREE, vec![0x500]));
    }

    const ALLOCATE: u32 = 0x013b_0000;
    const RELEASE: u32 = 0x013b_0010;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.register(RELEASE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, RELEASE);
        e.mem.set_u32(state + 0x1c, 3);
        (e, stream, state)
    }

    #[test]
    fn fn_00b45db0_frees_what_was_made() {
        let (mut e, stream, state) = engine();
        e.mem.set_u32(state + 0x2c, 0x7000);
        e.mem.set_u32(stream + 0x34, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), 0);
        let frees: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == RELEASE)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(frees, vec![0x7000, state]);
        assert_eq!(e.mem.u32(stream + 0x1c), 0);
        assert_eq!(e.call(0x00b4_5db0, &args![stream]).i32(), -2);
    }

    #[test]
    fn fn_00b45c00_keeps_the_last_output() {
        let (mut e, stream, state) = engine();
        let output = e.mem.alloc(0x20);
        for i in 0..0x20 {
            e.mem.set_u8(output + i, i as u8);
        }
        // 5 bytes of output into an empty 8-byte window.
        e.mem.set_u32(stream + 0xc, output + 5);
        e.mem.set_u32(stream + 0x10, 0x100);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x105u32]).i32(), 0);
        let window = e.mem.u32(state + 0x2c);
        assert_eq!(e.mem.u32(state + 0x20), 8);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (5, 5));
        assert_eq!(e.mem.u8(window + 4), 4);
        // 6 more: 3 fill the window, 3 wrap to its start.
        e.mem.set_u32(stream + 0xc, output + 11);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x106u32]).i32(), 0);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (3, 8));
        assert_eq!(e.mem.u8(window + 7), 7);
        assert_eq!(e.mem.u8(window), 8);
        // A window's worth or more: the last 8 bytes, from the start.
        e.mem.set_u32(stream + 0xc, output + 0x20);
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0x120u32]).i32(), 0);
        assert_eq!(e.mem.u8(window), 0x18);
        assert_eq!((e.mem.u32(state + 0x28), e.mem.u32(state + 0x24)), (0, 8));
    }

    #[test]
    fn fn_00b45c00_fails_without_a_window() {
        let (mut e, stream, _) = engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_5c00, &args![stream, 0u32]).i32(), 1);
    }

    #[test]
    fn fn_00b45bd0_installs_the_fixed_tables() {
        let mut e = Engine::new();
        let state = e.mem.alloc(0x60);
        e.call(0x00b4_5bd0, &args![state]);
        assert_eq!(e.mem.u32(state + 0x44), 0x010a_5a98);
        assert_eq!(e.mem.u32(state + 0x48), 0x010a_6298);
        assert_eq!((e.mem.u32(state + 0x4c), e.mem.u32(state + 0x50)), (9, 5));
    }

    /// The test's model of the header checksum double.
    fn header_sum(running: u32, bytes: &[u8]) -> u32 {
        bytes.iter().fold(running.wrapping_mul(31) ^ 0x55, |s, &b| {
            s.wrapping_mul(3).wrapping_add(u32::from(b))
        })
    }

    /// Bits written low bit first, after whole prefix bytes.
    struct BitWriter {
        bytes: Vec<u8>,
        bit: u32,
    }

    impl BitWriter {
        fn new(prefix: &[u8]) -> Self {
            BitWriter {
                bytes: prefix.to_vec(),
                bit: 0,
            }
        }
        fn put(&mut self, value: u32, count: u32) {
            for i in 0..count {
                if self.bit % 8 == 0 {
                    self.bytes.push(0);
                }
                if (value >> i) & 1 != 0 {
                    *self.bytes.last_mut().unwrap() |= 1 << (self.bit % 8);
                }
                self.bit += 1;
            }
        }
        fn finish(mut self, suffix: &[u8]) -> Vec<u8> {
            self.bytes.extend_from_slice(suffix);
            self.bytes
        }
    }

    /// An engine with a stream over `input`, an output of `size` bytes, a
    /// zeroed state (window bits 15, header kind `78 xx`) and doubles for
    /// the callees outside this file. Answers (engine, stream, state, output).
    fn inflater(input: &[u8], size: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let b = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, b);
            }
            a[0].into_ret()
        });
        e.register(DATA_CHECKSUM, |_, a| {
            assert_eq!(a, [0, 0, 0]);
            1u32.into_ret()
        });
        e.register(HEADER_CHECKSUM, |e, a| {
            let bytes: Vec<u8> = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            if a[1] == 0 {
                0u32.into_ret()
            } else {
                header_sum(a[0], &bytes).into_ret()
            }
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1] * a[2]).into_ret());
        e.mem.map(0x010a_5000, 0x2000);
        let stream = e.mem.alloc(0x38);
        let state = e.mem.alloc(0x2528);
        let at = e.mem.alloc(input.len() as u32 + 1);
        e.mem.write(at, input);
        let output = e.mem.alloc(size);
        e.mem.set_u32(stream, at);
        e.mem.set_u32(stream + 4, input.len() as u32);
        e.mem.set_u32(stream + 0xc, output);
        e.mem.set_u32(stream + 0x10, size);
        e.mem.set_u32(stream + 0x1c, state);
        e.mem.set_u32(stream + 0x20, ALLOCATE);
        e.mem.set_u32(stream + 0x24, RELEASE);
        e.mem.set_u32(state + 8, 1);
        e.mem.set_u32(state + 0x1c, 15);
        (e, stream, state, output)
    }

    fn output_bytes(e: &Engine, output: u32, count: u32) -> Vec<u8> {
        (0..count).map(|i| e.mem.u8(output + i)).collect()
    }

    #[test]
    fn inflate_refuses_a_missing_stream_and_stops_without_progress() {
        let (mut e, stream, state, _) = inflater(&[], 16);
        assert_eq!(e.call(0x00b4_4000, &args![0u32, 0i32]).i32(), -2);
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream + 4, 1);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -2);
        e.mem.set_u32(stream + 4, 0);
        // No input: nothing happens, -5; an unknown mode answers -2.
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -5);
        assert_eq!(e.mem.u32(state), 0);
        e.mem.set_u32(state, 0x1d);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -2);
    }

    #[test]
    fn inflate_copies_a_stored_block() {
        let mut input = vec![0x78, 0x01, 0x01, 5, 0, 0xfa, 0xff];
        input.extend_from_slice(b"hello");
        input.extend_from_slice(&[9, 9, 9, 9]);
        let (mut e, stream, state, output) = inflater(&input, 16);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        assert_eq!(output_bytes(&e, output, 5), b"hello");
        let copies: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == MEMCPY)
            .map(|c| c.1)
            .collect();
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][2], 5);
        assert_eq!(e.mem.u32(state), 0x1a);
        assert_eq!((e.mem.u32(stream + 4), e.mem.u32(stream + 0x10)), (0, 11));
        assert_eq!(e.mem.u32(stream + 8), 16);
        assert_eq!(e.mem.u32(stream + 0x14), 5);
        assert_eq!(e.mem.u32(state + 0x18), 5);
        // The trailer is read, not compared; the checksum is cleared.
        assert_eq!((e.mem.u32(state + 0x14), e.mem.u32(stream + 0x30)), (0, 0));
        assert_eq!(e.mem.u32(stream + 0x2c), 0x40);
        // Again: the done mode answers 1 at once.
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
    }

    #[test]
    fn inflate_fails_bad_headers_and_lengths() {
        let (mut e, stream, state, _) = inflater(&[0x78, 0x02], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INCORRECT_HEADER_CHECK);
        assert_eq!(e.mem.u32(state), 0x1b);
        // A window too large for the state's 15 bits.
        let (mut e, stream, _, _) = inflater(&[0x88, 0x1c], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_WINDOW_SIZE);
        // A stored block whose lengths disagree.
        let (mut e, stream, _, _) = inflater(&[0x78, 0x01, 0x01, 5, 0, 0xfa, 0xfe], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_STORED_LENGTHS);
        // Block type 3.
        let (mut e, stream, _, _) = inflater(&[0x78, 0x01, 0x07], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_BLOCK_TYPE);
    }

    #[test]
    fn inflate_answers_2_for_a_dictionary() {
        let (mut e, stream, state, _) = inflater(&[0x78, 0xbb, 0x12, 0x34, 0x56, 0x78], 16);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 2);
        assert_eq!(e.mem.u32(state), 0x0a);
        assert_eq!(e.mem.u32(stream + 0x30), 0x1234_5678);
        assert_eq!(e.mem.u32(stream + 4), 0);
        // With a dictionary set the data checksum starts and the block
        // header waits for input; flush kind 5 stops at the block start.
        e.mem.set_u32(state + 0xc, 1);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 5i32]).i32(), -5);
        assert_eq!(e.mem.u32(state), 0x0b);
        assert_eq!(e.mem.u32(stream + 0x30), 1);
        assert_eq!(e.mem.u32(stream + 0x2c), 0x80);
    }

    #[test]
    fn inflate_reads_a_header_with_flags() {
        // 1f 8b, method 8, flags: header checksum, extra field, name.
        let mut input = vec![0x1f, 0x8b, 0x08, 0x0e, 1, 2, 3, 4, 0, 3, 2, 0, b'a', b'b'];
        input.extend_from_slice(b"nm\0");
        let mut sum = header_sum(0, &[0x1f, 0x8b]);
        sum = header_sum(sum, &[0x08, 0x0e]);
        sum = header_sum(sum, &[1, 2, 3, 4]);
        sum = header_sum(sum, &[0, 3]);
        sum = header_sum(sum, &[2, 0]);
        sum = header_sum(sum, b"ab");
        input.extend_from_slice(&[sum as u8, (sum >> 8) as u8]);
        let (mut e, stream, state, _) = inflater(&input, 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(state + 0x10), 0x0e08);
        // The header checksum matched; the next checksum starts at 0.
        assert_eq!(e.mem.u32(state), 0x0b);
        assert_eq!(e.mem.u32(stream + 0x30), 0);
        assert_eq!(e.mem.u32(stream + 8), input.len() as u32);
        // A wrong header checksum fails.
        let last = input.len() - 1;
        input[last] ^= 1;
        let (mut e, stream, state, _) = inflater(&input, 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_HEADER_CRC_MISMATCH);
        // Unknown flags.
        let (mut e, stream, state, _) = inflater(&[0x1f, 0x8b, 0x08, 0x20], 16);
        e.mem.set_u32(state + 8, 2);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_UNKNOWN_HEADER_FLAGS);
    }

    /// Synthetic fixed tables: a length code for every low byte of the
    /// index (literals below 0xf0; 0xf0 a length 3 with 2 extra bits; 0xf1
    /// a link to a one-bit sub-table at 0xf2 holding `A`/`B`; 0xfe invalid;
    /// 0xff the block end), and distance codes `d + 1` with one extra bit.
    fn fixed_tables(e: &mut Engine) {
        for i in 0..0x200u32 {
            let low = i & 0xff;
            let code = match low {
                0xf0 => 0x0003_0812,
                0xf1 => 0x00f2_0801,
                0xfe => 0x0000_0840,
                0xff => 0x0000_0820,
                _ => (low << 16) | 0x0800,
            };
            e.mem.set_u32(0x010a_5a98 + i * 4, code);
        }
        e.mem.set_u32(0x010a_5a98 + 0xf2 * 4, 0x0041_0100);
        e.mem.set_u32(0x010a_5a98 + 0xf3 * 4, 0x0042_0100);
        for d in 0..0x20u32 {
            e.mem.set_u32(0x010a_6298 + d * 4, ((d + 1) << 16) | 0x0511);
        }
    }

    /// `78 01`, a last fixed block: `h`, `i`, length 4 at distance 2, the
    /// sub-table's `B`, block end; then a trailer.
    fn fixed_stream() -> Vec<u8> {
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(u32::from(b'h'), 8);
        w.put(u32::from(b'i'), 8);
        w.put(0xf0, 8);
        w.put(1, 2);
        w.put(1, 5);
        w.put(0, 1);
        w.put(0xf1, 8);
        w.put(1, 1);
        w.put(0xff, 8);
        w.finish(&[1, 2, 3, 4])
    }

    #[test]
    fn inflate_decodes_a_fixed_block() {
        let input = fixed_stream();
        let (mut e, stream, state, output) = inflater(&input, 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        assert_eq!(output_bytes(&e, output, 7), b"hihihiB");
        assert_eq!(e.mem.u32(state), 0x1a);
        assert_eq!(e.mem.u32(stream + 0x14), 7);
        assert_eq!(e.mem.u32(stream + 0x10), 57);
        // An invalid length code fails.
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(0xfe, 8);
        let (mut e, stream, _, _) = inflater(&w.finish(&[]), 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_INVALID_LENGTH_CODE);
        // Too far back: distance 2 after one byte.
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b011, 3);
        w.put(u32::from(b'h'), 8);
        w.put(0xf0, 8);
        w.put(0, 2);
        w.put(1, 5);
        w.put(0, 1);
        let (mut e, stream, _, _) = inflater(&w.finish(&[]), 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -3);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_DISTANCE_TOO_FAR);
    }

    #[test]
    fn inflate_keeps_the_window_or_fails_without_one() {
        let input = fixed_stream();
        // Stopped after `h` and `i`: the output goes to a new window.
        let (mut e, stream, state, _) = inflater(&input[..5], 64);
        fixed_tables(&mut e);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        let window = e.mem.u32(state + 0x2c);
        assert_ne!(window, 0);
        assert_eq!(e.mem.u32(state + 0x20), 0x8000);
        assert_eq!(output_bytes(&e, window, 2), b"hi");
        // Without memory for it: the no-memory mode, -4, and -4 again.
        let (mut e, stream, state, _) = inflater(&input[..5], 64);
        fixed_tables(&mut e);
        e.register(ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -4);
        assert_eq!(e.mem.u32(state), 0x1c);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), -4);
    }

    #[test]
    fn inflate_hands_long_runs_to_the_fast_decoder() {
        let input = fixed_stream();
        let (mut e, stream, state, output) = inflater(&input, 0x200);
        fixed_tables(&mut e);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(FAST_DECODER, move |e, a| {
            let state = e.mem.u32(a[0] + 0x1c);
            log.borrow_mut().push((
                a.to_vec(),
                e.mem.u32(a[0] + 4),
                e.mem.u32(state + 0x34),
                e.mem.u32(a[0] + 0xc),
            ));
            // Writes one byte and ends the stream.
            let put = e.mem.u32(a[0] + 0xc);
            e.mem.set_u8(put, b'z');
            e.mem.set_u32(a[0] + 0xc, put + 1);
            let left = e.mem.u32(a[0] + 0x10);
            e.mem.set_u32(a[0] + 0x10, left - 1);
            e.mem.set_u32(state, 0x1a);
            Ret::default()
        });
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 1);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        // Called with the output left at the start, after the copies were
        // written back (block header taken: 3 of the byte's 8 bits used).
        assert_eq!(seen[0].0, vec![stream, 0x200]);
        assert_eq!(seen[0].1, input.len() as u32 - 3);
        assert_eq!(seen[0].2, 5);
        assert_eq!(seen[0].3, output);
        assert_eq!(e.mem.u8(output), b'z');
        assert_eq!(e.mem.u32(stream + 0x10), 0x1ff);
        assert_eq!(e.mem.u32(state), 0x1a);
    }

    /// `78 01`, a last coded block: 257 + 1 symbols, 4 code-length code
    /// lengths (1..4), then the code-length symbols `symbols` (two bits each
    /// in the test's code-length table, with their extra bits).
    fn coded_engine(symbols: &[(u32, u32, u32)]) -> (Engine, u32, u32, TableCalls) {
        let mut w = BitWriter::new(&[0x78, 0x01]);
        w.put(0b101, 3);
        w.put(0, 5);
        w.put(0, 5);
        w.put(0, 4);
        for length in 1..=4 {
            w.put(length, 3);
        }
        for &(symbol, extra, extra_bits) in symbols {
            w.put(symbol, 2);
            w.put(extra, extra_bits);
        }
        w.put(0, 1);
        let (mut e, stream, state, _) = inflater(&w.finish(&[]), 64);
        for i in 0..0x13u32 {
            e.mem.set_u16(0x010a_6318 + i * 2, i as u16);
        }
        let calls: TableCalls = Default::default();
        let log = calls.clone();
        e.register_double(TABLE_BUILDER, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let next = e.mem.u32(a[3]);
            match a[0] {
                // Code lengths: index & 3 -> length 8, repeat, zeros, zeros.
                0 => {
                    for i in 0..0x80u32 {
                        let value = [8u32, 16, 17, 18][(i & 3) as usize];
                        e.mem.set_u32(next + i * 4, (value << 16) | 0x0200);
                    }
                }
                // Lengths: every code ends the block.
                1 => {
                    for i in 0..0x200u32 {
                        e.mem.set_u32(next + i * 4, 0x0000_0120);
                    }
                    e.mem.set_u32(a[3], next + 0x800);
                }
                _ => {}
            }
            Ret::default()
        });
        (e, stream, state, calls)
    }

    type TableCalls = std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>;

    #[test]
    fn inflate_builds_the_tables_of_a_coded_block() {
        // 8, repeat 6, 138 zeros, 113 zeros: 258 lengths.
        let (mut e, stream, state, calls) =
            coded_engine(&[(0, 0, 0), (1, 3, 2), (3, 127, 7), (3, 102, 7)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        let lengths = |e: &Engine, i: u32| e.mem.u16(state + 0x68 + i * 2);
        // The code-length code lengths, in the test's order.
        let calls = calls.borrow();
        assert_eq!(
            calls[0],
            vec![
                0,
                state + 0x68,
                0x13,
                state + 0x64,
                state + 0x4c,
                state + 0x2e8
            ]
        );
        assert_eq!(
            calls[1],
            vec![
                1,
                state + 0x68,
                0x101,
                state + 0x64,
                state + 0x4c,
                state + 0x2e8
            ]
        );
        assert_eq!(
            calls[2],
            vec![
                2,
                state + 0x68 + 0x202,
                1,
                state + 0x64,
                state + 0x50,
                state + 0x2e8
            ]
        );
        for i in 0..7 {
            assert_eq!(lengths(&e, i), 8);
        }
        assert_eq!(lengths(&e, 7), 0);
        assert_eq!(e.mem.u32(state + 0x60), 0x102);
        assert_eq!(e.mem.u32(state + 0x44), state + 0x528);
        assert_eq!(e.mem.u32(state + 0x48), state + 0x528 + 0x800);
        assert_eq!((e.mem.u32(state + 0x4c), e.mem.u32(state + 0x50)), (9, 6));
        // The block ended; the trailer is still to come.
        assert_eq!(e.mem.u32(state), 0x18);
    }

    #[test]
    fn inflate_goes_on_after_a_repeat_without_a_length() {
        // A repeat first: the message is left, yet the tables are built and
        // decoding goes on, as in the exe.
        let (mut e, stream, state, calls) = coded_engine(&[(1, 0, 2)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_REPEAT_WITHOUT_LENGTH);
        assert_eq!(calls.borrow().len(), 3);
        assert_eq!(e.mem.u32(state), 0x18);
        // A repeat past the 258 lengths.
        let (mut e, stream, _, _) = coded_engine(&[(0, 0, 0), (3, 127, 7), (3, 127, 7)]);
        assert_eq!(e.call(0x00b4_4000, &args![stream, 0i32]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0x18), MSG_REPEAT_TOO_LONG);
    }
}
