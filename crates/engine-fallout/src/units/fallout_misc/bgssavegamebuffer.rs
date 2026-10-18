//! `fallout/misc/saveload/bgssavegamebuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSSaveGameBuffer` (Xbox PDB), PC fields verified in 00865bd0.
    pub struct BGSSaveGameBuffer: 0x14 {
        0x04 pBuffer: Ptr<u8>,
        0x08 iBufferSize: u32,
        0x0c iBufferPosition: u32,
        0x10 iDebugOverhead: u32,
    }
}

// Translated from 00865bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::BGSSaveGameBuffer` (Xbox PDB): installs the PC
/// vtable and initializes all four data words to zero; returns this.
pub fn construct(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>) -> Ptr<BGSSaveGameBuffer> {
    e.mem.set_u32(this.addr(), 0x01082324);
    for offset in [4, 8, 12, 16] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    this
}

// Translated from 00865ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes at an explicit position, optionally appending the native '|'
/// separator. Capacity/position comparisons are unsigned with wrapping
/// arithmetic. Mode and buffer fields are reloaded across callbacks.
pub fn write_at(
    e: &mut Engine,
    this: Ptr<BGSSaveGameBuffer>,
    source: Ptr,
    size: u32,
    position: u32,
    no_separator: u8,
) {
    let end = position
        .wrapping_add(size)
        .wrapping_add(u32::from(no_separator == 0));
    while end > e.get(this, BGSSaveGameBuffer::iBufferSize) {
        e.call(0x008660b0, &args![this]);
    }
    let state = e.mem.u32(0x011de134);
    if e.call(0x00424940, &args![state]).u8() == 0 {
        let destination = e
            .get(this, BGSSaveGameBuffer::pBuffer)
            .addr()
            .wrapping_add(position);
        e.call(0x00401460, &args![destination, source, size]);
    }
    if no_separator == 0 {
        let state = e.mem.u32(0x011de134);
        if e.call(0x00424940, &args![state]).u8() == 0 {
            let buffer = e.get(this, BGSSaveGameBuffer::pBuffer).addr();
            e.mem
                .set_u8(buffer.wrapping_add(position).wrapping_add(size), b'|');
        }
        if position == e.get(this, BGSSaveGameBuffer::iBufferPosition) {
            let overhead = e
                .get(this, BGSSaveGameBuffer::iDebugOverhead)
                .wrapping_add(1);
            e.set(this, BGSSaveGameBuffer::iDebugOverhead, overhead);
        }
    }
    if e.get(this, BGSSaveGameBuffer::iBufferPosition) < end {
        e.set(this, BGSSaveGameBuffer::iBufferPosition, end);
    }
}

// Translated from 00865e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves with a separator. PC consumes three stack words (RET 0xc),
/// ignores the third, and always forwards zero to the append wrapper.
pub fn save_bytes(
    e: &mut Engine,
    this: Ptr<BGSSaveGameBuffer>,
    source: Ptr,
    size: u32,
    _unused: u32,
) {
    e.call(0x00865570, &args![this, source, size, 0u8]);
}

// Translated from 00865f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::StartVariableSizedValue` (Xbox PDB): writes '#|'
/// and returns the position captured before writing.
pub fn start_variable_value(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>) -> u32 {
    let start = e.get(this, BGSSaveGameBuffer::iBufferPosition);
    e.with_stack(4, |e, marker| {
        e.mem.set_u8(marker.addr(), b'#');
        let position = e.get(this, BGSSaveGameBuffer::iBufferPosition);
        write_at(e, this, marker, 1, position, 0);
    });
    start
}

// Translated from 00866140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts space before existing bytes. The PC uses signed comparisons here,
/// unlike write_at, and retains the original end across callbacks.
pub fn insert_space(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, position: u32, extra: u32) {
    let end = e
        .get(this, BGSSaveGameBuffer::iBufferPosition)
        .wrapping_add(extra);
    if end as i32 <= 0 {
        return;
    }
    while (end as i32) > (e.get(this, BGSSaveGameBuffer::iBufferSize) as i32) {
        e.call(0x008660b0, &args![this]);
    }
    let state = e.mem.u32(0x011de134);
    if e.call(0x00424940, &args![state]).u8() == 0 {
        let buffer = e.get(this, BGSSaveGameBuffer::pBuffer).addr();
        let source = buffer.wrapping_add(position);
        let destination = source.wrapping_add(extra);
        let capacity = e
            .get(this, BGSSaveGameBuffer::iBufferSize)
            .wrapping_sub(position.wrapping_add(extra));
        let size = e
            .get(this, BGSSaveGameBuffer::iBufferPosition)
            .wrapping_sub(position);
        e.call(0x008661e0, &args![destination, capacity, source, size]);
    }
    e.set(this, BGSSaveGameBuffer::iBufferPosition, end);
}

// Translated from 00865ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveVariableSizedValue_ov2` (Xbox PDB): patches the
/// placeholder using one, two or four tagged bytes followed by '|'.
pub fn finish_variable_value(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, value: u32, start: u32) {
    e.with_stack(4, |e, encoded| {
        if value < 0x40 {
            e.mem.set_u8(encoded.addr(), (value << 2) as u8);
            write_at(e, this, encoded, 1, start, 0);
        } else if value < 0x4000 {
            insert_space(e, this, start, 1);
            e.mem.set_u16(encoded.addr(), ((value << 2) | 1) as u16);
            write_at(e, this, encoded, 2, start, 0);
        } else if value < 0x40000000 {
            insert_space(e, this, start, 3);
            e.mem.set_u32(encoded.addr(), (value << 2) | 2);
            write_at(e, this, encoded, 4, start, 0);
        } else {
            e.call(0x00845cc0, &args![0x0107ee00u32, value]);
        }
    });
}

// Translated from 00865db0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `BGSSaveGameBuffer::SaveFormID` (Xbox PDB): encode the numeric form ID
/// and append its three bytes plus separator. The second stack word is unused.
pub fn save_numeric_form_id(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, id: u32, _unused: u32) {
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        e.call(0x00853570, &args![index, id]);
        e.call(0x00865570, &args![this, index, 3u32, 0u8]);
    });
}
// Translated from 00865df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveGameBuffer::SaveFormID_ov2` (Xbox PDB): writes a three-byte
/// numeric index for a form (zero for null). The second stack word is
/// consumed but unused by the PC body.
pub fn save_form_id(e: &mut Engine, this: Ptr<BGSSaveGameBuffer>, form: Ptr, _unused: u32) {
    let id = if form.is_null() {
        0
    } else {
        e.call(0x0084e3a0, &args![form]).u32()
    };
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        e.call(0x00853570, &args![index, id]);
        e.call(0x00865570, &args![this, index, 3u32, 0u8]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00865db0,
            save_numeric_form_id(Ptr<BGSSaveGameBuffer>, u32, u32)
        ),
        entry!(0x00865df0, save_form_id(Ptr<BGSSaveGameBuffer>, Ptr, u32)),
        entry!(0x00866140, insert_space(Ptr<BGSSaveGameBuffer>, u32, u32)),
        entry!(
            0x00865ff0,
            finish_variable_value(Ptr<BGSSaveGameBuffer>, u32, u32)
        ),
        entry!(
            0x00865bd0,
            construct(Ptr<BGSSaveGameBuffer>) -> Ptr<BGSSaveGameBuffer>
        ),
        entry!(
            0x00865ce0,
            write_at(Ptr<BGSSaveGameBuffer>, Ptr, u32, u32, u8)
        ),
        entry!(
            0x00865e50,
            save_bytes(Ptr<BGSSaveGameBuffer>, Ptr, u32, u32)
        ),
        entry!(
            0x00865f20,
            start_variable_value(Ptr<BGSSaveGameBuffer>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_form_writer_matches_native_created_id_vectors() {
        for (id, encoded) in [
            (0xff00_0000, [0x80, 0, 0]),
            (0xff12_3456, [0x92, 0x34, 0x56]),
            (0xff7f_ffff, [0xff, 0xff, 0xff]),
        ] {
            for unused in [0, 0xdead_beef] {
                let mut e = Engine::new();
                for address in [0x011de134, 0x011ddf38, 0x011c3f2c] {
                    e.map(address, 4);
                }
                let base = e.mem.alloc(256);
                e.mem.set_u32(0x011de134, base);
                e.mem.set_u32(0x011ddf38, base);
                e.mem.set_u32(0x011c3f2c, 0);
                let mut bytes = vec![0xa5; 256];
                bytes[12] = 0;
                for (offset, word) in [
                    (8, 0),
                    (0x24, base + 0x60),
                    (0x28, 64),
                    (0x2c, 5),
                    (0x30, 7),
                ] {
                    bytes[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
                }
                e.mem.write(base, &bytes);
                bytes[0x65..0x68].copy_from_slice(&encoded);
                bytes[0x68] = b'|';
                bytes[0x2c..0x30].copy_from_slice(&9u32.to_le_bytes());
                bytes[0x30..0x34].copy_from_slice(&8u32.to_le_bytes());
                e.call(0x00865db0, &[base + 0x20, id, unused]);
                assert_eq!(e.mem.bytes(base, 256), bytes);
            }
        }
    }

    #[test]
    fn numeric_form_writer_initializes_then_encodes_and_appends_exact_three_bytes() {
        for id in [0, 0x1234_5678] {
            let mut e = Engine::new();
            let buffer = e.new_object::<BGSSaveGameBuffer>();
            e.register_double(0x0084eaf0, |e, a| {
                assert_eq!(a.len(), 1);
                e.mem.write(a[0], &[0, 0, 0, 0xa5]);
                Ret::default()
            });
            e.register_double(0x00853570, move |e, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[1], id);
                assert_eq!(e.mem.bytes(a[0], 4), [0, 0, 0, 0xa5]);
                e.mem.write(a[0], &[0x12, 0x34, 0x56]);
                Ret::default()
            });
            e.register_double(0x00865570, move |e, a| {
                assert_eq!(a.len(), 4);
                assert_eq!(a[0], buffer.addr());
                assert_eq!(&a[2..], &[3, 0]);
                assert_eq!(e.mem.bytes(a[1], 4), [0x12, 0x34, 0x56, 0xa5]);
                Ret::default()
            });
            e.call_log = Some(Vec::new());
            e.call(0x00865db0, &args![buffer, id, u32::MAX]);
            let calls = e
                .call_log
                .unwrap()
                .into_iter()
                .map(|(address, _)| address)
                .collect::<Vec<_>>();
            assert_eq!(calls, [0x00865db0, 0x0084eaf0, 0x00853570, 0x00865570]);
        }
    }
    fn fixture() -> (Engine, Ptr<BGSSaveGameBuffer>, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011de134, 4);
        let this = e.new_object::<BGSSaveGameBuffer>();
        construct(&mut e, this);
        let bytes = e.mem.alloc(64);
        e.mem.write(bytes, &[0xcc; 64]);
        e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(bytes));
        e.set(this, BGSSaveGameBuffer::iBufferSize, 64);
        let mode = e.mem.alloc(16);
        e.mem.set_u8(mode + 12, 0);
        e.set_global(0x011de134, mode);
        // Generated copy dependency; these tests do not claim full save-game
        // startup coverage. Checked movement uses the real translated wrapper.
        e.register_double(0x00401460, |e, a| {
            assert_eq!(a.len(), 3);
            let data = e.mem.bytes(a[1], a[2]).to_vec();
            e.mem.write(a[0], &data);
            a[0].into_ret()
        });
        (e, this, bytes, mode)
    }

    #[test]
    fn save_form_id_reads_form_once_and_writes_three_encoded_bytes_for_null_too() {
        let (mut e, this, bytes, _) = fixture();
        let form = e.mem.alloc(16);
        e.mem.set_u32(form + 12, 0x12345678);
        e.register_double(0x00853570, move |e, a| {
            assert_eq!(a.len(), 2);
            assert_eq!(e.mem.bytes(a[0], 3), [0, 0, 0]);
            if a[1] == 0x12345678 {
                e.mem.set_u32(form + 12, 0xaabbccdd);
                e.mem.write(a[0], &[0x31, 0x42, 0x53]);
            } else {
                assert_eq!(a[1], 0);
                e.mem.write(a[0], &[0, 0, 7]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00865df0, &args![this, form, u32::MAX]);
        e.call(0x00865df0, &args![this, 0u32, 0x1234u32]);
        assert_eq!(
            e.mem.bytes(bytes, 8),
            [0x31, 0x42, 0x53, b'|', 0, 0, 7, b'|']
        );
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 8);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 2);
        let reads: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0084e3a0)
            .map(|(_, arguments)| arguments.clone())
            .collect();
        assert_eq!(reads, [vec![form]]);
    }

    #[test]
    fn append_wrappers_and_positioned_overwrite_preserve_cursor_and_overhead() {
        let (mut e, this, bytes, _) = fixture();
        let source = e.mem.alloc(3);
        e.mem.write(source, b"abc");
        e.call(0x00865e50, &args![this, source, 2u32, 0xffffffffu32]);
        assert_eq!(e.mem.bytes(bytes, 4), b"ab|\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        e.call(0x00865ce0, &args![this, source + 2, 1u32, 0u32, 0u8]);
        assert_eq!(e.mem.bytes(bytes, 4), b"c||\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        e.call(0x00865570, &args![this, source, 1u32, 0x101u32]);
        assert_eq!(e.mem.bytes(bytes, 5), b"c||a\xcc");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 4);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
    }

    #[test]
    fn positioned_write_reloads_mode_buffer_cursor_and_overhead_after_copy() {
        let (mut e, this, bytes, first_mode) = fixture();
        let moved = e.mem.alloc(64);
        e.mem.write(moved, &[0xdd; 64]);
        let second_mode = e.mem.alloc(16);
        let source = e.mem.alloc(2);
        e.mem.write(source, b"xy");
        e.register_double(0x00424940, move |_, a| {
            assert!(a == [first_mode] || a == [second_mode]);
            0u32.into_ret()
        });
        e.register_double(0x00401460, move |e, a| {
            assert_eq!(a, &[bytes, source, 2]);
            e.mem.write(bytes, b"xy");
            e.set_global(0x011de134, second_mode);
            e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(moved));
            e.set(this, BGSSaveGameBuffer::iBufferPosition, 0);
            e.set(this, BGSSaveGameBuffer::iDebugOverhead, u32::MAX);
            Ret::default()
        });
        e.set(this, BGSSaveGameBuffer::iBufferPosition, 55);
        e.call_log = Some(vec![]);
        e.call(0x00865ce0, &args![this, source, 2u32, 0u32, 0u8]);
        assert_eq!(e.mem.bytes(bytes, 3), b"xy\xcc");
        assert_eq!(e.mem.bytes(moved, 3), b"\xdd\xdd|");
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 3);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 0);
        let calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00424940)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(calls, vec![vec![first_mode], vec![second_mode]]);
    }

    #[test]
    fn sizing_mode_grows_without_touching_source_or_buffer_and_wraps_end() {
        let (mut e, this, bytes, mode) = fixture();
        e.mem.set_u8(mode + 12, 1);
        e.set(this, BGSSaveGameBuffer::iBufferSize, 0);
        e.call(
            0x00865ce0,
            &args![this, 0xdead0000u32, 0x10000u32, 0u32, 0u8],
        );
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferSize), 0x20000);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 0x10001);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        assert_eq!(e.mem.bytes(bytes, 64), [0xcc; 64]);
        e.call(0x00865ce0, &args![this, 0u32, 2u32, u32::MAX, 1u8]);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 0x10001);
    }

    #[test]
    fn variable_value_boundaries_shift_existing_payload_and_preserve_overhead() {
        for (value, encoded) in [
            (0u32, vec![0]),
            (0x3f, vec![0xfc]),
            (0x40, vec![1, 1]),
            (0x3fff, vec![0xfd, 0xff]),
            (0x4000, vec![2, 0, 1, 0]),
            (0x3fffffff, vec![0xfe, 0xff, 0xff, 0xff]),
        ] {
            let (mut e, this, bytes, _) = fixture();
            let start = e.call(0x00865f20, &args![this]).u32();
            assert_eq!(start, 0);
            assert_eq!(e.mem.bytes(bytes, 2), b"#|");
            let source = e.mem.alloc(2);
            e.mem.write(source, b"ab");
            e.call(0x00865570, &args![this, source, 2u32, 1u8]);
            e.call(0x00865ff0, &args![this, value, start]);
            let mut expected = encoded;
            expected.extend_from_slice(b"|ab");
            assert_eq!(e.mem.bytes(bytes, expected.len() as u32), expected);
            assert_eq!(
                e.get(this, BGSSaveGameBuffer::iBufferPosition),
                expected.len() as u32
            );
            assert_eq!(e.get(this, BGSSaveGameBuffer::iDebugOverhead), 1);
        }
    }

    #[test]
    fn invalid_variable_value_reports_exact_format_and_value_without_writing() {
        for value in [0x40000000u32, u32::MAX] {
            let (mut e, this, bytes, _) = fixture();
            e.register_double(0x00845cc0, move |_, a| {
                assert_eq!(a, &[0x0107ee00, value]);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00865ff0, &args![this, value, 0u32]);
            assert_eq!(e.mem.bytes(bytes, 64), [0xcc; 64]);
            assert_eq!(
                e.call_log
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|(address, _)| *address == 0x00845cc0)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn insertion_rejects_nonpositive_signed_end_before_callbacks() {
        for (position, extra) in [(0u32, 0u32), (0x7fffffff, 1), (u32::MAX, 1)] {
            let (mut e, this, _, _) = fixture();
            e.set(this, BGSSaveGameBuffer::iBufferPosition, position);
            e.register_double(0x00424940, |_, _| panic!("early exit must not query mode"));
            e.call(0x00866140, &args![this, 0u32, extra]);
            assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), position);
        }
    }

    #[test]
    fn insertion_uses_fresh_fields_but_commits_end_captured_before_mode_callback() {
        let (mut e, this, _, _) = fixture();
        let moved = e.mem.alloc(20);
        e.mem.write(moved, b"abcdefghijklmnopqrst");
        e.set(this, BGSSaveGameBuffer::iBufferPosition, 6);
        e.register_double(0x00424940, move |e, _| {
            e.set(this, BGSSaveGameBuffer::pBuffer, Ptr::new(moved));
            e.set(this, BGSSaveGameBuffer::iBufferSize, 20);
            e.set(this, BGSSaveGameBuffer::iBufferPosition, 10);
            0u32.into_ret()
        });
        e.register_double(0x00ec7ce3, move |_, a| {
            assert_eq!(a, &[moved + 5, 15, moved + 2, 8]);
            0u32.into_ret()
        });
        e.call(0x00866140, &args![this, 2u32, 3u32]);
        assert_eq!(e.get(this, BGSSaveGameBuffer::iBufferPosition), 9);
    }
}
