//! A plugin file in the game's memory (`TESFile`, Xbox PDB), read by the
//! game's own code from the bytes the world's reader already has.
//!
//! The game reads a record with `TESFile::SetOffset` (`004723a0`: seek to
//! the record, read its header) and then the form's `Load`, which reads its
//! chunks (`GetTESChunk`, `GetChunkData`, `NextChunk`, all translated)
//! through the file's `BSFile`. `BSFile` is platform: its reads and seeks are
//! stand-ins over an in-memory copy of the parts of the file the host
//! supplies, at their real offsets (`engine::units::platform::open_memory_file`).
//!
//! [`GameFile::open`] makes the `TESFile` with the game's constructor
//! (`TESFile::TESFile`, `00470710`, given no name, so it opens nothing) and
//! then leaves it as `TESFile::OpenTES` (`00470c70`) leaves an open file:
//! its `BSFile` (`m_pFile`), its size (`m_filesize`) and its load-order
//! index (`cCompileIndex`, which `ReadFormHeader` puts in the form ids it
//! reads). Bridged, not run: `OpenTES` itself (it finds the file on disk,
//! constructs the `BSFile` and reads the file's own header, `TES4`, and
//! its masters). So a plugin with masters isn't supported yet: its form ids
//! need the master table `OpenTES` builds ([`GameFile::open`] refuses it).
//!
//! Compressed records need zlib (`TESFile::DecompressCurrentForm`), which
//! isn't translated: [`GameFile::supply`] refuses them.

use engine::units::platform::{open_memory_file, supply_file_bytes};
use engine::Engine;
use esm::{LoadedPlugin, RecordEntry};

/// `operator new` (`00401000`) and `TESFile::TESFile(directory, name, mode)`
/// (`00470710`).
const NEW: u32 = 0x0040_1000;
const TES_FILE_CONSTRUCT: u32 = 0x0047_0710;
/// `TESFile` (Xbox PDB): its size and the fields `OpenTES` leaves.
const TES_FILE_SIZE: u32 = 0x42c;
const M_PFILE: u32 = 0x10;
const M_FILESIZE: u32 = 0x260;
const C_COMPILE_INDEX: u32 = 0x40c;
/// `TESFile::SetOffset` (Xbox PDB, `004723a0`).
const SET_OFFSET: u32 = 0x0047_23a0;
/// A record header's size (`FORM`, 0x18 bytes).
const RECORD_HEADER_SIZE: usize = 0x18;
/// A record's compressed flag (`fn_00472100`).
const RECORD_COMPRESSED: u32 = 0x0004_0000;

/// A plugin's `TESFile` in the game's memory.
#[derive(Debug, Clone, Copy)]
pub struct GameFile {
    /// The `TESFile`.
    pub file: u32,
    /// Its `BSFile` (an in-memory file).
    pub bs_file: u32,
}

impl GameFile {
    /// The `TESFile` of `plugin`, its compile index its load-order index;
    /// see the module for what is bridged.
    pub fn open(e: &mut Engine, plugin: &LoadedPlugin) -> Result<GameFile, String> {
        let index = plugin.load_index;
        let name = plugin.name.clone();
        if !plugin.plugin.header().masters.is_empty() {
            return Err(format!(
                "{name}: has masters, whose form ids need the master table TESFile::OpenTES \
                 builds (not run yet)"
            ));
        }
        let size = u32::try_from(plugin.plugin.file_size())
            .map_err(|_| format!("{name}: larger than 4 GB"))?;
        let block = e.call(NEW, &[TES_FILE_SIZE]).u32();
        let file = e.call(TES_FILE_CONSTRUCT, &[block, 0, 0, 0]).u32();
        let bs_file = open_memory_file(e, &name, size);
        e.mem.set_u32(file + M_PFILE, bs_file);
        e.mem.set_u32(file + M_FILESIZE, size);
        e.mem.set_u8(file + C_COMPILE_INDEX, index);
        Ok(GameFile { file, bs_file })
    }

    /// Supplies the record `entry`'s bytes (header and data, as in the
    /// file) and goes to it as the game does (`TESFile::SetOffset`), so
    /// the form's `Load` reads it next. A compressed record is refused.
    pub fn supply(
        &self,
        e: &mut Engine,
        plugin: &LoadedPlugin,
        entry: &RecordEntry,
    ) -> Result<(), String> {
        if entry.header.flags & RECORD_COMPRESSED != 0 {
            return Err(format!(
                "{} {:?} in {}: compressed; reading it needs zlib (TESFile::DecompressCurrentForm, \
                 00b44000 and nearby), which isn't translated",
                entry.header.kind, entry.header.form_id, plugin.name
            ));
        }
        // The record, and the 0x18 bytes after it (the next record's or
        // group's header): reading a record's last chunk can read the next
        // chunk header past its end, where the game reads the file's next
        // bytes before its bound check stops it.
        let offset = entry.offset as u32;
        let end = entry.offset + plugin.plugin.record_bytes(entry).len() + RECORD_HEADER_SIZE;
        let bytes = plugin.plugin.file_bytes(entry.offset..end);
        supply_file_bytes(e, self.bs_file, offset, bytes);
        if !e.call(SET_OFFSET, &[self.file, offset]).bool() {
            return Err(format!(
                "{} {:?} in {}: TESFile::SetOffset failed at {offset:#x}",
                entry.header.kind, entry.header.form_id, plugin.name
            ));
        }
        Ok(())
    }
}
