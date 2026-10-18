//! `fallout/ai/spectatorpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `0072ba80(this, address, count)`: the per-element destroy step
/// (translated in `fallout/interface/menus/recipemenu.cpp`).
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
/// `0042fc00(this, new_reserved, size)` (`extradatalist.cpp`): resizes the
/// buffer for the new reserved size.
const RESIZE_BUFFER: u32 = 0x0042_fc00;

// Translated from 0084b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<Actor *, 1024>::SetReservedSize` (engine map name; the
/// body sets the reserved size field, +0xc). Nothing happens when the
/// reserved size is unchanged. When the new size is below `iSize`, the
/// elements from the new size on are destroyed (`0072ba80`) and `iSize` is
/// cut to the new size. Then the buffer is resized (`0042fc00`, new size and
/// current `iSize`) and the reserved size is stored.
pub fn bs_simple_array_set_reserved_size(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32) {
    if new_size == e.get(this, BSSimpleArray::iReservedSize) {
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let first = buffer.wrapping_add(new_size.wrapping_mul(4));
        e.call(DESTROY_ELEMENTS, &args![this, first, size - new_size]);
        e.set(this, BSSimpleArray::iSize, new_size);
    }
    let size = e.get(this, BSSimpleArray::iSize);
    e.call(RESIZE_BUFFER, &args![this, new_size, size]);
    e.set(this, BSSimpleArray::iReservedSize, new_size);
}

/// `009f8ac0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_009F8AC0: u32 = 0x009f_8ac0;

// Translated from 009f8a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (spectator package; callers `009f8420`, `009f8020`, `009f76e0`):
/// forwards `this` and an index to `009f8ac0`, which returns the word at `this+4` plus `index`
/// times 0x34: the address of a 0x34-byte element. The wrapper returns that address.
pub fn fn_009f8a80(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_009F8AC0, &args![this, index]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSpectatorThreatInfo@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01093980: u32 = 0x0109_3980;
/// `009f8d00` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_009F8D00: u32 = 0x009f_8d00;

// Translated from 009f8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSpectatorThreatInfo@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `009f8d00(this, 1)`.
pub fn fn_009f8b10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01093980);
    e.call(ARRAY_RELEASE_009F8D00, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_8a80, fn_009f8a80(Ptr, u32) -> Ptr),
        entry!(
            0x0084b5c0,
            bs_simple_array_set_reserved_size(Ptr<BSSimpleArray>, u32)
        ),
        entry!(0x009f_8b10, fn_009f8b10(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> (Engine, Ptr<BSSimpleArray>) {
        let mut e = Engine::new();
        e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
        e.register(RESIZE_BUFFER, |_, _| Ret::default());
        let this = e.new_object::<BSSimpleArray>();
        e.set(this, BSSimpleArray::pBuffer, 0x5000);
        e.set(this, BSSimpleArray::iSize, 6);
        e.set(this, BSSimpleArray::iReservedSize, 8);
        (e, this)
    }

    #[test]
    fn an_unchanged_reserved_size_does_nothing() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 8u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "only the call itself");
        assert_eq!(e.get(this, BSSimpleArray::iSize), 6);
    }

    #[test]
    fn shrinking_below_the_size_destroys_the_tail() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (DESTROY_ELEMENTS, vec![this.addr(), 0x5010, 2]));
        assert_eq!(log[2], (RESIZE_BUFFER, vec![this.addr(), 4, 4]));
        assert_eq!(e.get(this, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
    }

    #[test]
    fn growing_keeps_the_elements() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (RESIZE_BUFFER, vec![this.addr(), 20, 6]));
        assert_eq!(e.get(this, BSSimpleArray::iSize), 6);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 20);
    }

    #[test]
    fn a_new_size_equal_to_the_size_destroys_nothing() {
        let (mut e, this) = world();
        e.call_log = Some(vec![]);
        e.call(0x0084b5c0, &args![this, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2, "no destroy step when new size == iSize");
        assert_eq!(log[1], (RESIZE_BUFFER, vec![this.addr(), 6, 6]));
    }

    #[test]
    fn forwarder_fn_009f8a80_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_009F8AC0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_8a80, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_009F8AC0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
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
    fn fn_009f8b10_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009f_8b10, ARRAY_VTABLE_01093980, ARRAY_RELEASE_009F8D00);
    }
}
