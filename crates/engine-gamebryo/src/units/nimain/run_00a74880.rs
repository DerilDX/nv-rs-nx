//! `NiMain/run_00a74880` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Byte `011f4a54`: when 0, `00a74da0` first runs `00a74a10`.
const SETUP_DONE_FLAG: u32 = 0x011f_4a54;
/// Word `011f4a50`: a mode number; 1 to 6 (signed) select the second pair of steps.
const MODE_WORD: u32 = 0x011f_4a50;
/// `00a74a10` (no stack words): run first when the flag is 0.
const SETUP_STEP: u32 = 0x00a7_4a10;
/// `00a74d90` / `00a74d80` (`cdecl`, one word each): store their argument at `011aa050`
/// and `011aa04c`.
const SET_FIRST_CALLBACK: u32 = 0x00a7_4d90;
const SET_SECOND_CALLBACK: u32 = 0x00a7_4d80;

// Translated from 00a74da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiMain subsystem; the last step of the
/// start-up `00a89cc0`, no arguments): when the byte `011f4a54` is 0 runs `00a74a10`;
/// sets the callbacks `00a74bc0` (`00a74d90`) and `00a74cb0` (`00a74d80`); and when the
/// word `011f4a50` is from 1 to 6 (signed) sets `00a74900` and `00a74880` the same way.
pub fn fn_00a74da0(e: &mut Engine) {
    if e.mem.u8(SETUP_DONE_FLAG) == 0 {
        e.call(SETUP_STEP, &args![]);
    }
    e.call(SET_FIRST_CALLBACK, &args![0x00a7_4bc0u32]);
    e.call(SET_SECOND_CALLBACK, &args![0x00a7_4cb0u32]);
    let mode = e.mem.u32(MODE_WORD) as i32;
    if (1..=6).contains(&mode) {
        e.call(SET_FIRST_CALLBACK, &args![0x00a7_4900u32]);
        e.call(SET_SECOND_CALLBACK, &args![0x00a7_4880u32]);
    }
}

// Translated from 00a74d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores its argument at `011aa050`.
/// `fn_00a74da0` passes it `00a74bc0` and, in modes 1 to 6, `00a74900`.
pub fn fn_00a74d90(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x011a_a050, value);
}

// Translated from 00a74d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): stores its argument at `011aa04c`.
/// `fn_00a74da0` passes it `00a74cb0` and, in modes 1 to 6, `00a74880`.
pub fn fn_00a74d80(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x011a_a04c, value);
}

/// `011f4a58`: the word the CPU probe keeps its last answer in.
const FEATURE_WORD: u32 = 0x011f_4a58;

/// The `CPUID` instruction on the host (`None` where the host has none: the exe's
/// exception handlers then run).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn host_cpuid(leaf: u32) -> Option<[u32; 4]> {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::__cpuid;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::__cpuid;
    // SAFETY: CPUID is available on every x86 CPU the game runs on.
    #[allow(unused_unsafe)] // an unsafe fn on the MSRV (1.75), safe on newer toolchains
    let answer = unsafe { __cpuid(leaf) };
    Some([answer.eax, answer.ebx, answer.ecx, answer.edx])
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn host_cpuid(_leaf: u32) -> Option<[u32; 4]> {
    None
}

/// The body of `00a74a10` with the `CPUID` instruction given as `cpuid(EAX)`, answering
/// `[EAX, EBX, ECX, EDX]` or `None` when the instruction raises an exception (the exe's
/// three `__except` handlers).
fn probe_cpu(e: &mut Engine, mut cpuid: impl FnMut(u32) -> Option<[u32; 4]>) {
    e.mem.set_u8(SETUP_DONE_FLAG, 1);
    // First try block: leaf 0x80000000. An exception sets the mode to 0 and ends.
    let Some(answer) = cpuid(0x8000_0000) else {
        e.mem.set_u32(MODE_WORD, 0);
        return;
    };
    e.mem.set_u32(FEATURE_WORD, answer[0]);
    let leaf = e.mem.u32(FEATURE_WORD) & 0x8000_0000;
    if leaf != 0 {
        // Second try block: the leaf is the masked `EAX` (the exe loads `EDX` with
        // 0x80000001 but `CPUID` reads `EAX`). An exception stores 0 and goes on.
        match cpuid(leaf) {
            Some(answer) => e.mem.set_u32(FEATURE_WORD, answer[0]),
            None => e.mem.set_u32(FEATURE_WORD, 0),
        }
        if e.mem.u32(FEATURE_WORD) & 0x8000_0000 != 0 {
            e.mem.set_u32(MODE_WORD, 6);
            return;
        }
    }
    // Third try block: leaf 1, the feature word in `EDX`. An exception sets the mode to 0.
    let Some(answer) = cpuid(1) else {
        e.mem.set_u32(MODE_WORD, 0);
        return;
    };
    e.mem.set_u32(FEATURE_WORD, answer[3]);
    if e.mem.u32(FEATURE_WORD) & 0x0080_0000 == 0 {
        if e.mem.u32(FEATURE_WORD) & 0x8000 != 0 {
            e.mem.set_u32(MODE_WORD, 3);
        } else {
            e.mem.set_u32(MODE_WORD, 1);
        }
    } else if e.mem.u32(FEATURE_WORD) & 0x8000 == 0 {
        e.mem.set_u32(MODE_WORD, 2);
    } else {
        e.mem.set_u32(MODE_WORD, 4);
        if e.mem.u32(FEATURE_WORD) & 0x0200_0000 != 0 {
            e.mem.set_u32(MODE_WORD, 5);
        }
    }
}

// Translated from 00a74a10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a74a10 - SEH frame (three __except handlers) modelled by CPUID answering None
// (the registered entry runs the host CPUID, or None where the host has none)
/// Unnamed in the engine map (range inside the NiMain subsystem; no arguments): sets the
/// byte `011f4a54` to 1 and probes the CPU with `CPUID` to choose the mode word at
/// `011f4a50`: 6 when the extended leaf answers with bit 31 set (twice), otherwise from
/// the feature word of leaf 1: 1 (no MMX, no CMOV), 3 (no MMX, CMOV), 2 (MMX, no CMOV), 4
/// (MMX and CMOV), 5 (also SSE); 0 when the instruction faults. (The names of the modes are
/// not confirmed.)
pub fn fn_00a74a10(e: &mut Engine) {
    probe_cpu(e, host_cpuid);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_4d80, fn_00a74d80(u32)),
        entry!(0x00a7_4d90, fn_00a74d90(u32)),
        entry!(0x00a7_4da0, fn_00a74da0()),
        entry!(0x00a7_4a10, fn_00a74a10()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn callback_setters_store_their_argument() {
        let mut e = Engine::new();
        e.map(0x011a_a000, 0x1000);
        e.call(0x00a7_4d90, &args![0x1111u32]);
        e.call(0x00a7_4d80, &args![0x2222u32]);
        assert_eq!(e.mem.u32(0x011a_a050), 0x1111);
        assert_eq!(e.mem.u32(0x011a_a04c), 0x2222);
    }

    fn run(flag: u8, mode: i32) -> Vec<(u32, Vec<u32>)> {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u8(SETUP_DONE_FLAG, flag);
        e.mem.set_u32(MODE_WORD, mode as u32);
        for address in [SETUP_STEP, SET_FIRST_CALLBACK, SET_SECOND_CALLBACK] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00a7_4da0, &args![]);
        e.call_log.unwrap().into_iter().skip(1).collect()
    }

    fn call(address: u32, words: &[u32]) -> (u32, Vec<u32>) {
        (address, words.to_vec())
    }

    #[test]
    fn mode_outside_one_to_six_sets_only_the_first_pair() {
        for mode in [0, -1, 7, i32::MIN] {
            assert_eq!(
                run(1, mode),
                vec![
                    call(SET_FIRST_CALLBACK, &[0x00a7_4bc0]),
                    call(SET_SECOND_CALLBACK, &[0x00a7_4cb0]),
                ]
            );
        }
    }

    #[test]
    fn mode_one_to_six_sets_the_second_pair_too() {
        for mode in [1, 6] {
            let log = run(1, mode);
            assert_eq!(log.len(), 4);
            assert_eq!(log[2], call(SET_FIRST_CALLBACK, &[0x00a7_4900]));
            assert_eq!(log[3], call(SET_SECOND_CALLBACK, &[0x00a7_4880]));
        }
    }

    #[test]
    fn the_setup_step_runs_first_only_while_the_flag_is_zero() {
        let log = run(0, 0);
        assert_eq!(log[0], call(SETUP_STEP, &[]));
        assert_eq!(log.len(), 3);
        assert_eq!(run(1, 0).len(), 2);
    }
}

#[cfg(test)]
mod probe_tests {
    use super::*;

    /// Runs the probe with the answers of the first and second extended leaf and of leaf 1
    /// (`None`: the instruction faults); the mode word, the feature word and the leaves
    /// asked for.
    fn probe(
        first: Option<u32>,
        second: Option<u32>,
        features: Option<u32>,
    ) -> (u32, u32, Vec<u32>) {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        let mut leaves = vec![];
        let mut extended = 0;
        probe_cpu(&mut e, |leaf| {
            leaves.push(leaf);
            if leaf == 1 {
                return features.map(|edx| [0, 0, 0, edx]);
            }
            extended += 1;
            let answer = if extended == 1 { first } else { second };
            answer.map(|eax| [eax, 0, 0, 0])
        });
        assert_eq!(e.mem.u8(SETUP_DONE_FLAG), 1);
        (e.mem.u32(MODE_WORD), e.mem.u32(FEATURE_WORD), leaves)
    }

    #[test]
    fn a_second_extended_answer_with_bit_31_selects_mode_six() {
        let (mode, feature, leaves) = probe(Some(0x8000_0008), Some(0x8000_0001), Some(0));
        assert_eq!((mode, feature), (6, 0x8000_0001));
        assert_eq!(leaves, vec![0x8000_0000, 0x8000_0000]);
    }

    #[test]
    fn feature_bits_select_the_mode() {
        // MMX is bit 23, CMOV bit 15, SSE bit 25; no extended leaves (bit 31 clear).
        for (features, mode) in [
            (0u32, 1u32),
            (0x8000, 3),
            (0x80_0000, 2),
            (0x80_8000, 4),
            (0x0280_8000, 5),
            (0x0200_0000, 1),
        ] {
            let (found, feature, leaves) = probe(Some(0x7fff_ffff), None, Some(features));
            assert_eq!(found, mode, "{features:#x}");
            assert_eq!(feature, features);
            assert_eq!(leaves, vec![0x8000_0000, 1]);
        }
    }

    #[test]
    fn a_clear_second_answer_carries_on_with_leaf_one() {
        let (mode, _, leaves) = probe(Some(0x8000_0000), Some(1), Some(0x80_8000));
        assert_eq!(mode, 4);
        assert_eq!(leaves, vec![0x8000_0000, 0x8000_0000, 1]);
    }

    #[test]
    fn faults_set_the_mode_to_zero_or_fall_through() {
        // The first block: mode 0 and nothing else is asked.
        let (mode, feature, leaves) = probe(None, None, Some(0x80_8000));
        assert_eq!((mode, feature, leaves), (0, 0, vec![0x8000_0000]));
        // The second block stores 0 in the feature word and carries on with leaf 1.
        let (mode, _, leaves) = probe(Some(0x8000_0000), None, Some(0x0280_8000));
        assert_eq!(mode, 5);
        assert_eq!(leaves, vec![0x8000_0000, 0x8000_0000, 1]);
        // The third block: mode 0.
        let (mode, _, _) = probe(Some(0x7fff_ffff), None, None);
        assert_eq!(mode, 0);
    }
}
