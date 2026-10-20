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

/// The `MemoryManager` singleton (`thiscall` object of `00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the singleton, one stack word: the
/// block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// Head of the first chain of blocks `00a8a200` frees (the first word of each block is the
/// next one) and the word cleared with it.
const FIRST_CHAIN_HEAD: u32 = 0x011c_5f54;
const FIRST_CHAIN_COUNT: u32 = 0x011c_5f58;
/// Head of the chain `00a8a240` frees and the word cleared with it.
const SECOND_CHAIN_HEAD: u32 = 0x011f_6034;
const SECOND_CHAIN_COUNT: u32 = 0x011f_6038;
/// Byte `011f5b80`: set by the module initializer `00a89cc0`; `00a8a280` runs only while it
/// is set and clears it.
const MODULE_INITIALIZED: u32 = 0x011f_5b80;
/// `00a63520` (`cdecl`, one word; unit `NiMain/nistream.cpp`): called by `00a8a280` with each
/// class-name string of [`SHUTDOWN_NAMES`].
const UNREGISTER_NAME: u32 = 0x00a6_3520;
/// The 73 string addresses `00a8a280` passes to `00a63520`, in its order.
const SHUTDOWN_NAMES: [u32; 73] = [
    0x0109_df08,
    0x0109_d378,
    0x0109_f220,
    0x0109_b4e0,
    0x0109_f210,
    0x0109_f1fc,
    0x0109_f1e4,
    0x0109_f1c8,
    0x0109_dec4,
    0x0109_df58,
    0x0109_f1b4,
    0x0109_f1a8,
    0x0109_cd04,
    0x0109_f190,
    0x0109_b0f8,
    0x0109_ed6c,
    0x0109_ca34,
    0x0109_d890,
    0x0109_f17c,
    0x0109_dab0,
    0x0109_f168,
    0x0109_ecb0,
    0x0109_f158,
    0x0109_f144,
    0x0109_f130,
    0x0109_f124,
    0x0109_d64c,
    0x0109_f118,
    0x0109_d79c,
    0x0109_b6b8,
    0x0109_f10c,
    0x0109_f0f8,
    0x0109_f0e0,
    0x0109_f0d4,
    0x0109_f0c4,
    0x0109_f0a0,
    0x0109_dc94,
    0x0109_de0c,
    0x0109_f090,
    0x0109_f07c,
    0x0109_f064,
    0x0109_f054,
    0x0109_f044,
    0x0109_e314,
    0x0109_e6a0,
    0x0109_e6ec,
    0x0109_e560,
    0x0109_f030,
    0x0109_ea78,
    0x0109_ebd4,
    0x0109_baa4,
    0x0109_f01c,
    0x0109_f010,
    0x0109_cb84,
    0x0109_d43c,
    0x0109_effc,
    0x0109_efec,
    0x0109_efd4,
    0x0109_efc4,
    0x0109_c7a0,
    0x0109_d548,
    0x0109_db9c,
    0x0109_e408,
    0x0109_ce38,
    0x0109_d990,
    0x0109_efac,
    0x0109_ef98,
    0x0109_b7dc,
    0x0109_ef30,
    0x0109_ef70,
    0x0109_ef58,
    0x0109_ef48,
    0x0109_ef84,
];
/// The calls (no arguments) `00a8a280` makes after the names, in order, before its own
/// `00a8a200` and `00a8a240`.
const SHUTDOWN_STEPS: [u32; 19] = [
    0x00a9_fda0,
    0x00a9_28b0,
    0x00a5_cf60,
    0x00a9_a120,
    0x00a7_5a40,
    0x00a9_bb60,
    0x00a8_1590,
    0x00a6_f4d0,
    0x00a6_a6f0,
    0x00a8_f620,
    0x00a7_6b30,
    0x00a6_32b0,
    0x00a6_2ae0,
    0x00a9_1930,
    0x00a6_2280,
    0x00a7_e940,
    0x00a8_f730,
    0x00a8_fcc0,
    0x00a8_ff30,
];
/// The last call of `00a8a280`, reached with a `JMP`.
const SHUTDOWN_LAST_STEP: u32 = 0x00a5_b660;

/// Gives every block of the chain starting at `head` back to the memory manager, then clears
/// `head` and `count`.
fn free_chain(e: &mut Engine, head: u32, count: u32) {
    let mut block = e.mem.u32(head);
    while block != 0 {
        let next = e.mem.u32(block);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
        block = next;
    }
    e.mem.set_u32(count, 0);
    e.mem.set_u32(head, 0);
}

// Translated from 00a8a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): frees the chain of blocks headed at `011c5f54`
/// (each block's first word is the next block) through `MemoryManager::Deallocate`
/// (`00aa4060`), then clears `011c5f58` and the head.
pub fn fn_00a8a200(e: &mut Engine) {
    free_chain(e, FIRST_CHAIN_HEAD, FIRST_CHAIN_COUNT);
}

// Translated from 00a8a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the same for the chain headed at `011f6034`,
/// clearing `011f6038` and the head.
pub fn fn_00a8a240(e: &mut Engine) {
    free_chain(e, SECOND_CHAIN_HEAD, SECOND_CHAIN_COUNT);
}

// Translated from 00a8a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map and without a Ghidra function (no arguments): the function the
/// `NiMain` module's constructor `00a8a5f0` stores as the start-up hook's second word. When
/// the initialized byte `011f5b80` is set, clears it, passes each of 73 class-name strings to
/// `00a63520`, makes the 19 calls of [`SHUTDOWN_STEPS`], frees the two block chains
/// (`00a8a200`, `00a8a240`) and ends with `00a5b660`; otherwise does nothing.
pub fn fn_00a8a280(e: &mut Engine) {
    if e.mem.u8(MODULE_INITIALIZED) == 0 {
        return;
    }
    e.mem.set_u8(MODULE_INITIALIZED, 0);
    for name in SHUTDOWN_NAMES {
        e.call(UNREGISTER_NAME, &args![name]);
    }
    for step in SHUTDOWN_STEPS {
        e.call(step, &args![]);
    }
    fn_00a8a200(e);
    fn_00a8a240(e);
    e.call(SHUTDOWN_LAST_STEP, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a8_a620,
            fn_00a8a620(Ptr<PoolBlock>, u32) -> Ptr<PoolBlock>
        ),
        entry!(0x00a8_a200, fn_00a8a200()),
        entry!(0x00a8_a240, fn_00a8a240()),
        entry!(0x00a8_a280, fn_00a8a280()),
    ]
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

#[cfg(test)]
mod tests_shutdown {
    use super::*;

    fn calls(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != top)
            .collect()
    }

    fn chain(e: &mut Engine, head: u32, count: u32, blocks: usize) -> Vec<u32> {
        e.map(head & !0xfff, 0x1000);
        let list: Vec<u32> = (0..blocks).map(|_| e.mem.alloc(8)).collect();
        for pair in list.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        e.mem.set_u32(head, list.first().copied().unwrap_or(0));
        e.mem.set_u32(count, 0x55);
        list
    }

    #[test]
    fn fn_00a8a200_frees_every_block_of_its_chain() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        let list = chain(&mut e, FIRST_CHAIN_HEAD, FIRST_CHAIN_COUNT, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a8_a200, &args![]);
        let expected: Vec<_> = list
            .iter()
            .map(|&block| (DEALLOCATE, vec![MEMORY_MANAGER, block]))
            .collect();
        assert_eq!(calls(&mut e, 0x00a8_a200), expected);
        assert_eq!(e.mem.u32(FIRST_CHAIN_HEAD), 0);
        assert_eq!(e.mem.u32(FIRST_CHAIN_COUNT), 0);
    }

    #[test]
    fn fn_00a8a240_with_an_empty_chain_only_clears() {
        let mut e = Engine::new();
        e.register(DEALLOCATE, |_, _| Ret::default());
        chain(&mut e, SECOND_CHAIN_HEAD, SECOND_CHAIN_COUNT, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_a240, &args![]);
        assert!(calls(&mut e, 0x00a8_a240).is_empty());
        assert_eq!(e.mem.u32(SECOND_CHAIN_COUNT), 0);
        let list = chain(&mut e, SECOND_CHAIN_HEAD, SECOND_CHAIN_COUNT, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a8_a240, &args![]);
        assert_eq!(
            calls(&mut e, 0x00a8_a240),
            vec![
                (DEALLOCATE, vec![MEMORY_MANAGER, list[0]]),
                (DEALLOCATE, vec![MEMORY_MANAGER, list[1]]),
            ]
        );
        assert_eq!(e.mem.u32(SECOND_CHAIN_HEAD), 0);
    }

    #[test]
    fn fn_00a8a280_runs_once_while_the_module_is_initialized() {
        let mut e = Engine::new();
        e.map(MODULE_INITIALIZED & !0xfff, 0x1000);
        chain(&mut e, FIRST_CHAIN_HEAD, FIRST_CHAIN_COUNT, 0);
        chain(&mut e, SECOND_CHAIN_HEAD, SECOND_CHAIN_COUNT, 0);
        e.register(UNREGISTER_NAME, |_, _| Ret::default());
        for step in SHUTDOWN_STEPS {
            e.register(step, |_, _| Ret::default());
        }
        e.register(SHUTDOWN_LAST_STEP, |_, _| Ret::default());
        // Not initialized: nothing runs.
        e.call_log = Some(vec![]);
        e.call(0x00a8_a280, &args![]);
        assert!(calls(&mut e, 0x00a8_a280).is_empty());
        e.mem.set_u8(MODULE_INITIALIZED, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a8_a280, &args![]);
        let log = calls(&mut e, 0x00a8_a280);
        let mut expected: Vec<(u32, Vec<u32>)> = SHUTDOWN_NAMES
            .iter()
            .map(|&name| (UNREGISTER_NAME, vec![name]))
            .collect();
        expected.extend(SHUTDOWN_STEPS.iter().map(|&step| (step, vec![])));
        expected.push((SHUTDOWN_LAST_STEP, vec![]));
        assert_eq!(log, expected);
        assert_eq!(e.mem.u8(MODULE_INITIALIZED), 0);
        assert_eq!(e.mem.u32(FIRST_CHAIN_COUNT), 0);
        assert_eq!(e.mem.u32(SECOND_CHAIN_COUNT), 0);
    }
}
