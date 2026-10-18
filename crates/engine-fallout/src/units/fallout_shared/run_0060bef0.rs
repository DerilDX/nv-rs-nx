//! `fallout shared/run_0060bef0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor of `TESQuest` (`0060c4b0`; the engine map gives it no name,
//! but `CreateFormOfType` allocates 0x6c bytes for form type 0x47 and calls it)
//! and the small members it calls that sit in this run.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::TESForm` (`00483370`).
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
/// `TESScriptableForm` base constructor (`0048cc50`).
const SCRIPTABLE_FORM_CONSTRUCT: u32 = 0x0048_cc50;
/// `TESIcon` base constructor (`00402c20`).
const ICON_CONSTRUCT: u32 = 0x0040_2c20;
/// `TESFullName` base constructor (`00402d00`).
const FULL_NAME_CONSTRUCT: u32 = 0x0040_2d00;
/// The two-word list node constructor (`0096a2d0`: item and next, both 0).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The condition list constructor (`00680890`, a call of `0096a2d0`).
const CONDITION_LIST_CONSTRUCT: u32 = 0x0068_0890;
/// The `BSStringT<char>` constructor (`004037b0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
/// Writes `TESForm::cFormType` (`004f15a0`, one stack word).
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `this + 0x3c`, the address of the quest data (`005a8080`).
const QUEST_DATA_ADDRESS: u32 = 0x005a_8080;

/// `TESQuest`'s vtable and those of its three other bases.
const TES_QUEST_VTABLE: u32 = 0x0104_ac44;
const SCRIPTABLE_FORM_VTABLE: u32 = 0x0104_ac2c;
const ICON_VTABLE: u32 = 0x0104_ac0c;
const FULL_NAME_VTABLE: u32 = 0x0104_abf8;
/// The form type of a quest (`QUST`).
const FORM_TYPE_QUEST: u8 = 0x47;
/// Byte offset of the vtable slot the enable setter calls to flag a change
/// (`AddChange` in the Xbox PDB's `TESForm` vtable).
const ADD_CHANGE_SLOT: u32 = 0x48;
/// The change flag the enable setter passes.
const CHANGE_QUEST_FLAGS: u32 = 2;

layout! {
    /// `QUEST_DATA` (Xbox PDB), 8 bytes.
    pub struct QuestData: 0x08 {
        /// `ucFlags` (Xbox PDB): bit 0 is "enabled".
        0x00 ucFlags: u8,
        /// `ucPriority` (Xbox PDB).
        0x01 ucPriority: u8,
        /// `fQuestDelayTime` (Xbox PDB).
        0x04 fQuestDelayTime: f32,
    }

    /// `TESQuest` (Xbox PDB), 0x6c bytes; the same offsets on PC.
    pub struct TESQuest: 0x6c {
        /// `pScriptLocals` (Xbox PDB): `ScriptLocals*`.
        0x5C pScriptLocals: Ptr,
        /// `cCurrentStage` (Xbox PDB).
        0x60 cCurrentStage: u8,
    }
}

/// Offsets in a `TESQuest` (Xbox PDB names): `data` (a [`QuestData`]),
/// `m_listStages`, `m_listObjectives`, `objConditions` and `cFormEditorID`.
const DATA: u32 = 0x3c;
const LIST_STAGES: u32 = 0x44;
const LIST_OBJECTIVES: u32 = 0x4c;
const CONDITIONS: u32 = 0x54;
const EDITOR_ID: u32 = 0x64;

// Translated from 0060c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESQuest`: constructs the `TESForm` base and the
/// `TESScriptableForm` (+0x18), `TESIcon` (+0x24) and `TESFullName` (+0x30)
/// bases, sets the four vtables, constructs the quest data (+0x3c), the stage
/// and objective lists (+0x44, +0x4c), the condition list (+0x54) and the
/// editor ID string (+0x64), sets the form type to 0x47, clears the script
/// locals (+0x5c) and finishes with `0060c740` (quest enabled, current stage
/// 0). Returns `this`.
// Translation incomplete: 0060c4b0 - SEH exception unwinding and the stack security cookie are unsupported.
pub fn fn_0060c4b0(e: &mut Engine, this: Ptr<TESQuest>) -> Ptr<TESQuest> {
    let base = this.addr();
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.call(SCRIPTABLE_FORM_CONSTRUCT, &args![base + 0x18]);
    e.call(ICON_CONSTRUCT, &args![base + 0x24]);
    e.call(FULL_NAME_CONSTRUCT, &args![base + 0x30]);
    e.mem.set_u32(base, TES_QUEST_VTABLE);
    e.mem.set_u32(base + 0x18, SCRIPTABLE_FORM_VTABLE);
    e.mem.set_u32(base + 0x24, ICON_VTABLE);
    e.mem.set_u32(base + 0x30, FULL_NAME_VTABLE);
    fn_0060c5c0(e, Ptr::new(base + DATA));
    e.call(LIST_CONSTRUCT, &args![base + LIST_STAGES]);
    e.call(LIST_CONSTRUCT, &args![base + LIST_OBJECTIVES]);
    e.call(CONDITION_LIST_CONSTRUCT, &args![base + CONDITIONS]);
    e.call(STRING_CONSTRUCT, &args![base + EDITOR_ID]);
    e.call(SET_FORM_TYPE, &args![this, FORM_TYPE_QUEST as u32]);
    e.set(this, TESQuest::pScriptLocals, Ptr::NULL);
    fn_0060c740(e, this);
    this
}

// Translated from 0060c5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `QUEST_DATA`: flags 0, priority 0, delay time 0.0.
pub fn fn_0060c5c0(e: &mut Engine, this: Ptr<QuestData>) -> Ptr<QuestData> {
    e.set(this, QuestData::ucFlags, 0);
    e.set(this, QuestData::ucPriority, 0);
    e.set(this, QuestData::fQuestDelayTime, 0.0);
    this
}

// Translated from 0060c740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the quest enabled (`0060c9c0` with 1, which also flags the change)
/// and sets the current stage (`cCurrentStage`) to 0.
pub fn fn_0060c740(e: &mut Engine, this: Ptr<TESQuest>) {
    fn_0060c9c0(e, this, 1);
    e.set(this, TESQuest::cCurrentStage, 0);
}

// Translated from 0060c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`enable` non-zero) or clears bit 0 of the quest data's flags
/// (found through `005a8080`), then calls the vtable slot at `0x48` with 2
/// (the Xbox PDB's `TESForm::AddChange` with the quest-flags change; the
/// slot's PC body is not checked here). `RET 4`: one byte-sized argument.
pub fn fn_0060c9c0(e: &mut Engine, this: Ptr<TESQuest>, enable: u8) {
    let data: Ptr<QuestData> = e.call_as(QUEST_DATA_ADDRESS, &args![this]);
    let flags = e.get(data, QuestData::ucFlags);
    let flags = if enable != 0 { flags | 1 } else { flags & 0xfe };
    e.set(data, QuestData::ucFlags, flags);
    e.vcall(this.addr(), ADD_CHANGE_SLOT, &args![CHANGE_QUEST_FLAGS]);
}

// Translated from 0060c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xdc (`SUB ECX,0xdc`) and jumps to `00601370`. `00601370` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0060c440(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0060_1370, &args![this.addr().wrapping_sub(0xdc), flags])
        .u32()
}

// Translated from 0060c450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608da0`. `00608da0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c450(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0060_8da0,
        &args![this.addr().wrapping_sub(0x7c), argument],
    )
    .u16()
}

// Translated from 0060c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608e80`. `00608e80` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0060c460(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0060_8e80,
        &args![this.addr().wrapping_sub(0x90), first, second],
    );
}

// Translated from 0060c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608e00`. `00608e00` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c470(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0060_8e00,
        &args![this.addr().wrapping_sub(0x90), argument],
    );
}

// Translated from 0060c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608e80`. `00608e80` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0060c480(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0060_8e80,
        &args![this.addr().wrapping_sub(0x7c), first, second],
    );
}

// Translated from 0060c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x90 (`SUB ECX,0x90`) and jumps to `00608da0`. `00608da0` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c490(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0060_8da0,
        &args![this.addr().wrapping_sub(0x90), argument],
    )
    .u16()
}

// Translated from 0060c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x7c (`SUB ECX,0x7c`) and jumps to `00608e00`. `00608e00` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0060c4a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0060_8e00,
        &args![this.addr().wrapping_sub(0x7c), argument],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060c4b0, fn_0060c4b0(Ptr<TESQuest>) -> Ptr<TESQuest>),
        entry!(0x0060c5c0, fn_0060c5c0(Ptr<QuestData>) -> Ptr<QuestData>),
        entry!(0x0060c740, fn_0060c740(Ptr<TESQuest>)),
        entry!(0x0060c9c0, fn_0060c9c0(Ptr<TESQuest>, u8)),
        entry!(0x0060_c440, fn_0060c440(Ptr, u32) -> u32),
        entry!(0x0060_c450, fn_0060c450(Ptr, u32) -> u16),
        entry!(0x0060_c460, fn_0060c460(Ptr, u32, u32)),
        entry!(0x0060_c470, fn_0060c470(Ptr, u32)),
        entry!(0x0060_c480, fn_0060c480(Ptr, u32, u32)),
        entry!(0x0060_c490, fn_0060c490(Ptr, u32) -> u16),
        entry!(0x0060_c4a0, fn_0060c4a0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TESForm;

    /// The test slot-0x48 function and a vtable made of it.
    const ADD_CHANGE: u32 = 0x00aa_0001;
    const TEST_VTABLE: u32 = 0x00ab_0000;
    /// Bytes after the quest's 0x6c where the slot double counts its calls
    /// and keeps the last argument.
    const CALLS: u32 = 0x70;
    const LAST_ARGUMENT: u32 = 0x74;

    /// An engine with test doubles for every callee outside this file and a
    /// vtable for the quest with a counting double in slot 0x48.
    fn quest_engine() -> Engine {
        let mut e = Engine::new();
        for callee in [
            TES_FORM_CONSTRUCT,
            SCRIPTABLE_FORM_CONSTRUCT,
            ICON_CONSTRUCT,
            FULL_NAME_CONSTRUCT,
            LIST_CONSTRUCT,
            CONDITION_LIST_CONSTRUCT,
            STRING_CONSTRUCT,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        // The form-type setter writes the byte at +4 like the real one.
        e.register(SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(QUEST_DATA_ADDRESS, |_, a| Ret {
            eax: a[0] + 0x3c,
            ..Ret::default()
        });
        e.register(ADD_CHANGE, |e, a| {
            let seen = e.mem.u32(a[0] + CALLS);
            e.mem.set_u32(a[0] + CALLS, seen + 1);
            e.mem.set_u32(a[0] + LAST_ARGUMENT, a[1]);
            Ret::default()
        });
        e.put_vtable(TEST_VTABLE, &[ADD_CHANGE; 0x20]);
        e.put_vtable(TES_QUEST_VTABLE, &[ADD_CHANGE; 0x20]);
        e
    }

    /// A zeroed quest block with room for the double's words, carrying the
    /// test vtable.
    fn quest_block(e: &mut Engine) -> Ptr<TESQuest> {
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block, TEST_VTABLE);
        Ptr::new(block)
    }

    #[test]
    fn quest_data_starts_cleared() {
        let mut e = quest_engine();
        let data: Ptr<QuestData> = e.new_object();
        e.set(data, QuestData::ucFlags, 0xff);
        e.set(data, QuestData::ucPriority, 0xff);
        e.set(data, QuestData::fQuestDelayTime, 3.0);
        let back = e.call(0x0060_c5c0, &args![data]).ptr::<QuestData>();
        assert_eq!(back, data);
        assert_eq!(e.get(data, QuestData::ucFlags), 0);
        assert_eq!(e.get(data, QuestData::ucPriority), 0);
        assert_eq!(e.get(data, QuestData::fQuestDelayTime), 0.0);
    }

    #[test]
    fn enable_sets_or_clears_bit_zero_and_flags_the_change() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        let data = quest.addr() + DATA;
        e.mem.set_u8(data, 0b1010);
        e.call(0x0060_c9c0, &args![quest, 1u32]);
        assert_eq!(e.mem.u8(data), 0b1011);
        assert_eq!(e.mem.u32(quest.addr() + CALLS), 1);
        assert_eq!(e.mem.u32(quest.addr() + LAST_ARGUMENT), 2);
        // Only the low byte of the stack word counts as the flag.
        e.call(0x0060_c9c0, &args![quest, 0x100u32]);
        assert_eq!(e.mem.u8(data), 0b1010);
        e.call(0x0060_c9c0, &args![quest, 3u32]);
        assert_eq!(e.mem.u8(data), 0b1011);
        assert_eq!(e.mem.u32(quest.addr() + CALLS), 3);
    }

    #[test]
    fn a_new_quest_is_enabled_at_stage_zero() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        e.set(quest, TESQuest::cCurrentStage, 9);
        e.call(0x0060_c740, &args![quest]);
        assert_eq!(e.mem.u8(quest.addr() + DATA) & 1, 1);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 0);
    }

    #[test]
    fn constructor_builds_the_bases_in_the_games_order_and_sets_the_vtables() {
        let mut e = quest_engine();
        let quest = quest_block(&mut e);
        let base = quest.addr();
        e.set(quest, TESQuest::pScriptLocals, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        let back = e.call(0x0060_c4b0, &args![quest]).ptr::<TESQuest>();
        assert_eq!(back, quest);
        let log = e.call_log.take().unwrap();
        // The slot call is made through the vtable: leave it out of the list.
        let calls: Vec<(u32, Vec<u32>)> = log
            .into_iter()
            .filter(|(addr, _)| *addr != ADD_CHANGE)
            .collect();
        assert_eq!(
            calls[1..],
            [
                (TES_FORM_CONSTRUCT, vec![base]),
                (SCRIPTABLE_FORM_CONSTRUCT, vec![base + 0x18]),
                (ICON_CONSTRUCT, vec![base + 0x24]),
                (FULL_NAME_CONSTRUCT, vec![base + 0x30]),
                (LIST_CONSTRUCT, vec![base + 0x44]),
                (LIST_CONSTRUCT, vec![base + 0x4c]),
                (CONDITION_LIST_CONSTRUCT, vec![base + 0x54]),
                (STRING_CONSTRUCT, vec![base + 0x64]),
                (SET_FORM_TYPE, vec![base, 0x47]),
                (QUEST_DATA_ADDRESS, vec![base]),
            ]
        );
        assert_eq!(e.mem.u32(base), TES_QUEST_VTABLE);
        assert_eq!(e.mem.u32(base + 0x18), SCRIPTABLE_FORM_VTABLE);
        assert_eq!(e.mem.u32(base + 0x24), ICON_VTABLE);
        assert_eq!(e.mem.u32(base + 0x30), FULL_NAME_VTABLE);
        assert_eq!(e.get(quest.cast::<TESForm>(), TESForm::cFormType), 0x47);
        assert_eq!(e.get(quest, TESQuest::pScriptLocals), Ptr::NULL);
        assert_eq!(e.get(quest, TESQuest::cCurrentStage), 0);
        assert_eq!(e.mem.u8(base + DATA) & 1, 1);
        assert_eq!(e.mem.u32(base + CALLS), 1);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0060c440() {
        assert_eq!(
            check(0x0060_c440, 0x0060_1370, 0xdc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0060c450() {
        assert_eq!(check(0x0060_c450, 0x0060_8da0, 0x7c, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_0060c460() {
        check(0x0060_c460, 0x0060_8e80, 0x90, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0060c470() {
        check(0x0060_c470, 0x0060_8e00, 0x90, &[0x77]);
    }

    #[test]
    fn le81_thunk_0060c480() {
        check(0x0060_c480, 0x0060_8e80, 0x7c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0060c490() {
        assert_eq!(check(0x0060_c490, 0x0060_8da0, 0x90, &[0x77]).u16(), 0xbeef);
    }

    #[test]
    fn le81_thunk_0060c4a0() {
        check(0x0060_c4a0, 0x0060_8e00, 0x7c, &[0x77]);
    }
}
