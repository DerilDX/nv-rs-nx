//! `(unplaced)/run_0044dec0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0044e120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the function whose address is the word at `+0xc` (`cdecl`, five
/// words: the object, then the four stack words of this function) and adds
/// its answer to the word at `+4` (unnamed in the map; `ECX`, four stack
/// words, `RET 0x10`; answers the callee's result). `TESFile` writes a
/// `BSFile` through it (`00473180`: data, size, the address of a 1, 1), the
/// word at `+4` then being the file position.
pub fn fn_0044e120(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    let function = e.mem.u32(this.addr() + 0xc);
    let done = e
        .call(function, &args![this, first, second, third, fourth])
        .u32();
    let position = e.mem.u32(this.addr() + 4).wrapping_add(done);
    e.mem.set_u32(this.addr() + 4, position);
    done
}

/// The function `fn_0044df20` calls on the member (`0044dff0`).
const CALLEE_0044DFF0: u32 = 0x0044_dff0;

// Translated from 0044df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044dff0` with `ECX` = `this + 0x8` and passes on its `bool` result (`AL`).
pub fn fn_0044df20(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_0044DFF0, &args![this.byte_add(0x8)]).bool()
}

// Translated from 0044ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0044ee40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0044ed20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_ee40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0044e120,
            fn_0044e120(Ptr, u32, u32, u32, u32) -> u32
        ),
        entry!(0x0044_df20, fn_0044df20(Ptr) -> bool),
        entry!(0x0044_ed20, fn_0044ed20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const WRITER: u32 = 0x0f00_0000;

    #[test]
    fn calls_the_function_at_plus_0xc_and_advances_plus_4() {
        let mut e = Engine::new();
        e.register(WRITER, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, 0x100);
        e.mem.set_u32(stream + 0xc, WRITER);
        e.call_log = Some(vec![]);
        let this = Ptr::<()>::new(stream);
        let back = e.call(0x0044_e120, &args![this, 0xaau32, 0x30u32, 0xccu32, 1u32]);
        assert_eq!(back.u32(), 0x30);
        assert_eq!(e.mem.u32(stream + 4), 0x130);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == WRITER).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![stream, 0xaa, 0x30, 0xcc, 1]);
        e.mem.set_u32(stream + 4, 0xffff_fff0);
        e.call(0x0044_e120, &args![this, 0u32, 0x20u32, 0u32, 0u32]);
        assert_eq!(e.mem.u32(stream + 4), 0x10);
    }

    #[test]
    fn l68d_0044df20_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DFF0, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_df20, &args![0x4100u32]).bool());
        assert!(e.call(0x0044_df20, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DFF0).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }
    #[test]
    fn at_0044ed20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_ee40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_ed20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_ed20)
                .collect();
            let mut expected = vec![(0x0044_ee40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
