//! `NiSystem/run_00aa1380` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Gamebryo allocator reference: the word at `011f6080` points to a word
/// that holds the allocator object (set up by `00aa2020`).
const ALLOCATOR_REFERENCE: u32 = 0x011f_6080;
/// The allocator's allocation method, byte offset in its vtable.
const ALLOCATE_SLOT: u32 = 0x4;

// Translated from 00aa13e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `size` bytes (a request for 0 is made for 1) through the
/// Gamebryo allocator's virtual allocation method (vtable `+4`), the sibling
/// of `00aa1070`; it passes 1, 1 where that passes 5, 0. The size and an
/// word holding 4 are passed by address, as
/// locals of this function. Returns the allocator's result (`EAX`), which the
/// decompiler does not show: callers use it as the new block.
///
/// Arguments of the call, in order: size address, address of the 4, 1, 1,
/// 0, -1, 0 (their names are not known).
pub fn fn_00aa13e0(e: &mut Engine, size: u32) -> Ptr {
    let size = if size == 0 { 1 } else { size };
    let reference = e.mem.u32(ALLOCATOR_REFERENCE);
    let allocator = e.mem.u32(reference);
    e.with_stack(8, |e, locals| {
        e.mem.set_u32(locals.addr(), size);
        e.mem.set_u32(locals.addr() + 4, 4);
        e.vcall(
            allocator,
            ALLOCATE_SLOT,
            &args![
                locals.addr(),
                locals.addr() + 4,
                1u32,
                1u32,
                0u32,
                0xffff_ffffu32,
                0u32
            ],
        )
        .ptr()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa13e0, fn_00aa13e0(u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATE: u32 = 0x0070_0000;
    const VTABLE: u32 = 0x0300_0000;

    /// An allocator whose allocation method records the values behind the two
    /// address arguments and the remaining arguments, and returns 0x4242.
    fn engine_with_allocator() -> (Engine, std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let allocator = e.mem.alloc(8);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, allocator);
        e.mem.set_u32(ALLOCATOR_REFERENCE, holder);
        e.put_vtable(VTABLE, &[0, ALLOCATE]);
        e.mem.set_u32(allocator, VTABLE);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(ALLOCATE, move |e, a| {
            let mut words = vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[2])];
            words.extend_from_slice(&a[3..]);
            log.borrow_mut().push(words);
            Ret {
                eax: 0x4242,
                ..Ret::default()
            }
        });
        (e, seen)
    }

    #[test]
    fn allocates_through_the_virtual_method() {
        let (mut e, seen) = engine_with_allocator();
        let allocator = e.mem.u32(e.mem.u32(ALLOCATOR_REFERENCE));
        let result = e.call(0x00aa_13e0, &args![0x280u32]).u32();
        assert_eq!(result, 0x4242);
        assert_eq!(
            seen.borrow()[0],
            vec![allocator, 0x280, 4, 1, 1, 0, 0xffff_ffff, 0]
        );
    }

    #[test]
    fn a_zero_size_is_requested_as_one() {
        let (mut e, seen) = engine_with_allocator();
        e.call(0x00aa_13e0, &args![0u32]);
        assert_eq!(seen.borrow()[0][1], 1);
    }
}
