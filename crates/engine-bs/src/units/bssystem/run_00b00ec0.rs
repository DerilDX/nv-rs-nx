//! `BSSystem/run_00b00ec0` (Xbox PDB source unit), subsystem `BSSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSWin32TaskletData` and `BSTaskletData` vtables (their RTTI names).
const WIN32_TASKLET_DATA_VTABLE: u32 = 0x0106_c800;
const TASKLET_DATA_VTABLE: u32 = 0x0106_c814;
/// `BSFileCache`'s vtable (its RTTI name).
const FILE_CACHE_VTABLE: u32 = 0x010a_47cc;
/// Import slots: `InterlockedCompareExchange` and `Sleep` (Ghidra's names).
const INTERLOCKED_COMPARE_EXCHANGE: u32 = 0x00fd_f1bc;
const SLEEP: u32 = 0x00fd_f104;
/// The two words `00b012f0` stores.
const STORED_FIRST: u32 = 0x011a_d028;
const STORED_SECOND: u32 = 0x011a_d02c;

// Translated from 00b00ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32TaskletData`'s destructor: takes the lock word at `+0x10` (spins
/// on `InterlockedCompareExchange(+0x10, 1, 0)` with `Sleep(0)` until it
/// was 0), clears the word at `+8` when set and the lock, and leaves the
/// `BSTaskletData` vtable.
pub fn fn_00b00ec0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, WIN32_TASKLET_DATA_VTABLE);
    while e
        .call(
            INTERLOCKED_COMPARE_EXCHANGE,
            &args![base.wrapping_add(0x10), 1u32, 0u32],
        )
        .u32()
        != 0
    {
        e.call(SLEEP, &args![0u32]);
    }
    if e.mem.u32(base.wrapping_add(8)) != 0 {
        e.mem.set_u32(base.wrapping_add(8), 0);
    }
    e.mem.set_u32(base.wrapping_add(0x10), 0);
    e.mem.set_u32(base, TASKLET_DATA_VTABLE);
}

// Translated from 00b00f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFileCache`'s constructor: its vtable and the start values of its
/// fields (`+0x5C` and `+0x2C` 1, `+0x4C` 2, `+0x28` -1, the others listed
/// 0). Returns `this`.
pub fn fn_00b00f20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, FILE_CACHE_VTABLE);
    for (offset, value) in [
        (0x24u32, 0u32),
        (0x50, 0),
        (0x60, 0),
        (0x5c, 1),
        (0x04, 0),
        (0x08, 0),
        (0x0c, 0),
        (0x14, 0),
        (0x18, 0),
        (0x28, 0xffff_ffff),
        (0x10, 0),
        (0x1c, 0),
        (0x2c, 1),
        (0x20, 0),
        (0x30, 0),
        (0x40, 0),
    ] {
        e.mem.set_u32(base.wrapping_add(offset), value);
    }
    for offset in [0x44u32, 0x45, 0x47] {
        e.mem.set_u8(base.wrapping_add(offset), 0);
    }
    for (offset, value) in [(0x34u32, 0u32), (0x48, 0), (0x4c, 2), (0x58, 0), (0x54, 0)] {
        e.mem.set_u32(base.wrapping_add(offset), value);
    }
    this
}

// Translated from 00b01020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFileCache`'s destructor body: puts back its vtable.
pub fn fn_00b01020(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FILE_CACHE_VTABLE);
}

// Translated from 00b011b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the entry the object's vtable +0x14 finds for `key` is in state
/// 2 (`fn_00b011e0`).
pub fn fn_00b011b0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let index = e.vcall(this.addr(), 0x14, &args![key]).u32();
    fn_00b011e0(e, this, index as i32)
}

// Translated from 00b011e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether entry `index` (signed, below the count at `+0x24`) of the array
/// at `+0x50` (its storage at `+4`) has the state word (`+0xC`) 2; false
/// for an index out of range.
pub fn fn_00b011e0(e: &mut Engine, this: Ptr, index: i32) -> bool {
    let count = e.mem.u32(this.addr().wrapping_add(0x24)) as i32;
    if index < 0 || index >= count {
        return false;
    }
    let array = e.mem.u32(this.addr().wrapping_add(0x50));
    let storage = e.mem.u32(array.wrapping_add(4));
    let entry = e
        .mem
        .u32(storage.wrapping_add((index as u32).wrapping_mul(4)));
    e.mem.u32(entry.wrapping_add(0xc)) == 2
}

// Translated from 00b012f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores its two words at `011ad028` and `011ad02c` (`this` is not read).
pub fn fn_00b012f0(e: &mut Engine, _this: Ptr, first: u32, second: u32) {
    e.set_global(STORED_FIRST, first);
    e.set_global(STORED_SECOND, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_0ec0, fn_00b00ec0(Ptr)),
        entry!(0x00b0_0f20, fn_00b00f20(Ptr) -> Ptr),
        entry!(0x00b0_1020, fn_00b01020(Ptr)),
        entry!(0x00b0_11b0, fn_00b011b0(Ptr, u32) -> bool),
        entry!(0x00b0_11e0, fn_00b011e0(Ptr, i32) -> bool),
        entry!(0x00b0_12f0, fn_00b012f0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tasklet_destructor_waits_for_the_lock() {
        let mut e = Engine::new();
        let mut tries = 0;
        e.register_double(INTERLOCKED_COMPARE_EXCHANGE, move |_, _| {
            tries += 1;
            u32::from(tries < 3).into_ret()
        });
        e.register(SLEEP, |_, _| Ret::default());
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(data + 8, 5);
        e.mem.set_u32(data + 0x10, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b0_0ec0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == SLEEP).count(), 2);
        assert!(log.contains(&(INTERLOCKED_COMPARE_EXCHANGE, vec![data + 0x10, 1, 0])));
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 0x10), 0);
        assert_eq!(e.mem.u32(data), TASKLET_DATA_VTABLE);
    }

    #[test]
    fn file_cache_constructor_and_destructor() {
        let mut e = Engine::new();
        let cache = e.mem.alloc(0x64);
        for offset in (0..0x64).step_by(4) {
            e.mem.set_u32(cache + offset, 0x7777_7777);
        }
        assert_eq!(e.call(0x00b0_0f20, &args![cache]).u32(), cache);
        assert_eq!(e.mem.u32(cache), FILE_CACHE_VTABLE);
        assert_eq!(e.mem.u32(cache + 0x5c), 1);
        assert_eq!(e.mem.u32(cache + 0x4c), 2);
        assert_eq!(e.mem.u32(cache + 0x28), 0xffff_ffff);
        assert_eq!(e.mem.u32(cache + 0x44), 0x0077_0000);
        assert_eq!(e.mem.u32(cache + 0x38), 0x7777_7777);
        e.mem.set_u32(cache, 0);
        e.call(0x00b0_1020, &args![cache]);
        assert_eq!(e.mem.u32(cache), FILE_CACHE_VTABLE);
    }

    #[test]
    fn entry_state_checks_the_index_and_the_state() {
        let mut e = Engine::new();
        let cache = e.mem.alloc(0x64);
        let array = e.mem.alloc(8);
        let storage = e.mem.alloc(8);
        let entries = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        e.mem.set_u32(entries[1] + 0xc, 2);
        e.mem.set_u32(storage, entries[0]);
        e.mem.set_u32(storage + 4, entries[1]);
        e.mem.set_u32(array + 4, storage);
        e.mem.set_u32(cache + 0x50, array);
        e.mem.set_u32(cache + 0x24, 2);
        assert!(!e.call(0x00b0_11e0, &args![cache, 0i32]).bool());
        assert!(e.call(0x00b0_11e0, &args![cache, 1i32]).bool());
        assert!(!e.call(0x00b0_11e0, &args![cache, 2i32]).bool());
        assert!(!e.call(0x00b0_11e0, &args![cache, -1i32]).bool());
        e.put_vtable(0x0060_0000, &[0, 0, 0, 0, 0, 0x0061_0014]);
        e.mem.set_u32(cache, 0x0060_0000);
        e.register(0x0061_0014, |_, a| (a[1] / 10).into_ret());
        assert!(e.call(0x00b0_11b0, &args![cache, 15u32]).bool());
        assert!(!e.call(0x00b0_11b0, &args![cache, 5u32]).bool());
    }

    #[test]
    fn two_words_are_stored() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        e.call(0x00b0_12f0, &args![0u32, 3u32, 4u32]);
        assert_eq!(e.global::<u32>(STORED_FIRST), 3);
        assert_eq!(e.global::<u32>(STORED_SECOND), 4);
    }
}
