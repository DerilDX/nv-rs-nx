//! `(unplaced)/run_00653e50` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00633c90` (thiscall, object): a smart-pointer constructor (stores the
/// object and adds a reference when it is not null).
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0057c5d0` (thiscall: out smart pointer, smart pointer; unit
/// `tesobjectrefr.cpp` in the engine map).
const CALLEE_0057C5D0: u32 = 0x0057_c5d0;
/// `0045cec0` (thiscall): the smart-pointer destructor (releases a
/// non-null object).
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;

// Translated from 00654640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: wraps `object` in a smart pointer on the
/// stack (`00633c90`), passes it with a second, out smart pointer to
/// `0057c5d0` on the member at `this + 0x24`, then destroys the out pointer
/// and the wrapper (`0045cec0`).
// Translation incomplete: 00654640 - SEH exception unwinding (the smart pointers' destructors on unwind) is unsupported.
pub fn fn_00654640(e: &mut Engine, this: Ptr, object: u32) {
    // Locals: the wrapper at +0, the out pointer at +4.
    e.with_stack(8, |e, locals| {
        let wrapper = locals.addr();
        let out = wrapper + 4;
        e.call(SMART_POINTER_CONSTRUCT, &args![wrapper, object]);
        e.call(CALLEE_0057C5D0, &args![this.byte_add(0x24), out, wrapper]);
        e.call(SMART_POINTER_DESTRUCT, &args![out]);
        e.call(SMART_POINTER_DESTRUCT, &args![wrapper]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0065_4640, fn_00654640(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00654640_passes_the_wrapped_object_and_destroys_both_pointers() {
        let mut e = Engine::new();
        e.register(SMART_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(CALLEE_0057C5D0, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x7777);
            Ret::default()
        });
        e.register(SMART_POINTER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_4640, &args![0x6000u32, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        let wrapper = log[1].1[0];
        assert_eq!(log[1], (SMART_POINTER_CONSTRUCT, vec![wrapper, 0x7777]));
        let out = log[2].1[1];
        assert_eq!(log[2], (CALLEE_0057C5D0, vec![0x6024, out, wrapper]));
        assert_eq!(log[3], (SMART_POINTER_DESTRUCT, vec![out]));
        assert_eq!(log[4], (SMART_POINTER_DESTRUCT, vec![wrapper]));
    }
}
