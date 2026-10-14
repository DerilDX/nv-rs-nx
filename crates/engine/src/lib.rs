//! FalloutNV.exe 1.4.0.525 translated into Rust function by function, on a
//! model of the game's own memory (ADR-0006, docs/ENGINE_CRATE.md).
//!
//! Game objects live in [`Mem`] at their real field offsets and pointers are
//! the game's 32-bit addresses ([`Ptr`]). Every translated function is
//! registered under its exe address in a uniform calling form
//! ([`abi`]), so a translation calls any other function by address
//! ([`Engine::call`], [`Engine::vcall`]) whether or not it has been
//! translated yet; a call to an untranslated one stops with its address.
//! The data sections of the player's own executable are mapped at run time
//! ([`exe`]), so constants, strings, vtables and initial globals are the
//! game's.
//!
//! Translations live in `units/<subsystem>/<unit>.rs`, one file per source
//! unit of the Xbox 360 prototype (Xbox PDB, ADR-0002), as listed in
//! `research/engine-map/engine_map.tsv`. Each is marked
//! `// Translated from <addr> (decompiled, FalloutNV.exe 1.4.0.525)`
//! (ADR-0003); `docs/LEDGER.md` counts them.

// Translation doc comments quote offsets and arithmetic ("+ 0x3c", "+ 1")
// that markdown reads as list items; that only affects rendering, so the
// lint is allowed for the crate rather than reflowing thousands of comments.
#![allow(clippy::doc_lazy_continuation)]

pub mod abi;
mod engine;
pub mod exe;
pub mod extra_data;
pub mod mem;
pub mod ptr;
pub mod reference;
pub mod static_init;
pub mod types;
pub mod units;
pub mod x87;

pub use abi::{AbiFn, Ret};
pub use engine::Engine;
pub use mem::Mem;
pub use ptr::{Field, Inline, Layout, Ptr, Scalar};

/// What every unit file imports.
pub mod prelude {
    pub use crate::abi::{AbiFn, Arg, Ret, RetVal};
    pub use crate::ptr::{Field, Inline, Layout, Ptr, Scalar};
    pub use crate::x87::{x87_quiet, x87_quiet_bits};
    pub use crate::Engine;
    pub use crate::{args, entry, layout};
}
