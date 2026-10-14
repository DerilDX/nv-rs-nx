//! `fallout shared/testexture.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// The current chunk's size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall
/// (file; buffer, size)`, here with size 0.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// The CRT's `strlen` (`00ec6130`, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// The string setter at `00489100` (`thiscall (texture; text)`; engine map
/// name `TESSoundFile::SetSoundFile`, a shared body).
const SET_STRING: u32 = 0x0048_9100;

// Translated from 0048e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048e3d0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESTexture::LoadTextureChunk` (Xbox PDB), `cdecl (texture, file)`: with
/// both non-null, reads the current chunk into a stack buffer of the chunk's
/// size (`_alloca_probe_16`), drops one leading backslash (shifting the text,
/// its terminator included, one byte left, with `strlen` taken again on each
/// step as the game does) and hands the text to the texture's string setter
/// (`00489100`). An empty chunk sets a null string. The chunk id is not
/// checked.
pub fn tes_texture_load_texture_chunk(e: &mut Engine, texture: Ptr, file: Ptr<TESFile>) {
    if file.addr() == 0 || texture.addr() == 0 {
        return;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() == 0 {
        e.call(SET_STRING, &args![texture, 0u32]);
        return;
    }
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = size.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, buffer| {
        let text = buffer.addr();
        e.call(GET_CHUNK_DATA, &args![file, text, 0u32]);
        if e.call(STRLEN, &args![text]).u32() != 0 && e.mem.u8(text) == b'\\' {
            let mut index = 0u32;
            while index < e.call(STRLEN, &args![text]).u32() {
                let next = e.mem.u8(text.wrapping_add(index).wrapping_add(1));
                e.mem.set_u8(text.wrapping_add(index), next);
                index = index.wrapping_add(1);
            }
        }
        e.call(SET_STRING, &args![texture, text]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0048e3d0,
        tes_texture_load_texture_chunk(Ptr, Ptr<TESFile>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Loads a chunk holding `data` (its size is `data.len()`); returns the
    /// log and the text the setter saw.
    fn run(texture: u32, file: u32, data: &'static [u8]) -> (CallLog, Option<String>) {
        let mut e = Engine::new();
        let size = data.len() as u32;
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register_double(GET_CHUNK_DATA, move |e, a| {
            e.mem.write(a[1], data);
            Ret::default()
        });
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let record = seen.clone();
        e.register_double(SET_STRING, move |e, a| {
            let text = if a[1] == 0 {
                String::from("<null>")
            } else {
                let mut bytes = Vec::new();
                let mut at = a[1];
                while e.mem.u8(at) != 0 {
                    bytes.push(e.mem.u8(at));
                    at += 1;
                }
                String::from_utf8(bytes).unwrap()
            };
            *record.borrow_mut() = Some(text);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_e3d0, &args![texture, file]);
        let log = e.call_log.take().unwrap();
        let text = seen.borrow().clone();
        (log, text)
    }

    #[test]
    fn drops_one_leading_backslash() {
        let (log, text) = run(0x5000, 0x6000, b"\\\\a\\b.dds\0");
        assert_eq!(text.as_deref(), Some("\\a\\b.dds"));
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!((reads[0][0], reads[0][2]), (0x6000, 0));
        assert_eq!(calls_to(&log, SET_STRING)[0][0], 0x5000);
        // Once for the test, then once per step (nine, the terminator moves
        // at the last) and once more to end the loop.
        assert_eq!(calls_to(&log, STRLEN).len(), 1 + 9 + 1);
    }

    #[test]
    fn keeps_a_path_without_a_leading_backslash() {
        let (log, text) = run(0x5000, 0x6000, b"a\\b.dds\0");
        assert_eq!(text.as_deref(), Some("a\\b.dds"));
        assert_eq!(calls_to(&log, STRLEN).len(), 1);
        let (_, text) = run(0x5000, 0x6000, b"\0");
        assert_eq!(text.as_deref(), Some(""));
    }

    #[test]
    fn an_empty_chunk_sets_a_null_string_and_null_arguments_do_nothing() {
        let (log, text) = run(0x5000, 0x6000, b"");
        assert_eq!(text.as_deref(), Some("<null>"));
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());
        let (log, text) = run(0, 0x6000, b"x\0");
        assert!(text.is_none());
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
        let (log, _) = run(0x5000, 0, b"x\0");
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
    }
}
