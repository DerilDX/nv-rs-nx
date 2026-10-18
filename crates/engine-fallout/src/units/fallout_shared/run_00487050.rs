//! `fallout shared/run_00487050` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesfile::TESFile;

/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`): the current chunk id.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// The current chunk's size (`00401660`, `TESFile` `+0x25C`).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB overload `_ov3`, `00472890`): `thiscall
/// (file; buffer, size)`, here with size 0.
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// The string setter at `00489100` (`thiscall (component; text)`; engine map
/// name `TESSoundFile::SetSoundFile`, a shared body).
const SET_STRING: u32 = 0x0048_9100;
/// The chunk id this loader reads: the bytes `FULL` as a little-endian word.
const FULL_CHUNK: u32 = 0x4c4c_5546;

// Translated from 00487050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00487050 - the /GS stack cookie check (00ec408c) is not translated.
/// `cdecl (component, file)`: with both non-null and the file's current
/// chunk a `FULL` chunk, reads the chunk into a stack buffer of the chunk's
/// size (`_alloca_probe_16`) and hands it to the component's string setter
/// (`00489100`); an empty chunk sets a null string. Any other chunk leaves
/// the component as it is. Other translations call it the `TESFullName`
/// loader; the engine map gives it no name (unconfirmed).
pub fn fn_00487050(e: &mut Engine, component: Ptr, file: Ptr<TESFile>) {
    if file.addr() == 0 || component.addr() == 0 {
        return;
    }
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    if chunk != FULL_CHUNK {
        return;
    }
    if e.call(CHUNK_SIZE, &args![file]).u32() == 0 {
        e.call(SET_STRING, &args![component, 0u32]);
        return;
    }
    let size = e.call(CHUNK_SIZE, &args![file]).u32();
    // `_alloca_probe_16`: the buffer is the size rounded up to 16 bytes.
    let reserved = size.wrapping_add(15) & !15;
    e.with_stack(reserved, |e, buffer| {
        e.call(GET_CHUNK_DATA, &args![file, buffer, 0u32]);
        e.call(SET_STRING, &args![component, buffer]);
    });
}

// Translated from 004871d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form component `TESHealthForm` (the engine map
/// gives it no name; the RTTI name of the vtable it stores, `0101cb94`, is
/// `.?AVTESHealthForm@@`): runs the base constructor (`00402d30`), stores the
/// vtable at `+0`, then calls `TESHealthForm::InitializeDataComponent`
/// (Xbox PDB name of `00487220`) directly. Returns `this`.
pub fn fn_004871d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_cb94);
    e.call(0x0048_7220, &args![this]);
    this
}

// Translated from 00487200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `0101cb94` (RTTI `.?AVTESHealthForm@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00483710`. One word, plain `RET`, returns nothing.
pub fn fn_00487200(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_cb94);
    e.call(0x0048_3710, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_7200, fn_00487200(Ptr)),
        entry!(0x00487050, fn_00487050(Ptr, Ptr<TESFile>)),
        entry!(0x004871d0, fn_004871d0(Ptr) -> Ptr),
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

    /// Runs the loader on a file whose current chunk is `chunk` with `size`
    /// bytes "NAME\0"; returns the log and the text the setter saw.
    fn run(component: u32, file: u32, chunk: u32, size: u32) -> (CallLog, Option<String>) {
        let mut e = Engine::new();
        e.register_double(GET_TES_CHUNK, move |_, _| chunk.into_ret());
        e.register_double(CHUNK_SIZE, move |_, _| size.into_ret());
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.write(a[1], b"NAME\0");
            Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let record = seen.clone();
        e.register_double(SET_STRING, move |e, a| {
            let text = if a[1] == 0 {
                String::from("<null>")
            } else {
                String::from_utf8(e.mem.bytes(a[1], 4)).unwrap()
            };
            *record.borrow_mut() = Some(text);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_7050, &args![component, file]);
        let log = e.call_log.take().unwrap();
        let text = seen.borrow().clone();
        (log, text)
    }

    #[test]
    fn reads_a_full_chunk_into_the_component() {
        let (log, text) = run(0x5000, 0x6000, FULL_CHUNK, 5);
        assert_eq!(text.as_deref(), Some("NAME"));
        let reads = calls_to(&log, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!((reads[0][0], reads[0][2]), (0x6000, 0));
        assert_eq!(calls_to(&log, SET_STRING)[0][0], 0x5000);
        assert_eq!(calls_to(&log, CHUNK_SIZE).len(), 2);
    }

    #[test]
    fn an_empty_full_chunk_sets_a_null_string() {
        let (log, text) = run(0x5000, 0x6000, FULL_CHUNK, 0);
        assert_eq!(text.as_deref(), Some("<null>"));
        assert_eq!(calls_to(&log, SET_STRING), vec![vec![0x5000, 0]]);
        assert!(calls_to(&log, GET_CHUNK_DATA).is_empty());
    }

    #[test]
    fn other_chunks_and_null_arguments_do_nothing() {
        let (log, text) = run(0x5000, 0x6000, 0x4c4c_5547, 5);
        assert!(text.is_none());
        assert!(calls_to(&log, CHUNK_SIZE).is_empty());
        let (log, _) = run(0, 0x6000, FULL_CHUNK, 5);
        assert!(calls_to(&log, GET_TES_CHUNK).is_empty());
        let (log, _) = run(0x5000, 0, FULL_CHUNK, 5);
        assert!(calls_to(&log, GET_TES_CHUNK).is_empty());
    }
}

#[cfg(test)]
mod component_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004871d0_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_71d0,
            size: 0x10,
            globals: &[],
            returns: &[],
            calls: &[(0x0040_2d30, &[A::T(0x0)]), (0x0048_7220, &[A::T(0x0)])],
            stores: &[(0x0, 4, 0x0101_cb94)],
        });
    }

    #[test]
    fn fn_00487200_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0048_3710, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0048_7200, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0048_3710)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0101_cb94);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0101_cb94,
            "the vtable was stored before the call"
        );
    }
}
