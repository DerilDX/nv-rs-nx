//! `BSMain/run_00c3f160` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The object's lock (+0x20; its owner word, then the count at +0x24).
const LOCK: u32 = 0x20;
const LOCK_COUNT: u32 = 0x24;
/// The array of 12-byte entries `00c3f530` builds from (+0x04).
const ENTRIES: u32 = 0x04;

/// `0040fbf0` (`thiscall` on the lock, one word): takes the lock.
const LOCK_ENTER: u32 = 0x0040_fbf0;
/// `00c3f750` (unit `iomanager.obj`, `thiscall`): the map the calls work
/// on (a pointer to it).
const GET_MAP: u32 = 0x00c3_f750;
/// The map's virtual slot `0x24` (`(key low, key high)`): the bucket of a
/// key.
const SLOT_BUCKET: u32 = 0x24;
/// `00c3ffb0` and `00c403d0` (unit `iomanager.obj`, `thiscall` on the map
/// pointer): the two map operations, `(bucket, key low, key high, value,
/// flag)` and `(bucket, key low, key high)`.
const MAP_INSERT: u32 = 0x00c3_ffb0;
const MAP_REMOVE: u32 = 0x00c4_03d0;
/// `00c3fe00` (unit `iomanager.obj`, `thiscall`, four words): the
/// constructor of what `00c3f530` makes.
const ENTRY_OBJECT_CONSTRUCT: u32 = 0x00c3_fe00;
/// The memory manager singleton and its `Allocate` (`00aa3e40`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The thread's word at TLS +0x2b4 that `00c3f530` sets to 6 around its
/// allocation.
const TLS_ALLOCATION_CONTEXT: u32 = 0x2b4;

/// Releases the lock: the count goes down and, at zero, the owner word is
/// cleared.
fn lock_leave(e: &mut Engine, this: u32) {
    let lock = this.wrapping_add(LOCK);
    let count = e.mem.u32(lock.wrapping_add(4)).wrapping_sub(1);
    e.mem.set_u32(lock.wrapping_add(4), count);
    if e.mem.u32(this.wrapping_add(LOCK_COUNT)) == 0 {
        e.mem.set_u32(lock, 0);
    }
}

/// The map pointer and the bucket of the key, under the lock.
fn bucket(e: &mut Engine, this: u32, key_low: u32, key_high: u32) -> (u32, u32) {
    e.call(LOCK_ENTER, &args![this.wrapping_add(LOCK), 0u32]);
    let map = e.call(GET_MAP, &args![this]).u32();
    let object = e.mem.u32(map);
    let bucket = e
        .vcall(object, SLOT_BUCKET, &args![key_low, key_high])
        .u32();
    (map, bucket)
}

// Translated from 00c3f1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3f160`): under the object's lock
/// (+0x20), finds the bucket of the 64-bit key (the map's slot `0x24`) and
/// runs `00c3ffb0` with it, the key, `value` and the low byte of `flag`;
/// returns that call's byte.
pub fn fn_00c3f1f0(
    e: &mut Engine,
    this: Ptr,
    key_low: u32,
    key_high: u32,
    value: u32,
    flag: u32,
) -> bool {
    let this = this.addr();
    let (map, bucket) = bucket(e, this, key_low, key_high);
    let result = e
        .call(
            MAP_INSERT,
            &args![map, bucket, key_low, key_high, value, flag & 0xff],
        )
        .u8();
    lock_leave(e, this);
    result != 0
}

// Translated from 00c3f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3f160`): under the object's lock
/// (+0x20), finds the bucket of the 64-bit key and runs `00c403d0` with it
/// and the key; returns that call's byte.
pub fn fn_00c3f2b0(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32) -> bool {
    let this = this.addr();
    let (map, bucket) = bucket(e, this, key_low, key_high);
    let result = e
        .call(MAP_REMOVE, &args![map, bucket, key_low, key_high])
        .u8();
    lock_leave(e, this);
    result != 0
}

// Translated from 00c3f530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c3f530 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2df03, state variable) is not translated
/// Unnamed in the engine map (unit `run_00c3f160`): with the thread's word
/// at TLS +0x2b4 set to 6 (and restored after), allocates 0x24 bytes from
/// the memory manager and, when that succeeds, constructs it (`00c3fe00`)
/// from `this` and the three words of entry `index` of the array at +4.
/// Returns it, or 0.
pub fn fn_00c3f530(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let context = e.tls().wrapping_add(TLS_ALLOCATION_CONTEXT);
    let previous = e.mem.u32(context);
    e.mem.set_u32(context, 6);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x24u32])
        .u32();
    let object = if block == 0 {
        0
    } else {
        let entries = e.mem.u32(this.addr().wrapping_add(ENTRIES));
        let entry = entries.wrapping_add(index.wrapping_mul(12));
        e.call(
            ENTRY_OBJECT_CONSTRUCT,
            &args![
                block,
                this,
                entry,
                entry.wrapping_add(4),
                entry.wrapping_add(8)
            ],
        )
        .u32()
    };
    let context = e.tls().wrapping_add(TLS_ALLOCATION_CONTEXT);
    e.mem.set_u32(context, previous);
    object
}

// Translated from 00c3f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3f160`): returns its two words
/// as a 64-bit value (`EDX:EAX`). `this` is not read.
pub fn fn_00c3f670(_e: &mut Engine, _this: Ptr, low: u32, high: u32) -> u64 {
    u64::from(low) | (u64::from(high) << 32)
}

// Translated from 00c3f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3f160`): stores its two words at
/// `out`. `this` is not read.
pub fn fn_00c3f690(e: &mut Engine, _this: Ptr, low: u32, high: u32, out: Ptr) {
    e.mem.set_u32(out.addr(), low);
    e.mem.set_u32(out.addr().wrapping_add(4), high);
}

// Translated from 00c3f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `run_00c3f160`): whether the signed
/// 64-bit value `a` is `>=` the signed 64-bit value `b` (high words
/// compared signed, then low words unsigned). `this` is not read.
pub fn fn_00c3f6b0(
    _e: &mut Engine,
    _this: Ptr,
    a_low: u32,
    a_high: u32,
    b_low: u32,
    b_high: u32,
) -> bool {
    let (a_high, b_high) = (a_high as i32, b_high as i32);
    if a_high < b_high {
        return false;
    }
    if a_high > b_high {
        return true;
    }
    a_low >= b_low
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_f1f0, fn_00c3f1f0(Ptr, u32, u32, u32, u32) -> bool),
        entry!(0x00c3_f2b0, fn_00c3f2b0(Ptr, u32, u32) -> bool),
        entry!(0x00c3_f530, fn_00c3f530(Ptr, u32) -> u32),
        entry!(0x00c3_f670, fn_00c3f670(Ptr, u32, u32) -> u64),
        entry!(0x00c3_f690, fn_00c3f690(Ptr, u32, u32, Ptr)),
        entry!(0x00c3_f6b0, fn_00c3f6b0(Ptr, u32, u32, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const BUCKET: u32 = 0x00ee_0100;

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            eax.into_ret()
        });
    }

    /// An object whose map pointer (`00c3f750`) leads to a map with the
    /// `BUCKET` slot; the lock taken once (count 1).
    fn object(e: &mut Engine, log: &Log) -> (u32, u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + SLOT_BUCKET, BUCKET);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map, vtable);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, map);
        let this = e.mem.alloc(0x28);
        logged(e, log, GET_MAP, pointer);
        logged(e, log, BUCKET, 3);
        let l = log.clone();
        e.register_double(LOCK_ENTER, move |e, a| {
            l.borrow_mut().push((LOCK_ENTER, a.to_vec()));
            e.mem.set_u32(a[0], 0x77);
            let count = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, count + 1);
            Ret::default()
        });
        (this, pointer, map)
    }

    #[test]
    fn fn_00c3f1f0_inserts_under_the_lock() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, pointer, map) = object(&mut e, &log);
        logged(&mut e, &log, MAP_INSERT, 0x101);
        assert!(e
            .call(0x00c3_f1f0, &args![this, 10u32, 20u32, 30u32, 0x1ffu32])
            .bool());
        assert_eq!(
            *log.borrow(),
            [
                (LOCK_ENTER, vec![this + LOCK, 0]),
                (GET_MAP, vec![this]),
                (BUCKET, vec![map, 10, 20]),
                (MAP_INSERT, vec![pointer, 3, 10, 20, 30, 0xff]),
            ]
        );
        assert_eq!(e.mem.u32(this + LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(this + LOCK), 0);
        // A nested lock keeps its owner.
        e.mem.set_u32(this + LOCK_COUNT, 1);
        logged(&mut e, &log, MAP_INSERT, 0);
        assert!(!e
            .call(0x00c3_f1f0, &args![this, 10u32, 20u32, 30u32, 0u32])
            .bool());
        assert_eq!(e.mem.u32(this + LOCK_COUNT), 1);
        assert_eq!(e.mem.u32(this + LOCK), 0x77);
    }

    #[test]
    fn fn_00c3f2b0_removes_under_the_lock() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, pointer, _) = object(&mut e, &log);
        logged(&mut e, &log, MAP_REMOVE, 1);
        assert!(e.call(0x00c3_f2b0, &args![this, 10u32, 20u32]).bool());
        assert_eq!(log.borrow()[3], (MAP_REMOVE, vec![pointer, 3, 10, 20]));
        assert_eq!(e.mem.u32(this + LOCK), 0);
    }

    #[test]
    fn fn_00c3f530_builds_from_an_entry_in_allocation_context_6() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x24);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_ALLOCATION_CONTEXT, 2);
        let l = log.clone();
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |e, a| {
            let context = e.tls() + TLS_ALLOCATION_CONTEXT;
            l.borrow_mut().push((
                MEMORY_MANAGER_ALLOCATE,
                vec![a[0], a[1], e.mem.u32(context)],
            ));
            block.into_ret()
        });
        logged(&mut e, &log, ENTRY_OBJECT_CONSTRUCT, block);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + ENTRIES, 0x9000);
        assert_eq!(e.call(0x00c3_f530, &args![this, 2u32]).u32(), block);
        assert_eq!(
            *log.borrow(),
            [
                (MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 0x24, 6]),
                (
                    ENTRY_OBJECT_CONSTRUCT,
                    vec![block, this, 0x9018, 0x901c, 0x9020]
                ),
            ]
        );
        assert_eq!(e.mem.u32(tls + TLS_ALLOCATION_CONTEXT), 2);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, 0);
        assert_eq!(e.call(0x00c3_f530, &args![this, 2u32]).u32(), 0);
    }

    #[test]
    fn small_helpers_pack_store_and_compare_64_bit_values() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c3_f670, &args![0u32, 0x89ab_cdefu32, 0x0123_4567u32])
                .u64(),
            0x0123_4567_89ab_cdef
        );
        let out = e.mem.alloc(8);
        e.call(0x00c3_f690, &args![0u32, 5u32, 6u32, out]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (5, 6));
        for (a, b, expected) in [
            (5i64, 5i64, true),
            (4, 5, false),
            (-1, 0, false),
            (0, -1, true),
            (0x1_0000_0000, 0xffff_ffff, true),
        ] {
            let result = e
                .call(
                    0x00c3_f6b0,
                    &args![0u32, a as u32, (a >> 32) as u32, b as u32, (b >> 32) as u32],
                )
                .bool();
            assert_eq!(result, expected, "{a} >= {b}");
        }
    }
}
