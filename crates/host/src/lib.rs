//! The game's own code, started inside nv-rs.
//!
//! The engine crate holds FalloutNV.exe 1.4.0.525 translated function by
//! function on a model of the game's memory (docs/ENGINE_CRATE.md). This
//! crate starts it for the running game: it finds the player's own
//! FalloutNV.exe in the installation that holds `Data`, refuses any build
//! but 1.4.0.525 (the one every translated address belongs to), maps the
//! executable's data sections (read-only use of the file: nothing is
//! written to the installation, nothing from it is stored) and runs the
//! allocator start-up the game runs before any form exists.
//!
//! The engine is not `Send`, `Sync` or `Clone` (its memory and test doubles
//! are `Rc<RefCell<..>>`); a [`Host`] is owned by one thread and handed to
//! the world as plain owned data. What it is connected to: docs/ENGINE_CRATE.md
//! ("In the running game").

pub mod global_form;
pub mod script_form;

use engine::Engine;
use std::path::{Path, PathBuf};

/// The game's translated code, started on the player's executable.
pub struct Host {
    pub engine: Engine,
    /// The executable whose data sections are mapped.
    pub exe: PathBuf,
}

impl Host {
    /// Starts the game's code for the installation whose `Data` folder is
    /// `data`: its `FalloutNV.exe` ([`engine::exe::installed_exe`]),
    /// checked to be 1.4.0.525 ([`engine::exe::checked_sections`], the check
    /// `engine::exe::verify` makes) and mapped, then the allocator started
    /// ([`start_allocator`]). The error says which file and why (missing,
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
        start_allocator(&mut engine);
        Ok(Host { engine, exe })
    }

    /// The start-up diagnostic line.
    pub fn describe(&self) -> String {
        format!(
            "Game code: {} (1.4.0.525), {} translated functions, allocator started.",
            self.exe.display(),
            self.engine.translated_count()
        )
    }
}

/// The allocator start-up that runs before any form exists, as the engine's
/// probes and the opt-in interpreter run (`crates/engine/tests/real_scda.rs`)
/// run it: `operator new` (`00401000`) of the holder (8 bytes) and of a
/// `BSNiAllocator` (Xbox PDB, 4 bytes) that `00aa2230` gives its vtable,
/// the holder made by `00aa21d0` (allocator stored, byte +4 cleared) and
/// stored at `011f6118`, then `00aa2020`, which resolves that allocator and
/// calls its `Initialize` (slot +0x1c). The rest of the game's system
/// start-up (`WinMain`'s) is not reproduced.
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
