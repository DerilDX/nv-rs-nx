//! `fallout/misc/saveload/bgsloadgamesubbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00865570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded append wrapper: reads position at +0xc and forwards the low
/// byte of the last stack argument to the positioned save-buffer writer.
pub fn append_bytes(e: &mut Engine, this: Ptr, source: Ptr, size: u32, no_separator: u8) {
    let position = e.mem.u32(this.addr().wrapping_add(0xc));
    e.call(
        0x00865ce0,
        &args![this, source, size, position, no_separator],
    );
}

/// `00aa1070`: the allocator (cdecl, byte count).
const ALLOCATE: u32 = 0x00aa_1070;
/// `00403d30` (`memset`, cdecl: destination, value, byte count).
const MEMSET: u32 = 0x0040_3d30;
/// `00438af0(this)`: the base teardown (`fallout shared/run_00438af0`).
const BASE_TEARDOWN: u32 = 0x0043_8af0;
/// `00aa10f0(pointer)`: releases a block (cdecl, one argument).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// `__ehvec_ctor` (`00ec782f`, CRT): constructs an array of elements.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;

// Translated from 008656c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, the bucket count
/// at +4 and 0 at +0xc, allocates `4 * count` bytes for the table (+8) and
/// zeroes it. Returns `this`.
pub fn fn_008656c0(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x0108_22a8);
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

// Translated from 00865660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `008656c0(this, word)`, then the vtable
/// `01082288`. Returns `this`.
pub fn fn_00865660(e: &mut Engine, this: Ptr, word: u32) -> Ptr {
    fn_008656c0(e, this, word);
    e.mem.set_u32(this.addr(), 0x0108_2288);
    this
}

// Translated from 00865200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00865660(this, 0x25)`; `EAX` is whatever that
/// returns (it returns `this`).
pub fn fn_00865200(e: &mut Engine, this: Ptr) -> u32 {
    fn_00865660(e, this, 0x25).addr()
}

// Translated from 00865840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `010822a8`, runs
/// `00438af0(this)` and releases the block at +8 (`00aa10f0`). `EAX` is the
/// release call's result, returned here.
pub fn fn_00865840(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_22a8);
    e.call(BASE_TEARDOWN, &args![this]);
    let block = e.mem.u32(this.addr() + 8);
    e.call(FREE_BLOCK, &args![block]).u32()
}

// Translated from 008657e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores the vtable `01082288`, runs
/// `00438af0(this)` and then `00865840`; `EAX` is the latter's result.
// Translation incomplete: 008657e0 - SEH exception unwinding is unsupported.
pub fn fn_008657e0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_2288);
    e.call(BASE_TEARDOWN, &args![this]);
    fn_00865840(e, this)
}

// Translated from 008651d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs the array of three 0x10-byte
/// elements at `this` with `__ehvec_ctor` (`00ec782f`: element constructor
/// `00865200`, element destructor `008657e0`). Returns `this`.
pub fn fn_008651d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this, 0x10u32, 3u32, 0x0086_5200u32, 0x0086_57e0u32],
    );
    this
}

/// The function `fn_00865870` calls on the member (`00865900`).
const CALLEE_00865900: u32 = 0x0086_5900;

// Translated from 00865870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `NiTMap<TESForm*,BGSLoadGameSubBuffer>::NewItem` (Xbox PDB name in the engine map). Thin wrapper: calls `00865900` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00865870(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00865900, &args![this.byte_add(0xc)]).ptr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00865570, append_bytes(Ptr, Ptr, u32, u8)),
        entry!(0x008656c0, fn_008656c0(Ptr, u32) -> Ptr),
        entry!(0x00865660, fn_00865660(Ptr, u32) -> Ptr),
        entry!(0x00865200, fn_00865200(Ptr) -> u32),
        entry!(0x00865840, fn_00865840(Ptr) -> u32),
        entry!(0x008657e0, fn_008657e0(Ptr) -> u32),
        entry!(0x008651d0, fn_008651d0(Ptr) -> Ptr),
        entry!(0x0086_5870, fn_00865870(Ptr) -> Ptr),
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
        e.register(BASE_TEARDOWN, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn fn_008656c0_builds_a_zeroed_bucket_table() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008656c0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.bytes(table, 12), vec![0; 12]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ALLOCATE, vec![12]));
        assert_eq!(log[2], (MEMSET, vec![table, 0, 12]));
    }

    #[test]
    fn fn_00865660_and_00865200_store_the_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00865660, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0108_2288);
        assert_eq!(e.mem.u32(this + 4), 9, "the base initializer ran");
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00865200, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this + 4), 0x25);
    }

    #[test]
    fn fn_00865840_releases_the_block_at_8_and_returns_the_result() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00865840, &args![this]).u32(), 0x7001);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_TEARDOWN, vec![this]));
        assert_eq!(log[2], (FREE_BLOCK, vec![0x7000]));
    }

    #[test]
    fn fn_008657e0_tears_down_twice_and_ends_on_the_second_vtable() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x008657e0, &args![this]);
        let order: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect();
        assert_eq!(order, vec![BASE_TEARDOWN, BASE_TEARDOWN, FREE_BLOCK]);
        assert_eq!(e.mem.u32(this), 0x0108_22a8);
    }

    #[test]
    fn fn_008651d0_constructs_three_elements() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008651d0, &args![0x6000u32]).u32(), 0x6000);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (
                VECTOR_CONSTRUCT,
                vec![0x6000, 0x10, 3, 0x0086_5200, 0x0086_57e0]
            )
        );
    }

    #[test]
    fn l68d_00865870_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00865900, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0086_5870, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00865900).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}
