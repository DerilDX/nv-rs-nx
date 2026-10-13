//! `fallout shared/pathfinding/pathsmootherpov.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `00726bf0(this, address, count)`: runs a no-result per-element step over
/// `count` words (the unit `fallout/interface/run_00726bf0`).
const FILL_ELEMENTS: u32 = 0x0072_6bf0;

// Translated from 006b3eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor shape of a `BSSimpleArray`
/// (buffer at +4, size at +8, reserved size at +0xc; the three start at 0).
/// `reserved_hint` and `size` are the two stack words: the reserved size is
/// the larger of the two (unsigned). When it is nonzero the buffer comes from
/// virtual slot +4 of the object (argument: the reserved size) and
/// +0xc is set; when `size` is nonzero the buffer's `size` elements are
/// prepared with `00726bf0` and +8 is set. The class is not confirmed from
/// the exe, only the layout.
pub fn fn_006b3eb0(e: &mut Engine, this: Ptr<BSSimpleArray>, reserved_hint: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let mut reserved = reserved_hint;
    if size > reserved {
        reserved = size;
    }
    if reserved > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserved]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, reserved);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(FILL_ELEMENTS, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x006b3eb0,
        fn_006b3eb0(Ptr<BSSimpleArray>, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object whose virtual slot +4 hands out a fixed buffer and records
    /// the requested size in the object's first dword after the vtable.
    fn array(e: &mut Engine) -> (Ptr<BSSimpleArray>, u32) {
        let buffer = e.mem.alloc(64);
        e.put_vtable(0x4300_0000, &[0, 0x4400_0000]);
        e.register_double(0x4400_0000, move |_, _| Ret {
            eax: buffer,
            ..Ret::default()
        });
        let this = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(this.addr(), 0x4300_0000);
        (this, buffer)
    }

    #[test]
    fn size_larger_than_hint_reserves_the_size_and_fills() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| Ret::default());
        let (this, buffer) = array(&mut e);
        e.set(this, BSSimpleArray::iSize, 99);
        e.call_log = Some(vec![]);
        e.call(0x006b3eb0, &args![this, 2u32, 5u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 5);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x4400_0000, vec![this.addr(), 5]));
        assert_eq!(log[2], (FILL_ELEMENTS, vec![this.addr(), buffer, 5]));
    }

    #[test]
    fn a_larger_hint_is_the_reserved_size() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| Ret::default());
        let (this, buffer) = array(&mut e);
        e.call(0x006b3eb0, &args![this, 8u32, 3u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 3, "size 3 is filled");
    }

    #[test]
    fn zero_zero_allocates_nothing() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| panic!("nothing to prepare"));
        let (this, _) = array(&mut e);
        e.mem.set_u32(this.addr() + 4, 0x1111);
        e.call(0x006b3eb0, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn hint_without_size_reserves_and_leaves_size_zero() {
        let mut e = Engine::new();
        e.register(FILL_ELEMENTS, |_, _| panic!("no elements to prepare"));
        let (this, buffer) = array(&mut e);
        e.call(0x006b3eb0, &args![this, 8u32, 0u32]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), buffer);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 8);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
    }
}
