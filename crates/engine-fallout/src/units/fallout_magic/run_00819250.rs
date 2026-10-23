//! `fallout/magic/run_00819250` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (`thiscall`): the word at `this`.
const FIRST_WORD: u32 = 0x0055_9450;
/// `0043b4f0` (`thiscall`, one word: an out word): stores a value found for `this` into the
/// out word and returns its address.
const LOOKUP_0043B4F0: u32 = 0x0043_b4f0;
/// `004a3a20` (`thiscall`): the high 16 bits of the word at `this`.
const HIGH_HALF: u32 = 0x004a_3a20;

// Translated from 00819250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`). When the first word of the member
/// at `this + 0x594` (`00559450`) is not null, it is looked up through `0043b4f0` into a
/// stack word and the result is the high 16 bits of that word (`004a3a20` on the address
/// `0043b4f0` returns); otherwise 0.
pub fn fn_00819250(e: &mut Engine, this: Ptr) -> u32 {
    let member = this.addr() + 0x594;
    if e.call(FIRST_WORD, &args![member]).u32() == 0 {
        return 0;
    }
    e.with_stack(4, |e, out| {
        let first = e.call(FIRST_WORD, &args![member]).u32();
        let found = e.call(LOOKUP_0043B4F0, &args![first, out]).u32();
        e.call(HIGH_HALF, &args![found]).u32()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_9250, fn_00819250(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn looks_up_the_member_and_answers_the_high_half() {
        let mut e = Engine::new();
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LOOKUP_0043B4F0, |e, a| {
            e.mem.set_u32(a[1], a[0].wrapping_mul(0x10));
            a[1].into_ret()
        });
        e.register(HIGH_HALF, |e, a| (e.mem.u32(a[0]) >> 16).into_ret());
        let this = e.mem.alloc(0x598);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_9250, &[this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == LOOKUP_0043B4F0));

        e.mem.set_u32(this + 0x594, 0x0012_3450);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0081_9250, &[this]).u32(), 0x0123);
        let log = e.call_log.take().unwrap();
        let lookups: Vec<_> = log.iter().filter(|(a, _)| *a == LOOKUP_0043B4F0).collect();
        assert_eq!(lookups[0].1[0], 0x0012_3450);
        assert_eq!(log.iter().filter(|(a, _)| *a == FIRST_WORD).count(), 2);
    }
}
