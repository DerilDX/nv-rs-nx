//! `Havok SDK/hkplistagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00d34810` (cdecl, five stack words, no result): the body `00d36930` jumps to. The engine
/// map files it under `hkpconvexlistagent.cpp`.
const LINEAR_CAST_BODY: u32 = 0x00d3_4810;

// Translated from 00d36930 (decompiled, FalloutNV.exe 1.4.0.525)
/// A bare `JMP 00d34810`: the five stack words go to that function unchanged (cdecl, nothing
/// is returned).
pub fn fn_00d36930(e: &mut Engine, a: u32, b: u32, c: u32, d: u32, f: u32) {
    e.call(LINEAR_CAST_BODY, &args![a, b, c, d, f]);
}

// Translated from 00d36960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListAgent::linearCast` (Xbox PDB): a virtual method that does not use `this` and passes
/// its five stack words on to `fn_00d36930`.
pub fn hkp_list_agent_linear_cast(
    e: &mut Engine,
    _this: Ptr,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    f: u32,
) {
    fn_00d36930(e, a, b, c, d, f);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d36930, fn_00d36930(u32, u32, u32, u32, u32)),
        entry!(
            0x00d36960,
            hkp_list_agent_linear_cast(Ptr, u32, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> (Engine, Rc<RefCell<Vec<Vec<u32>>>>) {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(LINEAR_CAST_BODY, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (e, seen)
    }

    #[test]
    fn the_jump_passes_all_five_words_on() {
        let (mut e, seen) = engine();
        e.call(0x00d36930, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(*seen.borrow(), vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn linear_cast_forwards_its_stack_words_and_drops_this() {
        let (mut e, seen) = engine();
        e.call(0x00d36960, &args![0x99u32, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(*seen.borrow(), vec![vec![1, 2, 3, 4, 5]]);
    }
}
