//! `(unplaced)/run_00c64c30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c6ba30`: the jump target of `00c657f0`.
const JUMP_TARGET_00C657F0: u32 = 0x00c6_ba30;

// Translated from 00c657f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00c6ba30`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00c657f0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00C657F0, &args![this, first, second]);
}

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c65160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c39bc (unit `run_00c64c30`). Forwards `this` and
/// the stream to `00a7b3b0` (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c65160(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

// Translated from 00c657e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043f0 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00c657e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43f0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_57e0, fn_00c657e0(Ptr) -> u32),
        entry!(0x00c6_57f0, fn_00c657f0(Ptr, Ptr, Ptr)),
        entry!(0x00c6_5160, fn_00c65160(Ptr, Ptr)),
        entry!(0x00c6_5530, fn_00c65530()),
        entry!(0x00c6_55d0, fn_00c655d0()),
    ]
}

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `"BSAttachPoints"`.
const ATTACH_POINTS_NAME: u32 = 0x010c_3a18;
/// The global holding the allocated `NiFixedString`.
const ATTACH_POINTS_STRING: u32 = 0x0120_43ec;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c65530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65530 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f306a5, state variable) is not translated
/// A start-up initializer: allocates a 4-byte `NiFixedString` from the memory manager and
/// sets it to `"BSAttachPoints"` (`00a5b690`); the compiler kept a test of the constant
/// text's address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) instead never runs (translated anyway). The string (or null when the
/// allocation failed) goes to `012043ec`.
pub fn fn_00c65530(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(ATTACH_POINTS_NAME);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![ATTACH_POINTS_NAME])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(ATTACH_POINTS_STRING, stored);
}

// Translated from 00c655d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the string at `012043ec` exists, releases its handle (unless
/// it is the empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the
/// text), frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always
/// true), and clears the global.
pub fn fn_00c655d0(e: &mut Engine) {
    let string = e.global::<u32>(ATTACH_POINTS_STRING);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(ATTACH_POINTS_STRING, 0);
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn initializer_makes_the_string_or_stores_null() {
        for allocated in [false, true] {
            let mut e = Engine::new();
            e.map(ATTACH_POINTS_STRING, 4);
            e.mem.set_u32(ATTACH_POINTS_STRING, 0xffff);
            let block = if allocated { e.mem.alloc(4) } else { 0 };
            e.register_double(ALLOCATE, move |_, _| block.into_ret());
            e.register(FIXED_STRING_CREATE, |_, _| 0x7777u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x00c6_5530, &args![]);
            let mut want = vec![(ALLOCATE, vec![MEMORY_MANAGER, 4])];
            if block != 0 {
                want.push((FIXED_STRING_CREATE, vec![ATTACH_POINTS_NAME]));
                assert_eq!(e.mem.u32(block), 0x7777);
            }
            assert_eq!(log_of(&mut e, 0x00c6_5530), want);
            assert_eq!(e.mem.u32(ATTACH_POINTS_STRING), block);
        }
    }

    #[test]
    fn destructor_releases_the_handle_and_frees_the_string() {
        for (handle, released) in [(0x8008u32, true), (0x4444, false)] {
            let mut e = Engine::new();
            e.map(ATTACH_POINTS_STRING, 4);
            e.map(FIXED_STRING_EMPTY, 4);
            e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
            let string = e.mem.alloc(4);
            e.mem.set_u32(string, handle);
            e.mem.set_u32(ATTACH_POINTS_STRING, string);
            e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x00c6_55d0, &args![]);
            let mut want = vec![];
            if released {
                want.push((INTERLOCKED_DECREMENT, vec![0x8000]));
            }
            want.push((DEALLOCATE, vec![MEMORY_MANAGER, string]));
            assert_eq!(log_of(&mut e, 0x00c6_55d0), want);
            assert_eq!(e.mem.u32(ATTACH_POINTS_STRING), 0);
        }
        // No string: only the global is cleared.
        let mut e = Engine::new();
        e.map(ATTACH_POINTS_STRING, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c6_55d0, &args![]);
        assert!(log_of(&mut e, 0x00c6_55d0).is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c657f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_57f0;
        const TARGET: u32 = 0x00c6_ba30;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn forwarder_fn_00c65160_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5160, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c657e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_57e0, &args![0x1000u32]).u32(), 0x0120_43f0);
        assert_eq!(e.call(0x00c6_57e0, &args![0u32]).u32(), 0x0120_43f0);
    }
}
