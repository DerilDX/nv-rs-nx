//! `BSShader/mtrenderingsystem.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The byte that enables the second rendering task thread (`011f94b4`).
const SECOND_THREAD_ENABLED_011F94B4: u32 = 0x011f_94b4;
/// The word passed as `AddThread`'s second argument (`01200080`).
const THREAD_ARGUMENT_01200080: u32 = 0x0120_0080;
/// `MTRenderingSystem::AddThread` (engine map, `00ba2e00`): five stack words, cleaned by the
/// callee.
const ADD_THREAD: u32 = 0x00ba_2e00;
/// The thread's code (`00ba3190`), passed as an address.
const THREAD_CODE_00BA3190: u32 = 0x00ba_3190;
/// `"Rendering Task Thread 2"` (`010b7f84`).
const THREAD_NAME: u32 = 0x010b_7f84;

// Translated from 00ba3720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MTRenderingSystem::CreateThreads` (Xbox PDB). No arguments: when the byte at `011f94b4`
/// is not 0, calls `MTRenderingSystem::AddThread` (`00ba2e00`) with 1, the word at
/// `01200080`, the code address `00ba3190`, the name `"Rendering Task Thread 2"`
/// (`010b7f84`) and 0x20000.
pub fn mt_rendering_system_create_threads(e: &mut Engine) {
    if e.global::<u8>(SECOND_THREAD_ENABLED_011F94B4) != 0 {
        let word = e.global::<u32>(THREAD_ARGUMENT_01200080);
        e.call(
            ADD_THREAD,
            &args![1u32, word, THREAD_CODE_00BA3190, THREAD_NAME, 0x20000u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba_3720, mt_rendering_system_create_threads())]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_thread_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ADD_THREAD)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn create_threads_adds_the_second_thread_only_when_enabled() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(THREAD_ARGUMENT_01200080, 0x4242u32);
        e.register(ADD_THREAD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ba_3720, &args![]);
        assert!(add_thread_calls(&e).is_empty());

        e.set_global(SECOND_THREAD_ENABLED_011F94B4, 1u8);
        e.call(0x00ba_3720, &args![]);
        assert_eq!(
            add_thread_calls(&e),
            vec![vec![1, 0x4242, 0x00ba_3190, 0x010b_7f84, 0x20000]]
        );
    }
}
