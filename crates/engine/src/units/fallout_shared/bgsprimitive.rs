//! `fallout shared/bgsprimitive.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004a7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__fastcall` on a `float[3]`: returns its squared length
/// `z*z + y*y + x*x` in ST0, stored to a `float` local and reloaded. Computed
/// in `f64` and rounded to `f32` once, as the crate does for x87 code.
pub fn fn_004a7290(e: &mut Engine, this: Ptr) -> f32 {
    let x = e.mem.f32(this.addr()) as f64;
    let y = e.mem.f32(this.addr() + 4) as f64;
    let z = e.mem.f32(this.addr() + 8) as f64;
    (x * x + y * y + z * z) as f32
}

/// The function `fn_004a7040` calls on the member (`00457990`).
const CALLEE_00457990: u32 = 0x0045_7990;

// Translated from 004a7040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `BSMultiBoundAABB::QMultiBoundRadius` (Xbox PDB name in the engine map). Thin wrapper: calls `00457990` with `ECX` = `this + 0x18` and passes on its result (`ST0`).
pub fn fn_004a7040(e: &mut Engine, this: Ptr) -> f64 {
    e.call(CALLEE_00457990, &args![this.byte_add(0x18)]).f64()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a7290, fn_004a7290(Ptr) -> f32),
        entry!(0x004a_7040, fn_004a7040(Ptr) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_squared_length() {
        let mut e = Engine::new();
        let v = Ptr::<()>::new(e.mem.alloc(12));
        e.mem.set_f32(v.addr(), 1.0);
        e.mem.set_f32(v.addr() + 4, 2.0);
        e.mem.set_f32(v.addr() + 8, 3.0);
        assert_eq!(e.call(0x004a_7290, &args![v]).f32(), 14.0);
    }

    #[test]
    fn l68d_004a7040_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00457990, |_, a| Ret {
            st0: a[0] as f64 / 4.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_7040, &args![0x4100u32]).f64(), 4166.0);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00457990).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}
