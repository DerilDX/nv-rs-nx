//! `BSHavok/bhkconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c89720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::UnMarkRead` (map name, taken from a vtable; the body
/// is only this jump). A tail jump: `jmp [vtable + 0x94]` with `ECX` still
/// `this`, so the caller's own stack words (`forwarded`, none known) reach
/// the virtual function unchanged and its result is returned.
pub fn fn_00c89720(e: &mut Engine, this: Ptr, forwarded: &[u32]) -> Ret {
    e.vcall(this.addr(), 0x94, forwarded)
}

/// Uniform form of [`fn_00c89720`]: `this`, then the forwarded words.
fn fn_00c89720_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_00c89720(e, Ptr::new(words[0]), &words[1..])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00c89720, fn_00c89720_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c89720_jumps_to_vtable_slot_0x94_with_the_forwarded_words() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x26];
        slots[0x94 / 4] = 0x0090_5000;
        e.put_vtable(0x0090_2000, &slots);
        e.register(0x0090_5000, |_, a| Ret {
            eax: a.iter().sum(),
            ..Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0090_2000);
        assert_eq!(
            e.call(0x00c8_9720, &args![this, 5u32, 7u32]).u32(),
            this + 12
        );
    }
}
