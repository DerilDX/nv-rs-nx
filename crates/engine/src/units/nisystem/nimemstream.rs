//! `NiSystem/nimemstream.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base class constructor `00aa1d10` (it sets the base virtual table and zeroes
/// the position and the two function pointers) and destructor `00aa1d30` (it
/// sets the base virtual table back); both `thiscall` with no stack word.
pub(crate) const BASE_CONSTRUCTOR: u32 = 0x00aa_1d10;
const BASE_DESTRUCTOR: u32 = 0x00aa_1d30;
/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
pub(crate) const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
/// The virtual table the memory stream constructors store (`010a256c`; its
/// scalar deleting destructor is the first slot).
pub(crate) const MEM_STREAM_VTABLE: u32 = 0x010a_256c;
/// The read and write functions the constructors store in the base class's
/// `m_pfnRead` and `m_pfnWrite` (`00aa2e60`, `00aa2ea0`; not translated here).
pub(crate) const MEM_STREAM_READ: u32 = 0x00aa_2e60;
pub(crate) const MEM_STREAM_WRITE: u32 = 0x00aa_2ea0;

layout! {
    /// `NiMemStream` (Xbox PDB), 0x24 bytes: `NiBinaryStream` (vtable, current
    /// position, read and write function pointers) and the buffer.
    pub struct NiMemStream: 0x24 {
        /// `m_uiAbsoluteCurrentPos` (Xbox PDB, `NiBinaryStream`).
        0x04 m_uiAbsoluteCurrentPos: u32,
        /// `m_pfnRead` (Xbox PDB, `NiBinaryStream`).
        0x08 m_pfnRead: u32,
        /// `m_pfnWrite` (Xbox PDB, `NiBinaryStream`).
        0x0C m_pfnWrite: u32,
        /// `m_pBuffer` (Xbox PDB).
        0x10 m_pBuffer: u32,
        /// `m_uiPos` (Xbox PDB).
        0x14 m_uiPos: u32,
        /// `m_uiEnd` (Xbox PDB).
        0x18 m_uiEnd: u32,
        /// `m_uiAllocSize` (Xbox PDB).
        0x1C m_uiAllocSize: u32,
        /// `m_bUserMemory` (Xbox PDB): the buffer belongs to the caller.
        0x20 m_bUserMemory: bool,
        /// `m_bFreeze` (Xbox PDB).
        0x21 m_bFreeze: bool,
    }
}

// Translated from 00aa2fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::NiMemStream(char* buffer, unsigned int size)` (engine map): a stream
/// over the caller's `buffer` of `size` bytes (user memory, already full:
/// `m_uiEnd` and `m_uiAllocSize` are `size`, the position 0). Returns `this`.
pub fn ni_mem_stream_constructor(
    e: &mut Engine,
    this: Ptr<NiMemStream>,
    buffer: u32,
    size: u32,
) -> Ptr<NiMemStream> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.set(this, NiMemStream::m_pBuffer, buffer);
    e.set(this, NiMemStream::m_uiAllocSize, size);
    e.set(this, NiMemStream::m_uiEnd, size);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    e.set(this, NiMemStream::m_uiPos, 0);
    e.set(this, NiMemStream::m_bUserMemory, true);
    e.set(this, NiMemStream::m_bFreeze, false);
    e.set(this, NiMemStream::m_pfnRead, MEM_STREAM_READ);
    e.set(this, NiMemStream::m_pfnWrite, MEM_STREAM_WRITE);
    this
}

// Translated from 00aa2d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMemStream::~NiMemStream` (engine map): puts the memory stream's virtual table
/// back, frees the buffer unless it is user memory or was taken
/// (`m_bFreeze`, see `00aa2d60`), then runs the base class destructor.
pub fn ni_mem_stream_destructor(e: &mut Engine, this: Ptr<NiMemStream>) {
    let user_memory = e.get(this, NiMemStream::m_bUserMemory);
    e.mem.set_u32(this.addr(), MEM_STREAM_VTABLE);
    if !user_memory && !e.get(this, NiMemStream::m_bFreeze) {
        let buffer = e.get(this, NiMemStream::m_pBuffer);
        e.call(FREE, &args![buffer]);
    }
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00aa2d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `m_bFreeze` (Xbox PDB field at +0x21), which
/// stops the destructor from freeing the buffer, and returns `m_pBuffer`, so the
/// caller owns the buffer from here on.
pub fn fn_00aa2d60(e: &mut Engine, this: Ptr<NiMemStream>) -> u32 {
    e.set(this, NiMemStream::m_bFreeze, true);
    e.get(this, NiMemStream::m_pBuffer)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_2fb0,
            ni_mem_stream_constructor(Ptr<NiMemStream>, u32, u32) -> Ptr<NiMemStream>
        ),
        entry!(0x00aa_2d30, ni_mem_stream_destructor(Ptr<NiMemStream>)),
        entry!(0x00aa_2d60, fn_00aa2d60(Ptr<NiMemStream>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], target: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == target)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn constructor_over_a_user_buffer() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        let stream = e.new_object::<NiMemStream>();
        // Dirty fields prove every one is written.
        for off in (0u32..0x24).step_by(4) {
            e.mem.set_u32(stream.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        let result = e.call(0x00aa_2fb0, &args![stream, 0x4000u32, 0x30u32]);
        assert_eq!(result.u32(), stream.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BASE_CONSTRUCTOR), vec![vec![stream.addr()]]);
        let at = stream.addr();
        assert_eq!(e.mem.u32(at), MEM_STREAM_VTABLE);
        assert_eq!(e.mem.u32(at + 0x08), 0x00aa_2e60);
        assert_eq!(e.mem.u32(at + 0x0c), 0x00aa_2ea0);
        assert_eq!(e.mem.u32(at + 0x10), 0x4000);
        assert_eq!(e.mem.u32(at + 0x14), 0);
        assert_eq!(e.mem.u32(at + 0x18), 0x30);
        assert_eq!(e.mem.u32(at + 0x1c), 0x30);
        assert_eq!(e.mem.u8(at + 0x20), 1);
        assert_eq!(e.mem.u8(at + 0x21), 0);
    }

    #[test]
    fn destructor_frees_only_an_owned_buffer_that_was_not_taken() {
        // (user memory, taken, buffer freed)
        for (user, taken, freed) in [
            (0u8, 0u8, true),
            (1, 0, false),
            (0, 1, false),
            (1, 1, false),
        ] {
            let mut e = Engine::new();
            e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
            e.register(FREE, |_, _| Ret::default());
            let stream = e.new_object::<NiMemStream>();
            e.mem.set_u32(stream.addr() + 0x10, 0x7000);
            e.mem.set_u8(stream.addr() + 0x20, user);
            e.mem.set_u8(stream.addr() + 0x21, taken);
            e.call_log = Some(vec![]);
            e.call(0x00aa_2d30, &args![stream]);
            let log = e.call_log.take().unwrap();
            let frees = calls_to(&log, FREE);
            assert_eq!(frees, if freed { vec![vec![0x7000]] } else { vec![] });
            assert_eq!(calls_to(&log, BASE_DESTRUCTOR), vec![vec![stream.addr()]]);
            assert_eq!(e.mem.u32(stream.addr()), MEM_STREAM_VTABLE);
        }
    }

    #[test]
    fn taking_the_buffer_sets_the_freeze_flag_and_returns_it() {
        let mut e = Engine::new();
        let stream = e.new_object::<NiMemStream>();
        e.mem.set_u32(stream.addr() + 0x10, 0x7000);
        assert_eq!(e.call(0x00aa_2d60, &args![stream]).u32(), 0x7000);
        assert_eq!(e.mem.u8(stream.addr() + 0x21), 1);
    }
}
