//! `NiMain/run_00a8a200` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa1070`: the `cdecl` allocate (size).
const ALLOCATE: u32 = 0x00aa_1070;

layout! {
    /// A memory pool object: the shape of the Xbox PDB `NiObjectGroup` (size 0x10:
    /// `m_uiSize`, `m_pvBuffer`, `m_pvFree`, `m_uiRefCount`). That the object
    /// `00a8a620` constructs is an `NiObjectGroup` is not confirmed by the engine
    /// map (the function has no name there); the fields are named by the layout.
    pub struct PoolBlock: 0x10 {
        /// `m_uiSize` (Xbox PDB `NiObjectGroup`).
        0x00 m_uiSize: u32,
        /// `m_pvBuffer` (Xbox PDB `NiObjectGroup`).
        0x04 m_pvBuffer: u32,
        /// `m_pvFree` (Xbox PDB `NiObjectGroup`).
        0x08 m_pvFree: u32,
        /// `m_uiRefCount` (Xbox PDB `NiObjectGroup`).
        0x0C m_uiRefCount: u32,
    }
}

// Translated from 00a8a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructor of the 0x10 byte object [`PoolBlock`]
/// (`thiscall`, one stack word, returns `this`). Stores `size`, allocates a buffer of
/// that many bytes (`00aa1070`; null for size 0), sets the free pointer to the buffer
/// and the reference count to 0.
pub fn fn_00a8a620(e: &mut Engine, this: Ptr<PoolBlock>, size: u32) -> Ptr<PoolBlock> {
    e.set(this, PoolBlock::m_uiSize, size);
    let buffer = if size != 0 {
        e.call(ALLOCATE, &args![size]).u32()
    } else {
        0
    };
    e.set(this, PoolBlock::m_pvBuffer, buffer);
    e.set(this, PoolBlock::m_pvFree, buffer);
    e.set(this, PoolBlock::m_uiRefCount, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00a8_a620,
        fn_00a8a620(Ptr<PoolBlock>, u32) -> Ptr<PoolBlock>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_size_allocates_the_buffer_and_points_the_free_pointer_at_it() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        let block = e.new_object::<PoolBlock>();
        for off in (0u32..0x10).step_by(4) {
            e.mem.set_u32(block.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a8_a620, &args![block, 0x40u32]).u32(),
            block.addr()
        );
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![0x40u32]);
        let at = block.addr();
        assert_eq!(
            [
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u32(at + 8),
                e.mem.u32(at + 12)
            ],
            [0x40, 0x7000, 0x7000, 0]
        );
    }

    #[test]
    fn size_zero_allocates_nothing() {
        let mut e = Engine::new();
        let block = e.new_object::<PoolBlock>();
        for off in (0u32..0x10).step_by(4) {
            e.mem.set_u32(block.addr() + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a8_a620, &args![block, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let at = block.addr();
        assert_eq!(
            [
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u32(at + 8),
                e.mem.u32(at + 12)
            ],
            [0, 0, 0, 0]
        );
    }
}
