//! `fallout shared/run_005f6ed0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: the base step `00484d60` (`thiscall`, `(buffer)`), the buffer's `00428110`
/// (`thiscall`, `(&local)`, answers the local) and the local's test `004280f0`
/// (`thiscall`, `(2)`, answers a byte), and the buffer's read `00865e50` (`thiscall`,
/// `(destination, size, 0)`).
const BASE_00484D60: u32 = 0x0048_4d60;
const BUFFER_LOCAL_00428110: u32 = 0x0042_8110;
const LOCAL_TEST_004280F0: u32 = 0x0042_80f0;
const BUFFER_READ_00865E50: u32 = 0x0086_5e50;

// Translated from 005f7150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(buffer)`): the base step `00484d60(buffer)`, then,
/// when the buffer's local (`00428110`) passes `004280f0(2)`, four 4-byte reads from the
/// buffer into `this + 0x44`, `+0x48`, `+0x4C` and `+0x50`.
pub fn fn_005f7150(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(BASE_00484D60, &args![this, buffer]);
    let passed = e.with_stack(4, |e, local| {
        let filled = e.call(BUFFER_LOCAL_00428110, &args![buffer, local]).u32();
        e.call(LOCAL_TEST_004280F0, &args![filled, 2u32]).bool()
    });
    if passed {
        for index in 0..4u32 {
            let destination = this.addr() + 0x44 + 4 * index;
            e.call(
                BUFFER_READ_00865E50,
                &args![buffer, destination, 4u32, 0u32],
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f7150, fn_005f7150(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005f7150_reads_four_words_when_the_test_passes() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_00484D60, |_, _| Ret::default());
        e.register(BUFFER_LOCAL_00428110, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(LOCAL_TEST_004280F0, |_, _| Ret::default());
        e.register(BUFFER_READ_00865E50, |_, _| Ret::default());
        e.call(0x005f7150, &args![0x5000u32, 0x6000u32]);
        let reads = |e: &Engine| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == BUFFER_READ_00865E50)
                .map(|c| c.1.clone())
                .collect()
        };
        assert!(reads(&e).is_empty());
        e.register(LOCAL_TEST_004280F0, |_, a| Ret {
            eax: u32::from(a[1] == 2),
            ..Ret::default()
        });
        e.call(0x005f7150, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            reads(&e),
            vec![
                vec![0x6000, 0x5044, 4, 0],
                vec![0x6000, 0x5048, 4, 0],
                vec![0x6000, 0x504c, 4, 0],
                vec![0x6000, 0x5050, 4, 0]
            ]
        );
    }
}
