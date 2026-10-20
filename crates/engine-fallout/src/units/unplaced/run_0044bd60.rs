//! `(unplaced)/run_0044bd60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040fbf0` (`ECX` = the lock at `this + 0x20`, one word; `RET 4`): taken
/// with 0 before the work.
const LOCK: u32 = 0x0040_fbf0;
/// `0040fba0` (`ECX` = the lock at `this + 0x20`): released after it.
const UNLOCK: u32 = 0x0040_fba0;
/// `00449f80` (`ECX` = `this`): the object the work is asked of.
const TARGET: u32 = 0x0044_9f80;
/// `0044c8e0` (`ECX` = that object; two words; `RET 8`): the work, a byte
/// result.
const WORK: u32 = 0x0044_c8e0;

// Translated from 0044bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20` (`0040fbf0` with 0, released by
/// `0040fba0`), asks `00449f80`'s object for `0044c8e0` with the two words
/// given and returns its byte. The decompiler hangs the two words on
/// `00449f80`; that function takes none (`RET`), they belong to `0044c8e0`
/// (`RET 8`). The engine map gives it no name.
pub fn fn_0044bd60(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u8 {
    let lock = this.addr() + 0x20;
    e.call(LOCK, &args![lock, 0u32]);
    let target = e.call(TARGET, &args![this]).u32();
    let result = e.call(WORK, &args![target, first, second]).u8();
    e.call(UNLOCK, &args![lock]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_bd60, fn_0044bd60(Ptr, u32, u32) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0044bd60_works_under_the_lock() {
        let mut e = Engine::new();
        e.register(LOCK, |_, _| Ret::default());
        e.register(UNLOCK, |_, _| Ret::default());
        e.register(TARGET, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register(WORK, |_, _| Ret {
            eax: 0x1234_5602,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_bd60, &args![0x4000u32, 7u32, 9u32]);
        // Only `AL` of the work's result comes back.
        assert_eq!(result.u32(), 2);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0044_bd60)
            .collect();
        assert_eq!(
            log,
            vec![
                (LOCK, vec![0x4020, 0]),
                (TARGET, vec![0x4000]),
                (WORK, vec![0x6000, 7, 9]),
                (UNLOCK, vec![0x4020]),
            ]
        );
    }
}
