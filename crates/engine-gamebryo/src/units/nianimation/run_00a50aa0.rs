//! `NiAnimation/run_00a50aa0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineInterpolator::CopyMembers` (`00a57fd0`, `thiscall (this,
/// dest, cloning process)`; the engine map has no name).
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;

// Translated from 00a50aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::CopyMembers` (body; the engine map has no
/// name): the base class's `CopyMembers`, then the `float` at +0x1C
/// (through the FPU) and the control-point handle at +0x20 copied to
/// `dest`.
pub fn fn_00a50aa0(e: &mut Engine, this: Ptr, dest: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + 0x1c));
    e.mem.set_u32(dest.addr().wrapping_add(0x1c), value);
    let handle = e.mem.u32(this.addr() + 0x20);
    e.mem.set_u32(dest.addr().wrapping_add(0x20), handle);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_0aa0, fn_00a50aa0(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_spline_float_copy_members_copies_the_value_and_handle() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        let dest = e.mem.alloc(0x24);
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        e.mem.set_u32(this + 0x20, 0x1234);
        e.call(0x00a5_0aa0, &args![this, dest, 0x7000u32]);
        assert_eq!(e.mem.u32(dest + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0x20), 0x1234);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_COPY_MEMBERS && call.1 == vec![this, dest, 0x7000]));
    }
}
