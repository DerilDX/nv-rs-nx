//! `NiAnimation/run_00a26d70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

// Translated from 00a26df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two stack words): reads a 4-byte value from the
/// stream's input stream into `destination`. Calls the input stream's read
/// function as `function(input, destination, 4, &count, 1)` with `count` a local set
/// to 4, and adds the byte count it returns to the input stream's position.
pub fn fn_00a26df0(e: &mut Engine, stream: Ptr<NiStream>, destination: Ptr) {
    let input = e.get(stream, NiStream::m_pkIstr);
    let input = Ptr::<NiBinaryStream>::new(input);
    e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(function, &args![input, destination, 4u32, count, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

// Translated from 00a26e20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3654` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a26e20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3654) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d20, 0x0);
    e.mem.set_u32(0x011f_3c00, 0x0);
    e.mem.set_u32(0x011f_3b70, 0x0);
    e.mem.set_u32(0x011f_3810, 0x0);
    e.mem.set_u8(0x011f_3654, 0x1);
    e.mem.set_u32(0x011f_3db0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a50, 0xa29680);
    e.mem.set_u32(0x011f_39c0, 0xa29680);
    e.mem.set_u32(0x011f_3930, 0xe7cd40);
    e.mem.set_u32(0x011f_38a0, 0xa29680);
    e.mem.set_u8(0x011f_3758, 0x8);
    e.mem.set_u32(0x011f_3c90, 0xa29680);
    e.mem.set_u32(0x011f_3ae0, 0xa26b00);
    e.mem.set_u32(0x011f_3780, 0xa29680);
    1
}

// Translated from 00a273a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3655` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a273a0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3655) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d28, 0x0);
    e.mem.set_u32(0x011f_3c08, 0x0);
    e.mem.set_u32(0x011f_3b78, 0x0);
    e.mem.set_u8(0x011f_3655, 0x1);
    e.mem.set_u32(0x011f_3db8, 0xa27250);
    e.mem.set_u32(0x011f_3a58, 0xa27320);
    e.mem.set_u32(0x011f_39c8, 0xa26f80);
    e.mem.set_u32(0x011f_3938, 0xa27040);
    e.mem.set_u32(0x011f_38a8, 0xa2a600);
    e.mem.set_u8(0x011f_375a, 0x10);
    e.mem.set_u32(0x011f_3c98, 0xa26fe0);
    e.mem.set_u32(0x011f_3ae8, 0xa26f30);
    e.mem.set_u32(0x011f_3788, 0xa29680);
    e.mem.set_u32(0x011f_3818, 0xa27080);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_6df0, fn_00a26df0(Ptr<NiStream>, Ptr)),
        entry!(0x00a26e20, fn_00a26e20() -> u32),
        entry!(0x00a273a0, fn_00a273a0() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const READ: u32 = 0x00aa_2e60;

    #[test]
    fn fn_00a26df0_reads_four_bytes_through_the_input_stream() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 4, 10);
        e.mem.set_u32(input.addr() + 8, READ);
        let destination = e.mem.alloc(8);
        e.register(READ, |e, a| {
            // (input, destination, 4, &count, 1) with count set to 4
            assert_eq!(a[2], 4);
            assert_eq!(a[4], 1);
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2_6df0, &args![stream, destination]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].1[..3], [input.addr(), destination, 4]);
        assert_eq!(e.mem.u32(destination), 0x1234_5678);
        assert_eq!(e.mem.u32(input.addr() + 4), 14);
    }

    #[test]
    fn fn_00a26df0_adds_a_short_read_to_the_position() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.mem.set_u32(stream.addr() + 0x24c, input.addr());
        e.mem.set_u32(input.addr() + 8, READ);
        e.register(READ, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let destination = e.mem.alloc(8);
        e.call(0x00a2_6df0, &args![stream, destination]);
        assert_eq!(e.mem.u32(input.addr() + 4), 2);
    }
}
