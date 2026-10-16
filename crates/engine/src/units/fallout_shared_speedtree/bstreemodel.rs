//! `fallout shared/speedtree/bstreemodel.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006667d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00666800` and frees 0x50 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_tree_model_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0066_6800, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x50u32]);
    }
    this
}

// Translated from 006694e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00633c90` on `this` with a null argument and returns its answer unchanged (the code
/// does not touch `EAX` afterwards; `00633c90` stores the argument in the pointer slot at
/// `this` and, when it is not null, runs `0040f6e0` on it, and returns `this`). The engine map
/// gives it no name, so the class is unconfirmed.
pub fn fn_006694e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0063_3c90, &args![this, 0u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0066_67d0, bs_tree_model_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0066_94e0, fn_006694e0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_tree_model_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0066_67d0, 0x0066_6800, 0x50);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_006694e0_passes_a_null_target_and_returns_the_answer() {
        let mut e = Engine::new();
        e.register_double(0x0063_3c90, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0066_94e0, &args![Ptr::<()>::new(0x5000)]);
        assert_eq!(result.u32(), 0x1234);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0063_3c90, vec![0x5000, 0])]);
    }
}
