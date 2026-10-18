//! `fallout shared/run_0048cb40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BaseFormComponent`'s constructor (`00402d30`: sets the base vtable).
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
/// This class's vtable.
const VTABLE: u32 = 0x0101_d518;

// Translated from 0048cc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESScriptableForm` (the engine map gives it no name;
/// the Xbox PDB's `TESScriptableForm` is a `BaseFormComponent` followed by
/// `pFormScript` and `bIsInitialized`, which is what `0048cc80` clears):
/// runs the base constructor, sets the vtable and clears the script fields.
/// Returns `this`.
pub fn fn_0048cc50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    fn_0048cc80(e, this);
    this
}

// Translated from 0048cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the script pointer (`pFormScript`, +4) and the initialized flag
/// (`bIsInitialized`, +8, a byte) of a `TESScriptableForm` part. The decompiler
/// names it after a library function (`CDllIsolationWrapperBase::CommonConstruct`),
/// a false match.
pub fn fn_0048cc80(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 8, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048cc50, fn_0048cc50(Ptr) -> Ptr),
        entry!(0x0048cc80, fn_0048cc80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_sets_the_vtable_and_clears_the_script_fields() {
        let mut e = Engine::new();
        e.register(BASE_COMPONENT_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, 0x1111);
        e.mem.set_u8(object + 8, 1);
        e.mem.set_u8(object + 9, 7);
        e.call_log = Some(vec![]);
        let back = e.call(0x0048_cc50, &args![object]).u32();
        assert_eq!(back, object);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u8(object + 8), 0);
        // Only the flag byte is cleared, not its neighbour.
        assert_eq!(e.mem.u8(object + 9), 7);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_COMPONENT_CONSTRUCT, vec![object]));
    }

    #[test]
    fn the_clearing_step_leaves_the_vtable_alone() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x2222);
        e.mem.set_u32(object + 4, 0x3333);
        e.call(0x0048_cc80, &args![object]);
        assert_eq!(e.mem.u32(object), 0x2222);
        assert_eq!(e.mem.u32(object + 4), 0);
    }
}
