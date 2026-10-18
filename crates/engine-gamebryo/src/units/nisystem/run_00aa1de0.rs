//! `NiSystem/run_00aa1de0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::NiBinaryStream;

/// Calls the stream's function pointer at `member` (`m_pfnRead` +8 or
/// `m_pfnWrite` +0xc) once as the game does:
/// `function(stream, buffer, 1, &count, 1)` with `count` a local set to 1 first.
/// Adds the byte count the function returns to the stream's position and
/// returns it.
fn transfer_one_byte(
    e: &mut Engine,
    stream: Ptr<NiBinaryStream>,
    member: u32,
    buffer: u32,
    count: Ptr,
) -> u32 {
    e.mem.set_u32(count.addr(), 1);
    let function = e.mem.u32(stream.addr().wrapping_add(member));
    let done = e
        .call(function, &args![stream, buffer, 1u32, count, 1u32])
        .u32();
    let position = e.get(stream, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        stream,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
    done
}

// Translated from 00aa1f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reads one line from a binary stream into `buffer`
/// (`thiscall`, `RET 8`: `stream`, `buffer`, `size`). Reads byte by byte
/// through the stream's read function; stops at a line feed (not stored),
/// skips carriage returns, stops when `size - 1` characters are stored or when
/// a read does not give exactly one byte, and terminates the text with 0.
/// Returns the number of bytes read from the stream (the characters, the
/// skipped carriage returns and the line feed). With a `size` below 2 it stores
/// only the terminator and reads nothing.
pub fn fn_00aa1f10(e: &mut Engine, stream: Ptr<NiBinaryStream>, buffer: Ptr, size: u32) -> i32 {
    if size <= 1 {
        e.mem.set_u8(buffer.addr(), 0);
        return 0;
    }
    e.with_stack(8, |e, locals| {
        let count = locals.byte_add(4);
        let mut total = 0u32;
        let mut stored = 0u32;
        loop {
            let done = transfer_one_byte(e, stream, 0x08, locals.addr(), count);
            total = total.wrapping_add(done);
            if done != 1 {
                break;
            }
            let c = e.mem.u8(locals.addr());
            if c == 0x0a {
                break;
            }
            if c != 0x0d {
                e.mem.set_u8(buffer.addr().wrapping_add(stored), c);
                stored += 1;
            }
            if stored + 1 >= size {
                break;
            }
        }
        e.mem.set_u8(buffer.addr().wrapping_add(stored), 0);
        total as i32
    })
}

// Translated from 00aa1fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: writes the characters of the C string `text` to a binary
/// stream, without the terminator (`thiscall`, `RET 4`). Writes byte by byte
/// through the stream's write function and stops at the first write that does not
/// give exactly one byte. Returns the number of bytes written.
pub fn fn_00aa1fd0(e: &mut Engine, stream: Ptr<NiBinaryStream>, text: Ptr) -> i32 {
    e.with_stack(4, |e, count| {
        let mut written = 0i32;
        let mut at = text.addr();
        while e.mem.u8(at) != 0 {
            let done = transfer_one_byte(e, stream, 0x0c, at, count);
            at = at.wrapping_add(1);
            if done != 1 {
                break;
            }
            written += 1;
        }
        written
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_1f10,
            fn_00aa1f10(Ptr<NiBinaryStream>, Ptr, u32) -> i32
        ),
        entry!(0x00aa_1fd0, fn_00aa1fd0(Ptr<NiBinaryStream>, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const READ: u32 = 0x00aa_2e60;
    const WRITE: u32 = 0x00aa_2ea0;

    type Input = Rc<RefCell<VecDeque<u8>>>;
    type Output = Rc<RefCell<Vec<u8>>>;
    type Fixture = (Engine, Ptr<NiBinaryStream>, Input, Output);

    /// A stream whose read function hands out `input` one byte at a time
    /// (0 bytes read when it is empty) and whose write function collects the
    /// bytes, accepting at most `accept` of them.
    fn fixture(input: &[u8], accept: usize) -> Fixture {
        let mut e = Engine::new();
        let stream = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 8, READ);
        e.mem.set_u32(stream.addr() + 0xc, WRITE);
        let queue: Input = Rc::new(RefCell::new(input.iter().copied().collect()));
        let sink: Output = Rc::default();
        let source = queue.clone();
        e.register_double(READ, move |e, a| {
            // (stream, buffer, 1, &count, 1)
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            match source.borrow_mut().pop_front() {
                Some(byte) => {
                    e.mem.set_u8(a[1], byte);
                    1u32.into_ret()
                }
                None => 0u32.into_ret(),
            }
        });
        let collected = sink.clone();
        e.register_double(WRITE, move |e, a| {
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            if collected.borrow().len() >= accept {
                return 0u32.into_ret();
            }
            collected.borrow_mut().push(e.mem.u8(a[1]));
            1u32.into_ret()
        });
        (e, stream, queue, sink)
    }

    fn read_line(input: &[u8], size: u32) -> (i32, Vec<u8>, u32, Vec<u8>) {
        let (mut e, stream, queue, _) = fixture(input, 0);
        let buffer = e.mem.alloc(0x40);
        e.mem.write(buffer, &[0x55; 0x40]);
        let read = e.call(0x00aa_1f10, &args![stream, buffer, size]).i32();
        let left: Vec<u8> = queue.borrow().iter().copied().collect();
        let position = e.mem.u32(stream.addr() + 4);
        (read, e.mem.cstr(buffer), position, left)
    }

    #[test]
    fn read_line_stops_at_the_line_feed_and_skips_carriage_returns() {
        let (read, text, position, left) = read_line(b"ab\r\ncd", 0x20);
        assert_eq!(read, 4);
        assert_eq!(text, b"ab");
        assert_eq!(position, 4);
        assert_eq!(left, b"cd");
    }

    #[test]
    fn read_line_stops_when_the_buffer_is_full() {
        let (read, text, _, left) = read_line(b"abcdef", 4);
        assert_eq!(read, 3);
        assert_eq!(text, b"abc");
        assert_eq!(left, b"def");
    }

    #[test]
    fn read_line_ends_at_the_end_of_the_input() {
        let (read, text, position, _) = read_line(b"xy", 0x20);
        assert_eq!(read, 2);
        assert_eq!(text, b"xy");
        assert_eq!(position, 2);
    }

    #[test]
    fn read_line_with_a_tiny_buffer_only_terminates() {
        let (read, text, _, left) = read_line(b"abc", 1);
        assert_eq!(read, 0);
        assert!(text.is_empty());
        assert_eq!(left, b"abc");
    }

    #[test]
    fn write_string_writes_the_characters_without_the_terminator() {
        let (mut e, stream, _, sink) = fixture(b"", usize::MAX);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"NIF");
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 3);
        assert_eq!(*sink.borrow(), b"NIF");
        assert_eq!(e.mem.u32(stream.addr() + 4), 3);
    }

    #[test]
    fn write_string_stops_at_the_first_short_write() {
        let (mut e, stream, _, sink) = fixture(b"", 2);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"NIF");
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 2);
        assert_eq!(*sink.borrow(), b"NI");
        assert_eq!(e.mem.u32(stream.addr() + 4), 2);
    }

    #[test]
    fn write_string_of_the_empty_string_writes_nothing() {
        let (mut e, stream, _, sink) = fixture(b"", usize::MAX);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00aa_1fd0, &args![stream, text]).i32(), 0);
        assert!(sink.borrow().is_empty());
    }
}
