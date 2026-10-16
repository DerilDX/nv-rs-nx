//! `BSMain/bsvaluenode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a643b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStreamLoadBinary<unsigned char>` (engine map name
/// `NiStreamLoadBinary<unsigned_char>_ov2`, the second copy; `cdecl`, two stack words): reads one byte
/// from the stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 1, &count, 1)` with `count` a local set
/// to 1, and adds the byte count it returns to the input stream's position.
pub fn ni_stream_load_binary_unsigned_char_ov2(
    e: &mut Engine,
    stream: Ptr<NiStream>,
    destination: Ptr,
) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 1);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 1u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

/// `00a5e9d0` (named `NiNode::LinkObject` in the engine map): the base body the wrapper
/// forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A5E9D0: u32 = 0x00a5_e9d0;

// Translated from 00c43900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSValueNode` vtable word 010c19ac, right after
/// `BSValueNode::LoadBinary` (010c19a8). Forwards `this` and the stream to `00a5e9d0`
/// (`NiNode::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c43900(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A5E9D0, &args![this, stream]);
}

// Translated from 00c43cf0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202de8 without reading the receiver. The map names this body `BSValueNode::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c43cf0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2de8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3cf0, fn_00c43cf0(Ptr) -> u32),
        entry!(
            0x00a6_43b0,
            ni_stream_load_binary_unsigned_char_ov2(Ptr<NiStream>, Ptr)
        ),
        entry!(0x00c4_3900, fn_00c43900(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c43900_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A5E9D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3900, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A5E9D0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn load_binary_unsigned_char_reads_one_byte_through_the_input_stream() {
        const READ: u32 = 0x00aa_2e60;
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 7);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 1, &count, 1) with count set to 1
            assert_eq!(a[2], 1);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 0x5a);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a6_43b0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 1]);
        assert_eq!(e.mem.u8(destination), 0x5a);
        assert_eq!(e.mem.u32(input.addr() + 4), 8);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c43cf0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_3cf0, &args![0x1000u32]).u32(), 0x0120_2de8);
        assert_eq!(e.call(0x00c4_3cf0, &args![0u32]).u32(), 0x0120_2de8);
    }
}
