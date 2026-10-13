//! Opt-in: the executable's static initializers, which the game runs before
//! `WinMain` and [`host::Host::start`] does not run yet.
//!
//! `__cinit` (`00eccd14`) runs two tables of function pointers in `.rdata`:
//! `_initterm_e(010113c4, 010113e8)` (the C initializers) and
//! `_initterm(00fdf4a4, 010113c0)` (the C++ dynamic initializers: the
//! constructors of the game's global objects, `Setting`s with their names and
//! default values among them, each followed by its `atexit` destructor).
//! Until they run, those globals read as zero in the game's memory.
//!
//! ```text
//! NV_EXE="C:\...\Fallout New Vegas\FalloutNV.exe" cargo test -p host --release \
//!     --test static_initializers -- --ignored --nocapture
//! ```
//!
//! `NV_DUMP=<file>` also writes every non-null entry's address, one per line.

use std::path::PathBuf;

const C_TABLE: (u32, u32) = (0x0101_13c4, 0x0101_13e8);
const CPP_TABLE: (u32, u32) = (0x00fd_f4a4, 0x0101_13c0);

fn host() -> host::Host {
    let exe = PathBuf::from(std::env::var("NV_EXE").expect("NV_EXE: the installed FalloutNV.exe"));
    host::Host::start(&exe.parent().unwrap().join("Data")).unwrap_or_else(|e| panic!("{e}"))
}

fn entries(host: &host::Host, (start, end): (u32, u32)) -> Vec<u32> {
    (start..end)
        .step_by(4)
        .map(|slot| host.engine.mem.u32(slot))
        .filter(|&f| f != 0)
        .collect()
}

/// How many initializers there are and how many are translated.
#[test]
#[ignore = "needs the player's FalloutNV.exe (set NV_EXE)"]
fn the_initializer_tables() {
    let host = host();
    let c = entries(&host, C_TABLE);
    let cpp = entries(&host, CPP_TABLE);
    let translated = |list: &[u32]| {
        list.iter()
            .filter(|&&f| host.engine.is_translated(f))
            .count()
    };
    println!(
        "C initializers: {} ({} translated); C++ initializers: {} ({} translated)",
        c.len(),
        translated(&c),
        cpp.len(),
        translated(&cpp)
    );
    if let Ok(path) = std::env::var("NV_DUMP") {
        let text: String = c.iter().chain(&cpp).map(|f| format!("{f:08x}\n")).collect();
        std::fs::write(path, text).unwrap();
    }
    assert!(cpp.len() > 10_000);
}

/// Three game settings as their initializers leave them: the `Setting`
/// (value +4, name +8, `0040e130`) at its global, with the name and the
/// default the initializer passes (`FLD` of a float in `.rdata`). Fails
/// until the initializers run in [`host::Host::start`].
#[test]
#[ignore = "the static initializers are not run yet (script track S6 prerequisite)"]
fn sample_settings_hold_the_executables_defaults() {
    let host = host();
    // (initializer, Setting global, name string, default float)
    for (initializer, setting, name, default) in [
        (
            0x00f6_2250u32,
            0x011c_e7c0u32,
            0x0105_4438u32,
            0x0101_712cu32,
        ), // fGunShellLifetime
        (0x00f6_91f0, 0x011d_0ad8, 0x0105_7dd4, 0x0102_bc30), // fPlayerMaxResistance
        (0x00f6_91c0, 0x011d_1020, 0x0105_7dc4, 0x0102_49d8), // fMaxArmorRating
    ] {
        let text = String::from_utf8_lossy(&host.engine.mem.cstr(name)).into_owned();
        assert_eq!(
            host.engine.mem.u32(setting + 8),
            name,
            "{text} ({initializer:08x}): name"
        );
        assert_eq!(
            host.engine.mem.f32(setting + 4),
            host.engine.mem.f32(default),
            "{text} ({initializer:08x}): default"
        );
    }
}
