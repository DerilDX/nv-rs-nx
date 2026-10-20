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

/// `ICON`.
const CHUNK_ICON: u32 = 0x4e4f_4349;
/// `"%s_n.dds"`.
const NORMAL_FORMAT: u32 = 0x0101_d7e0;

// Translated from 0048e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESTexture::IsTextureChunk` (engine map), cdecl: whether the chunk is
/// `ICON`.
pub fn tes_texture_is_texture_chunk(_e: &mut Engine, chunk: u32) -> bool {
    chunk == CHUNK_ICON
}

// Translated from 0048e5b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0048e5b0 - the /GS stack cookie check (00ec408c) is not translated.
/// `TESTexture::GetAsNormalFile` (engine map), cdecl: sets the string
/// `result` to `path` with its extension replaced by `_n.dds` (the path
/// copied with `00406d30`, the last `.` found with `0040ab30`, formatted
/// with `"%s_n.dds"`, set with `004037f0`) and answers its text
/// (`00559450`); answers 0 without a path or an extension. The stack
/// cookie check is not translated.
pub fn tes_texture_get_as_normal_file(e: &mut Engine, result: Ptr, path: u32) -> u32 {
    if path == 0 {
        return 0;
    }
    e.with_stack(0x210, |e, buffers| {
        let copy = buffers.addr();
        let normal = buffers.addr() + 0x108;
        e.call(0x0040_6d30, &args![copy, 0x104u32, path]);
        let dot = e.call(0x0040_ab30, &args![copy, 0x2eu32]).u32();
        if dot == 0 {
            return 0;
        }
        e.mem.set_u8(dot, 0);
        e.call(0x0040_6d00, &args![normal, 0x104u32, NORMAL_FORMAT, copy]);
        e.call(0x0040_37f0, &args![result, normal, 0u32]);
        e.call(0x0055_9450, &args![result]).u32()
    })
}

// Translated from 0048e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The normal-map name of this texture: `TESTexture::GetAsNormalFile`
/// into `result` with the texture's path (`00408da0`, engine map name
/// `MapMarkerData::GetLocationName`), whose `EAX` it passes on.
/// `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_0048e580(e: &mut Engine, this: Ptr, result: Ptr) -> u32 {
    let path = e.call(0x0040_8da0, &args![this]).u32();
    tes_texture_get_as_normal_file(e, result, path)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0048e3d0,
            tes_texture_load_texture_chunk(Ptr, Ptr<TESFile>)
        ),
        entry!(0x0048_e3b0, tes_texture_is_texture_chunk(u32) -> bool),
        entry!(0x0048_e5b0, tes_texture_get_as_normal_file(Ptr, u32) -> u32),
        entry!(0x0048_e580, fn_0048e580(Ptr, Ptr) -> u32),
    ]
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

#[cfg(test)]
mod b022_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn only_icon_is_a_texture_chunk() {
        let mut e = Engine::new();
        assert!(e.call(0x0048_e3b0, &[CHUNK_ICON]).bool());
        assert!(!e.call(0x0048_e3b0, &[0x4c44_4f4d]).bool());
    }

    /// Doubles that behave as the CRT helpers: copy, last `.`, format
    /// (`%s_n.dds`), set (records the text).
    fn engine() -> (Engine, Rc<RefCell<Vec<u8>>>) {
        let mut e = Engine::new();
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(0x0040_ab30, |e, a| {
            let text = e.mem.cstr(a[0]);
            ret(text
                .iter()
                .rposition(|&c| c == a[1] as u8)
                .map_or(0, |i| a[0] + i as u32))
        });
        e.register(0x0040_6d00, |e, a| {
            let mut text = e.mem.cstr(a[3]);
            text.extend(b"_n.dds");
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        let set = Rc::new(RefCell::new(vec![]));
        let seen = set.clone();
        e.register_double(0x0040_37f0, move |e, a| {
            *seen.borrow_mut() = e.mem.cstr(a[1]);
            Ret::default()
        });
        e.register(0x0055_9450, |_, a| ret(a[0] + 0x1000));
        e.call_log = Some(vec![]);
        (e, set)
    }

    #[test]
    fn the_normal_map_name_replaces_the_extension() {
        let (mut e, set) = engine();
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"textures/a.b/rock.dds");
        let result = e.mem.alloc(8);
        assert_eq!(e.call(0x0048_e5b0, &[result, path]).u32(), result + 0x1000);
        assert_eq!(*set.borrow(), b"textures/a.b/rock_n.dds");
        let formats: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x0040_6d00)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(formats[0][1..3], [0x104, NORMAL_FORMAT]);
    }

    #[test]
    fn no_path_or_no_extension_answers_zero() {
        let (mut e, set) = engine();
        let result = e.mem.alloc(8);
        assert_eq!(e.call(0x0048_e5b0, &[result, 0]).u32(), 0);
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"textures/rock");
        assert_eq!(e.call(0x0048_e5b0, &[result, path]).u32(), 0);
        assert!(set.borrow().is_empty());
    }

    #[test]
    fn the_texture_passes_its_own_path() {
        let (mut e, set) = engine();
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, b"x.dds");
        e.register_double(0x0040_8da0, move |_, _| ret(path));
        let result = e.mem.alloc(8);
        assert_eq!(
            e.call(0x0048_e580, &[0x5000, result]).u32(),
            result + 0x1000
        );
        assert_eq!(*set.borrow(), b"x_n.dds");
    }
}
