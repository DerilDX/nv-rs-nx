//! `NiMain/run_00ee8850` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiObject`'s constructor (`00a5d3a0`, unit `niobject.cpp`; `thiscall`, no stack words) and
/// `NiShaderDeclaration`'s vtable (`011011ec`, RTTI `.?AVNiShaderDeclaration@@`).
const NI_OBJECT_CONSTRUCT_00A5D3A0: u32 = 0x00a5_d3a0;
const SHADER_DECLARATION_VTABLE_011011EC: u32 = 0x0110_11ec;

// Translated from 00ee88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ee8850`): the `NiShaderDeclaration` constructor (the
/// vtable it stores has that class's RTTI), `thiscall`, no stack words: `NiObject`'s
/// constructor, then the vtable `011011ec`. Answers `this`.
pub fn fn_00ee88b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT_00A5D3A0, &args![this]);
    e.mem
        .set_u32(this.addr(), SHADER_DECLARATION_VTABLE_011011EC);
    this
}

/// `NiObject`'s destructor (`00a5d3d0`, unit `niobject.cpp`; `thiscall`, no
/// stack words).
const NI_OBJECT_DESTRUCT_00A5D3D0: u32 = 0x00a5_d3d0;

// Translated from 00ee88d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiShaderDeclaration` destructor body
/// (`thiscall`, no stack words): stores the vtable `011011ec`, then jumps to
/// `NiObject`'s destructor `00a5d3d0` with the same `this`.
pub fn fn_00ee88d0(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), SHADER_DECLARATION_VTABLE_011011EC);
    e.call(NI_OBJECT_DESTRUCT_00A5D3D0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_88b0, fn_00ee88b0(Ptr) -> Ptr),
        entry!(0x00ee_88d0, fn_00ee88d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00ee88b0_runs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT_00A5D3A0, |e, a| {
            e.mem.set_u32(a[0], 1);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00ee_88b0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SHADER_DECLARATION_VTABLE_011011EC);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_OBJECT_CONSTRUCT_00A5D3A0, vec![this]));
    }
}

#[cfg(test)]
mod tests_b008 {
    use super::*;

    #[test]
    fn fn_00ee88d0_stores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_DESTRUCT_00A5D3D0, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00ee_88d0, &args![this]);
        assert_eq!(e.mem.u32(this), SHADER_DECLARATION_VTABLE_011011EC);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == NI_OBJECT_DESTRUCT_00A5D3D0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this]);
    }
}
