//! `fallout/ai/run_00978ed0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00978ef0` calls on the member (`00559450`).
const CALLEE_00559450: u32 = 0x0055_9450;

// Translated from 00978ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00559450` with `ECX` = `this + 0x120` and passes on its result.
pub fn fn_00978ef0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_00559450, &args![this.byte_add(0x120)]).u32()
}

// Translated from 00978ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns `pointer` to the `NiPointer` member at `this + 0x120` through
/// `NiPointer<ActorCause>::operator=` (`0042f780`, engine map name; the body
/// is a smart-pointer assignment). One stack word.
pub fn fn_00978ed0(e: &mut Engine, this: Ptr, pointer: u32) {
    e.call(0x0042_f780, &args![this.byte_add(0x120), pointer]);
}

// Translated from 00978f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual that returns the constant 1 (`ECX` only, no stack words).
pub fn fn_00978f30(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00979260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at `this + 0xD4`, loaded into `ST0` (`FLD dword`).
pub fn fn_00979260(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0xd4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_8ef0, fn_00978ef0(Ptr) -> u32),
        entry!(0x0097_8ed0, fn_00978ed0(Ptr, u32)),
        entry!(0x0097_8f30, fn_00978f30(Ptr) -> u32),
        entry!(0x0097_9260, fn_00979260(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00978ef0_passes_this_plus_120_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00559450, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_8ef0, &args![0x4100u32]).u32(), 0x1775);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00559450).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x120]);
    }
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    #[test]
    fn fn_00978ed0_assigns_the_pointer_at_0x120() {
        let mut e = Engine::new();
        e.register(0x0042_f780, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_8ed0, &args![0x4100u32, 0x7000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == 0x0042_f780).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x4220, 0x7000]);
    }

    #[test]
    fn fn_00978f30_returns_one() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0097_8f30, &args![0x4100u32]).u32(), 1);
    }

    #[test]
    fn fn_00979260_loads_the_float_at_0xd4() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x100);
        e.mem.set_f32(object + 0xd4, 2.5);
        assert_eq!(e.call(0x0097_9260, &args![object]).f32(), 2.5);
        // A signalling NaN comes back quiet, as `FLD` leaves it.
        e.mem.set_u32(object + 0xd4, 0x7f80_0001);
        let back = e.call(0x0097_9260, &args![object]).f32().to_bits();
        assert_eq!(back, 0x7fc0_0001);
    }
}
