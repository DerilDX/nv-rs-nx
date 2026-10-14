//! The game's own code, started inside nv-rs.
//!
//! The engine crate holds FalloutNV.exe 1.4.0.525 translated function by
//! function on a model of the game's memory (docs/ENGINE_CRATE.md). This
//! crate starts it for the running game: it finds the player's own
//! FalloutNV.exe in the installation that holds `Data`, refuses any build
//! but 1.4.0.525 (the one every translated address belongs to), maps the
//! executable's data sections (read-only use of the file: nothing is
//! written to the installation, nothing from it is stored), runs the
//! executable's static initializers as `__cinit` does before `WinMain`
//! ([`engine::static_init`]) and the allocator start-up the game runs before
//! any form exists.
//!
//! The engine is not `Send`, `Sync` or `Clone` (its memory and test doubles
//! are `Rc<RefCell<..>>`); a [`Host`] is owned by one thread and handed to
//! the world as plain owned data. What it is connected to: docs/ENGINE_CRATE.md
//! ("In the running game").

pub mod global_form;
pub mod quest_form;
pub mod script_form;

use engine::Engine;
use std::path::{Path, PathBuf};

/// The game's translated code, started on the player's executable.
pub struct Host {
    pub engine: Engine,
    /// The executable whose data sections are mapped.
    pub exe: PathBuf,
    /// What the static initializers' run did (entries run, skipped).
    pub static_init: engine::static_init::Report,
}

impl Host {
    /// Starts the game's code for the installation whose `Data` folder is
    /// `data`: its `FalloutNV.exe` ([`engine::exe::installed_exe`]),
    /// checked to be 1.4.0.525 ([`engine::exe::checked_sections`], the check
    /// `engine::exe::verify` makes) and mapped; then the static initializers
    /// ([`engine::static_init::run`], in `__cinit`'s order, the untranslated
    /// ones skipped and counted), the memory manager's among them replaced by
    /// [`start_allocator`] ([`STAND_INS`]); then the game objects forms need
    /// when they are made ([`start_game_objects`]). The error says which file and why (missing,
    /// not a PE file, another build).
    pub fn start(data: &Path) -> Result<Host, String> {
        let exe = engine::exe::installed_exe(data);
        if !exe.is_file() {
            return Err(format!(
                "{}: not found. nv-rs reads the game's own FalloutNV.exe (version \
                 1.4.0.525) from the installation whose Data folder it is given.",
                exe.display()
            ));
        }
        let mut engine = Engine::with_exe(&exe)?;
        let static_init = engine::static_init::run(&mut engine, &STAND_INS);
        start_game_objects(&mut engine);
        Ok(Host {
            engine,
            exe,
            static_init,
        })
    }

    /// The start-up diagnostic line.
    pub fn describe(&self) -> String {
        format!(
            "Game code: {} (1.4.0.525), {} translated functions; static initializers: \
             {} run from the table, {} by address, {} by platform code, {} skipped \
             (not translated yet).",
            self.exe.display(),
            self.engine.translated_count(),
            self.static_init.rows,
            self.static_init.translated,
            self.static_init.platform,
            self.static_init.skipped()
        )
    }
}

/// The objects of the game's start-up (`WinMain`) that making forms needs,
/// made by the game's own code. Today one: `BGSSaveLoadGame::Create`
/// (Xbox PDB, `00847480`), which `TES::TES` (`0044fb20`) calls: the save/load
/// game singleton at `011ddf38`, which every form constructor that records a
/// change (`TESForm` `00484b60`/`00484b90` → `BGSSaveLoadGame::AddChange`
/// `0084a690`) reads; the game makes it before it loads any form. The rest
/// of `TES::TES` doesn't run: eight of its callees aren't translated
/// (`00c44980`, `004ba280`, `00528cb0`, `00626fd0`, `0084fc70`, `00856380`,
/// `00c3da50`, `00624bf0`).
pub fn start_game_objects(e: &mut Engine) {
    e.call(0x0084_7480, &[]);
}

/// The static initializers nv-rs runs its own code for. Entry `00fb66a0`
/// constructs the `MemoryManager` (Xbox PDB) singleton at `011f6238`
/// (`00aa3880`): its fields, a memory check, `MemoryManager::Initialize`,
/// the Ni allocator set-up `00866690` and `00aa58d0`. The memory manager is
/// platform (the engine's heap, `Mem`), so in its place, at the same point of
/// the table, runs the allocator set-up of `00866690` ([`start_allocator`]),
/// which the settings' maps need (`00aa1070` allocates through `011f6080`).
pub const STAND_INS: [engine::static_init::StandIn; 1] = [(0x00fb_66a0, start_allocator)];

/// The Ni allocator set-up of `00866690` (called by the memory manager's
/// constructor, `00aa3880`), as the engine's probes and the opt-in
/// interpreter (`crates/engine/tests/real_scda.rs`) run it: `operator new` (`00401000`) of the holder (8 bytes) and of a
/// `BSNiAllocator` (Xbox PDB, 4 bytes) that `00aa2230` gives its vtable,
/// the holder made by `00aa21d0` (allocator stored, byte +4 cleared) and
/// stored at `011f6118`, then `00aa2020`, which resolves that allocator and
/// calls its `Initialize` (slot +0x1c). (`00866690` ends with a call to a
/// library singleton's slot +4 that is not reproduced.)
pub fn start_allocator(e: &mut Engine) {
    let holder = e.call(0x0040_1000, &[8]).u32();
    let allocator = e.call(0x0040_1000, &[4]).u32();
    let allocator = e.call(0x00aa_2230, &[allocator]).u32();
    let holder = e.call(0x00aa_21d0, &[holder, allocator]).u32();
    e.mem.set_u32(0x011f_6118, holder);
    e.call(0x00aa_2020, &[]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_installation_without_the_executable_is_refused_by_path() {
        let data = std::env::temp_dir()
            .join("nv-rs-host-test-no-game")
            .join("Data");
        let err = match Host::start(&data) {
            Ok(_) => panic!("started without an executable"),
            Err(err) => err,
        };
        assert!(err.contains("FalloutNV.exe: not found"), "{err}");
        assert!(err.contains("1.4.0.525"), "{err}");
    }

    /// With `NV_EXE` set to an installed FalloutNV.exe 1.4.0.525: the game
    /// code starts on its folder, the allocator holder is in place and
    /// `operator new` hands out memory.
    #[test]
    fn starts_on_the_installed_executable_when_given() {
        let Some(path) = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty()) else {
            return;
        };
        let exe = PathBuf::from(path);
        let data = exe.parent().unwrap().join("Data");
        let mut host = Host::start(&data).unwrap();
        assert_eq!(host.exe, exe);
        assert_ne!(host.engine.mem.u32(0x011f_6118), 0);
        let block = host.engine.call(0x0040_1000, &[0x20]).u32();
        assert_ne!(block, 0);
        assert!(host.describe().contains("1.4.0.525"));
    }
}
