//! `fallout shared/tesquest.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0060d4d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Runs a quest's script: when the quest's `TESScriptableForm` (+0x18) has
/// a script (`00726070`, asked twice), `Script::Run` (`005ac1e0`) with no
/// reference, the quest's script locals (+0x5c), no container and no
/// forcing. The result is not used.
pub fn fn_0060d4d0(e: &mut Engine, this: Ptr) {
    let scriptable = this.addr() + 0x18;
    if e.call(0x0072_6070, &args![scriptable]).u32() != 0 {
        let locals = e.mem.u32(this.addr() + 0x5c);
        let script = e.call(0x0072_6070, &args![scriptable]).u32();
        e.call(0x005a_c1e0, &args![script, 0u32, locals, 0u32, 0u32]);
    }
}

// Translated from 0060d700 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `TESQuest::GetCurrentStage` (Xbox PDB): the stage byte at +0x60,
/// zero-extended into `EAX`.
pub fn tes_quest_get_current_stage(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u8(this.addr() + 0x60) as u32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060d4d0, fn_0060d4d0(Ptr)),
        entry!(0x0060d700, tes_quest_get_current_stage(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_stage_zero_extends_the_byte() {
        let mut e = Engine::new();
        let quest = e.mem.alloc(0x64);
        e.mem.write(quest + 0x5c, &[0xaa; 8]);
        e.mem.set_u8(quest + 0x60, 0xff);
        assert_eq!(e.call(0x0060d700, &[quest]).u32(), 0xff);
        e.mem.set_u8(quest + 0x60, 10);
        assert_eq!(e.call(0x0060d700, &[quest]).u32(), 10);
    }

    #[test]
    fn quest_run_needs_a_script_and_passes_its_locals() {
        for script in [0u32, 0x4444] {
            let mut e = Engine::new();
            let quest = e.mem.alloc(0x64);
            e.mem.set_u32(quest + 0x5c, 0x5555);
            e.register_double(0x00726070, move |_, a| {
                assert_eq!(a, [quest + 0x18]);
                script.into_ret()
            });
            e.register_double(0x005ac1e0, |_, a| {
                assert_eq!(a, [0x4444, 0, 0x5555, 0, 0]);
                1u32.into_ret()
            });
            e.call_log = Some(Vec::new());
            e.call(0x0060d4d0, &[quest]);
            let calls: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
            if script == 0 {
                assert_eq!(calls, [0x0060d4d0, 0x00726070]);
            } else {
                assert_eq!(calls, [0x0060d4d0, 0x00726070, 0x00726070, 0x005ac1e0]);
            }
        }
    }
}
