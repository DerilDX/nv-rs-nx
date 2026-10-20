//! `(unplaced)/run_0059b270` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059b880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b880 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x75 (0x2c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_4b4c`) is `TESCaravanDeck`, so this is the constructor of `TESCaravanDeck`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0059b880(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_4b4c);
    e.mem.set_u32(base + 0x18, 0x0103_4b38);
    e.call(0x004f_15a0, &args![this, 0x75u32]);
    e.call(0x0059_b9f0, &args![this]);
    this
}

// Translated from 0059b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0059b940` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0059b910(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0059_b940, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

use crate::units::fallout_shared::tescaravan::{
    cast_or_null, chunk_word, load_editor_id, CHUNK_DATA, CHUNK_EDID, CHUNK_FULL, CHUNK_ICON,
    CHUNK_MICO, CHUNK_MODD, CHUNK_MODL, CHUNK_MODS, CHUNK_MODT, CHUNK_MOSD, CHUNK_OBND, CHUNK_SIZE,
    CHUNK_YNAM, CHUNK_ZNAM, GET_CHUNK_DATA_SIZED, GET_TES_CHUNK, GET_TES_FORM, LOAD_FORM,
    NEXT_CHUNK, SET_FLAG_00484AB0,
};

/// Chunk types `MOD2` and `MOD3` (the four characters, little-endian).
const CHUNK_MOD2: u32 = 0x3244_4f4d;
const CHUNK_MOD3: u32 = 0x3344_4f4d;

/// The `MOD2`/`MOD3` chunks: the chunk into a stack buffer of its size (read with a size of
/// 0), the buffer given to vtable slot +0x18 of the model at `model`, then `004893e0`
/// (cdecl) on the model with the file.
fn load_extra_model(e: &mut Engine, file: Ptr, model: u32) {
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    e.with_stack(size, |e, buffer| {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0u32]);
        e.vcall(model, 0x18, &args![buffer]);
    });
    e.call(0x0048_93e0, &args![model, file]);
}

// Translated from 0059b290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b290 - the /GS cookie check (00ec408c) is not translated; the EDID, MODS, MOD2 and MOD3 buffers from __alloca_probe_16 (00ec5ec0) are with_stack blocks of the chunk size (their 16-byte alignment padding, and the stack growing with each such chunk, are not modelled)
/// The `TESCaravanMoney` `Load` (form type 0x74, the type its constructor `0059ae60`
/// gives; unnamed in the engine map; `ECX` and one stack word, the file, `RET 4`): false
/// unless `TESFile::GetTESForm`'s `AL` is 0x74. Otherwise `TESForm::LoadForm`, `00484ab0`
/// with 0, then each chunk:
/// - `EDID`: the editor ID (stack buffer, at most 0x200 bytes, vtable slot +0x134);
///   `OBND`: vtable slot +0xe0 with the file;
/// - `FULL`: `00487050` (cdecl) on +0x30; `MODL`, `MODT`, `MODD`, `MOSD`: `004892d0`
///   (cdecl) on +0x3c; `MODS`: the chunk into a stack buffer of its size, then `0048a7b0`
///   on +0x3c with the buffer, the size and `00403570`'s 16-bit count; `ICON`: `0048e3d0`
///   (cdecl) on +0x5c; `MICO`: `0047a290` (cdecl) on +0x68 (these casts null-checked);
/// - `YNAM`: a word given to `006ecd40` on +0x80; `ZNAM`: a word given to `00403550` on
///   +0x80;
/// - `DATA`: `004861f0` (`TESForm::LoadData`) with the file, 0, 0;
/// - `MOD2` and `MOD3`: the models at +0x8c and +0xac (see `load_extra_model`).
///
/// Then true.
pub fn fn_0059b290(e: &mut Engine, this: Ptr, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u8() != 0x74 {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FLAG_00484AB0, &args![this, 0u32]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MODD | CHUNK_MOSD | CHUNK_MODT => {
                let model = cast_or_null(this, 0x3c);
                e.call(0x0048_92d0, &args![model, file]);
            }
            CHUNK_MOD2 => load_extra_model(e, file, base + 0x8c),
            CHUNK_MOD3 => load_extra_model(e, file, base + 0xac),
            CHUNK_DATA => {
                e.call(0x0048_61f0, &args![this, file, 0u32, 0u32]);
            }
            CHUNK_EDID => load_editor_id(e, this, file),
            CHUNK_OBND => {
                e.vcall(base, 0xe0, &args![file]);
            }
            CHUNK_ICON => {
                let texture = cast_or_null(this, 0x5c);
                e.call(0x0048_e3d0, &args![texture, file]);
            }
            CHUNK_FULL => {
                let name = cast_or_null(this, 0x30);
                e.call(0x0048_7050, &args![name, file]);
            }
            CHUNK_YNAM => {
                let word = chunk_word(e, file);
                e.call(0x006e_cd40, &args![base + 0x80, word]);
            }
            CHUNK_ZNAM => {
                let word = chunk_word(e, file);
                e.call(0x0040_3550, &args![base + 0x80, word]);
            }
            CHUNK_MICO => {
                let icon = cast_or_null(this, 0x68);
                e.call(0x0047_a290, &args![icon, file]);
            }
            CHUNK_MODS => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, size]);
                    let count = u32::from(e.call(0x0040_3570, &args![file]).u16());
                    e.call(0x0048_a7b0, &args![base + 0x3c, buffer, size, count]);
                });
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

// Translated from 0059b9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): runs `00470470` with the pointer at
/// +0x24 as `this` (also when it is null), then `00403d30` (cdecl) with `this+0x28`, 0 and
/// 4. The `TESCaravanDeck` destructor `0059b940` runs it first.
pub fn fn_0059b9c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let list = e.mem.u32(base + 0x24);
    e.call(0x0047_0470, &args![list]);
    e.call(0x0040_3d30, &args![base + 0x28, 0u32, 4u32]);
}

// Translated from 0059b940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b940 - the exception-unwinding frame (FS:[0] chain, handler 00ef8ac3, state variable) is not translated.
/// Destructor of `TESCaravanDeck` (the class whose vtable `0x0103_4b4c` the constructor
/// `0059b880` stores; unnamed in the engine map; `ECX` only, plain `RET`): stores the
/// vtables at +0 and +0x18 again, runs `0059b9c0`, then `00591300` on +0x18 and
/// `00483630` (the `TESForm` destructor) on the object.
pub fn fn_0059b940(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_4b4c);
    e.mem.set_u32(base + 0x18, 0x0103_4b38);
    fn_0059b9c0(e, this);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 0059b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059b9f0 - the exception-unwinding frame (FS:[0] chain, handler 00eed64b, state variable) is not translated.
/// Unnamed in the engine map (`ECX` only, plain `RET`; the `TESCaravanDeck` constructor
/// `0059b880` runs it last): allocates 8 bytes through `00401000` (cdecl), constructs them
/// with `0096a2d0` (its `EAX` result is kept; 0 when the allocation failed), stores the
/// result at +0x24 and clears +0x28.
pub fn fn_0059b9f0(e: &mut Engine, this: Ptr) {
    let block = e.call(0x0040_1000, &args![8u32]).u32();
    let list = if block == 0 {
        0
    } else {
        e.call(0x0096_a2d0, &args![block]).u32()
    };
    let base = this.addr();
    e.mem.set_u32(base + 0x24, list);
    e.mem.set_u32(base + 0x28, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_b880, fn_0059b880(Ptr) -> Ptr),
        entry!(0x0059_b910, fn_0059b910(Ptr, u32) -> Ptr),
        entry!(0x0059_b290, fn_0059b290(Ptr, Ptr) -> bool),
        entry!(0x0059_b9c0, fn_0059b9c0(Ptr)),
        entry!(0x0059_b940, fn_0059b940(Ptr)),
        entry!(0x0059_b9f0, fn_0059b9f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0059b880_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_b880,
            size: 0x2c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x75)]),
                (0x0059_b9f0, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_4b4c), (0x18, 4, 0x0103_4b38)],
        });
    }
    #[test]
    fn at_0059b910_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0059_b940, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_b910, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0059_b910)
                .collect();
            let mut expected = vec![(0x0059_b940, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// The calls a test made, without the top-level call and the file's chunk-walking calls.
    fn calls_after(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(address, _)| {
                ![top, GET_TES_FORM, GET_TES_CHUNK, NEXT_CHUNK].contains(address)
            })
            .collect()
    }

    /// Doubles for a file whose record has `form_type` and the given chunks, and no-op
    /// doubles for the loader's callees.
    fn file_with_chunks(e: &mut Engine, form_type: u32, chunks: Vec<u32>) {
        let position = Rc::new(Cell::new(0usize));
        e.register_double(GET_TES_FORM, move |_, _| Ret {
            eax: form_type,
            ..Ret::default()
        });
        let current = position.clone();
        let list = chunks.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| Ret {
            eax: list.get(current.get()).copied().unwrap_or(0),
            ..Ret::default()
        });
        let count = chunks.len();
        e.register_double(NEXT_CHUNK, move |_, _| {
            position.set(position.get() + 1);
            Ret {
                eax: u32::from(position.get() < count),
                ..Ret::default()
            }
        });
        for address in [
            LOAD_FORM,
            SET_FLAG_00484AB0,
            0x0047_27f0,
            GET_CHUNK_DATA_SIZED,
            0x0048_92d0,
            0x0048_61f0,
            0x0048_e3d0,
            0x006e_cd40,
            0x0047_a290,
            0x0040_3550,
            0x0048_7050,
            0x0048_a7b0,
            0x0048_93e0,
            0x0040_3570,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CHUNK_SIZE, |_, _| Ret {
            eax: 0x20,
            ..Ret::default()
        });
    }

    #[test]
    fn caravan_money_load_refuses_another_form_type() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xcc);
        file_with_chunks(&mut e, 0x73, vec![CHUNK_DATA]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_b290, &args![form, 0x5000u32]).bool());
        assert!(calls_after(&mut e, 0x0059_b290).is_empty());
    }

    #[test]
    fn caravan_money_load_dispatches_each_chunk() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xcc);
        let file = 0x5000u32;
        e.put_vtable(0x0300_0000, &[0; 0x80]);
        e.mem.set_u32(0x0300_0000 + 0xe0, 0x0300_10e0);
        e.mem.set_u32(0x0300_0000 + 0x134, 0x0300_1134);
        e.mem.set_u32(form, 0x0300_0000);
        e.put_vtable(0x0300_0400, &[0; 8]);
        e.mem.set_u32(0x0300_0400 + 0x18, 0x0300_1418);
        e.mem.set_u32(form + 0x8c, 0x0300_0400);
        e.mem.set_u32(form + 0xac, 0x0300_0400);
        for address in [0x0300_10e0u32, 0x0300_1134, 0x0300_1418] {
            e.register(address, |_, _| Ret::default());
        }
        file_with_chunks(
            &mut e,
            0x74,
            vec![
                CHUNK_MODL,
                CHUNK_MODT,
                CHUNK_MODD,
                CHUNK_MOSD,
                CHUNK_MOD2,
                CHUNK_MOD3,
                CHUNK_DATA,
                CHUNK_EDID,
                CHUNK_OBND,
                CHUNK_ICON,
                CHUNK_FULL,
                CHUNK_YNAM,
                CHUNK_ZNAM,
                CHUNK_MICO,
                CHUNK_MODS,
                0x5858_5858,
            ],
        );
        e.register(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0059_b290, &args![form, file]).bool());
        let calls = calls_after(&mut e, 0x0059_b290);
        let buffers: Vec<_> = calls
            .iter()
            .filter(|call| call.0 == GET_CHUNK_DATA_SIZED)
            .map(|call| (call.1[1], call.1[2]))
            .collect();
        assert_eq!(buffers.len(), 4);
        let calls: Vec<_> = calls
            .into_iter()
            .filter(|call| {
                ![GET_CHUNK_DATA_SIZED, CHUNK_SIZE, 0x0047_27f0, 0x0040_3570].contains(&call.0)
            })
            .collect();
        assert_eq!(
            calls,
            vec![
                (LOAD_FORM, vec![form, file]),
                (SET_FLAG_00484AB0, vec![form, 0]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0048_92d0, vec![form + 0x3c, file]),
                (0x0300_1418, vec![form + 0x8c, buffers[0].0]),
                (0x0048_93e0, vec![form + 0x8c, file]),
                (0x0300_1418, vec![form + 0xac, buffers[1].0]),
                (0x0048_93e0, vec![form + 0xac, file]),
                (0x0048_61f0, vec![form, file, 0, 0]),
                (0x0300_1134, vec![form, buffers[2].0]),
                (0x0300_10e0, vec![form, file]),
                (0x0048_e3d0, vec![form + 0x5c, file]),
                (0x0048_7050, vec![form + 0x30, file]),
                (0x006e_cd40, vec![form + 0x80, 0x77]),
                (0x0040_3550, vec![form + 0x80, 0x77]),
                (0x0047_a290, vec![form + 0x68, file]),
                (0x0048_a7b0, vec![form + 0x3c, buffers[3].0, 0x20, 0]),
            ]
        );
        assert_eq!(
            buffers.iter().map(|buffer| buffer.1).collect::<Vec<_>>(),
            vec![0, 0, 0x200, 0x20]
        );
    }

    #[test]
    fn fn_0059b9c0_releases_the_list_and_clears_the_count() {
        for list in [0u32, 0x6000] {
            let mut e = Engine::new();
            let deck = e.mem.alloc(0x2c);
            e.mem.set_u32(deck + 0x24, list);
            e.register(0x0047_0470, |_, _| Ret::default());
            e.register(0x0040_3d30, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0059_b9c0, &args![deck]);
            assert_eq!(
                calls_after(&mut e, 0x0059_b9c0),
                vec![
                    (0x0047_0470, vec![list]),
                    (0x0040_3d30, vec![deck + 0x28, 0, 4]),
                ]
            );
        }
    }

    #[test]
    fn fn_0059b940_stores_the_vtables_and_runs_the_destructors() {
        let mut e = Engine::new();
        let deck = e.mem.alloc(0x2c);
        e.mem.set_u32(deck + 0x24, 0x6000);
        for address in [0x0047_0470u32, 0x0040_3d30, 0x0059_1300, 0x0048_3630] {
            e.register(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0059_b940, &args![deck]);
        assert_eq!(
            calls_after(&mut e, 0x0059_b940),
            vec![
                (0x0047_0470, vec![0x6000]),
                (0x0040_3d30, vec![deck + 0x28, 0, 4]),
                (0x0059_1300, vec![deck + 0x18]),
                (0x0048_3630, vec![deck]),
            ]
        );
        assert_eq!(e.mem.u32(deck), 0x0103_4b4c);
        assert_eq!(e.mem.u32(deck + 0x18), 0x0103_4b38);
    }

    #[test]
    fn fn_0059b9f0_allocates_and_constructs_the_list() {
        for (block, expected) in [(0u32, 0u32), (0x7000, 0x7004)] {
            let mut e = Engine::new();
            let deck = e.mem.alloc(0x2c);
            e.mem.set_u32(deck + 0x28, 0xdead);
            e.register_double(0x0040_1000, move |_, _| Ret {
                eax: block,
                ..Ret::default()
            });
            e.register(0x0096_a2d0, |_, a| Ret {
                eax: a[0] + 4,
                ..Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x0059_b9f0, &args![deck]);
            let calls = calls_after(&mut e, 0x0059_b9f0);
            assert_eq!(calls[0], (0x0040_1000, vec![8]));
            assert_eq!(calls.len(), if block == 0 { 1 } else { 2 });
            assert_eq!(e.mem.u32(deck + 0x24), expected);
            assert_eq!(e.mem.u32(deck + 0x28), 0);
        }
    }
}
