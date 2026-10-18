//! `fallout shared/run_0051af50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0051b140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b140 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x20 (0x58 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0051b140(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0102_ba2c);
    e.mem.set_u32(base + 0x30, 0x0102_ba04);
    e.call(0x004f_15a0, &args![this, 0x20]);
    e.mem.set_u8(base + 0x50, 0xff);
    e.mem.set_u32(base + 0x54, 0x0);
    this
}

// Translated from 0051b130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `0051a810`. `0051a810` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0051b130(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_a810, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `0051b280` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");
const MODS: u32 = u32::from_le_bytes(*b"MODS");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const BRUS: u32 = u32::from_le_bytes(*b"BRUS");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");

// Translated from 0051b280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0051b280 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of the form `0051b140` constructs (form type 0x20; the engine map gives the
/// function no name, so the class is unconfirmed). `this` is the form and `file` the `TESFile`
/// positioned on its record. Always answers true.
///
/// It asks the file for the record's form type (`00472660`, the answer is not used), runs
/// `TESForm::LoadForm` (`00485110`) and then walks the record's chunks (`TESFile::GetTESChunk`
/// `004726b0` / `NextChunk` `004726f0`):
///
/// - `MODL`, `MODD`, `MOSD`, `MODT`: `TESModel::LoadModelChunk` (`004892d0`) on the model at
///   `+0x30` (null for a null form);
/// - `MODS`: reads the chunk and unpacks its texture swaps into the model at `+0x30`
///   (`0048a7b0` with the buffer, its size and the file's `AX` word at `00403570`);
/// - `OBND`: virtual slot `+0xe0` with the file;
/// - `EDID`: reads the chunk (limit 0x200) into a buffer and runs virtual slot `+0x134` on it;
/// - `BRUS`: reads a word with `TESFile::GetChunkData` and stores it as a byte at `+0x50`,
///   0xff when it is above 10;
/// - `RNAM`: reads a word into `+0x54`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_0051b280(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    e.call(0x0047_2660, &args![file]);
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            MODL | MODD | MOSD | MODT => {
                let model = if base != 0 { base + 0x30 } else { 0 };
                e.call(0x0048_92d0, &args![model, file]);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            MODS => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, size]);
                let swap_flags = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                e.call(0x0048_a7b0, &args![base + 0x30, buffer, size, swap_flags]);
                e.mem.free(buffer);
            }
            BRUS => {
                let value = e.mem.alloc(4);
                e.call(0x0047_27f0, &args![file, value]);
                let value_read = e.mem.u32(value);
                let stored = if value_read > 10 { 0xff } else { value_read };
                e.mem.set_u8(base + 0x50, stored as u8);
                e.mem.free(value);
            }
            RNAM => {
                let value = e.mem.alloc(4);
                e.call(0x0047_27f0, &args![file, value]);
                let value_read = e.mem.u32(value);
                e.mem.set_u32(base + 0x54, value_read);
                e.mem.free(value);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0051b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0051b200` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0051b1d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_b200, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_b140, fn_0051b140(Ptr) -> Ptr),
        entry!(0x0051_b130, fn_0051b130(Ptr, u32) -> u32),
        entry!(0x0051_b280, fn_0051b280(Ptr, Ptr) -> bool),
        entry!(0x0051_b1d0, fn_0051b1d0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0051b140_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_b140,
            size: 0x58,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x30)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x20)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_ba2c),
                (0x30, 4, 0x0102_ba04),
                (0x50, 1, 0xff),
                (0x54, 4, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0051b130() {
        assert_eq!(
            check(0x0051_b130, 0x0051_a810, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{
        self, calls_to, run, stub, tag, CHUNK_SIZE, CHUNK_VALUE, FILE,
    };

    /// Runs the loader on a form and a file holding `chunks`; returns the form, the answer and
    /// the calls. The form's virtual slots `+0xe0` and `+0x134` are the doubles `0x0090_0500`
    /// and `0x0090_0600`.
    fn load(chunks: &[u32]) -> (Engine, u32, bool, load_test::Calls) {
        let mut e = load_test::engine(0x20, chunks, false);
        stub(&mut e, &[0x0048_5110, 0x0048_92d0, 0x0048_a7b0]);
        let this = load_test::object(
            &mut e,
            0x60,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        let (ret, calls) = run(&mut e, 0x0051_b280, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    #[test]
    fn an_empty_record_loads_the_form_and_answers_true() {
        let (_, this, answer, calls) = load(&[]);
        assert!(answer);
        assert_eq!(calls, [(0x0048_5110, vec![this, FILE])]);
    }

    #[test]
    fn the_model_chunks_go_to_the_model_loader() {
        for chunk in [tag(b"MODL"), tag(b"MODD"), tag(b"MOSD"), tag(b"MODT")] {
            let (_, this, _, calls) = load(&[chunk]);
            assert_eq!(
                calls_to(&calls, 0x0048_92d0),
                [vec![this + 0x30, FILE]],
                "{chunk:08x}"
            );
            assert_eq!(calls.len(), 2);
        }
    }

    #[test]
    fn the_bound_chunk_runs_virtual_slot_0xe0() {
        let (_, this, _, calls) = load(&[tag(b"OBND")]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);
    }

    #[test]
    fn the_editor_id_is_read_into_a_buffer_and_given_to_slot_0x134() {
        let (e, this, _, calls) = load(&[tag(b"EDID")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!(read.len(), 1);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);
        assert_eq!(e.mem.block_size(read[0][1]), None);
    }

    #[test]
    fn the_texture_swap_chunk_is_unpacked_with_the_files_flag_word() {
        let (_, this, _, calls) = load(&[tag(b"MODS")]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, CHUNK_SIZE));
        // The unpacker gets the buffer, its size and the low word of the file's answer.
        assert_eq!(
            calls_to(&calls, 0x0048_a7b0),
            [vec![this + 0x30, read[0][1], CHUNK_SIZE, 0x1234]]
        );
    }

    #[test]
    fn the_brus_byte_is_capped_at_0xff_and_the_rnam_word_is_kept() {
        let (e, this, _, _) = load(&[tag(b"BRUS"), tag(b"RNAM")]);
        // The double writes 0x12345678: above 10, so 0xff.
        assert_eq!(e.mem.u8(this + 0x50), 0xff);
        assert_eq!(e.mem.u32(this + 0x54), CHUNK_VALUE);

        for (value, stored) in [(0u32, 0u8), (10, 10), (11, 0xff)] {
            let mut e = load_test::engine(0x20, &[tag(b"BRUS")], false);
            stub(&mut e, &[0x0048_5110]);
            e.register_double(0x0047_27f0, move |e, a| {
                e.mem.set_u32(a[1], value);
                Ret::default()
            });
            let this = e.mem.alloc(0x60);
            run(&mut e, 0x0051_b280, &[this, FILE]);
            assert_eq!(e.mem.u8(this + 0x50), stored, "{value}");
        }
    }

    #[test]
    fn unknown_chunks_are_skipped_and_the_walk_goes_on() {
        let (e, this, answer, calls) = load(&[tag(b"XXXX"), tag(b"RNAM")]);
        assert!(answer);
        assert_eq!(calls_to(&calls, 0x0047_27f0).len(), 1);
        assert_eq!(e.mem.u32(this + 0x54), CHUNK_VALUE);
    }
    #[test]
    fn at_0051b1d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_b200, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_b1d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_b1d0)
                .collect();
            let mut expected = vec![(0x0051_b200, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
