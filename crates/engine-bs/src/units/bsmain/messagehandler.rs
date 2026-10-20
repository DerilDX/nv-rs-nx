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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c4_a890,
        bs_simple_array_ni_frustum_planes_deallocate(Ptr, u32)
    )]
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
