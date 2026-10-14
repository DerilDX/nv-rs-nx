//! Opt-in survey: every quest of the player's `FalloutNV.esm` that has a
//! script, made in the game's memory and run once through the game's
//! interpreter (`0060d4d0` → `Script::Run`), recording what each run
//! changed and where it stopped (an untranslated function, an unmapped
//! read).
//!
//! ```text
//! NV_DATA="C:\...\Fallout New Vegas\Data" cargo test -p host --release \
//!     --test live_quest_survey -- --ignored --nocapture
//! ```

use esm::{FormId, FourCC, Plugin};
use host::quest_form::QuestForm;
use host::script_form::ScriptForm;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
#[ignore = "needs the player's FalloutNV.exe and FalloutNV.esm (set NV_DATA)"]
fn every_scripted_quest_runs_once() {
    let data = PathBuf::from(std::env::var("NV_DATA").expect("NV_DATA: the game's Data folder"));
    let mut host = host::Host::start(&data).unwrap_or_else(|e| panic!("{e}"));
    let plugin = Plugin::open(data.join("FalloutNV.esm")).expect("FalloutNV.esm");
    let e = &mut host.engine;
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut stops: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let (mut quests, mut ran, mut changed) = (0, 0, Vec::new());
    for entry in plugin.records_of_type(FourCC(*b"QUST")) {
        let record = plugin.record(entry).expect("QUST");
        let name = record.editor_id().unwrap_or_default();
        let Some(script_id) = record
            .get(FourCC(*b"SCRI"))
            .filter(|s| s.data.len() >= 4)
            .map(|s| FormId(u32::from_le_bytes(s.data[..4].try_into().unwrap())))
        else {
            continue;
        };
        let Some(script_entry) = plugin.get(script_id) else {
            continue;
        };
        let script_record = plugin.record(script_entry).expect("SCPT");
        quests += 1;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let script = ScriptForm::load(e, script_id, &script_record, |id| id)?;
            let quest = QuestForm::load(e, record.header.form_id, &record, &script)?;
            let before = quest.snapshot(e, &script);
            quest.run(e);
            let after = quest.snapshot(e, &script);
            Ok::<_, String>((before, after))
        }));
        match result {
            Ok(Ok((before, after))) => {
                ran += 1;
                if before != after {
                    changed.push(format!("{name}: {before:?} -> {after:?}"));
                }
            }
            Ok(Err(err)) => stops.entry(format!("load: {err}")).or_default().push(name),
            Err(payload) => {
                let text = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_default();
                let first = text.lines().next().unwrap_or("").to_string();
                stops.entry(first).or_default().push(name);
            }
        }
    }
    std::panic::set_hook(hook);
    println!("quests with a script: {quests}; ran to the end: {ran}");
    let mut by_count: Vec<_> = stops.iter().collect();
    by_count.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));
    for (stop, names) in by_count {
        println!(
            "  stopped {}: {stop}  [{}]",
            names.len(),
            names.iter().take(4).cloned().collect::<Vec<_>>().join(", ")
        );
    }
    println!("changed by one run: {}", changed.len());
    for c in changed.iter().take(40) {
        println!("  {c}");
    }
}
