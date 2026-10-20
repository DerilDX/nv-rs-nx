//! `fallout/ai/run_009a3a80` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `memmove` (`00ec7230`, `cdecl`: destination, source, size).
const MEMMOVE: u32 = 0x00ec_7230;

// Translated from 009a3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; `this` is stored but never read): moves
/// `count` 0x14-byte elements from `source` to `destination` one element at a time (`memmove`
/// of 0x14 bytes each): forwards from the first element when `destination` is below
/// `source`, backwards from the last one when it is above, nothing when they are equal or
/// `count` is 0.
pub fn fn_009a3b20(e: &mut Engine, _this: Ptr, destination: u32, source: u32, count: u32) {
    if count == 0 {
        return;
    }
    if destination < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(0x14);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(destination),
                    offset.wrapping_add(source),
                    0x14u32
                ],
            );
            index = index.wrapping_add(1);
        }
    } else if destination > source {
        let mut index = count.wrapping_sub(1);
        while index as i32 >= 0 {
            let offset = index.wrapping_mul(0x14);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(destination),
                    offset.wrapping_add(source),
                    0x14u32
                ],
            );
            index = index.wrapping_sub(1);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009a_3b20, fn_009a3b20(Ptr, u32, u32, u32))]
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn moves(e: &mut Engine, destination: u32, source: u32, count: u32) -> Vec<Vec<u32>> {
        e.register(MEMMOVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009a_3b20, &args![0u32, destination, source, count]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 == MEMMOVE)
            .map(|call| call.1)
            .collect()
    }

    #[test]
    fn fn_009a3b20_moves_forwards_or_backwards() {
        let mut e = Engine::new();
        assert_eq!(
            moves(&mut e, 0x100, 0x200, 2),
            vec![vec![0x100, 0x200, 0x14], vec![0x114, 0x214, 0x14]]
        );
        assert_eq!(
            moves(&mut e, 0x200, 0x100, 2),
            vec![vec![0x214, 0x114, 0x14], vec![0x200, 0x100, 0x14]]
        );
        assert!(moves(&mut e, 0x100, 0x100, 2).is_empty());
        assert!(moves(&mut e, 0x100, 0x200, 0).is_empty());
    }
}
