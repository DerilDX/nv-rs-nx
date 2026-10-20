//! `fallout shared/mediaset.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::GetFile` (`00484e60`), `TESForm::AddCompileIndex` (`00485d50`,
/// cdecl), the form lookup by id (`004839c0`, cdecl) and `__RTDynamicCast`
/// (`00ec43fb`, cdecl, five words) from `.?AVTESForm@@` to
/// `.?AVTESSound@@` (`011832fc`).
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const LOOKUP_FORM: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_SOUND: u32 = 0x0118_32fc;
/// The warning printer (`005b5e40`, cdecl, variable arguments) and
/// `"MASTERFILE: Could not find open sound (%08X) for MediaSet '%s'
/// (%08X)."`
const WARNING: u32 = 0x005b_5e40;
const MISSING_SOUND_FORMAT: u32 = 0x0103_4428;
/// `"Data\Music\"`.
const MUSIC_FOLDER: u32 = 0x0103_4470;

/// `TESFile` chunk reading (see `bgsaddonnode.rs`).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_CHUNK_DATA: u32 = 0x0047_27f0;
const FILE_CHUNK_DATA_SIZED: u32 = 0x0047_2890;

/// `name` (four characters) as a chunk type.
const fn chunk(name: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*name)
}

/// The track name chunks and their tracks' offsets.
const TRACK_NAMES: [(u32, u32); 6] = [
    (chunk(b"NAM2"), 0x48),
    (chunk(b"NAM3"), 0x58),
    (chunk(b"NAM4"), 0x68),
    (chunk(b"NAM5"), 0x78),
    (chunk(b"NAM6"), 0x88),
    (chunk(b"NAM7"), 0x98),
];
/// The `float` chunks and their fields, in the exe's three groups.
const FLOATS: [(u32, u32); 16] = [
    (chunk(b"NAM8"), 0x50),
    (chunk(b"NAM9"), 0x60),
    (chunk(b"NAM0"), 0x70),
    (chunk(b"ANAM"), 0x80),
    (chunk(b"BNAM"), 0x90),
    (chunk(b"CNAM"), 0xa0),
    (chunk(b"JNAM"), 0x54),
    (chunk(b"KNAM"), 0x64),
    (chunk(b"LNAM"), 0x74),
    (chunk(b"MNAM"), 0x84),
    (chunk(b"NNAM"), 0x94),
    (chunk(b"ONAM"), 0xa4),
    (chunk(b"DNAM"), 0xac),
    (chunk(b"ENAM"), 0xb0),
    (chunk(b"FNAM"), 0xb4),
    (chunk(b"GNAM"), 0xb8),
];

/// The sound form id at `offset` resolved (adjusted to the set's file,
/// looked up, cast to `TESSound`), with a warning when nothing is found.
fn resolve_sound(e: &mut Engine, this: Ptr, offset: u32) {
    let base = this.addr();
    let stored = e.mem.u32(base + offset);
    if stored == 0 {
        return;
    }
    let id = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stored);
        let file = e.call(FORM_GET_FILE, &args![this, 0xffff_ffffu32]).u32();
        e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        e.mem.u32(slot.addr())
    });
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let sound = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_SOUND, 0u32],
        )
        .u32();
    e.mem.set_u32(base + offset, sound);
    if e.mem.u32(base + offset) == 0 {
        let form_id = e.call(0x0084_e3a0, &args![this]).u32();
        let name = e.vcall(base, 0x130, &args![]).u32();
        e.call(WARNING, &args![MISSING_SOUND_FORMAT, id, name, form_id]);
    }
}

// Translated from 00598a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MediaSet::InitItem` (Xbox PDB): unless `004013e0` answers true,
/// resolves the two sound form ids at +0xBC and +0xC0 (a warning with the
/// id, the name and the form id for each one not found), then
/// `00484ab0(1)`.
pub fn media_set_init_item(e: &mut Engine, this: Ptr) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    resolve_sound(e, this, 0xbc);
    resolve_sound(e, this, 0xc0);
    e.call(0x0048_4ab0, &args![this, 1u32]);
}

// Translated from 00598bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00598bc0 - C++ exception unwinding (FS:[0] frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `MediaSet::Load` (Xbox PDB): `TESForm::LoadForm`, `00484ab0(0)` and each
/// chunk: `EDID`, `OBND`, `FULL` (+0x18), `NAM1` (the kind at +0x44, through
/// a local that keeps the last word read), `NAM2`..`NAM7` (a non-empty
/// name becomes `"Data\Music\<name>"` in the track's string,
/// `BSStringT::Set`), the `float` chunks (`NAM8`, `NAM9`, `NAM0`,
/// `ANAM`..`CNAM`, `JNAM`..`ONAM`, `DNAM`..`GNAM`, each read into a 0.0
/// local and copied with FLD/FSTP), `PNAM` (one byte at +0xA8) and
/// `HNAM`/`INAM` (the sound form ids at +0xBC/+0xC0). Returns true.
pub fn media_set_load(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    let kind = e.mem.alloc(4);
    e.mem.set_u32(kind, 0);
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk_type = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk_type == 0 {
            break;
        }
        if let Some(&(_, track)) = TRACK_NAMES.iter().find(|(c, _)| *c == chunk_type) {
            load_track_name(e, file, base + track);
        } else if let Some(&(_, field)) = FLOATS.iter().find(|(c, _)| *c == chunk_type) {
            let value = e.with_stack(4, |e, slot| {
                e.mem.set_f32(slot.addr(), 0.0);
                e.call(FILE_CHUNK_DATA, &args![file, slot]);
                e.mem.u32(slot.addr())
            });
            e.mem.set_u32(base + field, x87_quiet_bits(value));
        } else if chunk_type == chunk(b"NAM1") {
            e.call(FILE_CHUNK_DATA, &args![file, kind]);
            e.mem.set_u32(base + 0x44, e.mem.u32(kind));
        } else if chunk_type == chunk(b"HNAM") || chunk_type == chunk(b"INAM") {
            let id = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0);
                e.call(FILE_CHUNK_DATA, &args![file, slot]);
                e.mem.u32(slot.addr())
            });
            let field = if chunk_type == chunk(b"HNAM") {
                0xbc
            } else {
                0xc0
            };
            e.mem.set_u32(base + field, id);
        } else if chunk_type == chunk(b"PNAM") {
            e.call(0x0048_61f0, &args![this, file, base + 0xa8, 1u32]);
        } else if chunk_type == chunk(b"EDID") {
            let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
            // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
            e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
                e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0x200u32]);
                e.vcall(base, 0x134, &args![text]);
            });
        } else if chunk_type == chunk(b"OBND") {
            e.vcall(base, 0xe0, &args![file]);
        } else if chunk_type == chunk(b"FULL") {
            let full = if this.is_null() { 0 } else { base + 0x18 };
            e.call(0x0048_7050, &args![full, file]);
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    e.mem.free(kind);
    true
}

/// A track name chunk: when not empty, `"Data\Music\"` (`0040c0e0`) plus
/// the chunk's text (`BSStringT::operator+=`, `00404820`) in a local string
/// that is set into the track (`BSStringT::Set`, `00438470`) and destroyed
/// (`004037d0`).
fn load_track_name(e: &mut Engine, file: Ptr, track: u32) {
    if e.call(FILE_CHUNK_SIZE, &args![file]).u32() == 0 {
        return;
    }
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
    // `__alloca_probe_16` (`00ec5ec0`): the buffer on the stack.
    e.with_stack((size.wrapping_add(15) & !15).max(16), |e, text| {
        e.call(FILE_CHUNK_DATA_SIZED, &args![file, text, 0u32]);
        e.with_stack(8, |e, path| {
            e.call(0x0040_c0e0, &args![path, MUSIC_FOLDER]);
            e.call(0x0040_4820, &args![path, text]);
            e.call(0x0043_8470, &args![track, path]);
            e.call(0x0040_37d0, &args![path]);
        });
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_8a70, media_set_init_item(Ptr)),
        entry!(0x0059_8bc0, media_set_load(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn set(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xc4);
        let vtable = e.mem.alloc(0x140);
        for slot in [0x130u32, 0x134, 0xe0] {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        e.mem.set_u32(this, vtable);
        e.register(0x7000_0130, |_, _| ret(0x5555));
        quiet(e, &[0x7000_0134, 0x7000_00e0]);
        this
    }

    #[test]
    fn init_item_resolves_both_sounds() {
        let mut e = Engine::new();
        let this = set(&mut e);
        e.mem.set_u32(this + 0xbc, 0x11);
        e.mem.set_u32(this + 0xc0, 0x22);
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(FORM_GET_FILE, |_, _| ret(0x4400));
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        // Only the first is found.
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] & 0xff == 0x11 { 0x9000 } else { 0 })
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0084_e3a0, |_, _| ret(0x0012_3456));
        quiet(&mut e, &[WARNING, 0x0048_4ab0]);
        e.call_log = Some(vec![]);
        e.call(0x0059_8a70, &args![this]);
        assert_eq!(e.mem.u32(this + 0xbc), 0x9000);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![0x9000, 0, TYPE_TES_FORM, TYPE_TES_SOUND, 0]
        );
        assert_eq!(
            calls(&e, WARNING),
            vec![vec![MISSING_SOUND_FORMAT, 0x0100_0022, 0x5555, 0x0012_3456]]
        );
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 1]]);

        // No ids: no lookups; skipped: nothing.
        let mut e = Engine::new();
        let this = set(&mut e);
        e.register(0x0040_13e0, |_, _| ret(0));
        quiet(&mut e, &[0x0048_4ab0]);
        e.call_log = Some(vec![]);
        e.call(0x0059_8a70, &args![this]);
        assert!(calls(&e, LOOKUP_FORM).is_empty());
        assert_eq!(calls(&e, 0x0048_4ab0).len(), 1);
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0059_8a70, &args![this]);
        assert!(calls(&e, 0x0048_4ab0).is_empty());
    }

    #[test]
    fn load_reads_every_chunk_group() {
        let mut e = Engine::new();
        let this = set(&mut e);
        let mut chunks: Vec<(u32, u32, u32)> = vec![
            (chunk(b"EDID"), 8, 0),
            (chunk(b"OBND"), 12, 0),
            (chunk(b"FULL"), 8, 0),
            (chunk(b"NAM1"), 4, 2),
            (chunk(b"NAM2"), 8, 0x6f6c),
            (chunk(b"NAM7"), 0, 0),
            (chunk(b"PNAM"), 1, 0),
            (chunk(b"HNAM"), 4, 0x11),
            (chunk(b"INAM"), 4, 0x22),
            (chunk(b"ZNAM"), 4, 0x33),
        ];
        for (i, (name, _)) in FLOATS.iter().enumerate() {
            let bits = if i == 0 {
                0x7f80_0001
            } else {
                (i as f32 + 1.0).to_bits()
            };
            chunks.push((*name, 4, bits));
        }
        let file = e.mem.alloc(0x10);
        let chunks = Rc::new(RefCell::new((chunks, 0usize)));
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            let c = c.borrow();
            ret(c.0.get(c.1).map_or(0, |x| x.0))
        });
        let c = chunks.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            let mut c = c.borrow_mut();
            c.1 += 1;
            ret(u32::from(c.1 < c.0.len()))
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            let c = c.borrow();
            ret(c.0[c.1].1)
        });
        let c = chunks.clone();
        e.register_double(FILE_CHUNK_DATA, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].2);
            Ret::default()
        });
        let c = chunks;
        e.register_double(FILE_CHUNK_DATA_SIZED, move |e, a| {
            let c = c.borrow();
            e.mem.set_u32(a[1], c.0[c.1].2);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_61f0,
                0x0040_c0e0,
                0x0040_4820,
                0x0043_8470,
                0x0040_37d0,
            ],
        );
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_8bc0, &args![this, file]).bool());
        assert_eq!(calls(&e, 0x0048_5110), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_4ab0), vec![vec![this, 0]]);
        assert_eq!(calls(&e, 0x7000_0134).len(), 1);
        assert_eq!(calls(&e, 0x7000_00e0), vec![vec![this, file]]);
        assert_eq!(calls(&e, 0x0048_7050), vec![vec![this + 0x18, file]]);
        assert_eq!(e.mem.u32(this + 0x44), 2);
        // Only the non-empty name is set.
        let path = calls(&e, 0x0040_c0e0);
        assert_eq!(path.len(), 1);
        assert_eq!(path[0][1], MUSIC_FOLDER);
        assert_eq!(calls(&e, 0x0040_4820)[0][0], path[0][0]);
        assert_eq!(calls(&e, 0x0043_8470), vec![vec![this + 0x48, path[0][0]]]);
        assert_eq!(calls(&e, 0x0040_37d0), vec![vec![path[0][0]]]);
        assert_eq!(
            calls(&e, 0x0048_61f0),
            vec![vec![this, file, this + 0xa8, 1]]
        );
        assert_eq!(e.mem.u32(this + 0xbc), 0x11);
        assert_eq!(e.mem.u32(this + 0xc0), 0x22);
        // The signalling NaN comes back quiet through FLD/FSTP.
        assert_eq!(e.mem.u32(this + 0x50), 0x7fc0_0001);
        for (i, (_, field)) in FLOATS.iter().enumerate().skip(1) {
            assert_eq!(e.mem.f32(this + field), i as f32 + 1.0);
        }
    }
}
