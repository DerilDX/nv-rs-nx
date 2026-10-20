//! `fallout shared/run_004fd400` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fd400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0xa0.
pub fn fn_004fd400(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xa0)
}

// Translated from 004fd420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): whether the word at the
/// address `004ea950` gives for `this` has any of the bits of `mask` (`SETNZ AL`; the rest
/// of `EAX` keeps the masked word's upper bits, which callers do not read).
pub fn fn_004fd420(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    let flags = e.call(0x004e_a950, &args![this]).u32();
    e.mem.u32(flags) & mask != 0
}

// Translated from 004fd4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): the word at +0x60 masked with
/// `0x1f0000`.
pub fn fn_004fd4f0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x60) & 0x001f_0000
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_d400, fn_004fd400(Ptr) -> u32),
        entry!(0x004f_d420, fn_004fd420(Ptr, u32) -> bool),
        entry!(0x004f_d4f0, fn_004fd4f0(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004fd400_and_fn_004fd4f0_read_their_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xa4);
        e.mem.set_u32(object + 0xa0, 0x5555_6666);
        e.mem.set_u32(object + 0x60, 0xffff_ffff);
        assert_eq!(e.call(0x004f_d400, &args![object]).u32(), 0x5555_6666);
        assert_eq!(e.call(0x004f_d4f0, &args![object]).u32(), 0x001f_0000);
        e.mem.set_u32(object + 0x60, 0x0012_3456);
        assert_eq!(e.call(0x004f_d4f0, &args![object]).u32(), 0x0012_0000);
    }

    #[test]
    fn fn_004fd420_tests_the_mask_against_the_flags_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, 0x0000_0102);
        e.register_double(0x004e_a950, move |_, _| Ret {
            eax: flags,
            ..Ret::default()
        });
        for (mask, expected) in [
            (2u32, true),
            (1, false),
            (0x100, true),
            (0xffff_0000, false),
        ] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x004f_d420, &args![object, mask]).bool(), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (0x004e_a950, vec![object]));
        }
    }
}
