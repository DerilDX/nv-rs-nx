//! `fallout shared/run_0067da20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A 0x10-byte block (`PackageEventAction` keeps one; `PackageDataPatrol`
//! holds one at +4): words at +0, +8 and +0xc, and at +4 an object deleted
//! through its virtual slot `0x10`.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0067da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the four words (+0, +8, +0xc, +4, in that order). Returns `this`.
/// The engine map gives it no name.
pub fn fn_0067da40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x4, 0);
    this
}

// Translated from 0067da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes +0 and +8 and deletes the object at +4 (its virtual slot `0x10`
/// with 1) when there is one; +4 itself is left as it is. The engine map
/// gives it no name.
pub fn fn_0067da80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x8, 0);
    let object = e.mem.u32(base + 0x4);
    if object != 0 {
        e.vcall(object, 0x10, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_da40, fn_0067da40(Ptr) -> Ptr),
        entry!(0x0067_da80, fn_0067da80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0067da40_zeroes_the_block() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0067_da40, &args![this]).u32(), this);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0, 0]);
    }

    fn deletes(e: &mut Engine, this: u32) -> Vec<Vec<u32>> {
        e.call_log = Some(vec![]);
        e.call(0x0067_da80, &args![this]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_1000)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn at_0067da80_deletes_the_object() {
        let mut e = Engine::new();
        e.register(0x0f00_1000, |_, _| Ret::default());
        e.put_vtable(0x0f00_0000, &[0, 0, 0, 0, 0x0f00_1000]);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0f00_0000);
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0x77);
        }
        e.mem.set_u32(this + 4, object);
        assert_eq!(deletes(&mut e, this), vec![vec![object, 1]]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 4 * i)).collect();
        assert_eq!(words, vec![0, object, 0, 0x77]);
        e.mem.set_u32(this + 4, 0);
        assert!(deletes(&mut e, this).is_empty());
    }
}
