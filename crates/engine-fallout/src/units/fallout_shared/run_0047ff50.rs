//! `fallout shared/run_0047ff50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0047ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `u16` argument at `+4` (unnamed in the map; `ECX`, one stack
/// word, `RET 4`). `TESForm::LoadData` (`004861f0`) calls it as the setter
/// of the form's `TESAttackDamageForm` component (the cast it makes first).
pub fn fn_0047ff50(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + 4, value);
}

// Translated from 0047ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101bf84` (RTTI `.?AVTESAttributes@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_0047ffa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_bf84);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_ffa0, fn_0047ffa0(Ptr)),
        entry!(0x0047ff50, fn_0047ff50(Ptr, u16)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_low_half_of_the_word_at_plus_4() {
        let mut e = Engine::new();
        let component = e.mem.alloc(8);
        e.mem.set_u32(component, 0x1111_1111);
        e.mem.set_u32(component + 4, 0xaaaa_aaaa);
        e.call(
            0x0047_ff50,
            &args![Ptr::<()>::new(component), 0xdead_1234u32],
        );
        assert_eq!(e.mem.u32(component + 4), 0xaaaa_1234);
        assert_eq!(e.mem.u32(component), 0x1111_1111);
    }

    #[test]
    fn fn_0047ffa0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0047_ffa0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_bf84);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_bf84,
            "the vtable was stored before the call"
        );
    }
}
