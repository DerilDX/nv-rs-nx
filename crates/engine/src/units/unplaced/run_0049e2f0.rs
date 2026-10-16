//! `(unplaced)/run_0049e2f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0049e390`: cdecl (a, b, tolerance): compares `a - b` (after `00408840`)
/// with the tolerance and returns a bool in AL.
const FLOATS_CLOSE: u32 = 0x0049_e390;

// Translated from 0049e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name. cdecl (`ECX` is not read): true when all three
/// components of the two `float[3]` vectors pass `0049e390` with the
/// tolerance. Each component and the tolerance go through an `FLD`/`FSTP`
/// pair, which quiets a signalling NaN.
pub fn fn_0049e2f0(e: &mut Engine, a: Ptr, b: Ptr, tolerance: f32) -> bool {
    let tolerance = x87_quiet(tolerance);
    for i in 0..3 {
        let x = x87_quiet(e.mem.f32(a.addr() + 4 * i));
        let y = x87_quiet(e.mem.f32(b.addr() + 4 * i));
        if !e.call(FLOATS_CLOSE, &args![x, y, tolerance]).bool() {
            return false;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0049e2f0, fn_0049e2f0(Ptr, Ptr, f32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let p = Ptr::<()>::new(e.mem.alloc(12));
        for (i, c) in v.iter().enumerate() {
            e.mem.set_f32(p.addr() + 4 * i as u32, *c);
        }
        p
    }

    fn engine_with_close_double() -> Engine {
        let mut e = Engine::new();
        e.register(FLOATS_CLOSE, |_, a| {
            let (x, y, tol) = (
                f32::from_bits(a[0]),
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
            );
            Ret {
                eax: ((x - y).abs() < tol) as u32,
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn true_when_every_component_is_close() {
        let mut e = engine_with_close_double();
        let a = vector(&mut e, [1.0, 2.0, 3.0]);
        let b = vector(&mut e, [1.1, 2.1, 3.1]);
        assert!(e.call(0x0049_e2f0, &args![a, b, 0.5f32]).bool());
    }

    #[test]
    fn stops_at_the_first_component_that_is_not_close() {
        let mut e = engine_with_close_double();
        let a = vector(&mut e, [1.0, 2.0, 3.0]);
        let b = vector(&mut e, [1.1, 9.0, 3.1]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0049_e2f0, &args![a, b, 0.5f32]).bool());
        let log = e.call_log.take().unwrap();
        let compares = log.iter().filter(|(addr, _)| *addr == FLOATS_CLOSE).count();
        assert_eq!(compares, 2);
    }
}
