//! `NiCollision/run_00e99300` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The use counter (`.data`): its first increment, from 0, registers the exit handler.
const USE_COUNT_0126FA60: u32 = 0x0126_fa60;
/// `00aa2080` (cdecl, two words): registers the function given in the first word.
const REGISTER_EXIT_HANDLER: u32 = 0x00aa_2080;
/// The two words handed to it.
const EXIT_HANDLER_00E98FF0: u32 = 0x00e9_8ff0;
const EXIT_HANDLER_ARGUMENT_00E99260: u32 = 0x00e9_9260;

// Translated from 00e99300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: increments the word at `0126fa60`; when it was 0, calls
/// `00aa2080(00e98ff0, 00e99260)`. Returns `this`, which it does not otherwise read.
pub fn fn_00e99300(e: &mut Engine, this: Ptr) -> Ptr {
    let count = e.global::<u32>(USE_COUNT_0126FA60);
    e.set_global(USE_COUNT_0126FA60, count.wrapping_add(1));
    if count == 0 {
        e.call(
            REGISTER_EXIT_HANDLER,
            &args![EXIT_HANDLER_00E98FF0, EXIT_HANDLER_ARGUMENT_00E99260],
        );
    }
    this
}

/// `00a64900` (`cdecl`, two words, `NiStream::RegisterLoader`): registers a class name
/// (the address of a string in `.rdata`) with its creation function.
const REGISTER_LOADER: u32 = 0x00a6_4900;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// Byte `0126fa5c`: set to 1 when `fn_00e98ff0` has started.
const OBJECTS_BUILT: u32 = 0x0126_fa5c;
/// The six function addresses `fn_00e98ff0` stores first: (global, function).
const FUNCTION_GLOBALS: [(u32, u32); 6] = [
    (0x0126_fa80, 0x00ea_7490),
    (0x0126_fa84, 0x00ea_04b0),
    (0x0126_fa90, 0x00e9_e780),
    (0x0126_fa88, 0x00ea_cb00),
    (0x0126_fa7c, 0x00ea_c170),
    (0x0126_fa8c, 0x00e9_efe0),
];
/// Words of the first 0x90-byte object (offset, value).
const FIRST_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_b9d0),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_7530),
    (0x1c, 0x00ea_0ad0),
    (0x20, 0x00ea_2030),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f620),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f760),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_e980),
    (0x64, 0x00e9_e980),
    (0x68, 0x00e9_e980),
    (0x6c, 0x0000_0000),
    (0x70, 0x00e9_e9e0),
    (0x74, 0x00e9_e980),
    (0x78, 0x00e9_cd10),
    (0x7c, 0x00e9_cde0),
    (0x80, 0x00e9_cf50),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_d070),
];
/// Words of the second 0x90-byte object (offset, value).
const SECOND_OBJECT: [(u32, u32); 36] = [
    (0x0, 0x00ea_bb20),
    (0x4, 0x0000_0000),
    (0x8, 0x0000_0000),
    (0xc, 0x0000_0000),
    (0x10, 0x0000_0000),
    (0x14, 0x0000_0000),
    (0x18, 0x00ea_3950),
    (0x1c, 0x00ea_7d40),
    (0x20, 0x00ea_3c50),
    (0x24, 0x0000_0000),
    (0x28, 0x0000_0000),
    (0x2c, 0x0000_0000),
    (0x30, 0x00e9_f400),
    (0x34, 0x0000_0000),
    (0x38, 0x00e9_f510),
    (0x3c, 0x0000_0000),
    (0x40, 0x0000_0000),
    (0x44, 0x0000_0000),
    (0x48, 0x0000_0000),
    (0x4c, 0x0000_0000),
    (0x50, 0x0000_0000),
    (0x54, 0x0000_0000),
    (0x58, 0x0000_0000),
    (0x5c, 0x0000_0000),
    (0x60, 0x00e9_eae0),
    (0x64, 0x00e9_eae0),
    (0x68, 0x00e9_eae0),
    (0x6c, 0x00e9_eae0),
    (0x70, 0x00e9_eb60),
    (0x74, 0x00e9_eae0),
    (0x78, 0x00e9_d2c0),
    (0x7c, 0x00e9_d540),
    (0x80, 0x00e9_da30),
    (0x84, 0x0000_0000),
    (0x88, 0x0000_0000),
    (0x8c, 0x00e9_dfa0),
];
/// Words of the first 0x18-byte object (offset, value).
const THIRD_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c240),
    (0x4, 0x00ea_2190),
    (0x8, 0x00e9_f8c0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ea70),
    (0x14, 0x00e9_d150),
];
/// Words of the second 0x18-byte object (offset, value).
const FOURTH_OBJECT: [(u32, u32); 6] = [
    (0x0, 0x00ea_c3f0),
    (0x4, 0x00ea_3d60),
    (0x8, 0x00e9_fdc0),
    (0xc, 0x0000_0000),
    (0x10, 0x00e9_ec10),
    (0x14, 0x00e9_e2c0),
];
/// The three function addresses `fn_00e98ff0` stores last: (global, function).
const LAST_GLOBALS: [(u32, u32); 3] = [
    (0x0126_faa4, 0x00e9_c7d0),
    (0x0126_faa8, 0x00e9_c800),
    (0x0126_faac, 0x00e9_c850),
];

/// Allocates `size` bytes (`00aa1070`) and stores the (offset, value) words in them.
fn make_object(e: &mut Engine, size: u32, words: &[(u32, u32)]) -> u32 {
    let block = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    for (offset, value) in words {
        e.mem.set_u32(block.wrapping_add(*offset), *value);
    }
    block
}

// Translated from 00e98ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the start-up function that `fn_00e99300` registers; no
/// arguments, NiCollision): once only (byte `0126fa5c`), registers the class name at
/// `0109b0e8` with the creation function `00ead090` (`00a64900`), stores six function
/// addresses at `0126fa7c` to `0126fa90`, builds two 0x90-byte and two 0x18-byte objects of
/// function addresses (`00aa1070`; no check for a failed allocation, as in the exe), keeps
/// them in `0126fa94`, `0126fa98`, `0126fa9c` and `0126faa0` and stores three more
/// function addresses at `0126faa4` to `0126faac`.
pub fn fn_00e98ff0(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) != 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 1);
    e.call(REGISTER_LOADER, &args![0x0109_b0e8u32, 0x00ea_d090u32]);
    for (address, function) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, function);
    }
    let first = make_object(e, 0x90, &FIRST_OBJECT);
    let second = make_object(e, 0x90, &SECOND_OBJECT);
    let third = make_object(e, 0x18, &THIRD_OBJECT);
    let fourth = make_object(e, 0x18, &FOURTH_OBJECT);
    e.mem.set_u32(0x0126_fa98, second);
    e.mem.set_u32(0x0126_fa94, first);
    e.mem.set_u32(0x0126_fa9c, third);
    e.mem.set_u32(0x0126_faa0, fourth);
    for (address, function) in LAST_GLOBALS {
        e.mem.set_u32(address, function);
    }
}

/// `00a63520` (`cdecl`, one word, unit `NiMain/nistream.cpp`, no Xbox name): called by
/// `fn_00e99260` with the class name `fn_00e98ff0` registered.
const RELEASE_LOADER: u32 = 0x00a6_3520;
/// `00aa10f0` (`cdecl`, one word, no Xbox name): called on each of the four blocks
/// `fn_00e98ff0` built (by its use here the free paired with `00aa1070`; unconfirmed).
const FREE_BLOCK: u32 = 0x00aa_10f0;
/// The four object globals `fn_00e98ff0` fills, in the order `fn_00e99260` frees them.
const OBJECT_GLOBALS: [u32; 4] = [0x0126_fa94, 0x0126_fa98, 0x0126_fa9c, 0x0126_faa0];

// Translated from 00e99260 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; the exit handler `fn_00e99300` registers, no arguments,
/// `cdecl`): undoes `fn_00e98ff0` once (byte `0126fa5c` set): clears the byte, calls
/// `00a63520` with the class name `0109b0e8`, clears the six function globals, hands the
/// four objects (`0126fa94`, `0126fa98`, `0126fa9c`, `0126faa0`) to `00aa10f0` one by one
/// and clears those four and the three last globals (`0126faa4` to `0126faac`).
pub fn fn_00e99260(e: &mut Engine) {
    if e.mem.u8(OBJECTS_BUILT) == 0 {
        return;
    }
    e.mem.set_u8(OBJECTS_BUILT, 0);
    e.call(RELEASE_LOADER, &args![0x0109_b0e8u32]);
    let first = e.mem.u32(OBJECT_GLOBALS[0]);
    for (address, _) in FUNCTION_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    e.call(FREE_BLOCK, &args![first]);
    for address in &OBJECT_GLOBALS[1..] {
        let block = e.mem.u32(*address);
        e.call(FREE_BLOCK, &args![block]);
    }
    for address in OBJECT_GLOBALS {
        e.mem.set_u32(address, 0);
    }
    for (address, _) in LAST_GLOBALS {
        e.mem.set_u32(address, 0);
    }
}

/// The memory manager singleton and its `Allocate` (`thiscall`, the size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00e98aa0` (`thiscall`, one word, no Xbox name): run on the element `fn_00e99330` hands out.
const ELEMENT_SET: u32 = 0x00e9_8aa0;
/// `00a90180` (`thiscall`, one word: the new capacity; unit `NiMain/nioldscreenspace.cpp` in
/// the engine map, no Xbox name): grows the pool's pointer array (by its use here).
const ARRAY_GROW: u32 = 0x00a9_0180;
/// `00ba3aa0` (`thiscall`, (index, address of the pointer), no Xbox name): stores a pointer
/// into the pool's array (by its use here).
const ARRAY_STORE: u32 = 0x00ba_3aa0;

// Translated from 00e99330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): hands out the next element of
/// a pool of 0x44-byte objects. The pool keeps its pointer array at +0x4, the array's
/// capacity (`u16`) at +0x8, its length (`u16`) at +0xA, the growth step (`u16`) at +0xE
/// and the count handed out at +0x10. While the count is below the length, the element at
/// that index gets `value` through `00e98aa0`. Otherwise a new 0x44-byte block comes from
/// the memory manager (when it is not null: words +0x0 and +0x4 zeroed, the floats at
/// +0x34 to +0x40 set to 0.0, then `00e98aa0` with `value`); when the length has reached
/// the capacity the array grows to step + length (`00a90180`), and the new pointer (0 when
/// the allocation failed) is stored at index length (`00ba3aa0`). Either way the count goes
/// up by one and the element is answered.
pub fn fn_00e99330(e: &mut Engine, this: u32, value: u32) -> u32 {
    let length = u32::from(e.mem.u16(this + 0xa));
    let handed_out = e.mem.u32(this + 0x10);
    if handed_out < length {
        let array = e.mem.u32(this + 0x4);
        let element = e.mem.u32(array.wrapping_add(handed_out.wrapping_mul(4)));
        e.call(ELEMENT_SET, &args![element, value]);
        let count = e.mem.u32(this + 0x10);
        e.mem.set_u32(this + 0x10, count.wrapping_add(1));
        return element;
    }
    let element = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x44u32])
        .u32();
    if element != 0 {
        e.mem.set_u32(element, 0);
        e.mem.set_u32(element + 0x4, 0);
        for offset in [0x34, 0x38, 0x3c, 0x40] {
            e.mem.set_f32(element + offset, 0.0);
        }
        e.call(ELEMENT_SET, &args![element, value]);
    }
    let capacity = u32::from(e.mem.u16(this + 0x8));
    let length = u32::from(e.mem.u16(this + 0xa));
    if length >= capacity {
        let step = u32::from(e.mem.u16(this + 0xe));
        e.call(ARRAY_GROW, &args![this, step + length]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), element);
        e.call(ARRAY_STORE, &args![this, length, slot]);
    });
    let count = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0x10, count.wrapping_add(1));
    element
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_9300, fn_00e99300(Ptr) -> Ptr),
        entry!(0x00e9_8ff0, fn_00e98ff0()),
        entry!(0x00e9_9260, fn_00e99260()),
        entry!(0x00e9_9330, fn_00e99330(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_00e99260_releases_once_and_clears_the_globals() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(RELEASE_LOADER, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        // Not built: nothing.
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        e.mem.set_u8(OBJECTS_BUILT, 1);
        for (i, address) in (0x5000u32..).zip(OBJECT_GLOBALS) {
            e.mem.set_u32(address, i);
        }
        for (address, f) in FUNCTION_GLOBALS.iter().chain(LAST_GLOBALS.iter()) {
            e.mem.set_u32(*address, *f);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e9_9260, &[]);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 0);
        assert_eq!(calls_to(&e, RELEASE_LOADER), vec![vec![0x0109_b0e8]]);
        assert_eq!(
            calls_to(&e, FREE_BLOCK),
            vec![vec![0x5000], vec![0x5001], vec![0x5002], vec![0x5003]]
        );
        for address in (0x0126_fa7c..=0x0126_faac).step_by(4) {
            assert_eq!(e.mem.u32(address), 0, "{address:08x}");
        }
    }

    #[test]
    fn fn_00e99330_reuses_then_allocates_and_grows() {
        let mut e = Engine::new();
        let pool = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let reused = e.mem.alloc(0x44);
        e.mem.set_u32(pool + 0x4, array);
        e.mem.set_u32(array, reused);
        e.mem.set_u16(pool + 0x8, 2);
        e.mem.set_u16(pool + 0xa, 1);
        e.mem.set_u16(pool + 0xe, 3);
        e.register(ELEMENT_SET, |_, _| Ret::default());
        e.register(ARRAY_GROW, |_, _| Ret::default());
        let fresh = e.mem.alloc(0x44);
        e.mem.set_u32(fresh, 0xffff_ffff);
        e.mem.set_u32(fresh + 0x40, 0xffff_ffff);
        e.register_double(MEMORY_ALLOCATE, move |_, _| fresh.into_ret());
        e.register_double(ARRAY_STORE, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), fresh);
            Ret::default()
        });

        // Count 0 < length 1: the element at index 0.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 9]).u32(), reused);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![reused, 9]]);
        assert_eq!(e.mem.u32(pool + 0x10), 1);

        // Count 1 == length 1 < capacity 2: a new block, stored at index 1, no growth.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), fresh);
        assert_eq!(e.mem.u32(fresh), 0);
        assert_eq!(e.mem.u32(fresh + 0x40), 0);
        assert_eq!(calls_to(&e, ELEMENT_SET), vec![vec![fresh, 7]]);
        assert!(calls_to(&e, ARRAY_GROW).is_empty());
        let stores = calls_to(&e, ARRAY_STORE);
        assert_eq!(stores.len(), 1);
        assert_eq!(&stores[0][..2], &[pool, 1]);
        assert_eq!(e.mem.u32(pool + 0x10), 2);

        // Length 2 == capacity 2: grows to 3 + 2 first; a failed allocation stores 0.
        e.mem.set_u16(pool + 0xa, 2);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.register(ARRAY_STORE, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9330, &[pool, 7]).u32(), 0);
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![pool, 5]]);
        assert!(calls_to(&e, ELEMENT_SET).is_empty());
        assert_eq!(e.mem.u32(pool + 0x10), 3);
    }
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn registers_the_handler_on_the_first_use_only() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(REGISTER_EXIT_HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_9300, &args![0x1234u32]).u32(), 0x1234);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 1);
        assert_eq!(
            calls_to(&mut e, REGISTER_EXIT_HANDLER),
            vec![vec![0x00e9_8ff0, 0x00e9_9260]]
        );
        e.call(0x00e9_9300, &args![0x1234u32]);
        assert_eq!(e.global::<u32>(USE_COUNT_0126FA60), 2);
        assert_eq!(calls_to(&mut e, REGISTER_EXIT_HANDLER).len(), 1);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_four_objects_once() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.register(REGISTER_LOADER, |_, _| Ret::default());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        let loaders: Vec<_> = e
            .call_log
            .clone()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == REGISTER_LOADER)
            .collect();
        assert_eq!(
            loaders,
            vec![(REGISTER_LOADER, vec![0x0109_b0e8, 0x00ea_d090])]
        );
        let first = e.mem.u32(0x0126_fa94);
        let second = e.mem.u32(0x0126_fa98);
        let third = e.mem.u32(0x0126_fa9c);
        let fourth = e.mem.u32(0x0126_faa0);
        assert_eq!(e.mem.u32(first), 0x00ea_b9d0);
        assert_eq!(e.mem.u32(first + 0x60), 0x00e9_e980);
        assert_eq!(e.mem.u32(first + 0x8c), 0x00e9_d070);
        assert_eq!(e.mem.u32(second), 0x00ea_bb20);
        assert_eq!(e.mem.u32(second + 0x60), 0x00e9_eae0);
        assert_eq!(e.mem.u32(second + 0x70), 0x00e9_eb60);
        assert_eq!(e.mem.u32(third), 0x00ea_c240);
        assert_eq!(e.mem.u32(third + 0x14), 0x00e9_d150);
        assert_eq!(e.mem.u32(fourth), 0x00ea_c3f0);
        assert_eq!(e.mem.u32(fourth + 0x14), 0x00e9_e2c0);
        assert_eq!(e.mem.u32(0x0126_fa80), 0x00ea_7490);
        assert_eq!(e.mem.u32(0x0126_faac), 0x00e9_c850);
        assert_eq!(e.mem.u8(OBJECTS_BUILT), 1);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8ff0, &args![]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }
}
