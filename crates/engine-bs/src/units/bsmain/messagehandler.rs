//! `BSMain/messagehandler.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton and its `Deallocate` (`00aa4060`,
/// `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00c4a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes,1024>::_Deallocate` (engine-map name; the
/// body is the generic one, so identical code of other element types may
/// share it): gives `buffer` back to the memory manager. `this` is not read.
pub fn bs_simple_array_ni_frustum_planes_deallocate(e: &mut Engine, _this: Ptr, buffer: u32) {
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
}

// Translated from 00c3c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MessageHandler::Output` (engine map): a variadic `cdecl` function (its callers push a
/// handler word, a format and its values, and clean the stack themselves) whose body only
/// answers 0; none of its words is read.
pub fn message_handler_output(_e: &mut Engine, _words: &[u32]) -> u32 {
    0
}

/// The uniform form of [`message_handler_output`]: every word the caller pushed.
fn message_handler_output_entry(e: &mut Engine, words: &[u32]) -> Ret {
    message_handler_output(e, words).into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4_a890,
            bs_simple_array_ni_frustum_planes_deallocate(Ptr, u32)
        ),
        (0x00c3_c220, message_handler_output_entry as AbiFn),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deallocate_gives_the_buffer_to_the_memory_manager() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_a890, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MEMORY_MANAGER_DEALLOCATE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, [MEMORY_MANAGER, 0x2000]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn output_answers_zero_whatever_it_is_given() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_c220, &args![]).u32(), 0);
        let words = args![0x0118_6084u32, 0x0101_7acc_u32, 1u32, 2u32, 3u32];
        assert_eq!(e.call(0x00c3_c220, &words).u32(), 0);
    }
}
