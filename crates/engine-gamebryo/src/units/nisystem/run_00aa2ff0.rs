//! `NiSystem/run_00aa2ff0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nisystem::nimemstream::{
    NiMemStream, ALLOCATE, BASE_CONSTRUCTOR, MEM_STREAM_READ, MEM_STREAM_VTABLE, MEM_STREAM_WRITE,
};

/// `00aa2710`: the jump target of `00aa3040`.
const JUMP_TARGET_00AA3040: u32 = 0x00aa_2710;

/// The size of the buffer the default constructor allocates.
const DEFAULT_BUFFER_SIZE: u32 = 0x400;

// Translated from 00aa2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the default constructor of the memory stream (it stores the
/// virtual table of `NiMemStream` constructed by `00aa2fb0`, and has the same fields):
/// a buffer of 0x400 bytes of its own (`m_bUserMemory` clear, so the
/// destructor frees it), position and end 0. Returns `this`.
pub fn fn_00aa2ff0(e: &mut Engine, this: Ptr<NiMemStream>) -> Ptr<NiMemStream> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    e.set(this, NiMemStream::m_uiAllocSize, DEFAULT_BUFFER_SIZE);
    let buffer = e.call(ALLOCATE, &args![DEFAULT_BUFFER_SIZE]).u32();
    e.set(this, NiMemStream::m_pBuffer, buffer);
    e.set(this, NiMemStream::m_uiPos, 0);
    e.set(this, NiMemStream::m_uiEnd, 0);
    e.set(this, NiMemStream::m_bUserMemory, false);
    e.set(this, NiMemStream::m_bFreeze, false);
    e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_READ);
    e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_WRITE);
    this
}

// Translated from 00aa3040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00aa2710`, passing on `this`; returns the target result word. Return type (u32): the target `00aa2710` returns EAX at every return (-1, or the result of an imported call); its meaning is unconfirmed.
pub fn fn_00aa3040(e: &mut Engine, this: Ptr) -> u32 {
    e.call(JUMP_TARGET_00AA3040, &args![this]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_2ff0,
            fn_00aa2ff0(Ptr<NiMemStream>) -> Ptr<NiMemStream>
        ),
        entry!(0x00aa_3040, fn_00aa3040(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00aa3040_forwards_this_and_returns_the_target_result() {
        const THUNK: u32 = 0x00aa_3040;
        const TARGET: u32 = 0x00aa_2710;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32]).u32(), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32]);
    }

    #[test]
    fn default_constructor_allocates_its_own_buffer() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        let stream = e.new_object::<NiMemStream>();
        for off in (0u32..0x24).step_by(4) {
            e.mem.set_u32(stream.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_2ff0, &args![stream]).u32(), stream.addr());
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![0x400u32]);
        let at = stream.addr();
        assert_eq!(e.mem.u32(at), MEM_STREAM_VTABLE);
        assert_eq!(e.mem.u32(at + 0x08), MEM_STREAM_READ);
        assert_eq!(e.mem.u32(at + 0x0c), MEM_STREAM_WRITE);
        assert_eq!(e.mem.u32(at + 0x10), 0x6000);
        assert_eq!(e.mem.u32(at + 0x14), 0);
        assert_eq!(e.mem.u32(at + 0x18), 0);
        assert_eq!(e.mem.u32(at + 0x1c), 0x400);
        assert_eq!(e.mem.u8(at + 0x20), 0);
        assert_eq!(e.mem.u8(at + 0x21), 0);
    }
}
