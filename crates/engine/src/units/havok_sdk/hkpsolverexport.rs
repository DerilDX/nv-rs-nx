//! `Havok SDK/hkpsolverexport.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkSolverExport::exportImpulsesAndRhs` (`00def570`, cdecl, four stack words).
const EXPORT_IMPULSES_AND_RHS: u32 = 0x00de_f570;

// Translated from 00df0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkExportImpulsesAndRhs` (Xbox PDB): a jump to `hkSolverExport::exportImpulsesAndRhs`
/// (`00def570`) with the caller's four stack words unchanged (that function reads exactly
/// four and returns nothing).
pub fn hk_export_impulses_and_rhs(
    e: &mut Engine,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) {
    e.call(
        EXPORT_IMPULSES_AND_RHS,
        &args![word_1, word_2, word_3, word_4],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00df_0680,
        hk_export_impulses_and_rhs(u32, u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_export_jumps_to_the_solver_export_with_the_same_words() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        let double_log = Rc::clone(&log);
        e.register_double(EXPORT_IMPULSES_AND_RHS, move |_, a| {
            double_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00df_0680, &args![1u32, 2u32, 3u32, 4u32]);
        assert_eq!(*log.borrow(), vec![vec![1, 2, 3, 4]]);
    }
}
