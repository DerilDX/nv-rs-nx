//! `(unplaced)/run_006ecd60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `006ed010` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_006ED010: u32 = 0x006e_d010;
/// The element-array constructor helper `00ec782f` (cdecl-looking, `RET 0x14`: array, element
/// size, count, element constructor, element destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The element constructor (`006ed0e0`) and destructor (`00483710`) handed to it.
const ELEMENT_CONSTRUCTOR_006ED0E0: u32 = 0x006e_d0e0;
const ELEMENT_DESTRUCTOR_00483710: u32 = 0x0048_3710;
/// The follow-up call `006ed040` (`this` plus two stack words, `RET 8`).
const FOLLOW_UP_006ED040: u32 = 0x006e_d040;
/// The vtable stored at `+0` (RTTI class `BSTStaticFreeList<ActorPathingMessage,256>`).
const VTABLE_006ECE90: u32 = 0x0106_d798;

// Translated from 006ece90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ece90 - the compiler's exception frame (FS:[0] chain and the
// unwinding that would destroy the element array) is not translated.
/// Unnamed in the engine map: runs `006ed010` on `this`, stores the vtable `0106d798` (RTTI
/// class `BSTStaticFreeList<ActorPathingMessage,256>`), constructs 0x100 elements of 0xc
/// bytes at `this + 0xc` through `00ec782f` (constructor `006ed0e0`, destructor `00483710`),
/// then calls `006ed040(this, 0x100, this + 0xc)`. Returns `this`.
pub fn fn_006ece90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_006ED010, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_006ECE90);
    let elements = this.addr() + 0xc;
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            elements,
            0xcu32,
            0x100u32,
            ELEMENT_CONSTRUCTOR_006ED0E0,
            ELEMENT_DESTRUCTOR_00483710
        ],
    );
    e.call(FOLLOW_UP_006ED040, &args![this, 0x100u32, elements]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_ce90, fn_006ece90(Ptr) -> Ptr)]
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
    fn constructs_the_base_the_elements_and_the_free_list() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_006ED010, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            // The vtable is already stored when the elements are constructed.
            let seen = e.mem.u32(a[0] - 0xc);
            e.mem.set_u32(a[0], seen);
            Ret::default()
        });
        e.register(FOLLOW_UP_006ED040, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_ce90, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_006ED010),
            vec![vec![this]]
        );
        assert_eq!(
            calls_to(&mut e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0xc, 0xc, 0x100, 0x006e_d0e0, 0x0048_3710]]
        );
        assert_eq!(
            calls_to(&mut e, FOLLOW_UP_006ED040),
            vec![vec![this, 0x100, this + 0xc]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_006ECE90);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_006ECE90);
    }
}
