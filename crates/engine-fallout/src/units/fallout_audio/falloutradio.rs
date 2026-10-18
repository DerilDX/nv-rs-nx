//! `fallout/audio/falloutradio.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_008321c0` calls on the member (`00836030`).
const CALLEE_00836030: u32 = 0x0083_6030;

// Translated from 008321c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00836030` with `ECX` = `this + 0x4` and passes on nothing (the callee returns no value).
pub fn fn_008321c0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_00836030, &args![this.byte_add(0x4)]);
}

// Translated from 008372c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0107cf18` (RTTI `.?AVRadioConvTask@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3cea0`. One word, plain `RET`, returns nothing.
pub fn fn_008372c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_cf18);
    e.call(0x00c3_cea0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_72c0, fn_008372c0(Ptr)),
        entry!(0x0083_21c0, fn_008321c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_008321c0_passes_this_plus_4_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00836030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0083_21c0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00836030).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x4]);
    }

    #[test]
    fn fn_008372c0_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_cea0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0083_72c0, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_cea0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_cf18);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_cf18,
            "the vtable was stored before the call"
        );
    }
}
