//! `(unplaced)/run_00ec7ce3` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec7ce3 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked CRT movement. Zero length succeeds even for null pointers. Invalid
/// pointers or insufficient capacity set the thread error and call the native
/// invalid-parameter handler; valid moves use the existing memmove boundary.
pub fn checked_move(
    e: &mut Engine,
    destination: Ptr,
    capacity: u32,
    source: Ptr,
    size: u32,
) -> u32 {
    if size == 0 {
        return 0;
    }
    let error = if destination.is_null() || source.is_null() {
        0x16u32
    } else if capacity < size {
        0x22u32
    } else {
        e.call(0x00ec7230, &args![destination, source, size]);
        return 0;
    };
    let thread_error = e.call(0x00ec85e3, &[]).u32();
    e.mem.set_u32(thread_error, error);
    e.call(0x00ec7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    error
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec7ce3, checked_move(Ptr, u32, Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_move_zero_length_accepts_null_and_nonzero_moves_overlap() {
        // Three generated native vectors: FalloutNV.exe 1.4.0.525,
        // SHA256 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f,
        // Ryzen AI 9 HX 370, FCW027F/MXCSR1F80; all returned zero without fault.
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec7ce3, &[0, 0, 0, 0]).u32(), 0);
        let bytes = e.mem.alloc(8);
        e.mem.write(bytes, b"abcdefgh");
        assert_eq!(e.call(0x00ec7ce3, &[bytes + 2, 6, bytes, 6]).u32(), 0);
        assert_eq!(e.mem.bytes(bytes, 8), b"ababcdef");
        e.mem.write(bytes, b"abcdefgh");
        assert_eq!(e.call(0x00ec7ce3, &[bytes, 8, bytes + 2, 6]).u32(), 0);
        assert_eq!(e.mem.bytes(bytes, 8), b"cdefghgh");
    }

    #[test]
    fn checked_move_errors_set_thread_error_before_handler_and_preserve_data() {
        for (null_destination, null_source, capacity, expected) in [
            (true, false, 8u32, 0x16u32),
            (false, true, 0, 0x16),
            (false, false, 3, 0x22),
        ] {
            let mut e = Engine::new();
            let bytes = e.mem.alloc(8);
            e.mem.write(bytes, b"abcdefgh");
            let error = e.mem.alloc(4);
            e.register_double(0x00ec85e3, move |_, a| {
                assert!(a.is_empty());
                error.into_ret()
            });
            e.register_double(0x00ec7c30, move |e, a| {
                assert_eq!(a, &[0; 5]);
                assert_eq!(e.mem.u32(error), expected);
                Ret::default()
            });
            let destination = if null_destination { 0 } else { bytes };
            let source = if null_source { 0 } else { bytes + 4 };
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00ec7ce3, &[destination, capacity, source, 4])
                    .u32(),
                expected
            );
            assert_eq!(e.mem.bytes(bytes, 8), b"abcdefgh");
            assert!(e
                .call_log
                .unwrap()
                .iter()
                .any(|(address, _)| *address == 0x00ec7c30));
        }
    }
}
