//! `fallout shared/tessound.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e2b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e2b70 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// `TESSound::TESSound` (Xbox PDB): the form constructor `CreateFormOfType` (`00465110`) runs for form type 0xd (0x6c bytes).
/// `ECX` is the object; it runs the base and component constructors, stores the vtable pointers of the object and its components, and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn tessound_tessound(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0059_12d0, &args![base + 0x30]);
    e.mem.set_u32(base, 0x0104_4ffc);
    e.mem.set_u32(base + 0x30, 0x0104_4fe4);
    e.call(0x0040_37b0, &args![base + 0x3c]);
    e.call(0x004f_15a0, &args![this, 0xd]);
    e.mem.set_u8(base + 0x68, 0x0);
    e.call(0x005e_2ef0, &args![this]);
    this
}

/// Record tags (`u32::from_le_bytes` of the four characters) the loader `005e3160` tells apart.
const FNAM: u32 = u32::from_le_bytes(*b"FNAM");
const OBND: u32 = u32::from_le_bytes(*b"OBND");
const SNDD: u32 = u32::from_le_bytes(*b"SNDD");
const EDID: u32 = u32::from_le_bytes(*b"EDID");
const ANAM: u32 = u32::from_le_bytes(*b"ANAM");
const RNAM: u32 = u32::from_le_bytes(*b"RNAM");
const GNAM: u32 = u32::from_le_bytes(*b"GNAM");
const HNAM: u32 = u32::from_le_bytes(*b"HNAM");
const SNDX: u32 = u32::from_le_bytes(*b"SNDX");

// Translated from 005e2ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `005e2b70` (`TESSound`, form type 0xd) builds: the sound data bytes
/// at `+0x44`..`+0x47` are cleared, the word at `+0x48` and the words at `+0x4c` and `+0x4e` are
/// cleared, the words at `+0x50`..`+0x5a` are set to 100, 0x32, 0x14, 5, 0 and 0x50, the word
/// at `+0x5c` is set to 0x80 and the words at `+0x60` and `+0x64` are cleared.
pub fn fn_005e2ef0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u8(base + 0x45, 0);
    e.mem.set_u8(base + 0x44, 0);
    e.mem.set_u8(base + 0x46, 0);
    e.mem.set_u16(base + 0x4c, 0);
    e.mem.set_u8(base + 0x47, 0);
    e.mem.set_u16(base + 0x4e, 0);
    e.mem.set_u16(base + 0x50, 100);
    e.mem.set_u16(base + 0x52, 0x32);
    e.mem.set_u16(base + 0x54, 0x14);
    e.mem.set_u16(base + 0x56, 5);
    e.mem.set_u16(base + 0x58, 0);
    e.mem.set_u16(base + 0x5a, 0x50);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0x80);
}

// Translated from 005e3160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e3160 - the security-cookie check (00ec408c) at the end is not translated.
/// Record loader of `TESSound` (form type 0xd; the engine map gives the function no name).
/// `this` is the form and `file` the `TESFile` positioned on its record. Answers false without
/// touching the form when the file's record is not of type 0xd, otherwise true.
///
/// It runs `TESForm::LoadForm` (`00485110`) and walks the record's chunks:
///
/// - `FNAM`: reads the chunk into a buffer and runs virtual slot `+0x10` of the component at
///   `+0x30` on it;
/// - `OBND`: virtual slot `+0xe0` with the file;
/// - `EDID`: reads the chunk (limit 0x200) and runs virtual slot `+0x134` on it;
/// - `SNDD`: `TESForm::LoadData` (`004861f0`) of 0x24 bytes into `+0x44`, then `005e3020` on it
///   when `00401680` answers true;
/// - `SNDX`: loads 0xc bytes into a local, runs `005e3510` on it when `00401680` answers true,
///   and stores its bytes at `+0x44`, `+0x45`, `+0x46`, its word at local offset 4 at `+0x48`
///   and its words at local offsets 8 and 10 at `+0x4c` and `+0x4e`;
/// - `ANAM`: loads 10 bytes into `+0x50`; when `00401680` answers true, runs `00407a90` with 0 on
///   each of the five words; when the words are then 100, 0x4b, 0x32, 0x19 and 0 they are
///   replaced by 100, 0x32, 0x14, 5 and 0;
/// - `RNAM`: loads 1 byte into `+0x68`; `GNAM`: 2 bytes into `+0x5a`; `HNAM`: 4 bytes into `+0x5c`.
///
/// The buffers the game takes from the stack (`__alloca_probe_16`) are heap blocks here.
pub fn fn_005e3160(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    let base = this.addr();
    if e.call(0x0047_2660, &args![file]).u32() != 0xd {
        return false;
    }
    e.call(0x0048_5110, &args![this, file]);
    loop {
        let chunk = e.call(0x0047_26b0, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            FNAM => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0u32]);
                e.vcall(base + 0x30, 0x10, &args![buffer]);
                e.mem.free(buffer);
            }
            OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            SNDD => {
                e.call(0x0048_61f0, &args![this, file, base + 0x44, 0x24u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005e_3020, &args![base + 0x44]);
                }
            }
            EDID => {
                let size = e.call(0x0040_1660, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                e.call(0x0047_2890, &args![file, buffer, 0x200u32]);
                e.vcall(base, 0x134, &args![buffer]);
                e.mem.free(buffer);
            }
            ANAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x50, 10u32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    for offset in [0x50, 0x52, 0x54, 0x56, 0x58] {
                        e.call(0x0040_7a90, &args![base + offset, 0u32]);
                    }
                }
                let words = [0x50, 0x52, 0x54, 0x56, 0x58].map(|offset| e.mem.u16(base + offset));
                if words == [100, 0x4b, 0x32, 0x19, 0] {
                    for (offset, value) in [
                        (0x50, 100),
                        (0x52, 0x32),
                        (0x54, 0x14),
                        (0x56, 5),
                        (0x58, 0),
                    ] {
                        e.mem.set_u16(base + offset, value);
                    }
                }
            }
            RNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x68, 1u32]);
            }
            GNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x5a, 2u32]);
            }
            HNAM => {
                e.call(0x0048_61f0, &args![this, file, base + 0x5c, 4u32]);
            }
            SNDX => {
                let data = e.mem.alloc(12);
                e.call(0x0048_61f0, &args![this, file, data, 0xcu32]);
                if e.call(0x0040_1680, &args![file]).bool() {
                    e.call(0x005e_3510, &args![data]);
                }
                let first = e.mem.u8(data);
                let second = e.mem.u8(data + 1);
                let third = e.mem.u8(data + 2);
                let long = e.mem.u32(data + 4);
                let low_word = e.mem.u16(data + 8);
                let high_word = e.mem.u16(data + 10);
                e.mem.set_u8(base + 0x44, first);
                e.mem.set_u8(base + 0x45, second);
                e.mem.set_u8(base + 0x46, third);
                e.mem.set_u32(base + 0x48, long);
                e.mem.set_u16(base + 0x4c, low_word);
                e.mem.set_u16(base + 0x4e, high_word);
                e.mem.free(data);
            }
            _ => {}
        }
        if !e.call(0x0047_26f0, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 005e2c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005e2ca0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005e2c70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005e_2ca0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_2b70, tessound_tessound(Ptr) -> Ptr),
        entry!(0x005e_2ef0, fn_005e2ef0(Ptr)),
        entry!(0x005e_3160, fn_005e3160(Ptr, Ptr) -> bool),
        entry!(0x005e_2c70, fn_005e2c70(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn tessound_tessound_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_2b70,
            size: 0x6c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0059_12d0, &[A::T(0x30)]),
                (0x0040_37b0, &[A::T(0x3c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xd)]),
                (0x005e_2ef0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_4ffc),
                (0x30, 4, 0x0104_4fe4),
                (0x68, 1, 0x0),
            ],
        });
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;
    use crate::units::fallout_shared::tesmodel::load_test::{self, calls_to, run, stub, tag, FILE};

    type Loaded = (Engine, u32, bool, load_test::Calls);

    /// Runs the loader on a sound form and a file holding `chunks`. `TESForm::LoadData`
    /// (`004861f0`) copies `data` into the buffer it is given; `version_flag` is what `00401680`
    /// answers. The form's virtual slots `+0xe0`, `+0x134` and (component at `+0x30`) `+0x10`
    /// are the doubles `0x0090_0500`, `0x0090_0600` and `0x0090_0700`.
    fn load(form_type: u32, chunks: &[u32], version_flag: bool, data: &[u8]) -> Loaded {
        let mut e = load_test::engine(form_type, chunks, version_flag);
        stub(
            &mut e,
            &[0x0048_5110, 0x005e_3020, 0x005e_3510, 0x0040_7a90],
        );
        let data = data.to_vec();
        e.register_double(0x0048_61f0, move |e, a| {
            let count = data.len().min(a[3] as usize);
            e.mem.write(a[2], &data[..count]);
            Ret::default()
        });
        let this = load_test::object(
            &mut e,
            0x70,
            0x0900_0000,
            &[(0xe0, 0x0090_0500), (0x134, 0x0090_0600)],
        );
        e.map(0x0900_1000, 0x200);
        e.mem.set_u32(this + 0x30, 0x0900_1000);
        e.mem.set_u32(0x0900_1010, 0x0090_0700);
        stub(&mut e, &[0x0090_0700]);
        let (ret, calls) = run(&mut e, 0x005e_3160, &[this, FILE]);
        (e, this, ret.bool(), calls)
    }

    fn sound(chunks: &[u32], version_flag: bool, data: &[u8]) -> Loaded {
        load(0xd, chunks, version_flag, data)
    }

    #[test]
    fn fn_005e2ef0_sets_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.mem.write(this, &[0xcd; 0x70]);
        e.call(0x005e_2ef0, &args![Ptr::<()>::new(this)]);
        let mut want = vec![0xcd_u8; 0x70];
        let mut put =
            |offset: usize, bytes: &[u8]| want[offset..offset + bytes.len()].copy_from_slice(bytes);
        put(0x44, &[0, 0, 0, 0]);
        put(0x48, &0u32.to_le_bytes());
        put(0x4c, &[0; 4]);
        for (offset, value) in [
            (0x50, 100u16),
            (0x52, 0x32),
            (0x54, 0x14),
            (0x56, 5),
            (0x58, 0),
            (0x5a, 0x50),
        ] {
            put(offset, &value.to_le_bytes());
        }
        put(0x5c, &0x80u32.to_le_bytes());
        put(0x60, &[0; 8]);
        assert_eq!(e.mem.bytes(this, 0x70), want);
    }

    #[test]
    fn another_form_type_is_refused_untouched() {
        let (e, this, answer, calls) = load(0xc, &[tag(b"EDID")], false, &[]);
        assert!(!answer);
        assert!(calls.is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn the_buffer_chunks_reach_their_virtual_slots() {
        let (_, this, answer, calls) = sound(&[tag(b"FNAM")], false, &[]);
        assert!(answer);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0));
        assert_eq!(
            calls_to(&calls, 0x0090_0700),
            [vec![this + 0x30, read[0][1]]]
        );

        let (_, this, _, calls) = sound(&[tag(b"EDID")], false, &[]);
        let read = calls_to(&calls, 0x0047_2890);
        assert_eq!((read[0][0], read[0][2]), (FILE, 0x200));
        assert_eq!(calls_to(&calls, 0x0090_0600), [vec![this, read[0][1]]]);

        let (_, this, _, calls) = sound(&[tag(b"OBND")], false, &[]);
        assert_eq!(calls_to(&calls, 0x0090_0500), [vec![this, FILE]]);
    }

    #[test]
    fn the_small_data_chunks_load_into_their_fields() {
        for (chunk, offset, size) in [
            (tag(b"RNAM"), 0x68, 1),
            (tag(b"GNAM"), 0x5a, 2),
            (tag(b"HNAM"), 0x5c, 4),
        ] {
            let (_, this, _, calls) = sound(&[chunk], false, &[]);
            assert_eq!(
                calls_to(&calls, 0x0048_61f0),
                [vec![this, FILE, this + offset, size]]
            );
        }
    }

    #[test]
    fn the_sound_data_chunk_runs_its_fix_up_only_for_the_flag() {
        let (_, this, _, calls) = sound(&[tag(b"SNDD")], false, &[]);
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x44, 0x24]]
        );
        assert!(calls_to(&calls, 0x005e_3020).is_empty());

        let (_, this, _, calls) = sound(&[tag(b"SNDD")], true, &[]);
        assert_eq!(calls_to(&calls, 0x005e_3020), [vec![this + 0x44]]);
    }

    #[test]
    fn the_extended_sound_data_is_split_over_the_form() {
        let data = [
            1, 2, 3, 0xee, 0x44, 0x33, 0x22, 0x11, 0x66, 0x55, 0x88, 0x77,
        ];
        let (e, this, _, calls) = sound(&[tag(b"SNDX")], false, &data);
        assert_eq!(e.mem.u8(this + 0x44), 1);
        assert_eq!(e.mem.u8(this + 0x45), 2);
        assert_eq!(e.mem.u8(this + 0x46), 3);
        assert_eq!(e.mem.u8(this + 0x47), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x1122_3344);
        assert_eq!(e.mem.u16(this + 0x4c), 0x5566);
        assert_eq!(e.mem.u16(this + 0x4e), 0x7788);
        assert!(calls_to(&calls, 0x005e_3510).is_empty());
        let load = calls_to(&calls, 0x0048_61f0);
        assert_eq!((load[0][0], load[0][1], load[0][3]), (this, FILE, 0xc));

        let (_, _, _, calls) = sound(&[tag(b"SNDX")], true, &data);
        let load = calls_to(&calls, 0x0048_61f0);
        assert_eq!(calls_to(&calls, 0x005e_3510), [vec![load[0][2]]]);
    }

    fn attenuation(words: [u16; 5]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn the_old_default_attenuation_is_replaced() {
        let old = attenuation([100, 0x4b, 0x32, 0x19, 0]);
        let (e, this, _, calls) = sound(&[tag(b"ANAM")], false, &old);
        let words: Vec<u16> = (0..5).map(|i| e.mem.u16(this + 0x50 + 2 * i)).collect();
        assert_eq!(words, [100, 0x32, 0x14, 5, 0]);
        assert!(calls_to(&calls, 0x0040_7a90).is_empty());
        assert_eq!(
            calls_to(&calls, 0x0048_61f0),
            [vec![this, FILE, this + 0x50, 10]]
        );
    }

    #[test]
    fn other_attenuation_values_are_kept_and_the_flag_runs_the_clearing_calls() {
        let other = attenuation([100, 0x4c, 0x32, 0x19, 0]);
        let (e, this, _, _) = sound(&[tag(b"ANAM")], false, &other);
        let words: Vec<u16> = (0..5).map(|i| e.mem.u16(this + 0x50 + 2 * i)).collect();
        assert_eq!(words, [100, 0x4c, 0x32, 0x19, 0]);

        let old = attenuation([100, 0x4b, 0x32, 0x19, 0]);
        let (e, this, _, calls) = sound(&[tag(b"ANAM")], true, &old);
        assert_eq!(
            calls_to(&calls, 0x0040_7a90),
            [
                vec![this + 0x50, 0],
                vec![this + 0x52, 0],
                vec![this + 0x54, 0],
                vec![this + 0x56, 0],
                vec![this + 0x58, 0]
            ]
        );
        // The clearing calls are doubles; the old defaults are then still replaced.
        assert_eq!(e.mem.u16(this + 0x52), 0x32);
    }
    #[test]
    fn at_005e2c70_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005e_2ca0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005e_2c70, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005e_2c70)
                .collect();
            let mut expected = vec![(0x005e_2ca0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}
