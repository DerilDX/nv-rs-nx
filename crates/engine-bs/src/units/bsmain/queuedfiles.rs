//! `BSMain/queuedfiles.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c3c520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010c14e8` (RTTI `.?AVQueuedChildren@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3d930`. One word, plain `RET`, returns nothing.
pub fn fn_00c3c520(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_14e8);
    e.call(0x00c3_d930, &args![this]);
}

// Translated from 00c3c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `010c1510` (RTTI `.?AVQueuedParents@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00c3d930`. One word, plain `RET`, returns nothing.
pub fn fn_00c3c570(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010c_1510);
    e.call(0x00c3_d930, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_c570, fn_00c3c570(Ptr)),
        entry!(0x00c3_c520, fn_00c3c520(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00c3c520_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_d930, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_c520, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_d930)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x010c_14e8);
        assert_eq!(
            e.mem.u32(this + 4),
            0x010c_14e8,
            "the vtable was stored before the call"
        );
    }

    #[test]
    fn fn_00c3c570_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00c3_d930, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_c570, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00c3_d930)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x010c_1510);
        assert_eq!(
            e.mem.u32(this + 4),
            0x010c_1510,
            "the vtable was stored before the call"
        );
    }
}
