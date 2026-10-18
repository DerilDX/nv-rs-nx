//! `fallout shared/bsanimgroupsequence.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a344f0` (named `NiControllerSequence::CreateClone` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const CREATE_CLONE_TARGET_00A344F0: u32 = 0x00a3_44f0;

// Translated from 004eec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::CreateClone` (Xbox PDB
/// `?CreateClone@BSAnimGroupSequence@@UAAPAVNiObject@@AAVNiCloningProcess@@@Z`, vtable word
/// 0101de84): forwards `this` and the `NiCloningProcess` reference to the base body `00a344f0`
/// and returns its `NiObject *`. Return type from the decorated name and from the callee,
/// whose last instructions are `MOV EAX,ESI` with the new clone in ESI.
pub fn bs_anim_group_sequence_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    e.call(CREATE_CLONE_TARGET_00A344F0, &args![this, cloning_process])
        .ptr()
}

/// `00a34540` (named `NiControllerSequence::GetViewerStrings` in the engine map): the base
/// body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const VIEWER_STRINGS_TARGET_00A34540: u32 = 0x00a3_4540;

// Translated from 004eec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::GetViewerStrings` (Xbox PDB
/// `?GetViewerStrings@BSAnimGroupSequence@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z`, vtable word
/// 0101de9c): forwards `this` and the `NiTPrimitiveArray<char *>` pointer to the base body
/// `00a34540`. Returns nothing: the decorated name is `X` (void), and the callee ends with its
/// stack cookie check, not a result.
pub fn bs_anim_group_sequence_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VIEWER_STRINGS_TARGET_00A34540, &args![this, strings]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_ec20, bs_anim_group_sequence_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x004e_ec40,
            bs_anim_group_sequence_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_anim_group_sequence_create_clone_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(CREATE_CLONE_TARGET_00A344F0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x004e_ec20, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CREATE_CLONE_TARGET_00A344F0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }

    #[test]
    fn forwarder_bs_anim_group_sequence_get_viewer_strings_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET_00A34540, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004e_ec40, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VIEWER_STRINGS_TARGET_00A34540)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}
