//! `fallout/interface/menus/bookmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0073ad30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01070ecc` (RTTI `.?AVBookMenu@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00a1c520`. One word, plain `RET`, returns nothing.
pub fn fn_0073ad30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_0ecc);
    e.call(0x00a1_c520, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0073_ad30, fn_0073ad30(Ptr))]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_0073ad30_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00a1_c520, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0073_ad30, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00a1_c520)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0107_0ecc);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0107_0ecc,
            "the vtable was stored before the call"
        );
    }
}
