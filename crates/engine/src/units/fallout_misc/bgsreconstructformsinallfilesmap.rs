//! `fallout/misc/saveload/bgsreconstructformsinallfilesmap.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Functions marked `Translation incomplete` have an exception-unwinding
//! frame (`FS:[0]`) that is not translated; everything else in them is.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00844db0(this, bucket_count)`: the hash-table base initializer below.
/// `00844130` and `008433d0` pass `0x25` as the bucket count.
const BUCKET_COUNT: u32 = 0x25;
/// `008453a0(this, 0, 0)` (`run_00845230`): runs after `00844a90` stores its
/// vtable.
const MAP_INITIALIZER: u32 = 0x0084_53a0;
/// `00aa1070`: the allocator (cdecl, byte count), as `00853270` uses it.
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;

// Translated from 00844db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed1c`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the bucket table
/// (+8) and zeroes it. Returns `this`.
pub fn fn_00844db0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0107_ed1c);
    e.mem.set_u32(t + 4, bucket_count);
    e.mem.set_u32(t + 0xc, 0);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    e.mem.set_u32(t + 8, table);
    let bytes = e.mem.u32(t + 4) << 2;
    let table = e.mem.u32(t + 8);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 00844130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00844db0(this, bucket_count)`, then the vtable
/// `0107ec48` is stored. Returns `this`.
pub fn fn_00844130(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    fn_00844db0(e, this, bucket_count);
    e.mem.set_u32(this.addr(), 0x0107_ec48);
    this
}

// Translated from 00844a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `0107ed08` and runs
/// `008453a0(this, 0, 0)`. Returns `this`.
pub fn fn_00844a90(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_ed08);
    e.call(MAP_INITIALIZER, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00843470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00844a90` on `this` and on the member at
/// +0x10. Returns `this`.
// Translation incomplete: 00843470 - SEH exception unwinding is unsupported.
pub fn fn_00843470(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844a90(e, this);
    fn_00844a90(e, this.byte_add(0x10).cast());
    this
}

// Translated from 008433d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSReconstructFormsInAllFilesMap::BGSReconstructFormsInAllFilesMap`
/// (Xbox PDB): the base `00844130(this, 0x25)`, the vtable `0107ea74`, the
/// member at +0x10 through `00843470`, and 0 at +0x30. Returns `this`.
// Translation incomplete: 008433d0 - SEH exception unwinding is unsupported.
pub fn bgsreconstructformsinallfilesmap_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00844130(e, this, BUCKET_COUNT);
    e.mem.set_u32(this.addr(), 0x0107_ea74);
    fn_00843470(e, this.byte_add(0x10).cast());
    e.mem.set_u32(this.addr() + 0x30, 0);
    this
}

// Translated from 00843220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00844ff0`, a destructor with an exception frame
/// that stores the vtable `0107ed3c` and runs `008454f0` and `00844fa0`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `00844ff0`'s disassembly ends with its exception-frame
/// unlink and returns no value (Ghidra types it void).
pub fn fn_00843220(e: &mut Engine, this: Ptr) {
    e.call(0x0084_4ff0, &args![this]);
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0107ED08: u32 = 0x0107_ed08;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 00844ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UReconstructFormData@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_00844ac0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0107ED08);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084_3220, fn_00843220(Ptr)),
        entry!(0x00844db0, fn_00844db0(Ptr, u32) -> Ptr),
        entry!(0x00844130, fn_00844130(Ptr, u32) -> Ptr),
        entry!(0x00844a90, fn_00844a90(Ptr) -> Ptr),
        entry!(0x00843470, fn_00843470(Ptr) -> Ptr),
        entry!(0x008433d0, bgsreconstructformsinallfilesmap_constructor(Ptr) -> Ptr),
        entry!(0x0084_4ac0, fn_00844ac0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(MAP_INITIALIZER, |_, _| Ret::default());
    }

    #[test]
    fn fn_00844db0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 77);
        let r = e.call(0x00844db0, &args![this, 5u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed1c);
        assert_eq!(e.mem.u32(this + 4), 5);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.block_size(table), Some(24), "20 bytes, rounded up");
        assert_eq!(e.mem.bytes(table, 20), vec![0; 20]);
    }

    #[test]
    fn fn_00844130_replaces_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x20);
        let r = e.call(0x00844130, &args![this, 3u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ec48);
        assert_eq!(e.mem.u32(this + 4), 3);
    }

    #[test]
    fn fn_00844a90_stores_the_vtable_and_initializes() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00844a90, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
    }

    #[test]
    fn fn_00843470_initializes_both_members() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        let r = e.call(0x00843470, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_INITIALIZER, vec![this, 0, 0]));
        assert_eq!(log[2], (MAP_INITIALIZER, vec![this + 0x10, 0, 0]));
        assert_eq!(e.mem.u32(this), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
    }

    #[test]
    fn the_constructor_builds_the_base_member_and_clears_the_tail() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 9);
        let r = e.call(0x008433d0, &args![this]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), 0x0107_ea74, "the final vtable");
        assert_eq!(e.mem.u32(this + 4), 0x25, "base bucket count");
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_ed08);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00843220_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0084_4ff0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0084_3220, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0084_4ff0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_00844ac0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0084_4ac0, ARRAY_VTABLE_0107ED08, ARRAY_RELEASE_008454F0);
    }
}
