//! `(unplaced)/run_004f05a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f0770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f0770 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x37 (0x38 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_397c`) is `TESRegion`, so this is the constructor of `TESRegion`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It allocates two objects with `operator new` (0xc and 8 bytes) and, when an allocation succeeds, constructs it (`004f3330(object, 1)` and `0096a2d0(object)`) and keeps the result at `+0x18` and `+0x1c` (null otherwise); it passes three zero floats to `00416870` for the member at `+0x2c`, and copies the float at `01012054` to `+0x28`.
/// The exception-unwinding frame is not translated.
pub fn fn_004f0770(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0102_397c);
    e.call(0x0041_6870, &args![base + 0x2c, 0x0u32, 0x0u32, 0x0u32]);
    let v3 = e.call(0x0040_1000, &args![0xcu32]).u32();
    let v5 = if v3 != 0 {
        e.call(0x004f_3330, &args![v3, 0x1u32]).u32()
    } else {
        0x0
    };
    e.mem.set_u32(base + 0x18, v5);
    let v6 = e.call(0x0040_1000, &args![0x8u32]).u32();
    let v8 = if v6 != 0 {
        e.call(0x0096_a2d0, &args![v6]).u32()
    } else {
        0x0
    };
    e.mem.set_u32(base + 0x1c, v8);
    e.call(0x004f_15a0, &args![this, 0x37u32]);
    e.mem.set_u32(base + 0x20, 0x0);
    e.mem.set_u32(base + 0x24, 0x0);
    let f1 = e.global::<f32>(0x0101_2054);
    e.mem.set_f32(base + 0x28, f1);
    this
}

// Translated from 004f0890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f08c0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f0890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_08c0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f3360` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f09e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_3360, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f0a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004f7740` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004f0a10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_7740, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004f05a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `004f0360` on `this` with 8 (the engine map has no name; files
/// `004f0360` under `bsbipnode.cpp`); its `EAX` is answered as it is.
pub fn fn_004f05a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x004f_0360, &args![this, 8u32]).u32()
}

/// `DynamicCast` (`00653270`, `cdecl (rtti, object)`) and the two `NiRTTI`
/// records `004f05c0` tries (`011f36bc`, `011f36e4`).
const DYNAMIC_CAST: u32 = 0x0065_3270;
const FIRST_CAST_RTTI: u32 = 0x011f_36bc;
const SECOND_CAST_RTTI: u32 = 0x011f_36e4;

// Translated from 004f05c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a list from its first entry (`0043b230`) to the end (each next
/// from `004a8a90`), the engine map has no name: an entry of the class
/// `011f36bc` gets `004f0640` with `value`, one of the class `011f36e4`
/// gets `004f06d0` with `value` (both tried on every entry).
pub fn fn_004f05c0(e: &mut Engine, this: Ptr, value: u32) {
    let mut entry = e.call(0x0043_b230, &args![this]).u32();
    while entry != 0 {
        let first = e.call(DYNAMIC_CAST, &args![FIRST_CAST_RTTI, entry]).u32();
        let second = e.call(DYNAMIC_CAST, &args![SECOND_CAST_RTTI, entry]).u32();
        if first != 0 {
            fn_004f0640(e, Ptr::new(first), value);
        }
        if second != 0 {
            e.call(0x004f_06d0, &args![second, value]);
        }
        entry = e.call(0x004a_8a90, &args![entry]).u32();
    }
}

/// The "no index" 16-bit value (`01096320`).
const INVALID_INDEX: u32 = 0x0109_6320;

// Translated from 004f0640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits `object` and, when it is a node, its children, recursively (the
/// engine map has no name): a non-null object found in this collection
/// (`0043b3b0` answers its 16-bit index, not `01096320`) is handed to
/// `0043b320` with that index and 0. Then the object's vtable +0xC (asked
/// even for a null object, as the exe does) answers the node; each of its
/// children (`0043b480` the count, `0043b4a0` the child) is visited.
pub fn fn_004f0640(e: &mut Engine, this: Ptr, object: u32) {
    if object != 0 {
        let index = e.call(0x0043_b3b0, &args![this, object]).u16();
        if index != e.global::<u16>(INVALID_INDEX) {
            e.call(0x0043_b320, &args![this, u32::from(index), 0u32]);
        }
    }
    let node = e.vcall(object, 0xc, &args![]).u32();
    if node == 0 {
        return;
    }
    let mut child = 0u32;
    while child < e.call(0x0043_b480, &args![node]).u32() {
        let item = e.call(0x0043_b4a0, &args![node, child]).u32();
        fn_004f0640(e, this, item);
        child = child.wrapping_add(1);
    }
}

// Translated from 004f08c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f08c0 - its C++ exception frame (FS:[0]) is not translated
/// The `TESRegion` destructor body (the engine map has no name; the
/// vtable is the one `004f0770` gives `TESRegion`): the vtable, the object
/// at +0x18 deleted (`004f09e0` with 1), every element taken off the list
/// at +0x1C (`006815c0` answers the first slot, `0063f7b0` removes it) and
/// deleted (`004f0a10` with 1), the list itself deleted (`004702f0` with
/// 1), then `TESForm::~TESForm`.
pub fn fn_004f08c0(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, 0x0102_397c);
    let first = e.mem.u32(object + 0x18);
    if first != 0 {
        fn_004f09e0(e, Ptr::new(first), 1);
    }
    if e.mem.u32(object + 0x1c) == 0 {
        e.call(0x0048_3630, &args![this]);
        return;
    }
    loop {
        let list = e.mem.u32(object + 0x1c);
        let slot = e.call(0x0068_15c0, &args![list]).u32();
        let element = e.mem.u32(slot);
        if element == 0 {
            break;
        }
        let list = e.mem.u32(object + 0x1c);
        e.call(0x0063_f7b0, &args![list]);
        fn_004f0a10(e, Ptr::new(element), 1);
    }
    let list = e.mem.u32(object + 0x1c);
    if list != 0 {
        e.call(0x0047_02f0, &args![list, 1u32]);
    }
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_0770, fn_004f0770(Ptr) -> Ptr),
        entry!(0x004f_0890, fn_004f0890(Ptr, u32) -> Ptr),
        entry!(0x004f_09e0, fn_004f09e0(Ptr, u32) -> Ptr),
        entry!(0x004f_0a10, fn_004f0a10(Ptr, u32) -> Ptr),
        entry!(0x004f_05a0, fn_004f05a0(Ptr) -> u32),
        entry!(0x004f_05c0, fn_004f05c0(Ptr, u32)),
        entry!(0x004f_0640, fn_004f0640(Ptr, u32)),
        entry!(0x004f_08c0, fn_004f08c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004f0770_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_0770,
            size: 0x38,
            globals: &[(0x0101_2054, 0x0100_0001)],
            returns: &[
                (0x0040_1000, 0x2000_2000),
                (0x004f_3330, 0x2000_3000),
                (0x0096_a2d0, 0x2000_4000),
            ],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0041_6870, &[A::T(0x2c), A::V(0x0), A::V(0x0), A::V(0x0)]),
                (0x0040_1000, &[A::V(0xc)]),
                (0x004f_3330, &[A::V(0x2000_2000), A::V(0x1)]),
                (0x0040_1000, &[A::V(0x8)]),
                (0x0096_a2d0, &[A::V(0x2000_2000)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x37)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_397c),
                (0x18, 4, 0x2000_3000),
                (0x1c, 4, 0x2000_4000),
                (0x20, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0100_0001),
            ],
        });
    }

    #[test]
    fn fn_004f0770_without_memory_leaves_the_objects_null() {
        // `operator new` answers null: neither object is constructed.
        ctor_test::run(&ctor_test::Case {
            addr: 0x004f_0770,
            size: 0x38,
            globals: &[(0x0101_2054, 0x0100_0001)],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0041_6870, &[A::T(0x2c), A::V(0x0), A::V(0x0), A::V(0x0)]),
                (0x0040_1000, &[A::V(0xc)]),
                (0x0040_1000, &[A::V(0x8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x37)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_397c),
                (0x18, 4, 0x0),
                (0x1c, 4, 0x0),
                (0x20, 4, 0x0),
                (0x24, 4, 0x0),
                (0x28, 4, 0x0100_0001),
            ],
        });
    }
    #[test]
    fn at_004f0890_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_08c0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_0890, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_0890)
                .collect();
            let mut expected = vec![(0x004f_08c0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f09e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_3360, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_09e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_09e0)
                .collect();
            let mut expected = vec![(0x004f_3360, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004f0a10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_7740, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_0a10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_0a10)
                .collect();
            let mut expected = vec![(0x004f_7740, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    use super::*;

    fn region_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_004f05a0_passes_eight() {
        let mut e = Engine::new();
        e.register(0x004f_0360, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(e.call(0x004f_05a0, &args![0x1000u32]).u32(), 0x1008);
    }

    #[test]
    fn list_walk_casts_each_entry_both_ways() {
        let mut e = Engine::new();
        // Entries 0x10 and 0x20; 0x10 is of the first class, 0x20 of both.
        e.register(0x0043_b230, |_, _| 0x10u32.into_ret());
        e.register(0x004a_8a90, |_, a| {
            (if a[0] == 0x10 { 0x20u32 } else { 0 }).into_ret()
        });
        e.register(DYNAMIC_CAST, |_, a| {
            match (a[0], a[1]) {
                (FIRST_CAST_RTTI, entry) => entry + 1,
                (SECOND_CAST_RTTI, 0x20) => 0x22,
                _ => 0,
            }
            .into_ret()
        });
        e.register(0x004f_06d0, |_, _| Ret::default());
        // The first-class handler is this file's 004f0640: its callees.
        e.register(0x0043_b3b0, |_, _| 0xffffu32.into_ret());
        e.map(INVALID_INDEX, 4);
        e.set_global(INVALID_INDEX, 0xffffu16);
        // The value handed on is an object that is no node.
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, 0x0300_0000);
        e.register(0x0300_0000, |_, _| Ret::default());
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, vtable);
        e.call_log = Some(vec![]);
        e.call(0x004f_05c0, &args![0x9000u32, value]);
        assert_eq!(region_calls_to(&e, 0x004f_06d0), vec![vec![0x22, value]]);
        assert_eq!(
            region_calls_to(&e, 0x0043_b3b0),
            vec![vec![0x11, value], vec![0x21, value]]
        );
    }

    #[test]
    fn node_visit_handles_found_objects_and_recurses_into_children() {
        const IS_NODE: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.map(INVALID_INDEX, 4);
        e.set_global(INVALID_INDEX, 0xffffu16);
        // Object 0x100 is a node (itself) with children 0x200 and 0x300;
        // only 0x300 is in the collection, at index 5.
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 0xc, IS_NODE);
        for object in [0x100u32, 0x200, 0x300] {
            e.map(object, 4);
            e.mem.set_u32(object, vtable);
        }
        e.register(IS_NODE, |_, a| {
            (if a[0] == 0x100 { a[0] } else { 0 }).into_ret()
        });
        e.register(0x0043_b480, |_, _| 2u32.into_ret());
        e.register(0x0043_b4a0, |_, a| (0x200 + 0x100 * a[1]).into_ret());
        e.register(0x0043_b3b0, |_, a| {
            (if a[1] == 0x300 { 5u32 } else { 0xffff }).into_ret()
        });
        e.register(0x0043_b320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_0640, &args![0x9000u32, 0x100u32]);
        assert_eq!(region_calls_to(&e, 0x0043_b320), vec![vec![0x9000, 5, 0]]);
        assert_eq!(region_calls_to(&e, 0x0043_b3b0).len(), 3);
    }

    #[test]
    fn region_destructor_deletes_its_members_then_runs_the_form_destructor() {
        let mut e = Engine::new();
        for addr in [
            0x004f_3360u32,
            0x004f_7740,
            0x0040_1030,
            0x0047_02f0,
            0x0048_3630,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x38);
        let list = e.mem.alloc(0x10);
        // The list's first slot holds 0x5000, then (after one removal) null.
        e.register_double(0x0068_15c0, move |_, a| (a[0] + 4).into_ret());
        e.register_double(0x0063_f7b0, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.mem.set_u32(list + 4, 0x5000);
        e.mem.set_u32(this + 0x18, 0x6000);
        e.mem.set_u32(this + 0x1c, list);
        e.call_log = Some(vec![]);
        e.call(0x004f_08c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0102_397c);
        assert_eq!(region_calls_to(&e, 0x004f_3360), vec![vec![0x6000]]);
        assert_eq!(region_calls_to(&e, 0x004f_7740), vec![vec![0x5000]]);
        assert_eq!(region_calls_to(&e, 0x0047_02f0), vec![vec![list, 1]]);
        assert_eq!(region_calls_to(&e, 0x0048_3630), vec![vec![this]]);
        // Without members only the vtable and the form destructor.
        let bare = e.mem.alloc(0x38);
        e.call_log = Some(vec![]);
        e.call(0x004f_08c0, &args![bare]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
