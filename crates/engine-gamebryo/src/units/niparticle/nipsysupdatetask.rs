//! `NiParticle/nipsysupdatetask.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c25870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysUpdateTask::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2580.
pub fn ni_p_sys_update_task_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2580
}

/// The words `011aeba0` (cleared), `011aeba4` (the object freed) and `011aeba8` (set from
/// `011aebac`) `fn_00c25ac0` works on.
const CLEARED_011AEBA0: u32 = 0x011a_eba0;
const OBJECT_011AEBA4: u32 = 0x011a_eba4;
const TARGET_011AEBA8: u32 = 0x011a_eba8;
const SOURCE_011AEBAC: u32 = 0x011a_ebac;
/// The memory manager singleton (`011f6238`) and its free (`00aa4060`, `thiscall`: block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00e7f900` (unit `nid3dpass.cpp`, unnamed in the engine map; `thiscall`, one word).
const RELEASE_00E7F900: u32 = 0x00e7_f900;

// Translated from 00c25ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, read from its bytes; no arguments; the class
/// unregistration `00c1a660` ends in a jump to it). Clears the word at `011aeba0`. When the object at
/// `011aeba4` is not null: its array at +0 (an `operator new[]` block with the count in the
/// word before it), when not null, is deleted through the first element's vtable slot 0 with
/// 3 when the count is not 0, else its block (the count word's address) freed through the
/// memory manager; the object at +8, when not null, gets `00e7f900` with 1; and the object is
/// freed through the memory manager (`011aeba4` keeps its value). In both cases `011aeba8`
/// takes the word at `011aebac`.
pub fn fn_00c25ac0(e: &mut Engine) {
    let object = e.mem.u32(OBJECT_011AEBA4);
    e.mem.set_u32(CLEARED_011AEBA0, 0);
    if object != 0 {
        let array = e.mem.u32(object);
        if array != 0 {
            let block = array.wrapping_sub(4);
            if e.mem.u32(block) != 0 {
                e.vcall(array, 0, &args![3u32]);
            } else {
                e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
            }
        }
        let member = e.mem.u32(object.wrapping_add(8));
        if member != 0 {
            e.call(RELEASE_00E7F900, &args![member, 1u32]);
        }
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, object]);
    }
    let source = e.mem.u32(SOURCE_011AEBAC);
    e.mem.set_u32(TARGET_011AEBA8, source);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c25870, ni_p_sys_update_task_get_r_t_t_i() -> u32),
        entry!(0x00c2_5ac0, fn_00c25ac0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_update_task_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_5870, &args![]).u32(), 0x0120_2580);
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    const ELEMENT_DELETE: u32 = 0x0e20_0000;

    /// The globals mapped with 0x1111 in `011aebac` and `011aeba0` set; answers the engine.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_e000, 0x1000);
        e.set_global(CLEARED_011AEBA0, 0x2222u32);
        e.set_global(SOURCE_011AEBAC, 0x1111u32);
        for callee in [MEMORY_FREE, RELEASE_00E7F900, ELEMENT_DELETE] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// An object with an array (count word `count`, one element of a vtable whose slot 0 is
    /// [`ELEMENT_DELETE`]) and the member at +8; answers the object and the array.
    fn object(e: &mut Engine, count: u32, member: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, ELEMENT_DELETE);
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, count);
        e.mem.set_u32(block + 4, vtable);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, block + 4);
        e.mem.set_u32(object + 8, member);
        e.set_global(OBJECT_011AEBA4, object);
        (object, block + 4)
    }

    #[test]
    fn fn_00c25ac0_deletes_a_counted_array_and_releases_the_member() {
        let mut e = engine();
        let (object, array) = object(&mut e, 2, 0x0e30_0000);
        e.call(0x00c2_5ac0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (ELEMENT_DELETE, vec![array, 3]),
                (RELEASE_00E7F900, vec![0x0e30_0000, 1]),
                (MEMORY_FREE, vec![MEMORY_MANAGER, object]),
            ]
        );
        assert_eq!(e.mem.u32(CLEARED_011AEBA0), 0);
        assert_eq!(e.mem.u32(OBJECT_011AEBA4), object);
        assert_eq!(e.mem.u32(TARGET_011AEBA8), 0x1111);
    }

    #[test]
    fn fn_00c25ac0_frees_an_empty_array_block_and_skips_a_null_member() {
        let mut e = engine();
        let (object, array) = object(&mut e, 0, 0);
        e.call(0x00c2_5ac0, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (MEMORY_FREE, vec![MEMORY_MANAGER, array - 4]),
                (MEMORY_FREE, vec![MEMORY_MANAGER, object]),
            ]
        );
    }

    #[test]
    fn fn_00c25ac0_without_an_object_only_copies_the_word() {
        let mut e = engine();
        e.call(0x00c2_5ac0, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(CLEARED_011AEBA0), 0);
        assert_eq!(e.mem.u32(TARGET_011AEBA8), 0x1111);
    }
}
