//! `Havok SDK/hkmemorysystem.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The global word `00cbbfb0` stores its argument in. The only other code that
/// touches it is the function at `00cbbfc0` (Ghidra name `getInstance`), which
/// reads it.
const MEMORY_SYSTEM_INSTANCE: u32 = 0x0126_8418;

layout! {
    /// `hkMemorySystem::FrameInfo` (Xbox PDB), 8 bytes.
    pub struct HkMemorySystemFrameInfo: 0x8 {
        /// `m_stackSize` (Xbox PDB).
        0x00 m_stackSize: i32,
        /// `m_solverBufferSize` (Xbox PDB).
        0x04 m_solverBufferSize: i32,
    }
}

// Translated from 00cbbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemorySystem::FrameInfo::FrameInfo` (Xbox PDB): sets the stack size to
/// `0x32000` and the solver buffer size to `0x600000`. Returns `this`.
pub fn hk_memory_system_frame_info_frame_info(
    e: &mut Engine,
    this: Ptr<HkMemorySystemFrameInfo>,
) -> Ptr<HkMemorySystemFrameInfo> {
    e.set(this, HkMemorySystemFrameInfo::m_stackSize, 0x32000);
    e.set(
        this,
        HkMemorySystemFrameInfo::m_solverBufferSize,
        0x0060_0000,
    );
    this
}

// Translated from 00cbbfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one stack word): stores its argument in
/// the global at `01268418`, the word the function at `00cbbfc0` (Ghidra name
/// `getInstance`) reads. The Havok start-up calls it with the memory system it
/// has just built (`00c682f1` and `00c6673e` call it). Returns nothing.
pub fn fn_00cbbfb0(e: &mut Engine, instance: u32) {
    e.set_global(MEMORY_SYSTEM_INSTANCE, instance);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cbbfa0,
            hk_memory_system_frame_info_frame_info(
                Ptr<HkMemorySystemFrameInfo>,
            ) -> Ptr<HkMemorySystemFrameInfo>
        ),
        entry!(0x00cbbfb0, fn_00cbbfb0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_info_constructor_sets_the_default_sizes_and_returns_this() {
        let mut e = Engine::new();
        let info: Ptr<HkMemorySystemFrameInfo> = e.new_object();
        e.mem.set_u32(info.addr(), 0xdead_beef);
        e.mem.set_u32(info.addr() + 4, 0xdead_beef);
        assert_eq!(e.call(0x00cb_bfa0, &args![info]).u32(), info.addr());
        assert_eq!(e.mem.u32(info.addr()), 0x32000);
        assert_eq!(e.mem.u32(info.addr() + 4), 0x0060_0000);
    }

    #[test]
    fn the_instance_setter_stores_its_argument_in_the_global() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.call(0x00cb_bfb0, &args![0x0120_4410u32]);
        assert_eq!(e.global::<u32>(MEMORY_SYSTEM_INSTANCE), 0x0120_4410);
        e.call(0x00cb_bfb0, &args![0u32]);
        assert_eq!(e.global::<u32>(MEMORY_SYSTEM_INSTANCE), 0);
    }
}
