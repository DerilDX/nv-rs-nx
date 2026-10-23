//! `fallout shared/sky/skytexturemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00404eb0` (`thiscall`, `RET 0x10`: four words) constructs the 4-byte scope local
/// `SkyTask::Run` keeps on its stack; `00404ee0` (`thiscall`) destroys it.
const SCOPE_CONSTRUCT_00404EB0: u32 = 0x0040_4eb0;
const SCOPE_DESTROY_00404EE0: u32 = 0x0040_4ee0;
/// The source file name `SkyTask::Run` passes to the scope local (a string in `.rdata`).
const SOURCE_FILE_0104F148: u32 = 0x0104_f148;
/// `0045cd60` (`thiscall`, plain `RET`): returns the word at +0x28 of its object.
const TASK_0045CD60: u32 = 0x0045_cd60;
/// `BSShaderManager::GetTexture` (`00b55840`, `cdecl`, five words).
const SHADER_MANAGER_GET_TEXTURE: u32 = 0x00b5_5840;

// Translated from 0063fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SkyTask::Run` (Xbox PDB), `thiscall`, plain `RET`. Constructs its scope local
/// (`00404eb0` with 0x21, 1, the source file name and line 0x32), then calls
/// `BSShaderManager::GetTexture(0045cd60(this), 1, this->+0x30, 1, 0)` (the five words
/// all go to `GetTexture`: `0045cd60` takes none, and the caller removes 0x14 bytes), and
/// destroys the local.
// Translation incomplete: 0063fac0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the scope local) is not translated.
pub fn sky_task_run(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_CONSTRUCT_00404EB0,
            &args![scope, 0x21u32, 1u32, SOURCE_FILE_0104F148, 0x32u32],
        );
        let name = e.mem.u32(this.addr().wrapping_add(0x30));
        let texture = e.call(TASK_0045CD60, &args![this]).u32();
        e.call(
            SHADER_MANAGER_GET_TEXTURE,
            &args![texture, 1u32, name, 1u32, 0u32],
        );
        e.call(SCOPE_DESTROY_00404EE0, &args![scope]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0063_fac0, sky_task_run(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn requests_the_texture_inside_the_scope_local() {
        let mut e = Engine::new();
        e.register(SCOPE_CONSTRUCT_00404EB0, |_, _| Ret::default());
        e.register(SCOPE_DESTROY_00404EE0, |_, _| Ret::default());
        e.register(TASK_0045CD60, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(SHADER_MANAGER_GET_TEXTURE, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x28, 0x4444);
        e.mem.set_u32(this + 0x30, 0x3030);
        e.call_log = Some(vec![]);
        e.call(0x0063_fac0, &args![this]);
        let scope = calls_to(&e, SCOPE_CONSTRUCT_00404EB0);
        assert_eq!(scope.len(), 1);
        assert_eq!(scope[0][1..], [0x21, 1, SOURCE_FILE_0104F148, 0x32]);
        assert_eq!(
            calls_to(&e, SHADER_MANAGER_GET_TEXTURE),
            vec![vec![0x4444, 1, 0x3030, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, SCOPE_DESTROY_00404EE0),
            vec![vec![scope[0][0]]]
        );
    }
}
