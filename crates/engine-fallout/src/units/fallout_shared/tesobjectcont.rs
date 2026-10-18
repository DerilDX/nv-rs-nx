//! `fallout shared/tesobjectcont.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005160f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005160f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESObjectCONT::TESObjectCONT` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0x1b (0x9c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tesobjectcont_tesobjectcont(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0048_1610, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x3c]);
    e.call(0x0048_a3d0, &args![base + 0x48]);
    e.call(0x0048_cc50, &args![base + 0x68]);
    e.call(0x0048_eac0, &args![base + 0x74]);
    e.call(0x0047_5350, &args![base + 0x7c]);
    e.call(0x0051_0620, &args![base + 0x84]);
    e.mem.set_u32(base, 0x0102_aeb4);
    e.mem.set_u32(base + 0x30, 0x0102_ae9c);
    e.mem.set_u32(base + 0x3c, 0x0102_ae88);
    e.mem.set_u32(base + 0x48, 0x0102_ae64);
    e.mem.set_u32(base + 0x68, 0x0102_ae50);
    e.mem.set_u32(base + 0x74, 0x0102_ae3c);
    e.mem.set_u32(base + 0x7c, 0x0102_ae28);
    e.mem.set_u32(base + 0x84, 0x0102_ae18);
    e.call(0x004f_15a0, &args![this, 0x1b]);
    e.call(0x0051_6330, &args![this]);
    this
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `00516580` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const FULL: u32 = u32::from_le_bytes(*b"FULL");
const SCRI: u32 = u32::from_le_bytes(*b"SCRI");
const CNTO: u32 = u32::from_le_bytes(*b"CNTO");
const COED: u32 = u32::from_le_bytes(*b"COED");
const DATA: u32 = u32::from_le_bytes(*b"DATA");
const DSTD: u32 = u32::from_le_bytes(*b"DSTD");
const DEST: u32 = u32::from_le_bytes(*b"DEST");
const SNAM: u32 = u32::from_le_bytes(*b"SNAM");
const QNAM: u32 = u32::from_le_bytes(*b"QNAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
/// The container item entry (`004817f0`'s answer) whose extra data a following `COED` chunk fills.
const PENDING_CONTAINER_ITEM: u32 = 0x011c_9348;

// Translated from 00516330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of `TESObjectCONT` (form type 0x1b, constructed by `005160f0`): clears the byte
/// at `+0x98` and the words at `+0x8c`, `+0x90` and `+0x94`.
pub fn fn_00516330(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 0x98, 0);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0);
}

// Translated from 00516580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00516580 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Record loader of `TESObjectCONT` (form type 0x1b; the engine map gives the function no name).
/// `this` is the form and `file` the `TESFile` positioned on its record. Answers false without
/// touching the form when the record is not of type 0x1b, otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`), `00484ab0` with 0, and walks the chunks:
///
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it; `OBND`: slot
///   `+0xe0` with the file;
/// - `FULL`: `00487050` on `+0x3c`; `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk`
///   (`004892d0`) on `+0x48`; `MODS`: the texture swaps into `+0x48` (`0048a7b0`);
/// - `SCRI`: reads a word, `006ecd40` on `+0x68` with it and `0048cd90` on `+0x68` with the form;
/// - `CNTO` and `COED` (the file's chunk is asked again): for `CNTO` it reads 8 bytes into a
///   local (zeroed with `memset`), runs `00462230` on it when `00401680` answers true and keeps
///   `004817f0(+0x30, local)` in the global at `0x011c9348`; for `COED` with that global set it
///   makes a 0xc byte object (`00401000`, `0040e690`), stores it at `+8` of the entry, loads it
///   (`0040e780` with the file) and clears the global;
/// - `DATA`: `TESForm::LoadData` (`004861f0`) of 1 byte into `+0x98`; `DSTD`, `DEST`:
///   `004781e0` on `+0x7c`;
/// - `SNAM`, `QNAM`, `RNAM`: reads a word and gives it to `008cc6d0`, `004fb0b0` and
///   `004febf0`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here. The
/// exception frame of the allocation of the 0xc byte object is not translated.
pub fn fn_00516580(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u8() != 0x1b {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    e.call(0x0048_4ab0, &args![this, 0u32]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            FULL => {
                let target = if base != 0 { base + 0x3c } else { 0 };
                e.call(0x0048_7050, &args![target, file]);
            }
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x48 } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x48, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            SCRI => {
                let value = read_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x68, value]);
                e.call(0x0048_cd90, &args![base + 0x68, this]);
            }
            CNTO | COED => {
                let current = e.call(0x0047_26b0, &args![file]).u32();
                if current == COED {
                    let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                    if entry != 0 {
                        let block = e.call(0x0040_1000, &args![0xcu32]).u32();
                        let extra = if block != 0 {
                            e.call(0x0040_e690, &args![block]).u32()
                        } else {
                            0
                        };
                        let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                        e.mem.set_u32(entry + 8, extra);
                        let entry = e.global::<u32>(PENDING_CONTAINER_ITEM);
                        let extra = e.mem.u32(entry + 8);
                        e.call(0x0040_e780, &args![extra, file]);
                        e.set_global(PENDING_CONTAINER_ITEM, 0u32);
                    }
                } else if current == CNTO {
                    let local = e.mem.alloc(8);
                    e.call(0x0040_3d30, &args![local, 0u32, 8u32]);
                    e.call(0x0047_2890, &args![file, local, 8u32]);
                    if e.call(0x0040_1680, &args![file]).bool() {
                        e.call(0x0046_2230, &args![local]);
                    }
                    let item = e.call(0x0048_17f0, &args![base + 0x30, local]).u32();
                    e.set_global(PENDING_CONTAINER_ITEM, item);
                    e.mem.free(local);
                }
            }
            DATA => {
                e.call(0x0048_61f0, &args![this, file, base + 0x98, 1u32]);
            }
            DSTD | DEST => {
                let target = if base != 0 { base + 0x7c } else { 0 };
                e.call(0x0047_81e0, &args![target, file]);
            }
            SNAM => {
                let value = read_word(e, file);
                e.call(0x008c_c6d0, &args![this, value]);
            }
            QNAM => {
                let value = read_word(e, file);
                e.call(0x004f_b0b0, &args![this, value]);
            }
            RNAM => {
                let value = read_word(e, file);
                e.call(0x004f_ebf0, &args![this, value]);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

/// A zeroed local read with `TESFile::GetChunkData` (`004727f0`).
fn read_word(e: &mut Engine, file: Ptr) -> u32 {
    let local = e.mem.alloc(4);
    e.call(0x0047_27f0, &args![file, local]);
    let value = e.mem.u32(local);
    e.mem.free(local);
    value
}

// Translated from 00516210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00516240` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00516210(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_6240, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_60f0, tesobjectcont_tesobjectcont(Ptr) -> Ptr),
        entry!(0x0051_6330, fn_00516330(Ptr)),
        entry!(0x0051_6580, fn_00516580(Ptr, Ptr) -> bool),
        entry!(0x0051_6210, fn_00516210(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tesobjectcont_tesobjectcont_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_60f0,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0048_1610, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x3c)]),
                (0x0048_a3d0, &[A::T(0x48)]),
                (0x0048_cc50, &[A::T(0x68)]),
                (0x0048_eac0, &[A::T(0x74)]),
                (0x0047_5350, &[A::T(0x7c)]),
                (0x0051_0620, &[A::T(0x84)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1b)]),
                (0x0051_6330, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_aeb4),
                (0x30, 4, 0x0102_ae9c),
                (0x3c, 4, 0x0102_ae88),
                (0x48, 4, 0x0102_ae64),
                (0x68, 4, 0x0102_ae50),
                (0x74, 4, 0x0102_ae3c),
                (0x7c, 4, 0x0102_ae28),
                (0x84, 4, 0x0102_ae18),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    const PENDING: u32 = 0x011c_9348;

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a container form and a file holding `chunks`. The form's virtual
    /// slots `+0xe0` and `+0x134` are the doubles `0x0090_0500` and `0x0090_0600`.
    fn load(form_type: u32, chunks: &[u32], version_flag: bool, pending: u32) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, version_flag);
        stub(
            &mut e,
            &[
                0x0048_5110,
                0x0048_4ab0,
                0x0048_7050,
                0x0048_92d0,
                0x0048_a7b0,
                0x006e_cd40,
                0x0048_cd90,
                0x0047_81e0,
                0x008c_c6d0,
                0x004f_b0b0,
                0x004f_ebf0,
                0x0048_61f0,
                0x0040_3d30,
                0x0046_2230,
                0x0040_e780,
            ],
        );
        e.register(0x0048_17f0, |_, _| Ret {
            eax: 0x0777_0001,
            ..Ret::default()
        });
        e.register(0x0040_1000, |_, _| Ret {
            eax: 0x0b10_c000,
            ..Ret::default()
        });
        e.register(0x0040_e690, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.map(PENDING, 4);
        e.set_global(PENDING, pending);
        let this = load_test::object(
            &mut e,
            0xa0,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_6580, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn container(chunks: &[u32]) -> Loaded {
        load(0x1b, chunks, false, 0)
    }

    #[test]
    fn fn_00516330_clears_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        e.mem.write(this, &[0xcd; 0xa0]);
        e.call(0x0051_6330, &args![Ptr::<()>::new(this)]);
        let mut want = vec![0xcd_u8; 0xa0];
        want[0x8c..0x98].fill(0);
        want[0x98] = 0;
        assert_eq!(e.mem.bytes(this, 0xa0), want);
    }

    #[test]
    fn another_form_type_is_refused_before_anything_is_loaded() {
        let (_, _, answer, calls) = load(0x1c, &[tag(b"EDID")], false, 0);
        assert!(!answer);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_form_is_loaded_first_and_the_flag_bit_cleared() {
        let (_, this, answer, calls) = container(&[]);
        assert!(answer);
        assert_eq!(
            calls,
            [
                (0x0048_5110, vec![this, FILE]),
                (0x0048_4ab0, vec![this, 0])
            ]
        );
    }

    #[test]
    fn the_buffer_and_virtual_chunks() {
        let (_, this, _, calls) = container(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = container(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);

        let (_, this, _, calls) = container(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x48, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_components_get_their_chunks() {
        for (chunk, callee, offset) in [
            (tag(b"FULL"), 0x0048_7050, 0x3c),
            (tag(b"MODL"), 0x0048_92d0, 0x48),
            (tag(b"MODD"), 0x0048_92d0, 0x48),
            (tag(b"MOSD"), 0x0048_92d0, 0x48),
            (tag(b"MODT"), 0x0048_92d0, 0x48),
            (tag(b"DSTD"), 0x0047_81e0, 0x7c),
            (tag(b"DEST"), 0x0047_81e0, 0x7c),
        ] {
            let (_, this, _, calls) = container(&[chunk]);
            assert_eq!(
                calls_to(&calls, callee),
                [vec![this + offset, FILE]],
                "{chunk:08x}"
            );
        }
    }

    #[test]
    fn the_word_chunks_reach_their_setters() {
        let (_, this, _, calls) = container(&[tag(b"SCRI")]);
        assert_eq!(
            calls_to(&calls, 0x006e_cd40),
            [vec![this + 0x68, CHUNK_VALUE]]
        );
        assert_eq!(calls_to(&calls, 0x0048_cd90), [vec![this + 0x68, this]]);
        for (chunk, callee) in [
            (tag(b"SNAM"), 0x008c_c6d0),
            (tag(b"QNAM"), 0x004f_b0b0),
            (tag(b"RNAM"), 0x004f_ebf0),
        ] {
            let (_, this, _, calls) = container(&[chunk]);
            assert_eq!(calls_to(&calls, callee), [vec![this, CHUNK_VALUE]]);
        }
        let (_, this, _, calls) = container(&[tag(b"DATA")]);
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x98, 1]]
        );
    }

    #[test]
    fn a_container_item_chunk_remembers_the_entry_and_its_extra_data_chunk_loads_it() {
        // CNTO: the 8 byte local is cleared, read, fixed up and handed to 004817f0.
        let (e, this, _, calls) = load(0x1b, &[tag(b"CNTO")], true, 0);
        let read = calls_to(&calls, 0x0047_2890);
        let local = read[0][1];
        assert_eq!((read[0][0], read[0][2]), (FILE, 8));
        assert_eq!(calls_to(&calls, 0x0040_3d30), [vec![local, 0, 8]]);
        assert_eq!(calls_to(&calls, 0x0046_2230), [vec![local]]);
        assert_eq!(calls_to(&calls, 0x0048_17f0), [vec![this + 0x30, local]]);
        assert_eq!(e.global::<u32>(PENDING), 0x0777_0001);

        // Without the flag the fix-up is skipped.
        let (_, _, _, calls) = load(0x1b, &[tag(b"CNTO")], false, 0);
        assert!(calls_to(&calls, 0x0046_2230).is_empty());

        // COED with an entry waiting: a new extra-data object is stored in it and loaded.
        let mut e = load_test::engine(0x1b, &[tag(b"COED")], false);
        stub(&mut e, &[0x0048_5110, 0x0048_4ab0, 0x0040_e780]);
        e.register(0x0040_1000, |_, _| Ret {
            eax: 0x0b10_c000,
            ..Ret::default()
        });
        e.register(0x0040_e690, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let entry = e.mem.alloc(0x10);
        e.map(PENDING, 4);
        e.set_global(PENDING, entry);
        let form = e.mem.alloc(0xa0);
        let (_, calls) = run(&mut e, 0x0051_6580, &[form, FILE]);
        assert_eq!(calls_to(&calls, 0x0040_1000), [vec![0xc]]);
        assert_eq!(calls_to(&calls, 0x0040_e690), [vec![0x0b10_c000]]);
        assert_eq!(e.mem.u32(entry + 8), 0x0b10_c001);
        assert_eq!(calls_to(&calls, 0x0040_e780), [vec![0x0b10_c001, FILE]]);
        assert_eq!(e.global::<u32>(PENDING), 0);

        // COED with nothing waiting does nothing.
        let (_, _, _, calls) = load(0x1b, &[tag(b"COED")], false, 0);
        assert!(calls_to(&calls, 0x0040_1000).is_empty());
        assert!(calls_to(&calls, 0x0040_e780).is_empty());
    }
    #[test]
    fn at_00516210_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_6240, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_6210, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_6210)
                .collect();
            let mut expected = vec![(0x0051_6240, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
