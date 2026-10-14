//! A quest in the game's memory, its script run by the game's interpreter.
//!
//! [`QuestForm::load`] makes a `TESQuest` (Xbox PDB) with the game's own
//! constructor and leaves it as `TESQuest::Load` (`0060ceb0`) and the
//! quest's `InitItem` (`0060cba0`) leave a quest that has a script;
//! [`QuestForm::run`] runs its script once as the game runs a quest's script
//! (`0060d4d0`: `Script::Run` with the quest's locals).
//!
//! Bridged, not translated, and why:
//! - `TESQuest::Load` is translated, but it reads the record through
//!   `TESFile`, whose chunk reading isn't (`00401660`, `004726f0`,
//!   `00401680`), nor are the loaders of stages, stage items, objectives,
//!   targets and conditions it calls. So the record's `DATA` is put where
//!   `Load` puts it, and stages, objectives and conditions are not loaded:
//!   the quest has none.
//! - `InitItem` is translated, but the script component's `InitItem`
//!   (`0048cd90`, which resolves the `SCRI` form id through the form map)
//!   and the condition list's (`005f5760`, `00680a20`) are not. So the
//!   script component holds the script already resolved, with its
//!   "initialized" byte set, as `0048cd90` leaves it, and the script branch
//!   of `InitItem` runs through its translated callees in its order. With
//!   no stages, objectives or conditions, the rest of `InitItem` does
//!   nothing but the two flag updates at its end, which run too.

use crate::script_form::ScriptForm;
use engine::units::fallout_shared::run_0060bef0::TESQuest;
use engine::{Engine, Ptr};
use esm::{FormId, FourCC, Record};

/// `TESDataHandler::CreateFormOfType` (Xbox PDB, `00465110`).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
/// Form type of `TESQuest` (`QUST`).
const FORM_TYPE_QUEST: u32 = 0x47;
/// `TESForm::iFormID` (Xbox PDB) +0xC.
const FORM_ID: u32 = 0x0c;
/// The quest's `TESScriptableForm` (Xbox PDB) component: the script (+4 of
/// it) and its "initialized" byte (+8 of it).
const SCRIPTABLE: u32 = 0x18;
const SCRIPTABLE_SCRIPT: u32 = SCRIPTABLE + 4;
const SCRIPTABLE_INITIALIZED: u32 = SCRIPTABLE + 8;
/// `QUEST_DATA` (Xbox PDB) +0x3C, 8 bytes, as `TESForm::LoadData`
/// (`004861f0`) copies `DATA`.
const DATA: u32 = 0x3c;
const DATA_SIZE: usize = 8;

/// The callees of the script branch of `InitItem` (`0060cba0`), in its
/// order: the script's new `ScriptLocals` (`005abf60`), the script's owner
/// (`0087ce80`, `thiscall` on the script), `TESForm::ForceChange`
/// (`00484b90`) with the script-locals change flag, the INI setting
/// (`0040_8d60` on `011cb81c`) that enables the quest (`0060c9c0`), then the
/// quest's enabled flag (`00455620`) copied to bit 4 of its data
/// (`0060cd00`) and the form's changed flag (`00484ab0` with 1).
const NEW_LOCALS: u32 = 0x005a_bf60;
const SCRIPT_SET_OWNER: u32 = 0x0087_ce80;
const FORCE_CHANGE: u32 = 0x0048_4b90;
const CHANGE_SCRIPT_LOCALS: u32 = 0x4000_0000;
const SETTING_BOOL: u32 = 0x0040_8d60;
const SETTING_ENABLE_QUESTS: u32 = 0x011c_b81c;
const SET_ENABLED: u32 = 0x0060_c9c0;
const QUEST_ENABLED: u32 = 0x0045_5620;
const COPY_ENABLED_TO_FLAGS: u32 = 0x0060_cd00;
const SET_FORM_FLAG_CHANGED: u32 = 0x0048_4ab0;
/// Runs the quest's script (`0060d4d0`).
const RUN_QUEST_SCRIPT: u32 = 0x0060_d4d0;
/// `ScriptLocals::GetVariable(id, script)` (Xbox PDB, `005a9140`).
const GET_VARIABLE: u32 = 0x005a_9140;

/// A quest made in the game's memory.
#[derive(Debug, Clone, Copy)]
pub struct QuestForm {
    pub form: u32,
    pub id: FormId,
    /// Its script form.
    pub script: u32,
}

/// What a quest run can change: the current stage and the script's
/// variables (by name, in the script's order).
#[derive(Debug, Clone, PartialEq)]
pub struct QuestState {
    pub stage: u8,
    pub variables: Vec<(String, f64)>,
}

impl QuestForm {
    /// Makes the quest `id` from its `record`, its script `script` already
    /// made ([`ScriptForm::load`]); see the module for what is bridged.
    pub fn load(
        e: &mut Engine,
        id: FormId,
        record: &Record,
        script: &ScriptForm,
    ) -> Result<QuestForm, String> {
        let form = e.call(CREATE_FORM_OF_TYPE, &[FORM_TYPE_QUEST]).u32();
        if form == 0 {
            return Err(format!("quest {id}: CreateFormOfType gave no form"));
        }
        e.mem.set_u32(form + FORM_ID, id.0);
        if let Some(data) = record.get(FourCC(*b"DATA")) {
            let n = data.data.len().min(DATA_SIZE);
            e.mem.write(form + DATA, &data.data[..n]);
        }
        e.mem.set_u32(form + SCRIPTABLE_SCRIPT, script.form);
        e.mem.set_u8(form + SCRIPTABLE_INITIALIZED, 1);
        let quest = Ptr::<TESQuest>::new(form);
        let locals = e.call(NEW_LOCALS, &[script.form]).u32();
        e.set(quest, TESQuest::pScriptLocals, Ptr::new(locals));
        e.call(SCRIPT_SET_OWNER, &[script.form, form]);
        if locals != 0 {
            e.call(FORCE_CHANGE, &[form, CHANGE_SCRIPT_LOCALS]);
        }
        let setting = e.call(SETTING_BOOL, &[SETTING_ENABLE_QUESTS]).u32();
        if e.mem.u8(setting) != 0 {
            e.call(SET_ENABLED, &[form, 1]);
        }
        let enabled = e.call(QUEST_ENABLED, &[form]).u32() & 0xff;
        e.call(COPY_ENABLED_TO_FLAGS, &[form, enabled]);
        e.call(SET_FORM_FLAG_CHANGED, &[form, 1]);
        Ok(QuestForm {
            form,
            id,
            script: script.form,
        })
    }

    /// The quest's `ScriptLocals`.
    pub fn locals(&self, e: &mut Engine) -> u32 {
        e.get(Ptr::<TESQuest>::new(self.form), TESQuest::pScriptLocals)
            .addr()
    }

    /// Runs the quest's script once (`0060d4d0`).
    pub fn run(&self, e: &mut Engine) {
        e.call(RUN_QUEST_SCRIPT, &[self.form]);
    }

    /// The quest's current stage and its script's variables now.
    pub fn snapshot(&self, e: &mut Engine, script: &ScriptForm) -> QuestState {
        let locals = self.locals(e);
        let stage = e.get(Ptr::<TESQuest>::new(self.form), TESQuest::cCurrentStage);
        let variables = script
            .variables
            .iter()
            .map(|v| {
                let value = e.call(GET_VARIABLE, &[locals, v.id, self.script]).f64();
                (v.name.clone(), value)
            })
            .collect();
        QuestState { stage, variables }
    }
}
