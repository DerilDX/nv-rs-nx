//! `NiParticle/nipsysmeshemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPSysMeshEmitter::ms_RTTI` (the class's `NiRTTI` record).
const NI_P_SYS_MESH_EMITTER_RTTI: u32 = 0x0120_2500;

// Translated from 00c1f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshEmitter::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_p_sys_mesh_emitter_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_P_SYS_MESH_EMITTER_RTTI)
}

/// The memory manager object (`ECX` of `00aa3e40`, the allocator).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The element constructor `00b54620` handed to the vector constructor loop.
const ELEMENT_CONSTRUCTOR: u32 = 0x00b5_4620;

// Translated from 00c1d790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one stack word): allocates an array of
/// `count` four-byte elements with a one-word count in front, the shape of an
/// `operator new[]` for a class with a constructor. The size `count * 4 + 4`
/// saturates to `0xffffffff` on overflow (the compiler's `SETO`/`SETC` idiom).
/// Allocates through `00aa3e40` on the memory manager; on failure answers null.
/// Otherwise it stores `count` in the first word, runs the vector constructor
/// loop `00401050` (`stdcall`: block, element size 4, count, constructor
/// `00b54620`) over the elements and answers the address of the first element.
pub fn fn_00c1d790(e: &mut Engine, count: u32) -> Ptr {
    let bytes: u32 = (u64::from(count) * 4).try_into().unwrap_or(u32::MAX);
    let total = bytes.saturating_add(4);
    let block = e.call(0x00aa_3e40, &args![MEMORY_MANAGER, total]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block, count);
    let elements = block + 4;
    e.call(
        0x0040_1050,
        &args![elements, 4u32, count, ELEMENT_CONSTRUCTOR],
    );
    Ptr::new(elements)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1f4f0, ni_p_sys_mesh_emitter_get_rtti() -> Ptr),
        entry!(0x00c1d790, fn_00c1d790(u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_mesh_emitter_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00c1f4f0, &args![]).u32(),
            NI_P_SYS_MESH_EMITTER_RTTI
        );
    }

    #[test]
    fn fn_00c1d790_allocates_the_array_with_its_count_in_front() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |e, a| {
            assert_eq!(a[0], MEMORY_MANAGER);
            e.mem.alloc(a[1]).into_ret()
        });
        e.register(0x0040_1050, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let elements = e.call(0x00c1_d790, &args![5u32]).u32();
        let log = e.call_log.take().unwrap();
        let alloc: Vec<_> = log.iter().filter(|c| c.0 == 0x00aa_3e40).collect();
        assert_eq!(alloc[0].1, vec![MEMORY_MANAGER, 24]);
        assert_eq!(e.mem.u32(elements - 4), 5);
        let init: Vec<_> = log.iter().filter(|c| c.0 == 0x0040_1050).collect();
        assert_eq!(init[0].1, vec![elements, 4, 5, ELEMENT_CONSTRUCTOR]);
    }

    #[test]
    fn fn_00c1d790_saturates_the_size_and_answers_null_on_failure() {
        let mut e = Engine::new();
        e.register(0x00aa_3e40, |_, _| Ret::default());
        e.register(0x0040_1050, |_, _| panic!("no elements without a block"));
        e.call_log = Some(vec![]);
        // 0x4000_0000 * 4 overflows; so does 0x3fff_ffff * 4 + 4.
        assert_eq!(e.call(0x00c1_d790, &args![0x4000_0000u32]).u32(), 0);
        assert_eq!(e.call(0x00c1_d790, &args![0x3fff_ffffu32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == 0x00aa_3e40)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(sizes, vec![u32::MAX, u32::MAX]);
    }
}
