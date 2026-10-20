//! `NiXenonRenderer/nid3dshaderfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::ALLOCATE;

/// `00aa0390` (`thiscall`, no stack word, no unit): the base constructor.
const BASE_CONSTRUCTOR: u32 = 0x00aa_0390;
/// `00fdf054`: import slot of `InitializeCriticalSection` (`stdcall`, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00a29680` (`cdecl`, one word here): the empty call the exe makes with a string.
const EMPTY_CALL: u32 = 0x00a2_9680;
/// `0126fa0c`: the shader factory object `fn_00e92e60` builds.
const FACTORY_SLOT: u32 = 0x0126_fa0c;
/// `011f6044`: a second pointer to the same object.
const FACTORY_ALIAS_SLOT: u32 = 0x011f_6044;

/// Builds a hash-map-like object of `size` pointer slots in 0x14 bytes (the vtables
/// `first` then `second`, the size at `+4`, a zeroed slot block of `block_bytes` at `+8`,
/// the word at `+0xc` 0, the byte at `+0x10` `flag`); answers it, or 0 when the allocation
/// fails.
fn make_slot_table(
    e: &mut Engine,
    first: u32,
    second: u32,
    size: u32,
    block_bytes: u32,
    flag: u8,
) -> u32 {
    let table = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if table == 0 {
        return 0;
    }
    e.mem.set_u32(table, first);
    e.mem.set_u32(table + 4, size);
    e.mem.set_u32(table + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![block_bytes]).u32();
    let bytes = e.mem.u32(table + 4).wrapping_mul(4);
    e.mem.set_u32(table + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(table + 0x10, flag);
    e.mem.set_u32(table, second);
    table
}

// Translated from 00e92cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderfactory.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00aa0390`, sets the vtable
/// `010f0cc4`, clears the word at `+0x24`, initializes the critical section at `+0x80`,
/// then makes two 0x14-byte tables kept at `+0x28` (vtables `010f0bc0` then `010f0c80`,
/// 0x3b slots from a 0xec-byte block, flag byte 1) and `+0x30` (vtables `010f0c20` then
/// `010f0ca0`, 0x25 slots from a 0x94-byte block, flag byte 0); a table whose allocation
/// fails is stored as null. Returns `this`.
pub fn fn_00e92cf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, 0x010f_0cc4);
    e.mem.set_u32(base + 0x24, 0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![base + 0x80]);
    let first = make_slot_table(e, 0x010f_0bc0, 0x010f_0c80, 0x3b, 0xec, 1);
    e.mem.set_u32(base + 0x28, first);
    let second = make_slot_table(e, 0x010f_0c20, 0x010f_0ca0, 0x25, 0x94, 0);
    e.mem.set_u32(base + 0x30, second);
    this
}

// Translated from 00e92e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::_SDMInit` (Xbox PDB; run by the start-up `00e91dc0`, no arguments):
/// while `0126fa0c` is null, allocates 0x100 bytes (`00aa13e0`), constructs the factory
/// with `fn_00e92cf0` (null when the allocation fails) and keeps it in `0126fa0c` and
/// `011f6044`; when `011f6044` is null it calls `00a29680` with the string at `010f0d40`.
pub fn ni_d3d_shader_factory_sdm_init(e: &mut Engine) {
    if e.mem.u32(FACTORY_SLOT) == 0 {
        let block = e.call(ALLOCATE, &args![0x100u32]).u32();
        let factory = if block != 0 {
            fn_00e92cf0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(FACTORY_SLOT, factory);
        e.mem.set_u32(FACTORY_ALIAS_SLOT, factory);
    }
    if e.mem.u32(FACTORY_ALIAS_SLOT) == 0 {
        e.call(EMPTY_CALL, &args![0x010f_0d40u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_2cf0, fn_00e92cf0(Ptr) -> Ptr),
        entry!(0x00e9_2e60, ni_d3d_shader_factory_sdm_init()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x2000);
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| 1u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(EMPTY_CALL, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_builds_two_tables() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_2cf0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![this + 0x80]]
        );
        assert_eq!(e.mem.u32(this), 0x010f_0cc4);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        let first = e.mem.u32(this + 0x28);
        assert_eq!(e.mem.u32(first), 0x010f_0c80);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        assert_eq!(e.mem.u32(first + 0xc), 0);
        assert_eq!(e.mem.u8(first + 0x10), 1);
        let slots = e.mem.u32(first + 8);
        assert_eq!(e.mem.bytes(slots, 0xec), vec![0; 0xec]);
        let second = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(second), 0x010f_0ca0);
        assert_eq!(e.mem.u32(second + 4), 0x25);
        assert_eq!(e.mem.u8(second + 0x10), 0);
        let allocations = calls_to(&e, ALLOCATE_BLOCK);
        assert_eq!(allocations, vec![vec![0xec], vec![0x94]]);
    }

    #[test]
    fn constructor_stores_null_for_a_failed_table() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.call(0x00e9_2cf0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn start_up_makes_the_factory_once() {
        let mut e = engine();
        e.call(0x00e9_2e60, &args![]);
        let factory = e.mem.u32(FACTORY_SLOT);
        assert_ne!(factory, 0);
        assert_eq!(e.mem.u32(FACTORY_ALIAS_SLOT), factory);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), factory);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(calls_to(&e, EMPTY_CALL).is_empty());
    }

    #[test]
    fn start_up_with_a_failed_allocation_calls_the_empty_function() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), 0);
        assert_eq!(calls_to(&e, EMPTY_CALL), vec![vec![0x010f_0d40]]);
    }
}
