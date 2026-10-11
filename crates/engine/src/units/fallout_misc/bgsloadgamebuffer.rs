//! `fallout/misc/saveload/bgsloadgamebuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSLoadGameBuffer` (Xbox PDB), PC fields verified in 008646b0.
    pub struct BGSLoadGameBuffer: 0x10 {
        0x04 pBuffer: Ptr<u8>,
        0x08 iBufferSize: u32,
        0x0c iBufferPosition: u32,
    }
}

// Translated from 008646b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Installs the PC vtable and clears the three buffer words.
pub fn construct(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>) -> Ptr<BGSLoadGameBuffer> {
    e.mem.set_u32(this.addr(), 0x01082048);
    for offset in [4, 8, 12] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    this
}

// Translated from 008647f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches an existing buffer and cursor without taking ownership.
pub fn set_buffer(
    e: &mut Engine,
    this: Ptr<BGSLoadGameBuffer>,
    buffer: Ptr<u8>,
    size: u32,
    position: u32,
) {
    e.set(this, BGSLoadGameBuffer::pBuffer, buffer);
    e.set(this, BGSLoadGameBuffer::iBufferSize, size);
    e.set(this, BGSLoadGameBuffer::iBufferPosition, position);
}

// Translated from 00864820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the requested bytes and consumes the following separator. Native
/// code checks only whether the size field is zero; it does not bounds-check
/// this read. Cursor and buffer fields are reloaded after callbacks.
pub fn read_data(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>, destination: Ptr, size: u32) {
    if e.get(this, BGSLoadGameBuffer::iBufferSize) == 0 {
        return;
    }
    let source = e
        .get(this, BGSLoadGameBuffer::pBuffer)
        .addr()
        .wrapping_add(e.get(this, BGSLoadGameBuffer::iBufferPosition));
    e.call(0x00401460, &args![destination, source, size]);
    let position = e
        .get(this, BGSLoadGameBuffer::iBufferPosition)
        .wrapping_add(size);
    e.set(this, BGSLoadGameBuffer::iBufferPosition, position);
    let separator = e
        .get(this, BGSLoadGameBuffer::pBuffer)
        .addr()
        .wrapping_add(position);
    if e.mem.u8(separator) != b'|' {
        e.call(0x00845cc0, &args![0x0107ee40u32]);
    }
    let position = e
        .get(this, BGSLoadGameBuffer::iBufferPosition)
        .wrapping_add(1);
    e.set(this, BGSLoadGameBuffer::iBufferPosition, position);
}

// Translated from 008648a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a three-byte `FormIDIndex` (Xbox PDB) and resolves the saved ID.
pub fn load_form_id(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>) -> u32 {
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        read_data(e, this, index, 3);
        e.call(0x00853500, &args![index]).u32()
    })
}

// Translated from 008648e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// `BGSLoadGameBuffer::LoadFormID_ov2` (Xbox PDB): stores the resolved form
/// ID and reports a nonempty saved index that failed to resolve.
pub fn load_form_id_into(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>, output: Ptr) -> bool {
    e.with_stack(4, |e, index| {
        e.call(0x0084eaf0, &args![index]);
        read_data(e, this, index, 3);
        let form = e.call(0x00853500, &args![index]).u32();
        e.mem.set_u32(output.addr(), form);
        !compact_id_is_zero(e, index) && e.mem.u32(output.addr()) == 0
    })
}

// Translated from 00864940 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Tests the three compact-index bytes in order, stopping at the first nonzero byte.
pub fn compact_id_is_zero(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr()) == 0
        && e.mem.u8(this.addr().wrapping_add(1)) == 0
        && e.mem.u8(this.addr().wrapping_add(2)) == 0
}

// Translated from 00864980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Two-word raw-data wrapper (RET 8).
pub fn load_raw(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>, destination: Ptr, size: u32) {
    read_data(e, this, destination, size);
}

// Translated from 00864a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Low two tag bits choose a one-, two- or four-byte unsigned value.
/// Tag three returns zero without advancing; each destination starts zero.
pub fn load_variable(e: &mut Engine, this: Ptr<BGSLoadGameBuffer>) -> u32 {
    let source = e
        .get(this, BGSLoadGameBuffer::pBuffer)
        .addr()
        .wrapping_add(e.get(this, BGSLoadGameBuffer::iBufferPosition));
    let size = match e.mem.u8(source) & 3 {
        0 => 1,
        1 => 2,
        2 => 4,
        _ => return 0,
    };
    e.with_stack(4, |e, value| {
        match size {
            1 => e.mem.set_u8(value.addr(), 0),
            2 => e.mem.set_u16(value.addr(), 0),
            _ => e.mem.set_u32(value.addr(), 0),
        }
        read_data(e, this, value, size);
        let encoded = match size {
            1 => u32::from(e.mem.u8(value.addr())),
            2 => u32::from(e.mem.u16(value.addr())),
            _ => e.mem.u32(value.addr()),
        };
        encoded >> 2
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x008646b0,
            construct(Ptr<BGSLoadGameBuffer>) -> Ptr<BGSLoadGameBuffer>
        ),
        entry!(
            0x008647f0,
            set_buffer(Ptr<BGSLoadGameBuffer>, Ptr<u8>, u32, u32)
        ),
        entry!(0x00864820, read_data(Ptr<BGSLoadGameBuffer>, Ptr, u32)),
        entry!(0x008648a0, load_form_id(Ptr<BGSLoadGameBuffer>) -> u32),
        entry!(
            0x008648e0,
            load_form_id_into(Ptr<BGSLoadGameBuffer>, Ptr) -> bool
        ),
        entry!(0x00864940, compact_id_is_zero(Ptr) -> bool),
        entry!(0x00864980, load_raw(Ptr<BGSLoadGameBuffer>, Ptr, u32)),
        entry!(0x00864a60, load_variable(Ptr<BGSLoadGameBuffer>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_zero_predicate_matches_native_guarded_vectors() {
        // Eight native00864940 cases, AL result, RET0, all eleven bytes unchanged.
        for mask in 0..8 {
            let mut e = Engine::new();
            let block = e.mem.alloc(11);
            let mut bytes = vec![0xa5; 11];
            bytes[4] = if mask & 1 != 0 { 0x80 } else { 0 };
            bytes[5] = if mask & 2 != 0 { 0x7f } else { 0 };
            bytes[6] = if mask & 4 != 0 { 0xff } else { 0 };
            e.mem.write(block, &bytes);
            assert_eq!(e.call(0x00864940, &[block + 4]).bool(), mask == 0);
            assert_eq!(e.mem.bytes(block, 11), bytes);
        }
    }

    #[test]
    fn form_id_output_load_matches_native_created_ids() {
        for (encoded, expected) in [
            ([0x80, 0, 0], 0xff00_0000u32),
            ([0x92, 0x34, 0x56], 0xff12_3456),
            ([0xff, 0xff, 0xff], 0xff7f_ffff),
        ] {
            let mut e = Engine::new();
            let block = e.mem.alloc(96);
            let mut bytes = vec![0xa5; 96];
            bytes[8..12].copy_from_slice(&(block + 48).to_le_bytes());
            bytes[12..16].copy_from_slice(&4u32.to_le_bytes());
            bytes[16..20].copy_from_slice(&0u32.to_le_bytes());
            bytes[48..51].copy_from_slice(&encoded);
            bytes[51] = b'|';
            e.mem.write(block, &bytes);
            bytes[16..20].copy_from_slice(&4u32.to_le_bytes());
            bytes[64..68].copy_from_slice(&expected.to_le_bytes());
            assert!(!e.call(0x008648e0, &[block + 4, block + 64]).bool());
            assert_eq!(e.mem.bytes(block, 96), bytes);
        }
    }

    #[test]
    fn form_id_output_load_checks_live_index_after_decode_and_stores_result() {
        for (encoded, resolved, clear_index, failed) in [
            ([0, 0, 0], 0, false, false),
            ([0, 0, 1], 0, false, true),
            ([0, 0, 1], 0x1234_5678, false, false),
            ([0, 0, 1], 0, true, false),
        ] {
            let (mut e, buffer, _) = fixture(&[encoded[0], encoded[1], encoded[2], b'|']);
            let output = e.mem.alloc(12);
            e.mem.write(output, &[0xa5; 12]);
            e.register_double(0x00853500, move |e, a| {
                assert_eq!(a.len(), 1);
                assert_eq!(e.mem.bytes(a[0], 3), encoded);
                if clear_index {
                    e.mem.write(a[0], &[0; 3]);
                }
                resolved.into_ret()
            });
            assert_eq!(
                e.call(0x008648e0, &args![buffer, output + 4]).bool(),
                failed
            );
            assert_eq!(e.mem.u32(output + 4), resolved);
            assert_eq!(e.mem.bytes(output, 4), [0xa5; 4]);
            assert_eq!(e.mem.bytes(output + 8, 4), [0xa5; 4]);
            assert_eq!(e.get(buffer, BGSLoadGameBuffer::iBufferPosition), 4);
        }
    }

    fn fixture(data: &[u8]) -> (Engine, Ptr<BGSLoadGameBuffer>, u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(20);
        let buffer = e.mem.alloc(data.len() as u32);
        e.mem.write(buffer, data);
        e.mem.set_u32(object + 16, 0xaabbccdd);
        let this = Ptr::new(object);
        e.call(0x008646b0, &args![this]);
        e.call(0x008647f0, &args![this, buffer, data.len() as u32, 0u32]);
        // Generated memcpy dependency, not a full startup claim.
        e.register_double(0x00401460, |e, a| {
            assert_eq!(a.len(), 3);
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        (e, this, buffer)
    }

    #[test]
    fn constructor_and_setter_preserve_guard_and_read_raw_separator() {
        let (mut e, this, buffer) = fixture(b"abc|");
        let destination = e.mem.alloc(4);
        e.mem.set_u32(destination, 0xdddddddd);
        e.call(0x00864980, &args![this, destination, 3u32]);
        assert_eq!(e.mem.bytes(destination, 4), b"abc\xdd");
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 4);
        assert_eq!(e.mem.u32(this.addr()), 0x01082048);
        assert_eq!(e.mem.u32(this.addr() + 16), 0xaabbccdd);
        e.call(0x008646b0, &args![this]);
        assert_eq!(e.mem.bytes(this.addr() + 4, 12), vec![0; 12]);
        assert_eq!(e.mem.bytes(buffer, 4), b"abc|");
    }

    #[test]
    fn read_reloads_fields_after_copy_and_cursor_after_warning() {
        let (mut e, this, original) = fixture(b"ab|");
        let replacement = e.mem.alloc(16);
        e.mem.set_u8(replacement + 7, b'?');
        let output = e.mem.alloc(2);
        e.register_double(0x00401460, move |e, a| {
            assert_eq!(a, &[output, original, 2]);
            e.mem.write(output, b"ab");
            e.set(this, BGSLoadGameBuffer::pBuffer, Ptr::new(replacement));
            e.set(this, BGSLoadGameBuffer::iBufferPosition, 5);
            Ret::default()
        });
        e.register_double(0x00845cc0, move |e, a| {
            assert_eq!(a, &[0x0107ee40]);
            assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 7);
            e.set(this, BGSLoadGameBuffer::iBufferPosition, u32::MAX);
            Ret::default()
        });
        e.call(0x00864820, &args![this, output, 2u32]);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 0);
        assert_eq!(e.mem.bytes(output, 2), b"ab");
    }

    #[test]
    fn zero_size_field_leaves_destination_and_cursor_untouched() {
        let (mut e, this, _) = fixture(b"|");
        e.set(this, BGSLoadGameBuffer::iBufferSize, 0);
        e.set(this, BGSLoadGameBuffer::iBufferPosition, 123);
        e.call_log = Some(vec![]);
        e.call(0x00864820, &args![this, 0u32, u32::MAX]);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 123);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn variable_tags_decode_widths_and_reserved_tag_does_not_advance() {
        for (bytes, expected) in [
            (vec![0xfc, b'|'], 63),
            (vec![0xfd, 0xff, b'|'], 16383),
            (vec![0xfe, 0xff, 0xff, 0xff, b'|'], 0x3fffffff),
        ] {
            let (mut e, this, _) = fixture(&bytes);
            assert_eq!(e.call(0x00864a60, &args![this]).u32(), expected);
            assert_eq!(
                e.get(this, BGSLoadGameBuffer::iBufferPosition),
                bytes.len() as u32
            );
            e.set(this, BGSLoadGameBuffer::iBufferPosition, 0);
            e.set(this, BGSLoadGameBuffer::iBufferSize, 0);
            assert_eq!(e.call(0x00864a60, &args![this]).u32(), 0);
            assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 0);
        }
        let (mut e, this, _) = fixture(&[3]);
        assert_eq!(e.call(0x00864a60, &args![this]).u32(), 0);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 0);
    }

    #[test]
    fn created_form_id_uses_real_three_byte_decoder() {
        let (mut e, this, _) = fixture(&[0x81, 0x23, 0x45, b'|']);
        assert_eq!(e.call(0x008648a0, &args![this]).u32(), 0xff012345);
        assert_eq!(e.get(this, BGSLoadGameBuffer::iBufferPosition), 4);
    }
}
