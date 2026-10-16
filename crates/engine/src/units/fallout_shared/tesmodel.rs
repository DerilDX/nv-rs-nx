//! `fallout shared/tesmodel.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_004890e0` calls on the member (`00489620`).
const CALLEE_00489620: u32 = 0x0048_9620;

// Translated from 004890e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESModel::ClearDataComponent` (Xbox PDB name in the engine map). Thin wrapper: calls `00489620` with `ECX` = `this + 0xc` and passes on its `bool` result (`AL`).
pub fn fn_004890e0(e: &mut Engine, this: Ptr) -> bool {
    e.call(CALLEE_00489620, &args![this.byte_add(0xc)]).bool()
}

/// Record tags (`u32::from_le_bytes` of the four characters) `TESModel::LoadModelChunk` tells apart.
const MODL: u32 = u32::from_le_bytes(*b"MODL");
const PNAM: u32 = u32::from_le_bytes(*b"PNAM");
const MODD: u32 = u32::from_le_bytes(*b"MODD");
const MOSD: u32 = u32::from_le_bytes(*b"MOSD");
const MODT: u32 = u32::from_le_bytes(*b"MODT");

// Translated from 004892d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004892d0 - the security-cookie check (00ec408c) at the end is not translated.
/// `TESModel::LoadModelChunk` (Xbox PDB): reads the file's current chunk into `model`; does nothing
/// when `file` or `model` is null. cdecl, two arguments.
///
/// - `MODL` and `PNAM`: reads the whole chunk into a buffer of the chunk's size
///   (`TESFile::GetChunkData`, `00472890`, with limit 0) and runs virtual slot `+0x18` of
///   `model` on it.
/// - `MODD` and `MOSD`: reads one byte (`00472890` with 1) and stores it through `004893c0`.
/// - `MODT`: runs `TESModel::LoadModelTextureChunk` (`004893e0`) on `model` and `file`.
///
/// The buffer the game takes from the stack (`__alloca_probe_16`) is a heap block here.
pub fn tesmodel_load_model_chunk(e: &mut Engine, model: Ptr, file: Ptr) {
    if file.addr() == 0 || model.addr() == 0 {
        return;
    }
    // TESFile::GetTESChunk
    let chunk = e.call(0x0047_26b0, &args![file]).u32();
    match chunk {
        MODL | PNAM => {
            // The chunk's size: 00401660.
            let size = e.call(0x0040_1660, &args![file]).u32();
            let buffer = e.mem.alloc(size);
            e.call(0x0047_2890, &args![file, buffer, 0u32]);
            e.vcall(model.addr(), 0x18, &args![buffer]);
            e.mem.free(buffer);
        }
        MODD | MOSD => {
            let byte = e.mem.alloc(1);
            e.call(0x0047_2890, &args![file, byte, 1u32]);
            let value = e.mem.u8(byte);
            e.call(0x0048_93c0, &args![model, u32::from(value)]);
            e.mem.free(byte);
        }
        MODT => {
            e.call(0x0048_93e0, &args![model, file]);
        }
        _ => {}
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_90e0, fn_004890e0(Ptr) -> bool),
        entry!(0x0048_92d0, tesmodel_load_model_chunk(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_004890e0_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00489620, |_, a| Ret {
            eax: a[0] & 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_90e0, &args![0x4100u32]).bool());
        assert!(e.call(0x0048_90e0, &args![0x4100u32 + 1]).bool());
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00489620).collect();
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

/// Test support shared by the record loaders of this work package: `TESFile` doubles that walk a
/// list of chunks.
#[cfg(test)]
pub(crate) mod load_test {
    use crate::prelude::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// The file object the tests pass (never dereferenced: every `TESFile` function is a double).
    pub const FILE: u32 = 0x0f11_e000;
    /// The size `00401660` reports for the current chunk.
    pub const CHUNK_SIZE: u32 = 0x20;
    /// The value `TESFile::GetChunkData` (`004727f0`) writes.
    pub const CHUNK_VALUE: u32 = 0x1234_5678;
    pub type Calls = Vec<(u32, Vec<u32>)>;

    /// Addresses of the `TESFile` callees every loader runs; [`run`] leaves them out of the log.
    const FILE_CALLS: [u32; 6] = [
        0x0047_2660,
        0x0047_26b0,
        0x0047_26f0,
        0x0040_1660,
        0x0040_1680,
        0x0040_3570,
    ];

    /// An engine whose file is positioned on the first of `chunks`. `form_type` is what
    /// `TESFile::GetTESForm` answers and `version_flag` what `00401680` answers. `00472890`
    /// fills the buffer with `0x5a` (at most `CHUNK_SIZE` bytes) and `004727f0` stores
    /// [`CHUNK_VALUE`].
    pub fn engine(form_type: u32, chunks: &[u32], version_flag: bool) -> Engine {
        let mut e = Engine::new();
        let position = Rc::new(Cell::new(0usize));
        let chunks = chunks.to_vec();
        e.register_double(0x0047_2660, move |_, _| Ret {
            eax: form_type,
            ..Ret::default()
        });
        let at = position.clone();
        e.register_double(0x0047_26b0, move |_, _| Ret {
            eax: chunks.get(at.get()).copied().unwrap_or(0),
            ..Ret::default()
        });
        e.register_double(0x0047_26f0, move |_, _| {
            position.set(position.get() + 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1660, |_, _| Ret {
            eax: CHUNK_SIZE,
            ..Ret::default()
        });
        e.register_double(0x0040_1680, move |_, _| Ret {
            eax: u32::from(version_flag),
            ..Ret::default()
        });
        e.register_double(0x0040_3570, |_, _| Ret {
            eax: 0xabcd_1234,
            ..Ret::default()
        });
        e.register_double(0x0047_2890, |e, a| {
            let (buffer, limit) = (a[1], a[2]);
            let count = if limit == 0 {
                CHUNK_SIZE
            } else {
                limit.min(CHUNK_SIZE)
            };
            e.mem.write(buffer, &vec![0x5a; count as usize]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0047_27f0, |e, a| {
            e.mem.set_u32(a[1], CHUNK_VALUE);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e
    }

    /// Registers do-nothing doubles for `callees`.
    pub fn stub(e: &mut Engine, callees: &[u32]) {
        for callee in callees {
            e.register(*callee, |_, _| Ret::default());
        }
    }

    /// Calls `addr` with `words` and returns the answer and the calls it made, without the
    /// `TESFile` queries of [`FILE_CALLS`].
    pub fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Calls) {
        e.call_log = Some(vec![]);
        let ret = e.call(addr, words);
        let log = e.call_log.take().unwrap();
        let calls = log
            .into_iter()
            .skip(1)
            .filter(|(callee, _)| !FILE_CALLS.contains(callee))
            .collect();
        (ret, calls)
    }

    /// The record tag of four characters.
    pub fn tag(text: &[u8; 4]) -> u32 {
        u32::from_le_bytes(*text)
    }

    /// A zeroed object of `size` bytes whose vtable, placed at `vtable`, has the given
    /// (byte offset, target) slots; every target is a do-nothing double.
    pub fn object(e: &mut Engine, size: u32, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(size);
        e.map(vtable, 0x200);
        e.mem.set_u32(this, vtable);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
            e.register(*target, |_, _| Ret::default());
        }
        this
    }

    /// The calls to `addr`.
    pub fn calls_to(calls: &Calls, addr: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }
}

#[cfg(test)]
mod movement_path_tests {
    use super::load_test::{self, tag, FILE};
    use crate::prelude::*;

    const MODEL: u32 = 0x0800_0000;

    /// An engine with a model whose virtual slot `+0x18` is a double at `0x0090_0200`.
    fn setup(chunks: &[u32]) -> Engine {
        let mut e = load_test::engine(0, chunks, false);
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0x0090_0200]);
        e.map(MODEL, 0x40);
        e.mem.set_u32(MODEL, 0x0090_1000);
        load_test::stub(&mut e, &[0x0090_0200, 0x0048_93c0, 0x0048_93e0]);
        e
    }

    #[test]
    fn nothing_happens_without_a_model_or_a_file() {
        let mut e = setup(&[tag(b"MODL")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[0, FILE]);
        assert!(calls.is_empty());
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, 0]);
        assert!(calls.is_empty());
    }

    #[test]
    fn the_name_chunks_are_read_whole_and_given_to_the_model() {
        for chunk in [tag(b"MODL"), tag(b"PNAM")] {
            let mut e = setup(&[chunk]);
            let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
            assert_eq!(calls.len(), 2);
            let read = &calls[0];
            assert_eq!((read.0, read.1[0], read.1[2]), (0x0047_2890, FILE, 0));
            let buffer = read.1[1];
            assert_eq!(calls[1], (0x0090_0200, vec![MODEL, buffer]));
            // The buffer is released again.
            assert_eq!(e.mem.block_size(buffer), None);
        }
    }

    #[test]
    fn the_flag_chunks_read_one_byte_and_store_it() {
        for chunk in [tag(b"MODD"), tag(b"MOSD")] {
            let mut e = setup(&[chunk]);
            let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
            assert_eq!(calls.len(), 2);
            assert_eq!(
                (calls[0].0, calls[0].1[0], calls[0].1[2]),
                (0x0047_2890, FILE, 1)
            );
            assert_eq!(calls[1], (0x0048_93c0, vec![MODEL, 0x5a]));
        }
    }

    #[test]
    fn the_texture_chunk_goes_to_the_texture_loader_and_others_are_ignored() {
        let mut e = setup(&[tag(b"MODT")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert_eq!(calls, [(0x0048_93e0, vec![MODEL, FILE])]);

        let mut e = setup(&[tag(b"EDID")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert!(calls.is_empty());
        // Only the first chunk is read.
        let mut e = setup(&[tag(b"EDID"), tag(b"MODT")]);
        let (_, calls) = load_test::run(&mut e, 0x0048_92d0, &[MODEL, FILE]);
        assert!(calls.is_empty());
    }
}
