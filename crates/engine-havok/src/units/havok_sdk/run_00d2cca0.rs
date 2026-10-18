//! `Havok SDK/run_00d2cca0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The table of 17 function words the function fills with the default handler `00d01e50`.
const HANDLER_TABLE_01268A90: u32 = 0x0126_8a90;
const DEFAULT_HANDLER: u32 = 0x00d0_1e50;

// Translated from 00d2cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InitMTSystem`'s helper (cdecl, one word `queue`; descriptions, not confirmed): sets the 17
/// words of the table at `01268a90` to `0x00d01e50`; raises the word at `queue + 0x60` to 2 when
/// it is below (signed); calls `00d04a20` on `queue` with `(0, 0xc, 1)`; registers two sets of
/// handlers with `00d044c0` on `queue`, each a `(table, 17, function, function)` block passed by
/// value after a first word (0, then 1): `(0x00d96600, 0x00d969e0)` and `(0x00d967a0,
/// 0x00d968e0)`; and calls `00da3400` and `00da3480` with `queue`.
pub fn fn_00d2cca0(e: &mut Engine, queue: Ptr) {
    for index in 0..17 {
        e.mem
            .set_u32(HANDLER_TABLE_01268A90 + index * 4, DEFAULT_HANDLER);
    }
    if e.mem.i32(queue.addr() + 0x60) < 2 {
        e.mem.set_i32(queue.addr() + 0x60, 2);
    }
    e.call(0x00d0_4a20, &args![queue, 0u32, 0xcu32, 1u32]);
    e.call(
        0x00d0_44c0,
        &args![
            queue,
            0u32,
            HANDLER_TABLE_01268A90,
            0x11u32,
            0x00d9_6600u32,
            0x00d9_69e0u32
        ],
    );
    e.call(
        0x00d0_44c0,
        &args![
            queue,
            1u32,
            HANDLER_TABLE_01268A90,
            0x11u32,
            0x00d9_67a0u32,
            0x00d9_68e0u32
        ],
    );
    e.call(0x00da_3400, &args![queue]);
    e.call(0x00da_3480, &args![queue]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_cca0, fn_00d2cca0(Ptr))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d2cca0_fills_the_table_and_registers_the_handlers() {
        for (before, after) in [(0, 2), (1, 2), (2, 2), (5, 5), (-3, 2)] {
            let mut e = Engine::new();
            e.map(0x0126_8000, 0x1000);
            for address in [0x00d0_4a20, 0x00d0_44c0, 0x00da_3400, 0x00da_3480] {
                e.register(address, |_, _| Ret::default());
            }
            let queue = e.mem.alloc(0x80);
            e.mem.set_i32(queue + 0x60, before);
            e.call_log = Some(vec![]);
            e.call(0x00d2_cca0, &args![queue]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.i32(queue + 0x60), after);
            for index in 0..17 {
                assert_eq!(e.mem.u32(HANDLER_TABLE_01268A90 + index * 4), 0x00d0_1e50);
            }
            assert_eq!(e.mem.u32(HANDLER_TABLE_01268A90 + 17 * 4), 0);
            assert_eq!(
                log[1..],
                [
                    (0x00d0_4a20, vec![queue, 0, 0xc, 1]),
                    (
                        0x00d0_44c0,
                        vec![queue, 0, 0x0126_8a90, 0x11, 0x00d9_6600, 0x00d9_69e0]
                    ),
                    (
                        0x00d0_44c0,
                        vec![queue, 1, 0x0126_8a90, 0x11, 0x00d9_67a0, 0x00d9_68e0]
                    ),
                    (0x00da_3400, vec![queue]),
                    (0x00da_3480, vec![queue]),
                ]
            );
        }
    }
}
