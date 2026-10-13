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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0084b5c0,
        bs_simple_array_set_reserved_size(Ptr<BSSimpleArray>, u32)
    )]
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
}
