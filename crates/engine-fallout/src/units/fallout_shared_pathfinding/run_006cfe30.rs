//! `fallout shared/pathfinding/run_006cfe30` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006d0510` (unit `run_006d03a0`, `thiscall`, `RET 4`): an element's
/// scalar deleting destructor; called with flags 0 it only destroys.
const ELEMENT_DESTROY: u32 = 0x006d_0510;
/// `006d0540` (`thiscall`, `RET 0xc`): `(array, new buffer, old buffer,
/// count)`, moves the elements to a new buffer.
const MOVE_ELEMENTS: u32 = 0x006d_0540;
/// `006a8500` (`thiscall`): releases the array's current buffer.
const FREE_BUFFER: u32 = 0x006a_8500;
/// `006e6da0` (`cdecl`: size, place): placement `new`, answers the place.
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `PathingLocation::PathingLocation` (engine map), `thiscall`.
const LOCATION_CONSTRUCT: u32 = 0x006d_cbb0;

/// Destroys `count` elements of `stride` bytes from `first` on with
/// `006d0510(element, 0)`.
fn destroy_elements(e: &mut Engine, first: u32, count: u32, stride: u32) {
    let mut index = 0u32;
    while index < count {
        let element = index.wrapping_mul(stride).wrapping_add(first);
        e.call(ELEMENT_DESTROY, &args![element, 0u32]);
        index = index.wrapping_add(1);
    }
}

// Translated from 006cfe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` 0x30-byte elements (`VirtualPathingNode`s) from
/// `first` on (`thiscall`, `ECX` not read, `RET 8`; no name in the engine
/// map).
pub fn fn_006cfe30(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    destroy_elements(e, first, count, 0x30);
}

// Translated from 006cfe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reallocates the 0x30-byte-element array (`RET 8`; buffer at +4,
/// capacity at +0xc; no name in the engine map): with no buffer, the buffer
/// is the array's slot +4 with `capacity` and the capacity becomes
/// `capacity`; when `count` equals the capacity, slot +0xc reallocates the
/// buffer; otherwise slot +4 gives a new buffer, `006d0540` moves the
/// `count` elements, `006a8500` frees the old buffer and the new one is
/// stored.
pub fn fn_006cfe70(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let base = this.addr();
    if e.mem.u32(base + 4) == 0 {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, capacity);
    } else if count == e.mem.u32(base + 0xc) {
        let old = e.mem.u32(base + 4);
        let buffer = e.vcall(base, 0xc, &args![old, capacity]).u32();
        e.mem.set_u32(base + 4, buffer);
    } else {
        let buffer = e.vcall(base, 4, &args![capacity]).u32();
        let old = e.mem.u32(base + 4);
        e.call(MOVE_ELEMENTS, &args![this, buffer, old, count]);
        e.call(FREE_BUFFER, &args![this]);
        e.mem.set_u32(base + 4, buffer);
    }
}

// Translated from 006cff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `count` 0x3c-byte elements from `first` on (`thiscall`, `ECX`
/// not read, `RET 8`; no name in the engine map).
pub fn fn_006cff20(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    destroy_elements(e, first, count, 0x3c);
}

// Translated from 006d00a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006d00a0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Constructs `count` `PathingLocation`s (0x28 bytes) from `first` on
/// (`thiscall`, `ECX` not read, `RET 8`; no name in the engine map):
/// placement `new` (`006e6da0`) and, when it gives an address, the
/// `PathingLocation` constructor (`006dcbb0`).
pub fn fn_006d00a0(e: &mut Engine, _unused_0: Ptr, first: u32, count: u32) {
    let mut index = 0u32;
    while index < count {
        let place = e
            .call(
                PLACEMENT_NEW,
                &args![0x28u32, index.wrapping_mul(0x28).wrapping_add(first)],
            )
            .u32();
        if place != 0 {
            e.call(LOCATION_CONSTRUCT, &args![place]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 006d0140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingLocation` array's initialisation (`RET 8`; no name in the
/// engine map): clears the buffer, size and capacity; `size` is raised to
/// `used` (unsigned); with a non-zero size the buffer is slot +4 (allocate)
/// and the capacity `size`; with a non-zero `used` that many locations are
/// constructed (`006d00a0`) and the size is `used`.
pub fn fn_006d0140(e: &mut Engine, this: Ptr, size: u32, used: u32) {
    let base = this.addr();
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    let size = size.max(used);
    if size != 0 {
        let buffer = e.vcall(base, 4, &args![size]).u32();
        e.mem.set_u32(base + 4, buffer);
        e.mem.set_u32(base + 0xc, size);
    }
    if used != 0 {
        let buffer = e.mem.u32(base + 4);
        fn_006d00a0(e, this, buffer, used);
        e.mem.set_u32(base + 8, used);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_fe30, fn_006cfe30(Ptr, u32, u32)),
        entry!(0x006c_fe70, fn_006cfe70(Ptr, u32, u32)),
        entry!(0x006c_ff20, fn_006cff20(Ptr, u32, u32)),
        entry!(0x006d_00a0, fn_006d00a0(Ptr, u32, u32)),
        entry!(0x006d_0140, fn_006d0140(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_006cfe30_destroys_0x30_byte_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTROY, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006c_fe30, &args![0u32, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTROY),
            vec![vec![0x1000, 0], vec![0x1030, 0], vec![0x1060, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006c_fe30, &args![0u32, 0x1000u32, 0u32]);
        assert!(calls_to(&e, ELEMENT_DESTROY).is_empty());
    }

    #[test]
    fn fn_006cff20_destroys_0x3c_byte_elements() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTROY, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006c_ff20, &args![0u32, 0x1000u32, 2u32]);
        assert_eq!(
            calls_to(&e, ELEMENT_DESTROY),
            vec![vec![0x1000, 0], vec![0x103c, 0]]
        );
    }

    #[test]
    fn fn_006cfe70_takes_each_of_its_three_paths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000, 0, 0x0300_2000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(0x0300_2000, |_, a| word(a[1] + a[2]));
        e.register(MOVE_ELEMENTS, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006c_fe70, &args![this, 8u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7008);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        e.call(0x006c_fe70, &args![this, 0x10u32, 8u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7018);
        assert!(calls_to(&e, MOVE_ELEMENTS).is_empty());
        e.call(0x006c_fe70, &args![this, 0x20u32, 4u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7020);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(
            calls_to(&e, MOVE_ELEMENTS),
            vec![vec![this, 0x7020, 0x7018, 4]]
        );
        assert_eq!(calls_to(&e, FREE_BUFFER), vec![vec![this]]);
    }

    #[test]
    fn fn_006d00a0_constructs_where_the_placement_gives() {
        let mut e = Engine::new();
        e.register(PLACEMENT_NEW, |_, a| {
            word(if a[1] == 0x1028 { 0 } else { a[1] })
        });
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006d_00a0, &args![0u32, 0x1000u32, 3u32]);
        assert_eq!(
            calls_to(&e, PLACEMENT_NEW),
            vec![vec![0x28, 0x1000], vec![0x28, 0x1028], vec![0x28, 0x1050]]
        );
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT),
            vec![vec![0x1000], vec![0x1050]]
        );
    }

    #[test]
    fn fn_006d0140_allocates_and_constructs() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.put_vtable(0x0300_0000, &[0, 0x0300_1000]);
        e.mem.set_u32(this, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(0x7000 + a[1]));
        e.register(PLACEMENT_NEW, |_, a| word(a[1]));
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x006d_0140, &args![this, 1u32, 2u32]);
        assert_eq!(e.mem.u32(this + 4), 0x7002);
        assert_eq!(e.mem.u32(this + 8), 2);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(
            calls_to(&e, LOCATION_CONSTRUCT),
            vec![vec![0x7002], vec![0x702a]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006d_0140, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
    }
}
